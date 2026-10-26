//! Showcase self-tests (spec v0.5.9 §6.1)
//!
//! `test = true` on the example target (Cargo.toml) puts these under a plain
//! `cargo test`, so CI and the nightly dependency canary run them with no
//! workflow change. They build the real `Showcase` on GPUI's headless test
//! platform — the same view, the same `Root`, the same window width `main`
//! opens — and drive it with real input.

use gpui::{
    AbsoluteLength, AnyWindowHandle, App, Bounds, DefiniteLength, Entity, Focusable as _, Length,
    Modifiers, MouseButton, Pixels, Point, TestAppContext, VisualTestContext, point, prelude::*,
    px, rems, size,
};
use gpui_base::ScrollbarHandle as _;
use gpui_component::{Colorize as _, IconName, Root, WindowExt as _, theme::Theme};
use native_theme_gpui::{ActiveNativeTheme, geometry};
use std::{rc::Rc, time::Duration};

use crate::app::{
    AppColorMode, OpenCommandPalette, OpenPreferences, Quit, ReloadTheme, SetColorMode, SetPreset,
    ShowPage, Showcase, ToggleSidePanel,
};
use crate::chrome::menus;
use crate::demo::{AREA_FILL_OPACITY, IconSizeContext, RADAR_FILL_OPACITY};
use crate::info::{
    GEOMETRY_NOTES, INFO_SETTLE, InfoExt as _, InfoRegistry, WidgetInfo, as_rgba, claim,
    native_info, percent_text,
};
use crate::inspector::InspectorTab;
use crate::support::{
    CAROUSEL_SLIDES, ChromeIcon, load_gpui_icons, native_color, native_geometry, native_value,
};
use crate::{
    BASIC_INPUT_DISABLED, BUTTONS_DANGER, BUTTONS_DISABLED_SECONDARY, BUTTONS_HEADING_VARIANTS,
    BUTTONS_PRIMARY, BUTTONS_TEXT, CHARTS_AREA_CHART, CHARTS_BAR_CHART, CHARTS_CANDLESTICK_CHART,
    CHARTS_LINE_CHART, CHARTS_PIE_CHART, CHARTS_RADAR_CHART, CHARTS_SANKEY_CHART,
    CHROME_APP_MENU_BAR, CHROME_HANDLE, CHROME_LABEL_ICON_THEME, CHROME_LABEL_MODE,
    CHROME_LABEL_THEME, CHROME_MENU_BAR, CHROME_PAGE_TABS, CHROME_SIDE_PANEL,
    CHROME_SIDE_PANEL_SEPARATOR, CHROME_SIDE_PANEL_TOGGLE, CHROME_SPLITTER_LINE, CHROME_STATUS_BAR,
    CHROME_THEME_SETTINGS, CHROME_TITLE_BAR, CHROME_TOOLBAR, CHROME_TOOLBAR_PALETTE,
    CHROME_TOOLBAR_PREFERENCES, CHROME_TOOLBAR_RELOAD, CONTENT_ALERT, CONTENT_PANEL,
    CONTENT_SCROLL, DATA_PAGINATION, DATA_PAGINATION_COMPACT, DATA_TABLE_HEADER,
    FEEDBACK_ALERT_INFO, FEEDBACK_BADGE_COUNT, FEEDBACK_BADGE_DOT, FEEDBACK_CIRCLE_LOADING,
    FEEDBACK_SPINNER_SMALL, FEEDBACK_TAG_DANGER, FEEDBACK_TAG_PRIMARY, INPUTS_CHECKBOX_AUTOSAVE,
    INPUTS_CHECKBOX_NOTIFICATIONS, INPUTS_FIELD, INPUTS_FIELD_HEIGHT_ONLY, INPUTS_TEXTAREA,
    INSPECTOR_COPY, INSPECTOR_PANEL, INSPECTOR_TABS, INSPECTOR_TITLE, INSPECTOR_TOKENS_NOTE,
    LAYOUT_BREADCRUMB, LAYOUT_COLLAPSIBLE, LAYOUT_COLLAPSIBLE_CONTENT, LAYOUT_COLLAPSIBLE_TOGGLE,
    LAYOUT_FOLDER_TAB_DISABLED, LAYOUT_FOLDER_TAB_LABELS, LAYOUT_FOLDER_TABS,
    LAYOUT_GROUP_BOX_NORMAL, LAYOUT_GROUP_BOX_OUTLINE, LAYOUT_SEPARATOR_DASHED,
    LAYOUT_SEPARATOR_SOLID, LAYOUT_SIDEBAR_COLLAPSED, LAYOUT_SIDEBAR_EXPANDED,
    LAYOUT_SIDEBAR_ITEMS, LAYOUT_TITLE_BAR, LEFT_PANEL_WIDTH, LIST_DEMO, OVERLAY_ABOUT_LINK,
    OVERLAY_ABOUT_NAME, OVERLAY_ABOUT_TEXT, OVERLAY_ABOUT_TITLE, OVERLAY_PALETTE,
    OVERLAY_PALETTE_TITLE, OVERLAY_PREFERENCES, OVERLAYS_DIALOG_CLOSE, OVERLAYS_DIALOG_FOOTER,
    OVERLAYS_DIALOG_TRIGGER, OVERLAYS_SHEET_BOTTOM, OVERLAYS_SHEET_BOTTOM_TITLE,
    OVERLAYS_SHEET_RIGHT, OVERLAYS_SHEET_RIGHT_TITLE, PAGE_ROOT, PAGE_WIDTH_PX, POINTER_SHIELD,
    PREF_REDUCE_MOTION, PROBE_ALERT_DIALOG, PROBE_ATTACHMENT, PROBE_CAROUSEL_LAST, PROBE_CHAT_SEND,
    PROBE_CLIPBOARD, PROBE_COLOR_MODE, PROBE_COMBOBOX, PROBE_ICON_THEME, PROBE_NOTIFICATION,
    PROBE_PAGINATION, PROBE_RATING, PROBE_SETTINGS_ROW, PROBE_STEPPER, Page, STATUS_ENVIRONMENT,
    STATUS_HOVERED, STATUS_MIDDLE, TREE_DEMO, TYPOGRAPHY_H1, TYPOGRAPHY_H2, TYPOGRAPHY_LABEL_PLAIN,
    TYPOGRAPHY_LABEL_SECONDARY, WINDOW_SIZE, WINDOW_TITLE,
};
use native_theme::icons::IconSetChoice;

/// The painted scene, read as the widget tests read it.
#[path = "../../src/widgets/scene_read.rs"]
mod scene_read;
use scene_read::{scene_quads, scene_underlines};

/// The window the interaction test lays the showcase out in.
///
/// The width is the application's own. The height is not: a page is one
/// long scrolling column, and an element scrolled out of the viewport is
/// clipped out of the frame and cannot be clicked, so this window is tall
/// enough to hold the longest page whole. `every_page_lays_out` uses
/// `WINDOW_SIZE` instead, which is what puts the scroll container to work.
const TALL_WINDOW: gpui::Size<Pixels> = size(WINDOW_SIZE.width, px(9000.));

/// The desktop's own theme, for a test that installs it. On Linux it is
/// never an error: a desktop with no reader falls back to the platform
/// preset (native-theme `pipeline.rs`, `linux_fallback`), so an error there
/// fails the test rather than skip its check. Off Linux a build without a
/// reader feature has no OS theme, and the error says so.
fn system_theme() -> native_theme::Result<native_theme::SystemTheme> {
    let system = native_theme::SystemTheme::from_system();
    if cfg!(target_os = "linux") {
        return Ok(system.expect("Linux falls back to the platform preset"));
    }
    system
}

/// Build the showcase the way `main` does — `gpui_kit::init`, the showcase's
/// actions and key bindings, the window options, the view, and the `Root`
/// that owns the dialog and notification layers — in a window of
/// `window_size`.
///
/// One thing `main` has that a test window cannot: the asset source.
/// `gpui_kit::application().with_assets(gpui_kit::assets::Assets)` has no
/// counterpart here — `TestAppContext::build` hands the app `Arc::new(())`
/// through `build_with_platform` and exposes no setter
/// (`gpui-pre-0.3.10/src/app/test_context.rs:132-144`, `:176-180`) — so
/// gpui-component's own `IconName` SVGs resolve to nothing. The pages still
/// lay out, which is what these tests measure; the showcase's native icons
/// do not come from the asset source at all, they are decoded into
/// `ImageSource` by the connector.
fn open(
    cx: &mut TestAppContext,
    window_size: gpui::Size<Pixels>,
) -> (Entity<Showcase>, AnyWindowHandle, VisualTestContext) {
    open_with(
        cx,
        crate::window_options(Bounds {
            origin: Point::default(),
            size: window_size,
        }),
    )
}

/// [`open`], in a window opened with `options`.
fn open_with(
    cx: &mut TestAppContext,
    options: gpui::WindowOptions,
) -> (Entity<Showcase>, AnyWindowHandle, VisualTestContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        crate::app::init(cx);
    });
    let handle = cx
        .update(|cx| {
            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| Showcase::new(window, cx))
            })
        })
        .expect("the window opened");
    let (window_handle, showcase) = handle;
    let mut cx = VisualTestContext::from_window(window_handle, cx);
    cx.run_until_parked();
    draw(&mut cx);
    (showcase, window_handle, cx)
}

/// Lay the window out and paint it, which is what fills `debug_bounds`,
/// and check that no frame drawn so far drew two info targets under one id.
fn draw(cx: &mut VisualTestContext) {
    cx.update(|window, cx| window.draw(cx).clear(cx));
    let twice = ids_drawn_twice(cx);
    assert!(
        twice.is_empty(),
        "one frame drew two info targets under the same id, which the registry \
         takes for one widget: {twice:?}"
    );
}

/// The info ids a frame of this window has drawn two targets under: the
/// showcase's registry, or the test view's. Empty for a window whose root
/// holds neither, which has no registry to read.
fn ids_drawn_twice(cx: &mut VisualTestContext) -> Vec<String> {
    cx.update(|window, cx| {
        let ui = if let Some(Some(root)) = window.root::<Root>() {
            root.read(cx)
                .view()
                .clone()
                .downcast::<Showcase>()
                .ok()
                .map(|showcase| showcase.read(cx).info_ui.clone())
        } else if let Some(Some(nested)) = window.root::<Nested>() {
            Some(nested.read(cx).ui.clone())
        } else {
            None
        };
        ui.map(|ui| ui.read(cx).drawn_twice.iter().cloned().collect())
            .unwrap_or_default()
    })
}

/// Switch to `page` and draw the frame that shows it, then the frame a
/// scroll area asks for when the page's overflow changed its scrollbar's
/// strip (`support::gutter_scroll`).
fn show(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, page: Page) {
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| this.show_page(page, cx));
    });
    cx.run_until_parked();
    draw(cx);
    cx.run_until_parked();
    draw(cx);
}

/// Where the element tagged `selector` was laid out.
fn bounds_of(cx: &mut VisualTestContext, selector: &'static str) -> Bounds<Pixels> {
    cx.debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} was not laid out"))
}

/// Click `at`, let the click's work finish and draw the frame it produced.
fn click_at(cx: &mut VisualTestContext, at: Point<Pixels>) {
    cx.simulate_click(at, Modifiers::default());
    cx.run_until_parked();
    draw(cx);
}

/// Click the leading edge of the element tagged `selector`.
///
/// The leading edge, not the middle: a probe around a control in a block
/// container is as wide as the column, and the control it holds sits at the
/// left of it, so the middle of the probe can be empty space. Upstream's own
/// interaction tests click the same way (`tests/controlled_change_callbacks.rs`).
fn click(cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = bounds_of(cx, selector);
    click_at(cx, point(bounds.left() + px(8.), bounds.center().y));
}

/// Read something off the showcase's model.
fn read<R>(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    f: impl FnOnce(&Showcase, &App) -> R,
) -> R {
    cx.update(|_window, cx| f(showcase.read(cx), cx))
}

/// Every page lays out: the TabBar's eleven pages each render on the test
/// platform, each leaves a page root behind, and that root has a size.
///
/// And no frame draws two info targets under one id (info/registry.rs,
/// `drawn_twice`): the registry keys a target by its id alone, so two would
/// be taken for one widget. That is checked where the ids are drawn, so it
/// holds for an id built at run time -- a `format!` or a part named after
/// its widget -- as much as for a literal. Its limit is what is drawn: a
/// page's widgets here, and in the other tests, through `draw`, the chrome
/// and whichever dialogs, sheets and menus a test opens. An overlay no test
/// opens is not checked.
#[gpui::test]
fn every_page_lays_out(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(Page::ALL.len(), 11, "the TabBar no longer has eleven pages");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Basic,
        "the showcase does not open on the Basic page"
    );
    assert_eq!(Page::ALL[0], Page::Basic, "Basic is not the first page");
    for page in Page::ALL {
        show(&mut cx, &showcase, page);
        assert_eq!(read(&mut cx, &showcase, |this, _| this.active_page), page);
        assert_eq!(
            ids_drawn_twice(&mut cx),
            Vec::<String>::new(),
            "{page:?}: a frame drew two info targets under one id"
        );
        let bounds = cx
            .debug_bounds(PAGE_ROOT)
            .unwrap_or_else(|| panic!("{page:?}: nothing was laid out in the content panel"));
        assert!(
            bounds.size.width > px(0.) && bounds.size.height > px(0.),
            "{page:?}: the page root laid out at {:?}",
            bounds.size
        );
    }
}

/// Install a bundled preset, so a measurement does not depend on the
/// desktop the test runs on. A preset that does not install fails the test,
/// which would otherwise measure the theme installed before.
fn use_preset(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, preset: &str) {
    let installed = install_by_name(cx, showcase, preset);
    assert!(installed, "the preset {preset:?} did not install");
}

/// Ask for a preset that does not exist, to provoke a theme error: the
/// install must fail.
fn use_missing_preset(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, preset: &str) {
    let installed = install_by_name(cx, showcase, preset);
    assert!(!installed, "the missing preset {preset:?} installed");
}

/// Install the preset of `name` through `apply_theme_by_name`, draw the
/// frame it produced, and return whether it installed.
fn install_by_name(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, name: &str) -> bool {
    let installed = cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            let installed = this.apply_theme_by_name(name, window, cx);
            cx.notify();
            installed
        })
    });
    cx.run_until_parked();
    draw(cx);
    installed
}

/// The installed theme's scrollbar groove width, and whether the platform
/// draws its scrollbars as overlays; a zero width means no native theme is
/// installed, which the caller asserts against.
fn scrollbar_of(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> (Pixels, bool) {
    read(cx, showcase, |_this, cx| match cx.native() {
        Some(n) => (
            px(native_theme_gpui::scrollbar_groove_width(n.resolved)),
            n.resolved.scrollbar.overlay_mode,
        ),
        None => (px(0.), false),
    })
}

/// A scrollbar that is not an overlay keeps off the content.
///
/// gpui-component overlays its scrollbar on the scroll area whatever the
/// platform does (`src/scroll/scrollable.rs`, `Scrollable`), and gpui-base
/// draws the vertical track flush with the area's right edge, at the width
/// the connector installed (gpui-base `src/scrollbar.rs:1422-1446`). Where
/// the platform's scrollbars are not overlays the connector also asks for
/// an always-visible bar (`src/lib.rs`, `scrollbar_mode`), so that width
/// has to be reserved beside the content or the bar sits on it.
#[gpui::test]
fn a_non_overlay_scrollbar_keeps_off_the_content(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    // kde-breeze puts its groove beside the content; macOS overlays its
    // own, and must not lose a strip of width to it.
    for preset in ["kde-breeze", "macos-monterey"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Inputs);
        let (groove, overlay) = scrollbar_of(&mut cx, &showcase);
        assert!(
            groove > px(0.),
            "{preset}: no native theme is installed, so nothing was measured"
        );
        let content = bounds_of(&mut cx, CONTENT_SCROLL);
        let page = bounds_of(&mut cx, PAGE_ROOT);
        assert!(
            page.size.height > WINDOW_SIZE.height,
            "{preset}: the page is shorter than the whole window, so the pane may \
                 not scroll at all and this step would prove nothing"
        );
        let gutter = if overlay { px(0.) } else { groove };
        assert_eq!(
            content.right() - page.right(),
            gutter,
            "{preset}: the page ends at {:?} and the scroll area at {:?}, which \
                 leaves {gutter:?} free for a {groove:?} scrollbar",
            page.right(),
            content.right(),
        );
    }
}

/// A page that fits its pane keeps no strip for a scrollbar: gpui-base
/// draws none while the content fits (gpui-base src/scrollbar.rs:1416-1419),
/// so the Basic page takes the pane's whole width under kde-breeze, whose
/// scrollbars are not overlays.
#[gpui::test]
fn a_page_that_fits_keeps_no_scrollbar_strip(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let (groove, overlay) = scrollbar_of(&mut cx, &showcase);
    assert!(
        !overlay && groove > px(0.),
        "kde-breeze must draw a non-overlay groove"
    );
    let content = bounds_of(&mut cx, CONTENT_SCROLL);
    let page = bounds_of(&mut cx, PAGE_ROOT);
    assert!(
        page.size.height <= content.size.height,
        "the Basic page does not fit its pane, so this proves nothing"
    );
    assert_eq!(
        content.right(),
        page.right(),
        "the Basic page fits, yet keeps a strip for a scrollbar that is not drawn"
    );
}

/// A Settings row keeps off the page's scrollbar.
///
/// The Layout page's scroll area scrolls its content inside its frame: the
/// frame stays put while the items move, and the scrolled content, as wide
/// as the box the scrollbar is drawn at the right edge of, starts inside the
/// frame's border on every side but the bottom (it runs on below, clipped),
/// so the bar cannot cover the border's right edge.
#[gpui::test]
fn the_scroll_area_scrolls_inside_its_frame(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Layout);
    let frame = bounds_of(&mut cx, "layout-scroll-area-frame");
    let content = bounds_of(&mut cx, "layout-scroll-area-viewport");
    let border = cx.update(|_window, cx| crate::support::demo_border_width(cx));
    assert!(
        content.left() >= frame.left() + border
            && content.right() <= frame.right() - border
            && content.top() >= frame.top() + border,
        "the scrolled content {content:?} reaches into the frame's {border:?} border {frame:?}"
    );
    let first = bounds_of(&mut cx, "layout-scroll-area-item-1");
    scroll_at(&mut cx, frame.center(), -px(60.));
    assert_eq!(
        bounds_of(&mut cx, "layout-scroll-area-frame"),
        frame,
        "the frame moved with the content"
    );
    assert!(
        bounds_of(&mut cx, "layout-scroll-area-item-1").top() < first.top(),
        "the content did not scroll"
    );
}

/// The page body lays gpui-component's `ScrollbarLayer` over its right edge
/// (setting/page.rs:227-257, scroll/scrollable.rs:19-29) and reserves only the
/// `px_4` each group's row is wrapped in, inside the list (setting/page.rs:241-245);
/// kde-breeze's groove is 21px, wider than that, so without a gutter on the
/// group the bar covers the rows.
#[gpui::test]
fn a_settings_row_keeps_off_the_page_scrollbar(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Layout);
    let (groove, overlay) = scrollbar_of(&mut cx, &showcase);
    assert!(
        !overlay && groove > px(0.),
        "kde-breeze must draw a non-overlay groove"
    );
    let frame = bounds_of(&mut cx, "settings-frame");
    let row = bounds_of(&mut cx, PROBE_SETTINGS_ROW);
    assert!(
        frame.right() - row.right() >= groove,
        "the row ends {:?} before the frame's right edge; a {groove:?} groove covers it",
        frame.right() - row.right()
    );
}

/// Turn the wheel by `delta` pixels over `at`, and draw the frame it
/// produced. A negative delta is a scroll towards the end of the content.
fn scroll_at(cx: &mut VisualTestContext, at: Point<Pixels>, delta: Pixels) {
    cx.simulate_event(gpui::ScrollEvent {
        position: at,
        delta: gpui::ScrollDelta::Pixels(point(px(0.), delta)),
        modifiers: Modifiers::default(),
        touch_phase: gpui::TouchPhase::Moved,
        source: gpui::ScrollSource::Pointer,
    });
    cx.run_until_parked();
    draw(cx);
}

/// Scroll the page so the element tagged `selector` sits in the middle of
/// the content pane, or as near to it as the page's ends allow, and return
/// where it is then and the pane's bounds.
fn centre_in_page(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    selector: &'static str,
) -> (Bounds<Pixels>, Bounds<Pixels>) {
    let site = bounds_of(cx, selector);
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            let pane = this.content_scroll.bounds();
            let offset = this.content_scroll.offset();
            let max = this.content_scroll.max_offset();
            let y = (offset.y - (site.center().y - pane.center().y)).clamp(-max.y, px(0.));
            this.content_scroll.set_offset(point(offset.x, y));
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(cx);
    let pane = read(cx, showcase, |this, _| this.content_scroll.bounds());
    (bounds_of(cx, selector), pane)
}

/// Where a nested scroller's content is scrolled to, written with `{:?}`:
/// `a_nested_scroller_keeps_the_wheel_to_itself` compares two of them for a
/// change only.
type ScrolledTo = fn(&mut VisualTestContext, &Entity<Showcase>) -> String;

/// The Diff's lines that the left line-number column holds a quarter, half
/// and three quarters of the way down the Diff, which a click there selects
/// (gpui-component `diff/mod.rs:1334-1343`, `begin_line_selection`):
/// `DiffState` has no public scroll offset, so the lines under fixed points
/// stand for it. A point over a hunk separator selects none, as the row has
/// nothing there to click (`render_hunk_separator`, `diff/mod.rs:951-956`),
/// so at least one of the three has to select a line. A file header and a
/// fold row hold a button there instead, which a click presses: the
/// header's collapse button (`:1212-1230`) or the fold's expand buttons
/// (`:1005-1026`), which would change what the Diff shows. Today's sample is
/// one file, whose header lies above the quarter point. Each click starts
/// from no selection: it is cleared before each one.
fn diff_lines_down_the_middle(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> String {
    let diff = bounds_of(cx, "typography-diff");
    let lines: Vec<_> = [0.25, 0.5, 0.75]
        .into_iter()
        .map(|down| {
            cx.update(|_window, cx| {
                let state = showcase.read(cx).diff_state.clone();
                state.update(cx, |state, cx| state.set_selected_lines(None, cx));
            });
            click_at(
                cx,
                point(diff.left() + px(10.), diff.top() + diff.size.height * down),
            );
            read(cx, showcase, |this, cx| {
                this.diff_state.read(cx).selected_lines()
            })
        })
        .collect();
    assert!(
        lines.iter().any(Option::is_some),
        "typography-diff: no click down its middle selected a line"
    );
    format!("{lines:?}")
}

/// A wheel turned inside a nested scroller scrolls that widget, not the page
/// under it, at each of the showcase's eight: the DataTable, List, Tree and
/// MessageScroller (Data page), the Scrollable Area and the Settings (Layout
/// page), the code Editor and the Diff (Typography page).
///
/// gpui's own scroll listener runs in the bubble phase and stops at no one:
/// every scroll container whose hitbox is under the pointer moves by the
/// same delta (gpui-pre `src/elements/div.rs`, `paint_scroll_listener`), and
/// the page's is one (`support::gutter_scroll`). gpui-component 0.8.1 wraps
/// each of its scrolling widgets in gpui-base's `ScrollBounce`: in the
/// capture phase the innermost area under the pointer that can scroll claims
/// the wheel, scrolls and stops its propagation (gpui-base
/// `src/scroll_bounce.rs:248-270`, `:556-617`), so the page's listener never
/// runs. gpui-base's editor claims it in the capture phase too, and scrolls
/// and stops it in the bubble phase, before the page's listener, an outer
/// element's (`src/input/base/element.rs:496-513`,
/// `src/input/base/state.rs:2842-2912`). There is no chaining: at its end the
/// inner area keeps the wheel. An inner area whose content fits lets it go
/// (`scroll_bounce.rs:570`), so a site here is one whose content overflows.
///
/// Each site is centred in a 1280 x 720 window, its page scrolled to it, and
/// turned once towards its end and once back. Neither turn may move the
/// page, and one of them has to move the site's content while the page had
/// room to move the same way, or the site proves nothing. The three sites at
/// the end of their page (the MessageScroller, the Diff, the Settings) leave
/// the page no room towards the end, so their turn back is that one.
#[gpui::test]
fn a_nested_scroller_keeps_the_wheel_to_itself(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    // windows-11's list rows (40px) are the tallest any bundled preset
    // states, so the List's six sample rows and the Tree's are certain to
    // overflow their 200px boxes; the other sites overflow under it too.
    use_preset(&mut cx, &showcase, "windows-11");

    let sites: [(Page, &'static str, ScrolledTo); 8] = [
        (Page::Data, "data-table", |cx, showcase| {
            read(cx, showcase, |this, cx| {
                let offset = this.table_state.read(cx).vertical_scroll_handle.offset();
                format!("{:?}", offset.y)
            })
        }),
        (Page::Data, LIST_DEMO, |cx, showcase| {
            read(cx, showcase, |this, cx| {
                format!("{:?}", this.list_state.read(cx).scroll_handle().offset().y)
            })
        }),
        (Page::Data, TREE_DEMO, |cx, showcase| {
            read(cx, showcase, |this, cx| {
                format!("{:?}", this.tree_state.read(cx).scroll_handle().offset().y)
            })
        }),
        (Page::Data, "data-message-scroller", |cx, showcase| {
            read(cx, showcase, |this, cx| {
                format!("{:?}", this.chat_scroller.read(cx).logical_scroll_top())
            })
        }),
        // The area's own handle is the showcase's div's, which it does not
        // keep, so its first item stands for it.
        (Page::Layout, "layout-scroll-area", |cx, _| {
            format!("{:?}", bounds_of(cx, "layout-scroll-area-item-1").top())
        }),
        // The frame itself does not scroll: the Settings fills it. What
        // scrolls is the Settings' page list
        // (gpui-component `setting/page.rs:229-257`), whose first group
        // holds the row probe.
        (Page::Layout, "settings-frame", |cx, _| {
            format!("{:?}", bounds_of(cx, PROBE_SETTINGS_ROW).top())
        }),
        (Page::Typography, "typography-editor", |cx, showcase| {
            read(cx, showcase, |this, cx| {
                format!("{:?}", this.editor_state.read(cx).scroll_offset().y)
            })
        }),
        (
            Page::Typography,
            "typography-diff",
            diff_lines_down_the_middle,
        ),
    ];

    // The same turn over the page outside every nested scroller moves the
    // page, so a page that stays put below is the scroller's doing.
    show(&mut cx, &showcase, Page::Data);
    let (heading, _pane) = centre_in_page(&mut cx, &showcase, "data-heading-list");
    let page_before = bounds_of(&mut cx, CONTENT_SCROLL).top();
    scroll_at(&mut cx, heading.center(), -px(50.));
    assert_ne!(
        bounds_of(&mut cx, CONTENT_SCROLL).top(),
        page_before,
        "a wheel over the List's heading did not move the page"
    );

    for (page, selector, scrolled_to) in sites {
        show(&mut cx, &showcase, page);
        let (site, pane) = centre_in_page(&mut cx, &showcase, selector);
        assert!(
            pane.top() <= site.top() && site.bottom() <= pane.bottom(),
            "{selector}: is not wholly in the content pane {pane:?}, it sits at {site:?}"
        );
        let mut proved = false;
        for towards_end in [true, false] {
            let (offset, max) = read(&mut cx, &showcase, |this, _| {
                (
                    this.content_scroll.offset(),
                    this.content_scroll.max_offset(),
                )
            });
            let page_had_room = if towards_end {
                offset.y > -max.y
            } else {
                offset.y < px(0.)
            };
            let page_before = bounds_of(&mut cx, CONTENT_SCROLL).top();
            let inner_before = scrolled_to(&mut cx, &showcase);
            let delta = site.size.height / 4.;
            scroll_at(
                &mut cx,
                site.center(),
                if towards_end { -delta } else { delta },
            );
            let inner_after = scrolled_to(&mut cx, &showcase);
            let page_after = bounds_of(&mut cx, CONTENT_SCROLL).top();
            assert_eq!(
                page_after, page_before,
                "{selector}: the wheel moved the page from {page_before:?} to {page_after:?} \
                 while it was over a widget that scrolls"
            );
            proved |= page_had_room && inner_after != inner_before;
        }
        assert!(
            proved,
            "{selector}: no turn of the wheel moved its content while the page had room \
             to move the same way, so this site proves nothing"
        );
    }
}

/// Table, List and Tree are framed alike, and by the list theme.
///
/// The `DataTable` draws a frame of its own from `Theme::radius` and
/// `Theme::border` (gpui-component `src/table/data_table.rs:221-225`); the
/// `List` and the `Tree` draw none — both only refine a plain `div()`
/// (`src/list/list.rs`, `RenderOnce for List<D>`; `src/tree.rs`,
/// `RenderOnce for Tree`) — so the box around them is the showcase's own
/// and has to carry the same border.
///
/// gpui's test API reports bounds, not corner radii, so the radius is
/// asserted where it is built rather than where it is painted: on the
/// refinement the showcase puts on both boxes. Two presets, because a
/// radius that came from nowhere would still match one of them. None of
/// the bundled presets states a zero radius, so the square-cornered end of
/// the range is windows-11's 4px.
#[gpui::test]
fn the_list_frames_agree_with_the_list_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for preset in ["material-3", "windows-11"] {
        use_preset(&mut cx, &showcase, preset);
        let native = read(&mut cx, &showcase, |_this, cx| {
            let b = |f: fn(&native_theme::theme::ResolvedWidgetBorder) -> f32| {
                native_value(cx, |n| px(f(&n.resolved.list.border)))
            };
            b(|b| b.corner_radius).zip(b(|b| b.line_width))
        });
        let frame = cx.update(|_w, cx| native_geometry(cx, geometry::list));
        assert!(
            native.is_some() && frame.is_some(),
            "{preset}: no native theme is installed, so nothing was measured"
        );
        let (Some((radius, width)), Some(frame)) = (native, frame) else {
            continue;
        };

        let theme = cx.update(|_w, cx| Theme::global(cx).clone());
        assert_eq!(
            theme.radius, radius,
            "{preset}: the DataTable rounds itself to {:?} and the list theme states {radius:?}",
            theme.radius
        );
        for (corner, got) in [
            ("top left", frame.corner_radii.top_left),
            ("top right", frame.corner_radii.top_right),
            ("bottom left", frame.corner_radii.bottom_left),
            ("bottom right", frame.corner_radii.bottom_right),
        ] {
            assert_eq!(
                got,
                Some(gpui::AbsoluteLength::Pixels(radius)),
                "{preset}: the List and Tree frame's {corner} corner is {got:?}, \
                     not the list theme's {radius:?}"
            );
        }
        assert_eq!(
            frame.border_widths.top,
            Some(gpui::AbsoluteLength::Pixels(width)),
            "{preset}: the frame's line width is not the list theme's {width:?}"
        );
        assert_eq!(
            frame.border_color,
            Some(theme.border),
            "{preset}: the frame's colour is not the one the DataTable's frame takes"
        );
    }
}

/// Every control the showcase advertises as interactive answers a click.
///
/// Each step drives the real widget through the test platform's mouse and
/// keyboard and then asks the model what changed, so a handler that stops
/// being wired up fails the step that names it.
#[gpui::test]
fn interactive_controls_respond(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);

    // --- Buttons page -------------------------------------------------
    show(&mut cx, &showcase, Page::Buttons);

    // Clipboard: the card is one icon button, and the test platform holds
    // a real in-memory clipboard.
    click(&mut cx, PROBE_CLIPBOARD);
    assert_eq!(
        cx.read_from_clipboard().and_then(|item| item.text()),
        Some("cargo add native-theme".to_string()),
        "Clipboard: the Copy button wrote nothing"
    );

    // --- Inputs page --------------------------------------------------
    show(&mut cx, &showcase, Page::Inputs);

    // Rating: clicking a star at or below the current value clears down to
    // the one before it (rating.rs, Rating::render on_click), so the first
    // star takes the three the showcase starts with to none.
    click(&mut cx, PROBE_RATING);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.rating_value),
        0,
        "Rating: clicking the first star left the value alone"
    );

    // --- Data page ----------------------------------------------------
    show(&mut cx, &showcase, Page::Data);

    // Pagination: the leading end of the strip is the previous-page control.
    let before = read(&mut cx, &showcase, |this, _| this.page);
    click(&mut cx, PROBE_PAGINATION);
    assert_ne!(
        read(&mut cx, &showcase, |this, _| this.page),
        before,
        "Pagination: the page did not move"
    );

    // Attachment: the whole card advances its status.
    let before = read(&mut cx, &showcase, |this, _| this.attachment_status);
    click(&mut cx, PROBE_ATTACHMENT);
    assert_ne!(
        read(&mut cx, &showcase, |this, _| this.attachment_status),
        before,
        "Attachment: the status did not advance"
    );

    // MessageScroller: Send appends to the thread the scroller renders.
    let before = read(&mut cx, &showcase, |this, _| this.chat_messages.len());
    click(&mut cx, PROBE_CHAT_SEND);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.chat_messages.len()),
        before + 1,
        "MessageScroller: Send added no message"
    );

    // --- Layout page --------------------------------------------------
    show(&mut cx, &showcase, Page::Layout);

    // Stepper: the steps run left to right, so the leading one is the first.
    click(&mut cx, PROBE_STEPPER);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.step),
        0,
        "Stepper: clicking the first step left the selection alone"
    );

    // Carousel: the last pagination dot goes to the last slide.
    click(&mut cx, PROBE_CAROUSEL_LAST);
    assert_eq!(
        read(&mut cx, &showcase, |this, cx| this
            .carousel_state
            .read(cx)
            .selected_index()),
        Some(CAROUSEL_SLIDES.len() - 1),
        "Carousel: the last pagination dot did not select the last slide"
    );

    // --- Overlays page ------------------------------------------------
    show(&mut cx, &showcase, Page::Overlays);

    // AlertDialog: opened by a click, then answered from the keyboard the
    // dialog binds — Enter confirms, Escape cancels.
    click(&mut cx, PROBE_ALERT_DIALOG);
    assert!(
        cx.debug_bounds("dialog-layer").is_some(),
        "AlertDialog: the dialog layer never reached the screen"
    );
    cx.simulate_keystrokes("enter");
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.alert_choice.clone()),
        Some("Discard".into()),
        "AlertDialog: confirming did not report a choice"
    );
    click(&mut cx, PROBE_ALERT_DIALOG);
    cx.simulate_keystrokes("escape");
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.alert_choice.clone()),
        Some("Keep".into()),
        "AlertDialog: cancelling did not report a choice"
    );

    // --- Feedback page ------------------------------------------------
    show(&mut cx, &showcase, Page::Feedback);

    // Notification: the button pushes one onto the window's notification layer.
    let before = cx.update(|window, cx| window.notifications(cx).len());
    click(&mut cx, PROBE_NOTIFICATION);
    assert_eq!(
        cx.update(|window, cx| window.notifications(cx).len()),
        before + 1,
        "Notification: nothing was pushed"
    );

    // --- The theme settings' colour-mode Select ------------------------
    //
    // Its rows are System, Light and Dark, and they are not searchable, so
    // the keyboard walks from the chosen row to Dark and back to System, and
    // Enter takes each. Dark is chosen from light, so the theme's mode always
    // has to change: on a dark desktop, System already is dark, and the
    // Theme menu's Light sets the start, which the Select has to show too.
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        AppColorMode::System,
        "the showcase no longer starts in System, so choosing Dark may prove nothing"
    );
    let start = if native_theme::detect::system_is_dark() {
        run_menu_item(&mut cx, "Theme", "Light");
        AppColorMode::Light
    } else {
        AppColorMode::System
    };
    assert!(
        !cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        "the step does not start from light, so choosing Dark may change nothing"
    );
    let shown = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            this.color_mode_select.read(cx).selected_value().cloned()
        })
    };
    assert_eq!(
        shown(&mut cx).as_deref(),
        Some(start.short_label()),
        "the colour-mode Select does not show the mode the showcase is in"
    );
    let select = bounds_of(&mut cx, PROBE_COLOR_MODE);
    assert!(
        within(select, bounds_of(&mut cx, CHROME_THEME_SETTINGS)),
        "the colour-mode Select at {select:?} is not in the theme settings"
    );
    click(&mut cx, PROBE_COLOR_MODE);
    let steps = if start == AppColorMode::Light { 1 } else { 2 };
    for _ in 0..steps {
        cx.simulate_keystrokes("down");
    }
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        AppColorMode::Dark,
        "the colour-mode Select's Dark did not reach SetColorMode"
    );
    assert!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        "the colour-mode Select's Dark did not reach Theme::mode"
    );
    click(&mut cx, PROBE_COLOR_MODE);
    cx.simulate_keystrokes("up");
    cx.simulate_keystrokes("up");
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        AppColorMode::System,
        "the colour-mode Select's System did not reach SetColorMode"
    );
    assert_eq!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        native_theme::detect::system_is_dark(),
        "the colour-mode Select's System did not install the desktop's mode"
    );
    run_menu_item(&mut cx, "Theme", "Dark");
    assert_eq!(
        shown(&mut cx).as_deref(),
        Some(AppColorMode::Dark.short_label()),
        "Theme > Dark did not show Dark in the colour-mode Select"
    );
}

/// The window asks the window manager to draw its frame (spec S8): it
/// requests server-side decorations, and leaves the system's title bar in
/// place on macOS and Windows, which hide it only for a titlebar that
/// `appears_transparent` (gpui-pre-macos window.rs, `MacWindow::open`;
/// gpui-pre-windows window.rs, `WindowsWindow::new`) -- the option
/// `TitleBar::window_options` sets for a window that draws its own
/// (title_bar.rs, `TitleBar::title_bar_options`).
#[gpui::test]
fn the_window_asks_the_window_manager_for_its_frame(_cx: &mut TestAppContext) {
    let options = crate::window_options(Bounds {
        origin: Point::default(),
        size: WINDOW_SIZE,
    });
    assert_eq!(
        options.window_decorations,
        Some(gpui::WindowDecorations::Server),
        "the window does not ask the window manager to draw its frame"
    );
    assert!(
        options
            .titlebar
            .as_ref()
            .is_some_and(|titlebar| !titlebar.appears_transparent),
        "the window hides the system's title bar on macOS and Windows"
    );
    assert!(
        !options.app_owns_titlebar_drag,
        "the window claims the title bar's drag, which only a window drawing its own TitleBar has"
    );
}

/// Opened asking for server-side decorations, the test window is granted
/// them, and the chrome is the one for a frame the window manager draws.
#[gpui::test]
fn a_window_asking_for_server_decorations_is_granted_them(cx: &mut TestAppContext) {
    frame_under_request(cx, gpui::WindowDecorations::Server);
}

/// Opened asking for client-side decorations, the test window is granted
/// server-side ones: gpui's test platform keeps the default
/// `PlatformWindow::window_decorations` (gpui-pre platform.rs), which
/// answers `Decorations::Server` whatever was requested. The client-side
/// arm is reached through `Showcase::frame_for_test` instead
/// (`under_client_decorations_the_title_bar_holds_the_menus`).
#[gpui::test]
fn a_window_asking_for_client_decorations_is_granted_server_ones_here(cx: &mut TestAppContext) {
    frame_under_request(cx, gpui::WindowDecorations::Client);
}

/// Open the showcase asking for `request`, check the test platform granted
/// server-side decorations, and that the chrome is the one for a frame the
/// window manager draws.
fn frame_under_request(cx: &mut TestAppContext, request: gpui::WindowDecorations) {
    let options = gpui::WindowOptions {
        window_decorations: Some(request),
        ..crate::window_options(Bounds {
            origin: Point::default(),
            size: WINDOW_SIZE,
        })
    };
    let (_showcase, _root, mut cx) = open_with(cx, options);
    let granted = cx.update(|window, _cx| window.window_decorations());
    assert_eq!(
        granted,
        gpui::Decorations::Server,
        "asked for {request:?}, the test platform granted {granted:?}"
    );
    server_chrome(&mut cx);
}

/// The chrome of a window whose frame the window manager draws (spec S8):
/// no TitleBar, and at the top of the window, across its whole width, the
/// menu-bar row with the menus in it, the toolbar right under it. On
/// macOS the menus are in the system's menu bar and there is no row.
fn server_chrome(cx: &mut VisualTestContext) {
    assert_eq!(
        cx.debug_bounds(CHROME_TITLE_BAR),
        None,
        "a TitleBar is drawn, and the window manager draws the window's title bar"
    );
    let toolbar = bounds_of(cx, CHROME_TOOLBAR);
    if cfg!(target_os = "macos") {
        assert_eq!(
            cx.debug_bounds(CHROME_MENU_BAR),
            None,
            "a menu-bar row is drawn, and the menus are in the system's menu bar"
        );
        assert_eq!(
            toolbar.top(),
            px(0.),
            "the toolbar starts at {:?}",
            toolbar.top()
        );
        return;
    }
    let row = bounds_of(cx, CHROME_MENU_BAR);
    assert_eq!(
        row.top(),
        px(0.),
        "the menu-bar row starts at {:?}",
        row.top()
    );
    assert_eq!(
        row.left(),
        px(0.),
        "the menu-bar row starts at {:?}",
        row.left()
    );
    assert_eq!(
        row.size.width, WINDOW_SIZE.width,
        "the menu-bar row is {:?} wide in a {:?} window",
        row.size.width, WINDOW_SIZE.width
    );
    assert_eq!(
        toolbar.top(),
        row.bottom(),
        "the toolbar is not right under the menu-bar row"
    );
    let menus = bounds_of(cx, CHROME_APP_MENU_BAR);
    assert!(
        menus.size.width > px(0.) && menus.size.height > px(0.),
        "the menu bar laid out at {:?}",
        menus.size
    );
    assert!(
        within(menus, row),
        "the menu bar at {menus:?} is not inside the menu-bar row at {row:?}"
    );
}

/// Say the window was granted `frame`, and draw the frame that follows.
fn grant(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, frame: gpui::Decorations) {
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.frame_for_test = Some(frame);
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(cx);
}

/// Client-side decorations, as the showcase is granted them where the
/// window manager leaves the frame to the application.
const CLIENT_SIDE: gpui::Decorations = gpui::Decorations::Client {
    tiling: gpui::Tiling {
        top: false,
        left: false,
        right: false,
        bottom: false,
    },
};

/// Granted client-side decorations -- where a compositor draws none, as
/// GNOME's Mutter does for a Wayland client -- the TitleBar is the window's
/// title bar (spec S8): the first thing in the window, across its whole
/// width, with the menus inside it where the platform has no menu bar
/// of its own, and no menu-bar row. The Layout page then draws no TitleBar
/// sample: the window's own title bar is one.
///
/// The test platform grants server-side decorations only, so the grant is
/// the test's (`Showcase::frame_for_test`). In a window really granted
/// client-side decorations gpui-component's `window_border` insets the content by
/// `window_paddings` and, on each side not tiled, its own border
/// (window_border.rs, `window_content_insets`, which is not public), so the
/// bar is held inside the paddings and across the content's whole width --
/// the toolbar's -- rather than to a pixel.
#[gpui::test]
fn under_client_decorations_the_title_bar_holds_the_menus(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    grant(&mut cx, &showcase, CLIENT_SIDE);
    let bar = bounds_of(&mut cx, CHROME_TITLE_BAR);
    let toolbar = bounds_of(&mut cx, CHROME_TOOLBAR);
    let (paddings, viewport) = cx.update(|window, _cx| {
        (
            gpui_component::window_paddings(window),
            window.viewport_size(),
        )
    });
    assert!(
        bar.top() >= paddings.top && bar.left() >= paddings.left,
        "the title bar starts at {:?}, outside the frame's paddings {paddings:?}",
        bar.origin
    );
    assert!(
        bar.right() <= viewport.width - paddings.right,
        "the title bar ends at {:?}, past the frame's right padding in a {:?} window",
        bar.right(),
        viewport.width
    );
    assert!(
        bar.left() == toolbar.left() && bar.right() == toolbar.right(),
        "the title bar at {bar:?} is not as wide as the content, the toolbar at {toolbar:?}"
    );
    assert!(bar.size.height > px(0.), "the title bar has no height");
    assert_eq!(
        cx.debug_bounds(CHROME_MENU_BAR),
        None,
        "a menu-bar row is drawn beside a title bar"
    );
    assert_eq!(
        toolbar.top(),
        bar.bottom(),
        "the toolbar is not right under the title bar"
    );

    if cfg!(not(target_os = "macos")) {
        let menus = bounds_of(&mut cx, CHROME_APP_MENU_BAR);
        assert!(
            menus.size.width > px(0.) && menus.size.height > px(0.),
            "the menu bar laid out at {:?}",
            menus.size
        );
        assert!(
            bar.contains(&menus.origin) && menus.bottom() <= bar.bottom(),
            "the menu bar at {menus:?} is not inside the title bar at {bar:?}"
        );
    }

    show(&mut cx, &showcase, Page::Layout);
    assert_eq!(
        cx.debug_bounds(LAYOUT_TITLE_BAR),
        None,
        "the Layout page draws a TitleBar sample under the window's own TitleBar"
    );
}

/// While the window manager draws the window's frame, the Layout page shows
/// a TitleBar sample (spec S8), which reports itself, and leaves the window
/// alone: a double click on it zooms nothing and a drag moves nothing. The
/// test window's `zoom` and `start_window_move` are `unimplemented!()`
/// (gpui-pre platform/test/window.rs), so a sample that passed either on to
/// the window would fail here.
#[gpui::test]
fn the_title_bar_sample_is_drawn_and_leaves_the_window_alone(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    let sample = bounds_of(&mut cx, LAYOUT_TITLE_BAR);
    assert!(
        sample.size.width > px(0.) && sample.size.height > px(0.),
        "the TitleBar sample laid out at {:?}",
        sample.size
    );
    let at = sample.center();
    hover(&mut cx, at);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("TitleBar"),
        "the pointer settled on the TitleBar sample, and the inspector does not show its info"
    );

    let modifiers = Modifiers::default();
    cx.simulate_click(at, modifiers);
    cx.simulate_event(gpui::MouseDownEvent {
        button: MouseButton::Left,
        position: at,
        modifiers,
        click_count: 2,
        first_mouse: false,
    });
    cx.simulate_event(gpui::MouseUpEvent {
        button: MouseButton::Left,
        position: at,
        modifiers,
        click_count: 2,
    });
    cx.run_until_parked();
    cx.simulate_mouse_down(at, MouseButton::Left, modifiers);
    cx.simulate_mouse_move(at + point(px(24.), px(0.)), MouseButton::Left, modifiers);
    cx.simulate_mouse_up(at + point(px(24.), px(0.)), MouseButton::Left, modifiers);
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        cx.debug_bounds(LAYOUT_TITLE_BAR).is_some(),
        "the TitleBar sample is gone after a click and a drag"
    );
}

/// The Theme tab's Window section names the decorations the window was
/// granted and what draws the frame under them (spec S8): the window
/// manager's frame, with none of the facts of gpui-component's client frame,
/// or that client frame.
#[test]
fn the_theme_tabs_window_section_names_the_mode() {
    let value = |rows: &[(&str, String)], what: &str| {
        rows.iter()
            .find(|(w, _)| *w == what)
            .map(|(_, v)| v.clone())
    };
    let server =
        crate::inspector::window_rows(gpui::Decorations::Server, None, gpui::Edges::all(px(0.)));
    assert!(
        value(&server, "decorations").is_some_and(|v| v.starts_with("server-side")),
        "the server-side rows do not name the mode: {server:?}"
    );
    assert!(
        value(&server, "frame")
            .is_some_and(|v| v.starts_with(crate::inspector::window_frame(std::env::consts::OS))),
        "the server-side rows do not name this platform's frame: {server:?}"
    );
    for client_only in ["frame fill", "frame colour", "frame shadow"] {
        assert_eq!(
            value(&server, client_only),
            None,
            "the server-side rows state {client_only}, a fact of the client frame"
        );
    }
    let client =
        crate::inspector::window_rows(CLIENT_SIDE, Some(px(20.)), gpui::Edges::all(px(20.)));
    assert!(
        value(&client, "decorations").is_some_and(|v| v.starts_with("client-side")),
        "the client-side rows do not name the mode: {client:?}"
    );
    assert!(
        value(&client, "frame").is_some_and(|v| v.starts_with("the client frame")),
        "the client-side rows do not say the client frame is drawn: {client:?}"
    );
    for client_only in ["frame fill", "frame colour", "frame shadow"] {
        assert!(
            value(&client, client_only).is_some(),
            "the client-side rows do not state {client_only}"
        );
    }
}

/// M9-082: under server-side decorations the Theme tab's "frame" row names
/// the platform's frame, the window manager's on Linux and the system's
/// title bar on macOS and Windows, as the iced showcase's does
/// (`window_frame` there), and its "decorations" and "resize band" rows say
/// the same of who draws and resizes it; the rows the inspector prints are
/// the running platform's.
#[test]
fn the_window_frame_row_names_the_platforms_frame() {
    use crate::inspector::{resize_band, window_decorations, window_frame};
    let linux = window_frame("linux");
    assert!(
        linux.contains("KWin") && linux.contains("Mutter"),
        "linux: {linux}"
    );
    let macos = window_frame("macos");
    let windows = window_frame("windows");
    for (os, row) in [("macos", macos), ("windows", windows)] {
        assert!(
            row.contains("title bar") && !row.contains("KWin") && !row.contains("Mutter"),
            "{os}: {row}"
        );
    }
    assert!(macos.contains("traffic-light"), "macos: {macos}");
    assert!(windows.contains("caption"), "windows: {windows}");
    let band = resize_band("linux");
    assert!(band.contains("window manager"), "linux: {band}");
    for os in ["macos", "windows"] {
        let band = resize_band(os);
        assert!(
            band.contains("system") && !band.contains("window manager"),
            "{os}: {band}"
        );
    }
    let decorations = window_decorations("linux");
    assert!(
        decorations.contains("window manager"),
        "linux: {decorations}"
    );
    for os in ["macos", "windows"] {
        let decorations = window_decorations(os);
        assert!(
            decorations.contains("system") && !decorations.contains("window manager"),
            "{os}: {decorations}"
        );
    }
    let rows =
        crate::inspector::window_rows(gpui::Decorations::Server, None, gpui::Edges::all(px(0.)));
    let value = |what: &str| {
        rows.iter()
            .find(|(w, _)| *w == what)
            .map(|(_, v)| v.clone())
    };
    let os = std::env::consts::OS;
    assert!(
        value("frame").is_some_and(|v| v.starts_with(window_frame(os))),
        "{os}: the frame row is not this platform's: {rows:?}"
    );
    assert!(
        value("decorations").is_some_and(|v| v.ends_with(&window_decorations(os))),
        "{os}: the decorations row is not this platform's: {rows:?}"
    );
    assert_eq!(
        value("resize band"),
        Some(resize_band(os)),
        "{os}: the resize band row is not this platform's: {rows:?}"
    );
}

/// The texts shown under server-side decorations outside the Theme tab's
/// Window section name who draws the frame per platform, as M9-082's rows
/// do: the window manager, and a KDE application's menu placement, on Linux
/// only; the system on macOS and Windows. They are the Layout page's
/// TitleBar sample (its heading and its info's "sample" line) and the
/// menu-bar row's info ("window"); the infos show the running platform's.
#[test]
fn the_server_side_texts_name_the_platforms_frame_drawer() {
    use crate::info::chrome::{menu_row_window, sample_window};
    use crate::pages::layout::title_bar_heading;
    let texts = |os: &str| {
        [
            sample_window(os),
            menu_row_window(os),
            title_bar_heading(os),
        ]
    };
    for text in texts("linux") {
        assert!(text.contains("window manager"), "linux: {text}");
    }
    for os in ["macos", "windows"] {
        for text in texts(os) {
            assert!(
                text.contains("system")
                    && !text.contains("window manager")
                    && !text.contains("KDE"),
                "{os}: {text}"
            );
        }
    }
    let t = Theme::from(&gpui_component::theme::ThemeColor::default());
    let instance = |info: &WidgetInfo, what: &str| {
        info.instance
            .iter()
            .find(|note| note.what == what)
            .map(|note| note.text.clone())
    };
    let os = std::env::consts::OS;
    assert_eq!(
        instance(
            &crate::info::chrome::title_bar_sample(&t, "title"),
            "sample"
        ),
        Some(sample_window(os)),
        "{os}: the TitleBar sample's info is not this platform's"
    );
    assert_eq!(
        instance(&crate::info::chrome::menu_bar(&t, None, px(8.)), "window"),
        Some(menu_row_window(os)),
        "{os}: the menu-bar row's info is not this platform's"
    );
}

/// The window is titled with this crate's name and version (spec §3.4): the
/// title bar's label, the title the OS shows and the string the Windows
/// screenshot capture finds the window by are one string. The capture is
/// Windows-only code; it looks the window up by `WINDOW_TITLE`, which is
/// checked here. The label is read off the info of the TitleBar the window
/// draws under client-side decorations (spec S8), which names the text it
/// draws.
#[gpui::test]
fn the_window_is_titled_with_the_crates_version(cx: &mut TestAppContext) {
    let expected = format!("native-theme-gpui {} showcase", env!("CARGO_PKG_VERSION"));
    assert_eq!(
        WINDOW_TITLE, expected,
        "the title the screenshot capture looks the window up by is not the crate's name and version"
    );
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    cx.update(|window, _cx| crate::name_window(window));
    assert_eq!(
        cx.window_title().as_deref(),
        Some(expected.as_str()),
        "the OS window title is not the crate's name and version"
    );
    grant(&mut cx, &showcase, CLIENT_SIDE);
    let bar = bounds_of(&mut cx, CHROME_TITLE_BAR);
    hover(&mut cx, bar.center());
    settle(&mut cx);
    draw(&mut cx);
    let label = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "label")
                .map(|n| n.text.clone())
        })
    });
    assert!(
        label
            .as_deref()
            .is_some_and(|l| l.starts_with(&format!("\"{expected}\""))),
        "the title bar's label is not the crate's name and version: {label:?}"
    );
}

/// The toolbar is the model's toolbar (spec §2.3, §9): at least
/// `toolbar.bar_height` tall where the theme states one, with
/// `toolbar.item_gap` between its items: its first two, the Command Palette
/// and Reload System Theme buttons, are measured.
///
/// kde-breeze states a 0px gap and adwaita a 6px one, so a row that kept a
/// gap of its own fails one of the two. Both state one, so the gap is
/// measured under each.
#[gpui::test]
fn the_toolbar_is_the_models_toolbar(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for preset in ["kde-breeze", "adwaita"] {
        use_preset(&mut cx, &showcase, preset);
        let model = read(&mut cx, &showcase, |_this, cx| {
            cx.native()
                .map(|n| (n.resolved.toolbar.bar_height, n.resolved.toolbar.item_gap))
        });
        assert!(
            model.is_some(),
            "{preset}: no native theme is installed, so nothing was measured"
        );
        let (bar_height, item_gap) = model.unwrap_or_default();
        let bar = bounds_of(&mut cx, CHROME_TOOLBAR);
        // A theme that states no bar height leaves the row to size to its
        // content, so there is nothing to hold it to.
        if let Some(bar_height) = bar_height {
            assert!(
                bar.size.height >= px(bar_height),
                "{preset}: the toolbar is {:?} tall, under toolbar.bar_height {bar_height}px",
                bar.size.height
            );
        }
        // Under the menu-bar row, which the window manager's frame leaves
        // at the top of the window, or at the top itself on macOS.
        let above = cx
            .debug_bounds(CHROME_MENU_BAR)
            .map_or(px(0.), |row| row.bottom());
        assert_eq!(
            bar.top(),
            above,
            "{preset}: the toolbar is not right under the menu-bar row"
        );
        // The toolbar's first two children: the Command Palette and Reload
        // System Theme buttons.
        let first = bounds_of(&mut cx, CHROME_TOOLBAR_PALETTE);
        let second = bounds_of(&mut cx, CHROME_TOOLBAR_RELOAD);
        assert!(
            within(first, bar) && within(second, bar),
            "{preset}: the Command Palette button at {first:?} or the Reload button at \
             {second:?} is not inside the toolbar at {bar:?}"
        );
        let Some(item_gap) = item_gap else {
            panic!("{preset}: states no toolbar.item_gap, so nothing was measured");
        };
        assert_eq!(
            second.left() - first.right(),
            px(item_gap),
            "{preset}: the toolbar's first two items are {:?} apart, toolbar.item_gap is {item_gap}px",
            second.left() - first.right()
        );
    }
}

/// The toolbar holds the actions and nothing else (spec §3.3): the Command
/// Palette, Reload System Theme and Preferences buttons, in that order. The
/// theme settings and the side-panel toggle are drawn elsewhere, clear of it,
/// and the Preferences button opens the Preferences sheet.
#[gpui::test]
fn the_toolbar_holds_the_actions(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let bar = bounds_of(&mut cx, CHROME_TOOLBAR);
    let buttons = [
        CHROME_TOOLBAR_PALETTE,
        CHROME_TOOLBAR_RELOAD,
        CHROME_TOOLBAR_PREFERENCES,
    ]
    .map(|selector| (selector, bounds_of(&mut cx, selector)));
    for (selector, button) in buttons {
        assert!(
            within(button, bar),
            "{selector} at {button:?} is not in the toolbar at {bar:?}"
        );
    }
    for pair in buttons.windows(2) {
        if let [(first, a), (second, b)] = pair {
            assert!(
                a.right() <= b.left(),
                "{first} at {a:?} is not before {second} at {b:?}"
            );
        }
    }
    for selector in [
        PROBE_COMBOBOX,
        PROBE_COLOR_MODE,
        PROBE_ICON_THEME,
        CHROME_SIDE_PANEL_TOGGLE,
    ] {
        let elsewhere = bounds_of(&mut cx, selector);
        assert!(
            !bar.intersects(&elsewhere),
            "{selector} at {elsewhere:?} is still in the toolbar at {bar:?}"
        );
    }
    // The row's end holds nothing, so the pointer there settles on the row.
    hover(&mut cx, point(bar.right() - px(8.), bar.center().y));
    settle(&mut cx);
    draw(&mut cx);
    let items = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "items")
                .map(|n| n.text.clone())
        })
    });
    assert_eq!(
        items.as_deref(),
        Some("icon Buttons for the command palette, a theme reload and the Preferences sheet"),
        "the toolbar's info does not name what it holds"
    );
    click(&mut cx, CHROME_TOOLBAR_PREFERENCES);
    assert!(
        cx.update(|window, cx| window.has_active_sheet(cx)),
        "the toolbar's Preferences button did not open the Preferences sheet"
    );
}

/// The toolbar row is padded where the theme states no padding for it (spec
/// §3.1). nord, a colour-scheme preset, states no `toolbar.border` side, so
/// each side is `layout.container_margin`; with that unstated too, the
/// showcase's own `TOOLBAR_PADDING`, which the row's info names. Under
/// kde-breeze and windows-11, which state their sides, the stated left side
/// stands. nord states no `toolbar.bar_height` (windows-11 does, 48px), so
/// under nord the row is as tall as its buttons and its padding, and the
/// first button's top is the top padding.
#[gpui::test]
fn the_toolbar_is_padded_where_the_theme_states_none(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let inset = |cx: &mut VisualTestContext| {
        let (bar, first) = (
            bounds_of(cx, CHROME_TOOLBAR),
            bounds_of(cx, CHROME_TOOLBAR_PALETTE),
        );
        (first.left() - bar.left(), first.top() - bar.top())
    };
    let toolbar_padding = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |_this, cx| {
            cx.native().map(|n| n.resolved.toolbar.border.padding)
        })
    };
    for preset in ["kde-breeze", "windows-11"] {
        use_preset(&mut cx, &showcase, preset);
        let left = toolbar_padding(&mut cx).and_then(|p| p.left);
        assert!(
            left.is_some(),
            "{preset} states no left toolbar padding, so its stated side is not measured"
        );
        assert_eq!(
            Some(inset(&mut cx).0),
            left.map(px),
            "{preset}: the toolbar's first button is not its stated left padding in"
        );
    }

    use_preset(&mut cx, &showcase, "nord");
    assert_eq!(
        toolbar_padding(&mut cx),
        Some(native_theme::theme::ResolvedPadding::default()),
        "nord states a toolbar padding side, so the fallback is not what is measured"
    );
    let margin = read(&mut cx, &showcase, |this, _| {
        geometry::container_margin(&this.layout)
    });
    assert!(
        margin.is_some(),
        "nord states no container_margin, so the showcase's constant is measured twice"
    );
    let (left, top) = inset(&mut cx);
    assert_eq!(
        (Some(left), Some(top)),
        (margin, margin),
        "nord: the toolbar is not padded by layout.container_margin"
    );

    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.layout = native_theme::theme::LayoutTheme::default();
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        inset(&mut cx),
        (crate::demo::TOOLBAR_PADDING, crate::demo::TOOLBAR_PADDING),
        "with neither the toolbar's padding nor container_margin stated, the toolbar is \
         not padded by the showcase's own TOOLBAR_PADDING"
    );
    let bar = bounds_of(&mut cx, CHROME_TOOLBAR);
    hover(&mut cx, point(bar.right() - px(8.), bar.center().y));
    settle(&mut cx);
    draw(&mut cx);
    let padding = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "padding")
                .map(|n| n.text.clone())
        })
    });
    assert!(
        padding
            .as_deref()
            .is_some_and(|p| p.contains("TOOLBAR_PADDING") && p.contains("the showcase's own")),
        "the toolbar's info does not say its padding is the showcase's own TOOLBAR_PADDING: \
         {padding:?}"
    );
}

/// The theme settings' Combobox is the real preset switch: choosing a preset
/// in it installs that preset.
#[gpui::test]
fn the_theme_settings_switch_the_preset(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let combobox = bounds_of(&mut cx, PROBE_COMBOBOX);
    assert!(
        within(combobox, bounds_of(&mut cx, CHROME_THEME_SETTINGS)),
        "the preset Combobox at {combobox:?} is not in the theme settings"
    );
    assert_ne!(
        read(&mut cx, &showcase, |this, _| this
            .current_theme_name
            .clone()),
        "nord",
        "the showcase starts on nord, so choosing it proves nothing"
    );
    // Typing filters the rows and puts the cursor on the first match
    // (list/list.rs, ListState::start_search), which Enter takes. Nord lands
    // in row 0, where `default` already is: the case a selection told apart
    // by filtered row indices alone misses (support.rs, PresetDelegate).
    click(&mut cx, PROBE_COMBOBOX);
    cx.simulate_input("nord");
    cx.run_until_parked();
    draw(&mut cx);
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this
            .current_theme_name
            .clone()),
        "nord",
        "choosing nord in the theme settings' Combobox did not install it"
    );
}

/// The presets the side panel's fit is checked under: every bundled preset
/// (`Theme::list_presets`), each offered on some platform -- the preset
/// switch offers those `Theme::list_presets_for_platform` gives, plus
/// `default`, the desktop's own theme, which is built on one of them and
/// which a test cannot reproduce -- with the font DPI it is resolved at.
///
/// A native preset is resolved at its own platform's DPI, as the seams test
/// does (tests/seams.rs, `NATIVE`): macOS at 72, where a point is a pixel
/// (native-theme `detect.rs`, `detect_system_font_dpi`), the others at 96.
/// A native preset is only offered on its own platform, so macos-monterey
/// resolved on a 96 DPI Linux host is not a configuration the showcase
/// shows; ios-13 is offered on macOS too (its `platforms` are `macos` and
/// `ios`), so it is resolved at 72. A colour-scheme preset, offered
/// everywhere (no `platforms`), keeps the host's DPI (`None`), as
/// `ResolutionContext::from_system` reads it.
fn side_panel_presets() -> Vec<(&'static str, Option<f32>)> {
    native_theme::theme::Theme::list_presets()
        .iter()
        .map(|info| {
            let dpi = if info.platforms.is_empty() {
                None
            } else if info.platforms.iter().any(|p| *p == "macos" || *p == "ios") {
                Some(72.0)
            } else {
                Some(96.0)
            };
            (info.key, dpi)
        })
        .collect()
}

/// Install `preset` as `use_preset` does, then re-install its theme resolved
/// at `dpi`, or at the host's DPI where it is `None`, with the text scaled by
/// `text_scale`: the point sizes it states become pixels at that DPI, as they
/// do on the platform whose DPI it is. The accessibility preferences are
/// otherwise the defaults, not the host's: `native_theme::detect::accessibility_preferences`,
/// which the showcase's own preset path reads, would scale the text by
/// whatever the desktop the test runs on is set to (native-theme-gpui
/// lib.rs, `to_theme`).
fn use_preset_scaled(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    preset: &str,
    dpi: Option<f32>,
    text_scale: f32,
) {
    use_preset(cx, showcase, preset);
    let font_dpi = dpi.unwrap_or_else(|| native_theme::ResolutionContext::from_system().font_dpi);
    cx.update(|_window, cx| {
        let is_dark = showcase.read(cx).is_dark;
        let mode = if is_dark {
            native_theme::theme::ColorMode::Dark
        } else {
            native_theme::theme::ColorMode::Light
        };
        let resolved = native_theme::theme::Theme::preset(preset)
            .expect("the preset loads")
            .into_variant(mode)
            .expect("the preset has the variant")
            .into_resolved(&native_theme::ResolutionContext {
                font_dpi,
                ..native_theme::ResolutionContext::for_tests()
            })
            .expect("the preset resolves");
        let mut prefs = native_theme::AccessibilityPreferences::default();
        prefs.text_scaling_factor = text_scale;
        native_theme_gpui::apply(
            native_theme_gpui::to_theme(&resolved, preset, is_dark, &prefs),
            &resolved,
            &prefs,
            cx,
        );
    });
    cx.run_until_parked();
    draw(cx);
}

/// The narrowest the theme settings' triggers can be: the minimum width
/// `geometry::select` and `geometry::combobox` give them, `None` where no
/// native theme is installed or they give none. A trigger takes the
/// settings' width and this minimum where that is wider, and its wrapper,
/// which the probes are on, is the settings' width either way, so a trigger
/// wider than the settings does not show in the wrapper's bounds.
fn settings_trigger_min_width(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
) -> Option<Pixels> {
    read(cx, showcase, |_this, cx| {
        [
            native_value(cx, geometry::select),
            native_value(cx, geometry::combobox),
        ]
        .into_iter()
        .flatten()
        .filter_map(|style| match style.min_size.width {
            Some(Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(w)))) => Some(w),
            _ => None,
        })
        .max()
    })
}

/// The side panel holds, top to bottom (spec S2): the theme settings --
/// Theme, Mode and Icon theme, each a label above its control, the gaps
/// `layout.widget_gap` -- then a Separator, then the inspector, its TabBar
/// first, down to the panel's bottom. The panel opens `LEFT_PANEL_WIDTH`
/// wide, and everything in it fits that width: each control is a trigger
/// that takes the settings' width and truncates its text (select.rs,
/// `SelectState::render`; combobox.rs, `Combobox`), so its text never widens
/// it, and the narrowest it can be is the minimum its builder gives it,
/// which the settings have to be at least as wide as
/// (`settings_trigger_min_width`). Checked under every native preset, at its
/// platform's DPI, and every colour-scheme preset at the host's
/// (`side_panel_presets`), whose font sizes differ -- at a text scale of 1,
/// whatever the host's, so the measurement is the same on every machine
/// (`use_preset_scaled`) -- and again at a text scale of 2, where every text
/// grows.
#[gpui::test]
fn the_side_panel_holds_the_theme_settings_and_the_inspector(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let rows = [
        (CHROME_LABEL_THEME, PROBE_COMBOBOX),
        (CHROME_LABEL_MODE, PROBE_COLOR_MODE),
        (CHROME_LABEL_ICON_THEME, PROBE_ICON_THEME),
    ];
    let presets = side_panel_presets();
    for native in [
        "kde-breeze",
        "adwaita",
        "macos-monterey",
        "macos-tahoe",
        "ios-13",
        "windows-11",
    ] {
        assert!(
            presets
                .iter()
                .any(|(key, dpi)| *key == native && dpi.is_some()),
            "{native} is not among the presets checked as a native one: {presets:?}"
        );
    }
    for text_scale in [1.0, 2.0] {
        for &(preset, dpi) in &presets {
            use_preset_scaled(&mut cx, &showcase, preset, dpi, text_scale);
            let at = format!("{preset} at text scale {text_scale}");
            let panel = bounds_of(&mut cx, CHROME_SIDE_PANEL);
            // Less the strip it keeps clear for a splitter line wider than
            // the handle's own (demo.rs, `splitter_reserve`).
            let reserve = read(&mut cx, &showcase, |_, cx| {
                crate::demo::splitter_reserve(cx)
            });
            assert_eq!(
                panel.size.width + reserve.unwrap_or_default(),
                LEFT_PANEL_WIDTH,
                "{at}: the side panel is not LEFT_PANEL_WIDTH wide, so this is not its fit there"
            );
            let settings = bounds_of(&mut cx, CHROME_THEME_SETTINGS);
            assert!(
                within(settings, panel),
                "{at}: the theme settings at {settings:?} are not in the side panel at {panel:?}"
            );
            let gap = read(&mut cx, &showcase, |this, _| {
                geometry::widget_gap(&this.layout)
            });
            assert!(gap.is_some(), "{preset} states no widget_gap");
            let mut above: Option<Bounds<Pixels>> = None;
            for (label, control) in rows {
                let (l, c) = (bounds_of(&mut cx, label), bounds_of(&mut cx, control));
                assert!(
                    within(l, settings) && within(c, settings),
                    "{at}: {label} at {l:?} or {control} at {c:?} is not in the settings at \
                     {settings:?}"
                );
                if let Some(above) = above {
                    assert_eq!(
                        Some(l.top() - above.bottom()),
                        gap,
                        "{at}: {label} is not widget_gap under the row above"
                    );
                }
                assert_eq!(
                    Some(c.top() - l.bottom()),
                    gap,
                    "{at}: {control} is not widget_gap under its label {label}"
                );
                assert_eq!(
                    c.size.width, settings.size.width,
                    "{at}: {control} does not take the settings' width"
                );
                above = Some(c);
            }
            let min = settings_trigger_min_width(&mut cx, &showcase);
            assert!(
                min.is_some_and(|min| min <= settings.size.width),
                "{at}: the settings' triggers are at least {min:?} wide, wider than the \
                 settings' {:?}",
                settings.size.width
            );

            let separator = bounds_of(&mut cx, CHROME_SIDE_PANEL_SEPARATOR);
            let tabs = bounds_of(&mut cx, INSPECTOR_TABS);
            let inspector = bounds_of(&mut cx, INSPECTOR_PANEL);
            for (part, b) in [
                ("the Separator", separator),
                ("the inspector's TabBar", tabs),
                ("the inspector", inspector),
            ] {
                assert!(
                    within(b, panel),
                    "{at}: {part} at {b:?} runs past the side panel at {panel:?}"
                );
            }
            assert!(
                above.is_some_and(|last| separator.top() >= last.bottom()),
                "{at}: the Separator at {separator:?} is not below the last row, which ends at \
                 {above:?}"
            );
            assert!(
                tabs.top() >= separator.bottom() && inspector.top() >= separator.bottom(),
                "{at}: the inspector at {inspector:?}, its TabBar at {tabs:?}, is not below the \
                 Separator at {separator:?}"
            );
            // The panel's own bottom padding where the theme states a
            // sidebar padding (adwaita), else none.
            let bottom = panel.bottom() - side_panel_padding(&mut cx, &showcase).bottom;
            assert!(
                (inspector.bottom() - bottom).abs() <= px(0.01),
                "{at}: the inspector ends at {:?}, not at the side panel's bottom less its \
                 padding, {bottom:?}",
                inspector.bottom(),
            );
        }
    }

    // With widget_gap unstated, the gaps are the showcase's own, and the
    // settings' info says so.
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.layout = native_theme::theme::LayoutTheme::default();
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    let (label, control) = (
        bounds_of(&mut cx, CHROME_LABEL_MODE),
        bounds_of(&mut cx, PROBE_COLOR_MODE),
    );
    assert_eq!(
        control.top() - label.bottom(),
        crate::demo::THEME_SETTINGS_GAP,
        "with widget_gap unstated, the settings' gap is not the showcase's own THEME_SETTINGS_GAP"
    );
    let above = bounds_of(&mut cx, PROBE_COMBOBOX);
    assert_eq!(
        label.top() - above.bottom(),
        crate::demo::THEME_SETTINGS_GAP,
        "with widget_gap unstated, the settings' rows are not THEME_SETTINGS_GAP apart"
    );
    // Right of the Mode label, which is as wide as its text: the settings'
    // own ground.
    hover(&mut cx, point(label.right() + px(8.), label.center().y));
    settle(&mut cx);
    draw(&mut cx);
    let gaps = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "gaps")
                .map(|n| n.text.clone())
        })
    });
    assert!(
        gaps.as_deref()
            .is_some_and(|g| g.contains("THEME_SETTINGS_GAP") && g.contains("the showcase's own")),
        "the settings' info does not say its gaps are the showcase's own THEME_SETTINGS_GAP: \
         {gaps:?}"
    );
}

/// The side panel and its Separator report themselves (spec S2): the
/// pointer in the middle of the panel, over the inspector's content, which
/// reports nothing by design (inspector.rs), settles on the panel's own
/// info, and on the Separator, on the Separator's.
#[gpui::test]
fn the_side_panel_and_its_separator_report_themselves(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    settle_on_each(
        &mut cx,
        &showcase,
        &[
            (CHROME_SIDE_PANEL, "Side panel"),
            (CHROME_SIDE_PANEL_SEPARATOR, "Separator · horizontal"),
        ],
    );
}

/// The theme settings' third row reads "Icon theme" (spec S6): what it
/// chooses among -- the preset's own theme, the system's, the installed
/// freedesktop themes, gpui-component's built-in icons, Lucide and Material
/// -- are icon themes. The frame's text is not readable from the test, so
/// the label's width shows it: the side panel's label is set in
/// `sidebar.font`, and `self_start` keeps it as wide as its text, and "Icon
/// theme" and "Icon set" differ in width.
#[gpui::test]
fn the_third_row_reads_icon_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let label = bounds_of(&mut cx, CHROME_LABEL_ICON_THEME);
    let size = read(&mut cx, &showcase, |_this, cx| {
        native_value(cx, |n| {
            px(native_theme_gpui::scaled_text_size(
                n.resolved.sidebar.font.size,
                n.accessibility,
            ))
        })
    });
    assert!(size.is_some(), "no native theme is installed");
    let size = size.unwrap_or_default();
    let width = |cx: &mut VisualTestContext, text: &'static str| {
        cx.update(|window, _| {
            let text = gpui::SharedString::from(text);
            let run = window.text_style().to_run(text.len());
            window
                .text_system()
                .shape_line(text, size, &[run], None)
                .width()
        })
    };
    let (theme, set) = (width(&mut cx, "Icon theme"), width(&mut cx, "Icon set"));
    assert_ne!(
        theme, set,
        "the two texts are as wide, so this proves nothing"
    );
    // Nearer the one than the other: the label box rounds the text's width
    // to the pixel grid, by how much depends on the font installed.
    assert!(
        (label.size.width - theme).abs() < (label.size.width - set).abs(),
        "the third row's label is {:?} wide, not \"Icon theme\" at {theme:?} (\"Icon set\" is \
         {set:?})",
        label.size.width
    );
}

/// The theme settings' icon-theme Select is the real icon-theme switch:
/// choosing a theme in it loads that theme. The rows are not searchable, so
/// the keyboard walks down to the last, Material, and Enter takes it.
#[gpui::test]
fn the_theme_settings_switch_the_icon_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let select = bounds_of(&mut cx, PROBE_ICON_THEME);
    assert!(
        within(select, bounds_of(&mut cx, CHROME_THEME_SETTINGS)),
        "the icon-theme Select at {select:?} is not in the theme settings"
    );
    let (count, current) = read(&mut cx, &showcase, |this, cx| {
        (
            this.icon_set_dropdown_names().len(),
            this.icon_theme_select
                .read(cx)
                .selected_index(cx)
                .map(|ix| ix.row),
        )
    });
    let last = count.saturating_sub(1);
    assert_ne!(
        current,
        Some(last),
        "the showcase starts on the last icon theme, so choosing it proves nothing"
    );
    click(&mut cx, PROBE_ICON_THEME);
    for _ in current.unwrap_or(0)..last {
        cx.simulate_keystrokes("down");
    }
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.icon_theme_name.clone()),
        "material",
        "choosing Material in the theme settings' icon-theme Select did not load it"
    );
}

/// The action a menu item named `item` in the menu named `menu` carries.
fn menu_action(menu: &str, item: &str) -> Box<dyn gpui::Action> {
    menus()
        .into_iter()
        .filter(|m| m.name.as_ref() == menu)
        .flat_map(|m| m.items)
        .find_map(|i| match i {
            gpui::MenuItem::Action { name, action, .. } if name.as_ref() == item => Some(action),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no {menu} > {item} menu item"))
}

/// The Theme menu's and the command palette's colour modes are
/// `AppColorMode::ALL`'s, in its order, each setting its mode: the menu,
/// built once for the platform's menu bar, names a mode by its
/// `short_label`; the palette, built each time it opens, by its `label`,
/// which names the mode the OS is in beside System.
#[gpui::test]
fn the_theme_menu_and_the_palette_offer_every_colour_mode(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let sets_a_mode = |action: &dyn gpui::Action| {
        AppColorMode::ALL.map(|mode| action.partial_eq(&SetColorMode(mode)))
    };
    let in_menu: Vec<(String, [bool; 3])> = menus()
        .into_iter()
        .filter(|m| m.name.as_ref() == "Theme")
        .flat_map(|m| m.items)
        .filter_map(|item| match item {
            gpui::MenuItem::Action { name, action, .. } => {
                let sets = sets_a_mode(action.as_ref());
                sets.contains(&true).then(|| (name.to_string(), sets))
            }
            _ => None,
        })
        .collect();
    let in_palette: Vec<(String, [bool; 3])> = read(&mut cx, &showcase, |this, _| {
        crate::chrome::palette_groups(this)
            .into_iter()
            .filter(|(group, _)| *group == "Colour mode")
            .flat_map(|(_, entries)| entries)
            .map(|entry| (entry.label.to_string(), sets_a_mode(entry.action.as_ref())))
            .collect()
    });
    let each = |ix: usize| std::array::from_fn::<bool, 3, _>(|at| at == ix);
    let menu_rows: Vec<(String, [bool; 3])> = AppColorMode::ALL
        .iter()
        .enumerate()
        .map(|(ix, mode)| (mode.short_label().to_string(), each(ix)))
        .collect();
    let palette_rows: Vec<(String, [bool; 3])> = AppColorMode::ALL
        .iter()
        .enumerate()
        .map(|(ix, mode)| (mode.label(), each(ix)))
        .collect();
    assert_eq!(in_menu, menu_rows, "the Theme menu's colour modes");
    assert_eq!(
        in_palette, palette_rows,
        "the command palette's colour modes"
    );
}

/// Run a menu item the way a menu does: dispatch its action into the window.
fn run_menu_item(cx: &mut VisualTestContext, menu: &str, item: &str) {
    let action = menu_action(menu, item);
    cx.update(|window, cx| window.dispatch_action(action, cx));
    cx.run_until_parked();
    draw(cx);
}

/// Run a menu item the way upstream's `AppMenuBar` menu does, which the
/// platform's own menu bar runs alike: record the focus the
/// menu interrupted when it opens, refocus it on confirm, then dispatch the
/// item's action into the window (gpui-component menu/app_menu_bar.rs,
/// AppMenuBar::set_selected_index; menu/popup_menu.rs,
/// PopupMenu::dispatch_confirm_action).
fn run_menu_item_from_focus(cx: &mut VisualTestContext, menu: &str, item: &str) {
    let action = menu_action(menu, item);
    cx.update(|window, cx| {
        if let Some(context) = window.focused(cx) {
            context.focus(window, cx);
        }
        window.dispatch_action(action, cx);
    });
    cx.run_until_parked();
    draw(cx);
}

/// A menu still acts after the widget that had the focus left the screen.
///
/// A page's widget keeps its focus handle for the showcase's lifetime, so
/// leaving its page leaves the window focused on a handle no element draws;
/// gpui then dispatches from the root (gpui-pre window.rs,
/// `focus_node_id_in_rendered_frame`), where the showcase's own action
/// handlers are out of reach.
#[gpui::test]
fn a_menu_acts_after_the_focused_widget_left_the_page(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    show(&mut cx, &showcase, Page::Inputs);
    cx.update(|window, cx| {
        let input = showcase.read(cx).input_state.clone();
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();
    draw(&mut cx);

    run_menu_item_from_focus(&mut cx, "View", "Buttons");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Buttons,
        "View > Buttons did not leave the Inputs page"
    );
    run_menu_item_from_focus(&mut cx, "View", "Inputs");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Inputs,
        "View > Inputs did nothing once the focused input had left the screen"
    );
}

/// Each item of `menu` as a line: an action's row by its name and its
/// action's, a separator as `-`, an OS submenu by its name.
fn menu_lines(menu: &gpui::Menu) -> Vec<String> {
    menu.items
        .iter()
        .map(|item| match item {
            gpui::MenuItem::Separator => "-".to_string(),
            gpui::MenuItem::Submenu(menu) => format!("{} (submenu)", menu.name),
            gpui::MenuItem::SystemMenu(menu) => format!("{} (the OS's)", menu.name),
            gpui::MenuItem::Action { name, action, .. } => format!("{name}: {}", action.name()),
        })
        .collect()
}

/// M9-117: the macOS menu bar is Apple's HIG list (*The menu bar*), under
/// the titles the shared crate gives the three showcases: the app menu,
/// holding About, Settings…, the OS's Services, Hide, Hide Others, Show All
/// and Quit, with the HIG's separators; File, holding Close; View, holding
/// the in-window View menu's rows; Theme, holding the in-window Theme
/// menu's rows less Preferences…, which is the app menu's Settings…;
/// Window, holding Minimize and Zoom (gpui offers no Bring All to Front);
/// and no Help menu. The list is built on every host; gpui hands it to the
/// menu bar only on macOS.
#[test]
fn the_macos_menus_are_the_hig_list() {
    use crate::chrome::{APP_NAME, macos_menus};
    use native_theme_showcase_support::menu::macos_menu_titles;

    let built = macos_menus();
    let titles: Vec<String> = built.iter().map(|m| m.name.to_string()).collect();
    assert_eq!(titles, macos_menu_titles(APP_NAME), "the menu bar's titles");
    let in_window = |title: &str| -> Vec<String> {
        menus()
            .iter()
            .filter(|m| m.name.as_ref() == title)
            .flat_map(menu_lines)
            .collect()
    };
    let lines: Vec<Vec<String>> = built.iter().map(menu_lines).collect();
    assert_eq!(
        lines[0],
        [
            format!("About {APP_NAME}: showcase::OpenAbout"),
            "-".to_string(),
            "Settings…: showcase::OpenPreferences".to_string(),
            "-".to_string(),
            "Services (the OS's)".to_string(),
            "-".to_string(),
            format!("Hide {APP_NAME}: showcase::Hide"),
            "Hide Others: showcase::HideOthers".to_string(),
            "Show All: showcase::ShowAll".to_string(),
            "-".to_string(),
            format!("Quit {APP_NAME}: showcase::Quit"),
        ],
        "the app menu"
    );
    assert_eq!(lines[1], ["Close: showcase::CloseWindow"], "File");
    assert_eq!(lines[2], in_window("View"), "View");
    let theme = in_window("Theme");
    assert_eq!(
        theme[theme.len().saturating_sub(2)..],
        ["-", "Preferences…: showcase::OpenPreferences"],
        "the in-window Theme menu no longer ends with Preferences…"
    );
    assert_eq!(lines[3], theme[..theme.len() - 2], "Theme");
    assert_eq!(
        lines[4],
        ["Minimize: showcase::Minimize", "Zoom: showcase::Zoom"],
        "Window"
    );
}

/// The keys the macOS menus' rows show are the HIG's: ⌘H Hide, ⌥⌘H Hide
/// Others, ⌘M Minimize and ⌘W Close, beside the bindings every platform
/// takes (⌘Q Quit, ⌘, Settings…).
#[gpui::test]
fn the_macos_menus_keys_are_the_higs(cx: &mut TestAppContext) {
    use crate::app::{CloseWindow, Hide, HideOthers, Minimize};

    let (bindings, os) = cx.update(|cx| (crate::app::macos_bindings(cx), cx.platform_info().os));
    let bound = |keys: &str, action: &dyn gpui::Action| {
        let typed = [gpui::Keystroke::parse_with_os(keys, os).expect("a keystroke")];
        bindings.iter().any(|binding| {
            binding.match_keystrokes(&typed) == Some(false) && binding.action().partial_eq(action)
        })
    };
    assert!(bound("cmd-h", &Hide), "⌘H does not hide");
    assert!(
        bound("alt-cmd-h", &HideOthers),
        "⌥⌘H does not hide the others"
    );
    assert!(bound("cmd-m", &Minimize), "⌘M does not minimize");
    assert!(bound("cmd-w", &CloseWindow), "⌘W does not close");
    assert_eq!(bindings.len(), 4, "a binding the HIG list does not name");
}

/// The showcase's menus answer the keyboard as upstream's `AppMenuBar` and
/// `PopupMenu` do (demo.rs, `MenuBar`): with a menu open, Left and Right
/// move to the neighbouring menu, wrapping; Down and Up move between its
/// items over the separators; Enter runs the highlighted one and closes the
/// menu; Escape closes it and hands the focus back.
#[gpui::test]
fn the_menus_answer_the_keyboard(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let open_menu = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| this.menus.read(cx).open_menu())
    };
    let names: Vec<String> = menus().into_iter().map(|m| m.name.to_string()).collect();
    let at = |name: &str| names.iter().position(|n| n == name);

    click(&mut cx, "menu-title-View");
    assert_eq!(open_menu(&mut cx), at("View"), "a click did not open View");
    cx.simulate_keystrokes("right");
    draw(&mut cx);
    assert_eq!(
        open_menu(&mut cx),
        at("Theme"),
        "Right did not move to Theme"
    );
    cx.simulate_keystrokes("left left left");
    draw(&mut cx);
    assert_eq!(
        open_menu(&mut cx),
        at("Help"),
        "Left did not wrap from File to Help"
    );
    cx.simulate_keystrokes("escape");
    draw(&mut cx);
    assert_eq!(open_menu(&mut cx), None, "Escape did not close the menu");

    // View's items: Basic, Buttons, ... -- Down twice highlights the second.
    click(&mut cx, "menu-title-View");
    cx.simulate_keystrokes("down down");
    draw(&mut cx);
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(open_menu(&mut cx), None, "Enter did not close the menu");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::ALL[1],
        "View > Down, Down, Enter did not run the second item"
    );

    // Up from nothing highlighted wraps to the last item, over the
    // separator before it: View's last is Command Palette.
    click(&mut cx, "menu-title-View");
    cx.simulate_keystrokes("up");
    draw(&mut cx);
    let lit = bounds_of(&mut cx, "menu-row-Command Palette");
    assert!(
        lit.size.width > px(0.),
        "View's last item is not drawn while highlighted"
    );
    cx.simulate_keystrokes("escape");
}

/// The menus act (spec §2.2): a View menu page item shows that page, a Theme
/// menu colour mode installs it, and each shortcut of the table is bound to
/// its action.
#[gpui::test]
fn the_menus_run_actions(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);

    assert_ne!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Feedback,
        "the showcase starts on Feedback, so showing it proves nothing"
    );
    assert!(
        menu_action("View", "Feedback").partial_eq(&ShowPage(4)),
        "View > Feedback does not carry ShowPage(4)"
    );
    run_menu_item(&mut cx, "View", "Feedback");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Feedback,
        "View > Feedback did not show the Feedback page"
    );

    // Whichever mode the host is in, ask for the other one.
    let was_dark = cx.update(|_w, cx| Theme::global(cx).mode.is_dark());
    let (item, wanted) = match was_dark {
        true => ("Light", AppColorMode::Light),
        false => ("Dark", AppColorMode::Dark),
    };
    assert!(
        menu_action("Theme", item).partial_eq(&SetColorMode(wanted)),
        "Theme > {item} does not carry SetColorMode({wanted:?})"
    );
    run_menu_item(&mut cx, "Theme", item);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        wanted,
        "Theme > {item} did not set the colour mode"
    );
    assert_eq!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        !was_dark,
        "Theme > {item} did not reach Theme::mode"
    );

    let os = cx.update(|_window, cx| cx.platform_info().os);
    assert_the_bindings_take(&mut cx, os);
}

/// Under gpui's macOS identity, `app::init`'s four bindings take Cmd, as
/// they do on a Mac: `KeyBinding::load` resolves `secondary` with gpui's
/// runtime OS, which the test platform reports as set here.
///
/// It does not cover what is drawn: `Kbd::format` draws by the compile
/// target, not by gpui's OS (gpui-component kbd.rs:105-223), and the macOS
/// menu bar's own bindings are bound only under `cfg!(target_os = "macos")`
/// (`app::init`, `the_macos_menus_keys_are_the_higs`).
#[gpui::test]
fn the_bindings_take_cmd_under_macos(cx: &mut TestAppContext) {
    cx.set_platform_info(gpui::PlatformInfo {
        os: gpui::OperatingSystem::Mac,
        target: gpui::PlatformTarget::Native,
    });
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(
        secondary("q", gpui::OperatingSystem::Mac),
        "cmd-q",
        "secondary does not resolve to Cmd under macOS"
    );
    assert_the_bindings_take(&mut cx, gpui::OperatingSystem::Mac);
}

/// `app::init`'s four bindings, each the highest-precedence binding of its
/// action, on the `secondary` modifier of `os`.
fn assert_the_bindings_take(cx: &mut VisualTestContext, os: gpui::OperatingSystem) {
    let bound: [(String, &dyn gpui::Action); 4] = [
        (secondary("q", os), &Quit),
        (secondary("b", os), &ToggleSidePanel),
        (secondary("k", os), &OpenCommandPalette),
        (secondary(",", os), &OpenPreferences),
    ];
    for (keys, action) in bound {
        let got = cx.update(|window, _cx| {
            window
                .highest_precedence_binding_for_action(action)
                .map(|b| {
                    b.keystrokes()
                        .iter()
                        .map(|k| k.unparse(os))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
        });
        assert_eq!(
            got.as_deref(),
            Some(keys.as_str()),
            "{} is not bound to {keys}",
            action.name()
        );
    }
}

/// `key` with gpui's `secondary` modifier under `os`, as a binding of it
/// unparses: `ctrl-` off macOS, `cmd-` on it (gpui-pre keystroke.rs:158-168).
/// A key gpui cannot parse fails the test.
fn secondary(key: &str, os: gpui::OperatingSystem) -> String {
    let keys = format!("secondary-{key}");
    let parsed = gpui::Keystroke::parse_with_os(&keys, os);
    assert!(parsed.is_ok(), "{keys:?} does not parse: {parsed:?}");
    parsed.map(|k| k.unparse(os)).unwrap_or_default()
}

/// Every item of `items` and of the submenus among them, depth first.
fn all_menu_items(items: Vec<gpui::MenuItem>) -> Vec<gpui::MenuItem> {
    items
        .into_iter()
        .flat_map(|item| match item {
            gpui::MenuItem::Submenu(menu) => all_menu_items(menu.items),
            item => vec![item],
        })
        .collect()
}

/// The inspector has no panel of its own to hide (spec S4): it hides with
/// the side panel it is in. So no `ToggleInspector` action is registered, no
/// menu item names the inspector, in a menu or a submenu of one, and Ctrl+I,
/// which ran it -- or the secondary modifier's I, as the bindings are now
/// written -- is bound to nothing.
#[gpui::test]
fn no_inspector_toggle_remains(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let (names, ctrl_i) = cx.update(|_window, cx| {
        let os = cx.platform_info().os;
        let secondary_i = secondary("i", os);
        let names: Vec<&'static str> = cx.all_action_names().to_vec();
        let ctrl_i: Vec<&'static str> = cx
            .key_bindings()
            .borrow()
            .bindings()
            .filter(|b| {
                b.keystrokes()
                    .iter()
                    .map(|k| k.unparse(os))
                    .collect::<Vec<_>>()
                    .join(" ")
                    == secondary_i
            })
            .map(|b| b.action().name())
            .collect();
        (names, ctrl_i)
    });
    assert!(
        names.contains(&"showcase::ToggleSidePanel"),
        "the showcase's actions are not among the registered ones, so their absence proves \
         nothing: {names:?}"
    );
    assert!(
        !names.contains(&"showcase::ToggleInspector"),
        "ToggleInspector is still a registered action"
    );
    assert_eq!(ctrl_i, Vec::<&str>::new(), "Ctrl+I is still bound");
    let items = all_menu_items(menus().into_iter().flat_map(|m| m.items).collect());
    assert!(
        items.len() > menus().len(),
        "the walk found {} items in {} menus, so it walked nothing",
        items.len(),
        menus().len()
    );
    let inspector_items: Vec<String> = items
        .into_iter()
        .filter_map(|i| match i {
            gpui::MenuItem::Action { name, .. } if name.contains("Inspector") => {
                Some(name.to_string())
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        inspector_items,
        Vec::<String>::new(),
        "a menu item still toggles the inspector"
    );
}

/// Whether the menu item named `item` in the menu named `menu` is disabled.
fn menu_item_disabled(menu: &str, item: &str) -> bool {
    menus()
        .into_iter()
        .filter(|m| m.name.as_ref() == menu)
        .flat_map(|m| m.items)
        .find_map(|i| match i {
            gpui::MenuItem::Action { name, disabled, .. } if name.as_ref() == item => {
                Some(disabled)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no {menu} > {item} menu item"))
}

/// Dragging the handle between the side panel and the content moves their
/// boundary by the distance dragged (spec S1).
///
/// The handle is the content panel's, laid over its left edge (gpui-base
/// resizable/panel.rs, `ResizablePanel::render`), and a drag puts the side
/// panel's right edge where the pointer is (`ResizePanelGroupElement::paint`),
/// so pressing on the boundary itself makes the distance dragged the distance
/// moved.
#[gpui::test]
fn dragging_the_handle_resizes_both_panels(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    let panel = bounds_of(&mut cx, CHROME_SIDE_PANEL);
    let line = splitter_line(&mut cx);
    assert_eq!(
        panel.right() + line,
        content.left(),
        "the side panel ends at {:?} and the content starts at {:?}, not past the {line:?} line",
        panel.right(),
        content.left()
    );
    let dragged = px(40.);
    drag_handle(&mut cx, boundary(content), content.center().y, dragged);
    let content_after = bounds_of(&mut cx, CONTENT_PANEL);
    let panel_after = bounds_of(&mut cx, CHROME_SIDE_PANEL);
    let grew = panel_after.size.width - panel.size.width;
    let shrank = content.size.width - content_after.size.width;
    assert!(
        (grew - dragged).abs() <= px(1.),
        "dragging the handle {dragged:?} right widened the side panel by {grew:?}"
    );
    assert!(
        (shrank - dragged).abs() <= px(1.),
        "dragging the handle {dragged:?} right narrowed the content by {shrank:?}"
    );
}

/// A captured window opens at the size a normal run does, under an app id no
/// desktop stored a geometry for.
#[test]
fn a_capture_opens_at_the_default_size() {
    let bounds = Bounds {
        origin: gpui::point(px(0.), px(0.)),
        size: WINDOW_SIZE,
    };
    let normal = crate::window_options(bounds);
    let capture = crate::capture_window_options(bounds);
    assert_eq!(normal.app_id, None);
    assert_eq!(
        capture.app_id,
        Some(native_theme_showcase_support::capture::app_id(
            native_theme_showcase_support::Kind::Gpui
        ))
    );
    assert_eq!(
        format!("{:?}", capture.window_bounds),
        format!("{:?}", normal.window_bounds)
    );
}

/// The window is two panels wide (spec S1): the side panel, which opens at
/// `LEFT_PANEL_WIDTH`, and the content panel, which opens at the rest of the
/// window's width.
#[gpui::test]
fn the_window_fits_the_side_panel_and_a_page(cx: &mut TestAppContext) {
    assert_eq!(
        WINDOW_SIZE.width,
        LEFT_PANEL_WIDTH + px(PAGE_WIDTH_PX),
        "WINDOW_SIZE is not LEFT_PANEL_WIDTH + PAGE_WIDTH_PX"
    );
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(
        bounds_of(&mut cx, CHROME_SIDE_PANEL).size.width,
        LEFT_PANEL_WIDTH,
        "the side panel does not open at LEFT_PANEL_WIDTH"
    );
    let line = splitter_line(&mut cx);
    assert_eq!(
        bounds_of(&mut cx, CONTENT_PANEL).size.width,
        px(PAGE_WIDTH_PX) - line,
        "the content panel does not open at the pages' width less the splitter's line"
    );
}

/// The body is two panels, the side panel and the content, and one handle
/// between them (spec S1): the inspector sits in the side panel, not in a
/// panel of its own, so its right edge is the side panel's, and the content
/// panel runs to the window's right edge.
#[gpui::test]
fn the_body_is_two_panels(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let viewport = cx.update(|window, _| window.viewport_size());
    let panel = bounds_of(&mut cx, CHROME_SIDE_PANEL);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    let inspector = bounds_of(&mut cx, INSPECTOR_PANEL);
    assert_eq!(
        panel.left(),
        px(0.),
        "the side panel is not the body's first panel"
    );
    let line = splitter_line(&mut cx);
    assert_eq!(
        content.left(),
        panel.right() + line,
        "the content panel does not follow the side panel past the splitter's line"
    );
    assert_eq!(
        content.right(),
        viewport.width,
        "the content panel does not run to the window's right edge: a panel follows it"
    );
    assert!(
        within(inspector, panel),
        "the inspector at {inspector:?} is not in the side panel at {panel:?}"
    );
    assert!(
        cx.debug_bounds(CHROME_HANDLE).is_some(),
        "the handle between the two panels was not laid out"
    );
}

/// How far past the side panel's edge the content starts: the splitter's
/// line, which the handle draws back over the strip the side panel keeps for
/// it (`demo::splitter_reserve`), and its own pixel (`demo::HANDLE_SIZE`).
fn splitter_line(cx: &mut VisualTestContext) -> Pixels {
    cx.update(|_, cx| {
        crate::demo::splitter_reserve(cx).unwrap_or_default() + crate::demo::HANDLE_SIZE
    })
}

/// The boundary a drag of the handle starts at: the handle's pixel, just
/// before the content (app.rs, the content panel's `pl`).
fn boundary(content: Bounds<Pixels>) -> Pixels {
    content.left() - crate::demo::HANDLE_SIZE
}

/// Drag the handle whose line is at `x` by `by` along the row: the first move
/// starts the drag, the second is the one that lands.
fn drag_handle(cx: &mut VisualTestContext, x: Pixels, y: Pixels, by: Pixels) {
    let start = if by < px(0.) { px(-10.) } else { px(10.) };
    cx.simulate_mouse_down(point(x, y), MouseButton::Left, Modifiers::default());
    cx.simulate_mouse_move(
        point(x + start, y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(x + by, y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_up(point(x + by, y), MouseButton::Left, Modifiers::default());
    cx.run_until_parked();
    draw(cx);
}

/// The width the side panel was dragged to survives hiding it and showing
/// it again (spec S4).
#[gpui::test]
fn the_dragged_width_survives_the_toggle(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    let width = bounds_of(&mut cx, CHROME_SIDE_PANEL).size.width;
    drag_handle(&mut cx, boundary(content), content.center().y, px(30.));
    let dragged = bounds_of(&mut cx, CHROME_SIDE_PANEL).size.width;
    assert!(
        (dragged - width - px(30.)).abs() <= px(1.),
        "dragging the side panel's handle 30px right made it {dragged:?} from {width:?}"
    );
    run_menu_item(&mut cx, "View", "Toggle Side Panel");
    assert_eq!(
        cx.debug_bounds(CHROME_SIDE_PANEL),
        None,
        "View > Toggle Side Panel did not hide the side panel"
    );
    run_menu_item(&mut cx, "View", "Toggle Side Panel");
    assert_eq!(
        bounds_of(&mut cx, CHROME_SIDE_PANEL).size.width,
        dragged,
        "the side panel came back at another width than it was dragged to"
    );
    let line = splitter_line(&mut cx);
    assert_eq!(
        bounds_of(&mut cx, CONTENT_PANEL).left(),
        dragged + line,
        "the content does not start past the splitter's line where the side panel came back to"
    );
}

/// The side panel keeps its width when the window resizes, as the iced and
/// egui showcases' do: the width it opened at, the width it was dragged
/// to, and the width it comes back at once hidden and shown again, which a
/// fresh state lays out (app.rs, `Showcase::rearrange`).
///
/// A group rescales its panels in proportion when its container resizes,
/// once every panel has a measured size (gpui-base resizable/mod.rs,
/// `ResizableState::adjust_to_container_size`); a fixed panel keeps its own
/// (resizable/panel.rs, `ResizablePanel::fixed`). Every width the window
/// takes here fits the side panel and a page.
#[gpui::test]
fn the_side_panel_keeps_its_width_when_the_window_resizes(cx: &mut TestAppContext) {
    let opened = size(WINDOW_SIZE.width + px(160.), WINDOW_SIZE.height);
    let wider = size(opened.width + px(160.), opened.height);
    let (_showcase, _root, mut cx) = open(cx, opened);
    let resize_keeps = |cx: &mut VisualTestContext, case: &str| {
        let width = bounds_of(cx, CHROME_SIDE_PANEL).size.width;
        for window in [wider, WINDOW_SIZE] {
            cx.simulate_resize(window);
            // The group's rescale lands in a frame of its own (gpui-base
            // resizable/panel.rs, the group's `on_prepaint`).
            cx.run_until_parked();
            draw(cx);
            cx.run_until_parked();
            draw(cx);
            assert_eq!(
                bounds_of(cx, CHROME_SIDE_PANEL).size.width,
                width,
                "{case}: the side panel did not keep its width in a {:?} wide window",
                window.width
            );
        }
        cx.simulate_resize(opened);
        cx.run_until_parked();
        draw(cx);
        cx.run_until_parked();
        draw(cx);
        width
    };
    assert_eq!(
        resize_keeps(&mut cx, "as opened"),
        LEFT_PANEL_WIDTH,
        "the side panel did not open at LEFT_PANEL_WIDTH"
    );
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    drag_handle(&mut cx, boundary(content), content.center().y, px(30.));
    let dragged = resize_keeps(&mut cx, "dragged");
    assert!(
        (dragged - LEFT_PANEL_WIDTH - px(30.)).abs() <= px(1.),
        "dragging the side panel's handle 30px right made it {dragged:?}"
    );
    run_menu_item(&mut cx, "View", "Toggle Side Panel");
    assert_eq!(
        cx.debug_bounds(CHROME_SIDE_PANEL),
        None,
        "View > Toggle Side Panel did not hide the side panel"
    );
    run_menu_item(&mut cx, "View", "Toggle Side Panel");
    assert_eq!(
        resize_keeps(&mut cx, "hidden and shown again"),
        dragged,
        "the side panel came back at another width than it was dragged to"
    );
}

/// Hiding the side panel in the middle of a drag of its handle, then moving
/// the pointer, leaves nothing holding the dragged handle's index.
///
/// The drag records which handle it is on in the group's state
/// (gpui-base resizable/panel.rs, `ResizablePanel::render`), and the group
/// looks that panel up on every move (`ResizePanelGroupElement::paint`); a
/// state emptied under a live drag would hand it an index it no longer has.
#[gpui::test]
fn toggling_the_side_panel_mid_drag_is_safe(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    let (x, y) = (content.left(), content.center().y);
    cx.simulate_mouse_down(point(x, y), MouseButton::Left, Modifiers::default());
    cx.simulate_mouse_move(
        point(x + px(10.), y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(x + px(20.), y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_keystrokes("secondary-b");
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        cx.debug_bounds(CHROME_SIDE_PANEL),
        None,
        "Ctrl+B did not hide the side panel"
    );
    cx.simulate_mouse_move(
        point(x + px(40.), y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(x + px(40.), y),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        bounds_of(&mut cx, CONTENT_PANEL).size.width > px(0.),
        "the content panel was not laid out after the drag"
    );
    cx.simulate_keystrokes("secondary-b");
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        cx.debug_bounds(CHROME_SIDE_PANEL).is_some(),
        "Ctrl+B did not show the side panel again after the drag"
    );
}

/// The splitter's line takes a margin of neither panel: where the theme
/// states it wider than the handle's own 1px line (material-3, 4px), the side
/// panel keeps that strip clear, so its content ends where the line starts,
/// and the line reaches no further into the content panel than the handle's
/// own line does.
#[gpui::test]
fn the_splitter_line_keeps_off_both_panels(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let slack = device_pixel(&mut cx);
    for preset in ["kde-breeze", "material-3"] {
        use_preset(&mut cx, &showcase, preset);
        let line = bounds_of(&mut cx, CHROME_SPLITTER_LINE);
        let side = bounds_of(&mut cx, CHROME_SIDE_PANEL);
        let content = bounds_of(&mut cx, CONTENT_PANEL);
        assert!(
            side.right() <= line.left() + slack,
            "{preset}: the side panel at {side:?} runs under the splitter line at {line:?}"
        );
        assert!(
            line.right() <= content.left() + px(1.) + slack,
            "{preset}: the splitter line at {line:?} reaches into the content panel at \
             {content:?} past the handle's own 1px"
        );
    }
}

/// The resizable group's handle reports itself (spec §4.3.5), over its
/// whole hit area, not only its 1px line.
#[gpui::test]
fn the_resize_handle_reports_itself(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let boundary = boundary(bounds_of(&mut cx, CONTENT_PANEL));
    let title = "ResizeHandle · side panel | content";
    let target = bounds_of(&mut cx, CHROME_HANDLE);
    // The handle's hit area: 4px either side of the boundary, the line
    // taking the first pixel right of it (demo.rs, resize_handles).
    assert_eq!(
        (target.left(), target.right()),
        (boundary - px(4.), boundary + px(4.)),
        "the target at {target:?} is not the hit area around {boundary:?}"
    );
    // Just past the hit area, where no handle takes a press.
    let y = target.center().y;
    hover(&mut cx, point(boundary + px(4.5), y));
    settle(&mut cx);
    assert_ne!(
        read(&mut cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().map(|info| info.title())
        })
        .as_deref(),
        Some(title),
        "a hover past the handle's hit area showed its info"
    );
    // Off the line, in the hit area beside it.
    hover(&mut cx, point(boundary + px(3.5), y));
    settle(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().map(|info| info.title())
        })
        .as_deref(),
        Some(title),
        "hovering the handle's hit area did not show its info"
    );
}

/// The title the page TabBar's info shows.
#[cfg(feature = "widgets")]
const PAGE_TABS_TITLE: &str = "TabBar · widgets::TabBar, menu";
#[cfg(not(feature = "widgets"))]
const PAGE_TABS_TITLE: &str = "TabBar · Underline, small, menu";

/// The title Widget Info shows for the page TabBar, an element of
/// docs/showcase-elements.toml: its name and state there.
const PAGE_TABS_LISTED_TITLE: &str = "Tab bar · Normal";

/// The title the inspector's TabBar's info shows.
#[cfg(feature = "widgets")]
const INSPECTOR_TABS_TITLE: &str = "TabBar · widgets::TabBar";
#[cfg(not(feature = "widgets"))]
const INSPECTOR_TABS_TITLE: &str = "TabBar · Underline, small";

/// The page TabBar sits at the top of the content panel, above the page's
/// scroll area, across the panel (spec S3); it reports itself, and clicking a
/// tab shows its page.
#[gpui::test]
fn the_page_tabs_navigate(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    let tabs = bounds_of(&mut cx, CHROME_PAGE_TABS);
    let scroll = bounds_of(&mut cx, CONTENT_SCROLL);
    assert!(
        within(tabs, content),
        "the page TabBar at {tabs:?} is not in the content panel at {content:?}"
    );
    assert_eq!(
        (tabs.top(), tabs.size.width),
        (content.top(), content.size.width),
        "the page TabBar is not across the top of the content panel"
    );
    assert!(
        tabs.bottom() <= scroll.top(),
        "the page TabBar at {tabs:?} is not above the page's scroll area at {scroll:?}"
    );
    let info = settle_on(&mut cx, &showcase, Page::Charts.tab());
    assert_eq!(
        info.as_ref().map(|info| info.title()).as_deref(),
        Some(PAGE_TABS_TITLE),
        "the pointer settled on a page tab, and the inspector does not show the page TabBar"
    );
    for page in [Page::Charts, Page::Buttons] {
        if read(&mut cx, &showcase, |this, _| this.active_page) == page {
            continue;
        }
        let tab = bounds_of(&mut cx, page.tab());
        assert!(
            within(tab, tabs),
            "the {page:?} tab at {tab:?} is not in the page TabBar at {tabs:?}"
        );
        click(&mut cx, page.tab());
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.active_page),
            page,
            "clicking the {page:?} tab did not show the {page:?} page"
        );
        let root = bounds_of(&mut cx, PAGE_ROOT);
        assert!(
            root.size.width > px(0.) && root.size.height > px(0.),
            "the {page:?} page laid out at {:?}",
            root.size
        );
    }
}

/// The page row's menu icon, the chosen set's ChevronDown, is
/// `geometry::icon_size_small` as every other small icon is (M9-015), and
/// the row's info names that builder while the icon shows.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_page_tabs_menu_icon_is_icon_size_small(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Material (bundled)", window, cx);
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    let (small, icon) = read(&mut cx, &showcase, |this, cx| {
        (
            native_value(cx, geometry::icon_size_small),
            this.info_ui
                .read(cx)
                .layout_drawn()
                .get("chrome.page_tabs.menu.icon")
                .copied(),
        )
    });
    let small = match small {
        Some(gpui_component::Size::Size(small)) => Some(small),
        _ => None,
    };
    assert!(
        small.is_some(),
        "geometry::icon_size_small gives no size under kde-breeze"
    );
    assert!(
        icon.is_some(),
        "Material's ChevronDown is not drawn as the page row's menu icon"
    );
    let (Some(small), Some(icon)) = (small, icon) else {
        return;
    };
    assert_eq!(
        (icon.size.width, icon.size.height),
        (small, small),
        "the page row's menu icon is laid out at {:?}, not geometry::icon_size_small's {small:?}",
        icon.size
    );
    let info = settle_on(&mut cx, &showcase, Page::Charts.tab());
    assert!(
        info.as_ref().is_some_and(|i| i
            .config
            .iter()
            .any(|n| n.what == "geometry" && n.text.starts_with("geometry::icon_size_small:"))),
        "the page row's info does not name geometry::icon_size_small while its menu icon shows: {info:?}"
    );
}

/// The Basic page's theme-drawn controls paint the theme's colours
/// (ISSUES D1-D4): each widest painted box is the leaf that fills it -- the
/// checkbox indicator's fill, the switch track, the slider rail, the
/// progress track.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_basic_controls_paint_the_themes_colours(cx: &mut TestAppContext) {
    use native_theme::color::Rgba;
    use native_theme_gpui::ResolvedTheme;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for preset in ["kde-breeze", "material-3"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Basic);
        let expect = |cx: &mut VisualTestContext, pick: fn(&ResolvedTheme) -> Rgba| {
            read(cx, &showcase, |_, cx| {
                native_color(cx, |n| pick(n.resolved))
            })
        };
        for (control, leaf, pick) in [
            (
                "basic-checkbox-checked",
                "checkbox.checked_background",
                (|r| r.checkbox.checked_background) as fn(&ResolvedTheme) -> Rgba,
            ),
            (
                "basic-checkbox-unchecked",
                "checkbox.unchecked_background",
                |r| {
                    r.checkbox
                        .unchecked_background
                        .unwrap_or(r.checkbox.background_color)
                },
            ),
            ("basic-switch-on", "switch.checked_background", |r| {
                r.switch.checked_background
            }),
            ("basic-switch-off", "switch.unchecked_background", |r| {
                r.switch.unchecked_background
            }),
            ("basic-slider", "slider.track_color", |r| {
                r.slider.track_color
            }),
            ("basic-progress", "progress_bar.track_color", |r| {
                r.progress_bar.track_color
            }),
        ] {
            // A transparent leaf paints no box (material-3's unchecked box has
            // no container, docs/platform-facts.md §2.5).
            let want = expect(&mut cx, pick).filter(|c| !c.is_transparent());
            assert_eq!(
                painted_fill(&mut cx, control),
                want,
                "{preset}: {control} is not painted in {leaf}"
            );
        }
    }
}

/// The disabled Basic checkbox under adwaita, which fades a disabled control
/// by `checkbox.disabled_opacity`, paints the fill its info claims
/// (M9-053): the swatch is the faded fill `CheckboxLook::faded_over` gives,
/// not the stated colour.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_disabled_basic_checkbox_paints_the_fill_it_claims(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "adwaita");
    show(&mut cx, &showcase, Page::Basic);
    let r = resolved_theme(&mut cx, &showcase);
    assert!(
        r.checkbox.disabled_opacity < 1.0,
        "adwaita no longer fades a disabled checkbox, so this proves nothing"
    );
    let info = settle_on(&mut cx, &showcase, "basic-checkbox-disabled");
    let claimed = info
        .as_ref()
        .and_then(|info| claim_as(info, "fill"))
        .map(|(_, colour)| colour);
    assert!(
        claimed.is_some(),
        "the disabled checkbox's info claims no fill: {info:?}"
    );
    assert_eq!(
        painted_fill(&mut cx, "basic-checkbox-disabled"),
        claimed,
        "the disabled checkbox does not paint the fill its info claims: {info:?}"
    );
}

/// A Link is underlined at rest only where the theme says so (M9-061):
/// under ios-13 and material-3, which state `link.underline_enabled =
/// false`, no underline is painted inside the resting Link and its info
/// claims none and says `geometry::link` removes it; under kde-breeze one is,
/// and its info claims it.
#[gpui::test]
fn a_link_is_underlined_at_rest_only_where_the_theme_says(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let mut reached = (false, false);
    for preset in ["ios-13", "material-3", "kde-breeze"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Basic);
        let underlined = resolved_theme(&mut cx, &showcase).link.underline_enabled;
        if underlined {
            reached.0 = true;
        } else {
            reached.1 = true;
        }
        hover(&mut cx, point(px(4.), px(4.)));
        draw(&mut cx);
        let link = bounds_of(&mut cx, "basic-link");
        let painted = cx.update(|window, _| {
            let link = link.scale(window.scale_factor());
            scene_underlines(window)
                .into_iter()
                .filter(|u| u.bounds.intersects(&link))
                .count()
        });
        assert_eq!(
            painted,
            usize::from(underlined),
            "{preset}: link.underline_enabled is {underlined}, and the resting Link paints {painted} underlines"
        );
        let info = settle_on(&mut cx, &showcase, "basic-link");
        let resting = info.as_ref().is_some_and(|info| {
            info.colors
                .iter()
                .any(|c| c.role.starts_with("resting underline"))
        });
        assert_eq!(
            resting, underlined,
            "{preset}: link.underline_enabled is {underlined}, and the Link's info claims a resting underline: {resting}: {info:?}"
        );
        let removed = info.as_ref().is_some_and(|info| {
            info.to_text().contains(
                "geometry::link removes the resting underline; hover and press still underline",
            )
        });
        assert_eq!(
            removed, !underlined,
            "{preset}: link.underline_enabled is {underlined}, and the Link's info says geometry::link removes the underline: {removed}: {info:?}"
        );
        assert!(
            info.as_ref()
                .is_some_and(|info| !info.to_text().contains("no receiver")),
            "{preset}: the Link's info says link.underline_enabled has no receiver: {info:?}"
        );
    }
    assert_eq!(
        reached,
        (true, true),
        "the presets no longer leave one Link underlined and one not, so a branch went unread"
    );
}

/// The Empty state's info prints the size its icon is drawn at (M9-058):
/// the installed theme's `defaults.icon_sizes.large` where a native theme is
/// installed (kde-breeze), and upstream's size where none is, in place of
/// the literals "32px, or 48px on KDE".
#[gpui::test]
fn the_empty_states_info_prints_its_icons_size(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    choose_icon_theme(&mut cx, &showcase, "gpui-component built-in (Lucide)");
    show(&mut cx, &showcase, Page::Feedback);
    let large = resolved_theme(&mut cx, &showcase).defaults.icon_sizes.large;
    let line = |info: &Option<WidgetInfo>| {
        info.as_ref()
            .and_then(|info| note_of(&info.config, "icon size").map(str::to_string))
    };
    let info = settle_on(&mut cx, &showcase, "feedback-empty");
    let want = format!(
        "defaults.icon_sizes.large, {}px",
        crate::info::px_text(large)
    );
    assert!(
        line(&info).is_some_and(|text| text.starts_with(&want)),
        "under kde-breeze the Empty state's icon size line does not print {want:?}: {info:?}"
    );
    assert!(
        info.as_ref()
            .is_some_and(|info| !info.to_text().contains("48px on KDE")),
        "the Empty state's info still names the literal sizes: {info:?}"
    );
    cx.update(|_window, cx| {
        if cx.has_global::<native_theme_gpui::NativeTheme>() {
            cx.remove_global::<native_theme_gpui::NativeTheme>();
        }
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        cx.update(|_window, cx| cx.native_theme().is_none()),
        "the native theme is still installed, so this proves nothing"
    );
    let info = settle_on(&mut cx, &showcase, "feedback-empty");
    assert!(
        line(&info).is_some_and(|text| text.starts_with("upstream's size")),
        "with no native theme the Empty state's icon size line does not say upstream's size: {info:?}"
    );
}

/// Both tables draw their header row `list.header_height` tall where the
/// theme states it (adwaita, 30), and as tall as a body row is drawn where it
/// does not (kde-breeze, which states no header height): the Data page's
/// `DataTable` at `geometry::data_table_size`'s row height, the Basic page's
/// table at `list.row_height`, or else its padding and the header font's line,
/// on the pixel grid.
#[gpui::test]
fn the_tables_headers_take_the_themes_header_height(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    for preset in ["adwaita", "kde-breeze"] {
        use_preset(&mut cx, &showcase, preset);
        let (stated, data_row, basic_row) = read(&mut cx, &showcase, |_, cx| {
            native_value(cx, |n| {
                let l = &n.resolved.list;
                let pad = &l.border.padding;
                let line = native_theme_gpui::scaled_text_size(l.header_font.size, n.accessibility)
                    * n.resolved.defaults.line_height;
                (
                    geometry::list_header_height(n),
                    geometry::data_table_size(n).table_row_height(),
                    px(l.row_height
                        .unwrap_or(pad.top.unwrap_or(0.) + line + pad.bottom.unwrap_or(0.))),
                )
            })
        })
        .unwrap_or_else(|| panic!("{preset}: no native theme is installed"));
        assert_eq!(
            stated.is_some(),
            preset == "adwaita",
            "precondition: {preset}'s header height"
        );
        show(&mut cx, &showcase, Page::Data);
        assert_eq!(
            bounds_of(&mut cx, DATA_TABLE_HEADER).size.height,
            stated.unwrap_or(data_row),
            "{preset}: the DataTable's header row"
        );
        show(&mut cx, &showcase, Page::Basic);
        // Laid out on the pixel grid: a padding-and-line height rounds.
        let (drawn, want) = (
            bounds_of(&mut cx, "basic-table-header").size.height,
            stated.unwrap_or(basic_row),
        );
        assert!(
            (drawn.as_f32() - want.as_f32()).abs() < 0.5,
            "{preset}: the Basic table's header row is {drawn:?}, not {want:?}"
        );
    }
}

/// The Basic expander draws what `expander.*` states (ISSUES D8): each title
/// row `header_height` tall, its arrow `arrow_icon_size` across; and the
/// Basic card is padded inside its edge by the padding `card.border` states
/// (kde-breeze), and by `layout.container_margin` where it states none
/// (material-3; ISSUES D9), a card being a container.
#[gpui::test]
fn the_expander_and_the_card_take_the_themes_sizes(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    // A set every machine has, so the arrow is drawn: kde-breeze's own,
    // Breeze, need not be installed (a missing icon is drawn as nothing).
    choose_icon_theme(&mut cx, &showcase, "gpui-component built-in (Lucide)");
    for preset in ["kde-breeze", "material-3"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Basic);
        let (header, arrow, card_padding, card_line) = read(&mut cx, &showcase, |_, cx| {
            native_value(cx, |n| {
                let e = &n.resolved.expander;
                (
                    e.header_height,
                    e.arrow_icon_size,
                    n.resolved.card.border.padding,
                    n.resolved.card.border.line_width,
                )
            })
        })
        .unwrap_or_default();
        for (row, icon) in [
            ("basic-expander-header-0", "basic-expander-arrow-0"),
            ("basic-expander-header-1", "basic-expander-arrow-1"),
        ] {
            assert_eq!(
                bounds_of(&mut cx, row).size.height,
                px(header),
                "{preset}: {row} is not expander.header_height"
            );
            let arrow_box = bounds_of(&mut cx, icon).size;
            assert_eq!(
                (arrow_box.width, arrow_box.height),
                (px(arrow), px(arrow)),
                "{preset}: {icon} is not expander.arrow_icon_size"
            );
        }
        let margin = read(&mut cx, &showcase, |this, _| {
            geometry::container_margin(&this.layout)
        });
        let (Some(inset), card, text) = (
            card_padding.left.map(px).or(margin),
            bounds_of(&mut cx, "basic-card"),
            bounds_of(&mut cx, "basic-card-text"),
        ) else {
            continue;
        };
        assert_eq!(
            text.left() - card.left(),
            px(card_line) + inset,
            "{preset}: the card's content is not card.border.padding_left, or \
             layout.container_margin where that is unstated, inside its edge"
        );
    }
}

/// Whether `info` records `geometry::container_margin`'s line.
fn names_the_container_margin(info: &WidgetInfo) -> bool {
    info.config
        .iter()
        .any(|n| n.what == "geometry" && n.text.starts_with("geometry::container_margin:"))
}

/// The card's and the GroupBoxes' infos record `geometry::container_margin`
/// exactly where it pads a side of their content: where the layout states it
/// and `card.border.padding` leaves a side unstated, not where the padding
/// states all four (windows-11 and kde-breeze), which leaves the margin
/// nothing to pad (the M8 T5 carry-over). Every bundled preset in both
/// modes, each expectation read from the theme installed.
#[gpui::test]
fn the_card_and_the_group_boxes_name_the_container_margin_where_it_pads(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let mut cases = (false, false);
    for mode in [AppColorMode::Light, AppColorMode::Dark] {
        switch_mode(&mut cx, mode);
        for preset in native_theme::theme::Theme::list_presets() {
            use_preset(&mut cx, &showcase, preset.key);
            let p = resolved_theme(&mut cx, &showcase).card.border.padding;
            let margin = read(&mut cx, &showcase, |this, _| {
                geometry::container_margin(&this.layout)
            });
            let pads = margin.is_some()
                && [p.top, p.right, p.bottom, p.left]
                    .iter()
                    .any(Option::is_none);
            if margin.is_some() {
                if pads {
                    cases.0 = true;
                } else {
                    cases.1 = true;
                }
            }
            show(&mut cx, &showcase, Page::Basic);
            // A corner of the card, inside its content's padding, off the
            // Label that reports itself.
            let card = bounds_of(&mut cx, "basic-card");
            let info = settle_at(&mut cx, &showcase, card.origin + point(px(1.), px(1.)));
            assert_eq!(
                info.as_ref().map(names_the_container_margin),
                Some(pads),
                "{} {mode:?}: the card's info {} geometry::container_margin, and the margin \
                 pads {} side of card.border.padding {p:?} (container_margin {margin:?}): {info:?}",
                preset.key,
                if pads { "does not record" } else { "records" },
                if pads { "an unstated" } else { "no" },
            );
            show(&mut cx, &showcase, Page::Layout);
            for group_box in [LAYOUT_GROUP_BOX_NORMAL, LAYOUT_GROUP_BOX_OUTLINE] {
                let info = settle_on(&mut cx, &showcase, group_box);
                assert_eq!(
                    info.as_ref().map(names_the_container_margin),
                    Some(pads),
                    "{} {mode:?}: {group_box}'s info {} geometry::container_margin, and the \
                     margin pads {} side of card.border.padding {p:?} (container_margin \
                     {margin:?}): {info:?}",
                    preset.key,
                    if pads { "does not record" } else { "records" },
                    if pads { "an unstated" } else { "no" },
                );
            }
        }
    }
    assert_eq!(
        cases,
        (true, true),
        "the presets no longer cover a card.border.padding that leaves a side to the margin \
         and one that states all four"
    );
}

/// The card's padding line names each side by where it comes from (the M8
/// T5 carry-over): `card.border's` for exactly the sides `card.border.padding`
/// states, and `layout.container_margin` for the rest where the layout states
/// one, the variant's own `p_4` where it does not; under every bundled preset
/// in both modes, with the preset's own layout.
#[test]
fn the_cards_padding_line_names_where_each_side_comes_from() {
    use crate::info::px_text;
    for (name, r, t) in every_bundled_variant() {
        let key = name.split(' ').next().unwrap_or_default();
        let layout = native_theme::theme::Theme::preset(key).map(|theme| theme.layout);
        assert!(layout.is_ok(), "{key} does not load");
        let Ok(layout) = layout else { continue };
        let margin = geometry::container_margin(&layout);
        let card = crate::info::layout::card(&t, Some(&r), margin);
        let line = note_of(&card.config, "padding").unwrap_or_default();
        let p = &r.card.border.padding;
        for (side, stated) in [
            ("top", p.top),
            ("right", p.right),
            ("bottom", p.bottom),
            ("left", p.left),
        ] {
            let want = match (stated, margin) {
                (Some(v), _) => format!("{side} {}px, card.border's", px_text(v)),
                (None, Some(m)) => {
                    format!("{side} {}px, layout.container_margin", px_text(m.as_f32()))
                }
                (None, None) => format!("{side} p_4, the Fill variant's own"),
            };
            assert!(
                line.contains(&want),
                "{name}: the card's padding line {line:?} does not say {want:?}"
            );
        }
    }
}

/// The theme-drawn expander's Widget Info says what it frames and how it
/// pads (M9-046): no frame claimed where `expander.frame_enabled` is false
/// (kde-breeze, KDE's expander draws none); where it is true each item
/// framed on its own by `expander.border.color` at
/// `expander.border.line_width`, `layout.widget_gap` from the next
/// (adwaita, windows-11); each title row padded side by side by the
/// `expander.border.padding` the header takes, upstream's px_3 on an
/// unstated left or right and none on an unstated top or bottom; and the
/// body's own padding where no `layout.widget_gap` parts it from its title:
/// where `frame_enabled` is unstated (material-3), and where it is stated
/// but the layout states no `widget_gap` (adwaita with the gap taken out).
/// Every expected value is the resolved theme's, read at test time.
#[gpui::test]
fn the_native_expanders_info_says_what_it_frames_and_pads(cx: &mut TestAppContext) {
    use crate::info::{px_text, stated};
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    let mut cases = (false, false, false, false, false);
    for (preset, no_gap) in [
        ("kde-breeze", false),
        ("adwaita", false),
        ("windows-11", false),
        ("material-3", false),
        ("adwaita", true),
    ] {
        use_preset(&mut cx, &showcase, preset);
        if no_gap {
            cx.update(|_window, cx| {
                showcase.update(cx, |this, cx| {
                    this.layout.widget_gap = None;
                    cx.notify();
                });
            });
            cx.run_until_parked();
            draw(&mut cx);
        }
        show(&mut cx, &showcase, Page::Basic);
        let r = resolved_theme(&mut cx, &showcase);
        let e = &r.expander;
        let gap = read(&mut cx, &showcase, |this, _| {
            geometry::widget_gap(&this.layout)
        });
        let info = settle_on(&mut cx, &showcase, "basic-expander-header-0")
            .unwrap_or_else(|| panic!("{preset}: the expander's item shows no info"));
        let framed = e.frame_enabled != Some(false) && e.border.line_width > 0.0;
        assert_eq!(
            claim_of(&info, "border"),
            framed.then(|| stated(e.border.color)),
            "{preset}: the expander's info claims a frame colour where none is drawn, or none where one is: {info:?}"
        );
        let edge = note_of(&info.config, "edge").unwrap_or_default();
        match e.frame_enabled {
            Some(false) => {
                cases.0 = true;
                assert!(
                    edge.starts_with("none: expander.frame_enabled is false"),
                    "{preset}: the expander draws no frame, and its edge line says {edge:?}"
                );
            }
            Some(true) if framed => {
                cases.1 = true;
                let apart = match gap {
                    Some(gap) => format!("layout.widget_gap, {}px, apart", px_text(gap.as_f32())),
                    None => "no gap between them: layout.widget_gap is unstated".to_string(),
                };
                for want in [
                    "each item framed on its own".to_string(),
                    format!(
                        "expander.border.color at expander.border.line_width, {}px",
                        px_text(e.border.line_width)
                    ),
                    apart,
                ] {
                    assert!(
                        edge.contains(&want),
                        "{preset}: the expander frames each item on its own, and its edge line {edge:?} does not say {want:?}"
                    );
                }
            }
            _ => {}
        }
        let padding = note_of(&info.config, "padding").unwrap_or_default();
        let p = &e.border.padding;
        for (side, value, across) in [
            ("top", p.top, false),
            ("right", p.right, true),
            ("bottom", p.bottom, false),
            ("left", p.left, true),
        ] {
            let want = match (value, across) {
                (Some(v), _) => format!("{side} {}px", px_text(v)),
                (None, true) => {
                    cases.2 = true;
                    format!("{side} upstream's px_3")
                }
                (None, false) => format!("{side} none"),
            };
            assert!(
                padding.contains(&want),
                "{preset}: the expander's padding line {padding:?} does not say {want:?}"
            );
        }
        let body = padding.contains("pb_2");
        cases.3 |= e.frame_enabled.is_none() && body;
        cases.4 |= e.frame_enabled.is_some() && gap.is_none() && body;
        assert_eq!(
            body,
            e.frame_enabled.is_none() || gap.is_none(),
            "{preset}: the body is padded as upstream's AccordionItem pads it exactly where no layout.widget_gap parts it from its title (frame_enabled unstated, or widget_gap unstated), and the padding line says {padding:?}"
        );
    }
    assert_eq!(
        cases,
        (true, true, true, true, true),
        "the presets no longer cover an unframed expander, one framed per item, an unstated side, a frame the theme leaves unstated, and a stated frame with no widget_gap"
    );
}

/// The tab rows draw exactly what `tab.*` states (ISSUES D7), through the
/// connector's `widgets::TabBar`: the selected page tab is filled with
/// `tab.active_background` and outlined in `tab.border`, an unselected one
/// filled with `tab.background_color` and outlined in nothing, and nothing
/// in the row is painted in the accent that upstream's Underline bar marks
/// it with, under a preset whose unselected tab differs from its bar
/// (kde-breeze) and one whose does not (material-3).
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_tab_rows_draw_what_tab_states(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for preset in ["kde-breeze", "material-3"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Basic);
        let (active, idle, edge) = read(&mut cx, &showcase, |_, cx| {
            (
                native_color(cx, |n| n.resolved.tab.active_background),
                native_color(cx, |n| n.resolved.tab.background_color),
                // An outline shows where it has a width and a colour.
                native_color(cx, |n| n.resolved.tab.border.color).filter(|c| {
                    !c.is_transparent()
                        && native_value(cx, |n| n.resolved.tab.border.line_width)
                            .is_some_and(|w| w > 0.)
                }),
            )
        });
        assert_eq!(
            painted_fill(&mut cx, Page::Basic.tab()),
            active,
            "{preset}: the selected page tab is not filled with tab.active_background"
        );
        assert_eq!(
            painted_fill(&mut cx, Page::Buttons.tab()),
            idle,
            "{preset}: an unselected page tab is not filled with tab.background_color"
        );
        assert_eq!(
            painted_edge(&mut cx, Page::Basic.tab()).filter(|c| !c.is_transparent()),
            edge,
            "{preset}: the selected page tab is not outlined in tab.border.color"
        );
        if preset == "kde-breeze" {
            assert!(
                edge.is_some(),
                "kde-breeze states the selected tab's outline"
            );
        }
        assert!(
            painted_edge(&mut cx, Page::Buttons.tab()).is_none_or(|c| c.is_transparent()),
            "{preset}: an unselected page tab is outlined"
        );
        let row = bounds_of(&mut cx, CHROME_PAGE_TABS);
        let accent = cx.update(|window, cx| {
            let primary = Theme::global(cx).primary;
            let scale = window.scale_factor();
            let row = row.scale(scale);
            // The selected tab's line, where tab.* states one, may be the
            // accent's colour: Breeze's is the selection colour.
            let line = native_value(cx, |n| n.resolved.tab.active_indicator_width)
                .flatten()
                .map(|w| px(w).scale(scale));
            scene_quads(window).into_iter().any(|q| {
                q.background.as_solid() == Some(primary)
                    && q.bounds.top() >= row.top()
                    && q.bounds.bottom() <= row.bottom()
                    && line != Some(q.bounds.size.height)
            })
        });
        assert!(
            !accent,
            "{preset}: the page tab row paints the accent beyond what tab.* states"
        );
    }
}

/// Under windows-11 with an outward corner radius of 4 set on it, installed
/// as the connector installs a theme, after the showcase opens: the selected
/// page tab curves out at both feet. At each, about the centre 4 outside and
/// 4 above the foot, set the outline's width in, a flare is painted clipped
/// to its 4px square: its fill in tab.active_background and its outline in
/// tab.border.color, as `widgets::TabBar` draws them.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_selected_page_tab_curves_out_where_the_theme_states_it(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    show(&mut cx, &showcase, Page::Basic);
    let is_dark = read(&mut cx, &showcase, |this, _| this.is_dark);
    let (theme, r) = windows_11_curving_out(is_dark);
    cx.update(|_window, cx| {
        let prefs = native_theme_gpui::AccessibilityPreferences::default();
        native_theme_gpui::apply(theme, &r, &prefs, cx);
    });
    cx.run_until_parked();
    draw(&mut cx);
    let look = native_theme_gpui::widgets::TabLook::of(&r).expect("windows-11's tab look builds");
    let tab = bounds_of(&mut cx, Page::Basic.tab());
    let w = f32::from(look.border_width);
    let feet = [
        f32::from(tab.left()) + w - 4.0,
        f32::from(tab.right()) - w + 4.0,
    ];
    let y = f32::from(tab.bottom()) - 4.0;
    let far = 4.0 * std::f32::consts::SQRT_2 + 1.0;
    let missing = cx.update(|window, _| {
        let s = window.scale_factor();
        let quads = scene_quads(window);
        let mut missing = Vec::new();
        for x in feet {
            for (colour, radius) in [(look.active, far), (look.border, 4.0)] {
                let painted = quads.iter().any(|q| {
                    let centre = (
                        q.bounds.origin.x.0 + q.bounds.size.width.0 / 2.0,
                        q.bounds.origin.y.0 + q.bounds.size.height.0 / 2.0,
                    );
                    q.background.is_transparent()
                        && q.border_color == colour
                        && (q.corner_radii.top_left.0 - radius * s).abs() < 1e-3
                        && (centre.0 - x * s).abs() <= 1.0
                        && (centre.1 - y * s).abs() <= 1.0
                        && q.content_mask.bounds.size.width.0 <= 4.0 * s + 2.0
                        && q.content_mask.bounds.size.height.0 <= 4.0 * s + 2.0
                });
                if !painted {
                    missing.push((x, colour, radius));
                }
            }
        }
        missing
    });
    assert!(missing.is_empty(), "no flare quad at {missing:?}");
}

/// The smallest box that hovering `at` paints and that was not painted
/// before, and that holds `at`: the hovered element's hover fill.
fn hover_box(cx: &mut VisualTestContext, at: Point<Pixels>) -> Option<Bounds<Pixels>> {
    let painted = |cx: &mut VisualTestContext| {
        cx.update(|window, _| {
            let scale = window.scale_factor();
            scene_quads(window)
                .into_iter()
                .filter(|q| !q.background.is_transparent())
                .map(|q| Bounds {
                    origin: point(
                        px(q.bounds.origin.x.0 / scale),
                        px(q.bounds.origin.y.0 / scale),
                    ),
                    size: size(
                        px(q.bounds.size.width.0 / scale),
                        px(q.bounds.size.height.0 / scale),
                    ),
                })
                .collect::<Vec<_>>()
        })
    };
    let before = painted(cx);
    hover(cx, at);
    draw(cx);
    painted(cx)
        .into_iter()
        .filter(|b| !before.contains(b) && b.contains(&at))
        .min_by(|a, b| {
            let area = |b: &Bounds<Pixels>| b.size.width.as_f32() * b.size.height.as_f32();
            area(a).total_cmp(&area(b))
        })
}

/// Upstream's Underline TabBar pads neither itself nor its tabs
/// (tab/tab.rs:121-122, tab/tab_bar.rs:412-421): it leaves the inset to the
/// container it is in. So the page TabBar is inset by
/// `layout.container_margin`, and the inspector's by what pads the side
/// panel's content: the panel's own padding where the theme states a
/// `sidebar.border.padding` (adwaita), a side it leaves unstated taking
/// `layout.container_margin`, else `layout.container_margin` on the content
/// (kde-breeze) -- read from the theme installed. Its first tab starts that
/// far from the edge of its panel, not against the resize handle's line or
/// the window's edge.
///
/// On the right, the page TabBar's menu Button (tab/tab_bar.rs, `menu`)
/// ends that far from the panel's right edge, and the inspector's last tab
/// ends no nearer to it. The page row's rule is drawn under the bar, which
/// spans the panel, so it spans the panel's whole width too.
#[gpui::test]
fn the_tab_bars_are_inset_by_the_container_margin(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let slack = device_pixel(&mut cx);
    for preset in ["kde-breeze", "adwaita"] {
        use_preset(&mut cx, &showcase, preset);
        let margin = read(&mut cx, &showcase, |this, _| {
            geometry::container_margin(&this.layout)
        });
        let Some(margin) = margin else {
            panic!("{preset} states no layout.container_margin");
        };
        let pad = side_panel_padding(&mut cx, &showcase);
        let content = read(&mut cx, &showcase, |_, cx| {
            crate::demo::side_panel_content_margin(cx, Some(margin))
        })
        .unwrap_or_default();
        for (bar, selector, panel, (left, right), first, last) in [
            (
                "page",
                CHROME_PAGE_TABS,
                CONTENT_PANEL,
                (margin, margin),
                Page::ALL[0].tab(),
                None,
            ),
            (
                "inspector",
                INSPECTOR_TABS,
                CHROME_SIDE_PANEL,
                (pad.left + content, pad.right + content),
                InspectorTab::Widget.tab(),
                Some(InspectorTab::Theme.tab()),
            ),
        ] {
            let panel = bounds_of(&mut cx, panel);
            let tab = bounds_of(&mut cx, first);
            assert!(
                (tab.left() - panel.left() - left).abs() <= slack,
                "{preset}: the {bar} TabBar's first tab starts at {:?}, {:?} from its panel's \
                 left edge, not {left:?} (layout.container_margin {margin:?}, the side panel's \
                 padding {pad:?})",
                tab.left(),
                tab.left() - panel.left()
            );
            let tabs = bounds_of(&mut cx, selector);
            // The page row alone has a rule under it (demo::native_tab_bar,
            // the theme-drawn tab row of the `widgets` feature), as
            // docs/showcase-elements.toml's `chrome.page_tabs.rule`.
            if cfg!(feature = "widgets") && bar == "page" {
                // Under the strip itself, the bar's first child, which the
                // dump records as `chrome.page_tabs`.
                let (rule, strip) = read(&mut cx, &showcase, |this, cx| {
                    let drawn = this.info_ui.read(cx).layout_drawn();
                    (
                        drawn.get("chrome.page_tabs.rule").copied(),
                        drawn.get("chrome.page_tabs").copied(),
                    )
                });
                let under = strip.map_or(tabs.bottom(), |strip| strip.bottom());
                assert!(
                    rule.is_some_and(|rule| (rule.left() - panel.left()).abs() <= slack
                        && (rule.right() - panel.right()).abs() <= slack
                        && (rule.top() - under).abs() <= slack),
                    "{preset}: the {bar} TabBar's rule at {rule:?} does not span its panel \
                     at {panel:?} under the bar at {tabs:?}"
                );
            }
            match last {
                Some(last) => {
                    let last = bounds_of(&mut cx, last);
                    assert!(
                        last.right() <= panel.right() - right + slack,
                        "{preset}: the {bar} TabBar's last tab ends at {:?}, within {right:?} \
                         of its panel's right edge at {:?}",
                        last.right(),
                        panel.right()
                    );
                }
                // The menu Button shows the icon set's ChevronDown, which a
                // build with no icon set does not have.
                None if !cfg!(any(
                    feature = "material-icons",
                    feature = "lucide-icons",
                    feature = "system-icons"
                )) => {}
                None => {
                    let menu = hover_box(
                        &mut cx,
                        point(panel.right() - right - px(4.), tabs.center().y),
                    );
                    assert!(
                        menu.is_some_and(
                            |menu| (panel.right() - menu.right() - right).abs() <= slack
                        ),
                        "{preset}: the {bar} TabBar's menu Button at {menu:?} does not end \
                         layout.container_margin, {right:?}, from its panel's right edge at {:?}",
                        panel.right()
                    );
                }
            }
        }
    }
}

/// The side panel's own padding per side, as `demo::side_panel` draws it
/// under the theme installed: where the theme states any side of
/// `sidebar.border.padding`, that side, else `layout.container_margin`, else
/// none; where it states no side, none (the margin then pads the content,
/// `demo::side_panel_content_margin`).
fn side_panel_padding(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
) -> gpui::Edges<Pixels> {
    read(cx, showcase, |this, cx| {
        let margin = geometry::container_margin(&this.layout);
        crate::demo::sidebar_padding(cx).map(|p| {
            let side = |v: Option<f32>| v.map(px).or(margin).unwrap_or_default();
            gpui::Edges {
                top: side(p.top),
                right: side(p.right),
                bottom: side(p.bottom),
                left: side(p.left),
            }
        })
    })
    .unwrap_or_default()
}

/// The title of the info the inspector drew in the last frame, and whether
/// the label that shows it was laid out.
fn inspector_title(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> Option<String> {
    let title = read(cx, showcase, |this, cx| {
        this.inspector.read(cx).title_drawn.clone()
    });
    let label = cx.debug_bounds(INSPECTOR_TITLE);
    assert_eq!(
        title.is_some(),
        label.is_some(),
        "the inspector drew {title:?}, and its title label was laid out at {label:?}"
    );
    title.map(|t| t.to_string())
}

/// The inspector shows the info the pointer has settled on (spec §2.6, §4.2).
///
/// A page tab is hovered.
#[gpui::test]
fn the_inspector_shows_the_settled_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(
        inspector_title(&mut cx, &showcase),
        None,
        "the inspector shows an info before anything was hovered"
    );
    let tab = bounds_of(&mut cx, Page::Charts.tab());
    hover(&mut cx, tab.center());
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some(PAGE_TABS_LISTED_TITLE),
        "the inspector does not show the page TabBar the pointer settled on"
    );
}

/// A shown info follows the state of its widget while the pointer stays
/// still: clicking the hovered, unchecked Checkbox checks it, and the
/// inspector says so without another hover event.
#[gpui::test]
fn a_shown_info_follows_its_widgets_state(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Inputs);
    // The leading edge, where the box is, as `click` presses.
    let bounds = bounds_of(&mut cx, INPUTS_CHECKBOX_AUTOSAVE);
    let at = point(bounds.left() + px(8.), bounds.center().y);
    hover(&mut cx, at);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("Checkbox · unchecked")
    );
    click_at(&mut cx, at);
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("Checkbox · checked"),
        "the Checkbox the pointer rests on was checked, and the inspector did not follow"
    );
}

/// How often the inspector has serialised a resolved theme on this thread
/// (`inspector::THEME_SERIALISATIONS`).
fn theme_serialisations() -> usize {
    crate::inspector::THEME_SERIALISATIONS.with(std::cell::Cell::get)
}

/// The inspector serialises the installed theme once an install, not once a
/// frame (M9-020): the Basic page's Spinner asks for a frame every frame, and
/// gpui renders the inspector, an uncached view, on every draw. A preset is
/// installed and ten frames are drawn on the Basic page, the inspector
/// showing its hint.
#[gpui::test]
fn the_inspector_serialises_the_theme_once_an_install(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Basic
    );
    let before = theme_serialisations();
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.apply_theme_by_name("kde-breeze", window, cx);
        });
    });
    cx.run_until_parked();
    for _ in 0..10 {
        draw(&mut cx);
    }
    assert_eq!(
        theme_serialisations() - before,
        1,
        "an install and ten frames on the Basic page did not serialise the theme once"
    );
}

/// The look the inspector is drawn in, as the showcase keeps it
/// (`Showcase::panel_look`), is the installed theme's after every kind of
/// install: a preset install, a colour-mode switch under `default`, which
/// re-picks the other variant of the desktop's theme it kept, a Preferences
/// change, the theme watcher's re-read, and an `apply` the connector is
/// called for directly. After each, the look the next frame keeps equals one
/// built afresh, and it changed where the install changed what it reads: the
/// two modes under `default`, the text scale, a colour.
#[gpui::test]
fn the_kept_panel_look_follows_every_install(cx: &mut TestAppContext) {
    use crate::inspector::PanelLook;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let kept = |cx: &mut VisualTestContext, what: &str| -> Option<PanelLook> {
        cx.run_until_parked();
        draw(cx);
        let (kept, fresh) = read(cx, &showcase, |this, cx| {
            (this.panel_look.clone(), PanelLook::of(this, cx))
        });
        assert!(kept.is_some(), "{what}: the showcase keeps no panel look");
        assert_eq!(
            kept, fresh,
            "{what}: the panel look kept is not the installed theme's"
        );
        kept
    };
    set_preset(&mut cx, &showcase, "kde-breeze");
    let breeze = kept(&mut cx, "a preset install");

    set_preset(&mut cx, &showcase, "default");
    let shown = read(&mut cx, &showcase, |this, _| this.is_dark);
    let mut looks = Vec::new();
    for mode in [!shown, shown] {
        let mode = if mode {
            AppColorMode::Dark
        } else {
            AppColorMode::Light
        };
        cx.update(|window, cx| window.dispatch_action(Box::new(SetColorMode(mode)), cx));
        looks.push(kept(
            &mut cx,
            &format!("a switch to {mode:?} under default"),
        ));
    }
    assert_ne!(
        looks.first(),
        looks.last(),
        "the two modes under default keep one panel look"
    );

    let before = looks.last().cloned().flatten();
    cx.update(|_window, cx| {
        let ui = showcase.read(cx).info_ui.clone();
        crate::demo::change_preferences(&ui, cx, |overrides| {
            overrides.text_scaling_factor = Some(1.5);
        });
    });
    let scaled = kept(&mut cx, "a Preferences change");
    assert_ne!(
        scaled, before,
        "a text scale of 1.5 left the panel look as it was"
    );

    cx.update(|window, cx| showcase.update(cx, |this, cx| this.on_os_theme_change(window, cx)));
    kept(&mut cx, "the theme watcher's re-read");

    set_preset(&mut cx, &showcase, "kde-breeze");
    let mut r = resolved_theme(&mut cx, &showcase);
    let is_dark = read(&mut cx, &showcase, |this, _| this.is_dark);
    assert_ne!(r.sidebar.font.color, r.defaults.accent_color);
    r.sidebar.font.color = r.defaults.accent_color;
    cx.update(|_window, cx| {
        let prefs = native_theme::AccessibilityPreferences::default();
        native_theme_gpui::apply(
            native_theme_gpui::to_theme(&r, "kde-breeze", is_dark, &prefs),
            &r,
            &prefs,
            cx,
        );
    });
    let applied = kept(&mut cx, "an apply of the connector's");
    assert_ne!(
        applied, breeze,
        "a changed sidebar.font.color left the panel look as it was"
    );
}

/// The inspector's Copy button puts the shown info's text on the clipboard
/// (spec §2.6).
#[gpui::test]
fn the_inspector_copies_the_shown_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let tab = bounds_of(&mut cx, Page::Charts.tab());
    hover(&mut cx, tab.center());
    settle(&mut cx);
    draw(&mut cx);
    // The page TabBar is an element of docs/showcase-elements.toml: the panel
    // shows its title and a row per leaf the list names, and Copy copies
    // that text.
    let shown = read(&mut cx, &showcase, |this, cx| {
        this.info_ui
            .read(cx)
            .shown()
            .map(|info| (crate::inspector::shown_title(info), info.listed))
    });
    let element = shown
        .as_ref()
        .and_then(|(_, listed)| *listed)
        .and_then(crate::elements::element);
    assert!(
        element.is_some(),
        "the page TabBar is not shown as an element of the list: {shown:?}"
    );
    click(&mut cx, INSPECTOR_COPY);
    let copied = cx
        .read_from_clipboard()
        .and_then(|item| item.text())
        .unwrap_or_default();
    let title = shown.map(|(title, _)| title).unwrap_or_default();
    assert!(
        copied.starts_with(&format!("{title}\n\nTheme\n")),
        "the Copy button did not put the shown info on the clipboard: {copied:?}"
    );
    for leaf in element.iter().flat_map(|e| e.leaves.iter()) {
        assert!(
            copied.contains(&format!("\n{leaf} ")),
            "the copied info has no row for {leaf}: {copied:?}"
        );
    }
}

/// The inspector's Copy box letters its text as its routes say (M9-037): in
/// `sidebar.font.color` at rest, `button.hover_text_color` under the pointer
/// and `button.active_text_color` while pressed. The text is the box's own
/// child, so it is painted in the text colour the box is laid out under,
/// which the registry records for `chrome.info.copy` (a text colour is no
/// quad of the scene). kde-breeze light states the same colour for its
/// sidebar text, its hovered button text and its pressed button text, so
/// the hovered one is set to the theme's accent and the pressed one to its
/// selection text, colours of the same theme that differ from the sidebar's
/// and from each other.
/// The routes are read from the panel's own text for `chrome.info.copy`
/// (`inspector::listed`, beside the info of the page tab the pointer settled
/// on), where the panel names each leaf with its value and how it applies.
#[gpui::test]
fn the_copy_box_takes_the_buttons_hover_and_pressed_text_colours(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_light_preset(&mut cx, &showcase, "kde-breeze");
    let mut r = resolved_theme(&mut cx, &showcase);
    r.button.hover_text_color = r.defaults.accent_color;
    r.button.active_text_color = r.defaults.selection_text_color;
    for (state, colour) in [
        ("hovered", r.button.hover_text_color),
        ("pressed", r.button.active_text_color),
    ] {
        assert_ne!(
            colour, r.sidebar.font.color,
            "kde-breeze light's {state} button text is its sidebar text, so the test could not tell the two apart"
        );
    }
    assert_ne!(r.button.hover_text_color, r.button.active_text_color);
    cx.update(|_window, cx| {
        let prefs = native_theme::AccessibilityPreferences::default();
        native_theme_gpui::apply(
            native_theme_gpui::to_theme(&r, "kde-breeze", false, &prefs),
            &r,
            &prefs,
            cx,
        );
    });
    cx.run_until_parked();
    draw(&mut cx);
    // A listed element's info, so the panel shows its title row and Copy.
    let info = settle_on(&mut cx, &showcase, Page::Charts.tab());
    assert!(
        info.as_ref().is_some_and(|info| info.listed.is_some()),
        "the page tab's info is not an element of the list, so no Copy box is drawn: {info:?}"
    );
    let Some(info) = info else { return };
    let ui = read(&mut cx, &showcase, |this, _| this.info_ui.clone());
    let text = |cx: &mut VisualTestContext| {
        cx.update(|_window, cx| ui.read(cx).text_colours.get("chrome.info.copy").copied())
    };
    assert_eq!(
        text(&mut cx),
        Some(stated(r.sidebar.font.color)),
        "the Copy box's text at rest is not sidebar.font.color"
    );
    let copy = bounds_of(&mut cx, INSPECTOR_COPY).center();
    hover(&mut cx, copy);
    draw(&mut cx);
    assert_eq!(
        text(&mut cx),
        Some(stated(r.button.hover_text_color)),
        "the Copy box's text under the pointer is not button.hover_text_color"
    );
    cx.simulate_mouse_down(copy, MouseButton::Left, Modifiers::default());
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        text(&mut cx),
        Some(stated(r.button.active_text_color)),
        "the Copy box's text while pressed is not button.active_text_color"
    );
    cx.simulate_mouse_up(copy, MouseButton::Left, Modifiers::default());
    cx.run_until_parked();
    let panel = read(&mut cx, &showcase, |this, _| {
        let look = this.panel_look.as_ref()?;
        let element = crate::elements::element("chrome.info.copy")?;
        Some(crate::inspector::listed(look, element, &info))
    });
    assert!(panel.is_some(), "no panel text for chrome.info.copy");
    let Some(panel) = panel else { return };
    for (leaf, colour, says) in [
        (
            "button.hover_text_color",
            r.button.hover_text_color,
            "the Copy box's text under the pointer",
        ),
        (
            "button.active_text_color",
            r.button.active_text_color,
            "the Copy box's text while pressed",
        ),
    ] {
        let row = panel.rows.iter().find(|row| row.leaf == leaf);
        assert!(
            row.is_some_and(|row| row.how.contains(says) && row.colour == Some(colour)),
            "the panel's {leaf} row does not say {says:?} with the colour drawn: {row:?}"
        );
    }
}

/// A toolbar button's Widget Info names the installed theme's icon theme
/// under `defaults.icon_theme`: under kde-breeze the variant's own, as the
/// iced and egui panels name it, where the resolved theme's JSON holds none.
#[gpui::test]
fn a_tool_buttons_info_names_its_icon_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let at = bounds_of(&mut cx, CHROME_TOOLBAR_PALETTE).center();
    hover(&mut cx, at);
    settle(&mut cx);
    draw(&mut cx);
    let (is_dark, loaded) = read(&mut cx, &showcase, |this, _| {
        (this.is_dark, this.current_icon_theme.clone())
    });
    let stated = native_theme::theme::Theme::preset("kde-breeze")
        .ok()
        .and_then(|theme| if is_dark { theme.dark } else { theme.light })
        .and_then(|variant| variant.defaults.icon_theme)
        .map(|name| name.to_string());
    assert!(
        stated.is_some(),
        "kde-breeze's variant states no icon theme"
    );
    assert_eq!(
        loaded, stated,
        "the showcase did not take kde-breeze's own icon theme"
    );
    let stated = stated.unwrap_or_default();
    click(&mut cx, INSPECTOR_COPY);
    let copied = cx
        .read_from_clipboard()
        .and_then(|item| item.text())
        .unwrap_or_default();
    assert!(
        copied.contains(&format!("\ndefaults.icon_theme {stated}\n")),
        "the tool button's info does not name the icon theme {stated}: {copied:?}"
    );
}

/// A page change keeps an info whose target is still drawn, as chrome is
/// (spec §4.3.4 as ruled: only what left the screen is cleared).
#[gpui::test]
fn a_page_change_keeps_the_chromes_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let tab = bounds_of(&mut cx, Page::Charts.tab());
    hover(&mut cx, tab.center());
    settle(&mut cx);
    draw(&mut cx);
    let before = inspector_title(&mut cx, &showcase);
    assert!(
        before.is_some(),
        "nothing was shown before the page changed"
    );
    run_menu_item(&mut cx, "View", "Typography");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Typography
    );
    assert_eq!(
        inspector_title(&mut cx, &showcase),
        before,
        "the page change cleared the info of the page TabBar, which is still drawn"
    );
}

/// A point in the page's own padding, which no page draws a widget in: half
/// of `layout.window_margin` in from the page root's top left corner, the
/// margin every page root is padded by (`pages::page_root`, and the Basic
/// page's own root). The theme has to state the margin, as every bundled
/// preset does, or the page has no padding to point at.
fn in_page_padding(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> Point<Pixels> {
    let margin = read(cx, showcase, |this, _| {
        geometry::window_margin(&this.layout)
    });
    assert!(
        margin.is_some_and(|m| m > px(0.)),
        "the theme states no layout.window_margin ({margin:?}), so a page root has no padding \
         to point at"
    );
    let half = margin.unwrap_or_default() * 0.5;
    let page = bounds_of(cx, PAGE_ROOT);
    point(page.left() + half, page.top() + half)
}

/// A page change clears an info whose target the new frame no longer draws
/// (spec §4.3.4 as ruled), through every route to a page: here the View
/// menu. The target is a Buttons page Button, which the Charts page does not
/// draw.
///
/// The pointer first moves into the page's own padding, which no page draws
/// a widget in: leaving the Button keeps its info (§4.3.3), so what clears
/// it is the page change, and what the Charts page draws there cannot settle
/// in its place.
#[gpui::test]
fn a_page_change_clears_what_left_the_screen(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    show(&mut cx, &showcase, Page::Buttons);
    let button = bounds_of(&mut cx, BUTTONS_PRIMARY);
    hover(&mut cx, button.center());
    settle(&mut cx);
    let shown = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().map(|info| info.title())
        })
    };
    let before = shown(&mut cx);
    assert!(
        before.as_deref().is_some_and(|t| t.starts_with("Button")),
        "the Buttons page's Primary Button did not report itself: {before:?}"
    );
    let padding = in_page_padding(&mut cx, &showcase);
    hover(&mut cx, padding);
    settle(&mut cx);
    assert_eq!(
        shown(&mut cx),
        before,
        "leaving the Button already cleared its info: leaving keeps it"
    );
    run_menu_item(&mut cx, "View", "Charts");
    settle(&mut cx);
    assert_eq!(
        shown(&mut cx),
        None,
        "after the page change the info of a Button no longer drawn is still shown"
    );
}

/// With no native theme installed the inspector says, once, that its
/// swatches show ThemeColor fields while upstream may paint derived tokens;
/// with one installed it does not.
#[gpui::test]
fn the_inspector_warns_of_tokens_only_without_a_native_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let tab = bounds_of(&mut cx, Page::Charts.tab());
    hover(&mut cx, tab.center());
    settle(&mut cx);
    draw(&mut cx);
    assert!(
        inspector_title(&mut cx, &showcase).is_some(),
        "nothing is shown, so no note could be"
    );
    assert!(
        cx.debug_bounds(INSPECTOR_TOKENS_NOTE).is_none(),
        "a native theme is installed, and the inspector still warns of tokens"
    );
    cx.update(|_window, cx| {
        if cx.has_global::<native_theme_gpui::NativeTheme>() {
            cx.remove_global::<native_theme_gpui::NativeTheme>();
        }
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        cx.update(|_window, cx| cx.native_theme().is_none()),
        "the native theme is still installed, so this proves nothing"
    );
    assert!(
        cx.debug_bounds(INSPECTOR_TOKENS_NOTE).is_some(),
        "with no native theme installed, the inspector does not warn of tokens"
    );
}

/// The inspector's Theme tab lays out the theme's and the window's facts in
/// place of the Widget tab.
#[gpui::test]
fn the_inspectors_theme_tab_lays_out(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let tab = bounds_of(&mut cx, Page::Charts.tab()).center();
    hover(&mut cx, tab);
    settle(&mut cx);
    draw(&mut cx);
    assert!(cx.debug_bounds(INSPECTOR_TITLE).is_some());
    cx.update(|_window, cx| {
        let inspector = showcase.read(cx).inspector.clone();
        inspector.update(cx, |i, cx| {
            i.tab = InspectorTab::Theme;
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        cx.debug_bounds(INSPECTOR_TITLE),
        None,
        "the Theme tab still shows the Widget tab's title"
    );
    let panel = bounds_of(&mut cx, INSPECTOR_PANEL);
    assert!(panel.size.width > px(0.) && panel.size.height > px(0.));
}

/// The status bar's "item gap" note names the gap inside its middle
/// region's first item, the toggle and the environment `layout.widget_gap` apart
/// (demo::status_bar), beside upstream's `gap_2` between a region's items
/// (M9-032).
#[gpui::test]
fn the_status_bars_item_gap_note_names_the_widget_gap(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let info = settle_on(&mut cx, &showcase, CHROME_STATUS_BAR);
    let note = info
        .as_ref()
        .and_then(|info| info.not_themeable.iter().find(|n| n.what == "item gap"));
    assert!(
        note.is_some_and(|n| n.text.contains("gap_2")
            && n.text.contains("layout.widget_gap")
            && n.text.contains("demo::status_bar")),
        "the status bar's item gap note does not name the toggle item's layout.widget_gap: {note:?}"
    );
}

/// The fill the panel toggle tagged `selector` paints with the pointer
/// elsewhere: the ghost variant's active colour while it is selected
/// (button/button.rs:1332, `colors.active`, which `variants::ghost_button`
/// fills with `secondary_active`), and none at rest -- the variant is
/// transparent there.
fn toggle_fill(cx: &mut VisualTestContext, selector: &'static str) -> Option<gpui::Hsla> {
    // The status bar's middle region holds nothing, so no widget there is
    // hovered.
    let middle = bounds_of(cx, CHROME_STATUS_BAR).center();
    hover(cx, middle);
    draw(cx);
    painted_fill(cx, selector)
}

/// What a selected panel toggle is filled with.
fn selected_fill(cx: &mut VisualTestContext) -> gpui::Hsla {
    cx.update(|_w, cx| Theme::global(cx).secondary_active)
}

/// Whether `inner` lies within `outer`, to a hundredth of a pixel.
fn within(inner: Bounds<Pixels>, outer: Bounds<Pixels>) -> bool {
    let slack = px(0.01);
    inner.left() >= outer.left() - slack
        && inner.right() <= outer.right() + slack
        && inner.top() >= outer.top() - slack
        && inner.bottom() <= outer.bottom() + slack
}

/// The status bar's side-panel toggle hides the side panel and shows it
/// again (spec S4), through `ToggleSidePanel`, and is selected while the
/// panel is shown. Hidden, nothing of the panel is drawn -- no rail -- and
/// the content takes the whole body.
#[gpui::test]
fn the_side_panel_toggle_hides_and_shows_it(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    assert!(
        !menu_item_disabled("View", "Toggle Side Panel"),
        "View > Toggle Side Panel is disabled"
    );
    let toggle = bounds_of(&mut cx, CHROME_SIDE_PANEL_TOGGLE);
    assert!(
        within(toggle, bounds_of(&mut cx, CHROME_STATUS_BAR)),
        "the side-panel toggle at {toggle:?} is not in the status bar"
    );
    let selected = selected_fill(&mut cx);
    assert_eq!(
        toggle_fill(&mut cx, CHROME_SIDE_PANEL_TOGGLE),
        Some(selected),
        "the side-panel toggle is not selected while the side panel is shown"
    );
    let shown = bounds_of(&mut cx, CHROME_SIDE_PANEL);
    click(&mut cx, CHROME_SIDE_PANEL_TOGGLE);
    for selector in [
        CHROME_SIDE_PANEL,
        CHROME_THEME_SETTINGS,
        PROBE_COMBOBOX,
        PROBE_COLOR_MODE,
        PROBE_ICON_THEME,
        CHROME_SIDE_PANEL_SEPARATOR,
        INSPECTOR_PANEL,
        INSPECTOR_TABS,
        CHROME_HANDLE,
    ] {
        assert_eq!(
            cx.debug_bounds(selector),
            None,
            "{selector} is still drawn with the side panel hidden"
        );
    }
    let viewport = cx.update(|window, _| window.viewport_size());
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    assert_eq!(
        (content.left(), content.size.width),
        (px(0.), viewport.width),
        "the content does not take the whole body with the side panel hidden"
    );
    assert_eq!(
        toggle_fill(&mut cx, CHROME_SIDE_PANEL_TOGGLE),
        None,
        "the side-panel toggle is still selected with the side panel hidden"
    );
    run_menu_item(&mut cx, "View", "Toggle Side Panel");
    assert_eq!(
        bounds_of(&mut cx, CHROME_SIDE_PANEL).size.width,
        shown.size.width,
        "View > Toggle Side Panel did not show the side panel at its width again"
    );
    for selector in [CHROME_THEME_SETTINGS, INSPECTOR_TABS] {
        assert!(
            cx.debug_bounds(selector).is_some(),
            "the side panel came back without {selector}"
        );
    }
    assert_eq!(
        toggle_fill(&mut cx, CHROME_SIDE_PANEL_TOGGLE),
        Some(selected),
        "the side-panel toggle is not selected again with the side panel shown"
    );
}

/// The presets the Sidebar samples' icons are checked under: every native
/// preset and one colour-scheme preset.
const SIDEBAR_ICON_PRESETS: [&str; 7] = [
    "kde-breeze",
    "adwaita",
    "macos-monterey",
    "macos-tahoe",
    "ios-13",
    "windows-11",
    "nord",
];

/// The Layout page's Sidebar samples' icons fit their items (spec S5),
/// expanded and in the collapsed sample's rail.
///
/// Upstream gives no hook to measure the drawn icon: `SidebarMenuItem`
/// places the `Icon` it is given itself (sidebar/menu.rs:321), and an `Icon`
/// builds its `svg()` with nothing of it reachable but its style
/// (icon.rs:175-183), so no debug selector gets there. What is checked:
///
/// - the Icon `demo::sidebar_icon_sized` hands an item has the size
///   `geometry::icon_size_small` gives, read off its style;
/// - in the rail, every item's measured bounds lie within the rail's width,
///   and no two items overlap. Neither catches an oversized icon on its
///   own: an item is as wide as the rail lets it be whatever its icon, and
///   grows taller rather than overlap. What does is the item's height:
///   there a row has no height of its own and holds its icon alone
///   (sidebar/menu.rs:322-329), inside `p_2` on every side (:305), so the
///   measured height less that padding is the icon's, and the icon, as
///   wide as it is tall (the first check), has to fit the item's measured
///   width;
/// - expanded, where every row is `h_7` whatever its icon
///   (sidebar/menu.rs:329), the icon is no taller than that: 1.75 rem at the
///   rem the `WindowState` root plugin installs, the theme's font size
///   (root.rs:714). The items'
///   measured heights are checked to be that row. The icon's is a model
///   check, not a measurement: an icon taller than its row overflows it
///   without moving any bound a test can read.
///
/// gpui-component's own icons are chosen, so every item has an icon to
/// check: with an icon theme that has none for an item, it shows nothing.
#[gpui::test]
fn the_sidebar_samples_icons_fit_their_items(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("gpui-component built-in (Lucide)", window, cx);
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    show(&mut cx, &showcase, Page::Layout);
    for preset in SIDEBAR_ICON_PRESETS {
        use_preset(&mut cx, &showcase, preset);
        let (small, rem) = read(&mut cx, &showcase, |_this, cx| {
            (
                native_value(cx, geometry::icon_size_small),
                gpui_component::ActiveTheme::theme(cx).font_size,
            )
        });
        let small = match small {
            Some(gpui_component::Size::Size(small)) => small,
            other => panic!("{preset}: geometry::icon_size_small gives {other:?}, not a size"),
        };
        let row = rems(1.75).to_pixels(rem);
        let expanded = bounds_of(&mut cx, LAYOUT_SIDEBAR_EXPANDED);
        let rail = bounds_of(&mut cx, LAYOUT_SIDEBAR_COLLAPSED);
        assert!(
            rail.size.width < expanded.size.width,
            "{preset}: the collapsed sample is {:?} wide, the expanded one {:?}",
            rail.size.width,
            expanded.size.width
        );
        for (label, icon, expanded_id, _) in LAYOUT_SIDEBAR_ITEMS {
            let mut icon = read(&mut cx, &showcase, |_this, cx| {
                crate::demo::sidebar_icon_sized(cx, gpui_component::Icon::new(icon))
            });
            let size = Styled::style(&mut icon).size.clone();
            assert_eq!(
                (size.width, size.height),
                (Some(small.into()), Some(small.into())),
                "{preset}: the {label} item's icon is not icon_size_small's {small:?}"
            );
            let Some(Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(height)))) =
                size.height
            else {
                panic!("{preset}: the {label} item's icon has no pixel height");
            };
            assert!(
                height <= row,
                "{preset}: the {label} item's icon is {height:?} tall, over the expanded row's h_7, {row:?}"
            );
            let item = bounds_of(&mut cx, expanded_id);
            assert!(
                within(item, expanded),
                "{preset}: the expanded {label} item at {item:?} is not in its Sidebar at {expanded:?}"
            );
            assert!(
                (item.size.height - row).abs() < px(1.),
                "{preset}: the expanded {label} item is {:?} tall, not h_7's {row:?}",
                item.size.height
            );
        }

        let items: Vec<(&str, Bounds<Pixels>)> = LAYOUT_SIDEBAR_ITEMS
            .into_iter()
            .map(|(label, _, _, collapsed_id)| (label, bounds_of(&mut cx, collapsed_id)))
            .collect();
        let padding = rems(0.5).to_pixels(rem);
        for (label, item) in &items {
            assert!(
                item.left() >= rail.left() && item.right() <= rail.right(),
                "{preset}: in the rail, the {label} item at {item:?} is wider than the rail at {rail:?}"
            );
            let icon = item.size.height - padding * 2.;
            assert!(
                icon <= item.size.width,
                "{preset}: in the rail, the {label} item is {:?} tall, so its icon is {icon:?}, \
                 wider than the item's {:?}",
                item.size.height,
                item.size.width
            );
        }
        for (i, (label, item)) in items.iter().enumerate() {
            for (other, next) in items.iter().skip(i + 1) {
                assert!(
                    !item.intersects(next),
                    "{preset}: in the rail, the {label} item at {item:?} overlaps the {other} item at {next:?}"
                );
            }
        }
    }
}

/// A Layout page Sidebar item drawn without its icon, then with it, then
/// without again, under presets of different text sizes, is laid out and
/// drawn each time: GPUI Fast 0.1.7 reclaimed the row's earlier nodes with
/// stale child lists and panicked prepainting the unmeasured label
/// (src/elements/text.rs:650), which gpui-fast 0.1.9 fixed and GPUI Kit
/// 0.8.1's floor of gpui-fast 0.1.12 includes. On any host: the icons are
/// removed by hand, not by an icon theme the host cannot read.
#[gpui::test]
fn a_sidebar_item_losing_and_regaining_its_icon_is_drawn(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let set = |cx: &mut VisualTestContext, icons: bool, preset: &str| {
        cx.update(|window, cx| {
            showcase.update(cx, |this, cx| {
                this.apply_theme_by_name(preset, window, cx);
                if icons {
                    this.select_icon_set("gpui-component built-in (Lucide)", window, cx);
                } else {
                    this.select_icon_set("Material (bundled)", window, cx);
                    this.gpui_icons.clear();
                }
                cx.notify();
            });
        });
        cx.run_until_parked();
        draw(cx);
        let shown = read(cx, &showcase, |this, _| {
            this.chrome_icon(&IconName::LayoutDashboard).shown()
        });
        assert_eq!(shown, icons, "{preset}: the Sidebar items' icons");
    };
    set(&mut cx, false, "macos-tahoe");
    show(&mut cx, &showcase, Page::Layout);
    set(&mut cx, true, "material-3");
    set(&mut cx, false, "ios-13");
}

/// The Layout page's two Sidebars and their items report themselves (spec
/// S5): the pointer on an item settles on that item's info, active or not,
/// and below the items, on the Sidebar's own.
#[gpui::test]
fn the_sidebar_samples_report_themselves(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    let [
        (first, _, first_expanded, first_collapsed),
        (second, _, _, second_collapsed),
        _,
    ] = LAYOUT_SIDEBAR_ITEMS;
    let (active, inactive) = (
        format!("SidebarMenuItem · {first}, active"),
        format!("SidebarMenuItem · {second}"),
    );
    settle_on_each(
        &mut cx,
        &showcase,
        &[
            (first_expanded, active.as_str()),
            (first_collapsed, active.as_str()),
            (second_collapsed, inactive.as_str()),
        ],
    );
    for (selector, title) in [
        (LAYOUT_SIDEBAR_EXPANDED, "Sidebar · expanded"),
        (LAYOUT_SIDEBAR_COLLAPSED, "Sidebar · collapsed"),
    ] {
        let sidebar = bounds_of(&mut cx, selector);
        // Below the last item, which the Sidebar's own ground fills.
        hover(
            &mut cx,
            point(sidebar.center().x, sidebar.bottom() - px(4.)),
        );
        settle(&mut cx);
        draw(&mut cx);
        assert_eq!(
            read(&mut cx, &showcase, |this, cx| {
                this.info_ui.read(cx).shown().map(|info| info.title())
            })
            .as_deref(),
            Some(title),
            "the pointer settled on {selector}'s ground, and the inspector does not show it"
        );
    }
}

/// The Icons page's Icon Sizes section shows an icon at every size
/// `defaults.icon_sizes` names: each laid out at what its builder gives,
/// and each reporting that builder and the field it reads. gpui-component's
/// own set is chosen, so the icon is there to measure.
///
/// A size platform-facts §2.1.8 documents none for on the installed native
/// preset's platform says it has no platform source: adwaita's dialog and
/// panel (platform-facts.md:1139-1140), and none of kde-breeze's. The preset
/// is named the way the preset Combobox names it, which `use_preset`
/// leaves alone. Under `default` the caveat is the preset's the desktop
/// theme is built on: `default` standing for adwaita, as it does on GNOME,
/// marks adwaita's dialog and panel too.
#[gpui::test]
fn the_icon_sizes_section_shows_every_icon_size(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("gpui-component built-in (Lucide)", window, cx);
        });
    });
    for (chosen, preset, unsourced) in [
        ("kde-breeze", "kde-breeze", &[][..]),
        (
            "adwaita",
            "adwaita",
            &[IconSizeContext::Dialog, IconSizeContext::Panel][..],
        ),
        (
            "default",
            "adwaita",
            &[IconSizeContext::Dialog, IconSizeContext::Panel][..],
        ),
    ] {
        use_preset(&mut cx, &showcase, preset);
        // After the install, which names the theme it installed: `default`
        // stands for the desktop's own theme built on `preset`.
        cx.update(|_window, cx| {
            showcase.update(cx, |this, _cx| {
                this.current_theme_name = chosen.to_string();
                this.default_preset = preset.to_string();
            });
        });
        let preset = if chosen == preset {
            preset.to_string()
        } else {
            format!("{chosen} ({preset})")
        };
        show(&mut cx, &showcase, Page::Icons);
        for context in IconSizeContext::ALL {
            let size = read(&mut cx, &showcase, |_this, cx| {
                native_value(cx, context.builder())
            });
            let Some(gpui_component::Size::Size(size)) = size else {
                panic!("{preset}: the {context:?} builder gives {size:?}, not a size");
            };
            let icon = bounds_of(&mut cx, context.icon_box());
            assert_eq!(
                (icon.size.width, icon.size.height),
                (size, size),
                "{preset}: the {context:?} icon is laid out at {:?}, its builder gives {size:?}",
                icon.size
            );
            let name = context.name();
            let info = settle_on(&mut cx, &showcase, context.cell());
            assert_eq!(
                info.as_ref().map(|i| i.title()),
                Some(format!("Icon · {name} size")),
                "{preset}: the pointer settled on the {name} cell, and the inspector does not show its info"
            );
            assert!(
                info.as_ref()
                    .and_then(|i| i.config.iter().find(|n| n.what == "geometry"))
                    .is_some_and(|g| g.text.starts_with(&format!("geometry::icon_size_{name}:"))),
                "{preset}: the {name} cell does not name geometry::icon_size_{name}: {info:?}"
            );
            assert!(
                info.as_ref()
                    .and_then(|i| i.instance.iter().find(|n| n.what == "field"))
                    .is_some_and(|f| f.text.contains(&format!("icon_sizes.{name}"))),
                "{preset}: the {name} cell does not name its model field: {info:?}"
            );
            let marked = info
                .as_ref()
                .and_then(|i| i.instance.iter().find(|n| n.what == "size"))
                .is_some_and(|n| {
                    n.text.contains("has no platform source")
                        && n.text.contains("native-theme's adwaita preset")
                });
            assert_eq!(
                marked,
                unsourced.contains(&context),
                "{preset}: the {name} cell's size line marks an unsourced number wrongly: {info:?}"
            );
        }
    }
}

/// The title the status bar's hover label drew in the last frame, and
/// whether that label was laid out inside the status bar.
fn status_title(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> Option<String> {
    let title = read(cx, showcase, |this, _| this.status_title_drawn.clone());
    let label = cx.debug_bounds(STATUS_HOVERED);
    assert_eq!(
        title.is_some(),
        label.is_some(),
        "the status bar drew {title:?}, and its hover label was laid out at {label:?}"
    );
    if let Some(label) = label {
        let bar = bounds_of(cx, CHROME_STATUS_BAR);
        assert!(
            bar.contains(&label.origin) && label.bottom() <= bar.bottom(),
            "the hover label at {label:?} is not inside the status bar at {bar:?}"
        );
    }
    title.map(|t| t.to_string())
}

/// The status bar names the widget whose info the inspector shows (spec
/// §2.7): a toolbar button, then a page's widget, and after a page change
/// the two still agree. gpui-component's own icons are chosen, so the
/// button has its icon whatever the desktop's set holds.
#[gpui::test]
fn the_status_bar_names_the_hovered_widget(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Buttons);
    assert_eq!(
        status_title(&mut cx, &showcase),
        None,
        "the status bar names a widget before anything was hovered"
    );
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("gpui-component built-in (Lucide)", window, cx)
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    let button = bounds_of(&mut cx, CHROME_TOOLBAR_PALETTE);
    hover(&mut cx, button.center());
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        status_title(&mut cx, &showcase).as_deref(),
        Some("Tool button · Normal"),
        "the status bar does not name the toolbar button the pointer settled on"
    );
    assert_eq!(
        status_title(&mut cx, &showcase),
        inspector_title(&mut cx, &showcase),
        "the status bar and the inspector name different widgets"
    );
    let clipboard = bounds_of(&mut cx, PROBE_CLIPBOARD).center();
    hover(&mut cx, clipboard);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("Clipboard"),
        "the inspector does not show the Clipboard the pointer settled on"
    );
    assert_eq!(
        status_title(&mut cx, &showcase).as_deref(),
        Some("Clipboard"),
        "the status bar does not name the page widget the inspector shows"
    );
    show(&mut cx, &showcase, Page::Inputs);
    assert_eq!(
        status_title(&mut cx, &showcase),
        inspector_title(&mut cx, &showcase),
        "after the page change the status bar and the inspector disagree"
    );
}

/// The status bar is the bottom of the window (spec §1.1): below the body,
/// across the whole width.
#[gpui::test]
fn the_status_bar_is_the_bottom_of_the_window(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let viewport = cx.update(|window, _| window.viewport_size());
    let bar = bounds_of(&mut cx, CHROME_STATUS_BAR);
    assert_eq!(
        bar.bottom(),
        viewport.height,
        "the status bar ends at {:?} in a window {:?} tall",
        bar.bottom(),
        viewport.height
    );
    assert_eq!(
        bar.left(),
        px(0.),
        "the status bar starts at {:?}",
        bar.left()
    );
    assert_eq!(
        bar.size.width, viewport.width,
        "the status bar is {:?} wide in a {:?} window",
        bar.size.width, viewport.width
    );
    assert!(bar.size.height > px(0.), "the status bar has no height");
    let body = bounds_of(&mut cx, CONTENT_PANEL);
    assert!(
        body.bottom() <= bar.top(),
        "the body ends at {:?}, under the status bar's top at {:?}",
        body.bottom(),
        bar.top()
    );
}

/// The status bar's first and last children are inset from its edges by the
/// padding it draws (spec §3.6), the reported defect. With nothing shown,
/// the last is the middle region, which runs to the bar's right end. Under kde-breeze that
/// is `status_bar.border`'s 2px left, Qt's status-bar item layout, and 14px
/// right, the size grip Breeze paints as nothing (platform-facts §2.14). The bar has no side border
/// (status_bar.rs:91, `border_t_1`), so its edges are its padding's.
#[gpui::test]
fn the_status_bars_ends_are_inset_by_its_padding(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let padding = read(&mut cx, &showcase, |_this, cx| {
        cx.native().map(|n| n.resolved.status_bar.border.padding)
    });
    let (left, right) = (padding.and_then(|p| p.left), padding.and_then(|p| p.right));
    assert_eq!(
        (left, right),
        (Some(2.0), Some(14.0)),
        "kde-breeze no longer states Qt's 2px left and the grip's 14px right, so this measures something else"
    );
    let bar = bounds_of(&mut cx, CHROME_STATUS_BAR);
    let (first, last) = (
        bounds_of(&mut cx, CHROME_SIDE_PANEL_TOGGLE),
        bounds_of(&mut cx, STATUS_MIDDLE),
    );
    assert_eq!(
        Some(first.left() - bar.left()),
        left.map(px),
        "the status bar's first child, the side-panel toggle, is not its left padding in"
    );
    assert_eq!(
        Some(bar.right() - last.right()),
        right.map(px),
        "the status bar's last child, its middle region, is not its right padding in"
    );
}

/// One pixel of the device the test window draws on, in logical pixels: how
/// far gpui may move an element when it puts it on the pixel grid.
fn device_pixel(cx: &mut VisualTestContext) -> Pixels {
    cx.update(|window, _| px(1. / window.scale_factor()))
}

/// The width gpui's text system gives `text` at the status bar's text size,
/// which `geometry::status_bar` sets from `status_bar.font`: the width the
/// bar draws that text at. `None` before a native theme is installed.
fn status_text_width(cx: &mut VisualTestContext, text: &str) -> Option<Pixels> {
    cx.update(|window, cx| {
        let size = native_geometry(cx, geometry::status_bar)?
            .text
            .font_size?
            .to_pixels(window.rem_size());
        let text = gpui::SharedString::from(text.to_string());
        let run = window.text_style().to_run(text.len());
        Some(
            window
                .text_system()
                .shape_line(text, size, &[run], None)
                .width(),
        )
    })
}

/// The status bar's items, read off the frame, end to end: the side-panel
/// toggle, the environment text, the empty middle region, and the shown
/// title where one is shown (spec S4), each next to the one before it by the
/// bar's `gap_2` (status_bar.rs:84, :88), the toggle and the environment by
/// `layout.widget_gap`, the last ending the bar's right padding in. So
/// nothing else is drawn between them, none overlaps the next, and they keep
/// their order. The environment draws `environment`, the text the showcase
/// names (`chrome::status_environment`), whole and as wide as gpui's text
/// system lays it out, or, where the bar leaves it too little room, a prefix
/// of it ended with an ellipsis, narrower; the shown title is as wide as its
/// title, so neither carries more. With nothing shown, and then with the
/// toolbar's Command Palette button's title shown.
fn the_status_bar_items_abut(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    environment: &str,
) {
    let right_padding = read(cx, showcase, |_this, cx| {
        cx.native()
            .and_then(|n| n.resolved.status_bar.border.padding.right)
    });
    assert!(
        right_padding.is_some(),
        "the preset states no right status-bar padding, so the bar's end is not known"
    );
    let gap = cx.update(|window, _| rems(0.5).to_pixels(window.rem_size()));
    // gpui places elements on the device's pixel grid, so a position may be
    // off by up to one device pixel.
    let slack = device_pixel(cx);
    let whole = status_text_width(cx, environment);
    assert!(whole.is_some(), "no status-bar text size is installed");

    for hovered in [None, Some(CHROME_TOOLBAR_PALETTE)] {
        if let Some(selector) = hovered {
            let at = bounds_of(cx, selector).center();
            hover(cx, at);
            settle(cx);
            draw(cx);
        }
        let bar = bounds_of(cx, CHROME_STATUS_BAR);
        let left = bounds_of(cx, CHROME_SIDE_PANEL_TOGGLE);
        let env = bounds_of(cx, STATUS_ENVIRONMENT);
        let middle = bounds_of(cx, STATUS_MIDDLE);
        let text = read(cx, showcase, |this, _| this.status_environment_drawn.text());
        if text == environment {
            assert_eq!(
                Some(env.size.width),
                whole,
                "the status bar's environment text is not the environment alone"
            );
        } else {
            assert!(
                text.strip_suffix('…')
                    .is_some_and(|kept| environment.starts_with(kept)),
                "the status bar draws {text:?} for the environment {environment:?}, neither \
                 the whole nor a prefix of it ended with an ellipsis"
            );
            assert!(
                whole.is_some_and(|whole| env.size.width < whole),
                "the status bar cuts its environment short at {:?}, no narrower than the \
                 whole {whole:?}",
                env.size.width
            );
        }
        let mut drawn = vec![
            ("side-panel toggle", left),
            ("environment", env),
            ("middle", middle),
        ];
        let shown = read(cx, showcase, |this, _| this.status_title_drawn.clone());
        assert_eq!(shown.is_some(), hovered.is_some());
        if let Some(title) = shown {
            let label = bounds_of(cx, STATUS_HOVERED);
            assert_eq!(
                Some(label.size.width),
                status_text_width(cx, &title),
                "the status bar's shown title is not {title:?} alone"
            );
            drawn.push(("shown title", label));
        }
        // The toggle and the environment are one item, `layout.widget_gap`
        // apart (demo::status_bar); the bar's regions stay `gap_2` apart.
        let widget_gap =
            read(cx, showcase, |this, _| geometry::widget_gap(&this.layout)).unwrap_or(px(0.));
        for (ix, pair) in drawn.windows(2).enumerate() {
            if let [(a, before), (b, after)] = pair {
                let expected = if ix == 0 { widget_gap } else { gap };
                assert!(
                    (after.left() - before.right() - expected).abs() <= slack,
                    "the status bar draws something between its {a} at {before:?} and its {b} \
                     at {after:?}, which are not {expected:?} apart"
                );
            }
        }
        let (name, last) = drawn.last().copied().unwrap_or(("middle", middle));
        assert_eq!(
            Some(bar.right() - last.right()),
            right_padding.map(px),
            "the status bar draws something after its {name}"
        );
    }
}

/// The status bar no longer carries the version (spec §3.2): the title bar
/// does. Its items abut end to end (`the_status_bar_items_abut`), so it
/// draws nothing beside the environment the showcase names, which does not
/// carry it either. Its info says the same.
#[gpui::test]
fn the_status_bar_carries_no_version(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let environment = read(&mut cx, &showcase, |this, cx| {
        crate::chrome::status_environment(this, cx).join(" · ")
    });
    assert!(
        !environment.contains(env!("CARGO_PKG_VERSION")),
        "the environment the showcase names carries the version: {environment}"
    );
    the_status_bar_items_abut(&mut cx, &showcase, &environment);

    let bar = bounds_of(&mut cx, CHROME_STATUS_BAR);
    hover(&mut cx, bar.center());
    settle(&mut cx);
    draw(&mut cx);
    let info = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().map(|info| (**info).clone())
    });
    assert_eq!(
        info.as_ref().map(|info| info.title()).as_deref(),
        Some("StatusBar")
    );
    let text = info.map(|info| info.to_text()).unwrap_or_default();
    assert!(
        !text.contains("version") && !text.contains(env!("CARGO_PKG_VERSION")),
        "the status bar's info still names a version:\n{text}"
    );
}

/// An environment wider than the status bar, as a host that lacks the
/// theme's font draws it ("<stated> (not on this system; drawn in the
/// platform's substitute, <drawn>)", `status::family_label`), is cut short
/// with an ellipsis in the room the toggle and the shown title leave it: the
/// items still abut in order and none overlaps the next
/// (`the_status_bar_items_abut`), and the bar's info carries the whole text.
/// The stated family here is one no host has, lengthened until the
/// environment is wider than the bar, whatever this host's fonts.
#[gpui::test]
fn an_environment_too_wide_for_the_status_bar_is_cut_short(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let bar = bounds_of(&mut cx, CHROME_STATUS_BAR);
    let mut family = String::from("Native Theme Absent Sans");
    let mut environment = String::new();
    for _ in 0..64 {
        cx.update(|_window, cx| {
            showcase.update(cx, |this, cx| {
                this.original_font.family = family.as_str().into();
                cx.notify();
            });
        });
        environment = read(&mut cx, &showcase, |this, cx| {
            crate::chrome::status_environment(this, cx).join(" · ")
        });
        if status_text_width(&mut cx, &environment).is_some_and(|w| w > bar.size.width) {
            break;
        }
        family.push_str(" Extended");
    }
    assert!(
        status_text_width(&mut cx, &environment).is_some_and(|w| w > bar.size.width),
        "the environment {environment:?} is no wider than the {:?} bar, so nothing is cut short",
        bar.size.width
    );
    assert!(
        environment.contains("(not on this system; drawn in the platform's substitute, "),
        "the environment does not name the substitute for {family:?}: {environment}"
    );
    draw(&mut cx);
    the_status_bar_items_abut(&mut cx, &showcase, &environment);
    let text = read(&mut cx, &showcase, |this, _| {
        this.status_environment_drawn.text()
    });
    assert!(
        text != environment && text.ends_with('…'),
        "the status bar does not cut its environment short: it draws {text:?}"
    );

    hover(&mut cx, bar.center());
    settle(&mut cx);
    draw(&mut cx);
    let info = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().map(|info| (**info).clone())
    });
    assert_eq!(
        info.as_ref().map(|info| info.title()).as_deref(),
        Some("StatusBar")
    );
    let text = info.map(|info| info.to_text()).unwrap_or_default();
    assert!(
        text.contains(&environment),
        "the status bar's info does not carry the whole environment {environment:?}:\n{text}"
    );
}

/// Where the chosen icon theme has no `PanelLeft`, the side-panel toggle
/// shows its tooltip's text as its label, never another icon theme's icon
/// (spec §3.2): with Material's taken out of the loaded gallery, the toggle
/// is as wide as its tooltip's text in `button.font` (a Small Button's
/// `text_sm`, sizing.rs:340, without a native theme) plus the theme's button
/// padding, or the `px_2` on either side (button/button.rs:697-699) where it
/// states none, and says why in its info.
#[cfg(feature = "material-icons")] // the bundled Material set it chooses
#[gpui::test]
fn a_panel_toggle_the_set_has_no_icon_for_is_labelled(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Material (bundled)", window, cx)
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    let cases = [(
        CHROME_SIDE_PANEL_TOGGLE,
        IconName::PanelLeft,
        "PanelLeft",
        "Toggle Side Panel",
    )];
    let mut iconic = Vec::new();
    for (selector, icon, name, _) in &cases {
        let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(icon));
        assert!(
            matches!(drawn, ChromeIcon::Loaded(n, _) if n == *name),
            "Material has no {name} in its gallery, so taking it out proves nothing: {drawn:?}"
        );
        iconic.push(bounds_of(&mut cx, selector));
    }
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            for entry in &mut this.gpui_icons {
                if entry.0 == "PanelLeft" {
                    entry.3 = None;
                }
            }
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    for ((selector, icon, name, tooltip), with_icon) in cases.into_iter().zip(iconic) {
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.chrome_icon(&icon)),
            ChromeIcon::Missing(name)
        );
        let labelled = bounds_of(&mut cx, selector);
        // In `button.font`'s size and weight where a native theme is
        // installed (demo::labelled), else in a Small Button's `text_sm`.
        let font = read(&mut cx, &showcase, |_this, cx| {
            native_value(cx, |n| {
                let f = &n.resolved.button.font;
                (
                    px(native_theme_gpui::scaled_text_size(f.size, n.accessibility)),
                    gpui::FontWeight(f32::from(f.weight)),
                )
            })
        });
        let expected = cx.update(|window, _| {
            let rem = window.rem_size();
            let text = gpui::SharedString::from(tooltip);
            let mut style = window.text_style();
            let size = match font {
                Some((size, weight)) => {
                    style.font_weight = weight;
                    size
                }
                None => rems(0.875).to_pixels(rem),
            };
            let run = style.to_run(text.len());
            window
                .text_system()
                .shape_line(text, size, &[run], None)
                .width()
        });
        // Padded by the theme's button padding where it states a side, and
        // by the Small Button's own `px_2` where not (demo::tool_button_box).
        let sides = cx.update(|_, cx| {
            native_value(cx, |n| {
                let p = &n.resolved.button.border.padding;
                (p.left, p.right)
            })
        });
        let upstream = cx.update(|window, _| rems(0.5).to_pixels(window.rem_size()));
        let side = |v: Option<f32>| v.map_or(upstream, px);
        let expected = match sides {
            Some((left, right)) => expected + side(left) + side(right),
            None => expected + upstream * 2.,
        };
        // gpui places elements on the device's pixel grid.
        assert!(
            (labelled.size.width - expected).abs() <= device_pixel(&mut cx),
            "{selector} is {:?} wide, not its tooltip text {tooltip:?} as a label, {expected:?}",
            labelled.size.width
        );
        assert!(
            labelled.size.width > with_icon.size.width,
            "{selector} at {labelled:?} is no wider than with its icon at {with_icon:?}"
        );
        let info = settle_on(&mut cx, &showcase, selector);
        assert_eq!(
            info.as_ref().map(|info| info.title()).as_deref(),
            Some("Button · Ghost, labelled, selected"),
            "{selector} with no icon of the chosen set is not labelled"
        );
        let note = info.as_ref().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "icon")
                .map(|n| n.text.clone())
        });
        assert!(
            note.as_deref()
                .is_some_and(|n| n.starts_with(&format!("none: material holds no SVG for {name}"))),
            "{selector} does not say material has no {name}: {note:?}"
        );
    }
}

/// The boxes painted in the tooltip's fill in the last frame: upstream fills
/// a tooltip with `popover` (tooltip.rs:118-119), and the connector's
/// `geometry::tooltip` refinement repaints it in `tooltip.background_color`.
fn tooltip_boxes(cx: &mut VisualTestContext) -> Vec<Bounds<gpui::ScaledPixels>> {
    cx.update(|window, cx| {
        let fill = native_color(cx, |n| n.resolved.tooltip.background_color);
        scene_quads(window)
            .into_iter()
            .filter(|q| fill.is_some() && q.background.as_solid() == fill)
            .map(|q| q.bounds)
            .collect()
    })
}

/// The width of the tooltip that shows once the pointer rests on the
/// element tagged `selector` past gpui's tooltip delay (gpui-pre
/// elements/div.rs:53, 500ms): the widest box painted in the tooltip's
/// `tooltip.background_color` fill that was not painted before. `None`
/// where no such box is painted.
fn tooltip_width(cx: &mut VisualTestContext, selector: &'static str) -> Option<Pixels> {
    // Off any widget first, so a tooltip shown before is gone.
    let middle = bounds_of(cx, STATUS_MIDDLE).center();
    hover(cx, middle);
    cx.executor()
        .advance_clock(std::time::Duration::from_secs(1));
    draw(cx);
    let before = tooltip_boxes(cx);
    let at = bounds_of(cx, selector).center();
    hover(cx, at);
    cx.executor()
        .advance_clock(std::time::Duration::from_secs(1));
    cx.run_until_parked();
    draw(cx);
    let scale = cx.update(|window, _| window.scale_factor());
    tooltip_boxes(cx)
        .into_iter()
        .filter(|b| !before.contains(b))
        .map(|b| px(b.size.width.0 / scale))
        .max_by(|a, b| a.as_f32().total_cmp(&b.as_f32()))
}

/// The side-panel toggle's tooltip names its key binding (spec S4): the
/// action is bound to Ctrl+B (Cmd+B on macOS), and the tooltip is wider while the action has
/// that binding than once the bindings are gone -- upstream's Tooltip adds
/// the binding's Kbd beside the text only where the action has one
/// (tooltip.rs:99-111, :138-146). The tooltip's text is not readable from
/// the test's frame, so its width is what shows the Kbd.
#[gpui::test]
fn the_side_panel_toggles_tooltip_names_its_key(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    // A tooltip fades in (tooltip.rs:183-186); without motion it is drawn
    // in full at once.
    without_motion(&mut cx);
    let os = cx.update(|_window, cx| cx.platform_info().os);
    let cases: [(&'static str, Box<dyn gpui::Action>, String); 1] = [(
        CHROME_SIDE_PANEL_TOGGLE,
        Box::new(ToggleSidePanel),
        secondary("b", os),
    )];
    let mut bound = Vec::new();
    for (selector, action, keys) in &cases {
        let binding = cx.update(|window, _| {
            window
                .highest_precedence_binding_for_action(action.as_ref())
                .map(|b| {
                    b.keystrokes()
                        .iter()
                        .map(|k| k.unparse(os))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
        });
        assert_eq!(
            binding.as_deref(),
            Some(keys.as_str()),
            "{selector}'s action is not bound to {keys}"
        );
        let width = tooltip_width(&mut cx, selector);
        assert!(width.is_some(), "{selector} showed no tooltip");
        bound.push(width);
    }
    cx.update(|_window, cx| cx.clear_key_bindings());
    for ((selector, _, keys), with) in cases.iter().zip(bound) {
        let without = tooltip_width(&mut cx, selector);
        assert!(
            without.is_some() && with > without,
            "{selector}'s tooltip is {with:?} wide with {keys} bound and {without:?} without, \
             so it does not name the binding"
        );
    }
}

/// The window's bars report themselves (spec §4.3.5): the pointer on an
/// empty stretch of each, clear of the widgets it holds, settles on the
/// bar's own info. In this order a bar that reported nothing would leave the
/// previous bar's info on show, and fail as surely as the first.
///
/// Under the window manager's frame the bars are the toolbar, the menu-bar
/// row (not on macOS) and the status bar; granted client-side decorations,
/// the TitleBar is one too (spec S8).
#[gpui::test]
fn the_chrome_bars_report_themselves(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let toolbar = bounds_of(&mut cx, CHROME_TOOLBAR);
    let status_bar = bounds_of(&mut cx, CHROME_STATUS_BAR);
    let mut cases = vec![
        // The toolbar's items are packed at its start; its end is the row.
        // Elements of docs/showcase-elements.toml, titled as the list names
        // them.
        (
            "Toolbar · Normal",
            point(toolbar.right() - px(8.), toolbar.center().y),
        ),
        // The middle region, which holds no item of this bar's.
        ("Status bar · Normal", status_bar.center()),
    ];
    if cfg!(not(target_os = "macos")) {
        let row = bounds_of(&mut cx, CHROME_MENU_BAR);
        let menus = bounds_of(&mut cx, CHROME_APP_MENU_BAR);
        // The menus are packed at the row's start; its end is the row.
        let at = point(row.right() - px(8.), row.center().y);
        assert!(
            menus.right() < at.x,
            "the menu bar at {menus:?} reaches the menu-bar row's end at {row:?}"
        );
        cases.insert(1, ("Menu bar · Normal", at));
    }
    for (title, at) in cases {
        hover(&mut cx, at);
        settle(&mut cx);
        draw(&mut cx);
        assert_eq!(
            inspector_title(&mut cx, &showcase).as_deref(),
            Some(title),
            "the pointer at {at:?} settled, and the inspector does not show the {title}"
        );
    }

    grant(&mut cx, &showcase, CLIENT_SIDE);
    let title_bar = bounds_of(&mut cx, CHROME_TITLE_BAR);
    if cfg!(not(target_os = "macos")) {
        // The title bar's middle is empty only while the menus sit right of it.
        let menus = bounds_of(&mut cx, CHROME_APP_MENU_BAR);
        assert!(
            menus.left() > title_bar.center().x,
            "the menu bar at {menus:?} reaches the title bar's middle"
        );
    }
    // Between the label at the start and the menus at the end.
    hover(&mut cx, title_bar.center());
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("TitleBar"),
        "the pointer at the title bar's middle settled, and the inspector does not show the TitleBar"
    );
}

/// Two Buttons of different variants show different infos (spec §4.3.2):
/// the pointer settled on the Primary Button shows the Primary's info, and
/// moved to the Danger Button beside it, the Danger's.
#[gpui::test]
fn two_buttons_of_different_variants_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Buttons);
    let mut texts = Vec::new();
    for (selector, title) in [
        (BUTTONS_PRIMARY, "Button · Primary"),
        (BUTTONS_DANGER, "Button · Danger"),
    ] {
        let at = bounds_of(&mut cx, selector).center();
        hover(&mut cx, at);
        settle(&mut cx);
        draw(&mut cx);
        assert_eq!(
            inspector_title(&mut cx, &showcase).as_deref(),
            Some(title),
            "the pointer settled on {selector}, and the inspector does not show its info"
        );
        texts.push(read(&mut cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().map(|info| info.to_text())
        }));
    }
    assert!(
        texts.first() != texts.get(1),
        "the Primary and the Danger Button show the same info: {texts:?}"
    );
}

/// Settle the pointer on the element tagged `selector` and return the info
/// the inspector then shows.
fn settle_on(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    selector: &'static str,
) -> Option<WidgetInfo> {
    let at = bounds_of(cx, selector).center();
    settle_at(cx, showcase, at)
}

/// Settle the pointer at `at` and return the info the inspector then shows:
/// [`settle_on`] for a target whose centre another target covers.
fn settle_at(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    at: Point<Pixels>,
) -> Option<WidgetInfo> {
    hover(cx, at);
    settle(cx);
    draw(cx);
    read(cx, showcase, |this, cx| {
        this.info_ui.read(cx).shown().map(|info| (**info).clone())
    })
}

/// A swatch shows the colour upstream paints, not the token it dims: the
/// Text Button's label is foreground at 90% (button/button.rs:1074). A text
/// colour is not a quad of the scene, so the swatch is checked against that
/// line, not against the frame.
#[gpui::test]
fn the_text_buttons_text_swatch_is_foreground_at_90_percent(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Buttons);
    let info = settle_on(&mut cx, &showcase, BUTTONS_TEXT);
    let painted = cx.update(|_window, cx| Theme::global(cx).foreground.opacity(0.9));
    let text = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role.starts_with("text")));
    assert_eq!(
        text.map(|c| c.value),
        Some(painted),
        "the Text Button's text swatch is not foreground at 90%: {info:?}"
    );
}

/// The disabled row's Secondary Button is a Secondary Button, and says so.
#[gpui::test]
fn the_disabled_secondary_button_is_secondary(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Buttons);
    let info = settle_on(&mut cx, &showcase, BUTTONS_DISABLED_SECONDARY);
    assert_eq!(
        info.map(|info| info.title()).as_deref(),
        Some("Button · Secondary, disabled")
    );
}

/// A section heading is page text, and reports itself as a Label (spec
/// §5.3).
#[gpui::test]
fn a_section_heading_reports_itself(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Buttons);
    let info = settle_on(&mut cx, &showcase, BUTTONS_HEADING_VARIANTS);
    assert_eq!(
        info.map(|info| info.title()).as_deref(),
        Some("Label · heading")
    );
}

/// The Basic page's body text is laid out in the theme's line box,
/// `defaults.line_height` times the text size, where a Label's own is
/// `rems(1.25)`.
#[gpui::test]
fn basic_body_text_takes_the_themes_line_height(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let expected = read(&mut cx, &showcase, |_this, cx| {
        cx.native()
            .map(|n| n.resolved.defaults.font.size * n.resolved.defaults.line_height)
    });
    let body = bounds_of(&mut cx, "basic-type-body").size.height.as_f32();
    // gpui lays a text line out on whole pixels; upstream's own would be
    // 16.67px, a pixel less.
    assert!(
        expected.is_some_and(|e| (body - e).abs() < 0.5),
        "body text is {body}px tall, the theme's line box is {expected:?}"
    );
}

/// The Basic page's tooltip is built by the application and refined by
/// `geometry::tooltip`, so it takes the platform's padding, radius and
/// colours, as the egui Basic page's tooltip does. The iced one takes the
/// padding only where the theme states all four sides alike: iced's
/// `Tooltip::padding` is one number.
#[gpui::test]
fn the_basic_tooltip_is_refined(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let info = settle_on(&mut cx, &showcase, "basic-button-tooltip");
    assert_eq!(
        info.as_ref().map(|i| i.title()).as_deref(),
        Some("Tooltip · built by the application")
    );
    assert!(
        info.as_ref()
            .is_some_and(|i| i.to_text().contains("geometry::tooltip:")),
        "the Basic tooltip names no geometry::tooltip: {info:?}"
    );
}

/// A section heading is set in the theme's section-heading role, as the iced
/// showcase's section titles are: its info says so, and under kde-breeze,
/// whose role is 16px (12pt) against a 13.33px body, the heading is taller
/// than a body-text label.
#[gpui::test]
fn a_section_heading_takes_the_section_heading_role(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let info = settle_on(&mut cx, &showcase, "basic-heading-buttons");
    assert!(
        info.as_ref()
            .is_some_and(|i| i.to_text().contains("text_scale.section_heading")),
        "the heading's info names no text_scale.section_heading: {info:?}"
    );
    let heading = bounds_of(&mut cx, "basic-heading-buttons");
    let body = bounds_of(&mut cx, "basic-type-body");
    assert!(
        heading.size.height > body.size.height,
        "the heading ({heading:?}) is no taller than body text ({body:?})"
    );
}

/// The Basic page's column rule, the iced and egui showcases' too: under a
/// preset whose widest fixed-width control fits a fifth of the page, five
/// columns; under material-3, whose drop-down is 210 wide, four; on a
/// narrower page two or one; and each arrangement, five columns to one,
/// holds every group once, a column's groups in reading order.
#[test]
fn the_basic_page_counts_its_columns_from_the_theme() {
    use crate::pages::basic::{
        BASIC_FIVE, BASIC_FOUR, BASIC_ONE, BASIC_THREE, BASIC_TWO, basic_arrangement,
        basic_column_count,
    };
    let resolved = |preset: &str| {
        native_theme::theme::Theme::preset(preset)
            .and_then(|t| t.into_variant(native_theme::theme::ColorMode::Light))
            .and_then(|v| v.into_resolved(&native_theme::ResolutionContext::for_tests()))
            .map_err(|e| format!("{preset}: {e}"))
    };
    let layout = |preset: &str| {
        native_theme::theme::Theme::preset(preset)
            .map(|t| t.layout)
            .unwrap_or_default()
    };
    // The content panel beside the 300px side panel and a 1px splitter.
    for (preset, content, columns) in [
        ("kde-breeze", 979.0, 5),
        ("adwaita", 979.0, 5),
        ("catppuccin-mocha", 976.0, 5),
        ("material-3", 976.0, 4),
    ] {
        let r = resolved(preset);
        assert!(r.is_ok(), "{r:?}");
        if let Ok(r) = r {
            assert_eq!(
                basic_column_count(&r, &layout(preset), content, 0.0),
                columns,
                "{preset}"
            );
        }
    }
    let r = resolved("kde-breeze");
    if let Ok(r) = r {
        // Below three columns where three do not hold the widest control
        // (item 15(1) of the round 2-3 review), down to one.
        assert_eq!(basic_column_count(&r, &layout("kde-breeze"), 400.0, 0.0), 2);
        assert_eq!(basic_column_count(&r, &layout("kde-breeze"), 250.0, 0.0), 1);
        // A Segmented control wider than a fifth of the page drops a column
        // (M9-166).
        assert_eq!(
            basic_column_count(&r, &layout("kde-breeze"), 979.0, 200.0),
            4
        );
    }
    for (count, arrangement) in [
        (5, &BASIC_FIVE[..]),
        (4, &BASIC_FOUR[..]),
        (3, &BASIC_THREE[..]),
        (2, &BASIC_TWO[..]),
        (1, &BASIC_ONE[..]),
    ] {
        assert_eq!(basic_arrangement(count), arrangement);
        assert_eq!(arrangement.len(), count);
        let mut all: Vec<usize> = arrangement.iter().flat_map(|c| c.iter().copied()).collect();
        for column in arrangement {
            assert!(
                column.windows(2).all(|w| w.first() < w.get(1)),
                "{count} columns: {column:?} is not in reading order"
            );
        }
        all.sort_unstable();
        assert_eq!(all, (0..20).collect::<Vec<_>>(), "{count} columns");
    }
}

/// The `windows-11` theme, light or, with `is_dark`, dark, resolved as the
/// connector resolves a preset, with `tab.active_outward_corner_radius` set
/// on it to 4, the value the preset states (G3 T15), so the tests hold
/// spec §12.2's numbers whatever the preset says; a resolved theme's fields
/// are public.
#[cfg(feature = "widgets")]
fn windows_11_curving_out(is_dark: bool) -> (Theme, native_theme_gpui::ResolvedTheme) {
    let prefs = native_theme_gpui::AccessibilityPreferences::default();
    let (theme, mut r) =
        native_theme_gpui::from_preset("windows-11", is_dark, &prefs).expect("windows-11 loads");
    r.tab.active_outward_corner_radius = Some(4.0);
    (theme, r)
}

/// The Basic page's column rule makes room for both of the selected tab's
/// flares: under windows-11, whose widest fixed-width control is the two
/// tabs, a page whose fifth holds the two tabs and 4px more lays out five
/// columns, and four once the theme states an outward corner radius of 4,
/// whose two flares need 8; five again for a radius `widgets::TabBar` draws
/// no flares for, 0, negative or not finite.
#[cfg(feature = "widgets")]
#[test]
fn the_basic_page_makes_room_for_both_flares() {
    use crate::pages::basic::basic_column_count;
    let (_, stated) = windows_11_curving_out(false);
    let mut unstated = stated.clone();
    unstated.tab.active_outward_corner_radius = None;
    let layout = native_theme::theme::Theme::preset("windows-11")
        .map(|t| t.layout)
        .unwrap_or_default();
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
        basic_column_count(&unstated, &layout, content, 0.0),
        5,
        "without outward corners the two tabs fit a fifth"
    );
    assert_eq!(
        basic_column_count(&stated, &layout, content, 0.0),
        4,
        "the two tabs and both flares do not fit a fifth"
    );
    // A radius the bar does not take draws no flares, and takes no room.
    for radius in [0.0, -1.0, f32::NAN] {
        let mut undrawn = stated.clone();
        undrawn.tab.active_outward_corner_radius = Some(radius);
        assert_eq!(
            basic_column_count(&undrawn, &layout, content, 0.0),
            5,
            "a radius of {radius} keeps room for flares the bar does not draw"
        );
    }
}

/// The Basic page lays its groups out as the three showcases do: under
/// kde-breeze five columns, under material-3 four, left to right, each a stack
/// of its groups in order, every group its heading over its controls, and
/// every control the page shows drawn.
#[gpui::test]
fn the_basic_page_holds_every_group_in_its_column(cx: &mut TestAppContext) {
    use crate::pages::basic::{BASIC_GROUPS, basic_arrangement};
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for (preset, count) in [("material-3", 4), ("kde-breeze", 5)] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Basic);
        let mut previous_left = None;
        for column in basic_arrangement(count) {
            let mut above: Option<Bounds<Pixels>> = None;
            let mut first = None;
            for &(id, text) in column.iter().filter_map(|&ix| BASIC_GROUPS.get(ix)) {
                let heading = bounds_of(&mut cx, id);
                if let Some(above) = above {
                    assert!(
                        heading.top() > above.bottom(),
                        "{preset}: {text} is not below the group above it"
                    );
                    assert_eq!(
                        heading.left(),
                        above.left(),
                        "{preset}: {text} does not start where its column does"
                    );
                }
                first.get_or_insert(heading.left());
                above = Some(heading);
            }
            assert!(
                previous_left < first,
                "{preset}: a column is not right of the one before"
            );
            previous_left = first;
        }
    }
    let names: Vec<&str> = BASIC_GROUPS.iter().map(|&(_, text)| text).collect();
    assert_eq!(
        names,
        [
            "Buttons",
            "Checkboxes",
            "Radio buttons",
            "Drop-down",
            "Text inputs",
            "Text area",
            "Slider",
            "Switches",
            "Number input",
            "Spinner",
            "Segmented control",
            "Card",
            "Typography",
            "Separator",
            "Progress bar",
            "List",
            "Icons",
            "Tabs",
            "Expander",
            "Table",
        ],
        "the Basic page's groups are not the three showcases' groups"
    );
    for control in [
        "basic-button",
        "basic-button-primary",
        "basic-button-disabled",
        "basic-button-tooltip",
        "basic-checkbox-unchecked",
        "basic-checkbox-checked",
        "basic-checkbox-disabled",
        "basic-radio",
        "basic-switch-off",
        "basic-switch-on",
        "basic-switch-disabled",
        "basic-input-placeholder",
        "basic-input-filled",
        crate::BASIC_INPUT_DISABLED,
        "basic-textarea",
        "basic-toggle-off",
        "basic-toggle-on",
        "basic-input-focused",
        "basic-select",
        "basic-type-body",
        "basic-link",
        "basic-slider",
        "basic-progress",
        "basic-spinner",
        "basic-tabs",
        "basic-segmented",
        "basic-icon-copy",
        "basic-list",
        "basic-expander",
        "basic-card",
        "basic-separator",
        "basic-table",
    ] {
        let bounds = bounds_of(&mut cx, control);
        assert!(
            bounds.size.width > px(0.) && bounds.size.height > px(0.),
            "{control} is drawn with no size: {bounds:?}"
        );
    }
    // Stacked one control to a row, as the other two showcases stack them.
    let first = bounds_of(&mut cx, "basic-switch-off");
    let second = bounds_of(&mut cx, "basic-switch-on");
    assert!(
        second.top() >= first.bottom(),
        "the switches are not one to a row"
    );
}

/// Under a native theme the Basic page's checkboxes, radios, switches,
/// slider, progress bar and spinner are the connector's theme-drawn widgets
/// (docs/archive/todo_gpui-widgets-spec.md), each at the sizes the theme states,
/// under two presets whose sizes all differ.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_basic_controls_take_the_themes_sizes(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for preset in ["kde-breeze", "material-3"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Basic);
        let r = read(&mut cx, &showcase, |_, cx| {
            cx.native_theme().and_then(|nt| nt.resolved(cx)).cloned()
        });
        assert!(r.is_some(), "{preset}: no native theme is installed");
        let Some(r) = r else { return };
        let size =
            |cx: &mut VisualTestContext, selector: &'static str| bounds_of(cx, selector).size;
        // The last indicator the page lays out, a radio button's.
        let indicator = size(&mut cx, "native-checkbox-indicator");
        let radio = px(r
            .checkbox
            .radio_indicator_width
            .unwrap_or(r.checkbox.indicator_width));
        assert_eq!(
            (indicator.width, indicator.height),
            (radio, radio),
            "{preset}: checkbox.radio_indicator_width, or indicator_width"
        );
        let track = size(&mut cx, "native-switch-track");
        assert_eq!(
            (track.width, track.height),
            (px(r.switch.track_width), px(r.switch.track_height)),
            "{preset}: switch.track_width × track_height"
        );
        let thumb = size(&mut cx, "native-switch-thumb");
        assert_eq!(
            thumb.width,
            px(r.switch.thumb_diameter),
            "{preset}: switch.thumb_diameter"
        );
        let rail = size(&mut cx, "native-slider-rail");
        assert_eq!(
            rail.height,
            px(r.slider.track_height),
            "{preset}: slider.track_height"
        );
        let knob = size(&mut cx, "native-slider-thumb");
        assert_eq!(
            knob.width,
            px(r.slider.thumb_diameter),
            "{preset}: slider.thumb_diameter"
        );
        let bar = size(&mut cx, "native-progress");
        assert_eq!(
            bar.height,
            px(r.progress_bar.track_height),
            "{preset}: progress_bar.track_height"
        );
        let spinner = size(&mut cx, "native-spinner");
        assert_eq!(
            spinner.width,
            px(r.spinner.diameter.max(r.spinner.min_diameter)),
            "{preset}: spinner.diameter"
        );
    }
}

/// Under a native theme the Feedback page's Small, Medium and Large Spinners
/// are all the connector's theme-drawn widget, one look at three sizes:
/// `SpinnerLook::of_size`'s diameters, which strictly increase.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_feedback_spinners_are_the_platforms_at_three_sizes(cx: &mut TestAppContext) {
    use crate::demo::SpinnerKind;
    use native_theme_gpui::widgets::SpinnerLook;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    for preset in ["kde-breeze", "material-3"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Feedback);
        let r = read(&mut cx, &showcase, |_, cx| {
            cx.native_theme().and_then(|nt| nt.resolved(cx)).cloned()
        });
        assert!(r.is_some(), "{preset}: no native theme is installed");
        let Some(r) = r else { return };
        let mut widths = Vec::new();
        for (id, kind) in [
            (FEEDBACK_SPINNER_SMALL, SpinnerKind::Small),
            ("feedback-spinner-medium", SpinnerKind::Medium),
            ("feedback-spinner-large", SpinnerKind::Large),
        ] {
            let width = bounds_of(&mut cx, id).size.width;
            let look = SpinnerLook::of_size(&r, kind.size());
            assert_eq!(
                Some(width),
                look.map(|look| look.diameter),
                "{preset}: {id} is not SpinnerLook::of_size's diameter"
            );
            let info = settle_on(&mut cx, &showcase, id);
            assert!(
                info.as_ref()
                    .is_some_and(|i| i.to_text().contains("native_theme_gpui::widgets::Spinner")),
                "{preset}: {id} is not the theme-drawn widgets::Spinner: {info:?}"
            );
            widths.push(width);
        }
        assert!(
            widths.windows(2).all(|w| w[0] < w[1]),
            "{preset}: the spinners' widths do not increase: {widths:?}"
        );
    }
}

/// The Basic List shows three of its eight rows: its box is three rows and
/// its frame tall, so the rest scroll.
#[gpui::test]
fn the_basic_list_shows_three_rows(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let list = bounds_of(&mut cx, "basic-list");
    let row = bounds_of(&mut cx, "basic-list-row-0");
    let frame = read(&mut cx, &showcase, |_this, cx| {
        native_value(cx, |n| n.resolved.list.border.line_width)
    })
    .map(px);
    assert!(frame.is_some(), "no native theme is installed");
    let frame = frame.unwrap_or_default();
    assert_eq!(
        list.size.height,
        row.size.height * 3. + frame + frame,
        "the List box is not three rows ({row:?}) and its frame ({frame:?}) tall"
    );
    // Item 2 is selected, its text in the list's selection text colour.
    let info = settle_on(&mut cx, &showcase, "basic-list-row-1");
    assert_eq!(
        info.as_ref().map(|i| i.title()).as_deref(),
        Some("ListItem · Item 2, selected")
    );
    assert!(
        info.as_ref()
            .is_some_and(|i| i.to_text().contains("list.selection_text_color")),
        "the selected row's text is not list.selection_text_color: {info:?}"
    );
}

/// The Basic page never needs sideways scrolling under the Linux presets the
/// captures take, in both modes, and under every bundled preset at a text
/// scale of 2.25 (round 2-3 review, item 15(1)): every control ends inside
/// the scroll area, and the page lays out the columns its rule counts from
/// the content panel's width. It may be taller than the scroll area
/// (material-3 is), and then scrolls, at rest at the top; the test reports
/// by how much.
#[gpui::test]
fn the_basic_page_fits_the_window_across(cx: &mut TestAppContext) {
    use crate::pages::basic::basic_column_count;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for preset in ["kde-breeze", "material-3", "catppuccin-mocha", "adwaita"] {
        use_preset(&mut cx, &showcase, preset);
        for mode in ["Light", "Dark"] {
            run_menu_item(&mut cx, "Theme", mode);
            show(&mut cx, &showcase, Page::Basic);
            let page = bounds_of(&mut cx, PAGE_ROOT);
            // The scroll area: what the content panel shows of the page.
            let viewport = read(&mut cx, &showcase, |this, _| this.content_scroll.bounds());
            let content = bounds_of(&mut cx, CONTENT_PANEL);
            let computed = cx.update(|window, cx| showcase.read(cx).content_width(window, cx));
            assert_eq!(
                computed, content.size.width,
                "{preset} {mode}: the content width the page counts columns from is not the panel's"
            );
            let count = cx.update(|window, cx| {
                let layout = showcase.read(cx).layout.clone();
                native_value(cx, |n| {
                    let segmented = crate::demo::segmented_width(
                        window,
                        &n,
                        &crate::pages::basic::BASIC_SEGMENTS,
                    );
                    basic_column_count(n.resolved, &layout, f32::from(computed), segmented)
                })
            });
            let drawn = read(&mut cx, &showcase, |this, cx| {
                this.info_ui.read(cx).layout_drawn()
            });
            let columns = drawn
                .keys()
                .filter(|id| id.starts_with("basic.column_"))
                .count();
            assert_eq!(
                Some(columns),
                count,
                "{preset} {mode}: the columns laid out"
            );
            for (id, bounds) in drawn.iter().filter(|(id, _)| id.starts_with("basic.")) {
                assert!(
                    bounds.right() <= viewport.right(),
                    "{preset} {mode}: {id} ends at {:?}, past the scroll area's {:?}",
                    bounds.right(),
                    viewport.right()
                );
            }
            println!(
                "{preset} {mode}: {columns} columns; page {:?} in scroll area {:?}",
                page.size, viewport.size
            );
            if page.size.height > viewport.size.height {
                println!(
                    "{preset} {mode}: the Basic page is {}px tall in a {}px scroll area: it scrolls",
                    page.size.height.as_f32(),
                    viewport.size.height.as_f32()
                );
                assert_eq!(
                    page.top(),
                    viewport.top(),
                    "{preset} {mode}: the page is not at rest at the top"
                );
            }
        }
    }
    let mut faults = Vec::new();
    for preset in native_theme::theme::Theme::list_presets()
        .iter()
        .map(|p| p.key)
    {
        use_preset_scaled(&mut cx, &showcase, preset, Some(96.0), 2.25);
        show(&mut cx, &showcase, Page::Basic);
        let viewport = read(&mut cx, &showcase, |this, _| this.content_scroll.bounds());
        let content = bounds_of(&mut cx, CONTENT_PANEL);
        let count = cx.update(|window, cx| {
            let layout = showcase.read(cx).layout.clone();
            native_value(cx, |n| {
                let segmented =
                    crate::demo::segmented_width(window, &n, &crate::pages::basic::BASIC_SEGMENTS);
                basic_column_count(n.resolved, &layout, content.size.width.as_f32(), segmented)
            })
        });
        let drawn = read(&mut cx, &showcase, |this, cx| {
            this.info_ui.read(cx).layout_drawn()
        });
        let columns = drawn
            .keys()
            .filter(|id| id.starts_with("basic.column_"))
            .count();
        if Some(columns) != count {
            faults.push(format!(
                "{preset} at 2.25: {columns} columns laid out, the rule counts {count:?}"
            ));
        }
        for (id, bounds) in drawn.iter().filter(|(id, _)| id.starts_with("basic.")) {
            if bounds.right() > viewport.right() + px(0.5) {
                faults.push(format!(
                    "{preset} at 2.25: {id} ends at {:?}, past the scroll area's {:?}",
                    bounds.right(),
                    viewport.right()
                ));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// Keys typed while a capture runs reach the window the desktop gives the
/// focus, and changed what a capture recorded (the Basic page's focused field
/// read "Focusedones?" in the iced showcase): under `--capture`,
/// `--screenshot` and `--dump-layout` the window takes no keystroke
/// (`Showcase::keyboard_shield`, whose doc names the input-method commits it
/// cannot stop). Text typed into the Basic page's filled
/// field and the side-panel toggle's binding change nothing; the same keys
/// in a run that neither captures nor dumps type into the field and hide the
/// panel, which is what makes the first half mean something.
#[gpui::test]
fn a_captured_or_dumped_window_takes_no_keyboard_input(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    show(&mut cx, &showcase, Page::Basic);
    let runs = [
        (
            "--capture",
            crate::CliArgs {
                capture: true,
                ..Default::default()
            },
        ),
        (
            "--screenshot",
            crate::CliArgs {
                screenshot: Some("out.png".into()),
                ..Default::default()
            },
        ),
        (
            "--dump-layout",
            crate::CliArgs {
                dump_layout: Some("dump.json".into()),
                ..Default::default()
            },
        ),
        ("no flag", crate::CliArgs::default()),
    ];
    for (flag, args) in runs {
        let shielded = args.shields_keyboard();
        apply_cli(&mut cx, &showcase, &args);
        let before = cx.update(|window, cx| {
            let field = showcase.read(cx).basic_text_state.clone();
            field.read(cx).focus_handle(cx).focus(window, cx);
            field.read(cx).value()
        });
        let panel = read(&mut cx, &showcase, |this, _| this.side_panel_visible);
        cx.simulate_input("ones?");
        cx.simulate_keystrokes("secondary-b");
        cx.run_until_parked();
        draw(&mut cx);
        let after = cx.update(|_window, cx| {
            let field = showcase.read(cx).basic_text_state.clone();
            field.read(cx).value()
        });
        assert_eq!(
            after != before,
            !shielded,
            "{flag}: the filled field holds {after:?}, from {before:?}"
        );
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.side_panel_visible) == panel,
            shielded,
            "{flag}: the side-panel toggle's binding"
        );
    }
}

/// A capture that holds no pointer of its own keeps the pointer off every
/// widget: wherever the desktop's pointer is, no widget is hovered, so the
/// inspector and the status bar show nothing hovered.
#[gpui::test]
fn a_capture_hovers_nothing(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let shield = |cx: &mut VisualTestContext, on: bool| {
        cx.update(|_window, cx| {
            showcase.update(cx, |this, cx| {
                this.pointer_shield = on;
                cx.notify();
            });
        });
        cx.run_until_parked();
        draw(cx);
    };
    shield(&mut cx, true);
    show(&mut cx, &showcase, Page::Basic);
    for target in ["basic-button", "basic-switch-on", CHROME_TOOLBAR] {
        let info = settle_on(&mut cx, &showcase, target);
        assert_eq!(
            info.map(|info| info.title()),
            None,
            "under the shield the pointer over {target} still shows its info"
        );
    }
    assert!(
        cx.debug_bounds(POINTER_SHIELD).is_some(),
        "the shield was not drawn"
    );
    shield(&mut cx, false);
    assert!(
        settle_on(&mut cx, &showcase, "basic-button").is_some(),
        "without the shield the Button's info is not shown either, so the \
         shield proved nothing"
    );
    assert!(
        crate::CliArgs {
            capture: true,
            ..Default::default()
        }
        .shields_pointer(),
        "a --capture run keeps the pointer on the widgets"
    );
    assert!(
        crate::CliArgs {
            screenshot: Some("out.png".into()),
            ..Default::default()
        }
        .shields_pointer(),
        "a --screenshot run keeps the pointer on the widgets"
    );
    assert!(
        crate::CliArgs {
            dump_layout: Some("dump.json".into()),
            ..Default::default()
        }
        .shields_pointer(),
        "a --dump-layout run keeps the pointer on the widgets"
    );
    assert!(
        !crate::CliArgs {
            capture: true,
            pointer: Some((10, 10)),
            ..Default::default()
        }
        .shields_pointer(),
        "a --capture with --pointer cannot show what the pointer hovers"
    );
    assert!(
        !crate::CliArgs::default().shields_pointer(),
        "an interactive run keeps the pointer off the widgets"
    );
}

/// M8-44: a layout dump (`--dump-layout`) records the chrome at rest
/// wherever the desktop's pointer is: with the pointer settled on the Basic
/// page's Button the dump is byte for byte the one with no pointer, Widget
/// Info holds its hint (`chrome.info.hint`) and no hovered widget's rows,
/// and the status bar names no hovered widget (`chrome.status_bar.shown`).
/// Without the shield the same pointer changes the dump, which is what makes
/// the first half mean something.
#[gpui::test]
fn a_layout_dump_is_the_same_under_the_pointer(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let shield = |cx: &mut VisualTestContext, on: bool| {
        cx.update(|_window, cx| {
            showcase.update(cx, |this, cx| {
                this.pointer_shield = on;
                cx.notify();
            });
        });
        cx.run_until_parked();
        draw(cx);
    };
    // The dump's own object (`dump::layout_json`), as
    // `serde_json::to_string_pretty` writes it (`{:#}`).
    let dump = |cx: &mut VisualTestContext| {
        cx.update(|window, cx| crate::layout_dump(showcase.read(cx), window, cx))
    };
    let dumping = crate::CliArgs {
        dump_layout: Some("dump.json".into()),
        ..Default::default()
    };
    shield(&mut cx, dumping.shields_pointer());
    show(&mut cx, &showcase, Page::Basic);
    let rest = dump(&mut cx);
    let _ = settle_on(&mut cx, &showcase, "basic-button");
    let pointed = dump(&mut cx);
    assert!(
        format!("{rest:#}") == format!("{pointed:#}"),
        "the pointer changed the layout dump:\n{rest:#}\n---\n{pointed:#}"
    );
    let elements = &rest["elements"];
    assert!(
        elements.get("chrome.info.hint").is_some(),
        "the dump holds no Widget Info hint"
    );
    for id in ["chrome.info.title", "chrome.status_bar.shown"] {
        assert!(elements.get(id).is_none(), "the dump at rest holds {id}");
    }
    shield(&mut cx, false);
    let _ = settle_on(&mut cx, &showcase, "basic-button");
    assert!(
        dump(&mut cx) != rest,
        "without the shield the pointer does not change the dump either, so \
         the dump half of this test proves nothing"
    );
}

/// The showcase follows the OS theme in an interactive run only: a capture
/// (`--capture`, `--screenshot`) and a layout dump (`--dump-layout`) show
/// the theme the run started with, and its watcher is never started there.
#[test]
fn only_an_interactive_run_follows_the_os() {
    assert!(
        crate::CliArgs::default().follows_os(),
        "a run without flags does not follow the OS"
    );
    for (flag, args) in [
        (
            "--capture",
            crate::CliArgs {
                capture: true,
                ..Default::default()
            },
        ),
        (
            "--screenshot p",
            crate::CliArgs {
                screenshot: Some("p".into()),
                ..Default::default()
            },
        ),
        (
            "--dump-layout f",
            crate::CliArgs {
                dump_layout: Some("f".into()),
                ..Default::default()
            },
        ),
    ] {
        assert!(!args.follows_os(), "{flag} follows the OS");
    }
}

/// The chrome takes what the theme states: the side panel is filled with
/// `sidebar.background_color`; the splitter is `splitter.divider_width`
/// wide in `splitter.divider_color`; a menu title under the pointer fills
/// with `menu.hover_background`, opens its menu, and a row of it runs its
/// action; and a page tab is at least `tab.min_width` by `tab.min_height`.
/// Under material-3 dark, whose splitter is wider than upstream's 1px line.
#[gpui::test]
fn the_chrome_follows_the_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "material-3");
    run_menu_item(&mut cx, "Theme", "Dark");
    show(&mut cx, &showcase, Page::Basic);
    let stated =
        |cx: &mut VisualTestContext,
         pick: fn(&native_theme_gpui::ResolvedTheme) -> native_theme::color::Rgba| {
            read(cx, &showcase, |_this, cx| {
                native_color(cx, |n| pick(n.resolved))
            })
        };
    let sidebar = stated(&mut cx, |r| r.sidebar.background_color);
    assert!(sidebar.is_some(), "no native theme is installed");
    assert_eq!(
        painted_fill(&mut cx, CHROME_SIDE_PANEL),
        sidebar,
        "the side panel is not filled with sidebar.background_color"
    );
    assert_eq!(
        painted_fill(&mut cx, "basic-select"),
        stated(&mut cx, |r| r.combo_box.background_color),
        "the drop-down is not filled with combo_box.background_color"
    );
    assert_eq!(
        painted_fill(&mut cx, "basic-textarea"),
        stated(&mut cx, |r| r.input.background_color),
        "the text area is not filled with input.background_color"
    );

    let (divider, width) = read(&mut cx, &showcase, |_this, cx| {
        (
            native_color(cx, |n| n.resolved.splitter.divider_color),
            native_value(cx, |n| n.resolved.splitter.divider_width),
        )
    });
    let line = bounds_of(&mut cx, CHROME_SPLITTER_LINE);
    assert_eq!(
        Some(line.size.width.as_f32()),
        width,
        "the splitter line is not splitter.divider_width wide"
    );
    assert_eq!(
        painted_fill(&mut cx, CHROME_SPLITTER_LINE),
        divider,
        "the splitter line is not splitter.divider_color"
    );

    let title = bounds_of(&mut cx, "menu-title-View");
    hover(&mut cx, title.center());
    draw(&mut cx);
    assert_eq!(
        painted_fill(&mut cx, "menu-title-View"),
        stated(&mut cx, |r| r.menu.hover_background),
        "a hovered menu title is not filled with menu.hover_background"
    );
    click(&mut cx, "menu-title-View");
    click(&mut cx, "menu-row-Buttons");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Buttons,
        "View > Buttons did not show the Buttons page"
    );

    let (min_width, min_height) = read(&mut cx, &showcase, |_this, cx| {
        native_value(cx, |n| {
            (n.resolved.tab.min_width, n.resolved.tab.min_height)
        })
    })
    .unwrap_or_default();
    let tab = bounds_of(&mut cx, Page::Basic.tab());
    assert!(
        tab.size.width.as_f32() >= min_width && tab.size.height.as_f32() >= min_height,
        "a page tab is {:?}, under tab.min_width {min_width} by tab.min_height {min_height}",
        tab.size
    );

    // The chrome Buttons' tooltips are the application's, refined.
    for button in [CHROME_TOOLBAR_PALETTE, CHROME_SIDE_PANEL_TOGGLE] {
        let info = settle_on(&mut cx, &showcase, button);
        assert!(
            info.as_ref()
                .is_some_and(|i| i.to_text().contains("geometry::tooltip:")),
            "{button}'s tooltip names no geometry::tooltip: {info:?}"
        );
    }
}

/// Two Checkboxes in different states show different infos (spec §4.3.2):
/// the one the showcase starts checked says so, and the one beside it, which
/// starts unchecked, says that.
#[gpui::test]
fn two_checkboxes_in_different_states_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Inputs);
    let mut texts = Vec::new();
    for (selector, title) in [
        (INPUTS_CHECKBOX_NOTIFICATIONS, "Checkbox · checked"),
        (INPUTS_CHECKBOX_AUTOSAVE, "Checkbox · unchecked"),
    ] {
        let info = settle_on(&mut cx, &showcase, selector);
        assert_eq!(
            info.as_ref().map(|info| info.title()).as_deref(),
            Some(title),
            "the pointer settled on {selector}, and the inspector does not show its info"
        );
        texts.push(info.map(|info| info.to_text()));
    }
    assert!(
        texts.first() != texts.get(1),
        "the checked and the unchecked Checkbox show the same info: {texts:?}"
    );
}

/// The claim of `info` that `Theme::input_background()` stands behind: the
/// one cited in theme/mod.rs, where that accessor is.
fn input_fill(info: &Option<WidgetInfo>) -> Option<gpui::Hsla> {
    info.as_ref()?
        .colors
        .iter()
        .find(|c| c.cited_at.starts_with("gpui-component/theme/mod.rs"))
        .map(|c| c.value)
}

/// A refined Input is filled with the platform's `input.background_color`
/// in either mode, through `geometry::input_fill` -- not with
/// `Theme::input_background()`, which is the window background in light mode
/// and input mixed toward transparent in dark (theme/mod.rs:485-490) -- and
/// its info shows no swatch for the fill upstream no longer paints, but the
/// builder's geometry line. The Basic page's disabled field is filled with
/// `input.disabled_background`.
#[gpui::test]
fn a_refined_input_is_filled_with_the_platforms_fill(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    for (item, dark) in [("Light", false), ("Dark", true)] {
        run_menu_item(&mut cx, "Theme", item);
        assert_eq!(
            cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
            dark,
            "Theme > {item} did not reach Theme::mode, so this proves nothing"
        );
        // The builder's fills, which its unit test holds to the two fields.
        let (fill, disabled_fill) = read(&mut cx, &showcase, |_this, cx| {
            match cx.native() {
                Some(n) => {
                    let solid = |r: gpui::StyleRefinement| {
                        r.background
                            .and_then(|f| f.color())
                            .and_then(|b| b.as_solid())
                    };
                    // A disabled field is faded by `input.disabled_opacity`
                    // too, which gpui folds into the painted fill's alpha.
                    let disabled = geometry::input_fill(n, true);
                    let fade = disabled.opacity.unwrap_or(1.);
                    (
                        solid(geometry::input_fill(n, false)),
                        solid(disabled).map(|c| gpui::Hsla { a: c.a * fade, ..c }),
                    )
                }
                None => (None, None),
            }
        });
        assert!(fill.is_some(), "no native theme is installed");

        show(&mut cx, &showcase, Page::Inputs);
        let info = settle_on(&mut cx, &showcase, INPUTS_FIELD);
        assert_eq!(
            painted_fill(&mut cx, INPUTS_FIELD),
            fill,
            "in {item} mode the fill painted inside the Input is not input.background_color"
        );
        assert_eq!(
            input_fill(&info),
            None,
            "in {item} mode the Input's info still shows upstream's fill: {info:?}"
        );
        assert!(
            info.as_ref()
                .is_some_and(|i| i.to_text().contains("geometry::input_fill")),
            "in {item} mode the Input's info names no geometry::input_fill: {info:?}"
        );

        show(&mut cx, &showcase, Page::Basic);
        assert_eq!(
            painted_fill(&mut cx, BASIC_INPUT_DISABLED),
            disabled_fill,
            "in {item} mode the disabled field is not filled with input.disabled_background"
        );
    }
}

/// The Basic page's disabled Button is filled with the platform's disabled
/// fill, through `geometry::button_disabled` -- not with upstream's
/// `input_background()` at half opacity (button/button.rs:1371-1375) -- in
/// either mode, and its info names the builder.
#[gpui::test]
fn a_disabled_button_is_filled_with_the_platforms_disabled_fill(cx: &mut TestAppContext) {
    const DISABLED: &str = "basic-button-disabled";
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    for (item, dark) in [("Light", false), ("Dark", true)] {
        run_menu_item(&mut cx, "Theme", item);
        assert_eq!(
            cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
            dark,
            "Theme > {item} did not reach Theme::mode, so this proves nothing"
        );
        let stated = read(&mut cx, &showcase, |_this, cx| {
            native_value(cx, |n| {
                let b = &n.resolved.button;
                let [r, g, bl, a] = b
                    .disabled_background
                    .unwrap_or(b.background_color)
                    .to_f32_array();
                // Faded by `button.disabled_opacity` too, which gpui folds
                // into the painted fill's alpha.
                let fade = geometry::button_disabled(n).opacity.unwrap_or(1.);
                gpui::Hsla::from(gpui::Rgba {
                    r,
                    g,
                    b: bl,
                    a: a * fade,
                })
            })
        });
        assert!(stated.is_some(), "no native theme is installed");
        show(&mut cx, &showcase, Page::Basic);
        let info = settle_on(&mut cx, &showcase, DISABLED);
        assert_eq!(
            painted_fill(&mut cx, DISABLED),
            stated,
            "in {item} mode the disabled Button is not filled with button.disabled_background"
        );
        assert!(
            info.as_ref()
                .is_some_and(|i| i.to_text().contains("geometry::button_disabled")),
            "in {item} mode the disabled Button's info names no geometry::button_disabled: {info:?}"
        );
    }
}

/// Set the text-scaling factor the connector lays controls out at, and draw
/// the frame that follows.
fn scale_text(cx: &mut VisualTestContext, factor: f32) {
    let mut prefs = native_theme_gpui::AccessibilityPreferences::default();
    prefs.text_scaling_factor = factor;
    cx.update(|_window, cx| native_theme_gpui::apply_accessibility(&prefs, cx));
    cx.run_until_parked();
    draw(cx);
}

/// The Input given `geometry::input_height` alone takes the height rule the
/// refined Input takes: at text scale 1 both are `input.min_height` tall; at
/// text scale 2 both grow past it.
///
/// Above text scale 1 the two do not line up, and this test does not claim
/// they do. The coordinator's ruling on Task 3 (fix round 1): above scale 1
/// the HeightOnly field carries the height rule alone, so upstream's own text
/// size and padding decide its growth -- `text_sm` and `input_py` for
/// `Size::Medium` (input/input.rs, Input::render) -- while the refined field
/// grows around the platform's; equality holds at scale 1. Under kde-breeze
/// at scale 2 that is 50px against 44px.
#[gpui::test]
fn the_height_only_field_takes_the_height_rule(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Inputs);
    let stated = cx.update(|_w, cx| native_value(cx, |n| px(n.resolved.input.min_height)));
    assert!(
        stated.is_some(),
        "no native theme is installed, so no field takes the height rule"
    );
    let field = bounds_of(&mut cx, INPUTS_FIELD_HEIGHT_ONLY).size.height;
    let refined = bounds_of(&mut cx, INPUTS_FIELD).size.height;
    assert_eq!(
        Some(field),
        stated,
        "at text scale 1 the height-only field is not input.min_height tall"
    );
    assert_eq!(
        field, refined,
        "at text scale 1 the height-only field does not line up with the refined one"
    );

    scale_text(&mut cx, 2.0);
    let field = bounds_of(&mut cx, INPUTS_FIELD_HEIGHT_ONLY).size.height;
    let refined = bounds_of(&mut cx, INPUTS_FIELD).size.height;
    assert!(
        stated.is_some_and(|s| field > s && refined > s),
        "at text scale 2 the fields did not grow past input.min_height {stated:?}: \
         height-only {field:?}, refined {refined:?}"
    );
}

/// The Textarea keeps its own 90px under `geometry::text_area`, which sets
/// no height, at text scale 1 and 2, and is padded as `text_area.border`
/// states, the multi-line field's own padding.
#[gpui::test]
fn the_textarea_keeps_its_own_height(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Inputs);
    let info = settle_on(&mut cx, &showcase, INPUTS_TEXTAREA);
    assert!(
        info.as_ref()
            .and_then(|i| i.config.iter().find(|n| n.what == "geometry"))
            .is_some_and(|g| g.text.starts_with("geometry::text_area:")),
        "the Textarea does not take geometry::text_area: {info:?}"
    );
    assert!(
        info.as_ref()
            .and_then(|i| i.instance.iter().find(|n| n.what == "padding"))
            .is_some_and(|p| p.text.starts_with("text_area.border.padding")),
        "the Textarea's info does not say it takes the text area's padding: {info:?}"
    );
    for factor in [1.0, 2.0] {
        scale_text(&mut cx, factor);
        assert_eq!(
            bounds_of(&mut cx, INPUTS_TEXTAREA).size.height,
            px(90.0),
            "at text scale {factor} the Textarea is not its own 90px"
        );
    }
}

/// A Switch's corner line follows upstream's condition: the theme's radius
/// under 4px, the track's own height from 4px up (switch.rs:205-209), 4px
/// itself included (`radius >= px(4.)`, switch.rs:205).
#[gpui::test]
fn a_switchs_corner_line_follows_upstreams_condition(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let theme = cx.update(|_w, cx| Theme::global(cx).clone());
    let corner = |radius: f32| {
        let mut t = theme.clone();
        t.radius = px(radius);
        crate::info::inputs::switch(&t, "Feature toggle", false, false, None)
            .config
            .into_iter()
            .find(|n| n.what == "border-radius")
            .map(|n| n.text)
    };
    assert_eq!(corner(2.).as_deref(), Some("radius: 2px"));
    assert!(
        corner(4.).is_some_and(|text| text.starts_with("fully round")),
        "at a 4px radius upstream rounds the track by its height, and the line says {:?}",
        corner(4.)
    );
    assert!(
        corner(6.).is_some_and(|text| text.starts_with("fully round")),
        "at a 6px radius the track is rounded by its height, and the line says {:?}",
        corner(6.)
    );
}

/// The height-only Input says it takes the height rule only when a native
/// theme gives it one.
#[gpui::test]
fn the_height_only_fields_note_follows_the_native_theme(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let theme = cx.update(|_w, cx| Theme::global(cx).clone());
    let height = |styled: bool| {
        crate::info::inputs::input(&theme, crate::demo::InputField::HeightOnly, styled)
            .instance
            .into_iter()
            .find(|n| n.what == "height")
            .map(|n| n.text)
    };
    assert!(height(true).is_some_and(|t| t.starts_with("the height rule")));
    assert!(height(false).is_some_and(|t| t.starts_with("upstream's own")));
}

/// The preset Combobox's swatches are the colours it paints: its fill is
/// `combo_box.background_color`, which the showcase refines the trigger
/// with over upstream's `input_background()`, and a hovered row is accent at
/// 70% (searchable_list/item.rs:114) -- checked against that line, since no
/// row is drawn while the popup is shut.
#[gpui::test]
fn the_preset_combobox_paints_its_fill_swatch_and_names_its_row_hover(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    run_menu_item(&mut cx, "Theme", "Dark");
    assert!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        "Theme > Dark did not reach Theme::mode, so this proves nothing"
    );
    let info = settle_on(&mut cx, &showcase, PROBE_COMBOBOX);
    let fill = read(&mut cx, &showcase, |_this, cx| {
        native_color(cx, |n| n.resolved.combo_box.background_color)
    });
    let row = cx.update(|_w, cx| Theme::global(cx).accent.opacity(0.7));
    let swatch = info.as_ref().and_then(|info| {
        info.colors
            .iter()
            .find(|c| c.role == "trigger fill")
            .map(|c| c.value)
    });
    assert!(fill.is_some(), "no native theme is installed");
    assert_eq!(
        swatch, fill,
        "the preset Combobox's fill swatch is not combo_box.background_color: {info:?}"
    );
    // `settle_on` leaves the pointer on the Combobox, so the surface under
    // its trigger shows `combo_box.hover_background` over the fill.
    let hovered = read(&mut cx, &showcase, |_this, cx| {
        native_value(cx, |n| {
            let c = &n.resolved.combo_box;
            let fill = crate::info::stated(c.background_color);
            c.hover_background
                .map_or(fill, |hover| source_over(fill, crate::info::stated(hover)))
        })
    });
    assert_eq!(
        painted_fill(&mut cx, PROBE_COMBOBOX),
        hovered,
        "the fill painted under the hovered preset Combobox is not combo_box.hover_background \
         over combo_box.background_color"
    );
    let hover = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role.starts_with("row hover")));
    assert_eq!(
        hover.map(|c| c.value),
        Some(row),
        "the preset Combobox's row hover swatch is not accent at 70%: {info:?}"
    );
}

/// Settle on each selector of `cases` in turn, assert the info shown is
/// titled as the case says, and return the infos' texts.
fn settle_on_each(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    cases: &[(&'static str, &str)],
) -> Vec<Option<String>> {
    let mut texts = Vec::new();
    for (selector, title) in cases {
        let info = settle_on(cx, showcase, selector);
        assert_eq!(
            info.as_ref().map(|info| info.title()).as_deref(),
            Some(*title),
            "the pointer settled on {selector}, and the inspector does not show its info"
        );
        texts.push(info.map(|info| info.to_text()));
    }
    texts
}

/// The Data page's two Paginations show different infos (spec §4.3.2): the
/// full one lists its pages and the current page's outlined button, the
/// compact one only its two arrows.
#[gpui::test]
fn two_paginations_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Data);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (DATA_PAGINATION, "Pagination"),
            (DATA_PAGINATION_COMPACT, "Pagination · compact"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the full and the compact Pagination show the same info: {texts:?}"
    );
}

/// A selected List row and an unselected one show different infos (spec
/// §4.3.2): the row the List marks selected after the delegate built it
/// says so.
#[gpui::test]
fn a_selected_list_row_and_an_unselected_one_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Data);
    click(&mut cx, "data-list-row-1");
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            ("data-list-row-1", "ListItem · Starred, selected"),
            ("data-list-row-0", "ListItem · Inbox"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the selected and the unselected List row show the same info: {texts:?}"
    );
}

/// The DataTable's header and its rows report themselves, and a row's info
/// follows the table's selection, which the delegate learns from the
/// table's events.
#[gpui::test]
fn a_table_row_reports_what_the_table_paints_on_it(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Data);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (DATA_TABLE_HEADER, "DataTable row · header"),
            ("data-table-row-0", "DataTable row"),
            ("data-table-row-1", "DataTable row · striped"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the header and a body row show the same info: {texts:?}"
    );
    click(&mut cx, "data-table-row-0");
    let selected = settle_on_each(
        &mut cx,
        &showcase,
        &[("data-table-row-0", "DataTable row · selected")],
    );
    assert!(
        selected.first() != texts.get(1),
        "the selected row shows the info it had unselected: {selected:?}"
    );
}

/// The DataTable's rows, as its delegate holds them for the table to draw.
fn table_rows(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> Vec<[String; 3]> {
    read(cx, showcase, |this, cx| {
        this.table_state
            .read(cx)
            .delegate()
            .rows
            .iter()
            .map(|row| row.clone().map(|cell| cell.to_string()))
            .collect()
    })
}

/// The DataTable's column keys, in the order its delegate holds them.
fn table_columns(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> Vec<String> {
    read(cx, showcase, |this, cx| {
        this.table_state
            .read(cx)
            .delegate()
            .columns
            .iter()
            .map(|column| column.key.to_string())
            .collect()
    })
}

/// `rows` with each row's cells in the order `order` names, each the index
/// a row holds that cell at.
fn reordered(rows: &[[String; 3]], order: [usize; 3]) -> Vec<[String; 3]> {
    rows.iter()
        .map(|row| order.map(|ix| row.get(ix).cloned().unwrap_or_default()))
        .collect()
}

/// `rows` sorted by the cell at `by`: descending where `descending`, else
/// ascending.
fn sorted(mut rows: Vec<[String; 3]>, by: usize, descending: bool) -> Vec<[String; 3]> {
    rows.sort_by(|a, b| {
        let order = a.get(by).cmp(&b.get(by));
        if descending { order.reverse() } else { order }
    });
    rows
}

/// Click the sort button of the DataTable's column `col_ix`: upstream's
/// `("icon-sort", col_ix)` (gpui-component 0.8.1 table/state.rs:1823, with
/// `test_support()`).
fn click_sort(cx: &mut VisualTestContext, col_ix: usize) {
    cx.update(|window, cx| {
        gpui_kit::test::TestWindowExt::click(window, ("icon-sort", col_ix), cx);
    });
    cx.run_until_parked();
    draw(cx);
}

/// The Data page's sort button sorts the rows (`SampleTableDelegate::
/// perform_sort`): one click descending by the Role column, the next
/// ascending, the third back to the rows as given, the cycle upstream's
/// `TableState::perform_sort` steps the button through.
#[gpui::test]
fn the_tables_sort_button_sorts_its_rows(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Data);
    let given = table_rows(&mut cx, &showcase);
    let sortable = read(&mut cx, &showcase, |this, cx| {
        this.table_state
            .read(cx)
            .delegate()
            .columns
            .iter()
            .position(|column| column.sort.is_some())
    });
    assert!(sortable.is_some(), "the table has no sortable column");
    let role = sortable.unwrap_or_default();
    for (descending, label) in [(true, "descending"), (false, "ascending")] {
        click_sort(&mut cx, role);
        assert_eq!(
            table_rows(&mut cx, &showcase),
            sorted(given.clone(), role, descending),
            "the sort button did not sort the rows {label} by column {role}"
        );
    }
    click_sort(&mut cx, role);
    assert_eq!(
        table_rows(&mut cx, &showcase),
        given,
        "the sort button's third click did not restore the rows as given"
    );
}

/// A header dragged to another place takes its cells with it
/// (`SampleTableDelegate::move_column`, which upstream asks for in
/// table/delegate.rs:155-165): the Role column dragged before the Name
/// column holds the roles, and its sort button then sorts the rows by them.
#[gpui::test]
fn a_moved_column_keeps_its_data(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Data);
    let given = table_rows(&mut cx, &showcase);
    assert_eq!(
        table_columns(&mut cx, &showcase),
        ["name", "role", "status"],
        "the table's columns are not the ones this test moves"
    );
    // The drag lands left of the first column's centre: upstream's gap
    // before it (table/state.rs, `drag_gap_at`).
    cx.update(|window, cx| {
        let from = gpui_kit::test::TestWindowExt::find(window, ("col-header", 1_usize)).bounds();
        let to = gpui_kit::test::TestWindowExt::find(window, ("col-header", 0_usize)).bounds();
        gpui_kit::test::TestWindowExt::drag(
            window,
            from.center(),
            point(to.left() + px(4.), to.center().y),
            cx,
        );
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        table_columns(&mut cx, &showcase),
        ["role", "name", "status"],
        "the drag did not move the Role column first"
    );
    let moved = reordered(&given, [1, 0, 2]);
    assert_eq!(
        table_rows(&mut cx, &showcase),
        moved,
        "the moved Role column does not hold the roles"
    );
    click_sort(&mut cx, 0);
    assert_eq!(
        table_rows(&mut cx, &showcase),
        sorted(moved, 0, true),
        "the moved Role column's sort button did not sort the rows by the roles"
    );
}

/// A List row's text is plain text, so it takes the list font
/// `geometry::list_item` gives the row, and its info says so; without a
/// native theme it is ListItem's own foreground.
#[gpui::test]
fn a_list_rows_text_is_the_list_font(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Data);
    let styled = cx.update(|_w, cx| native_value(cx, |_| ()).is_some());
    let info = settle_on(&mut cx, &showcase, "data-list-row-0");
    let text_claim = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role == "text"));
    let font_note = info.as_ref().is_some_and(|info| {
        info.instance
            .iter()
            .any(|n| n.what == "text" && n.text.starts_with("list.item_font"))
    });
    if styled {
        assert!(
            text_claim.is_none() && font_note,
            "the row takes list.item_font, and its info does not say so: {info:?}"
        );
    } else {
        assert_eq!(
            text_claim.map(|c| c.cited_at),
            Some("gpui-component/list/list_item.rs:205"),
            "the unstyled row's text is ListItem's foreground: {info:?}"
        );
    }
}

/// A Tree row reports itself though `Tree::new` gives nothing room to wrap
/// it, and its info follows a click that selects it.
#[gpui::test]
fn a_tree_row_reports_itself(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Data);
    let before = settle_on_each(
        &mut cx,
        &showcase,
        &[("data-tree-row-1", "ListItem · lib.rs")],
    );
    click(&mut cx, "data-tree-row-1");
    let after = settle_on_each(
        &mut cx,
        &showcase,
        &[("data-tree-row-1", "ListItem · lib.rs, selected")],
    );
    assert!(
        before != after,
        "the Tree row shows the same info selected: {after:?}"
    );
}

/// A List row and a Tree row under the pointer letter their label in
/// `list.hover_text_color` (M9-008), and in `list.item_font`'s colour at
/// rest. The label is plain text, so it is painted in the text colour it is
/// laid out under, which the registry records per row (a text colour is no
/// quad of the scene). kde-breeze light states the same colour for its list
/// text and its hovered list text, so the hovered one is set to the theme's
/// accent, a colour of the same theme that differs from both.
#[gpui::test]
fn a_hovered_list_or_tree_row_letters_its_label_in_the_lists_hover_text_colour(
    cx: &mut TestAppContext,
) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_light_preset(&mut cx, &showcase, "kde-breeze");
    let mut r = resolved_theme(&mut cx, &showcase);
    r.list.hover_text_color = r.defaults.accent_color;
    assert_ne!(
        r.list.hover_text_color, r.list.item_font.color,
        "kde-breeze light's accent is its list text, so the test could not tell the two apart"
    );
    cx.update(|_window, cx| {
        let prefs = native_theme::AccessibilityPreferences::default();
        native_theme_gpui::apply(
            native_theme_gpui::to_theme(&r, "kde-breeze", false, &prefs),
            &r,
            &prefs,
            cx,
        );
    });
    cx.run_until_parked();
    show(&mut cx, &showcase, Page::Data);
    let ui = read(&mut cx, &showcase, |this, _| this.info_ui.clone());
    for row in ["data-list-row-0", "data-tree-row-1"] {
        let text = |cx: &mut VisualTestContext| {
            cx.update(|_window, cx| ui.read(cx).label_colours.get(row).copied())
        };
        assert_eq!(
            text(&mut cx),
            Some(stated(r.list.item_font.color)),
            "{row}: the label at rest is not list.item_font's colour"
        );
        let bounds = bounds_of(&mut cx, row);
        hover(&mut cx, bounds.center());
        draw(&mut cx);
        assert_eq!(
            text(&mut cx),
            Some(stated(r.list.hover_text_color)),
            "{row}: the label under the pointer is not list.hover_text_color"
        );
        hover(&mut cx, point(px(0.), px(0.)));
        draw(&mut cx);
    }
}

/// A selected List or Tree row's info says its text is
/// `list.selection_text_color`, which `geometry::list_item_selected` sets
/// over the list font's colour (M9-056), and an unselected one's says the
/// list font's colour and names the hovered text the showcase sets
/// (M9-008), under a native theme.
#[gpui::test]
fn a_selected_list_or_tree_rows_info_names_the_selection_text_colour(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Data);
    let text = |info: &Option<WidgetInfo>| {
        info.as_ref()
            .and_then(|info| info.instance.iter().find(|n| n.what == "text"))
            .map(|n| n.text.clone())
    };
    let hovered_text = |info: &Option<WidgetInfo>| {
        info.as_ref()
            .is_some_and(|info| info.config.iter().any(|n| n.what == "hovered text"))
    };
    for (selected, unselected) in [
        ("data-list-row-1", "data-list-row-0"),
        ("data-tree-row-1", "data-tree-row-2"),
    ] {
        click(&mut cx, selected);
        let info = settle_on(&mut cx, &showcase, selected);
        let says = text(&info).unwrap_or_default();
        assert!(
            says.starts_with("list.selection_text_color")
                && says.contains("geometry::list_item_selected"),
            "{selected}: the selected row's text is list.selection_text_color through \
                 geometry::list_item_selected, and its info says {says:?}"
        );
        assert!(
            !hovered_text(&info),
            "{selected}: a selected row does not hover, and its info names a hovered text: {info:?}"
        );
        let info = settle_on(&mut cx, &showcase, unselected);
        let says = text(&info).unwrap_or_default();
        assert!(
            says.starts_with("list.item_font"),
            "{unselected}: the unselected row's text is list.item_font's, and its info says {says:?}"
        );
        assert!(
            hovered_text(&info),
            "{unselected}: the row's info does not name the hovered text: {info:?}"
        );
    }
}

/// A Tree row keeps clear of the Tree's scrollbar where the theme's is not
/// an overlay, as the List's rows do (M9-008): the Tree lays its bar over
/// its rows (gpui-component `src/tree.rs`, `RenderOnce for Tree`), so the
/// row ends the groove short of the box's inner edge, and at that edge
/// where the bar is an overlay. kde-breeze's scrollbar is no overlay;
/// macos-monterey's is, and must not lose a strip of width to it.
#[gpui::test]
fn a_tree_row_keeps_clear_of_a_scrollbar_that_is_no_overlay(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    for preset in ["kde-breeze", "macos-monterey"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Data);
        let (groove, overlay) = scrollbar_of(&mut cx, &showcase);
        assert!(
            groove > px(0.),
            "{preset}: no native theme is installed, so nothing was measured"
        );
        let edge = px(resolved_theme(&mut cx, &showcase).list.border.line_width);
        let gutter = if overlay { px(0.) } else { groove };
        let tree = bounds_of(&mut cx, TREE_DEMO);
        let row = bounds_of(&mut cx, "data-tree-row-1");
        assert_eq!(
            tree.right() - edge - row.right(),
            gutter,
            "{preset}: the row ends at {:?} inside a Tree whose inner edge is at {:?}, \
                 which leaves no {gutter:?} for a {groove:?} scrollbar (overlay: {overlay})",
            row.right(),
            tree.right() - edge,
        );
    }
}

/// A List's and a Tree's row's Widget Info names the room it keeps for the
/// scrollbar (`demo::list_gutter`): the groove's width,
/// `scrollbar.groove_width`, where the theme's scrollbar is no overlay
/// (kde-breeze), and none where it is one (macos-monterey).
#[gpui::test]
fn a_list_or_tree_rows_info_names_the_scrollbar_room_it_keeps(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    for preset in ["kde-breeze", "macos-monterey"] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Data);
        let (groove, overlay) = scrollbar_of(&mut cx, &showcase);
        let says = if overlay {
            "none: the theme's scrollbar is an overlay".to_string()
        } else {
            format!(
                "{}px, scrollbar.groove_width",
                crate::info::px_text(groove.as_f32())
            )
        };
        for row in ["data-list-row-0", "data-tree-row-1"] {
            let info = settle_on(&mut cx, &showcase, row);
            let note = shown_note(info.as_ref(), "right margin");
            assert!(
                note.as_deref().is_some_and(|t| t.starts_with(&says)),
                "{preset}: {row}'s info does not name the scrollbar room it keeps, {says:?}: {note:?}"
            );
        }
    }
}

/// Two Tags of different variants show different infos (spec §4.3.2): the
/// Primary Tag's info names Primary and its colours, and the Danger Tag's
/// beside it, Danger's -- not one info for the whole gallery.
#[gpui::test]
fn two_tags_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Feedback);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (FEEDBACK_TAG_PRIMARY, "Tag · Primary"),
            (FEEDBACK_TAG_DANGER, "Tag · Danger"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the Primary and the Danger Tag show the same info: {texts:?}"
    );
}

/// A count Badge and a dot Badge show different infos (spec §4.3.2): only
/// the count's says what its digits are painted with.
#[gpui::test]
fn a_count_badge_and_a_dot_badge_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Feedback);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (FEEDBACK_BADGE_COUNT, "Badge · count"),
            (FEEDBACK_BADGE_DOT, "Badge · dot"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the count and the dot Badge show the same info: {texts:?}"
    );
}

/// What an animated widget's info says holds only while motion is on:
/// under reduced motion gpui draws a repeating animation's start state and
/// schedules no frames (gpui-pre elements/animation.rs, AnimationExt). The
/// Spinner then stands still, and the indeterminate ProgressCircle draws no
/// arc at all.
#[gpui::test]
fn an_animations_info_follows_reduced_motion(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let note = |info: &Option<WidgetInfo>| {
        info.as_ref().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "animation")
                .map(|n| n.text.clone())
        })
    };
    let has_arc = |info: &Option<WidgetInfo>| {
        info.as_ref()
            .is_some_and(|info| info.colors.iter().any(|c| c.role == "arc"))
    };
    show(&mut cx, &showcase, Page::Feedback);
    let moving = settle_on(&mut cx, &showcase, FEEDBACK_SPINNER_SMALL);
    let looping = settle_on(&mut cx, &showcase, FEEDBACK_CIRCLE_LOADING);
    assert!(
        has_arc(&looping),
        "the ProgressCircle claims no arc with motion on: {looping:?}"
    );

    show(&mut cx, &showcase, Page::Buttons);
    cx.update(|_window, cx| cx.set_reduce_motion(true));
    show(&mut cx, &showcase, Page::Feedback);
    let still = settle_on(&mut cx, &showcase, FEEDBACK_SPINNER_SMALL);
    let held = settle_on(&mut cx, &showcase, FEEDBACK_CIRCLE_LOADING);
    assert!(
        note(&moving).is_some() && note(&moving) != note(&still),
        "the Spinner's animation note does not follow reduced motion: {:?} / {:?}",
        note(&moving),
        note(&still)
    );
    assert!(
        !has_arc(&held),
        "under reduced motion the ProgressCircle draws no arc, and its info claims one: {held:?}"
    );
}

/// An Alert's swatches are the tints upstream paints, not its variant's
/// colour at full strength: the Info Alert fills with 4% info mixed toward
/// transparent white (alert.rs:39), and that is the fill the frame holds
/// inside it.
#[gpui::test]
fn an_alerts_fill_swatch_is_the_painted_tint(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Feedback);
    let info = settle_on(&mut cx, &showcase, FEEDBACK_ALERT_INFO);
    let painted = cx.update(|_window, cx| {
        Theme::global(cx)
            .info
            .mix_oklab(gpui::transparent_white(), 0.04)
    });
    let fill = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role.starts_with("bg")));
    assert_eq!(
        fill.map(|c| c.value),
        Some(painted),
        "the Info Alert's fill swatch is not info at 4%: {info:?}"
    );
    assert_eq!(
        painted_fill(&mut cx, FEEDBACK_ALERT_INFO),
        Some(painted),
        "the fill painted inside the Info Alert is not its fill swatch"
    );
}

/// A plain Label and a Label with secondary text show different infos (spec
/// §4.3.2): only the second paints part of its text in muted_foreground
/// (label.rs:171), and only its info says so.
#[gpui::test]
fn a_plain_label_and_a_secondary_one_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Typography);
    let muted = |selector: &'static str, cx: &mut VisualTestContext| {
        settle_on(cx, &showcase, selector)
            .map(|info| info.colors.iter().any(|c| c.field == "muted_foreground"))
    };
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (TYPOGRAPHY_LABEL_PLAIN, "Label · plain"),
            (TYPOGRAPHY_LABEL_SECONDARY, "Label · with secondary"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the plain and the secondary Label show the same info: {texts:?}"
    );
    assert_eq!(
        (
            muted(TYPOGRAPHY_LABEL_PLAIN, &mut cx),
            muted(TYPOGRAPHY_LABEL_SECONDARY, &mut cx)
        ),
        (Some(false), Some(true)),
        "only the secondary Label paints muted_foreground"
    );
}

/// Two heading levels show different infos (spec §4.3.2): each states its
/// own rem and the size it renders at.
#[gpui::test]
fn two_heading_levels_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Typography);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[(TYPOGRAPHY_H1, "Text · H1"), (TYPOGRAPHY_H2, "Text · H2")],
    );
    assert!(
        texts.first() != texts.get(1),
        "the H1 and the H2 show the same info: {texts:?}"
    );
}

/// The Typography page's monospace sample takes `defaults.mono_font` under a
/// native theme (M9-014), as the Basic page's Monospace type line does: its
/// family, size (`Theme::mono_font_size`, the stated size the text-scaling
/// factor scales), weight and line box, at text scale 1 and 1.5; and its
/// info's size line names that size.
#[gpui::test]
fn the_mono_sample_takes_the_mono_font(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Typography);
    let r = resolved_theme(&mut cx, &showcase);
    for scale in [1.0, 1.5] {
        scale_text(&mut cx, scale);
        let (text, theme) = read(&mut cx, &showcase, |_this, cx| {
            let mut sample = crate::demo::mono_sample(cx, "mono");
            (sample.style().text.clone(), Theme::global(cx).clone())
        });
        let size = theme.mono_font_size;
        assert_eq!(
            (
                text.font_family,
                text.font_size,
                text.font_weight,
                text.line_height
            ),
            (
                Some(theme.mono_font_family.clone()),
                Some(size.into()),
                Some(gpui::FontWeight(f32::from(
                    native_theme_gpui::mono_font_weight(&r)
                ))),
                Some((size * r.defaults.line_height).into()),
            ),
            "at text scale {scale} the monospace sample does not take defaults.mono_font's family, size, weight and line box"
        );
        let info = settle_on(&mut cx, &showcase, "typography-mono");
        let line = info
            .as_ref()
            .and_then(|i| i.config.iter().find(|n| n.what == "size"));
        assert!(
            line.is_some_and(|l| l.text.starts_with(&format!(
                "{}px: defaults.mono_font.size scaled by the text-scaling factor",
                crate::info::px_text(size.as_f32())
            ))),
            "at text scale {scale} the monospace sample's size line does not claim the scaled defaults.mono_font.size: {line:?}"
        );
    }
}

/// The Typography page's `Diff` sample is one file's patch that removes and
/// adds lines: `parse` reads it whole, so the `unwrap_or_default` the
/// showcase builds its state with never stands in for it.
#[gpui::test]
fn the_diff_sample_parses_to_one_file(_cx: &mut TestAppContext) {
    let files = gpui_component::diff::DiffFile::parse(crate::support::DIFF_SAMPLE);
    let counts = files.as_ref().ok().map(|files| {
        files
            .iter()
            .map(|file| (file.additions(), file.deletions()))
            .collect::<Vec<_>>()
    });
    assert!(
        counts.as_ref().is_some_and(|counts| counts.len() == 1
            && counts
                .iter()
                .all(|&(added, removed)| added > 0 && removed > 0)),
        "the Diff sample is not one file with added and removed lines: {counts:?} ({:?})",
        files.err()
    );
}

/// The Typography page shows a `Diff`.
#[gpui::test]
fn the_typography_page_shows_a_diff(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Typography);
    assert!(
        cx.debug_bounds("typography-diff").is_some(),
        "the Typography page renders no `typography-diff`"
    );
}

/// Two GroupBoxes of different variants show different infos (spec
/// §4.3.2): the Normal one and the Outline one each report their own
/// variant, not one info for the whole gallery.
#[gpui::test]
fn two_group_boxes_of_different_variants_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (LAYOUT_GROUP_BOX_NORMAL, "GroupBox · Normal"),
            (LAYOUT_GROUP_BOX_OUTLINE, "GroupBox · Outline"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the Normal and the Outline GroupBox show the same info: {texts:?}"
    );
}

/// A solid Separator and a dashed one show different infos (spec §4.3.2):
/// only the dashed one says its line is stroked in dashes.
#[gpui::test]
fn a_solid_separator_and_a_dashed_one_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (LAYOUT_SEPARATOR_SOLID, "Separator · horizontal"),
            (LAYOUT_SEPARATOR_DASHED, "Separator · horizontal, dashed"),
        ],
    );
    assert!(
        texts.first() != texts.get(1),
        "the solid and the dashed Separator show the same info: {texts:?}"
    );
}

/// Two unselected Folder tabs side by side, the second and the third
/// (`demo::folder_tab_row` tags each tab `{id}-tab-{ix}`; the first is
/// selected), and the disabled one.
const FOLDER_TAB_SECOND: &str = "layout-folder-tabs-tab-1";
const FOLDER_TAB_THIRD: &str = "layout-folder-tabs-tab-2";
const FOLDER_TAB_DISABLED: &str = "layout-folder-tabs-tab-3";

/// The Layout page shows the Folder TabBar GPUI Kit 0.8.0 added (spec §5), and the
/// Widget Info it registers names the tab it draws selected, the first, and
/// the one `LAYOUT_FOLDER_TAB_DISABLED` names disabled.
#[gpui::test]
fn the_layout_page_draws_the_folder_tab_bar(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    assert!(
        cx.debug_bounds(LAYOUT_FOLDER_TABS).is_some(),
        "the Layout page renders no `{LAYOUT_FOLDER_TABS}`"
    );
    let info = settle_on(&mut cx, &showcase, LAYOUT_FOLDER_TABS);
    let instance = |what: &str| {
        info.as_ref().and_then(|info| {
            info.instance
                .iter()
                .find(|note| note.what == what)
                .map(|note| note.text.clone())
        })
    };
    assert_eq!(
        instance("selected").as_deref(),
        LAYOUT_FOLDER_TAB_LABELS.first().copied(),
        "the Folder TabBar's info does not name its first tab selected: {info:?}"
    );
    assert_eq!(
        instance("disabled").as_deref(),
        LAYOUT_FOLDER_TAB_LABELS
            .get(LAYOUT_FOLDER_TAB_DISABLED)
            .copied(),
        "the Folder TabBar's info does not name the disabled tab: {info:?}"
    );
}

/// The Folder TabBar paints its bar in `tab_bar` (gpui-component
/// `tab/tab_bar.rs:424`) and the 1px separator between two unselected tabs
/// in `border` (`tab/tab.rs:1025`), the colours its Widget Info claims, read
/// off the painted scene under the first two bundled presets whose mapped
/// `tab_bar` and `border` are opaque. The pointer is kept off the bar: a
/// hovered tab hides the separators beside it (`tab/tab_bar.rs:499-528`).
#[gpui::test]
fn the_folder_tab_bar_paints_its_bar_and_separators_in_their_claims(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    let mut checked = Vec::new();
    for preset in native_theme::theme::Theme::list_presets() {
        if checked.len() == 2 {
            break;
        }
        use_preset(&mut cx, &showcase, preset.key);
        let (bar, border) = cx.update(|_window, cx| {
            let t = Theme::global(cx);
            (t.tab_bar, t.border)
        });
        if bar.a < 1.0 || border.a < 1.0 {
            continue;
        }
        hover(&mut cx, point(px(4.), px(4.)));
        draw(&mut cx);
        let row = bounds_of(&mut cx, LAYOUT_FOLDER_TABS);
        let second = bounds_of(&mut cx, FOLDER_TAB_SECOND);
        let third = bounds_of(&mut cx, FOLDER_TAB_THIRD);
        let (bar_painted, separator_painted) = cx.update(|window, _cx| {
            let scale = window.scale_factor();
            let (row, second, third) = (row.scale(scale), second.scale(scale), third.scale(scale));
            let quads = scene_quads(window);
            let bar_painted = quads.iter().any(|q| {
                q.background.as_solid() == Some(bar)
                    && (q.bounds.top().0 - row.top().0).abs() < 0.5
                    && (q.bounds.bottom().0 - second.bottom().0).abs() < 0.5
                    && q.bounds.left() <= second.left()
                    && q.bounds.right() >= third.right()
            });
            let separator_painted = quads.iter().any(|q| {
                q.background.as_solid() == Some(border)
                    && (q.bounds.size.width.0 - scale).abs() < 0.01
                    && q.bounds.left() >= second.right()
                    && q.bounds.right() <= third.left()
            });
            (bar_painted, separator_painted)
        });
        assert!(
            bar_painted,
            "{}: no quad under the Folder tabs is painted in tab_bar {bar:?}",
            preset.key
        );
        assert!(
            separator_painted,
            "{}: no 1px quad between two unselected Folder tabs is painted in border {border:?}",
            preset.key
        );
        checked.push(preset.key);
    }
    assert_eq!(
        checked.len(),
        2,
        "fewer than two bundled presets map an opaque tab_bar and border: {checked:?}"
    );
}

/// A click on a Folder tab selects it, the showcase keeping the state; a
/// click on the disabled one selects nothing (gpui-base `tabs.rs:174-179`
/// drops a disabled tab's click handler).
#[gpui::test]
fn a_click_selects_a_folder_tab_and_not_the_disabled_one(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    click(&mut cx, FOLDER_TAB_SECOND);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.folder_tab),
        1,
        "a click on the second Folder tab did not select it"
    );
    click(&mut cx, FOLDER_TAB_DISABLED);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.folder_tab),
        1,
        "a click on the disabled Folder tab changed the selection"
    );
}

/// A Breadcrumb link shows its page: the first link, Buttons, dispatches
/// `ShowPage` for it, which goes through `show_page` as every other route
/// to a page does.
#[gpui::test]
fn a_breadcrumb_link_shows_its_page(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    click(&mut cx, LAYOUT_BREADCRUMB);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Buttons,
        "a click on the Breadcrumb's first link did not show the Buttons page"
    );
}

/// Toggling the Collapsible redraws it: the click flips the showcase's state
/// and asks for a frame, so the content goes and the toggle's info follows.
#[gpui::test]
fn toggling_the_collapsible_redraws_it(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Layout);
    let open = settle_on(&mut cx, &showcase, LAYOUT_COLLAPSIBLE_TOGGLE);
    assert!(
        cx.debug_bounds(LAYOUT_COLLAPSIBLE_CONTENT).is_some(),
        "the Collapsible starts open, and its content was not laid out"
    );
    // A test draws its frames itself, so what shows the click asked for one
    // is a notification of the showcase, which is what marks its view dirty
    // (gpui-pre app/context.rs, Context::notify).
    let notified = Rc::new(std::cell::Cell::new(0usize));
    let _observer = cx.update(|_window, cx| {
        let notified = notified.clone();
        cx.observe(&showcase, move |_, _| notified.set(notified.get() + 1))
    });
    click(&mut cx, LAYOUT_COLLAPSIBLE_TOGGLE);
    assert!(
        !read(&mut cx, &showcase, |this, _| this.collapsible_open),
        "the toggle did not close the Collapsible"
    );
    assert!(
        notified.get() > 0,
        "the toggle closed the Collapsible without asking for a frame, so the \
         window keeps showing it open until something else redraws it"
    );
    assert!(
        cx.debug_bounds(LAYOUT_COLLAPSIBLE_CONTENT).is_none(),
        "the Collapsible closed, and the frame still shows its content"
    );
    let closed = settle_on(&mut cx, &showcase, LAYOUT_COLLAPSIBLE);
    assert_eq!(
        closed.map(|info| info.title()).as_deref(),
        Some("Collapsible · closed"),
        "the Collapsible's info did not follow it closing"
    );
    let toggle = settle_on(&mut cx, &showcase, LAYOUT_COLLAPSIBLE_TOGGLE);
    assert!(
        open.is_some() && toggle.is_some() && open != toggle,
        "the toggle's info did not follow the Collapsible closing: {open:?} / {toggle:?}"
    );
}

/// Innermost wins inside an overlay (spec §4.3.1): the pointer over the
/// Button in the Overlays page's Dialog shows the Button, and over the
/// Dialog's surface beside it, the Dialog. Motion is reduced, so the
/// Dialog's note about its entrance says it does not slide in.
#[gpui::test]
fn a_button_inside_the_dialog_shows_the_button_and_its_surface_the_dialog(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    show(&mut cx, &showcase, Page::Overlays);
    click(&mut cx, OVERLAYS_DIALOG_TRIGGER);
    draw(&mut cx);
    assert!(a_dialog_is_open(&mut cx), "the Dialog did not open");
    let button = settle_on(&mut cx, &showcase, OVERLAYS_DIALOG_CLOSE);
    assert_eq!(
        button.as_ref().map(|info| info.title()).as_deref(),
        Some("Button · Default"),
        "the pointer settled on the Dialog's Button, and the inspector does not show the Button"
    );
    let footer = bounds_of(&mut cx, OVERLAYS_DIALOG_FOOTER);
    let close = bounds_of(&mut cx, OVERLAYS_DIALOG_CLOSE);
    // Beside the Button, on the footer row of the Dialog's surface.
    let beside = point(footer.left() + px(4.), close.center().y);
    assert!(
        !close.contains(&beside),
        "the point beside the Button at {beside:?} is on the Button at {close:?}"
    );
    hover(&mut cx, beside);
    settle(&mut cx);
    draw(&mut cx);
    let dialog = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().map(|info| (**info).clone())
    });
    assert_eq!(
        dialog.as_ref().map(|info| info.title()).as_deref(),
        Some("Dialog · Confirm Action"),
        "the pointer settled on the Dialog's surface beside its Button, and the inspector does not show the Dialog"
    );
    let animation = dialog.as_ref().and_then(|info| {
        info.not_themeable
            .iter()
            .find(|n| n.what == "animation")
            .map(|n| n.text.clone())
    });
    assert!(
        animation.as_deref().is_some_and(|t| t.starts_with("none")),
        "motion is reduced, and the Dialog's animation note does not say it appears at once: {animation:?}"
    );
}

/// The showcase applies the theme's dialog entrance and tooltip delay, and
/// says so: under kde-breeze, which states both, the Overlays page's Dialog
/// and AlertDialog record `geometry::dialog_entrance` and a toolbar Button's
/// tooltip delay is the theme's 700ms; under catppuccin-mocha, which states
/// neither, the dialogs keep upstream's slide and the tooltip gpui's 500ms
/// (Review Focus 2). Motion is reduced, so the entrance is named and not
/// drawn.
#[gpui::test]
fn the_dialog_entrance_and_the_tooltip_delay_follow_the_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    // A sliding Dialog is not where its bounds were laid out until it lands.
    without_motion(&mut cx);
    for (preset, stated) in [("kde-breeze", true), ("catppuccin-mocha", false)] {
        use_preset(&mut cx, &showcase, preset);
        let (entrance, delay) = read(&mut cx, &showcase, |_, cx| {
            native_value(cx, |n| {
                (
                    geometry::dialog_entrance(n),
                    native_theme_gpui::tooltip_show_delay(n.resolved),
                )
            })
        })
        .unwrap_or_else(|| panic!("{preset}: no native theme is installed"));
        assert_eq!(
            (entrance.is_some(), delay.is_some()),
            (stated, stated),
            "precondition: what {preset} states"
        );

        let button = settle_on(&mut cx, &showcase, CHROME_TOOLBAR_PALETTE);
        let note = |notes: &[crate::info::Note]| {
            notes
                .iter()
                .find(|n| n.what == "tooltip delay")
                .map(|n| n.text.clone())
        };
        let (config, fixed) = button
            .as_ref()
            .map(|info| (note(&info.config), note(&info.not_themeable)))
            .unwrap_or_default();
        if stated {
            assert!(
                config.as_deref().is_some_and(|t| t.contains("700ms")) && fixed.is_none(),
                "{preset}: the toolbar Button's tooltip delay is not the theme's: {config:?} / {fixed:?}"
            );
        } else {
            assert!(
                fixed.as_deref().is_some_and(|t| t.starts_with("500ms")) && config.is_none(),
                "{preset}: the toolbar Button's tooltip delay is not gpui's: {config:?} / {fixed:?}"
            );
        }

        // Every kind of dialog the showcase opens: the Dialog (the palette's
        // and the About dialog's take the same `demo::dialog_frame`) and the
        // AlertDialog, which applies the entrance itself.
        for (trigger, selector) in [
            (OVERLAYS_DIALOG_TRIGGER, "overlays-dialog-title"),
            (PROBE_ALERT_DIALOG, "overlays-alert-dialog-title"),
        ] {
            show(&mut cx, &showcase, Page::Overlays);
            click(&mut cx, trigger);
            draw(&mut cx);
            assert!(
                a_dialog_is_open(&mut cx),
                "{preset}: {trigger} opened no dialog"
            );
            let dialog = settle_on(&mut cx, &showcase, selector);
            let recorded = dialog.as_ref().is_some_and(|info| {
                info.config
                    .iter()
                    .any(|n| n.text.starts_with("geometry::dialog_entrance:"))
            });
            let animation = dialog.as_ref().and_then(|info| {
                info.not_themeable
                    .iter()
                    .find(|n| n.what == "animation")
                    .map(|n| n.text.clone())
            });
            assert_eq!(
                recorded, stated,
                "{preset}: {selector}'s geometry::dialog_entrance line: {dialog:?}"
            );
            assert!(
                animation
                    .as_deref()
                    .is_some_and(|t| t.contains("DialogEntrance::Fade") == stated),
                "{preset}: {selector}'s animation note: {animation:?}"
            );
            cx.update(|window, cx| window.close_dialog(cx));
            cx.run_until_parked();
            draw(&mut cx);
        }
    }
}

/// Each dialog the showcase builds reads the theme's entrance once
/// (`demo::themed_entrance`), and its info and its drawing take that one
/// value: the command palette, the About dialog, the Overlays page's Dialog
/// and its AlertDialog, each built once.
#[gpui::test]
fn a_dialog_reads_the_themes_entrance_once(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let reads = || crate::demo::THEMED_ENTRANCES.with(std::cell::Cell::get);
    type Parts = (
        Entity<InfoRegistry>,
        Entity<gpui_component::command::CommandState>,
        crate::support::SampleIcon,
        crate::support::SampleIcon,
    );
    let parts: Parts = read(&mut cx, &showcase, |this, _| {
        (
            this.info_ui.clone(),
            this.palette_state.clone(),
            this.sample_icon(IconName::CircleX),
            this.sample_icon(IconName::TriangleAlert),
        )
    });
    type Build = fn(&Parts, &mut App);
    let builds: [(&str, Build); 4] = [
        ("the command palette", |(ui, state, ..), cx| {
            let dialog = gpui_component::dialog::Dialog::new(cx);
            let _ = crate::demo::command_palette(ui, cx, dialog, state, Vec::new(), "set".into());
        }),
        ("the About dialog", |(ui, ..), cx| {
            let dialog = gpui_component::dialog::Dialog::new(cx);
            let _ = crate::demo::about(ui, cx, dialog, "name", "url", None);
        }),
        ("the Overlays page's Dialog", |(ui, _, icon, _), cx| {
            let dialog = gpui_component::dialog::Dialog::new(cx);
            let _ = crate::demo::confirm_dialog(ui, cx, dialog, None, icon);
        }),
        ("the Overlays page's AlertDialog", |(ui, _, _, icon), cx| {
            let alert = gpui_component::dialog::AlertDialog::new(cx);
            let _ = crate::demo::alert_dialog(ui, cx, alert, icon);
        }),
    ];
    for (dialog, build) in builds {
        let before = reads();
        cx.update(|_window, cx| build(&parts, cx));
        assert_eq!(
            reads() - before,
            1,
            "{dialog}'s build read the theme's entrance more than once"
        );
    }
}

/// The Overlays page's two Sheets show different infos (spec §4.3.2): the
/// one at the right edge and the one at the bottom edge are edged on
/// different sides, and only the right one clears the title bar.
#[gpui::test]
fn a_right_sheet_and_a_bottom_sheet_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    show(&mut cx, &showcase, Page::Overlays);
    let mut texts = Vec::new();
    for (trigger, title, expected) in [
        (
            OVERLAYS_SHEET_RIGHT,
            OVERLAYS_SHEET_RIGHT_TITLE,
            "Sheet · Right",
        ),
        (
            OVERLAYS_SHEET_BOTTOM,
            OVERLAYS_SHEET_BOTTOM_TITLE,
            "Sheet · Bottom",
        ),
    ] {
        click(&mut cx, trigger);
        draw(&mut cx);
        texts.extend(settle_on_each(&mut cx, &showcase, &[(title, expected)]));
        cx.update(|window, cx| window.close_sheet(cx));
        cx.run_until_parked();
        draw(&mut cx);
    }
    assert!(
        texts.first() != texts.get(1),
        "the right and the bottom Sheet show the same info: {texts:?}"
    );
}

/// The Charts page's seven charts each show their own info (spec §4.3.2):
/// each paints its own series and names its own upstream lines.
#[gpui::test]
fn every_chart_shows_its_own_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Charts);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (CHARTS_BAR_CHART, "BarChart"),
            (CHARTS_LINE_CHART, "LineChart"),
            (CHARTS_AREA_CHART, "AreaChart"),
            (CHARTS_PIE_CHART, "PieChart · donut"),
            (CHARTS_CANDLESTICK_CHART, "CandlestickChart"),
            (CHARTS_RADAR_CHART, "RadarChart"),
            (CHARTS_SANKEY_CHART, "SankeyChart"),
        ],
    );
    for (i, text) in texts.iter().enumerate() {
        assert!(
            !texts.iter().skip(i + 1).any(|other| other == text),
            "two charts show the same info: {texts:?}"
        );
    }
}

/// The AreaChart's fill swatch is the colour the showcase asks upstream to
/// paint, `demo::area_series`' fill: the series colour at
/// `AREA_FILL_OPACITY`, not the colour at full. The area is a path, not a
/// quad of the scene, so the swatch is checked against what the showcase
/// passes, not against the frame.
#[gpui::test]
fn the_area_charts_fill_swatch_is_the_series_at_the_fill_opacity(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Charts);
    let info = settle_on(&mut cx, &showcase, CHARTS_AREA_CHART);
    let painted = cx.update(|_window, cx| crate::demo::area_series(Theme::global(cx)).1);
    let fill = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role.starts_with("fill")));
    assert_eq!(
        fill.map(|c| c.value),
        Some(painted),
        "the AreaChart's fill swatch is not what the showcase passes, chart_3 at {}: {info:?}",
        percent_text(AREA_FILL_OPACITY)
    );
}

/// The side of the dot a chart marks a hovered point with:
/// gpui-component's `HOVER_DOT_SIZE` (src/chart/mod.rs:128), crate-private.
const HOVER_DOT_SIDE: f32 = 8.;

/// The side of the halo behind that dot at full hover:
/// gpui-component's `HOVER_HALO_SIZE` (src/chart/mod.rs:134), crate-private.
const HOVER_HALO_SIDE: f32 = 20.;

/// The marks a chart paints at the hovered point, with the pointer settled
/// on `selector`'s centre: the fills of the `HOVER_DOT_SIDE` quads, the
/// edges of the bordered ones, and the fills of the `HOVER_HALO_SIDE`
/// quads, inside the chart's bounds. gpui paints a dot as a filled quad and
/// an edge quad over the same bounds (plot/tooltip.rs, Dot).
fn hover_marks(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    selector: &'static str,
) -> (Vec<gpui::Hsla>, Vec<gpui::Hsla>, Vec<gpui::Hsla>) {
    settle_on(cx, showcase, selector);
    let bounds = bounds_of(cx, selector);
    cx.update(|window, _cx| {
        let scale = window.scale_factor();
        let bounds = bounds.scale(scale);
        let quads: Vec<gpui::Quad> = scene_quads(window)
            .into_iter()
            .filter(|q| {
                q.bounds.left() >= bounds.left()
                    && q.bounds.top() >= bounds.top()
                    && q.bounds.right() <= bounds.right()
                    && q.bounds.bottom() <= bounds.bottom()
            })
            .collect();
        let sided = |side: f32| {
            let side = side * scale;
            quads
                .iter()
                .filter(move |q| {
                    (q.bounds.size.width.0 - side).abs() < 0.5
                        && (q.bounds.size.height.0 - side).abs() < 0.5
                })
                .cloned()
                .collect::<Vec<gpui::Quad>>()
        };
        let fills = |quads: &[gpui::Quad]| -> Vec<gpui::Hsla> {
            quads
                .iter()
                .filter(|q| !q.background.is_transparent())
                .filter_map(|q| q.background.as_solid())
                .collect()
        };
        let dots = sided(HOVER_DOT_SIDE);
        let edges = dots
            .iter()
            .filter(|q| q.border_widths.top.0 > 0.)
            .map(|q| q.border_color)
            .collect();
        (fills(&dots), edges, fills(&sided(HOVER_HALO_SIDE)))
    })
}

/// The AreaChart's hovered point is marked in the colour the showcase
/// strokes its line with, `demo::area_series`' line: the dot filled in it
/// (chart/area_chart.rs, AreaChart::tooltip) and the halo in it at 20%
/// (plot/tooltip.rs, Dot), the colours the Info's "line" claims. The line
/// is a path; the dot and the halo are quads, so a line the showcase strokes
/// in another colour turns this red.
#[gpui::test]
fn the_area_charts_hover_marks_are_its_line_colour(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    show(&mut cx, &showcase, Page::Charts);
    let (dots, _, halos) = hover_marks(&mut cx, &showcase, CHARTS_AREA_CHART);
    let (line, info_line) = (
        cx.update(|_window, cx| crate::demo::area_series(Theme::global(cx)).0),
        settle_on(&mut cx, &showcase, CHARTS_AREA_CHART).and_then(|info| {
            info.colors
                .iter()
                .find(|c| c.role == "line")
                .map(|c| c.value)
        }),
    );
    assert_eq!(dots, [line], "the hover dot is not the area's line colour");
    assert_eq!(
        halos,
        [line.opacity(0.2)],
        "the halo is not the line at 20%"
    );
    assert_eq!(
        info_line,
        Some(line),
        "the Info's line swatch is not the line painted"
    );
}

/// The RadarChart's fill swatches are what the showcase passes,
/// `demo::radar_series`' fills: each series colour at `RADAR_FILL_OPACITY`.
/// The fills are paths, so they are checked against what the showcase
/// passes, as the AreaChart's is.
#[gpui::test]
fn the_radar_charts_fill_swatches_are_the_series_at_the_fill_opacity(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Charts);
    let info = settle_on(&mut cx, &showcase, CHARTS_RADAR_CHART);
    let passed: Vec<gpui::Hsla> = cx.update(|_window, cx| {
        crate::demo::radar_series(Theme::global(cx))
            .into_iter()
            .map(|(_, fill)| fill)
            .collect()
    });
    let fills: Vec<gpui::Hsla> = info
        .as_ref()
        .map(|info| {
            info.colors
                .iter()
                .filter(|c| c.role.contains("series fill"))
                .map(|c| c.value)
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(
        fills,
        passed,
        "the RadarChart's fill swatches are not what the showcase passes, chart_1 and chart_2 at {}: {info:?}",
        percent_text(RADAR_FILL_OPACITY)
    );
}

/// The RadarChart marks the hovered spoke with one dot per series, filled in
/// the colour the showcase strokes the series with (`demo::radar_series`,
/// `chart_1` and `chart_2`) and edged in `background`, over a halo of that
/// colour at 20%; the Info's halo swatches are those halos. They are quads,
/// so a series the showcase paints in another colour turns this red.
#[gpui::test]
fn the_radar_charts_hover_dots_are_its_series_colours(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    show(&mut cx, &showcase, Page::Charts);
    let (fills, edges, halos) = hover_marks(&mut cx, &showcase, CHARTS_RADAR_CHART);
    let (lines, background) = cx.update(|_window, cx| {
        let t = Theme::global(cx);
        (
            crate::demo::radar_series(t).map(|(line, _)| line),
            t.background,
        )
    });
    let info = settle_on(&mut cx, &showcase, CHARTS_RADAR_CHART);
    let info_halos: Vec<gpui::Hsla> = info
        .as_ref()
        .map(|info| {
            info.colors
                .iter()
                .filter(|c| c.role.contains("hover halo"))
                .map(|c| c.value)
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(
        edges,
        [background, background],
        "not two dots edged in background"
    );
    let sorted = |mut colours: Vec<gpui::Hsla>| {
        colours.sort_by(|a, b| a.h.total_cmp(&b.h));
        colours
    };
    let mut expected_dots = lines.to_vec();
    expected_dots.sort_by(|a, b| a.h.total_cmp(&b.h));
    assert_eq!(
        sorted(fills),
        expected_dots,
        "the hover dots are not the series colours"
    );
    let mut expected_halos: Vec<gpui::Hsla> = lines.iter().map(|l| l.opacity(0.2)).collect();
    expected_halos.sort_by(|a, b| a.h.total_cmp(&b.h));
    assert_eq!(
        sorted(halos),
        expected_halos,
        "the halos are not the series at 20%"
    );
    assert_eq!(
        sorted(info_halos),
        expected_halos,
        "the Info's halo swatches are not the halos painted"
    );
}

/// The SankeyChart's node quads are `chart_1` to `chart_5`, one node each in
/// `SANKEY_NODES`' order, and nothing else it paints is a filled quad: a
/// node the showcase colours otherwise turns this red.
#[gpui::test]
fn the_sankey_charts_nodes_are_the_chart_palette(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    show(&mut cx, &showcase, Page::Charts);
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    let bounds = bounds_of(&mut cx, CHARTS_SANKEY_CHART);
    let (fills, palette) = cx.update(|window, cx| {
        let t = Theme::global(cx);
        let bounds = bounds.scale(window.scale_factor());
        let fills: Vec<Option<gpui::Hsla>> = scene_quads(window)
            .into_iter()
            .filter(|q| {
                !q.background.is_transparent()
                    && q.bounds.left() >= bounds.left()
                    && q.bounds.top() >= bounds.top()
                    && q.bounds.right() <= bounds.right()
                    && q.bounds.bottom() <= bounds.bottom()
            })
            .map(|q| q.background.as_solid())
            .collect();
        (
            fills,
            [t.chart_1, t.chart_2, t.chart_3, t.chart_4, t.chart_5],
        )
    });
    assert_eq!(
        fills.len(),
        palette.len(),
        "not one quad per node: {fills:?}"
    );
    for colour in palette {
        assert!(
            fills.contains(&Some(colour)),
            "no node is {colour:?}: {fills:?}"
        );
    }
}

/// The ids and debug selectors of three of the Icons page's icons, one from
/// each gallery, with Material's icons loaded. The page forms them from the
/// animation's place, the role's name and the IconName's (pages/icons.rs).
#[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
const ICONS_ANIMATED_FRAMES: &str = "icons-animated-frames-0";
#[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
const ICONS_NATIVE_DIALOG_WARNING: &str = "icons-native-DialogWarning";
#[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
const ICONS_GPUI_TRIANGLE_ALERT: &str = "icons-gpui-TriangleAlert";
/// The role gpui-component has no icon for (native-theme-gpui icons.rs,
/// `icon_name`), whose cell the page forms the same way.
const ICONS_NATIVE_SHIELD: &str = "icons-native-Shield";

/// Load the icons of the icon set the Select names `display`, as confirming
/// it there does, and show the Icons page.
fn show_icons(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, display: &str) {
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| this.select_icon_set(display, window, cx));
    });
    show(cx, showcase, Page::Icons);
}

/// The Icons page's galleries report each icon, not one info for a block
/// (spec §4.3.2): an animated icon, a role's icon and an IconName's each
/// show their own.
// The bundled Material set, and its frames drawn on their first draw, which
// only `svg-rasterize` promises (`icons::animated_frames_to_image_sources`).
#[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
#[gpui::test]
fn icons_from_different_galleries_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show_icons(&mut cx, &showcase, "Material (bundled)");
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (ICONS_ANIMATED_FRAMES, "Animated icon · material, frames"),
            (ICONS_NATIVE_DIALOG_WARNING, "IconRole · DialogWarning"),
            (ICONS_GPUI_TRIANGLE_ALERT, "IconName · TriangleAlert"),
        ],
    );
    for (i, text) in texts.iter().enumerate() {
        assert!(
            !texts.iter().skip(i + 1).any(|other| other == text),
            "two icons of different galleries show the same info: {texts:?}"
        );
    }
}

/// An animated icon's info holds under reduced motion as the page does:
/// with motion on the showcase steps through the frames, and under reduced
/// motion it shows the first and says that nothing moves.
// The bundled Material set, and its frames drawn on their first draw, which
// only `svg-rasterize` promises (`icons::animated_frames_to_image_sources`).
#[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
#[gpui::test]
fn an_animated_icons_info_follows_reduced_motion(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let note = |info: &Option<WidgetInfo>| {
        info.as_ref().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "animation")
                .map(|n| n.text.clone())
        })
    };
    show_icons(&mut cx, &showcase, "Material (bundled)");
    let moving = note(&settle_on(&mut cx, &showcase, ICONS_ANIMATED_FRAMES));

    show(&mut cx, &showcase, Page::Buttons);
    cx.update(|_window, cx| cx.set_reduce_motion(true));
    show(&mut cx, &showcase, Page::Icons);
    let still = note(&settle_on(&mut cx, &showcase, ICONS_ANIMATED_FRAMES));
    assert!(
        moving.is_some() && moving != still,
        "the animated icon's note does not follow reduced motion: {moving:?} / {still:?}"
    );
    assert!(
        still.as_deref().is_some_and(|n| n.starts_with("none")),
        "under reduced motion the animated icon's note says it moves: {still:?}"
    );
}

/// The Icons page's frame animation steps, and redraws the window, only
/// while the Icons page is shown (M9-021): it is the one page that draws the
/// frames. With Material chosen (a frame animation), the executor's clock
/// is moved on by ten frame durations on the Basic page, then on the Icons
/// page.
// The bundled Material set, and its frames drawn on their first draw, which
// only `svg-rasterize` promises (`icons::animated_frames_to_image_sources`).
#[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
#[gpui::test]
fn the_icon_animation_steps_only_on_the_icons_page(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let steps = || crate::app::ANIMATION_STEPS.with(std::cell::Cell::get);
    cx.update(|window, cx| {
        cx.set_reduce_motion(false);
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Material (bundled)", window, cx)
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    let frame = read(&mut cx, &showcase, |this, _| {
        this.animated_frame_durations.iter().copied().min()
    });
    assert!(
        frame.is_some(),
        "Material's indicator is no frame animation"
    );
    let Some(frame) = frame else { return };
    let frame = Duration::from_millis(u64::from(frame));
    let ten_frames = |cx: &mut VisualTestContext| {
        for _ in 0..10 {
            cx.executor().advance_clock(frame);
            cx.run_until_parked();
        }
    };
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Basic
    );
    let before = steps();
    ten_frames(&mut cx);
    assert_eq!(
        steps() - before,
        0,
        "the animation stepped and redrew the window on the Basic page, which draws no frame of it"
    );
    show(&mut cx, &showcase, Page::Icons);
    let before = steps();
    ten_frames(&mut cx);
    assert!(
        steps() - before > 0,
        "the animation did not step on the Icons page"
    );
}

/// `--tab` opens a page as the page tabs do (`show_page`), so `--tab icons`
/// starts the Icons page's animation timer and the frames step, and
/// `--tab basic` stops it again. With Material chosen (a frame animation),
/// the executor's clock is moved on by ten frame durations after each.
// The bundled Material set, and its frames drawn on their first draw, which
// only `svg-rasterize` promises (`icons::animated_frames_to_image_sources`).
#[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
#[gpui::test]
fn the_tab_flag_starts_the_icon_animation_on_the_icons_page_alone(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let steps = || crate::app::ANIMATION_STEPS.with(std::cell::Cell::get);
    cx.update(|window, cx| {
        cx.set_reduce_motion(false);
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Material (bundled)", window, cx)
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    let frame = read(&mut cx, &showcase, |this, _| {
        this.animated_frame_durations.iter().copied().min()
    });
    assert!(
        frame.is_some(),
        "Material's indicator is no frame animation"
    );
    let Some(frame) = frame else { return };
    let frame = Duration::from_millis(u64::from(frame));
    let ten_frames = |cx: &mut VisualTestContext| {
        for _ in 0..10 {
            cx.executor().advance_clock(frame);
            cx.run_until_parked();
        }
    };
    let timer = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, _| {
            (this.active_page, this.animation_timer.is_some())
        })
    };
    assert_eq!(timer(&mut cx), (Page::Basic, false));
    for (tab, page, runs) in [("icons", Page::Icons, true), ("basic", Page::Basic, false)] {
        apply_cli(
            &mut cx,
            &showcase,
            &crate::CliArgs {
                tab: Some(tab.into()),
                ..Default::default()
            },
        );
        assert_eq!(
            timer(&mut cx),
            (page, runs),
            "--tab {tab}: the page shown and whether the animation timer runs"
        );
        let before = steps();
        ten_frames(&mut cx);
        assert_eq!(
            steps() > before,
            runs,
            "--tab {tab}: the animation stepped {} times in ten frame durations",
            steps() - before
        );
    }
}

/// A role the icon set has no icon for says so and shows the placeholder,
/// never another set's icon: gpui-component has none for Shield, and the
/// cell's info claims the placeholder's colour and no icon's.
#[gpui::test]
fn a_missing_icon_says_it_is_missing(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show_icons(&mut cx, &showcase, "gpui-component built-in (Lucide)");
    let info = settle_on(&mut cx, &showcase, ICONS_NATIVE_SHIELD);
    assert_eq!(
        info.as_ref().map(|info| info.title()).as_deref(),
        Some("IconRole · Shield")
    );
    let resolved = info.as_ref().and_then(|info| {
        info.instance
            .iter()
            .find(|n| n.what == "resolved")
            .map(|n| n.text.clone())
    });
    assert!(
        resolved
            .as_deref()
            .is_some_and(|r| r.starts_with("None: gpui-builtin has no icon")),
        "the Shield cell does not say gpui-builtin lacks it: {resolved:?}"
    );
    let roles: Vec<&str> = info
        .as_ref()
        .map(|info| info.colors.iter().map(|c| c.role).collect())
        .unwrap_or_default();
    assert!(
        roles.contains(&"missing-icon placeholder") && !roles.iter().any(|r| r.starts_with("icon")),
        "the Shield cell claims an icon's colour, or no placeholder: {roles:?}"
    );
}

/// The icons taken from the chosen icon theme: the toolbar buttons', the
/// status bar's side-panel toggle's, the command palette's entries', the
/// theme-error Alert's and the Layout page's Sidebar samples' items'. The
/// toolbar's are named in chrome::toolbar, the toggle's in
/// chrome::status_bar, the pages' in `Page::icon`, the palette's in
/// chrome::palette_groups, the Alert's where app.rs draws it, and the
/// samples' in `LAYOUT_SIDEBAR_ITEMS`.
fn chrome_icon_names() -> Vec<IconName> {
    let mut names = vec![
        IconName::CircleX,
        IconName::SquareTerminal,
        IconName::RotateCw,
        IconName::PanelLeft,
        IconName::Palette,
        IconName::Settings,
        IconName::Sun,
        IconName::Moon,
    ];
    names.extend(Page::ALL.map(Page::icon));
    names.extend(LAYOUT_SIDEBAR_ITEMS.map(|(_, icon, ..)| icon));
    names
}

/// The chrome's icons come from the chosen icon theme, and from no other
/// (the maintainer's rule: never mix icon themes). With Material chosen, every
/// chrome icon is the SVG Material's gallery holds for its IconName, as the
/// Icons page loads it, or none -- never gpui-component's own -- and the
/// toolbar's Command Palette button says which. With gpui-component's
/// built-in set chosen, every one is gpui-component's own, which also
/// proves each is in the gallery the lookup searches.
#[gpui::test]
fn the_chrome_icons_come_from_the_chosen_set(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let choose = |cx: &mut VisualTestContext, display: &str| {
        cx.update(|window, cx| {
            showcase.update(cx, |this, cx| this.select_icon_set(display, window, cx));
        });
        cx.run_until_parked();
        draw(cx);
    };
    choose(&mut cx, "gpui-component built-in (Lucide)");
    for name in chrome_icon_names() {
        let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(&name));
        assert!(
            matches!(drawn, ChromeIcon::Builtin(_)),
            "with the built-in set chosen, a chrome icon is not gpui-component's own: {drawn:?}"
        );
    }

    choose(&mut cx, "Material (bundled)");
    let material = load_gpui_icons(Some(native_theme::theme::IconSet::Material), None, None);
    for name in chrome_icon_names() {
        let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(&name));
        let path = gpui_component::IconNamed::path(name.clone());
        let entry = material
            .iter()
            .find(|(_, icon, ..)| gpui_component::IconNamed::path(icon.clone()) == path);
        let expected = match entry {
            Some((n, _, _, Some(native_theme::icons::IconData::Svg(bytes)), _)) => {
                ChromeIcon::Loaded(n, bytes.clone())
            }
            Some((n, ..)) => ChromeIcon::Missing(n),
            None => ChromeIcon::Unlisted(path),
        };
        assert_eq!(
            drawn, expected,
            "with Material chosen, a chrome icon is not Material's own, or not absent"
        );
    }
    for (selector, name) in [
        (CHROME_TOOLBAR_PALETTE, "SquareTerminal"),
        (CHROME_SIDE_PANEL_TOGGLE, "PanelLeft"),
    ] {
        let info = settle_on(&mut cx, &showcase, selector);
        let note = info.as_ref().and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "icon")
                .map(|n| n.text.clone())
        });
        assert!(
            note.as_deref()
                .is_some_and(|n| n.starts_with(&format!("material's icon for {name}"))
                    || n.starts_with(&format!("none: material holds no SVG for {name}"))),
            "with Material chosen, {selector} does not say its icon is Material's {name}: {note:?}"
        );
    }
}

/// A chrome icon the chosen set has none for is absent, not another set's:
/// with Material's SquareTerminal taken out of the loaded gallery, the
/// Command Palette button shows its tooltip's text and says why.
#[cfg(feature = "material-icons")] // the bundled Material set it chooses
#[gpui::test]
fn a_chrome_icon_the_set_lacks_is_absent(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Material (bundled)", window, cx);
            for entry in &mut this.gpui_icons {
                if entry.0 == "SquareTerminal" {
                    entry.3 = None;
                }
            }
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this
            .chrome_icon(&IconName::SquareTerminal)),
        ChromeIcon::Missing("SquareTerminal")
    );
    // Drawn, not only reported: the label is wider than the icon the
    // Reload button beside it still shows.
    let (labelled, iconic) = (
        bounds_of(&mut cx, CHROME_TOOLBAR_PALETTE),
        bounds_of(&mut cx, CHROME_TOOLBAR_RELOAD),
    );
    assert!(
        labelled.size.width > iconic.size.width,
        "the Command Palette button at {labelled:?} is no wider than the Reload icon \
         button at {iconic:?}, so it drew something in place of its label"
    );
    let info = settle_on(&mut cx, &showcase, CHROME_TOOLBAR_PALETTE);
    assert_eq!(
        info.as_ref().map(|info| info.title()).as_deref(),
        Some("Button · Ghost, labelled"),
        "the Command Palette button with no icon of the chosen set is not labelled"
    );
    let note = info.as_ref().and_then(|info| {
        info.instance
            .iter()
            .find(|n| n.what == "icon")
            .map(|n| n.text.clone())
    });
    assert!(
        note.as_deref()
            .is_some_and(|n| n.starts_with("none: material holds no SVG for SquareTerminal")),
        "the Command Palette button does not say material has no icon for it: {note:?}"
    );
    assert!(
        info.as_ref()
            .is_some_and(|info| !info.not_themeable.iter().any(|n| n.what == "icon")),
        "the labelled Command Palette button still describes an icon it does not draw: {info:?}"
    );
}

/// The theme-error Alert's icon is the chosen set's CircleX, or none: with
/// Material chosen it reports Material's, and with that icon taken out of
/// the loaded gallery it reports none, as `Showcase::chrome_icon` says,
/// never gpui-component's own. What an Alert draws for its icon is an
/// SVG sprite, which no test reads from the frame (gpui-pre window.rs has
/// `painted_quads` and `painted_underlines` only), so the check is on the
/// `ChromeIcon` app.rs hands `demo::alert`, which both its drawing and its
/// info are built from.
#[cfg(feature = "material-icons")] // the bundled Material set it chooses
#[gpui::test]
fn the_theme_error_alerts_icon_follows_the_chosen_set(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let alert_icon_note = |cx: &mut VisualTestContext| {
        settle_on(cx, &showcase, CONTENT_ALERT).and_then(|info| {
            info.instance
                .iter()
                .find(|n| n.what == "icon")
                .map(|n| n.text.clone())
        })
    };
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Material (bundled)", window, cx)
        });
    });
    use_missing_preset(&mut cx, &showcase, "no-such-preset");
    let expected = read(&mut cx, &showcase, |this, _| {
        this.chrome_icon(&IconName::CircleX)
    });
    assert!(
        matches!(expected, ChromeIcon::Loaded("CircleX", _)),
        "Material has no CircleX in its gallery, so this proves nothing: {expected:?}"
    );
    let note = alert_icon_note(&mut cx);
    assert!(
        note.as_deref()
            .is_some_and(|n| n.starts_with("material's icon for CircleX")),
        "with Material chosen, the theme-error Alert does not report Material's CircleX: {note:?}"
    );

    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            for entry in &mut this.gpui_icons {
                if entry.0 == "CircleX" {
                    entry.3 = None;
                }
            }
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    let note = alert_icon_note(&mut cx);
    assert!(
        note.as_deref()
            .is_some_and(|n| n.starts_with("none: material holds no SVG for CircleX")),
        "with Material's CircleX gone, the theme-error Alert does not report no icon: {note:?}"
    );
}

/// The page samples that show an icon, as (the page they are on, the
/// selector of the info that reports the icon, the IconName they show): the
/// Buttons page's icon Button, its loading Button's spinner and its Toggle,
/// the Inputs page's input groups, the Data page's Attachment, the Feedback
/// page's Empty state and Plain Marker, the Layout page's Collapsible toggle
/// (open at start) and icon Stepper, and the Overlays page's menu rows.
const PAGE_SAMPLE_ICONS: [(Page, &str, IconName); 14] = [
    (Page::Buttons, "buttons-icon-search", IconName::Search),
    (Page::Buttons, "buttons-loading", IconName::Loader),
    (Page::Buttons, "buttons-toggle-star", IconName::Star),
    (Page::Inputs, "inputs-input-groups", IconName::Search),
    (Page::Inputs, "inputs-input-groups", IconName::Copy),
    (Page::Data, "data-attachment-complete", IconName::Inbox),
    (Page::Feedback, "feedback-empty", IconName::Inbox),
    (
        Page::Feedback,
        "feedback-marker-plain",
        IconName::CircleCheck,
    ),
    (
        Page::Layout,
        LAYOUT_COLLAPSIBLE_TOGGLE,
        IconName::ChevronDown,
    ),
    (Page::Layout, "layout-stepper-horizontal", IconName::Search),
    (
        Page::Layout,
        "layout-stepper-horizontal",
        IconName::Settings,
    ),
    (
        Page::Layout,
        "layout-stepper-horizontal",
        IconName::CircleCheck,
    ),
    (Page::Overlays, "overlays-menu-row-undo", IconName::Undo),
    (Page::Overlays, "overlays-menu-row-delete", IconName::Delete),
];

/// How a Widget Info's note on `drawn`, of the icon theme named `set`,
/// begins (`info::chrome::chrome_icon_note`).
fn icon_note_start(drawn: &ChromeIcon, set: &str) -> String {
    match drawn {
        ChromeIcon::Builtin(name) => format!("gpui-component's own {name}"),
        ChromeIcon::Loaded(name, _) | ChromeIcon::Raster(name, _) => {
            format!("{set}'s icon for {name}")
        }
        ChromeIcon::Missing(name) => format!("none: {set} holds no SVG for {name}"),
        ChromeIcon::Unlisted(path) => format!("none: {path} is not in the Icons page's gallery"),
    }
}

/// Whether a note of `info` -- a not-themeable line or an instance note --
/// holds `text`.
fn info_says(info: &WidgetInfo, text: &str) -> bool {
    info.not_themeable
        .iter()
        .chain(&info.instance)
        .any(|n| n.text.contains(text))
}

/// Choose the icon theme the theme settings' Select names `display`.
fn choose_icon_theme(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, display: &str) {
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| this.select_icon_set(display, window, cx));
    });
    cx.run_until_parked();
    draw(cx);
}

/// Take `names` out of the loaded gallery, as if the chosen icon theme had
/// no icon for them.
#[cfg(feature = "material-icons")]
fn remove_icons(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, names: &[&str]) {
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            for entry in &mut this.gpui_icons {
                if names.contains(&entry.0) {
                    entry.3 = None;
                }
            }
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(cx);
}

/// Every page sample's icon is the chosen icon theme's, as the chrome's
/// are (the maintainer's decision D1): with gpui-component's built-in set
/// chosen, gpui-component's own; with Material chosen, the SVG Material's
/// gallery holds for its IconName, or none -- and each sample's info says
/// which. An `Icon`'s source is private to gpui-component, and gpui keeps
/// no record of the SVG sprites a frame paints that a test can read
/// (gpui-pre window.rs has `painted_quads` and `painted_underlines` only),
/// so the check is on the `SampleIcon` the page hands its sample, which
/// both the drawing and the info are built from.
#[gpui::test]
fn the_page_samples_icons_come_from_the_chosen_set(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    choose_icon_theme(&mut cx, &showcase, "gpui-component built-in (Lucide)");
    for (page, selector, name) in PAGE_SAMPLE_ICONS {
        show(&mut cx, &showcase, page);
        let icon = read(&mut cx, &showcase, |this, _| this.sample_icon(name.clone()));
        assert!(
            matches!(icon.drawn, ChromeIcon::Builtin(_)),
            "with the built-in set chosen, {selector}'s icon is not gpui-component's own: {:?}",
            icon.drawn
        );
        let info = settle_on(&mut cx, &showcase, selector);
        let start = icon_note_start(&icon.drawn, &icon.set);
        assert!(
            info.as_ref().is_some_and(|info| info_says(info, &start)),
            "{selector} does not say its icon is {start:?}: {info:?}"
        );
    }

    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    let material = load_gpui_icons(Some(native_theme::theme::IconSet::Material), None, None);
    for (page, selector, name) in PAGE_SAMPLE_ICONS {
        show(&mut cx, &showcase, page);
        let icon = read(&mut cx, &showcase, |this, _| this.sample_icon(name.clone()));
        let path = gpui_component::IconNamed::path(name.clone());
        let expected = match material
            .iter()
            .find(|(_, icon, ..)| gpui_component::IconNamed::path(icon.clone()) == path)
        {
            Some((n, _, _, Some(native_theme::icons::IconData::Svg(bytes)), _)) => {
                ChromeIcon::Loaded(n, bytes.clone())
            }
            Some((n, ..)) => ChromeIcon::Missing(n),
            None => ChromeIcon::Unlisted(path),
        };
        assert_eq!(
            icon.drawn, expected,
            "with Material chosen, {selector}'s icon is not Material's own, or not absent"
        );
        let info = settle_on(&mut cx, &showcase, selector);
        let start = icon_note_start(&icon.drawn, "material");
        assert!(
            info.as_ref().is_some_and(|info| info_says(info, &start)),
            "with Material chosen, {selector} does not say its icon is {start:?}: {info:?}"
        );
    }
}

/// A page sample whose icon the chosen icon theme lacks draws none, never
/// another icon theme's, and says so: with the samples' icons taken out of
/// Material's loaded gallery, every sample's info reports none, the icon
/// Button loses the width its icon took, and the Toggle shows its icon's
/// name instead.
#[cfg(feature = "material-icons")] // the bundled Material set it chooses
#[gpui::test]
fn a_page_sample_icon_the_set_lacks_is_absent(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    show(&mut cx, &showcase, Page::Buttons);
    let (button, toggle) = (
        bounds_of(&mut cx, "buttons-icon-search"),
        bounds_of(&mut cx, "buttons-toggle-star"),
    );

    let names: Vec<&'static str> = PAGE_SAMPLE_ICONS
        .iter()
        .filter_map(
            |(.., name)| match read(&mut cx, &showcase, |this, _| this.chrome_icon(name)) {
                ChromeIcon::Loaded(n, _) | ChromeIcon::Missing(n) => Some(n),
                _ => None,
            },
        )
        .collect();
    assert_eq!(
        names.len(),
        PAGE_SAMPLE_ICONS.len(),
        "a sample's icon is not in the gallery, so taking it out proves nothing"
    );
    remove_icons(&mut cx, &showcase, &names);

    for (page, selector, name) in PAGE_SAMPLE_ICONS {
        show(&mut cx, &showcase, page);
        let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(&name));
        assert!(
            matches!(drawn, ChromeIcon::Missing(_)),
            "{selector}'s icon was taken out of the gallery, and it is still {drawn:?}"
        );
        let info = settle_on(&mut cx, &showcase, selector);
        let start = icon_note_start(&drawn, "material");
        assert!(
            info.as_ref().is_some_and(|info| info_says(info, &start)),
            "{selector} does not say it has no icon, {start:?}: {info:?}"
        );
    }

    show(&mut cx, &showcase, Page::Buttons);
    let without = bounds_of(&mut cx, "buttons-icon-search");
    assert!(
        without.size.width < button.size.width,
        "the Search Button at {without:?} is no narrower than with its icon at {button:?}"
    );
    let info = settle_on(&mut cx, &showcase, "buttons-toggle-star");
    assert_eq!(
        info.as_ref().map(|info| info.title()).as_deref(),
        Some("Toggle · Ghost, labelled"),
        "the Star Toggle with no icon of the chosen set is not labelled"
    );
    let labelled = bounds_of(&mut cx, "buttons-toggle-star");
    assert!(
        labelled.size.width != toggle.size.width,
        "the Star Toggle at {labelled:?} is as wide as with its icon at {toggle:?}, so it shows no label"
    );
}

/// The chevrons the Basic page builds itself -- the theme-drawn expander's
/// arrows and the number field's steps -- are the chosen icon theme's, as
/// the page samples' are (M9-002): under kde-breeze with Material chosen,
/// the expander's info names Material's icon for its arrow (ChevronRight,
/// before the title, as kde-breeze's `expander.arrow_side` says) and the
/// number field's its ChevronUp and ChevronDown steps; with those icons
/// taken out of Material's loaded gallery, each info says none and the
/// expander draws no arrow, never gpui-component's own.
#[cfg(feature = "material-icons")] // the bundled Material set it chooses
#[gpui::test]
fn the_basic_pages_own_chevrons_come_from_the_chosen_set(cx: &mut TestAppContext) {
    own_chevrons_come_from_the_chosen_set(cx, "kde-breeze", true);
}

/// [`the_basic_pages_own_chevrons_come_from_the_chosen_set`] for a
/// trailing arrow, ChevronDown after the title: adwaita's.
#[cfg(feature = "material-icons")] // the bundled Material set it chooses
#[gpui::test]
fn a_trailing_expander_arrow_comes_from_the_chosen_set(cx: &mut TestAppContext) {
    own_chevrons_come_from_the_chosen_set(cx, "adwaita", false);
}

/// The Basic page's own chevrons under `preset`, whose expander arrow is
/// `leading` or trailing, with Material chosen and then with them taken
/// out of its gallery.
#[cfg(feature = "material-icons")]
fn own_chevrons_come_from_the_chosen_set(cx: &mut TestAppContext, preset: &str, leading: bool) {
    use native_theme::theme::ArrowSide;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    use_preset(&mut cx, &showcase, preset);
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    show(&mut cx, &showcase, Page::Basic);
    let side = read(&mut cx, &showcase, |_, cx| {
        native_value(cx, |n| n.resolved.expander.arrow_side)
    })
    .flatten();
    assert_eq!(
        side == Some(ArrowSide::Leading),
        leading,
        "{preset}'s expander.arrow_side is {side:?}, not the side this case tests"
    );
    let arrow = if leading {
        (IconName::ChevronRight, "ChevronRight")
    } else {
        (IconName::ChevronDown, "ChevronDown")
    };
    let cases = [
        ("basic-expander-header-0", arrow),
        ("basic-number", (IconName::ChevronUp, "ChevronUp")),
        ("basic-number", (IconName::ChevronDown, "ChevronDown")),
    ];
    let mut names = Vec::new();
    for (selector, (name, label)) in cases.clone() {
        let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(&name));
        let ChromeIcon::Loaded(n, _) = drawn else {
            panic!("Material has no {label} in its gallery, so this proves nothing: {drawn:?}");
        };
        names.push(n);
        let info = settle_on(&mut cx, &showcase, selector);
        let start = icon_note_start(&drawn, "material");
        assert!(
            info.as_ref().is_some_and(|info| info_says(info, &start)),
            "with Material chosen, {selector} does not say its {label} is {start:?}: {info:?}"
        );
    }
    assert!(
        cx.debug_bounds("basic-expander-arrow-0").is_some(),
        "the expander draws no arrow with Material's chevron in its gallery"
    );

    remove_icons(&mut cx, &showcase, &names);
    show(&mut cx, &showcase, Page::Basic);
    for (selector, (name, label)) in cases {
        let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(&name));
        assert!(
            matches!(drawn, ChromeIcon::Missing(_)),
            "{label} was taken out of the gallery, and it is still {drawn:?}"
        );
        let info = settle_on(&mut cx, &showcase, selector);
        let start = icon_note_start(&drawn, "material");
        assert!(
            info.as_ref().is_some_and(|info| info_says(info, &start)),
            "{selector} does not say it has no {label}, {start:?}: {info:?}"
        );
    }
    for arrow in ["basic-expander-arrow-0", "basic-expander-arrow-1"] {
        assert!(
            cx.debug_bounds(arrow).is_none(),
            "{arrow} is drawn with Material's chevron gone from its gallery, so it is another icon theme's"
        );
    }
}

/// The Overlays page's dialogs show the chosen icon theme's icon, or none:
/// the Dialog's CircleX beside its description and the AlertDialog's
/// TriangleAlert, each reported by the dialog's info, with Material chosen
/// and then with those icons taken out of its loaded gallery.
#[cfg(feature = "material-icons")] // the bundled Material set it chooses
#[gpui::test]
fn the_dialog_samples_icons_follow_the_chosen_set(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    let dialogs = [
        (
            OVERLAYS_DIALOG_TRIGGER,
            "overlays-dialog-content",
            IconName::CircleX,
        ),
        (
            PROBE_ALERT_DIALOG,
            "overlays-alert-dialog-title",
            IconName::TriangleAlert,
        ),
    ];
    for removed in [false, true] {
        if removed {
            remove_icons(&mut cx, &showcase, &["CircleX", "TriangleAlert"]);
        }
        for (trigger, selector, name) in dialogs.clone() {
            show(&mut cx, &showcase, Page::Overlays);
            click(&mut cx, trigger);
            draw(&mut cx);
            assert!(a_dialog_is_open(&mut cx), "{trigger} opened no dialog");
            let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(&name));
            assert_eq!(
                drawn.shown(),
                !removed,
                "{selector}'s icon is {drawn:?} with the gallery's taken out: {removed}"
            );
            let info = settle_on(&mut cx, &showcase, selector);
            let start = icon_note_start(&drawn, "material");
            assert!(
                info.as_ref().is_some_and(|info| info_says(info, &start)),
                "{selector} does not say its icon is {start:?}: {info:?}"
            );
            press(&mut cx, "escape");
            assert!(
                !a_dialog_is_open(&mut cx),
                "Escape did not close {trigger}'s dialog"
            );
        }
    }
}

/// The Overlays page's dialogs draw a raster set's icon as an image
/// (M9-163): the Dialog's CircleX beside its description and the
/// AlertDialog's TriangleAlert, which takes any element as its icon.
#[cfg(feature = "material-icons")] // the bundled set the pixels stand in for
#[gpui::test]
fn the_dialog_samples_draw_a_raster_sets_icon(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    rasterize_icons(&mut cx, &showcase, &["CircleX", "TriangleAlert"], &[]);
    for (trigger, selector, image) in [
        (
            OVERLAYS_DIALOG_TRIGGER,
            "overlays-dialog-content",
            "raster-CircleX",
        ),
        (
            PROBE_ALERT_DIALOG,
            "overlays-alert-dialog-icon",
            "raster-TriangleAlert",
        ),
    ] {
        show(&mut cx, &showcase, Page::Overlays);
        click(&mut cx, trigger);
        draw(&mut cx);
        assert!(a_dialog_is_open(&mut cx), "{trigger} opened no dialog");
        let frame = bounds_of(&mut cx, selector);
        let at = raster_bounds(&mut cx, image);
        assert!(
            at.is_some_and(|at| within(at, frame)),
            "{selector} at {frame:?} holds no {image}: {at:?}"
        );
        press(&mut cx, "escape");
        assert!(
            !a_dialog_is_open(&mut cx),
            "Escape did not close {trigger}'s dialog"
        );
    }
}

/// The Buttons page's component Toolbar lays its two groups out in one row:
/// six Buttons side by side and apart, the vertical Separator between Copy
/// and Search as tall as the Buttons (a zero-height rule would be invisible),
/// and the Inputs page's TimeField and token Input laid out with a size.
#[gpui::test]
fn the_new_widgets_lay_out(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Buttons);
    let ids = [
        "buttons-toolbar-undo",
        "buttons-toolbar-redo",
        "buttons-toolbar-copy",
        "buttons-toolbar-search",
        "buttons-toolbar-maximize",
        "buttons-toolbar-settings",
    ];
    let buttons: Vec<Bounds<Pixels>> = ids.iter().map(|id| bounds_of(&mut cx, id)).collect();
    for (id, b) in ids.iter().zip(&buttons) {
        assert!(
            b.size.width > px(0.) && b.size.height > px(0.),
            "{id} has no size: {b:?}"
        );
    }
    for pair in buttons.windows(2) {
        if let [left, right] = pair {
            assert!(
                left.right() <= right.left() && (left.center().y - right.center().y).abs() < px(1.),
                "the toolbar's Buttons are not one row of separate boxes: {left:?} then {right:?}"
            );
        }
    }
    let separator = bounds_of(&mut cx, "buttons-toolbar-separator");
    let (copy, search) = (buttons[2], buttons[3]);
    assert!(
        copy.right() <= separator.left() && separator.left() <= search.left(),
        "the Separator {separator:?} is not between Copy {copy:?} and Search {search:?}"
    );
    assert!(
        separator.size.height >= copy.size.height,
        "the Separator {separator:?} is shorter than the Buttons {copy:?}"
    );
    show(&mut cx, &showcase, Page::Inputs);
    for id in ["inputs-time-field", "inputs-token-input"] {
        let b = bounds_of(&mut cx, id);
        assert!(
            b.size.width > px(0.) && b.size.height > px(0.),
            "{id} has no size: {b:?}"
        );
    }
}

/// The Inputs page's token Input holds the one token `Showcase::new` inserts
/// (spec §8.3): a refused token only prints a line, so this is what notices
/// it.
#[gpui::test]
fn the_token_input_holds_its_token(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let tokens = read(&mut cx, &showcase, |this, cx| {
        this.token_input_state
            .read(cx)
            .tokens()
            .iter()
            .map(|span| span.token().text().to_string())
            .collect::<Vec<_>>()
    });
    assert_eq!(
        tokens,
        ["@native-theme"],
        "the token Input holds {tokens:?}"
    );
}

/// Every sample whose gpui-component widget draws icons of its own says
/// they are gpui-component's whichever icon theme is chosen (decision D1):
/// upstream loads them by asset path, and gpui keeps the first SVG it drew
/// for a path (Task 6's spike, `docs/todo.md`), so they cannot follow the
/// chosen theme. The widgets are those whose shown configuration draws one
/// (gpui-component 0.8.1): a DropdownButton's caret, a Clipboard's Copy, a
/// NumberInput's steps, a Checkbox's check, a Rating's stars, a Select's,
/// a Combobox's and a ColorSelect's caret, a DatePicker's Calendar, a
/// Calendar's month buttons, the command palette's Search, a Dialog's and a
/// Sheet's close button, a Settings page's search and reset, a Pagination's
/// arrows, the MessageScroller's jump button, an Alert's and a
/// Notification's severity icon, a Spinner's Loader, the Marker's spinner,
/// an Accordion's chevrons, the Carousel's controls, the Breadcrumb's
/// separators, the Editor's fold buttons, the Diff's collapse button, the
/// DataTable's sort button, and an Attachment's status glyph (Ban when
/// failed, a Spinner while uploading or processing).
#[gpui::test]
fn a_widgets_own_icons_are_named_gpui_components(cx: &mut TestAppContext) {
    use crate::demo::{ButtonKind, MarkerKind, Severity, SheetSide, SpinnerKind};
    use crate::info::{buttons, chrome, data, feedback, inputs, layout, overlays, typography};
    use gpui_component::attachment::AttachmentStatus;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let t = cx.update(|_window, cx| Theme::global(cx).clone());
    let icon = read(&mut cx, &showcase, |this, _| {
        this.sample_icon(IconName::CircleX)
    });
    let infos: Vec<(&str, WidgetInfo)> = vec![
        (
            "DropdownButton",
            buttons::dropdown_button(&t, ButtonKind::Primary),
        ),
        ("Clipboard", buttons::clipboard(&t, "value")),
        ("NumberInput", inputs::number_input(&t)),
        ("Checkbox", inputs::checkbox(&t, "label", true, false)),
        ("Rating", inputs::rating(&t, 3, false)),
        ("Select", inputs::select(&t, None)),
        ("ColorSelect", inputs::color_select(&t, false)),
        ("DatePicker", inputs::date_picker(&t)),
        ("Calendar", inputs::calendar(&t)),
        ("preset Combobox", chrome::preset_combobox(&t, None)),
        ("colour-mode Select", chrome::color_mode_select(&t, None)),
        ("icon-theme Select", chrome::icon_set_select(&t, None)),
        ("command palette", chrome::command_palette(&t, "set")),
        ("Preferences Sheet", chrome::preferences_sheet(&t, false)),
        (
            "About Dialog",
            chrome::about_dialog(&t, false, false, false, false),
        ),
        ("Settings", layout::settings(&t)),
        ("Pagination", data::pagination(&t, false, 1, 12, false)),
        ("MessageScroller", data::message_scroller(&t, 3)),
        ("Alert", feedback::severity_alert(&t, Severity::Info)),
        (
            "Notification",
            feedback::notification(&t, Severity::Success, "label", "message"),
        ),
        (
            "Spinner",
            feedback::spinner(&t, SpinnerKind::Small, false, false, None),
        ),
        (
            "Marker spinner",
            feedback::marker(&t, MarkerKind::Spinner, "text", false, None),
        ),
        ("Accordion", layout::accordion(&t, false, 3, false)),
        ("Carousel", layout::carousel(&t, false)),
        (
            "Breadcrumb",
            layout::breadcrumb(&t, &[Page::Buttons], Page::Layout),
        ),
        ("Dialog", overlays::dialog(&t, false, false, false, &icon)),
        ("Sheet", overlays::sheet(&t, SheetSide::Right, false)),
        ("Editor", typography::editor(&t)),
        ("Diff", typography::diff(&t)),
        ("DataTable", data::data_table(&t, 5, 3, false, false)),
        (
            "failed Attachment",
            data::attachment(&t, AttachmentStatus::Failed, "file", false, &icon),
        ),
        (
            "uploading Attachment",
            data::attachment(&t, AttachmentStatus::Uploading, "file", false, &icon),
        ),
    ];
    for (name, info) in infos {
        let note = info.not_themeable.iter().find(|n| n.what == "own icons");
        assert!(
            note.is_some_and(|n| n
                .text
                .contains("gpui-component's own icons whichever icon theme is chosen")),
            "{name} does not say the icons it draws of its own are gpui-component's: {info:?}"
        );
    }
}

/// Paint-level check (rationale §3.6): the fill gpui painted inside the
/// Primary Tag at rest is the colour its info's bg claim shows. The colour
/// gate reads the line a claim cites; this reads the frame. At rest, because
/// a hovered Tag paints at 90% (tag.rs:269).
#[gpui::test]
fn a_tags_painted_fill_is_its_bg_claim(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Feedback);
    let info = settle_on(&mut cx, &showcase, FEEDBACK_TAG_PRIMARY);
    let claimed = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role == "bg"))
        .map(|c| c.value);
    let tag = bounds_of(&mut cx, FEEDBACK_TAG_PRIMARY);
    hover(&mut cx, point(tag.left() - px(40.), tag.center().y));
    draw(&mut cx);
    let painted = painted_fill(&mut cx, FEEDBACK_TAG_PRIMARY);
    assert!(
        claimed.is_some(),
        "the Primary Tag's info claims no bg: {info:?}"
    );
    assert_eq!(
        painted, claimed,
        "the fill painted inside the Primary Tag is not its bg claim"
    );
}

/// The fill gpui painted last frame inside the element tagged `selector`.
///
/// gpui paints a bordered box as its fill and, apart, its border strips over
/// a transparent fill (gpui-pre window.rs, Window::paint_quad), so the fill
/// is the largest quad inside the element's bounds that is not transparent.
/// `layer` over `base`, straight-alpha source-over in sRGB, written out here
/// so a layered fill the showcase paints is checked against the rule rather
/// than against `native_theme_gpui::layer_over`, which it calls.
fn source_over(base: gpui::Hsla, layer: gpui::Hsla) -> gpui::Hsla {
    if layer.a >= 1.0 {
        return layer;
    }
    if layer.a <= 0.0 {
        return base;
    }
    let (b, l) = (gpui::Rgba::from(base), gpui::Rgba::from(layer));
    let under = b.a * (1.0 - l.a);
    let a = l.a + under;
    let mix = |below: f32, above: f32| ((above * l.a) + (below * under)) / a;
    gpui::Rgba {
        r: mix(b.r, l.r),
        g: mix(b.g, l.g),
        b: mix(b.b, l.b),
        a,
    }
    .into()
}

fn painted_fill(cx: &mut VisualTestContext, selector: &'static str) -> Option<gpui::Hsla> {
    let bounds = bounds_of(cx, selector);
    cx.update(|window, _cx| {
        let bounds = bounds.scale(window.scale_factor());
        scene_quads(window)
            .into_iter()
            .filter(|q| {
                !q.background.is_transparent()
                    && q.bounds.left() >= bounds.left()
                    && q.bounds.top() >= bounds.top()
                    && q.bounds.right() <= bounds.right()
                    && q.bounds.bottom() <= bounds.bottom()
            })
            .max_by(|a, b| {
                let area = |q: &gpui::Quad| q.bounds.size.width.0 * q.bounds.size.height.0;
                area(a).total_cmp(&area(b))
            })
            .and_then(|q| q.background.as_solid())
    })
}

/// A Basic text field under the pointer is framed by
/// `input.hover_border_color`, at rest by `input.border.color` (DIFFS H5),
/// under a preset that states the hover edge.
#[gpui::test]
fn a_hovered_field_takes_the_hover_border(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let (edge, hovered_edge) = read(&mut cx, &showcase, |_, cx| {
        (
            native_color(cx, |n| n.resolved.input.border.color),
            native_value(cx, |n| n.resolved.input.hover_border_color)
                .flatten()
                .map(crate::info::stated),
        )
    });
    assert!(
        hovered_edge.is_some(),
        "kde-breeze states input.hover_border_color"
    );
    let field = "basic-input-placeholder";
    let edge_of = |cx: &mut VisualTestContext| {
        let bounds = bounds_of(cx, field);
        cx.update(|window, _| {
            let bounds = bounds.scale(window.scale_factor());
            scene_quads(window)
                .into_iter()
                .find(|q| q.bounds == bounds && q.border_widths.top.0 > 0.)
                .map(|q| q.border_color)
        })
    };
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert_eq!(
        edge_of(&mut cx),
        edge,
        "a field at rest is not input.border.color"
    );
    let at = bounds_of(&mut cx, field).center();
    hover(&mut cx, at);
    draw(&mut cx);
    assert_eq!(
        edge_of(&mut cx),
        hovered_edge,
        "a hovered field is not input.hover_border_color"
    );
}

/// The border colour of the largest box with a border painted exactly over
/// `selector`'s bounds.
fn painted_edge(cx: &mut VisualTestContext, selector: &'static str) -> Option<gpui::Hsla> {
    let bounds = bounds_of(cx, selector);
    cx.update(|window, _cx| {
        let bounds = bounds.scale(window.scale_factor());
        scene_quads(window)
            .into_iter()
            .find(|q| q.bounds == bounds && q.border_widths.top.0 > 0.)
            .map(|q| q.border_color)
    })
}

/// Install the bundled `preset` in its light variant.
fn use_light_preset(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, preset: &str) {
    use_preset(cx, showcase, preset);
    cx.update(|window, cx| window.dispatch_action(Box::new(SetColorMode(AppColorMode::Light)), cx));
    cx.run_until_parked();
    draw(cx);
    assert!(
        !cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        "{preset} was not installed in its light variant"
    );
}

/// The colour `info` claims for `field`, where it claims one.
fn claim_of(info: &WidgetInfo, field: &str) -> Option<gpui::Hsla> {
    info.colors
        .iter()
        .find(|c| c.field == field)
        .map(|c| c.value)
}

/// The installed native theme, which the test installed.
fn resolved_theme(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
) -> native_theme_gpui::ResolvedTheme {
    read(cx, showcase, |_, cx| {
        native_value(cx, |n| n.resolved.clone())
    })
    .expect("a native theme is installed")
}

/// The Basic page's text area lists the routes its surface takes (M9-005):
/// its edge `text_area.border.color`, its hovered edge
/// `input.hover_border_color` and its fill `input.background_color` are
/// drawn by `demo::input_surface`, not left to upstream's `input` token or
/// the field's own fill.
#[test]
fn the_text_areas_routes_name_its_surface() {
    let field = "basic.text_area.field";
    for leaf in [
        "text_area.border.color",
        "input.hover_border_color",
        "input.background_color",
    ] {
        let how = crate::info::leaves::how(field, leaf);
        assert!(
            how.is_some_and(
                |line| line.contains("demo::input_surface") && !line.contains("not applied")
            ),
            "{field}'s route for {leaf} does not name the surface that draws it: {how:?}"
        );
    }
}

/// The Inputs page's fixed-height Textarea is drawn over the input surface
/// the Basic page's takes (M9-004): framed by `text_area.border.color`, and
/// by `input.hover_border_color` under the pointer, under a preset that
/// strokes the text area's line and states the hover edge.
#[gpui::test]
fn the_inputs_pages_textarea_takes_the_hover_edge(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Inputs);
    let r = resolved_theme(&mut cx, &showcase);
    let hovered_edge = r.input.hover_border_color.map(stated);
    assert!(
        r.text_area.border.line_width > 0.0 && hovered_edge.is_some(),
        "kde-breeze no longer strokes its text area or states input.hover_border_color, which this test reads"
    );
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert_eq!(
        painted_edge(&mut cx, INPUTS_TEXTAREA),
        Some(stated(r.text_area.border.color)),
        "the Textarea at rest is not framed by text_area.border.color"
    );
    let at = bounds_of(&mut cx, INPUTS_TEXTAREA).center();
    hover(&mut cx, at);
    draw(&mut cx);
    assert_eq!(
        painted_edge(&mut cx, INPUTS_TEXTAREA),
        hovered_edge,
        "the hovered Textarea is not framed by input.hover_border_color"
    );
}

/// Where the theme strokes no text-area line, the Inputs page's Textarea
/// has no edge, at rest or hovered: adwaita's `text_area.border.line_width`
/// is 0 (it inherits `input.border`'s).
#[gpui::test]
fn an_unstroked_textarea_draws_no_edge(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "adwaita");
    show(&mut cx, &showcase, Page::Inputs);
    let r = resolved_theme(&mut cx, &showcase);
    assert!(
        r.text_area.border.line_width <= 0.0,
        "adwaita now strokes its text area, so this test no longer runs the no-edge case"
    );
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert_eq!(
        painted_edge(&mut cx, INPUTS_TEXTAREA),
        None,
        "an edge is painted round the Textarea at rest, whose line width is 0"
    );
    let at = bounds_of(&mut cx, INPUTS_TEXTAREA).center();
    hover(&mut cx, at);
    draw(&mut cx);
    assert_eq!(
        painted_edge(&mut cx, INPUTS_TEXTAREA),
        None,
        "an edge is painted round the hovered Textarea, whose line width is 0"
    );
}

/// Both text areas' infos say what their surface draws (M9-004, M9-005,
/// M9-054): the edge `text_area.border.color`, the hovered edge
/// `input.hover_border_color`, and the surface line, with no claim of
/// upstream's `input` token, which the field's own edge no longer shows.
/// Under adwaita, whose line width is 0, they claim neither edge and say
/// so.
#[gpui::test]
fn a_textareas_info_claims_its_surfaces_edges(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    for (page, field) in [
        (Page::Inputs, INPUTS_TEXTAREA),
        (Page::Basic, "basic-textarea"),
    ] {
        use_preset(&mut cx, &showcase, "kde-breeze");
        show(&mut cx, &showcase, page);
        let r = resolved_theme(&mut cx, &showcase);
        let info = settle_on(&mut cx, &showcase, field);
        assert!(info.is_some(), "{field} shows no info");
        let Some(info) = info else { return };
        assert_eq!(
            claim_of(&info, "border"),
            Some(stated(r.text_area.border.color)),
            "{field}'s info does not claim the text_area.border.color its surface is edged with: {info:?}"
        );
        assert_eq!(
            claim_of(&info, "hover_border_color"),
            r.input.hover_border_color.map(stated),
            "{field}'s info does not claim the input.hover_border_color its surface takes under the pointer: {info:?}"
        );
        assert!(
            info.config
                .iter()
                .any(|n| n.what == "surface" && n.text.contains("text_area.border")),
            "{field}'s info carries no surface line naming text_area.border: {info:?}"
        );
        assert_eq!(
            claim_of(&info, "input"),
            None,
            "{field}'s info claims upstream's input token, which its own edge no longer shows: {info:?}"
        );
        use_preset(&mut cx, &showcase, "adwaita");
        show(&mut cx, &showcase, page);
        let info = settle_on(&mut cx, &showcase, field);
        assert!(info.is_some(), "{field} shows no info under adwaita");
        let Some(info) = info else { return };
        assert!(
            ["border", "hover_border_color", "input"]
                .iter()
                .all(|edge| claim_of(&info, edge).is_none()),
            "{field}'s info claims an edge adwaita does not stroke: {info:?}"
        );
        assert_eq!(
            info.config
                .iter()
                .find(|n| n.what == "edge")
                .map(|n| n.text.as_str()),
            Some("none: text_area.border.line_width is 0"),
            "{field}'s info does not say it strokes no edge: {info:?}"
        );
        let surface = info.config.iter().find(|n| n.what == "surface");
        assert!(
            surface.is_some_and(|n| !n.text.contains("framed by")),
            "{field}'s surface line names a frame adwaita does not stroke: {surface:?}"
        );
    }
}

/// A text area's surface is framed by `text_area.border.color`, not by the
/// `input.border.color` it inherits in every bundled preset: under
/// kde-breeze light with its text area's edge colour set to
/// `defaults.text_color`, a colour of the same theme that differs from
/// `input.border.color`, both text areas are edged in it at rest, and
/// their infos claim it.
#[gpui::test]
fn a_text_area_is_framed_by_its_own_edge_colour(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_light_preset(&mut cx, &showcase, "kde-breeze");
    let mut r = resolved_theme(&mut cx, &showcase);
    r.text_area.border.color = r.defaults.text_color;
    assert_ne!(
        r.text_area.border.color, r.input.border.color,
        "kde-breeze light's text colour is its field's edge colour, so the test could not tell the two apart"
    );
    assert!(
        r.text_area.border.line_width > 0.0,
        "kde-breeze no longer strokes its text area, which this test reads"
    );
    cx.update(|_window, cx| {
        let prefs = native_theme::AccessibilityPreferences::default();
        native_theme_gpui::apply(
            native_theme_gpui::to_theme(&r, "kde-breeze", false, &prefs),
            &r,
            &prefs,
            cx,
        );
    });
    cx.run_until_parked();
    draw(&mut cx);
    let edge = Some(stated(r.text_area.border.color));
    for (page, field) in [
        (Page::Inputs, INPUTS_TEXTAREA),
        (Page::Basic, "basic-textarea"),
    ] {
        show(&mut cx, &showcase, page);
        let info = settle_on(&mut cx, &showcase, field);
        assert!(info.is_some(), "{field} shows no info");
        let Some(info) = info else { return };
        assert_eq!(
            claim_of(&info, "border"),
            edge,
            "{field}'s info does not claim text_area.border.color: {info:?}"
        );
        hover(&mut cx, point(px(4.), px(4.)));
        draw(&mut cx);
        assert_eq!(
            painted_edge(&mut cx, field),
            edge,
            "{field} at rest is not framed by text_area.border.color"
        );
    }
}

/// Where the theme strokes no field line, a refined, enabled Input's info
/// claims no edge and says so (M9-055): under adwaita light
/// `input.border.line_width` is 0, so the surface strokes nothing, at rest
/// or under the pointer, and its surface line names no frame.
#[gpui::test]
fn an_unstroked_inputs_info_claims_no_edge(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_light_preset(&mut cx, &showcase, "adwaita");
    show(&mut cx, &showcase, Page::Inputs);
    let r = resolved_theme(&mut cx, &showcase);
    assert!(
        r.input.border.line_width <= 0.0,
        "adwaita now strokes its field, so this test no longer runs the no-edge case"
    );
    let info = settle_on(&mut cx, &showcase, INPUTS_FIELD);
    assert!(info.is_some(), "the field shows no info");
    let Some(info) = info else { return };
    assert!(
        ["border", "hover_border_color", "input"]
            .iter()
            .all(|edge| claim_of(&info, edge).is_none()),
        "the field's info claims an edge adwaita does not stroke: {info:?}"
    );
    assert_eq!(
        info.config
            .iter()
            .find(|n| n.what == "edge")
            .map(|n| n.text.as_str()),
        Some("none: input.border.line_width is 0"),
        "the field's info does not say it strokes no edge: {info:?}"
    );
    let surface = info.config.iter().find(|n| n.what == "surface");
    assert!(
        surface.is_some_and(|n| !n.text.contains("framed by")),
        "the field's surface line names a frame adwaita does not stroke: {surface:?}"
    );
}

/// The Layout page's Form fields are refined as the Inputs page's are
/// (M9-006): under kde-breeze light they are filled with
/// `input.background_color`, the view colour, not upstream's
/// `input_background()`, the window's.
#[gpui::test]
fn the_form_fields_take_the_input_fill(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_light_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Layout);
    let r = resolved_theme(&mut cx, &showcase);
    let fill = stated(r.input.background_color);
    assert_ne!(
        fill,
        stated(r.defaults.background_color),
        "kde-breeze light's field fill is the window's colour, so the fill could not be told from upstream's"
    );
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    for field in ["layout-form-name", "layout-form-email"] {
        assert_eq!(
            painted_fill(&mut cx, field),
            Some(fill),
            "the Form's {field} is not filled with input.background_color"
        );
    }
}

/// A refined, enabled Input's info claims the edge its surface draws,
/// `input.border.color`, as the showcase's own, and not upstream's `input`
/// token, which the field's own edge no longer shows (M9-055); the Inputs
/// page's field and the Layout page's Form field alike.
#[gpui::test]
fn a_refined_inputs_info_claims_its_surfaces_edge(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    for (page, field) in [
        (Page::Inputs, INPUTS_FIELD),
        (Page::Layout, "layout-form-name"),
    ] {
        show(&mut cx, &showcase, page);
        let r = resolved_theme(&mut cx, &showcase);
        assert!(
            r.input.border.line_width > 0.0,
            "kde-breeze no longer strokes its field, which this test reads"
        );
        let info = settle_on(&mut cx, &showcase, field);
        assert!(info.is_some(), "{field} shows no info");
        let Some(info) = info else { return };
        let edge = info.colors.iter().find(|c| c.field == "border");
        assert!(
            edge.is_some_and(
                |c| c.value == stated(r.input.border.color) && c.cited_at == "showcase"
            ),
            "{field}'s info does not claim the input.border.color its surface is edged with: {info:?}"
        );
        assert_eq!(
            claim_of(&info, "input"),
            None,
            "{field}'s info claims upstream's input token, which its own edge no longer shows: {info:?}"
        );
    }
}

/// Under a native theme `geometry::select` letters a Select's trigger in
/// `combo_box.font`'s colour, so its info claims no upstream `foreground`
/// trigger text (M9-055).
#[gpui::test]
fn a_native_selects_info_claims_no_upstream_trigger_text(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let info = settle_on(&mut cx, &showcase, "basic-select");
    assert!(info.is_some(), "the Select shows no info");
    assert_eq!(
        info.as_ref().and_then(|info| claim_of(info, "foreground")),
        None,
        "the Select's info claims upstream's foreground trigger text under a native theme: {info:?}"
    );
    let trigger = info
        .as_ref()
        .and_then(|info| info.instance.iter().find(|n| n.what == "trigger text"));
    assert!(
        trigger.is_some_and(
            |n| n.text.contains("combo_box.font") && n.text.contains("geometry::select")
        ),
        "the Select's info does not say geometry::select letters its trigger in combo_box.font's colour: {info:?}"
    );
    // Without a native theme no refinement letters it, and upstream's
    // foreground stands.
    let theme = cx.update(|_w, cx| Theme::global(cx).clone());
    let plain = crate::info::inputs::select(&theme, None);
    assert_eq!(
        claim_of(&plain, "foreground"),
        Some(theme.foreground),
        "the Select's info without a native theme does not claim upstream's foreground trigger text: {plain:?}"
    );
    assert!(
        plain.instance.iter().all(|n| n.what != "trigger text"),
        "the Select's info without a native theme names a refinement it does not take: {plain:?}"
    );
}

/// The Overlays page's menu rows, which the application draws itself, are
/// painted as the showcase's own menus are (M9-007): the frame in
/// `menu.background_color` and edged by `popover.border`, a hovered row in
/// `menu.hover_background`, not in the popover's fill. Under adwaita light
/// `menu.background_color` inherits `defaults.background_color`, which is
/// not the popover's fill, so the two can be told apart. The connector
/// fills the `accent` token the rows hovered in before from
/// `menu.hover_background` (src/colors.rs, `tc.accent`), so the test
/// overwrites the installed `accent` with a colour of its own: a row hovered
/// in the token would then show that colour, not the leaf's.
#[gpui::test]
fn the_application_drawn_menu_rows_take_the_menu_theme(cx: &mut TestAppContext) {
    use crate::demo::MENU_ROWS_FRAME;
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_light_preset(&mut cx, &showcase, "adwaita");
    show(&mut cx, &showcase, Page::Overlays);
    let r = resolved_theme(&mut cx, &showcase);
    let fill = stated(r.menu.background_color);
    assert_ne!(
        fill,
        stated(r.popover.background_color),
        "adwaita light's menu fill is the popover's, so this test cannot tell them apart"
    );
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert_eq!(
        painted_fill(&mut cx, MENU_ROWS_FRAME),
        Some(fill),
        "the menu rows' frame is not filled with menu.background_color"
    );
    assert_eq!(
        painted_edge(&mut cx, MENU_ROWS_FRAME),
        (r.popover.border.line_width > 0.0).then(|| stated(r.popover.border.color)),
        "the menu rows' frame is not edged as popover.border strokes it"
    );
    let hovered_fill = stated(r.menu.hover_background);
    let token = gpui::red();
    assert_ne!(
        token, hovered_fill,
        "adwaita light's menu.hover_background is the test's own accent colour"
    );
    cx.update(|window, cx| {
        Theme::global_mut(cx).accent = token;
        window.refresh();
    });
    cx.run_until_parked();
    let row = "overlays-menu-row-copy";
    let at = bounds_of(&mut cx, row).center();
    hover(&mut cx, at);
    draw(&mut cx);
    assert_eq!(
        painted_fill(&mut cx, row),
        Some(hovered_fill),
        "the hovered menu row is not filled with menu.hover_background"
    );
}

/// The Overlays page's menu rows' info claims the menu leaves the rows are
/// painted with (M9-007), and no popover or accent token; its frame's edge
/// where `popover.border` strokes one (adwaita), and where it strokes none
/// (material-3) a line saying so that names the radius still rounding the
/// frame.
#[gpui::test]
fn the_menu_rows_info_claims_the_menu_leaves(cx: &mut TestAppContext) {
    use crate::info::{px_text, stated};
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let mut branches = (false, false);
    for preset in ["adwaita", "material-3"] {
        use_light_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Overlays);
        let r = resolved_theme(&mut cx, &showcase);
        let info = settle_on(&mut cx, &showcase, "overlays-menu-row-copy");
        assert!(info.is_some(), "{preset}: the menu row shows no info");
        let Some(info) = info else { return };
        let by_role = |role: &str| {
            info.colors
                .iter()
                .find(|c| c.role == role)
                .map(|c| (c.field, c.value))
        };
        for (role, field, value) in [
            (
                "bg (the frame's)",
                "background_color",
                r.menu.background_color,
            ),
            ("text", "color", r.menu.font.color),
            ("hover", "hover_background", r.menu.hover_background),
            ("hover text", "hover_text_color", r.menu.hover_text_color),
        ] {
            assert_eq!(
                by_role(role),
                Some((field, stated(value))),
                "{preset}: the menu row's info does not claim menu.{field} as its {role}: {info:?}"
            );
        }
        for token in [
            "popover",
            "popover_foreground",
            "accent",
            "accent_foreground",
        ] {
            assert_eq!(
                claim_of(&info, token),
                None,
                "{preset}: the menu row's info claims the {token} token, which no longer paints it: {info:?}"
            );
        }
        if r.popover.border.line_width > 0.0 {
            branches.0 = true;
            assert_eq!(
                by_role("frame edge"),
                Some(("border", stated(r.popover.border.color))),
                "{preset}: the menu row's info does not claim the popover.border.color its frame is edged with: {info:?}"
            );
        } else {
            branches.1 = true;
            assert_eq!(
                by_role("frame edge"),
                None,
                "{preset}: the menu row's info claims an edge popover.border does not stroke: {info:?}"
            );
            let expected = format!(
                "none: popover.border.line_width is 0; the frame is still rounded by popover.border.corner_radius, {}px",
                px_text(r.popover.border.corner_radius)
            );
            assert_eq!(
                info.config
                    .iter()
                    .find(|n| n.what == "frame edge")
                    .map(|n| n.text.as_str()),
                Some(expected.as_str()),
                "{preset}: the menu row's info does not say its frame strokes no edge and what rounds it: {info:?}"
            );
        }
    }
    assert_eq!(
        branches,
        (true, true),
        "adwaita no longer strokes popover.border or material-3 now does, so a branch of the frame-edge info went unread"
    );
}

/// The chrome popup's rows and titles take `menu.font` as
/// `geometry::menu_item` sets it, its style included, and the rows the
/// builder's control-height rule on `menu.row_height` -- `h` at text scale
/// 1, `min_h` above it (M9-012) -- read from a fixture whose font is
/// italic.
///
/// It guards the plumbing: that `MenuLook::row` and `MenuLook::title` apply
/// the refinement `MenuLook::of` takes from `geometry::menu_item`. Both
/// sides of the comparison are that builder's, so it does not show that the
/// builder's font and height are the theme's: the connector's own geometry
/// tests (`src/geometry.rs`) hold that.
#[test]
fn the_menu_popups_rows_are_refined_as_geometry_menu_item() {
    use native_theme_gpui::Native;
    let resolved = native_theme::theme::Theme::preset("adwaita")
        .ok()
        .and_then(|t| t.into_variant(native_theme::theme::ColorMode::Light).ok())
        .and_then(|v| {
            v.into_resolved(&native_theme::ResolutionContext::for_tests())
                .ok()
        });
    assert!(resolved.is_some(), "adwaita light does not resolve");
    let Some(mut resolved) = resolved else { return };
    assert!(
        resolved.menu.row_height.is_some(),
        "adwaita no longer states menu.row_height, which this test reads"
    );
    resolved.menu.font.style = native_theme::theme::FontStyle::Italic;
    for scale in [1.0, 2.0] {
        let mut prefs = native_theme_gpui::AccessibilityPreferences::default();
        prefs.text_scaling_factor = scale;
        let n = Native {
            resolved: &resolved,
            accessibility: &prefs,
        };
        let expected = geometry::menu_item(n);
        assert_eq!(
            expected.text.font_style,
            Some(gpui::FontStyle::Italic),
            "geometry::menu_item no longer sets the italic fixture's style"
        );
        // The font as the builder sets it; the colour is the showcase's own,
        // which the builder leaves unset.
        let font = |style: &gpui::StyleRefinement| gpui::TextStyleRefinement {
            color: None,
            ..style.text.clone()
        };
        let look = crate::demo::MenuLook::of(&n);
        for lit in [false, true] {
            let mut row = look.row(lit);
            let style = row.style();
            assert_eq!(
                font(style),
                expected.text,
                "a popup row at text scale {scale} does not take menu.font as geometry::menu_item sets it"
            );
            assert_eq!(
                (style.size.height, style.min_size.height),
                (expected.size.height, expected.min_size.height),
                "a popup row at text scale {scale} is not sized by geometry::menu_item's height rule"
            );
        }
        let mut title = look.title(gpui::div().id("title"), false);
        assert_eq!(
            font(title.style()),
            expected.text,
            "a menu title at text scale {scale} does not take menu.font as geometry::menu_item sets it"
        );
    }
}

/// A label the showcase draws in a widget's font (`demo::text_in`: the side
/// panel's labels, the tab rows' labels without `widgets`, the segments, the
/// native expander's titles and the Basic table's cells) takes the font as
/// the connector's `with_text` sets it (M9-013, M9-040): family, size,
/// weight and style as `geometry::menu_item` sets `menu.font`, and its line
/// box, the scaled size by `defaults.line_height`, read from a fixture whose
/// font is italic in a family other than the window's.
#[test]
fn a_label_in_a_widgets_font_takes_it_as_with_text_does() {
    use native_theme_gpui::Native;
    let resolved = native_theme::theme::Theme::preset("kde-breeze")
        .ok()
        .and_then(|t| t.into_variant(native_theme::theme::ColorMode::Light).ok())
        .and_then(|v| {
            v.into_resolved(&native_theme::ResolutionContext::for_tests())
                .ok()
        });
    assert!(resolved.is_some(), "kde-breeze light does not resolve");
    let Some(mut resolved) = resolved else { return };
    let family = resolved.defaults.mono_font.family.clone();
    assert_ne!(
        family, resolved.defaults.font.family,
        "kde-breeze's mono family is now the window's, which this test needs apart"
    );
    resolved.menu.font.family = family;
    resolved.menu.font.style = native_theme::theme::FontStyle::Italic;
    let line_height = resolved.defaults.line_height;
    for scale in [1.0, 2.0] {
        let mut prefs = native_theme_gpui::AccessibilityPreferences::default();
        prefs.text_scaling_factor = scale;
        let n = Native {
            resolved: &resolved,
            accessibility: &prefs,
        };
        let expected = geometry::menu_item(n).text;
        let mut label = crate::demo::text_in(&n, &resolved.menu.font, "label");
        let text = label.style().text.clone();
        assert_eq!(
            (
                text.font_family.clone(),
                text.font_style,
                text.font_size,
                text.font_weight
            ),
            (
                expected.font_family.clone(),
                expected.font_style,
                expected.font_size,
                expected.font_weight
            ),
            "at text scale {scale} the label does not take the font's family, style, size and weight as with_text does"
        );
        assert_eq!(
            text.font_style,
            Some(gpui::FontStyle::Italic),
            "at text scale {scale} the label is not in the fixture's italic"
        );
        assert_eq!(
            expected.line_height,
            Some(gpui::relative(line_height)),
            "with_text no longer sets defaults.line_height as a fraction of the size"
        );
        let size = native_theme_gpui::scaled_text_size(resolved.menu.font.size, &prefs);
        assert_eq!(
            text.line_height,
            Some(px(size * line_height).into()),
            "at text scale {scale} the label's line box is not the scaled size by defaults.line_height"
        );
    }
}

/// `demo::text_in` maps every `FontStyle` to gpui's as the connector's own
/// `font_style` does (native-theme-gpui `src/lib.rs`, private to the crate),
/// read through `geometry::menu_item`, which applies it.
#[test]
fn text_ins_style_is_the_connectors_on_every_font_style() {
    use native_theme::theme::FontStyle;
    use native_theme_gpui::Native;
    let resolved = native_theme::theme::Theme::preset("kde-breeze")
        .ok()
        .and_then(|t| t.into_variant(native_theme::theme::ColorMode::Light).ok())
        .and_then(|v| {
            v.into_resolved(&native_theme::ResolutionContext::for_tests())
                .ok()
        });
    assert!(resolved.is_some(), "kde-breeze light does not resolve");
    let Some(mut resolved) = resolved else { return };
    let prefs = native_theme_gpui::AccessibilityPreferences::default();
    for style in [FontStyle::Normal, FontStyle::Italic, FontStyle::Oblique] {
        resolved.menu.font.style = style;
        let n = Native {
            resolved: &resolved,
            accessibility: &prefs,
        };
        let connector = geometry::menu_item(n).text.font_style;
        let mut label = crate::demo::text_in(&n, &resolved.menu.font, "label");
        assert_eq!(
            label.style().text.font_style,
            connector,
            "text_in draws {style:?} otherwise than the connector's font_style"
        );
    }
}

/// The font lines of the showcase's own Widget Info name a family the
/// system lacks as `info::drawn_font` does, with the substitute gpui is
/// asked for, where the chrome menus' line named no family and the
/// theme-drawn Checkbox's and TabBar's named the stated family alone.
///
/// Its premise is the host's: the family is absent from the font database
/// and the platform names a substitute for it (`font_family`), which only
/// fontconfig does, with feature `system-fonts`. Where either does not
/// hold, the test says so and checks nothing, as the two forms it tells
/// apart are then one.
#[test]
fn a_substituted_family_is_named_in_every_own_font_line() {
    let resolved = native_theme::theme::Theme::preset("kde-breeze")
        .ok()
        .and_then(|t| t.into_variant(native_theme::theme::ColorMode::Light).ok())
        .and_then(|v| {
            v.into_resolved(&native_theme::ResolutionContext::for_tests())
                .ok()
        });
    assert!(resolved.is_some(), "kde-breeze light does not resolve");
    let Some(mut resolved) = resolved else { return };
    let absent: std::sync::Arc<str> = "Native Theme Absent Sans".into();
    #[cfg(feature = "system-fonts")]
    let installed =
        native_theme::fonts::system_face(&absent, 400, native_theme::theme::FontStyle::Normal)
            .is_some();
    #[cfg(not(feature = "system-fonts"))]
    let installed = false;
    if installed {
        println!("skipped: this host has a family named {absent:?}, so it is drawn as stated");
        return;
    }
    let drawn = native_theme_gpui::font_family(&absent);
    if drawn.as_ref() == absent.as_ref() {
        println!(
            "skipped: this host names no substitute for {absent:?} (fontconfig, feature \
             system-fonts), so a substituted family's form is the stated one"
        );
        return;
    }
    let label = native_theme_showcase_support::status::family_label(&absent, Some(drawn.as_ref()));
    resolved.menu.font.family = absent.clone();
    resolved.checkbox.font.family = absent.clone();
    resolved.tab.font.family = absent;
    let prefs = native_theme_gpui::AccessibilityPreferences::default();
    let theme = native_theme_gpui::to_theme(&resolved, "kde-breeze", false, &prefs);
    let line = |info: WidgetInfo, what: &str| {
        info.config
            .iter()
            .find(|n| n.what == what)
            .map(|n| n.text.clone())
            .unwrap_or_default()
    };
    #[cfg_attr(not(feature = "widgets"), allow(unused_mut))]
    let mut lines = vec![(
        "the chrome menus",
        line(
            crate::info::app_menus(&theme, crate::info::MenuHost::Row, Some(&resolved)),
            "font",
        ),
    )];
    #[cfg(feature = "widgets")]
    lines.extend([
        (
            "the theme-drawn Checkbox",
            line(
                crate::info::inputs::native_checkbox(&resolved, "label", false, false),
                "label font",
            ),
        ),
        (
            "the theme-drawn TabBar",
            line(
                crate::info::layout::native_tab_row(&resolved, "widgets::TabBar"),
                "font",
            ),
        ),
    ]);
    let missed: Vec<String> = lines
        .into_iter()
        .filter(|(_, text)| !text.contains(&label))
        .map(|(whose, text)| format!("{whose}: {text:?}"))
        .collect();
    assert!(
        missed.is_empty(),
        "these font lines do not name the substituted family as {label:?}:\n{}",
        missed.join("\n")
    );
}

/// Widget Info's listed font rows name the family gpui draws, as the
/// inspector's Theme tab does: `defaults.font.family` and the family part
/// of `defaults.font` and of `defaults.mono_font` are `status::family_label`
/// of the stated family and `font_family`'s, under windows-11 set directly.
/// Conditioned on the host: where the system lacks "Segoe UI" or "Cascadia
/// Mono" and the platform names a substitute (fontconfig, feature
/// `system-fonts`), the rows name it; elsewhere the label is the stated name
/// alone, which the rows then show.
#[gpui::test]
fn a_listed_font_row_names_the_family_gpui_draws(cx: &mut TestAppContext) {
    use crate::inspector::PanelLook;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_light_preset(&mut cx, &showcase, "windows-11");
    let r = resolved_theme(&mut cx, &showcase);
    let label = |family: &std::sync::Arc<str>| {
        let drawn = native_theme_gpui::font_family(family);
        let label = native_theme_showcase_support::status::family_label(family, Some(&drawn));
        println!("windows-11's {family:?} is drawn in {drawn:?}: its rows read {label:?}");
        label
    };
    let (body, mono) = (
        label(&r.defaults.font.family),
        label(&r.defaults.mono_font.family),
    );
    let mut value = |id: &str, leaf: &str| {
        let row = read(&mut cx, &showcase, |this, cx| {
            let look = PanelLook::of(this, cx)?;
            let element = crate::elements::element(id)?;
            let info = crate::inspector::listed(&look, element, &WidgetInfo::default());
            info.rows
                .into_iter()
                .find(|row| row.leaf == leaf)
                .map(|row| row.value)
        });
        row.unwrap_or_else(|| panic!("no Widget Info row {leaf} for {id}"))
    };
    assert_eq!(value("basic.buttons.heading", "defaults.font.family"), body);
    for (id, leaf, label) in [
        ("basic.switches.off", "defaults.font", &body),
        ("basic.typography.monospace", "defaults.mono_font", &mono),
    ] {
        let font = value(id, leaf);
        assert!(
            font.starts_with(&format!("{label} ")),
            "the {leaf} row does not name the drawn family {label:?}: {font}"
        );
    }
}

/// A platform text system that holds the families `held` alone and lays text out as gpui's
/// `NoopTextSystem`: a host that lacks a theme's family, for a test. The test platform's own,
/// `NoopTextSystem`, resolves every family to one face and lists none (gpui-pre 0.3.10
/// `src/platform.rs:1578-1584`). As gpui's platform text systems do, it loads a family asked
/// for by an alias under the family the alias names (`gpui::font_name_with_fallbacks`, with
/// Linux's system UI font, gpui-pre-linux 0.3.10 `src/linux/platform.rs:126`, gpui-pre-wgpu
/// `src/cosmic_text_system.rs:386`), and gives each family asked for a face id of its own
/// (`:44-47`, `:400-443`), `asked` holding them in their order.
struct HeldFamilies {
    held: Vec<String>,
    asked: std::sync::Mutex<Vec<String>>,
}

impl HeldFamilies {
    /// A text system that holds `held` alone.
    fn new(held: Vec<String>) -> Self {
        Self {
            held,
            asked: std::sync::Mutex::default(),
        }
    }
}

impl gpui::PlatformTextSystem for HeldFamilies {
    fn add_fonts(&self, fonts: Vec<std::borrow::Cow<'static, [u8]>>) -> gpui::Result<()> {
        gpui::NoopTextSystem.add_fonts(fonts)
    }

    fn all_font_names(&self) -> Vec<String> {
        self.held.clone()
    }

    fn font_id(&self, descriptor: &gpui::Font) -> gpui::Result<gpui::FontId> {
        let loaded = gpui::font_name_with_fallbacks(&descriptor.family, "IBM Plex Sans");
        if !self.held.iter().any(|family| family == loaded) {
            return Err(std::io::Error::other(format!("no face of {}", descriptor.family)).into());
        }
        let mut asked = self
            .asked
            .lock()
            .expect("no test panicked holding the lock");
        let id = match asked
            .iter()
            .position(|family| *family == descriptor.family.as_ref())
        {
            Some(id) => id,
            None => {
                asked.push(descriptor.family.to_string());
                asked.len() - 1
            }
        };
        Ok(gpui::FontId(id))
    }

    fn font_metrics(&self, font_id: gpui::FontId) -> gpui::FontMetrics {
        gpui::NoopTextSystem.font_metrics(font_id)
    }

    fn typographic_bounds(
        &self,
        font_id: gpui::FontId,
        glyph_id: gpui::GlyphId,
    ) -> gpui::Result<Bounds<f32>> {
        gpui::NoopTextSystem.typographic_bounds(font_id, glyph_id)
    }

    fn advance(
        &self,
        font_id: gpui::FontId,
        glyph_id: gpui::GlyphId,
    ) -> gpui::Result<gpui::Size<f32>> {
        gpui::NoopTextSystem.advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: gpui::FontId, ch: char) -> Option<gpui::GlyphId> {
        gpui::NoopTextSystem.glyph_for_char(font_id, ch)
    }

    fn glyph_raster_bounds(
        &self,
        params: &gpui::RenderGlyphParams,
    ) -> gpui::Result<Bounds<gpui::DevicePixels>> {
        gpui::NoopTextSystem.glyph_raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        params: &gpui::RenderGlyphParams,
        raster_bounds: Bounds<gpui::DevicePixels>,
    ) -> gpui::Result<(gpui::Size<gpui::DevicePixels>, Vec<u8>)> {
        gpui::NoopTextSystem.rasterize_glyph(params, raster_bounds)
    }

    fn layout_line(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[gpui::FontRun],
    ) -> gpui::LineLayout {
        gpui::NoopTextSystem.layout_line(text, font_size, runs)
    }

    fn recommended_rendering_mode(
        &self,
        font_id: gpui::FontId,
        font_size: Pixels,
    ) -> gpui::TextRenderingMode {
        gpui::NoopTextSystem.recommended_rendering_mode(font_id, font_size)
    }
}

/// Run `test` as `#[gpui::test]` runs a test's body (gpui-pre-macros 0.3.10 `src/test.rs:159-196`),
/// in an app whose platform text system holds the families `held` alone ([`HeldFamilies`]).
fn with_held_families(
    held: &[&str],
    test: impl Fn(&mut TestAppContext) + std::panic::RefUnwindSafe,
) {
    let held: Vec<String> = held.iter().map(|family| family.to_string()).collect();
    gpui::run_test(
        1,
        &[],
        0,
        &mut |dispatcher, _seed| {
            let mut cx = TestAppContext::build_with_text_system(
                dispatcher,
                None,
                std::sync::Arc::new(HeldFamilies::new(held.clone())),
            );
            test(&mut cx);
            cx.run_until_parked();
            cx.update(|cx| {
                cx.background_executor().forbid_parking();
                cx.quit();
            });
            cx.run_until_parked();
        },
        None,
    );
}

/// The installed theme's font labels, as the three sites show them: the status bar's font item,
/// the Theme tab's `font_family` and `mono_font_family` rows, and Widget Info's listed
/// `defaults.font.family` row of the Basic page's heading button and the family part of its
/// `defaults.mono_font` row of the monospace text, each with the frame drawn that keeps the
/// install's look.
fn font_labels(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> [(String, String); 5] {
    use crate::inspector::PanelLook;
    draw(cx);
    read(cx, showcase, |this, cx| {
        let status = crate::chrome::status_environment(this, cx);
        let rows = crate::inspector::font_rows(this, cx);
        let row = |what: &str| {
            rows.iter()
                .find(|(name, _)| *name == what)
                .map(|(_, value)| value.clone())
                .unwrap_or_default()
        };
        let size = native_theme_showcase_support::status::defined_size(&this.original_font);
        let listed = |id: &str, leaf: &str| {
            PanelLook::of(this, cx)
                .and_then(|look| {
                    let element = crate::elements::element(id)?;
                    crate::inspector::listed(&look, element, &WidgetInfo::default())
                        .rows
                        .into_iter()
                        .find(|row| row.leaf == leaf)
                        .map(|row| row.value)
                })
                .unwrap_or_default()
        };
        // "<family> <size> <unit> <weight>": the monospace font's style is normal.
        let mono = listed("basic.typography.monospace", "defaults.mono_font");
        let mono_family = mono.rsplitn(4, ' ').nth(3).unwrap_or_default().to_string();
        let (body, mono_stated) = (
            this.original_font.family.to_string(),
            this.original_mono_font.family.to_string(),
        );
        [
            (
                "the status bar's font item".to_string(),
                status
                    .get(2)
                    .and_then(|item| item.strip_suffix(&format!(" {size}")))
                    .unwrap_or_default()
                    .to_string(),
            ),
            (
                "the Theme tab's font_family row".to_string(),
                row("font_family"),
            ),
            (
                "the Theme tab's mono_font_family row".to_string(),
                row("mono_font_family"),
            ),
            (
                format!("Widget Info's defaults.font.family row ({body})"),
                listed("basic.buttons.heading", "defaults.font.family"),
            ),
            (
                format!("Widget Info's defaults.mono_font row ({mono_stated})"),
                mono_family,
            ),
        ]
    })
}

/// The family gpui is asked for in a theme's `stated` one (the connector's `font_family`).
fn asked_family(stated: &str) -> String {
    native_theme_gpui::font_family(&std::sync::Arc::from(stated)).to_string()
}

/// The stack families a [`HeldFamilies`] test holds the first of that is none of `asked`, so
/// gpui draws text it finds no face for in that one.
fn held_fallback(asked: &[String]) -> &'static str {
    ["DejaVu Sans", "Cantarell", "Arial"]
        .into_iter()
        .find(|family| !asked.iter().any(|asked| asked == family))
        .unwrap_or("Arial")
}

/// Where gpui's text system finds no face of a theme's family, the status bar, the Theme tab and
/// Widget Info name the family gpui draws in its place, the first of gpui's fallback stack its
/// text system holds, and the family it found no face of: under kde-breeze on a host whose text
/// system holds neither "Noto Sans" nor "Hack" (the platform names no substitute where the
/// system has them, as this host's fontconfig, feature `system-fonts`, or as macOS and Windows
/// never do), and under a theme that states "NoSuchFamily-T10b", which no system has (on this
/// host, the platform's substitute, which the text system does not hold either).
#[test]
fn a_family_gpui_finds_no_face_of_is_named_by_its_fallback() {
    let light = |preset: &str| {
        native_theme::theme::Theme::preset(preset)
            .and_then(|t| t.resolve(native_theme::theme::ColorMode::Light))
            .map(|r| r.variant.defaults)
            .expect("the preset resolves")
    };
    let breeze = light("kde-breeze");
    let (absent, absent_mono) = ("NoSuchFamily-T10b", "NoSuchFamily-T10b Mono");
    let asked: Vec<String> = [
        &*breeze.font.family,
        &*breeze.mono_font.family,
        absent,
        absent_mono,
    ]
    .into_iter()
    .map(asked_family)
    .collect();
    let held = held_fallback(&asked);
    let expected = |stated: &str| {
        let asked = asked_family(stated);
        let missing = if asked == stated {
            "gpui finds no face of it".to_string()
        } else {
            format!("not on this system; gpui finds no face of the platform's substitute, {asked}")
        };
        format!(
            "{stated} ({missing}; drawn in the first family of gpui's fallback stack it finds, \
             {held})"
        )
    };
    with_held_families(&[held], |cx| {
        let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
        use_light_preset(&mut cx, &showcase, "kde-breeze");
        let check = |cx: &mut VisualTestContext, under: &str, body: &str, mono: &str| {
            let labels = font_labels(cx, &showcase);
            for (ix, (site, label)) in labels.iter().enumerate() {
                let stated = if ix == 2 || ix == 4 { mono } else { body };
                println!("{under}: {site}: {label}");
                assert_eq!(label, &expected(stated), "{under}: {site}");
            }
        };
        check(
            &mut cx,
            "kde-breeze",
            &breeze.font.family,
            &breeze.mono_font.family,
        );
        let Ok(system) = system_theme() else {
            return;
        };
        let overlay = native_theme::theme::Theme::from_toml(&format!(
            "[light.defaults.font]\nfamily = \"{absent}\"\n[light.defaults.mono_font]\nfamily = \
             \"{absent_mono}\"\n[dark.defaults.font]\nfamily = \"{absent}\"\n\
             [dark.defaults.mono_font]\nfamily = \"{absent_mono}\"\n"
        ))
        .and_then(|overlay| system.with_overlay(&overlay))
        .expect("the overlay applies");
        cx.update(|window, cx| {
            showcase.update(cx, |this, cx| this.install_system(overlay, window, cx))
        });
        cx.run_until_parked();
        check(&mut cx, absent, absent, absent_mono);
    });
}

/// Where gpui's text system finds a theme's family, the three sites name it as
/// `status::family_label` of the stated family and the one gpui is asked for: the stated name
/// alone where the platform names no substitute, as on a host that has "Noto Sans" and "Hack".
#[test]
fn a_family_gpui_finds_is_named_as_stated() {
    let breeze = native_theme::theme::Theme::preset("kde-breeze")
        .and_then(|t| t.resolve(native_theme::theme::ColorMode::Light))
        .map(|r| r.variant.defaults)
        .expect("the preset resolves");
    let (body, mono) = (
        asked_family(&breeze.font.family),
        asked_family(&breeze.mono_font.family),
    );
    let fallback = held_fallback(&[body.clone(), mono.clone()]);
    let held = [body.as_str(), mono.as_str(), fallback];
    with_held_families(&held, |cx| {
        let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
        use_light_preset(&mut cx, &showcase, "kde-breeze");
        for (ix, (site, label)) in font_labels(&mut cx, &showcase).iter().enumerate() {
            let (stated, asked) = if ix == 2 || ix == 4 {
                (&breeze.mono_font.family, &mono)
            } else {
                (&breeze.font.family, &body)
            };
            println!("kde-breeze: {site}: {label}");
            assert_eq!(
                label,
                &native_theme_showcase_support::status::family_label(stated, Some(asked)),
                "{site}"
            );
            if stated.as_ref() == asked.as_str() {
                assert_eq!(label, stated.as_ref(), "{site}");
            }
        }
    });
}

/// The label the lookup gives `stated`, a family the text system of `cx` does not hold, asked
/// for as itself: built as an install builds it (`DrawnFamilies::of`), before any window lays
/// text out.
fn label_of_absent(cx: &mut TestAppContext, stated: &str) -> String {
    cx.update(|cx| {
        crate::inspector::DrawnFamilies::of([gpui::SharedString::from(stated.to_string())], cx)
            .label(stated, stated)
    })
}

/// A text system that holds no family of gpui's fallback stack is not asked: `resolve_font`
/// would panic there for a family it lacks (gpui-pre 0.3.10 `src/text_system.rs:622-628`), so
/// the label is the family asked for.
#[test]
fn a_text_system_without_a_stack_family_is_not_asked() {
    with_held_families(&["Hack"], |cx| {
        assert_eq!(
            label_of_absent(cx, "NoSuchFamily-T10b"),
            "NoSuchFamily-T10b"
        );
    });
}

/// Each family of gpui's fallback stack, held alone, is the one a family the text system lacks
/// is drawn in, named by the family its platform loads for it (`.ZedMono` is Lilex, `.ZedSans`
/// IBM Plex Sans, `gpui::font_name_with_fallbacks`); of two held in a row, the first.
#[test]
fn each_family_of_the_stack_is_named_where_it_is_drawn() {
    use crate::inspector::GPUI_FALLBACK_STACK;
    let loaded = |entry: &'static str| gpui::font_name_with_fallbacks(entry, entry);
    let expected = |family: &str| {
        format!(
            "NoSuchFamily-T10b (gpui finds no face of it; drawn in the first family of gpui's \
             fallback stack it finds, {family})"
        )
    };
    for entry in GPUI_FALLBACK_STACK {
        with_held_families(&[loaded(entry)], |cx| {
            assert_eq!(
                label_of_absent(cx, "NoSuchFamily-T10b"),
                expected(loaded(entry)),
                "{entry} held alone"
            );
        });
    }
    for pair in GPUI_FALLBACK_STACK.windows(2) {
        let (first, second) = (loaded(pair[0]), loaded(pair[1]));
        with_held_families(&[second, first], |cx| {
            assert_eq!(
                label_of_absent(cx, "NoSuchFamily-T10b"),
                expected(first),
                "{first} and {second} held"
            );
        });
    }
}

/// The showcase's copy of gpui's fallback stack (`inspector::GPUI_FALLBACK_STACK`) is each
/// core's, at the lines its doc cites: gpui-pre 0.3.10 `src/text_system.rs:262-273`, gpui-fast
/// 0.1.12 `:263-274`, read from the sources `cargo metadata` names. The field is private in
/// both, so the source is the only place to read it whole.
#[test]
fn the_fallback_stack_copy_is_gpuis() {
    use crate::inspector::GPUI_FALLBACK_STACK;
    let out = std::process::Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo metadata runs");
    assert!(
        out.status.success(),
        "cargo metadata failed: {:?}",
        out.status
    );
    let meta: serde_json::Value = serde_json::from_slice(&out.stdout).expect("metadata is JSON");
    let packages = meta["packages"]
        .as_array()
        .expect("cargo metadata lists packages");
    for (core, version, first, last) in [
        ("gpui-pre", "0.3.10", 262, 273),
        ("gpui-fast", "0.1.12", 263, 274),
    ] {
        let manifest = packages
            .iter()
            .find(|p| p["name"] == core && p["version"] == version)
            .and_then(|p| p["manifest_path"].as_str())
            .unwrap_or_else(|| {
                panic!(
                    "{core} {version} is not in the cargo metadata: re-cite the fallback-stack \
                     lines, in inspector::GPUI_FALLBACK_STACK's doc and this test's table and \
                     doc, at the gpui-pre and gpui-fast versions Cargo.lock now holds"
                )
            });
        let source = std::path::Path::new(manifest)
            .with_file_name("src")
            .join("text_system.rs");
        let text = std::fs::read_to_string(&source).expect("the core's text_system.rs reads");
        let lines: Vec<&str> = text.lines().collect();
        let cited: Vec<String> = lines[first - 1..last]
            .iter()
            .filter_map(|line| {
                let start = line.find("font(\"")? + "font(\"".len();
                let end = line[start..].find('"')? + start;
                Some(line[start..end].to_string())
            })
            .collect();
        assert_eq!(
            cited, GPUI_FALLBACK_STACK,
            "{core} {version} src/text_system.rs:{first}-{last}"
        );
        assert!(
            lines[first - 1].contains("fallback_font_stack: smallvec!["),
            "{core} {version}: :{first} does not start the stack: {}",
            lines[first - 1]
        );
        assert!(
            lines[last].trim() == "],",
            "{core} {version}: the line after :{last} does not end the stack: {}",
            lines[last]
        );
    }
}

/// A chrome popup row is padded on each side the theme states by
/// `menu.border.padding` (`geometry::menu_item`), on an unstated left or
/// right by `MENU_ROW_PADDING_X`, upstream's row padding, and on an
/// unstated top or bottom by nothing, under every bundled preset in both
/// modes, lit or not.
#[test]
fn a_menu_popup_row_pads_each_side_as_the_theme_states_it() {
    use gpui::DefiniteLength;
    let prefs = native_theme_gpui::AccessibilityPreferences::default();
    for (name, r, _) in every_bundled_variant() {
        let n = native_theme_gpui::Native {
            resolved: &r,
            accessibility: &prefs,
        };
        let p = r.menu.border.padding;
        let side = |stated: Option<f32>, unstated: Option<Pixels>| {
            stated.map(px).or(unstated).map(DefiniteLength::from)
        };
        let x = Some(crate::demo::MENU_ROW_PADDING_X);
        let expected = (
            side(p.top, None),
            side(p.right, x),
            side(p.bottom, None),
            side(p.left, x),
        );
        let look = crate::demo::MenuLook::of(&n);
        for lit in [false, true] {
            let mut row = look.row(lit);
            let padding = &row.style().padding;
            assert_eq!(
                (padding.top, padding.right, padding.bottom, padding.left),
                expected,
                "{name}: a popup row (lit {lit}) is not padded as menu.border.padding {p:?} states"
            );
        }
    }
}

/// The chrome menus' padding line says what pads a popup row's unstated
/// sides as `MenuLook::row` pads them: `MENU_ROW_PADDING_X` at the left and
/// right, and nothing above or below, the row's height being the item-height
/// line's, under every bundled preset in both modes.
#[test]
fn the_menus_padding_line_names_what_pads_a_rows_unstated_sides() {
    let sides = format!(
        "an item's {}px at the sides (MENU_ROW_PADDING_X) and nothing above or below",
        crate::info::px_text(crate::demo::MENU_ROW_PADDING_X.as_f32())
    );
    for (name, r, t) in every_bundled_variant() {
        let info = crate::info::app_menus(&t, crate::info::MenuHost::Row, Some(&r));
        let line = info
            .config
            .iter()
            .find(|n| n.what == "padding")
            .map(|n| n.text.clone());
        assert!(
            line.as_deref().is_some_and(|l| l.contains(&sides)),
            "{name}: the menus' padding line does not say {sides:?}: {line:?}"
        );
    }
}

/// Where the theme states neither `menu.row_height` nor a vertical padding
/// side (catppuccin-mocha), a chrome popup row is `MENU_ROW_HEIGHT` by the
/// connector's control-height rule (`geometry.rs`, `with_height_rule`): that
/// height at text scale 1, a minimum with an automatic height above it, so a
/// scaled font grows the row.
#[test]
fn a_menu_popup_rows_fallback_height_follows_the_control_height_rule() {
    use native_theme_gpui::Native;
    let resolved = native_theme::theme::Theme::preset("catppuccin-mocha")
        .ok()
        .and_then(|t| t.into_variant(native_theme::theme::ColorMode::Dark).ok())
        .and_then(|v| {
            v.into_resolved(&native_theme::ResolutionContext::for_tests())
                .ok()
        });
    assert!(resolved.is_some(), "catppuccin-mocha dark does not resolve");
    let Some(resolved) = resolved else { return };
    let p = resolved.menu.border.padding;
    assert!(
        resolved.menu.row_height.is_none() && p.top.is_none() && p.bottom.is_none(),
        "catppuccin-mocha now states a menu row height or a vertical padding, which this test needs unstated"
    );
    let height = gpui::Length::from(crate::demo::MENU_ROW_HEIGHT);
    for (scale, expected) in [
        (1.0, (Some(height), None)),
        (2.0, (Some(gpui::Length::Auto), Some(height))),
    ] {
        let mut prefs = native_theme_gpui::AccessibilityPreferences::default();
        prefs.text_scaling_factor = scale;
        let n = Native {
            resolved: &resolved,
            accessibility: &prefs,
        };
        let mut row = crate::demo::MenuLook::of(&n).row(false);
        let style = row.style();
        assert_eq!(
            (style.size.height, style.min_size.height),
            expected,
            "a popup row at text scale {scale} is not MENU_ROW_HEIGHT by the control-height rule"
        );
    }
}
/// The number field a native theme draws -- the showcase's own frame round
/// gpui-base's NumberInput -- reports what it paints (M9-001): under
/// kde-breeze light its fill is `input.background_color`, the view colour
/// and not the window's; its edge `input.border.color`, under the pointer
/// `input.hover_border_color` and focused `input.focus_border_color`; its
/// value padded by `geometry::input`; its steps ChevronUp and ChevronDown.
/// The fill painted inside it is the fill claimed, and the edge painted
/// round it once it is focused the focused edge claimed.
#[gpui::test]
fn the_native_number_field_reports_what_it_draws(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_light_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let r = resolved_theme(&mut cx, &showcase);
    let i = &r.input;
    let fill = stated(i.background_color);
    assert_ne!(
        fill,
        stated(r.defaults.background_color),
        "kde-breeze light's field fill is the window's colour, so the fill claim could not be told from upstream's"
    );
    assert!(
        i.border.line_width > 0.0
            && i.hover_border_color.is_some()
            && i.focus_border_color.is_some(),
        "kde-breeze no longer frames its field and states its hover and focus edges, which this test reads"
    );
    let field = "basic-number";
    let info = settle_on(&mut cx, &showcase, field).expect("the number field shows an info");
    assert_eq!(
        claim_of(&info, "background_color"),
        Some(fill),
        "the number field's info does not claim the input.background_color it is filled with: {info:?}"
    );
    for (leaf, want) in [
        ("border", Some(stated(i.border.color))),
        ("hover_border_color", i.hover_border_color.map(stated)),
        ("focus_border_color", i.focus_border_color.map(stated)),
    ] {
        assert_eq!(
            claim_of(&info, leaf),
            want,
            "the number field's info does not claim input.{leaf}, which its frame is edged with: {info:?}"
        );
    }
    let padding = info
        .config
        .iter()
        .chain(&info.instance)
        .find(|n| n.what == "padding");
    assert!(
        padding
            .is_some_and(|n| n.text.contains("geometry::input") && !n.text.contains("none from")),
        "the number field's info does not say its value takes geometry::input's padding: {padding:?}"
    );
    for (step, chevron) in [
        ("increment step", "ChevronUp"),
        ("decrement step", "ChevronDown"),
    ] {
        let note = note_of(&info.instance, step);
        assert!(
            note.is_some_and(|text| text.starts_with(chevron)),
            "the number field's info does not name its {step}'s {chevron}: {note:?}"
        );
    }
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert_eq!(
        painted_fill(&mut cx, field),
        Some(fill),
        "the fill painted inside the number field is not the fill its info claims"
    );
    // Focused: the info built in that frame claims the focused edge, which
    // is the edge painted.
    click(&mut cx, field);
    let focused = settle_on(&mut cx, &showcase, field).expect("the number field shows an info");
    let edge = claim_of(&focused, "focus_border_color");
    assert!(
        edge.is_some() && claim_of(&focused, "hover_border_color").is_none(),
        "the focused number field's info does not claim its focused edge alone: {focused:?}"
    );
    assert_eq!(
        painted_edge(&mut cx, field),
        edge,
        "the edge painted round the focused number field is not the edge its info claims"
    );
}

/// The Basic page's table under a native theme -- the showcase's own rows,
/// header and frame -- reports what it paints (M9-003), under kde-breeze
/// light: the selected row `list.selection_background` and
/// `selection_text_color`, the others `list.item_font`'s colour and the
/// `list.hover_*` pair, no row a top edge, the two lines `list.grid_color`,
/// the header `list.header_background` and `header_font`, the frame
/// `list.border` round `list.background_color`. The fill painted on the
/// selected row is its claim.
#[gpui::test]
fn the_basic_tables_infos_report_what_it_draws(cx: &mut TestAppContext) {
    use crate::info::stated;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_light_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let r = resolved_theme(&mut cx, &showcase);
    let l = &r.list;
    assert!(
        l.border.line_width > 0.0 && r.separator.line_width > 0.0,
        "kde-breeze no longer frames its list or rules its grid, which this test reads"
    );
    let grid = Some(stated(l.grid_color));
    let no_upstream_row = |info: &WidgetInfo, what: &str| {
        assert!(
            info.colors.iter().all(|c| !c.role.contains("top")
                && !["table", "table_row_border", "foreground"].contains(&c.field)),
            "the {what} claims the declarative Table's row: {info:?}"
        );
    };
    let selected =
        settle_on(&mut cx, &showcase, "basic-table-row-1").expect("the selected row shows an info");
    for (leaf, want) in [
        ("selection_background", Some(stated(l.selection_background))),
        ("selection_text_color", Some(stated(l.selection_text_color))),
        ("grid_color", grid),
    ] {
        assert_eq!(
            claim_of(&selected, leaf),
            want,
            "the selected row's info does not claim list.{leaf}: {selected:?}"
        );
    }
    no_upstream_row(&selected, "selected row");
    let other =
        settle_on(&mut cx, &showcase, "basic-table-row-0").expect("the first row shows an info");
    for (leaf, want) in [
        ("item_font", Some(stated(l.item_font.color))),
        ("hover_background", Some(stated(l.hover_background))),
        ("hover_text_color", Some(stated(l.hover_text_color))),
        ("grid_color", grid),
    ] {
        assert_eq!(
            claim_of(&other, leaf),
            want,
            "the unselected row's info does not claim list.{leaf}: {other:?}"
        );
    }
    no_upstream_row(&other, "unselected row");
    let header =
        settle_on(&mut cx, &showcase, "basic-table-header").expect("the header shows an info");
    for (leaf, want) in [
        ("header_background", Some(stated(l.header_background))),
        ("header_font", Some(stated(l.header_font.color))),
        ("grid_color", grid),
    ] {
        assert_eq!(
            claim_of(&header, leaf),
            want,
            "the header's info does not claim list.{leaf}: {header:?}"
        );
    }
    // The frame's own target: inside its edge, which no row reaches.
    let frame = bounds_of(&mut cx, "basic-table");
    let inside_edge = point(
        frame.left() + px(l.border.line_width / 2.),
        frame.center().y,
    );
    let table = settle_at(&mut cx, &showcase, inside_edge).expect("the frame shows an info");
    for (leaf, want) in [
        ("background_color", Some(stated(l.background_color))),
        ("border", Some(stated(l.border.color))),
    ] {
        assert_eq!(
            claim_of(&table, leaf),
            want,
            "the table frame's info does not claim list.{leaf}: {table:?}"
        );
    }
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert_eq!(
        painted_fill(&mut cx, "basic-table-row-1"),
        claim_of(&selected, "selection_background"),
        "the fill painted on the selected row is not the fill its info claims"
    );
}

/// Where the theme strokes no line, the native number field's and the
/// Basic table's infos claim none (M9-001, M9-003). Under adwaita light the
/// field's `input.border.line_width` is 0: its info claims no edge, says so,
/// and no edge is painted round it. adwaita states the list's and the
/// separator's widths (1px, inherited), so the table's infos run their
/// 0-width branches on its resolved theme with those two widths set to 0:
/// no frame edge, no grid line.
#[gpui::test]
fn an_unstroked_edge_is_claimed_by_no_native_info(cx: &mut TestAppContext) {
    use crate::info::data::{native_table_frame, native_table_header, native_table_row};
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_light_preset(&mut cx, &showcase, "adwaita");
    show(&mut cx, &showcase, Page::Basic);
    let mut r = resolved_theme(&mut cx, &showcase);
    assert!(
        r.input.border.line_width <= 0.0,
        "adwaita now strokes its field, so this test no longer runs the number field's no-edge branch"
    );
    let edges = ["border", "hover_border_color", "focus_border_color"];
    let config = |info: &WidgetInfo, what: &str| {
        info.config
            .iter()
            .find(|n| n.what == what)
            .map(|n| n.text.clone())
    };
    let field = "basic-number";
    let info = settle_on(&mut cx, &showcase, field).expect("the number field shows an info");
    assert!(
        info.colors
            .iter()
            .all(|c| !edges.contains(&c.field) || c.role.starts_with("step")),
        "the number field claims an edge it does not stroke: {info:?}"
    );
    assert_eq!(
        config(&info, "edge").as_deref(),
        Some("none: input.border.line_width is 0"),
        "the number field's info does not say it strokes no edge: {info:?}"
    );
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert_eq!(
        painted_edge(&mut cx, field),
        None,
        "an edge is painted round the number field, whose info claims none"
    );
    r.list.border.line_width = 0.0;
    r.separator.line_width = 0.0;
    let frame = native_table_frame(&r, 2);
    assert!(
        claim_of(&frame, "border").is_none()
            && config(&frame, "edge").as_deref() == Some("none: list.border.line_width is 0"),
        "the table frame claims an edge at a 0 width: {frame:?}"
    );
    for info in [
        native_table_header(&r, "Name, Size"),
        native_table_row(&r, true, "b.png, 20 KB"),
        native_table_row(&r, false, "a.txt, 1 KB"),
    ] {
        assert!(
            claim_of(&info, "grid_color").is_none()
                && config(&info, "grid").as_deref() == Some("none: separator.line_width is 0"),
            "a table row claims a grid line at a 0 width: {info:?}"
        );
    }
}

/// A hovered Tag fades to 90% (tag.rs:269), and its info names the fill
/// that paints -- the painted value, not the token it fades.
#[gpui::test]
fn a_hovered_tags_info_names_its_painted_fill(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Feedback);
    // `settle_on` leaves the pointer on the Tag, so this frame is hovered.
    let info = settle_on(&mut cx, &showcase, FEEDBACK_TAG_PRIMARY);
    let painted = painted_fill(&mut cx, FEEDBACK_TAG_PRIMARY);
    let hover = info.as_ref().and_then(|info| {
        info.not_themeable
            .iter()
            .find(|n| n.what == "hover")
            .map(|n| n.text.clone())
    });
    assert!(
        painted.is_some(),
        "nothing was painted inside the Primary Tag"
    );
    assert!(
        painted.is_some_and(|fill| hover
            .as_deref()
            .is_some_and(|text| text.contains(&as_rgba(fill).to_string()))),
        "the hovered Primary Tag paints {:?}, which its hover note does not name: {hover:?}",
        painted.map(as_rgba)
    );
}

/// The ids and debug selectors of three of the Theme Map's swatches. The
/// page forms a swatch's from the name of the token it shows
/// (pages/theme_map.rs).
const THEME_MAP_BACKGROUND: &str = "theme-map-swatch-background";
const THEME_MAP_PRIMARY_HOVER: &str = "theme-map-swatch-primary_hover";
const THEME_MAP_DROP_TARGET: &str = "theme-map-swatch-drop_target";

/// The Theme Map reports each row of its table (spec §4.3.2): two swatches
/// each show their own token, and the connector line that writes it.
#[gpui::test]
fn two_swatches_show_different_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::ThemeMap);
    let texts = settle_on_each(
        &mut cx,
        &showcase,
        &[
            (THEME_MAP_BACKGROUND, "ThemeColor · background"),
            (THEME_MAP_PRIMARY_HOVER, "ThemeColor · primary_hover"),
        ],
    );
    for (i, text) in texts.iter().enumerate() {
        assert!(
            !texts.iter().skip(i + 1).any(|other| other == text),
            "two swatches show the same info: {texts:?}"
        );
    }
    for text in &texts {
        assert!(
            text.as_deref()
                .is_some_and(|t| t.contains("(native-theme-gpui/colors.rs:")),
            "a swatch does not cite the connector line that writes it: {text:?}"
        );
    }
}

/// A swatch paints the value its info claims, translucent or not:
/// drop_target is primary at 20%, and the swatch shows that, not primary.
#[gpui::test]
fn a_swatchs_painted_fill_is_its_value_claim(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::ThemeMap);
    let info = settle_on(&mut cx, &showcase, THEME_MAP_DROP_TARGET);
    let claimed = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role == "value"))
        .map(|c| c.value);
    let installed = cx.update(|_window, cx| Theme::global(cx).drop_target);
    assert_eq!(
        claimed,
        Some(installed),
        "the drop_target swatch does not claim the installed drop_target: {info:?}"
    );
    assert_eq!(
        painted_fill(&mut cx, THEME_MAP_DROP_TARGET),
        claimed,
        "the fill painted in the drop_target swatch is not its value claim"
    );
}

/// Install the preset of `key` as the theme settings' preset switch and the
/// command palette do, through `SetPreset`, and draw the frame after. A
/// preset that does not install fails the test (`current_theme_name` is set
/// only on a successful install, `apply_theme_by_name`); `"default"`, the
/// OS theme, is not asserted, since the OS read may fail on a runner.
fn set_preset(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, key: &str) {
    dispatch_preset(cx, key);
    if key != "default" {
        let installed = read(cx, showcase, |this, _| this.current_theme_name.clone());
        assert_eq!(installed, key, "the preset {key:?} did not install");
    }
}

/// Ask for a preset that does not exist through `SetPreset`, to provoke a
/// theme error: the installed theme's name must not change.
fn set_missing_preset(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, key: &str) {
    let before = read(cx, showcase, |this, _| this.current_theme_name.clone());
    dispatch_preset(cx, key);
    let after = read(cx, showcase, |this, _| this.current_theme_name.clone());
    assert_eq!(
        after, before,
        "the missing preset {key:?} changed the installed theme's name"
    );
}

/// Dispatch `SetPreset(key)` and draw the frame after.
fn dispatch_preset(cx: &mut VisualTestContext, key: &str) {
    let action = SetPreset(key.to_string().into());
    cx.update(|window, cx| window.dispatch_action(Box::new(action), cx));
    cx.run_until_parked();
    draw(cx);
}

/// The accessibility preferences the installed native theme carries.
fn installed_preferences(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
) -> Option<native_theme_gpui::AccessibilityPreferences> {
    read(cx, showcase, |_this, cx| {
        cx.native_theme().map(|nt| nt.accessibility().clone())
    })
}

/// A theme install keeps the accessibility preferences the user set in the
/// Preferences sheet: a preset switch, a reload or the
/// desktop's own theme must not put the OS's back in their place. The ones
/// the user did not set are the OS's, read again at the install
/// (`an_untouched_preference_follows_the_os_on_reinstall`); here the OS's
/// differ from the user's in every field, so each field shows whose it is.
#[gpui::test]
fn a_theme_install_keeps_the_installed_preferences(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "kde-breeze");
    let mut os = native_theme_gpui::AccessibilityPreferences::default();
    os.text_scaling_factor = 1.25;
    os.reduce_motion = false;
    os.high_contrast = true;
    os.reduce_transparency = true;
    cx.update(|_window, cx| {
        let ui = showcase.read(cx).info_ui.clone();
        crate::demo::change_preferences(&ui, cx, |overrides| {
            overrides.text_scaling_factor = Some(1.75);
            overrides.reduce_motion = Some(true);
            overrides.high_contrast = Some(false);
        });
        showcase.update(cx, |this, _| {
            this.os_preferences_for_test = Some(os.clone())
        });
    });
    cx.run_until_parked();
    let mut chosen = native_theme_gpui::AccessibilityPreferences::default();
    chosen.text_scaling_factor = 1.75;
    chosen.reduce_motion = true;
    chosen.high_contrast = false;
    chosen.reduce_transparency = os.reduce_transparency;
    assert_eq!(
        installed_preferences(&mut cx, &showcase).map(|p| (
            p.text_scaling_factor,
            p.reduce_motion,
            p.high_contrast
        )),
        Some((1.75, true, false)),
        "the chosen preferences were not installed, so keeping them proves nothing"
    );
    set_preset(&mut cx, &showcase, "adwaita");
    assert_eq!(
        installed_preferences(&mut cx, &showcase).as_ref(),
        Some(&chosen),
        "switching the preset replaced the preferences the user chose"
    );
    cx.update(|window, cx| window.dispatch_action(Box::new(ReloadTheme), cx));
    cx.run_until_parked();
    assert_eq!(
        installed_preferences(&mut cx, &showcase).as_ref(),
        Some(&chosen),
        "reloading the theme, as the theme watcher does, replaced the preferences the user \
         chose"
    );
    // The desktop's own theme, where this host has one to read (always on
    // Linux), with the OS's preferences it carries made the test's.
    if let Ok(mut system) = system_theme() {
        system.accessibility = os.clone();
        cx.update(|window, cx| {
            showcase.update(cx, |this, cx| this.install_system(system, window, cx))
        });
        cx.run_until_parked();
        assert_eq!(
            installed_preferences(&mut cx, &showcase).as_ref(),
            Some(&chosen),
            "installing the desktop's own theme replaced the preferences the user chose"
        );
    }
}

/// A preference the user did not set in the Preferences sheet follows the
/// OS: every theme install -- a preset switch, a reload as the theme
/// watcher's -- reads the OS's preferences again, so one changed while the
/// showcase runs lands with the next install, as it does in a native
/// application. The OS's are the test's (`os_preferences_for_test`).
#[gpui::test]
fn an_untouched_preference_follows_the_os_on_reinstall(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "kde-breeze");
    let mut changed = native_theme_gpui::AccessibilityPreferences::default();
    changed.text_scaling_factor = 1.5;
    changed.reduce_motion = true;
    changed.high_contrast = true;
    changed.reduce_transparency = true;
    assert_ne!(
        installed_preferences(&mut cx, &showcase).as_ref(),
        Some(&changed),
        "the preferences the OS changes to were installed before it changed them, so \
         following them proves nothing"
    );
    let os_has = |cx: &mut VisualTestContext,
                  prefs: &native_theme_gpui::AccessibilityPreferences| {
        let prefs = prefs.clone();
        cx.update(|_window, cx| {
            showcase.update(cx, |this, _| this.os_preferences_for_test = Some(prefs))
        });
    };
    os_has(&mut cx, &changed);
    set_preset(&mut cx, &showcase, "adwaita");
    assert_eq!(
        installed_preferences(&mut cx, &showcase).as_ref(),
        Some(&changed),
        "a preset switch kept preferences the OS no longer has"
    );
    let back = native_theme_gpui::AccessibilityPreferences::default();
    os_has(&mut cx, &back);
    cx.update(|window, cx| window.dispatch_action(Box::new(ReloadTheme), cx));
    cx.run_until_parked();
    assert_eq!(
        installed_preferences(&mut cx, &showcase).as_ref(),
        Some(&back),
        "reloading the theme, as the theme watcher does, kept preferences the OS no longer has"
    );
}

/// `--variant` without `--theme` installs the current theme again in the
/// mode it names: gpui-component's mode and the native variant installed
/// follow it, not only the colour-mode Select.
#[gpui::test]
fn a_variant_alone_reinstalls_the_theme_in_its_mode(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "kde-breeze");
    let was_dark = cx.update(|_w, cx| Theme::global(cx).mode.is_dark());
    let (variant, wanted) = match was_dark {
        true => ("light", AppColorMode::Light),
        false => ("dark", AppColorMode::Dark),
    };
    let args = crate::CliArgs {
        variant: Some(variant.to_string()),
        ..Default::default()
    };
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            crate::apply_cli_args(this, &args, window, cx)
        })
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        wanted,
        "--variant {variant} did not set the colour mode"
    );
    assert_eq!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        !was_dark,
        "--variant {variant} alone did not install the theme in that mode"
    );
    assert!(
        read(&mut cx, &showcase, |_this, cx| cx.native().is_some()),
        "--variant {variant} alone left no native variant installed for its mode"
    );
}

/// Whether the icon-theme Select has a row reading `label`. The Select
/// shows its rows only in its popup, so this chooses `label` and asks
/// whether that found a row: it changes what is chosen.
fn icon_theme_row_exists(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    label: &str,
) -> bool {
    let label = gpui::SharedString::from(label.to_string());
    cx.update(|window, cx| {
        let select = showcase.read(cx).icon_theme_select.clone();
        select.update(cx, |select, cx| {
            select.set_selected_value(&label, window, cx);
            select.selected_index(cx).is_some()
        })
    })
}

/// The icon-theme Select follows the preset: its rows are rebuilt on every
/// preset change, a choice that followed the preset keeps following it --
/// `default`, or `system` where the preset's own icon theme is not installed
/// (`default_icon_choice`) -- and an icon theme the user chose stays chosen,
/// its rows offering the new preset's default and not the last one's.
#[gpui::test]
fn the_icon_theme_select_follows_the_preset(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let choice = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            (
                this.icon_set_choice.clone(),
                this.icon_theme_select.read(cx).selected_value().cloned(),
            )
        })
    };
    // A choice that followed the preset onto `system`, as it does where the
    // preset's own icon theme is not installed.
    set_preset(&mut cx, &showcase, "kde-breeze");
    cx.update(|_window, cx| {
        showcase.update(cx, |this, _| this.icon_set_choice = IconSetChoice::System)
    });
    set_preset(&mut cx, &showcase, "nord");
    assert_eq!(
        choice(&mut cx),
        (
            IconSetChoice::Default("lucide".into()),
            Some("default (lucide)".into())
        ),
        "a choice that followed kde-breeze onto system did not follow nord"
    );

    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Material (bundled)", window, cx)
        })
    });
    set_preset(&mut cx, &showcase, "kde-breeze");
    assert_eq!(
        choice(&mut cx),
        (IconSetChoice::Material, Some("Material (bundled)".into())),
        "an icon theme the user chose did not stay chosen across a preset change"
    );
    let fresh = read(
        &mut cx,
        &showcase,
        |this, _| match native_theme::icons::default_icon_choice(
            this.current_icon_set,
            this.current_icon_theme.as_deref(),
        ) {
            choice @ IconSetChoice::Default(_) => Some(choice.to_string()),
            _ => None,
        },
    );
    assert!(
        !icon_theme_row_exists(&mut cx, &showcase, "default (lucide)"),
        "under kde-breeze the icon-theme Select still offers nord's default row"
    );
    if let Some(fresh) = fresh {
        assert!(
            icon_theme_row_exists(&mut cx, &showcase, &fresh),
            "under kde-breeze the icon-theme Select does not offer its default row, {fresh}"
        );
    }
}

/// A system icon theme that cannot be detected is shown as such: the Icons
/// page's label says `unavailable` where it would name the theme, never a
/// theme nothing detected, and leaves the reason to the icon-theme Select's
/// hover text, as a label is short. Linux only, where the system's icons
/// are a freedesktop theme.
#[cfg(target_os = "linux")]
#[gpui::test]
fn a_failed_icon_theme_detection_is_shown_as_a_failure(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.icon_theme_detection_for_test = Some(|| {
                Err(native_theme::error::Error::PlatformUnsupported {
                    platform: "the test's failing detection",
                })
            });
            this.select_icon_set(&IconSetChoice::System.to_string(), window, cx);
        })
    });
    cx.run_until_parked();
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.icon_set_label()),
        "freedesktop (unavailable)",
        "the Icons page names an icon theme though its detection failed"
    );
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.icon_theme_unavailable()).as_deref(),
        Some(
            "System icon theme: unavailable (platform not supported: the test's failing detection)"
        ),
        "the Select's hover text does not give the reason"
    );
}

/// The Icons page's line on why the system icon theme is unavailable.
const ICONS_THEME_UNAVAILABLE: &str = "icons-theme-unavailable";

/// Where the system icon theme cannot be detected, the Icons page says why
/// under its Native Theme Icons heading, in the shared crate's text
/// (`rows::icon_theme_unavailable`, read through
/// `Showcase::icon_theme_unavailable`), as the iced and egui showcases'
/// Icons pages do; where it is detected, the line is not laid out. The first
/// arm takes the test's failing detection, so it runs on every host; the
/// second the system's own, which fails in the runner environment
/// (`scripts/run_in_runner_env.sh`) and on a desktop the detection does not
/// know.
#[gpui::test]
fn the_icons_page_says_why_the_system_icon_theme_is_unavailable(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let failing: fn() -> native_theme::Result<String> = || {
        Err(native_theme::error::Error::PlatformUnsupported {
            platform: "the test's failing detection",
        })
    };
    for detect in [Some(failing), None] {
        cx.update(|_window, cx| {
            showcase.update(cx, |this, cx| {
                this.icon_theme_detection_for_test = detect;
                cx.notify();
            })
        });
        show(&mut cx, &showcase, Page::Icons);
        let failed = detect
            .map_or_else(native_theme::detect::system_icon_theme, |detect| detect())
            .is_err();
        assert_eq!(
            cx.debug_bounds(ICONS_THEME_UNAVAILABLE).is_some(),
            failed,
            "the Icons page's line on the unavailable system icon theme (detection failed: \
             {failed})"
        );
        if detect.is_some() {
            assert_eq!(
                read(&mut cx, &showcase, |this, _| this.icon_theme_unavailable()).as_deref(),
                Some(
                    "System icon theme: unavailable (platform not supported: the test's \
                     failing detection)"
                ),
                "the Icons page's line does not give the reason"
            );
        }
    }
}

/// Where the system icon theme cannot be detected, the icon-theme Select
/// shows a tooltip on hover, the reason its `system` row's short label
/// leaves out; where it is detected, none. The tooltip's text is not
/// readable from the test's frame, so its box is what shows it: on hover
/// the Select paints a narrow box of its own in the tooltip's fill too
/// (18 px under kde-breeze), and the tooltip is wider than that.
#[gpui::test]
fn the_icon_theme_select_says_on_hover_why_the_system_theme_is_unavailable(
    cx: &mut TestAppContext,
) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    without_motion(&mut cx);
    let mut hovered = |detect: fn() -> native_theme::Result<String>| {
        cx.update(|_window, cx| {
            showcase.update(cx, |this, cx| {
                this.icon_theme_detection_for_test = Some(detect);
                cx.notify();
            })
        });
        cx.run_until_parked();
        tooltip_width(&mut cx, PROBE_ICON_THEME)
    };
    let detected = hovered(|| Ok("breeze".to_string()));
    let failed = hovered(|| {
        Err(native_theme::error::Error::PlatformUnsupported {
            platform: "the test's failing detection",
        })
    });
    assert!(
        failed.is_some() && failed > detected,
        "the Select's widest box in the tooltip's fill is {failed:?} with a failed detection \
         and {detected:?} with a detected theme: no tooltip says why"
    );
}

/// The preset switch's `default` row and the status bar name the same
/// preset: the one the pipeline settled on (`SystemTheme::preset`), which
/// on a desktop `native_theme::detect` does not recognise can differ from
/// `platform_preset_name`'s guess (native-theme pipeline.rs, `select_reader`).
#[gpui::test]
fn the_default_row_and_the_status_bar_name_one_preset(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let Ok(mut system) = system_theme() else {
        return;
    };
    // A platform preset other than the one platform_preset_name gives, as
    // the pipeline settles on where the desktop is not recognised.
    let guessed = native_theme::pipeline::platform_preset_name().name;
    let settled = if guessed == "adwaita" {
        "kde-breeze"
    } else {
        "adwaita"
    };
    system.preset = settled.to_string();
    set_preset(&mut cx, &showcase, "default");
    cx.update(|window, cx| showcase.update(cx, |this, cx| this.install_system(system, window, cx)));
    cx.run_until_parked();
    draw(&mut cx);
    let wanted = format!("default ({settled})");
    let status = read(&mut cx, &showcase, |this, cx| {
        crate::chrome::status_environment(this, cx)
    });
    assert!(
        status
            .get(1)
            .is_some_and(|item| item.starts_with(&format!("{wanted} "))),
        "the status bar does not name {wanted}: {status:?}"
    );
    let row = read(&mut cx, &showcase, |this, cx| {
        this.preset_combobox
            .read(cx)
            .selection()
            .first()
            .map(|(_, item)| item.display_name.to_string())
    });
    assert_eq!(
        row.as_deref(),
        Some(wanted.as_str()),
        "the preset switch's default row does not name the preset the status bar names"
    );
}

/// Switch the colour mode to `mode`, as the colour-mode Select does.
fn switch_mode(cx: &mut VisualTestContext, mode: AppColorMode) {
    cx.update(|window, cx| window.dispatch_action(Box::new(SetColorMode(mode)), cx));
    cx.run_until_parked();
    draw(cx);
}

/// Three colour-mode switches from the mode shown: to the other mode, back,
/// and to the other again.
fn three_switches(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> [AppColorMode; 3] {
    let (other, shown) = match read(cx, showcase, |this, _| this.is_dark) {
        true => (AppColorMode::Light, AppColorMode::Dark),
        false => (AppColorMode::Dark, AppColorMode::Light),
    };
    [other, shown, other]
}

/// The OS-theme reads and the icon loads the showcase has made on this
/// thread (`support::SYSTEM_READS`, `support::ICON_LOADS`).
fn reads_and_loads() -> (usize, usize) {
    (
        crate::support::SYSTEM_READS.with(std::cell::Cell::get),
        crate::support::ICON_LOADS.with(std::cell::Cell::get),
    )
}

/// The desktop's theme `default` was last installed from
/// (`Showcase::system`), asserted kept.
fn kept_system(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
) -> Option<native_theme::SystemTheme> {
    let system = read(cx, showcase, |this, _| this.system.clone());
    assert!(system.is_some(), "`default` kept no desktop theme");
    system
}

/// A colour-mode switch under `default` installs the other variant of the
/// desktop's theme it installed, without reading the OS again: three
/// switches read it no more, and Reload System Theme reads it once.
///
/// Each switch shows its mode with that mode's own variant's icon theme
/// (`SystemTheme::icon_theme_for`), not the one of the mode the OS is in:
/// on KDE with the OS light and the showcase dark, `breeze-dark`, where
/// `breeze` was loaded. A test cannot choose the OS's mode, so it shows the
/// showcase both and compares each with what a read of its own states.
#[gpui::test]
fn a_mode_switch_under_default_reads_the_os_only_on_reload(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let Ok(system) = system_theme() else {
        return;
    };
    set_preset(&mut cx, &showcase, "default");
    let (reads, _) = reads_and_loads();
    for mode in three_switches(&mut cx, &showcase) {
        switch_mode(&mut cx, mode);
        let (is_dark, icon_theme) = read(&mut cx, &showcase, |this, _| {
            (this.is_dark, this.current_icon_theme.clone())
        });
        assert_eq!(is_dark, mode.is_dark(), "{mode:?} was not the mode shown");
        assert_eq!(
            icon_theme.as_deref(),
            system.icon_theme_for(crate::app::color_mode_of(is_dark)),
            "{mode:?} under `default` took another variant's icon theme"
        );
    }
    assert_eq!(
        reads_and_loads().0 - reads,
        0,
        "three colour-mode switches under `default` read the OS theme again"
    );
    let (reads, _) = reads_and_loads();
    cx.update(|window, cx| window.dispatch_action(Box::new(ReloadTheme), cx));
    cx.run_until_parked();
    assert_eq!(
        reads_and_loads().0 - reads,
        1,
        "Reload System Theme did not read the OS theme once"
    );
}

/// An OS theme change the watcher reports is read in every colour mode: a
/// chosen Light or Dark picks only the variant, while the OS still owns the
/// accent, the fonts and the icon theme `default` shows. So under Dark, and
/// under Light, it reads the OS once and keeps the mode, and a switch to
/// System afterwards shows that read, not the one from before the change,
/// and reads nothing more.
#[gpui::test]
fn an_os_theme_change_is_read_in_every_colour_mode(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    if system_theme().is_err() {
        return;
    }
    set_preset(&mut cx, &showcase, "default");
    for (mode, dark) in [(AppColorMode::Dark, true), (AppColorMode::Light, false)] {
        switch_mode(&mut cx, mode);
        let stale = "the read before the OS theme changed";
        cx.update(|_window, cx| {
            showcase.update(cx, |this, _| {
                if let Some(system) = this.system.as_mut() {
                    system.name = stale.into();
                }
            })
        });
        assert_eq!(
            kept_system(&mut cx, &showcase).map(|s| s.name.to_string()),
            Some(stale.to_string()),
            "the kept read was not marked, so its replacement proves nothing"
        );
        let (reads, _) = reads_and_loads();
        cx.update(|window, cx| showcase.update(cx, |this, cx| this.on_os_theme_change(window, cx)));
        cx.run_until_parked();
        draw(&mut cx);
        assert_eq!(
            reads_and_loads().0 - reads,
            1,
            "under {mode:?} an OS theme change was not read"
        );
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.is_dark),
            dark,
            "an OS theme change under {mode:?} left {mode:?}"
        );
        switch_mode(&mut cx, AppColorMode::System);
        assert_eq!(
            reads_and_loads().0 - reads,
            1,
            "switching from {mode:?} to System read the OS theme again"
        );
        assert_ne!(
            kept_system(&mut cx, &showcase).map(|s| s.name.to_string()),
            Some(stale.to_string()),
            "switching from {mode:?} to System showed the read from before the OS theme changed"
        );
    }
}

/// Under a bundled preset an OS theme change keeps that preset installed,
/// and where its icons are a bundled set's, which no OS change reaches,
/// loads none of them again (`Showcase::reload_system_theme`).
#[gpui::test]
fn a_bundled_preset_stays_installed_through_an_os_theme_change(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let preset = "material-3";
    set_preset(&mut cx, &showcase, preset);
    let set = read(&mut cx, &showcase, |this, _| {
        this.icon_set_choice
            .effective_icon_set(this.current_icon_set)
    });
    assert!(
        !crate::support::is_system_set(set),
        "{preset}'s icons are {set:?}, a system set, so this test proves nothing"
    );
    let (_, loads) = reads_and_loads();
    cx.update(|window, cx| showcase.update(cx, |this, cx| this.on_os_theme_change(window, cx)));
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this
            .current_theme_name
            .clone()),
        preset,
        "an OS theme change did not keep {preset} installed"
    );
    assert_eq!(
        reads_and_loads().1 - loads,
        0,
        "an OS theme change loaded {preset}'s bundled {set:?} icons again"
    );
}

/// The icons load where what they are loaded as changed -- the set, its
/// icon theme, or, for a system set (freedesktop, SF Symbols, Segoe
/// Fluent), whose loaders colour a symbolic icon once, the colour they
/// colour it in, `defaults.font.color` -- and are recoloured otherwise. So
/// under `default` with Material chosen three colour-mode switches load
/// nothing; with the theme's own set each loads once where that set is a
/// system set and the two variants differ in icon theme or font colour
/// (computed from the desktop's theme kept, so it holds on every host),
/// else not at all; and a preset switch from a bundled set to a system one
/// loads once.
#[gpui::test]
fn icons_load_only_where_what_they_load_as_changed(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    if system_theme().is_err() {
        return;
    }
    set_preset(&mut cx, &showcase, "default");
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set(&IconSetChoice::Material.to_string(), window, cx)
        })
    });
    let (_, loads) = reads_and_loads();
    let mut recoloured = false;
    for mode in three_switches(&mut cx, &showcase) {
        // Read in the switch's own update, before a frame is drawn, which
        // would rebuild the caches for a new foreground itself (`render`).
        let (before, after, cached) = cx.update(|window, cx| {
            showcase.update(cx, |this, cx| {
                let before = Theme::global(cx).foreground;
                this.on_set_color_mode(&SetColorMode(mode), window, cx);
                (before, Theme::global(cx).foreground, this.icon_cache_fg)
            })
        });
        cx.run_until_parked();
        draw(&mut cx);
        assert_eq!(
            cached, after,
            "with Material chosen, {mode:?} under `default` left the icons in the foreground \
             before it"
        );
        recoloured |= before != after;
    }
    assert!(
        recoloured,
        "no switch changed the foreground, so the icons' recolouring was not tested"
    );
    assert_eq!(
        reads_and_loads().1 - loads,
        0,
        "with Material chosen, colour-mode switches under `default` loaded its icons again"
    );

    // The theme's own set again, as `--icon-set default` chooses it.
    apply_cli(
        &mut cx,
        &showcase,
        &crate::CliArgs {
            icon_set: Some("default".into()),
            ..Default::default()
        },
    );
    let [light, dark] = [
        native_theme::theme::ColorMode::Light,
        native_theme::theme::ColorMode::Dark,
    ];
    for mode in three_switches(&mut cx, &showcase) {
        let (_, loads) = reads_and_loads();
        switch_mode(&mut cx, mode);
        let system_set = read(&mut cx, &showcase, |this, _| {
            this.icon_set_enum
                .is_some_and(crate::support::is_system_set)
        });
        let variants_differ = kept_system(&mut cx, &showcase).is_some_and(|s| {
            s.icon_theme_for(light) != s.icon_theme_for(dark)
                || s.pick(light).defaults.font.color != s.pick(dark).defaults.font.color
        });
        assert_eq!(
            reads_and_loads().1 - loads,
            usize::from(system_set && variants_differ),
            "{mode:?} under `default` with the theme's own set (a system set: {system_set}; \
             the variants' icon theme or font colour differ: {variants_differ})"
        );
    }

    set_preset(&mut cx, &showcase, "material-3");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.icon_set_enum),
        Some(native_theme::theme::IconSet::Material),
        "material-3 did not take its bundled Material icons, so the switch below does not \
         start from a bundled set"
    );
    let (_, loads) = reads_and_loads();
    set_preset(&mut cx, &showcase, "kde-breeze");
    assert!(
        read(&mut cx, &showcase, |this, _| this
            .icon_set_enum
            .is_some_and(crate::support::is_system_set)),
        "kde-breeze did not take a system icon set"
    );
    assert_eq!(
        reads_and_loads().1 - loads,
        1,
        "a preset switch from a bundled set to a system one did not load its icons once"
    );
}

/// How long a colour-mode switch under `default` takes, with the theme's
/// own icon set and with Material chosen: the median of 21, each switch to
/// the mode not shown. A timing probe, ignored by default; run it on the
/// release profile, before and after a perf fix:
///
/// `cargo test -p native-theme-gpui --release --example showcase-gpui -- --ignored --nocapture --test-threads=1 timing_`
#[gpui::test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn timing_a_mode_switch_under_default(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "default");
    for (label, icons) in [
        ("the theme's own icon set", None),
        ("Material chosen", Some(IconSetChoice::Material)),
    ] {
        if let Some(choice) = icons {
            cx.update(|window, cx| {
                showcase.update(cx, |this, cx| {
                    this.select_icon_set(&choice.to_string(), window, cx)
                })
            });
        }
        let mut times: Vec<Duration> = (0..21)
            .map(|_| {
                let other = match read(&mut cx, &showcase, |this, _| this.is_dark) {
                    true => AppColorMode::Light,
                    false => AppColorMode::Dark,
                };
                let took = cx.update(|window, cx| {
                    showcase.update(cx, |this, cx| {
                        let start = std::time::Instant::now();
                        this.on_set_color_mode(&SetColorMode(other), window, cx);
                        start.elapsed()
                    })
                });
                cx.run_until_parked();
                draw(&mut cx);
                took
            })
            .collect();
        times.sort();
        let at = |i: usize| times.get(i).copied().unwrap_or_default();
        println!(
            "timing a mode switch under default, {label}: median {:?}, min {:?}, max {:?}",
            at(10),
            at(0),
            at(20)
        );
    }
}

/// The colour a system icon set's loaders are given: the installed native
/// theme's font colour, and none where no native theme is installed, so a
/// symbolic icon keeps its own colours. The placeholder font the showcase
/// shows where the OS theme could not be read is transparent black, and
/// the loaders keep only its red, green and blue: every recoloured icon
/// drew black, on a dark window too.
#[test]
fn icons_take_no_tint_without_a_native_theme() {
    let colour = native_theme::Rgba::rgb(0x23, 0x26, 0x29);
    assert_eq!(
        crate::app::icon_tint(false, native_theme::Rgba::TRANSPARENT),
        None
    );
    assert_eq!(crate::app::icon_tint(false, colour), None);
    assert_eq!(crate::app::icon_tint(true, colour), Some(colour));
}

/// A theme that fails to load leaves the installed one in charge of
/// everything shown: its name in the status bar and the preset switch, and
/// its layout. (`--theme` with a name the preset switch does not offer is
/// rejected before anything is loaded:
/// `the_command_line_rejects_what_it_cannot_honour`.)
#[gpui::test]
fn a_failed_theme_leaves_the_installed_one_named(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "nord");
    let nord_layout = native_theme::theme::Theme::preset("nord")
        .map(|t| t.layout)
        .ok();
    let named = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            (
                this.current_theme_name.clone(),
                this.preset_combobox
                    .read(cx)
                    .selected_value()
                    .map(|v| v.to_string()),
                crate::chrome::status_environment(this, cx)
                    .get(1)
                    .cloned()
                    .unwrap_or_default(),
                Some(this.layout.clone()),
            )
        })
    };
    let check = |cx: &mut VisualTestContext, how: &str| {
        let (name, row, status, layout) = named(cx);
        assert_eq!(
            name, "nord",
            "{how}: the showcase names the theme that failed"
        );
        assert_eq!(
            row.as_deref(),
            Some("nord"),
            "{how}: the preset switch shows the theme that failed"
        );
        assert!(
            status.starts_with("nord "),
            "{how}: the status bar does not name nord: {status}"
        );
        assert!(
            layout == nord_layout,
            "{how}: the layout is no longer nord's"
        );
    };
    set_missing_preset(&mut cx, &showcase, "no-such-preset");
    assert!(
        read(&mut cx, &showcase, |this, _| this.error_message.is_some()),
        "no-such-preset loaded, so nothing failed"
    );
    check(&mut cx, "SetPreset(no-such-preset)");
}

/// What the window draws with: gpui-component's mode, every `ThemeColor`
/// and `ThemeTokens` value, and the showcase's own colour mode, light/dark
/// flag, theme name and layout.
fn drawn_state(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
) -> (bool, String, String, AppColorMode, bool, String, String) {
    read(cx, showcase, |this, cx| {
        let theme = Theme::global(cx);
        (
            theme.mode.is_dark(),
            format!("{:?}", theme.colors),
            format!("{:?}", theme.tokens),
            this.color_mode,
            this.is_dark,
            this.current_theme_name.clone(),
            format!("{:?}", this.layout),
        )
    })
}

/// A theme that fails to load while a native theme is installed leaves the
/// window exactly as the installed theme drew it: its mode, every colour
/// and token, and the showcase's colour mode, name and layout. kde-breeze
/// is installed through `apply`, one variant, so a switch of upstream's
/// mode would show upstream's own colours for the other.
///
/// Two failures: a preset that does not exist, and a colour-mode switch
/// whose re-install fails -- as it does under `default` where the OS theme
/// cannot be read, which a test cannot bring about, so the installed
/// theme's name is made one that fails to load.
#[gpui::test]
fn a_failed_install_leaves_the_window_as_the_installed_theme_drew_it(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "kde-breeze");
    let before = drawn_state(&mut cx, &showcase);

    set_missing_preset(&mut cx, &showcase, "no-such-preset");
    let after = drawn_state(&mut cx, &showcase);
    assert!(
        after == before,
        "a preset that failed to load changed what the window draws: {:?}",
        changed_fields(&before, &after)
    );

    cx.update(|_window, cx| {
        showcase.update(cx, |this, _| {
            this.current_theme_name = "no-such-preset".into()
        })
    });
    let other = if before.0 {
        AppColorMode::Light
    } else {
        AppColorMode::Dark
    };
    cx.update(|window, cx| window.dispatch_action(Box::new(SetColorMode(other)), cx));
    cx.run_until_parked();
    draw(&mut cx);
    let after = drawn_state(&mut cx, &showcase);
    assert!(
        (after.0, &after.1, &after.2, after.3, after.4)
            == (before.0, &before.1, &before.2, before.3, before.4),
        "a colour-mode switch whose re-install failed changed what the window draws: {:?}",
        changed_fields(&before, &after)
    );
}

/// Put the showcase into the state `args` asks for, as `main` does.
fn apply_cli(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, args: &crate::CliArgs) {
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| crate::apply_cli_args(this, args, window, cx))
    });
    cx.run_until_parked();
    draw(cx);
}

/// The colour mode the colour-mode Select shows.
fn color_mode_shown(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> Option<String> {
    read(cx, showcase, |this, cx| {
        this.color_mode_select
            .read(cx)
            .selected_value()
            .map(|v| v.to_string())
    })
}

/// `--theme X --variant V` where the preset switch does not offer X ignores
/// X, as if `--theme` were not given: V installs the theme installed, in
/// its mode, and the colour-mode Select shows V.
#[gpui::test]
fn a_rejected_cli_theme_leaves_the_variant_to_apply(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "kde-breeze");
    let was_dark = cx.update(|_w, cx| Theme::global(cx).mode.is_dark());
    let (variant, wanted) = if was_dark {
        ("light", AppColorMode::Light)
    } else {
        ("dark", AppColorMode::Dark)
    };
    apply_cli(
        &mut cx,
        &showcase,
        &crate::CliArgs {
            theme: Some("no-such-preset".to_string()),
            variant: Some(variant.to_string()),
            ..Default::default()
        },
    );
    let how = format!("--theme no-such-preset --variant {variant}");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| (
            this.current_theme_name.clone(),
            this.color_mode
        )),
        ("kde-breeze".to_string(), wanted),
        "{how}: not kde-breeze in the mode --variant names"
    );
    assert_eq!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        !was_dark,
        "{how}: the window is not drawn in the mode --variant names"
    );
    assert_eq!(
        color_mode_shown(&mut cx, &showcase),
        Some(wanted.short_label().to_string()),
        "{how}: the colour-mode Select does not show the mode --variant names"
    );
}

/// `--variant V` whose install fails leaves the window in the mode the
/// installed theme drew, and the colour-mode Select shows that mode, as a
/// failed colour-mode switch does. A `--theme` the preset switch offers
/// fails to load only where the OS theme cannot be read, which a test
/// cannot bring about; `--variant` alone installs through the same
/// `Showcase::install_in_mode`, so the installed theme's name is made one
/// that fails to load.
#[gpui::test]
fn a_cli_variant_whose_install_fails_keeps_the_colour_mode_shown(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    set_preset(&mut cx, &showcase, "kde-breeze");
    let before = drawn_state(&mut cx, &showcase);
    let variant = if before.0 { "light" } else { "dark" };
    cx.update(|_window, cx| {
        showcase.update(cx, |this, _| {
            this.current_theme_name = "no-such-preset".into()
        })
    });
    apply_cli(
        &mut cx,
        &showcase,
        &crate::CliArgs {
            variant: Some(variant.to_string()),
            ..Default::default()
        },
    );
    let after = drawn_state(&mut cx, &showcase);
    assert!(
        (after.0, &after.1, &after.2, after.3, after.4)
            == (before.0, &before.1, &before.2, before.3, before.4),
        "--variant {variant} whose install failed changed what the window draws: {:?}",
        changed_fields(&before, &after)
    );
    assert_eq!(
        color_mode_shown(&mut cx, &showcase),
        Some(before.3.short_label().to_string()),
        "--variant {variant} whose install failed: the colour-mode Select shows a mode \
         the window is not drawn in"
    );
}

/// `--variant` takes `light`, `dark` or `system` (follow the OS); `--theme`
/// takes what the preset switch offers, which is only this platform's
/// presets. Anything else is ignored: the theme and mode stay, and the
/// preset switch still shows the theme installed.
#[gpui::test]
fn the_command_line_rejects_what_it_cannot_honour(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let offered = native_theme::theme::Theme::list_presets_for_platform();
    let Some(home) = offered.first().map(|info| info.key.to_string()) else {
        panic!("no preset is offered on this platform");
    };
    set_preset(&mut cx, &showcase, &home);
    let state = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            (
                this.current_theme_name.clone(),
                this.preset_combobox
                    .read(cx)
                    .selected_value()
                    .map(|v| v.to_string()),
                this.color_mode,
                this.is_dark,
            )
        })
    };
    let before = state(&mut cx);
    apply_cli(
        &mut cx,
        &showcase,
        &crate::CliArgs {
            variant: Some("sideways".to_string()),
            ..Default::default()
        },
    );
    assert_eq!(
        state(&mut cx),
        before,
        "--variant sideways changed the state"
    );

    for (variant, wanted) in [
        ("dark", AppColorMode::Dark),
        ("light", AppColorMode::Light),
        ("system", AppColorMode::System),
    ] {
        apply_cli(
            &mut cx,
            &showcase,
            &crate::CliArgs {
                variant: Some(variant.to_string()),
                ..Default::default()
            },
        );
        assert_eq!(
            read(&mut cx, &showcase, |this, _| (
                this.current_theme_name.clone(),
                this.color_mode
            )),
            (home.clone(), wanted),
            "--variant {variant} did not install {home} in its mode"
        );
    }

    let foreign = native_theme::theme::Theme::list_presets()
        .iter()
        .map(|info| info.key)
        .find(|key| !offered.iter().any(|info| info.key == *key));
    let before = state(&mut cx);
    match foreign {
        Some(foreign) => {
            apply_cli(
                &mut cx,
                &showcase,
                &crate::CliArgs {
                    theme: Some(foreign.to_string()),
                    ..Default::default()
                },
            );
            assert_eq!(
                state(&mut cx),
                before,
                "--theme {foreign}, a preset of another platform, changed the state"
            );
        }
        None => eprintln!("every preset is offered on this platform: no foreign preset checked"),
    }
    apply_cli(
        &mut cx,
        &showcase,
        &crate::CliArgs {
            theme: Some("no-such-preset".to_string()),
            ..Default::default()
        },
    );
    assert_eq!(
        state(&mut cx),
        before,
        "--theme no-such-preset changed the state"
    );
}

/// `--theme` installs what the preset switch offers, which the screenshot
/// workflow passes: each preset meant for this platform, and `default`, the
/// desktop's own theme. The theme is installed, and the preset switch and
/// the status bar name it. `default` is skipped where the host's OS theme
/// cannot be read, which on Linux is never ([`system_theme`]).
#[gpui::test]
fn the_command_line_installs_what_it_accepts(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let mut accepted: Vec<String> = native_theme::theme::Theme::list_presets_for_platform()
        .iter()
        .map(|info| info.key.to_string())
        .collect();
    assert!(
        !accepted.is_empty(),
        "no preset is offered on this platform"
    );
    match system_theme() {
        Ok(_) => accepted.push("default".to_string()),
        Err(e) => eprintln!("the OS theme cannot be read here ({e}): --theme default not checked"),
    }
    for key in accepted {
        apply_cli(
            &mut cx,
            &showcase,
            &crate::CliArgs {
                theme: Some(key.clone()),
                ..Default::default()
            },
        );
        let (name, row, status, error, native) = read(&mut cx, &showcase, |this, cx| {
            (
                this.current_theme_name.clone(),
                this.preset_combobox
                    .read(cx)
                    .selected_value()
                    .map(|v| v.to_string()),
                crate::chrome::status_environment(this, cx)
                    .get(1)
                    .cloned()
                    .unwrap_or_default(),
                this.error_message.clone(),
                cx.native().is_some(),
            )
        });
        assert_eq!(name, key, "--theme {key} did not install it");
        assert_eq!(error, None, "--theme {key} reported an error");
        assert!(native, "--theme {key} left no native variant installed");
        assert_eq!(
            row.as_deref(),
            Some(key.as_str()),
            "--theme {key}: the preset switch does not show it"
        );
        let named = match key.as_str() {
            "default" => status.starts_with("default ("),
            preset => status.starts_with(&format!("{preset} ")),
        };
        assert!(
            named,
            "--theme {key}: the status bar does not name it: {status}"
        );
    }
}

/// Make `names` the installed freedesktop themes the icon-theme Select
/// lists, whatever the host has installed, and rebuild its rows.
fn list_icon_themes(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, names: &[&str]) {
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.installed_themes = names.iter().map(|name| name.to_string()).collect();
            this.show_icon_choice(window, cx);
        })
    });
    cx.run_until_parked();
}

/// The icon choice, the set drawn from (`None` for gpui-component's own),
/// the Icons page's name for it and the icon-theme Select's row.
fn icon_choice_state(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
) -> (
    IconSetChoice,
    Option<native_theme::theme::IconSet>,
    String,
    Option<String>,
) {
    read(cx, showcase, |this, cx| {
        (
            this.icon_set_choice.clone(),
            this.icon_set_enum,
            this.icon_theme_name.clone(),
            this.icon_theme_select
                .read(cx)
                .selected_value()
                .map(|v| v.to_string()),
        )
    })
}

/// `--icon-set` takes what the icon-theme Select offers -- `default`, a
/// bundled set, `gpui-builtin`, `system` or `freedesktop` (the system icon
/// theme), or a freedesktop theme the Select lists -- and the Select shows
/// it. Anything else is ignored: the icon choice stays. `default` follows
/// the theme again: the Select's `default` row where the theme's icon theme
/// is available, `system` where it is not, as the Select's rows are.
#[gpui::test]
fn the_icon_set_flag_takes_what_the_select_offers(cx: &mut TestAppContext) {
    use native_theme::theme::IconSet;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let listed = "a-listed-icon-theme";
    list_icon_themes(&mut cx, &showcase, &[listed]);
    let icon_set = |cx: &mut VisualTestContext, name: &str| {
        apply_cli(
            cx,
            &showcase,
            &crate::CliArgs {
                icon_set: Some(name.to_string()),
                ..Default::default()
            },
        );
        icon_choice_state(cx, &showcase)
    };
    let system = IconSetChoice::System.effective_icon_set(IconSet::Material);
    let cases = [
        (
            "material",
            (
                IconSetChoice::Material,
                Some(IconSet::Material),
                IconSet::Material.name().to_string(),
                Some(IconSetChoice::Material.to_string()),
            ),
        ),
        (
            "freedesktop",
            (
                IconSetChoice::System,
                Some(system),
                system.name().to_string(),
                Some(IconSetChoice::System.to_string()),
            ),
        ),
        (
            "gpui-builtin",
            (
                IconSetChoice::Lucide,
                None,
                "gpui-builtin".to_string(),
                Some(crate::app::GPUI_BUILTIN_ROW.to_string()),
            ),
        ),
        (
            "lucide",
            (
                IconSetChoice::Lucide,
                Some(IconSet::Lucide),
                IconSet::Lucide.name().to_string(),
                Some(IconSetChoice::Lucide.to_string()),
            ),
        ),
        (
            listed,
            (
                IconSetChoice::Freedesktop(listed.to_string()),
                Some(IconSet::Freedesktop),
                IconSet::Freedesktop.name().to_string(),
                Some(listed.to_string()),
            ),
        ),
    ];
    for (name, wanted) in cases {
        assert_eq!(
            icon_set(&mut cx, name),
            wanted,
            "--icon-set {name} did not choose, and show, the row the Select offers for it"
        );
        for unlisted in ["no-such-icon-theme", "hicolor"] {
            assert_eq!(
                icon_set(&mut cx, unlisted),
                wanted,
                "--icon-set {unlisted}, which the Select does not offer, replaced --icon-set \
                 {name}'s choice"
            );
        }
    }

    // The choice `default` gives, and whether the choice then follows the
    // theme, read from the installed theme at test time.
    let following = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, _| {
            let theme = this
                .current_icon_theme
                .as_deref()
                .filter(|_| this.has_toml_icon_theme);
            (
                native_theme::icons::default_icon_choice(this.current_icon_set, theme),
                this.icon_choice_follows_preset,
            )
        })
    };
    // A bundled set's icon theme is always available: its `default` row.
    set_preset(&mut cx, &showcase, "catppuccin-mocha");
    icon_set(&mut cx, "material");
    let (choice, state) = (following(&mut cx).0, icon_set(&mut cx, "default"));
    assert!(
        matches!(choice, IconSetChoice::Default(_)),
        "catppuccin-mocha's icon theme is not its default row: {choice:?}"
    );
    assert_eq!(
        (state.0, state.3),
        (choice.clone(), Some(choice.to_string())),
        "--icon-set default did not choose, and show, the Select's default row"
    );
    assert!(
        following(&mut cx).1,
        "after --icon-set default the icon choice does not follow the theme"
    );
    // A freedesktop theme naming an icon theme that is not installed: the
    // Select has no `default` row, and `default` is `system`.
    set_preset(&mut cx, &showcase, "adwaita");
    cx.update(|_window, cx| {
        showcase.update(cx, |this, _| {
            this.current_icon_theme = Some("no-such-icon-theme".to_string())
        })
    });
    icon_set(&mut cx, "material");
    let state = icon_set(&mut cx, "default");
    assert_eq!(
        following(&mut cx),
        (IconSetChoice::System, true),
        "under an icon theme that is not installed --icon-set default does not follow the \
         theme to system"
    );
    assert_eq!(
        (state.0, state.3),
        (
            IconSetChoice::System,
            Some(IconSetChoice::System.to_string())
        ),
        "under an icon theme that is not installed --icon-set default does not choose, and \
         show, system"
    );
}

/// `--tab` takes each page's name; another name is reported with the pages
/// it could have been.
#[test]
fn the_tab_flag_names_every_page() {
    for page in Page::ALL {
        assert_eq!(Page::named(page.flag()), Ok(page));
    }
    assert_eq!(Page::named("basic"), Ok(Page::Basic));
    match Page::named("no-such-page") {
        Ok(page) => panic!("--tab no-such-page opened {page:?}"),
        Err(error) => {
            for page in Page::ALL {
                assert!(error.contains(page.flag()), "{error}");
            }
        }
    }
}

/// The pages are in the order of the counterpart table in
/// `docs/showcase-elements.toml`'s header, under the labels its gpui column
/// gives them, and each page's `--tab` key is its label as the header says:
/// lower case, each space a `-`.
#[test]
fn the_pages_follow_the_counterpart_table() {
    let table = match native_theme_showcase_support::elements::page_order(
        native_theme_showcase_support::Kind::Gpui,
    ) {
        Ok(table) => table,
        Err(error) => panic!("{error}"),
    };
    let labels: Vec<&str> = Page::ALL.iter().map(|page| page.label()).collect();
    assert_eq!(labels, table, "the TabBar against the counterpart table");
    for page in Page::ALL {
        assert_eq!(
            page.flag(),
            page.label().to_lowercase().replace(' ', "-"),
            "{page:?}'s --tab key"
        );
    }
}

/// The parts of two `drawn_state`s that differ, as `name: before -> after`,
/// the colours and tokens field by field.
fn changed_fields(
    before: &(bool, String, String, AppColorMode, bool, String, String),
    after: &(bool, String, String, AppColorMode, bool, String, String),
) -> Vec<String> {
    let fields = |debug: &str| -> Vec<String> {
        debug.split("}, ").map(|field| field.to_string()).collect()
    };
    let mut changed = Vec::new();
    for (name, b, a) in [
        ("mode dark", before.0.to_string(), after.0.to_string()),
        (
            "color_mode",
            format!("{:?}", before.3),
            format!("{:?}", after.3),
        ),
        ("is_dark", before.4.to_string(), after.4.to_string()),
        ("name", before.5.clone(), after.5.clone()),
    ] {
        if b != a {
            changed.push(format!("{name}: {b} -> {a}"));
        }
    }
    for (b, a) in [(&before.1, &after.1), (&before.2, &after.2)] {
        changed.extend(
            fields(b)
                .into_iter()
                .zip(fields(a))
                .filter(|(b, a)| b != a)
                .map(|(b, a)| format!("{b} -> {a}")),
        );
    }
    if before.6 != after.6 {
        changed.push("layout".to_string());
    }
    changed
}

/// Hiding the side panel takes its widgets off the screen, so an info of
/// one of them is no longer shown (spec §4.3.4): the status bar would
/// otherwise name a widget nothing draws.
#[gpui::test]
fn hiding_the_side_panel_clears_what_left_the_screen(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let tabs = bounds_of(&mut cx, INSPECTOR_TABS);
    hover(&mut cx, point(tabs.left() + px(16.), tabs.center().y));
    settle(&mut cx);
    let shown = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().map(|info| info.title())
        })
    };
    assert_eq!(
        shown(&mut cx).as_deref(),
        Some(INSPECTOR_TABS_TITLE),
        "the inspector's TabBar did not report itself"
    );
    run_menu_item(&mut cx, "View", "Toggle Side Panel");
    let padding = in_page_padding(&mut cx, &showcase);
    hover(&mut cx, padding);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        shown(&mut cx),
        None,
        "with the side panel hidden the info of its TabBar is still shown"
    );
}

/// The toolbar's items are spaced where the theme states no
/// `toolbar.item_gap` (spec §3.1): nord states none, so they are
/// `layout.widget_gap` apart; with that unstated too, the showcase's own
/// `TOOLBAR_GAP`. The row's info names which.
#[gpui::test]
fn the_toolbar_items_are_spaced_where_the_theme_states_no_gap(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "nord");
    let (item_gap, widget_gap) = read(&mut cx, &showcase, |this, cx| {
        (
            cx.native().map(|n| n.resolved.toolbar.item_gap),
            geometry::widget_gap(&this.layout),
        )
    });
    assert_eq!(
        item_gap,
        Some(None),
        "nord states a toolbar.item_gap, or no native theme is installed, so the fallback is \
         not what is measured"
    );
    let Some(widget_gap) = widget_gap else {
        panic!("nord states no layout.widget_gap, so the showcase's constant is measured twice");
    };
    let gap = |cx: &mut VisualTestContext| {
        bounds_of(cx, CHROME_TOOLBAR_RELOAD).left() - bounds_of(cx, CHROME_TOOLBAR_PALETTE).right()
    };
    assert_eq!(
        gap(&mut cx),
        widget_gap,
        "nord: the toolbar's first two items are not layout.widget_gap apart"
    );
    let gap_line = |cx: &mut VisualTestContext| {
        let bar = bounds_of(cx, CHROME_TOOLBAR);
        hover(cx, point(bar.right() - px(8.), bar.center().y));
        settle(cx);
        draw(cx);
        read(cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().and_then(|info| {
                info.instance
                    .iter()
                    .find(|n| n.what == "gap")
                    .map(|n| n.text.clone())
            })
        })
    };
    let line = gap_line(&mut cx);
    assert!(
        line.as_deref()
            .is_some_and(|l| l.starts_with("6px, layout.widget_gap")),
        "nord: the toolbar's info does not say its gap is layout.widget_gap: {line:?}"
    );

    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.layout = native_theme::theme::LayoutTheme::default();
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        gap(&mut cx),
        crate::demo::TOOLBAR_GAP,
        "with neither toolbar.item_gap nor widget_gap stated, the toolbar's items are not the \
         showcase's own TOOLBAR_GAP apart"
    );
    let line = gap_line(&mut cx);
    assert!(
        line.as_deref()
            .is_some_and(|l| l.contains("TOOLBAR_GAP") && l.contains("the showcase's own")),
        "the toolbar's info does not say its gap is the showcase's own TOOLBAR_GAP: {line:?}"
    );
}

/// The status bar reports the installed accessibility preferences (spec
/// §2.7): the text-scale factor always, and a flag only while it is set.
#[gpui::test]
fn the_status_bar_reports_the_accessibility_preferences(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let mut prefs = native_theme_gpui::AccessibilityPreferences::default();
    prefs.text_scaling_factor = 1.5;
    prefs.reduce_motion = false;
    prefs.high_contrast = true;
    prefs.reduce_transparency = false;
    cx.update(|_window, cx| native_theme_gpui::apply_accessibility(&prefs, cx));
    cx.run_until_parked();
    let items = read(&mut cx, &showcase, crate::chrome::status_environment);
    assert!(
        items.iter().any(|i| i == "text ×1.5"),
        "the status bar does not report the 1.5 text scale: {items:?}"
    );
    assert!(
        items.iter().any(|i| i == "high_contrast"),
        "the status bar does not report high contrast, which is set: {items:?}"
    );
    for unset in ["reduce_motion", "reduce_transparency"] {
        assert!(
            !items.iter().any(|i| i == unset),
            "the status bar reports {unset}, which is not set: {items:?}"
        );
    }
}

/// The status bar's font item and the Theme tab's `font_family` and
/// `mono_font_family` rows name the family gpui draws, `Theme::font_family`
/// and `mono_font_family` (the connector's `font_family`): the stated name
/// alone where it is drawn, else the stated name and the platform's
/// substitute. windows-11 and kde-breeze, each in the form this host gives:
/// on a system without Segoe UI the first names its substitute.
///
/// It guards the plumbing: that the status bar and the Theme tab print the
/// family gpui-component's `Theme` holds, and the stated name beside it.
/// Its expectation reads `Theme::font_family`, the value the code reads, so
/// it does not show that this family is the one the platform draws: the
/// connector's `font_family` and native-theme's fontconfig lookup
/// (`native_theme::fonts`) hold that under their own tests.
#[gpui::test]
fn the_status_bar_and_the_theme_tab_name_the_drawn_family(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let label = |stated: &str, drawn: &str| {
        if stated == drawn {
            stated.to_string()
        } else {
            format!("{stated} (not on this system; drawn in the platform's substitute, {drawn})")
        }
    };
    for preset in ["windows-11", "kde-breeze"] {
        use_preset(&mut cx, &showcase, preset);
        let (status, rows, font, mono, drawn) = read(&mut cx, &showcase, |this, cx| {
            let theme = Theme::global(cx);
            (
                crate::chrome::status_environment(this, cx),
                crate::inspector::font_rows(this, cx),
                this.original_font.clone(),
                this.original_mono_font.clone(),
                (
                    theme.font_family.to_string(),
                    theme.mono_font_family.to_string(),
                ),
            )
        });
        let family = label(&font.family, &drawn.0);
        let mono_family = label(&mono.family, &drawn.1);
        println!("{preset}: {family}; {mono_family}");
        let size = native_theme_showcase_support::status::defined_size(&font);
        assert_eq!(
            status.get(2),
            Some(&format!("{family} {size}")),
            "{preset}: the status bar's font item does not name the drawn family: {status:?}"
        );
        let row = |what: &str| {
            rows.iter()
                .find(|(name, _)| *name == what)
                .map(|(_, value)| value.clone())
        };
        assert_eq!(
            row("font_family"),
            Some(family),
            "{preset}: the Theme tab's font_family row"
        );
        assert_eq!(
            row("mono_font_family"),
            Some(mono_family),
            "{preset}: the Theme tab's mono_font_family row"
        );
    }
}

/// Press `keys` and draw the frames that follow: an overlay pushed onto
/// `Root` mounts in the first and settles in the second.
fn press(cx: &mut VisualTestContext, keys: &str) {
    cx.simulate_keystrokes(keys);
    cx.run_until_parked();
    draw(cx);
    draw(cx);
}

/// Motion reduced, so an overlay's entrance animation settles on the frame
/// that mounts it and a measurement of it is a measurement at rest
/// (upstream's own dialog tests do the same, dialog/dialog.rs, `window`).
fn without_motion(cx: &mut VisualTestContext) {
    cx.update(|_window, cx| cx.set_reduce_motion(true));
}

/// Whether a dialog is on screen: upstream names each dialog's surface
/// `dialog-<layer>` (dialog/dialog.rs, Dialog::render).
fn a_dialog_is_open(cx: &mut VisualTestContext) -> bool {
    cx.debug_bounds("dialog-0").is_some()
}

/// Ctrl+K opens the command palette (spec §2.2, §2.8): a Dialog, with the
/// palette's Command on its surface.
#[gpui::test]
fn ctrl_k_opens_the_command_palette(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert!(!a_dialog_is_open(&mut cx), "a dialog is open at start");
    press(&mut cx, "secondary-k");
    assert!(
        cx.debug_bounds("dialog-layer").is_some(),
        "Ctrl+K drew no dialog layer"
    );
    let surface = bounds_of(&mut cx, "dialog-0");
    let palette = bounds_of(&mut cx, OVERLAY_PALETTE);
    assert!(
        surface.contains(&palette.origin) && palette.bottom() <= surface.bottom(),
        "the palette at {palette:?} is not on the dialog's surface at {surface:?}"
    );
}

/// The palette's entries run: typing a page's name and pressing Enter shows
/// that page and closes the palette.
#[gpui::test]
fn the_palette_switches_page(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_ne!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Charts,
        "the showcase starts on Charts, so showing it proves nothing"
    );
    press(&mut cx, "secondary-k");
    cx.simulate_input("Charts");
    cx.run_until_parked();
    draw(&mut cx);
    press(&mut cx, "enter");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Charts,
        "\"Charts\" and Enter in the palette did not show the Charts page"
    );
    assert!(
        !a_dialog_is_open(&mut cx),
        "the palette stayed open after running an entry"
    );
}

/// The palette's preset entries install the preset, and the theme settings'
/// preset switch shows the one installed.
#[gpui::test]
fn the_palette_installs_a_preset(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    press(&mut cx, "secondary-k");
    cx.simulate_input("Nord");
    cx.run_until_parked();
    draw(&mut cx);
    press(&mut cx, "enter");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this
            .current_theme_name
            .clone()),
        "nord",
        "\"Nord\" and Enter in the palette did not install nord"
    );
    assert_eq!(
        read(&mut cx, &showcase, |this, cx| {
            this.preset_combobox.read(cx).selected_value()
        })
        .as_deref(),
        Some("nord"),
        "the preset switch does not show the preset the palette installed"
    );
}

/// A shortcut pressed while the palette's query has the focus still reaches
/// the showcase: its handlers wrap the overlays (host.rs), not only the view.
#[gpui::test]
fn a_shortcut_works_while_the_palette_has_the_focus(cx: &mut TestAppContext) {
    let (showcase, _window, mut cx) = open(cx, WINDOW_SIZE);
    let before = read(&mut cx, &showcase, |this, _| this.side_panel_visible);
    press(&mut cx, "secondary-k");
    assert!(a_dialog_is_open(&mut cx), "the palette did not open");
    press(&mut cx, "secondary-b");
    assert_ne!(
        read(&mut cx, &showcase, |this, _| this.side_panel_visible),
        before,
        "Ctrl+B inside the palette did not toggle the side panel"
    );
}

/// The capture shield covers an open dialog too: nothing on it is hovered.
/// A dialog paints deferred (gpui-base dialog.rs, `Dialog::render`), so the
/// shield must be deferred above it, not merely last.
#[gpui::test]
fn the_pointer_shield_covers_an_open_dialog(cx: &mut TestAppContext) {
    let (showcase, _window, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.pointer_shield = true;
            cx.notify();
        });
    });
    run_menu_item(&mut cx, "Help", "About");
    draw(&mut cx);
    assert!(
        settle_on(&mut cx, &showcase, OVERLAY_ABOUT_NAME).is_none(),
        "under the shield the About dialog is still hovered"
    );
}

/// The palette's preset entries are the preset switch's (ledger ruling for T12):
/// the rows the preset Combobox's delegate holds, in its order -- `default`,
/// then only presets meant for this platform.
#[test]
fn the_palette_offers_the_preset_switchs_presets() {
    use gpui_component::searchable_list::{SearchableListDelegate as _, SearchableListItem as _};
    // Any platform preset will do: both lists label `default` with the one
    // they are given.
    let default_preset = "adwaita";
    let rows: Vec<(String, String)> = crate::chrome::palette_presets(default_preset)
        .into_iter()
        .map(|(key, name)| (key.to_string(), name.to_string()))
        .collect();
    let delegate = crate::support::PresetDelegate::new(default_preset);
    let switch: Vec<(String, String)> = (0..delegate.items_count(0))
        .filter_map(|row| delegate.item(gpui_component::IndexPath::default().row(row)))
        .map(|item| (item.value().to_string(), item.title().to_string()))
        .collect();
    assert_eq!(
        rows, switch,
        "the palette's presets are not the preset Combobox's rows"
    );
    assert_eq!(
        rows.first(),
        Some(&("default".to_string(), "default (adwaita)".to_string())),
        "the palette's first preset is not the desktop's own, named by its preset"
    );
    let offered: Vec<String> = rows.into_iter().map(|(key, _)| key).collect();
    let platform: Vec<&str> = native_theme::theme::Theme::list_presets_for_platform()
        .iter()
        .map(|info| info.key)
        .collect();
    for key in offered.iter().skip(1) {
        assert!(
            platform.contains(&key.as_str()),
            "the palette offers {key}, which is not a preset for this platform"
        );
    }
    assert!(
        offered.len() > 1,
        "the palette offers no preset beside default"
    );
}

/// An overlay opens once (fix round 1): while one is open, asking for any of
/// the three again -- by key or from a menu -- opens no second layer over it,
/// as a desktop application's modal dialog keeps the others out.
#[gpui::test]
fn an_overlay_opens_once(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    press(&mut cx, "secondary-k");
    assert!(a_dialog_is_open(&mut cx), "Ctrl+K opened nothing");
    press(&mut cx, "secondary-k");
    assert!(
        cx.debug_bounds("dialog-1").is_none(),
        "a second Ctrl+K opened a second palette over the first"
    );
    run_menu_item_from_focus(&mut cx, "Help", "About");
    assert!(
        cx.debug_bounds("dialog-1").is_none(),
        "Help > About opened over the palette"
    );
    press(&mut cx, "secondary-,");
    assert!(
        cx.debug_bounds(OVERLAY_PREFERENCES).is_none(),
        "Ctrl+, opened the Preferences sheet under the palette"
    );
    press(&mut cx, "escape");
    press(&mut cx, "secondary-,");
    assert!(
        cx.debug_bounds(OVERLAY_PREFERENCES).is_some(),
        "Ctrl+, did not open the Preferences sheet"
    );
    run_menu_item_from_focus(&mut cx, "Help", "About");
    assert!(
        !a_dialog_is_open(&mut cx),
        "Help > About opened over the Preferences sheet"
    );
    press(&mut cx, "secondary-k");
    assert!(
        !a_dialog_is_open(&mut cx),
        "Ctrl+K opened the palette over the Preferences sheet"
    );
}

/// A theme switch clears an info whose target is no longer drawn, as a
/// page change does: settled on a Preferences switch, the sheet closed and
/// the colour mode switched, the inspector would otherwise keep showing the
/// switch's colours under the theme before. The Preferences sheet is shut by
/// Escape, and the pointer is moved into the page's own padding, where no
/// widget can settle in its place.
#[gpui::test]
fn a_theme_switch_clears_what_left_the_screen(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    press(&mut cx, "secondary-,");
    let info = settle_on(&mut cx, &showcase, PREF_REDUCE_MOTION);
    assert!(
        info.as_ref().is_some_and(|info| info.kind == "Switch"),
        "the Reduce motion switch did not report itself: {info:?}"
    );
    press(&mut cx, "escape");
    assert!(
        cx.debug_bounds(OVERLAY_PREFERENCES).is_none(),
        "Escape did not close the Preferences sheet"
    );
    let padding = in_page_padding(&mut cx, &showcase);
    hover(&mut cx, padding);
    settle(&mut cx);
    let shown = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().map(|info| info.kind)
        })
    };
    assert_eq!(
        shown(&mut cx),
        Some("Switch"),
        "closing the sheet alone already cleared the info: leaving keeps it"
    );
    let dark = cx.update(|_w, cx| Theme::global(cx).mode.is_dark());
    run_menu_item(&mut cx, "Theme", if dark { "Light" } else { "Dark" });
    assert_ne!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        dark,
        "the colour mode did not change, so this proves nothing"
    );
    settle(&mut cx);
    assert_eq!(
        shown(&mut cx),
        None,
        "after the colour mode changed, the info of a switch no longer drawn \
         still shows the colours of the theme before"
    );
}

/// Escape clears a typed query first and closes the palette only on an
/// empty one, as the palette's info says (command/state.rs,
/// `on_action_cancel`).
#[gpui::test]
fn escape_clears_the_query_then_closes_the_palette(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    press(&mut cx, "secondary-k");
    cx.simulate_input("Charts");
    cx.run_until_parked();
    draw(&mut cx);
    press(&mut cx, "escape");
    assert!(
        a_dialog_is_open(&mut cx),
        "Escape on a typed query closed the palette"
    );
    assert_eq!(
        read(&mut cx, &showcase, |this, cx| this
            .palette_state
            .read(cx)
            .query(cx)),
        "",
        "Escape did not clear the query"
    );
    press(&mut cx, "escape");
    assert!(
        !a_dialog_is_open(&mut cx),
        "a second Escape did not close the palette"
    );
}

/// Closing the palette leaves the menus and the key bindings working: the
/// focus goes back where the showcase's handlers reach it.
#[gpui::test]
fn closing_the_palette_keeps_the_actions_working(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    press(&mut cx, "secondary-k");
    assert!(a_dialog_is_open(&mut cx), "Ctrl+K opened nothing");
    press(&mut cx, "escape");
    assert!(
        !a_dialog_is_open(&mut cx),
        "Escape on an empty query did not close the palette"
    );
    run_menu_item_from_focus(&mut cx, "View", "Charts");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Charts,
        "View > Charts did nothing after the palette closed"
    );
    press(&mut cx, "secondary-k");
    assert!(
        a_dialog_is_open(&mut cx),
        "Ctrl+K did nothing after the palette closed"
    );
}

/// The three overlays' menu items and the toolbar's palette button are
/// enabled, now that their actions are handled, and the button opens the
/// palette.
#[gpui::test]
fn the_overlays_are_reachable(cx: &mut TestAppContext) {
    for (menu, item) in [
        ("View", "Command Palette"),
        ("Theme", "Preferences…"),
        ("Help", "About"),
    ] {
        assert!(
            !menu_item_disabled(menu, item),
            "{menu} > {item} is still disabled"
        );
    }
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    click(&mut cx, CHROME_TOOLBAR_PALETTE);
    draw(&mut cx);
    assert!(
        cx.debug_bounds(OVERLAY_PALETTE).is_some(),
        "the toolbar's Command Palette button did not open the palette"
    );
}

/// The palette's Dialog and its Command report themselves (spec §4.3.1,
/// §4.3.5): the Dialog's title is the Dialog's, the Command is the Command's.
#[gpui::test]
fn the_palette_reports_itself(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    press(&mut cx, "secondary-k");
    let cases = [
        ("Dialog · Command Palette", OVERLAY_PALETTE_TITLE),
        ("Command", OVERLAY_PALETTE),
    ];
    for (title, selector) in cases {
        let at = bounds_of(&mut cx, selector).center();
        hover(&mut cx, at);
        settle(&mut cx);
        draw(&mut cx);
        assert_eq!(
            inspector_title(&mut cx, &showcase).as_deref(),
            Some(title),
            "the pointer settled on {selector}, and the inspector does not show the {title}"
        );
    }
}

/// A preference changed in the Preferences sheet reaches the installed
/// theme through `apply_accessibility` (spec §2.8): the Reduce motion switch,
/// clicked, switches gpui's reduced motion on.
#[gpui::test]
fn a_preference_reaches_apply_accessibility(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let unset = native_theme_gpui::AccessibilityPreferences::DEFAULT;
    cx.update(|_window, cx| native_theme_gpui::apply_accessibility(&unset, cx));
    cx.run_until_parked();
    assert!(
        !cx.update(|_window, cx| cx.reduce_motion()),
        "motion is reduced before the switch is clicked, so the click could prove nothing"
    );
    press(&mut cx, "secondary-,");
    assert!(
        cx.debug_bounds(OVERLAY_PREFERENCES).is_some(),
        "Ctrl+, did not open the Preferences sheet"
    );
    click(&mut cx, PREF_REDUCE_MOTION);
    assert!(
        read(&mut cx, &showcase, |_this, cx| {
            cx.native_theme()
                .is_some_and(|nt| nt.accessibility().reduce_motion)
        }),
        "the switch did not install reduce_motion"
    );
    assert!(
        cx.update(|_window, cx| cx.reduce_motion()),
        "the installed preference did not reach gpui's reduced motion"
    );
}

/// The Reduce motion switch off is no request (spec v0.5.9 §5, the Step 4
/// ruling the three showcases share): `forward_reduce_motion(false)` undoes
/// only the connector's own switch and re-reads the system, so an OS request
/// keeps motion reduced whatever the switch shows. The row and the switch's
/// info say so, as the iced and egui showcases' rows do.
#[test]
fn the_reduce_motion_texts_say_that_off_is_no_request() {
    let says = |text: &str| {
        text.contains("off is no request")
            && text.contains("an OS request keeps motion reduced whatever the switch shows")
    };
    assert!(
        says(crate::demo::REDUCE_MOTION_DESCRIPTION),
        "the row: {}",
        crate::demo::REDUCE_MOTION_DESCRIPTION
    );
    let t = Theme::from(&gpui_component::theme::ThemeColor::default());
    for checked in [false, true] {
        let info =
            crate::info::chrome::preference_switch(&t, "reduce_motion", checked, false, None);
        let line = info
            .instance
            .iter()
            .find(|note| note.what == "preference")
            .map(|note| note.text.clone());
        assert!(
            line.as_deref().is_some_and(says),
            "the switch's info, checked {checked}: {line:?}"
        );
    }
}

/// A theme that fails to load is reported by an Alert across the content
/// (spec §2.5), which reports itself: under the page TabBar, which does not
/// move, and above the page.
#[gpui::test]
fn a_theme_error_is_an_alert(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    assert!(
        cx.debug_bounds(CONTENT_ALERT).is_none(),
        "an Alert shows before any theme failed"
    );
    let tabs_before = bounds_of(&mut cx, CHROME_PAGE_TABS);
    use_missing_preset(&mut cx, &showcase, "no-such-preset");
    let alert = bounds_of(&mut cx, CONTENT_ALERT);
    let tabs = bounds_of(&mut cx, CHROME_PAGE_TABS);
    let scroll = bounds_of(&mut cx, CONTENT_SCROLL);
    assert_eq!(
        tabs, tabs_before,
        "the page TabBar moved when the Alert appeared"
    );
    // Under the rule that parts the page TabBar from the page.
    let under = read(&mut cx, &showcase, |this, cx| {
        this.info_ui
            .read(cx)
            .layout_drawn()
            .get("chrome.page_tabs.rule")
            .map(|rule| rule.bottom())
    })
    .unwrap_or(tabs.bottom());
    assert_eq!(
        alert.top(),
        under,
        "the Alert at {alert:?} is not right under the page TabBar at {tabs:?} and its rule"
    );
    assert!(
        alert.bottom() <= scroll.top(),
        "the Alert at {alert:?} is not above the page's scroll area at {scroll:?}"
    );
    hover(&mut cx, alert.center());
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("Alert · Error, banner"),
        "the pointer settled on the Alert, and the inspector does not show it"
    );
    use_preset(&mut cx, &showcase, "kde-breeze");
    assert!(
        cx.debug_bounds(CONTENT_ALERT).is_none(),
        "the Alert stayed after a theme loaded"
    );
}

/// The About dialog draws its name-and-version line on its surface, and its
/// link opens the README's Compatibility table (spec §2.8). What the line
/// and the link read is not checked here: gpui's test context reports where
/// an element was laid out (`VisualTestContext::debug_bounds`), not the text
/// it drew.
#[gpui::test]
fn the_about_dialog_links_to_the_compatibility_table(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    run_menu_item(&mut cx, "Help", "About");
    draw(&mut cx);
    let name = bounds_of(&mut cx, OVERLAY_ABOUT_NAME);
    assert!(
        bounds_of(&mut cx, "dialog-0").contains(&name.origin),
        "the crate's name and version are not on the About dialog"
    );
    click(&mut cx, OVERLAY_ABOUT_LINK);
    let opened = cx.opened_url();
    assert_eq!(
        opened.as_deref(),
        Some(crate::chrome::COMPATIBILITY_URL),
        "the About dialog's link did not open the compatibility table"
    );
}

/// The About dialog's Link is refined as every Link is (T15 fix round 1):
/// under a theme that states `link.underline_enabled = false` (ios-13,
/// material-3) it paints no underline at rest and its info claims none and
/// says `geometry::link` removes it, under kde-breeze it paints one, and
/// its info names `geometry::link`.
#[gpui::test]
fn the_about_dialogs_link_takes_the_themes_underline(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    let mut reached = (false, false);
    for preset in ["ios-13", "kde-breeze"] {
        use_preset(&mut cx, &showcase, preset);
        let underlined = resolved_theme(&mut cx, &showcase).link.underline_enabled;
        if underlined {
            reached.0 = true;
        } else {
            reached.1 = true;
        }
        run_menu_item(&mut cx, "Help", "About");
        draw(&mut cx);
        hover(&mut cx, point(px(4.), px(4.)));
        draw(&mut cx);
        let link = bounds_of(&mut cx, OVERLAY_ABOUT_LINK);
        let painted = cx.update(|window, _| {
            let link = link.scale(window.scale_factor());
            scene_underlines(window)
                .into_iter()
                .filter(|u| u.bounds.intersects(&link))
                .count()
        });
        // One underline per line the text wraps to.
        assert_eq!(
            painted > 0,
            underlined,
            "{preset}: link.underline_enabled is {underlined}, and the About dialog's resting Link paints {painted} underlines"
        );
        let info = settle_on(&mut cx, &showcase, OVERLAY_ABOUT_LINK);
        let text = info.as_ref().map(WidgetInfo::to_text).unwrap_or_default();
        let resting = info.as_ref().is_some_and(|info| {
            info.colors
                .iter()
                .any(|c| c.role.starts_with("resting underline"))
        });
        assert_eq!(
            resting, underlined,
            "{preset}: link.underline_enabled is {underlined}, and the About Link's info claims a resting underline: {text}"
        );
        assert_eq!(
            text.contains(
                "geometry::link removes the resting underline; hover and press still underline"
            ),
            !underlined,
            "{preset}: the About Link's info does not say whether geometry::link removes the underline: {text}"
        );
        assert!(
            text.contains("geometry::link: "),
            "{preset}: the About Link's info names no geometry::link line: {text}"
        );
        cx.update(|window, cx| window.close_dialog(cx));
        cx.run_until_parked();
        draw(&mut cx);
    }
    assert_eq!(
        reached,
        (true, true),
        "the presets no longer leave one Link underlined and one not, so a branch went unread"
    );
}

/// The About dialog follows the installed theme while it is open: its lines
/// are `layout.widget_gap` apart as the theme installed now states it, not
/// as the one it opened under (fix round 1).
#[gpui::test]
fn the_about_dialog_follows_a_theme_switch(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    let gap_between = |cx: &mut VisualTestContext| {
        bounds_of(cx, OVERLAY_ABOUT_TEXT).top() - bounds_of(cx, OVERLAY_ABOUT_NAME).bottom()
    };
    let gap_of = |cx: &mut VisualTestContext, showcase: &Entity<Showcase>| {
        read(cx, showcase, |this, _| geometry::widget_gap(&this.layout))
    };
    use_preset(&mut cx, &showcase, "kde-breeze");
    run_menu_item(&mut cx, "Help", "About");
    draw(&mut cx);
    let breeze = gap_of(&mut cx, &showcase);
    assert_eq!(Some(gap_between(&mut cx)), breeze);
    use_preset(&mut cx, &showcase, "macos-monterey");
    draw(&mut cx);
    let monterey = gap_of(&mut cx, &showcase);
    assert_ne!(
        breeze, monterey,
        "the two presets state the same gap, so the switch proves nothing"
    );
    assert_eq!(
        Some(gap_between(&mut cx)),
        monterey,
        "the open About dialog kept the gap of the theme it opened under"
    );
}

/// The About dialog's title reports the Dialog, as the palette's does.
///
/// Under each preset the pointer is placed on the title itself: adwaita's
/// dialog pads its top by 32px, so a point placed by the dialog's frame
/// lands in padding the title does not cover.
#[gpui::test]
fn the_about_title_reports_the_dialog(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    for preset in ["kde-breeze", "adwaita"] {
        use_preset(&mut cx, &showcase, preset);
        run_menu_item(&mut cx, "Help", "About");
        draw(&mut cx);
        let at = bounds_of(&mut cx, OVERLAY_ABOUT_TITLE).center();
        hover(&mut cx, at);
        settle(&mut cx);
        draw(&mut cx);
        assert_eq!(
            inspector_title(&mut cx, &showcase).as_deref(),
            Some("Dialog · About"),
            "under {preset}, the pointer settled on the About dialog's title, and the inspector does not show it"
        );
        press(&mut cx, "escape");
        draw(&mut cx);
    }
}

/// The About dialog's content is inset from its frame by the dialog's
/// padding (spec §3.6), the reported defect. Under kde-breeze that is
/// `Layout_TopLevelMarginWidth`, 10px (platform-facts §2.22), measured from
/// the frame's inner edge: upstream draws the frame with `border_1`
/// (dialog/dialog.rs:780) and pads each section inside it with the
/// refinement's sides (dialog/dialog.rs:694-706, :815-817).
#[gpui::test]
fn the_about_content_is_inset_by_the_dialogs_padding(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    without_motion(&mut cx);
    let padding = read(&mut cx, &showcase, |_this, cx| {
        cx.native().map(|n| n.resolved.dialog.border.padding)
    });
    let (left, right) = (padding.and_then(|p| p.left), padding.and_then(|p| p.right));
    assert_eq!(
        (left, right),
        (Some(10.0), Some(10.0)),
        "kde-breeze no longer states Breeze's 10px dialog margin, so this measures something else"
    );
    run_menu_item(&mut cx, "Help", "About");
    draw(&mut cx);
    let frame = bounds_of(&mut cx, "dialog-0");
    let name = bounds_of(&mut cx, OVERLAY_ABOUT_NAME);
    let border = px(1.);
    assert_eq!(
        Some(name.left() - frame.left() - border),
        left.map(px),
        "the About dialog's content is not its left padding in from the frame"
    );
    assert_eq!(
        Some(frame.right() - border - name.right()),
        right.map(px),
        "the About dialog's content is not its right padding in from the frame"
    );
}

/// The link's anchor is a heading the README has: `## Compatibility`, which
/// the README's own first lines link to as `#compatibility`.
#[test]
fn the_compatibility_anchor_is_the_readmes() {
    let readme = include_str!("../../README.md");
    assert!(
        readme.lines().any(|l| l == "## Compatibility"),
        "the README has no `## Compatibility` heading"
    );
    assert!(
        readme.contains("(#compatibility)"),
        "the README does not link to its own #compatibility anchor"
    );
    let url = crate::chrome::COMPATIBILITY_URL;
    assert!(
        url.ends_with("/connectors/native-theme-gpui/README.md#compatibility"),
        "{url} is not the connector README's compatibility table"
    );
    assert!(
        url.contains(&format!("/blob/v{}/", env!("CARGO_PKG_VERSION"))),
        "{url} is not the README at this version's tag"
    );
}

#[test]
fn a_widget_info_titles_itself_by_kind_and_variant() {
    let plain = WidgetInfo::new("Tag");
    assert_eq!(plain.title(), "Tag");
    let with = WidgetInfo::new("Tag").variant("Danger, outline");
    assert_eq!(with.title(), "Tag · Danger, outline");
}

#[test]
fn to_text_prints_the_four_sections_in_order() {
    let t = Theme::from(&gpui_component::theme::ThemeColor {
        danger: gpui::hsla(0.0, 1.0, 0.5, 1.0),
        ..Default::default()
    });
    let info = WidgetInfo::new("Tag")
        .variant("Danger")
        .color(claim("bg", "danger", t.danger, "gpui-component/tag.rs:31"))
        .config("border-radius", "radius: 4px")
        .not_themeable("padding", "a rem literal")
        .instance("label", "Danger");
    let text = info.to_text();
    let order: Vec<usize> = [
        "Theme colors:",
        "Theme config:",
        "Not themeable:",
        "This instance:",
    ]
    .iter()
    .map(|h| text.find(h).unwrap_or(usize::MAX))
    .collect();
    assert!(
        order.windows(2).all(|w| w[0] < w[1]),
        "sections out of order:\n{text}"
    );
    assert!(text.starts_with("Tag · Danger\n"), "{text}");
    assert!(
        text.contains("  bg: danger #ff0000 (gpui-component/tag.rs:31)"),
        "{text}"
    );
}

/// An opaque colour prints as `#rrggbb`, and a translucent one carries its
/// alpha as `#rrggbbaa`: a swatch whose alpha was dropped would claim an
/// opaque colour the widget never paints. Opaque is decided on the alpha's
/// byte, as native-theme's `Rgba` prints a theme colour: an alpha from
/// 254.5/255 (about 0.998) up rounds to 255 and prints as `#rrggbb`.
#[test]
fn a_translucent_colour_prints_its_alpha() {
    let hex = |c: gpui::Hsla| as_rgba(c).to_string();
    let red = gpui::hsla(0.0, 1.0, 0.5, 1.0);
    assert_eq!(hex(red), "#ff0000");
    assert_eq!(hex(red.opacity(0.8)), "#ff0000cc");
    assert_eq!(hex(gpui::hsla(0.0, 0.0, 0.0, 0.0)), "#00000000");
    assert_eq!(hex(red.opacity(0.998)), "#ff0000fe");
    for alpha in [0.9981, 0.999, 0.9999] {
        assert_eq!(hex(red.opacity(alpha)), "#ff0000", "alpha {alpha}");
    }
}

/// A Theme Map swatch is noted translucent exactly where its colour text
/// carries an alpha: an alpha that rounds to 255 prints `#rrggbb` and has no
/// note, one that rounds to 254 prints `#rrggbbfe` and has it.
#[test]
fn a_swatch_is_translucent_where_its_text_carries_an_alpha() {
    use crate::info::theme_map::translucent_note;
    let red = gpui::hsla(0.0, 1.0, 0.5, 1.0);
    for (alpha, text, noted) in [
        (1.0, "#ff0000", false),
        (0.9999, "#ff0000", false),
        (0.998, "#ff0000fe", true),
        (0.8, "#ff0000cc", true),
    ] {
        let colour = red.opacity(alpha);
        assert_eq!(as_rgba(colour).to_string(), text, "alpha {alpha}");
        assert_eq!(
            translucent_note(colour).is_some(),
            noted,
            "alpha {alpha}: the note does not follow the printed {text}"
        );
    }
}

#[test]
fn an_empty_section_is_not_printed() {
    let text = WidgetInfo::new("Label").to_text();
    assert_eq!(text, "Label\n");
}

#[test]
fn a_geometry_line_comes_from_the_table() {
    let what = GEOMETRY_NOTES
        .iter()
        .find(|(name, _)| *name == "button")
        .map(|(_, what)| *what);
    assert!(what.is_some(), "GEOMETRY_NOTES has no entry for button");
    let text = WidgetInfo::new("Button").geometry("button").to_text();
    assert_eq!(
        text,
        format!(
            "Button\n\nTheme config:\n  geometry: geometry::button: {}\n",
            what.unwrap_or_default()
        )
    );
}

#[test]
fn a_builder_the_table_lacks_says_so() {
    let text = WidgetInfo::new("Button")
        .geometry("no_such_builder")
        .to_text();
    assert!(
        text.contains("  geometry: geometry::no_such_builder: (no GEOMETRY_NOTES entry)\n"),
        "{text}"
    );
}

/// `native_info` records a builder exactly when it applies it: before a
/// native theme is installed the widget keeps upstream's geometry and its info
/// names no builder; after, the widget takes the builder's refinement and its
/// info carries the builder's line.
#[gpui::test]
fn native_info_records_the_builder_only_when_it_applies_it(cx: &mut TestAppContext) {
    let mut bare = WidgetInfo::new("Button");
    let mut unstyled =
        cx.update(|cx| native_info(gpui::div(), cx, geometry::button, "button", &mut bare));
    assert_eq!(bare, WidgetInfo::new("Button"));
    assert_eq!(unstyled.style().clone(), gpui::StyleRefinement::default());

    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let mut info = WidgetInfo::new("Button");
    let expected = cx.update(|_w, cx| native_geometry(cx, geometry::button));
    let mut styled =
        cx.update(|_w, cx| native_info(gpui::div(), cx, geometry::button, "button", &mut info));
    assert!(expected.is_some(), "no native theme is installed");
    assert_eq!(Some(styled.style().clone()), expected);
    assert_eq!(info, WidgetInfo::new("Button").geometry("button"));
}

/// The Typography page's code Editor takes the platform's line height
/// through `geometry::editor` under a native theme, as `demo::editor`
/// refines it, and its info names the builder (`native_info` records it only
/// where it applies it, which
/// `native_info_records_the_builder_only_when_it_applies_it` holds). Its
/// fill claim is the platform's field fill, which the connector's highlight
/// theme gives the editor.
#[gpui::test]
fn the_code_editor_is_refined_by_geometry_editor(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let r = resolved_theme(&mut cx, &showcase);
    let (line_height, fill) = read(&mut cx, &showcase, |this, cx| {
        let mut info = WidgetInfo::new("Editor");
        let mut editor = native_info(
            gpui_component::input::Editor::new(&this.editor_state),
            cx,
            geometry::editor,
            "editor",
            &mut info,
        );
        let fill = crate::info::typography::editor(Theme::global(cx))
            .colors
            .iter()
            .find(|c| c.field == "editor_background")
            .map(|c| c.value);
        (editor.style().text.line_height, fill)
    });
    assert_eq!(line_height, Some(gpui::relative(r.defaults.line_height)));
    assert_eq!(fill, Some(crate::info::stated(r.input.background_color)));
    show(&mut cx, &showcase, Page::Typography);
    let shown = settle_on(&mut cx, &showcase, "typography-editor");
    assert!(
        shown
            .as_ref()
            .and_then(|info| note_of(&info.config, "geometry"))
            .is_some_and(|line| line.starts_with("geometry::editor")),
        "the page's Editor info does not name geometry::editor: {shown:?}"
    );
}

/// Two targets for the registry's tests, one inside the other: an outer
/// 200×200 target at (20, 20) and, while `show_inner` holds, a 40×40 target
/// centred in it at (120, 120).
struct Nested {
    ui: Entity<InfoRegistry>,
    show_inner: bool,
}

impl Render for Nested {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let inner = self.show_inner.then(|| {
            gpui::div()
                .info(&self.ui, "inner", WidgetInfo::new("Inner"))
                .absolute()
                .left(px(80.))
                .top(px(80.))
                .size(px(40.))
        });
        gpui::div().relative().size_full().child(
            gpui::div()
                .relative()
                .size_full()
                .children(inner)
                .info(&self.ui, "outer", WidgetInfo::new("Outer"))
                .absolute()
                .left(px(20.))
                .top(px(20.))
                .size(px(200.)),
        )
    }
}

/// Draw the frame that shows or hides the inner target.
fn show_inner(cx: &mut VisualTestContext, view: &Entity<Nested>, show: bool) {
    cx.update(|_window, cx| {
        view.update(cx, |this, cx| {
            this.show_inner = show;
            cx.notify();
        })
    });
    draw(cx);
}

const INNER: Point<Pixels> = point(px(120.), px(120.));
const OUTER_ONLY: Point<Pixels> = point(px(30.), px(30.));
const OUTSIDE: Point<Pixels> = point(px(500.), px(500.));

fn open_nested(
    cx: &mut TestAppContext,
) -> (Entity<Nested>, Entity<InfoRegistry>, &mut VisualTestContext) {
    let (view, cx) = cx.add_window_view(|_window, cx| Nested {
        ui: cx.new(|_| InfoRegistry::new()),
        show_inner: true,
    });
    let ui = cx.update(|_window, cx| view.read(cx).ui.clone());
    draw(cx);
    (view, ui, cx)
}

/// Move the pointer to `at` and let the hover handlers run.
fn hover(cx: &mut VisualTestContext, at: Point<Pixels>) {
    cx.simulate_mouse_move(at, None, Modifiers::default());
    cx.run_until_parked();
}

/// Let the current choice stay the choice for `INFO_SETTLE`.
fn settle(cx: &mut VisualTestContext) {
    cx.executor().advance_clock(INFO_SETTLE);
    cx.run_until_parked();
}

/// The title of the info the registry shows.
fn shown(cx: &mut VisualTestContext, ui: &Entity<InfoRegistry>) -> Option<String> {
    cx.update(|_window, cx| ui.read(cx).shown().map(|info| info.title()))
}

#[gpui::test]
fn the_innermost_hovered_target_wins(cx: &mut TestAppContext) {
    let (_view, ui, cx) = open_nested(cx);
    hover(cx, INNER);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
    hover(cx, OUTER_ONLY);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Outer"));
}

#[gpui::test]
fn leaving_every_target_keeps_what_is_shown(cx: &mut TestAppContext) {
    let (_view, ui, cx) = open_nested(cx);
    hover(cx, INNER);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
    hover(cx, OUTSIDE);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
}

#[gpui::test]
fn crossing_is_not_hovering(cx: &mut TestAppContext) {
    let (_view, ui, cx) = open_nested(cx);
    hover(cx, OUTER_ONLY);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Outer"));
    hover(cx, INNER);
    cx.executor().advance_clock(INFO_SETTLE / 2);
    cx.run_until_parked();
    hover(cx, OUTSIDE);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Outer"));
}

#[gpui::test]
fn a_target_no_longer_drawn_never_wins(cx: &mut TestAppContext) {
    let (view, ui, cx) = open_nested(cx);
    hover(cx, INNER);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
    show_inner(cx, &view, false);
    hover(cx, INNER);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Outer"));
}

/// A target that stopped being drawn while hovered gets no hover end from
/// gpui, so drawing it again away from the pointer must not bring it back.
#[gpui::test]
fn a_target_drawn_again_away_from_the_pointer_does_not_win(cx: &mut TestAppContext) {
    let (view, ui, cx) = open_nested(cx);
    hover(cx, INNER);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
    show_inner(cx, &view, false);
    hover(cx, OUTER_ONLY);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Outer"));
    show_inner(cx, &view, true);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Outer"));
}

/// Draw the frame after a page change: the registry is told, and the inner
/// target is shown or hidden, as a new page draws its own targets.
fn change_page(
    cx: &mut VisualTestContext,
    view: &Entity<Nested>,
    ui: &Entity<InfoRegistry>,
    show: bool,
) {
    cx.update(|_window, cx| ui.update(cx, |r, _| r.screen_changed()));
    show_inner(cx, view, show);
}

/// After a page change, an info whose target the new frame did not draw is
/// cleared back to the hint (spec §4.3.4 as ruled), even with the pointer
/// away from every target, where leaving would otherwise keep it (§4.3.3).
#[gpui::test]
fn a_page_change_clears_an_info_no_longer_drawn(cx: &mut TestAppContext) {
    let (view, ui, cx) = open_nested(cx);
    hover(cx, INNER);
    settle(cx);
    hover(cx, OUTSIDE);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
    // No page change: a target that stops being drawn is kept, as leaving it
    // keeps it.
    show_inner(cx, &view, false);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
    change_page(cx, &view, &ui, false);
    settle(cx);
    assert_eq!(shown(cx, &ui), None);
}

/// After a page change, an info whose target is still drawn stays.
#[gpui::test]
fn a_page_change_keeps_an_info_still_drawn(cx: &mut TestAppContext) {
    let (view, ui, cx) = open_nested(cx);
    hover(cx, INNER);
    settle(cx);
    hover(cx, OUTSIDE);
    settle(cx);
    change_page(cx, &view, &ui, true);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Inner"));
}

/// How many times a view rendered: a test reads it to tell a frame that
/// built the view from one that drew it from the last frame.
type Renders = Rc<std::cell::Cell<usize>>;

fn rendered(renders: &Renders) {
    renders.set(renders.get().saturating_add(1));
}

/// A view of its own inside [`Host`]: while `show` holds, a 200×200 target
/// at (20, `top`), `hosted`, that is also an element of the layout dump,
/// `HOSTED_ELEMENT`.
struct Hosted {
    ui: Entity<InfoRegistry>,
    renders: Renders,
    show: bool,
    top: f32,
}

/// Any id serves: only `the_layout_dump_holds_every_element_drawn` holds the
/// recorded ids to the list.
const HOSTED_ELEMENT: &str = "test.hosted";

/// Inside the hosted target at its first place, and inside the host's own
/// where it has one.
const IN_HOSTED: Point<Pixels> = point(px(60.), px(120.));

impl Render for Hosted {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        rendered(&self.renders);
        if !self.show {
            return gpui::div().into_any_element();
        }
        gpui::div()
            .size_full()
            .child(crate::elements::record(&self.ui, HOSTED_ELEMENT))
            .info(&self.ui, "hosted", WidgetInfo::new("Hosted"))
            .absolute()
            .left(px(20.))
            .top(px(self.top))
            .size(px(200.))
            .into_any_element()
    }
}

/// A view of its own inside [`Host`], over part of [`Hosted`]: a 300×200
/// target at (100, 20), `sibling`, larger than the hosted one.
struct Sibling {
    ui: Entity<InfoRegistry>,
    renders: Renders,
}

/// Inside the sibling target, outside the hosted one.
const IN_SIBLING: Point<Pixels> = point(px(350.), px(100.));

/// Inside both the sibling target and the hosted one at its first place.
const IN_BOTH: Point<Pixels> = point(px(150.), px(100.));

impl Render for Sibling {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        rendered(&self.renders);
        gpui::div()
            .info(&self.ui, "sibling", WidgetInfo::new("Sibling"))
            .absolute()
            .left(px(100.))
            .top(px(20.))
            .w(px(300.))
            .h(px(200.))
    }
}

/// The window's root view: it draws [`Hosted`] while `show_hosted` holds,
/// the [`Sibling`] where it has one, and, where `own_target` holds, a
/// 460×460 target of its own at (10, 10), `host`, under both; else it
/// records nothing itself.
struct Host {
    ui: Entity<InfoRegistry>,
    renders: Renders,
    hosted: Entity<Hosted>,
    show_hosted: bool,
    sibling: Option<Entity<Sibling>>,
    own_target: bool,
}

impl Render for Host {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        rendered(&self.renders);
        let own = self.own_target.then(|| {
            gpui::div()
                .info(&self.ui, "host", WidgetInfo::new("Host"))
                .absolute()
                .left(px(10.))
                .top(px(10.))
                .size(px(460.))
        });
        gpui::div()
            .relative()
            .size_full()
            .children(own)
            .children(self.show_hosted.then(|| self.hosted.clone()))
            .children(self.sibling.clone())
    }
}

/// A window whose root is a [`Host`], drawn once; and the host's registry
/// and the render counts of the host, the hosted view and the sibling.
#[allow(clippy::type_complexity)]
fn open_host(
    cx: &mut TestAppContext,
    own_target: bool,
    sibling: bool,
) -> (
    Entity<Host>,
    Entity<InfoRegistry>,
    [Renders; 3],
    &mut VisualTestContext,
) {
    let (host, cx) = cx.add_window_view(move |_window, cx| {
        let ui = cx.new(|_| InfoRegistry::new());
        let hosted = cx.new(|_| Hosted {
            ui: ui.clone(),
            renders: Renders::default(),
            show: true,
            top: 20.,
        });
        let sibling = sibling.then(|| {
            cx.new(|_| Sibling {
                ui: ui.clone(),
                renders: Renders::default(),
            })
        });
        Host {
            ui,
            renders: Renders::default(),
            hosted,
            show_hosted: true,
            sibling,
            own_target,
        }
    });
    let (ui, renders) = cx.update(|_window, cx| {
        let this = host.read(cx);
        let sibling = this
            .sibling
            .as_ref()
            .map(|s| s.read(cx).renders.clone())
            .unwrap_or_default();
        (
            this.ui.clone(),
            [
                this.renders.clone(),
                this.hosted.read(cx).renders.clone(),
                sibling,
            ],
        )
    });
    draw(cx);
    (host, ui, renders, cx)
}

/// Change `view` and notify it alone, then let the window draw its next
/// frame as the app draws one: no `refresh`, so GPUI Fast draws every view
/// nothing changed for from the last frame.
fn notify_alone<V: 'static>(
    cx: &mut VisualTestContext,
    view: &Entity<V>,
    change: impl FnOnce(&mut V),
) {
    cx.update(|_window, cx| {
        view.update(cx, |this, cx| {
            change(this);
            cx.notify();
        })
    });
    cx.run_until_parked();
}

/// What the registry drew twice so far, its targets' ids and its elements'.
fn twice(
    cx: &mut VisualTestContext,
    ui: &Entity<InfoRegistry>,
) -> (
    std::collections::BTreeSet<String>,
    std::collections::BTreeSet<&'static str>,
) {
    cx.update(|_window, cx| {
        let ui = ui.read(cx);
        (ui.drawn_twice.clone(), ui.listed_twice.clone())
    })
}

/// A frame that builds a view again but draws a view nested in it from the
/// last frame, as GPUI Fast draws one (gpui-fast 0.1.12
/// src/fast/retained.rs, `reuse_retained_prepaint`), runs none of the
/// nested view's code: its target records nothing. It is still drawn, so
/// it stays hovered and chosen over the larger target around it, and
/// listed. On gpui-pre the frame builds both views, and the test holds as
/// well. The render counts show which frames the test drew: the host built
/// in the notified frame on both cores, the hosted view drawn from the last
/// frame on GPUI Fast through the settle after it, and built again on
/// gpui-pre.
///
/// The hover and the settle come first, and a frame `draw` draws after
/// them: in it GPUI Fast builds again a view whose hover changed, and one
/// that read the registry since an update of it (gpui-fast
/// src/fast/retained.rs:601-626), as settling the choice is.
#[gpui::test]
fn a_reused_views_target_is_hovered_and_listed_after_a_partial_frame(cx: &mut TestAppContext) {
    let (host, ui, [host_renders, hosted_renders, _], cx) = open_host(cx, true, false);
    hover(cx, IN_HOSTED);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Hosted"));
    draw(cx);
    let (host_before, hosted_before) = (host_renders.get(), hosted_renders.get());
    notify_alone(cx, &host, |_| {});
    assert_eq!(
        host_renders.get(),
        host_before.saturating_add(1),
        "the notify drew no frame that built the host"
    );
    let reused = cfg!(feature = "gpui-fast");
    if reused {
        assert_eq!(
            hosted_renders.get(),
            hosted_before,
            "GPUI Fast built the hosted view again, so the frame is no partial one"
        );
    } else {
        assert!(
            hosted_renders.get() > hosted_before,
            "gpui-pre drew the hosted view from the last frame, so the frame is no full one"
        );
    }
    settle(cx);
    assert_eq!(
        shown(cx, &ui).as_deref(),
        Some("Hosted"),
        "the hosted target, drawn from the last frame, is no longer chosen"
    );
    if reused {
        assert_eq!(
            hosted_renders.get(),
            hosted_before,
            "a frame drawn on the settle built the hosted view again, so its target \
             recorded afresh"
        );
    }
    let listed = cx.update(|_window, cx| ui.read(cx).layout_drawn());
    assert!(
        listed.contains_key(HOSTED_ELEMENT),
        "the layout dump lost the hosted view's element: {listed:?}"
    );
    let (targets, elements) = twice(cx, &ui);
    assert!(targets.is_empty(), "drawn twice in one frame: {targets:?}");
    assert!(
        elements.is_empty(),
        "listed twice in one frame: {elements:?}"
    );
}

/// The other way round: a frame that builds a view again without its
/// target, and draws the view beside it from the last frame, takes the
/// target and its element of the list away, though nothing records: the
/// host around them, which this frame builds again on GPUI Fast too,
/// records nothing itself. The element leaves the layout dump, and the
/// target, smaller than the sibling, is never chosen again: with the
/// pointer on the sibling, the sibling is shown. A frame `draw` draws
/// follows the settle, as in
/// `a_reused_views_target_is_hovered_and_listed_after_a_partial_frame`.
#[gpui::test]
fn a_target_a_partial_frame_no_longer_draws_is_gone(cx: &mut TestAppContext) {
    let (host, ui, [host_renders, hosted_renders, sibling_renders], cx) =
        open_host(cx, false, true);
    hover(cx, IN_HOSTED);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Hosted"));
    draw(cx);
    let before = [&host_renders, &hosted_renders, &sibling_renders].map(|r| r.get());
    let hosted = cx.update(|_window, cx| host.read(cx).hosted.clone());
    notify_alone(cx, &hosted, |this| this.show = false);
    let after = [&host_renders, &hosted_renders, &sibling_renders].map(|r| r.get());
    assert_eq!(
        after[1],
        before[1].saturating_add(1),
        "the notify drew no frame that built the hosted view"
    );
    assert_eq!(
        after[0],
        before[0].saturating_add(1),
        "the frame did not build the host once, as both cores do here"
    );
    if cfg!(feature = "gpui-fast") {
        assert_eq!(
            after[2], before[2],
            "GPUI Fast built the sibling again, so the frame recorded"
        );
    }
    let listed = cx.update(|_window, cx| ui.read(cx).layout_drawn());
    assert!(
        !listed.contains_key(HOSTED_ELEMENT),
        "the layout dump still holds the hosted view's element: {listed:?}"
    );
    hover(cx, IN_SIBLING);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Sibling"));
    let (targets, elements) = twice(cx, &ui);
    assert!(targets.is_empty(), "drawn twice in one frame: {targets:?}");
    assert!(
        elements.is_empty(),
        "listed twice in one frame: {elements:?}"
    );
}

/// A hovered target that a frame recording nothing took away gets no hover
/// end from gpui; drawn again away from the pointer, as a new element that
/// starts from not hovered (gpui-pre elements/div.rs:3213-3222), it must
/// not win over the larger target the pointer is still on, which the
/// pointer never left, so no hover event comes between. On GPUI Fast the
/// frame that takes it away builds the host only, which records nothing
/// itself, and draws the sibling from the last frame; on gpui-pre the
/// sibling records in it, which already prunes the hover. A full frame,
/// the test's own `window.refresh()` before `draw`, follows the first
/// settle: after a frame `draw` alone draws there, GPUI Fast builds the
/// sibling again in the frame that takes the target away, and the sibling
/// records in it.
#[gpui::test]
fn a_target_a_recordless_frame_took_away_does_not_win_when_drawn_again(cx: &mut TestAppContext) {
    let (host, ui, [host_renders, hosted_renders, sibling_renders], cx) =
        open_host(cx, false, true);
    hover(cx, IN_BOTH);
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Hosted"));
    cx.update(|window, _| window.refresh());
    draw(cx);
    let (host_before, sibling_before) = (host_renders.get(), sibling_renders.get());
    let hosted = cx.update(|_window, cx| host.read(cx).hosted.clone());
    // Out of the pointer's way once drawn again; no notify of its own.
    cx.update(|_window, cx| hosted.update(cx, |this, _| this.top = 240.));
    notify_alone(cx, &host, |this| this.show_hosted = false);
    assert_eq!(
        host_renders.get(),
        host_before.saturating_add(1),
        "the notify drew no frame that built the host"
    );
    if cfg!(feature = "gpui-fast") {
        assert_eq!(
            sibling_renders.get(),
            sibling_before,
            "GPUI Fast built the sibling again, so the frame recorded"
        );
    }
    let hosted_before = hosted_renders.get();
    notify_alone(cx, &host, |this| this.show_hosted = true);
    assert!(
        hosted_renders.get() > hosted_before,
        "the hosted view was drawn again from an earlier frame, not as a new element"
    );
    settle(cx);
    assert_eq!(shown(cx, &ui).as_deref(), Some("Sibling"));
}

/// Two sibling targets under one id, each holding an element of the list
/// under one id, as a duplicate id in the showcase would draw them.
struct Twins {
    ui: Entity<InfoRegistry>,
}

impl Render for Twins {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let twin = |left: f32| {
            gpui::div()
                .size_full()
                .child(crate::elements::record(&self.ui, "test.twin"))
                .info(&self.ui, "twin", WidgetInfo::new("Twin"))
                .absolute()
                .left(px(left))
                .top(px(20.))
                .size(px(100.))
        };
        gpui::div()
            .relative()
            .size_full()
            .child(twin(20.))
            .child(twin(200.))
    }
}

/// The tests' check of two targets under one id finds two drawn in the same
/// draw, and two elements of the list likewise.
#[gpui::test]
fn the_duplicate_check_finds_two_targets_drawn_together(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(|_window, cx| Twins {
        ui: cx.new(|_| InfoRegistry::new()),
    });
    let ui = cx.update(|_window, cx| view.read(cx).ui.clone());
    draw(cx);
    let (targets, elements) = twice(cx, &ui);
    assert!(
        targets.contains("twin"),
        "two targets under `twin` not found: {targets:?}"
    );
    assert!(
        elements.contains("test.twin"),
        "two elements under `test.twin` not found: {elements:?}"
    );
}

/// A cached view of its own, holding a target under the id the view around
/// it also draws one under once `fresh` holds; it counts its renders.
struct Cached {
    ui: Entity<InfoRegistry>,
    renders: Renders,
}

impl Render for Cached {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        rendered(&self.renders);
        gpui::div()
            .info(&self.ui, "pair", WidgetInfo::new("Cached"))
            .absolute()
            .left(px(20.))
            .top(px(20.))
            .size(px(100.))
    }
}

/// Draws [`Cached`] as a cached view, which both cores draw from the last
/// frame while nothing changed for it (gpui-pre src/view.rs, `cached`), and,
/// once `fresh` holds, a target of its own under the same id.
struct Pair {
    ui: Entity<InfoRegistry>,
    cached: Entity<Cached>,
    fresh: bool,
}

impl Render for Pair {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let fresh = self.fresh.then(|| {
            gpui::div()
                .info(&self.ui, "pair", WidgetInfo::new("Fresh"))
                .absolute()
                .left(px(200.))
                .top(px(20.))
                .size(px(100.))
        });
        gpui::div()
            .relative()
            .size_full()
            .child(
                self.cached
                    .clone()
                    .cached(gpui::StyleRefinement::default().size_full()),
            )
            .children(fresh)
    }
}

/// The tests' check of two targets under one id finds a target recorded
/// beside one under the same id that the frame drew from the last frame,
/// which recorded nothing: the frame builds the view around it and draws
/// the cached view from the last frame, on both cores.
#[gpui::test]
fn the_duplicate_check_finds_a_fresh_target_beside_a_reused_one(cx: &mut TestAppContext) {
    let renders = Renders::default();
    let counted = renders.clone();
    let (view, cx) = cx.add_window_view(move |_window, cx| {
        let ui = cx.new(|_| InfoRegistry::new());
        let cached = cx.new(|_| Cached {
            ui: ui.clone(),
            renders: counted,
        });
        Pair {
            ui,
            cached,
            fresh: false,
        }
    });
    let ui = cx.update(|_window, cx| view.read(cx).ui.clone());
    draw(cx);
    let before = renders.get();
    notify_alone(cx, &view, |this| this.fresh = true);
    assert_eq!(
        renders.get(),
        before,
        "the cached view was built again, so its target recorded beside the fresh one"
    );
    let (targets, _) = twice(cx, &ui);
    assert!(
        targets.contains("pair"),
        "two targets under `pair` not found: {targets:?}"
    );
}

/// No code of the showcase but its tests clones gpui-component's `Theme`:
/// a frame reads the fields it needs by reference (M9-020). The Showcase and
/// the inspector render on every frame the window draws, and the Basic
/// page's Spinner asks for one every frame, so a clone there copied the
/// whole theme, its configs and colour tables included, each frame. The
/// gate prints each clone it found.
///
/// It reads two spellings, not types: `theme().clone()` and
/// `Theme::global(cx).clone()`, each on one line. A clone through a binding
/// (`let t = cx.theme();` and later `t.clone()`) or through `Theme::clone(..)`
/// passes it.
#[test]
fn no_code_clones_the_gpui_theme() {
    let found: Vec<String> = showcase_sources()
        .into_iter()
        .filter(|(name, _)| name != "tests.rs")
        .flat_map(|(name, source)| {
            code_only(&source)
                .lines()
                .enumerate()
                .filter(|(_, line)| {
                    let line: String = line.split_whitespace().collect();
                    line.contains("theme().clone()") || line.contains("Theme::global(cx).clone()")
                })
                .map(|(ix, line)| format!("{name}:{}: {}", ix + 1, line.trim()))
                .collect::<Vec<_>>()
        })
        .collect();
    for clone in &found {
        println!("clones the gpui Theme: {clone}");
    }
    assert!(
        found.is_empty(),
        "the showcase clones gpui-component's Theme where it can read its fields by reference: {found:?}"
    );
}

// ---------------------------------------------------------------------------
// The listeners: a state a listener writes is drawn on the next frame
// ---------------------------------------------------------------------------

/// Every `.rs` file of the showcase example, as (path under
/// `examples/showcase-gpui/` written with `/`, contents), sorted by path:
/// what `every_listener_that_writes_state_notifies` reads, so a file added
/// to the example is read with no list to update.
fn showcase_sources() -> Vec<(String, String)> {
    fn walk(root: &std::path::Path, dir: &std::path::Path, out: &mut Vec<(String, String)>) {
        let entries = std::fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("{} cannot be read: {e}", dir.display()));
        for entry in entries {
            let path = entry
                .unwrap_or_else(|e| panic!("{} cannot be read: {e}", dir.display()))
                .path();
            if path.is_dir() {
                walk(root, &path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let source = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("{} cannot be read: {e}", path.display()));
                let relative = path.strip_prefix(root).unwrap_or(&path);
                let name = relative
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("/");
                out.push((name, source));
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/showcase-gpui");
    let mut sources = Vec::new();
    walk(&root, &root, &mut sources);
    sources.sort();
    sources
}

/// `source` with its line comments and the insides of its string and char
/// literals blanked, its line breaks kept, so a bracket in a text is not
/// counted and a line number in the result is one in `source`.
fn code_only(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '/' if chars.peek() == Some(&'/') => {
                while chars.next_if(|next| *next != '\n').is_some() {}
            }
            '"' => {
                out.push('"');
                while let Some(inner) = chars.next() {
                    match inner {
                        '\\' => {
                            if chars.next() == Some('\n') {
                                out.push('\n');
                            }
                        }
                        '"' => break,
                        '\n' => out.push('\n'),
                        _ => {}
                    }
                }
                out.push('"');
            }
            // A char literal, not a lifetime: `'x'` or an escape `'\n'`.
            '\'' if chars.peek() == Some(&'\\') || chars.clone().nth(1) == Some('\'') => {
                out.push_str("' '");
                if chars.next() == Some('\\') {
                    chars.next();
                }
                while chars.next_if(|next| *next != '\'').is_some() {}
                chars.next();
            }
            _ => out.push(c),
        }
    }
    out
}

/// The fields a closure body writes, as `this.<field> =` or a compound
/// `this.<field> += …`, each with its byte offset in `body`.
fn fields_written(body: &str) -> Vec<(usize, &str)> {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut written = Vec::new();
    for (at, _) in body.match_indices("this.") {
        if body[..at].chars().next_back().is_some_and(ident) {
            continue;
        }
        let rest = &body[at + "this.".len()..];
        let path_len = rest
            .find(|c: char| !ident(c) && c != '.')
            .unwrap_or(rest.len());
        let after = rest[path_len..].trim_start();
        let after = ["<<", ">>", "+", "-", "*", "/", "%", "|", "&", "^"]
            .iter()
            .find_map(|op| after.strip_prefix(op))
            .unwrap_or(after);
        if path_len > 0 && after.starts_with('=') && !after.starts_with("==") {
            written.push((at, &rest[..path_len]));
        }
    }
    written
}

/// Every `cx.listener(` closure in the showcase example that writes a field
/// of its view calls `cx.notify()` (M9-017).
///
/// `cx.listener` only updates the entity (gpui-pre 0.3.10
/// `src/app/context.rs:246-256`). A mouse click is followed by a refresh of
/// the window (`src/elements/div.rs:3145-3148`); a keyboard click calls the
/// same listeners and refreshes nothing (`:3099-3130`), so a control
/// activated from the keyboard by a listener that does not notify stays
/// drawn as it was until something else redraws the window. gpui-component
/// documents these values as controlled: the owner writes the value and calls
/// `cx.notify()` (gpui-component 0.8.1 `src/checkbox.rs:106-110`).
#[test]
fn every_listener_that_writes_state_notifies() {
    let sources = showcase_sources();
    assert!(
        sources.iter().any(|(path, _)| path == "main.rs")
            && sources.iter().any(|(path, _)| path.starts_with("pages/")),
        "the showcase's files were not found: {:?}",
        sources.iter().map(|(path, _)| path).collect::<Vec<_>>()
    );

    let mut listeners = 0;
    let mut offenders = Vec::new();
    for (path, source) in &sources {
        let code = code_only(source);
        for (at, call) in code.match_indices("cx.listener(") {
            let open = at + call.len() - 1;
            let mut depth = 0usize;
            let close = code[open..].char_indices().find_map(|(ix, c)| {
                match c {
                    '(' => depth += 1,
                    ')' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            return Some(open + ix);
                        }
                    }
                    _ => {}
                }
                None
            });
            let Some(close) = close else {
                panic!("{path}: the cx.listener( at byte {at} never closes");
            };
            listeners += 1;
            let body = &code[open..close];
            if body.contains("cx.notify()") {
                continue;
            }
            for (offset, field) in fields_written(body) {
                let line = code[..open + offset].matches('\n').count() + 1;
                offenders.push(format!("{path}:{line}: this.{field} ="));
            }
        }
    }
    assert!(
        listeners > 0,
        "no cx.listener( was found, so nothing was checked"
    );
    assert!(
        offenders.is_empty(),
        "these listeners write the showcase's state and do not call cx.notify(), \
         so a keyboard activation is not drawn:\n{}",
        offenders.join("\n")
    );
}

/// Space on the Inputs page's first checkbox notifies the showcase, which is
/// what asks for the frame that draws it unchecked (M9-017).
///
/// Focus moves to the checkbox by Tab from the number field before it, and
/// Space activates it as a keyboard click: gpui calls the listener and
/// refreshes nothing (gpui-pre 0.3.10 `src/elements/div.rs:3099-3130`), so
/// the window is marked dirty, and a frame drawn, only if the listener calls
/// `cx.notify()`. That missing frame request is the bug, and it is what this
/// counts, through an observer of the showcase, which runs on a notify
/// alone. A drawn frame cannot show it: a test draws its frames itself, dirty
/// window or not, and on GPUI Fast a view that is not cached is drawn again
/// whenever an entity it read was written, notified or not (gpui-fast 0.1.12
/// `src/fast/retained.rs:601-623`, `src/fast/dependencies.rs:513-527`), so
/// the frame after Space shows the new state on both backends either way.
#[gpui::test]
fn a_checkbox_toggled_from_the_keyboard_notifies_the_showcase(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    // A bundled icon set: no icon loads in the background and notifies the
    // showcase, which would count for a listener that does not.
    use_preset(&mut cx, &showcase, "material-3");
    show(&mut cx, &showcase, Page::Inputs);
    let field = read(&mut cx, &showcase, |this, cx| {
        this.number_input_state.read(cx).focus_handle(cx)
    });
    cx.update(|window, cx| {
        window.focus(&field, cx);
        window.focus_next(cx);
    });
    draw(&mut cx);
    assert!(
        read(&mut cx, &showcase, |this, _| this.checkbox_a),
        "the checkbox does not start checked"
    );
    let notified = Rc::new(std::cell::Cell::new(0usize));
    let count = notified.clone();
    let _observer =
        cx.update(|_window, cx| cx.observe(&showcase, move |_, _| count.set(count.get() + 1)));
    let space = gpui::Keystroke::parse("space").expect("space is a keystroke");
    cx.simulate_event(gpui::KeyDownEvent {
        keystroke: space.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(gpui::KeyUpEvent { keystroke: space });
    cx.run_until_parked();
    assert!(
        !read(&mut cx, &showcase, |this, _| this.checkbox_a),
        "Space did not reach the checkbox: Tab from the number field focused something else"
    );
    assert!(
        notified.get() > 0,
        "Space unchecked the checkbox without notifying the showcase, so no frame is asked for \
         and the checkbox stays drawn checked"
    );
}

// ---------------------------------------------------------------------------
// docs/showcase-elements.toml: the layout dump and Widget Info
// ---------------------------------------------------------------------------

/// The elements of docs/showcase-elements.toml the showcase draws but does
/// not record while the frame is laid out, as (element id, why): each is
/// drawn inside a gpui-component widget that reports no bounds of its parts,
/// or its bounds come from the field's state once laid out
/// (`Showcase::field_text_bounds`, which the layout dump reads).
#[cfg(feature = "widgets")]
const NOT_RECORDED: &[(&str, &str)] = &[
    (
        "basic.text_inputs.placeholder.text",
        "the Input's element draws the placeholder and reports no bounds of it (input/input.rs)",
    ),
    (
        "basic.text_inputs.filled.text",
        "Showcase::field_text_bounds",
    ),
    (
        "basic.text_inputs.disabled.text",
        "Showcase::field_text_bounds",
    ),
    (
        "basic.text_inputs.focused.text",
        "Showcase::field_text_bounds",
    ),
    (
        "basic.number_input.field.text",
        "Showcase::field_text_bounds",
    ),
    ("basic.text_area.field.text", "Showcase::field_text_bounds"),
    (
        "basic.drop_down.trigger.text",
        "the Select draws its value inside its trigger and reports no bounds of it (select.rs)",
    ),
    (
        "basic.drop_down.trigger.arrow",
        "the Select draws its arrow inside its trigger and reports no bounds of it (select.rs)",
    ),
];

/// Every id the showcase records is an element of the list, none is
/// recorded twice in one frame, and every element of the list the Basic
/// page and the chrome show at rest is recorded, but for [`NOT_RECORDED`].
/// The parts are recorded by the theme-drawn controls of the `widgets`
/// feature (`elements::PARTS`), so the test is that feature's.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_layout_dump_holds_every_element_drawn(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    // A bundled icon set, so every icon element is drawn on any machine:
    // kde-breeze's own, Breeze, need not be installed (a missing icon is
    // drawn as nothing, never as another set's).
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set(&IconSetChoice::Material.to_string(), window, cx);
        })
    });
    cx.run_until_parked();
    draw(&mut cx);
    let (drawn, twice) = read(&mut cx, &showcase, |this, cx| {
        let ui = this.info_ui.read(cx);
        (ui.layout_drawn(), ui.listed_twice.clone())
    });
    assert!(twice.is_empty(), "recorded twice in one frame: {twice:?}");
    let list = crate::elements::elements();
    for id in drawn.keys() {
        assert!(
            list.iter().any(|e| e.id == *id),
            "{id} is recorded but is no element of docs/showcase-elements.toml"
        );
    }
    // Shown only under a condition: an open menu, a hover, a scrollbar at
    // rest -- which the gpui List draws inside itself (list/list.rs). A
    // build with no icon set draws no icon.
    let icons = cfg!(any(
        feature = "material-icons",
        feature = "lucide-icons",
        feature = "system-icons"
    ));
    let missing: Vec<&str> = list
        .iter()
        .filter(|e| e.when.is_none())
        .map(|e| e.id.as_str())
        .filter(|id| icons || !id.ends_with(".icon"))
        .filter(|id| !drawn.contains_key(id))
        .filter(|id| !NOT_RECORDED.iter().any(|(n, _)| n == id))
        .collect();
    assert!(missing.is_empty(), "drawn but not recorded: {missing:?}");
    for (id, _) in NOT_RECORDED {
        assert!(
            list.iter().any(|e| e.id == *id),
            "NOT_RECORDED names {id}, no element of the list"
        );
    }
}

/// M9x CI item I2 (M9-170): each Basic list row's label is recorded for the
/// layout dump as its text's box, centred down its row, as the iced and egui
/// showcases record theirs: under windows-11 (rows `list.row_height` 40, no
/// vertical padding), `basic.list.row_N.text` is narrower than its row and
/// centred in it to half a pixel.
#[gpui::test]
fn the_basic_list_labels_are_recorded_centred_down_their_rows(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "windows-11");
    draw(&mut cx);
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    let mut faults = Vec::new();
    for n in 1..=3 {
        let row_id = format!("basic.list.row_{n}");
        let text_id = format!("{row_id}.text");
        let (Some(row), Some(label)) = (
            drawn.get(row_id.as_str()).copied(),
            drawn.get(text_id.as_str()).copied(),
        ) else {
            faults.push(format!("{row_id} or its label is not recorded"));
            continue;
        };
        if label.size.width >= row.size.width {
            faults.push(format!(
                "{text_id} {label:?} is not its text's box in {row:?}"
            ));
        }
        let off = (label.center().y - row.center().y).abs();
        if off > px(0.5) {
            faults.push(format!(
                "{text_id} {label:?} is {off:?} off the centre of {row:?}"
            ));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// The side panel's settings are padded by `layout.container_margin` where
/// the theme states no `sidebar.border.padding`, and by the stated sides of
/// it where it states them, the content then adding none.
#[gpui::test]
fn the_side_panel_takes_the_sidebars_padding(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let place = |cx: &mut VisualTestContext| {
        let drawn = read(cx, &showcase, |this, cx| {
            this.info_ui.read(cx).layout_drawn()
        });
        let panel = drawn.get("chrome.side_panel").copied();
        let settings = drawn.get("chrome.side_panel.settings").copied();
        panel
            .zip(settings)
            .map(|(p, s)| (s.left() - p.left(), s.top() - p.top()))
    };
    let margin = read(&mut cx, &showcase, |this, _| {
        geometry::container_margin(&this.layout)
    });
    assert!(margin.is_some(), "kde-breeze states no container_margin");
    assert_eq!(
        place(&mut cx),
        margin.map(|m| (m, m)),
        "unstated sidebar padding: the settings are not container_margin in"
    );
    let resolved = native_theme::theme::Theme::preset("kde-breeze")
        .ok()
        .and_then(|t| t.into_variant(native_theme::theme::ColorMode::Light).ok())
        .and_then(|v| {
            v.into_resolved(&native_theme::ResolutionContext::for_tests())
                .ok()
        });
    assert!(resolved.is_some(), "kde-breeze light does not resolve");
    let Some(mut resolved) = resolved else { return };
    resolved.sidebar.border.padding.top = Some(3.0);
    resolved.sidebar.border.padding.left = Some(7.0);
    cx.update(|_window, cx| {
        let prefs = native_theme::AccessibilityPreferences::default();
        native_theme_gpui::apply(
            native_theme_gpui::to_theme(&resolved, "kde-breeze", false, &prefs),
            &resolved,
            &prefs,
            cx,
        );
    });
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        place(&mut cx),
        Some((px(7.), px(3.))),
        "stated sidebar padding: the settings are not that padding in"
    );
}

/// The side panel's info says what pads it (M9-011): with no
/// `sidebar.border.padding` stated, `layout.container_margin` around the
/// theme settings, its geometry line recorded; with every side stated, each
/// side by `sidebar.border`'s value and no `container_margin` line, the
/// margin padding nothing; with some sides stated, those by `sidebar.border`
/// and the rest by `layout.container_margin`, whose line is then recorded.
/// The paddings are spliced into kde-breeze, which states none, and adwaita
/// states all four sides itself.
#[gpui::test]
fn the_side_panels_info_names_the_padding_it_draws(cx: &mut TestAppContext) {
    use crate::info::px_text;
    use native_theme::theme::ResolvedPadding;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let all = ResolvedPadding {
        top: Some(3.0),
        right: Some(5.0),
        bottom: Some(4.0),
        left: Some(7.0),
    };
    let some = ResolvedPadding {
        top: Some(3.0),
        left: Some(7.0),
        ..Default::default()
    };
    let mut bundled_all = false;
    for (preset, splice) in [
        ("kde-breeze", Some(ResolvedPadding::default())),
        ("kde-breeze", Some(all)),
        ("kde-breeze", Some(some)),
        ("adwaita", None),
    ] {
        use_preset(&mut cx, &showcase, preset);
        let margin = read(&mut cx, &showcase, |this, _| {
            geometry::container_margin(&this.layout)
        });
        let Some(margin) = margin else {
            panic!("{preset} states no container_margin");
        };
        if let Some(padding) = splice {
            let mut theme = resolved_theme(&mut cx, &showcase);
            assert_eq!(
                theme.sidebar.border.padding,
                ResolvedPadding::default(),
                "{preset} now states a sidebar padding; the spliced cases need another preset"
            );
            theme.sidebar.border.padding = padding;
            cx.update(|_window, cx| {
                let prefs = native_theme::AccessibilityPreferences::default();
                native_theme_gpui::apply(
                    native_theme_gpui::to_theme(&theme, preset, false, &prefs),
                    &theme,
                    &prefs,
                    cx,
                );
            });
            cx.run_until_parked();
            draw(&mut cx);
        }
        let padding = resolved_theme(&mut cx, &showcase).sidebar.border.padding;
        let at = format!("{preset}, sidebar.border.padding {padding:?}");
        let info = settle_on(&mut cx, &showcase, CHROME_SIDE_PANEL);
        let Some(info) = info else {
            panic!("{at}: the side panel shows no info");
        };
        let sides = [
            ("top", padding.top),
            ("right", padding.right),
            ("bottom", padding.bottom),
            ("left", padding.left),
        ];
        let stated = sides.iter().any(|(_, v)| v.is_some());
        let unstated = sides.iter().any(|(_, v)| v.is_none());
        bundled_all |= splice.is_none() && !unstated;
        assert_eq!(
            names_the_container_margin(&info),
            unstated,
            "{at}: the side panel's info {} geometry::container_margin, which pads {} side: \
             {info:?}",
            if unstated {
                "does not record"
            } else {
                "records"
            },
            if unstated { "an unstated" } else { "no" },
        );
        let line = note_of(&info.instance, "padding").unwrap_or_default();
        let wants: Vec<String> = if stated {
            sides
                .iter()
                .map(|(side, v)| match v {
                    Some(v) => format!("{side} {}px, sidebar.border's", px_text(*v)),
                    None => format!(
                        "{side} {}px, layout.container_margin",
                        px_text(margin.as_f32())
                    ),
                })
                .collect()
        } else {
            vec![format!(
                "{}px around the theme settings, layout.container_margin",
                px_text(margin.as_f32())
            )]
        };
        for want in wants {
            assert!(
                line.contains(&want),
                "{at}: the side panel's padding line {line:?} does not say {want:?}"
            );
        }
    }
    assert!(
        bundled_all,
        "adwaita no longer states every side of sidebar.border.padding, the bundled case of a \
         side panel the margin pads nothing of"
    );
}

/// The side panel's Widget Info routes each `sidebar.border.padding_*` leaf
/// as `demo::side_panel` draws it (Addendum I S2): under windows-11, which
/// states the sidebar's left and right padding alone, the panel's top is
/// padded by `layout.container_margin`, and the route the inspector shows
/// and copies for `sidebar.border.padding_top_px` says that a side the
/// padding leaves unstated takes the margin, not that the margin pads the
/// content, which the panel then leaves unpadded.
#[gpui::test]
fn the_side_panels_padding_routes_name_the_margin_on_an_unstated_side(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "windows-11");
    let p = resolved_theme(&mut cx, &showcase).sidebar.border.padding;
    assert!(
        p.top.is_none() && p.left.is_some(),
        "windows-11 no longer states its sidebar's left padding without its top: {p:?}"
    );
    let info = settle_on(&mut cx, &showcase, CHROME_SIDE_PANEL);
    assert!(
        info.as_ref().is_some_and(|info| info.listed.is_some()),
        "the side panel's info is not an element of the list: {info:?}"
    );
    click(&mut cx, INSPECTOR_COPY);
    let copied = cx
        .read_from_clipboard()
        .and_then(|item| item.text())
        .unwrap_or_default();
    let route = copied
        .lines()
        .skip_while(|line| !line.starts_with("sidebar.border.padding_top_px "))
        .nth(1)
        .unwrap_or_default();
    assert!(
        route.contains("a side it leaves unstated by layout.container_margin"),
        "windows-11: the side panel's sidebar.border.padding_top_px route {route:?} does not \
         say the margin pads the unstated top: {copied:?}"
    );
}

/// The side panel's and the page tabs' `layout.container_margin_px` routes
/// say that the margin pads nothing where the theme states none, as under
/// macos-tahoe, whose layout states no `container_margin`: each names a
/// padding the margin gives only where it is stated.
#[test]
fn the_container_margin_routes_say_none_where_it_is_unstated() {
    for element in [
        "chrome.side_panel",
        "chrome.side_panel.settings",
        "chrome.side_panel.inspector_tabs",
        "chrome.side_panel.inspector",
        "chrome.page_tabs",
    ] {
        let route = crate::info::leaves::how(element, "layout.container_margin_px");
        assert!(
            route.is_some_and(|r| r.contains("(none where that is unstated)")),
            "{element}: its layout.container_margin_px route {route:?} does not say the margin \
             pads nothing where the theme states none"
        );
    }
}

/// The segmented control's padding line says what pads each side of a
/// segment (M9-048): a stated `segmented_control.border.padding` side by its
/// value, an unstated left or right by `SEGMENT_PADDING`, upstream's 12px,
/// and an unstated top or bottom by nothing -- under material-3, which states
/// only the horizontal sides, in both modes, and with its left side taken
/// out.
#[test]
fn the_segmented_controls_padding_line_names_each_side() {
    use crate::info::px_text;
    let mut cases = (false, false);
    for (name, r, t) in every_bundled_variant()
        .into_iter()
        .filter(|(name, _, _)| name.starts_with("material-3 "))
    {
        let mut spliced = r.clone();
        spliced.segmented_control.border.padding.left = None;
        for (case, r) in [("as stated", r), ("left unstated", spliced)] {
            let info = crate::info::layout::segmented(&t, &["One", "Two"], 0, Some(&r));
            let line = note_of(&info.config, "padding").unwrap_or_default();
            let p = &r.segmented_control.border.padding;
            for (side, value, across) in [
                ("top", p.top, false),
                ("right", p.right, true),
                ("bottom", p.bottom, false),
                ("left", p.left, true),
            ] {
                let want = match (value, across) {
                    (Some(v), _) => format!("{side} {}px", px_text(v)),
                    (None, true) => {
                        cases.0 = true;
                        format!(
                            "{side} {}px, unstated",
                            px_text(crate::demo::SEGMENT_PADDING.as_f32())
                        )
                    }
                    (None, false) => {
                        cases.1 = true;
                        format!("{side} none, unstated")
                    }
                };
                assert!(
                    line.contains(&want),
                    "{name} ({case}): the segmented control's padding line {line:?} does not \
                     say {want:?}"
                );
            }
        }
    }
    assert_eq!(
        cases,
        (true, true),
        "material-3 no longer covers an unstated left or right and an unstated top or bottom"
    );
}

/// `--open-menu theme` opens the Theme menu, so the layout dump holds its
/// popup and every row the list shows while it is open; a menu of another
/// name is refused, naming the menus.
#[gpui::test]
fn the_open_theme_menu_is_dumped_with_its_rows(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let refused = cx.update(|window, cx| {
        let menus = showcase.read(cx).menus.clone();
        menus.update(cx, |m, cx| m.open_named("nonesuch", false, window, cx))
    });
    assert!(
        refused.as_ref().is_err_and(|e| e.contains("theme")),
        "an unknown menu is not refused with the menus named: {refused:?}"
    );
    let opened = cx.update(|window, cx| {
        let menus = showcase.read(cx).menus.clone();
        menus.update(cx, |m, cx| m.open_named("theme", false, window, cx))
    });
    assert!(opened.is_ok(), "the Theme menu does not open: {opened:?}");
    cx.run_until_parked();
    draw(&mut cx);
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    let missing: Vec<&str> = crate::elements::elements()
        .iter()
        .filter(|e| e.id.starts_with("chrome.menu.theme"))
        .map(|e| e.id.as_str())
        .filter(|id| !drawn.contains_key(id))
        .collect();
    assert!(
        missing.is_empty(),
        "the open Theme menu's dump lacks {missing:?}"
    );
}

/// Every leaf of every element of the list has the line saying how gpui
/// applies it (`info::leaves`), so Widget Info never shows the fallback.
#[test]
fn every_listed_leaf_has_a_route() {
    let missing: Vec<String> = crate::elements::elements()
        .iter()
        .flat_map(|e| {
            e.leaves
                .iter()
                .filter(|leaf| crate::info::leaves::how(&e.id, leaf).is_none())
                .map(move |leaf| format!("{} {leaf}", e.id))
        })
        .collect();
    assert!(missing.is_empty(), "no route: {missing:?}");
}

/// The Basic table's header and last row and the List's first and last rows
/// round their own fills by the frame's inner radius where they meet the
/// frame (demo::files_table, demo::ListRow), on either core: gpui-pre clips a
/// rounded frame's children to a rectangle (gpui-pre 0.3.10
/// src/window.rs:2238-2246, ContentMask), GPUI Fast to its curved inner edge
/// (gpui-fast 0.1.12 src/fast/rounded_clip.rs:68-104), and a fill rounded
/// inside that edge is drawn alike under both. So their
/// `list.border.corner_radius_px` and `list.border.line_width_px` routes name
/// the inner radius, where the frames' own routes name the frame's.
#[test]
fn the_row_fill_corner_routes_say_the_fills_are_rounded() {
    let mut checked = 0usize;
    for element in crate::elements::elements() {
        let row = ["basic.table.header", "basic.table.row_", "basic.list.row_"]
            .iter()
            .any(|prefix| element.id.starts_with(prefix));
        for leaf in ["list.border.corner_radius_px", "list.border.line_width_px"] {
            if !row || !element.leaves.iter().any(|l| l == leaf) {
                continue;
            }
            checked += 1;
            let how = crate::info::leaves::how(&element.id, leaf).unwrap_or_default();
            assert!(
                how.contains("inner radius") && !how.contains("square"),
                "{} {leaf}: {how:?}",
                element.id
            );
        }
    }
    assert_eq!(
        checked, 10,
        "the table's header and last row and the List's three rows"
    );
}

/// The fill of a row that meets a rounded list frame is rounded where it meets it, by the
/// frame's inner radius (`list.border.corner_radius` less `list.border.line_width`), as the egui
/// (M9c T9) and iced (T9b) Basic pages round theirs, on either core: under adwaita the Basic
/// table's header fill on its top corners, its last (selected) row's on its bottom corners,
/// and the List's first row, hovered, on its top corners; each square on its other corners.
/// The fill is the quad of the row's own bounds the frame paints.
#[gpui::test]
fn the_row_fills_are_rounded_where_they_meet_the_frame(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    show(&mut cx, &showcase, Page::Basic);
    use_preset(&mut cx, &showcase, "adwaita");
    without_motion(&mut cx);
    let inner = f32::from(crate::demo::list_inner_radius(
        &resolved_theme(&mut cx, &showcase).list.border,
    ));
    assert!(
        inner > 0.0,
        "adwaita's list frame is rounded inside its line"
    );
    let row = bounds_of(&mut cx, "basic-list-row-0");
    hover(&mut cx, row.center());
    draw(&mut cx);
    let mut wrong = Vec::new();
    for (selector, top, bottom) in [
        ("basic-table-header", true, false),
        ("basic-table-row-1", false, true),
        ("basic-list-row-0", true, false),
    ] {
        let bounds = bounds_of(&mut cx, selector);
        let radii = cx.update(|window, _| {
            let s = window.scale_factor();
            scene_quads(window)
                .into_iter()
                .filter(|q| !q.background.is_transparent())
                .find(|q| {
                    (q.bounds.origin.x.0 - f32::from(bounds.origin.x) * s).abs() <= 1.0
                        && (q.bounds.origin.y.0 - f32::from(bounds.origin.y) * s).abs() <= 1.0
                        && (q.bounds.size.width.0 - f32::from(bounds.size.width) * s).abs() <= 1.0
                        && (q.bounds.size.height.0 - f32::from(bounds.size.height) * s).abs() <= 1.0
                })
                .map(|q| {
                    let c = q.corner_radii;
                    [c.top_left, c.top_right, c.bottom_left, c.bottom_right].map(|r| r.0 / s)
                })
        });
        let want = [top, top, bottom, bottom].map(|rounded| if rounded { inner } else { 0.0 });
        match radii {
            Some(radii) if radii.iter().zip(want).all(|(r, w)| (r - w).abs() < 1e-3) => {}
            other => wrong.push(format!("{selector}: corners {other:?}, not {want:?}")),
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The labels the showcase lays out one `defaults.line_height` line tall
/// name the route that does it (M9-031): widgets::Switch's label, the side
/// panel's labels and the segments in demo::text_in's line box, the tab
/// rows' labels in widgets::TabBar's, the expander's title and body, and
/// the status bar's text in geometry::status_bar's; none says "not applied"
/// or names gpui's own line height. The status bar's toggle and environment
/// are `layout.widget_gap` apart, and its row says so (M9-032).
#[test]
fn the_line_height_and_status_gap_routes_name_what_applies_them() {
    let routes = [
        ("basic.switches.", "defaults.line_height", "widgets::Switch"),
        (
            "chrome.side_panel.settings.",
            "defaults.line_height",
            "demo::text_in",
        ),
        (
            "chrome.side_panel.inspector_tabs.",
            "defaults.line_height",
            "widgets::TabBar",
        ),
        (
            "chrome.page_tabs.",
            "defaults.line_height",
            "widgets::TabBar",
        ),
        ("basic.tabs.", "defaults.line_height", "widgets::TabBar"),
        ("basic.segmented.", "defaults.line_height", "demo::text_in"),
        ("basic.expander.", "defaults.line_height", "demo::text_in"),
        (
            "chrome.status_bar",
            "defaults.line_height",
            "geometry::status_bar",
        ),
        (
            "chrome.status_bar",
            "layout.widget_gap_px",
            "demo::status_bar",
        ),
    ];
    let mut checked = 0usize;
    let mut wrong = Vec::new();
    for element in crate::elements::elements() {
        for (prefix, leaf, names) in routes {
            if !element.id.starts_with(prefix)
                || !element.leaves.iter().any(|l| l == leaf)
                || (prefix == "chrome.side_panel.settings." && !element.id.ends_with("_label"))
            {
                continue;
            }
            checked += 1;
            let how = crate::info::leaves::how(&element.id, leaf).unwrap_or_default();
            if how.contains("not applied") || how.contains("phi") || !how.contains(names) {
                wrong.push(format!("{} {leaf}: {how:?}, not {names}", element.id));
            }
        }
    }
    assert!(
        checked >= 20,
        "only {checked} rows matched, so this proves little"
    );
    assert!(
        wrong.is_empty(),
        "routes that do not name what applies them:\n  {}",
        wrong.join("\n  ")
    );
}

/// The page row's rule is drawn by demo::native_tab_bar, a box under the
/// strip in `separator.line_color`, `separator.line_width` tall; the strip
/// itself, demo::native_tab_strip, is a bare widgets::TabBar.
#[test]
fn the_page_rows_rule_routes_name_what_draws_it() {
    for leaf in ["separator.line_color", "separator.line_width_px"] {
        let how = crate::info::leaves::how("chrome.page_tabs.rule", leaf).unwrap_or_default();
        assert!(
            how.contains("demo::native_tab_bar") && !how.contains("native_tab_strip"),
            "chrome.page_tabs.rule's {leaf} route: {how:?}"
        );
    }
}

/// The number field pads its value's Input left of the step buttons, which
/// stack at its right (demo::number_input), and a disabled text field's edge
/// is `input.border.color` itself, on the faded surface the showcase draws
/// under it (demo::faded_surface), not upstream's `input` token.
#[test]
fn the_number_and_disabled_field_routes_name_what_draws_them() {
    for side in ["top", "right", "bottom", "left"] {
        let leaf = format!("input.border.padding_{side}_px");
        let how = crate::info::leaves::how("basic.number_input.field", &leaf).unwrap_or_default();
        assert!(
            how.contains("left of the step buttons") && !how.contains("between"),
            "{leaf}: {how:?}"
        );
    }
    let how = crate::info::leaves::how("basic.text_inputs.disabled", "input.border.color")
        .unwrap_or_default();
    assert!(
        how.contains("demo::faded_surface") && !how.contains("ThemeColor input"),
        "the disabled field's edge route: {how:?}"
    );
}

/// Every id the showcase's tables name is an element of the list.
#[test]
fn every_listed_id_is_in_the_list() {
    let list = crate::elements::elements();
    for id in crate::elements::named_ids() {
        assert!(
            list.iter().any(|e| e.id == id),
            "{id} is named by the showcase but is no element of docs/showcase-elements.toml"
        );
    }
}

#[gpui::test]
fn the_synthetic_input_feeds_levels_and_stops_with_the_session(cx: &mut TestAppContext) {
    use crate::speech::{SilentRecognizer, SyntheticAudio};
    use gpui_component::speech::{SpeechState, SpeechStatus};

    let audio = SyntheticAudio::default();
    let blocks = audio.blocks.clone();
    let state = cx.update(|cx| {
        cx.new(|cx| {
            SpeechState::new(cx)
                .recognizer(SilentRecognizer)
                .input(audio)
                .system_fallback(false)
        })
    });

    cx.update(|cx| state.update(cx, |s, cx| s.start(cx)));
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    cx.read(|cx| {
        let s = state.read(cx);
        assert!(
            s.status().is_capturing(),
            "the silent recognizer reports ready"
        );
        assert!(
            s.levels().next().is_some(),
            "no level in a second of synthetic audio"
        );
    });

    cx.update(|cx| state.update(cx, |s, cx| s.stop(cx)));
    cx.run_until_parked();
    assert_eq!(cx.read(|cx| state.read(cx).status()), SpeechStatus::Idle);
    let delivered = blocks.get();
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    assert_eq!(
        blocks.get(),
        delivered,
        "the synthetic input kept running after stop"
    );
}

/// The Icons page's role of each gallery icon is the connector's: the
/// first `IconRole` whose `native_theme_gpui::icons::icon_name` is the icon
/// (`DialogError`, not `StatusError`, for `CircleX`), and none for an icon
/// no role names. The roles below are the hand table the showcase kept
/// until the role was derived, which the derivation matched for every
/// gallery icon; a change to the connector's mapping shows here.
#[test]
fn a_gallery_icons_role_is_the_first_the_connector_names_it_for() {
    use native_theme::icons::IconRole;
    let roles = [
        ("TriangleAlert", IconRole::DialogWarning),
        ("CircleX", IconRole::DialogError),
        ("Info", IconRole::DialogInfo),
        ("CircleCheck", IconRole::DialogSuccess),
        ("WindowClose", IconRole::WindowClose),
        ("WindowMinimize", IconRole::WindowMinimize),
        ("WindowMaximize", IconRole::WindowMaximize),
        ("WindowRestore", IconRole::WindowRestore),
        ("Delete", IconRole::ActionDelete),
        ("Copy", IconRole::ActionCopy),
        ("Undo2", IconRole::ActionUndo),
        ("Redo2", IconRole::ActionRedo),
        ("Search", IconRole::ActionSearch),
        ("Settings", IconRole::ActionSettings),
        ("Plus", IconRole::ActionAdd),
        ("Minus", IconRole::ActionRemove),
        ("RefreshCw", IconRole::ActionRefresh),
        ("ChevronLeft", IconRole::NavBack),
        ("ChevronRight", IconRole::NavForward),
        ("ChevronUp", IconRole::NavUp),
        ("ChevronDown", IconRole::NavDown),
        ("Menu", IconRole::NavMenu),
        ("File", IconRole::FileGeneric),
        ("FolderClosed", IconRole::FolderClosed),
        ("FolderOpen", IconRole::FolderOpen),
        ("Loader", IconRole::StatusBusy),
        ("Check", IconRole::StatusCheck),
        ("User", IconRole::UserAccount),
        ("Bell", IconRole::Notification),
    ];
    for (named, _) in roles {
        assert!(
            crate::support::GPUI_ICONS
                .iter()
                .any(|(name, _)| *name == named),
            "{named} is not in the gallery: its role is checked against nothing"
        );
    }
    for (name, icon) in crate::support::GPUI_ICONS {
        let wanted = roles
            .iter()
            .find(|(named, _)| named == name)
            .map(|(_, role)| *role);
        assert_eq!(crate::support::role_for_gpui_icon(icon), wanted, "{name}");
    }
}

/// The layout dump keeps to the showcases' exit rule (`cli::Outputs`):
/// `--dump-layout F` alone writes F and quits, and with `--screenshot P` it
/// writes F and leaves the window open for P, whose capture quits once both
/// are written.
#[gpui::test]
fn the_layout_dump_quits_only_once_every_file_is_written(cx: &mut TestAppContext) {
    for (case, argv) in [
        ("alone", &["--dump-layout", "F"][..]),
        (
            "with-screenshot",
            &["--dump-layout", "F", "--screenshot", "P"][..],
        ),
    ] {
        let (showcase, root, mut vcx) = open(cx, WINDOW_SIZE);
        show(&mut vcx, &showcase, Page::Basic);
        draw(&mut vcx);
        let path = std::env::temp_dir().join(format!(
            "showcase-gpui-{}-dump-{case}.json",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let cli = crate::CliArgs::parse(argv.iter().map(|arg| arg.to_string()));
        let outputs = Rc::new(std::cell::Cell::new(crate::Outputs::new(&cli)));
        let dumped = path.to_string_lossy().into_owned();
        vcx.update(|_window, cx| {
            crate::dump_layout(cx, root, showcase.clone(), dumped, outputs.clone())
        });
        for _ in 0..20 {
            cx.executor().advance_clock(Duration::from_millis(250));
            cx.run_until_parked();
        }
        assert!(path.is_file(), "{case}: the dump was not written");
        let _ = std::fs::remove_file(&path);
        let open_windows = cx.update(|cx| cx.windows().len());
        let quit = cli.screenshot.is_none();
        assert_eq!(
            open_windows == 0,
            quit,
            "{case}: the dump {} the window",
            if quit { "kept" } else { "closed" }
        );
        if !quit {
            let mut left = outputs.get();
            assert!(
                left.written(crate::Output::Screenshot),
                "{case}: the screenshot written after the dump does not quit"
            );
            vcx.update(|_window, cx| crate::app::quit(cx));
            for _ in 0..4 {
                cx.executor().advance_clock(Duration::from_millis(100));
                cx.run_until_parked();
            }
        }
    }
}

/// The layout dump waits for `DUMP_SETTLED_READINGS` equal readings, and
/// gives up after `DUMP_MAX_READINGS`, naming `--dump-layout` and the path,
/// so a layout that never settles fails the run rather than hold it open.
#[test]
fn a_layout_that_never_settles_fails_the_dump() {
    use crate::{DUMP_MAX_READINGS, DUMP_SETTLED_READINGS, DumpSettle, DumpStep};
    let dump = |x: usize| serde_json::json!({"elements": {"a": {"x": x}}});

    let mut settle = DumpSettle::default();
    for reading in 1..DUMP_SETTLED_READINGS {
        assert_eq!(settle.read(dump(0)), DumpStep::Wait, "reading {reading}");
    }
    assert_eq!(settle.read(dump(0)), DumpStep::Write(dump(0)));

    let mut empty = DumpSettle::default();
    for reading in 1..DUMP_MAX_READINGS {
        let step = empty.read(serde_json::json!({"elements": {}}));
        assert_eq!(step, DumpStep::Wait, "empty reading {reading}");
    }
    assert_eq!(
        empty.read(serde_json::json!({"elements": {}})),
        DumpStep::GiveUp
    );

    let mut moving = DumpSettle::default();
    for reading in 1..DUMP_MAX_READINGS {
        assert_eq!(
            moving.read(dump(reading)),
            DumpStep::Wait,
            "reading {reading}"
        );
    }
    assert_eq!(moving.read(dump(DUMP_MAX_READINGS)), DumpStep::GiveUp);
    let failure = crate::dump_unsettled("f.json");
    assert!(
        failure.starts_with("--dump-layout f.json: the layout did not settle")
            && failure.ends_with(" 20 s"),
        "{failure}"
    );
}

/// The dump's readings, `DUMP_POLL` apart, last the shared crate's
/// `CAPTURE_WAIT`, which it holds to the time `scripts/capture_window.sh`
/// waits for a dump.
#[test]
fn the_dump_readings_take_the_capture_wait() {
    let readings = u32::try_from(crate::DUMP_MAX_READINGS).unwrap_or(u32::MAX);
    assert_eq!(
        crate::DUMP_POLL.saturating_mul(readings),
        native_theme_showcase_support::CAPTURE_WAIT
    );
}

/// Every bundled preset in both modes, resolved as the tests resolve a
/// preset, named `<preset> <mode>`, with the gpui theme the connector
/// builds from it: what an info built under each one reads.
fn every_bundled_variant() -> Vec<(String, native_theme_gpui::ResolvedTheme, Theme)> {
    use native_theme::theme::ColorMode;
    let prefs = native_theme_gpui::AccessibilityPreferences::default();
    let mut out = Vec::new();
    for preset in native_theme::theme::Theme::list_presets() {
        for mode in [ColorMode::Light, ColorMode::Dark] {
            let resolved = native_theme::theme::Theme::preset(preset.key)
                .ok()
                .and_then(|t| t.into_variant(mode).ok())
                .and_then(|v| {
                    v.into_resolved(&native_theme::ResolutionContext::for_tests())
                        .ok()
                });
            assert!(
                resolved.is_some(),
                "{} {mode:?} does not resolve",
                preset.key
            );
            let Some(r) = resolved else { continue };
            let dark = mode == ColorMode::Dark;
            let t = native_theme_gpui::to_theme(&r, preset.key, dark, &prefs);
            let name = format!("{} {}", preset.key, if dark { "dark" } else { "light" });
            out.push((name, r, t));
        }
    }
    out
}

/// The colour `info` claims as `role`, with the field it names.
fn claim_as<'a>(info: &'a WidgetInfo, role: &str) -> Option<(&'a str, gpui::Hsla)> {
    info.colors
        .iter()
        .find(|c| c.role == role)
        .map(|c| (c.field, c.value))
}

/// The text of `info`'s note `what` in `notes`.
fn note_of<'a>(notes: &'a [crate::info::Note], what: &str) -> Option<&'a str> {
    notes
        .iter()
        .find(|n| n.what == what)
        .map(|n| n.text.as_str())
}

/// A refined Button's info claims an edge only where `geometry::button`
/// draws one (M9-059): `button.border.color` at rest, and upstream's colour
/// under the pointer, where `button.border.line_width` is above 0, and no
/// edge in any state where it is 0 (material-3, both modes), whose width
/// replaces upstream's `border_1` -- over every bundled preset in both
/// modes.
#[test]
fn a_refined_buttons_info_claims_an_edge_only_where_one_is_drawn() {
    use crate::demo::{ButtonKind, ButtonState};
    use crate::info::stated;
    let mut branches = (false, false);
    for (name, r, t) in every_bundled_variant() {
        for kind in [
            ButtonKind::Default,
            ButtonKind::DefaultOutline,
            ButtonKind::PrimaryOutline,
            ButtonKind::Primary,
        ] {
            let info = crate::info::buttons::button(
                &t,
                kind,
                ButtonState::Idle,
                false,
                None,
                Some(&r.button),
            );
            let hovered = claim_as(&info, "border, hovered or pressed");
            if r.button.border.line_width > 0.0 {
                branches.0 = true;
                assert_eq!(
                    claim_as(&info, "border at rest"),
                    Some(("border", stated(r.button.border.color))),
                    "{name}: a {} Button's info does not claim the button.border.color it is edged with: {info:?}",
                    kind.name()
                );
                assert_eq!(
                    hovered.is_some(),
                    kind != ButtonKind::Primary,
                    "{name}: a {} Button's info misstates upstream's edge under the pointer: {info:?}",
                    kind.name()
                );
            } else {
                branches.1 = true;
                assert!(
                    info.colors.iter().all(|c| !c.role.starts_with("border")),
                    "{name}: a {} Button's info claims an edge button.border.line_width 0 does not draw: {info:?}",
                    kind.name()
                );
                assert!(
                    note_of(&info.not_themeable, "edge")
                        .is_some_and(|text| text.starts_with("none in any state")),
                    "{name}: a {} Button's info does not say no edge is drawn: {info:?}",
                    kind.name()
                );
            }
        }
    }
    assert_eq!(
        branches,
        (true, true),
        "no bundled preset left a Button edged, or none left one unedged, so a branch went unread"
    );
}

/// A disabled refined Button's info prints the fade `geometry::button_disabled`
/// lays over the stated pair, `button.disabled_opacity`, where it said the
/// pair is painted "with no further fade" (M9-060) -- over every bundled
/// preset in both modes.
#[test]
fn a_disabled_buttons_info_prints_its_fade() {
    use crate::demo::{ButtonKind, ButtonState};
    for (name, r, t) in every_bundled_variant() {
        for kind in [ButtonKind::Default, ButtonKind::Primary] {
            let info = crate::info::buttons::button(
                &t,
                kind,
                ButtonState::Disabled,
                false,
                None,
                Some(&r.button),
            );
            assert!(
                !info.to_text().contains("no further fade"),
                "{name}: a disabled {} Button's info says it is painted with no further fade: {info:?}",
                kind.name()
            );
            let fade = format!(
                "button.disabled_opacity {}",
                percent_text(r.button.disabled_opacity)
            );
            assert!(
                note_of(&info.config, "opacity").is_some_and(|text| text.starts_with(&fade)),
                "{name}: a disabled {} Button's opacity line does not print {fade:?}: {info:?}",
                kind.name()
            );
        }
    }
}

/// The claim of `role` in `info`: that role, or a role that goes on from it
/// after a comma (a hover's, which says whether the theme states one).
#[cfg(feature = "widgets")]
fn claim_by_role<'a>(info: &'a WidgetInfo, role: &str) -> Option<&'a crate::info::ColorClaim> {
    let more = format!("{role},");
    info.colors
        .iter()
        .find(|c| c.role == role || c.role.starts_with(&more))
}

/// A Look colour's source in one state: the leaf the connector reads it from
/// and the line it reads it at, as a claim of it names and cites them.
#[cfg(feature = "widgets")]
type Leaf = (&'static str, &'static str);

/// The leaf each colour of a checkbox's or radio's look is read from,
/// `checked` or not, `disabled` or not, as `CheckboxLook::sized` reads them
/// (widgets/checkbox.rs:123-146): the fill `disabled_background` where a
/// disabled box states one (:129), else `checked_background` (:125), else
/// `unchecked_background`, `background_color` where that is unstated (:123);
/// the hover `hover_background` over that fill (:134), or the fill at rest;
/// the border `border.color` checked (:138), else `unchecked_border_color`,
/// `border.color` where that is unstated (:140); the mark and the label
/// `disabled_text_color` over a stated disabled fill (:143), the mark
/// `indicator_color` and the label `disabled_text_color` disabled over none
/// (:144), and enabled `indicator_color` and `font.color` (:145) -- as
/// (fill, hover, border, mark, label).
#[cfg(feature = "widgets")]
fn checkbox_leaves(
    c: &native_theme::theme::ResolvedCheckboxTheme,
    checked: bool,
    disabled: bool,
) -> [Leaf; 5] {
    const AT_123: &str = "native-theme-gpui/widgets/checkbox.rs:123";
    let stated_disabled = disabled && c.disabled_background.is_some();
    let unchecked = if c.unchecked_background.is_some() {
        ("unchecked_background", AT_123)
    } else {
        ("background_color", AT_123)
    };
    let fill = match (stated_disabled, checked) {
        (true, _) => (
            "disabled_background",
            "native-theme-gpui/widgets/checkbox.rs:129",
        ),
        (false, true) => (
            "checked_background",
            "native-theme-gpui/widgets/checkbox.rs:125",
        ),
        (false, false) => unchecked,
    };
    let hover = if c.hover_background.is_some() {
        (
            "hover_background",
            "native-theme-gpui/widgets/checkbox.rs:134",
        )
    } else {
        unchecked
    };
    let border = match (checked, c.unchecked_border_color.is_some()) {
        (true, _) => ("border", "native-theme-gpui/widgets/checkbox.rs:138"),
        (false, true) => (
            "unchecked_border_color",
            "native-theme-gpui/widgets/checkbox.rs:140",
        ),
        (false, false) => ("border", "native-theme-gpui/widgets/checkbox.rs:140"),
    };
    let (mark, label) = match (disabled, stated_disabled) {
        (true, true) => (
            (
                "disabled_text_color",
                "native-theme-gpui/widgets/checkbox.rs:143",
            ),
            (
                "disabled_text_color",
                "native-theme-gpui/widgets/checkbox.rs:143",
            ),
        ),
        (true, false) => (
            (
                "indicator_color",
                "native-theme-gpui/widgets/checkbox.rs:144",
            ),
            (
                "disabled_text_color",
                "native-theme-gpui/widgets/checkbox.rs:144",
            ),
        ),
        (false, _) => (
            (
                "indicator_color",
                "native-theme-gpui/widgets/checkbox.rs:145",
            ),
            ("font", "native-theme-gpui/widgets/checkbox.rs:145"),
        ),
    };
    [fill, hover, border, mark, label]
}

/// The leaf each colour of a switch's look is read from, `checked` or not,
/// `disabled` or not, as `SwitchLook::of` reads them
/// (widgets/switch.rs:93-144): the track `checked_background` (:94) or
/// `unchecked_background` (:96), disabled `disabled_checked_background`
/// (:99) or `disabled_unchecked_background` (:100) where stated; the hover
/// `hover_checked_background` (:104) or `hover_unchecked_background` (:106)
/// over the track, or the track at rest; the track's outline
/// `checked_track_border_color` (:121) or `unchecked_track_border_color`
/// (:123), disabled their `disabled_*` forms (:126, :128) where stated; the
/// thumb's outline `thumb_border_color` (:140); the thumb `thumb_background`
/// (:111), off `unchecked_thumb_background` where stated (:113), disabled
/// `disabled_thumb_color` where stated (:116); the label
/// `defaults.disabled_text_color` disabled (:142), else `text_color` (:144)
/// -- as (track, hover, track outline, thumb outline, thumb, label).
#[cfg(feature = "widgets")]
fn switch_leaves(
    s: &native_theme::theme::ResolvedSwitchTheme,
    checked: bool,
    disabled: bool,
) -> [Leaf; 6] {
    let base = if checked {
        (
            "checked_background",
            "native-theme-gpui/widgets/switch.rs:94",
        )
    } else {
        (
            "unchecked_background",
            "native-theme-gpui/widgets/switch.rs:96",
        )
    };
    let track = match (checked, disabled) {
        (true, true) if s.disabled_checked_background.is_some() => (
            "disabled_checked_background",
            "native-theme-gpui/widgets/switch.rs:99",
        ),
        (false, true) if s.disabled_unchecked_background.is_some() => (
            "disabled_unchecked_background",
            "native-theme-gpui/widgets/switch.rs:100",
        ),
        _ => base,
    };
    let hover = match checked {
        true if s.hover_checked_background.is_some() => (
            "hover_checked_background",
            "native-theme-gpui/widgets/switch.rs:104",
        ),
        false if s.hover_unchecked_background.is_some() => (
            "hover_unchecked_background",
            "native-theme-gpui/widgets/switch.rs:106",
        ),
        _ => base,
    };
    let rest_edge = if checked {
        (
            "checked_track_border_color",
            "native-theme-gpui/widgets/switch.rs:121",
        )
    } else {
        (
            "unchecked_track_border_color",
            "native-theme-gpui/widgets/switch.rs:123",
        )
    };
    let track_outline = match (checked, disabled) {
        (true, true) if s.disabled_checked_track_border_color.is_some() => (
            "disabled_checked_track_border_color",
            "native-theme-gpui/widgets/switch.rs:126",
        ),
        (false, true) if s.disabled_unchecked_track_border_color.is_some() => (
            "disabled_unchecked_track_border_color",
            "native-theme-gpui/widgets/switch.rs:128",
        ),
        _ => rest_edge,
    };
    let rest_thumb = match (checked, s.unchecked_thumb_background.is_some()) {
        (true, _) => (
            "thumb_background",
            "native-theme-gpui/widgets/switch.rs:111",
        ),
        (false, true) => (
            "unchecked_thumb_background",
            "native-theme-gpui/widgets/switch.rs:113",
        ),
        (false, false) => (
            "thumb_background",
            "native-theme-gpui/widgets/switch.rs:113",
        ),
    };
    let thumb = if disabled && s.disabled_thumb_color.is_some() {
        (
            "disabled_thumb_color",
            "native-theme-gpui/widgets/switch.rs:116",
        )
    } else {
        rest_thumb
    };
    let label = if disabled {
        (
            "disabled_text_color",
            "native-theme-gpui/widgets/switch.rs:142",
        )
    } else {
        ("text_color", "native-theme-gpui/widgets/switch.rs:144")
    };
    [
        track,
        hover,
        track_outline,
        (
            "thumb_border_color",
            "native-theme-gpui/widgets/switch.rs:140",
        ),
        thumb,
        label,
    ]
}

/// What is wrong with `info`'s colours against `want`, the (role, colour,
/// leaf) of each colour of the Look its widget draws from, the colour `None`
/// where the widget paints none: a role claimed in another colour, under
/// another leaf or at another line than the Look reads it from, one claimed
/// where nothing is painted, or one claim more than the Look has -- the
/// roles in `ignored` aside, which no Look holds.
#[cfg(feature = "widgets")]
fn look_findings(
    whose: &str,
    info: &WidgetInfo,
    want: &[(&str, Option<gpui::Hsla>, Leaf)],
    ignored: &[&str],
) -> Vec<String> {
    let mut wrong: Vec<String> = want
        .iter()
        .filter_map(|&(role, colour, (field, cited))| {
            let claimed = claim_by_role(info, role);
            let found = claimed.map(|c| (c.value, c.field, c.cited_at));
            let expected = colour.map(|colour| (colour, field, cited));
            (found != expected).then(|| {
                format!(
                    "{whose}: {role} claims {:?}, the Look has {:?}",
                    found.map(|(v, f, c)| (as_rgba(v), f, c)),
                    expected.map(|(v, f, c)| (as_rgba(v), f, c))
                )
            })
        })
        .collect();
    let claims = info
        .colors
        .iter()
        .filter(|c| !ignored.iter().any(|role| c.role.starts_with(role)))
        .count();
    let drawn = want
        .iter()
        .filter(|(_, colour, _)| colour.is_some())
        .count();
    if claims != drawn {
        wrong.push(format!(
            "{whose}: {claims} claims for the Look's {drawn} colours: {:?}",
            info.colors.iter().map(|c| c.role).collect::<Vec<_>>()
        ));
    }
    wrong
}

/// What is wrong with the focus ring `info` claims, if anything: a control
/// that can take the focus (`focusable`) claims `defaults.focus_ring_color` as
/// stated, read by `FocusRing::of` (native-theme-gpui/widgets/mod.rs:344), where
/// the theme states a ring, and none otherwise.
#[cfg(feature = "widgets")]
fn ring_findings(
    whose: &str,
    info: &WidgetInfo,
    r: &native_theme::theme::ResolvedTheme,
    focusable: bool,
) -> Vec<String> {
    let d = &r.defaults;
    let drawn = focusable
        && d.focus_ring_width.is_finite()
        && d.focus_ring_offset.is_finite()
        && d.focus_ring_width > 0.0;
    let found = claim_by_role(info, "focus ring").map(|c| (c.value, c.field, c.cited_at));
    let expected = drawn.then(|| {
        (
            crate::info::stated(d.focus_ring_color),
            "focus_ring_color",
            "native-theme-gpui/widgets/mod.rs:344",
        )
    });
    if found == expected {
        Vec::new()
    } else {
        vec![format!(
            "{whose}: the focus ring claims {:?}, the theme draws {:?}",
            found.map(|(v, f, c)| (as_rgba(v), f, c)),
            expected.map(|(v, f, c)| (as_rgba(v), f, c))
        )]
    }
}

/// A theme-drawn Checkbox's, Radio's and Switch's info prints what the
/// widget draws (M9-050, M9-051, M9-052, M9-053): each size it prints and
/// each colour it claims is the field of the connector's Look the widget
/// draws from, each colour under the leaf and the line the Look reads it
/// from in that state (`checkbox_leaves`, `switch_leaves`), the mark only
/// where it is painted, checked -- `CheckboxLook::of`, `CheckboxLook::radio`, `SwitchLook::of`,
/// faded over the window's `defaults.background_color` as the widget fades a
/// disabled control (`faded_over`), the switch's two outlines included --
/// the radio's circle `checkbox.radio_indicator_width` where the theme
/// states it (material-3: 20px, where its checkbox is 18), an off switch's
/// thumb `switch.unchecked_thumb_diameter` (material-3: 16px, where an on one
/// is 24), the check mark the connector's stroke at
/// `checkbox.check_mark_stroke_width` where it is stated and gpui-component's
/// own icon where not, and a disabled control's fade a line of its own --
/// over every bundled preset in both modes and every checked and disabled
/// state.
#[cfg(feature = "widgets")]
#[test]
fn a_theme_drawn_controls_info_prints_its_look() {
    use crate::info::{inputs, px_text, stated};
    use native_theme_gpui::widgets::{CheckboxLook, SwitchLook};
    let len = |length: Pixels| format!("{}px", px_text(f32::from(length)));
    let says = |text: Option<&str>, part: &str| text.is_some_and(|text| text.contains(part));
    // A line the widget draws at no width paints nothing.
    let edge = |colour: Option<gpui::Hsla>, width: Pixels| colour.filter(|_| width > px(0.));
    // Which branches the bundled presets reached: a stated radio width that
    // is not the box's, a stated off thumb that is not the on one's, a
    // stated stroke, and a disabled control faded by an opacity below 1.
    let mut reached = [false; 4];
    let mut wrong = Vec::new();
    for (name, r, _) in every_bundled_variant() {
        let ground = stated(r.defaults.background_color);
        for (checked, disabled) in [(false, false), (true, false), (false, true), (true, true)] {
            let whose = format!("{name}, checked {checked}, disabled {disabled}");

            let info = inputs::native_checkbox(&r, "label", checked, disabled);
            let Some(rest) = CheckboxLook::of(&r, checked, disabled) else {
                wrong.push(format!("{whose}: a bundled preset gives no CheckboxLook"));
                continue;
            };
            let look = rest.faded_over(ground);
            let [fill, hover, border, mark, label] =
                checkbox_leaves(&r.checkbox, checked, disabled);
            // The mark fades out while unchecked, so it is painted checked only.
            wrong.extend(look_findings(
                &format!("{whose}: the Checkbox"),
                &info,
                &[
                    ("fill", Some(look.fill), fill),
                    ("hover", look.hover_fill, hover),
                    ("border", edge(Some(look.border), look.border_width), border),
                    ("mark", checked.then_some(look.mark), mark),
                    ("label", Some(look.label), label),
                ],
                &["focus ring"],
            ));
            wrong.extend(ring_findings(
                &format!("{whose}: the Checkbox"),
                &info,
                &r,
                !disabled,
            ));
            for (what, length) in [
                ("indicator", look.indicator),
                ("corner radius", look.radius),
                ("label gap", look.label_gap),
            ] {
                if !says(note_of(&info.config, what), &len(length)) {
                    wrong.push(format!(
                        "{whose}: the Checkbox's {what} line does not print the Look's {}: {:?}",
                        len(length),
                        note_of(&info.config, what)
                    ));
                }
            }
            let stroke = note_of(&info.config, "check mark");
            match look.mark_stroke {
                Some(width) => {
                    reached[2] = true;
                    if !says(stroke, "checkbox.check_mark_stroke_width")
                        || !says(stroke, &len(width))
                        || note_of(&info.not_themeable, "check mark").is_some()
                    {
                        wrong.push(format!(
                            "{whose}: the Checkbox's check mark is stroked at checkbox.check_mark_stroke_width, {}, and its info says {stroke:?} / {:?}",
                            len(width),
                            note_of(&info.not_themeable, "check mark")
                        ));
                    }
                }
                None => {
                    if stroke.is_some() || note_of(&info.not_themeable, "check mark").is_none() {
                        wrong.push(format!(
                            "{whose}: the Checkbox's check mark is gpui-component's own icon, and its info says {stroke:?}"
                        ));
                    }
                }
            }
            let fade = format!("checkbox.disabled_opacity {}", percent_text(rest.opacity));
            let line = note_of(&info.config, "disabled");
            if disabled != line.is_some_and(|text| text.starts_with(&fade)) {
                wrong.push(format!(
                    "{whose}: the Checkbox's fade line is {line:?}, where it is {fade:?} disabled only"
                ));
            }
            reached[3] |= disabled && rest.opacity < 1. && look.fill != rest.fill;

            let info = inputs::native_switch(&r, "label", checked, disabled);
            let Some(rest) = SwitchLook::of(&r, checked, disabled) else {
                wrong.push(format!("{whose}: a bundled preset gives no SwitchLook"));
                continue;
            };
            let look = rest.faded_over(ground);
            let [track, hover, track_outline, thumb_outline, thumb, label] =
                switch_leaves(&r.switch, checked, disabled);
            wrong.extend(look_findings(
                &format!("{whose}: the Switch"),
                &info,
                &[
                    ("track", Some(look.track), track),
                    ("hover", look.hover_track, hover),
                    (
                        "track outline",
                        edge(look.track_border, look.track_border_width),
                        track_outline,
                    ),
                    (
                        "thumb outline",
                        edge(look.thumb_border, look.thumb_border_width),
                        thumb_outline,
                    ),
                    ("thumb", Some(look.thumb_color), thumb),
                    ("label", Some(look.label), label),
                ],
                &["focus ring"],
            ));
            wrong.extend(ring_findings(
                &format!("{whose}: the Switch"),
                &info,
                &r,
                !disabled,
            ));
            let thumb = note_of(&info.config, "thumb");
            let off_thumb = !checked && r.switch.unchecked_thumb_diameter.is_some();
            if !says(thumb, &len(look.thumb))
                || off_thumb
                    != thumb.is_some_and(|text| text.starts_with("switch.unchecked_thumb_diameter"))
            {
                wrong.push(format!(
                    "{whose}: the Switch's thumb is {} across{}, and its info says {thumb:?}",
                    len(look.thumb),
                    if off_thumb {
                        ", switch.unchecked_thumb_diameter"
                    } else {
                        ""
                    }
                ));
            }
            reached[1] |=
                off_thumb && r.switch.unchecked_thumb_diameter != Some(r.switch.thumb_diameter);
            let track = note_of(&info.config, "track");
            if [look.track_width, look.track_height, look.track_radius]
                .iter()
                .any(|length| !says(track, &len(*length)))
            {
                wrong.push(format!(
                    "{whose}: the Switch's track line does not print the Look's sizes: {track:?}"
                ));
            }
            let outlines = note_of(&info.config, "outlines");
            for (colour, width) in [
                (look.track_border, look.track_border_width),
                (look.thumb_border, look.thumb_border_width),
            ] {
                if colour.is_some() && !says(outlines, &len(width)) {
                    wrong.push(format!(
                        "{whose}: the Switch's outlines line does not print the Look's {}: {outlines:?}",
                        len(width)
                    ));
                }
            }
            let fade = format!("switch.disabled_opacity {}", percent_text(rest.opacity));
            let line = note_of(&info.config, "disabled");
            if disabled != line.is_some_and(|text| text.starts_with(&fade)) {
                wrong.push(format!(
                    "{whose}: the Switch's fade line is {line:?}, where it is {fade:?} disabled only"
                ));
            }
        }

        // The page's radios are never disabled.
        let info = inputs::native_radio_column(&r, &["A", "B"], Some(0));
        let (Some(selected), Some(unselected)) = (
            CheckboxLook::radio(&r, true, false),
            CheckboxLook::radio(&r, false, false),
        ) else {
            wrong.push(format!(
                "{name}: a bundled preset gives no radio CheckboxLook"
            ));
            continue;
        };
        let [fill, _, border, mark, label] = checkbox_leaves(&r.checkbox, true, false);
        let [off_fill, off_hover, off_border, _, _] = checkbox_leaves(&r.checkbox, false, false);
        wrong.extend(look_findings(
            &format!("{name}: the radios"),
            &info,
            &[
                ("fill", Some(selected.fill), fill),
                (
                    "border",
                    edge(Some(selected.border), selected.border_width),
                    border,
                ),
                ("mark", Some(selected.mark), mark),
                ("label", Some(selected.label), label),
                ("unselected fill", Some(unselected.fill), off_fill),
                (
                    "unselected border",
                    edge(Some(unselected.border), unselected.border_width),
                    off_border,
                ),
                ("hover", unselected.hover_fill, off_hover),
            ],
            &["focus ring"],
        ));
        wrong.extend(ring_findings(
            &format!("{name}: the radios"),
            &info,
            &r,
            true,
        ));
        if !says(note_of(&info.config, "label gap"), &len(selected.label_gap)) {
            wrong.push(format!(
                "{name}: the radio's label gap line does not print the Look's {}: {:?}",
                len(selected.label_gap),
                note_of(&info.config, "label gap")
            ));
        }
        let circle = note_of(&info.config, "indicator");
        let stated_width = r.checkbox.radio_indicator_width.is_some();
        if !says(circle, &len(selected.indicator))
            || stated_width
                != circle.is_some_and(|text| text.starts_with("checkbox.radio_indicator_width"))
        {
            wrong.push(format!(
                "{name}: the radio's circle is {} across{}, and its info says {circle:?}",
                len(selected.indicator),
                if stated_width {
                    ", checkbox.radio_indicator_width"
                } else {
                    ""
                }
            ));
        }
        reached[0] |= r
            .checkbox
            .radio_indicator_width
            .is_some_and(|w| w != r.checkbox.indicator_width);
        let mark = match (selected.dot, selected.mark_stroke) {
            (Some(dot), _) => says(note_of(&info.config, "dot"), &len(dot)),
            (None, Some(width)) => {
                let line = note_of(&info.config, "mark");
                says(line, "checkbox.check_mark_stroke_width") && says(line, &len(width))
            }
            (None, None) => note_of(&info.not_themeable, "mark").is_some(),
        };
        if !mark {
            wrong.push(format!(
                "{name}: the radio's mark line is not the Look's mark: {info:?}"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "{} of the theme-drawn controls' infos print another value than the Look the widget draws from:\n  {}",
        wrong.len(),
        wrong.join("\n  ")
    );
    assert_eq!(
        reached, [true; 4],
        "no bundled preset stated a radio width of its own, an off thumb of its own, a check \
         mark stroke, or a fade below 1 (in that order), so a branch went unread"
    );
}

/// The application's menus claim a `menu edge` only where the popup draws
/// one (M9-034): `popover.border.color` where `popover.border.line_width`
/// is above 0, and none where it is 0 (material-3, both modes), its config
/// line then saying so -- over every bundled preset in both modes.
#[test]
fn the_app_menus_claim_a_menu_edge_only_where_one_is_drawn() {
    use crate::info::{MenuHost, stated};
    let mut branches = (false, false);
    for (name, r, t) in every_bundled_variant() {
        let info = crate::info::app_menus(&t, MenuHost::Row, Some(&r));
        let line = note_of(&info.config, "menu edge");
        if r.popover.border.line_width > 0.0 {
            branches.0 = true;
            assert_eq!(
                claim_as(&info, "menu edge"),
                Some(("border", stated(r.popover.border.color))),
                "{name}: the menus' info does not claim the popover.border.color the popup is edged with: {info:?}"
            );
            assert!(
                line.is_some_and(|text| text.starts_with("popover.border: ")),
                "{name}: the menus' edge line does not name popover.border: {line:?}"
            );
        } else {
            branches.1 = true;
            assert_eq!(
                claim_as(&info, "menu edge"),
                None,
                "{name}: the menus' info claims an edge popover.border.line_width 0 does not draw: {info:?}"
            );
            assert!(
                line.is_some_and(|text| text.starts_with("none: popover.border.line_width is 0")),
                "{name}: the menus' edge line does not say no edge is drawn: {line:?}"
            );
        }
    }
    assert_eq!(
        branches,
        (true, true),
        "no bundled preset edged the popover, or none left it unedged, so a branch went unread"
    );
}

/// A theme-drawn ProgressBar claims its frame only where it draws one
/// (M9-057): `progress_bar.border.color` where `progress_bar.border.line_width`
/// is above 0, none where it is 0 (adwaita, windows-11, material-3), and
/// its frame line names no question only the archived spec holds -- over
/// every bundled preset in both modes.
#[cfg(feature = "widgets")]
#[test]
fn a_theme_drawn_progress_bar_claims_a_frame_only_where_one_is_drawn() {
    use crate::info::stated;
    let mut branches = (false, false);
    for (name, r, _) in every_bundled_variant() {
        let p = &r.progress_bar;
        let info = crate::info::feedback::native_progress(&r, "Progress", Some(40.), false);
        let line = note_of(&info.config, "frame");
        assert!(
            info.to_text().find("OPEN O4").is_none(),
            "{name}: the ProgressBar's info cites OPEN O4, which no open entry holds: {info:?}"
        );
        if p.border.line_width > 0.0 {
            branches.0 = true;
            assert_eq!(
                claim_as(&info, "frame"),
                Some(("border", stated(p.border.color))),
                "{name}: the ProgressBar's info does not claim the progress_bar.border.color it is framed in: {info:?}"
            );
        } else {
            branches.1 = true;
            assert_eq!(
                claim_as(&info, "frame"),
                None,
                "{name}: the ProgressBar's info claims a frame progress_bar.border.line_width 0 does not draw: {info:?}"
            );
            assert!(
                line.is_some_and(
                    |text| text.starts_with("none: progress_bar.border.line_width is 0")
                ),
                "{name}: the ProgressBar's frame line does not say none is drawn: {line:?}"
            );
        }
    }
    assert_eq!(
        branches,
        (true, true),
        "no bundled preset framed the progress bar, or none left it unframed, so a branch went unread"
    );
}

/// A GroupBox's and a card's info claim the card edge
/// `geometry::group_box_content` draws on the content -- `card.border.color`
/// at `card.border.line_width` -- only where that width is above 0
/// (windows-11 and kde-breeze at the time of writing), and elsewhere say
/// none is drawn, an Outline box's note that its own `border_1` gives way to
/// the 0 width (M9-044); and a card claims no content text, its content
/// being a Label that paints foreground on its own element (M9-049) --
/// over every bundled preset in both modes, and with no native theme.
#[test]
fn a_group_box_and_a_card_claim_the_card_edge_only_where_one_is_drawn() {
    use crate::demo::GroupBoxKind;
    use crate::info::{px_text, stated};
    let mut branches = (false, false);
    for (name, r, t) in every_bundled_variant() {
        let width = r.card.border.line_width;
        let infos = [
            GroupBoxKind::Normal,
            GroupBoxKind::Fill,
            GroupBoxKind::Outline,
        ]
        .map(|kind| {
            (
                kind,
                crate::info::layout::group_box(&t, kind, Some(&r), None, "Title"),
            )
        });
        let card = crate::info::layout::card(&t, Some(&r), None);
        assert!(
            card.colors
                .iter()
                .all(|c| c.role != "content text" && c.field != "group_box_foreground"),
            "{name}: the card's info claims a content text its Label paints over: {card:?}"
        );
        let all = infos
            .iter()
            .map(|(kind, info)| (Some(*kind), info))
            .chain([(None, &card)]);
        for (kind, info) in all {
            let what = kind.map_or("the card", GroupBoxKind::name);
            let line = note_of(&info.config, "edge");
            if width > 0.0 {
                branches.0 = true;
                assert_eq!(
                    claim_as(info, "edge"),
                    Some(("border", stated(r.card.border.color))),
                    "{name}: {what}'s info does not claim the card.border.color its content is edged with: {info:?}"
                );
                assert!(
                    line.is_some_and(|text| text.contains(&format!("{}px", px_text(width)))),
                    "{name}: {what}'s edge line does not print card.border.line_width: {line:?}"
                );
            } else {
                branches.1 = true;
                assert_eq!(
                    claim_as(info, "edge"),
                    None,
                    "{name}: {what}'s info claims an edge card.border.line_width 0 does not draw: {info:?}"
                );
                assert!(
                    line.is_some_and(|text| text.starts_with("none: card.border.line_width is 0")),
                    "{name}: {what}'s edge line does not say no edge is drawn: {line:?}"
                );
                if kind == Some(GroupBoxKind::Outline) {
                    assert!(
                        line.is_some_and(|text| text.contains("border_1 gives way")),
                        "{name}: the Outline box's edge line does not say its own border_1 gives way to the 0 width: {line:?}"
                    );
                }
            }
        }
    }
    assert_eq!(
        branches,
        (true, true),
        "no bundled preset edged the card, or none left it unedged, so a branch went unread"
    );
    let t = Theme::default();
    let card = crate::info::layout::card(&t, None, None);
    assert!(
        card.colors
            .iter()
            .all(|c| c.role != "content text" && c.field != "group_box_foreground"),
        "the card's info without a native theme claims a content text its Label paints over: {card:?}"
    );
}

/// The Basic page's icon buttons claim no edge (review of M9a T7): they are
/// Ghosts the showcase gives only `button.border`'s radius and padding
/// (`demo::tool_box_of`), not `geometry::button`, and upstream gives a Ghost
/// no border width -- under kde-breeze light, whose `button.border.line_width`
/// is above 0, where a Button `geometry::button` refines is edged.
#[gpui::test]
fn the_basic_pages_icon_buttons_claim_no_edge(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_light_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Basic);
    let r = resolved_theme(&mut cx, &showcase);
    assert!(
        r.button.border.line_width > 0.0,
        "kde-breeze no longer edges its buttons, so this test cannot tell an edge claim from none"
    );
    let info = settle_on(&mut cx, &showcase, "basic-icon-copy");
    assert!(info.is_some(), "the Copy icon button shows no info");
    let Some(info) = info else { return };
    assert!(
        info.colors.iter().all(|c| !c.role.starts_with("border")),
        "an icon button's info claims an edge it does not draw: {info:?}"
    );
    assert!(
        info.config
            .iter()
            .all(|n| !n.text.contains("geometry::button")),
        "an icon button's info names geometry::button, which it does not take: {info:?}"
    );
}

/// A disabled refined field's edge swatch is the colour painted: the
/// surface's `input.border.color` faded with the field by the opacity
/// `geometry::input_fill` gives it (`input.disabled_opacity`) -- under the
/// first bundled preset of this platform whose light field is stroked and
/// faded, chosen at test time.
#[gpui::test]
fn a_disabled_fields_edge_swatch_is_its_faded_edge(cx: &mut TestAppContext) {
    let offered = native_theme::theme::Theme::list_presets_for_platform();
    let preset = every_bundled_variant()
        .into_iter()
        .find(|(name, r, _)| {
            name.ends_with(" light")
                && r.input.border.line_width > 0.0
                && r.input.disabled_opacity < 1.0
                && offered
                    .iter()
                    .any(|p| name.starts_with(&format!("{} ", p.key)))
        })
        .and_then(|(name, _, _)| name.strip_suffix(" light").map(str::to_string));
    assert!(
        preset.is_some(),
        "no bundled preset of this platform strokes and fades a disabled field, which this test reads"
    );
    let Some(preset) = preset else { return };
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_light_preset(&mut cx, &showcase, &preset);
    show(&mut cx, &showcase, Page::Basic);
    let info = settle_on(&mut cx, &showcase, BASIC_INPUT_DISABLED);
    assert!(info.is_some(), "the disabled field shows no info");
    let Some(info) = info else { return };
    let claimed = info
        .colors
        .iter()
        .find(|c| c.role.starts_with("edge"))
        .map(|c| c.value);
    hover(&mut cx, point(px(4.), px(4.)));
    draw(&mut cx);
    assert!(
        claimed.is_some(),
        "the disabled field claims no edge: {info:?}"
    );
    assert_eq!(
        claimed,
        painted_edge(&mut cx, BASIC_INPUT_DISABLED),
        "the disabled field's edge swatch is not the edge painted: {info:?}"
    );
}

/// No page spaces or pads anything with one of gpui's bare rem multiples
/// (`gap_<n>()`, `gap_x_<n>()`, `gap_y_<n>()`, `p_<n>()` and its axes and
/// sides), which stand for no value of the theme's (M9-022). A page's root
/// takes `layout.window_margin` and `layout.section_gap`, a row of controls
/// `layout.widget_gap`, each none where the theme states none, as the Basic
/// page lays them out; a distance of another role is a named constant of
/// the showcase's, with its comment and its info line (spec §3.1).
///
/// The gate reads `pages/` and `demo.rs`, whose builders lay out much of
/// the pages' content. A `_0` size sets none and passes. A distance inside
/// a widget sample the showcase builds -- a cell's, a card's, a slide's
/// padding -- is a named constant too, its info naming it; in `demo.rs` a
/// call that is upstream's own value, kept so the sample shows upstream's
/// widget as upstream lays it out, is listed by its function with the
/// reason, and printed.
#[test]
fn no_page_spaces_by_a_bare_rem_multiple() {
    const BARE: [&str; 10] = [
        "gap_x_", "gap_y_", "gap_", "px_", "py_", "pt_", "pr_", "pb_", "pl_", "p_",
    ];
    const INSIDE_A_SAMPLE: [(&str, &[&str], &str); 1] = [(
        "component_toolbar",
        &[".gap_1()"],
        "a ToolbarGroup's gap where toolbar.item_gap is unstated: upstream's own, the gap_1 a Small Toolbar spaces its items by (gpui-component 0.8.1 toolbar.rs:254), which a group, spacing nothing itself (gpui-base 0.8.1 toolbar.rs:256-267), takes from the caller; the sample shows upstream's Toolbar, so it keeps upstream's value and names no constant",
    )];
    let sources = showcase_sources();
    let read: Vec<_> = sources
        .iter()
        .filter(|(path, _)| path.starts_with("pages/") || path == "demo.rs")
        .collect();
    assert!(
        read.len() >= 12,
        "the page files and demo.rs were not found: {:?}",
        read.iter().map(|(path, _)| path).collect::<Vec<_>>()
    );
    // The name of the last `fn` item that starts before `at`.
    let enclosing = |code: &str, at: usize| -> String {
        let before = &code[..at];
        let mut end = before.len();
        while let Some(ix) = before[..end].rfind("fn ") {
            let starts = ix == 0 || before[..ix].ends_with(|c: char| c.is_whitespace());
            if starts {
                let name = &before[ix + 3..];
                let len = name
                    .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .unwrap_or(name.len());
                return name[..len].to_string();
            }
            end = ix;
        }
        String::new()
    };
    let mut found = Vec::new();
    let mut listed = Vec::new();
    for (path, source) in read {
        let code = code_only(source);
        for (at, _) in code.match_indices('.') {
            let rest = &code[at + 1..];
            let Some(prefix) = BARE.iter().find(|p| rest.starts_with(**p)) else {
                continue;
            };
            let size = &rest[prefix.len()..];
            let size_len = size
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(size.len());
            let size = &size[..size_len];
            if size.is_empty() || size == "0" || !rest[prefix.len() + size_len..].starts_with("()")
            {
                continue;
            }
            let call = format!(".{prefix}{size}()");
            let line = code[..at].matches('\n').count() + 1;
            let function = enclosing(&code, at);
            let sample = (path == "demo.rs")
                .then(|| {
                    INSIDE_A_SAMPLE
                        .iter()
                        .find(|(f, calls, _)| *f == function && calls.contains(&call.as_str()))
                })
                .flatten();
            match sample {
                Some((_, _, why)) => {
                    listed.push(format!("{path}:{line}: {call} in {function}: {why}"))
                }
                None => found.push(format!("{path}:{line}: {call} in {function}")),
            }
        }
    }
    println!("inside a widget sample, listed: {listed:#?}");
    println!("bare rem multiples on the pages: {found:#?}");
    assert!(
        found.is_empty(),
        "these page sites space or pad by one of gpui's rem multiples, not by the theme's \
         layout or a named showcase constant:\n{}",
        found.join("\n")
    );
}

/// Each page other than Basic as `(page, first heading, (upper, lower))`:
/// the heading the page's content starts with, and two of the page root's
/// children, one right above the other, whose edges are their sections'
/// (the Icons page's first section ends where its cells do: they stand on
/// one line, bottom-aligned).
const PAGE_SECTIONS: [(Page, &str, (&str, &str)); 10] = [
    (
        Page::Buttons,
        BUTTONS_HEADING_VARIANTS,
        ("buttons-heading-disabled", "buttons-disabled-primary"),
    ),
    (
        Page::Inputs,
        "inputs-heading-text-input",
        ("inputs-heading-text-input", INPUTS_FIELD),
    ),
    (
        Page::Data,
        "data-heading-description-list",
        (LIST_DEMO, "data-heading-tree"),
    ),
    (
        Page::Feedback,
        "feedback-heading-alerts",
        ("feedback-heading-alerts", FEEDBACK_ALERT_INFO),
    ),
    (
        Page::Typography,
        "typography-heading-label",
        ("typography-heading-editor", "typography-editor"),
    ),
    (
        Page::Layout,
        "layout-heading-spacing",
        ("layout-heading-scroll", "layout-scroll-area"),
    ),
    (
        Page::Overlays,
        "overlays-heading-dialog",
        ("overlays-heading-dialog", OVERLAYS_DIALOG_TRIGGER),
    ),
    (
        Page::Charts,
        "charts-heading-bar-chart",
        ("charts-heading-bar-chart", CHARTS_BAR_CHART),
    ),
    (
        Page::Icons,
        "icons-heading-sizes",
        ("icons-size-toolbar", "icons-heading-animated"),
    ),
    (
        Page::ThemeMap,
        "theme-map-heading-all",
        ("theme-map-heading-all", "theme-map-heading-core"),
    ),
];

/// Where `selector` was laid out on the page shown, and its offsets from
/// the page root's top left corner.
fn page_offset(cx: &mut VisualTestContext, selector: &'static str) -> (Pixels, Pixels) {
    let page = bounds_of(cx, PAGE_ROOT);
    let at = bounds_of(cx, selector);
    (at.left() - page.left(), at.top() - page.top())
}

/// Every page other than Basic is padded by `layout.window_margin` and
/// spaces its sections `layout.section_gap` apart, read from the layout the
/// showcase installed, as the Basic page lays out its own (M9-022); the
/// Layout page prints the two values it is laid out by.
#[gpui::test]
fn each_page_is_padded_and_spaced_by_the_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let layout = read(&mut cx, &showcase, |this, _| this.layout.clone());
    let (Some(margin), Some(gap)) = (layout.window_margin, layout.section_gap) else {
        panic!("kde-breeze states no window_margin or section_gap: {layout:?}");
    };
    let mut wrong = Vec::new();
    for (page, first, (upper, lower)) in PAGE_SECTIONS {
        show(&mut cx, &showcase, page);
        let missing: Vec<_> = [first, upper, lower]
            .into_iter()
            .filter(|s| cx.debug_bounds(s).is_none())
            .collect();
        if !missing.is_empty() {
            wrong.push(format!("{page:?}: {missing:?} not laid out"));
            continue;
        }
        let (x, y) = page_offset(&mut cx, first);
        if (x - px(margin)).abs() > px(0.01) || (y - px(margin)).abs() > px(0.01) {
            wrong.push(format!(
                "{page:?}: {first} starts at {x:?}, {y:?} from the page's corner, not \
                 layout.window_margin {margin}px"
            ));
        }
        let apart = bounds_of(&mut cx, lower).top() - bounds_of(&mut cx, upper).bottom();
        if (apart - px(gap)).abs() > px(0.01) {
            wrong.push(format!(
                "{page:?}: {lower} is {apart:?} below {upper}, not layout.section_gap {gap}px"
            ));
        }
        if page == Page::Layout {
            // What the page prints, as its spacing summary's info holds the
            // text the label draws, against what it was measured laid out by.
            let info = settle_on(&mut cx, &showcase, "layout-spacing-summary");
            let printed = info.as_ref().and_then(|info| {
                info.instance
                    .iter()
                    .find(|n| n.what == "text")
                    .map(|n| n.text.clone())
            });
            let value = |role: &str| {
                printed.as_deref().and_then(|text| {
                    let (_, after) = text.split_once(&format!("{role} "))?;
                    let (number, _) = after.split_once("px")?;
                    number.parse::<f32>().ok().map(px)
                })
            };
            for (role, measured) in [("window_margin", x), ("section_gap", apart)] {
                let shown = value(role);
                if !shown.is_some_and(|v| (v - measured).abs() <= px(0.01)) {
                    wrong.push(format!(
                        "Layout: the page prints {role} {shown:?} ({printed:?}) and is laid out \
                         by {measured:?}"
                    ));
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "these pages are not spaced by the theme's layout:\n{}",
        wrong.join("\n")
    );
}

/// Where the theme states none of the layout's roles, no page draws one:
/// no padding round a page, and no gap between its sections or between the
/// controls of a row, on the Basic page as on the others (M9-022). Nothing
/// is invented to fill a role the theme leaves unstated.
#[gpui::test]
fn an_unstated_layout_spaces_no_page(cx: &mut TestAppContext) {
    let unstated = native_theme::theme::LayoutTheme::default();
    let mut root = crate::pages::page_root(&unstated);
    let style = root.style();
    assert_eq!(
        (
            style.padding.top,
            style.padding.right,
            style.padding.bottom,
            style.padding.left
        ),
        (None, None, None, None),
        "a page root is padded where the theme states no window_margin"
    );
    assert_eq!(
        (style.gap.width, style.gap.height),
        (None, None),
        "a page root spaces its sections where the theme states no section_gap"
    );

    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.layout = native_theme::theme::LayoutTheme::default();
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(&mut cx);

    show(&mut cx, &showcase, Page::Buttons);
    assert_eq!(
        page_offset(&mut cx, BUTTONS_HEADING_VARIANTS),
        (px(0.), px(0.)),
        "the Buttons page is padded where the theme states no window_margin"
    );
    assert_eq!(
        bounds_of(&mut cx, "buttons-disabled-primary").top()
            - bounds_of(&mut cx, "buttons-heading-disabled").bottom(),
        px(0.),
        "the Buttons page spaces its sections where the theme states no section_gap"
    );
    assert_eq!(
        bounds_of(&mut cx, "buttons-secondary").left()
            - bounds_of(&mut cx, BUTTONS_PRIMARY).right(),
        px(0.),
        "the Buttons page spaces a row's controls where the theme states no widget_gap"
    );

    show(&mut cx, &showcase, Page::Basic);
    let dump = cx.update(|window, cx| crate::layout_dump(showcase.read(cx), window, cx));
    let rect = |id: &str| {
        let r = &dump["elements"][id];
        let at = |k: &str| r[k].as_f64().map(|v| px(v as f32));
        match (at("x"), at("y"), at("w")) {
            (Some(x), Some(y), Some(w)) => (x, y, w),
            _ => panic!("the layout dump holds no {id}: {dump:#}"),
        }
    };
    let page = bounds_of(&mut cx, PAGE_ROOT);
    let (x1, y1, w1) = rect("basic.column_1");
    let (x2, _, _) = rect("basic.column_2");
    assert_eq!(
        (x1 - page.left(), y1 - page.top()),
        (px(0.), px(0.)),
        "the Basic page is padded where the theme states no window_margin"
    );
    assert_eq!(
        x2 - (x1 + w1),
        px(0.),
        "the Basic page spaces its columns where the theme states no section_gap"
    );
}

/// The note `what` of the info shown, from any of its note lists.
fn shown_note(info: Option<&WidgetInfo>, what: &str) -> Option<String> {
    info.and_then(|info| {
        info.config
            .iter()
            .chain(&info.not_themeable)
            .chain(&info.instance)
            .find(|n| n.what == what)
            .map(|n| n.text.clone())
    })
}

/// Each distance of the showcase's own that the theme states nothing for,
/// on the pages, inside their widget samples and in the inspector's
/// swatches, is a named constant whose size the info of what it spaces
/// prints, at the rem the window lays it out at (spec §3.1). Where the
/// sample's info is not the one its middle shows -- a Collapsible's content
/// and an animated icon's card report their children, a theme-drawn
/// Separator sample that is listed takes no room -- the info is read from
/// its builder.
#[gpui::test]
fn the_pages_own_distances_are_named_in_their_infos(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let rem = |cx: &mut VisualTestContext, r: gpui::Rems| {
        cx.update(|_w, cx| crate::info::rems_text(r, Theme::global(cx)))
    };
    let pixels = |p: Pixels| format!("{}px", crate::info::px_text(p.as_f32()));
    #[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
    let grid = vec![
        rem(&mut cx, crate::pages::ICON_GRID_GAP),
        "ICON_GRID_GAP".to_string(),
    ];
    let cell = |cx: &mut VisualTestContext| {
        vec![
            rem(cx, crate::demo::ICON_CELL_PADDING),
            rem(cx, crate::demo::ICON_CELL_GAP),
            "ICON_CELL_PADDING".to_string(),
            "ICON_CELL_GAP".to_string(),
        ]
    };
    let cell_spacing = cell(&mut cx);
    let cases: Vec<(Page, &'static str, bool, &str, Vec<String>)> = vec![
        #[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
        (
            Page::Icons,
            ICONS_NATIVE_DIALOG_WARNING,
            false,
            "grid gap",
            grid.clone(),
        ),
        #[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
        (
            Page::Icons,
            ICONS_GPUI_TRIANGLE_ALERT,
            false,
            "grid gap",
            grid,
        ),
        (
            Page::Layout,
            "layout-carousel",
            true,
            "side room",
            vec![
                rem(&mut cx, crate::pages::CAROUSEL_SIDE_ROOM),
                "CAROUSEL_SIDE_ROOM".to_string(),
            ],
        ),
        (
            Page::ThemeMap,
            "theme-map-swatch-background",
            false,
            "swatch gaps",
            vec![
                pixels(crate::pages::SWATCH_GAP_X),
                pixels(crate::pages::SWATCH_GAP_Y),
                rem(&mut cx, crate::pages::SWATCH_LABEL_GAP),
                "SWATCH_GAP_X".to_string(),
                "SWATCH_GAP_Y".to_string(),
                "SWATCH_LABEL_GAP".to_string(),
            ],
        ),
        (
            Page::ThemeMap,
            "theme-map-swatch-background",
            false,
            "geometry",
            vec![pixels(crate::pages::SWATCH_SIZE), "SWATCH_SIZE".to_string()],
        ),
        #[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
        (
            Page::Icons,
            ICONS_NATIVE_DIALOG_WARNING,
            false,
            "cell spacing",
            cell_spacing.clone(),
        ),
        #[cfg(all(feature = "material-icons", feature = "svg-rasterize"))]
        (
            Page::Icons,
            ICONS_GPUI_TRIANGLE_ALERT,
            false,
            "cell spacing",
            cell_spacing.clone(),
        ),
        (
            Page::Icons,
            "icons-size-small",
            false,
            "cell spacing",
            cell_spacing,
        ),
        (
            Page::Layout,
            LAYOUT_SEPARATOR_DASHED,
            false,
            "room",
            vec![
                rem(&mut cx, crate::demo::SEPARATOR_SAMPLE_ROOM),
                "SEPARATOR_SAMPLE_ROOM".to_string(),
            ],
        ),
        (
            Page::Layout,
            "layout-scroll-area",
            true,
            "content spacing",
            vec![
                rem(&mut cx, crate::demo::SCROLL_SAMPLE_GAP),
                rem(&mut cx, crate::demo::SCROLL_SAMPLE_PADDING),
                "SCROLL_SAMPLE_GAP".to_string(),
                "SCROLL_SAMPLE_PADDING".to_string(),
            ],
        ),
        (
            Page::Layout,
            "layout-carousel-slide-1",
            false,
            "slide padding",
            vec![
                rem(&mut cx, crate::demo::CAROUSEL_SLIDE_PADDING),
                rem(&mut cx, crate::demo::CAROUSEL_SLIDE_GAP),
                "CAROUSEL_SLIDE_PADDING".to_string(),
                "CAROUSEL_SLIDE_GAP".to_string(),
            ],
        ),
        (
            Page::Overlays,
            "overlays-context-menu",
            false,
            "area padding",
            vec![
                rem(&mut cx, crate::demo::CONTEXT_AREA_PADDING),
                "CONTEXT_AREA_PADDING".to_string(),
            ],
        ),
        (
            Page::Basic,
            CHROME_SIDE_PANEL,
            false,
            "swatches",
            vec![
                pixels(crate::inspector::SWATCH_SIZE),
                rem(&mut cx, crate::inspector::SWATCH_GAP),
                "SWATCH_SIZE".to_string(),
                "SWATCH_GAP".to_string(),
            ],
        ),
    ];
    for (page, selector, corner, what, texts) in cases {
        show(&mut cx, &showcase, page);
        let bounds = bounds_of(&mut cx, selector);
        // The Carousel's top left corner is the room it keeps beside its
        // frame, clear of the slides and controls.
        let at = if corner {
            bounds.origin + point(px(2.), px(2.))
        } else {
            bounds.center()
        };
        let info = settle_at(&mut cx, &showcase, at);
        let note = shown_note(info.as_ref(), what);
        assert!(
            note.as_deref()
                .is_some_and(|t| texts.iter().all(|text| t.contains(text.as_str()))),
            "{selector}: its {what} line does not name {texts:?}: {note:?}"
        );
    }
    let built = cx.update(|_window, cx| {
        let t = Theme::global(cx);
        #[cfg_attr(not(feature = "widgets"), allow(unused_mut))]
        let mut built = vec![
            (
                "the Collapsible",
                crate::info::layout::collapsible(t, true),
                "content padding",
                vec![
                    crate::info::rems_text(crate::demo::COLLAPSIBLE_CONTENT_PADDING, t),
                    "COLLAPSIBLE_CONTENT_PADDING".to_string(),
                ],
            ),
            (
                "an animated icon's card",
                crate::info::icons::animated_icon(
                    t,
                    "set",
                    crate::demo::AnimatedKind::Spin {
                        duration_ms: std::num::NonZeroU32::MIN,
                    },
                    false,
                    t.foreground,
                    false,
                ),
                "card spacing",
                vec![
                    crate::info::rems_text(crate::demo::ANIMATED_CARD_PADDING, t),
                    crate::info::rems_text(crate::demo::ANIMATED_CARD_GAP, t),
                    "ANIMATED_CARD_PADDING".to_string(),
                    "ANIMATED_CARD_GAP".to_string(),
                ],
            ),
        ];
        #[cfg(feature = "widgets")]
        if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
            built.push((
                "a theme-drawn Separator sample",
                crate::info::layout::native_separator(t, r, true),
                "room",
                vec![
                    crate::info::rems_text(crate::demo::SEPARATOR_SAMPLE_ROOM, t),
                    "SEPARATOR_SAMPLE_ROOM".to_string(),
                ],
            ));
        }
        built
    });
    for (sample, info, what, texts) in built {
        let note = shown_note(Some(&info), what);
        assert!(
            note.as_deref()
                .is_some_and(|t| texts.iter().all(|text| t.contains(text.as_str()))),
            "{sample}: its {what} line does not name {texts:?}: {note:?}"
        );
    }
}

/// The Feedback page's Badges are a row of controls: a count stands
/// `layout.widget_gap` from the next Badge, as the controls of every other
/// row are apart, and touches it where the theme states no gap. Upstream
/// draws a Medium count 3px per digit past its Button's right edge
/// (gpui-component 0.8.1 `badge.rs:137-141`, the `Size::Medium` arm), so
/// the Buttons are that much further apart (M9-022).
#[gpui::test]
fn a_badges_count_is_a_widget_gap_from_the_next(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    for stated in [true, false] {
        if !stated {
            cx.update(|_window, cx| {
                showcase.update(cx, |this, cx| {
                    this.layout.widget_gap = None;
                    cx.notify();
                });
            });
            cx.run_until_parked();
            draw(&mut cx);
        }
        show(&mut cx, &showcase, Page::Feedback);
        let gap = read(&mut cx, &showcase, |this, _| this.layout.widget_gap).map_or(px(0.), px);
        assert_eq!(stated, gap > px(0.), "kde-breeze states no widget_gap");
        // Messages (5) before Notifications (99): one digit; Notifications
        // before Updates (a dot): two.
        for (before, after, digits, count) in [
            (
                FEEDBACK_BADGE_COUNT,
                "feedback-badge-notifications",
                "one-digit",
                5,
            ),
            (
                "feedback-badge-notifications",
                FEEDBACK_BADGE_DOT,
                "two-digit",
                99,
            ),
        ] {
            let overhang = crate::pages::badge_count_overhang(Some(count));
            let apart = bounds_of(&mut cx, after).left() - bounds_of(&mut cx, before).right();
            assert!(
                (apart - (gap + overhang)).abs() <= px(0.01),
                "widget_gap {gap:?}: {before} and {after} are {apart:?} apart, not the gap and \
                 the {digits} count's {overhang:?}"
            );
        }
        let info = settle_on(&mut cx, &showcase, FEEDBACK_BADGE_COUNT);
        let note = shown_note(info.as_ref(), "next badge");
        let says = if stated {
            format!(
                "{}px after the Button: layout.widget_gap, {}px",
                crate::info::px_text((gap + px(3.)).as_f32()),
                crate::info::px_text(gap.as_f32())
            )
        } else {
            "3px after the Button".to_string()
        };
        assert!(
            note.as_deref().is_some_and(|t| t.starts_with(&says)
                && t.contains("BADGE_COUNT_OVERHANG")
                && (stated || t.contains("layout.widget_gap is unstated"))),
            "widget_gap {gap:?}: the Badge's info does not say what follows it: {note:?}"
        );
    }
}

/// How far a Badge's count stands past its Button is upstream's 3px a
/// printed character (gpui-component 0.8.1 `badge.rs:137-141`, the
/// `Size::Medium` arm): one digit 3px, two 6px, a count above 99 printed
/// `99+`, three characters (`badge.rs:130-134`); none for a dot, which sits
/// inside the Button (`badge.rs:129`), nor for a zero count, which upstream
/// does not draw (`badge.rs:103`).
#[test]
fn a_badges_overhang_is_its_printed_counts() {
    let per = crate::pages::BADGE_COUNT_OVERHANG;
    for (count, printed) in [
        (None, 0),
        (Some(0), 0),
        (Some(5), 1),
        (Some(99), 2),
        (Some(100), 3),
        (Some(12_345), 3),
    ] {
        assert_eq!(
            crate::pages::badge_count_overhang(count),
            per * printed as f32,
            "a Badge of {count:?} overhangs its Button by other than {printed} characters"
        );
    }
}

/// Under a native theme the status bar's top edge is
/// `status_bar.border.line_width` thick (`demo::status_bar`), as the listed
/// panel's leaf row states it: no note says upstream's `border_t_1` stands
/// there (M9-033). Without a native theme it does stand, and the info says so.
#[gpui::test]
fn the_status_bars_info_names_upstreams_edge_only_without_a_theme(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "adwaita");
    let info = settle_on(&mut cx, &showcase, CHROME_STATUS_BAR);
    assert!(info.is_some(), "the status bar shows no info");
    let upstream: Vec<&crate::info::Note> = info
        .iter()
        .flat_map(|info| &info.not_themeable)
        .filter(|n| n.text.contains("border_t_1"))
        .collect();
    assert!(
        upstream.is_empty(),
        "under adwaita the status bar's info says upstream's 1px edge stands, which demo::status_bar draws status_bar.border.line_width wide: {upstream:?}"
    );
    let width =
        cx.update(|_window, cx| native_value(cx, |n| n.resolved.status_bar.border.line_width));
    let edge = shown_note(info.as_ref(), "top edge");
    assert!(
        width.is_some_and(|w| edge
            .as_deref()
            .is_some_and(|t| t.contains("status_bar.border.line_width")
                && t.contains(&format!("{}px", crate::info::px_text(w))))),
        "the status bar's info does not give its top edge as status_bar.border.line_width, {width:?}: {edge:?}"
    );
    let bare = cx.update(|_window, cx| crate::info::status_bar(Theme::global(cx), false, ""));
    assert!(
        bare.not_themeable
            .iter()
            .any(|n| n.what == "edge" && n.text.contains("border_t_1")),
        "without a native theme the status bar's info does not name upstream's border_t_1: {:?}",
        bare.not_themeable
    );
}

/// The words of `text`: its runs of letters, digits and underscores, so
/// `px_2` is told from `px_2p5`.
fn words(text: &str) -> std::collections::BTreeSet<&str> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|w| !w.is_empty())
        .collect()
}

/// Under a theme that states `button.border.padding`, the toolbar's buttons
/// and the status bar's toggle are that padding round their icon, with no
/// fixed height where a top or bottom side is stated, and rounded by
/// `button.border.corner_radius` (`demo::tool_box_of`): their info names
/// upstream's height (`h_8`, `h_6`) and side padding (`px_2p5`, `px_2`) only
/// for what the theme leaves unstated (M9-035).
#[gpui::test]
fn the_tool_buttons_info_names_upstreams_size_only_where_unstated(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "adwaita");
    let padding = cx.update(|_window, cx| native_value(cx, |n| n.resolved.button.border.padding));
    assert!(
        padding.is_some_and(|p| p.top.is_some() || p.bottom.is_some()),
        "adwaita states no vertical button.border.padding side, which this test needs: {padding:?}"
    );
    let Some(p) = padding else { return };
    for (selector, height, across) in [
        (CHROME_TOOLBAR_PALETTE, "h_8", "px_2p5"),
        (CHROME_SIDE_PANEL_TOGGLE, "h_6", "px_2"),
    ] {
        let info = settle_on(&mut cx, &showcase, selector);
        let text = info.as_ref().map(WidgetInfo::to_text).unwrap_or_default();
        let said = words(&text);
        assert!(
            text.starts_with("Button"),
            "{selector} shows another widget's info: {text}"
        );
        assert_eq!(
            said.contains(height),
            p.top.is_none() && p.bottom.is_none(),
            "{selector}: upstream's {height} is named against the stated padding {p:?}:\n{text}"
        );
        assert_eq!(
            said.contains(across),
            p.left.is_none() || p.right.is_none(),
            "{selector}: upstream's {across} is named against the stated padding {p:?}:\n{text}"
        );
        assert!(
            text.contains("button.border.padding") && text.contains("button.border.corner_radius"),
            "{selector}: the info does not name button.border.padding and corner_radius:\n{text}"
        );
    }
}

/// The tool buttons' size notes under every bundled preset in both modes,
/// and with no native theme: upstream's height where no top or bottom side
/// of `button.border.padding` is stated, its side padding where a left or
/// right one is not, `button.border.corner_radius` and the stated sides
/// named wherever a native theme is installed (M9-035) -- for the toolbar's
/// buttons, the status bar's toggle, the Buttons page's toolbar items and
/// the Basic page's icon buttons, which `demo::tool_box_of` sizes alike.
#[test]
fn every_tool_buttons_size_note_follows_the_stated_padding() {
    let icon = ChromeIcon::Builtin("Search".into());
    let infos = |t: &Theme, native: Option<&native_theme::theme::ResolvedButtonTheme>| {
        [
            (
                "toolbar button",
                "h_8",
                "px_2p5",
                crate::info::toolbar_button(t, "a", &icon, "set", false, None, native),
            ),
            (
                "panel toggle",
                "h_6",
                "px_2",
                crate::info::panel_toggle(
                    t,
                    &crate::demo::PanelToggle {
                        button_id: "toggle",
                        icon: IconName::PanelLeft,
                        drawn: icon.clone(),
                        set: "set",
                        tooltip: "Toggle",
                        action: &ToggleSidePanel,
                        open: true,
                        about: "a",
                        state: "s",
                    },
                    false,
                    None,
                    native,
                ),
            ),
            (
                "toolbar item",
                "h_8",
                "px_2p5",
                crate::info::buttons::toolbar_item(t, &icon, "set", native),
            ),
            (
                "Basic icon button",
                "h_8",
                "px_2p5",
                crate::info::buttons::icon_button(t, true, native),
            ),
        ]
    };
    let variants = every_bundled_variant();
    for (name, r, t) in &variants {
        let p = r.button.border.padding;
        for (which, height, across, info) in infos(t, Some(&r.button)) {
            let text = info.to_text();
            let said = words(&text);
            assert_eq!(
                said.contains(height),
                p.top.is_none() && p.bottom.is_none(),
                "{name}: the {which}'s info names {height} against {p:?}:\n{text}"
            );
            assert_eq!(
                said.contains(across),
                p.left.is_none() || p.right.is_none(),
                "{name}: the {which}'s info names {across} against {p:?}:\n{text}"
            );
            let corners = format!(
                "button.border.corner_radius, {}px",
                crate::info::px_text(r.button.border.corner_radius.max(0.))
            );
            assert!(
                text.contains(&corners),
                "{name}: the {which}'s info does not round it by {corners}:\n{text}"
            );
            for (side, v) in [
                ("top", p.top),
                ("right", p.right),
                ("bottom", p.bottom),
                ("left", p.left),
            ] {
                if let Some(v) = v {
                    let stated = format!("{side} {}px", crate::info::px_text(v));
                    assert!(
                        text.contains("button.border.padding") && text.contains(&stated),
                        "{name}: the {which}'s info does not name the stated {stated}:\n{text}"
                    );
                }
            }
        }
    }
    let Some((_, _, t)) = variants.first() else {
        return;
    };
    for (which, height, across, info) in infos(t, None) {
        let note = shown_note(Some(&info), "size").unwrap_or_default();
        assert!(
            note.starts_with(&format!("{height} with {across} at ")),
            "with no native theme the {which}'s size note is not upstream's {height} with {across}: {note:?}"
        );
    }
}

/// A TabBar built from gpui-component's -- the chrome tab rows of a build
/// without `widgets`, and of every build without a native theme -- is a
/// per-Size height that `tab.min_height` raises where a native theme is
/// installed (`demo::tab_bar` gives each Tab `min_h`): its info says so
/// there, and that nothing applies it only where none is (M9-036). The
/// default build draws the theme's rows with `widgets::TabBar`, so the info
/// is built here as the build without `widgets` builds it.
#[test]
fn an_upstream_tab_bars_height_note_follows_the_native_theme() {
    let variants = every_bundled_variant();
    let height = |info: &WidgetInfo| {
        info.not_themeable
            .iter()
            .find(|n| n.what == "height")
            .map(|n| n.text.clone())
            .unwrap_or_default()
    };
    for (name, r, t) in &variants {
        for (which, info) in [
            (
                "inspector",
                crate::info::tab_font(crate::info::inspector_tab_bar(t, Some(r)), Some(r)),
            ),
            (
                "page",
                crate::info::tab_font(crate::info::page_tab_bar(t, Some(r)), Some(r)),
            ),
        ] {
            let note = height(&info);
            assert!(
                note.contains("tab.min_height") && !note.contains("Nothing applies it"),
                "{name}: the {which} tab bar's height note denies tab.min_height, which each Tab takes: {note:?}"
            );
        }
    }
    let Some((_, _, t)) = variants.first() else {
        return;
    };
    let note = height(&crate::info::tab_font(
        crate::info::page_tab_bar(t, None),
        None,
    ));
    assert!(
        note.contains("Nothing applies it"),
        "with no native theme the page tab bar's height note does not say nothing applies tab.min_height: {note:?}"
    );
}

/// Every theme-drawn tab row's info marks the selected tab as
/// `widgets::TabBar` does, from `TabLook::of(..).indicator`: a line
/// `tab.active_indicator_width` thick along `tab.active_indicator_side` in
/// `tab.active_indicator_color` where the look has one, claimed then and
/// only then, and none where it has not; and it gives the gap the look lays
/// the tabs apart by, `tab.item_gap` (M9-045). Over every bundled preset in
/// both modes, one with the indicator's colour taken away, and one whose
/// non-finite length leaves no look at all.
#[cfg(feature = "widgets")]
#[test]
fn every_native_tab_rows_selection_mark_is_the_looks() {
    let mut variants: Vec<(String, native_theme_gpui::ResolvedTheme)> = every_bundled_variant()
        .into_iter()
        .map(|(name, r, _)| (name, r))
        .collect();
    if let Some((name, r)) = variants.first().cloned() {
        let mut r = r;
        r.tab.active_indicator_color = None;
        variants.push((format!("{name}, no indicator colour"), r));
    }
    let mut seen = (false, false);
    for (name, r) in &variants {
        let look = native_theme_gpui::widgets::TabLook::of(r);
        assert!(look.is_some(), "{name}: the tab look does not build");
        let Some(look) = look else { continue };
        let info = crate::info::layout::native_tab_row(r, "widgets::TabBar");
        let mark = shown_note(Some(&info), "selection mark").unwrap_or_default();
        let claimed = info
            .colors
            .iter()
            .find(|c| c.field == "active_indicator_color")
            .map(|c| c.value);
        match look.indicator {
            Some(line) => {
                seen.0 = true;
                let side = match line.side {
                    native_theme::theme::TabIndicatorSide::Top => "top",
                    native_theme::theme::TabIndicatorSide::Bottom => "bottom",
                };
                let width = format!("{}px", crate::info::px_text(f32::from(line.width)));
                assert!(
                    mark.contains("tab.active_indicator_width")
                        && mark.contains(&width)
                        && mark.contains("tab.active_indicator_side")
                        && words(&mark).contains(side),
                    "{name}: the selection mark is not the look's {width} line along the {side}: {mark:?}"
                );
                assert_eq!(
                    claimed,
                    Some(line.color),
                    "{name}: the selected tab's line is not claimed in the look's colour"
                );
            }
            None => {
                seen.1 = true;
                assert!(
                    mark.starts_with("none: the theme states no indicator"),
                    "{name}: the look has no indicator, but the selection mark says {mark:?}"
                );
                assert_eq!(
                    claimed, None,
                    "{name}: an indicator colour is claimed that widgets::TabBar does not draw"
                );
            }
        }
        assert!(
            info.config.iter().any(|n| n.what == "selection mark")
                && !info
                    .not_themeable
                    .iter()
                    .any(|n| n.what == "selection mark"),
            "{name}: the selection mark is not a config line: {:?}",
            info.not_themeable
        );
        let gap = shown_note(Some(&info), "gap").unwrap_or_default();
        let stated = r
            .tab
            .item_gap
            .map(|_| format!("{}px", crate::info::px_text(f32::from(look.gap))));
        assert!(
            gap.contains("tab.item_gap")
                && stated.as_deref().is_none_or(|g| gap.starts_with(g))
                && (stated.is_some() || gap.starts_with("none")),
            "{name}: the gap line does not give tab.item_gap {stated:?}: {gap:?}"
        );
    }
    assert!(
        seen.0 && seen.1,
        "the presets did not cover both an indicator and none: {seen:?}"
    );
    // A length that is not finite: TabLook::of builds no look, the bar is
    // gpui-component's, and the info gives that reason, not a theme that
    // states no indicator.
    let Some((name, mut r)) = variants.first().cloned() else {
        return;
    };
    r.tab.min_width = f32::NAN;
    assert!(
        native_theme_gpui::widgets::TabLook::of(&r).is_none(),
        "{name}: a non-finite tab.min_width still builds a tab look"
    );
    let info = crate::info::layout::native_tab_row(&r, "widgets::TabBar");
    for what in ["selection mark", "gap"] {
        let text = shown_note(Some(&info), what).unwrap_or_default();
        assert!(
            text.contains("not finite") && !text.contains("states no indicator"),
            "{name}, a non-finite length: the {what} line does not give that reason: {text:?}"
        );
    }
    assert!(
        info.colors
            .iter()
            .all(|c| c.field != "active_indicator_color"),
        "{name}, a non-finite length: an indicator colour is claimed that no look draws"
    );
}

/// A theme-drawn tab row's info gives the selected tab's outward corners as
/// `widgets::TabBar` draws them, from `TabLook::outward_radius`: the radius
/// and the outline with no bottom edge where the theme states one, square
/// feet and the outline as before where it does not.
#[cfg(feature = "widgets")]
#[test]
fn the_native_tab_rows_info_gives_its_outward_corners() {
    let (_, stated) = windows_11_curving_out(false);
    let mut unstated = stated.clone();
    unstated.tab.active_outward_corner_radius = None;
    let info = crate::info::layout::native_tab_row(&stated, "widgets::TabBar");
    let corners = shown_note(Some(&info), "outward corners").unwrap_or_default();
    assert!(
        corners.starts_with("tab.active_outward_corner_radius, 4px")
            && corners.contains("no bottom edge")
            && corners.contains("TabLook::outward_radius"),
        "the stated radius is not given: {corners:?}"
    );
    let outline = shown_note(Some(&info), "outline").unwrap_or_default();
    assert!(
        outline.contains("no bottom edge"),
        "the outline keeps a bottom edge: {outline:?}"
    );
    let info = crate::info::layout::native_tab_row(&unstated, "widgets::TabBar");
    let corners = shown_note(Some(&info), "outward corners").unwrap_or_default();
    assert!(
        corners.starts_with("none: the theme states no outward corners"),
        "unstated: {corners:?}"
    );
    let outline = shown_note(Some(&info), "outline").unwrap_or_default();
    assert!(
        !outline.contains("no bottom edge"),
        "unstated, the outline has no bottom edge: {outline:?}"
    );
}

/// `tab.active_outward_corner_radius_px` has its route on each element the
/// shared list gives it: the three selected tabs draw the outward
/// corners, the three bars inset their tabs by it, and the Basic page's
/// column rule makes room for both flares.
#[test]
fn the_outward_corner_routes_name_what_draws_them() {
    let leaf = "tab.active_outward_corner_radius_px";
    for (element, says) in [
        ("basic.tabs.one", "painted after the tabs"),
        ("chrome.page_tabs.basic", "painted after the tabs"),
        (
            "chrome.side_panel.inspector_tabs.widget",
            "painted after the tabs",
        ),
        ("basic.tabs.bar", "inset at the row's ends"),
        ("chrome.page_tabs", "inset at the row's ends"),
        (
            "chrome.side_panel.inspector_tabs",
            "inset at the row's ends",
        ),
        ("basic.page", "column rule (basic_column_count)"),
    ] {
        let line = crate::info::leaves::how(element, leaf).unwrap_or_default();
        assert!(
            line.contains(says) && line.starts_with("gpui: "),
            "{element}: {line:?}"
        );
    }
}

/// The shapes of `button.border.padding` no bundled preset states, spliced
/// into one: across only (upstream's height stays), up and down only (its
/// side padding stays), one vertical side (the other none, under no fixed
/// height), and one horizontal side (upstream's side padding on the other).
/// Each tool button's size note says what `demo::tool_box_of` leaves of
/// upstream's, and no more (M9-035).
#[test]
fn a_partly_stated_tool_button_padding_leaves_upstream_the_rest() {
    let variants = every_bundled_variant();
    let Some((name, r, t)) = variants.first() else {
        return;
    };
    // One spliced padding, [top, right, bottom, left], and the words the
    // size note holds, those the info does not hold, and what the radius
    // and padding line says.
    struct Shape {
        name: &'static str,
        sides: [Option<f32>; 4],
        said: &'static [&'static str],
        unsaid: &'static [&'static str],
        config: &'static [&'static str],
    }
    let shape = |name, sides, said, unsaid, config| Shape {
        name,
        sides,
        said,
        unsaid,
        config,
    };
    let shapes = [
        shape(
            "across only",
            [None, Some(9.), None, Some(9.)],
            &["h_8"],
            &["px_2p5"],
            &[],
        ),
        shape(
            "up and down only",
            [Some(3.), None, Some(3.), None],
            &["px_2p5"],
            &["h_8"],
            &["h_auto"],
        ),
        shape(
            "the top alone, and across",
            [Some(3.), Some(9.), None, Some(9.)],
            &[],
            &["h_8", "px_2p5"],
            &["h_auto", "an unstated one none"],
        ),
        shape(
            "the left alone",
            [None, None, None, Some(9.)],
            &["h_8", "px_2p5", "right"],
            &[],
            &[],
        ),
        shape(
            "the right alone",
            [None, Some(9.), None, None],
            &["h_8", "px_2p5", "left"],
            &[],
            &[],
        ),
    ];
    for Shape {
        name: shape,
        sides: [top, right, bottom, left],
        said,
        unsaid,
        config,
    } in shapes
    {
        let mut button = r.button.clone();
        button.border.padding.top = top;
        button.border.padding.right = right;
        button.border.padding.bottom = bottom;
        button.border.padding.left = left;
        let info = crate::info::buttons::icon_button(t, true, Some(&button));
        let note = shown_note(Some(&info), "size");
        let note_words = note.as_deref().map(words).unwrap_or_default();
        assert_eq!(
            note.is_some(),
            !said.is_empty(),
            "{name}, {shape}: a size note where upstream keeps nothing, or none where it keeps something: {note:?}"
        );
        for word in said {
            assert!(
                note_words.contains(word),
                "{name}, {shape}: the size note does not name {word}: {note:?}"
            );
        }
        let text = info.to_text();
        let text_words = words(&text);
        for word in unsaid {
            assert!(
                !text_words.contains(word),
                "{name}, {shape}: the info names upstream's {word}, which the stated padding replaces:\n{text}"
            );
        }
        let radius = shown_note(Some(&info), "radius and padding").unwrap_or_default();
        for part in config {
            assert!(
                radius.contains(part),
                "{name}, {shape}: the radius and padding line does not say {part:?}: {radius:?}"
            );
        }
    }
}

/// The Layout page's "Supported toolkits" answer counts and names the
/// connectors from `pages::layout::CONNECTORS`, and that list is the
/// workspace's: every `connectors/` member of the root Cargo.toml is a
/// listed connector or the widget crate beside one (`<connector>-widgets`,
/// native-theme-egui-widgets beside native-theme-egui), and every listed
/// connector is a member.
#[test]
fn the_supported_toolkits_answer_counts_the_connectors() {
    use crate::pages::layout::{CONNECTORS, supported_toolkits};
    let manifest: toml::Table = toml::from_str(include_str!("../../../../Cargo.toml"))
        .expect("the workspace manifest parses");
    let members: Vec<&str> = manifest
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
        .expect("the workspace manifest lists its members")
        .iter()
        .filter_map(|m| m.as_str()?.strip_prefix("connectors/"))
        .collect();
    let listed: Vec<&str> = CONNECTORS.iter().map(|&(_, krate)| krate).collect();
    for krate in &listed {
        assert!(
            members.contains(krate),
            "{krate} is listed as a connector, but the workspace has no connectors/{krate}"
        );
    }
    for member in &members {
        assert!(
            listed.contains(member)
                || member
                    .strip_suffix("-widgets")
                    .is_some_and(|connector| listed.contains(&connector)),
            "connectors/{member} is a connector CONNECTORS leaves out"
        );
    }
    let answer = supported_toolkits();
    assert!(
        answer.starts_with(&format!("{} toolkits", CONNECTORS.len())),
        "the answer does not count the {} connectors: {answer:?}",
        CONNECTORS.len()
    );
    for (toolkit, krate) in CONNECTORS {
        assert!(
            answer.contains(&format!("{toolkit} ({krate})")),
            "the answer does not name {toolkit} through {krate}: {answer:?}"
        );
    }
}

/// The Feedback page's Tags heading counts the colours of `TAGS`' filled
/// Tags, one colour each, beside the outlined ones.
#[test]
fn the_tags_heading_counts_the_tags_colours() {
    use crate::demo::TagKind;
    use crate::pages::feedback::{TAGS, tags_heading};
    let filled: Vec<TagKind> = TAGS
        .iter()
        .filter(|&&(_, _, outlined, _)| !outlined)
        .map(|&(_, kind, _, _)| kind)
        .collect();
    for (ix, kind) in filled.iter().enumerate() {
        assert!(
            !filled.iter().take(ix).any(|k| k == kind),
            "two filled Tags of one colour, which the heading would count twice"
        );
    }
    assert!(
        TAGS.iter().any(|&(_, _, outlined, _)| outlined),
        "the heading names outlined Tags, and TAGS holds none"
    );
    assert_eq!(
        tags_heading(),
        format!("Tags ({} colors + outline)", filled.len())
    );
}

/// The Feedback page's Marker heading counts `MARKERS`: its variants, one
/// Marker each, and its loading styles, one each.
#[test]
fn the_markers_heading_counts_the_markers() {
    use crate::pages::feedback::{MARKERS, markers_heading};
    for (ix, &(_, kind, _)) in MARKERS.iter().enumerate() {
        assert!(
            !MARKERS.iter().take(ix).any(|&(_, k, _)| k == kind),
            "two Markers of one kind, which the heading would count twice"
        );
    }
    let loading = MARKERS.iter().filter(|&&(_, k, _)| k.is_loading()).count();
    assert!(
        loading > 0 && loading < MARKERS.len(),
        "the heading names variants and loading styles, and MARKERS holds {loading} loading of {}",
        MARKERS.len()
    );
    assert_eq!(
        markers_heading(),
        format!(
            "Marker ({} variants, {loading} loading styles)",
            MARKERS.len() - loading
        )
    );
}

/// The Typography page's Kbds take gpui's `secondary` modifier, as the
/// showcase's own bindings do (`app::init`): Ctrl off macOS and Cmd on it.
/// A `cmd-` keystroke is the platform key everywhere, which gpui-component
/// draws as Win off macOS (kbd.rs, Kbd::format).
#[gpui::test]
fn the_kbds_take_the_showcases_modifier(cx: &mut TestAppContext) {
    let os = cx.update(|cx| cx.platform_info().os);
    for &(id, keys) in &crate::pages::typography::KBDS {
        let stroke = gpui::Keystroke::parse_with_os(keys, os)
            .unwrap_or_else(|e| panic!("{id}: {keys:?} is no keystroke: {e}"));
        assert!(
            keys.starts_with("secondary-"),
            "{id}: {keys:?} is not on the showcase's modifier, secondary"
        );
        let drawn = gpui_component::kbd::Kbd::format(&stroke);
        // Cmd is drawn ⌘ on macOS, and Control ⌃ (kbd.rs, Kbd::format).
        if cfg!(target_os = "macos") {
            assert!(
                drawn.contains('⌘') && !drawn.contains('⌃'),
                "{id}: {keys:?} is drawn as {drawn:?}, not on Cmd"
            );
        } else {
            assert!(
                drawn.starts_with("Ctrl+") && !drawn.contains("Win"),
                "{id}: {keys:?} is drawn as {drawn:?}, not on Ctrl"
            );
        }
    }
}

/// The Typography page draws its four Kbds: each of `KBDS` is laid out, so
/// a `demo::kbd` whose keys do not parse, and which is then left out, fails
/// here. `the_kbds_take_the_showcases_modifier` parses `KBDS` itself and
/// would not see that.
#[gpui::test]
fn the_typography_page_draws_its_kbds(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Typography);
    let missing: Vec<&str> = crate::pages::typography::KBDS
        .iter()
        .map(|&(id, _)| id)
        .filter(|&id| cx.debug_bounds(id).is_none())
        .collect();
    assert!(
        missing.is_empty(),
        "the Typography page did not lay out these Kbds: {missing:?}"
    );
}

/// The Feedback page's shortcut tooltip names its keystroke as a Kbd draws
/// it on this platform, on the showcase's modifier: "Ctrl+S" off macOS,
/// "⌘S", Cmd's glyph, on it.
#[gpui::test]
fn the_shortcut_tooltip_names_its_keystroke(cx: &mut TestAppContext) {
    use crate::pages::feedback::{SAVE_KEYS, save_tooltip};
    let (os, tooltip) = cx.update(|cx| (cx.platform_info().os, save_tooltip(cx)));
    let stroke = gpui::Keystroke::parse_with_os(SAVE_KEYS, os).expect("SAVE_KEYS is a keystroke");
    assert!(
        SAVE_KEYS.starts_with("secondary-"),
        "{SAVE_KEYS:?} is not on the showcase's modifier, secondary"
    );
    let drawn = gpui_component::kbd::Kbd::format(&stroke);
    assert_eq!(tooltip, format!("Save file ({drawn})"));
    if cfg!(target_os = "macos") {
        assert_eq!(tooltip, "Save file (⌘S)");
    } else {
        assert_eq!(tooltip, "Save file (Ctrl+S)");
    }
}

/// A Typography Label that inherits its font says what sets its weight
/// and stops there: the note holds no history of the panel or the crate.
#[test]
fn a_labels_weight_note_says_what_sets_the_weight() {
    let t = Theme::default();
    for info in [
        crate::info::text::gallery_label(&t, crate::demo::LabelKind::Plain, "Regular label"),
        crate::info::text::sized_label(&t, crate::demo::TextSize::Base),
    ] {
        let note = shown_note(Some(&info), "font weight").expect("a font weight note");
        assert!(
            note.ends_with("renders at gpui's default weight"),
            "the note does not end at what sets the weight: {note:?}"
        );
        for history in ["used to", "since v", "the panel"] {
            assert!(
                !note.contains(history),
                "the note tells history ({history:?}): {note:?}"
            );
        }
    }
}

/// The chosen set's icons of the gallery's `names` and of `roles` as
/// pixels, as the SF Symbols and Segoe Fluent loaders give every icon
/// (`IconData::Rgba`), decoded as an install decodes them
/// (`Showcase::rebuild_icon_caches`): what macOS and Windows draw, on this
/// platform.
#[cfg(feature = "material-icons")]
fn rasterize_icons(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    names: &[&str],
    roles: &[native_theme::icons::IconRole],
) {
    let pixels = || native_theme::icons::IconData::rgba(24, 24, vec![128; 24 * 24 * 4]);
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            for entry in &mut this.gpui_icons {
                if names.contains(&entry.0) {
                    entry.3 = pixels();
                }
            }
            for entry in &mut this.loaded_icons {
                if roles.contains(&entry.0) {
                    entry.1 = pixels();
                }
            }
            let fg = this.icon_cache_fg;
            this.rebuild_icon_caches(fg, window, cx);
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(cx);
}

/// Where the raster glyph `selector` names was laid out: `raster-` and the
/// icon's name (`demo::raster_selector`).
#[cfg(feature = "material-icons")]
fn raster_bounds(cx: &mut VisualTestContext, selector: &'static str) -> Option<Bounds<Pixels>> {
    cx.debug_bounds(selector)
}

/// A `geometry::icon_size_*` builder.
#[cfg(feature = "material-icons")]
type IconSizeBuilder = fn(native_theme_gpui::Native<'_>) -> gpui_component::Size;

/// The side `builder` sizes an icon to under the installed native theme.
#[cfg(feature = "material-icons")]
fn icon_side(
    cx: &mut VisualTestContext,
    showcase: &Entity<Showcase>,
    builder: IconSizeBuilder,
) -> Pixels {
    match read(cx, showcase, |_this, cx| native_value(cx, builder)) {
        Some(gpui_component::Size::Size(side)) => side,
        other => panic!("the builder gives no size under the installed preset: {other:?}"),
    }
}

/// A set that gives its icons as pixels -- SF Symbols, Segoe Fluent --
/// has them drawn on the Basic page as the Icons page draws them, as
/// images, never the icon's name in its place (M9-163): each icon Button
/// shows its role's image at `toolbar.icon_size`, says it is an icon
/// Button, and each sized folder cell shows its image at its size.
#[cfg(feature = "material-icons")] // the bundled set the pixels stand in for
#[gpui::test]
fn a_raster_sets_icons_are_drawn_as_images_on_the_basic_page(cx: &mut TestAppContext) {
    use native_theme::icons::IconRole;
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "macos-tahoe");
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    // Each Button, the element recording its icon, its role, and the
    // selector of its image (`raster-` and `IconRole::name`).
    let buttons = [
        (
            "basic-icon-copy",
            "basic.icons.copy.icon",
            IconRole::ActionCopy,
            "raster-action-copy",
        ),
        (
            "basic-icon-paste",
            "basic.icons.paste.icon",
            IconRole::ActionPaste,
            "raster-action-paste",
        ),
        (
            "basic-icon-delete",
            "basic.icons.delete.icon",
            IconRole::ActionDelete,
            "raster-action-delete",
        ),
    ];
    let mut roles: Vec<IconRole> = buttons.iter().map(|(_, _, role, _)| *role).collect();
    roles.push(IconRole::FolderOpen);
    rasterize_icons(&mut cx, &showcase, &[], &roles);
    let toolbar = icon_side(&mut cx, &showcase, geometry::icon_size_toolbar);
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    for (button, listed, role, selector) in buttons {
        assert_eq!(
            format!("raster-{}", role.name()),
            selector,
            "the image's selector is not raster- and the role's name"
        );
        let icon = drawn.get(listed).copied();
        assert_eq!(
            icon.map(|b| (b.size.width, b.size.height)),
            Some((toolbar, toolbar)),
            "{button}: the set's pixels for {} are not drawn as its icon at toolbar.icon_size",
            role.name()
        );
        let image = raster_bounds(&mut cx, selector);
        let frame = bounds_of(&mut cx, button);
        assert!(
            image.is_some_and(|image| within(image, frame)),
            "{button}: no image of {} in the Button at {frame:?}: {image:?}",
            role.name()
        );
        let info = settle_on(&mut cx, &showcase, button);
        assert_eq!(
            info.as_ref().map(|info| info.title()).as_deref(),
            Some("Button · Ghost, icon"),
            "{button} does not say it shows its icon"
        );
    }
    let builders: [(&str, IconSizeBuilder); 3] = [
        ("basic-icon-small", geometry::icon_size_small),
        ("basic-icon-toolbar", geometry::icon_size_toolbar),
        ("basic-icon-large", geometry::icon_size_large),
    ];
    for (cell, builder) in builders {
        let side = icon_side(&mut cx, &showcase, builder);
        let frame = bounds_of(&mut cx, cell);
        assert_eq!(
            (frame.size.width, frame.size.height),
            (side, side),
            "{cell} does not hold the set's FolderOpen image at its size"
        );
    }
}

/// Part A's review, item 13(c): the page row's rule is laid out under the
/// tab strip, so it covers none of the strip's pixels: under macos-tahoe at
/// scale 1, where the rule is one device pixel thick for its 0.5 px
/// `separator.line_width`, its top is the strip's bottom, where a margin of
/// the stated 0.5 and an offset of -0.5 rounded to nothing and laid the rule
/// over the strip's last row.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_page_row_rule_lies_under_the_tab_strip_at_scale_one(cx: &mut TestAppContext) {
    let (showcase, root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "macos-tahoe");
    cx.simulate_window_scale_factor_change(root, 1.0);
    cx.run_until_parked();
    draw(&mut cx);
    let (rule, strip) = read(&mut cx, &showcase, |this, cx| {
        let drawn = this.info_ui.read(cx).layout_drawn();
        (
            drawn.get("chrome.page_tabs.rule").copied(),
            drawn.get("chrome.page_tabs").copied(),
        )
    });
    assert!(
        rule.zip(strip)
            .is_some_and(|(rule, strip)| rule.top() >= strip.bottom() && rule.size.height >= px(1.)),
        "the rule at {rule:?} is not laid out under the tab strip at {strip:?}"
    );
}

/// M9x CI item 12, the controller's ruling (M9-177): no step glyph of the
/// Number input overflows its step. Under macos-tahoe, whose 24 px field
/// holds two steps narrower than `defaults.icon_sizes.small` 16, with the
/// set's ChevronUp and ChevronDown given as pixels (the Expander closed, so
/// its arrows show ChevronRight alone), each step's image lies inside its
/// step and inside the field.
#[cfg(feature = "material-icons")]
#[gpui::test]
fn the_number_inputs_step_glyphs_fit_their_steps(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "macos-tahoe");
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.basic_expanded = [false, false];
            cx.notify();
        })
    });
    rasterize_icons(&mut cx, &showcase, &["ChevronUp", "ChevronDown"], &[]);
    let field = bounds_of(&mut cx, "basic-number");
    let mut faults = Vec::new();
    for (step, image) in [
        ("basic-number-step-Increment", "raster-ChevronUp"),
        ("basic-number-step-Decrement", "raster-ChevronDown"),
    ] {
        let (Some(step_box), Some(glyph)) = (cx.debug_bounds(step), cx.debug_bounds(image)) else {
            faults.push(format!("{step} or {image} is not laid out"));
            continue;
        };
        if !within(glyph, step_box) || !within(glyph, field) {
            faults.push(format!(
                "{image} at {glyph:?} is not inside its step {step_box:?} and the field {field:?}"
            ));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// The Number input's info names the size its step glyphs are drawn at and
/// no other (round 2-3 review, item 15(5)): under macos-tahoe, whose 24 px
/// field leaves each step 11.5 px, the glyphs are a step tall, and no
/// `geometry::icon_size_small` line claims `defaults.icon_sizes.small`'s 16;
/// under material-3, whose 56 px field leaves 27, they are at
/// `geometry::icon_size_small`, which the info names (with `material-icons`,
/// without which material-3's set draws no step glyph).
#[gpui::test]
fn the_number_inputs_info_names_its_step_glyphs_size(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let mut faults = Vec::new();
    for (preset, capped) in [
        ("macos-tahoe", true),
        #[cfg(feature = "material-icons")]
        ("material-3", false),
    ] {
        use_preset(&mut cx, &showcase, preset);
        show(&mut cx, &showcase, Page::Basic);
        let Some(info) = settle_on(&mut cx, &showcase, "basic-number") else {
            faults.push(format!("{preset}: the Number input shows no info"));
            continue;
        };
        let sized = info
            .config
            .iter()
            .any(|n| n.what == "geometry" && n.text.starts_with("geometry::icon_size_small:"));
        let glyphs = info
            .instance
            .iter()
            .find(|n| n.what == "step glyphs")
            .map(|n| n.text.clone());
        let Some(glyphs) = glyphs else {
            faults.push(format!("{preset}: no step glyphs line: {info:?}"));
            continue;
        };
        if sized == capped {
            faults.push(format!(
                "{preset}: the geometry::icon_size_small line is {}, the step glyphs {glyphs:?}",
                if sized { "there" } else { "missing" }
            ));
        }
        let shown = if capped { "11.5px:" } else { "16px:" };
        if !glyphs.starts_with(shown) {
            faults.push(format!(
                "{preset}: the step glyphs line {glyphs:?} is not {shown}"
            ));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// Part A's review, item 13(d): an open expander item's arrow is turned only
/// where it is the closed chevron itself; where the closed chevron is pixels
/// the open item shows the set's chevron of the turned direction, which is
/// not turned again, an SVG one of a mixed set (ChevronRight pixels,
/// ChevronDown an SVG) included, where the turn pointed it left.
#[test]
fn an_open_arrow_turns_only_where_it_is_the_closed_chevron() {
    use crate::demo::expander_arrow_turn;
    for open_turn in [0.25, 0.5] {
        assert_eq!(expander_arrow_turn(false, true, open_turn), open_turn);
        assert_eq!(expander_arrow_turn(false, false, open_turn), 0.);
        assert_eq!(
            expander_arrow_turn(true, true, open_turn),
            0.,
            "a swapped open arrow is turned by {open_turn}"
        );
        assert_eq!(expander_arrow_turn(true, false, open_turn), 0.);
    }
}

/// Part A's review, item 13(a): the icon-theme Select's info says what a set
/// of pixels shows since M9-163: the pixels drawn as images where a widget
/// takes any element, and no icon only in the widgets that take an `Icon`
/// alone; and it cites the page row's menu caret where the showcase lays it
/// in, as the Button's child (`demo::native_tab_bar`, M9-168), not upstream's
/// `RenderOnce for Button`.
#[gpui::test]
fn the_icon_theme_select_says_what_a_raster_set_shows(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let t = cx.update(|_window, cx| Theme::global(cx).clone());
    let info = crate::info::chrome::icon_set_select(&t, None);
    assert!(
        info_says(&info, "pixels as images") && info_says(&info, "a Toggle"),
        "the raster sets' line does not say the pixels are drawn: {:?}",
        info.instance
    );
    assert!(
        !info_says(&info, "show none of their icons"),
        "the raster sets' line still says no icon is shown"
    );
    assert!(
        info_says(&info, "demo::native_tab_bar"),
        "the menu caret's citation is not where the showcase lays it in"
    );
}

/// A set that gives its chevrons as pixels draws them as the Expander's
/// arrows and the Number input's steppers (M9-163): each title row holds an
/// arrow at `expander.arrow_icon_size`, a closed item's the set's
/// ChevronRight under a leading arrow side, and the increment step holds
/// the set's ChevronUp image; their info says they are the set's pixels.
#[cfg(feature = "material-icons")] // the bundled set the pixels stand in for
#[gpui::test]
fn a_raster_sets_chevrons_are_the_expanders_arrows_and_the_steppers(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    // macos-tahoe states a leading arrow (macos-tahoe.toml, expander.arrow_side).
    use_preset(&mut cx, &showcase, "macos-tahoe");
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    let (leading, side, open) = read(&mut cx, &showcase, |this, cx| {
        let e = native_value(cx, |n| {
            (
                n.resolved.expander.arrow_side,
                n.resolved.expander.arrow_icon_size,
            )
        });
        (
            e.map(|(s, _)| s),
            e.map(|(_, size)| size),
            this.basic_expanded,
        )
    });
    assert_eq!(
        leading,
        Some(Some(native_theme::theme::ArrowSide::Leading)),
        "macos-tahoe no longer states a leading arrow, so this case tests another one"
    );
    assert_eq!(
        open,
        [true, false],
        "the Basic expander no longer opens on its first item"
    );
    let side = px(side.expect("a native theme is installed"));
    rasterize_icons(
        &mut cx,
        &showcase,
        &["ChevronRight", "ChevronDown", "ChevronUp"],
        &[],
    );
    for arrow in ["basic-expander-arrow-0", "basic-expander-arrow-1"] {
        let at = cx.debug_bounds(arrow);
        assert_eq!(
            at.map(|b| (b.size.width, b.size.height)),
            Some((side, side)),
            "{arrow}: the set's pixels are not drawn as the arrow at expander.arrow_icon_size"
        );
    }
    let closed = bounds_of(&mut cx, "basic-expander-arrow-1");
    let image = raster_bounds(&mut cx, "raster-ChevronRight");
    assert!(
        image.is_some_and(|image| within(image, closed)),
        "the closed item's arrow at {closed:?} is not the set's ChevronRight image: {image:?}"
    );
    // The open item's arrow is the set's ChevronDown, pixels do not turn
    // (part A's review, item 13(e)): both arrows ChevronRight fails here. The
    // arrow's glyph names the set's image it is in test builds
    // (`demo::native_expander`), as the Number input's steps show a
    // ChevronDown image too.
    let opened = bounds_of(&mut cx, "basic-expander-arrow-0");
    let shows = cx.debug_bounds("basic-expander-arrow-0-ChevronDown");
    assert!(
        shows.is_some_and(|shows| within(shows, opened)),
        "the open item's arrow at {opened:?} does not show the set's ChevronDown image: {shows:?}"
    );
    let number = bounds_of(&mut cx, "basic-number");
    let up = raster_bounds(&mut cx, "raster-ChevronUp");
    assert!(
        // Its centre: a `defaults.icon_sizes.small` glyph is taller than a
        // step, half the field (`demo::number_input`), as an SVG one is.
        up.is_some_and(|up| number.contains(&up.center())),
        "the Number input at {number:?} holds no ChevronUp image: {up:?}"
    );
    let info = settle_on(&mut cx, &showcase, "basic-number");
    assert!(
        info.as_ref()
            .is_some_and(|info| info_says(info, "material gives it as pixels, not an SVG")),
        "the Number input's info does not say its chevrons are the set's pixels: {info:?}"
    );
}

/// A set that gives its icons as pixels has them drawn in the chrome as
/// images (M9-163): the toolbar's Preferences button shows the set's
/// Settings image, the status bar's toggle its PanelLeft, and the page
/// row's menu its ChevronDown, each where the chrome's elements record the
/// icon, at the size its builder gives -- not the button's name.
#[cfg(feature = "material-icons")] // the bundled set the pixels stand in for
#[gpui::test]
fn a_raster_sets_icons_are_drawn_as_images_in_the_chrome(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "windows-11");
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    rasterize_icons(
        &mut cx,
        &showcase,
        &["Settings", "PanelLeft", "ChevronDown"],
        &[],
    );
    // A page that draws no ChevronDown of its own, so the page row's menu
    // holds the one image of it.
    show(&mut cx, &showcase, Page::Charts);
    let toolbar = icon_side(&mut cx, &showcase, geometry::icon_size_toolbar);
    let small = icon_side(&mut cx, &showcase, geometry::icon_size_small);
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    for (listed, side, frame, name) in [
        (
            "chrome.toolbar.preferences.icon",
            toolbar,
            CHROME_TOOLBAR_PREFERENCES,
            "raster-Settings",
        ),
        (
            "chrome.status_bar.toggle.icon",
            small,
            CHROME_SIDE_PANEL_TOGGLE,
            "raster-PanelLeft",
        ),
        (
            "chrome.page_tabs.menu.icon",
            small,
            CHROME_PAGE_TABS,
            "raster-ChevronDown",
        ),
    ] {
        let icon = drawn.get(listed).copied();
        assert_eq!(
            icon.map(|b| (b.size.width, b.size.height)),
            Some((side, side)),
            "{listed}: the set's {name} pixels are not drawn as the icon at its size"
        );
        let frame = bounds_of(&mut cx, frame);
        let image = raster_bounds(&mut cx, name);
        assert!(
            image.is_some_and(|image| within(image, frame)),
            "{listed}: no {name} image at {frame:?}: {image:?}"
        );
    }
    let info = settle_on(&mut cx, &showcase, CHROME_TOOLBAR_PREFERENCES);
    assert_eq!(
        info.as_ref().map(|info| info.title()).as_deref(),
        Some("Button · Ghost, icon"),
        "the Preferences button with the set's Settings pixels is not an icon Button"
    );
}

/// The other pages draw a raster set's icons as images wherever their
/// widget takes any element -- the Inputs page's search addon, the Data
/// page's Attachment, the Feedback page's Empty state and Plain Marker, the
/// Overlays page's menu rows, the Buttons page's Toolbar and the Icons
/// page's sizes -- and show what they show without an icon where the
/// widget takes an `Icon` alone, saying the set holds no SVG for it
/// (M9-163): the Buttons page's Toggle, the Inputs page's Copy button.
#[cfg(feature = "material-icons")] // the bundled set the pixels stand in for
#[gpui::test]
fn a_raster_sets_icons_on_the_other_pages(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    without_motion(&mut cx);
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    rasterize_icons(
        &mut cx,
        &showcase,
        &[
            "Search",
            "Copy",
            "Inbox",
            "CircleCheck",
            "Undo",
            "Delete",
            "Star",
            "Folder",
        ],
        &[],
    );
    for (page, frame, name) in [
        (Page::Inputs, "inputs-input-groups", "raster-Search"),
        (Page::Data, "data-attachment-complete", "raster-Inbox"),
        (Page::Feedback, "feedback-empty", "raster-Inbox"),
        (
            Page::Feedback,
            "feedback-marker-plain",
            "raster-CircleCheck",
        ),
        (Page::Overlays, "overlays-menu-row-undo", "raster-Undo"),
        (Page::Overlays, "overlays-menu-row-delete", "raster-Delete"),
        (Page::Buttons, "buttons-toolbar-copy", "raster-Copy"),
        (Page::Icons, "icons-size-panel", "raster-Folder"),
    ] {
        show(&mut cx, &showcase, page);
        let frame_at = bounds_of(&mut cx, frame);
        let image = raster_bounds(&mut cx, name);
        assert!(
            // Its centre: a Marker's icon box reaches past the row's start
            // (marker.rs, MarkerIcon), for an SVG icon too.
            image.is_some_and(|image| frame_at.contains(&image.center())),
            "{frame}: no {name} image at {frame_at:?}: {image:?}"
        );
    }
    for (page, frame, name) in [
        (Page::Buttons, "buttons-toggle-star", "Star"),
        (Page::Inputs, "inputs-input-groups", "Copy"),
    ] {
        show(&mut cx, &showcase, page);
        let info = settle_on(&mut cx, &showcase, frame);
        let start = format!("none: material holds no SVG for {name}");
        assert!(
            info.as_ref().is_some_and(|info| info_says(info, &start)),
            "{frame}, which takes an Icon alone, does not say {start:?}: {info:?}"
        );
    }
}

/// The window's own frame, the title bar macOS draws, takes the mode the
/// installed theme is drawn in (M9-164): the showcase asks gpui for that
/// appearance when it opens and on every install, a mode switch or a
/// preset switch, so a dark variant's title bar is not left light.
#[gpui::test]
fn the_window_frame_takes_the_installed_themes_mode(cx: &mut TestAppContext) {
    use gpui::WindowAppearance;
    use gpui_component::ActiveTheme as _;
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let drawn = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            let expected = if cx.theme().mode.is_dark() {
                WindowAppearance::Dark
            } else {
                WindowAppearance::Light
            };
            (this.window_appearance_asked, expected)
        })
    };
    let (asked, expected) = drawn(&mut cx);
    assert_eq!(
        asked,
        Some(expected),
        "the window opened without the theme's mode"
    );
    for (preset, mode) in [
        ("kde-breeze", AppColorMode::Dark),
        ("kde-breeze", AppColorMode::Light),
        ("windows-11", AppColorMode::Dark),
        ("macos-tahoe", AppColorMode::Light),
    ] {
        cx.update(|window, cx| {
            showcase.update(cx, |this, cx| {
                this.install_in_mode(preset, mode, window, cx)
            });
        });
        cx.run_until_parked();
        draw(&mut cx);
        let (asked, expected) = drawn(&mut cx);
        assert_eq!(
            expected,
            if mode == AppColorMode::Dark {
                WindowAppearance::Dark
            } else {
                WindowAppearance::Light
            },
            "{preset} in {mode:?} was not drawn in that mode"
        );
        assert_eq!(
            asked,
            Some(expected),
            "{preset} in {mode:?}: the window's frame was not given the theme's mode"
        );
    }
}

/// Under macos-tahoe, whose `separator.line_width` is 0.5, on a 1x display
/// -- the macOS runners' -- the Basic page's Separator and the page row's
/// rule are one device pixel thick, as gpui draws the preset's 0.5 px
/// frames, where they took no room and painted nothing (M9-165).
#[cfg(feature = "widgets")]
#[gpui::test]
fn a_hairline_separator_is_drawn_at_scale_one(cx: &mut TestAppContext) {
    let (showcase, root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "macos-tahoe");
    let width = read(&mut cx, &showcase, |_this, cx| {
        native_value(cx, |n| n.resolved.separator.line_width)
    });
    assert!(
        width.is_some_and(|w| w > 0.0 && w < 1.0),
        "macos-tahoe's separator.line_width is {width:?}, not under one pixel"
    );
    cx.simulate_window_scale_factor_change(root, 1.0);
    cx.run_until_parked();
    draw(&mut cx);
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    for line in ["basic.separator.line", "chrome.page_tabs.rule"] {
        assert_eq!(
            drawn.get(line).map(|b| b.size.height),
            Some(px(1.)),
            "{line} is not one device pixel thick at scale 1"
        );
    }
}

/// M9x CI items 11 and 13(b) (M9-165): under macos-tahoe at scale 1 the
/// Basic page's Separator, the page row's rule and the Basic table's two grid
/// lines are painted as filled quads at least one device pixel thick, which
/// every gpui renderer draws, where the grid lines were boxes
/// `separator.line_width` 0.5 thick that took no room and the two lines a
/// border one device pixel thick that no renderer shades (the connector's
/// `a_separator_is_painted_as_a_filled_line_at_scale_one` gives the shaders'
/// lines).
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_hairlines_are_filled_quads_at_scale_one(cx: &mut TestAppContext) {
    let (showcase, root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "macos-tahoe");
    cx.simulate_window_scale_factor_change(root, 1.0);
    cx.run_until_parked();
    draw(&mut cx);
    let (grid, line) = read(&mut cx, &showcase, |_this, cx| {
        (
            native_value(cx, |n| crate::info::stated(n.resolved.list.grid_color)),
            native_value(cx, |n| crate::info::stated(n.resolved.separator.line_color)),
        )
    });
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    let table = drawn.get("basic.table.frame").copied();
    let mut filled = |colour: Option<gpui::Hsla>, within: Option<Bounds<Pixels>>| {
        cx.update(|window, _| {
            let (Some(colour), Some(within)) = (colour, within) else {
                return 0;
            };
            let within = within.scale(window.scale_factor());
            scene_quads(window)
                .into_iter()
                .filter(|q| {
                    q.background.as_solid() == Some(colour)
                        && q.bounds.size.width.0.min(q.bounds.size.height.0) >= 1.0
                        && q.bounds.intersects(&within)
                })
                .count()
        })
    };
    let grid_lines = filled(grid, table);
    // The header's line and a column line in each row (header and rows).
    assert!(
        grid_lines >= 2,
        "the table's grid lines: {grid_lines} filled quads"
    );
    for id in ["basic.separator.line", "chrome.page_tabs.rule"] {
        let n = filled(line, drawn.get(id).copied());
        assert!(n >= 1, "{id}: no filled quad of separator.line_color");
    }
    // The in-window menu's separators, `widgets::Separator`s in
    // menu.separator_color (part B's review, item 14(c)).
    let opened = cx.update(|window, cx| {
        let menus = showcase.read(cx).menus.clone();
        menus.update(cx, |m, cx| m.open_named("theme", false, window, cx))
    });
    assert!(opened.is_ok(), "the Theme menu does not open: {opened:?}");
    cx.run_until_parked();
    draw(&mut cx);
    let menu_line = read(&mut cx, &showcase, |_this, cx| {
        native_value(cx, |n| crate::info::stated(n.resolved.menu.separator_color))
    });
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    let separator = drawn.get("chrome.menu.theme.separator_1").copied();
    let n = cx.update(|window, _| {
        let (Some(colour), Some(within)) = (menu_line, separator) else {
            return 0;
        };
        let within = within.scale(window.scale_factor());
        scene_quads(window)
            .into_iter()
            .filter(|q| {
                q.background.as_solid() == Some(colour)
                    && q.bounds.size.width.0.min(q.bounds.size.height.0) >= 1.0
                    && q.bounds.intersects(&within)
            })
            .count()
    });
    assert!(
        n >= 1,
        "chrome.menu.theme.separator_1 at {separator:?}: no filled quad of menu.separator_color"
    );
}

/// Under windows-11's metrics, where the chosen set has no ChevronDown the
/// page row's menu button still shows upstream's caret (M9-168): the
/// button holds the caret beside the theme's `button.border.padding`, where
/// the caret-only Button, laid out as an icon Button of a fixed side
/// (gpui-component 0.8.1 `button/button.rs:684-691`, `size_5`), gave the
/// stated side padding the whole of that side and the caret none.
#[cfg(all(feature = "widgets", feature = "material-icons"))]
#[gpui::test]
fn the_page_menu_button_shows_its_caret_without_the_sets_chevron(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "windows-11");
    choose_icon_theme(&mut cx, &showcase, "Material (bundled)");
    remove_icons(&mut cx, &showcase, &["ChevronDown"]);
    let (padding, rem, menu) = read(&mut cx, &showcase, |this, cx| {
        let padding = native_value(cx, |n| {
            let p = &n.resolved.button.border.padding;
            (p.left, p.right)
        });
        (
            padding,
            gpui_component::ActiveTheme::theme(cx).font_size,
            this.info_ui
                .read(cx)
                .layout_drawn()
                .get("chrome.page_tabs.menu")
                .copied(),
        )
    });
    let Some((Some(left), Some(right))) = padding else {
        panic!("windows-11 no longer states the button's side padding: {padding:?}");
    };
    let menu = menu.expect("the page row's menu button is drawn");
    // The caret is an XSmall Icon, `size_3` (select.rs, `Caret`; icon.rs,
    // `Icon::into_svg`), a quarter less than the rem.
    let caret = rem * 0.75;
    assert!(
        menu.size.width >= px(left + right) + caret - px(0.01),
        "the menu button at {menu:?} leaves the caret {:?} of its {}px side padding, not {caret:?}",
        menu.size.width - px(left + right),
        left + right
    );
}

/// M9x CI item 9, the controller's rulings (M9-166, and item 15(1) of the
/// round 2-3 review): the Basic page's column count holds the Segmented
/// control's own width, so the control, on one row, may reach into the
/// section gap but keeps `layout.widget_gap`, the gap the page's rows keep
/// between neighbouring widgets, from the next column's left edge, and in the
/// last column may reach into the page's right margin but keeps
/// `layout.widget_gap` from the scroll area's edge, under every bundled preset at a
/// text scale of 1 and 2.25: under macos-monterey and macos-tahoe at 1 the
/// shared count left it 9 and 13 px into the next column, and at 2.25 three
/// columns, the old floor, left it past the window under macos-monterey and
/// ios-13. The four Linux parity presets keep their columns at 1: kde-breeze,
/// adwaita and catppuccin-mocha five, material-3 four. It prints each
/// preset's count.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_column_count_holds_the_segmented_control(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let mut faults = Vec::new();
    // The four Linux parity presets at 1; and catppuccin-mocha at 2.25, whose
    // control ends a pixel past the page in the last of three columns, inside
    // the right margin (the round 4 review's ruling, item 17 minor 9).
    let kept = [
        ("kde-breeze", 1.0, 5),
        ("adwaita", 1.0, 5),
        ("catppuccin-mocha", 1.0, 5),
        ("material-3", 1.0, 4),
        ("catppuccin-mocha", 2.25, 3),
    ];
    for preset in native_theme::theme::Theme::list_presets()
        .iter()
        .map(|p| p.key)
    {
        for scale in [1.0, 2.25] {
            use_preset_scaled(&mut cx, &showcase, preset, Some(96.0), scale);
            show(&mut cx, &showcase, Page::Basic);
            let (gap, margin) = read(&mut cx, &showcase, |this, _| {
                (
                    this.layout.widget_gap.unwrap_or_default(),
                    this.layout.window_margin.unwrap_or_default(),
                )
            });
            // A floating bar is drawn over the right margin, its stated groove wide.
            let floating = read(&mut cx, &showcase, |_, cx| {
                native_value(cx, |n| {
                    let bar = &n.resolved.scrollbar;
                    if bar.overlay_mode {
                        bar.groove_width
                    } else {
                        0.0
                    }
                })
            })
            .unwrap_or_default();
            let drawn = read(&mut cx, &showcase, |this, cx| {
                this.info_ui.read(cx).layout_drawn()
            });
            let laid: Vec<Bounds<Pixels>> = drawn
                .iter()
                .filter(|(id, _)| id.starts_with("basic.column_"))
                .map(|(_, b)| *b)
                .collect();
            println!("gpui {preset} at {scale}: {} columns", laid.len());
            let want = kept
                .iter()
                .find(|&&(name, at, _)| name == preset && at == scale)
                .map(|&(_, _, n)| n);
            if let Some(want) = want
                && laid.len() != want
            {
                faults.push(format!(
                    "{preset} at {scale}: {} columns, not {want}",
                    laid.len()
                ));
            }
            let (Some(control), Some(page)) = (
                drawn.get("basic.segmented.control").copied(),
                drawn.get("basic.page").copied(),
            ) else {
                faults.push(format!("{preset} at {scale}: no segmented control or page"));
                continue;
            };
            let Some(column) = laid
                .iter()
                .find(|c| c.left() <= control.left() + px(0.5) && control.left() < c.right())
            else {
                faults.push(format!("{preset} at {scale}: the control is in no column"));
                continue;
            };
            let next = laid
                .iter()
                .map(|c| c.left())
                .filter(|left| *left > column.left() + px(0.5))
                .min();
            match next {
                Some(next) if control.right() > next - px(gap) + px(0.5) => {
                    faults.push(format!(
                        "{preset} at {scale}: the control's right edge {:?} is within \
                         layout.widget_gap {gap} of the next column at {next:?}",
                        control.right()
                    ));
                }
                None if control.right()
                    > page.right() + px(margin) - px(floating) - px(gap) + px(0.5) =>
                {
                    faults.push(format!(
                        "{preset} at {scale}: the control in the last column ends at {:?}, \
                         within layout.widget_gap {gap} of the scroll area's edge or a floating \
                         bar {floating} wide, the page's {:?} and its {margin} margin",
                        control.right(),
                        page.right()
                    ));
                }
                _ => {}
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// The width the Basic page's column count holds for the Segmented control,
/// `demo::segmented_width`, is the width gpui lays the control out at: each
/// label's shaped width rounded up, as gpui's text element rounds a line's
/// width up (gpui-pre 0.3.10 `src/elements/text.rs:772`), with its padding,
/// the lines between the segments and the frame's two edges (round 2-3
/// review, item 15(3)).
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_segmented_width_is_the_drawn_controls(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let mut faults = Vec::new();
    for preset in [
        "macos-monterey",
        "kde-breeze",
        "catppuccin-mocha",
        "windows-11",
    ] {
        for scale in [1.0, 2.25] {
            use_preset_scaled(&mut cx, &showcase, preset, Some(96.0), scale);
            draw(&mut cx);
            let drawn = read(&mut cx, &showcase, |this, cx| {
                this.info_ui.read(cx).layout_drawn()
            });
            let Some(control) = drawn.get("basic.segmented.control").copied() else {
                faults.push(format!("{preset} at {scale}: no segmented control"));
                continue;
            };
            let counted = cx.update(|window, cx| {
                native_value(cx, |n| {
                    crate::demo::segmented_width(window, &n, &crate::pages::basic::BASIC_SEGMENTS)
                })
            });
            let Some(counted) = counted else {
                faults.push(format!("{preset} at {scale}: no native theme"));
                continue;
            };
            let laid = control.size.width.as_f32();
            if (laid - counted).abs() > 0.01 {
                faults.push(format!(
                    "{preset} at {scale}: the control is {laid} wide, the count holds {counted}"
                ));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// M9x CI item 9 (M9-166): the Basic Segmented control keeps its segments on
/// one row, as the platforms' segmented controls do (NSSegmentedControl,
/// libadwaita's `AdwToggleGroup`), where its column is narrower than they
/// are: under macos-monterey, whose 170 px columns are a few px narrower
/// than its three segments, the CI capture wrapped Month onto a second row.
/// Day, Week and Month lie at one y, the control one segment row tall.
#[cfg(feature = "widgets")]
#[gpui::test]
fn the_segmented_control_keeps_its_segments_on_one_row(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "macos-monterey");
    draw(&mut cx);
    let drawn = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).layout_drawn()
    });
    let at = |id: &str| drawn.get(id).copied();
    let found = (
        at("basic.segmented.control"),
        at("basic.segmented.day"),
        at("basic.segmented.week"),
        at("basic.segmented.month"),
    );
    assert!(
        found.0.is_some() && found.1.is_some() && found.2.is_some() && found.3.is_some(),
        "the segmented control is not recorded: {found:?}"
    );
    let (Some(control), Some(day), Some(week), Some(month)) = found else {
        return;
    };
    assert!(
        day.origin.y == week.origin.y && week.origin.y == month.origin.y,
        "the segments are not on one row: {day:?} {week:?} {month:?}"
    );
    let row = day.size.height.max(week.size.height).max(month.size.height);
    assert!(
        control.size.height <= row + px(2.),
        "the control {control:?} is taller than one row of segments, {row:?}"
    );
}
