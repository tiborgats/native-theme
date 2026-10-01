//! `--screenshot`: the window with its frame, as the gpui and iced showcases capture theirs,
//! through xcap's safe API where they call the OS through `unsafe` FFI
//! (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:1031-1349`). On macOS it is
//! `screencapture -l <id> -o` of the window xcap's list finds, the siblings' own capture. On
//! Windows it is the window's visible frame, `DWMWA_EXTENDED_FRAME_BOUNDS` as xcap reports it
//! (`xcap-0.9.8/src/windows/utils.rs:241-254`), copied from the screen by
//! `Monitor::capture_region`, a `BitBlt` of the desktop (`xcap-0.9.8/src/windows/gdi.rs:83-122`),
//! as the siblings' `BitBlt` copies it. Elsewhere the capture scripts take the window with
//! spectacle, and `--screenshot` is refused: egui's own frame capture is the content alone.
//! Each capture passes `check_frame_capture` or the showcase exits non-zero.

/// Why `--screenshot` is refused where the showcase cannot capture its frame.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(crate) const UNSUPPORTED: &str = "--screenshot captures the window with its frame on macOS \
     and Windows only; on Linux, scripts/generate_screenshots_egui.sh captures it with spectacle";

/// The flag of the second run of the showcase that finds the first one's window on Windows,
/// followed by that run's process id.
#[cfg(target_os = "windows")]
const FIND_WINDOW_OF: &str = "--find-window-of";

/// A capture under way: on Windows, the second run finding the window.
pub(crate) struct FrameCapture {
    path: String,
    #[cfg(target_os = "windows")]
    finder: Option<std::process::Child>,
}

impl FrameCapture {
    /// Begin the capture to `path`. On Windows, xcap leaves the calling process's own windows
    /// out of its list (`xcap-0.9.8/src/windows/impl_window.rs:113-116`), so a second run of
    /// the showcase, `--find-window-of <pid>`, finds the window and prints its frame.
    #[cfg(target_os = "windows")]
    pub(crate) fn start(path: &str) -> Result<Self, String> {
        let exe = std::env::current_exe().map_err(|e| format!("the showcase's path: {e}"))?;
        let finder = std::process::Command::new(exe)
            .args([FIND_WINDOW_OF, &std::process::id().to_string()])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("starting the window finder: {e}"))?;
        Ok(Self {
            path: path.to_string(),
            finder: Some(finder),
        })
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn start(path: &str) -> Result<Self, String> {
        Ok(Self {
            path: path.to_string(),
        })
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    pub(crate) fn start(_path: &str) -> Result<Self, String> {
        Err(UNSUPPORTED.to_string())
    }

    /// `None` while the capture is under way; then whether it was written and passed
    /// `check_frame_capture`. A capture that fails the check is not left at the path.
    pub(crate) fn poll(&mut self, ctx: &egui::Context) -> Option<Result<(), String>> {
        #[cfg(target_os = "windows")]
        {
            let finder = self.finder.as_mut()?;
            match finder.try_wait() {
                Ok(None) => return None,
                Ok(Some(_)) => {}
                Err(e) => return Some(Err(format!("waiting for the window finder: {e}"))),
            }
            let output = self.finder.take()?.wait_with_output();
            Some(output.map_err(|e| e.to_string()).and_then(|output| {
                if !output.status.success() {
                    return Err(format!("the window finder exited with {}", output.status));
                }
                let frame = parse_frame(&String::from_utf8_lossy(&output.stdout))?;
                capture_windows(ctx, frame, &self.path)
            }))
        }
        #[cfg(target_os = "macos")]
        {
            Some(capture_macos(ctx, &self.path))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = (ctx, &self.path);
            Some(Err(UNSUPPORTED.to_string()))
        }
    }
}

/// The window's content in physical pixels and the display's scale factor, as egui has them.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn content_px(ctx: &egui::Context) -> Result<((i64, i64), Option<f32>), String> {
    let (inner, scale) = ctx.input(|i| {
        (
            i.viewport().inner_rect,
            i.viewport().native_pixels_per_point,
        )
    });
    let inner = inner.ok_or("the window's content rect is unknown")?;
    let scale_by = scale.ok_or("the display's scale factor is unknown")?;
    let px = |points: f32| (points * scale_by).round() as i64;
    Ok(((px(inner.width()), px(inner.height())), scale))
}

