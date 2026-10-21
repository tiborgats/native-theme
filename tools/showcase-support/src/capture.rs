//! The window capture: the app id a captured window opens under
//! ([`app_id`]), the check that a capture is the window at its default size
//! with its frame ([`check_frame_capture`]), and `--screenshot`'s own
//! capture of the window with its frame ([`FrameCapture`]), through xcap's
//! safe API.
//!
//! On macOS the capture is `screencapture -l <id> -o` of the window xcap's
//! list finds: the window with its title bar, without its shadow (`-o`). On
//! Windows it is the window's visible frame, `DWMWA_EXTENDED_FRAME_BOUNDS`
//! as xcap reports it (`xcap-0.9.8/src/windows/utils.rs`), copied from the
//! screen by `Monitor::capture_region`, a `BitBlt` of the desktop
//! (`xcap-0.9.8/src/windows/gdi.rs`), and made opaque. Elsewhere the capture
//! scripts take the window with spectacle, and `--screenshot` is refused
//! ([`crate::cli::screenshot_support`]): a toolkit's own frame capture is
//! its content alone. Each capture passes [`check_frame_capture`] or is not
//! left at its path, and the showcase exits non-zero.
//!
//! What differs from a `BitBlt` of the window's frame through the Win32 API
//! itself, which needs `unsafe` FFI:
//!
//! - xcap's `BitBlt` is `SRCCOPY` alone, without `CAPTUREBLT`, so a layered
//!   window over the showcase's is not copied with it;
//! - the copy is of the one monitor at the frame's centre, so a window across
//!   two monitors is clipped at that monitor's edge, which the frame check
//!   then fails;
//! - xcap leaves the calling process's own windows out of its list on Windows
//!   (`xcap-0.9.8/src/windows/impl_window.rs:113-116`), so a second run of
//!   the showcase, `--find-window-of <pid>` ([`run_finder`]), finds the window
//!   and prints its frame.
//!
//! The copy is in the physical pixels the frame is given in only where the
//! process is per-monitor DPI aware, which each toolkit build settles:
//!
//! - egui: winit makes every event loop's process per-monitor aware by
//!   default (`winit-0.30.13/src/platform_impl/windows/event_loop.rs`,
//!   `dpi.rs`);
//! - gpui on gpui-pre (its default, which `screenshots.yml` builds): gpui-pre
//!   embeds a PerMonitorV2 manifest under its `windows-manifest` feature
//!   (gpui-pre 0.3.9 `build.rs:22-36`,
//!   `resources/windows/gpui.manifest.xml:17-18`);
//! - gpui on GPUI Fast: none is embedded, as gpui-base and gpui-component
//!   0.7.3 turn on `windows-manifest-provided-by-host` (gpui-fast 0.1.7
//!   `build.rs:37-38`), so its DPI awareness is whatever the host provides.
//!
//! `nudge_content_size`, on macOS, is gpui's alone: gpui-pre's Metal
//! drawable on a Retina display.

use crate::{Kind, WINDOW_SIZE_PX};

/// The app id of a captured window (`--capture`, `--screenshot`):
/// `showcase-<name>-capture-<pid>`, this process's own, so no geometry a
/// desktop stored for an earlier window applies to it (a KWin script that
/// remembers window geometry by app id, say). `scripts/capture_window.sh`
/// finds the window by it.
#[must_use]
pub fn app_id(kind: Kind) -> String {
    format!("showcase-{}-capture-{}", kind.name(), std::process::id())
}

/// Whether an OS capture of the window with its frame, `captured` pixels, is
/// the frame of a window whose content is [`WINDOW_SIZE_PX`] at the
/// display's scale factor `scale`, rounded as winit and gpui round a logical
/// size to a physical one: the capture less the content the window has now,
/// `content` pixels, is the frame, and around a `WINDOW_SIZE_PX` content it
/// makes the size the capture must be. Any other size is a window that did
/// not open at its default size (a display too small for it, or a size
/// restored from elsewhere); a capture no taller than the content has no
/// title bar, and fails too. A toolkit whose scale factor is unknown refuses
/// before it calls this, with its own message.
pub fn check_frame_capture(
    captured: (i64, i64),
    content: (i64, i64),
    scale: f64,
) -> Result<(), String> {
    if captured.1 <= content.1 || captured.0 < content.0 {
        return Err(format!(
            "the capture is {}x{} px around a {}x{} px content: it is not the window with its \
             frame and title bar",
            captured.0, captured.1, content.0, content.1
        ));
    }
    let default = (
        (f64::from(WINDOW_SIZE_PX.0) * scale).round() as i64,
        (f64::from(WINDOW_SIZE_PX.1) * scale).round() as i64,
    );
    let frame = (
        captured.0.saturating_sub(content.0),
        captured.1.saturating_sub(content.1),
    );
    let expected = (
        default.0.saturating_add(frame.0),
        default.1.saturating_add(frame.1),
    );
    if captured == expected {
        Ok(())
    } else {
        Err(format!(
            "the capture is {}x{} px, expected {}x{}: a {}x{} content area ({}x{} at scale \
             {scale}) in the frame's {}x{} px, but the content is {}x{} px, so the window did \
             not open at its default size",
            captured.0,
            captured.1,
            expected.0,
            expected.1,
            default.0,
            default.1,
            WINDOW_SIZE_PX.0,
            WINDOW_SIZE_PX.1,
            frame.0,
            frame.1,
            content.0,
            content.1,
        ))
    }
}

