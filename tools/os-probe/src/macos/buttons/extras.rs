//! The button pass's other windows and reads: a toolbar with bordered,
//! plain and view items, captured per appearance and state; a text view's
//! caret, its subviews and captures; and the buttons' geometry and fonts
//! read without a window.

use objc2::runtime::ProtocolObject;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSBezelStyle, NSButton, NSButtonType, NSControlSize,
    NSResponder, NSTextView, NSToolbar, NSToolbarItem, NSView,
};
use objc2_foundation::{NSOperatingSystemVersion, NSPoint, NSRect, NSSize};
use toml::{Table, Value};

use super::{BUTTON_SIZES, BUTTON_TITLE, Pass, SETTLE_STEPS, from_top_left, raw, symbol_image};
use crate::macos::layer::changed;
use crate::macos::{APPEARANCES, control_common, ffi, font_table, ns, rect, size};
use crate::out;

/// The toolbar window's content frame on the screen, in points.
const TOOLBAR_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 300.0), NSSize::new(560.0, 120.0));

/// The caret window's content frame on the screen, in points.
const CARET_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 300.0), NSSize::new(300.0, 100.0));

/// The text view's frame in the caret window's content view, in points.
const CARET_TEXT_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 20.0), NSSize::new(260.0, 60.0));

/// The caret window's captures and the steps the pump runs between two.
const CARET_CAPTURES: u32 = 4;
const CARET_CAPTURE_STEPS: u32 = 6;

/// How deep the caret's subview walk goes below the text view.
const SUBVIEW_DEPTH: u32 = 3;

