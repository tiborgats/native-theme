//! What the inspector shows for one widget instance (spec §3).

use gpui::{App, Hsla, StyleRefinement, Styled};
use native_theme_gpui::Native;

use crate::support::{native_geometry, refined};

pub mod buttons;
pub mod charts;
pub mod chrome;
pub mod data;
pub mod feedback;
pub mod icons;
pub mod inputs;
pub mod layout;
pub mod overlays;
pub mod registry;
pub mod text;
pub mod theme_map;
pub mod typography;
pub use chrome::*;
pub use registry::*;

/// One colour a widget paints, the token it reads, and where upstream reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorClaim {
    pub role: &'static str,
    pub field: &'static str,
    pub value: Hsla,
    /// `<crate>/<path>.rs:<line>`, or `showcase` for a colour the showcase
    /// itself chooses. Read by `every_colour_claim_is_read_at_the_line_it_cites`.
    pub cited_at: &'static str,
}

/// The only way a claim is written, so the gates can find every one.
pub fn claim(
    role: &'static str,
    field: &'static str,
    value: Hsla,
    cited_at: &'static str,
) -> ColorClaim {
    ColorClaim {
        role,
        field,
        value,
        cited_at,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub what: &'static str,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WidgetInfo {
    pub kind: &'static str,
    pub variant: Option<String>,
    pub colors: Vec<ColorClaim>,
    pub config: Vec<Note>,
    pub not_themeable: Vec<Note>,
    pub instance: Vec<Note>,
}

impl WidgetInfo {
    pub fn new(kind: &'static str) -> Self {
        Self {
            kind,
            ..Self::default()
        }
    }
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }
    pub fn color(mut self, claim: ColorClaim) -> Self {
        self.colors.push(claim);
        self
    }
    pub fn colors(mut self, claims: impl IntoIterator<Item = ColorClaim>) -> Self {
        self.colors.extend(claims);
        self
    }
    pub fn config(mut self, what: &'static str, text: impl Into<String>) -> Self {
        self.config.push(Note {
            what,
            text: text.into(),
        });
        self
    }
    pub fn not_themeable(mut self, what: &'static str, text: impl Into<String>) -> Self {
        self.not_themeable.push(Note {
            what,
            text: text.into(),
        });
        self
    }
    pub fn instance(mut self, what: &'static str, text: impl Into<String>) -> Self {
        self.instance.push(Note {
            what,
            text: text.into(),
        });
        self
    }
    /// The "geometry" line for the `geometry::` builder named `builder`,
    /// from [`GEOMETRY_NOTES`]; the only way such a line is written.
    pub fn geometry(self, builder: &'static str) -> Self {
        let what = GEOMETRY_NOTES
            .iter()
            .find(|(name, _)| *name == builder)
            .map_or("(no GEOMETRY_NOTES entry)", |(_, what)| *what);
        self.config("geometry", format!("geometry::{builder}: {what}"))
    }
    pub fn title(&self) -> String {
        match &self.variant {
            Some(v) => format!("{} · {v}", self.kind),
            None => self.kind.to_string(),
        }
    }
    /// The plain-text form: the Copy button's payload and what tests read.
    pub fn to_text(&self) -> String {
        let mut s = format!("{}\n", self.title());
        if !self.colors.is_empty() {
            s.push_str("\nTheme colors:\n");
            for c in &self.colors {
                s.push_str(&format!(
                    "  {}: {} {} ({})\n",
                    c.role,
                    c.field,
                    hsla_to_hex(c.value),
                    c.cited_at
                ));
            }
        }
        for (heading, notes) in [
            ("Theme config:", &self.config),
            ("Not themeable:", &self.not_themeable),
            ("This instance:", &self.instance),
        ] {
            if !notes.is_empty() {
                s.push_str(&format!("\n{heading}\n"));
                for n in notes {
                    s.push_str(&format!("  {}: {}\n", n.what, n.text));
                }
            }
        }
        s
    }
}

