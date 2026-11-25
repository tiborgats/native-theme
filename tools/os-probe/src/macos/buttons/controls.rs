//! The controls pass (`--button-pass --controls`): the controls other than
//! the push buttons (check boxes and radio buttons, enabled and disabled,
//! disclosure buttons, a pop-up, a text field, a segmented control, a
//! switch, a slider, two scroll views whose scrollers' knobs, legacy and
//! overlay, are the targets, two tables, a tab view, a split view, a box and
//! a toolbar button), one sheet of them in a titled window made key, each
//! enabled one hovered by a posted mouse-moved event and pressed by a real
//! mouse-down captured during the press, as the button pass
//! hovers and clicks its push buttons; then the toolbar sheet, the toolbar
//! button again in a window whose content view lies under its transparent
//! title bar. Every state is captured over the window's own background and
//! the three opaque backdrops, so each state's layer is solved
//! ([`super::layer`]). The analysis reads each control's part from the
//! layers at the rects this pass records.

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{MainThreadOnly, Message};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSAttributedStringNSStringDrawing, NSBezelStyle,
    NSBox, NSBoxType, NSButton, NSButtonType, NSClipView, NSColorSpace, NSControl, NSControlSize,
    NSEvent, NSPopUpButton, NSScrollView, NSScroller, NSScrollerPart, NSScrollerStyle,
    NSSegmentedControl, NSSlider, NSSliderCell, NSSplitView, NSSwitch, NSTabView, NSTabViewItem,
    NSTableCellView, NSTableColumn, NSTableView, NSTableViewStyle, NSTextField, NSTitlePosition,
    NSToolbar, NSToolbarItem, NSView, NSVisualEffectBlendingMode, NSVisualEffectView, NSWindow,
    NSWindowStyleMask,
};
use objc2_core_graphics::{CGEvent, CGEventTapLocation, CGScrollEventUnit};
use objc2_foundation::{NSIndexSet, NSPoint, NSRect, NSSize, NSString};
use toml::{Table, Value};

use super::{
    Backdrop, Case, DIFF_MARGIN, NO_WINDOW, Parking, Pass, Sheet, case_table, diff_table,
    from_top_left, px_ratio, sheet_window,
};
use crate::macos::layer::{PxRect, Rgba};
use crate::macos::{
    APPEARANCES, SETTLE_STEPS, control_common, ffi, ns, pixel_index, point_text, rect, rect_text,
    state_value,
};
use crate::out;

/// The sheet's window origin, in AppKit's screen points.
const CONTROLS_ORIGIN: NSPoint = NSPoint::new(40.0, 60.0);

/// The sheet's content size, in points.
const CONTROLS_CONTENT: NSSize = NSSize::new(900.0, 540.0);

/// The rows of single controls, from the content's top: each row's height
/// (`place`'s).
const ROW_STEP: f64 = super::BUTTON_ROW_STEP;

/// The x of each column of the single controls' rows.
const COLUMN_X: [f64; 5] = [20.0, 180.0, 340.0, 520.0, 700.0];

/// The tables' scroll views, the tab view, the split view and the box, in
/// the content view's points.
const TABLE_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 20.0), NSSize::new(200.0, 120.0));
const SOURCE_LIST_FRAME: NSRect = NSRect::new(NSPoint::new(240.0, 20.0), NSSize::new(200.0, 120.0));
const TAB_VIEW_FRAME: NSRect = NSRect::new(NSPoint::new(460.0, 20.0), NSSize::new(260.0, 200.0));
const SPLIT_FRAME: NSRect = NSRect::new(NSPoint::new(740.0, 20.0), NSSize::new(140.0, 120.0));
const BOX_FRAME: NSRect = NSRect::new(NSPoint::new(740.0, 160.0), NSSize::new(140.0, 80.0));

/// The scroll views, a vertical scroller in the legacy style and one in the
/// overlay style, between the tables and the slider's row.
const SCROLL_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 180.0), NSSize::new(200.0, 120.0));
const OVERLAY_SCROLL_FRAME: NSRect =
    NSRect::new(NSPoint::new(240.0, 180.0), NSSize::new(200.0, 120.0));

/// The scroll views' document height, as a multiple of the scroll view's.
const SCROLL_DOCUMENT_FACTOR: f64 = 3.0;

/// The lines of each scroll `reveal` posts (down, then as many up), and the
/// run-loop steps after each.
const REVEAL_LINES: i32 = 1;
const REVEAL_STEPS: u32 = 2;

/// The toolbar sheet's content size, in points (its window at
/// `CONTROLS_ORIGIN`, the controls sheet's window closed by then): the
/// controls sheet's width, at which macOS 26 shows the toolbar's item in the
/// toolbar (at 320 pt it put the item in the overflow menu).
const TOOLBAR_CONTENT: NSSize = NSSize::new(CONTROLS_CONTENT.width, 120.0);

/// The toolbars' identifiers: one each, so AppKit does not keep the two in
/// step.
const CONTROLS_TOOLBAR_ID: &str = "NativeThemeOsProbeControls";
const TOOLBAR_SHEET_ID: &str = "NativeThemeOsProbeControlsToolbar";

/// The tables' rows and the column width.
const TABLE_ROWS: usize = 3;
const TABLE_COLUMN_WIDTH: f64 = 180.0;

/// The row the pass hovers and presses in each table.
const TABLE_TARGET_ROW: isize = 1;

/// The segmented control's segments, each this wide, segment 0 selected:
/// with three segments of one width the middle one's centre is the
/// control's.
const SEGMENTS: [&str; 3] = ["One", "Two", "Six"];
const SEGMENT_WIDTH: f64 = 60.0;

/// The titled disclosure button's width after it is sized to fit (which
/// leaves it the triangle's width): room for the triangle and the whole
/// title within its column.
const DISCLOSURE_WIDTH: f64 = 160.0;

/// The slider's width after it is sized to fit, its range and value.
const SLIDER_WIDTH: f64 = 160.0;
const SLIDER_VALUE: f64 = 50.0;

/// The tab view's tabs, the first selected; the pass hovers and presses
/// the second.
const TABS: [&str; 3] = ["One", "Two", "Three"];
const TAB_TARGET: usize = 1;

/// The step, in points, of the scan that finds each tab's rect with
/// `tabViewItemAtPoint:`.
const TAB_SCAN_STEP: f64 = 1.0;

/// The titles the controls show.
const TITLE: &str = "Button";
const CHECK_TITLE: &str = "Check";
const RADIO_TITLE: &str = "Radio";

/// The targets whose title is read (`title_reads`): the titled disclosure
/// button, the check boxes and the radio buttons.
const TITLED_TARGETS: [&str; 9] = [
    "disclosure",
    "checkbox_off",
    "checkbox_on",
    "radio_off",
    "radio_on",
    "checkbox_disabled_off",
    "checkbox_disabled_on",
    "radio_disabled_off",
    "radio_disabled_on",
];
const DISCLOSURE_TITLE: &str = "Details";
const FIELD_TEXT: &str = "Text";
const POPUP_ITEMS: [&str; 2] = ["Option", "Other"];
const ROW_TEXT: &str = "Row";

/// One place the pass hovers and presses.
struct Target {
    name: &'static str,
    /// The view the target lies in.
    view: Retained<NSView>,
    /// The part the target is, in the view's own coordinates.
    part: NSRect,
    /// Whether the backdrop shows behind the part (`place_targets`).
    over_backdrop: bool,
    /// Whether the pass hovers it.
    hover: bool,
    /// Whether the pass presses it.
    press: bool,
    /// Whether an Escape follows the mouse-up (the pop-up's menu).
    escape: bool,
    /// What the control is set back to after each press, with what it
    /// reads back into the press's table.
    reset: Reset,
    /// The scroll view whose scroller's knob the target is: its scrollers
    /// are flashed before each hovered and pressed capture.
    scroll: Option<Retained<NSScrollView>>,
}

/// A target's reset (`Target::reset`).
type Reset = Box<dyn Fn(&mut Table)>;

/// A target as `build` makes it: its part is found in its view
/// (`locate`) once the window is shown and settled.
struct Pending {
    name: &'static str,
    view: Retained<NSView>,
    part: Part,
    hover: bool,
    press: bool,
    escape: bool,
    reset: Reset,
}

/// How a target's part is found in its view.
#[derive(Clone, Copy)]
enum Part {
    /// The view's bounds.
    Bounds,
    /// A check box's indicator: its cell's `imageRectForBounds:`.
    Indicator,
    /// A titled button's triangle and title (`titled_part`).
    Titled,
    /// The middle segment of a segmented control of segments of one width.
    MiddleThird,
    /// A slider's knob: its cell's `knobRectFlipped:`.
    Knob,
    /// A table's row `TABLE_TARGET_ROW`: `rectOfRow:`.
    Row,
    /// The tab view's tab `TAB_TARGET` (`tab_rects`).
    Tab,
    /// A split view's divider: from the first pane's right edge,
    /// `dividerThickness` wide, the view's height.
    Divider,
    /// A scroll view's vertical scroller's knob: the scroller's
    /// `rectForPart:` of `NSScrollerKnob`, in the scroll view's coordinates.
    ScrollerKnob,
}

/// The tab view and each tab's rect in its coordinates, where found.
struct Tabs {
    view: Retained<NSTabView>,
    rects: Vec<Option<NSRect>>,
}

