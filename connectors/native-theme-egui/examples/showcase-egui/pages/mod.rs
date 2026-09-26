//! The ten palette pages (spec §10.4).

pub(crate) mod buttons;
pub(crate) mod colour;
pub(crate) mod containers;
pub(crate) mod data;
pub(crate) mod icons;
pub(crate) mod inputs;
pub(crate) mod overlays;
pub(crate) mod range;
pub(crate) mod selection;
pub(crate) mod text;

use crate::app::{App, Page};
use crate::demo::{self, Registry};

/// The date the date picker starts from: a datum, evaluated at compile time, so a date
/// `Date::constant` rejected would fail the build, never the run.
const DEMO_DATE: jiff::civil::Date = jiff::civil::Date::constant(2026, 9, 26);

/// What the pages edit; the datum on display, never a theme value. Not `Debug`:
/// egui_extras's `CodeTheme` is not.
pub(crate) struct DemoState {
    pub checked: bool,
    pub indeterminate: bool,
    pub toggle: bool,
    pub switch: bool,
    pub segment: usize,
    pub selectable: usize,
    pub radio: usize,
    pub combo: usize,
    pub text: String,
    pub hint: String,
    pub multiline: String,
    pub password: String,
    pub code: String,
    pub number: f32,
    pub angle: f32,
    pub date: jiff::civil::Date,
    pub slider: f32,
    pub vertical: f32,
    pub progress: f32,
    pub colour: egui::Color32,
    pub hsva: egui::ecolor::Hsva,
    pub srgb: [u8; 3],
    pub rgb: [f32; 3],
    pub srgba_premultiplied: [u8; 4],
    pub srgba_unmultiplied: [u8; 4],
    pub rgba_premultiplied: [f32; 4],
    pub rgba_unmultiplied: [f32; 4],
    pub code_theme: Option<egui_extras::syntax_highlighting::CodeTheme>,
    pub table_selected: Option<usize>,
    pub window_open: bool,
    pub modal_open: bool,
    /// The part of the `Scene` in view; `Rect::ZERO` lets the scene fit its contents first.
    pub scene_rect: egui::Rect,
    /// The drag-and-drop demo: the items, and which column each sits in.
    pub dnd_columns: [Vec<&'static str>; 2],
}

impl Default for DemoState {
    fn default() -> Self {
        let colour = egui::Color32::from_rgb(0x3d, 0x8b, 0xd1);
        let rgba = egui::Rgba::from(colour);
        let [r, g, b, a] = colour.to_srgba_unmultiplied();
        let unmultiplied = [rgba.r(), rgba.g(), rgba.b(), rgba.a()];
        Self {
            checked: true,
            indeterminate: false,
            toggle: false,
            switch: true,
            segment: 0,
            selectable: 0,
            radio: 0,
            combo: 0,
            text: "Single line".to_string(),
            hint: String::new(),
            multiline: "First line\nSecond line".to_string(),
            password: "secret".to_string(),
            code: "fn main() {\n    println!(\"hello\");\n}".to_string(),
            number: 42.0,
            angle: std::f32::consts::FRAC_PI_4,
            date: DEMO_DATE,
            slider: 40.0,
            vertical: 60.0,
            progress: 0.4,
            colour,
            hsva: egui::ecolor::Hsva::from(colour),
            srgb: [r, g, b],
            rgb: [rgba.r(), rgba.g(), rgba.b()],
            srgba_premultiplied: colour.to_array(),
            srgba_unmultiplied: [r, g, b, a],
            rgba_premultiplied: rgba.to_array(),
            rgba_unmultiplied: unmultiplied,
            code_theme: None,
            table_selected: Some(1),
            window_open: true,
            modal_open: false,
            scene_rect: egui::Rect::ZERO,
            dnd_columns: [vec!["First", "Second"], vec!["Third"]],
        }
    }
}

/// A caption above a group of items: a `Label` in the base style, recorded like any other.
pub(crate) fn caption(reg: &mut Registry, ui: &mut egui::Ui, text: &str) {
    demo::base(reg, ui, "caption", |ui| ui.strong(text));
}

pub(crate) fn show(app: &mut App, ui: &mut egui::Ui) {
    let chosen = app.chosen_icons();
    // Disjoint borrows: the registry, the demo state and the atlas are separate fields (§10.4's fourth rule).
    let App {
        registry,
        demo_state,
        atlas,
        settings,
        ..
    } = app;
    match settings.page {
        Page::Buttons => buttons::show(registry, demo_state, atlas, ui, &chosen),
        Page::Selection => selection::show(registry, demo_state, atlas, ui),
        Page::Inputs => inputs::show(registry, demo_state, atlas, ui),
        Page::Range => range::show(registry, demo_state, atlas, ui),
        Page::Text => text::show(registry, demo_state, atlas, ui),
        Page::Colour => colour::show(registry, demo_state, atlas, ui),
        Page::Containers => containers::show(registry, demo_state, atlas, ui),
        Page::Data => data::show(registry, demo_state, atlas, ui),
        Page::Overlays => overlays::show(registry, demo_state, atlas, ui, &chosen),
        Page::Icons => icons::show(registry, demo_state, atlas, ui, &chosen),
    }
}