/// What each `geometry::` builder carries, one entry per `pub fn` of
/// `native-theme-gpui/src/geometry.rs` in its order, the value builders
/// included: `WidgetInfo::geometry` prints `geometry::<name>: <what>`.
/// `every_geometry_builder_has_a_note` holds it to geometry.rs.
pub const GEOMETRY_NOTES: &[(&str, &str)] = &[
    (
        "button",
        "button.min_height by the control-height rule (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), min_width, the border.padding sides the theme states, corner_radius, line_width, color, and button.font.weight but not its size (spec §9.2)",
    ),
    (
        "input",
        "input.min_height by the control-height rule (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), the border.padding sides the theme states -- which reach the field, because upstream pads it before the refinement (input/input.rs, Input::render: input_px then refine_style) --, border.corner_radius, line_width, input.font",
    ),
    (
        "menu_item",
        "menu.row_height, where the theme states one, by the control-height rule (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), the menu.border.padding sides the theme states, menu.icon_text_gap, menu.font",
    ),
    (
        "list_item",
        "list.row_height, where the theme states one, by the control-height rule (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), the list.border.padding sides the theme states, and list.item_font including its colour -- upstream labels the row with foreground before applying it (list/list_item.rs, ListItem::render)",
    ),
    (
        "tooltip",
        "the tooltip.border.padding sides the theme states, corner_radius, tooltip.font — including its colour, which upstream would otherwise paint with popover_foreground (tooltip.rs, Tooltip::render: text_color then refine_style)",
    ),
    (
        "tooltip_content",
        "tooltip.max_width less the bubble's left and right padding -- the theme's where stated, upstream's px_2 (0.5 rem at the installed font size) where not (tooltip.rs, Tooltip::render) -- and its 1px border (tooltip.rs, Tooltip::render: border_1), as the max width of the element passed to Tooltip::element, which the text wraps at",
    ),
    (
        "popover",
        "the popover.border.padding sides the theme states, and corner_radius; an unstated side keeps upstream's p_3 (popover.rs, Popover::render)",
    ),
    (
        "status_bar",
        "the status_bar.border.padding sides the theme states, status_bar.font — including its colour, which upstream would otherwise paint with muted_foreground (status_bar.rs, StatusBar::render: text_color then refine_style)",
    ),
    (
        "dialog",
        "the dialog.border.padding sides the theme states (an unstated side keeps upstream's 16px; upstream also spaces the sections by max(top, 8px) and a DialogContent by the bottom padding), min_height, max_height and border.corner_radius; the max_height does not reach upstream's Dialog, which sets its own max_h after the refinement (dialog/dialog.rs, RenderOnce for Dialog)",
    ),
    (
        "input_group_button",
        "button.border.corner_radius alone. gpui-component scales a control's radius with its size, so an in-group button would take radius/2 (input/group.rs, InputGroupButton::render_in_group), where the model records one radius per widget",
    ),
    (
        "dialog_footer",
        "dialog.button_gap between the buttons (dialog/footer.rs, DialogFooter::render)",
    ),
    ("dialog_title", "dialog.title_font"),
    (
        "dialog_description",
        "dialog.body_font including its colour, which upstream would otherwise paint with muted_foreground (dialog/description.rs, DialogDescription::render)",
    ),
    (
        "list",
        "list.border line width, colour and corner radius, and a clip to that radius",
    ),
    (
        "table",
        "list.item_font on the table root (table/table.rs, Table::render: text_sm then refine_style)",
    ),
    (
        "progress",
        "progress_bar.track_height, border.corner_radius, min_width",
    ),
    (
        "group_box_content",
        "the card.border.padding sides the theme states, corner_radius, line_width, color -- refined last, so the radius and the edge drawn are the card's, not upstream's radius (group_box.rs, GroupBox)",
    ),
    ("accordion_title", "expander.header_height"),
    ("checkbox", "checkbox.label_gap, checkbox.font"),
    (
        "radio",
        "checkbox.label_gap, and checkbox.font including its colour, which the label takes because upstream sets foreground on the row and mutes the label child instead (platform-facts §2.5: radio metrics are the checkbox's)",
    ),
    (
        "select",
        "combo_box.min_height by the control-height rule (defaults.line_height as the line height; the stated height as a minimum at a text scale of 1 or less, where upstream's own trigger height stands if larger, and a minimum with an automatic height above 1), min_width, the border.padding sides the theme states, border.corner_radius, combo_box.font -- and, unlike geometry::combobox, the font's colour as well (native-theme-gpui geometry.rs, select)",
    ),
    (
        "combobox",
        "combo_box.min_height by the control-height rule (defaults.line_height as the line height; the stated height as a minimum at a text scale of 1 or less, where upstream's own trigger height stands if larger, and a minimum with an automatic height above 1), min_width, the border.padding sides the theme states, border.corner_radius, combo_box.font",
    ),
    (
        "title_bar",
        "window.title_bar_font size, weight and colour -- upstream sets no text colour on the bar, so the colour displaces nothing (title_bar.rs, RenderOnce for TitleBar). The window controls set foreground on their own elements, out of its reach (title_bar.rs, RenderOnce for ControlIcon)",
    ),
    (
        "toolbar",
        "toolbar.bar_height as the minimum height where the theme states one, item_gap, the border.padding sides the theme states, background_color, font size and weight",
    ),
    ("spinner_size", "spinner.diameter"),
    (
        "icon_size_toolbar",
        "toolbar.icon_size, which inherits defaults.icon_sizes.toolbar",
    ),
    ("icon_size_small", "defaults.icon_sizes.small"),
    ("icon_size_large", "defaults.icon_sizes.large"),
    ("icon_size_dialog", "defaults.icon_sizes.dialog"),
    ("icon_size_panel", "defaults.icon_sizes.panel"),
    (
        "scrollbar_gutter",
        "scrollbar.groove_width as right padding where scrollbar.overlay_mode is false, and nothing where it is true, because gpui-component overlays the bar on the scroll area instead of putting it beside the content",
    ),
    (
        "dialog_max_width",
        "dialog.max_width, which caps the width: a Dialog's through Dialog::max_w (dialog/dialog.rs, Dialog::max_w), and an AlertDialog's, which has no max_w of its own, through Styled::max_w, landing on the surface with the rest of the refinement (dialog/alert_dialog.rs, AlertDialog; dialog/dialog.rs, Dialog::render)",
    ),
    (
        "input_height",
        "input.min_height by the control-height rule geometry::input applies (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), and nothing else of geometry::input",
    ),
    (
        "widget_gap",
        "layout.widget_gap, the space between adjacent widgets; none where the platform specifies none (platform-facts §2.20)",
    ),
    (
        "container_margin",
        "layout.container_margin, the padding inside containers; none where the platform specifies none (platform-facts §2.20)",
    ),
    (
        "window_margin",
        "layout.window_margin, the padding inside the main window; none where the platform specifies none (platform-facts §2.20)",
    ),
    (
        "section_gap",
        "layout.section_gap, the space between major content sections; none where the platform specifies none (platform-facts §2.20)",
    ),
];

