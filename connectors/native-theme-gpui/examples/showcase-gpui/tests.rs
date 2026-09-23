//! Showcase self-tests (spec v0.5.9 §6.1)
//!
//! `test = true` on the example target (Cargo.toml) puts these under a plain
//! `cargo test`, so CI and the nightly dependency canary run them with no
//! workflow change. They build the real `Showcase` on GPUI's headless test
//! platform — the same view, the same `Root`, the same window width `main`
//! opens — and drive it with real input.

use gpui::{
    App, Bounds, Entity, Focusable as _, Modifiers, MouseButton, Pixels, Point, TestAppContext,
    VisualTestContext, point, prelude::*, px, size,
};
use gpui_base::ScrollbarHandle as _;
use gpui_component::{Root, theme::Theme};
use native_theme_gpui::{ActiveNativeTheme, geometry};
use std::cell::RefCell;
use std::ops::Deref as _;
use std::rc::Rc;

use crate::app::{
    AppColorMode, OpenCommandPalette, OpenPreferences, Quit, SetColorMode, ShowPage, Showcase,
    ToggleInspector, ToggleSidebar,
};
use crate::chrome::menus;
use crate::info::{
    GEOMETRY_NOTES, INFO_SETTLE, InfoExt as _, InfoRegistry, WidgetInfo, claim, epoch_marker,
    hsla_to_hex, native_info,
};
use crate::inspector::InspectorTab;
use crate::support::{CAROUSEL_SLIDES, native_geometry, native_value};
use crate::{
    BUTTONS_DANGER, BUTTONS_DISABLED_SECONDARY, BUTTONS_HEADING_VARIANTS, BUTTONS_PRIMARY,
    BUTTONS_TEXT, CHROME_APP_MENU_BAR, CHROME_HANDLE_INSPECTOR, CHROME_HANDLE_NAV, CHROME_SIDEBAR,
    CHROME_SIDEBAR_TOGGLE, CHROME_STATUS_BAR, CHROME_TITLE_BAR, CHROME_TOOLBAR,
    CHROME_TOOLBAR_INSPECTOR, CHROME_TOOLBAR_PALETTE, CONTENT_ALERT, CONTENT_PANEL, CONTENT_SCROLL,
    DATA_PAGINATION, DATA_PAGINATION_COMPACT, DATA_TABLE_HEADER, INPUTS_CHECKBOX_AUTOSAVE,
    INPUTS_CHECKBOX_NOTIFICATIONS, INPUTS_FIELD, INPUTS_FIELD_HEIGHT_ONLY, INSPECTOR_COPY,
    INSPECTOR_PANEL, INSPECTOR_TABS, INSPECTOR_TITLE, INSPECTOR_WIDTH, LIST_DEMO, NAV_WIDTH,
    OVERLAY_ABOUT_LINK, OVERLAY_ABOUT_NAME, OVERLAY_ABOUT_TEXT, OVERLAY_PALETTE,
    OVERLAY_PALETTE_TITLE, OVERLAY_PREFERENCES, PAGE_ROOT, PAGE_WIDTH_PX, PREF_REDUCE_MOTION,
    PROBE_ALERT_DIALOG, PROBE_ATTACHMENT, PROBE_CAROUSEL_LAST, PROBE_CHAT_SEND, PROBE_CLIPBOARD,
    PROBE_COLOR_MODE, PROBE_COMBOBOX, PROBE_NOTIFICATION, PROBE_PAGINATION, PROBE_RATING,
    PROBE_SETTINGS_ROW, PROBE_STEPPER, Page, STATUS_HOVERED, TREE_DEMO, WINDOW_SIZE,
};

/// The window the interaction test lays the showcase out in.
///
/// The width is the application's own. The height is not: a page is one
/// long scrolling column, and an element scrolled out of the viewport is
/// clipped out of the frame and cannot be clicked, so this window is tall
/// enough to hold the longest page whole. `every_page_lays_out` uses
/// `WINDOW_SIZE` instead, which is what puts the scroll container to work.
const TALL_WINDOW: gpui::Size<Pixels> = size(WINDOW_SIZE.width, px(9000.));

/// Build the showcase the way `main` does — `gpui_kit::init`, the showcase's
/// actions and key bindings, the window options, the view, and the `Root`
/// that owns the dialog and notification layers — in a window of
/// `window_size`.
///
/// One thing `main` has that a test window cannot: the asset source.
/// `gpui_kit::application().with_assets(gpui_kit::assets::Assets)` has no
/// counterpart here — `TestAppContext::build` hands the app `Arc::new(())`
/// and exposes no setter (`gpui-pre-0.3.5/src/app/test_context.rs:132-136`)
/// — so gpui-component's own `IconName` SVGs resolve to nothing. The pages
/// still lay out, which is what these tests measure; the showcase's native
/// icons do not come from the asset source at all, they are decoded into
/// `ImageSource` by the connector.
fn open(
    cx: &mut TestAppContext,
    window_size: gpui::Size<Pixels>,
) -> (Entity<Showcase>, Entity<Root>, VisualTestContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        crate::app::init(cx);
    });
    let view: Rc<RefCell<Option<Entity<Showcase>>>> = Rc::new(RefCell::new(None));
    let options = crate::window_options(Bounds {
        origin: Point::default(),
        size: window_size,
    });
    let handle = cx
        .update(|cx| {
            cx.open_window(options, {
                let view = view.clone();
                move |window, cx| {
                    let showcase = cx.new(|cx| Showcase::new(window, cx));
                    *view.borrow_mut() = Some(showcase.clone());
                    cx.new(|cx| Root::new(showcase, window, cx))
                }
            })
        })
        .expect("the window opened");
    let root = handle.root(cx).expect("the root view was built");
    let showcase = view.borrow_mut().take().expect("the showcase was built");
    let mut cx = VisualTestContext::from_window(*handle.deref(), cx);
    cx.run_until_parked();
    draw(&mut cx);
    (showcase, root, cx)
}

/// Lay the window out and paint it, which is what fills `debug_bounds`.
fn draw(cx: &mut VisualTestContext) {
    cx.update(|window, cx| window.draw(cx).clear(cx));
}

/// Switch to `page` and draw the frame that shows it.
fn show(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, page: Page) {
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| this.show_page(page, cx));
    });
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