/// The objects the sheet's views need alive while it is captured, and the
/// views read once the window is shown.
struct Keep {
    _sources: Vec<Retained<ffi::TableSource>>,
    tables: Vec<Retained<NSTableView>>,
    _delegate: Retained<ffi::ToolbarDelegate>,
    tab_view: Retained<NSTabView>,
    /// The scroll views, each with its target's name and requested style.
    scrolls: Vec<(&'static str, NSScrollerStyle, Retained<NSScrollView>)>,
}

/// `--button-pass --controls`: the controls sheet, made key, captured per
/// appearance at rest, each target hovered and each target pressed.
pub(super) fn controls_table(pass: &Pass<'_>) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    let (sheet, pending, keep) = match build(pass) {
        Ok(built) => built,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let window = &sheet.window;
    let other = pass.present(window, &mut t);
    pass.settle(window, SETTLE_STEPS);
    let mut shown = Table::new();
    for (name, style, scroll) in &keep.scrolls {
        out::put(
            &mut shown,
            name,
            out::text(scroller_style_name(scroll.scrollerStyle())),
        );
        scroll.setScrollerStyle(*style);
        scroll.tile();
        scroll_to_middle(scroll);
    }
    pass.settle(window, SETTLE_STEPS);

    sheet_reads(&mut t, &sheet);
    let frame_size = window.frame().size;
    let tabs = Tabs {
        rects: tab_rects(&keep.tab_view),
        view: keep.tab_view.clone(),
    };
    let mut placed = Table::new();
    let targets = place_targets(pending, &tabs.rects, &sheet, &mut placed, &|target, row| {
        if TITLED_TARGETS.contains(&target.name)
            && let Some(button) = target.view.downcast_ref::<NSButton>()
        {
            title_reads(row, button, frame_size);
        }
    });
    out::put_table(&mut t, "targets", placed);
    out::put_table(&mut t, "tabs", tabs_table(&tabs, frame_size));
    out::put_table(&mut t, "scroll_views", {
        let mut scroll_views = scroll_views_table(pass, &keep.scrolls);
        out::put_table(&mut scroll_views, "scrollerStyle_when_shown", shown);
        scroll_views
    });

    let scroll_reads = |row: &mut Table| {
        let mut styles = Table::new();
        for (name, style, scroll) in &keep.scrolls {
            let mut s = Table::new();
            out::put(
                &mut s,
                "scrollerStyle_read",
                out::text(scroller_style_name(scroll.scrollerStyle())),
            );
            scroll.setScrollerStyle(*style);
            scroll.tile();
            scroll_to_middle(scroll);
            out::put(
                &mut s,
                "scrollerStyle_after_set",
                out::text(scroller_style_name(scroll.scrollerStyle())),
            );
            out::put_table(&mut styles, name, s);
        }
        out::put_table(row, "scroll_views", styles);
    };
    appearances(pass, &sheet, &targets, &scroll_reads, &mut t);
    close(window, other);
    for table in &keep.tables {
        ffi::set_table_source(table, None);
    }
    window.setToolbar(None);
    drop(keep);
    out::put_table(&mut t, "toolbar_sheet", toolbar_sheet_table(pass));
    t
}

/// The toolbar sheet: a window whose content view lies under its
/// transparent title bar, so that the backdrop box lies under the toolbar
/// too and the toolbar button's layer is solved from the black and white
/// captures as the controls' are; the controls sheet's toolbar button,
/// captured as the controls sheet's targets are.
fn toolbar_sheet_table(pass: &Pass<'_>) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(toolbar_method_text()));
    let (sheet, pending, delegate) = match build_toolbar_sheet(pass) {
        Ok(built) => built,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let window = &sheet.window;
    let other = pass.present(window, &mut t);
    pass.settle(window, SETTLE_STEPS);
    sheet.backdrop.setFrame(sheet.content.bounds());
    pass.settle(window, SETTLE_STEPS);

    out::put(
        &mut t,
        "styleMask_fullSizeContentView",
        out::boolean(
            window
                .styleMask()
                .contains(NSWindowStyleMask::FullSizeContentView),
        ),
    );
    out::put(
        &mut t,
        "titlebarAppearsTransparent",
        out::boolean(window.titlebarAppearsTransparent()),
    );
    out::put(
        &mut t,
        "content_in_window_from_top_left_xywh",
        rect(from_top_left(&sheet.content, window.frame().size)),
    );
    sheet_reads(&mut t, &sheet);
    let mut placed = Table::new();
    let targets = place_targets(pending, &[], &sheet, &mut placed, &|_, _| {});
    out::put_table(&mut t, "targets", placed);

    appearances(pass, &sheet, &targets, &|_| {}, &mut t);
    close(window, other);
    window.setToolbar(None);
    drop(delegate);
    t
}

/// The sheet's cases (`controls`), the window's backing scale and frame, and
/// the point the backdrop is read at.
fn sheet_reads(t: &mut Table, sheet: &Sheet) {
    let window = &sheet.window;
    let frame_size = window.frame().size;
    let mut buttons = Table::new();
    for case in &sheet.cases {
        out::put_table(&mut buttons, &case.name, case_table(case, frame_size));
    }
    out::put_table(t, "controls", buttons);
    out::put(
        t,
        "window_backingScaleFactor",
        out::float(window.backingScaleFactor()),
    );
    out::put(t, "window_frame_xywh", rect(window.frame()));
    out::put(
        t,
        "backdrop_point_xy",
        out::floats(&[sheet.backdrop_point().x, sheet.backdrop_point().y]),
    );
}

/// Each pending target's part found (`locate`): the targets found, and in
/// `placed` each target's place (with `extra`'s reads) or its error.
fn place_targets(
    pending: Vec<Pending>,
    tab_rects: &[Option<NSRect>],
    sheet: &Sheet,
    placed: &mut Table,
    extra: &dyn Fn(&Target, &mut Table),
) -> Vec<Target> {
    let frame_size = sheet.window.frame().size;
    let backdrop_rect = from_top_left(&sheet.backdrop, frame_size);
    let mut targets: Vec<Target> = Vec::new();
    let mut rows: Vec<(&'static str, Table)> = Vec::new();
    for p in pending {
        match locate(&p, tab_rects) {
            Ok(part) => {
                let knob = matches!(p.part, Part::ScrollerKnob);
                let (behind, described) = behind_part(
                    &p.view,
                    part,
                    &sheet.content,
                    &sheet.backdrop,
                    knob,
                    frame_size,
                );
                let covered = covering(&behind);
                let over_backdrop = contains(backdrop_rect, in_window(&p.view, part, frame_size))
                    && covered.is_empty();
                let target = Target {
                    name: p.name,
                    scroll: if knob {
                        p.view
                            .downcast_ref::<NSScrollView>()
                            .map(|scroll| scroll.retain())
                    } else {
                        None
                    },
                    view: p.view,
                    part,
                    over_backdrop,
                    hover: p.hover,
                    press: p.press,
                    escape: p.escape,
                    reset: p.reset,
                };
                let mut row = target_table(&target, frame_size);
                out::put(&mut row, "over_backdrop", out::boolean(over_backdrop));
                if !covered.is_empty() {
                    out::put(
                        &mut row,
                        "backdrop_covered_by",
                        out::array(covered.into_iter().map(out::text).collect()),
                    );
                }
                out::put(&mut row, "behind_part", out::array(described));
                if let Some(table) = target.view.downcast_ref::<NSTableView>()
                    && let Some(scroll) = table.enclosingScrollView()
                {
                    out::put(
                        &mut row,
                        "enclosing_scroll_view_tree",
                        out::array(view_tree(&scroll, frame_size)),
                    );
                }
                extra(&target, &mut row);
                rows.push((target.name, row));
                targets.push(target);
            }
            Err(reason) => {
                let mut row = Table::new();
                out::put(
                    &mut row,
                    "class",
                    out::text(p.view.class().name().to_string_lossy()),
                );
                out::put(
                    &mut row,
                    "view_in_window_from_top_left_xywh",
                    if p.view.window().is_some() {
                        rect(from_top_left(&p.view, frame_size))
                    } else {
                        out::unavailable(NO_WINDOW)
                    },
                );
                out::put(&mut row, "error", out::unavailable(reason));
                out::put_table(placed, p.name, row);
            }
        }
    }
    for (name, row) in rows {
        out::put_table(placed, name, row);
    }
    targets
}

/// Per appearance, `reads` (what the sheet reads once the appearance is
/// set, before its states) and the sheet's states (`appearance_table`).
fn appearances(
    pass: &Pass<'_>,
    sheet: &Sheet,
    targets: &[Target],
    reads: &dyn Fn(&mut Table),
    t: &mut Table,
) {
    let window = &sheet.window;
    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        window.setAppearance(Some(&appearance));
        sheet.content.setNeedsDisplay(true);
        pass.settle(window, SETTLE_STEPS);
        let mut read = Table::new();
        reads(&mut read);
        pass.settle(window, SETTLE_STEPS);
        let mut states = appearance_table(pass, sheet, targets, key);
        for (k, v) in read {
            states.insert(k, v);
        }
        out::put_table(t, key, states);
    }
    window.setAppearance(None);
}

/// The sheet's window and the mode's other window, if any, ordered out and
/// closed.
fn close(window: &NSWindow, other: Option<Retained<NSWindow>>) {
    window.orderOut(None);
    window.close();
    if let Some(other) = other {
        other.orderOut(None);
        other.close();
    }
}

/// Each scroll view's scroller style, requested and read back, and its
/// vertical scroller's state once the window is shown and settled; and the
/// system's preferred scroller style.
fn scroll_views_table(
    pass: &Pass<'_>,
    scrolls: &[(&'static str, NSScrollerStyle, Retained<NSScrollView>)],
) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "NSScroller_preferredScrollerStyle",
        out::text(scroller_style_name(NSScroller::preferredScrollerStyle(
            pass.mtm,
        ))),
    );
    for (name, requested, scroll) in scrolls {
        let mut row = Table::new();
        out::put(
            &mut row,
            "scrollerStyle_requested",
            out::text(scroller_style_name(*requested)),
        );
        out::put(
            &mut row,
            "scrollerStyle",
            out::text(scroller_style_name(scroll.scrollerStyle())),
        );
        out::put(
            &mut row,
            "drawsBackground",
            out::boolean(scroll.drawsBackground()),
        );
        out::put(
            &mut row,
            "contentView_drawsBackground",
            out::boolean(scroll.contentView().drawsBackground()),
        );
        match scroll.verticalScroller() {
            Some(scroller) => {
                out::put(
                    &mut row,
                    "verticalScroller_scrollerStyle",
                    out::text(scroller_style_name(scroller.scrollerStyle())),
                );
                out::put(
                    &mut row,
                    "verticalScroller_isHidden",
                    out::boolean(scroller.isHidden()),
                );
                out::put(
                    &mut row,
                    "verticalScroller_alphaValue",
                    out::float(scroller.alphaValue()),
                );
                out::put(
                    &mut row,
                    "verticalScroller_knobProportion",
                    out::float(scroller.knobProportion()),
                );
                out::put(
                    &mut row,
                    "verticalScroller_doubleValue",
                    out::float(scroller.doubleValue()),
                );
            }
            None => out::put(
                &mut row,
                "verticalScroller",
                out::unavailable("the scroll view has no vertical scroller"),
            ),
        }
        out::put_table(&mut t, name, row);
    }
    t
}

/// A scroller style's name.
fn scroller_style_name(style: NSScrollerStyle) -> String {
    match style {
        NSScrollerStyle::Legacy => "NSScrollerStyleLegacy".to_owned(),
        NSScrollerStyle::Overlay => "NSScrollerStyleOverlay".to_owned(),
        NSScrollerStyle(other) => format!("unknown ({other})"),
    }
}

/// One appearance: normal, highlighted (`highlight:` on the NSButtons), and
/// per target hovered and pressed, each over every backdrop.
fn appearance_table(pass: &Pass<'_>, sheet: &Sheet, targets: &[Target], key: &str) -> Table {
    let mut t = Table::new();
    let (dir, rel) = sheet.folder(pass.out_dir, key);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        out::put(
            &mut t,
            "error",
            out::unavailable(format!("cannot create {}: {e}", dir.display())),
        );
        return t;
    }
    let window = &sheet.window;
    let all: Vec<&Case> = sheet.cases.iter().collect();

    let mut normal = Table::new();
    out::put_table(&mut normal, "cursor", pass.park(window, Parking::Controls));
    let normal_shots = pass.shots(sheet, &mut normal, &dir, &rel, "normal", &all, false, None);
    out::put_table(
        &mut normal,
        "backdrop_shows",
        backdrop_shows(
            targets,
            normal_shots.black.as_ref(),
            normal_shots.white.as_ref(),
            window.frame().size,
        ),
    );
    let normal_rgba = normal_shots.window;
    out::put_table(&mut t, "normal", normal);

    let mut highlighted = Table::new();
    for case in &sheet.cases {
        if let Some(b) = &case.button
            && b.window().is_some()
        {
            b.highlight(true);
        }
    }
    pass.settle(window, SETTLE_STEPS);
    let lit: Vec<Value> = sheet
        .cases
        .iter()
        .filter(|c| {
            c.button
                .as_ref()
                .and_then(|b| b.cell())
                .is_some_and(|cell| cell.isHighlighted())
        })
        .map(|c| out::text(c.name.clone()))
        .collect();
    out::put(&mut highlighted, "cells_highlighted", out::array(lit));
    pass.shots(
        sheet,
        &mut highlighted,
        &dir,
        &rel,
        "highlighted",
        &all,
        false,
        None,
    );
    for case in &sheet.cases {
        if let Some(b) = &case.button {
            b.highlight(false);
        }
    }
    pass.settle(window, SETTLE_STEPS);
    out::put_table(&mut t, "highlighted", highlighted);

    let frame = window.frame().size;
    let mut hovered = Table::new();
    for target in targets.iter().filter(|target| target.hover) {
        let stem = format!("hovered-{}", target.name);
        let (mut h, rgba) = if target.scroll.is_some() {
            over_backdrops(pass, sheet, &dir, &rel, &stem, &|sub, file| {
                out::put_table(
                    sub,
                    "cursor",
                    pass.move_cursor(window, screen_point(window, target)),
                );
                reveal(pass, window, target, sub);
                pass.shot(window, sub, &dir, &rel, file)
            })
        } else {
            let mut h = Table::new();
            out::put_table(
                &mut h,
                "cursor",
                pass.move_cursor(window, screen_point(window, target)),
            );
            let rgba = pass
                .shots(sheet, &mut h, &dir, &rel, &stem, &[], false, None)
                .window;
            (h, rgba)
        };
        knob_drawn(&mut h, target, rgba.as_ref(), frame);
        out::put(
            &mut h,
            "diff_from_normal",
            compare(normal_rgba.as_ref(), rgba.as_ref(), target, frame),
        );
        out::put_table(&mut hovered, target.name, h);
    }
    out::put_table(
        &mut hovered,
        "cursor_parked",
        pass.park(window, Parking::Controls),
    );
    out::put_table(&mut t, "hovered", hovered);

    let mut pressed = Table::new();
    for target in targets.iter().filter(|target| target.press) {
        let p = press_table(pass, sheet, target, &dir, &rel, normal_rgba.as_ref());
        out::put_table(&mut pressed, target.name, p);
    }
    out::put_table(
        &mut pressed,
        "cursor_parked",
        pass.park(window, Parking::Controls),
    );
    out::put_table(&mut t, "pressed", pressed);
    t
}

/// One target pressed over every backdrop in turn (a press per backdrop,
/// the control set back after each), and the press's layer solved from the
/// black and white captures.
fn press_table(
    pass: &Pass<'_>,
    sheet: &Sheet,
    target: &Target,
    dir: &std::path::Path,
    rel: &str,
    normal: Option<&Rgba>,
) -> Table {
    let window = &sheet.window;
    let frame = window.frame().size;
    let stem = format!("clicked-{}", target.name);
    let (mut t, window_rgba) = over_backdrops(pass, sheet, dir, rel, &stem, &|sub, file| {
        let at = screen_point(window, target);
        out::put_table(sub, "cursor", pass.move_cursor(window, at));
        reveal(pass, window, target, sub);
        let rgba = pass.click_at(sub, window, dir, rel, file, pass.quartz(at), target.escape);
        out::put(
            sub,
            "firstResponder_class_after_press",
            first_responder(window),
        );
        (target.reset)(sub);
        rgba
    });
    knob_drawn(&mut t, target, window_rgba.as_ref(), frame);
    out::put(
        &mut t,
        "diff_from_normal",
        compare(normal, window_rgba.as_ref(), target, frame),
    );
    t
}

/// One state captured over every backdrop in turn: per backdrop the
/// backdrop shown, then `capture`, given the backdrop's table and the file
/// (`<stem>[-over-<backdrop>].png`), sets the state and takes the capture,
/// in which the backdrop is then read; and the state's layer solved from
/// the black and white captures. The state's table and its capture over the
/// window's background.
fn over_backdrops(
    pass: &Pass<'_>,
    sheet: &Sheet,
    dir: &std::path::Path,
    rel: &str,
    stem: &str,
    capture: &dyn Fn(&mut Table, &str) -> Option<Rgba>,
) -> (Table, Option<Rgba>) {
    let window = &sheet.window;
    let frame = window.frame().size;
    let point = sheet.backdrop_point();
    let mut t = Table::new();
    let mut captured: Vec<(Backdrop, Option<Rgba>, Option<[u8; 4]>)> = Vec::new();
    for backdrop in Backdrop::ALL {
        sheet.show(backdrop);
        pass.settle(window, SETTLE_STEPS);
        let mut sub = Table::new();
        if let Some(b) = backdrop.byte() {
            out::put(
                &mut sub,
                "requested",
                out::text(crate::macos::layer::hex([b, b, b, 0xff])),
            );
        }
        let file = format!("{stem}{}.png", backdrop.suffix());
        let rgba = capture(&mut sub, &file);
        pass.settle(window, SETTLE_STEPS);
        let sampled = rgba.as_ref().and_then(|rgba| {
            let ratio = px_ratio(rgba.width, frame.width);
            let x = pixel_index(point.x, ratio, isize::try_from(rgba.width).ok()?)?;
            let y = pixel_index(point.y, ratio, isize::try_from(rgba.height).ok()?)?;
            rgba.pixel(usize::try_from(x).ok()?, usize::try_from(y).ok()?)
        });
        match sampled {
            Some(raw) => out::put(
                &mut sub,
                "backdrop_raw",
                out::text(crate::macos::layer::hex(raw)),
            ),
            None => out::put(
                &mut sub,
                "backdrop_raw",
                out::unavailable("no capture, or the point lies outside it"),
            ),
        }
        out::put_table(&mut t, backdrop.key(), sub);
        captured.push((backdrop, rgba, sampled));
    }
    sheet.show(Backdrop::Window);
    pass.settle(window, SETTLE_STEPS);

    let find = |b: Backdrop| {
        captured
            .iter()
            .find(|(k, _, _)| *k == b)
            .and_then(|(_, rgba, raw)| Some((rgba.as_ref()?, (*raw)?)))
    };
    let layer = match (find(Backdrop::Black), find(Backdrop::White)) {
        (Some(black), Some(white)) => {
            pass.layer(
                sheet,
                dir,
                rel,
                stem,
                &[],
                false,
                [black, white],
                [find(Backdrop::Grey), find(Backdrop::Window)],
                None,
            )
            .0
        }
        _ => {
            let mut l = Table::new();
            out::put(
                &mut l,
                "error",
                out::unavailable("the black or white capture or its backdrop is missing"),
            );
            l
        }
    };
    out::put_table(&mut t, "layer", layer);
    let window_rgba = captured
        .into_iter()
        .find(|(k, _, _)| *k == Backdrop::Window)
        .and_then(|(_, rgba, _)| rgba);
    (t, window_rgba)
}

/// The class of the window's first responder, or "nil".
fn first_responder(window: &NSWindow) -> Value {
    match window.firstResponder() {
        Some(responder) => out::text(responder.class().name().to_string_lossy()),
        None => out::text("nil"),
    }
}

/// Show the scrollers of the target's scroll view, where it has one, for
/// the capture that follows (an overlay scroller's knob is drawn only while
/// shown): with the cursor over it, where the window under the cursor is the
/// probe's (read before each scroll; where it is not, that scroll is not
/// posted and is the reveal's error), a scroll of `REVEAL_LINES` lines with
/// a positive `wheel1` and one of as many with a negative one posted at the
/// cursor, each followed by `REVEAL_STEPS` steps and the vertical
/// scroller's `doubleValue` read; then the scroll view scrolled to its
/// middle again and `flashScrollers`; into `t.reveal`.
fn reveal(pass: &Pass<'_>, window: &NSWindow, target: &Target, t: &mut Table) {
    let Some(scroll) = &target.scroll else {
        return;
    };
    let mut r = Table::new();
    match scroll.verticalScroller() {
        Some(scroller) => out::put(
            &mut r,
            "verticalScroller_doubleValue_before",
            out::float(scroller.doubleValue()),
        ),
        None => out::put(
            &mut r,
            "error",
            out::unavailable("the scroll view has no vertical scroller"),
        ),
    }
    let mut scrolls = Vec::new();
    for lines in [REVEAL_LINES, -REVEAL_LINES] {
        let mut s = Table::new();
        out::put(&mut s, "wheel1", out::int(i64::from(lines)));
        let under = NSWindow::windowNumberAtPoint_belowWindowWithWindowNumber(
            NSEvent::mouseLocation(),
            0,
            pass.mtm,
        );
        let ours = under == window.windowNumber();
        out::put(&mut s, "window_under_cursor_is_ours", out::boolean(ours));
        if ours {
            let event =
                CGEvent::new_scroll_wheel_event2(None, CGScrollEventUnit::Line, 1, lines, 0, 0);
            out::put(&mut s, "created", out::boolean(event.is_some()));
            match event.as_deref() {
                Some(event) => {
                    CGEvent::post(CGEventTapLocation::HIDEventTap, Some(event));
                    pass.settle(window, REVEAL_STEPS);
                }
                None => out::put(
                    &mut s,
                    "error",
                    out::unavailable("the scroll-wheel CGEvent is NULL: the scroll is not posted"),
                ),
            }
        } else {
            out::put(
                &mut s,
                "error",
                out::unavailable(format!(
                    "the window under the cursor is window {under}, not the probe's {}: the scroll is not posted",
                    window.windowNumber()
                )),
            );
        }
        if let Some(scroller) = scroll.verticalScroller() {
            out::put(
                &mut s,
                "verticalScroller_doubleValue_after",
                out::float(scroller.doubleValue()),
            );
        }
        scrolls.push(Value::Table(s));
    }
    out::put(&mut r, "scrolls", out::array(scrolls));
    scroll_to_middle(scroll);
    scroll.flashScrollers();
    out::put_table(t, "reveal", r);
}

/// For a scroller knob's target, whether its knob is drawn in `capture`
/// (the state's capture over the window's background): the pixels of the
/// knob's own rect (the vertical scroller's `rectForPart:` of
/// `NSScrollerKnob`, read now, the view scrolled as at the capture) that
/// differ from the scroll view's background, read at the middle of the
/// scroll view's clip view, into `t.knob_drawn`; where none does, the
/// target's `error` in `t`.
fn knob_drawn(t: &mut Table, target: &Target, capture: Option<&Rgba>, frame: NSSize) {
    let Some(scroll) = &target.scroll else {
        return;
    };
    let Some(rgba) = capture else {
        out::put(t, "knob_drawn", out::unavailable("no capture"));
        return;
    };
    let Some(scroller) = scroll.verticalScroller() else {
        let reason = out::unavailable("the scroll view has no vertical scroller");
        out::put(t, "knob_drawn", reason.clone());
        out::put(t, "error", reason);
        return;
    };
    let knob = in_window(
        &target.view,
        scroller.convertRect_toView(
            scroller.rectForPart(NSScrollerPart::Knob),
            Some(&target.view),
        ),
        frame,
    );
    let clip = scroll.contentView().frame();
    let middle = in_window(
        &target.view,
        NSRect::new(
            NSPoint::new(
                clip.origin.x + clip.size.width / 2.0,
                clip.origin.y + clip.size.height / 2.0,
            ),
            NSSize::new(0.0, 0.0),
        ),
        frame,
    )
    .origin;
    let ratio = px_ratio(rgba.width, frame.width);
    let background = (|| {
        let x = pixel_index(middle.x, ratio, isize::try_from(rgba.width).ok()?)?;
        let y = pixel_index(middle.y, ratio, isize::try_from(rgba.height).ok()?)?;
        rgba.pixel(usize::try_from(x).ok()?, usize::try_from(y).ok()?)
    })();
    let part =
        ratio.and_then(|ratio| PxRect::from_points(knob, 0.0, ratio, rgba.width, rgba.height));
    let (Some(background), Some(part)) = (background, part) else {
        let reason =
            out::unavailable("the background point or the knob's rect lies outside the capture");
        out::put(t, "knob_drawn", reason.clone());
        out::put(t, "error", reason);
        return;
    };
    let differing = differing_from(rgba, &part, background);
    let mut k = Table::new();
    out::put(&mut k, "knob_in_window_from_top_left_xywh", rect(knob));
    out::put(
        &mut k,
        "background_in_window_from_top_left_xy",
        out::floats(&[middle.x, middle.y]),
    );
    out::put(
        &mut k,
        "background_raw",
        out::text(crate::macos::layer::hex(background)),
    );
    out::put(&mut k, "part_px_xywh", part.value());
    out::put(&mut k, "differing_pixels", out::int(differing));
    out::put_table(t, "knob_drawn", k);
    if differing == 0 {
        out::put(
            t,
            "error",
            out::unavailable(
                "the knob is not drawn: no pixel of its rect differs from the scroll view's background",
            ),
        );
    }
}

/// The pixels of `part` in `rgba` that differ from `colour`.
fn differing_from(rgba: &Rgba, part: &PxRect, colour: [u8; 4]) -> i64 {
    let mut differing: i64 = 0;
    for y in part.y0..part.y1 {
        for x in part.x0..part.x1 {
            if rgba.pixel(x, y).is_some_and(|p| p != colour) {
                differing = differing.saturating_add(1);
            }
        }
    }
    differing
}

/// The pixels of `part` that both `a` and `b` hold, and those of them that
/// differ between the two.
fn differing_between(a: &Rgba, b: &Rgba, part: &PxRect) -> (i64, i64) {
    let (mut pixels, mut differing): (i64, i64) = (0, 0);
    for y in part.y0..part.y1 {
        for x in part.x0..part.x1 {
            if let (Some(p), Some(q)) = (a.pixel(x, y), b.pixel(x, y)) {
                pixels = pixels.saturating_add(1);
                if p != q {
                    differing = differing.saturating_add(1);
                }
            }
        }
    }
    (pixels, differing)
}

/// Per target, the pixels of its part that differ between the normal
/// captures over the black and the white backdrop (where the backdrop shows
/// through, they do), beside its `over_backdrop`; where `over_backdrop` is
/// true and none differs, the target's `error`: something opaque the
/// covering set misses lies behind the part, or the part itself is drawn
/// opaque over its whole rect.
fn backdrop_shows(
    targets: &[Target],
    black: Option<&Rgba>,
    white: Option<&Rgba>,
    frame: NSSize,
) -> Table {
    let mut t = Table::new();
    let (Some(black), Some(white)) = (black, white) else {
        out::put(
            &mut t,
            "error",
            out::unavailable("the normal capture over the black or the white backdrop is missing"),
        );
        return t;
    };
    let ratio = px_ratio(black.width, frame.width);
    for target in targets {
        let mut row = Table::new();
        out::put(
            &mut row,
            "over_backdrop",
            out::boolean(target.over_backdrop),
        );
        let part = ratio.and_then(|ratio| {
            PxRect::from_points(
                in_window(&target.view, target.part, frame),
                0.0,
                ratio,
                black.width,
                black.height,
            )
        });
        match part {
            Some(part) => {
                let (pixels, differing) = differing_between(black, white, &part);
                out::put(&mut row, "part_px_xywh", part.value());
                out::put(&mut row, "pixels", out::int(pixels));
                out::put(&mut row, "differing_pixels", out::int(differing));
                if target.over_backdrop && differing == 0 {
                    out::put(
                        &mut row,
                        "error",
                        out::unavailable(
                            "over_backdrop is true, yet no pixel of the part differs between the black and white captures: either something opaque that the covering set misses lies behind it, or the part itself is drawn opaque over its whole rect",
                        ),
                    );
                }
            }
            None => out::put(
                &mut row,
                "error",
                out::unavailable("the part lies outside the captures"),
            ),
        }
        out::put_table(&mut t, target.name, row);
    }
    t
}

/// The two captures compared inside the target's view's frame grown by
/// `DIFF_MARGIN`.
fn compare(a: Option<&Rgba>, b: Option<&Rgba>, target: &Target, frame: NSSize) -> Value {
    match (a, b) {
        (Some(a), Some(b)) => diff_table(a, b, from_top_left(&target.view, frame), frame),
        _ => out::unavailable("a capture is missing"),
    }
}

/// The target's part's centre in AppKit's screen points.
fn screen_point(window: &NSWindow, target: &Target) -> NSPoint {
    let p = &target.part;
    let local = NSPoint::new(
        p.origin.x + p.size.width / 2.0,
        p.origin.y + p.size.height / 2.0,
    );
    window.convertPointToScreen(target.view.convertPoint_toView(local, None))
}

/// `r`, in `view`'s coordinates, in the window frame's points from its top
/// left.
fn in_window(view: &NSView, r: NSRect, frame: NSSize) -> NSRect {
    let w = view.convertRect_toView(r, None);
    NSRect::new(
        NSPoint::new(w.origin.x, frame.height - w.origin.y - w.size.height),
        w.size,
    )
}

/// Where a target lies: its view's class and frame and its part, in the
/// window frame's points from its top left.
fn target_table(target: &Target, frame: NSSize) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "class",
        out::text(target.view.class().name().to_string_lossy()),
    );
    out::put(
        &mut t,
        "view_in_window_from_top_left_xywh",
        rect(from_top_left(&target.view, frame)),
    );
    out::put(
        &mut t,
        "part_in_window_from_top_left_xywh",
        rect(in_window(&target.view, target.part, frame)),
    );
    out::put(&mut t, "hovered", out::boolean(target.hover));
    out::put(&mut t, "pressed", out::boolean(target.press));
    t
}