/// `w` with the `build` refinement applied and `info` given `build`'s
/// geometry line, both only when a native theme is installed; before `apply`
/// ran the widget keeps upstream's geometry and `info` names no builder.
/// `name` is `build`'s name, which `native_info_names_the_builder_it_applies`
/// holds to it.
pub fn native_info<W: Styled>(
    w: W,
    cx: &App,
    build: fn(Native<'_>) -> StyleRefinement,
    name: &'static str,
    info: &mut WidgetInfo,
) -> W {
    let style = native_geometry(cx, build);
    if style.is_some() {
        *info = std::mem::take(info).geometry(name);
    }
    refined(w, style.as_ref())
}

/// A size in logical pixels to two decimals, trailing zeros dropped: a rem
/// multiple of a 13.333333px font_size prints as what it is to the eye,
/// not as float noise.
pub fn px_text(v: f32) -> String {
    let s = format!("{v:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// An opacity as a percentage, the way `px_text` prints a size: the prose
/// derived from the value, so it cannot say another.
pub fn percent_text(opacity: f32) -> String {
    format!("{}%", px_text(opacity * 100.))
}

/// `c` as `#rrggbb`, or as `#rrggbbaa` where it is not opaque: a colour
/// painted at 80% printed without its alpha would claim a fill the widget
/// never paints.
pub fn hsla_to_hex(c: Hsla) -> String {
    // Convert HSL to RGB through gpui's Rgba
    let rgba: gpui::Rgba = c.into();
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    let (r, g, b) = (byte(rgba.r), byte(rgba.g), byte(rgba.b));
    if rgba.a < 1.0 {
        format!("#{r:02x}{g:02x}{b:02x}{:02x}", byte(rgba.a))
    } else {
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}
