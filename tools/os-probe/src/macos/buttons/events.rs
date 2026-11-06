//! The events pass (`--button-pass --events`; the 0.8.0 measurement plan's
//! MP-events): posted mouse and keyboard events over a window of the probe,
//! and what AppKit made of them. Two clicks at stepped distances and gaps,
//! each mouse-down's `clickCount` read (U321, U608, U329's
//! `double_click_height`); a press on selected text held for stepped times
//! and dragged by stepped distances, whether a drag session started (the
//! drag pasteboard's change count; U329's `drag_threshold` and
//! `drag_min_delay`); the cursor brought onto a button with a tool tip and
//! the tool tip's window watched for (U329's `hover_time`); and two screen
//! recordings ([`timeline`]) per appearance: typing into a text view under
//! the cursor, the cursor recorded (U320, U607, U329's
//! `hide_pointer_delay` and `hide_pointer_fade`), and a drag of text over a
//! table that validates drops, a text view and a spring-loaded button, with
//! window captures while it hovers each (U322, U600, U302's spring-loading
//! flash, U329's `spring_loading`). The user defaults these may follow are
//! read first.

use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSApplication, NSBezelStyle, NSButton, NSButtonType,
    NSControl, NSControlSize, NSEvent, NSEventMask, NSEventType, NSResponder, NSScreen,
    NSScrollView, NSTableColumn, NSTableView, NSTextField, NSTextView, NSView, NSWindow,
};
use objc2_core_graphics::{
    CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, CGWarpMouseCursorPosition,
};
use objc2_foundation::{NSArray, NSDate, NSPoint, NSRange, NSRect, NSSize, NSUserDefaults};
use toml::{Table, Value};

use super::timeline::{self, Posted, Recording, action, drag_line, grown, key, post, screen_frame};
use super::{Pass, SETTLE_STEPS};
use crate::macos::{
    APPEARANCES, control_common, default_value, ffi, ns, point_text, pump, rect, rect_text,
};
use crate::out;

/// The window's content frame on the screen.
const WINDOW_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 200.0), NSSize::new(560.0, 340.0));

/// The controls' frames in the content view (bottom-left origin).
const SOURCE_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 260.0), NSSize::new(220.0, 40.0));
const TIP_ORIGIN: NSPoint = NSPoint::new(260.0, 270.0);
const SPRING_ORIGIN: NSPoint = NSPoint::new(20.0, 200.0);
const TARGET_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 120.0), NSSize::new(220.0, 60.0));
const TABLE_FRAME: NSRect = NSRect::new(NSPoint::new(300.0, 20.0), NSSize::new(240.0, 200.0));
const TABLE_COLUMN_WIDTH: f64 = 220.0;
const TABLE_ROWS: usize = 6;
const TICKER_FRAME: NSRect = NSRect::new(NSPoint::new(544.0, 324.0), NSSize::new(8.0, 8.0));

/// The empty point the double clicks land on and the drag ends at, from
/// the content view's bottom left.
const EMPTY_POINT: NSPoint = NSPoint::new(130.0, 60.0);

/// The text dragged, all of it selected, and where in the source text view
/// the press lands, from its top left.
const DRAG_TEXT: &str = "drag drag drag drag";
const PRESS_AT: (f64, f64) = (12.0, 8.0);
const TARGET_TEXT: &str = "drop here";

/// The double clicks: the distances between the two clicks, in points
/// (along x, then along y, the second click 100 ms after the first), and
/// the gaps between their mouse-downs at one point, in milliseconds.
const CLICK_DISTANCES: [f64; 11] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 12.0, 16.0, 24.0];
const CLICK_GAP_MS: u64 = 100;
const CLICK_GAPS_MS: [u64; 9] = [100, 200, 300, 400, 450, 500, 550, 600, 800];
const CLICK_HOLD_MS: u64 = 30;

/// The drag trials: distances in points with the press held
/// `DRAG_HOLD_MS` first, and holds in milliseconds before a
/// `DRAG_DISTANCE` drag; one point of movement every `DRAG_STEP_MS`.
const DRAG_DISTANCES: [u32; 8] = [1, 2, 3, 4, 5, 6, 8, 10];
const DRAG_HOLD_MS: u64 = 800;
const DRAG_HOLDS_MS: [u64; 8] = [0, 50, 100, 150, 200, 300, 500, 800];
const DRAG_DISTANCE: u32 = 12;
const DRAG_STEP_MS: u64 = 16;
const DRAG_RELEASE_AFTER_MS: u64 = 300;

