//! The egui showcase: the gpui showcase's application, built from egui's own
//! containers (spec §10.4).
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

mod app;
mod chrome;
mod demo;
mod info;
mod pages;
#[cfg(test)]
mod tests;

use app::{App, Page, Settings};

/// The window's title: the crate's name and version, as the gpui showcase's
/// (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:222-227`).
pub(crate) const WINDOW_TITLE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " showcase"
);

/// The side panel's initial width. The model states no side-panel width (spec §10.4); this
/// is the gpui showcase's `LEFT_PANEL_WIDTH_PX`
/// (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:285`).
pub(crate) const LEFT_PANEL_WIDTH: f32 = 300.0;

/// The initial window size, 1280 × 720 logical pixels: the maintainer's default for every
/// showcase, the gpui showcase's `WINDOW_SIZE` and the iced showcase's `WINDOW_SIZE`. The model
/// states no such value (spec §10.4). The side panel opens `LEFT_PANEL_WIDTH` wide and the page
/// takes the rest.
pub(crate) const WINDOW_SIZE: egui::Vec2 = egui::Vec2::new(1280.0, 720.0);

/// How long a new Widget Info choice must stay the choice before it is shown. The model
/// states no hover delay (spec §10.4); this is the gpui showcase's
/// (`connectors/native-theme-gpui/examples/showcase-gpui/info/registry.rs:16`).
pub(crate) const INFO_SETTLE: std::time::Duration = std::time::Duration::from_millis(250);

/// The thickness of the line under the selected tab. The model states no tab indicator; this is
/// gpui-component's underline tab, whose selected tab has a 2px bottom border
/// (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-component-0.6.6/src/tab/tab.rs:253-261`),
/// in the colour the theme states for the primary button.
pub(crate) const TAB_UNDERLINE_WIDTH: f32 = 2.0;

/// The weight the showcase draws its headings in. The model states no heading weight; this is
/// gpui's `FontWeight::SEMIBOLD`
/// (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-pre-0.3.6/src/text_system.rs:1169`),
/// which the gpui showcase's headings take (`font_semibold()`,
/// `connectors/native-theme-gpui/examples/showcase-gpui/demo.rs:1373`). Without `system-fonts`
/// no face is looked up, and the headings keep the regular one.
#[cfg(feature = "system-fonts")]
pub(crate) const SEMIBOLD_WEIGHT: u16 = 600;

/// The side of a colour swatch in Widget Info and on the Theme Map. The model states no swatch;
/// this is the gpui showcase's `SWATCH_SIZE`
/// (`connectors/native-theme-gpui/examples/showcase-gpui/inspector.rs:334`), the square both of
/// its swatches draw.
pub(crate) const SWATCH_SIZE: f32 = 16.0;

/// How long `--screenshot` lets the showcase run before it captures, in seconds of
/// `InputState::time`: the iced showcase's delay, "60 ticks × 50ms = 3s render delay"
/// (`connectors/native-theme-iced/examples/showcase-iced.rs:1222-1223`). Not a style value.
pub(crate) const SCREENSHOT_DELAY_S: f64 = 3.0;

/// What `main` runs with and `the_window_asks_for_the_os_frame` reads back (§13.2):
/// `build_eframe` takes none, so the test calls this function too.
///
/// A capture (`capturing`: `--capture` or `--screenshot`) opens at `WINDOW_SIZE` whatever an
/// earlier run left. eframe restores a stored window size only with its `persistence` feature
/// (`eframe/src/epi.rs:378-379`), which the showcase does not enable, and `persist_window` is
/// off for a capture so that enabling it would not change what a capture shows. A desktop can
/// store a size of its own, by the window's app id (a KWin script that remembers window
/// geometry, say), so a capture's window takes `capture_app_id`, which nothing stored.
pub(crate) fn native_options(capturing: bool) -> eframe::NativeOptions {
    let viewport = egui::ViewportBuilder::default()
        .with_title(WINDOW_TITLE)
        .with_decorations(true)
        .with_inner_size(WINDOW_SIZE);
    eframe::NativeOptions {
        viewport: if capturing {
            viewport.with_app_id(capture_app_id())
        } else {
            viewport
        },
        persist_window: !capturing,
        #[cfg(all(target_os = "macos", not(test)))]
        event_loop_builder: Some(Box::new(|builder| {
            use winit::platform::macos::EventLoopBuilderExtMacOS as _;
            builder.with_default_menu(false);
        })),
        ..Default::default()
    }
}

