//! The controls pass (`--button-pass --controls`): the controls other than
//! the push buttons, one sheet of them in a titled window made key, each
//! hovered by a posted mouse-moved event and pressed by a real mouse-down
//! captured during the press, as the button pass hovers and clicks its push
//! buttons; every state is captured over the window's own background and the
//! three opaque backdrops, so each state's layer is solved
//! ([`super::layer`]). The analysis reads each control's part from the
//! layers at the rects this pass records.

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{MainThreadOnly, Message};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSBezelStyle, NSBox, NSBoxType, NSButton,
    NSButtonType, NSControl, NSControlSize, NSPopUpButton, NSScrollView, NSSegmentedControl,
    NSSlider, NSSliderCell, NSSplitView, NSSwitch, NSTabView, NSTabViewItem, NSTableCellView,
    NSTableColumn, NSTableView, NSTableViewStyle, NSTextField, NSTitlePosition, NSToolbar,
    NSToolbarItem, NSView, NSWindow,
};
use objc2_foundation::{NSIndexSet, NSPoint, NSRect, NSSize, NSString};
use toml::{Table, Value};

use super::{
    Backdrop, Case, DIFF_MARGIN, Pass, Sheet, away_point, case_table, diff_table, from_top_left,
    px_ratio, sheet_window,
};
use crate::macos::layer::Rgba;
use crate::macos::{
    APPEARANCES, SETTLE_STEPS, control_common, ffi, ns, pixel_index, point_text, rect, state_value,
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
    /// Whether the pass presses it (every target is hovered).
    press: bool,
    /// Whether an Escape follows the mouse-up (the pop-up's menu).
    escape: bool,
    /// What the control is set back to after each press.
    reset: Box<dyn Fn()>,
}

/// The tab view and each tab's rect in its coordinates, where found.
struct Tabs {
    view: Retained<NSTabView>,
    rects: Vec<Option<NSRect>>,
}

/// The objects the sheet's views need alive while it is captured.
struct Keep {
    _sources: Vec<Retained<ffi::TableSource>>,
    _delegate: Retained<ffi::ToolbarDelegate>,
}

