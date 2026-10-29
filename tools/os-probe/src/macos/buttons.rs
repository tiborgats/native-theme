//! The button pass (`--button-pass`): push buttons at every control size,
//! plain, default (key equivalent Return) and toggled, in a titled window
//! made key, captured per appearance in each state with `screencapture`,
//! the window's key status read just before and after each capture. The
//! offscreen renders of the control section draw the not-key look; this
//! pass draws the key-window one.

use std::cell::Cell;
use std::path::{Path, PathBuf};

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSApplication, NSApplicationActivationPolicy,
    NSBezelStyle, NSBitmapImageRep, NSButton, NSButtonType, NSControlSize, NSEvent, NSEventMask,
    NSEventType, NSScreen, NSWindow,
};
use objc2_core_graphics::{
    CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, CGPreflightPostEventAccess,
    CGWarpMouseCursorPosition,
};
use objc2_foundation::{NSData, NSDate, NSOperatingSystemVersion, NSPoint, NSRect, NSSize};
use toml::{Table, Value};

use super::{
    APPEARANCES, KEY_WAIT_STEPS, PUMP_STEP, SETTLE_STEPS, activate_ignoring, activation_table,
    control_common, ffi, insets, ns, pixel_ratio, point_text, pump, rect, screencapture, size,
    state_value,
};
use crate::out;

/// The control sizes the pass places, with the macOS major version each
/// needs: `NSControlSizeLarge` is macOS 11+, `NSControlSizeExtraLarge` 26+.
const BUTTON_SIZES: [(&str, NSControlSize, isize); 5] = [
    ("mini", NSControlSize::Mini, 0),
    ("small", NSControlSize::Small, 0),
    ("regular", NSControlSize::Regular, 0),
    ("large", NSControlSize::Large, 11),
    ("extraLarge", NSControlSize::ExtraLarge, 26),
];

/// Every button's title.
const BUTTON_TITLE: &str = "Button";

/// The window's content frame origin on the screen, in points.
const BUTTON_WINDOW_ORIGIN: NSPoint = NSPoint::new(200.0, 200.0);

/// The space round the grid of buttons in the content view, in points.
const BUTTON_MARGIN: f64 = 20.0;

/// The distance from one column's left edge to the next one's.
const BUTTON_COLUMN_STEP: f64 = 140.0;

/// The height of a row; each button is centred in its row, on whole points.
const BUTTON_ROW_STEP: f64 = 56.0;

/// The window's content size: four columns, five rows.
const BUTTON_CONTENT_SIZE: NSSize = NSSize::new(
    2.0 * BUTTON_MARGIN + 4.0 * BUTTON_COLUMN_STEP,
    2.0 * BUTTON_MARGIN + 5.0 * BUTTON_ROW_STEP,
);

/// How far round a button's frame its pixels are compared between two
/// captures, in points, so a glow or shadow outside the frame counts.
const DIFF_MARGIN: f64 = 6.0;

/// How far right of the window's frame the cursor is parked, in points.
const CURSOR_AWAY_OFFSET: f64 = 100.0;

/// The steps the pump runs after the cursor moves, so a hover the button
/// animates in has settled before the capture.
const HOVER_STEPS: u32 = 20;

/// How long after the mouse-down the click capture is taken.
const CLICK_CAPTURE_DELAY: std::time::Duration = std::time::Duration::from_millis(1000);

/// The most steps the pump waits for a click capture and its mouse-up.
const CLICK_WAIT_STEPS: u32 = 300;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Variant {
    Plain,
    Default,
    ToggleOff,
    ToggleOn,
}

impl Variant {
    const ALL: [Variant; 4] = [
        Variant::Plain,
        Variant::Default,
        Variant::ToggleOff,
        Variant::ToggleOn,
    ];

