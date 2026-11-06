//! The reads pass (`--reads`): what AppKit and the frameworks it loads
//! report without a control pass of their own. Core Animation's named media
//! timing functions and their control points, and `NSAnimationContext`'s
//! duration and timing function, outside and inside a grouping; the output
//! route button (`AVRoutePickerView`): its colour per button state and
//! appearance, bordered and not, and its renders; and the focus ring AppKit
//! draws round a plain view whose focus ring mask is its bounds, captured
//! focused and not per appearance in a key window.

use std::cell::RefCell;
use std::path::Path;

use block2::RcBlock;
use objc2::MainThreadMarker;
use objc2_app_kit::{
    NSAnimationContext, NSAppearance, NSAppearanceCustomization, NSApplication,
    NSApplicationActivationPolicy, NSColorSpace, NSResponder, NSView,
};
use objc2_foundation::{NSPoint, NSRect, NSSize};
use toml::{Table, Value};

use super::{
    APPEARANCES, KEY_WAIT_STEPS, SETTLE_STEPS, activate_ignoring, bitmap_at_scale, color_value,
    ffi, ns, pump, rect, screencapture, size,
};
use crate::out;

/// The media timing function names QuartzCore exports
/// (`CAMediaTimingFunction.h:54-62`).
const TIMING_NAMES: [&str; 5] = [
    "kCAMediaTimingFunctionLinear",
    "kCAMediaTimingFunctionEaseIn",
    "kCAMediaTimingFunctionEaseOut",
    "kCAMediaTimingFunctionEaseInEaseOut",
    "kCAMediaTimingFunctionDefault",
];

/// The route picker's button states (`AVRoutePickerView.h:62-67`).
const ROUTE_STATES: [(&str, isize); 4] = [
    ("normal", 0),
    ("normalHighlighted", 1),
    ("active", 2),
    ("activeHighlighted", 3),
];

/// The route picker's frame in the offscreen window, in points.
const ROUTE_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 20.0), NSSize::new(40.0, 40.0));

/// The offscreen window's content frame.
const OFFSCREEN_FRAME: NSRect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(200.0, 120.0));

/// The render scales.
const RENDER_SCALES: [(&str, f64); 2] = [("1x", 1.0), ("2x", 2.0)];

/// The focus window's content frame on the screen.
const FOCUS_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 300.0), NSSize::new(240.0, 120.0));

/// The plain focusable view's frame in the focus window's content view.
const FOCUS_VIEW_FRAME: NSRect = NSRect::new(NSPoint::new(40.0, 40.0), NSSize::new(120.0, 40.0));

/// Fill `[macos.reads]`.
pub(super) fn reads_table(mtm: MainThreadMarker, app: &NSApplication, out_dir: &Path) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "activationPolicy_regular_set",
        out::boolean(app.setActivationPolicy(NSApplicationActivationPolicy::Regular)),
    );
    app.finishLaunching();
    activate_ignoring(app);
    out::put_table(&mut t, "timing", timing_table());
    out::put_table(&mut t, "route_picker", route_picker_table(mtm, out_dir));
    out::put_table(&mut t, "focus_view", focus_view_table(mtm, app, out_dir));
    t
}

/// The named media timing functions and the animation context's defaults.
fn timing_table() -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(
            "per name QuartzCore exports (CAMediaTimingFunction.h:54-62): the constant's text, looked up with CFBundleGetDataPointerForName in com.apple.QuartzCore; +[CAMediaTimingFunction functionWithName:] of it; getControlPointAtIndex:values: at 0, 1, 2 and 3, each [x, y]; animation_context: +[NSAnimationContext currentContext] outside any grouping and inside +beginGrouping (then +endGrouping): duration, allowsImplicitAnimation, timingFunction (nil, or its control points)",
        ),
    );
    let mut functions = Table::new();
    for name in TIMING_NAMES {
        let mut f = Table::new();
        match ffi::exported_string("com.apple.QuartzCore", name) {
            Ok(text) => {
                out::put(&mut f, "constant", out::text(text.clone()));
                match ffi::timing_function(&ns(&text)) {
                    Ok(function) => out::put(&mut f, "control_points", points_value(&function)),
                    Err(reason) => out::put(&mut f, "control_points", out::unavailable(reason)),
                }
            }
            Err(reason) => out::put(&mut f, "constant", out::unavailable(reason)),
        }
        out::put_table(&mut functions, name, f);
    }
    out::put_table(&mut t, "functions", functions);

    let mut context = Table::new();
    out::put_table(&mut context, "outside_grouping", context_table());
    NSAnimationContext::beginGrouping();
    out::put_table(&mut context, "inside_grouping", context_table());
    NSAnimationContext::endGrouping();
    out::put_table(&mut t, "animation_context", context);
    t
}

/// The current animation context's duration and timing function.
fn context_table() -> Table {
    let mut t = Table::new();
    let context = NSAnimationContext::currentContext();
    out::put(&mut t, "duration", out::float(context.duration()));
    out::put(
        &mut t,
        "allowsImplicitAnimation",
        out::boolean(context.allowsImplicitAnimation()),
    );
    match ffi::animation_timing_function(&context) {
        Some(function) => out::put(&mut t, "timingFunction", points_value(&function)),
        None => out::put(&mut t, "timingFunction", out::text("nil")),
    }
    t
}