/// A view drawn behind a target's part, as `covering` reads it.
#[derive(Debug, Clone, PartialEq)]
enum Behind {
    /// A scroll view or clip view, and whether it draws its background.
    Background { class: String, draws: bool },
    /// A visual effect view: its material is drawn behind the part.
    Effect { class: String },
    /// A table view and the alpha, in sRGB, of the background colour it
    /// draws behind its rows; `None` where the colour has no sRGB form.
    Table { class: String, alpha: Option<f64> },
    /// The scroller whose knob is the part, and whether its style is
    /// legacy: a legacy scroller draws its knob slot behind its knob.
    Scroller { class: String, legacy: bool },
    /// Any other view.
    Other { class: String },
}

/// The views of `behind` that are drawn over the backdrop where the part
/// lies, each by its class (with what of it is drawn): where any is, the
/// backdrop does not show behind the part as it is, and the part's layer is
/// the composite over them.
fn covering(behind: &[Behind]) -> Vec<String> {
    behind
        .iter()
        .filter_map(|b| match b {
            Behind::Background { class, draws: true } => Some(class.clone()),
            Behind::Effect { class } => Some(class.clone()),
            Behind::Table {
                class,
                alpha: Some(alpha),
            } if *alpha > 0.0 => Some(format!("{class} (backgroundColor)")),
            Behind::Table { class, alpha: None } => Some(format!(
                "{class} (backgroundColor, not convertible to sRGB)"
            )),
            Behind::Scroller {
                class,
                legacy: true,
            } => Some(format!("{class} (legacy knob slot)")),
            _ => None,
        })
        .collect()
}

