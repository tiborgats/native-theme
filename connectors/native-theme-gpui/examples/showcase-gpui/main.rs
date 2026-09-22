//! native-theme-gpui — comprehensive widget showcase and designer reference.
//!
//! A full gpui-component widget gallery with tooltip-based documentation for
//! every theme-controlled property. Demonstrates all gpui-component widgets,
//! every `ThemeColor` field, every `IconName` variant, and live theme
//! switching across all bundled `native-theme` presets.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase-gpui
//! ```
//!
//! # What to look for
//!
//! - Sidebar on the left switches theme presets, color modes, and icon sets
//!   without restarting the app. Watch how the entire widget tree re-themes
//!   on each change — no manual rewiring per widget.
//! - Hover any widget to see tooltips explaining which `ResolvedTheme` fields
//!   drive its appearance.
//! - The Color Map tab exposes the full 138-field `ThemeColor` palette that
//!   gpui-component exposes, with each field's current value and the
//!   `native-theme` field it was derived from.
//! - The Icons tab demonstrates `IconRole` mapping across Material, Lucide,
//!   and freedesktop sets, plus animated spinner playback.
//!
//! # How the source is organised
//!
//! `main.rs` holds the entry point, the command line and the screenshot
//! capture; `app.rs` the `Showcase` view, its state and theme switching;
//! `pages/` one module per tab; `inspector.rs` the Widget Info panel; and
//! `support.rs` the sample content, helpers, icon loading and delegates
//! the pages share. Within a file, section-divider blocks (`// ─────`)
//! separate one widget category, tab or view from the next.

mod app;
mod chrome;
mod demo;
mod info;
mod inspector;
mod pages;
mod support;

use gpui::{
    App, Bounds, Div, IntoElement, ParentElement, Pixels, SharedString, WindowBounds,
    WindowDecorations, WindowOptions, div, prelude::*, px, size,
};
use gpui_component::{ActiveTheme, Root, TitleBar, select::SearchableVec};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use {gpui::Window, std::time::Duration};

use native_theme::icons::IconSetChoice;

use crate::app::{AppColorMode, Showcase};
use crate::support::{load_all_icons, load_gpui_icons};

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

/// The content area's tabs.
///
/// `Showcase::render` matches on this, so a new variant cannot be added without
/// the compiler asking what it renders, and the bar's labels, the `--tab` names
/// and the layout self-test all read [`Tab::ALL`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Tab {
    Buttons,
    Inputs,
    Data,
    Feedback,
    Typography,
    Layout,
    Overlays,
    Charts,
    Icons,
    ThemeMap,
}

impl Tab {
    /// Every tab, in the order the bar shows them.
    ///
    /// A new variant forces an arm in [`Tab::index`] and [`Tab::label`], whose
    /// matches are exhaustive, and the index it is given there has to be its
    /// position in this array — the `const` block below rejects the build
    /// otherwise. The one thing neither the compiler nor that block can see is
    /// a variant added to the enum and to both matches but not to this list:
    /// it would take an index the array does not have, and the assertion fires.
    const ALL: [Self; 10] = [
        Self::Buttons,
        Self::Inputs,
        Self::Data,
        Self::Feedback,
        Self::Typography,
        Self::Layout,
        Self::Overlays,
        Self::Charts,
        Self::Icons,
        Self::ThemeMap,
    ];

    /// The tab's position in the bar, which is what `TabBar` counts in.
    const fn index(self) -> usize {
        match self {
            Self::Buttons => 0,
            Self::Inputs => 1,
            Self::Data => 2,
            Self::Feedback => 3,
            Self::Typography => 4,
            Self::Layout => 5,
            Self::Overlays => 6,
            Self::Charts => 7,
            Self::Icons => 8,
            Self::ThemeMap => 9,
        }
    }

    /// The tab at a bar position; `None` past the end.
    fn at(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// The label the bar shows.
    const fn label(self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Inputs => "Inputs",
            Self::Data => "Data",
            Self::Feedback => "Feedback",
            Self::Typography => "Typography",
            Self::Layout => "Layout",
            Self::Overlays => "Overlays",
            Self::Charts => "Charts",
            Self::Icons => "Icons",
            Self::ThemeMap => "Theme Map",
        }
    }
}

/// `Tab::ALL` is in bar order and holds each tab once.
const _: () = {
    let mut i = 0;
    while i < Tab::ALL.len() {
        assert!(
            Tab::ALL[i].index() == i,
            "Tab::ALL is not the tabs in bar order"
        );
        i += 1;
    }
};