    fn name(self) -> &'static str {
        match self {
            Variant::Plain => "plain",
            Variant::Default => "default",
            Variant::ToggleOff => "toggle_off",
            Variant::ToggleOn => "toggle_on",
        }
    }

    fn setup(self) -> &'static str {
        match self {
            Variant::Plain => "buttonType momentaryPushIn",
            Variant::Default => "buttonType momentaryPushIn, keyEquivalent \"\\r\" (Return)",
            Variant::ToggleOff => "buttonType pushOnPushOff, state off",
            Variant::ToggleOn => "buttonType pushOnPushOff, state on",
        }
    }

    /// Whether the pass clicks it: a click flips a toggle's state.
    fn clicked(self) -> bool {
        matches!(self, Variant::Plain | Variant::Default)
    }
}

/// One button of the grid.
struct Case {
    name: String,
    size: NSControlSize,
    variant: Variant,
    button: Retained<NSButton>,
}

/// What every capture of the pass shares.
struct Pass<'a> {
    mtm: MainThreadMarker,
    app: &'a NSApplication,
    window: &'a NSWindow,
    cases: &'a [Case],
    out_dir: &'a Path,
    /// The window frame's size, in points.
    frame: NSSize,
    /// The primary screen's height, which turns AppKit's screen points
    /// (bottom-left origin) into Quartz's global ones (top-left origin).
    primary_height: f64,
    /// Where the cursor is parked, in AppKit's screen points.
    away: NSPoint,
    /// Whether every capture so far was taken while the window was key.
    all_key: Cell<bool>,
}

