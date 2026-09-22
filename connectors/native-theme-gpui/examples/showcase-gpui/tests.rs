//! Showcase self-tests (spec v0.5.9 §6.1)
//!
//! `test = true` on the example target (Cargo.toml) puts these under a plain
//! `cargo test`, so CI and the nightly dependency canary run them with no
//! workflow change. They build the real `Showcase` on GPUI's headless test
//! platform — the same view, the same `Root`, the same window width `main`
//! opens — and drive it with real input.

use gpui::{
    App, Axis, Bounds, Entity, Focusable as _, Modifiers, Pixels, Point, TestAppContext,
    VisualTestContext, point, prelude::*, px, size,
};
use gpui_base::{PANEL_MIN_SIZE, ScrollbarHandle as _};
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
    native_info,
};
use crate::support::{
    CAROUSEL_SLIDES, RESIZABLE_GROUPS, demo_border_width, native_geometry, native_value,
};
use crate::{
    CHROME_APP_MENU_BAR, CHROME_TITLE_BAR, CONTENT_SCROLL, LIST_DEMO, PROBE_ALERT_DIALOG,
    PROBE_ATTACHMENT, PROBE_CAROUSEL_LAST, PROBE_CHAT_SEND, PROBE_CLIPBOARD, PROBE_COLOR_MODE,
    PROBE_COMBOBOX, PROBE_NOTIFICATION, PROBE_PAGINATION, PROBE_RATING, PROBE_SETTINGS_ROW,
    PROBE_SIDEBAR_TOGGLE, PROBE_STEPPER, SIDEBAR_COLUMN, TAB_ROOT, TREE_DEMO, Tab, WIDGET_INFO,
    WIDGET_INFO_TEXT, WINDOW_SIZE,
};

/// The window the interaction test lays the showcase out in.
///
/// The width is the application's own, so the horizontal resizable group is
/// measured at the width it really gets. The height is not: a tab is one
/// long scrolling column, and an element scrolled out of the viewport is
/// clipped out of the frame and cannot be clicked, so this window is tall
/// enough to hold the longest tab whole. `every_tab_lays_out` uses
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
/// — so gpui-component's own `IconName` SVGs resolve to nothing. The tabs
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

/// Switch to `tab` and draw the frame that shows it.
fn show(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, tab: Tab) {
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.active_tab = tab;
            cx.notify();
        });
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

