//! The focus ring's transition (`--button-pass --focus-transition`): the
//! push sheet's regular plain and default buttons focused and unfocused
//! while `screencapture -v` records the screen, so the frames show how the
//! ring comes and goes. The recording's frames are cut out by the workflow
//! (ffmpeg, at `<stem>.crop`); the probe records when it focused and
//! unfocused the button, counted from starting `screencapture`.

use std::cell::Cell;
use std::path::Path;
use std::time::{Duration, Instant};

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSApplication, NSBox, NSBoxType, NSColor, NSEventMask,
    NSResponder, NSScreen, NSTitlePosition, NSView,
};
use objc2_foundation::{NSDate, NSOperatingSystemVersion, NSPoint, NSRect, NSSize};
use toml::Table;

use super::{Pass, SETTLE_STEPS, away_point, push_sheet};
use crate::macos::{APPEARANCES, ffi, ns, rect};
use crate::out;

/// The buttons whose ring is recorded.
const TRANSITION_CASES: [&str; 2] = ["regular-plain", "regular-default"];

/// How long each recording lasts (`screencapture -V`), in seconds.
const RECORD_SECONDS: u32 = 10;

/// When the button is focused, counted from starting `screencapture`: time
/// for the recording to begin (about 1.5 s on macos-26 in run 41189405926).
const FOCUS_AT: Duration = Duration::from_secs(3);

/// When it is unfocused.
const UNFOCUS_AT: Duration = Duration::from_secs(6);

/// The latest the run loop waits for `screencapture` to end.
const FINISH_BY: Duration = Duration::from_secs(25);

/// One step of the run loop while recording: short, so the application
/// draws the ring's animation at the display's pace (the key-window pump's
/// steps of `PUMP_STEP` drew it once a step, about every 100 ms).
const FINE_STEP: Duration = Duration::from_millis(5);

/// How far round the button's frame the workflow's crop reaches, in points.
const CROP_MARGIN: f64 = 12.0;

/// The ticker's frame in the push sheet's content view: a small box in its
/// top right corner, far from the recorded buttons and outside their crops,
/// whose fill flips between black and white at every step of the run loop
/// while recording, so the screen changes all the time and the recording
/// keeps a frame wherever the screen does (run 41192705332's recordings,
/// with a still screen, kept one about every 100 ms while the ring moved).
const TICKER_FRAME: NSRect = NSRect::new(
    NSPoint::new(
        super::BUTTON_CONTENT_SIZE.width - 16.0,
        super::BUTTON_CONTENT_SIZE.height - 16.0,
    ),
    NSSize::new(8.0, 8.0),
);

/// The ticker box and how often it flipped.
struct Ticker {
    view: Retained<NSBox>,
    on: Cell<bool>,
    flips: Cell<u32>,
}

impl Ticker {
    fn new(pass: &Pass<'_>, content: &NSView) -> Self {
        let view = NSBox::initWithFrame(NSBox::alloc(pass.mtm), TICKER_FRAME);
        view.setBoxType(NSBoxType::Custom);
        view.setTitlePosition(NSTitlePosition::NoTitle);
        view.setBorderWidth(0.0);
        view.setFillColor(&NSColor::blackColor());
        content.addSubview(&view);
        Self {
            view,
            on: Cell::new(false),
            flips: Cell::new(0),
        }
    }

    /// Flip the fill.
    fn flip(&self) {
        let on = !self.on.get();
        self.on.set(on);
        let fill = if on {
            NSColor::whiteColor()
        } else {
            NSColor::blackColor()
        };
        self.view.setFillColor(&fill);
        self.flips.set(self.flips.get().saturating_add(1));
    }
}

