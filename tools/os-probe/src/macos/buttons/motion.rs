//! The motion pass (`--button-pass --motion`; the 0.8.0 measurement plan's
//! MP-record): scripted screen recordings ([`timeline`]) of a window of
//! controls, per appearance: a push button and (macOS 26) a glass button
//! pressed and held, clicked, released early and pressed while the hover
//! runs (U103, U181, U332, U612), two radio buttons of one group clicked in
//! turn (U160), a search field typed into, cleared with Escape and cleared
//! by the program (the cancel button, U302), the push button's focus ring
//! interrupted early, under an appearance change and under a window
//! deactivation (U181, U182), and a text view's caret over a long time with
//! typing, at a mixed-direction boundary and in a read-only text view (U328,
//! U606). The workflow cuts each recording to its crop, frame by frame.

use objc2::rc::Retained;
use objc2::{MainThreadOnly, Message};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSBezelStyle, NSButton, NSButtonType, NSControl,
    NSControlSize, NSResponder, NSScreen, NSSearchField, NSTextView, NSView, NSWindow,
};
use objc2_foundation::{NSOperatingSystemVersion, NSPoint, NSRange, NSRect, NSSize};
use toml::Table;

use super::timeline::{
    self, Posted, Recording, action, click, grown, key, post, quartz_centre, screen_frame,
};
use super::{Pass, SETTLE_STEPS};
use crate::macos::{APPEARANCES, control_common, ffi, ns, point_text, rect, rect_text};
use crate::out;

/// The window's content frame on the screen.
const WINDOW_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 200.0), NSSize::new(520.0, 300.0));

/// The second window the deactivation recording makes key.
const OTHER_FRAME: NSRect = NSRect::new(NSPoint::new(860.0, 200.0), NSSize::new(200.0, 100.0));

/// Where each control's frame starts in the content view (bottom-left
/// origin); the buttons and radios are sized to fit.
const PUSH_ORIGIN: NSPoint = NSPoint::new(20.0, 250.0);
const GLASS_ORIGIN: NSPoint = NSPoint::new(160.0, 250.0);
const RADIO_A_ORIGIN: NSPoint = NSPoint::new(20.0, 200.0);
const RADIO_B_ORIGIN: NSPoint = NSPoint::new(120.0, 200.0);
const SEARCH_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 150.0), NSSize::new(180.0, 22.0));
const CARET_FRAME: NSRect = NSRect::new(NSPoint::new(20.0, 90.0), NSSize::new(80.0, 30.0));
const MIXED_FRAME: NSRect = NSRect::new(NSPoint::new(140.0, 90.0), NSSize::new(120.0, 30.0));
const READ_ONLY_FRAME: NSRect = NSRect::new(NSPoint::new(300.0, 90.0), NSSize::new(80.0, 30.0));
const TICKER_FRAME: NSRect = NSRect::new(NSPoint::new(504.0, 284.0), NSSize::new(8.0, 8.0));

/// How far each recording's crop reaches round its subject, in points.
const CROP_MARGIN: f64 = 12.0;

/// The texts.
const CARET_TEXT: &str = "Ag";
const MIXED_TEXT: &str = "abc \u{05d0}\u{05d1}\u{05d2}";
/// The insertion point in the mixed text: between the space and the first
/// Hebrew letter, where the left-to-right run meets the right-to-left one.
const MIXED_CARET: usize = 4;

/// Virtual key codes (HIToolbox's Events.h: kVK_ANSI_A, kVK_ANSI_S,
/// kVK_ANSI_D, kVK_Escape).
const KEY_A: u16 = 0;
const KEY_S: u16 = 1;
const KEY_D: u16 = 2;
const KEY_ESCAPE: u16 = 53;

/// The controls the recordings use.
struct Controls {
    window: Retained<NSWindow>,
    content: Retained<NSView>,
    push: Retained<NSButton>,
    glass: Option<Retained<NSButton>>,
    radio_a: Retained<NSButton>,
    radio_b: Retained<NSButton>,
    search: Retained<NSSearchField>,
    caret: Retained<NSTextView>,
    mixed: Retained<NSTextView>,
    read_only: Retained<NSTextView>,
}

