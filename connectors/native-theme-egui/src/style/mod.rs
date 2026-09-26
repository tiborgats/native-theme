//! The mapping: base style → role cells → surface frames, per colour scheme (spec §3.4, §5,
//! §6). Task 11 builds egui's own values; Tasks 13–18 write the theme's into them.

pub(crate) mod base;

use std::sync::Arc;

use crate::atlas::{Note, SchemeStyles};
use crate::roles::{ROLES, SURFACES, VARIANTS};
use crate::{AccessibilityPreferences, LayoutTheme, ResolvedTheme, Surface};

/// Everything one scheme's styles are built from.
pub(crate) struct BuildInput<'a> {
    pub scheme: egui::Theme,
    pub theme: &'a ResolvedTheme,
    pub prefs: &'a AccessibilityPreferences,
    pub layout: &'a LayoutTheme,
    /// epaint's row height for the Body face at the scaled Body size (§6.15); Task 13
    /// computes it. `None` leaves egui's own `extra_text_line_spacing`.
    pub row_height: Option<f32>,
}

/// The frame egui itself builds for a surface's container over `base` (§3.4, §4.5):
/// `Frame::window` for `Window` and `WindowTitleBar` (`egui/src/containers/window.rs:630-631`),
/// `Frame::popup` for `Dialog`, `Popover` and `Tooltip` (`egui/src/containers/modal.rs:100`,
/// `egui/src/containers/popup.rs:603`), `Frame::group` for `Card` (`egui/src/ui.rs:2147-2148`),
/// `Frame::side_top_panel` for `Panel` (`egui/src/containers/panel.rs:950`) and
/// `Frame::central_panel` for `CentralPanel` (`:1243`).
pub(crate) fn egui_preset(surface: Surface, base: &egui::Style) -> egui::Frame {
    match surface {
        Surface::Window | Surface::WindowTitleBar => egui::Frame::window(base),
        Surface::Dialog | Surface::Popover | Surface::Tooltip => egui::Frame::popup(base),
        Surface::Card => egui::Frame::group(base),
        Surface::Panel(_) => egui::Frame::side_top_panel(base),
        Surface::CentralPanel => egui::Frame::central_panel(base),
    }
}

/// One scheme's styles. Task 11: egui's own style for the scheme in the base and every cell,
/// and egui's own preset frame per surface — every value the theme does not supply keeps
/// egui's by construction (§3.4). Tasks 13–18 write the theme's values.
pub(crate) fn compile(input: &BuildInput<'_>, notes: &mut Vec<Note>) -> SchemeStyles {
    let base = Arc::new(base::base_style(input, notes));
    let cells = ROLES.map(|_| VARIANTS.map(|_| Arc::clone(&base)));
    let frames = SURFACES.map(|surface| egui_preset(surface, &base));
    SchemeStyles {
        base,
        cells,
        frames,
        focus_ring: None,
    }
}

/// Record `note` unless an equal one is already recorded: §7.2 reports a leaf once, and one
/// leaf reaches several sinks, every cell built from the base, and both schemes.
pub(crate) fn push_note(notes: &mut Vec<Note>, note: Note) {
    if !notes.contains(&note) {
        notes.push(note);
    }
}

/// §7.2's call-site test for a length `convert::i8_from_f32_saturating` will clamp: the two
/// values `round()` carries outside `-128..=127` (§6.17). The caller reports
/// `Note::ValueSaturated`, because the helper stays pure and single-valued.
pub(crate) fn saturates_i8(v: f32) -> bool {
    let d = crate::convert::denan(v);
    d >= 127.5 || d <= -128.5
}