/// The toolbar: a titled window with an NSToolbar of four items, made key
/// (and given the mode's condition), captured per appearance normal, with
/// its view buttons highlighted, and with every item disabled.
pub(super) fn toolbar_table(pass: &Pass<'_>) -> Table {
    let mtm = pass.mtm;
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "a titled NSWindow ({} x {} pt content from {}, empty title) given an NSToolbar (identifier NativeThemeOsProbeToolbar, its display mode, size mode and the window's toolbarStyle left at their defaults and recorded) whose delegate gives four NSToolbarItems, each with autovalidates NO and enabled YES: bordered (label \"Item\", the symbol image, bordered YES), plain (label \"Item\", the symbol image, bordered NO), view_toolbar (an NSButton view, bezelStyle toolbar, title \"{BUTTON_TITLE}\", sized to fit) and view_push (an NSButton view, bezelStyle push, title \"{BUTTON_TITLE}\", sized to fit); made key and the mode's condition set as a sheet's; per appearance the window captured normal, pressed (highlight: YES on the two view buttons) and disabled (setEnabled: NO on every item and view button) to <appearance>/toolbar/<state>.png; the view items' frames are in the window frame's points from its top left",
            TOOLBAR_FRAME.size.width,
            TOOLBAR_FRAME.size.height,
            super::point_text(TOOLBAR_FRAME.origin),
        )),
    );
    let window = match ffi::titled_window(mtm, TOOLBAR_FRAME) {
        Ok(window) => window,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    window.setTitle(&ns(""));
    let image = match symbol_image() {
        Ok(image) => Some(image),
        Err(reason) => {
            out::put(&mut t, "image", out::unavailable(reason));
            None
        }
    };
    let item = |id: &str| {
        let item = NSToolbarItem::initWithItemIdentifier(NSToolbarItem::alloc(mtm), &ns(id));
        item.setAutovalidates(false);
        item.setEnabled(true);
        item
    };
    let bordered = item("bordered");
    bordered.setLabel(&ns("Item"));
    bordered.setImage(image.as_deref());
    bordered.setBordered(true);
    let plain = item("plain");
    plain.setLabel(&ns("Item"));
    plain.setImage(image.as_deref());
    plain.setBordered(false);
    let view_button = |bezel: NSBezelStyle| {
        let b = NSButton::new(mtm);
        b.setButtonType(NSButtonType::MomentaryPushIn);
        b.setBezelStyle(bezel);
        b.setTitle(&ns(BUTTON_TITLE));
        b.sizeToFit();
        b
    };
    let toolbar_button = view_button(NSBezelStyle::Toolbar);
    let push_button = view_button(NSBezelStyle::Push);
    let view_toolbar = item("view_toolbar");
    let v: &NSView = &toolbar_button;
    view_toolbar.setView(Some(v));
    let view_push = item("view_push");
    let v: &NSView = &push_button;
    view_push.setView(Some(v));
    let items = vec![bordered, plain, view_toolbar, view_push];
    let views: [(&str, &NSButton); 2] = [
        ("view_toolbar", &*toolbar_button),
        ("view_push", &*push_button),
    ];

    let delegate = ffi::ToolbarDelegate::new(mtm, items.clone());
    let toolbar =
        NSToolbar::initWithIdentifier(NSToolbar::alloc(mtm), &ns("NativeThemeOsProbeToolbar"));
    toolbar.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    window.setToolbar(Some(&toolbar));
    out::put(&mut t, "toolbarStyle_raw", raw(window.toolbarStyle().0));
    out::put(&mut t, "displayMode_raw", raw(toolbar.displayMode().0));
    // Deprecated (AppKit says it will ignore it), still read as made.
    #[allow(deprecated)]
    let size_mode = toolbar.sizeMode().0;
    out::put(&mut t, "sizeMode_raw", raw(size_mode));
    out::put(&mut t, "items", raw(toolbar.items().count()));
    out::put(
        &mut t,
        "visibleItems",
        toolbar
            .visibleItems()
            .map_or_else(|| out::unavailable("nil"), |v| raw(v.count())),
    );

    let other = pass.present(&window, &mut t);
    let frame = window.frame();
    out::put(&mut t, "window_frame_xywh", rect(frame));
    out::put(
        &mut t,
        "titleBar_and_toolbar_height",
        out::float(frame.size.height - window.contentRectForFrameRect(frame).size.height),
    );
    let mut item_frames = Table::new();
    for (name, b) in views {
        out::put(&mut item_frames, name, rect(from_top_left(b, frame.size)));
    }
    out::put_table(&mut t, "view_frames_from_top_left_xywh", item_frames);
    for (name, b) in views {
        out::put(
            &mut t,
            &format!("{name}_intrinsicContentSize_wh"),
            size(b.intrinsicContentSize()),
        );
        out::put(
            &mut t,
            &format!("{name}_controlSize_raw"),
            raw(b.controlSize().0),
        );
        match b.font() {
            Some(font) => out::put_table(&mut t, &format!("{name}_font"), font_table(&font)),
            None => out::put(&mut t, &format!("{name}_font"), out::unavailable("nil")),
        }
    }

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
        pass.settle(&window, SETTLE_STEPS);
        let dir = pass.out_dir.join(key).join("toolbar");
        let rel = format!("{key}/toolbar");
        let mut run = Table::new();
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut run,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            out::put_table(&mut t, key, run);
            continue;
        }
        let mut normal = Table::new();
        pass.shot(&window, &mut normal, &dir, &rel, "normal.png");
        out::put_table(&mut run, "normal", normal);

        for (_, b) in views {
            b.highlight(true);
        }
        pass.settle(&window, SETTLE_STEPS);
        let mut pressed = Table::new();
        pass.shot(&window, &mut pressed, &dir, &rel, "pressed.png");
        for (_, b) in views {
            b.highlight(false);
        }
        out::put_table(&mut run, "pressed", pressed);

        for item in &items {
            item.setEnabled(false);
        }
        for (_, b) in views {
            b.setEnabled(false);
        }
        pass.settle(&window, SETTLE_STEPS);
        let mut disabled = Table::new();
        pass.shot(&window, &mut disabled, &dir, &rel, "disabled.png");
        for item in &items {
            item.setEnabled(true);
        }
        for (_, b) in views {
            b.setEnabled(true);
        }
        pass.settle(&window, SETTLE_STEPS);
        out::put_table(&mut run, "disabled", disabled);
        out::put_table(&mut t, key, run);
    }
    window.setAppearance(None);
    window.orderOut(None);
    window.setToolbar(None);
    window.close();
    if let Some(other) = other {
        other.orderOut(None);
        other.close();
    }
    t
}