/// The window the showcase opens. The self-tests lay the interface out at this
/// width, so a measurement they take is a measurement of the real thing.
pub(crate) const WINDOW_SIZE: gpui::Size<Pixels> = size(px(1100.), px(850.));

/// The debug selector the window's title bar carries, so
/// `the_title_bar_is_the_top_of_the_window` can see where it was laid out.
pub(crate) const CHROME_TITLE_BAR: &str = "chrome-title-bar";

/// The debug selector the AppMenuBar inside the title bar carries.
pub(crate) const CHROME_APP_MENU_BAR: &str = "chrome-app-menu-bar";

/// The debug selector the active tab's root carries, so `every_tab_lays_out`
/// can find the tab it switched to.
pub(crate) const TAB_ROOT: &str = "tab-root";

/// The debug selector the content pane's scrolled element carries. Its right
/// edge is the scroll area's, which is where a vertical scrollbar's track ends
/// (gpui-base `src/scrollbar.rs:1408-1432`), so
/// `a_non_overlay_scrollbar_keeps_off_the_content` can see whether the tab
/// reaches under the bar.
pub(crate) const CONTENT_SCROLL: &str = "content-scroll";

/// The debug selector the sidebar column carries. Its bottom edge is where the
/// Widget Info panel has to reach, which is what
/// `the_widget_info_panel_fills_the_sidebar` measures.
pub(crate) const SIDEBAR_COLUMN: &str = "sidebar-column";

/// The debug selector the Widget Info panel's root carries.
pub(crate) const WIDGET_INFO: &str = "widget-info";

/// The debug selectors the List and the Tree demo boxes carry, so
/// `a_nested_scroller_keeps_the_wheel_to_itself` can put a wheel event inside
/// one and `the_three_list_frames_agree` can measure their frames.
pub(crate) const LIST_DEMO: &str = "list-demo";
pub(crate) const TREE_DEMO: &str = "tree-demo";

/// The debug selector the box that holds the Widget Info textarea carries. It
/// is the element the textarea fills, so its height is the height the text has
/// before the textarea scrolls its own content.
pub(crate) const WIDGET_INFO_TEXT: &str = "widget-info-text";

// ---------------------------------------------------------------------------
// Debug selectors for the interactive controls
// ---------------------------------------------------------------------------
//
// `interactive_controls_respond` clicks each of these and asks the model what
// changed. A control that carries one is a control the self-test drives; the
// name is shared by the render code and the test, so neither can drift onto an
// element the other does not mean.
pub(crate) const PROBE_RATING: &str = "probe-rating";
pub(crate) const PROBE_COMBOBOX: &str = "probe-combobox";
pub(crate) const PROBE_CLIPBOARD: &str = "probe-clipboard";
pub(crate) const PROBE_PAGINATION: &str = "probe-pagination";
pub(crate) const PROBE_ATTACHMENT: &str = "probe-attachment";
pub(crate) const PROBE_CHAT_SEND: &str = "probe-chat-send";
pub(crate) const PROBE_STEPPER: &str = "probe-stepper";
pub(crate) const PROBE_SIDEBAR_TOGGLE: &str = "probe-sidebar-toggle";
pub(crate) const PROBE_CAROUSEL_LAST: &str = "probe-carousel-last";
pub(crate) const PROBE_ALERT_DIALOG: &str = "probe-alert-dialog";
pub(crate) const PROBE_NOTIFICATION: &str = "probe-notification";
pub(crate) const PROBE_COLOR_MODE: &str = "probe-color-mode";

/// The debug selector a full-width item in the Settings demo's first group
/// carries. Nothing clicks it: its right edge is where a Settings row ends,
/// which `a_settings_row_keeps_off_the_page_scrollbar` measures against the
/// page's scrollbar.
pub(crate) const PROBE_SETTINGS_ROW: &str = "probe-settings-row";

/// Tag a control with a debug selector, so the self-test can find what it has
/// to click. The wrapper is a plain box around the control and leaves the
/// layout to it.
pub(crate) fn probe(selector: &'static str, control: impl IntoElement) -> Div {
    div().debug_selector(move || selector.into()).child(control)
}

