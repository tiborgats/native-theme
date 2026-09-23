//! Showcase self-tests (spec v0.5.9 §6.1)
//!
//! `test = true` on the example target (Cargo.toml) puts these under a plain
//! `cargo test`, so CI and the nightly dependency canary run them with no
//! workflow change. They build the real `Showcase` on GPUI's headless test
//! platform — the same view, the same `Root`, the same window width `main`
//! opens — and drive it with real input.

use gpui::{
    AbsoluteLength, App, Bounds, DefiniteLength, Entity, Focusable as _, Length, Modifiers,
    MouseButton, Pixels, Point, TestAppContext, VisualTestContext, point, prelude::*, px, rems,
    size,
};
use gpui_base::ScrollbarHandle as _;
use gpui_component::{Colorize as _, IconName, Root, WindowExt as _, theme::Theme};
use native_theme_gpui::{ActiveNativeTheme, geometry};
use std::cell::RefCell;
use std::ops::Deref as _;
use std::rc::Rc;

use crate::app::{
    AppColorMode, OpenCommandPalette, OpenPreferences, Quit, SetColorMode, ShowPage, Showcase,
    ToggleSidePanel,
};
use crate::chrome::menus;
use crate::demo::{AREA_FILL_OPACITY, IconSizeContext};
use crate::info::{
    GEOMETRY_NOTES, INFO_SETTLE, InfoExt as _, InfoRegistry, WidgetInfo, claim, epoch_marker,
    hsla_to_hex, native_info, percent_text,
};
use crate::inspector::InspectorTab;
use crate::support::{
    CAROUSEL_SLIDES, ChromeIcon, load_all_icons, load_gpui_icons, native_geometry, native_value,
};
use crate::{
    BUTTONS_DANGER, BUTTONS_DISABLED_SECONDARY, BUTTONS_HEADING_VARIANTS, BUTTONS_PRIMARY,
    BUTTONS_TEXT, CHARTS_AREA_CHART, CHARTS_BAR_CHART, CHARTS_CANDLESTICK_CHART, CHARTS_LINE_CHART,
    CHARTS_PIE_CHART, CHROME_APP_MENU_BAR, CHROME_HANDLE, CHROME_LABEL_ICON_THEME,
    CHROME_LABEL_MODE, CHROME_LABEL_THEME, CHROME_MENU_BAR, CHROME_PAGE_TABS, CHROME_SIDE_PANEL,
    CHROME_SIDE_PANEL_SEPARATOR, CHROME_SIDE_PANEL_TOGGLE, CHROME_STATUS_BAR,
    CHROME_THEME_SETTINGS, CHROME_TITLE_BAR, CHROME_TOOLBAR, CHROME_TOOLBAR_PALETTE,
    CHROME_TOOLBAR_PREFERENCES, CHROME_TOOLBAR_RELOAD, CONTENT_ALERT, CONTENT_PANEL,
    CONTENT_SCROLL, DATA_PAGINATION, DATA_PAGINATION_COMPACT, DATA_TABLE_HEADER,
    FEEDBACK_ALERT_INFO, FEEDBACK_BADGE_COUNT, FEEDBACK_BADGE_DOT, FEEDBACK_CIRCLE_LOADING,
    FEEDBACK_SPINNER_SMALL, FEEDBACK_TAG_DANGER, FEEDBACK_TAG_PRIMARY, INPUTS_CHECKBOX_AUTOSAVE,
    INPUTS_CHECKBOX_NOTIFICATIONS, INPUTS_FIELD, INPUTS_FIELD_HEIGHT_ONLY, INPUTS_TEXTAREA,
    INSPECTOR_COPY, INSPECTOR_PANEL, INSPECTOR_TABS, INSPECTOR_TITLE, INSPECTOR_TOKENS_NOTE,
    LAYOUT_BREADCRUMB, LAYOUT_COLLAPSIBLE, LAYOUT_COLLAPSIBLE_CONTENT, LAYOUT_COLLAPSIBLE_TOGGLE,
    LAYOUT_GROUP_BOX_NORMAL, LAYOUT_GROUP_BOX_OUTLINE, LAYOUT_SEPARATOR_DASHED,
    LAYOUT_SEPARATOR_SOLID, LAYOUT_SIDEBAR_COLLAPSED, LAYOUT_SIDEBAR_EXPANDED,
    LAYOUT_SIDEBAR_ITEMS, LAYOUT_TITLE_BAR, LEFT_PANEL_WIDTH, LIST_DEMO, OVERLAY_ABOUT_LINK,
    OVERLAY_ABOUT_NAME, OVERLAY_ABOUT_TEXT, OVERLAY_PALETTE, OVERLAY_PALETTE_TITLE,
    OVERLAY_PREFERENCES, OVERLAYS_DIALOG_CLOSE, OVERLAYS_DIALOG_FOOTER, OVERLAYS_DIALOG_TRIGGER,
    OVERLAYS_SHEET_BOTTOM, OVERLAYS_SHEET_BOTTOM_TITLE, OVERLAYS_SHEET_RIGHT,
    OVERLAYS_SHEET_RIGHT_TITLE, PAGE_ROOT, PAGE_WIDTH_PX, PREF_REDUCE_MOTION, PROBE_ALERT_DIALOG,
    PROBE_ATTACHMENT, PROBE_CAROUSEL_LAST, PROBE_CHAT_SEND, PROBE_CLIPBOARD, PROBE_COLOR_MODE,
    PROBE_COMBOBOX, PROBE_ICON_THEME, PROBE_NOTIFICATION, PROBE_PAGINATION, PROBE_RATING,
    PROBE_SETTINGS_ROW, PROBE_STEPPER, Page, STATUS_ENVIRONMENT, STATUS_HOVERED, STATUS_MIDDLE,
    TREE_DEMO, TYPOGRAPHY_H1, TYPOGRAPHY_H2, TYPOGRAPHY_LABEL_PLAIN, TYPOGRAPHY_LABEL_SECONDARY,
    WINDOW_SIZE, WINDOW_TITLE,
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
) -> (Entity<Showcase>, Entity<Root>, VisualTestContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        crate::app::init(cx);
    });
    let view: Rc<RefCell<Option<Entity<Showcase>>>> = Rc::new(RefCell::new(None));
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