/// The tool tip trials' longest wait, and the rests before each.
const TIP_WAIT: Duration = Duration::from_secs(8);
const TIP_TRIALS: [(&str, u64); 3] = [("cold", 1000), ("warm", 300), ("rested", 5000)];

/// The user defaults read.
const DEFAULTS: [&str; 10] = [
    "NSInitialToolTipDelay",
    "NSDragAndDropTextDelay",
    "com.apple.springing.enabled",
    "com.apple.springing.delay",
    "com.apple.mouse.doubleClickThreshold",
    "com.apple.mouse.scaling",
    "AppleEnableMouseSwipeNavigateWithScrolls",
    "NSTextInsertionPointBlinkPeriodOn",
    "NSTextInsertionPointBlinkPeriodOff",
    "NSUserKeyEquivalents",
];

/// Virtual key codes (HIToolbox's Events.h: kVK_ANSI_A, kVK_ANSI_S,
/// kVK_ANSI_D).
const KEY_A: u16 = 0;
const KEY_S: u16 = 1;
const KEY_D: u16 = 2;

/// How far the typing recording's crop reaches round the text view.
const TYPING_CROP_MARGIN: f64 = 40.0;

struct Controls {
    window: Retained<NSWindow>,
    content: Retained<NSView>,
    source: Retained<NSTextView>,
    tip: Retained<NSButton>,
    spring: Retained<NSButton>,
    target: Retained<NSTextView>,
    table: Retained<NSTableView>,
    drop: Retained<ffi::DropTable>,
}

/// `[macos.button_pass.events]`.
pub(super) fn events_table(pass: &Pass<'_>) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    out::put_table(&mut t, "defaults", defaults_table());
    let mut event = Table::new();
    out::put(
        &mut event,
        "doubleClickInterval_s",
        out::float(NSEvent::doubleClickInterval()),
    );
    out::put(
        &mut event,
        "keyRepeatDelay_s",
        out::float(NSEvent::keyRepeatDelay()),
    );
    out::put_table(&mut t, "NSEvent", event);
    let c = match build(pass) {
        Ok(c) => c,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let ticker = timeline::ticker(pass, &c.content, TICKER_FRAME);
    let _ = pass.present(&c.window, &mut t);
    c.table.reloadData();
    pass.settle(&c.window, SETTLE_STEPS);
    let scale = NSScreen::mainScreen(pass.mtm).map(|s| s.backingScaleFactor());
    match scale {
        Some(scale) => out::put(&mut t, "mainScreen_backingScaleFactor", out::float(scale)),
        None => out::put(
            &mut t,
            "mainScreen_backingScaleFactor",
            out::unavailable("mainScreen is nil"),
        ),
    }
    out::put(&mut t, "window_frame_xywh", rect(c.window.frame()));
    let mut frames = Table::new();
    for (name, view) in [
        ("source", as_view(&c.source)),
        ("tip", as_view(&c.tip)),
        ("spring", as_view(&c.spring)),
        ("target", as_view(&c.target)),
        ("table", as_view(&c.table)),
    ] {
        out::put(&mut frames, name, rect(screen_frame(view)));
    }
    out::put_table(&mut t, "frames_screen_xywh", frames);
    out::put_table(
        &mut t,
        "cursor",
        pass.move_cursor(&c.window, super::away_point(&c.window)),
    );

    out::put_table(&mut t, "double_click", double_click_table(pass, &c));
    out::put_table(&mut t, "drag_start", drag_start_table(pass, &c));
    out::put_table(&mut t, "tool_tip", tool_tip_table(pass, &c));

    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        c.window.setAppearance(Some(&appearance));
        c.content.setNeedsDisplay(true);
        pass.settle(&c.window, SETTLE_STEPS);
        let mut per = Table::new();
        let typing = typing_recording(pass, &c, key);
        let r = timeline::record(pass, &ticker, scale, pass.out_dir, typing);
        out::put_table(&mut per, "typing-pointer", r);
        reset(pass, &c);
        let changes_before = ffi::drag_pasteboard_change_count();
        let validated_before = c.drop.ivars().validated.get();
        let spring_before = c.spring.state();
        let drag = drag_recording(pass, &c, key);
        let mut r = timeline::record(pass, &ticker, scale, pass.out_dir, drag);
        pass.settle(&c.window, SETTLE_STEPS);
        out::put(
            &mut r,
            "drag_pasteboard_changes",
            out::int_isize(ffi::drag_pasteboard_change_count().saturating_sub(changes_before)),
        );
        out::put(
            &mut r,
            "table_validateDrop_calls",
            out::int(i64::from(
                c.drop
                    .ivars()
                    .validated
                    .get()
                    .saturating_sub(validated_before),
            )),
        );
        let (row, operation) = c.drop.ivars().last.get();
        out::put(&mut r, "table_last_proposed_row", out::int_isize(row));
        out::put(
            &mut r,
            "table_last_drop_operation",
            out::text(match operation {
                0 => "on",
                1 => "above",
                _ => "other",
            }),
        );
        out::put(&mut r, "spring_state_before", out::int_isize(spring_before));
        out::put(
            &mut r,
            "spring_state_after",
            out::int_isize(c.spring.state()),
        );
        out::put(
            &mut r,
            "source_text_after",
            out::text(c.source.string().to_string()),
        );
        out::put(
            &mut r,
            "target_text_after",
            out::text(c.target.string().to_string()),
        );
        out::put_table(&mut per, "drag-over", r);
        reset(pass, &c);
        out::put_table(&mut t, key, per);
    }
    c.window.setAppearance(None);
    ffi::set_drop_table(&c.table, None);
    c.window.orderOut(None);
    c.window.close();
    t
}