/// A timing function's control points, `[[x, y]; 4]`.
fn points_value(function: &objc2::runtime::AnyObject) -> Value {
    match ffi::control_points(function) {
        Ok(points) => out::array(
            points
                .iter()
                .map(|[x, y]| out::floats(&[f64::from(*x), f64::from(*y)]))
                .collect(),
        ),
        Err(reason) => out::unavailable(reason),
    }
}

/// The output route button: colours per state and appearance, bordered and
/// not, and its renders.
fn route_picker_table(mtm: MainThreadMarker, out_dir: &Path) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "AVKit loaded with NSBundle bundleWithPath: /System/Library/Frameworks/AVKit.framework load; an AVRoutePickerView (new) at {} in an offscreen borderless window; per appearance (the window's appearance set, read inside performAsCurrentDrawingAppearance:) and per bordered (setRoutePickerButtonBordered:, the made view's isRoutePickerButtonBordered recorded first): routePickerButtonColorForState: of each AVRoutePickerViewButtonState in sRGB, intrinsicContentSize, fittingSize, the subviews ([depth, class, frame_xywh in the picker]), and the view rendered with cacheDisplayInRect:toBitmapImageRep: at 1 and 2 pixels a point to route/<appearance>-<bordered|borderless>-<scale>.png (8-bit RGBA, transparent where nothing draws); the button's own state is the view's, which the probe does not change",
            super::rect_text(ROUTE_FRAME)
        )),
    );
    let picker = match ffi::route_picker_view() {
        Ok(picker) => picker,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    out::put(
        &mut t,
        "class",
        out::text(picker.class().name().to_string_lossy()),
    );
    out::put(
        &mut t,
        "isRoutePickerButtonBordered_as_made",
        out::boolean(ffi::route_picker_bordered(&picker)),
    );
    let window = match ffi::offscreen_window(mtm, OFFSCREEN_FRAME) {
        Ok(window) => window,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let Some(content) = window.contentView() else {
        out::put(&mut t, "error", out::unavailable("no content view"));
        return t;
    };
    picker.setFrame(ROUTE_FRAME);
    content.addSubview(&picker);
    let dir = out_dir.join("route");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        out::put(
            &mut t,
            "error",
            out::unavailable(format!("cannot create {}: {e}", dir.display())),
        );
        return t;
    }
    let srgb = NSColorSpace::sRGBColorSpace();
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
        let result = RefCell::new(Table::new());
        {
            let block = RcBlock::new(|| {
                let mut a = Table::new();
                for (bordered_key, bordered) in [("bordered", true), ("borderless", false)] {
                    ffi::set_route_picker_bordered(&picker, bordered);
                    picker.layoutSubtreeIfNeeded();
                    let mut b = Table::new();
                    out::put(
                        &mut b,
                        "isRoutePickerButtonBordered",
                        out::boolean(ffi::route_picker_bordered(&picker)),
                    );
                    let mut colors = Table::new();
                    for (state_name, state) in ROUTE_STATES {
                        let value = match ffi::route_picker_color(&picker, state) {
                            Some(color) => color_value(&color, &srgb),
                            None => out::text("nil"),
                        };
                        out::put(&mut colors, state_name, value);
                    }
                    out::put_table(&mut b, "routePickerButtonColorForState", colors);
                    out::put(
                        &mut b,
                        "intrinsicContentSize_wh",
                        size(picker.intrinsicContentSize()),
                    );
                    out::put(&mut b, "fittingSize_wh", size(picker.fittingSize()));
                    out::put(&mut b, "subviews", subviews_value(&picker));
                    let mut renders = Table::new();
                    for (scale_name, scale) in RENDER_SCALES {
                        let file = format!("{key}-{bordered_key}-{scale_name}.png");
                        out::put(
                            &mut renders,
                            scale_name,
                            render_value(
                                &picker,
                                scale,
                                &dir.join(&file),
                                &format!("route/{file}"),
                            ),
                        );
                    }
                    out::put_table(&mut b, "renders", renders);
                    out::put_table(&mut a, bordered_key, b);
                }
                *result.borrow_mut() = a;
            });
            appearance.performAsCurrentDrawingAppearance(&block);
        }
        out::put_table(&mut t, key, result.into_inner());
    }
    picker.removeFromSuperview();
    window.setAppearance(None);
    t
}

/// The subviews of `view`, three deep: `[depth, class, frame_xywh]`, each
/// frame in `view`'s coordinates.
fn subviews_value(view: &NSView) -> Value {
    let mut rows = Vec::new();
    collect_subviews(view, view, 1, &mut rows);
    out::array(rows)
}