/// Every page lays out: the TabBar's ten pages each render on the test
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
    assert_eq!(Page::ALL.len(), 10, "the TabBar no longer has ten pages");
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
    // windows-11's list rows (40px) are the tallest any bundled preset
    // states, so the six sample rows are certain to overflow the demo box.
    use_preset(&mut cx, &showcase, "windows-11");
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
/// menu-bar row with the AppMenuBar in it, the toolbar right under it. On
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
/// width, with the AppMenuBar inside it where the platform has no menu bar
/// of its own, and no menu-bar row. The Layout page then draws no TitleBar
/// sample: the window's own title bar is one.
///
/// The test platform grants server-side decorations only, so the grant is
/// the test's (`Showcase::frame_for_test`). In a window really granted
/// client-side decorations `Root`'s `window_border` insets the content by
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
/// manager's frame, with none of the facts of `Root`'s client frame, or
/// `Root`'s client frame.
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
        value(&server, "frame").is_some_and(|v| v.starts_with("whatever the window manager draws")),
        "the server-side rows do not say the window manager draws the frame: {server:?}"
    );
    for client_only in ["frame fill", "frame colour", "frame shadow"] {
        assert_eq!(
            value(&server, client_only),
            None,
            "the server-side rows state {client_only}, a fact of Root's client frame"
        );
    }
    let client =
        crate::inspector::window_rows(CLIENT_SIDE, Some(px(20.)), gpui::Edges::all(px(20.)));
    assert!(
        value(&client, "decorations").is_some_and(|v| v.starts_with("client-side")),
        "the client-side rows do not name the mode: {client:?}"
    );
    assert!(
        value(&client, "frame").is_some_and(|v| v.starts_with("Root's client frame")),
        "the client-side rows do not say Root's client frame is drawn: {client:?}"
    );
    for client_only in ["frame fill", "frame colour", "frame shadow"] {
        assert!(
            value(&client, client_only).is_some(),
            "the client-side rows do not state {client_only}"
        );
    }
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
/// stands. The row is as tall as its buttons and its padding -- none of the
/// three states a `toolbar.bar_height` for nord -- so the first button's top
/// is the top padding.
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
            cx.native_theme()
                .and_then(|nt| nt.native(cx))
                .map(|n| n.resolved.toolbar.border.padding)
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
/// A native preset is only offered on its own platform, so macos-sonoma
/// resolved on a 96 DPI Linux host is not a configuration the showcase
/// shows; ios is offered on macOS too (its `platforms` are `macos` and
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
/// otherwise the defaults, not the host's: `AccessibilityPreferences::from_system`,
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
        let prefs = native_theme::AccessibilityPreferences {
            text_scaling_factor: text_scale,
            ..native_theme::AccessibilityPreferences::default()
        };
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
    for native in ["kde-breeze", "adwaita", "macos-sonoma", "ios", "windows-11"] {
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
            assert_eq!(
                panel.size.width, LEFT_PANEL_WIDTH,
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
            assert!(
                (inspector.bottom() - panel.bottom()).abs() <= px(0.01),
                "{at}: the inspector ends at {:?}, not at the side panel's bottom {:?}",
                inspector.bottom(),
                panel.bottom()
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
/// the label's width shows it: `demo::label` gives its Label `text_sm`,
/// 0.875rem, and `self_start` keeps it as wide as its text, and "Icon theme"
/// and "Icon set" differ in width.
#[gpui::test]
fn the_third_row_reads_icon_theme(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let label = bounds_of(&mut cx, CHROME_LABEL_ICON_THEME);
    let width = |cx: &mut VisualTestContext, text: &'static str| {
        cx.update(|window, _| {
            let text = gpui::SharedString::from(text);
            let run = window.text_style().to_run(text.len());
            window
                .text_system()
                .shape_line(text, rems(0.875).to_pixels(window.rem_size()), &[run], None)
                .width()
        })
    };
    let (theme, set) = (width(&mut cx, "Icon theme"), width(&mut cx, "Icon set"));
    assert_ne!(
        theme, set,
        "the two texts are as wide, so this proves nothing"
    );
    assert!(
        (label.size.width - theme).abs() <= device_pixel(&mut cx),
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
            this.icon_set_select
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
        read(&mut cx, &showcase, |this, _| this.icon_set_name.clone()),
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

    let bound: [(&str, &dyn gpui::Action); 4] = [
        ("ctrl-q", &Quit),
        ("ctrl-b", &ToggleSidePanel),
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

/// The inspector has no panel of its own to hide (spec S4): it hides with
/// the side panel it is in. So no `ToggleInspector` action is registered, no
/// menu item names the inspector, and Ctrl+I, which ran it, is bound to
/// nothing.
#[gpui::test]
fn no_inspector_toggle_remains(cx: &mut TestAppContext) {
    let (_showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let (names, ctrl_i) = cx.update(|_window, cx| {
        let names: Vec<&'static str> = cx.all_action_names().to_vec();
        let ctrl_i: Vec<&'static str> = cx
            .key_bindings()
            .borrow()
            .bindings()
            .filter(|b| {
                b.keystrokes()
                    .iter()
                    .map(|k| k.unparse())
                    .collect::<Vec<_>>()
                    .join(" ")
                    == "ctrl-i"
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
    let inspector_items: Vec<String> = menus()
        .into_iter()
        .flat_map(|m| m.items)
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
    assert_eq!(
        panel.right(),
        content.left(),
        "the side panel ends at {:?} and the content starts at {:?}",
        panel.right(),
        content.left()
    );
    let dragged = px(40.);
    drag_handle(&mut cx, content.left(), content.center().y, dragged);
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

/// The window is two panels wide (spec S1): the side panel, which opens at
/// `LEFT_PANEL_WIDTH`, and the content panel, which opens at the width the
/// pages were laid out for.
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
    assert_eq!(
        bounds_of(&mut cx, CONTENT_PANEL).size.width,
        px(PAGE_WIDTH_PX),
        "the content panel does not open at the pages' width"
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
    assert_eq!(
        content.left(),
        panel.right(),
        "the content panel does not follow the side panel"
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
    drag_handle(&mut cx, content.left(), content.center().y, px(30.));
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
    assert_eq!(
        bounds_of(&mut cx, CONTENT_PANEL).left(),
        dragged,
        "the content does not start where the side panel came back to"
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
    cx.simulate_keystrokes("ctrl-b");
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
    cx.simulate_keystrokes("ctrl-b");
    cx.run_until_parked();
    draw(&mut cx);
    assert!(
        cx.debug_bounds(CHROME_SIDE_PANEL).is_some(),
        "Ctrl+B did not show the side panel again after the drag"
    );
}

/// The resizable group's handle reports itself (spec §4.3.5), over its
/// whole hit area, not only its 1px line.
#[gpui::test]
fn the_resize_handle_reports_itself(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let boundary = bounds_of(&mut cx, CONTENT_PANEL).left();
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
const PAGE_TABS_TITLE: &str = "TabBar · Underline, small, menu";

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

/// Upstream's Underline TabBar pads neither itself nor its tabs
/// (tab/tab.rs:79-81, tab/tab_bar.rs:393-402): it leaves the inset to the
/// container it is in. So each of the showcase's two TabBars is inset by
/// `layout.container_margin`, the padding the side panel's settings and the
/// inspector's content take: its first tab starts that far from the edge of
/// its panel, not against the resize handle's line or the window's edge.
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
        for (bar, panel, first) in [
            ("page", CONTENT_PANEL, Page::ALL[0].tab()),
            ("inspector", INSPECTOR_PANEL, InspectorTab::Widget.tab()),
        ] {
            let panel = bounds_of(&mut cx, panel);
            let tab = bounds_of(&mut cx, first);
            assert!(
                (tab.left() - panel.left() - margin).abs() <= slack,
                "{preset}: the {bar} TabBar's first tab starts at {:?}, {:?} from its panel's \
                 left edge, not layout.container_margin, {margin:?}",
                tab.left(),
                tab.left() - panel.left()
            );
        }
    }
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
        Some(PAGE_TABS_TITLE),
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

/// The inspector's Copy button puts the shown info's text on the clipboard
/// (spec §2.6).
#[gpui::test]
fn the_inspector_copies_the_shown_info(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    let tab = bounds_of(&mut cx, Page::Charts.tab());
    hover(&mut cx, tab.center());
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

/// A page change clears an info whose target the new frame no longer draws
/// (spec §4.3.4 as ruled), through every route to a page: here the View
/// menu. The inspector's own TabBar is the target, taken off the screen by
/// hiding the side panel it is in, which no hover end reports.
///
/// Once the TabBar is gone the pointer moves into the page's own padding,
/// which no page draws a widget in, so what the page draws where the side
/// panel was cannot settle in its place.
#[gpui::test]
fn a_page_change_clears_what_left_the_screen(cx: &mut TestAppContext) {
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
        Some("TabBar · Underline, small"),
        "the inspector's TabBar did not report itself"
    );
    run_menu_item(&mut cx, "View", "Toggle Side Panel");
    // Every page root pads its content by p_4 (16px), so 4px in from its
    // corner is empty on every page.
    let page = bounds_of(&mut cx, PAGE_ROOT);
    hover(&mut cx, point(page.left() + px(4.), page.top() + px(4.)));
    settle(&mut cx);
    assert_eq!(
        shown(&mut cx).as_deref(),
        Some("TabBar · Underline, small"),
        "hiding the side panel alone already cleared the info: leaving keeps it"
    );
    run_menu_item(&mut cx, "View", "Charts");
    settle(&mut cx);
    assert_eq!(
        shown(&mut cx),
        None,
        "after the page change the info of a TabBar no longer drawn is still shown"
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

/// The fill the panel toggle tagged `selector` paints with the pointer
/// elsewhere: the ghost variant's active colour while it is selected
/// (button/button.rs:1252, `colors.active`, which `variants::ghost_button`
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
const SIDEBAR_ICON_PRESETS: [&str; 5] = [
    "kde-breeze",
    "adwaita",
    "macos-sonoma",
    "windows-11",
    "nord",
];

/// The Layout page's Sidebar samples' icons fit their items (spec S5),
/// expanded and in the collapsed sample's rail.
///
/// Upstream gives no hook to measure the drawn icon: `SidebarMenuItem`
/// places the `Icon` it is given itself (sidebar/menu.rs:300), and an `Icon`
/// builds its `svg()` with nothing of it reachable but its style
/// (icon.rs:169-177), so no debug selector gets there. What is checked:
///
/// - the Icon `demo::sidebar_icon_sized` hands an item has the size
///   `geometry::icon_size_small` gives, read off its style;
/// - in the rail, every item's measured bounds lie within the rail's width,
///   and no two items overlap. Neither catches an oversized icon on its
///   own: an item is as wide as the rail lets it be whatever its icon, and
///   grows taller rather than overlap. What does is the item's height:
///   there a row has no height of its own and holds its icon alone
///   (sidebar/menu.rs:301-308), inside `p_2` on every side (:284), so the
///   measured height less that padding is the icon's, and the icon, as
///   wide as it is tall (the first check), has to fit the item's measured
///   width;
/// - expanded, where every row is `h_7` whatever its icon
///   (sidebar/menu.rs:308), the icon is no taller than that: 1.75 rem at the
///   rem the Root installs, the theme's font size (root.rs:582). The items'
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
/// panel (platform-facts.md:1135-1136), and none of kde-breeze's. The preset
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
        cx.update(|_window, cx| {
            showcase.update(cx, |this, _cx| {
                this.current_theme_name = chosen.to_string();
                this.default_preset = preset.to_string();
            });
        });
        use_preset(&mut cx, &showcase, preset);
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
        Some("Button · Ghost, icon"),
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
        cx.native_theme()
            .and_then(|nt| nt.native(cx))
            .map(|n| n.resolved.status_bar.border.padding)
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

/// The status bar no longer carries the version (spec §3.2): the title bar
/// does. What it draws is read off the frame, end to end: the side-panel
/// toggle, the environment text, the empty middle region, and the shown
/// title where one is shown (spec S4), each next to the one before it by the
/// bar's `gap_2` (status_bar.rs:84, :88), the last ending the bar's right
/// padding in. So nothing else is drawn between them. The environment
/// text is as wide as gpui's text system lays out the environment the
/// showcase names (`chrome::status_environment`), and the shown title as its
/// title, so neither carries more. Its info says the same.
#[gpui::test]
fn the_status_bar_carries_no_version(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    let right_padding = read(&mut cx, &showcase, |_this, cx| {
        cx.native_theme()
            .and_then(|nt| nt.native(cx))
            .and_then(|n| n.resolved.status_bar.border.padding.right)
    });
    assert!(
        right_padding.is_some(),
        "kde-breeze states no right status-bar padding, so the bar's end is not known"
    );
    let gap = cx.update(|window, _| rems(0.5).to_pixels(window.rem_size()));
    // gpui places elements on the device's pixel grid, so a position may be
    // off by up to one device pixel.
    let slack = device_pixel(&mut cx);
    let environment = read(&mut cx, &showcase, |this, cx| {
        crate::chrome::status_environment(this, cx).join(" · ")
    });
    assert!(
        !environment.contains(env!("CARGO_PKG_VERSION")),
        "the environment the showcase names carries the version: {environment}"
    );
    let expected = status_text_width(&mut cx, &environment);
    assert!(expected.is_some(), "no status-bar text size is installed");

    // Nothing shown, and then the toolbar's Command Palette button.
    for hovered in [None, Some(CHROME_TOOLBAR_PALETTE)] {
        if let Some(selector) = hovered {
            let at = bounds_of(&mut cx, selector).center();
            hover(&mut cx, at);
            settle(&mut cx);
            draw(&mut cx);
        }
        let bar = bounds_of(&mut cx, CHROME_STATUS_BAR);
        let left = bounds_of(&mut cx, CHROME_SIDE_PANEL_TOGGLE);
        let env = bounds_of(&mut cx, STATUS_ENVIRONMENT);
        let middle = bounds_of(&mut cx, STATUS_MIDDLE);
        assert_eq!(
            Some(env.size.width),
            expected,
            "the status bar's environment text is not the environment alone"
        );
        let mut drawn = vec![
            ("side-panel toggle", left),
            ("environment", env),
            ("middle", middle),
        ];
        let shown = read(&mut cx, &showcase, |this, _| {
            this.status_title_drawn.clone()
        });
        assert_eq!(shown.is_some(), hovered.is_some());
        if let Some(title) = shown {
            let label = bounds_of(&mut cx, STATUS_HOVERED);
            assert_eq!(
                Some(label.size.width),
                status_text_width(&mut cx, &title),
                "the status bar's shown title is not {title:?} alone"
            );
            drawn.push(("shown title", label));
        }
        for pair in drawn.windows(2) {
            if let [(a, before), (b, after)] = pair {
                assert!(
                    (after.left() - before.right() - gap).abs() <= slack,
                    "the status bar draws something between its {a} at {before:?} and its {b} \
                     at {after:?}, which are not gap_2 apart"
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

/// Where the chosen icon theme has no `PanelLeft`, the side-panel toggle
/// shows its tooltip's text as its label, never another icon theme's icon
/// (spec §3.2): with Material's taken out of the loaded gallery, the toggle
/// is as wide as its tooltip's text at a Small Button's `text_sm`
/// (sizing.rs:322) plus the `px_2` on either side (button/button.rs:629-631),
/// and says why in its info.
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
        let expected = cx.update(|window, _| {
            let rem = window.rem_size();
            let text = gpui::SharedString::from(tooltip);
            let run = window.text_style().to_run(text.len());
            window
                .text_system()
                .shape_line(text, rems(0.875).to_pixels(rem), &[run], None)
                .width()
                + rems(0.5).to_pixels(rem) * 2.
        });
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

/// The popover-filled boxes painted in the last frame.
fn popover_boxes(cx: &mut VisualTestContext) -> Vec<Bounds<gpui::ScaledPixels>> {
    cx.update(|window, cx| {
        let popover = Theme::global(cx).popover;
        window
            .painted_quads()
            .into_iter()
            .filter(|q| q.background.as_solid() == Some(popover))
            .map(|q| q.bounds)
            .collect()
    })
}

/// The width of the tooltip that shows once the pointer rests on the
/// element tagged `selector` past gpui's tooltip delay (gpui-pre
/// elements/div.rs:53, 500ms): the widest box painted in the tooltip's
/// `popover` fill (tooltip.rs:113-114) that was not painted before. `None`
/// where no such box is painted.
fn tooltip_width(cx: &mut VisualTestContext, selector: &'static str) -> Option<Pixels> {
    // Off any widget first, so a tooltip shown before is gone.
    let middle = bounds_of(cx, STATUS_MIDDLE).center();
    hover(cx, middle);
    cx.executor()
        .advance_clock(std::time::Duration::from_secs(1));
    draw(cx);
    let before = popover_boxes(cx);
    let at = bounds_of(cx, selector).center();
    hover(cx, at);
    cx.executor()
        .advance_clock(std::time::Duration::from_secs(1));
    cx.run_until_parked();
    draw(cx);
    let scale = cx.update(|window, _| window.scale_factor());
    popover_boxes(cx)
        .into_iter()
        .filter(|b| !before.contains(b))
        .map(|b| px(b.size.width.0 / scale))
        .max_by(|a, b| a.as_f32().total_cmp(&b.as_f32()))
}

/// The side-panel toggle's tooltip names its key binding (spec S4): the
/// action is bound to Ctrl+B, and the tooltip is wider while the action has
/// that binding than once the bindings are gone -- upstream's Tooltip adds
/// the binding's Kbd beside the text only where the action has one
/// (tooltip.rs:94-106, :133-141). The tooltip's text is not readable from
/// the test's frame, so its width is what shows the Kbd.
#[gpui::test]
fn the_side_panel_toggles_tooltip_names_its_key(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    // A tooltip fades in (tooltip.rs:178-181); without motion it is drawn
    // in full at once.
    without_motion(&mut cx);
    let cases: [(&'static str, Box<dyn gpui::Action>, &str); 1] = [(
        CHROME_SIDE_PANEL_TOGGLE,
        Box::new(ToggleSidePanel),
        "ctrl-b",
    )];
    let mut bound = Vec::new();
    for (selector, action, keys) in &cases {
        let binding = cx.update(|window, _| {
            window
                .highest_precedence_binding_for_action(action.as_ref())
                .map(|b| {
                    b.keystrokes()
                        .iter()
                        .map(|k| k.unparse())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
        });
        assert_eq!(
            binding.as_deref(),
            Some(*keys),
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
        (
            "Toolbar",
            point(toolbar.right() - px(8.), toolbar.center().y),
        ),
        // The middle region, which holds no item of this bar's.
        ("StatusBar", status_bar.center()),
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
        cases.insert(1, ("Menu bar", at));
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
    hover(cx, at);
    settle(cx);
    draw(cx);
    read(cx, showcase, |this, cx| {
        this.info_ui.read(cx).shown().map(|info| (**info).clone())
    })
}

/// A swatch shows the colour upstream paints, not the token it dims: the
/// Text Button's label is foreground at 90% (button/button.rs:994). A text
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
/// And it is the fill the frame holds inside the Input.
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
        assert_eq!(
            painted_fill(&mut cx, INPUTS_FIELD),
            input_fill(&info),
            "in {item} mode the fill painted inside the Input is not its fill swatch"
        );
    }
}

/// Set the text-scaling factor the connector lays controls out at, and draw
/// the frame that follows.
fn scale_text(cx: &mut VisualTestContext, factor: f32) {
    let prefs = native_theme_gpui::AccessibilityPreferences {
        text_scaling_factor: factor,
        ..Default::default()
    };
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

/// The Textarea keeps its own 90px under `geometry::input`, whose height
/// rule is for a single-line field, at text scale 1 and 2, and takes none of
/// the builder's padding, which is a single-line field's too.
#[gpui::test]
fn the_textarea_keeps_its_own_height(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Page::Inputs);
    let info = settle_on(&mut cx, &showcase, INPUTS_TEXTAREA);
    assert!(
        info.as_ref()
            .and_then(|i| i.config.iter().find(|n| n.what == "geometry"))
            .is_some_and(|g| g.text.starts_with("geometry::input:")),
        "the Textarea does not take geometry::input: {info:?}"
    );
    assert!(
        info.as_ref()
            .and_then(|i| i.instance.iter().find(|n| n.what == "padding"))
            .is_some_and(|p| p.text.starts_with("none from geometry::input")),
        "the Textarea's info does not say it takes no padding: {info:?}"
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

/// The preset Combobox's swatches are the colours upstream paints: in dark
/// mode its fill is input mixed toward transparent (theme/mod.rs:381), which
/// is the fill the frame holds inside it, and a hovered row is accent at 70%
/// (searchable_list/item.rs:114) -- checked against that line, since no row
/// is drawn while the popup is shut.
#[gpui::test]
fn the_preset_combobox_paints_its_fill_swatch_and_names_its_row_hover(cx: &mut TestAppContext) {
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
    assert_eq!(
        painted_fill(&mut cx, PROBE_COMBOBOX),
        input_fill(&info),
        "the fill painted inside the preset Combobox is not its fill swatch"
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
            Some("gpui-component/list/list_item.rs:189"),
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

/// The Charts page's five charts each show their own info (spec §4.3.2):
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
/// paint: the series colour at `AREA_FILL_OPACITY`, not the colour at full.
/// The area is a path, not a quad of the scene, so the swatch is checked
/// against what the showcase passes, not against the frame.
#[gpui::test]
fn the_area_charts_fill_swatch_is_the_series_at_the_fill_opacity(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Page::Charts);
    let info = settle_on(&mut cx, &showcase, CHARTS_AREA_CHART);
    let painted = cx.update(|_window, cx| Theme::global(cx).chart_3.opacity(AREA_FILL_OPACITY));
    let fill = info
        .as_ref()
        .and_then(|info| info.colors.iter().find(|c| c.role.starts_with("fill")));
    assert_eq!(
        fill.map(|c| c.value),
        Some(painted),
        "the AreaChart's fill swatch is not chart_3 at {}: {info:?}",
        percent_text(AREA_FILL_OPACITY)
    );
}

/// The ids and debug selectors of three of the Icons page's icons, one from
/// each gallery, with Material's icons loaded. The page forms them from the
/// animation's place, the role's name and the IconName's (pages/icons.rs).
const ICONS_ANIMATED_FRAMES: &str = "icons-animated-frames-0";
const ICONS_NATIVE_DIALOG_WARNING: &str = "icons-native-DialogWarning";
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
    let material = load_gpui_icons(
        Some(native_theme::theme::IconSet::Material),
        None,
        None,
        None,
    );
    for name in chrome_icon_names() {
        let drawn = read(&mut cx, &showcase, |this, _| this.chrome_icon(&name));
        let path = gpui_component::IconNamed::path(name.clone());
        let entry = material
            .iter()
            .find(|(_, icon, ..)| gpui_component::IconNamed::path(icon.clone()) == path);
        let expected = match entry {
            Some((n, _, _, Some(native_theme::theme::IconData::Svg(bytes)), _)) => {
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
    use_preset(&mut cx, &showcase, "no-such-preset");
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

/// Paint-level check (rationale §3.6): the fill gpui painted inside the
/// Primary Tag at rest is the colour its info's bg claim shows. The colour
/// gate reads the line a claim cites; this reads the frame. At rest, because
/// a hovered Tag paints at 90% (tag.rs:265).
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
fn painted_fill(cx: &mut VisualTestContext, selector: &'static str) -> Option<gpui::Hsla> {
    let bounds = bounds_of(cx, selector);
    cx.update(|window, _cx| {
        let bounds = bounds.scale(window.scale_factor());
        window
            .painted_quads()
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

/// A hovered Tag fades to 90% (tag.rs:265), and its info names the fill
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
            .is_some_and(|text| text.contains(&hsla_to_hex(fill)))),
        "the hovered Primary Tag paints {:?}, which its hover note does not name: {hover:?}",
        painted.map(hsla_to_hex)
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

/// `--icon-theme` names the freedesktop theme the icons load from, and they
/// load from it: applying the override reloads them, so the Icons page's
/// label and the icons it shows agree. A freedesktop set is chosen first
/// (another theme's, which the override must win over), so the label names
/// a theme on every platform.
#[gpui::test]
fn the_icon_theme_override_reloads_the_icons(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let theme = "hicolor";
    cx.update(|window, cx| {
        showcase.update(cx, |this, cx| {
            this.select_icon_set("Adwaita", window, cx);
            this.set_icon_theme_override(theme.to_string(), window, cx);
        });
    });
    cx.run_until_parked();
    let (loaded, expected) = read(&mut cx, &showcase, |this, _cx| {
        let effective = this
            .icon_set_choice
            .effective_icon_set(this.current_icon_set);
        let fc = this.original_font.color;
        (
            this.loaded_icons.clone(),
            load_all_icons(
                effective,
                this.icon_set_choice.freedesktop_theme(),
                Some(theme),
                Some([fc.r, fc.g, fc.b]),
            ),
        )
    });
    assert!(
        loaded == expected,
        "the icons on show are not the ones --icon-theme {theme} loads"
    );
    assert_eq!(
        read(&mut cx, &showcase, |this, _cx| this.icon_set_label()),
        format!("freedesktop ({theme})"),
        "the Icons page names another theme than the one --icon-theme loads from"
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

/// The palette's preset entries install the preset, and the theme settings'
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
        "the preset switch does not show the preset the palette installed"
    );
}

/// The palette's preset entries are the preset switch's (ledger ruling for T12):
/// the rows the preset Combobox's delegate holds, in its order -- `default`,
/// then only presets meant for this platform.
#[test]
fn the_palette_offers_the_preset_switchs_presets() {
    use gpui_component::searchable_list::{SearchableListDelegate as _, SearchableListItem as _};
    let offered: Vec<String> = crate::chrome::palette_presets()
        .into_iter()
        .map(|(key, _)| key.to_string())
        .collect();
    let delegate = crate::support::PresetDelegate::new();
    let switch: Vec<String> = (0..delegate.items_count(0))
        .filter_map(|row| delegate.item(gpui_component::IndexPath::default().row(row)))
        .map(|item| item.value().to_string())
        .collect();
    assert_eq!(
        offered, switch,
        "the palette's presets are not the preset Combobox's rows"
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
    run_menu_item_from_focus(&mut cx, "Help", "About");
    assert!(
        !a_dialog_is_open(&mut cx),
        "Help > About opened over the Preferences sheet"
    );
    press(&mut cx, "ctrl-k");
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
    press(&mut cx, "ctrl-,");
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
    let page = bounds_of(&mut cx, PAGE_ROOT);
    hover(&mut cx, point(page.left() + px(4.), page.top() + px(4.)));
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
    use_preset(&mut cx, &showcase, "no-such-preset");
    let alert = bounds_of(&mut cx, CONTENT_ALERT);
    let tabs = bounds_of(&mut cx, CHROME_PAGE_TABS);
    let scroll = bounds_of(&mut cx, CONTENT_SCROLL);
    assert_eq!(
        tabs, tabs_before,
        "the page TabBar moved when the Alert appeared"
    );
    assert_eq!(
        alert.top(),
        tabs.bottom(),
        "the Alert at {alert:?} is not right under the page TabBar at {tabs:?}"
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

/// The About dialog's content is inset from its frame by the dialog's
/// padding (spec §3.6), the reported defect. Under kde-breeze that is
/// `Layout_TopLevelMarginWidth`, 10px (platform-facts §2.22), measured from
/// the frame's inner edge: upstream draws the frame with `border_1`
/// (dialog/dialog.rs:614) and pads each section inside it with the
/// refinement's sides (dialog/dialog.rs:540-552, :657-658).
#[gpui::test]
fn the_about_content_is_inset_by_the_dialogs_padding(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    use_preset(&mut cx, &showcase, "kde-breeze");
    without_motion(&mut cx);
    let padding = read(&mut cx, &showcase, |_this, cx| {
        cx.native_theme()
            .and_then(|nt| nt.native(cx))
            .map(|n| n.resolved.dialog.border.padding)
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