/// What is drawn behind `part` (in `view`'s coordinates), each with its
/// reads for the record: `view` itself; with a scroll view its clip view and,
/// where `knob`, its vertical scroller; then each view up from `view` to
/// `content` (left out) and, under the view above it in that chain, its
/// visible subviews that overlap the part, with theirs; and the content
/// view's own subviews under the chain's top that overlap the part, the
/// sheet's `backdrop` box left out, each read as the rest.
fn behind_part(
    view: &NSView,
    part: NSRect,
    content: &NSView,
    backdrop: &NSView,
    knob: bool,
    frame: NSSize,
) -> (Vec<Behind>, Vec<Value>) {
    let mut found = vec![read_behind(view, frame)];
    if let Some(scroll) = view.downcast_ref::<NSScrollView>() {
        found.push(read_behind(&scroll.contentView(), frame));
        if knob && let Some(scroller) = scroll.verticalScroller() {
            let legacy = scroller.scrollerStyle() == NSScrollerStyle::Legacy;
            found.push((
                Behind::Scroller {
                    class: scroller.class().name().to_string_lossy().into_owned(),
                    legacy,
                },
                out::text(format!(
                    "{} (the knob's own scroller) scrollerStyle {}",
                    describe_view(&scroller, frame),
                    scroller_style_name(scroller.scrollerStyle()),
                )),
            ));
        }
    }
    let Some(up) = chain(view, content) else {
        found.push((
            Behind::Other {
                class: "none".to_owned(),
            },
            out::unavailable("the view does not lie under the content view"),
        ));
        return found.into_iter().unzip();
    };
    for pair in up.windows(2) {
        let [child, parent] = pair else {
            continue;
        };
        found.push(read_behind(parent, frame));
        for sub in subviews_under(parent, child) {
            below(&sub, view, part, frame, &mut found);
        }
    }
    if let Some(top) = up.last() {
        found.push((
            Behind::Other {
                class: content.class().name().to_string_lossy().into_owned(),
            },
            out::text(format!(
                "{} (the content view; its subviews under the target's that overlap the part follow, the backdrop box left out)",
                describe_view(content, frame),
            )),
        ));
        for sub in subviews_under(content, top) {
            if !std::ptr::eq(&*sub, backdrop) {
                below(&sub, view, part, frame, &mut found);
            }
        }
    }
    found.into_iter().unzip()
}

/// `parent`'s subviews drawn under `child` (those before it in its
/// subviews); none where `child` is not among them.
fn subviews_under(parent: &NSView, child: &NSView) -> Vec<Retained<NSView>> {
    let subviews: Vec<Retained<NSView>> = parent.subviews().iter().collect();
    match subviews.iter().position(|s| std::ptr::eq(&**s, child)) {
        Some(at) => subviews.into_iter().take(at).collect(),
        None => Vec::new(),
    }
}

/// `sub` and its subviews, where visible and overlapping `part` (in
/// `view`'s coordinates), into `found`.
fn below(
    sub: &NSView,
    view: &NSView,
    part: NSRect,
    frame: NSSize,
    found: &mut Vec<(Behind, Value)>,
) {
    if sub.isHidden() || !intersects(sub.bounds(), view.convertRect_toView(part, Some(sub))) {
        return;
    }
    found.push(read_behind(sub, frame));
    for s in sub.subviews().iter() {
        below(&s, view, part, frame, found);
    }
}

/// One view behind a part, read as `covering` takes it, and described.
fn read_behind(v: &NSView, frame: NSSize) -> (Behind, Value) {
    let class = v.class().name().to_string_lossy().into_owned();
    let at = describe_view(v, frame);
    if let Some(scroll) = v.downcast_ref::<NSScrollView>() {
        let draws = scroll.drawsBackground();
        (
            Behind::Background { class, draws },
            out::text(format!("{at} drawsBackground {draws}")),
        )
    } else if let Some(clip) = v.downcast_ref::<NSClipView>() {
        let draws = clip.drawsBackground();
        (
            Behind::Background { class, draws },
            out::text(format!("{at} drawsBackground {draws}")),
        )
    } else if let Some(effect) = v.downcast_ref::<NSVisualEffectView>() {
        let blending = effect.blendingMode();
        let blending = if blending == NSVisualEffectBlendingMode::BehindWindow {
            "behindWindow".to_owned()
        } else if blending == NSVisualEffectBlendingMode::WithinWindow {
            "withinWindow".to_owned()
        } else {
            format!("unknown ({})", blending.0)
        };
        (
            Behind::Effect { class },
            out::text(format!(
                "{at} material {} blendingMode {blending} state {}",
                effect.material().0,
                crate::macos::effect_state_name(effect.state())
            )),
        )
    } else if let Some(table) = v.downcast_ref::<NSTableView>() {
        let colour = table.backgroundColor();
        let srgb = NSColorSpace::sRGBColorSpace();
        let alpha = colour
            .colorUsingColorSpace(&srgb)
            .map(|c| c.alphaComponent());
        (
            Behind::Table { class, alpha },
            out::text(format!(
                "{at} backgroundColor {} usesAlternatingRowBackgroundColors {} style {} effectiveStyle {}",
                crate::macos::color_value(&colour, &srgb),
                table.usesAlternatingRowBackgroundColors(),
                table.style().0,
                table.effectiveStyle().0
            )),
        )
    } else {
        (Behind::Other { class }, out::text(at))
    }
}

/// A view's class, frame in the window frame's points from its top left,
/// and `isOpaque`.
fn describe_view(v: &NSView, frame: NSSize) -> String {
    format!(
        "{} frame_in_window_from_top_left {} isOpaque {}",
        v.class().name().to_string_lossy(),
        rect_text(from_top_left(v, frame)),
        v.isOpaque()
    )
}

/// The subview tree of `view`, a line per view: its path of subview
/// indices, `describe_view` and whether it is hidden.
fn view_tree(view: &NSView, frame: NSSize) -> Vec<Value> {
    fn walk(v: &NSView, path: &str, frame: NSSize, rows: &mut Vec<Value>) {
        for (i, sub) in v.subviews().iter().enumerate() {
            let at = format!("{path}{i}");
            rows.push(out::text(format!(
                "{at}: {} isHidden {}",
                describe_view(&sub, frame),
                sub.isHidden()
            )));
            walk(&sub, &format!("{at}."), frame, rows);
        }
    }
    let mut rows = vec![out::text(format!("-: {}", describe_view(view, frame)))];
    walk(view, "", frame, &mut rows);
    rows
}

/// The views from `view` (included) up its superviews to `top` (left
/// out); `None` where `top` is not among them.
fn chain(view: &NSView, top: &NSView) -> Option<Vec<Retained<NSView>>> {
    path_up(
        view.retain(),
        |v| std::ptr::eq(&**v, top),
        |v| ffi::superview(v),
    )
}