/// `--button-pass`: activate the application, make a titled window holding
/// the grid of push buttons key, and capture it per appearance and state.
pub(super) fn button_pass_table(
    mtm: MainThreadMarker,
    app: &NSApplication,
    out_dir: &Path,
    version: &NSOperatingSystemVersion,
) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    let mut states = Table::new();
    for (state, how) in [
        (
            "normal",
            "nothing set; the cursor parked outside the window",
        ),
        (
            "pressed",
            "-[NSButton highlight:] YES on every button, as NSButtonCell's mouse tracking does, then the run loop pumped; the cells' isHighlighted read back (cells_highlighted)",
        ),
        ("disabled", "setEnabled: NO on every button"),
        (
            "hovered",
            "per button: the cursor warped to the button's centre (CGWarpMouseCursorPosition) and a mouse-moved CGEvent posted there at the HID tap, then the run loop pumped; the mouse events AppKit dispatched meanwhile are counted (cursor.events)",
        ),
        (
            "clicked",
            "per plain and default button at the regular size: the cursor moved there as for hovered, a left mouse-down CGEvent posted at the HID tap, and from a second thread the window captured after the delay and a mouse-up posted after the capture, while the main thread's run loop runs NSButtonCell's tracking; the window's key status is read before the mouse-down and after the mouse-up",
        ),
    ] {
        out::put(&mut states, state, out::text(how));
    }
    out::put_table(&mut t, "states", states);

    out::put(
        &mut t,
        "activationPolicy_regular_set",
        out::boolean(app.setActivationPolicy(NSApplicationActivationPolicy::Regular)),
    );
    app.finishLaunching();
    out::put_table(&mut t, "activation", activation_table(app));
    out::put(
        &mut t,
        "CGPreflightPostEventAccess",
        out::boolean(CGPreflightPostEventAccess()),
    );

    let window =
        match ffi::titled_window(mtm, NSRect::new(BUTTON_WINDOW_ORIGIN, BUTTON_CONTENT_SIZE)) {
            Ok(window) => window,
            Err(reason) => {
                out::put(&mut t, "error", out::unavailable(reason));
                return t;
            }
        };
    window.setTitle(&ns(""));
    let Some(content) = window.contentView() else {
        out::put(
            &mut t,
            "error",
            out::unavailable("the titled window has no content view"),
        );
        return t;
    };

    let mut cases = Vec::new();
    let mut buttons = Table::new();
    let mut row = 0.0_f64;
    for (size_name, control_size, needs) in BUTTON_SIZES {
        if version.majorVersion < needs {
            out::put(
                &mut buttons,
                size_name,
                out::unavailable(format!("{size_name} needs macOS {needs}")),
            );
            row += 1.0;
            continue;
        }
        let mut column = 0.0_f64;
        for variant in Variant::ALL {
            let b = NSButton::new(mtm);
            match variant {
                Variant::ToggleOff | Variant::ToggleOn => {
                    b.setButtonType(NSButtonType::PushOnPushOff);
                    b.setState(state_value(variant == Variant::ToggleOn));
                }
                Variant::Plain | Variant::Default => {
                    b.setButtonType(NSButtonType::MomentaryPushIn);
                }
            }
            if variant == Variant::Default {
                b.setKeyEquivalent(&ns("\r"));
            }
            b.setBezelStyle(NSBezelStyle::Push);
            b.setTitle(&ns(BUTTON_TITLE));
            control_common(&b, control_size);
            b.sizeToFit();
            let fitted = b.frame().size;
            let row_bottom =
                BUTTON_CONTENT_SIZE.height - BUTTON_MARGIN - (row + 1.0) * BUTTON_ROW_STEP;
            b.setFrameOrigin(NSPoint::new(
                BUTTON_MARGIN + column * BUTTON_COLUMN_STEP,
                (row_bottom + (BUTTON_ROW_STEP - fitted.height) / 2.0).floor(),
            ));
            content.addSubview(&b);
            cases.push(Case {
                name: format!("{size_name}-{}", variant.name()),
                size: control_size,
                variant,
                button: b,
            });
            column += 1.0;
        }
        row += 1.0;
    }

    window.makeKeyAndOrderFront(None);
    out::put(
        &mut t,
        "makeFirstResponder_nil",
        out::boolean(window.makeFirstResponder(None)),
    );
    let steps = pump(app, KEY_WAIT_STEPS, || {
        if !app.isActive() {
            activate_ignoring(app);
        }
        window.isKeyWindow()
    });
    out::put(&mut t, "isKeyWindow", out::boolean(window.isKeyWindow()));
    out::put(&mut t, "isMainWindow", out::boolean(window.isMainWindow()));
    out::put(&mut t, "NSApp_isActive", out::boolean(app.isActive()));
    out::put(&mut t, "key_wait_steps", out::int(i64::from(steps)));
    out::put(
        &mut t,
        "window_backingScaleFactor",
        out::float(window.backingScaleFactor()),
    );
    let window_frame = window.frame();
    out::put(&mut t, "window_frame_xywh", rect(window_frame));
    out::put(
        &mut t,
        "defaultButtonCell",
        default_cell_name(&window, &cases),
    );

    for case in &cases {
        out::put_table(
            &mut buttons,
            &case.name,
            case_table(case, window_frame.size),
        );
    }
    out::put_table(&mut t, "buttons", buttons);

    let primary_height = NSScreen::screens(mtm)
        .iter()
        .next()
        .map(|screen| screen.frame().size.height);
    let Some(primary_height) = primary_height else {
        out::put(
            &mut t,
            "error",
            out::unavailable("NSScreen screens is empty"),
        );
        return t;
    };
    let away = NSPoint::new(
        window_frame.origin.x + window_frame.size.width + CURSOR_AWAY_OFFSET,
        window_frame.origin.y + window_frame.size.height / 2.0,
    );
    out::put(
        &mut t,
        "cursor_away_screen_xy",
        out::floats(&[away.x, away.y]),
    );
    let pass = Pass {
        mtm,
        app,
        window: &window,
        cases: &cases,
        out_dir,
        frame: window_frame.size,
        primary_height,
        away,
        all_key: Cell::new(true),
    };
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
        content.setNeedsDisplay(true);
        window.displayIfNeeded();
        pump(app, SETTLE_STEPS, || false);
        out::put_table(&mut t, key, pass.appearance(key));
    }
    out::put(&mut t, "all_captures_key", out::boolean(pass.all_key.get()));
    window.setAppearance(None);
    window.orderOut(None);
    window.close();
    t
}

