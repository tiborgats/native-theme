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
#![allow(
    dead_code,
    reason = "the showcase is assembled over Tasks 31-36; Task 36 removes this line"
)]

mod app;
mod chrome;
mod demo;
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

/// The initial window size. The model states no such value (spec §10.4); this
/// is the gpui showcase's, the side panel beside a page
/// (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:207`, `:216-217`, `:285`).
pub(crate) const WINDOW_SIZE: egui::Vec2 = egui::Vec2::new(LEFT_PANEL_WIDTH + 880.0, 850.0);

/// How long `--screenshot` lets the showcase run before it captures, in seconds of
/// `InputState::time`: the iced showcase's delay, "60 ticks × 50ms = 3s render delay"
/// (`connectors/native-theme-iced/examples/showcase-iced.rs:1222-1223`). Not a style value.
pub(crate) const SCREENSHOT_DELAY_S: f64 = 3.0;

/// What `main` runs with and `the_window_asks_for_the_os_frame` reads back (§13.2):
/// `build_eframe` takes none, so the test calls this function too.
pub(crate) fn native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(WINDOW_TITLE)
            .with_decorations(true)
            .with_inner_size(WINDOW_SIZE),
        #[cfg(all(target_os = "macos", not(test)))]
        event_loop_builder: Some(Box::new(|builder| {
            use winit::platform::macos::EventLoopBuilderExtMacOS as _;
            builder.with_default_menu(false);
        })),
        ..Default::default()
    }
}

/// The five flags the capture pipeline passes (`.github/workflows/screenshots.yml:79-82`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CliArgs {
    pub theme: Option<String>,
    pub variant: Option<String>,
    pub tab: Option<String>,
    pub icon_set: Option<String>,
    pub screenshot: Option<String>,
}

impl CliArgs {
    /// `--flag value` pairs; an unknown flag is ignored, and `--variant` and
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
        native_options(),
        Box::new(move |cc| Ok(Box::new(App::new(cc, &cli)?))),
    )
}