/// From `start` (included) up through `parent` to the first item `is_top`
/// takes (left out); `None` where the chain ends before one.
fn path_up<T>(
    start: T,
    is_top: impl Fn(&T) -> bool,
    parent: impl Fn(&T) -> Option<T>,
) -> Option<Vec<T>> {
    let mut path = Vec::new();
    let mut at = start;
    while !is_top(&at) {
        let next = parent(&at)?;
        path.push(at);
        at = next;
    }
    Some(path)
}

/// Whether `a` and `b` overlap in an area.
fn intersects(a: NSRect, b: NSRect) -> bool {
    a.origin.x < b.origin.x + b.size.width
        && b.origin.x < a.origin.x + a.size.width
        && a.origin.y < b.origin.y + b.size.height
        && b.origin.y < a.origin.y + a.size.height
}

/// Whether `inner` lies within `outer`, edges included.
fn contains(outer: NSRect, inner: NSRect) -> bool {
    inner.origin.x >= outer.origin.x
        && inner.origin.y >= outer.origin.y
        && inner.origin.x + inner.size.width <= outer.origin.x + outer.size.width
        && inner.origin.y + inner.size.height <= outer.origin.y + outer.size.height
}

/// A pending target's part in its view's coordinates, read now (the window
/// shown and settled); an error where the view cannot give it, where it is
/// empty, or where it does not lie within the view's bounds.
fn locate(p: &Pending, tab_rects: &[Option<NSRect>]) -> Result<NSRect, String> {
    let view = &*p.view;
    let b = view.bounds();
    let part = match p.part {
        Part::Bounds => b,
        Part::Indicator => view
            .downcast_ref::<NSButton>()
            .and_then(|button| button.cell())
            .map(|cell| cell.imageRectForBounds(b))
            .ok_or("the check box or radio button has no button cell")?,
        Part::Titled => titled_part(
            view.downcast_ref::<NSButton>()
                .ok_or("the view is not an NSButton")?,
        )?,
        Part::MiddleThird => NSRect::new(
            NSPoint::new(b.origin.x + b.size.width / 3.0, b.origin.y),
            NSSize::new(b.size.width / 3.0, b.size.height),
        ),
        Part::Knob => {
            let slider = view
                .downcast_ref::<NSSlider>()
                .ok_or("the view is not an NSSlider")?;
            slider
                .cell()
                .and_then(|cell| {
                    cell.downcast_ref::<NSSliderCell>()
                        .map(|c| c.knobRectFlipped(slider.isFlipped()))
                })
                .ok_or("the slider's cell is not an NSSliderCell")?
        }
        Part::Row => view
            .downcast_ref::<NSTableView>()
            .map(|table| table.rectOfRow(TABLE_TARGET_ROW))
            .ok_or("the view is not an NSTableView")?,
        Part::Tab => tab_rects
            .get(TAB_TARGET)
            .copied()
            .flatten()
            .ok_or("tabViewItemAtPoint: found no rect for the target tab")?,
        Part::Divider => {
            let split = view
                .downcast_ref::<NSSplitView>()
                .ok_or("the view is not an NSSplitView")?;
            let first = split
                .subviews()
                .iter()
                .next()
                .map(|v| v.frame())
                .ok_or("the split view has no subview")?;
            NSRect::new(
                NSPoint::new(first.origin.x + first.size.width, b.origin.y),
                NSSize::new(split.dividerThickness(), b.size.height),
            )
        }
        Part::ScrollerKnob => {
            let scroller = view
                .downcast_ref::<NSScrollView>()
                .ok_or("the view is not an NSScrollView")?
                .verticalScroller()
                .ok_or("the scroll view has no vertical scroller")?;
            scroller.convertRect_toView(scroller.rectForPart(NSScrollerPart::Knob), Some(view))
        }
    };
    if view.window().is_none() {
        return Err("the view is in no window".to_owned());
    }
    if part.size.width <= 0.0 || part.size.height <= 0.0 {
        return Err(format!("the part {} is empty", rect_text(part)));
    }
    if !contains(b, part) {
        return Err(format!(
            "the part {} does not lie within the view's bounds {}",
            rect_text(part),
            rect_text(b)
        ));
    }
    Ok(part)
}

/// A titled button's triangle and title as drawn: its bounds from the left
/// edge to the end of the cell's `titleRectForBounds:` (the slot the title is
/// drawn in, clipped to it where the title is wider), so the part's centre
/// falls on the button's drawing, not on the room right of it; an error
/// where the button has no cell.
fn titled_part(button: &NSButton) -> Result<NSRect, String> {
    let b = button.bounds();
    let cell = button.cell().ok_or("the titled button has no cell")?;
    Ok(titled_rect(b, cell.titleRectForBounds(b)))
}

/// `bounds` from its left edge to the end of `title`, within `bounds`.
fn titled_rect(bounds: NSRect, title: NSRect) -> NSRect {
    let end = title.origin.x + title.size.width;
    NSRect::new(
        bounds.origin,
        NSSize::new(
            (end - bounds.origin.x).clamp(0.0, bounds.size.width),
            bounds.size.height,
        ),
    )
}

/// A titled button's title (`TITLED_TARGETS`): the cell's
/// `titleRectForBounds:` in the button's coordinates and in the window
/// frame's points from its top left, and the attributed title's own size
/// (the title is drawn whole where the rect is at least that wide).
fn title_reads(t: &mut Table, button: &NSButton, frame: NSSize) {
    let Some(cell) = button.cell() else {
        out::put(
            t,
            "title_rect_xywh",
            out::unavailable("the button has no cell"),
        );
        return;
    };
    let title = cell.titleRectForBounds(button.bounds());
    out::put(t, "title_rect_xywh", rect(title));
    out::put(
        t,
        "title_in_window_from_top_left_xywh",
        rect(in_window(button, title, frame)),
    );
    let size = button.attributedTitle().size();
    out::put(t, "title_size_wh", out::floats(&[size.width, size.height]));
}