/// The window the showcase opens at `bounds`: upstream's options for a
/// window that renders a `TitleBar` (title_bar.rs, `TitleBar::window_options`),
/// asking to draw its own decorations, so the `TitleBar` is the window's title
/// bar (spec §1.2). The self-tests open their windows with it too.
pub(crate) fn window_options(bounds: Bounds<Pixels>) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        window_decorations: Some(WindowDecorations::Client),
        ..TitleBar::window_options()
    }
}

// ---------------------------------------------------------------------------
// CLI argument parsing
// ---------------------------------------------------------------------------

/// Optional CLI arguments for launching the showcase in a specific state.
///
/// Parsed from `std::env::args()` — no external crate dependency.
/// When no arguments are provided the showcase behaves identically to before.
#[derive(Default)]
struct CliArgs {
    theme: Option<String>,
    variant: Option<String>,
    tab: Option<String>,
    icon_set: Option<String>,
    icon_theme: Option<String>,
    screenshot: Option<String>,
}

impl CliArgs {
    fn parse() -> Self {
        let mut args = Self::default();
        let argv: Vec<String> = std::env::args().collect();
        let mut i = 1; // skip binary name
        while i < argv.len() {
            match argv[i].as_str() {
                "--theme" => {
                    i += 1;
                    if i < argv.len() {
                        args.theme = Some(argv[i].clone());
                    }
                }
                "--variant" => {
                    i += 1;
                    if i < argv.len() {
                        args.variant = Some(argv[i].to_lowercase());
                    }
                }
                "--tab" => {
                    i += 1;
                    if i < argv.len() {
                        args.tab = Some(argv[i].to_lowercase());
                    }
                }
                "--icon-set" => {
                    i += 1;
                    if i < argv.len() {
                        args.icon_set = Some(argv[i].clone());
                    }
                }
                "--icon-theme" => {
                    i += 1;
                    if i < argv.len() {
                        args.icon_theme = Some(argv[i].clone());
                    }
                }
                "--screenshot" => {
                    i += 1;
                    if i < argv.len() {
                        args.screenshot = Some(argv[i].clone());
                    }
                }
                _ => {} // ignore unknown args
            }
            i += 1;
        }
        args
    }

    /// Map a `--tab` name to the tab it names.
    fn tab(name: &str) -> Option<Tab> {
        match name {
            "buttons" => Some(Tab::Buttons),
            "inputs" | "text-inputs" => Some(Tab::Inputs),
            "data" => Some(Tab::Data),
            "feedback" => Some(Tab::Feedback),
            "typography" => Some(Tab::Typography),
            "layout" => Some(Tab::Layout),
            "overlays" => Some(Tab::Overlays),
            "charts" => Some(Tab::Charts),
            "icons" => Some(Tab::Icons),
            "theme-map" => Some(Tab::ThemeMap),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (macOS only)
// ---------------------------------------------------------------------------

/// Get the NSWindow pointer for the main window via NSApplication.
#[cfg(target_os = "macos")]
fn get_main_window_ptr() -> Option<*mut objc2::runtime::AnyObject> {
    let ns_app_class = objc2::runtime::AnyClass::get(c"NSApplication")?;
    unsafe {
        let ns_app: *mut objc2::runtime::AnyObject =
            objc2::msg_send![ns_app_class, sharedApplication];
        // Try mainWindow first, then keyWindow, then first element of the
        // windows array.  On CI runners the second GUI process launched in
        // sequence may not get mainWindow promoted even after
        // cx.activate(true) and a 1.5 s delay.
        let main: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, mainWindow];
        if !main.is_null() {
            return Some(main);
        }
        let key: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, keyWindow];
        if !key.is_null() {
            return Some(key);
        }
        let windows: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, windows];
        let count: usize = objc2::msg_send![windows, count];
        if count > 0 {
            let first: *mut objc2::runtime::AnyObject =
                objc2::msg_send![windows, objectAtIndex: 0usize];
            if !first.is_null() {
                return Some(first);
            }
        }
        None
    }
}

/// Force the Metal drawable to update by nudging the window content size.
///
/// gpui initialises the Metal drawable at logical-pixel dimensions, ignoring
/// the Retina backing scale factor.  The correct device-pixel size is only
/// set inside the `setFrameSize:` callback, which early-returns when the
/// old size equals the new size.  A 1 px nudge-and-restore forces two real
/// resize events so `update_drawable_size` runs with the correct scale.
///
/// IMPORTANT: calls `[NSWindow setContentSize:]` directly via ObjC because
/// gpui's `window.resize()` spawns an async task that may not execute before
/// the screenshot capture.  Must be called **outside** `cx.update_window` to
/// avoid deadlocking the window-state mutex (since `setFrameSize:` acquires
/// it internally).
/// Minimal Core Graphics types for ObjC interop.
/// Based on objc2's encode_core_graphics example.
#[cfg(target_os = "macos")]
mod cg_types {
    use objc2::encode::{Encode, Encoding};

