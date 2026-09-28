//! The Basic page, the ten palette pages (spec §10.4) and the Theme Map.

pub(crate) mod basic;
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
pub(crate) mod theme_map;

use crate::app::{App, Page};
use crate::demo::{self, Registry};
use native_theme_egui::ThemeAtlas;

/// The date the date picker starts from: a datum, evaluated at compile time, so a date
/// `Date::constant` rejected would fail the build, never the run.
const DEMO_DATE: jiff::civil::Date = jiff::civil::Date::constant(2026, 9, 26);

/// What `CodeTheme::from_style` reads from a style: whether it is dark, and the font
/// (`egui_extras/src/syntax_highlighting.rs:237-248`).
pub(crate) type CodeThemeKey = (bool, egui::FontId);

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
    /// The code view's theme, with the scheme and `Monospace` font it was made from.
    pub code_theme: Option<(CodeThemeKey, egui_extras::syntax_highlighting::CodeTheme)>,
    pub table_selected: Option<usize>,
    pub window_open: bool,
    pub modal_open: bool,
    /// The part of the `Scene` in view; `Rect::ZERO` lets the scene fit its contents first.
    pub scene_rect: egui::Rect,
    /// The drag-and-drop demo: the items, and which column each sits in.
    pub dnd_columns: [Vec<&'static str>; 2],
    /// The Theme Map's verdict filter; `None` lists every row.
    pub theme_map_filter: Option<crate::info::Verdict>,
    /// The Basic page's radio button, text fields, drop-down and slider.
    pub basic_radio: usize,
    pub basic_hint: String,
    pub basic_text: String,
    pub basic_combo: usize,
    pub basic_slider: f32,
}

impl Default for DemoState {
    fn default() -> Self {
        let colour = demo_colour();
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
            window_open: false,
            modal_open: false,
            scene_rect: egui::Rect::ZERO,
            dnd_columns: [vec!["First", "Second"], vec!["Third"]],
            theme_map_filter: None,
            basic_radio: 0,
            basic_hint: String::new(),
            basic_text: "Text".to_string(),
            basic_combo: 0,
            basic_slider: 40.0,
        }
    }
}

/// The colour the Colour page's editors start from: the datum on display, not a style value.
fn demo_colour() -> egui::Color32 {
    egui::Color32::from_rgb(0x3d, 0x8b, 0xd1)
}

/// A heading above a group of items, as the gpui showcase's `demo::heading` and the iced
/// showcase's section titles: the theme's section-heading role, in the text colour
/// (`demo::heading_text`), not `ui.strong`, whose colour is the active-state text colour
/// (`egui/src/style.rs:1147-1149`); a `Label` in the base style, recorded like any other. Every
/// heading but a page's first is `layout.section_gap` below the section before it, where the
/// theme states that gap: the theme's space between sections (parity item 11).
pub(crate) fn caption(reg: &mut Registry, ui: &mut egui::Ui, text: &str) {
    let atlas = ThemeAtlas::from_ctx(ui.ctx());
    let section_gap = atlas.as_ref().and_then(|atlas| atlas.layout().section_gap);
    let first = ui.min_rect().height() <= 0.0;
    if let Some(gap) = section_gap.filter(|_| !first) {
        ui.add_space(gap);
    }
    demo::base(reg, ui, "heading", |ui| {
        let heading = demo::heading_text(ui, text);
        ui.label(heading)
    });
    let role = atlas.map(|atlas| {
        let t = atlas.resolved_for(ui.ctx().theme());
        let prefs = atlas.accessibility();
        let r = native_theme_egui::TextRole::SectionHeading;
        let weight = native_theme_egui::text_role_weight(t, r);
        (
            native_theme_egui::text_role_font(t, r, prefs),
            native_theme_egui::text_role_line_height(t, r, prefs),
            weight,
            demo::weighted_family(ui, weight, t.defaults.font.weight),
        )
    });
    reg.amend_last(|i| {
        i.read.push((
            "layout.section_gap",
            section_gap.map_or_else(|| "not stated: no gap".to_string(), |g| g.to_string()),
        ));
        if let Some((font, line_height, weight, family)) = role {
            i.read.push(("text_role_font", format!("{font:?}")));
            i.read
                .push(("text_role_line_height", format!("{line_height}")));
            i.read.push(("text_role_weight", format!("{weight}")));
            let drawn = match family {
                egui::FontFamily::Proportional => "the body font's face",
                _ => "the OS's face of the body family at that weight",
            };
            i.notes.push(("weight drawn in", drawn.to_string()));
        }
    });
}

/// What the page draws above its `ScrollArea`, which never scrolls: the Containers page's
/// nested panels, which egui does not clip to a scrolled page (`containers::nested_panels`).
pub(crate) fn show_fixed(app: &mut App, ui: &mut egui::Ui) {
    if app.settings.page == Page::Containers {
        containers::nested_panels(&mut app.registry, ui);
    }
}

pub(crate) fn show(app: &mut App, ui: &mut egui::Ui) {
    let chosen = app.chosen_icons();
    // Disjoint borrows: the registry, the demo state and the atlas are separate fields (§10.4's fourth rule).
    let App {
        registry,
        demo_state,
        atlas,
        settings,
        manifest,
        json,
        ..
    } = app;
    match settings.page {
        Page::Basic => basic::show(registry, demo_state, atlas, ui),
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
        Page::ThemeMap => theme_map::show(registry, demo_state, atlas, manifest, json, ui),
    }
}