/// The sheet, its targets, the tab view's tab rects and what must stay
/// alive.
fn build(pass: &Pass<'_>) -> Result<(Sheet, Vec<Pending>, Keep), String> {
    let mtm = pass.mtm;
    let count = isize::try_from(SEGMENTS.len()).map_err(|e| e.to_string())?;
    let (window, content, backdrop) = sheet_window(mtm, CONTROLS_ORIGIN, CONTROLS_CONTENT, false)?;
    let height = CONTROLS_CONTENT.height;
    let size = NSControlSize::Regular;
    let mut cases: Vec<Case> = Vec::new();
    let mut targets: Vec<Pending> = Vec::new();

    let place = |control: &NSControl, x: f64, row: f64| {
        super::place(control, &content, x, row, height);
    };
    let [c0, c1, c2, c3, c4] = COLUMN_X;
    let view = |v: &NSView| -> Retained<NSView> { v.retain() };

    // Row 0: the check boxes, the disclosure buttons and a push button.
    let check = |on: bool| {
        let b = NSButton::new(mtm);
        b.setButtonType(NSButtonType::Switch);
        b.setTitle(&ns(CHECK_TITLE));
        b.setState(state_value(on));
        control_common(&b, size);
        b
    };
    let check_off = check(false);
    let check_on = check(true);
    let disclosure = NSButton::new(mtm);
    disclosure.setButtonType(NSButtonType::PushOnPushOff);
    disclosure.setBezelStyle(NSBezelStyle::Disclosure);
    disclosure.setTitle(&ns(DISCLOSURE_TITLE));
    disclosure.setState(state_value(false));
    control_common(&disclosure, size);
    let push_disclosure = NSButton::new(mtm);
    push_disclosure.setButtonType(NSButtonType::PushOnPushOff);
    push_disclosure.setBezelStyle(NSBezelStyle::PushDisclosure);
    push_disclosure.setTitle(&ns(""));
    push_disclosure.setState(state_value(false));
    control_common(&push_disclosure, size);
    let push = NSButton::new(mtm);
    push.setButtonType(NSButtonType::MomentaryPushIn);
    push.setBezelStyle(NSBezelStyle::Push);
    push.setTitle(&ns(TITLE));
    control_common(&push, size);
    for (x, b) in [
        (c0, &check_off),
        (c1, &check_on),
        (c2, &disclosure),
        (c3, &push_disclosure),
        (c4, &push),
    ] {
        place(b, x, 0.0);
    }
    disclosure.setFrameSize(NSSize::new(
        DISCLOSURE_WIDTH,
        disclosure.frame().size.height,
    ));
    for (name, b, on) in [
        ("checkbox_off", &check_off, false),
        ("checkbox_on", &check_on, true),
    ] {
        let reset_button = b.retain();
        targets.push(Pending {
            name,
            view: view(b),
            part: Part::Indicator,
            hover: true,
            press: true,
            escape: false,
            reset: Box::new(move |_| reset_button.setState(state_value(on))),
        });
    }
    for (name, b, part) in [
        ("disclosure", &disclosure, Part::Titled),
        ("push_disclosure", &push_disclosure, Part::Bounds),
    ] {
        let reset_button = b.retain();
        targets.push(Pending {
            name,
            view: view(b),
            part,
            hover: true,
            press: true,
            escape: false,
            reset: Box::new(move |_| reset_button.setState(state_value(false))),
        });
    }
    targets.push(Pending {
        name: "push",
        view: view(&push),
        part: Part::Bounds,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(|_| {}),
    });

    // Row 1: the pop-up button, the text field, the segmented control and
    // the switch.
    let popup = NSPopUpButton::new(mtm);
    for item in POPUP_ITEMS {
        popup.addItemWithTitle(&ns(item));
    }
    popup.selectItemAtIndex(0);
    control_common(&popup, size);
    place(&popup, c0, 1.0);
    let field = NSTextField::new(mtm);
    field.setBezeled(true);
    field.setEditable(true);
    field.setStringValue(&ns(FIELD_TEXT));
    control_common(&field, size);
    place(&field, c1, 1.0);
    let segmented = NSSegmentedControl::new(mtm);
    segmented.setSegmentCount(count);
    for (i, label) in (0_isize..).zip(SEGMENTS) {
        segmented.setLabel_forSegment(&ns(label), i);
        segmented.setWidth_forSegment(SEGMENT_WIDTH, i);
    }
    segmented.setSelectedSegment(0);
    control_common(&segmented, size);
    place(&segmented, c2, 1.0);
    let switch = NSSwitch::new(mtm);
    switch.setState(state_value(false));
    control_common(&switch, size);
    place(&switch, c4, 1.0);
    let popup_button: Retained<NSButton> = Retained::into_super(popup.retain());
    let reset_popup = popup.retain();
    targets.push(Pending {
        name: "popup",
        view: view(&popup),
        part: Part::Bounds,
        hover: true,
        press: true,
        escape: true,
        reset: Box::new(move |_| reset_popup.selectItemAtIndex(0)),
    });
    let reset_field = field.retain();
    let reset_field_window = window.retain();
    targets.push(Pending {
        name: "text_field",
        view: view(&field),
        part: Part::Bounds,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(move |t| {
            reset_field_window.makeFirstResponder(None);
            out::put(
                t,
                "firstResponder_class_after_reset",
                first_responder(&reset_field_window),
            );
            reset_field.setStringValue(&ns(FIELD_TEXT));
        }),
    });
    let reset_segmented = segmented.retain();
    targets.push(Pending {
        name: "segment",
        view: view(&segmented),
        part: Part::MiddleThird,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(move |_| reset_segmented.setSelectedSegment(0)),
    });
    let reset_switch = switch.retain();
    targets.push(Pending {
        name: "switch_off",
        view: view(&switch),
        part: Part::Bounds,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(move |_| reset_switch.setState(state_value(false))),
    });

    // Row 2: the slider.
    let slider = NSSlider::new(mtm);
    slider.setVertical(false);
    slider.setMinValue(0.0);
    slider.setMaxValue(2.0 * SLIDER_VALUE);
    slider.setDoubleValue(SLIDER_VALUE);
    control_common(&slider, size);
    place(&slider, c0, 2.0);
    slider.setFrameSize(NSSize::new(SLIDER_WIDTH, slider.frame().size.height));
    let reset_slider = slider.retain();
    targets.push(Pending {
        name: "slider_knob",
        view: view(&slider),
        part: Part::Knob,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(move |_| reset_slider.setDoubleValue(SLIDER_VALUE)),
    });

    // Row 2, beside the slider: a radio button off and one on (no action,
    // so the two form no group); row 3: a check box and a radio button,
    // each off and on, disabled.
    let radio = |on: bool| {
        let b = NSButton::new(mtm);
        b.setButtonType(NSButtonType::Radio);
        b.setTitle(&ns(RADIO_TITLE));
        b.setState(state_value(on));
        control_common(&b, size);
        b
    };
    let radio_off = radio(false);
    let radio_on = radio(true);
    place(&radio_off, c2, 2.0);
    place(&radio_on, c3, 2.0);
    for (name, b, on) in [
        ("radio_off", &radio_off, false),
        ("radio_on", &radio_on, true),
    ] {
        let reset_button = b.retain();
        targets.push(Pending {
            name,
            view: view(b),
            part: Part::Indicator,
            hover: true,
            press: true,
            escape: false,
            reset: Box::new(move |_| reset_button.setState(state_value(on))),
        });
    }
    let disabled = [
        ("checkbox_disabled_off", "checkbox", check(false), false),
        ("checkbox_disabled_on", "checkbox", check(true), true),
        ("radio_disabled_off", "radio", radio(false), false),
        ("radio_disabled_on", "radio", radio(true), true),
    ];
    for ((name, kind, b, on), x) in disabled.iter().zip(COLUMN_X) {
        let (name, kind, on) = (*name, *kind, *on);
        b.setEnabled(false);
        place(b, x, 3.0);
        targets.push(Pending {
            name,
            view: view(b),
            part: Part::Indicator,
            hover: false,
            press: false,
            escape: false,
            reset: Box::new(|_| {}),
        });
        cases.push(Case {
            name: name.to_owned(),
            size,
            kind,
            setup: match (kind, on) {
                ("radio", false) => "NSButton, NSButtonTypeRadio, state off, disabled",
                ("radio", true) => "NSButton, NSButtonTypeRadio, state on, disabled",
                (_, false) => "NSButton, NSButtonTypeSwitch, state off, disabled",
                (_, true) => "NSButton, NSButtonTypeSwitch, state on, disabled",
            },
            control: Retained::into_super(b.retain()),
            button: Some(b.retain()),
            clickable: false,
            focus: false,
        });
    }
    for (name, setup, b) in [
        (
            "radio_off",
            "NSButton, NSButtonTypeRadio, state off",
            &radio_off,
        ),
        (
            "radio_on",
            "NSButton, NSButtonTypeRadio, state on",
            &radio_on,
        ),
    ] {
        cases.push(Case {
            name: name.to_owned(),
            size,
            kind: "radio",
            setup,
            control: Retained::into_super(b.retain()),
            button: Some(b.retain()),
            clickable: false,
            focus: false,
        });
    }

    // The scroll views: a vertical scroller in each style, its knob in the
    // middle of its track.
    let mut scrolls = Vec::new();
    for (name, frame, style) in [
        (
            "scroller_knob_legacy",
            SCROLL_FRAME,
            NSScrollerStyle::Legacy,
        ),
        (
            "scroller_knob_overlay",
            OVERLAY_SCROLL_FRAME,
            NSScrollerStyle::Overlay,
        ),
    ] {
        let scroll = scroll_in(mtm, frame, style);
        content.addSubview(&scroll);
        scroll.tile();
        scroll_to_middle(&scroll);
        let reset_scroll = scroll.retain();
        targets.push(Pending {
            name,
            view: view(&scroll),
            part: Part::ScrollerKnob,
            hover: true,
            press: true,
            escape: false,
            reset: Box::new(move |_| scroll_to_middle(&reset_scroll)),
        });
        scrolls.push((name, style, scroll));
    }

    // The tables: one in the default style, one in the source-list style.
    let mut sources = Vec::new();
    let mut tables = Vec::new();
    for (name, frame, style) in [
        ("table_row", TABLE_FRAME, None),
        (
            "source_list_row",
            SOURCE_LIST_FRAME,
            Some(NSTableViewStyle::SourceList),
        ),
    ] {
        let (scroll, table, source) = table_in(mtm, frame, style);
        content.addSubview(&scroll);
        table.reloadData();
        scroll.tile();
        table.tile();
        sources.push(source);
        tables.push(table.retain());
        let reset_table = table.retain();
        let reset_window = window.retain();
        targets.push(Pending {
            name,
            view: view(&table),
            part: Part::Row,
            hover: true,
            press: true,
            escape: false,
            reset: Box::new(move |t| {
                reset_table.selectRowIndexes_byExtendingSelection(&NSIndexSet::new(), false);
                reset_window.makeFirstResponder(None);
                out::put(
                    t,
                    "firstResponder_class_after_reset",
                    first_responder(&reset_window),
                );
            }),
        });
        cases.push(Case {
            name: name.to_owned(),
            size,
            kind: name,
            setup: if style.is_some() {
                "NSTableView, style NSTableViewStyleSourceList"
            } else {
                "NSTableView, default style"
            },
            control: Retained::into_super(table),
            button: None,
            clickable: false,
            focus: false,
        });
    }

    // The tab view, the split view and the box.
    let tab_view = NSTabView::new(mtm);
    tab_view.setFrame(TAB_VIEW_FRAME);
    for label in TABS {
        let item = NSTabViewItem::new();
        item.setLabel(&ns(label));
        tab_view.addTabViewItem(&item);
    }
    tab_view.selectTabViewItemAtIndex(0);
    content.addSubview(&tab_view);
    let reset_tabs = tab_view.retain();
    targets.push(Pending {
        name: "tab",
        view: view(&tab_view),
        part: Part::Tab,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(move |_| reset_tabs.selectTabViewItemAtIndex(0)),
    });

    let split = NSSplitView::new(mtm);
    split.setFrame(SPLIT_FRAME);
    split.setVertical(true);
    for _ in 0..2 {
        let pane = NSView::new(mtm);
        split.addSubview(&pane);
    }
    split.adjustSubviews();
    content.addSubview(&split);
    let divider_position = split.subviews().iter().next().map(|pane| {
        let f = pane.frame();
        f.origin.x + f.size.width
    });
    let reset_split = split.retain();
    targets.push(Pending {
        name: "split_divider",
        view: view(&split),
        part: Part::Divider,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(move |_| {
            if let Some(position) = divider_position {
                reset_split.setPosition_ofDividerAtIndex(position, 0);
            }
        }),
    });

    let card = NSBox::initWithFrame(NSBox::alloc(mtm), BOX_FRAME);
    card.setBoxType(NSBoxType::Primary);
    card.setTitlePosition(NSTitlePosition::NoTitle);
    content.addSubview(&card);
    targets.push(Pending {
        name: "box",
        view: view(&card),
        part: Part::Bounds,
        hover: true,
        press: false,
        escape: false,
        reset: Box::new(|_| {}),
    });

    // A button in a toolbar item.
    let (toolbar_button, delegate) = toolbar_with_button(mtm, &window, CONTROLS_TOOLBAR_ID);
    targets.push(Pending {
        name: "toolbar_button",
        view: view(&toolbar_button),
        part: Part::Bounds,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(|_| {}),
    });

    for (name, kind, setup, b) in [
        (
            "checkbox_off",
            "checkbox",
            "NSButton, NSButtonTypeSwitch, state off",
            &check_off,
        ),
        (
            "checkbox_on",
            "checkbox",
            "NSButton, NSButtonTypeSwitch, state on",
            &check_on,
        ),
        (
            "disclosure",
            "disclosure",
            "NSButton, NSButtonTypePushOnPushOff, NSBezelStyleDisclosure, titled, state off, widened to show its whole title",
            &disclosure,
        ),
        (
            "popup",
            "popup",
            "NSPopUpButton, two items, the first selected",
            &popup_button,
        ),
        (
            "push_disclosure",
            "push_disclosure",
            "NSButton, NSButtonTypePushOnPushOff, NSBezelStylePushDisclosure, empty title, state off",
            &push_disclosure,
        ),
        (
            "push",
            "push",
            "NSButton, NSButtonTypeMomentaryPushIn, NSBezelStylePush",
            &push,
        ),
        (
            "toolbar_button",
            "toolbar_button",
            "NSButton, NSButtonTypeMomentaryPushIn, NSBezelStyleToolbar, the view of an NSToolbarItem",
            &toolbar_button,
        ),
    ] {
        cases.push(Case {
            name: name.to_owned(),
            size,
            kind,
            setup,
            control: Retained::into_super(b.retain()),
            button: Some(b.retain()),
            clickable: false,
            focus: false,
        });
    }
    for (name, setup, control) in [
        (
            "text_field",
            "NSTextField, bezeled, editable",
            Retained::into_super(field.retain()),
        ),
        (
            "segmented",
            "NSSegmentedControl, three segments of one width, segment 0 selected",
            Retained::into_super(segmented.retain()),
        ),
        (
            "switch_off",
            "NSSwitch, state off",
            Retained::into_super(switch.retain()),
        ),
        (
            "slider",
            "NSSlider, horizontal, 0 to 100 at 50",
            Retained::into_super(slider.retain()),
        ),
    ] {
        cases.push(Case {
            name: name.to_owned(),
            size,
            kind: name,
            setup,
            control,
            button: None,
            clickable: false,
            focus: false,
        });
    }

    let mut notes = Table::new();
    out::put(
        &mut notes,
        "content_size_wh",
        out::floats(&[CONTROLS_CONTENT.width, CONTROLS_CONTENT.height]),
    );
    let sheet = Sheet {
        name: "controls",
        window,
        content,
        backdrop,
        cases,
        pointer_states: false,
        notes,
    };
    Ok((
        sheet,
        targets,
        Keep {
            _sources: sources,
            tables,
            _delegate: delegate,
            tab_view,
            scrolls,
        },
    ))
}

/// An NSButton (NSBezelStyleToolbar, `TITLE`) as the view of the one item of
/// an NSToolbar (identifier `id`) given to `window`; the button and the
/// toolbar's delegate, which must stay alive while the toolbar is shown.
fn toolbar_with_button(
    mtm: objc2::MainThreadMarker,
    window: &NSWindow,
    id: &str,
) -> (Retained<NSButton>, Retained<ffi::ToolbarDelegate>) {
    let button = NSButton::new(mtm);
    button.setButtonType(NSButtonType::MomentaryPushIn);
    button.setBezelStyle(NSBezelStyle::Toolbar);
    button.setTitle(&ns(TITLE));
    button.sizeToFit();
    let item = NSToolbarItem::initWithItemIdentifier(NSToolbarItem::alloc(mtm), &ns("button"));
    item.setAutovalidates(false);
    item.setEnabled(true);
    let v: &NSView = &button;
    item.setView(Some(v));
    let delegate = ffi::ToolbarDelegate::new(mtm, vec![item]);
    let toolbar = NSToolbar::initWithIdentifier(NSToolbar::alloc(mtm), &ns(id));
    toolbar.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    window.setToolbar(Some(&toolbar));
    (button, delegate)
}

/// The toolbar sheet: a titled window whose content view is given the whole
/// window (`NSWindowStyleMaskFullSizeContentView`) under a transparent title
/// bar (`titlebarAppearsTransparent`), its backdrop box made to follow the
/// content view, and the controls sheet's toolbar button in its toolbar.
fn build_toolbar_sheet(
    pass: &Pass<'_>,
) -> Result<(Sheet, Vec<Pending>, Retained<ffi::ToolbarDelegate>), String> {
    let mtm = pass.mtm;
    let (window, content, backdrop) = sheet_window(mtm, CONTROLS_ORIGIN, TOOLBAR_CONTENT, false)?;
    window.setStyleMask(window.styleMask() | NSWindowStyleMask::FullSizeContentView);
    window.setTitlebarAppearsTransparent(true);
    let (button, delegate) = toolbar_with_button(mtm, &window, TOOLBAR_SHEET_ID);
    backdrop.setFrame(content.bounds());
    let button_view: &NSView = &button;
    let pending = vec![Pending {
        name: "toolbar_button",
        view: button_view.retain(),
        part: Part::Bounds,
        hover: true,
        press: true,
        escape: false,
        reset: Box::new(|_| {}),
    }];
    let cases = vec![Case {
        name: "toolbar_button".to_owned(),
        size: NSControlSize::Regular,
        kind: "toolbar_button",
        setup: "NSButton, NSButtonTypeMomentaryPushIn, NSBezelStyleToolbar, the view of an NSToolbarItem, in a window whose content view lies under its transparent title bar",
        control: Retained::into_super(button.retain()),
        button: Some(button),
        clickable: false,
        focus: false,
    }];
    let mut notes = Table::new();
    out::put(
        &mut notes,
        "content_size_wh",
        out::floats(&[TOOLBAR_CONTENT.width, TOOLBAR_CONTENT.height]),
    );
    let sheet = Sheet {
        name: "controls_toolbar",
        window,
        content,
        backdrop,
        cases,
        pointer_states: false,
        notes,
    };
    Ok((sheet, pending, delegate))
}