/// The flag of the second run of the showcase that finds the first one's
/// window on Windows, followed by that run's process id.
#[cfg(target_os = "windows")]
const FIND_WINDOW_OF: &str = "--find-window-of";

/// How long [`FrameCapture::poll`] waits for the second run that finds the
/// window on Windows before it gives the capture up: 20 s, the time the
/// capture scripts wait for a showcase's window (`scripts/capture_window.sh`,
/// `SHOWCASE_WINDOW_TIMEOUT`), which the showcases' layout dumps wait too.
/// The crate's own bound; the model states none.
pub const FIND_WINDOW_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(20);

/// Whether the window finder `finder`, started at `started`, has exited by
/// `now`: `Ok(true)` once it has, `Ok(false)` while it runs within
/// [`FIND_WINDOW_TIMEOUT`], and an error once it has run that long, the
/// finder killed, so a finder that never finds the window cannot keep the
/// showcase open.
#[cfg(any(target_os = "windows", test))]
fn finder_exited(
    finder: &mut std::process::Child,
    started: std::time::Instant,
    now: std::time::Instant,
) -> Result<bool, String> {
    match finder.try_wait() {
        Ok(Some(_)) => Ok(true),
        Ok(None) if now.saturating_duration_since(started) < FIND_WINDOW_TIMEOUT => Ok(false),
        Ok(None) => {
            let _ = finder.kill();
            let _ = finder.wait();
            Err(format!(
                "the window finder found no window within {} s",
                FIND_WINDOW_TIMEOUT.as_secs()
            ))
        }
        Err(e) => Err(format!("waiting for the window finder: {e}")),
    }
}

/// The window's content in physical pixels and the display's scale factor,
/// as the toolkit has them when [`FrameCapture::poll`] asks.
pub type Content = ((i64, i64), f64);

/// `--screenshot`'s capture under way: on Windows, the second run finding
/// the window.
pub struct FrameCapture {
    path: String,
    #[cfg(target_os = "macos")]
    title: String,
    #[cfg(target_os = "windows")]
    finder: Option<std::process::Child>,
    /// When the finder run was started ([`FIND_WINDOW_TIMEOUT`]).
    #[cfg(target_os = "windows")]
    started: std::time::Instant,
}

impl FrameCapture {
    /// Begin the capture, to `path`, of this process's window titled
    /// `window_title`. On Windows it starts the second run that finds the
    /// window, which takes the title its own `main` hands [`run_finder`];
    /// elsewhere but macOS it is refused.
    pub fn start(window_title: &str, path: &str) -> Result<Self, String> {
        #[cfg(target_os = "windows")]
        {
            let exe = std::env::current_exe().map_err(|e| format!("the showcase's path: {e}"))?;
            let finder = std::process::Command::new(exe)
                .args([FIND_WINDOW_OF, &std::process::id().to_string()])
                .stdout(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| format!("starting the window finder: {e}"))?;
            let _ = window_title;
            Ok(Self {
                path: path.to_string(),
                finder: Some(finder),
                started: std::time::Instant::now(),
            })
        }
        #[cfg(target_os = "macos")]
        {
            Ok(Self {
                path: path.to_string(),
                title: window_title.to_string(),
            })
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = (window_title, path);
            Err(UNSUPPORTED.to_string())
        }
    }