fn as_view<T: AsRef<NSView>>(v: &Retained<T>) -> &NSView {
    let inner: &T = v;
    inner.as_ref()
}

fn defaults_table() -> Table {
    let defaults = NSUserDefaults::standardUserDefaults();
    let mut d = Table::new();
    for key in DEFAULTS {
        let value = match defaults.objectForKey(&ns(key)) {
            None => out::text("absent (objectForKey: is nil)"),
            Some(obj) => default_value(&obj),
        };
        out::put(&mut d, key, value);
    }
    d
}

/// Put the texts, the spring-loaded button and the first responder back.
fn reset(pass: &Pass<'_>, c: &Controls) {
    c.source.setString(&ns(DRAG_TEXT));
    c.target.setString(&ns(TARGET_TEXT));
    c.spring.setState(0);
    let _ = c.window.makeFirstResponder(None);
    c.window.makeKeyAndOrderFront(None);
    pass.settle(&c.window, SETTLE_STEPS);
}

fn build(pass: &Pass<'_>) -> Result<Controls, String> {
    let mtm = pass.mtm;
    let window = ffi::titled_window(mtm, WINDOW_FRAME)?;
    window.setTitle(&ns(""));
    let content = window
        .contentView()
        .ok_or_else(|| "the titled window has no content view".to_owned())?;
    let text = |frame: NSRect, string: &str| {
        let v = NSTextView::initWithFrame(NSTextView::alloc(mtm), frame);
        v.setString(&ns(string));
        v.setEditable(true);
        v.setSelectable(true);
        content.addSubview(&v);
        v
    };
    let source = text(SOURCE_FRAME, DRAG_TEXT);
    let target = text(TARGET_FRAME, TARGET_TEXT);
    let button = |title: &str, origin: NSPoint| {
        let b = NSButton::new(mtm);
        b.setButtonType(NSButtonType::MomentaryPushIn);
        b.setBezelStyle(NSBezelStyle::Push);
        b.setTitle(&ns(title));
        control_common(&b, NSControlSize::Regular);
        b.sizeToFit();
        b.setFrameOrigin(origin);
        content.addSubview(&b);
        b
    };
    let tip = button("Tip", TIP_ORIGIN);
    tip.setToolTip(Some(&ns("Tip")));
    let spring = button("Spring", SPRING_ORIGIN);
    spring.setButtonType(NSButtonType::PushOnPushOff);
    spring.setState(0);
    spring.setSpringLoaded(true);
    let control: &NSControl = &spring;
    ffi::set_harmless_action(control, pass.app);

    let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), TABLE_FRAME);
    let table = NSTableView::new(mtm);
    let column = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), &ns("column"));
    column.setWidth(TABLE_COLUMN_WIDTH);
    table.addTableColumn(&column);
    let rows: Vec<Retained<NSView>> = (0..TABLE_ROWS)
        .map(|i| {
            Retained::into_super(Retained::into_super(NSTextField::labelWithString(
                &ns(&format!("Row {i}")),
                mtm,
            )))
        })
        .collect();
    let drop = ffi::DropTable::new(mtm, rows);
    ffi::set_drop_table(&table, Some(&drop));
    let types = NSArray::from_slice(&[ffi::string_pasteboard_type()]);
    table.registerForDraggedTypes(&types);
    scroll.setDocumentView(Some(&table));
    content.addSubview(&scroll);
    table.reloadData();
    scroll.tile();
    table.tile();
    Ok(Controls {
        window,
        content,
        source,
        tip,
        spring,
        target,
        table,
        drop,
    })
}