/// A scroll view at `frame` with a vertical scroller in `style`, never
/// hidden, whose document view (an empty NSView) is `SCROLL_DOCUMENT_FACTOR`
/// times its height; neither the scroll view nor its clip view draws a
/// background, so the backdrop shows behind the scroller.
fn scroll_in(
    mtm: objc2::MainThreadMarker,
    frame: NSRect,
    style: NSScrollerStyle,
) -> Retained<NSScrollView> {
    let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), frame);
    scroll.setHasVerticalScroller(true);
    scroll.setAutohidesScrollers(false);
    scroll.setScrollerStyle(style);
    scroll.setDrawsBackground(false);
    scroll.contentView().setDrawsBackground(false);
    let document = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(frame.size.width, frame.size.height * SCROLL_DOCUMENT_FACTOR),
        ),
    );
    scroll.setDocumentView(Some(&document));
    scroll
}

/// Scroll `scroll`'s document so the middle of it is shown (the knob in the
/// middle of its track).
fn scroll_to_middle(scroll: &NSScrollView) {
    let Some(document) = scroll.documentView() else {
        return;
    };
    let clip = scroll.contentView();
    let y = ((document.frame().size.height - clip.bounds().size.height) / 2.0).max(0.0);
    clip.scrollToPoint(NSPoint::new(0.0, y));
    scroll.reflectScrolledClipView(&clip);
}

/// A view-based table of `TABLE_ROWS` rows of one column, each cell a label,
/// no header, no row selected, in a scroll view at `frame`.
fn table_in(
    mtm: objc2::MainThreadMarker,
    frame: NSRect,
    style: Option<NSTableViewStyle>,
) -> (
    Retained<NSScrollView>,
    Retained<NSTableView>,
    Retained<ffi::TableSource>,
) {
    let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), frame);
    let table = NSTableView::new(mtm);
    if let Some(style) = style {
        table.setStyle(style);
    }
    table.setHeaderView(None);
    let id = ns("column");
    let column = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), &id);
    column.setWidth(TABLE_COLUMN_WIDTH);
    table.addTableColumn(&column);
    let rows: Vec<Vec<Retained<NSView>>> = (0..TABLE_ROWS)
        .map(|_| {
            let cell = NSTableCellView::initWithFrame(
                NSTableCellView::alloc(mtm),
                NSRect::new(
                    NSPoint::new(0.0, 0.0),
                    NSSize::new(TABLE_COLUMN_WIDTH, table.rowHeight()),
                ),
            );
            let label = NSTextField::labelWithString(&ns(ROW_TEXT), mtm);
            label.sizeToFit();
            cell.addSubview(&label);
            ffi::set_text_field(&cell, &label);
            vec![Retained::into_super(cell)]
        })
        .collect();
    let ids: Vec<Retained<NSString>> = vec![id];
    let source = ffi::TableSource::new(mtm, ids, rows);
    ffi::set_table_source(&table, Some(&source));
    scroll.setDocumentView(Some(&table));
    (scroll, table, source)
}

/// Each tab's rect in the tab view's coordinates: the bounds of the points,
/// on a `TAB_SCAN_STEP` grid over the tab view's bounds, at which
/// `tabViewItemAtPoint:` gives that tab's item, widened by one step on its
/// far sides.
fn tab_rects(tab_view: &NSTabView) -> Vec<Option<NSRect>> {
    let bounds = tab_view.bounds();
    let items: Vec<Retained<NSTabViewItem>> = tab_view.tabViewItems().iter().collect();
    let mut found: Vec<Option<(f64, f64, f64, f64)>> = vec![None; items.len()];
    let mut y = bounds.origin.y;
    while y < bounds.origin.y + bounds.size.height {
        let mut x = bounds.origin.x;
        while x < bounds.origin.x + bounds.size.width {
            if let Some(hit) = tab_view.tabViewItemAtPoint(NSPoint::new(x, y))
                && let Some(i) = items
                    .iter()
                    .position(|item| Retained::as_ptr(item) == Retained::as_ptr(&hit))
                && let Some(slot) = found.get_mut(i)
            {
                *slot = Some(match *slot {
                    None => (x, y, x, y),
                    Some((x0, y0, x1, y1)) => (x0.min(x), y0.min(y), x1.max(x), y1.max(y)),
                });
            }
            x += TAB_SCAN_STEP;
        }
        y += TAB_SCAN_STEP;
    }
    found
        .iter()
        .map(|f| {
            f.map(|(x0, y0, x1, y1)| {
                NSRect::new(
                    NSPoint::new(x0, y0),
                    NSSize::new(x1 - x0 + TAB_SCAN_STEP, y1 - y0 + TAB_SCAN_STEP),
                )
            })
        })
        .collect()
}

/// The tab view's tabs: each tab's rect in the tab view's coordinates and in
/// the window frame's points from its top left (the window frame `frame`).
fn tabs_table(tabs: &Tabs, frame: NSSize) -> Table {
    let tab_view = &*tabs.view;
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "tabViewItemAtPoint: at every point of a {TAB_SCAN_STEP} pt grid over the tab view's bounds; a tab's rect is the bounds of the points that give its item, widened by one step on its far sides; rects in the tab view's coordinates (rect_xywh) and in the window frame's points from its top left (in_window_from_top_left_xywh); the tab view's frame {}",
            point_text(TAB_VIEW_FRAME.origin)
        )),
    );
    for ((label, r), selected) in TABS
        .iter()
        .zip(&tabs.rects)
        .zip((0_usize..).map(|i| i == 0))
    {
        let mut row = Table::new();
        match r {
            Some(r) => {
                out::put(&mut row, "rect_xywh", rect(*r));
                out::put(
                    &mut row,
                    "in_window_from_top_left_xywh",
                    rect(in_window(tab_view, *r, frame)),
                );
            }
            None => out::put(
                &mut row,
                "rect_xywh",
                out::unavailable("tabViewItemAtPoint: gave this tab at no point"),
            ),
        }
        out::put(&mut row, "selected", out::boolean(selected));
        out::put_table(&mut t, label, row);
    }
    t
}

fn method_text() -> String {
    format!(
        "one titled NSWindow ({} x {} pt content from {}, empty title) holding the button pass's backdrop box and, at the regular control size with systemFontOfSize: of systemFontSizeForControlSize:, sized to fit and centred in rows {ROW_STEP} pt apart from the content's top: row 0 a check box off and one on (title \"{CHECK_TITLE}\"), a disclosure button (titled \"{DISCLOSURE_TITLE}\", sized to fit, then widened to {DISCLOSURE_WIDTH} pt; its title is drawn in the cell's titleRectForBounds:, clipped to it where the title is wider; targets.disclosure: title_rect_xywh, that rect, title_size_wh, the attributed title's size) and a push disclosure button, both push-on-push-off and off, and a push button (\"{TITLE}\"); row 1 a pop-up button ({POPUP_ITEMS:?}, the first selected), an editable bezeled text field (\"{FIELD_TEXT}\"), a segmented control ({SEGMENTS:?}, each {SEGMENT_WIDTH} pt wide, segment 0 selected) and, in the fifth column, a switch, off; row 2 a horizontal slider, 0 to {} at {SLIDER_VALUE}, {SLIDER_WIDTH} pt wide, and in the third and fourth columns a radio button off and one on (\"{RADIO_TITLE}\", NSButtonTypeRadio, no action, so the two form no group; set back to off and on after each press); row 3 a check box off and one on and a radio button off and one on, each disabled (setEnabled: NO; neither hovered nor pressed: their states are the sheet's normal and highlighted ones; targets.<name>: the indicator's rect and title_rect_xywh as the disclosure button's, for every check box and radio button); under it two scroll views at {scroll} and {overlay} (x, y), each {} x {} pt with a vertical scroller (autohidesScrollers NO), the first set to NSScrollerStyleLegacy, the second to NSScrollerStyleOverlay, each document view an empty NSView {SCROLL_DOCUMENT_FACTOR} times the scroll view's height, scrolled to its middle, and neither the scroll view nor its clip view drawing a background (drawsBackground NO on both), so the backdrop shows behind the scroller; once the window is shown and settled each scroll view's style read (scroll_views.scrollerStyle_when_shown), set again and the scroll view tiled and scrolled to its middle again (scroll_views: the styles read back after that, drawsBackground and contentView_drawsBackground, each vertical scroller's isHidden, alphaValue, knobProportion and doubleValue, and NSScroller's preferredScrollerStyle), and so again once each appearance is set (<appearance>.scroll_views: scrollerStyle_read, then scrollerStyle_after_set); below them two view-based tables of {TABLE_ROWS} rows (one column {TABLE_COLUMN_WIDTH} pt, no header, each cell a label \"{ROW_TEXT}\", no row selected) in scroll views at {} (default style) and {} (NSTableViewStyleSourceList), an NSTabView at {} with tabs {TABS:?} (the first selected, its items without views), a vertical NSSplitView at {} with two empty panes, and an NSBox (NSBoxPrimary, no title) at {}; the window's NSToolbar holds one item whose view is an NSButton (NSBezelStyleToolbar, \"{TITLE}\"); the window made key as a button sheet's; per appearance: normal (the cursor parked {park}; so again after the hovered and the pressed states, cursor_parked), highlighted (highlight: YES on every NSButton, the pop-up button among them: each cell highlighted and the control redrawn, the pop-up's menu not opened), then per target the cursor moved to the centre of the target's part as the button pass's hovered state moves it, captured over every backdrop (hovered-<target>), and per pressed target, per backdrop, the cursor moved there and a real mouse-down captured during the press as the button pass's clicked state takes it (clicked-<target>[-over-<backdrop>].png; the mouse-up posted once the capture is done, on the main thread by a timer in the run loop's common modes, which fires in the control's tracking loop, where windowNumberAtPoint: at the press point, read just before it, gives the probe's window or another window its process owns (kCGWindowOwnerPID; the pop-up's menu), else to the probe's process alone (CGEventPostToPid) with the press's error (mouseUp_window_under, mouseUp_window_is_ours, mouseUp_posted_to); the pop-up's mouse-up followed {delay} s later by an Escape down and up, to the HID event tap where the probe's application is active, else to its process alone with the error (escape_NSApp_isActive, escape_posted_to); where the timer has posted no release {wait_steps} pump steps after the capture, the capture thread posts it to the probe's process alone (release_by_watchdog, the press's error); firstResponder_class_after_press, the class of the window's first responder read after the mouse-up, before the control is set back: the text field's press makes its field editor the first responder), the control set back after each press (the text field: makeFirstResponder: nil, the first responder's class read back (firstResponder_class_after_reset), and its text set again; the tables: the selection cleared and makeFirstResponder: nil, read back as the text field's; the split view: setPosition:ofDividerAtIndex: 0 to the first pane's right edge read when it was built; the scroll views: scrolled to their middle again); a press is posted only where windowNumberAtPoint: at the press point gives the probe's window (window_at_press_is_ours), else it is skipped and its error recorded; for the scroller knobs the hovered state is taken per backdrop as the press is, and before each hovered and pressed capture, once the cursor is over the knob, the scrollers are shown (an overlay scroller's knob is drawn only while shown; reveal): a scroll-wheel CGEvent of {REVEAL_LINES} line (wheel1 {REVEAL_LINES}) and one of as many with wheel1 negated posted at the cursor (kCGScrollEventUnitLine), each where windowNumberAtPoint: at the cursor, read just before it, gives the probe's window (else that scroll is not posted and is the reveal's error), each followed by {REVEAL_STEPS} steps and the vertical scroller's doubleValue read (reveal.verticalScroller_doubleValue_before, reveal.scrolls), then the scroll view scrolled to its middle again and its flashScrollers; knob_drawn: in the capture over the window's background, the pixels of the knob's own rect (the vertical scroller's rectForPart: NSScrollerKnob read after the state's captures, the view scrolled to its middle as at them; knob_in_window_from_top_left_xywh) that differ from the scroll view's background read at the middle of its clip view (background_raw), where none does the state's error (the knob is not drawn) (targets: their parts' rects, each read once the window is shown and settled, the cases' reads too; a part whose view is in no window, or that is empty or does not lie within its view's bounds, is the target's error and the target is neither hovered nor pressed; a case whose control is in no window gets an error for its geometry and its layer's measures and is not highlighted; behind_part: what is drawn behind the part, a line per view (class, frame, isOpaque and, for a scroll view or clip view, drawsBackground; for a visual effect view, material, blendingMode and state; for a table, backgroundColor in sRGB, usesAlternatingRowBackgroundColors, style and effectiveStyle): the target's view, with a scroll view its clip view and, for a knob, its vertical scroller, then each view up to the content view and, under the next one up, its visible subviews that overlap the part, with theirs, and the content view's subviews under the target's that overlap it, the backdrop box left out; a table's enclosing_scroll_view_tree, its scroll view's every subview; over_backdrop: whether the part lies within the backdrop box and nothing behind it covers it (backdrop_covered_by: a scroll view or clip view that draws its background, a visual effect view, a table whose backgroundColor has alpha above 0 or no sRGB form, a legacy scroller's knob slot behind its knob), where not its layer is the composite over what lies behind it; per appearance, normal.backdrop_shows: per target the pixels of its part that differ between the normal captures over the black and the white backdrop, and the target's error where over_backdrop is true and none does (the flag checked by the data; a covering view may be translucent, so a covered part's differing pixels are no error); the table rows: rectOfRow: {TABLE_TARGET_ROW}; the segment: the middle third; the slider: knobRectFlipped:; the disclosure button: from its left edge to the end of the cell's titleRectForBounds:, the slot its title is drawn in; the tab: tab {TAB_TARGET}'s rect from the tabs table; the split divider: from the first pane's right edge, dividerThickness wide; the scroller knobs: the vertical scroller's rectForPart: NSScrollerKnob, in the scroll view's coordinates); each layer is solved from the black and white captures as the button pass's (layer_method); diff_from_normal compares a state's capture over the window's background with the normal one inside the target's view frame grown by {DIFF_MARGIN} pt; then the toolbar sheet (toolbar_sheet, its method)",
        CONTROLS_CONTENT.width,
        CONTROLS_CONTENT.height,
        point_text(CONTROLS_ORIGIN),
        2.0 * SLIDER_VALUE,
        SCROLL_FRAME.size.width,
        SCROLL_FRAME.size.height,
        point_text(TABLE_FRAME.origin),
        point_text(SOURCE_LIST_FRAME.origin),
        point_text(TAB_VIEW_FRAME.origin),
        point_text(SPLIT_FRAME.origin),
        point_text(BOX_FRAME.origin),
        scroll = point_text(SCROLL_FRAME.origin),
        overlay = point_text(OVERLAY_SCROLL_FRAME.origin),
        delay = super::CLICK_CAPTURE_DELAY.as_secs_f64(),
        wait_steps = super::CLICK_WAIT_STEPS,
        park = super::park_text(Parking::Controls),
    )
}