/// `[macos.button_pass.motion]`.
pub(super) fn motion_table(pass: &Pass<'_>, version: &NSOperatingSystemVersion) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    let controls = match build(pass, version) {
        Ok(controls) => controls,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    if controls.glass.is_none() {
        out::put(
            &mut t,
            "glass",
            out::not_probed("NSBezelStyleGlass needs macOS 26"),
        );
    }
    let ticker = timeline::ticker(pass, &controls.content, TICKER_FRAME);
    let other = ffi::titled_window(pass.mtm, OTHER_FRAME).ok();
    if let Some(other) = &other {
        other.setTitle(&ns(""));
    }
    let _ = pass.present(&controls.window, &mut t);
    let scale = NSScreen::mainScreen(pass.mtm).map(|s| s.backingScaleFactor());
    match scale {
        Some(scale) => out::put(&mut t, "mainScreen_backingScaleFactor", out::float(scale)),
        None => out::put(
            &mut t,
            "mainScreen_backingScaleFactor",
            out::unavailable("mainScreen is nil"),
        ),
    }
    out::put(&mut t, "window_frame_xywh", rect(controls.window.frame()));
    out::put_table(
        &mut t,
        "cursor",
        pass.move_cursor(&controls.window, super::away_point(&controls.window)),
    );
    let mut views = Table::new();
    let c = &controls;
    let glass_view = match &c.glass {
        Some(g) => Some::<&NSView>(g),
        None => None,
    };
    let named: [(&str, Option<&NSView>); 8] = [
        ("push", Some::<&NSView>(&c.push)),
        ("glass", glass_view),
        ("radio_a", Some::<&NSView>(&c.radio_a)),
        ("radio_b", Some::<&NSView>(&c.radio_b)),
        ("search", Some::<&NSView>(&c.search)),
        ("caret", Some::<&NSView>(&c.caret)),
        ("mixed", Some::<&NSView>(&c.mixed)),
        ("read_only", Some::<&NSView>(&c.read_only)),
    ];
    for (name, view) in named {
        if let Some(view) = view {
            out::put(&mut views, name, rect(screen_frame(view)));
        }
    }
    out::put_table(&mut t, "frames_screen_xywh", views);

    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        let opposite = APPEARANCES
            .iter()
            .find(|(k, _)| *k != key)
            .and_then(|(_, n)| NSAppearance::appearanceNamed(&ns(n)));
        controls.window.setAppearance(Some(&appearance));
        controls.content.setNeedsDisplay(true);
        pass.settle(&controls.window, SETTLE_STEPS);
        let mut per = Table::new();
        let recordings = recordings(
            pass,
            &controls,
            other.as_deref(),
            key,
            &appearance,
            opposite.as_deref(),
        );
        for (name, rec) in recordings {
            let _ = controls.window.makeFirstResponder(None);
            pass.settle(&controls.window, SETTLE_STEPS);
            let r = timeline::record(pass, &ticker, scale, pass.out_dir, rec);
            out::put_table(&mut per, name, r);
            reset(&controls);
            pass.settle(&controls.window, SETTLE_STEPS);
        }
        out::put_table(&mut t, key, per);
    }
    controls.window.setAppearance(None);
    controls.window.orderOut(None);
    controls.window.close();
    if let Some(other) = other {
        other.orderOut(None);
        other.close();
    }
    t
}

/// Put every control back as it was built.
fn reset(c: &Controls) {
    c.search.setStringValue(&ns(""));
    c.caret.setString(&ns(CARET_TEXT));
    c.caret.setSelectedRange(NSRange::new(CARET_TEXT.len(), 0));
    c.radio_a.setState(0);
    c.radio_b.setState(0);
    let _ = c.window.makeFirstResponder(None);
    c.window.makeKeyAndOrderFront(None);
}

/// The window and its controls.
fn build(pass: &Pass<'_>, version: &NSOperatingSystemVersion) -> Result<Controls, String> {
    let mtm = pass.mtm;
    let window = ffi::titled_window(mtm, WINDOW_FRAME)?;
    window.setTitle(&ns(""));
    let content = window
        .contentView()
        .ok_or_else(|| "the titled window has no content view".to_owned())?;
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
    let push = button("Button", PUSH_ORIGIN);
    let glass = (version.majorVersion >= 26).then(|| {
        let g = button("Button", GLASS_ORIGIN);
        g.setBezelStyle(NSBezelStyle::Glass);
        g.sizeToFit();
        g
    });
    let radio = |title: &str, origin: NSPoint| {
        let r = NSButton::new(mtm);
        r.setButtonType(NSButtonType::Radio);
        r.setTitle(&ns(title));
        control_common(&r, NSControlSize::Regular);
        r.setState(0);
        let control: &NSControl = &r;
        ffi::set_harmless_action(control, pass.app);
        r.sizeToFit();
        r.setFrameOrigin(origin);
        content.addSubview(&r);
        r
    };
    let radio_a = radio("One", RADIO_A_ORIGIN);
    let radio_b = radio("Two", RADIO_B_ORIGIN);
    let search = NSSearchField::initWithFrame(NSSearchField::alloc(mtm), SEARCH_FRAME);
    control_common(&search, NSControlSize::Regular);
    content.addSubview(&search);
    let text = |frame: NSRect, string: &str, caret: usize, editable: bool| {
        let v = NSTextView::initWithFrame(NSTextView::alloc(mtm), frame);
        v.setString(&ns(string));
        v.setEditable(editable);
        v.setSelectable(true);
        v.setSelectedRange(NSRange::new(caret, 0));
        content.addSubview(&v);
        v
    };
    let caret = text(CARET_FRAME, CARET_TEXT, CARET_TEXT.len(), true);
    let mixed_len = MIXED_TEXT.encode_utf16().count();
    let mixed = text(MIXED_FRAME, MIXED_TEXT, MIXED_CARET.min(mixed_len), true);
    let read_only = text(READ_ONLY_FRAME, CARET_TEXT, 1, false);
    Ok(Controls {
        window,
        content,
        push,
        glass,
        radio_a,
        radio_b,
        search,
        caret,
        mixed,
        read_only,
    })
}