/// `[macos.button_pass.focus_transition]`.
pub(super) fn transition_table(pass: &Pass<'_>, version: &NSOperatingSystemVersion) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    let sheet = match push_sheet(pass.mtm, version) {
        Ok(sheet) => sheet,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let window = &sheet.window;
    let ticker = Ticker::new(pass, &sheet.content);
    out::put(&mut t, "ticker_frame_xywh", rect(TICKER_FRAME));
    let other = pass.present(window, &mut t);
    let scale = NSScreen::mainScreen(pass.mtm).map(|s| s.backingScaleFactor());
    match scale {
        Some(scale) => out::put(&mut t, "mainScreen_backingScaleFactor", out::float(scale)),
        None => out::put(
            &mut t,
            "mainScreen_backingScaleFactor",
            out::unavailable("mainScreen is nil"),
        ),
    }
    out::put(&mut t, "window_frame_xywh", rect(window.frame()));
    out::put_table(
        &mut t,
        "cursor",
        pass.move_cursor(window, away_point(window)),
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
        sheet.content.setNeedsDisplay(true);
        pass.settle(window, SETTLE_STEPS);
        let dir = pass.out_dir.join(key);
        let mut per = Table::new();
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut per,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            out::put_table(&mut t, key, per);
            continue;
        }
        for case in sheet
            .cases
            .iter()
            .filter(|c| TRANSITION_CASES.contains(&c.name.as_str()))
        {
            let responder: &NSResponder = case.control.as_ref();
            let frame = window
                .convertRectToScreen(case.control.convertRect_toView(case.control.bounds(), None));
            let r = record(
                pass,
                &sheet.window,
                &ticker,
                responder,
                frame,
                scale,
                &dir,
                &format!("{key}/transition-{}", case.name),
            );
            out::put_table(&mut per, &case.name, r);
        }
        out::put_table(&mut t, key, per);
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

/// One recording: start `screencapture -v`, focus `responder` at
/// `FOCUS_AT`, unfocus it at `UNFOCUS_AT`, and wait for the recording to
/// end; `stem` is the files' path from the output directory.
#[allow(clippy::too_many_arguments)]
fn record(
    pass: &Pass<'_>,
    window: &Retained<objc2_app_kit::NSWindow>,
    ticker: &Ticker,
    responder: &NSResponder,
    frame: NSRect,
    scale: Option<f64>,
    dir: &Path,
    stem: &str,
) -> Table {
    let mut t = Table::new();
    let Some(file_stem) = Path::new(stem).file_name() else {
        out::put(
            &mut t,
            "error",
            out::unavailable(format!("no file name in {stem}")),
        );
        return t;
    };
    let file_stem = file_stem.to_string_lossy().into_owned();
    out::put(
        &mut t,
        "makeFirstResponder_nil",
        out::boolean(window.makeFirstResponder(None)),
    );
    pass.settle(window, SETTLE_STEPS);
    out::put(&mut t, "frame_screen_xywh", rect(frame));
    let quartz = NSRect::new(
        pass.quartz(objc2_foundation::NSPoint::new(
            frame.origin.x,
            frame.origin.y + frame.size.height,
        )),
        frame.size,
    );
    out::put(&mut t, "frame_quartz_xywh", rect(quartz));
    let crop = scale.and_then(|s| crop_text(quartz, s));
    match &crop {
        Some(text) => {
            out::put(&mut t, "crop_px_whxy", out::text(text.clone()));
            let path = dir.join(format!("{file_stem}.crop"));
            if let Err(e) = std::fs::write(&path, text) {
                out::put(
                    &mut t,
                    "crop_file",
                    out::unavailable(format!("cannot write {}: {e}", path.display())),
                );
            } else {
                out::put(&mut t, "crop_file", out::text(format!("{stem}.crop")));
            }
        }
        None => out::put(
            &mut t,
            "crop_px_whxy",
            out::unavailable("no backing scale, or the crop lies off the screen"),
        ),
    }

    let path = dir.join(format!("{file_stem}.mov"));
    let start = Instant::now();
    let child = std::process::Command::new("screencapture")
        .arg("-x")
        .arg("-v")
        .arg(format!("-V{RECORD_SECONDS}"))
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
    let flips_before = ticker.flips.get();
    let lead = pump_until(pass.app, start, FOCUS_AT, || {
        ticker.flip();
        false
    });
    let focus_at = start.elapsed();
    let made = window.makeFirstResponder(Some(responder));
    let focus_done = start.elapsed();
    out::put(&mut t, "makeFirstResponder", out::boolean(made));
    let first = window
        .firstResponder()
        .is_some_and(|r| std::ptr::eq(&*r, responder));
    out::put(&mut t, "firstResponder_is_the_control", out::boolean(first));
    out::put(
        &mut t,
        "isKeyWindow_focused",
        out::boolean(window.isKeyWindow()),
    );
    let hold = pump_until(pass.app, start, UNFOCUS_AT, || {
        ticker.flip();
        false
    });
    let unfocus_at = start.elapsed();
    let unmade = window.makeFirstResponder(None);
    let unfocus_done = start.elapsed();
    out::put(&mut t, "makeFirstResponder_nil_after", out::boolean(unmade));
    let status: Cell<Option<Result<std::process::ExitStatus, String>>> = Cell::new(None);
    let finish = pump_until(pass.app, start, FINISH_BY, || {
        ticker.flip();
        match child.try_wait() {
            Ok(Some(s)) => {
                status.set(Some(Ok(s)));
                true
            }
            Ok(None) => false,
            Err(e) => {
                status.set(Some(Err(format!("could not be waited for: {e}"))));
                true
            }
        }
    });
    out::put(
        &mut t,
        "ticker_flips",
        out::int(i64::from(ticker.flips.get().saturating_sub(flips_before))),
    );
    let ended_at = start.elapsed();
    let outcome = match status.into_inner() {
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
                FINISH_BY.as_secs()
            ))
        }
    };
    for (name, steps) in [("lead", lead), ("hold", hold), ("finish", finish)] {
        out::put_table(&mut t, &format!("pump_{name}"), steps);
    }
    out::put(&mut t, "recording", outcome);
    out::put(&mut t, "focus_at_ms", ms(focus_at));
    out::put(&mut t, "focus_returned_ms", ms(focus_done));
    out::put(&mut t, "unfocus_at_ms", ms(unfocus_at));
    out::put(&mut t, "unfocus_returned_ms", ms(unfocus_done));
    out::put(&mut t, "recording_ended_ms", ms(ended_at));
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