/// Every page lays out: the Sidebar's ten pages each render on the test
/// platform, each leaves a page root behind, and that root has a size.
#[gpui::test]
fn every_page_lays_out(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(Page::ALL.len(), 10, "the Sidebar no longer has ten pages");
    for page in Page::ALL {
        show(&mut cx, &showcase, page);
        assert_eq!(read(&mut cx, &showcase, |this, _| this.active_page), page);
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
/// desktop the test runs on.
fn use_preset(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, preset: &str) {
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.apply_theme_by_name(preset, window, cx);
            cx.notify();
        });
    });
    cx.run_until_parked();
    draw(cx);
}

/// The installed theme's scrollbar groove width, and whether the platform
/// draws its scrollbars as overlays; a zero width means no native theme is
/// installed, which the caller asserts against.
fn scrollbar_of(cx: &mut VisualTestContext, showcase: &Entity<Showcase>) -> (Pixels, bool) {
    read(cx, showcase, |_this, cx| {
        match cx.native_theme().and_then(|nt| nt.native(cx)) {
            Some(n) => (
                px(native_theme_gpui::scrollbar_width(n.resolved)),
                n.resolved.scrollbar.overlay_mode,
            ),
            None => (px(0.), false),
        }
    })
}

/// A scrollbar that is not an overlay keeps off the content.
///
/// gpui-component overlays its scrollbar on the scroll area whatever the
/// platform does (`src/scroll/scrollable.rs`, `Scrollable`), and gpui-base
/// draws the vertical track flush with the area's right edge, at the width
/// the connector installed (gpui-base `src/scrollbar.rs:1408-1432`). Where
/// the platform's scrollbars are not overlays the connector also asks for
/// an always-visible bar (`src/lib.rs`, `scrollbar_mode`), so that width
/// has to be reserved beside the content or the bar sits on it.
#[gpui::test]
fn a_non_overlay_scrollbar_keeps_off_the_content(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    // kde-breeze puts its groove beside the content; macOS overlays its
    // own, and must not lose a strip of width to it.
    for preset in ["kde-breeze", "macos-sonoma"] {
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

/// A Settings row keeps off the page's scrollbar.
///
/// The page body lays gpui-component's `ScrollbarLayer` over its right edge
/// (setting/page.rs:222-248, scroll/scrollable.rs:19-29) and reserves only its
/// own `px_4`; kde-breeze's groove is 21px, wider than that, so without a
/// gutter on the group the bar covers the rows.
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
    cx.simulate_event(gpui::ScrollWheelEvent {
        position: at,
        delta: gpui::ScrollDelta::Pixels(point(px(0.), delta)),
        modifiers: Modifiers::default(),
        touch_phase: gpui::TouchPhase::Moved,
    });
    cx.run_until_parked();
    draw(cx);
}

/// A window tall enough to show the Data page's List and Tree demos without
/// scrolling the page first, and still far shorter than that page, so the
/// page has somewhere to scroll to.
const NESTED_SCROLL_WINDOW: gpui::Size<Pixels> = size(WINDOW_SIZE.width, px(1500.));

/// A wheel turned inside the List or the Tree scrolls that widget, not the
/// page under it.
///
/// gpui's scroll listener runs in the bubble phase and stops at no one:
/// every scroll container whose hitbox is under the pointer moves by the
/// same delta (gpui-pre `src/elements/div.rs`, `paint_scroll_listener`,
/// and `Hitbox::should_handle_scroll`). A nested scroller therefore has to
/// take the pointer out of the ancestors' hit test, which is what
/// `occlude` does (gpui-pre `src/window.rs`, `HitboxBehavior::BlockMouse`).
#[gpui::test]
fn a_nested_scroller_keeps_the_wheel_to_itself(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, NESTED_SCROLL_WINDOW);
    // material's list rows are the tallest of the bundled presets, so the
    // six sample rows are certain to overflow the demo box.
    use_preset(&mut cx, &showcase, "material");
    show(&mut cx, &showcase, Page::Data);
    assert!(
        bounds_of(&mut cx, PAGE_ROOT).size.height > NESTED_SCROLL_WINDOW.height,
        "the Data page fits in the window, so the page could not scroll either way"
    );

    // How far each demo's own scroller has been scrolled, so the step can
    // tell "the page stayed put because the list took the wheel" from
    // "nothing moved at all".
    type Offset = fn(&Showcase, &App) -> Pixels;
    let list_offset: Offset = |this, cx| this.list_state.read(cx).scroll_handle().offset().y;
    let tree_offset: Offset = |this, cx| this.tree_state.read(cx).scroll_handle().offset().y;

    for (selector, inner) in [(LIST_DEMO, list_offset), (TREE_DEMO, tree_offset)] {
        let demo = bounds_of(&mut cx, selector);
        assert!(
            demo.top() > px(0.) && demo.bottom() < NESTED_SCROLL_WINDOW.height,
            "{selector}: is not on screen, it sits at {demo:?}"
        );
        let page_before = bounds_of(&mut cx, CONTENT_SCROLL).top();
        let inner_before = read(&mut cx, &showcase, inner);
        scroll_at(&mut cx, demo.center(), -demo.size.height / 4.);
        let page_after = bounds_of(&mut cx, CONTENT_SCROLL).top();
        let inner_after = read(&mut cx, &showcase, inner);
        assert_ne!(
            inner_after, inner_before,
            "{selector}: its own scroller did not move, so this step proves nothing"
        );
        assert_eq!(
            page_after, page_before,
            "{selector}: the wheel moved the page from {page_before:?} to {page_after:?} \
                 while the widget under the pointer still had room to scroll"
        );
    }
}