    /// `None` while the capture is under way; then, once, whether it was
    /// written and passed [`check_frame_capture`], with the content size and
    /// scale `content` gives at that moment. A capture that fails the check
    /// is not left at the path, and on Windows a finder run that has not
    /// found the window within [`FIND_WINDOW_TIMEOUT`] fails it. Poll until it
    /// returns `Some`, then stop: on Windows a later poll returns `None`
    /// again, as the finder run is gone.
    pub fn poll(
        &mut self,
        content: impl FnOnce() -> Result<Content, String>,
    ) -> Option<Result<(), String>> {
        #[cfg(target_os = "windows")]
        {
            let finder = self.finder.as_mut()?;
            match finder_exited(finder, self.started, std::time::Instant::now()) {
                Ok(false) => return None,
                Ok(true) => {}
                Err(error) => {
                    self.finder = None;
                    return Some(Err(error));
                }
            }
            let output = self.finder.take()?.wait_with_output();
            Some(output.map_err(|e| e.to_string()).and_then(|output| {
                if !output.status.success() {
                    return Err(format!("the window finder exited with {}", output.status));
                }
                let frame = parse_frame(&String::from_utf8_lossy(&output.stdout))?;
                capture_windows(frame, &self.path, content)
            }))
        }
        #[cfg(target_os = "macos")]
        {
            Some(capture_macos(&self.title, &self.path, content))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = (content, &self.path);
            Some(Err(UNSUPPORTED.to_string()))
        }
    }
}

/// Why [`FrameCapture::start`] refuses where the showcase cannot capture its
/// frame; [`crate::cli::screenshot_support`] says it before the window
/// opens, with the script that captures it.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const UNSUPPORTED: &str =
    "--screenshot captures the window with its frame on macOS and Windows only";

/// [`check_frame_capture`] of a capture `captured` pixels in size, around
/// the content `content` gives, reported with the frame.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn checked(
    captured: (u32, u32),
    content: impl FnOnce() -> Result<Content, String>,
) -> Result<(), String> {
    let (content, scale) = content()?;
    eprintln!(
        "frame capture: {}x{} px around a {}x{} px content, scale {scale}",
        captured.0, captured.1, content.0, content.1
    );
    check_frame_capture(
        (i64::from(captured.0), i64::from(captured.1)),
        content,
        scale,
    )
}

/// The one window of process `pid` titled `title` in xcap's list.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn find_window(pid: u32, title: &str) -> Result<xcap::Window, String> {
    let windows = xcap::Window::all().map_err(|e| format!("listing the windows: {e}"))?;
    let mut ours: Vec<xcap::Window> = windows
        .into_iter()
        .filter(|w| w.pid().is_ok_and(|p| p == pid))
        .filter(|w| w.title().is_ok_and(|t| t == title))
        .collect();
    let count = ours.len();
    match (ours.pop(), count) {
        (Some(window), 1) => Ok(window),
        (None, _) => Err(format!(
            "no window of process {pid} titled {title:?} is listed"
        )),
        (Some(_), count) => Err(format!(
            "{count} windows of process {pid} are titled {title:?}, not one"
        )),
    }
}

/// `screencapture -l <id> -o <path>` of the window titled `title`: the
/// window with its title bar, without its shadow (`-o`); then checked, and
/// removed where the check fails.
#[cfg(target_os = "macos")]
fn capture_macos(
    title: &str,
    path: &str,
    content: impl FnOnce() -> Result<Content, String>,
) -> Result<(), String> {
    let window = find_window(std::process::id(), title)?;
    let id = window.id().map_err(|e| format!("the window's id: {e}"))?;
    let status = std::process::Command::new("screencapture")
        .args(["-l", &id.to_string(), "-o", path])
        .status()
        .map_err(|e| format!("running screencapture: {e}"))?;
    if !status.success() {
        return Err(format!("screencapture exited with {status}"));
    }
    let size = image::image_dimensions(path).map_err(|e| format!("{path}: {e}"))?;
    checked(size, content).inspect_err(|_| {
        let _ = std::fs::remove_file(path);
    })
}

/// A window's visible frame in physical screen pixels: left, top, width,
/// height.
#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Frame {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// What the window finder prints: `x y width height`.
#[cfg(any(target_os = "windows", test))]
pub(crate) fn parse_frame(line: &str) -> Result<Frame, String> {
    let bad = || format!("the window finder printed {line:?}, not `x y width height`");
    let mut fields = line.split_whitespace();
    let mut next = || fields.next().ok_or_else(bad);
    let frame = Frame {
        x: next()?.parse().map_err(|_| bad())?,
        y: next()?.parse().map_err(|_| bad())?,
        width: next()?.parse().map_err(|_| bad())?,
        height: next()?.parse().map_err(|_| bad())?,
    };
    match fields.next() {
        None => Ok(frame),
        Some(_) => Err(bad()),
    }
}