/// The app id of a capture's window: this process's own, so no geometry a desktop stored
/// for an earlier window applies to it.
pub(crate) fn capture_app_id() -> String {
    format!("showcase-egui-capture-{}", std::process::id())
}

/// Whether a frame `--screenshot` captured, `measured` physical pixels, is `WINDOW_SIZE` at the
/// display's scale factor `scale`, rounded as winit rounds a logical size to a physical one. The
/// frame is the window's content alone, so any other size is a window that did not open at its
/// default size (a display too small for it, or a size restored from elsewhere).
pub(crate) fn check_capture_size(measured: [usize; 2], scale: Option<f32>) -> Result<(), String> {
    let scale = scale.ok_or("the display's scale factor is unknown")?;
    let expected = [
        (WINDOW_SIZE.x * scale).round() as usize,
        (WINDOW_SIZE.y * scale).round() as usize,
    ];
    if measured == expected {
        Ok(())
    } else {
        let ([w, h], [ew, eh]) = (measured, expected);
        Err(format!(
            "the captured frame is {w}x{h} px, expected {ew}x{eh} ({}x{} at scale {scale}): \
             the window did not open at its default size",
            WINDOW_SIZE.x, WINDOW_SIZE.y
        ))
    }
}

/// The flags the capture pipeline passes (`.github/workflows/screenshots.yml`,
/// `scripts/generate_screenshots_egui.sh`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CliArgs {
    pub theme: Option<String>,
    pub variant: Option<String>,
    pub tab: Option<String>,
    pub icon_set: Option<String>,
    pub screenshot: Option<String>,
    /// `--capture`: a tool outside the showcase captures its window, which opens as a
    /// `--screenshot` run's does (`native_options`).
    pub capture: bool,
}

impl CliArgs {
    /// Whether the window is captured: by the showcase itself (`--screenshot`) or by a tool
    /// outside it (`--capture`).
    pub(crate) fn capturing(&self) -> bool {
        self.capture || self.screenshot.is_some()
    }

    /// `--flag value` pairs and `--capture`; an unknown flag is ignored, and `--variant` and
    /// `--tab` are lower-cased, as the iced showcase does
    /// (`connectors/native-theme-iced/examples/showcase-iced.rs:228-270`); a
    /// theme or icon-theme name keeps its case, since a freedesktop theme's is
    /// a directory name.
    pub(crate) fn parse(argv: impl IntoIterator<Item = String>) -> Self {
        let mut cli = Self::default();
        let mut argv = argv.into_iter();
        while let Some(flag) = argv.next() {
            let lower = matches!(flag.as_str(), "--variant" | "--tab");
            let slot = match flag.as_str() {
                "--capture" => {
                    cli.capture = true;
                    continue;
                }
                "--theme" => &mut cli.theme,
                "--variant" => &mut cli.variant,
                "--tab" => &mut cli.tab,
                "--icon-set" => &mut cli.icon_set,
                "--screenshot" => &mut cli.screenshot,
                _ => continue,
            };
            *slot = argv
                .next()
                .map(|v| if lower { v.to_lowercase() } else { v });
        }
        cli
    }
}

/// Each flag takes exactly what its picker offers; a value the showcase cannot
/// honour is reported on stderr and ignored (§10.4).
pub(crate) fn apply_cli_args(settings: &mut Settings, cli: &CliArgs) {
    if let Some(mode) = cli
        .variant
        .as_deref()
        .and_then(|v| reported(Settings::mode_choice(v)))
    {
        settings.mode = mode;
    }
    if let Some(theme) = cli
        .theme
        .as_deref()
        .and_then(|v| reported(Settings::theme_choice(v)))
    {
        settings.theme = theme;
    }
    if let Some(pick) = cli
        .icon_set
        .as_deref()
        .and_then(|v| reported(settings.icon_choice(v)))
    {
        settings.pick_icon(pick);
    }
    if let Some(page) = cli.tab.as_deref().and_then(|v| reported(Page::from_key(v))) {
        settings.page = page;
    }
    settings.screenshot.clone_from(&cli.screenshot);
}

/// The value of `result`; its error reported on stderr, and `None`.
fn reported<T>(result: Result<T, String>) -> Option<T> {
    result.map_err(|error| eprintln!("{error}; ignored")).ok()
}

fn main() -> eframe::Result {
    let cli = CliArgs::parse(std::env::args().skip(1));
    eframe::run_native(
        WINDOW_TITLE,
        native_options(cli.capturing()),
        Box::new(move |cc| Ok(Box::new(App::new(cc, &cli)?))),
    )
}