/// Table, List and Tree are framed alike, and by the list theme.
///
/// The `DataTable` draws a frame of its own from `Theme::radius` and
/// `Theme::border` (gpui-component `src/table/data_table.rs:167-171`); the
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
    for preset in ["material", "windows-11"] {
        use_preset(&mut cx, &showcase, preset);
        let native = read(&mut cx, &showcase, |_this, cx| {
            let b = |f: fn(&native_theme::theme::ResolvedBorderSpec) -> f32| {
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
    let (showcase, root, mut cx) = open(cx, TALL_WINDOW);

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

    // Notification: the button pushes one onto the Root's own layer.
    let before = cx.update(|_w, cx| root.read(cx).notification.read(cx).notifications().len());
    click(&mut cx, PROBE_NOTIFICATION);
    assert_eq!(
        cx.update(|_w, cx| root.read(cx).notification.read(cx).notifications().len()),
        before + 1,
        "Notification: nothing was pushed"
    );

    // --- The toolbar's colour mode switch -----------------------------
    //
    // The group's toggles are System, Light, Dark, and a Toggle carries no
    // selector of its own, so the step clicks the group's two ends. Dark is
    // clicked from light, so the theme's mode always has to change: on a
    // dark desktop, System already is dark, and the Theme menu's Light sets
    // the start. Then System, back from Dark.
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        AppColorMode::System,
        "the showcase no longer starts in System, so choosing Dark may prove nothing"
    );
    if native_theme::detect::system_is_dark() {
        run_menu_item(&mut cx, "Theme", "Light");
    }
    assert!(
        !cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        "the step does not start from light, so choosing Dark may change nothing"
    );
    let group = bounds_of(&mut cx, PROBE_COLOR_MODE);
    click_at(&mut cx, point(group.right() - px(4.), group.center().y));
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        AppColorMode::Dark,
        "the colour mode switch's Dark did not reach SetColorMode"
    );
    assert!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        "the colour mode switch's Dark did not reach Theme::mode"
    );
    click_at(&mut cx, point(group.left() + px(4.), group.center().y));
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        AppColorMode::System,
        "the colour mode switch's System did not reach SetColorMode"
    );
    assert_eq!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        native_theme::detect::system_is_dark(),
        "the colour mode switch's System did not install the desktop's mode"
    );
}

/// The TitleBar is the window's own title bar (spec §1.2, §2.1): the first
/// thing in the window, across its whole width, with the AppMenuBar inside it
/// where the platform has no menu bar of its own.
///
/// The test platform's window is server-decorated whatever it is asked for
/// (gpui-pre `platform.rs`, `PlatformWindow::window_decorations`), so `Root`'s
/// `window_border` adds no inset here and the bar's top is the window's.
#[gpui::test]
fn the_title_bar_is_the_top_of_the_window(cx: &mut TestAppContext) {
    let options = crate::window_options(Bounds {
        origin: Point::default(),
        size: WINDOW_SIZE,
    });
    assert_eq!(
        options.window_decorations,
        Some(gpui::WindowDecorations::Client),
        "the window does not ask to draw its own decorations"
    );
    assert!(
        options.app_owns_titlebar_drag,
        "the window options are not TitleBar::window_options"
    );

    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let bar = bounds_of(&mut cx, CHROME_TITLE_BAR);
    assert_eq!(bar.top(), px(0.), "the title bar starts at {:?}", bar.top());
    assert_eq!(
        bar.left(),
        px(0.),
        "the title bar starts at {:?}",
        bar.left()
    );
    assert_eq!(
        bar.size.width, WINDOW_SIZE.width,
        "the title bar is {:?} wide in a {:?} window",
        bar.size.width, WINDOW_SIZE.width
    );
    assert!(bar.size.height > px(0.), "the title bar has no height");

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
}

/// The toolbar is the model's toolbar (spec §2.3, §9): at least
/// `toolbar.bar_height` tall, with `toolbar.item_gap` between its items.
///
/// kde-breeze states a 0px gap and adwaita a 6px one, so a row that kept a
/// gap of its own fails one of the two.
#[gpui::test]
fn the_toolbar_is_the_models_toolbar(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    for preset in ["kde-breeze", "adwaita"] {
        use_preset(&mut cx, &showcase, preset);
        let model = read(&mut cx, &showcase, |_this, cx| {
            cx.native_theme()
                .and_then(|nt| nt.native(cx))
                .map(|n| (n.resolved.toolbar.bar_height, n.resolved.toolbar.item_gap))
        });
        assert!(
            model.is_some(),
            "{preset}: no native theme is installed, so nothing was measured"
        );
        let (bar_height, item_gap) = model.unwrap_or_default();
        let bar = bounds_of(&mut cx, CHROME_TOOLBAR);
        assert!(
            bar.size.height >= px(bar_height),
            "{preset}: the toolbar is {:?} tall, under toolbar.bar_height {bar_height}px",
            bar.size.height
        );
        let title = bounds_of(&mut cx, CHROME_TITLE_BAR);
        assert_eq!(
            bar.top(),
            title.bottom(),
            "{preset}: the toolbar is not right under the title bar"
        );
        // The toolbar's first two children: the SidebarToggleButton and the
        // preset Combobox.
        let first = bounds_of(&mut cx, CHROME_SIDEBAR_TOGGLE);
        let second = bounds_of(&mut cx, PROBE_COMBOBOX);
        assert!(
            bar.contains(&first.origin) && bar.contains(&second.origin),
            "{preset}: the SidebarToggleButton at {first:?} or the Combobox at {second:?} \
             is not inside the toolbar at {bar:?}"
        );
        assert_eq!(
            second.left() - first.right(),
            px(item_gap),
            "{preset}: the toolbar's first two items are {:?} apart, toolbar.item_gap is {item_gap}px",
            second.left() - first.right()
        );
    }
}