fn method_text() -> String {
    format!(
        "activation policy set to Regular, finishLaunching and activation as in the key-window section; a titled NSWindow ({} x {} pt content from {}, empty title) holding a grid of NSButtons, bezelStyle push, title \"{BUTTON_TITLE}\", each given setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize: and sized to fit: a row per control size (mini, small, regular, large, extraLarge where the OS has it) from the top, {BUTTON_ROW_STEP} pt apart, each button centred in its row on whole points, and a column per variant (plain, default, toggle_off, toggle_on) every {BUTTON_COLUMN_STEP} pt from x {BUTTON_MARGIN}; made key with makeKeyAndOrderFront: and makeFirstResponder: nil, the run loop pumped until isKeyWindow; per appearance the window's appearance is set and the run loop pumped {SETTLE_STEPS} steps of {} ms, then each state is set (states), the window displayed and the run loop pumped {SETTLE_STEPS} steps ({HOVER_STEPS} after a cursor move), and the window captured with screencapture -x -o -l <windowNumber> to <appearance>/<state>[-<button>].png; isKeyWindow and NSApp isActive are read just before and just after each capture (key_at_capture: both reads of isKeyWindow are true); each capture is compared with the same appearance's normal capture inside each button's frame grown by {DIFF_MARGIN} pt (diff_from_normal: the pixels whose bytes differ and the largest difference of one byte); the cursor is parked {CURSOR_AWAY_OFFSET} pt right of the window's frame before the normal capture and after the hovered and clicked ones",
        BUTTON_CONTENT_SIZE.width,
        BUTTON_CONTENT_SIZE.height,
        point_text(BUTTON_WINDOW_ORIGIN),
        PUMP_STEP.as_millis(),
    )
}

/// Which button's cell the window holds as its default button cell.
fn default_cell_name(window: &NSWindow, cases: &[Case]) -> Value {
    let Some(cell) = window.defaultButtonCell() else {
        return out::text("nil");
    };
    let cell: &AnyObject = cell.as_ref();
    for case in cases {
        if let Some(own) = case.button.cell() {
            let own: &AnyObject = own.as_ref();
            if std::ptr::eq(own, cell) {
                return out::text(case.name.clone());
            }
        }
    }
    out::text("a cell of none of the pass's buttons")
}

/// A button's setup and geometry; its frame in the window frame's points
/// from the top left, as the captures lay it out.
fn case_table(case: &Case, frame: NSSize) -> Table {
    let b = &case.button;
    let mut t = Table::new();
    out::put(&mut t, "variant", out::text(case.variant.name()));
    out::put(&mut t, "setup", out::text(case.variant.setup()));
    let raw =
        |s: NSControlSize| i64::try_from(s.0).map_or_else(|_| out::text(s.0.to_string()), out::int);
    out::put(&mut t, "controlSize_requested_raw", raw(case.size));
    out::put(&mut t, "controlSize_raw", raw(b.controlSize()));
    out::put(
        &mut t,
        "keyEquivalent",
        out::text(b.keyEquivalent().to_string()),
    );
    out::put(&mut t, "state", out::int_isize(b.state()));
    out::put(&mut t, "frame_xywh", rect(b.frame()));
    out::put(
        &mut t,
        "intrinsicContentSize_wh",
        size(b.intrinsicContentSize()),
    );
    out::put(
        &mut t,
        "alignmentRectInsets_tlbr",
        insets(b.alignmentRectInsets()),
    );
    out::put(
        &mut t,
        "in_window_frame_from_top_left_xywh",
        rect(from_top_left(b, frame)),
    );
    t
}

/// The button's frame in the window frame's points from its top left.
fn from_top_left(button: &NSButton, frame: NSSize) -> NSRect {
    let r = button.convertRect_toView(button.bounds(), None);
    NSRect::new(
        NSPoint::new(r.origin.x, frame.height - r.origin.y - r.size.height),
        r.size,
    )
}