/// The second run [`FrameCapture::start`] starts on Windows,
/// `--find-window-of <pid>`: prints the frame of process `pid`'s window
/// titled `window_title` and returns the exit code. `None` where `args`, the
/// arguments after the program name, are not this run's, and always off
/// Windows. A showcase calls it first in `main`, before its window opens, so
/// the finder run opens none.
#[must_use]
pub fn run_finder(args: &[String], window_title: &str) -> Option<i32> {
    #[cfg(target_os = "windows")]
    {
        let [flag, pid] = args else { return None };
        if flag != FIND_WINDOW_OF {
            return None;
        }
        let found = pid
            .parse::<u32>()
            .map_err(|e| format!("{FIND_WINDOW_OF} {pid}: {e}"))
            .and_then(|pid| find_window(pid, window_title))
            .and_then(|w| {
                let frame = |e: xcap::XCapError| format!("the window's frame: {e}");
                Ok(Frame {
                    x: w.x().map_err(frame)?,
                    y: w.y().map_err(frame)?,
                    width: w.width().map_err(frame)?,
                    height: w.height().map_err(frame)?,
                })
            });
        Some(match found {
            Ok(Frame {
                x,
                y,
                width,
                height,
            }) => {
                println!("{x} {y} {width} {height}");
                0
            }
            Err(error) => {
                eprintln!("ERROR: {error}");
                1
            }
        })
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (args, window_title);
        None
    }
}

/// The window's visible `frame` copied from the monitor at its centre, made
/// opaque, checked, and written to `path` only where the check passes.
#[cfg(target_os = "windows")]
fn capture_windows(
    frame: Frame,
    path: &str,
    content: impl FnOnce() -> Result<Content, String>,
) -> Result<(), String> {
    let centre = |start: i32, len: u32| start.saturating_add_unsigned(len.div_euclid(2));
    let monitor =
        xcap::Monitor::from_point(centre(frame.x, frame.width), centre(frame.y, frame.height))
            .map_err(|e| format!("the window's monitor: {e}"))?;
    let origin = |e: xcap::XCapError| format!("the monitor's origin: {e}");
    let (mx, my) = (monitor.x().map_err(origin)?, monitor.y().map_err(origin)?);
    let offset = |at: i32, from: i32| {
        i64::from(at)
            .checked_sub(i64::from(from))
            .and_then(|offset| u32::try_from(offset).ok())
            .ok_or_else(|| format!("the window's frame {frame:?} starts off its monitor"))
    };
    let mut image = monitor
        .capture_region(
            offset(frame.x, mx)?,
            offset(frame.y, my)?,
            frame.width,
            frame.height,
        )
        .map_err(|e| format!("capturing the window's frame: {e}"))?;
    for pixel in image.pixels_mut() {
        let [.., alpha] = &mut pixel.0;
        *alpha = u8::MAX;
    }
    checked(image.dimensions(), content)?;
    image.save(path).map_err(|e| format!("{path}: {e}"))
}

