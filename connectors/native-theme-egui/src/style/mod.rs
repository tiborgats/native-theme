//! The mapping: base style → role cells → surface frames, per colour scheme (spec §3.4, §5,
//! §6). Task 11 builds egui's own values; Tasks 13–18 write the theme's into them.

pub(crate) mod base;
mod derived;
mod frames;
mod roles;
mod states;
mod variants;

use std::sync::Arc;

use crate::atlas::{Note, SchemeStyles};
use crate::{AccessibilityPreferences, LayoutTheme, ResolvedTheme, Role, Surface};

/// Everything one scheme's styles are built from.
pub(crate) struct BuildInput<'a> {
    pub scheme: egui::Theme,
    pub theme: &'a ResolvedTheme,
    pub prefs: &'a AccessibilityPreferences,
    pub layout: &'a LayoutTheme,
    /// epaint's row height for the Body face at the scaled Body size (§6.15); Task 13
    /// computes it. `None` leaves egui's own `extra_text_line_spacing`.
    pub row_height: Option<f32>,
    /// `Builder::style_patch`'s closure, applied last to every style (§4.3); `None` without one.
    pub patch: Option<&'a dyn Fn(&mut egui::Style)>,
}

/// §4.3's two application-owned adjustments, after all theme data: reduced motion, then the
/// patch. Run on a `Style` before it is wrapped in its `Arc`, so a cell that shares its
/// `Normal` `Arc` (§3.4) is adjusted once and stays shared.
pub(crate) fn finish(style: &mut egui::Style, input: &BuildInput<'_>) {
    if input.prefs.reduce_motion {
        // "no animation": egui reaches the end value at once at `0.0`
        // (`egui/src/animation_manager.rs:56-60`, `:88-93`), and `none()` is egui's own
        // constructor (`egui/src/style.rs:858`) — §6.17's listed exemption.
        style.animation_time = 0.0;
        style.scroll_animation = egui::style::ScrollAnimation::none();
    }
    if let Some(patch) = input.patch {
        patch(style);
    }
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
    let mut cells: [[Arc<egui::Style>; 3]; 25] =
        std::array::from_fn(|_| [Arc::clone(&base), Arc::clone(&base), Arc::clone(&base)]);
    for (slot, role) in cells.iter_mut().zip(Role::all()) {
        let mut normal = roles::role_cell(*role, &base, input, notes);
        let finished = |mut style: egui::Style| {
            finish(&mut style, input);
            Arc::new(style)
        };
        let selected = variants::selected_cell(*role, &normal, input, notes).map(finished);
        let disabled = variants::disabled_cell(*role, &normal, input, notes).map(finished);
        finish(&mut normal, input);
        let normal = Arc::new(normal);
        *slot = [
            Arc::clone(&normal),
            selected.unwrap_or_else(|| Arc::clone(&normal)),
            disabled.unwrap_or_else(|| Arc::clone(&normal)),
        ];
    }
    let mut frames = [egui::Frame::NONE; 11];
    for (slot, surface) in frames.iter_mut().zip(Surface::all()) {
        *slot = frames::surface_frame(*surface, &base, input, notes);
    }
    let mut finished_base = egui::Style::clone(&base);
    finish(&mut finished_base, input);
    SchemeStyles {
        base: Arc::new(finished_base),
        cells,
        frames,
        focus_ring: crate::plugin::build_focus_ring(input.theme, notes),
    }
}

/// Record `note` unless an equal one is already recorded: §7.2 reports a leaf once, and one
/// leaf reaches several sinks, every cell built from the base, and both schemes.
pub(crate) fn push_note(notes: &mut Vec<Note>, note: Note) {
    if !notes.contains(&note) {
        notes.push(note);
    }
}

/// §6.4: a colour with alpha `0` written into a `WidgetVisuals::bg_fill` — which egui documents
/// "Must never be `Color32::TRANSPARENT`" (`egui/src/style.rs:1292-1294`) — is real platform
/// data, written as given and reported once per leaf (§7.2); nothing is substituted. `written`
/// is the colour as it lands in the field, after any composite (§6.1), and `path` the leaf
/// whose value it is.
pub(crate) fn note_transparent_fill(
    written: egui::Color32,
    path: &'static str,
    notes: &mut Vec<Note>,
) {
    if written.a() == 0 {
        push_note(notes, Note::TransparentFill { path });
    }
}

/// §7.2's call-site test for a length `convert::i8_from_f32_saturating` will clamp: the two
/// values `round()` carries outside `-128..=127` (§6.17). The caller reports
/// `Note::ValueSaturated`, because the helper stays pure and single-valued.
pub(crate) fn saturates_i8(v: f32) -> bool {
    let d = crate::convert::denan(v);
    d >= 127.5 || d <= -128.5
}