/// The caret: a text view made first responder in a key window, its
/// subviews (where AppKit keeps its insertion indicator) and a few captures
/// a blink apart, per appearance.
pub(super) fn caret_table(pass: &Pass<'_>) -> Table {
    let mtm = pass.mtm;
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "a titled NSWindow ({} x {} pt content from {}, empty title) holding an NSTextView (initWithFrame: {}) whose string is \"Text\", made key, then makeFirstResponder: with the text view; per appearance the window's appearance set, the run loop pumped {SETTLE_STEPS} steps, the text view's subviews walked {SUBVIEW_DEPTH} levels deep (class, frame in the text view's coordinates, hidden), and the window captured {CARET_CAPTURES} times {CARET_CAPTURE_STEPS} pump steps apart to <appearance>/caret/caret-<n>.png, as the caret blinks; each capture after the first is compared with the first (changed_from_capture_0: the pixels whose bytes differ and their bounds, in pixels from the window frame's top left: the caret where it blinked between the two)",
            CARET_FRAME.size.width,
            CARET_FRAME.size.height,
            super::point_text(CARET_FRAME.origin),
            crate::macos::rect_text(CARET_TEXT_FRAME),
        )),
    );
    let window = match ffi::titled_window(mtm, CARET_FRAME) {
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
    let text = NSTextView::initWithFrame(NSTextView::alloc(mtm), CARET_TEXT_FRAME);
    text.setString(&ns("Text"));
    content.addSubview(&text);
    let other = pass.present(&window, &mut t);
    let responder: &NSResponder = text.as_ref();
    out::put(
        &mut t,
        "makeFirstResponder_textView",
        out::boolean(window.makeFirstResponder(Some(responder))),
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
        pass.settle(&window, SETTLE_STEPS);
        let mut run = Table::new();
        let mut list = Vec::new();
        subviews(&text, &text, 0, &mut list);
        out::put(&mut run, "subviews", out::array(list));
        let dir = pass.out_dir.join(key).join("caret");
        let rel = format!("{key}/caret");
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut run,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            out::put_table(&mut t, key, run);
            continue;
        }
        let mut first: Option<crate::macos::layer::Rgba> = None;
        for n in 0..CARET_CAPTURES {
            let mut c = Table::new();
            let rgba = pass.shot(&window, &mut c, &dir, &rel, &format!("caret-{n}.png"));
            if let Some(rgba) = rgba {
                if let Some(base) = &first {
                    out::put_table(&mut c, "changed_from_capture_0", changed(base, &rgba));
                } else {
                    first = Some(rgba);
                }
            }
            out::put_table(&mut run, &format!("capture_{n}"), c);
            pass.settle(&window, CARET_CAPTURE_STEPS);
        }
        out::put_table(&mut t, key, run);
    }
    window.setAppearance(None);
    window.orderOut(None);
    window.close();
    if let Some(other) = other {
        other.orderOut(None);
        other.close();
    }
    t
}

/// Each subview of `view`, `depth` levels below `root`, with its class,
/// frame in `root`'s coordinates and whether it is hidden.
fn subviews(view: &NSView, root: &NSView, depth: u32, list: &mut Vec<Value>) {
    for sub in view.subviews().iter() {
        let mut s = Table::new();
        out::put(
            &mut s,
            "class",
            out::text(sub.class().name().to_string_lossy()),
        );
        out::put(&mut s, "depth", out::int(i64::from(depth)));
        out::put(
            &mut s,
            "frame_in_textView_xywh",
            rect(sub.convertRect_toView(sub.bounds(), Some(root))),
        );
        out::put(&mut s, "isHidden", out::boolean(sub.isHidden()));
        list.push(Value::Table(s));
        if depth < SUBVIEW_DEPTH {
            subviews(&sub, root, depth.saturating_add(1), list);
        }
    }
}