/// Forces gpui's Metal drawable to its Retina size by nudging the content
/// size of the application's window -- its main window, else its key
/// window, else its first -- by `delta_w` × `delta_h` points; called with a
/// nudge and then with its opposite. Nothing happens off the main thread or
/// where the application has no window.
///
/// gpui's alone (the other toolkits need none): gpui-pre initialises the
/// Metal drawable at logical-pixel dimensions, ignoring the Retina backing
/// scale factor, and sets the device-pixel size only inside the
/// `setFrameSize:` callback, which returns early when the size is
/// unchanged; a 1 px nudge and its restore force two real resizes, so the
/// drawable takes the right scale. `-[NSWindow setContentSize:]` is called
/// directly because gpui's `Window::resize` runs in an async task that may
/// not run before the capture. gpui calls it outside `update_window`, since
/// `setFrameSize:` takes the window-state lock gpui holds there.
#[cfg(target_os = "macos")]
pub fn nudge_content_size(delta_w: f64, delta_h: f64) {
    let Some(mtm) = objc2::MainThreadMarker::new() else {
        return;
    };
    let app = objc2_app_kit::NSApplication::sharedApplication(mtm);
    let window = app
        .mainWindow()
        .or_else(|| app.keyWindow())
        .or_else(|| app.windows().firstObject());
    let Some(window) = window else {
        return;
    };
    let Some(view) = window.contentView() else {
        return;
    };
    let mut size = view.frame().size;
    size.width += delta_w;
    size.height += delta_h;
    window.setContentSize(size);
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// A captured window's app id names the showcase and this process, as
    /// `scripts/capture_window.sh` looks for it.
    #[test]
    fn a_capture_takes_an_app_id_of_its_own() {
        for kind in [Kind::Gpui, Kind::Iced, Kind::Egui] {
            assert_eq!(
                app_id(kind),
                format!("showcase-{}-capture-{}", kind.name(), std::process::id())
            );
        }
    }

    /// An OS capture of the window passes only when it is the window with
    /// its frame and title bar around a content of `WINDOW_SIZE_PX` times the
    /// display's scale factor.
    #[test]
    fn a_frame_capture_of_another_size_fails() {
        // A frame 2px wider and 32px taller than the content, as the Windows
        // runner's captures measured at the old default size (1182 x 882
        // around a 1180 x 850 content), and a 28px title bar alone.
        assert_eq!(check_frame_capture((1282, 752), (1280, 720), 1.), Ok(()));
        assert_eq!(check_frame_capture((1600, 928), (1600, 900), 1.25), Ok(()));
        assert_eq!(check_frame_capture((2560, 1496), (2560, 1440), 2.), Ok(()));
        // A 1024px-wide display clamped the window.
        let clamped = check_frame_capture((1024, 674), (1024, 646), 1.);
        assert!(
            clamped
                .as_ref()
                .is_err_and(|e| e.contains("1024x674") && e.contains("1280x748")),
            "{clamped:?}"
        );
        assert!(check_frame_capture((1282, 752), (1280, 720), 2.).is_err());
        // A toolkit's own frame capture, the content alone: never passed as
        // the window.
        let frameless = check_frame_capture((1280, 720), (1280, 720), 1.);
        assert!(
            frameless.as_ref().is_err_and(|e| e.contains("title bar")),
            "{frameless:?}"
        );
        assert!(check_frame_capture((1278, 752), (1280, 720), 1.).is_err());
    }

    /// The Windows window finder's line, `x y width height`, and nothing
    /// else.
    #[test]
    fn the_window_finders_line_is_four_numbers() {
        assert_eq!(
            parse_frame("-8 0 1282 752\n"),
            Ok(Frame {
                x: -8,
                y: 0,
                width: 1282,
                height: 752
            })
        );
        for bad in ["", "1 2 3", "1 2 3 4 5", "1 2 -3 4", "a 2 3 4"] {
            assert!(parse_frame(bad).is_err(), "{bad:?} parsed");
        }
    }

    /// The finder run answers only to its own arguments, and off Windows to
    /// none.
    #[test]
    fn the_finder_run_takes_only_its_own_arguments() {
        let args =
            |argv: &[&str]| -> Vec<String> { argv.iter().map(|a| (*a).to_string()).collect() };
        assert_eq!(run_finder(&args(&[]), "title"), None);
        assert_eq!(run_finder(&args(&["--theme", "adwaita"]), "title"), None);
        if !cfg!(target_os = "windows") {
            assert_eq!(run_finder(&args(&["--find-window-of", "1"]), "title"), None);
        }
    }

    /// A window finder that runs past [`FIND_WINDOW_TIMEOUT`] is killed and
    /// fails the capture, one that exits ends the wait, and the bound is the
    /// time `scripts/capture_window.sh` waits for a window.
    #[cfg(unix)]
    #[test]
    fn a_window_finder_that_never_finds_is_given_up() {
        let started = std::time::Instant::now();
        let mut hung = std::process::Command::new("sleep")
            .arg("60")
            .spawn()
            .unwrap();
        assert_eq!(finder_exited(&mut hung, started, started), Ok(false));
        let overdue = finder_exited(&mut hung, started, started + FIND_WINDOW_TIMEOUT);
        assert_eq!(
            overdue,
            Err(format!(
                "the window finder found no window within {} s",
                FIND_WINDOW_TIMEOUT.as_secs()
            ))
        );
        assert!(
            hung.try_wait().unwrap().is_some(),
            "the finder was not killed"
        );

        let mut done = std::process::Command::new("true").spawn().unwrap();
        let _ = done.wait();
        assert_eq!(
            finder_exited(&mut done, started, started + FIND_WINDOW_TIMEOUT),
            Ok(true)
        );

        let script = include_str!("../../../scripts/capture_window.sh");
        let timeout = script
            .lines()
            .find_map(|line| line.strip_prefix("SHOWCASE_WINDOW_TIMEOUT="))
            .and_then(|seconds| seconds.trim().parse::<u64>().ok());
        assert_eq!(timeout, Some(FIND_WINDOW_TIMEOUT.as_secs()));
    }

    /// Where the frame cannot be captured, the capture does not start.
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[test]
    fn the_capture_is_refused_where_the_frame_cannot_be_taken() {
        assert_eq!(
            FrameCapture::start("title", "x.png").err().as_deref(),
            Some(UNSUPPORTED)
        );
    }
}
