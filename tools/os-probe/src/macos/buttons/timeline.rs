//! A scripted screen recording, shared by the motion and events passes:
//! `screencapture -v` records the main display while a worker thread posts
//! mouse and keyboard events (and takes window captures) at set times and
//! the main thread runs its own actions at set times, pumping the run loop
//! between them. A timer in the run loop's common modes, which fires inside
//! AppKit's mouse tracking and drag loops too, flips a ticker box every few
//! milliseconds (so the screen never stands still and the recording keeps a
//! frame wherever the screen changes, as in the focus-transition recordings)
//! and samples a view's alpha and presentation layers and the cursor's
//! visibility, keeping each sample that differs from the one before.
//!
//! The recording starts some time after `screencapture` does, so its times
//! are not the probe's: every time here is counted from starting
//! `screencapture`, and the ticker's flips mark the probe's steps in the
//! frames.

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2_app_kit::{NSBox, NSColor, NSEventMask, NSView};
use objc2_core_graphics::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use objc2_foundation::{NSDate, NSPoint, NSRect};
use toml::{Table, Value};

use super::Pass;
use crate::macos::{ffi, rect};
use crate::out;

/// One step of the main thread's pump while recording.
const FINE_STEP: Duration = Duration::from_millis(5);

/// How often the timer flips the ticker and samples, in seconds.
const TICK_SECONDS: f64 = 0.005;

/// How long after the recording's planned end the probe waits for
/// `screencapture` to exit before killing it.
const FINISH_MARGIN: Duration = Duration::from_secs(15);

/// How deep and how many presentation layers each sample reads.
const SAMPLE_DEPTH: usize = 3;
const SAMPLE_LAYERS: usize = 16;

/// The most samples one recording keeps.
const SAMPLE_LIMIT: usize = 4000;

/// The step between two drag events of [`drag_line`], in milliseconds.
pub(super) const DRAG_STEP_MS: u64 = 16;

/// How long a typed key is held, in milliseconds.
const KEY_HOLD_MS: u64 = 40;

/// What the worker thread posts or does.
pub(super) enum Posted {
    /// A mouse-moved event at a point in Quartz's global display points.
    Move(NSPoint),
    /// A left mouse-down.
    Down(NSPoint),
    /// A left mouse-up.
    Up(NSPoint),
    /// A left mouse-dragged event.
    Drag(NSPoint),
    /// A key-down (true) or key-up (false) of a virtual key code.
    Key(u16, bool),
    /// `screencapture -l` of a window, to a path, named in the log.
    Capture(isize, PathBuf, String),
}

/// One timed post of the worker thread.
pub(super) struct Post {
    pub(super) at: Duration,
    pub(super) label: String,
    pub(super) event: Posted,
}

/// One timed action of the main thread; what it returns is recorded.
pub(super) struct Action<'a> {
    pub(super) at: Duration,
    pub(super) label: String,
    pub(super) run: Box<dyn FnMut() -> Value + 'a>,
}

/// A recording to make.
pub(super) struct Recording<'a> {
    /// The files' path from the output directory, without an extension.
    pub(super) stem: String,
    /// The region the workflow cuts each frame to, in AppKit's screen points.
    pub(super) crop: NSRect,
    /// How long `screencapture` records, in seconds.
    pub(super) seconds: u32,
    /// Whether the recording shows the cursor (`screencapture -C`).
    pub(super) cursor: bool,
    pub(super) actions: Vec<Action<'a>>,
    pub(super) posts: Vec<Post>,
    /// The view whose alpha and presentation layers are sampled.
    pub(super) sample: Option<Retained<NSView>>,
    /// Whether each sample also reads `CGCursorIsVisible`.
    pub(super) sample_cursor: bool,
}

/// A timed post.
pub(super) fn post(at_ms: u64, label: &str, event: Posted) -> Post {
    Post {
        at: Duration::from_millis(at_ms),
        label: label.to_owned(),
        event,
    }
}

/// A timed main-thread action.
pub(super) fn action<'a>(at_ms: u64, label: &str, run: impl FnMut() -> Value + 'a) -> Action<'a> {
    Action {
        at: Duration::from_millis(at_ms),
        label: label.to_owned(),
        run: Box::new(run),
    }
}

/// A click at `at` (Quartz points): a mouse-down at `down_ms`, its mouse-up
/// `hold_ms` later.
pub(super) fn click(posts: &mut Vec<Post>, down_ms: u64, hold_ms: u64, at: NSPoint, label: &str) {
    posts.push(post(down_ms, &format!("{label} down"), Posted::Down(at)));
    posts.push(post(
        down_ms.saturating_add(hold_ms),
        &format!("{label} up"),
        Posted::Up(at),
    ));
}

