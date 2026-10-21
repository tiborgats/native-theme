//! What the gpui, iced and egui showcases share (dev-only, never published):
//! a dev-dependency of each connector, so the three read one command line,
//! check a capture of their window one way, write one layout dump, read
//! `docs/showcase-elements.toml` through one reader and show its Widget Info
//! values, rows and Copy text, its side panel's picker rows and its status
//! bar's text alike. It names no toolkit: each showcase keeps its window, its
//! pages and menus, the drawing of every panel, the rectangles its dump
//! takes and the "how" line of each Widget Info row, the one part of Widget
//! Info the list's header lets differ.
//!
//! - [`cli`]: the flags, their parser, one validator per flag with one
//!   error text, and the exit rules;
//! - [`capture`]: the capture's app id, the frame check, `--screenshot`'s
//!   capture on macOS and Windows and the Retina nudge;
//! - [`dump`]: the layout dump `--dump-layout` writes;
//! - [`elements`]: the element list, parsed once;
//! - [`values`]: a leaf's value in the text the three showcases agreed;
//! - [`info`]: a listed element's Widget Info rows and Copy text;
//! - [`rows`]: the preset and icon-theme pickers' rows;
//! - [`status`]: the status bar's environment items.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::time::Duration;

pub mod capture;
pub mod cli;
pub mod dump;
pub mod elements;
pub mod info;
pub mod rows;
pub mod status;
pub mod values;

/// The showcase a call is for, where its name or its capture script goes
/// into a text, an app id or the layout dump.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Gpui,
    Iced,
    Egui,
}

impl Kind {
    /// The showcase's name: `gpui`, `iced` or `egui`, the layout dump's
    /// `kind`.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Kind::Gpui => "gpui",
            Kind::Iced => "iced",
            Kind::Egui => "egui",
        }
    }

    /// The script that captures the showcase on Linux, where `--screenshot`
    /// is refused.
    #[must_use]
    pub fn capture_script(self) -> &'static str {
        match self {
            Kind::Gpui => "scripts/generate_screenshots_gpui.sh",
            Kind::Iced => "scripts/generate_screenshots_iced.sh",
            Kind::Egui => "scripts/generate_screenshots_egui.sh",
        }
    }
}

/// The window's initial size, 1280 × 720 logical pixels: the maintainer's
/// default for every showcase, which the screenshots are taken at. The
/// model states no window size. Each showcase builds its toolkit-typed
/// `WINDOW_SIZE` from it, so one number holds the three, and the frame check
/// ([`capture::check_frame_capture`]) measures a capture against it.
pub const WINDOW_SIZE_PX: (u32, u32) = (1280, 720);

/// How long a new Widget Info choice must stay the choice before it is
/// shown. The model states no such value: its `tooltip.show_delay_ms` is how
/// long a tooltip waits, and Widget Info is not a tooltip. The gpui
/// showcase's (`connectors/native-theme-gpui/examples/showcase-gpui/info/registry.rs`,
/// `INFO_SETTLE`), which the egui showcase copied.
pub const INFO_SETTLE: Duration = Duration::from_millis(250);

/// A dialog's padding on a side the theme's `dialog.border.padding` leaves
/// unstated, inside the dialog's border line: gpui-component's `Dialog`'s
/// own, `Edges::all(px(16.))`, which each stated side replaces
/// (gpui-component 0.7.3 `src/dialog/dialog.rs:635-647`), and which the gpui
/// showcase's dialogs keep. The iced showcase pads its dialogs so (and the
/// egui showcase after M9c T7), as the shared chrome looks like gpui's; the
/// model states no such default.
/// Logical pixels, not a rem multiple: text scale leaves it alone.
pub const DIALOG_PADDING_PX: f32 = 16.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// The script a kind names is in the repository, under the kind's name.
    #[test]
    fn each_kind_names_its_capture_script() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        for kind in [Kind::Gpui, Kind::Iced, Kind::Egui] {
            let script = kind.capture_script();
            assert!(script.contains(kind.name()), "{script}");
            assert!(
                std::path::Path::new(root).join(script).is_file(),
                "{script} is not in the repository"
            );
        }
    }
}