/// Post a mouse event of `kind` at `at` (Quartz points) from this thread.
fn post_mouse(kind: CGEventType, at: NSPoint) -> bool {
    let e = CGEvent::new_mouse_event(None, kind, at, CGMouseButton::Left);
    let made = e.is_some();
    CGEvent::post(CGEventTapLocation::HIDEventTap, e.as_deref());
    made
}

/// Pump the run loop for `span`, dispatching every event and noting each
/// left mouse-down's click count.
fn pump_clicks(app: &NSApplication, span: Duration) -> Vec<Value> {
    let start = Instant::now();
    let mode = ffi::default_run_loop_mode();
    let mut counts = Vec::new();
    while start.elapsed() < span {
        let until = NSDate::dateWithTimeIntervalSinceNow(0.005);
        while let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
            NSEventMask::Any,
            Some(&until),
            mode,
            true,
        ) {
            if event.r#type() == NSEventType::LeftMouseDown {
                counts.push(out::int_isize(event.clickCount()));
            }
            app.sendEvent(&event);
        }
        app.updateWindows();
    }
    counts
}

/// The double-click trials over the empty point.
fn double_click_table(pass: &Pass<'_>, c: &Controls) -> Table {
    let mut t = Table::new();
    let interval = NSEvent::doubleClickInterval();
    let rest = Duration::from_secs_f64(interval.max(0.0)) + Duration::from_millis(300);
    let p0 = content_quartz(pass, c, EMPTY_POINT);
    out::put(&mut t, "first_click_quartz_xy", out::floats(&[p0.x, p0.y]));
    let trial = |p1: NSPoint, gap_ms: u64| -> Value {
        let gap = Duration::from_millis(gap_ms.saturating_sub(CLICK_HOLD_MS));
        let hold = Duration::from_millis(CLICK_HOLD_MS);
        let mut made = post_mouse(CGEventType::LeftMouseDown, p0);
        std::thread::sleep(hold);
        made &= post_mouse(CGEventType::LeftMouseUp, p0);
        std::thread::sleep(gap);
        made &= post_mouse(CGEventType::LeftMouseDown, p1);
        std::thread::sleep(hold);
        made &= post_mouse(CGEventType::LeftMouseUp, p1);
        let mut counts = pump_clicks(pass.app, Duration::from_millis(300));
        counts.extend(pump_clicks(pass.app, rest));
        let mut row = Table::new();
        out::put(&mut row, "events_created", out::boolean(made));
        out::put(
            &mut row,
            "second_click_quartz_xy",
            out::floats(&[p1.x, p1.y]),
        );
        out::put(&mut row, "mouseDown_clickCounts", out::array(counts));
        Value::Table(row)
    };
    let mut along_x = Table::new();
    let mut along_y = Table::new();
    for d in CLICK_DISTANCES {
        out::put(
            &mut along_x,
            &format!("{d} pt"),
            trial(NSPoint::new(p0.x + d, p0.y), CLICK_GAP_MS),
        );
    }
    for d in CLICK_DISTANCES {
        out::put(
            &mut along_y,
            &format!("{d} pt"),
            trial(NSPoint::new(p0.x, p0.y + d), CLICK_GAP_MS),
        );
    }
    let mut gaps = Table::new();
    for gap in CLICK_GAPS_MS {
        out::put(&mut gaps, &format!("{gap} ms"), trial(p0, gap));
    }
    out::put_table(&mut t, "distance_x", along_x);
    out::put_table(&mut t, "distance_y", along_y);
    out::put_table(&mut t, "gap_same_point", gaps);
    t
}