/// The buttons' geometry and fonts without a window: the button
/// `buttonWithTitle:target:action:` makes, and per control size a new
/// button's font before and after `setControlSize:` and an empty-title push
/// button sized to fit.
pub(super) fn geometry_table(mtm: MainThreadMarker, version: &NSOperatingSystemVersion) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "standard_button: +[NSButton buttonWithTitle:\"{BUTTON_TITLE}\" target:nil action:NULL], read as made; per control size: font_new, a new NSButton's font; font_after_setControlSize, the same button's after setControlSize: alone; font_control_common, after the sheets' setup (setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize:); empty_title, a new NSButton, buttonType momentaryPushIn, bezelStyle push, title \"\", given the sheets' setup, read before and after sizeToFit"
        )),
    );
    let standard = ffi::standard_button(mtm, &ns(BUTTON_TITLE));
    out::put_table(&mut t, "standard_button", button_geometry(&standard));

    let mut sizes = Table::new();
    for (size_name, control_size, needs) in BUTTON_SIZES {
        if version.majorVersion < needs {
            out::put(
                &mut sizes,
                size_name,
                out::unavailable(format!("{size_name} needs macOS {needs}")),
            );
            continue;
        }
        let mut s = Table::new();
        let fresh = NSButton::new(mtm);
        put_font(&mut s, "font_new", &fresh);
        fresh.setControlSize(control_size);
        put_font(&mut s, "font_after_setControlSize", &fresh);
        control_common(&fresh, control_size);
        put_font(&mut s, "font_control_common", &fresh);

        let empty = NSButton::new(mtm);
        empty.setButtonType(NSButtonType::MomentaryPushIn);
        empty.setBezelStyle(NSBezelStyle::Push);
        empty.setTitle(&ns(""));
        control_common(&empty, control_size);
        let mut e = Table::new();
        out::put(&mut e, "frame_before_sizeToFit_xywh", rect(empty.frame()));
        out::put(&mut e, "fittingSize_wh", size(empty.fittingSize()));
        out::put(
            &mut e,
            "intrinsicContentSize_wh",
            size(empty.intrinsicContentSize()),
        );
        empty.sizeToFit();
        out::put(&mut e, "frame_after_sizeToFit_xywh", rect(empty.frame()));
        if let Some(cell) = empty.cell() {
            out::put(&mut e, "cell_cellSize_wh", size(cell.cellSize()));
        }
        out::put_table(&mut s, "empty_title", e);
        out::put_table(&mut sizes, size_name, s);
    }
    out::put_table(&mut t, "sizes", sizes);
    t
}

/// A windowless button's size, style and font, as made.
fn button_geometry(b: &NSButton) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "frame_xywh", rect(b.frame()));
    out::put(&mut t, "fittingSize_wh", size(b.fittingSize()));
    out::put(
        &mut t,
        "intrinsicContentSize_wh",
        size(b.intrinsicContentSize()),
    );
    out::put(&mut t, "controlSize_raw", raw(b.controlSize().0));
    out::put(
        &mut t,
        "controlSize_is_regular",
        out::boolean(b.controlSize() == NSControlSize::Regular),
    );
    out::put(&mut t, "bezelStyle_raw", raw(b.bezelStyle().0));
    out::put(&mut t, "isBordered", out::boolean(b.isBordered()));
    put_font(&mut t, "font", b);
    t
}

fn put_font(t: &mut Table, key: &str, b: &NSButton) {
    match b.font() {
        Some(font) => out::put_table(t, key, font_table(&font)),
        None => out::put(t, key, out::unavailable("nil")),
    }
}