fn toolbar_method_text() -> String {
    format!(
        "after the controls sheet's window is closed, one titled NSWindow ({} x {} pt content from {}, empty title) given NSWindowStyleMaskFullSizeContentView and titlebarAppearsTransparent YES, so its content view lies under its title bar and toolbar, holding the button pass's backdrop box made the content view's bounds once the window is shown (styleMask_fullSizeContentView, titlebarAppearsTransparent and content_in_window_from_top_left_xywh read back); its NSToolbar (identifier {TOOLBAR_SHEET_ID}) holds one item whose view is an NSButton (NSBezelStyleToolbar, \"{TITLE}\"), as the controls sheet's; the window made key as a button sheet's; per appearance normal, highlighted (highlight: YES on the button), the button hovered and the button pressed, each over every backdrop, as the controls sheet's targets (targets.toolbar_button: its part the button's bounds; over_backdrop, behind_part, backdrop_covered_by and normal.backdrop_shows as the controls sheet's; the button lies in the toolbar, not under the content view, so behind_part says so and over_backdrop is whether it lies within the backdrop box); its captures in <appearance>/controls_toolbar/; each layer solved from the black and white captures as the button pass's (layer_method)",
        TOOLBAR_CONTENT.width,
        TOOLBAR_CONTENT.height,
        point_text(CONTROLS_ORIGIN),
    )
}

/// The controls sheet: every control placed from the first column's x, the
/// tables' and the scroll views' at theirs, each compared and read inside
/// its frame grown by `DIFF_MARGIN`. The toolbar sheet's one target lies in
/// the toolbar, a frame no constant gives, so it has no case.
#[cfg(test)]
pub(super) fn park_cases() -> Vec<super::ParkCase> {
    let left = COLUMN_X
        .iter()
        .copied()
        .chain([TABLE_FRAME.origin.x, SCROLL_FRAME.origin.x])
        .fold(CONTROLS_CONTENT.width, f64::min);
    vec![super::ParkCase {
        parking: Parking::Controls,
        window: "controls",
        content: CONTROLS_CONTENT,
        crops: vec![(
            "every control",
            super::from_origin(NSPoint::new(left, 0.0), CONTROLS_CONTENT, DIFF_MARGIN),
        )],
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tree of four items by parent: 0 the root, 1 under it, 2 and 3
    /// under 1.
    const PARENTS: [Option<usize>; 4] = [None, Some(0), Some(1), Some(1)];

    fn parent(i: &usize) -> Option<usize> {
        PARENTS.get(*i).copied().flatten()
    }

    #[test]
    fn a_chain_runs_from_its_start_up_to_the_top_left_out() {
        assert_eq!(path_up(2, |i| *i == 0, parent), Some(vec![2, 1]));
        assert_eq!(path_up(3, |i| *i == 1, parent), Some(vec![3]));
        assert_eq!(path_up(1, |i| *i == 1, parent), Some(Vec::new()));
    }

    #[test]
    fn a_chain_whose_top_is_not_above_its_start_is_none() {
        assert_eq!(path_up(2, |i| *i == 3, parent), None);
        assert_eq!(path_up(0, |i| *i == 1, parent), None);
    }

    fn class(name: &str) -> String {
        name.to_owned()
    }

    #[test]
    fn only_what_draws_over_the_backdrop_covers_it() {
        let behind = [
            Behind::Other {
                class: class("NSTableView"),
            },
            Behind::Background {
                class: class("NSClipView"),
                draws: false,
            },
            Behind::Background {
                class: class("NSScrollView"),
                draws: true,
            },
            Behind::Table {
                class: class("NSTableView"),
                alpha: Some(0.0),
            },
            Behind::Table {
                class: class("NSOutlineView"),
                alpha: Some(1.0),
            },
            Behind::Table {
                class: class("NSTableView"),
                alpha: None,
            },
            Behind::Effect {
                class: class("NSVisualEffectView"),
            },
            Behind::Scroller {
                class: class("NSScroller"),
                legacy: false,
            },
            Behind::Scroller {
                class: class("NSScroller"),
                legacy: true,
            },
        ];
        assert_eq!(
            covering(&behind),
            vec![
                class("NSScrollView"),
                class("NSOutlineView (backgroundColor)"),
                class("NSTableView (backgroundColor, not convertible to sRGB)"),
                class("NSVisualEffectView"),
                class("NSScroller (legacy knob slot)"),
            ]
        );
    }

    #[test]
    fn nothing_drawn_behind_covers_nothing() {
        let behind = [
            Behind::Background {
                class: class("NSScrollView"),
                draws: false,
            },
            Behind::Scroller {
                class: class("NSScroller"),
                legacy: false,
            },
        ];
        assert!(covering(&behind).is_empty());
        assert!(covering(&[]).is_empty());
    }

    const GREY: [u8; 4] = [0x80, 0x80, 0x80, 0xff];
    const KNOB: [u8; 4] = [0x40, 0x40, 0x40, 0xff];

    /// A 4 x 3 capture, grey but for `KNOB` at (1, 1) and (2, 1).
    fn capture() -> Rgba {
        let mut pixels = vec![GREY; 12];
        for i in [5, 6] {
            if let Some(p) = pixels.get_mut(i) {
                *p = KNOB;
            }
        }
        Rgba::from_pixels(4, 3, pixels)
    }

    #[test]
    fn a_knob_is_counted_only_inside_its_rect() {
        let rgba = capture();
        let whole = PxRect {
            x0: 0,
            y0: 0,
            x1: 4,
            y1: 3,
        };
        let one = PxRect {
            x0: 2,
            y0: 0,
            x1: 4,
            y1: 3,
        };
        let none = PxRect {
            x0: 0,
            y0: 0,
            x1: 1,
            y1: 3,
        };
        assert_eq!(differing_from(&rgba, &whole, GREY), 2);
        assert_eq!(differing_from(&rgba, &one, GREY), 1);
        assert_eq!(differing_from(&rgba, &none, GREY), 0);
        assert_eq!(differing_from(&rgba, &whole, KNOB), 10);
    }

    #[test]
    fn pixels_beyond_a_capture_are_not_counted() {
        let rgba = capture();
        let beyond = PxRect {
            x0: 2,
            y0: 1,
            x1: 9,
            y1: 9,
        };
        assert_eq!(differing_from(&rgba, &beyond, GREY), 1);
        let other = Rgba::from_pixels(4, 3, vec![GREY; 12]);
        assert_eq!(differing_between(&rgba, &other, &beyond), (4, 1));
    }

    #[test]
    fn two_captures_differ_where_their_pixels_do() {
        let rgba = capture();
        let grey = Rgba::from_pixels(4, 3, vec![GREY; 12]);
        let whole = PxRect {
            x0: 0,
            y0: 0,
            x1: 4,
            y1: 3,
        };
        assert_eq!(differing_between(&rgba, &grey, &whole), (12, 2));
        assert_eq!(differing_between(&rgba, &capture(), &whole), (12, 0));
    }

    fn r(x: f64, y: f64, w: f64, h: f64) -> NSRect {
        NSRect::new(NSPoint::new(x, y), NSSize::new(w, h))
    }

    #[test]
    fn a_titled_part_ends_where_its_title_does_within_the_bounds() {
        let bounds = r(0.0, 0.0, 160.0, 20.0);
        assert_eq!(
            titled_rect(bounds, r(20.0, 2.0, 41.5, 16.0)),
            r(0.0, 0.0, 61.5, 20.0)
        );
        assert_eq!(
            titled_rect(bounds, r(20.0, 2.0, 400.0, 16.0)),
            r(0.0, 0.0, 160.0, 20.0)
        );
        assert_eq!(
            titled_rect(bounds, r(-30.0, 2.0, 10.0, 16.0)),
            r(0.0, 0.0, 0.0, 20.0)
        );
    }

    #[test]
    fn the_pass_checks_the_appearances_and_backdrops_the_probe_writes() {
        let appearances: Vec<&str> = crate::macos::APPEARANCES.iter().map(|(k, _)| *k).collect();
        assert_eq!(appearances, crate::CONTROLS_APPEARANCES);
        let backdrops: Vec<&str> = Backdrop::ALL
            .iter()
            .filter(|b| **b != Backdrop::Window)
            .map(|b| b.key())
            .collect();
        assert_eq!(backdrops, crate::OVER_BACKDROPS);
    }
}