/// The point `p` of the content view (bottom-left origin) in Quartz points.
fn content_quartz(pass: &Pass<'_>, c: &Controls, p: NSPoint) -> NSPoint {
    let r = c
        .content
        .convertRect_toView(NSRect::new(p, NSSize::new(0.0, 0.0)), None);
    pass.quartz(c.window.convertRectToScreen(r).origin)
}

/// The drag-start trials on the source text view.
fn drag_start_table(pass: &Pass<'_>, c: &Controls) -> Table {
    let mut t = Table::new();
    let press = timeline::quartz_at(pass, &c.source, PRESS_AT.0, PRESS_AT.1);
    out::put(&mut t, "press_quartz_xy", out::floats(&[press.x, press.y]));
    let source: &NSResponder = &c.source;
    let trial = |hold_ms: u64, distance: u32| -> Value {
        let mut row = Table::new();
        c.source.setString(&ns(DRAG_TEXT));
        c.source
            .setSelectedRange(NSRange::new(0, DRAG_TEXT.encode_utf16().count()));
        out::put(
            &mut row,
            "makeFirstResponder",
            out::boolean(c.window.makeFirstResponder(Some(source))),
        );
        pass.settle(&c.window, SETTLE_STEPS);
        let before = ffi::drag_pasteboard_change_count();
        let worker = std::thread::spawn(move || {
            let mut made = post_mouse(CGEventType::LeftMouseDown, press);
            std::thread::sleep(Duration::from_millis(hold_ms));
            let mut end = press;
            for i in 1..=distance {
                end = NSPoint::new(press.x + f64::from(i), press.y);
                made &= post_mouse(CGEventType::LeftMouseDragged, end);
                std::thread::sleep(Duration::from_millis(DRAG_STEP_MS));
            }
            std::thread::sleep(Duration::from_millis(DRAG_RELEASE_AFTER_MS));
            made &= post_mouse(CGEventType::LeftMouseUp, end);
            made
        });
        let steps = pump(pass.app, 400, || worker.is_finished());
        let made = worker.join().unwrap_or(false);
        pass.settle(&c.window, SETTLE_STEPS);
        out::put(&mut row, "events_created", out::boolean(made));
        out::put(&mut row, "pump_steps", out::int(i64::from(steps)));
        out::put(
            &mut row,
            "drag_pasteboard_changes",
            out::int_isize(ffi::drag_pasteboard_change_count().saturating_sub(before)),
        );
        out::put(
            &mut row,
            "source_text_after",
            out::text(c.source.string().to_string()),
        );
        let selected = c.source.selectedRange();
        out::put(
            &mut row,
            "selectedRange_after",
            out::text(format!("{} +{}", selected.location, selected.length)),
        );
        Value::Table(row)
    };
    let mut distances = Table::new();
    for d in DRAG_DISTANCES {
        out::put(&mut distances, &format!("{d} pt"), trial(DRAG_HOLD_MS, d));
    }
    let mut holds = Table::new();
    for h in DRAG_HOLDS_MS {
        out::put(&mut holds, &format!("{h} ms"), trial(h, DRAG_DISTANCE));
    }
    out::put_table(&mut t, "distance_after_hold", distances);
    out::put_table(&mut t, "hold_before_distance", holds);
    let _ = c.window.makeFirstResponder(None);
    c.source.setString(&ns(DRAG_TEXT));
    t
}

/// Move the cursor to `at` (Quartz points): warp it and post a mouse-moved
/// event there.
fn move_to(at: NSPoint) -> bool {
    let _ = CGWarpMouseCursorPosition(at);
    post_mouse(CGEventType::MouseMoved, at)
}