    #[repr(C)]
    pub struct CGPoint {
        pub x: f64,
        pub y: f64,
    }
    // SAFETY: repr(C) struct with correct encoding.
    unsafe impl Encode for CGPoint {
        const ENCODING: Encoding = Encoding::Struct("CGPoint", &[f64::ENCODING, f64::ENCODING]);
    }

    #[repr(C)]
    pub struct CGSize {
        pub width: f64,
        pub height: f64,
    }
    // SAFETY: repr(C) struct with correct encoding.
    unsafe impl Encode for CGSize {
        const ENCODING: Encoding = Encoding::Struct("CGSize", &[f64::ENCODING, f64::ENCODING]);
    }

    #[repr(C)]
    pub struct CGRect {
        pub origin: CGPoint,
        pub size: CGSize,
    }
    // SAFETY: repr(C) struct with correct encoding.
    unsafe impl Encode for CGRect {
        const ENCODING: Encoding =
            Encoding::Struct("CGRect", &[CGPoint::ENCODING, CGSize::ENCODING]);
    }
}

#[cfg(target_os = "macos")]
fn nudge_content_size(delta_w: f64, delta_h: f64) {
    if let Some(main_window) = get_main_window_ptr() {
        unsafe {
            let content_view: *mut objc2::runtime::AnyObject =
                objc2::msg_send![main_window, contentView];
            let frame: cg_types::CGRect = objc2::msg_send![content_view, frame];
            let new_size = cg_types::CGSize {
                width: frame.size.width + delta_w,
                height: frame.size.height + delta_h,
            };
            let _: () = objc2::msg_send![main_window, setContentSize: new_size];
        }
    }
}