/// A main-thread action making `responder` first responder in `window`.
fn focus<'a>(at_ms: u64, window: &'a NSWindow, responder: &'a NSResponder) -> timeline::Action<'a> {
    action(at_ms, "makeFirstResponder", move || {
        out::boolean(window.makeFirstResponder(Some(responder)))
    })
}

/// A main-thread action resigning `window`'s first responder.
fn unfocus(at_ms: u64, window: &NSWindow) -> timeline::Action<'_> {
    action(at_ms, "makeFirstResponder nil", move || {
        out::boolean(window.makeFirstResponder(None))
    })
}

/// The recordings, in order, for the appearance `key`.
fn recordings<'a>(
    pass: &Pass<'_>,
    c: &'a Controls,
    other: Option<&'a NSWindow>,
    key: &str,
    appearance: &'a NSAppearance,
    opposite: Option<&'a NSAppearance>,
) -> Vec<(&'static str, Recording<'a>)> {
    let mut list = Vec::new();
    let away = pass.quartz(super::away_point(&c.window));
    let mut presses: Vec<(&'static str, &NSButton)> = vec![("press-push", &*c.push)];
    if let Some(glass) = &c.glass {
        presses.push(("press-glass", &**glass));
    }
    for (name, button) in presses {
        let view: &NSView = button;
        let at = quartz_centre(pass, view);
        let mut posts = vec![post(2000, "move in", Posted::Move(at))];
        click(&mut posts, 3000, 1000, at, "held press");
        click(&mut posts, 5000, 80, at, "release at 80 ms");
        click(&mut posts, 6000, 150, at, "release at 150 ms");
        click(&mut posts, 7000, 250, at, "release at 250 ms");
        posts.push(post(8000, "move away", Posted::Move(away)));
        posts.push(post(8500, "move in again", Posted::Move(at)));
        click(&mut posts, 8600, 600, at, "press 100 ms into the hover");
        posts.push(post(10000, "move away", Posted::Move(away)));
        list.push((
            name,
            Recording {
                stem: format!("{key}/{name}"),
                crop: grown(screen_frame(view), CROP_MARGIN),
                seconds: 12,
                cursor: false,
                actions: Vec::new(),
                posts,
                sample: Some(view.retain()),
                sample_cursor: false,
            },
        ));
    }

    let a = quartz_centre(pass, &c.radio_a);
    let b = quartz_centre(pass, &c.radio_b);
    let mut posts = Vec::new();
    click(&mut posts, 3000, 100, a, "click One");
    click(&mut posts, 5000, 100, b, "click Two");
    click(&mut posts, 7000, 100, a, "click One");
    posts.push(post(8000, "move away", Posted::Move(away)));
    let fa = screen_frame(&c.radio_a);
    let fb = screen_frame(&c.radio_b);
    list.push((
        "radio",
        Recording {
            stem: format!("{key}/radio"),
            crop: grown(union(fa, fb), CROP_MARGIN),
            seconds: 9,
            cursor: false,
            actions: Vec::new(),
            posts,
            sample: Some(Retained::into_super(Retained::into_super(c.radio_a.clone()))),
            sample_cursor: false,
        },
    ));

    let mut posts = Vec::new();
    key_typed(&mut posts, 3000, KEY_A, "type a");
    key_typed(&mut posts, 3200, KEY_S, "type s");
    key_typed(&mut posts, 5000, KEY_ESCAPE, "Escape");
    key_typed(&mut posts, 6500, KEY_D, "type d");
    let search: &NSResponder = &c.search;
    let field = &c.search;
    list.push((
        "search-cancel",
        Recording {
            stem: format!("{key}/search-cancel"),
            crop: grown(screen_frame(&c.search), CROP_MARGIN),
            seconds: 10,
            cursor: false,
            actions: vec![
                focus(2500, &c.window, search),
                action(8000, "setStringValue empty", move || {
                    field.setStringValue(&ns(""));
                    out::text("done")
                }),
            ],
            posts,
            sample: None,
            sample_cursor: false,
        },
    ));

    let push: &NSResponder = &c.push;
    let window: &NSWindow = &c.window;
    let mut actions = Vec::new();
    for (i, (at, hold)) in [(3000u64, 70u64), (4500, 120), (6000, 170)]
        .into_iter()
        .enumerate()
    {
        actions.push(focus(at, window, push));
        actions.push(action(
            at.saturating_add(hold),
            &format!("unfocus {} ms after focus ({})", hold, i),
            move || out::boolean(window.makeFirstResponder(None)),
        ));
    }
    actions.push(focus(7500, window, push));
    if let Some(opposite) = opposite {
        actions.push(action(7600, "window appearance flipped", move || {
            window.setAppearance(Some(opposite));
            out::text(opposite.name().to_string())
        }));
        actions.push(action(9000, "window appearance restored", move || {
            window.setAppearance(Some(appearance));
            out::text(appearance.name().to_string())
        }));
    }
    actions.push(unfocus(10000, window));
    list.push((
        "focus-interrupt",
        Recording {
            stem: format!("{key}/focus-interrupt"),
            crop: grown(screen_frame(&c.push), CROP_MARGIN),
            seconds: 12,
            cursor: false,
            actions,
            posts: Vec::new(),
            sample: Some(Retained::into_super(Retained::into_super(c.push.clone()))),
            sample_cursor: false,
        },
    ));

    if let Some(other) = other {
        list.push((
            "focus-deactivate",
            Recording {
                stem: format!("{key}/focus-deactivate"),
                crop: grown(screen_frame(&c.push), CROP_MARGIN),
                seconds: 9,
                cursor: false,
                actions: vec![
                    focus(3000, window, push),
                    action(3100, "other window made key", move || {
                        other.makeKeyAndOrderFront(None);
                        out::boolean(other.isKeyWindow())
                    }),
                    action(5000, "window made key again", move || {
                        window.makeKeyAndOrderFront(None);
                        out::boolean(window.isKeyWindow())
                    }),
                    action(5100, "other window ordered out", move || {
                        other.orderOut(None);
                        out::boolean(window.isKeyWindow())
                    }),
                    unfocus(7000, window),
                ],
                posts: Vec::new(),
                sample: None,
                sample_cursor: false,
            },
        ));
    }

    let caret: &NSResponder = &c.caret;
    let mut posts = Vec::new();
    for (at, code) in [(30000, KEY_A), (30150, KEY_S), (30300, KEY_D)] {
        key_typed(&mut posts, at, code, "type");
    }
    list.push((
        "caret-long",
        Recording {
            stem: format!("{key}/caret-long"),
            crop: grown(screen_frame(&c.caret), CROP_MARGIN),
            seconds: 40,
            cursor: false,
            actions: vec![focus(2000, window, caret), unfocus(38000, window)],
            posts,
            sample: None,
            sample_cursor: false,
        },
    ));
    let mixed: &NSResponder = &c.mixed;
    list.push((
        "caret-mixed",
        Recording {
            stem: format!("{key}/caret-mixed"),
            crop: grown(screen_frame(&c.mixed), CROP_MARGIN),
            seconds: 8,
            cursor: false,
            actions: vec![focus(2000, window, mixed), unfocus(7000, window)],
            posts: Vec::new(),
            sample: None,
            sample_cursor: false,
        },
    ));
    let read_only: &NSResponder = &c.read_only;
    list.push((
        "caret-read-only",
        Recording {
            stem: format!("{key}/caret-read-only"),
            crop: grown(screen_frame(&c.read_only), CROP_MARGIN),
            seconds: 8,
            cursor: false,
            actions: vec![focus(2000, window, read_only), unfocus(7000, window)],
            posts: Vec::new(),
            sample: None,
            sample_cursor: false,
        },
    ));
    list
}

/// A key typed (key-down, key-up).
fn key_typed(posts: &mut Vec<timeline::Post>, at_ms: u64, code: u16, label: &str) {
    key(posts, at_ms, code, label);
}

/// The smallest rect holding `a` and `b`.
fn union(a: NSRect, b: NSRect) -> NSRect {
    let x0 = a.origin.x.min(b.origin.x);
    let y0 = a.origin.y.min(b.origin.y);
    let x1 = (a.origin.x + a.size.width).max(b.origin.x + b.size.width);
    let y1 = (a.origin.y + a.size.height).max(b.origin.y + b.size.height);
    NSRect::new(NSPoint::new(x0, y0), NSSize::new(x1 - x0, y1 - y0))
}

fn method_text() -> String {
    format!(
        "a titled window (empty title, content {} from {}) holding, in its content view (bottom-left origin): a regular push button (NSButton, momentaryPushIn, bezelStyle push, title \"Button\", system font of the regular size, sized to fit) from {}; on macOS 26 the same with bezelStyle glass from {}; two radio buttons (buttonType radio, titles \"One\" and \"Two\", state off, both with the action updateWindows at NSApp, so they form one group) from {} and {}; an NSSearchField {}; three NSTextViews: editable \"{CARET_TEXT}\" with the insertion point at its end {}, editable \"{}\" (a left-to-right run, a space and three Hebrew letters) with the insertion point at UTF-16 index {MIXED_CARET} (between the space and the first Hebrew letter) {}, and a selectable, non-editable \"{CARET_TEXT}\" with the insertion point at index 1 {}; and a ticker (an NSBox, custom, no border) {}; a second titled window {} for focus-deactivate. The window presented as the key mode presents (made key, the application active), the cursor parked {} pt right of it. Per appearance (the window's appearance set, the run loop pumped {SETTLE_STEPS} steps), each recording: makeFirstResponder: nil, then `screencapture -x -v -V<seconds> <appearance>/<name>.mov` (the whole main display) while a worker thread posts CGEvents (mouse moved, left mouse down, up, dragged; key down and up) at the HID event tap at its planned times and the main thread runs its actions at theirs, pumping the run loop in steps of 5 ms between; a repeating NSTimer in the run loop's common modes (so it fires inside AppKit's mouse tracking too) flips the ticker black and white every 5 ms and, where the recording names a view, reads its alphaValue and the presentation layers of its layer tree (3 levels, at most 16 layers: path, class, opacity, bounds size, affine transform, the 3D transform's m33, m34 and m43; the model layer where none is presented), keeping each reading that differs from the one before as a sample, its time counted from starting screencapture; <name>.crop holds ffmpeg's crop=w:h:x:y in pixels for the subject's screen frame grown by {CROP_MARGIN} pt. After each recording the search field is emptied, the caret text view's text and insertion point restored, both radios set off and the window made key again. The recordings: press-push and press-glass (move in at 2 s; a press held 1 s from 3 s; clicks released 80, 150 and 250 ms after their mouse-down at 5, 6 and 7 s; move away at 8 s, in again at 8.5 s, a press 100 ms later held 600 ms; away at 10 s; 12 s; the button sampled); radio (clicks of 100 ms on One at 3 s, Two at 5 s, One at 7 s; 9 s; One sampled); search-cancel (the field made first responder at 2.5 s, a and s typed at 3 and 3.2 s, Escape at 5 s, d at 6.5 s, the program empties it at 8 s; 10 s); focus-interrupt (the push button made first responder at 3, 4.5 and 6 s and resigned 70, 120 and 170 ms later; made first responder at 7.5 s, the window's appearance set to the other one at 7.6 s and back at 9 s, resigned at 10 s; 12 s; the button sampled); focus-deactivate (the push button first responder at 3 s, the second window made key at 3.1 s, this window key again at 5 s, the second ordered out at 5.1 s, resigned at 7 s; 9 s); caret-long (the editable text view first responder at 2 s, a, s, d typed from 30 s 150 ms apart, resigned at 38 s; 40 s); caret-mixed and caret-read-only (first responder at 2 s, resigned at 7 s; 8 s). Each key is typed as a key-down and a key-up 40 ms later (virtual key codes 0, 1, 2 and 53 of HIToolbox's Events.h); each click as a mouse-down and a mouse-up at the view's centre",
        rect_text(NSRect::new(NSPoint::new(0.0, 0.0), WINDOW_FRAME.size)),
        point_text(WINDOW_FRAME.origin),
        point_text(PUSH_ORIGIN),
        point_text(GLASS_ORIGIN),
        point_text(RADIO_A_ORIGIN),
        point_text(RADIO_B_ORIGIN),
        rect_text(SEARCH_FRAME),
        rect_text(CARET_FRAME),
        MIXED_TEXT,
        rect_text(MIXED_FRAME),
        rect_text(READ_ONLY_FRAME),
        rect_text(TICKER_FRAME),
        rect_text(OTHER_FRAME),
        super::CURSOR_AWAY_OFFSET,
    )
}