/// The visible windows of the application whose class name holds
/// "ToolTip": their class names and frames.
fn tool_tip_windows(app: &NSApplication) -> Vec<String> {
    app.windows()
        .iter()
        .filter(|w| w.isVisible())
        .filter_map(|w| {
            let any: &objc2::runtime::AnyObject = &w;
            let class = any.class().name().to_string_lossy().into_owned();
            class
                .contains("ToolTip")
                .then(|| format!("{class} {}", rect_text(w.frame())))
        })
        .collect()
}

/// The tool tip trials over the Tip button.
fn tool_tip_table(pass: &Pass<'_>, c: &Controls) -> Table {
    let mut t = Table::new();
    let away = pass.quartz(super::away_point(&c.window));
    let tip = timeline::quartz_centre(pass, &c.tip);
    out::put(&mut t, "tip_quartz_xy", out::floats(&[tip.x, tip.y]));
    for (name, rest_ms) in TIP_TRIALS {
        let mut row = Table::new();
        out::put(&mut row, "moved_away", out::boolean(move_to(away)));
        let _ = pump_clicks(pass.app, Duration::from_millis(rest_ms));
        out::put(
            &mut row,
            "rest_ms",
            out::int(i64::try_from(rest_ms).unwrap_or(i64::MAX)),
        );
        out::put(
            &mut row,
            "visible_before",
            out::array(
                tool_tip_windows(pass.app)
                    .into_iter()
                    .map(out::text)
                    .collect(),
            ),
        );
        let start = Instant::now();
        out::put(&mut row, "moved_onto_tip", out::boolean(move_to(tip)));
        let mut found = None;
        while start.elapsed() < TIP_WAIT {
            let _ = pump_clicks(pass.app, Duration::from_millis(5));
            let windows = tool_tip_windows(pass.app);
            if !windows.is_empty() {
                found = Some((start.elapsed(), windows));
                break;
            }
        }
        match found {
            Some((at, windows)) => {
                out::put(
                    &mut row,
                    "shown_after_ms",
                    out::float(at.as_secs_f64() * 1000.0),
                );
                out::put(
                    &mut row,
                    "windows",
                    out::array(windows.into_iter().map(out::text).collect()),
                );
            }
            None => {
                let all: Vec<Value> = pass
                    .app
                    .windows()
                    .iter()
                    .filter(|w| w.isVisible())
                    .map(|w| {
                        let any: &objc2::runtime::AnyObject = &w;
                        out::text(format!(
                            "{} {}",
                            any.class().name().to_string_lossy(),
                            rect_text(w.frame())
                        ))
                    })
                    .collect();
                out::put(
                    &mut row,
                    "shown_after_ms",
                    out::unavailable(format!(
                        "no visible window whose class name holds ToolTip within {} ms",
                        TIP_WAIT.as_millis()
                    )),
                );
                out::put(&mut row, "visible_windows", out::array(all));
            }
        }
        out::put_table(&mut t, name, row);
    }
    let _ = move_to(away);
    let _ = pump_clicks(pass.app, Duration::from_millis(500));
    t
}

/// Typing into the target text view under the cursor, the cursor recorded.
fn typing_recording<'a>(pass: &Pass<'_>, c: &'a Controls, key_name: &str) -> Recording<'a> {
    let target: &NSResponder = &c.target;
    let window: &NSWindow = &c.window;
    let text = &c.target;
    let inside = timeline::quartz_centre(pass, &c.target);
    let nudged = NSPoint::new(inside.x + 1.0, inside.y);
    let mut posts = vec![post(2200, "move onto the text view", Posted::Move(inside))];
    key(&mut posts, 3000, KEY_A, "type a");
    key(&mut posts, 3200, KEY_S, "type s");
    key(&mut posts, 3400, KEY_D, "type d");
    posts.push(post(6000, "move by 1 pt", Posted::Move(nudged)));
    key(&mut posts, 7000, KEY_A, "type a");
    key(&mut posts, 7200, KEY_S, "type s");
    Recording {
        stem: format!("{key_name}/typing-pointer"),
        crop: grown(screen_frame(&c.target), TYPING_CROP_MARGIN),
        seconds: 10,
        cursor: true,
        actions: vec![
            action(2000, "makeFirstResponder the text view", move || {
                let made = window.makeFirstResponder(Some(target));
                text.setSelectedRange(NSRange::new(text.string().length(), 0));
                out::boolean(made)
            }),
            action(9000, "makeFirstResponder nil", move || {
                out::boolean(window.makeFirstResponder(None))
            }),
        ],
        posts,
        sample: None,
        sample_cursor: true,
    }
}