/// A key typed: its key-down at `down_ms`, its key-up `KEY_HOLD_MS` later.
pub(super) fn key(posts: &mut Vec<Post>, down_ms: u64, code: u16, label: &str) {
    posts.push(post(
        down_ms,
        &format!("{label} down"),
        Posted::Key(code, true),
    ));
    posts.push(post(
        down_ms.saturating_add(KEY_HOLD_MS),
        &format!("{label} up"),
        Posted::Key(code, false),
    ));
}

/// Drag events from `from` to `to` (Quartz points), one every
/// `DRAG_STEP_MS` from `start_ms` to `end_ms`, the last at `to`.
pub(super) fn drag_line(
    posts: &mut Vec<Post>,
    start_ms: u64,
    end_ms: u64,
    from: NSPoint,
    to: NSPoint,
    label: &str,
) {
    let count = end_ms.saturating_sub(start_ms).div_euclid(DRAG_STEP_MS).max(1);
    for i in 1..=count {
        #[allow(clippy::cast_precision_loss)]
        let f = i as f64 / count as f64;
        let p = NSPoint::new(from.x + (to.x - from.x) * f, from.y + (to.y - from.y) * f);
        posts.push(post(
            start_ms.saturating_add(i.saturating_mul(DRAG_STEP_MS)),
            &format!("{label} {i}/{count}"),
            Posted::Drag(p),
        ));
    }
}

/// The ticker: a small box whose fill the timer flips black and white.
pub(super) fn ticker(pass: &Pass<'_>, content: &NSView, frame: NSRect) -> Retained<NSBox> {
    use objc2::MainThreadOnly;
    let view = NSBox::initWithFrame(NSBox::alloc(pass.mtm), frame);
    view.setBoxType(objc2_app_kit::NSBoxType::Custom);
    view.setTitlePosition(objc2_app_kit::NSTitlePosition::NoTitle);
    view.setBorderWidth(0.0);
    view.setFillColor(&NSColor::blackColor());
    content.addSubview(&view);
    view
}

/// What the timer shares with the recording.
struct Shared {
    start: Instant,
    on: Cell<bool>,
    flips: Cell<u64>,
    last: RefCell<String>,
    samples: RefCell<Vec<Value>>,
}