/// The toolbar's Combobox is the real preset switch: choosing a preset in it
/// installs that preset.
#[gpui::test]
fn the_toolbar_switches_the_preset(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
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
        "choosing nord in the toolbar's Combobox did not install it"
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

/// Run a menu item the way a menu does: dispatch its action into the window.
fn run_menu_item(cx: &mut VisualTestContext, menu: &str, item: &str) {
    let action = menu_action(menu, item);
    cx.update(|window, cx| window.dispatch_action(action, cx));
    cx.run_until_parked();
    draw(cx);
}

/// Run a menu item the way an `AppMenuBar` menu does: record the focus the
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
        menu_action("View", "Feedback").partial_eq(&ShowPage(3)),
        "View > Feedback does not carry ShowPage(3)"
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

    let bound: [(&str, &dyn gpui::Action); 5] = [
        ("ctrl-q", &Quit),
        ("ctrl-b", &ToggleSidebar),
        ("ctrl-i", &ToggleInspector),
        ("ctrl-k", &OpenCommandPalette),
        ("ctrl-,", &OpenPreferences),
    ];
    for (keys, action) in bound {
        let got = cx.update(|window, _cx| {
            window
                .highest_precedence_binding_for_action(action)
                .map(|b| {
                    b.keystrokes()
                        .iter()
                        .map(|k| k.unparse())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
        });
        assert_eq!(
            got.as_deref(),
            Some(keys),
            "{} is not bound to {keys}",
            action.name()
        );
    }
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

/// Dragging the handle between the content and the inspector moves their
/// boundary by the distance dragged (spec §1.1, §10.3).
///
/// The handle is the inspector panel's, laid over its left edge (gpui-base
/// resizable/panel.rs, `ResizablePanel::render`), and a drag puts the content
/// panel's right edge where the pointer is (`ResizePanelGroupElement::paint`),
/// so pressing on the boundary itself makes the distance dragged the distance
/// moved. The first move starts the drag; the second is the one measured.
#[gpui::test]
fn dragging_a_handle_resizes_its_neighbours(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    let inspector = bounds_of(&mut cx, INSPECTOR_PANEL);
    assert_eq!(
        content.right(),
        inspector.left(),
        "the content panel ends at {:?} and the inspector starts at {:?}",
        content.right(),
        inspector.left()
    );
    let (from, y) = (inspector.left(), inspector.center().y);
    let dragged = px(40.);
    cx.simulate_mouse_down(point(from, y), MouseButton::Left, Modifiers::default());
    cx.simulate_mouse_move(
        point(from - px(10.), y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(from - dragged, y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(from - dragged, y),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.run_until_parked();
    draw(&mut cx);

    let content_after = bounds_of(&mut cx, CONTENT_PANEL);
    let inspector_after = bounds_of(&mut cx, INSPECTOR_PANEL);
    let grew = inspector_after.size.width - inspector.size.width;
    let shrank = content.size.width - content_after.size.width;
    assert!(
        (grew - dragged).abs() <= px(1.),
        "dragging the handle {dragged:?} left widened the inspector by {grew:?}"
    );
    assert!(
        (shrank - dragged).abs() <= px(1.),
        "dragging the handle {dragged:?} left narrowed the content by {shrank:?}"
    );
}

/// The window is as wide as the Sidebar, a page and the inspector, and the
/// content panel opens at the width the pages were laid out for.
#[gpui::test]
fn the_window_fits_the_panels_and_a_page(cx: &mut TestAppContext) {
    assert_eq!(
        WINDOW_SIZE.width,
        NAV_WIDTH + px(PAGE_WIDTH_PX) + INSPECTOR_WIDTH,
        "WINDOW_SIZE is not NAV_WIDTH + PAGE_WIDTH_PX + INSPECTOR_WIDTH"
    );
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(
        bounds_of(&mut cx, CONTENT_PANEL).size.width,
        px(PAGE_WIDTH_PX),
        "the content panel does not open at the pages' width"
    );
}

/// Drag the handle whose line is at `x` by `by` along the row, the way
/// `dragging_a_handle_resizes_its_neighbours` does: the first move starts
/// the drag, the second is the one that lands.
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

/// The widths the handles were dragged to survive the Sidebar collapsing and
/// expanding and the inspector hiding and showing again.
#[gpui::test]
fn dragged_widths_survive_the_toggles(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let y = bounds_of(&mut cx, CONTENT_PANEL).center().y;

    let nav = bounds_of(&mut cx, CHROME_SIDEBAR).size.width;
    let at = bounds_of(&mut cx, CONTENT_PANEL).left();
    drag_handle(&mut cx, at, y, px(30.));
    let dragged_nav = bounds_of(&mut cx, CHROME_SIDEBAR).size.width;
    assert!(
        (dragged_nav - nav - px(30.)).abs() <= px(1.),
        "dragging the Sidebar's handle 30px right made it {dragged_nav:?} from {nav:?}"
    );

    let inspector = bounds_of(&mut cx, INSPECTOR_PANEL).size.width;
    let at = bounds_of(&mut cx, INSPECTOR_PANEL).left();
    drag_handle(&mut cx, at, y, px(-40.));
    let dragged_inspector = bounds_of(&mut cx, INSPECTOR_PANEL).size.width;
    assert!(
        (dragged_inspector - inspector - px(40.)).abs() <= px(1.),
        "dragging the inspector's handle 40px left made it {dragged_inspector:?} from {inspector:?}"
    );

    run_menu_item(&mut cx, "View", "Toggle Inspector");
    run_menu_item(&mut cx, "View", "Toggle Inspector");
    assert_eq!(
        bounds_of(&mut cx, INSPECTOR_PANEL).size.width,
        dragged_inspector,
        "the inspector came back at another width than it was dragged to"
    );
    assert_eq!(
        bounds_of(&mut cx, CHROME_SIDEBAR).size.width,
        dragged_nav,
        "hiding and showing the inspector moved the Sidebar's width"
    );

    run_menu_item(&mut cx, "View", "Toggle Sidebar");
    run_menu_item(&mut cx, "View", "Toggle Sidebar");
    assert_eq!(
        bounds_of(&mut cx, CHROME_SIDEBAR).size.width,
        dragged_nav,
        "the Sidebar expanded to another width than it was dragged to"
    );
    assert_eq!(
        bounds_of(&mut cx, INSPECTOR_PANEL).size.width,
        dragged_inspector,
        "collapsing and expanding the Sidebar moved the inspector's width"
    );
}

/// Toggling both side panels in the middle of a drag, then moving the
/// pointer, leaves nothing holding the dragged handle's index.
///
/// The drag records which handle it is on in the group's state
/// (gpui-base resizable/panel.rs, `ResizablePanel::render`), and the group
/// looks that panel up on every move (`ResizePanelGroupElement::paint`); a
/// state emptied under a live drag would hand it an index it no longer has.
#[gpui::test]
fn toggling_the_panels_mid_drag_is_safe(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let inspector = bounds_of(&mut cx, INSPECTOR_PANEL);
    let (x, y) = (inspector.left(), inspector.center().y);
    cx.simulate_mouse_down(point(x, y), MouseButton::Left, Modifiers::default());
    cx.simulate_mouse_move(
        point(x - px(10.), y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point(x - px(20.), y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_keystrokes("ctrl-b");
    cx.simulate_keystrokes("ctrl-i");
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        read(&mut cx, &showcase, |this, _| this.nav_collapsed
            && !this.inspector_visible),
        "Ctrl+B and Ctrl+I did not collapse the Sidebar and hide the inspector"
    );
    cx.simulate_mouse_move(
        point(x - px(40.), y),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    cx.simulate_mouse_up(
        point(x - px(40.), y),
        MouseButton::Left,
        Modifiers::default(),
    );
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        bounds_of(&mut cx, CONTENT_PANEL).size.width > px(0.),
        "the content panel was not laid out after the drag"
    );
}

/// Each handle of the resizable group reports itself (spec §4.3.5), over
/// its whole hit area, not only its 1px line.
#[gpui::test]
fn the_resize_handles_report_themselves(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    for (selector, boundary, title) in [
        (
            CHROME_HANDLE_NAV,
            content.left(),
            "ResizeHandle · Sidebar | content",
        ),
        (
            CHROME_HANDLE_INSPECTOR,
            content.right(),
            "ResizeHandle · content | inspector",
        ),
    ] {
        let target = bounds_of(&mut cx, selector);
        // The handle's hit area: 4px either side of the boundary, the line
        // taking the first pixel right of it (demo.rs, resize_handles).
        assert_eq!(
            (target.left(), target.right()),
            (boundary - px(4.), boundary + px(4.)),
            "{selector}: the target at {target:?} is not the hit area around {boundary:?}"
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
            "{selector}: a hover past the handle's hit area showed its info"
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
            "{selector}: hovering the handle's hit area did not show its info"
        );
    }
}

/// Clicking a Sidebar item shows its page (spec §2.4).
#[gpui::test]
fn the_sidebar_navigates(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_ne!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Charts,
        "the showcase starts on Charts, so showing it proves nothing"
    );
    click(&mut cx, Page::Charts.nav_item());
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_page),
        Page::Charts,
        "clicking the Sidebar's Charts item did not show the Charts page"
    );
    let page = bounds_of(&mut cx, PAGE_ROOT);
    assert!(
        page.size.width > px(0.) && page.size.height > px(0.),
        "the Charts page laid out at {:?}",
        page.size
    );
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
/// A Sidebar item is hovered: the pages report only their text panels until
/// they are migrated (plan Tasks 14-23), and chrome carries its info already.
#[gpui::test]
fn the_inspector_shows_the_settled_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(
        inspector_title(&mut cx, &showcase),
        None,
        "the inspector shows an info before anything was hovered"
    );
    let item = bounds_of(&mut cx, Page::Charts.nav_item());
    hover(&mut cx, item.center());
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("SidebarMenuItem · Charts"),
        "the inspector does not show the Sidebar item the pointer settled on"
    );
}

/// A shown info follows the state of its widget while the pointer stays
/// still: clicking the hovered Sidebar item makes it active, and the
/// inspector says so without another hover event.
#[gpui::test]
fn a_shown_info_follows_its_widgets_state(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let item = bounds_of(&mut cx, Page::Charts.nav_item()).center();
    hover(&mut cx, item);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("SidebarMenuItem · Charts")
    );
    click_at(&mut cx, item);
    cx.run_until_parked();
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("SidebarMenuItem · Charts, active"),
        "the item the pointer rests on became active, and the inspector did not follow"
    );
}

/// The inspector's Copy button puts the shown info's text on the clipboard
/// (spec §2.6).
#[gpui::test]
fn the_inspector_copies_the_shown_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let item = bounds_of(&mut cx, Page::Charts.nav_item());
    hover(&mut cx, item.center());
    settle(&mut cx);
    draw(&mut cx);
    let shown = read(&mut cx, &showcase, |this, cx| {
        this.info_ui.read(cx).shown().map(|info| info.to_text())
    });
    assert!(
        shown.is_some(),
        "nothing is shown, so nothing can be copied"
    );
    click(&mut cx, INSPECTOR_COPY);
    assert_eq!(
        cx.read_from_clipboard().and_then(|item| item.text()),
        shown,
        "the Copy button did not put the shown info on the clipboard"
    );
}

/// A page change keeps an info whose target is still drawn, as chrome is
/// (spec §4.3.4 as ruled: only what left the screen is cleared).
#[gpui::test]
fn a_page_change_keeps_the_chromes_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let item = bounds_of(&mut cx, Page::Charts.nav_item());
    hover(&mut cx, item.center());
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
        "the page change cleared the info of a Sidebar item that is still drawn"
    );
}