/// Pump the run loop in steps of `FINE_STEP` until `deadline`, counted from
/// `start`, has passed or `done` returns true (it runs before each step):
/// the steps taken and the longest time between the starts of two.
fn pump_until(
    app: &NSApplication,
    start: Instant,
    deadline: Duration,
    mut done: impl FnMut() -> bool,
) -> Table {
    let mode = ffi::default_run_loop_mode();
    let mut steps: u32 = 0;
    let mut longest = Duration::ZERO;
    let mut last = Instant::now();
    while start.elapsed() < deadline && !done() {
        let now = Instant::now();
        longest = longest.max(now.duration_since(last));
        last = now;
        let until = NSDate::dateWithTimeIntervalSinceNow(FINE_STEP.as_secs_f64());
        while let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
            NSEventMask::Any,
            Some(&until),
            mode,
            true,
        ) {
            app.sendEvent(&event);
        }
        app.updateWindows();
        steps = steps.saturating_add(1);
    }
    let mut t = Table::new();
    out::put(&mut t, "steps", out::int(i64::from(steps)));
    out::put(&mut t, "longest_step_ms", ms(longest));
    out::put(&mut t, "until_ms", ms(start.elapsed()));
    t
}

/// A duration as whole milliseconds, a float.
fn ms(d: Duration) -> toml::Value {
    out::float(d.as_secs_f64() * 1000.0)
}

/// ffmpeg's `crop=` argument, `w:h:x:y` in pixels, for `quartz` (global
/// display points, top-left origin) grown by `CROP_MARGIN` at `scale`
/// pixels per point; `None` where it would start off the screen's top left.
fn crop_text(quartz: NSRect, scale: f64) -> Option<String> {
    let x = ((quartz.origin.x - CROP_MARGIN) * scale).floor();
    let y = ((quartz.origin.y - CROP_MARGIN) * scale).floor();
    let w = ((quartz.size.width + 2.0 * CROP_MARGIN) * scale).ceil();
    let h = ((quartz.size.height + 2.0 * CROP_MARGIN) * scale).ceil();
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

fn method_text() -> String {
    format!(
        "the push sheet's window presented as in the key mode (made key, the application active) and the cursor parked {} pt right of it; per appearance (the window's appearance set, the run loop pumped {SETTLE_STEPS} steps) and per button ({}): makeFirstResponder: nil and the run loop pumped {SETTLE_STEPS} steps; `screencapture -x -v -V{RECORD_SECONDS} <appearance>/transition-<button>.mov` started (the whole main display); the run loop pumped in steps of {} ms, before each the ticker (an NSBox, custom, no border, no title, at {} in the content view, outside every crop) filled black or white in turn, so the screen changes at every step (ticker_flips) (pump_lead: the steps and the longest between two starts) until {} ms after starting screencapture; makeFirstResponder: with the button (focus_at_ms, counted from starting screencapture); pumped until {} ms (pump_hold); makeFirstResponder: nil (unfocus_at_ms); pumped until screencapture exits (pump_finish, at most until {} ms); <appearance>/transition-<button>.crop holds ffmpeg's crop=w:h:x:y in pixels for the button's frame grown by {CROP_MARGIN} pt, in the display's pixels (frame_quartz_xywh times mainScreen_backingScaleFactor), which the workflow cuts each frame of the recording to (<stem>-frames/NNNNN.png, their times in <stem>-frames.csv, ffprobe's pts_time); the recording starts some time after screencapture does, so its times are not the probe's",
        super::CURSOR_AWAY_OFFSET,
        TRANSITION_CASES.join(", "),
        FINE_STEP.as_millis(),
        crate::macos::rect_text(TICKER_FRAME),
        FOCUS_AT.as_millis(),
        UNFOCUS_AT.as_millis(),
        FINISH_BY.as_millis(),
    )
}
