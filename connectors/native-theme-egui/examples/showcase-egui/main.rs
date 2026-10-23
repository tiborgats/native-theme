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

use native_theme_showcase_support::cli;
use native_theme_showcase_support::{Kind, WINDOW_SIZE_PX, capture};

use app::{App, Page, Settings};

/// What the three showcases share (`native-theme-showcase-support`): its command line, the
/// element list's reader and the Widget Info settle time, under the names the showcase's
/// modules use.
pub(crate) use native_theme_showcase_support::INFO_SETTLE;
pub(crate) use native_theme_showcase_support::cli::CliArgs;
pub(crate) use native_theme_showcase_support::elements;

/// The window's title: the crate's name and version, as the gpui showcase's
/// (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:277-282`).
pub(crate) const WINDOW_TITLE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " showcase"
);

/// The side panel's initial width. The model states no side-panel width (spec §10.4); this
/// is the gpui showcase's `LEFT_PANEL_WIDTH_PX`
/// (`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:340`).
pub(crate) const LEFT_PANEL_WIDTH: f32 = 300.0;

/// The initial window size: the shared crate's `WINDOW_SIZE_PX`, 1280 × 720 logical pixels,
/// the maintainer's default for every showcase, which the gpui and iced showcases open at too.
/// The model states no such value (spec §10.4). The side panel opens `LEFT_PANEL_WIDTH` wide and
/// the page takes the rest.
pub(crate) const WINDOW_SIZE: egui::Vec2 =
    egui::Vec2::new(WINDOW_SIZE_PX.0 as f32, WINDOW_SIZE_PX.1 as f32);

/// The weight the showcase draws its inspector's headings in (a page's section headings take
/// `text_scale.section_heading.weight`). The model states no inspector heading weight; this is
/// gpui's `FontWeight::SEMIBOLD`
/// (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-pre-0.3.6/src/text_system.rs:1169`),
/// which the gpui showcase's inspector headings take (`font_semibold()`,
/// `connectors/native-theme-gpui/examples/showcase-gpui/inspector.rs:154`, `:383`). Without
/// `system-fonts` no face is looked up, and the headings keep the regular one.
#[cfg(feature = "system-fonts")]
pub(crate) const SEMIBOLD_WEIGHT: u16 = 600;

/// The side of a colour swatch in Widget Info and on the Theme Map. The model states no swatch;
/// this is the gpui showcase's `SWATCH_SIZE`
/// (`connectors/native-theme-gpui/examples/showcase-gpui/inspector.rs:400`), the square both of
/// its swatches draw.
pub(crate) const SWATCH_SIZE: f32 = 16.0;

/// How long `--screenshot` lets the showcase run before it captures, in seconds of
/// `InputState::time`: the iced showcase's delay, "60 ticks × 50ms = 3s render delay"
/// (`connectors/native-theme-iced/examples/showcase-iced.rs:2171`). Not a style value.
pub(crate) const SCREENSHOT_DELAY_S: f64 = 3.0;

/// What `main` runs with and `the_window_asks_for_the_os_frame` reads back (§13.2):
/// `build_eframe` takes none, so the test calls this function too.
///
/// A capture (`capturing`: `--capture` or `--screenshot`) opens at `WINDOW_SIZE` whatever an
/// earlier run left. eframe restores a stored window size only with its `persistence` feature
/// (`eframe/src/epi.rs:378-379`), which the showcase does not enable, and `persist_window` is
/// off for a capture so that enabling it would not change what a capture shows. A desktop can
/// store a size of its own, by the window's app id (a KWin script that remembers window
/// geometry, say), so a capture's window takes the three showcases' capture app id
/// (`capture::app_id`), which nothing stored and `scripts/capture_window.sh` finds it by.
pub(crate) fn native_options(capturing: bool) -> eframe::NativeOptions {
    let viewport = egui::ViewportBuilder::default()
        .with_title(WINDOW_TITLE)
        .with_decorations(true)
        .with_inner_size(WINDOW_SIZE);
    eframe::NativeOptions {
        viewport: if capturing {
            viewport.with_app_id(capture::app_id(Kind::Egui))
        } else {
            viewport
        },
        persist_window: !capturing,
        // Off: egui-wgpu's dithering adds noise to a vertex colour that falls between two 8-bit
        // values after its sRGB conversion (`eframe/src/epi.rs:384-392`), so a flat fill of a
        // theme colour would be painted a unit off here and there; a toolkit paints the colour
        // the theme states.
        dithering: false,
        #[cfg(all(target_os = "macos", not(test)))]
        event_loop_builder: Some(Box::new(|builder| {
            use winit::platform::macos::EventLoopBuilderExtMacOS as _;
            builder.with_default_menu(false);
        })),
        ..Default::default()
    }
}

/// How long, in seconds of `InputState::time`, the layout must stay the same from pass to pass
/// before `--dump-layout` writes it: the first settled frame, after the fonts, the icons and a
/// held pointer's Widget Info (`INFO_SETTLE`) have landed. Not a style value.
pub(crate) const DUMP_SETTLE_S: f64 = 1.0;

/// Each flag takes exactly what its picker offers; a value the showcase cannot honour is
/// reported on stderr and ignored (§10.4), by the shared crate's validators (`cli`), whose error
/// texts the three showcases share.
pub(crate) fn apply_cli_args(settings: &mut Settings, cli: &CliArgs) {
    if let Some(mode) = cli
        .variant
        .as_deref()
        .and_then(|v| cli::reported(cli::color_mode(v)))
    {
        settings.mode = mode;
    }
    if let Some(theme) = cli
        .theme
        .as_deref()
        .and_then(|v| cli::reported(cli::theme_key(v)))
    {
        settings.theme = theme;
    }
    let installed = &settings.installed_themes;
    match cli
        .icon_set
        .as_deref()
        .and_then(|v| cli::reported(cli::icon_set_choice(v, installed, &[])))
    {
        Some(cli::IconPick::FollowTheme) => settings.pick_icon(None),
        Some(cli::IconPick::Choice(choice)) => settings.pick_icon(Some(choice)),
        // The egui picker has no row of its own, and none is passed.
        Some(cli::IconPick::Toolkit(_)) | None => {}
    }
    if let Some(page) = cli
        .tab
        .as_deref()
        .and_then(|v| cli::reported(Page::from_key(v)))
    {
        settings.page = page;
    }
    settings.screenshot.clone_from(&cli.screenshot);
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // The second run `--screenshot` starts on Windows to find the window (`capture::run_finder`),
    // before any window opens.
    if let Some(code) = capture::run_finder(&args, WINDOW_TITLE) {
        std::process::exit(code);
    }
    let cli = CliArgs::parse(args);
    if let Some(value) = &cli.bad_pointer {
        cli::refuse(&format!(
            "--pointer {value:?}: not X,Y in whole logical pixels"
        ));
    }
    if cli.screenshot.is_some()
        && let Err(error) = cli::screenshot_support(Kind::Egui)
    {
        cli::refuse(&error);
    }
    eframe::run_native(
        WINDOW_TITLE,
        native_options(cli.capturing()),
        Box::new(move |cc| Ok(Box::new(App::new(cc, &cli)?))),
    )
}