/// Make `rec`, its files under `out_dir`: `<stem>.mov`, the crop in
/// `<stem>.crop` (ffmpeg's `crop=w:h:x:y` in pixels) and the window captures
/// its posts take. What happened, with every time counted from starting
/// `screencapture`.
pub(super) fn record(
    pass: &Pass<'_>,
    ticker: &Retained<NSBox>,
    scale: Option<f64>,
    out_dir: &Path,
    rec: Recording<'_>,
) -> Table {
    let mut t = Table::new();
    let Recording {
        stem,
        crop,
        seconds,
        cursor,
        mut actions,
        posts,
        sample,
        sample_cursor,
    } = rec;
    let path = out_dir.join(format!("{stem}.mov"));
    if let Some(parent) = path.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        out::put(
            &mut t,
            "error",
            out::unavailable(format!("cannot create {}: {e}", parent.display())),
        );
        return t;
    }
    out::put(&mut t, "crop_screen_xywh", rect(crop));
    let quartz = NSRect::new(
        pass.quartz(NSPoint::new(
            crop.origin.x,
            crop.origin.y + crop.size.height,
        )),
        crop.size,
    );
    out::put(&mut t, "crop_quartz_xywh", rect(quartz));
    match scale.and_then(|s| crop_text(quartz, s)) {
        Some(text) => {
            let crop_path = out_dir.join(format!("{stem}.crop"));
            match std::fs::write(&crop_path, &text) {
                Ok(()) => out::put(&mut t, "crop_file", out::text(format!("{stem}.crop"))),
                Err(e) => out::put(
                    &mut t,
                    "crop_file",
                    out::unavailable(format!("cannot write {}: {e}", crop_path.display())),
                ),
            }
            out::put(&mut t, "crop_px_whxy", out::text(text));
        }
        None => out::put(
            &mut t,
            "crop_px_whxy",
            out::unavailable("no backing scale, or the crop lies off the screen"),
        ),
    }

    actions.sort_by_key(|a| a.at);
    let shared = Rc::new(Shared {
        start: Instant::now(),
        on: Cell::new(false),
        flips: Cell::new(0),
        last: RefCell::new(String::new()),
        samples: RefCell::new(Vec::new()),
    });
    let start = shared.start;
    let mut command = std::process::Command::new("screencapture");
    command.arg("-x").arg("-v").arg(format!("-V{seconds}"));
    if cursor {
        command.arg("-C");
    }
    let child = command
        .arg(&path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    let mut child = match child {
        Ok(child) => child,
        Err(e) => {
            out::put(
                &mut t,
                "recording",
                out::unavailable(format!("cannot run screencapture: {e}")),
            );
            return t;
        }
    };

    let timer = {
        let shared = Rc::clone(&shared);
        let ticker = ticker.clone();
        ffi::common_modes_timer(pass.mtm, TICK_SECONDS, move || {
            let on = !shared.on.get();
            shared.on.set(on);
            let fill = if on {
                NSColor::whiteColor()
            } else {
                NSColor::blackColor()
            };
            ticker.setFillColor(&fill);
            shared.flips.set(shared.flips.get().saturating_add(1));
            if sample.is_none() && !sample_cursor {
                return;
            }
            let mut now = String::new();
            if let Some(view) = &sample {
                now.push_str(&format!("alpha {:.4}", view.alphaValue()));
                for row in ffi::presentation_tree(view, SAMPLE_DEPTH, SAMPLE_LAYERS) {
                    now.push_str(" | ");
                    now.push_str(&row);
                }
            }
            if sample_cursor {
                #[allow(deprecated)]
                let visible = objc2_core_graphics::CGCursorIsVisible();
                now.push_str(&format!(" | CGCursorIsVisible {visible}"));
            }
            let changed = *shared.last.borrow() != now;
            if changed && shared.samples.borrow().len() < SAMPLE_LIMIT {
                let ms = shared.start.elapsed().as_secs_f64() * 1000.0;
                shared
                    .samples
                    .borrow_mut()
                    .push(out::text(format!("{ms:.1} ms: {now}")));
                *shared.last.borrow_mut() = now;
            }
        })
    };

    let worker = std::thread::spawn(move || {
        let mut log = Vec::new();
        for p in posts {
            if let Some(wait) = p.at.checked_sub(start.elapsed()) {
                std::thread::sleep(wait);
            }
            let at = start.elapsed();
            let outcome = post_one(&p.event);
            log.push(format!(
                "{:.1} ms (planned {} ms): {}: {outcome}",
                at.as_secs_f64() * 1000.0,
                p.at.as_millis(),
                p.label
            ));
        }
        log
    });

    let mut ran = Table::new();
    let mut next = 0usize;
    let deadline = Duration::from_secs(u64::from(seconds)) + FINISH_MARGIN;
    let mut status: Option<Result<std::process::ExitStatus, String>> = None;
    let mode = ffi::default_run_loop_mode();
    let mut steps: u64 = 0;
    while start.elapsed() < deadline {
        while let Some(a) = actions.get_mut(next) {
            if a.at > start.elapsed() {
                break;
            }
            let began = start.elapsed();
            let value = (a.run)();
            let ended = start.elapsed();
            let mut row = Table::new();
            out::put(
                &mut row,
                "planned_ms",
                out::float(a.at.as_secs_f64() * 1000.0),
            );
            out::put(&mut row, "ran_ms", out::float(began.as_secs_f64() * 1000.0));
            out::put(
                &mut row,
                "returned_ms",
                out::float(ended.as_secs_f64() * 1000.0),
            );
            out::put(&mut row, "result", value);
            out::put_table(&mut ran, &format!("{next:02} {}", a.label), row);
            next = next.saturating_add(1);
        }
        if next >= actions.len() && worker.is_finished() {
            match child.try_wait() {
                Ok(Some(s)) => {
                    status = Some(Ok(s));
                    break;
                }
                Ok(None) => {}
                Err(e) => {
                    status = Some(Err(format!("could not be waited for: {e}")));
                    break;
                }
            }
        }
        let until = NSDate::dateWithTimeIntervalSinceNow(FINE_STEP.as_secs_f64());
        while let Some(event) = pass.app.nextEventMatchingMask_untilDate_inMode_dequeue(
            NSEventMask::Any,
            Some(&until),
            mode,
            true,
        ) {
            pass.app.sendEvent(&event);
        }
        pass.app.updateWindows();
        steps = steps.saturating_add(1);
    }
    timer.invalidate();
    let outcome = match status {
        Some(Ok(s)) if s.success() => out::text("exit 0"),
        Some(Ok(s)) => out::unavailable(format!("screencapture exited with {s}")),
        Some(Err(reason)) => out::unavailable(format!("screencapture {reason}")),
        None => {
            let killed = child.kill().map_or_else(
                |e| format!(", and could not be killed: {e}"),
                |()| String::new(),
            );
            // Reap it, so no zombie outlives the probe; its status says nothing more.
            let _ = child.wait();
            out::unavailable(format!(
                "screencapture did not finish within {} s of starting it{killed}",
                deadline.as_secs()
            ))
        }
    };
    let log = worker
        .join()
        .unwrap_or_else(|_| vec!["the posting thread panicked".to_owned()]);
    out::put(&mut t, "recording", outcome);
    out::put(
        &mut t,
        "ended_ms",
        out::float(start.elapsed().as_secs_f64() * 1000.0),
    );
    out::put(
        &mut t,
        "pump_steps",
        out::int(i64::try_from(steps).unwrap_or(i64::MAX)),
    );
    out::put(
        &mut t,
        "ticker_flips",
        out::int(i64::try_from(shared.flips.get()).unwrap_or(i64::MAX)),
    );
    out::put(
        &mut t,
        "posted",
        out::array(log.into_iter().map(out::text).collect()),
    );
    out::put_table(&mut t, "actions", ran);
    let samples = shared.samples.take();
    if !samples.is_empty() {
        out::put(&mut t, "samples", out::array(samples));
    }
    match std::fs::metadata(&path) {
        Ok(m) => {
            out::put(&mut t, "file", out::text(format!("{stem}.mov")));
            out::put(
                &mut t,
                "file_bytes",
                out::int(i64::try_from(m.len()).unwrap_or(i64::MAX)),
            );
        }
        Err(e) => out::put(
            &mut t,
            "file",
            out::unavailable(format!("cannot read {}: {e}", path.display())),
        ),
    }
    t
}

