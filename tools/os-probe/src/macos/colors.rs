//! The colours pass (`--colors`; the 0.8.0 measurement plan's MP-highlight
//! and MP-colorspace): every NSColor the main pass reads, per appearance,
//! with the highlight and accent user defaults beside them, so a run with
//! `AppleHighlightColor` changed shows what follows it (U163); and Core
//! Animation's colour interpolation (U158): layers whose background colour
//! animates between two colours of different alpha, paused at the start,
//! the middle and the end of the animation, captured over black and over
//! white, with static reference squares of known colours beside them.

use std::path::Path;

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSApplication, NSBox, NSBoxType, NSColor,
    NSTitlePosition, NSView,
};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::CGColor;
use objc2_foundation::{NSPoint, NSRect, NSSize, NSUserDefaults};
use toml::{Table, Value};

use super::{
    KEY_WAIT_STEPS, SETTLE_STEPS, activate_ignoring, colors_table, default_value, ffi, ns,
    point_text, pump, rect, rect_text, screencapture,
};
use crate::out;

/// The user defaults read beside the colours.
const DEFAULTS: [&str; 3] = [
    "AppleHighlightColor",
    "AppleAccentColor",
    "AppleAquaColorVariant",
];

/// The window's content frame on the screen.
const WINDOW_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 300.0), NSSize::new(520.0, 140.0));

/// The squares: their side, the step between two columns, the first
/// column's x and the y of the row over black (the content's top half) and
/// over white (its bottom half), in the content view (bottom-left origin).
const SQUARE: f64 = 20.0;
const COLUMN_STEP: f64 = 30.0;
const FIRST_X: f64 = 20.0;
const ROW_OVER_BLACK_Y: f64 = 95.0;
const ROW_OVER_WHITE_Y: f64 = 25.0;

/// The animation's length in seconds and the fractions of it each case is
/// paused at.
const DURATION: f64 = 1.0;
const FRACTIONS: [f64; 3] = [0.0, 0.5, 1.0];

/// An sRGB colour with alpha, components 0 to 1.
type Rgba = [f64; 4];

/// The animated cases: name, from, to.
const CASES: [(&str, Rgba, Rgba); 4] = [
    ("red_to_blue", [1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]),
    (
        "red_to_blue_alpha_0.2",
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 0.2],
    ),
    ("clear_to_green", [0.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 1.0]),
    ("red_to_clear", [1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0]),
];

/// The static references: name and colour.
const REFERENCES: [(&str, Rgba); 4] = [
    ("purple_0.5_opaque", [0.5, 0.0, 0.5, 1.0]),
    ("grey_0.5_opaque", [0.5, 0.5, 0.5, 1.0]),
    ("red_alpha_0.5", [1.0, 0.0, 0.0, 0.5]),
    ("purple_0.5_alpha_0.6", [0.5, 0.0, 0.5, 0.6]),
];

/// `[macos.colors]`.
pub(super) fn colors_pass_table(
    mtm: MainThreadMarker,
    app: &NSApplication,
    out_dir: &Path,
) -> Table {
    let mut t = Table::new();
    let defaults = NSUserDefaults::standardUserDefaults();
    let mut d = Table::new();
    for key in DEFAULTS {
        let value = match defaults.objectForKey(&ns(key)) {
            None => out::text("absent (objectForKey: is nil)"),
            Some(obj) => default_value(&obj),
        };
        out::put(&mut d, key, value);
    }
    out::put_table(&mut t, "user_defaults", d);
    out::put_table(&mut t, "colors", colors_table(Some(mtm)));
    out::put_table(&mut t, "color_space", color_space_table(mtm, app, out_dir));
    t
}

fn cg(c: Rgba) -> objc2_core_foundation::CFRetained<CGColor> {
    CGColor::new_srgb(c[0], c[1], c[2], c[3])
}

/// A backdrop box filled with `fill`.
fn backdrop(mtm: MainThreadMarker, frame: NSRect, fill: &NSColor) -> Retained<NSBox> {
    let b = NSBox::initWithFrame(NSBox::alloc(mtm), frame);
    b.setBoxType(NSBoxType::Custom);
    b.setTitlePosition(NSTitlePosition::NoTitle);
    b.setBorderWidth(0.0);
    b.setFillColor(fill);
    b
}