/// Every tab lays out: the bar's ten tabs each render on the test platform,
/// each leaves a tab root behind, and that root has a size.
#[gpui::test]
fn every_tab_lays_out(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, WINDOW_SIZE);
    assert_eq!(Tab::ALL.len(), 10, "the bar no longer has ten tabs");
    for tab in Tab::ALL {
        show(&mut cx, &showcase, tab);
        assert_eq!(read(&mut cx, &showcase, |this, _| this.active_tab), tab);
        let bounds = cx
            .debug_bounds(TAB_ROOT)
            .unwrap_or_else(|| panic!("{tab:?}: nothing was laid out under the tab bar"));
        assert!(
            bounds.size.width > px(0.) && bounds.size.height > px(0.),
            "{tab:?}: the tab root laid out at {:?}",
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
        show(&mut cx, &showcase, Tab::Inputs);
        let (groove, overlay) = scrollbar_of(&mut cx, &showcase);
        assert!(
            groove > px(0.),
            "{preset}: no native theme is installed, so nothing was measured"
        );
        let content = bounds_of(&mut cx, CONTENT_SCROLL);
        let tab = bounds_of(&mut cx, TAB_ROOT);
        assert!(
            tab.size.height > WINDOW_SIZE.height,
            "{preset}: the tab is shorter than the whole window, so the pane may \
                 not scroll at all and this step would prove nothing"
        );
        let gutter = if overlay { px(0.) } else { groove };
        assert_eq!(
            content.right() - tab.right(),
            gutter,
            "{preset}: the tab ends at {:?} and the scroll area at {:?}, which \
                 leaves {gutter:?} free for a {groove:?} scrollbar",
            tab.right(),
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
    show(&mut cx, &showcase, Tab::Layout);
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

/// A Widget Info text long enough to overflow whatever room the sidebar
/// has, so the panel cannot be as tall as its content and has to scroll.
fn long_info() -> String {
    (1..=200)
        .map(|line| format!("line {line}: a themed property and where it comes from\n"))
        .collect()
}

/// Drive the Widget Info panel the way a hover does.
fn set_info(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, text: String) {
    cx.update(|_window, cx| {
        let panel = showcase.read(cx).widget_info_panel.clone();
        panel.update(cx, |p, cx| p.set_text(text, cx));
    });
    cx.run_until_parked();
    draw(cx);
}

/// The Widget Info panel takes every pixel the sidebar column has left
/// under the controls above it, whatever the window size.
///
/// The panel is the last child of the sidebar column and the only one that
/// grows, so its bottom edge is the column's; the box that holds the
/// textarea then reaches the panel's bottom padding, which is the panel's
/// top padding read off the frame rather than a number typed out again.
/// A text far taller than the window is used, because a panel that sized
/// itself to its content would pass a short one.
#[gpui::test]
fn the_widget_info_panel_fills_the_sidebar(cx: &mut TestAppContext) {
    for height in [WINDOW_SIZE.height, WINDOW_SIZE.height + px(400.)] {
        let (showcase, _root, mut cx) = open(cx, size(WINDOW_SIZE.width, height));
        use_preset(&mut cx, &showcase, "kde-breeze");
        set_info(&mut cx, &showcase, long_info());

        let sidebar = bounds_of(&mut cx, SIDEBAR_COLUMN);
        let panel = bounds_of(&mut cx, WIDGET_INFO);
        let text = bounds_of(&mut cx, WIDGET_INFO_TEXT);
        assert!(
            panel.size.height > px(0.),
            "{height:?}: the sidebar left the panel no room at all"
        );
        assert_eq!(
            panel.bottom(),
            sidebar.bottom(),
            "{height:?}: the sidebar column ends at {:?} and the panel at {:?}, \
                 so {:?} of it is unused",
            sidebar.bottom(),
            panel.bottom(),
            sidebar.bottom() - panel.bottom(),
        );
        // The panel's padding is uniform, so the gap its left edge leaves
        // is the gap its bottom edge has to leave.
        let padding = text.left() - panel.left();
        assert!(
            padding > px(0.),
            "{height:?}: the panel has no padding, so this step measures nothing"
        );
        assert_eq!(
            text.bottom(),
            panel.bottom() - padding,
            "{height:?}: the text box ends at {:?} and the panel's bottom padding at {:?}, \
                 so the textarea is {:?} shorter than the room it has",
            text.bottom(),
            panel.bottom() - padding,
            panel.bottom() - padding - text.bottom(),
        );
    }
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

/// A window tall enough to show the Data tab's List and Tree demos without
/// scrolling the page first, and still far shorter than that tab, so the
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
    show(&mut cx, &showcase, Tab::Data);
    assert!(
        bounds_of(&mut cx, TAB_ROOT).size.height > NESTED_SCROLL_WINDOW.height,
        "the Data tab fits in the window, so the page could not scroll either way"
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

/// Every resizable group has room to drag: rationale §1.1 as a rule.
///
/// gpui-base clamps each panel to `PANEL_MIN_SIZE`, so a box narrower —
/// or shorter — than its panels' minimums put together holds a divider
/// that cannot move. The sizes come from the frame the showcase just drew
/// and the panel counts from the same `RESIZABLE_GROUPS` entries the render
/// code builds from, so neither is a number this test types out again.
#[gpui::test]
fn resizable_groups_have_room_to_drag(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    show(&mut cx, &showcase, Tab::Layout);
    assert!(!RESIZABLE_GROUPS.is_empty());
    for group in RESIZABLE_GROUPS {
        let bounds = bounds_of(&mut cx, group.id);
        let outer = match group.axis {
            Axis::Horizontal => bounds.size.width,
            Axis::Vertical => bounds.size.height,
        };
        let border = cx.update(|_w, cx| demo_border_width(cx));
        let room = outer - border * 2.;
        let needed = PANEL_MIN_SIZE * group.panels.len() as f32;
        assert!(
            room > needed,
            "{}: {} panels need more than {needed:?} between the borders, the box leaves {room:?} — the divider cannot move",
            group.id,
            group.panels.len(),
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

    // --- Buttons tab --------------------------------------------------
    show(&mut cx, &showcase, Tab::Buttons);

    // Clipboard: the card is one icon button, and the test platform holds
    // a real in-memory clipboard.
    click(&mut cx, PROBE_CLIPBOARD);
    assert_eq!(
        cx.read_from_clipboard().and_then(|item| item.text()),
        Some("cargo add native-theme".to_string()),
        "Clipboard: the Copy button wrote nothing"
    );

    // --- Inputs tab ---------------------------------------------------
    show(&mut cx, &showcase, Tab::Inputs);

    // Rating: clicking a star at or below the current value clears down to
    // the one before it (rating.rs, Rating::render on_click), so the first
    // star takes the three the showcase starts with to none.
    click(&mut cx, PROBE_RATING);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.rating_value),
        0,
        "Rating: clicking the first star left the value alone"
    );

    // Combobox: the trigger opens the list, and Enter takes the row the
    // list has under the cursor.
    click(&mut cx, PROBE_COMBOBOX);
    cx.simulate_keystrokes("down enter");
    draw(&mut cx);
    assert!(
        read(&mut cx, &showcase, |this, cx| !this
            .combobox_state
            .read(cx)
            .selection()
            .is_empty()),
        "Combobox: opening the list and confirming a row selected nothing"
    );

    // --- Data tab -----------------------------------------------------
    show(&mut cx, &showcase, Tab::Data);

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

    // --- Layout tab ---------------------------------------------------
    show(&mut cx, &showcase, Tab::Layout);

    // Stepper: the steps run left to right, so the leading one is the first.
    click(&mut cx, PROBE_STEPPER);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.step),
        0,
        "Stepper: clicking the first step left the selection alone"
    );

    // SidebarToggleButton: the flag it draws is the flag it flips.
    click(&mut cx, PROBE_SIDEBAR_TOGGLE);
    assert!(
        read(&mut cx, &showcase, |this, _| this.sidebar_collapsed),
        "SidebarToggleButton: the sidebar did not collapse"
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

    // --- Overlays tab -------------------------------------------------
    show(&mut cx, &showcase, Tab::Overlays);

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

    // --- Feedback tab -------------------------------------------------
    show(&mut cx, &showcase, Tab::Feedback);

    // Notification: the button pushes one onto the Root's own layer.
    let before = cx.update(|_w, cx| root.read(cx).notification.read(cx).notifications().len());
    click(&mut cx, PROBE_NOTIFICATION);
    assert_eq!(
        cx.update(|_w, cx| root.read(cx).notification.read(cx).notifications().len()),
        before + 1,
        "Notification: nothing was pushed"
    );

    // --- The sidebar's colour mode switch -----------------------------
    //
    // The list's rows are System, Light, Dark. Which one is asked for is
    // decided from the mode this host's theme is actually in, so the step
    // is always a change: a fixed "Dark" would assert nothing on a desktop
    // that is already dark.
    let was_dark = cx.update(|_w, cx| Theme::global(cx).mode.is_dark());
    let (keystrokes, wanted) = match was_dark {
        true => ("down enter", AppColorMode::Light),
        false => ("down down enter", AppColorMode::Dark),
    };
    click(&mut cx, PROBE_COLOR_MODE);
    cx.simulate_keystrokes(keystrokes);
    draw(&mut cx);
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.color_mode),
        wanted,
        "the colour mode switch did not reach {wanted:?}"
    );
    assert_eq!(
        cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
        !was_dark,
        "the colour mode switch did not reach Theme::mode"
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
    show(&mut cx, &showcase, Tab::Inputs);
    cx.update(|window, cx| {
        let input = showcase.read(cx).input_state.clone();
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();
    draw(&mut cx);

    run_menu_item_from_focus(&mut cx, "View", "Buttons");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_tab),
        Tab::Buttons,
        "View > Buttons did not leave the Inputs page"
    );
    run_menu_item_from_focus(&mut cx, "View", "Inputs");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_tab),
        Tab::Inputs,
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
        read(&mut cx, &showcase, |this, _| this.active_tab),
        Tab::Feedback,
        "the showcase starts on Feedback, so showing it proves nothing"
    );
    assert!(
        menu_action("View", "Feedback").partial_eq(&ShowPage(3)),
        "View > Feedback does not carry ShowPage(3)"
    );
    run_menu_item(&mut cx, "View", "Feedback");
    assert_eq!(
        read(&mut cx, &showcase, |this, _| this.active_tab),
        Tab::Feedback,
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