/// A point of the table in Quartz points: `(x, y)` in the table's own
/// (flipped) coordinates.
fn table_quartz(pass: &Pass<'_>, c: &Controls, x: f64, y: f64) -> NSPoint {
    let r = c
        .table
        .convertRect_toView(NSRect::new(NSPoint::new(x, y), NSSize::new(0.0, 0.0)), None);
    pass.quartz(c.window.convertRectToScreen(r).origin)
}

/// A drag of the source's text over the table (on a row, then between two),
/// the target text view and the spring-loaded button, released over the
/// empty point; the window captured while it hovers each.
fn drag_recording<'a>(pass: &Pass<'_>, c: &'a Controls, key_name: &str) -> Recording<'a> {
    let source: &NSResponder = &c.source;
    let window: &NSWindow = &c.window;
    let text = &c.source;
    let press = timeline::quartz_at(pass, &c.source, PRESS_AT.0, PRESS_AT.1);
    let started = NSPoint::new(press.x + 30.0, press.y);
    let row1 = c.table.rectOfRow(1);
    let row2 = c.table.rectOfRow(2);
    let on_row = table_quartz(
        pass,
        c,
        row1.origin.x + row1.size.width / 2.0,
        row1.origin.y + row1.size.height / 2.0,
    );
    let between = table_quartz(
        pass,
        c,
        row2.origin.x + row2.size.width / 2.0,
        row2.origin.y + row2.size.height,
    );
    let in_text = timeline::quartz_centre(pass, &c.target);
    let on_spring = timeline::quartz_centre(pass, &c.spring);
    let empty = content_quartz(pass, c, EMPTY_POINT);
    let number = c.window.windowNumber();
    let dir = pass.out_dir.join(key_name);
    let capture = |at: u64, file: &str| {
        post(
            at,
            &format!("capture {file}"),
            Posted::Capture(number, dir.join(file), format!("{key_name}/{file}")),
        )
    };
    let mut posts = vec![post(1800, "move onto the text", Posted::Move(press))];
    posts.push(post(
        2000,
        "mouse-down on the selection",
        Posted::Down(press),
    ));
    drag_line(&mut posts, 3000, 3300, press, started, "drag 30 pt");
    drag_line(&mut posts, 3300, 3800, started, on_row, "drag onto row 1");
    posts.push(capture(4500, "drag-over-table-on.png"));
    drag_line(
        &mut posts,
        5000,
        5300,
        on_row,
        between,
        "drag between rows 2 and 3",
    );
    posts.push(capture(6300, "drag-over-table-above.png"));
    drag_line(
        &mut posts,
        6600,
        7000,
        between,
        in_text,
        "drag onto the text view",
    );
    posts.push(capture(8000, "drag-over-text.png"));
    drag_line(
        &mut posts,
        8300,
        8600,
        in_text,
        on_spring,
        "drag onto the spring-loaded button",
    );
    posts.push(capture(9000, "drag-over-spring-early.png"));
    posts.push(capture(11000, "drag-over-spring-late.png"));
    drag_line(
        &mut posts,
        12500,
        12900,
        on_spring,
        empty,
        "drag to the empty point",
    );
    posts.push(post(13500, "mouse-up", Posted::Up(empty)));
    let _ = std::fs::create_dir_all(&dir);
    Recording {
        stem: format!("{key_name}/drag-over"),
        crop: screen_frame(&c.content),
        seconds: 16,
        cursor: false,
        actions: vec![action(1000, "select the source's text", move || {
            text.setSelectedRange(NSRange::new(0, text.string().length()));
            out::boolean(window.makeFirstResponder(Some(source)))
        })],
        posts,
        sample: None,
        sample_cursor: false,
    }
}