fn color_space_table(mtm: MainThreadMarker, app: &NSApplication, out_dir: &Path) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    let window = match ffi::titled_window(mtm, WINDOW_FRAME) {
        Ok(window) => window,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    window.setTitle(&ns(""));
    let Some(content) = window.contentView() else {
        out::put(&mut t, "error", out::unavailable("no content view"));
        return t;
    };
    let size = WINDOW_FRAME.size;
    let half = size.height / 2.0;
    content.addSubview(&backdrop(
        mtm,
        NSRect::new(NSPoint::new(0.0, half), NSSize::new(size.width, half)),
        &NSColor::colorWithSRGBRed_green_blue_alpha(0.0, 0.0, 0.0, 1.0),
    ));
    content.addSubview(&backdrop(
        mtm,
        NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(size.width, half)),
        &NSColor::colorWithSRGBRed_green_blue_alpha(1.0, 1.0, 1.0, 1.0),
    ));
    let host_view = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), size),
    );
    let host = objc2_quartz_core::CALayer::new();
    host_view.setLayer(Some(&host));
    host_view.setWantsLayer(true);
    content.addSubview(&host_view);

    let mut columns: Vec<(String, Rgba, Rgba, f64)> = Vec::new();
    for (name, from, to) in CASES {
        for f in FRACTIONS {
            columns.push((format!("{name} at {f}"), from, to, f));
        }
    }
    for (name, c) in REFERENCES {
        columns.push((format!("reference {name}"), c, c, 0.0));
    }
    let frame = window.frame();
    let title_bar = frame.size.height - size.height;
    let mut squares = Vec::new();
    let mut x = FIRST_X;
    for (name, from, to, f) in &columns {
        let (from_cg, to_cg) = (cg(*from), cg(*to));
        for (row, y) in [
            ("over_black", ROW_OVER_BLACK_Y),
            ("over_white", ROW_OVER_WHITE_Y),
        ] {
            let layer = ffi::paused_color_layer(
                CGRect::new(CGPoint::new(x, y), CGSize::new(SQUARE, SQUARE)),
                &from_cg,
                &to_cg,
                DURATION,
                f * DURATION,
            );
            host.addSublayer(&layer);
            let top = title_bar + size.height - y - SQUARE;
            squares.push(out::text(format!(
                "{name} {row}: from {} to {}, x {x} y {top} w {SQUARE} h {SQUARE}",
                hex(*from),
                hex(*to)
            )));
        }
        x += COLUMN_STEP;
    }
    out::put(&mut t, "squares_from_window_top_left", out::array(squares));

    window.makeKeyAndOrderFront(None);
    activate_ignoring(app);
    let steps = pump(app, KEY_WAIT_STEPS, || window.isKeyWindow());
    out::put(&mut t, "key_wait_steps", Value::Integer(i64::from(steps)));
    out::put(&mut t, "window_frame_xywh", rect(frame));
    let dir = out_dir.join("color_space");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        out::put(
            &mut t,
            "error",
            out::unavailable(format!("cannot create {}: {e}", dir.display())),
        );
        return t;
    }
    for (key, name) in super::APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        window.setAppearance(Some(&appearance));
        window.display();
        pump(app, SETTLE_STEPS, || false);
        let mut s = Table::new();
        let file = format!("{key}.png");
        match screencapture(window.windowNumber(), &dir.join(&file)) {
            Ok(()) => out::put(&mut s, "capture", out::text(format!("color_space/{file}"))),
            Err(reason) => out::put(&mut s, "capture", out::unavailable(reason)),
        }
        out::put(&mut s, "isKeyWindow", out::boolean(window.isKeyWindow()));
        out::put_table(&mut t, key, s);
    }
    window.setAppearance(None);
    window.orderOut(None);
    window.close();
    t
}

/// `#rrggbbaa` of an sRGB colour.
fn hex(c: Rgba) -> String {
    out::rgba_hex(c[0], c[1], c[2], c[3])
}

fn method_text() -> String {
    format!(
        "a titled window (empty title, content {} from {}): its content view's top half an NSBox filled sRGB black, its bottom half one filled sRGB white (custom, no border), and over both an NSView hosting a CALayer (setLayer: then setWantsLayer:); per column, in the hosting layer, one {SQUARE} pt square layer over black (y {ROW_OVER_BLACK_Y}) and one over white (y {ROW_OVER_WHITE_Y}), the columns {COLUMN_STEP} pt apart from x {FIRST_X}: each square a CALayer whose backgroundColor is the case's to colour, its speed 0 and timeOffset the fraction times {DURATION} s, holding a CABasicAnimation of backgroundColor from the case's from colour to its to colour (CGColorCreateSRGB), duration {DURATION} s, the linear timing function, beginTime 1e-6 s (in the layer's local time, which a speed of 0 holds at the time offset), fillMode both, not removed on completion: the layer shows the animation paused at the fraction (less a microsecond); the cases at the fractions {:?}, then the references, static squares (their from and to the same colour); the window made key, the application activated, and per appearance (the window's appearance set, display, the run loop pumped {SETTLE_STEPS} steps) captured with screencapture -x -o -l to color_space/<appearance>.png; squares_from_window_top_left gives each square's rect in the capture's points from the window frame's top left (the title bar included)",
        rect_text(NSRect::new(NSPoint::new(0.0, 0.0), WINDOW_FRAME.size)),
        point_text(WINDOW_FRAME.origin),
        FRACTIONS,
    )
}