/// A page change clears an info whose target the new frame no longer draws
/// (spec §4.3.4 as ruled), through every route to a page: here the View
/// menu. The inspector's own TabBar is the target, taken off the screen by
/// hiding the inspector, which no hover end reports.
///
/// Once the TabBar is gone the pointer moves into the page's own padding,
/// which no page draws a widget in, so what the page draws where the
/// inspector was cannot settle in its place.
#[gpui::test]
fn a_page_change_clears_what_left_the_screen(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let tabs = bounds_of(&mut cx, INSPECTOR_TABS);
    // Clear of the handle on the inspector's left edge, whose hit area
    // reaches into the panel.
    hover(&mut cx, point(tabs.left() + px(16.), tabs.center().y));
    settle(&mut cx);
    let shown = |cx: &mut VisualTestContext| {
        read(cx, &showcase, |this, cx| {
            this.info_ui.read(cx).shown().map(|info| info.title())
        })
    };
    assert_eq!(
        shown(&mut cx).as_deref(),
        Some("TabBar · Underline, small"),
        "the inspector's TabBar did not report itself"
    );
    run_menu_item(&mut cx, "View", "Toggle Inspector");
    // Every page root pads its content by p_4 (16px), so 4px in from its
    // corner is empty on every page.
    let page = bounds_of(&mut cx, PAGE_ROOT);
    hover(&mut cx, point(page.left() + px(4.), page.top() + px(4.)));
    settle(&mut cx);
    assert_eq!(
        shown(&mut cx).as_deref(),
        Some("TabBar · Underline, small"),
        "hiding the inspector alone already cleared the info: leaving keeps it"
    );
    run_menu_item(&mut cx, "View", "Charts");
    settle(&mut cx);
    assert_eq!(
        shown(&mut cx),
        None,
        "after the page change the info of a TabBar no longer drawn is still shown"
    );
}