fn method_text() -> String {
    format!(
        "a titled window (empty title, content {} from {}) holding, in its content view (bottom-left origin): an editable NSTextView \"{DRAG_TEXT}\" {} (the drag source); a push button \"Tip\" (regular, sized to fit) from {} with the tool tip \"Tip\"; a push button \"Spring\" from {}, buttonType pushOnPushOff, state off, springLoaded YES, action updateWindows at NSApp; an editable NSTextView \"{TARGET_TEXT}\" {} (the drop target and the typing target); an NSScrollView {} holding a view-based NSTableView (one {TABLE_COLUMN_WIDTH} pt column, {TABLE_ROWS} rows, each a labelWithString: \"Row <n>\") whose data source registers for NSPasteboardTypeString drags, answers tableView:validateDrop:proposedRow:proposedDropOperation: with NSDragOperationCopy for the row and operation AppKit proposes (counting the calls and keeping the last) and tableView:acceptDrop:row:dropOperation: with NO; a ticker {}. The window presented as the key mode presents it, the cursor parked {} pt right of it. Every event is a CGEvent posted at the HID event tap (left button). double_click: per trial, a mouse-down and a mouse-up {CLICK_HOLD_MS} ms later at the empty point {} of the content view, the second pair's mouse-down {CLICK_GAP_MS} ms after the first's at a point the distance along x (distance_x) or y (distance_y, Quartz's downwards) away, or at the same point after a gap (gap_same_point); then the run loop pumped 300 ms and doubleClickInterval + 300 ms, dispatching every event, each left mouse-down's NSEvent clickCount noted in order. drag_start: per trial the source's text set and all selected and the source made first responder; a mouse-down at {} pt from its top left, held, then one mouse-dragged event per point to the right every {DRAG_STEP_MS} ms, a mouse-up {DRAG_RELEASE_AFTER_MS} ms after the last (posted from a second thread while the main thread pumps); the drag pasteboard's (NSPasteboardNameDrag) changeCount before and after: a drag session that starts writes to it; distance_after_hold holds {DRAG_HOLD_MS} ms and moves each distance, hold_before_distance holds each time and moves {DRAG_DISTANCE} pt. tool_tip: per trial the cursor warped (CGWarpMouseCursorPosition) and a mouse-moved posted away from the window, the run loop pumped for the trial's rest, then warped and moved onto the Tip button's centre, and NSApp's windows polled every 5 ms (the run loop pumped between) for a visible one whose class name holds ToolTip, for at most {} ms; the trials cold, warm (right after the cold one's tip) and rested. Per appearance two recordings as the motion pass makes them (screencapture -v of the main display, events posted from a second thread at planned times, main-thread actions, a common-modes timer flipping the ticker every 5 ms): typing-pointer (screencapture -C, the cursor drawn; the target made first responder at 2 s, the cursor moved onto its centre at 2.2 s, a, s, d typed at 3, 3.2 and 3.4 s, the cursor moved 1 pt at 6 s, a and s typed at 7 and 7.2 s, resigned at 9 s; 10 s; each sample reads CGCursorIsVisible; the crop the text view grown by {TYPING_CROP_MARGIN} pt) and drag-over (the source's text selected and the source first responder at 1 s; the cursor moved onto the selection at 1.8 s, a mouse-down at 2 s, dragged 30 pt right from 3 to 3.3 s, onto row 1's centre by 3.8 s, the window captured (screencapture -l, the drag image is a window of its own and not in it) at 4.5 s as drag-over-table-on.png, between rows 2 and 3 by 5.3 s, captured at 6.3 s (drag-over-table-above.png), onto the target text view's centre by 7 s, captured at 8 s (drag-over-text.png), onto the spring-loaded button's centre by 8.6 s, captured at 9 and 11 s (drag-over-spring-early.png, -late.png), to the empty point by 12.9 s, a mouse-up there at 13.5 s; 16 s; the crop the content view), with the drag pasteboard's change count, the table's validation calls and last proposal, the spring-loaded button's state and both texts read after",
        rect_text(NSRect::new(NSPoint::new(0.0, 0.0), WINDOW_FRAME.size)),
        point_text(WINDOW_FRAME.origin),
        rect_text(SOURCE_FRAME),
        point_text(TIP_ORIGIN),
        point_text(SPRING_ORIGIN),
        rect_text(TARGET_FRAME),
        rect_text(TABLE_FRAME),
        rect_text(TICKER_FRAME),
        super::CURSOR_AWAY_OFFSET,
        point_text(EMPTY_POINT),
        point_text(NSPoint::new(PRESS_AT.0, PRESS_AT.1)),
        TIP_WAIT.as_millis(),
    )
}