impl Pass<'_> {
    /// One appearance's captures.
    fn appearance(&self, key: &str) -> Table {
        let mut t = Table::new();
        let dir = self.out_dir.join(key);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut t,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            return t;
        }
        let all: Vec<&Case> = self.cases.iter().collect();

        let mut normal = Table::new();
        out::put_table(&mut normal, "cursor", self.move_cursor(self.away));
        let normal_rep = self.shot(&mut normal, key, "normal.png");
        out::put_table(&mut t, "normal", normal);

        let mut pressed = Table::new();
        for case in self.cases {
            case.button.highlight(true);
        }
        self.settle(SETTLE_STEPS);
        let highlighted = self
            .cases
            .iter()
            .filter(|c| c.button.cell().is_some_and(|cell| cell.isHighlighted()))
            .map(|c| out::text(c.name.clone()))
            .collect();
        out::put(&mut pressed, "cells_highlighted", out::array(highlighted));
        let pressed_rep = self.shot(&mut pressed, key, "pressed.png");
        self.diffs(
            &mut pressed,
            "diff_from_normal",
            normal_rep.as_deref(),
            pressed_rep.as_deref(),
            &all,
        );
        for case in self.cases {
            case.button.highlight(false);
        }
        out::put_table(&mut t, "pressed", pressed);

        let mut disabled = Table::new();
        for case in self.cases {
            case.button.setEnabled(false);
        }
        self.settle(SETTLE_STEPS);
        let disabled_rep = self.shot(&mut disabled, key, "disabled.png");
        self.diffs(
            &mut disabled,
            "diff_from_normal",
            normal_rep.as_deref(),
            disabled_rep.as_deref(),
            &all,
        );
        for case in self.cases {
            case.button.setEnabled(true);
        }
        self.settle(SETTLE_STEPS);
        out::put_table(&mut t, "disabled", disabled);

        let mut hovered = Table::new();
        for case in self.cases {
            let mut h = Table::new();
            out::put_table(&mut h, "cursor", self.move_cursor(self.centre(case)));
            let rep = self.shot(&mut h, key, &format!("hovered-{}.png", case.name));
            self.diffs(
                &mut h,
                "diff_from_normal",
                normal_rep.as_deref(),
                rep.as_deref(),
                &[case],
            );
            out::put_table(&mut hovered, &case.name, h);
        }
        out::put_table(&mut hovered, "cursor_parked", self.move_cursor(self.away));
        out::put_table(&mut t, "hovered", hovered);

        let mut clicked = Table::new();
        for case in self
            .cases
            .iter()
            .filter(|c| c.size == NSControlSize::Regular && c.variant.clicked())
        {
            let mut c = Table::new();
            out::put_table(&mut c, "cursor", self.move_cursor(self.centre(case)));
            let rep = self.click(&mut c, key, case);
            self.diffs(
                &mut c,
                "diff_from_normal",
                normal_rep.as_deref(),
                rep.as_deref(),
                &[case],
            );
            self.diffs(
                &mut c,
                "diff_from_pressed",
                pressed_rep.as_deref(),
                rep.as_deref(),
                &[case],
            );
            out::put_table(&mut clicked, &case.name, c);
        }
        out::put_table(&mut clicked, "cursor_parked", self.move_cursor(self.away));
        out::put_table(&mut t, "clicked", clicked);
        t
    }

    /// Display the window and pump the run loop `steps` steps.
    fn settle(&self, steps: u32) {
        self.window.displayIfNeeded();
        pump(self.app, steps, || false);
    }

    /// The centre of a button in AppKit's screen points.
    fn centre(&self, case: &Case) -> NSPoint {
        let b = &case.button;
        let r = self
            .window
            .convertRectToScreen(b.convertRect_toView(b.bounds(), None));
        NSPoint::new(
            r.origin.x + r.size.width / 2.0,
            r.origin.y + r.size.height / 2.0,
        )
    }

    /// `p`, in AppKit's screen points, in Quartz's global display points.
    fn quartz(&self, p: NSPoint) -> NSPoint {
        NSPoint::new(p.x, self.primary_height - p.y)
    }

    /// Warp the cursor to `target` (AppKit's screen points), post a
    /// mouse-moved event there and pump the run loop `HOVER_STEPS` steps,
    /// counting the mouse events AppKit dispatches; where the cursor then is.
    fn move_cursor(&self, target: NSPoint) -> Table {
        let mut t = Table::new();
        let at = self.quartz(target);
        out::put(
            &mut t,
            "target_screen_xy",
            out::floats(&[target.x, target.y]),
        );
        out::put(&mut t, "target_quartz_xy", out::floats(&[at.x, at.y]));
        let warp = CGWarpMouseCursorPosition(at);
        out::put(
            &mut t,
            "CGWarpMouseCursorPosition",
            out::int(i64::from(warp.0)),
        );
        let moved =
            CGEvent::new_mouse_event(None, CGEventType::MouseMoved, at, CGMouseButton::Left);
        out::put(&mut t, "mouseMoved_created", out::boolean(moved.is_some()));
        CGEvent::post(CGEventTapLocation::HIDEventTap, moved.as_deref());
        self.window.displayIfNeeded();
        out::put_table(&mut t, "events", pump_counting(self.app, HOVER_STEPS));
        let location = NSEvent::mouseLocation();
        out::put(
            &mut t,
            "mouseLocation_screen_xy",
            out::floats(&[location.x, location.y]),
        );
        let under =
            NSWindow::windowNumberAtPoint_belowWindowWithWindowNumber(location, 0, self.mtm);
        out::put(
            &mut t,
            "window_under_cursor_is_ours",
            out::boolean(under == self.window.windowNumber()),
        );
        t
    }

    /// Capture the window to `<key>/<file>`, reading its key status just
    /// before and after; the capture, decoded, for the comparisons.
    fn shot(&self, t: &mut Table, key: &str, file: &str) -> Option<Retained<NSBitmapImageRep>> {
        let before = (self.window.isKeyWindow(), self.app.isActive());
        let path = self.out_dir.join(key).join(file);
        let captured = screencapture(self.window.windowNumber(), &path);
        let after = (self.window.isKeyWindow(), self.app.isActive());
        self.record_key(t, before, after);
        read_capture(t, captured, &path, &format!("{key}/{file}"))
    }

    /// Record the window's key status and the application's activity read
    /// before and after a capture.
    fn record_key(&self, t: &mut Table, before: (bool, bool), after: (bool, bool)) {
        out::put(t, "isKeyWindow_before", out::boolean(before.0));
        out::put(t, "isKeyWindow_after", out::boolean(after.0));
        out::put(t, "NSApp_isActive_before", out::boolean(before.1));
        out::put(t, "NSApp_isActive_after", out::boolean(after.1));
        let key = before.0 && after.0;
        out::put(t, "key_at_capture", out::boolean(key));
        self.all_key.set(self.all_key.get() && key);
    }

    /// Press `case` with a real mouse-down and capture the window from a
    /// second thread while NSButtonCell tracks the mouse on this one; the
    /// second thread posts the mouse-up once the capture is done.
    fn click(&self, t: &mut Table, key: &str, case: &Case) -> Option<Retained<NSBitmapImageRep>> {
        let at = self.quartz(self.centre(case));
        let file = format!("clicked-{}.png", case.name);
        let path: PathBuf = self.out_dir.join(key).join(&file);
        let Some(down) =
            CGEvent::new_mouse_event(None, CGEventType::LeftMouseDown, at, CGMouseButton::Left)
        else {
            out::put(
                t,
                "capture",
                out::unavailable("the mouse-down CGEvent is NULL"),
            );
            return None;
        };
        let before = (self.window.isKeyWindow(), self.app.isActive());
        let number = self.window.windowNumber();
        let worker_path = path.clone();
        let worker = std::thread::spawn(move || {
            std::thread::sleep(CLICK_CAPTURE_DELAY);
            let captured = screencapture(number, &worker_path);
            let up =
                CGEvent::new_mouse_event(None, CGEventType::LeftMouseUp, at, CGMouseButton::Left);
            let created = up.is_some();
            CGEvent::post(CGEventTapLocation::HIDEventTap, up.as_deref());
            (captured, created)
        });
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&*down));
        let steps = pump(self.app, CLICK_WAIT_STEPS, || worker.is_finished());
        out::put(t, "wait_steps", out::int(i64::from(steps)));
        let (captured, up_created) = worker
            .join()
            .unwrap_or_else(|_| (Err("the capture thread panicked".to_owned()), false));
        out::put(t, "mouseUp_created", out::boolean(up_created));
        // Deliver the mouse-up, so the tracking ends before the next state.
        self.settle(SETTLE_STEPS);
        let after = (self.window.isKeyWindow(), self.app.isActive());
        self.record_key(t, before, after);
        out::put(
            t,
            "key_status_read",
            out::text(
                "before the mouse-down and after the mouse-up: the main thread is in the button's mouse tracking while the capture is taken",
            ),
        );
        out::put(
            t,
            "highlighted_after",
            out::boolean(case.button.cell().is_some_and(|cell| cell.isHighlighted())),
        );
        read_capture(t, captured, &path, &format!("{key}/{file}"))
    }

    /// Compare `rep` with `base` inside each of `cases`' frames grown by
    /// `DIFF_MARGIN`, into `t[name]`.
    fn diffs(
        &self,
        t: &mut Table,
        name: &str,
        base: Option<&NSBitmapImageRep>,
        rep: Option<&NSBitmapImageRep>,
        cases: &[&Case],
    ) {
        let (Some(base), Some(rep)) = (base, rep) else {
            out::put(t, name, out::unavailable("a capture is missing"));
            return;
        };
        let mut d = Table::new();
        for case in cases {
            let frame = from_top_left(&case.button, self.frame);
            out::put(&mut d, &case.name, diff_table(base, rep, frame, self.frame));
        }
        out::put_table(t, name, d);
    }
}