/// A page that does not report its instances yet (plan Tasks 14-23) still
/// shows its text panel in the inspector, and an info that settles replaces
/// it, as the text panel replaces the info in turn; a page change takes the
/// text panel away.
#[gpui::test]
fn a_pages_text_panel_shows_until_an_info_settles(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Overlays);
    let alert = bounds_of(&mut cx, PROBE_ALERT_DIALOG).center();
    let item = bounds_of(&mut cx, Page::Charts.nav_item()).center();
    hover(&mut cx, alert);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("AlertDialog"),
        "the AlertDialog block's text panel is not shown"
    );
    hover(&mut cx, item);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("SidebarMenuItem · Charts"),
        "a settled info did not replace the text panel"
    );
    hover(&mut cx, alert);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("AlertDialog"),
        "the text panel did not replace the settled info"
    );
    show(&mut cx, &showcase, Page::Inputs);
    assert_eq!(
        inspector_title(&mut cx, &showcase),
        None,
        "the Overlays page's text panel stayed after the page changed"
    );
}

/// Crossing a page's text panel on the way to the inspector is not hovering
/// it (spec §4.2, §4.3.6): a Sidebar item's settled info stays when the
/// pointer passes over a `tt-` block for less than `INFO_SETTLE`.
#[gpui::test]
fn crossing_a_pages_text_panel_keeps_the_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Overlays);
    let alert = bounds_of(&mut cx, PROBE_ALERT_DIALOG).center();
    let item = bounds_of(&mut cx, Page::Charts.nav_item()).center();
    let inspector = bounds_of(&mut cx, INSPECTOR_PANEL).center();
    hover(&mut cx, item);
    settle(&mut cx);
    draw(&mut cx);
    hover(&mut cx, alert);
    cx.executor().advance_clock(INFO_SETTLE / 2);
    cx.run_until_parked();
    hover(&mut cx, inspector);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("SidebarMenuItem · Charts"),
        "passing over the AlertDialog block replaced the Sidebar item's info"
    );
}

/// The inspector's Theme tab lays out the theme's and the window's facts in
/// place of the Widget tab.
#[gpui::test]
fn the_inspectors_theme_tab_lays_out(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let item = bounds_of(&mut cx, Page::Charts.nav_item()).center();
    hover(&mut cx, item);
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

/// The toolbar's SidebarToggleButton collapses the Sidebar to its icons and
/// expands it again (spec §2.4), through `ToggleSidebar`.
#[gpui::test]
fn the_sidebar_toggle_collapses_the_sidebar(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert!(
        !menu_item_disabled("View", "Toggle Sidebar"),
        "View > Toggle Sidebar is still disabled"
    );
    let expanded = bounds_of(&mut cx, CHROME_SIDEBAR);
    click(&mut cx, CHROME_SIDEBAR_TOGGLE);
    assert!(
        read(&mut cx, &showcase, |this, _| this.nav_collapsed),
        "the SidebarToggleButton did not collapse the Sidebar"
    );
    let collapsed = bounds_of(&mut cx, CHROME_SIDEBAR);
    assert!(
        collapsed.size.width < expanded.size.width,
        "the collapsed Sidebar is {:?} wide, the expanded one {:?}",
        collapsed.size.width,
        expanded.size.width
    );
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    assert_eq!(
        content.left(),
        collapsed.right(),
        "the content does not take the room the collapsed Sidebar gave up"
    );
    run_menu_item(&mut cx, "View", "Toggle Sidebar");
    assert_eq!(
        bounds_of(&mut cx, CHROME_SIDEBAR).size.width,
        expanded.size.width,
        "View > Toggle Sidebar did not expand the Sidebar to its width again"
    );
}

/// `ToggleInspector` hides the inspector's panel and shows it again, from the
/// View menu and from the toolbar's button (spec §1.1, §2.2, §2.6).
#[gpui::test]
fn the_inspector_toggle_hides_and_shows_it(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert!(
        !menu_item_disabled("View", "Toggle Inspector"),
        "View > Toggle Inspector is still disabled"
    );
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    run_menu_item(&mut cx, "View", "Toggle Inspector");
    assert!(
        !read(&mut cx, &showcase, |this, _| this.inspector_visible),
        "View > Toggle Inspector did not hide the inspector"
    );
    assert_eq!(
        cx.debug_bounds(INSPECTOR_PANEL),
        None,
        "the hidden inspector was still laid out"
    );
    assert!(
        bounds_of(&mut cx, CONTENT_PANEL).size.width > content.size.width,
        "the content did not take the room the inspector gave up"
    );
    click(&mut cx, CHROME_TOOLBAR_INSPECTOR);
    assert!(
        read(&mut cx, &showcase, |this, _| this.inspector_visible),
        "the toolbar's Inspector button did not show the inspector again"
    );
    assert!(
        cx.debug_bounds(INSPECTOR_PANEL).is_some(),
        "the shown inspector was not laid out"
    );
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
/// §2.7). A toolbar button is hovered: chrome carries its info already, and
/// the pages report only their text panels until they are migrated (plan
/// Tasks 14-23).
#[gpui::test]
fn the_status_bar_names_the_hovered_widget(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(
        status_title(&mut cx, &showcase),
        None,
        "the status bar names a widget before anything was hovered"
    );
    let button = bounds_of(&mut cx, CHROME_TOOLBAR_INSPECTOR);
    hover(&mut cx, button.center());
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        status_title(&mut cx, &showcase).as_deref(),
        Some("Button · Ghost, icon"),
        "the status bar does not name the toolbar button the pointer settled on"
    );
    assert_eq!(
        status_title(&mut cx, &showcase),
        inspector_title(&mut cx, &showcase),
        "the status bar and the inspector name different widgets"
    );
}

/// While the inspector shows a page's text panel instead of an info (plan
/// Tasks 14-23), the status bar names what the panel describes, never the
/// info the panel replaced.
// Task 24: delete (legacy hover_info stopgap)
#[gpui::test]
fn the_status_bar_names_what_the_inspector_shows(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let item = bounds_of(&mut cx, Page::Charts.nav_item()).center();
    let clipboard = bounds_of(&mut cx, PROBE_CLIPBOARD).center();
    hover(&mut cx, item);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        status_title(&mut cx, &showcase).as_deref(),
        Some("SidebarMenuItem · Charts")
    );
    hover(&mut cx, clipboard);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("Clipboard"),
        "the Clipboard block's text panel is not shown"
    );
    assert_eq!(
        status_title(&mut cx, &showcase).as_deref(),
        Some("Clipboard"),
        "the status bar does not name the text panel the inspector shows"
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

/// The window's three bars report themselves (spec §4.3.5): the pointer on
/// an empty stretch of each, clear of the widgets it holds, settles on the
/// bar's own info. In this order a bar that reported nothing would leave the
/// previous bar's info on show, and fail as surely as the first.
#[gpui::test]
fn the_chrome_bars_report_themselves(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let toolbar = bounds_of(&mut cx, CHROME_TOOLBAR);
    let title_bar = bounds_of(&mut cx, CHROME_TITLE_BAR);
    let status_bar = bounds_of(&mut cx, CHROME_STATUS_BAR);
    if cfg!(not(target_os = "macos")) {
        // The title bar's middle is empty only while the menus sit right of it.
        let menus = bounds_of(&mut cx, CHROME_APP_MENU_BAR);
        assert!(
            menus.left() > title_bar.center().x,
            "the menu bar at {menus:?} reaches the title bar's middle"
        );
    }
    let cases = [
        // The toolbar's items are packed at its start; its end is the row.
        (
            "Toolbar",
            point(toolbar.right() - px(8.), toolbar.center().y),
        ),
        // Between the label at the start and the menus at the end.
        ("TitleBar", title_bar.center()),
        // The middle region, which holds no item of this bar's.
        ("StatusBar", status_bar.center()),
    ];
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
    hover(cx, at);
    settle(cx);
    draw(cx);
    read(cx, showcase, |this, cx| {
        this.info_ui.read(cx).shown().map(|info| (**info).clone())
    })
}

/// A swatch shows the colour upstream paints, not the token it dims: the
/// Text Button's label is foreground at 90% (button/button.rs:994).
#[gpui::test]
fn a_dimmed_swatch_shows_the_painted_colour(cx: &mut TestAppContext) {
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

/// An Input's fill swatch is what `Theme::input_background()` paints in
/// either mode: the window background in light mode, input mixed toward
/// transparent in dark (theme/mod.rs:379-384) -- not the background in both.
#[gpui::test]
fn an_input_fill_is_what_input_background_paints(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Inputs);
    for (item, dark) in [("Light", false), ("Dark", true)] {
        run_menu_item(&mut cx, "Theme", item);
        assert_eq!(
            cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
            dark,
            "Theme > {item} did not reach Theme::mode, so this proves nothing"
        );
        let info = settle_on(&mut cx, &showcase, INPUTS_FIELD);
        let painted = cx.update(|_w, cx| Theme::global(cx).input_background());
        assert_eq!(
            input_fill(&info),
            Some(painted),
            "in {item} mode the Input's fill swatch is not input_background(): {info:?}"
        );
    }
}

/// The Input sized by `geometry::input_height` alone is as tall as the one
/// the whole `geometry::input` refines, which is what its info says it
/// takes.
#[gpui::test]
fn the_height_only_field_takes_the_control_height(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Inputs);
    let height = cx.update(|_w, cx| native_value(cx, geometry::input_height));
    assert!(
        height.is_some(),
        "no native theme is installed, so no field takes the control height"
    );
    let field = bounds_of(&mut cx, INPUTS_FIELD_HEIGHT_ONLY);
    assert_eq!(
        Some(field.size.height),
        height,
        "the height-only field is not geometry::input_height tall"
    );
    assert_eq!(
        field.size.height,
        bounds_of(&mut cx, INPUTS_FIELD).size.height,
        "the height-only field does not line up with the refined one"
    );
}