/// `check_frame_capture` of a capture `captured` pixels in size, reported with the frame.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn checked(ctx: &egui::Context, captured: (u32, u32)) -> Result<(), String> {
    let (content, scale) = content_px(ctx)?;
    eprintln!(
        "frame capture: {}x{} px around a {}x{} px content, scale {scale:?}",
        captured.0, captured.1, content.0, content.1
    );
    crate::check_frame_capture(
        (i64::from(captured.0), i64::from(captured.1)),
        content,
        scale,
    )
}

/// The one window of process `pid` titled `WINDOW_TITLE` in xcap's list.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn find_window(pid: u32) -> Result<xcap::Window, String> {
    let windows = xcap::Window::all().map_err(|e| format!("listing the windows: {e}"))?;
    let mut ours: Vec<xcap::Window> = windows
        .into_iter()
        .filter(|w| w.pid().is_ok_and(|p| p == pid))
        .filter(|w| w.title().is_ok_and(|t| t == crate::WINDOW_TITLE))
        .collect();
    match (ours.pop(), ours.is_empty()) {
        (Some(window), true) => Ok(window),
        (None, _) => Err(format!(
            "no window of process {pid} titled {:?} is listed",
            crate::WINDOW_TITLE
        )),
        (Some(_), false) => Err(format!(
            "{} windows of process {pid} are titled {:?}, not one",
            ours.len() + 1,
            crate::WINDOW_TITLE
        )),
    }
}

/// `screencapture -l <id> -o <path>` of the showcase's window: the window with its title bar,
/// without its shadow (`-o`), as the gpui showcase's `screencapture_own_window`
/// (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:1175-1198`); then checked.
#[cfg(target_os = "macos")]
fn capture_macos(ctx: &egui::Context, path: &str) -> Result<(), String> {
    let window = find_window(std::process::id())?;
    let id = window.id().map_err(|e| format!("the window's id: {e}"))?;
    let status = std::process::Command::new("screencapture")
        .args(["-l", &id.to_string(), "-o", path])
        .status()
        .map_err(|e| format!("running screencapture: {e}"))?;
    if !status.success() {
        return Err(format!("screencapture exited with {status}"));
    }
    let size = image::image_dimensions(path).map_err(|e| format!("{path}: {e}"))?;
    checked(ctx, size).inspect_err(|_| {
        let _ = std::fs::remove_file(path);
    })
}

/// A window's visible frame in physical screen pixels: left, top, width, height.
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

/// `--find-window-of <pid>`: print the frame of process `pid`'s showcase window and return the
/// exit code; `None` when the arguments are not this run's.
#[cfg(target_os = "windows")]
pub(crate) fn run_finder(args: &[String]) -> Option<i32> {
    let [flag, pid] = args else { return None };
    if flag != FIND_WINDOW_OF {
        return None;
    }
    let found = pid
        .parse::<u32>()
        .map_err(|e| format!("{FIND_WINDOW_OF} {pid}: {e}"))
        .and_then(find_window)
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

/// The window's visible `frame` copied from the monitor it is on, made opaque as the siblings'
/// capture is (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:1329`),
/// checked, and written to `path`. This process is per-monitor DPI aware, as winit makes every
/// event loop's by default (`winit-0.30.13/src/platform_impl/windows/event_loop.rs:173`,
/// `:198-200`; `dpi.rs:20-42`), so the copy is in the physical pixels the frame is given in.
#[cfg(target_os = "windows")]
fn capture_windows(ctx: &egui::Context, frame: Frame, path: &str) -> Result<(), String> {
    let centre = |start: i32, len: u32| start.saturating_add_unsigned(len / 2);
    let monitor =
        xcap::Monitor::from_point(centre(frame.x, frame.width), centre(frame.y, frame.height))
            .map_err(|e| format!("the window's monitor: {e}"))?;
    let origin = |e: xcap::XCapError| format!("the monitor's origin: {e}");
    let (mx, my) = (monitor.x().map_err(origin)?, monitor.y().map_err(origin)?);
    let offset = |at: i32, from: i32| {
        u32::try_from(i64::from(at) - i64::from(from))
            .map_err(|_| format!("the window's frame {frame:?} starts off its monitor"))
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
    checked(ctx, image.dimensions())?;
    image.save(path).map_err(|e| format!("{path}: {e}"))
}