/// Record a capture's file and size, and decode it.
fn read_capture(
    t: &mut Table,
    captured: Result<(), String>,
    path: &Path,
    file: &str,
) -> Option<Retained<NSBitmapImageRep>> {
    if let Err(reason) = captured {
        out::put(t, "capture", out::unavailable(reason));
        return None;
    }
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            out::put(
                t,
                "capture",
                out::unavailable(format!("cannot read {}: {e}", path.display())),
            );
            return None;
        }
    };
    out::put(t, "file", out::text(file));
    let Some(rep) = NSBitmapImageRep::imageRepWithData(&NSData::with_bytes(&bytes)) else {
        out::put(
            t,
            "capture",
            out::unavailable("the capture is not a bitmap image"),
        );
        return None;
    };
    out::put(
        t,
        "pixels_wh",
        out::array(vec![
            out::int_isize(rep.pixelsWide()),
            out::int_isize(rep.pixelsHigh()),
        ]),
    );
    Some(rep)
}

/// The pixels of `frame` (window points from the top left, grown by
/// `DIFF_MARGIN`) whose bytes differ between `a` and `b`.
fn diff_table(a: &NSBitmapImageRep, b: &NSBitmapImageRep, frame: NSRect, window: NSSize) -> Value {
    let (wide, high) = (b.pixelsWide(), b.pixelsHigh());
    if (a.pixelsWide(), a.pixelsHigh()) != (wide, high) {
        return out::unavailable("the captures differ in size");
    }
    let Some(ratio) = pixel_ratio(wide, window.width) else {
        return out::unavailable("no pixel ratio");
    };
    let (Some((x0, x1)), Some((y0, y1))) = (
        span(frame.origin.x, frame.size.width, ratio, wide),
        span(frame.origin.y, frame.size.height, ratio, high),
    ) else {
        return out::unavailable("the frame lies outside the capture");
    };
    let mut differing: i64 = 0;
    let mut max_delta: u8 = 0;
    for y in y0..y1 {
        for x in x0..x1 {
            let (pa, pb) = match (ffi::pixel_bytes(a, x, y), ffi::pixel_bytes(b, x, y)) {
                (Ok(pa), Ok(pb)) => (pa, pb),
                (Err(e), _) | (_, Err(e)) => return out::unavailable(e),
            };
            let delta = pa
                .iter()
                .zip(pb.iter())
                .fold(0_u8, |m, (p, q)| m.max(p.abs_diff(*q)));
            if delta > 0 {
                differing = differing.saturating_add(1);
                max_delta = max_delta.max(delta);
            }
        }
    }
    let mut t = Table::new();
    let (w, h) = (x1.saturating_sub(x0), y1.saturating_sub(y0));
    out::put(
        &mut t,
        "rect_px_xywh",
        out::array(vec![
            out::int_isize(x0),
            out::int_isize(y0),
            out::int_isize(w),
            out::int_isize(h),
        ]),
    );
    out::put(&mut t, "pixels", out::int_isize(w.saturating_mul(h)));
    out::put(&mut t, "differing_pixels", out::int(differing));
    out::put(&mut t, "max_byte_delta", out::int(i64::from(max_delta)));
    Value::Table(t)
}