/// A Switch's corner line follows upstream's condition: the theme's radius
/// under 4px, the track's own height from 4px up (switch.rs:158-162).
#[gpui::test]
fn a_switchs_corner_line_follows_upstreams_condition(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let theme = cx.update(|_w, cx| Theme::global(cx).clone());
    let corner = |radius: f32| {
        let mut t = theme.clone();
        t.radius = px(radius);
        crate::info::inputs::switch(&t, "Feature toggle", false, false)
            .config
            .into_iter()
            .find(|n| n.what == "border-radius")
            .map(|n| n.text)
    };
    assert_eq!(corner(2.).as_deref(), Some("radius: 2px"));
    assert!(
        corner(6.).is_some_and(|text| text.starts_with("fully round")),
        "at a 6px radius the track is rounded by its height, and the line says {:?}",
        corner(6.)
    );
}

/// The height-only Input says it takes the control height only when a
/// native theme gives it one.
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
    assert!(height(true).is_some_and(|t| t.starts_with("the control height")));
    assert!(height(false).is_some_and(|t| t.starts_with("upstream's own")));
}

/// The preset Combobox's swatches are the colours upstream paints: in dark
/// mode its fill is input mixed toward transparent (theme/mod.rs:381), and a
/// hovered row is accent at 70% (searchable_list/item.rs:114).
#[gpui::test]
fn the_preset_comboboxs_swatches_are_the_painted_colours(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    run_menu_item(&mut cx, "Theme", "Dark");
    assert!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        "Theme > Dark did not reach Theme::mode, so this proves nothing"
    );
    let info = settle_on(&mut cx, &showcase, PROBE_COMBOBOX);
    let (fill, row) = cx.update(|_w, cx| {
        let t = Theme::global(cx);
        (t.input_background(), t.accent.opacity(0.7))
    });
    assert_eq!(
        input_fill(&info),
        Some(fill),
        "the preset Combobox's fill swatch is not input_background(): {info:?}"
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

/// The status bar reports the installed accessibility preferences (spec
/// §2.7): the text-scale factor always, and a flag only while it is set.
#[gpui::test]
fn the_status_bar_reports_the_accessibility_preferences(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let prefs = native_theme_gpui::AccessibilityPreferences {
        text_scaling_factor: 1.5,
        reduce_motion: false,
        high_contrast: true,
        reduce_transparency: false,
    };
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
    press(&mut cx, "ctrl-k");
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
    press(&mut cx, "ctrl-k");
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

/// The palette's preset entries install the preset, and the toolbar's
/// preset switch shows the one installed.
#[gpui::test]
fn the_palette_installs_a_preset(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    press(&mut cx, "ctrl-k");
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
        "the toolbar's preset switch does not show the preset the palette installed"
    );
}

/// The palette's preset entries are the toolbar's (ledger ruling for T12):
/// the rows the preset Combobox's delegate holds, in its order -- `default`,
/// then only presets meant for this platform.
#[test]
fn the_palette_offers_the_toolbars_presets() {
    use gpui_component::searchable_list::{SearchableListDelegate as _, SearchableListItem as _};
    let offered: Vec<String> = crate::chrome::palette_presets()
        .into_iter()
        .map(|(key, _)| key.to_string())
        .collect();
    let delegate = crate::support::PresetDelegate::new();
    let toolbar: Vec<String> = (0..delegate.items_count(0))
        .filter_map(|row| delegate.item(gpui_component::IndexPath::default().row(row)))
        .map(|item| item.value().to_string())
        .collect();
    assert_eq!(
        offered, toolbar,
        "the palette's presets are not the toolbar Combobox's rows"
    );
    assert_eq!(
        offered.first().map(String::as_str),
        Some("default"),
        "the palette's first preset is not the desktop's own"
    );
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
    press(&mut cx, "ctrl-k");
    assert!(a_dialog_is_open(&mut cx), "Ctrl+K opened nothing");
    press(&mut cx, "ctrl-k");
    assert!(
        cx.debug_bounds("dialog-1").is_none(),
        "a second Ctrl+K opened a second palette over the first"
    );
    run_menu_item_from_focus(&mut cx, "Help", "About");
    assert!(
        cx.debug_bounds("dialog-1").is_none(),
        "Help > About opened over the palette"
    );
    press(&mut cx, "ctrl-,");
    assert!(
        cx.debug_bounds(OVERLAY_PREFERENCES).is_none(),
        "Ctrl+, opened the Preferences sheet under the palette"
    );
    press(&mut cx, "escape");
    press(&mut cx, "ctrl-,");
    assert!(
        cx.debug_bounds(OVERLAY_PREFERENCES).is_some(),
        "Ctrl+, did not open the Preferences sheet"
    );
    press(&mut cx, "ctrl-k");
    assert!(
        !a_dialog_is_open(&mut cx),
        "Ctrl+K opened the palette over the Preferences sheet"
    );
}

/// Escape clears a typed query first and closes the palette only on an
/// empty one, as the palette's info says (command/state.rs,
/// `on_action_cancel`).
#[gpui::test]
fn escape_clears_the_query_then_closes_the_palette(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    press(&mut cx, "ctrl-k");
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
    press(&mut cx, "ctrl-k");
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
    press(&mut cx, "ctrl-k");
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
    press(&mut cx, "ctrl-k");
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
    let unset = native_theme_gpui::AccessibilityPreferences {
        text_scaling_factor: 1.0,
        reduce_motion: false,
        high_contrast: false,
        reduce_transparency: false,
    };
    cx.update(|_window, cx| native_theme_gpui::apply_accessibility(&unset, cx));
    cx.run_until_parked();
    assert!(
        !cx.update(|_window, cx| cx.reduce_motion()),
        "motion is reduced before the switch is clicked, so the click could prove nothing"
    );
    press(&mut cx, "ctrl-,");
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

/// A theme that fails to load is reported by an Alert at the top of the
/// content (spec §2.5), which reports itself.
#[gpui::test]
fn a_theme_error_is_an_alert(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    assert!(
        cx.debug_bounds(CONTENT_ALERT).is_none(),
        "an Alert shows before any theme failed"
    );
    use_preset(&mut cx, &showcase, "no-such-preset");
    let alert = bounds_of(&mut cx, CONTENT_ALERT);
    let content = bounds_of(&mut cx, CONTENT_PANEL);
    assert_eq!(
        alert.top(),
        content.top(),
        "the Alert at {alert:?} is not at the top of the content at {content:?}"
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
    use_preset(&mut cx, &showcase, "macos-sonoma");
    draw(&mut cx);
    let sonoma = gap_of(&mut cx, &showcase);
    assert_ne!(
        breeze, sonoma,
        "the two presets state the same gap, so the switch proves nothing"
    );
    assert_eq!(
        Some(gap_between(&mut cx)),
        sonoma,
        "the open About dialog kept the gap of the theme it opened under"
    );
}

/// The About dialog's title reports the Dialog, as the palette's does.
#[gpui::test]
fn the_about_title_reports_the_dialog(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    run_menu_item(&mut cx, "Help", "About");
    draw(&mut cx);
    let surface = bounds_of(&mut cx, "dialog-0");
    let name = bounds_of(&mut cx, OVERLAY_ABOUT_NAME);
    // Above the content, clear of the close button at the surface's right.
    let at = point(
        name.left() + px(4.),
        surface.top() + (name.top() - surface.top()) / 2.,
    );
    hover(&mut cx, at);
    settle(&mut cx);
    draw(&mut cx);
    assert_eq!(
        inspector_title(&mut cx, &showcase).as_deref(),
        Some("Dialog · About"),
        "the pointer settled on the About dialog's title, and the inspector does not show it"
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
    let red = gpui::hsla(0.0, 1.0, 0.5, 1.0);
    let info = WidgetInfo::new("Tag")
        .variant("Danger")
        .color(claim("bg", "danger", red, "gpui-component/tag.rs:31"))
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
/// opaque colour the widget never paints.
#[test]
fn a_translucent_colour_prints_its_alpha() {
    let red = gpui::hsla(0.0, 1.0, 0.5, 1.0);
    assert_eq!(hsla_to_hex(red), "#ff0000");
    assert_eq!(hsla_to_hex(red.opacity(0.8)), "#ff0000cc");
    assert_eq!(hsla_to_hex(gpui::hsla(0.0, 0.0, 0.0, 0.0)), "#00000000");
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
        gpui::div()
            .relative()
            .size_full()
            .child(epoch_marker(&self.ui))
            .child(
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
    cx.update(|_window, cx| ui.update(cx, |r, _| r.page_changed()));
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