/// `--button-pass --controls`: the controls sheet, made key, captured per
/// appearance at rest, each target hovered and each target pressed.
pub(super) fn controls_table(pass: &Pass<'_>) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    let (sheet, targets, tabs, keep) = match build(pass) {
        Ok(built) => built,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let window = &sheet.window;
    let mut buttons = Table::new();
    let frame_size = window.frame().size;
    for case in &sheet.cases {
        out::put_table(&mut buttons, &case.name, case_table(case, frame_size));
    }
    out::put_table(&mut t, "controls", buttons);

    let other = pass.present(window, &mut t);
    out::put(
        &mut t,
        "window_backingScaleFactor",
        out::float(window.backingScaleFactor()),
    );
    out::put(&mut t, "window_frame_xywh", rect(window.frame()));
    out::put(
        &mut t,
        "backdrop_point_xy",
        out::floats(&[sheet.backdrop_point().x, sheet.backdrop_point().y]),
    );
    let frame_size = window.frame().size;
    let mut placed = Table::new();
    for target in &targets {
        out::put_table(&mut placed, target.name, target_table(target, frame_size));
    }
    out::put_table(&mut t, "targets", placed);
    out::put_table(&mut t, "tabs", tabs_table(&tabs, frame_size));

    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        window.setAppearance(Some(&appearance));
        sheet.content.setNeedsDisplay(true);
        pass.settle(window, SETTLE_STEPS);
        out::put_table(&mut t, key, appearance_table(pass, &sheet, &targets, key));
    }
    window.setAppearance(None);
    window.orderOut(None);
    window.close();
    if let Some(other) = other {
        other.orderOut(None);
        other.close();
    }
    drop(keep);
    t
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
    let away = away_point(window);
    let all: Vec<&Case> = sheet.cases.iter().collect();

    let mut normal = Table::new();
    out::put_table(&mut normal, "cursor", pass.move_cursor(window, away));
    let normal_rgba = pass
        .shots(sheet, &mut normal, &dir, &rel, "normal", &all, false, None)
        .window;
    out::put_table(&mut t, "normal", normal);

    let mut highlighted = Table::new();
    for case in &sheet.cases {
        if let Some(b) = &case.button {
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
    for target in targets {
        let mut h = Table::new();
        out::put_table(
            &mut h,
            "cursor",
            pass.move_cursor(window, screen_point(window, target)),
        );
        let stem = format!("hovered-{}", target.name);
        let rgba = pass
            .shots(sheet, &mut h, &dir, &rel, &stem, &[], false, None)
            .window;
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
        pass.move_cursor(window, away),
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
        pass.move_cursor(window, away),
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
    let point = sheet.backdrop_point();
    let stem = format!("clicked-{}", target.name);
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
        let at = screen_point(window, target);
        out::put_table(&mut sub, "cursor", pass.move_cursor(window, at));
        let file = format!("{stem}{}.png", backdrop.suffix());
        let rgba = pass.click_at(
            &mut sub,
            window,
            dir,
            rel,
            &file,
            pass.quartz(at),
            target.escape,
        );
        (target.reset)();
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
                &stem,
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
    let window_rgba = find(Backdrop::Window).map(|(rgba, _)| rgba);
    out::put(
        &mut t,
        "diff_from_normal",
        compare(normal, window_rgba, target, frame),
    );
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
    out::put(&mut t, "pressed", out::boolean(target.press));
    t
}

/// The sheet, its targets, the tab view's tab rects and what must stay
/// alive.
fn build(pass: &Pass<'_>) -> Result<(Sheet, Vec<Target>, Tabs, Keep), String> {
    let mtm = pass.mtm;
    let (window, content, backdrop) = sheet_window(mtm, CONTROLS_ORIGIN, CONTROLS_CONTENT, false)?;
    let height = CONTROLS_CONTENT.height;
    let size = NSControlSize::Regular;
    let mut cases: Vec<Case> = Vec::new();
    let mut targets: Vec<Target> = Vec::new();

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
    let indicator = |b: &NSButton| {
        b.cell()
            .map(|cell| cell.imageRectForBounds(b.bounds()))
            .ok_or_else(|| "the check box has no cell".to_owned())
    };
    for (name, b, on) in [
        ("checkbox_off", &check_off, false),
        ("checkbox_on", &check_on, true),
    ] {
        let reset_button = b.retain();
        targets.push(Target {
            name,
            view: view(b),
            part: indicator(b)?,
            press: true,
            escape: false,
            reset: Box::new(move || reset_button.setState(state_value(on))),
        });
    }
    for (name, b) in [
        ("disclosure", &disclosure),
        ("push_disclosure", &push_disclosure),
    ] {
        let reset_button = b.retain();
        targets.push(Target {
            name,
            view: view(b),
            part: b.bounds(),
            press: true,
            escape: false,
            reset: Box::new(move || reset_button.setState(state_value(false))),
        });
    }
    targets.push(Target {
        name: "push",
        view: view(&push),
        part: push.bounds(),
        press: true,
        escape: false,
        reset: Box::new(|| {}),
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
    let count = isize::try_from(SEGMENTS.len()).map_err(|e| e.to_string())?;
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
    place(&switch, c3, 1.0);
    let reset_popup = popup.retain();
    targets.push(Target {
        name: "popup",
        view: view(&popup),
        part: popup.bounds(),
        press: true,
        escape: true,
        reset: Box::new(move || reset_popup.selectItemAtIndex(0)),
    });
    targets.push(Target {
        name: "text_field",
        view: view(&field),
        part: field.bounds(),
        press: false,
        escape: false,
        reset: Box::new(|| {}),
    });
    let b = segmented.bounds();
    let reset_segmented = segmented.retain();
    targets.push(Target {
        name: "segment",
        view: view(&segmented),
        part: NSRect::new(
            NSPoint::new(b.origin.x + b.size.width / 3.0, b.origin.y),
            NSSize::new(b.size.width / 3.0, b.size.height),
        ),
        press: true,
        escape: false,
        reset: Box::new(move || reset_segmented.setSelectedSegment(0)),
    });
    let reset_switch = switch.retain();
    targets.push(Target {
        name: "switch_off",
        view: view(&switch),
        part: switch.bounds(),
        press: true,
        escape: false,
        reset: Box::new(move || reset_switch.setState(state_value(false))),
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
    let knob = slider
        .cell()
        .and_then(|cell| {
            cell.downcast_ref::<NSSliderCell>()
                .map(|c| c.knobRectFlipped(slider.isFlipped()))
        })
        .ok_or_else(|| "the slider's cell is not an NSSliderCell".to_owned())?;
    let reset_slider = slider.retain();
    targets.push(Target {
        name: "slider_knob",
        view: view(&slider),
        part: knob,
        press: true,
        escape: false,
        reset: Box::new(move || reset_slider.setDoubleValue(SLIDER_VALUE)),
    });

    // The tables: one in the default style, one in the source-list style.
    let mut sources = Vec::new();
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
        let reset_table = table.retain();
        targets.push(Target {
            name,
            view: view(&table),
            part: table.rectOfRow(TABLE_TARGET_ROW),
            press: true,
            escape: false,
            reset: Box::new(move || {
                reset_table.selectRowIndexes_byExtendingSelection(&NSIndexSet::new(), false);
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
    let tabs = Tabs {
        rects: tab_rects(&tab_view),
        view: tab_view.retain(),
    };
    let tab_part = tabs
        .rects
        .get(TAB_TARGET)
        .copied()
        .flatten()
        .ok_or_else(|| "tabViewItemAtPoint: found no rect for the target tab".to_owned())?;
    let reset_tabs = tab_view.retain();
    targets.push(Target {
        name: "tab",
        view: view(&tab_view),
        part: tab_part,
        press: true,
        escape: false,
        reset: Box::new(move || reset_tabs.selectTabViewItemAtIndex(0)),
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
    let first = split
        .subviews()
        .iter()
        .next()
        .map(|v| v.frame())
        .ok_or_else(|| "the split view has no subview".to_owned())?;
    let thickness = split.dividerThickness();
    targets.push(Target {
        name: "split_divider",
        view: view(&split),
        part: NSRect::new(
            NSPoint::new(first.origin.x + first.size.width, 0.0),
            NSSize::new(thickness, split.bounds().size.height),
        ),
        press: false,
        escape: false,
        reset: Box::new(|| {}),
    });

    let card = NSBox::initWithFrame(NSBox::alloc(mtm), BOX_FRAME);
    card.setBoxType(NSBoxType::Primary);
    card.setTitlePosition(NSTitlePosition::NoTitle);
    content.addSubview(&card);
    targets.push(Target {
        name: "box",
        view: view(&card),
        part: card.bounds(),
        press: false,
        escape: false,
        reset: Box::new(|| {}),
    });

    // A button in a toolbar item.
    let toolbar_button = NSButton::new(mtm);
    toolbar_button.setButtonType(NSButtonType::MomentaryPushIn);
    toolbar_button.setBezelStyle(NSBezelStyle::Toolbar);
    toolbar_button.setTitle(&ns(TITLE));
    toolbar_button.sizeToFit();
    let item = NSToolbarItem::initWithItemIdentifier(NSToolbarItem::alloc(mtm), &ns("button"));
    item.setAutovalidates(false);
    item.setEnabled(true);
    let v: &NSView = &toolbar_button;
    item.setView(Some(v));
    let delegate = ffi::ToolbarDelegate::new(mtm, vec![item]);
    let toolbar =
        NSToolbar::initWithIdentifier(NSToolbar::alloc(mtm), &ns("NativeThemeOsProbeControls"));
    toolbar.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    window.setToolbar(Some(&toolbar));
    targets.push(Target {
        name: "toolbar_button",
        view: view(&toolbar_button),
        part: toolbar_button.bounds(),
        press: true,
        escape: false,
        reset: Box::new(|| {}),
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
            "NSButton, NSButtonTypePushOnPushOff, NSBezelStyleDisclosure, titled, state off",
            &disclosure,
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
            "popup",
            "NSPopUpButton, two items, the first selected",
            Retained::into_super(Retained::into_super(popup.retain())),
        ),
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
        tabs,
        Keep {
            _sources: sources,
            _delegate: delegate,
        },
    ))
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
        "one titled NSWindow ({} x {} pt content from {}, empty title) holding the button pass's backdrop box and, at the regular control size with systemFontOfSize: of systemFontSizeForControlSize:, sized to fit and centred in rows {ROW_STEP} pt apart from the content's top: row 0 a check box off and one on (title \"{CHECK_TITLE}\"), a disclosure button (titled \"{DISCLOSURE_TITLE}\") and a push disclosure button, both push-on-push-off and off, and a push button (\"{TITLE}\"); row 1 a pop-up button ({POPUP_ITEMS:?}, the first selected), an editable bezeled text field (\"{FIELD_TEXT}\"), a segmented control ({SEGMENTS:?}, each {SEGMENT_WIDTH} pt wide, segment 0 selected) and a switch, off; row 2 a horizontal slider, 0 to {} at {SLIDER_VALUE}, {SLIDER_WIDTH} pt wide; below them two view-based tables of {TABLE_ROWS} rows (one column {TABLE_COLUMN_WIDTH} pt, no header, each cell a label \"{ROW_TEXT}\", no row selected) in scroll views at {} (default style) and {} (NSTableViewStyleSourceList), an NSTabView at {} with tabs {TABS:?} (the first selected, its items without views), a vertical NSSplitView at {} with two empty panes, and an NSBox (NSBoxPrimary, no title) at {}; the window's NSToolbar holds one item whose view is an NSButton (NSBezelStyleToolbar, \"{TITLE}\"); the window made key as a button sheet's; per appearance: normal (the cursor parked right of the window), highlighted (highlight: YES on every NSButton), then per target the cursor moved to the centre of the target's part as the button pass's hovered state moves it, captured over every backdrop (hovered-<target>), and per pressed target, per backdrop, the cursor moved there and a real mouse-down captured during the press as the button pass's clicked state takes it (clicked-<target>[-over-<backdrop>].png; the pop-up's mouse-up followed by an Escape), the control set back after each press (targets: their parts' rects; the table rows: rectOfRow: {TABLE_TARGET_ROW}; the segment: the middle third; the slider: knobRectFlipped:; the tab: tab {TAB_TARGET}'s rect from the tabs table; the split divider: from the first pane's right edge, dividerThickness wide); each layer is solved from the black and white captures as the button pass's (layer_method); diff_from_normal compares a state's capture over the window's background with the normal one inside the target's view frame grown by {DIFF_MARGIN} pt",
        CONTROLS_CONTENT.width,
        CONTROLS_CONTENT.height,
        point_text(CONTROLS_ORIGIN),
        2.0 * SLIDER_VALUE,
        point_text(TABLE_FRAME.origin),
        point_text(SOURCE_LIST_FRAME.origin),
        point_text(TAB_VIEW_FRAME.origin),
        point_text(SPLIT_FRAME.origin),
        point_text(BOX_FRAME.origin),
    )
}