/// Post one event (or take one capture) from the worker thread: what came
/// of it.
fn post_one(event: &Posted) -> String {
    let mouse = |kind: CGEventType, at: NSPoint| {
        let e = CGEvent::new_mouse_event(None, kind, at, CGMouseButton::Left);
        let made = e.is_some();
        CGEvent::post(CGEventTapLocation::HIDEventTap, e.as_deref());
        if made {
            format!("posted at ({:.1}, {:.1})", at.x, at.y)
        } else {
            "the CGEvent is NULL".to_owned()
        }
    };
    match event {
        Posted::Move(at) => mouse(CGEventType::MouseMoved, *at),
        Posted::Down(at) => mouse(CGEventType::LeftMouseDown, *at),
        Posted::Up(at) => mouse(CGEventType::LeftMouseUp, *at),
        Posted::Drag(at) => mouse(CGEventType::LeftMouseDragged, *at),
        Posted::Key(code, down) => {
            let e = CGEvent::new_keyboard_event(None, *code, *down);
            let made = e.is_some();
            CGEvent::post(CGEventTapLocation::HIDEventTap, e.as_deref());
            if made {
                format!("posted key {code}")
            } else {
                "the CGEvent is NULL".to_owned()
            }
        }
        Posted::Capture(number, path, name) => match crate::macos::screencapture(*number, path) {
            Ok(()) => format!("captured {name}"),
            Err(reason) => format!("capture {name} failed: {reason}"),
        },
    }
}

/// ffmpeg's `crop=` argument, `w:h:x:y` in pixels, for `quartz` (global
/// display points, top-left origin) at `scale` pixels per point; `None`
/// where it would start off the screen's top left.
fn crop_text(quartz: NSRect, scale: f64) -> Option<String> {
    let x = (quartz.origin.x * scale).floor();
    let y = (quartz.origin.y * scale).floor();
    let w = (quartz.size.width * scale).ceil();
    let h = (quartz.size.height * scale).ceil();
    let all = [x, y, w, h];
    if all
        .iter()
        .any(|v| !v.is_finite() || *v < 0.0 || *v > 65536.0)
    {
        return None;
    }
    // Whole numbers in 0..=65536, so the casts are exact.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let [x, y, w, h] = all.map(|v| v as u32);
    Some(format!("{w}:{h}:{x}:{y}"))
}

/// `r` grown by `margin` points on every side.
pub(super) fn grown(r: NSRect, margin: f64) -> NSRect {
    NSRect::new(
        NSPoint::new(r.origin.x - margin, r.origin.y - margin),
        objc2_foundation::NSSize::new(r.size.width + 2.0 * margin, r.size.height + 2.0 * margin),
    )
}

/// `view`'s frame in AppKit's screen points.
pub(super) fn screen_frame(view: &NSView) -> NSRect {
    match view.window() {
        Some(window) => window.convertRectToScreen(view.convertRect_toView(view.bounds(), None)),
        None => view.frame(),
    }
}

/// The centre of `view` in Quartz's global display points.
pub(super) fn quartz_centre(pass: &Pass<'_>, view: &NSView) -> NSPoint {
    let f = screen_frame(view);
    pass.quartz(NSPoint::new(
        f.origin.x + f.size.width / 2.0,
        f.origin.y + f.size.height / 2.0,
    ))
}

/// A point `(x, y)` points from `view`'s top left (y downwards), in Quartz's
/// global display points.
pub(super) fn quartz_at(pass: &Pass<'_>, view: &NSView, x: f64, y: f64) -> NSPoint {
    let f = screen_frame(view);
    pass.quartz(NSPoint::new(f.origin.x + x, f.origin.y + f.size.height - y))
}