fn collect_subviews(root: &NSView, view: &NSView, depth: i64, rows: &mut Vec<Value>) {
    if depth > 3 {
        return;
    }
    for sub in view.subviews().iter() {
        rows.push(out::array(vec![
            Value::Integer(depth),
            out::text(sub.class().name().to_string_lossy()),
            rect(sub.convertRect_toView(sub.bounds(), Some(root))),
        ]));
        collect_subviews(root, &sub, depth.saturating_add(1), rows);
    }
}

/// `view` rendered at `scale` to `path`; the file as `rel` and its size.
fn render_value(view: &NSView, scale: f64, path: &Path, rel: &str) -> Value {
    let bounds = view.bounds();
    let rep = match bitmap_at_scale(bounds.size, scale) {
        Ok(rep) => rep,
        Err(reason) => return out::unavailable(reason),
    };
    view.cacheDisplayInRect_toBitmapImageRep(bounds, &rep);
    let data = match ffi::png_data(&rep) {
        Ok(data) => data,
        Err(reason) => return out::unavailable(reason),
    };
    if let Err(e) = std::fs::write(path, data) {
        return out::unavailable(format!("cannot write {}: {e}", path.display()));
    }
    let mut t = Table::new();
    out::put(&mut t, "file", out::text(rel));
    out::put(
        &mut t,
        "pixels_wh",
        out::array(vec![
            out::int_isize(rep.pixelsWide()),
            out::int_isize(rep.pixelsHigh()),
        ]),
    );
    Value::Table(t)
}

/// The ring round a plain focusable view, per appearance, focused and not.
fn focus_view_table(mtm: MainThreadMarker, app: &NSApplication, out_dir: &Path) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "a titled window (empty title, content {} from {}) holding at {} an NSView subclass (NativeThemeOsProbeFocusView) that accepts first responder and whose drawFocusRingMask fills its bounds (NSBezierPath fillRect:) and focusRingMaskBounds is its bounds, focusRingType as made; made key with makeKeyAndOrderFront: and the run loop pumped until isKeyWindow (up to {KEY_WAIT_STEPS} steps); per appearance (the window's appearance set) the window captured with screencapture -x -o -l after makeFirstResponder: nil (unfocused) and with the view (focused), each after {SETTLE_STEPS} steps, to focus_view/<appearance>-<unfocused|focused>.png; view_frame_from_top_left_xywh in the window frame's points from its top left",
            super::rect_text(NSRect::new(NSPoint::new(0.0, 0.0), FOCUS_FRAME.size)),
            super::point_text(FOCUS_FRAME.origin),
            super::rect_text(FOCUS_VIEW_FRAME),
        )),
    );
    let window = match ffi::titled_window(mtm, FOCUS_FRAME) {
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
    let view = ffi::FocusView::new(mtm, FOCUS_VIEW_FRAME);
    content.addSubview(&view);
    out::put(
        &mut t,
        "focusRingType_raw",
        i64::try_from(view.focusRingType().0).map_or_else(
            |_| out::text(view.focusRingType().0.to_string()),
            Value::Integer,
        ),
    );
    let dir = out_dir.join("focus_view");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        out::put(
            &mut t,
            "error",
            out::unavailable(format!("cannot create {}: {e}", dir.display())),
        );
        return t;
    }
    window.makeKeyAndOrderFront(None);
    activate_ignoring(app);
    let steps = pump(app, KEY_WAIT_STEPS, || window.isKeyWindow());
    out::put(&mut t, "key_wait_steps", Value::Integer(i64::from(steps)));
    let frame = window.frame();
    out::put(&mut t, "window_frame_xywh", rect(frame));
    let r = view.convertRect_toView(view.bounds(), None);
    out::put(
        &mut t,
        "view_frame_from_top_left_xywh",
        rect(NSRect::new(
            NSPoint::new(r.origin.x, frame.size.height - r.origin.y - r.size.height),
            r.size,
        )),
    );
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
        let mut a = Table::new();
        for (state, focused) in [("unfocused", false), ("focused", true)] {
            let mut s = Table::new();
            let view_responder: &NSResponder = &view;
            let responder = if focused { Some(view_responder) } else { None };
            out::put(
                &mut s,
                "makeFirstResponder",
                out::boolean(window.makeFirstResponder(responder)),
            );
            window.display();
            pump(app, SETTLE_STEPS, || false);
            out::put(&mut s, "isKeyWindow", out::boolean(window.isKeyWindow()));
            let first_is_view = window.firstResponder().is_some_and(|r| {
                let r: &NSResponder = &r;
                std::ptr::eq(r, view_responder)
            });
            out::put(
                &mut s,
                "firstResponder_is_view",
                out::boolean(first_is_view),
            );
            let file = format!("{key}-{state}.png");
            match screencapture(window.windowNumber(), &dir.join(&file)) {
                Ok(()) => out::put(&mut s, "capture", out::text(format!("focus_view/{file}"))),
                Err(reason) => out::put(&mut s, "capture", out::unavailable(reason)),
            }
            out::put_table(&mut a, state, s);
        }
        out::put_table(&mut t, key, a);
    }
    window.makeFirstResponder(None);
    window.setAppearance(None);
    window.close();
    t
}