/// The pixel range of `start..start + length` points grown by `DIFF_MARGIN`
/// at `ratio` pixels per point, clamped to `0..limit`; `None` where empty.
fn span(start: f64, length: f64, ratio: f64, limit: isize) -> Option<(isize, isize)> {
    #[allow(clippy::cast_precision_loss)]
    let limit_f = limit as f64;
    let lo = ((start - DIFF_MARGIN) * ratio).floor().clamp(0.0, limit_f);
    let hi = ((start + length + DIFF_MARGIN) * ratio)
        .ceil()
        .clamp(0.0, limit_f);
    if !(lo.is_finite() && hi.is_finite()) || hi <= lo {
        return None;
    }
    // Both in 0.0..=limit after the clamp, so the casts are exact.
    #[allow(clippy::cast_possible_truncation)]
    let range = (lo as isize, hi as isize);
    Some(range)
}

/// Pump the run loop `steps` steps of `PUMP_STEP`, as `pump` does, counting
/// the mouse events it dispatches.
fn pump_counting(app: &NSApplication, steps: u32) -> Table {
    let mode = ffi::default_run_loop_mode();
    let counted = [
        ("mouseMoved", NSEventType::MouseMoved),
        ("mouseEntered", NSEventType::MouseEntered),
        ("mouseExited", NSEventType::MouseExited),
        ("cursorUpdate", NSEventType::CursorUpdate),
    ];
    let mut counts = [0_i64; 4];
    let mut other: i64 = 0;
    for _ in 0..steps {
        let until = NSDate::dateWithTimeIntervalSinceNow(PUMP_STEP.as_secs_f64());
        while let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
            NSEventMask::Any,
            Some(&until),
            mode,
            true,
        ) {
            let kind = event.r#type();
            match counted
                .iter()
                .position(|(_, k)| *k == kind)
                .and_then(|i| counts.get_mut(i))
            {
                Some(n) => *n = n.saturating_add(1),
                None => other = other.saturating_add(1),
            }
            app.sendEvent(&event);
        }
        app.updateWindows();
    }
    let mut t = Table::new();
    for ((name, _), n) in counted.iter().zip(counts) {
        out::put(&mut t, name, out::int(n));
    }
    out::put(&mut t, "other", out::int(other));
    t
}