/// Capture the gpui window including decorations using macOS `screencapture -l`.
///
/// Gets the CGWindowID via NSApplication -> mainWindow -> windowNumber, then
/// shells out to `screencapture -l <id> -o <path>`. This avoids the deprecated
/// `CGWindowListCreateImage` API and produces a PNG with full title bar and
/// window chrome.
#[cfg(target_os = "macos")]
fn capture_own_window_macos(_window: &mut Window, output_path: &str) -> bool {
    let Some(window_ptr) = get_main_window_ptr() else {
        eprintln!("No main window found");
        return false;
    };
    let window_id: i64 = unsafe { objc2::msg_send![window_ptr, windowNumber] };
    let status = std::process::Command::new("screencapture")
        .args(["-l", &format!("{}", window_id), "-o", output_path])
        .status();
    match status {
        Ok(s) if s.success() => {
            eprintln!("Screenshot saved to {output_path}");
            true
        }
        Ok(s) => {
            eprintln!("screencapture exited with {s}");
            false
        }
        Err(e) => {
            eprintln!("Failed to run screencapture: {e}");
            false
        }
    }
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (Windows only)
// ---------------------------------------------------------------------------

/// Capture the gpui window including decorations using Windows BitBlt.
///
/// Uses `FindWindowW` with the known window title to locate the correct HWND
/// (more reliable than `GetForegroundWindow` which may return a console or
/// other window on CI), then `BitBlt` + `GetDIBits` to extract pixel data.
#[cfg(target_os = "windows")]
fn capture_own_window_windows(_window: &mut Window, output_path: &str) -> bool {
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Dwm::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::PCWSTR;

    unsafe {
        let title = format!(
            "Native Theme \u{2013} GPUI Showcase, v{}",
            env!("CARGO_PKG_VERSION")
        );
        let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = match FindWindowW(None, PCWSTR(title_w.as_ptr())) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("FindWindowW failed: {e}");
                return false;
            }
        };

        // DWMWA_EXTENDED_FRAME_BOUNDS gives visible bounds in physical
        // screen pixels (excluding the invisible DWM border), matching
        // the screen DC coordinate space.  Fall back to GetWindowRect.
        let mut rect = RECT::default();
        if DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut _ as *mut std::ffi::c_void,
            std::mem::size_of::<RECT>() as u32,
        )
        .is_err()
        {
            if let Err(e) = GetWindowRect(hwnd, &mut rect) {
                eprintln!("GetWindowRect failed: {e}");
                return false;
            }
        }

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        if width <= 0 || height <= 0 {
            eprintln!("Invalid window dimensions: {width}x{height}");
            return false;
        }
        eprintln!(
            "windows capture: rect=({},{},{},{}), size={}x{}",
            rect.left, rect.top, rect.right, rect.bottom, width, height
        );

        let screen_dc = GetDC(None);
        let mem_dc = CreateCompatibleDC(Some(screen_dc));
        let bitmap = CreateCompatibleBitmap(screen_dc, width, height);
        let old_obj = SelectObject(mem_dc, bitmap.into());

        let blt_result = BitBlt(
            mem_dc,
            0,
            0,
            width,
            height,
            Some(screen_dc),
            rect.left,
            rect.top,
            SRCCOPY | CAPTUREBLT,
        );

        if blt_result.is_err() {
            SelectObject(mem_dc, old_obj);
            let _ = DeleteObject(bitmap.into());
            let _ = DeleteDC(mem_dc);
            ReleaseDC(None, screen_dc);
            eprintln!("BitBlt failed");
            return false;
        }

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0 as u32,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut pixels = vec![0u8; (width * height * 4) as usize];
        let lines = GetDIBits(
            mem_dc,
            bitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(mem_dc, old_obj);
        let _ = DeleteObject(bitmap.into());
        let _ = DeleteDC(mem_dc);
        ReleaseDC(None, screen_dc);

        if lines == 0 {
            eprintln!("GetDIBits returned 0 lines");
            return false;
        }

        for chunk in pixels.as_chunks_mut::<4>().0 {
            chunk.swap(0, 2); // BGRA -> RGBA
            chunk[3] = 255; // force opaque
        }

        match image::save_buffer(
            output_path,
            &pixels,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        ) {
            Ok(()) => {
                eprintln!("Screenshot saved to {output_path}");
                true
            }
            Err(e) => {
                eprintln!("Failed to save PNG: {e}");
                false
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
fn main() {
    let cli_args = CliArgs::parse();

    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx: &mut App| {
            gpui_kit::init(cx);
            app::init(cx);

            // Apply CLI variant override before window opens so the initial
            // theme is resolved with the correct light/dark setting.
            let variant_override = cli_args.variant.as_deref().map(|v| v == "dark");

            let bounds = Bounds::centered(None, WINDOW_SIZE, cx);
            let window_handle = cx.open_window(window_options(bounds), |window, cx| {
                let showcase = cx.new(|cx| {
                    let mut s = Showcase::new(window, cx);

                    // Override color mode if --variant was specified
                    if let Some(is_dark) = variant_override {
                        let mode = if is_dark {
                            AppColorMode::Dark
                        } else {
                            AppColorMode::Light
                        };
                        s.color_mode = mode;
                        s.is_dark = is_dark;
                        // Update the color mode selector dropdown
                        let label = SharedString::from(mode.label());
                        s.dark_mode_select.update(cx, |select, cx| {
                            select.set_selected_value(&label, window, cx);
                        });
                    }

                    // Override theme if --theme was specified
                    if let Some(ref theme_name) = cli_args.theme {
                        s.current_theme_name = theme_name.clone();
                        s.apply_theme_by_name(theme_name, window, cx);
                        // Update the theme selector dropdown to show the overridden theme
                        let display = SharedString::from(theme_name.clone());
                        s.theme_select.update(cx, |select, cx| {
                            select.set_selected_value(&display, window, cx);
                        });
                    }

                    // Override tab if --tab was specified
                    if let Some(ref tab_name) = cli_args.tab
                        && let Some(tab) = CliArgs::tab(tab_name)
                    {
                        s.active_tab = tab;
                    }

                    // Override icon theme if --icon-theme was specified
                    if let Some(ref theme_name) = cli_args.icon_theme {
                        s.icon_theme_override = Some(theme_name.clone());
                    }

                    // Override icon set if --icon-set was specified
                    if let Some(ref set_name) = cli_args.icon_set {
                        // Map CLI set name to an IconSetChoice
                        s.icon_set_choice = match set_name.as_str() {
                            "material" => IconSetChoice::Material,
                            "lucide" => IconSetChoice::Lucide,
                            "freedesktop" => IconSetChoice::System,
                            _ => IconSetChoice::System,
                        };
                        let effective = s.icon_set_choice.effective_icon_set(s.current_icon_set);
                        let default_theme =
                            s.icon_set_choice.freedesktop_theme().map(|t| t.to_string());
                        s.icon_set_name = effective.name().to_string();
                        s.icon_set_enum = Some(effective);
                        let cli_ref = s.icon_theme_override.as_deref();
                        let fc = s.original_font.color;
                        let fg_rgb = Some([fc.r, fc.g, fc.b]);
                        s.loaded_icons =
                            load_all_icons(effective, default_theme.as_deref(), cli_ref, fg_rgb);
                        s.gpui_icons = load_gpui_icons(
                            Some(effective),
                            default_theme.as_deref(),
                            cli_ref,
                            fg_rgb,
                        );
                        let fg = cx.theme().foreground;
                        s.rebuild_icon_caches(fg, window, cx);
                        s.rebuild_animation_caches(window, cx);
                        s.start_animation_timer(cx);

                        // Update the icon theme selector dropdown
                        let icon_display: SharedString = s.icon_set_choice.to_string().into();
                        let mut icon_names = s.icon_set_dropdown_names();
                        // Add the override display name if not already in list
                        if !icon_names.contains(&icon_display) {
                            icon_names.push(icon_display.clone());
                        }
                        let new_delegate = SearchableVec::new(icon_names);
                        s.icon_set_select.update(cx, |select, cx| {
                            select.set_items(new_delegate, window, cx);
                            select.set_selected_value(&icon_display, window, cx);
                        });
                    }

                    s
                });
                cx.new(|cx| Root::new(showcase, window, cx))
            });
            let Ok(window_handle) = window_handle else {
                eprintln!("Fatal: failed to open main application window");
                cx.quit();
                return;
            };
            window_handle
                .update(cx, |_, window, _| {
                    window.set_window_title(&format!(
                        "Native Theme – GPUI Showcase, v{}",
                        env!("CARGO_PKG_VERSION")
                    ));
                })
                .ok();

            // Force Metal drawable to adopt the Retina scale factor by
            // nudging the content size synchronously via ObjC.  Must happen
            // outside update() to avoid deadlocking the window-state mutex.
            #[cfg(target_os = "macos")]
            {
                nudge_content_size(-1.0, 0.0);
                nudge_content_size(1.0, 0.0);
            }
            cx.activate(true);

            // Schedule delayed self-capture if --screenshot was provided
            if let Some(screenshot_path) = cli_args.screenshot.as_ref() {
                #[cfg(target_os = "macos")]
                {
                    let path = screenshot_path.clone();
                    let any_handle = *window_handle;
                    cx.spawn(async move |cx| {
                        // Force Metal drawable to update on Retina displays.
                        // Calls [NSWindow setContentSize:] directly (synchronous)
                        // rather than gpui's window.resize() which is async and
                        // may not execute before the capture.
                        nudge_content_size(-1.0, 0.0);
                        cx.background_executor()
                            .timer(Duration::from_millis(200))
                            .await;
                        nudge_content_size(1.0, 0.0);
                        cx.background_executor()
                            .timer(Duration::from_millis(1300))
                            .await;
                        let captured = cx
                            .update_window(any_handle, |_view, window, _cx| {
                                capture_own_window_macos(window, &path)
                            })
                            .unwrap_or(false);
                        if !captured {
                            eprintln!("ERROR: screenshot capture failed for {path}");
                            std::process::exit(1);
                        }
                        let _ = cx.update(|cx| cx.quit());
                    })
                    .detach();
                }
                #[cfg(target_os = "windows")]
                {
                    let path = screenshot_path.clone();
                    let any_handle = *window_handle;
                    cx.spawn(async move |cx| {
                        cx.background_executor()
                            .timer(Duration::from_millis(1500))
                            .await;
                        let captured = cx
                            .update_window(any_handle, |_view, window, _cx| {
                                capture_own_window_windows(window, &path)
                            })
                            .unwrap_or(false);
                        if !captured {
                            eprintln!("ERROR: screenshot capture failed for {path}");
                            std::process::exit(1);
                        }
                        let _ = cx.update(|cx| cx.quit());
                    })
                    .detach();
                }
                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                {
                    let _ = &screenshot_path;
                    eprintln!(
                        "Self-capture not supported on this platform. \
                         Use spectacle or generate_gpui_screenshots.sh instead."
                    );
                    // Continue running -- let the user capture manually
                }
            }
            let _ = &window_handle; // suppress unused warning when not used for capture
        });
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn main() {
    eprintln!("gpui showcase is not supported on this platform");
}

#[cfg(test)]
mod tests;
