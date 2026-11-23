//! What the inspector shows for one widget instance (spec §3).

use gpui::{App, Hsla, Rems, StyleRefinement, Styled};
use gpui_component::theme::Theme;
use native_theme_gpui::Native;
use native_theme_gpui::next::{Mapped, Unmapped};
/// The reads a claim in the 0.8.0 form makes of its home's property: the
/// connector's own (`native_theme_gpui::next`), each `None` where the theme
/// states no value gpui can take there, unstated or in a form gpui cannot
/// draw, so the panel shows the toolkit's own there (spec §10, rule 0).
pub use native_theme_gpui::next::{
    radius_of, state_stroke_color_of, stroke_color_of, stroke_width_of,
};

use crate::support::{native_geometry, next_value, refined};

pub mod buttons;
pub mod charts;
pub mod chrome;
pub mod data;
pub mod feedback;
pub mod icons;
pub mod inputs;
pub mod layout;
pub mod leaves;
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

/// A colour a claim shows: one of the theme's, which gpui paints as
/// [`stated`] converts it, or one gpui already holds -- a field of a
/// connector Look (`native_theme_gpui::widgets`), the colour a theme-drawn
/// widget paints.
pub trait Swatch {
    fn hsla(self) -> Hsla;
}

impl Swatch for native_theme::color::Rgba {
    fn hsla(self) -> Hsla {
        stated(self)
    }
}

impl Swatch for Hsla {
    fn hsla(self) -> Hsla {
        self
    }
}

/// The claim of a colour the theme may leave unstated, `None` where it does.
pub fn stated_claim(
    role: &'static str,
    field: &'static str,
    colour: Option<impl Swatch>,
    cited_at: &'static str,
) -> Option<ColorClaim> {
    colour.map(|colour| claim(role, field, colour.hsla(), cited_at))
}

/// The claim of a line in `colour` drawn `width` wide, `None` where either
/// is unstated or the width is 0: no line is drawn. `width` is the drawing's
/// (a text area's hovered edge is `input.hover_border_color` at
/// `text_area.border.line_width`); the caller's config line names it, or
/// says `none: <table>.border.line_width is 0`.
pub fn edge_claim(
    role: &'static str,
    field: &'static str,
    colour: Option<impl Swatch>,
    width: Option<f32>,
    cited_at: &'static str,
) -> Option<ColorClaim> {
    let drawn = width.is_some_and(|width| width > 0.0);
    stated_claim(role, field, colour.filter(|_| drawn), cited_at)
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
    /// The element of docs/showcase-elements.toml the widget is, if it is
    /// one: Widget Info then shows it in the list's form
    /// (`crate::inspector::listed_sections`), and the layout dump records
    /// where it was drawn.
    pub listed: Option<&'static str>,
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
    /// The widget is the element `id` of docs/showcase-elements.toml.
    pub fn listed(mut self, id: &'static str) -> Self {
        self.listed = Some(id);
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
    /// The "geometry" line for the `next::geometry` builder named `builder`,
    /// from [`NEXT_GEOMETRY_NOTES`]; the only way such a line is written on
    /// the 0.8.0 model.
    pub fn next_geometry(self, builder: &'static str) -> Self {
        let what = NEXT_GEOMETRY_NOTES
            .iter()
            .find(|(name, _)| *name == builder)
            .map_or("(no NEXT_GEOMETRY_NOTES entry)", |(_, what)| *what);
        self.config("geometry", format!("geometry::{builder}: {what}"))
    }
    /// The "geometry" line for the `next::geometry` builder named `builder`
    /// where the installed 0.8.0 theme states no value gpui can take at
    /// `unstated`'s home, which the builder needs: the widget keeps
    /// gpui-component's own (spec §10, rule 0).
    pub fn next_unmapped(self, builder: &'static str, unstated: Unmapped) -> Self {
        self.config(
            "geometry",
            format!(
                "gpui-component's own: the theme states no value gpui can take at {}, which geometry::{builder} needs",
                unstated.home()
            ),
        )
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
                    as_rgba(c.value),
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
/// included, but for those whose every caller is on the 0.8.0 model, which
/// [`NEXT_GEOMETRY_NOTES`] alone describes: `WidgetInfo::geometry` prints
/// `geometry::<name>: <what>`. `every_geometry_builder_has_a_note` holds the
/// two tables to geometry.rs.
pub const GEOMETRY_NOTES: &[(&str, &str)] = &[
    (
        "button",
        "button.min_height by the control-height rule (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), min_width, the border.padding sides the theme states, corner_radius, line_width, color, and button.font.weight but not its size (spec §9.2)",
    ),
    (
        "editor",
        "defaults.line_height as the line height, over the relative(1.5) upstream's Editor sets on its Input first: the Editor refines the caller's style last (input/editor.rs, Editor::render). Its font is the mono font the theme sets, mono_font_family at mono_font_size",
    ),
    (
        "menu_item",
        "menu.row_height, where the theme states one, by the control-height rule (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), else menu.min_row_height as a least height the content grows past, the menu.border.padding sides the theme states, menu.icon_text_gap, menu.font",
    ),
    (
        "list_item",
        "list.row_height, where the theme states one, by the control-height rule (defaults.line_height as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), the list.border.padding sides the theme states, and list.item_font including its colour -- upstream labels the row with foreground before applying it (list/list_item.rs, ListItem::render)",
    ),
    (
        "tooltip",
        "the tooltip.border.padding sides the theme states, corner_radius, tooltip.font — including its colour, which upstream would otherwise paint with popover_foreground —, defaults.line_height as the line height (upstream sets none, so wrapped text was spaced by gpui's phi), tooltip.background_color and tooltip.border.color, which upstream would otherwise paint with the popover's fill and the window's border, the edge tooltip.border.line_width wide in place of upstream's border_1, and shadow_md's two layers in defaults.shadow_color where tooltip.border.shadow_enabled holds, no shadow where it does not, in place of upstream's shadow_md (gpui-component/tooltip.rs, Tooltip::render: bg, text_color, border_1, border_color and shadow_md, then refine_style)",
    ),
    (
        "tooltip_content",
        "tooltip.max_width less the bubble's left and right padding -- the theme's where stated, upstream's px_2 (0.5 rem at the installed font size) where not (gpui-component/tooltip.rs, Tooltip::render) -- and its edge, tooltip.border.line_width on each side, which geometry::tooltip draws in place of upstream's border_1 (gpui-component/tooltip.rs, Tooltip::render), as the max width of the element passed to Tooltip::element, which the text wraps at",
    ),
    (
        "popover",
        "the popover.border.padding sides the theme states, corner_radius, and the edge, popover.border.color at line_width, inside the shadow ring popover_style draws; an unstated side keeps upstream's p_3 (gpui-component/popover.rs, Popover::render)",
    ),
    (
        "status_bar",
        "the status_bar.border.padding sides the theme states, status_bar.font — including its colour, which upstream would otherwise paint with muted_foreground (status_bar.rs, StatusBar::render: text_color then refine_style) — and defaults.line_height as the line height",
    ),
    (
        "dialog",
        "the dialog.border.padding sides the theme states (an unstated side keeps upstream's 16px; upstream also spaces the sections by max(top, 8px) and a DialogContent by the bottom padding), min_width as the least width, which the width upstream sets after the refinement keeps, min_height, max_height, border.corner_radius, and background_color and border.color over the window's background and border upstream paints the surface with first; the max_height does not reach upstream's Dialog, which sets its own max_h after the refinement (dialog/dialog.rs, RenderOnce for Dialog)",
    ),
    (
        "dialog_entrance",
        "dialog.entrance as the nearest entrance gpui-component draws, through Dialog::entrance and AlertDialog::entrance: DialogEntrance::Fade, a fade in place, for both styles the model has. scale_fade loses its scale, since gpui-component has no scaling entrance, and fade_rise its rise, since FadeSlide settles from just above the dialog's place instead (dialog/entrance.rs, DialogEntrance; dialog/dialog.rs, Dialog::entrance)",
    ),
    (
        "dialog_footer",
        "dialog.button_gap between the buttons (dialog/footer.rs, DialogFooter::render)",
    ),
    (
        "dialog_title",
        "dialog.title_font including its colour, which displaces nothing, upstream setting none on the title, and defaults.line_height as the line height, which replaces upstream's relative(1.25) (dialog/title.rs, RenderOnce for DialogTitle: line_height then refine_style)",
    ),
    (
        "dialog_description",
        "dialog.body_font including its colour, which upstream would otherwise paint with muted_foreground (dialog/description.rs, DialogDescription::render), and defaults.line_height as the line height",
    ),
    (
        "list",
        "list.border line width, colour and corner radius, a clip to that radius, and list.background_color as the fill: neither List nor Tree paints one, nor a ListItem at rest (list/list_item.rs, ListItem::render)",
    ),
    (
        "table",
        "list.item_font and defaults.line_height as the line height on the table root (table/table.rs, Table::render: text_sm then refine_style)",
    ),
    (
        "progress",
        "progress_bar.track_height, border.corner_radius, min_width",
    ),
    (
        "group_box_content",
        "the card.border.padding sides the theme states, corner_radius, line_width, color, and card.background_color -- refined last, so the radius, the edge and the fill drawn are the card's, not upstream's radius or the Fill variant's group_box token (group_box.rs, GroupBox)",
    ),
    (
        "accordion_title",
        "expander.header_height, and expander.font.color over the foreground upstream sets on an open item's title row first; a closed one's sets none (gpui-component/accordion.rs, AccordionItem::render)",
    ),
    (
        "accordion",
        "expander.border.color on the Accordion's card and on each item's line under it, over the border token upstream draws both in first (gpui-component/accordion.rs, RenderOnce for Accordion and AccordionItem::render); where neither is drawn, nothing",
    ),
    (
        "accordion_hover",
        "expander.hover_background as an item's title row fill under the pointer, through AccordionItem::hover, where the theme states one; nothing where it does not, so the row stays at rest (gpui-component/accordion.rs, AccordionItem::hover)",
    ),
    (
        "select",
        "combo_box.min_height by the control-height rule (defaults.line_height as the line height; the stated height itself at a text scale of 1 or less, and a minimum with an automatic height above 1), min_width, the border.padding sides the theme states -- but not the right side where combo_box.arrow_area_width is stated: the platform measures it to an arrow column the trigger does not have, its caret sitting inside the padded row, so upstream's own right padding stands --, border.corner_radius, combo_box.font -- and, unlike geometry::combobox, the font's colour as well (native-theme-gpui/geometry.rs, select)",
    ),
    (
        "combobox",
        "combo_box.min_height by the control-height rule (defaults.line_height as the line height; the stated height itself at a text scale of 1 or less, and a minimum with an automatic height above 1), min_width, the border.padding sides the theme states -- but not the right side where combo_box.arrow_area_width is stated: the platform measures it to an arrow column the trigger does not have, its caret sitting inside the padded row, so upstream's own right padding stands --, border.corner_radius, combo_box.font (native-theme-gpui/geometry.rs, combobox)",
    ),
    (
        "title_bar",
        "window.title_bar_font's family, size, weight and style, and its colour, and defaults.line_height as the line height -- upstream sets no text colour on the bar, so the colour displaces nothing (title_bar.rs, RenderOnce for TitleBar). The window controls set foreground on their own elements, out of its reach (title_bar.rs, RenderOnce for ControlIcon)",
    ),
    (
        "title_bar_inactive",
        "window.inactive_title_bar_background, drawn as upstream's gradient is from title_bar, and window.inactive_title_bar_text_color over geometry::title_bar's text, for a TitleBar while Window::is_window_active is false: TitleBar reads no window activation, painting every bar alike (title_bar.rs, default_title_bar_background and RenderOnce for TitleBar)",
    ),
    (
        "toolbar",
        "toolbar.bar_height as the minimum height where the theme states one, item_gap where it states one, the border.padding sides the theme states, background_color, the bottom edge, border.color at border.line_width, toolbar.font's family, size, weight and style, and its colour -- upstream's Toolbar sets no colour (gpui-component/toolbar.rs, RenderOnce for Toolbar) --, and defaults.line_height as the line height",
    ),
    ("spinner_size", "spinner.diameter"),
    (
        "data_table_size",
        "list.row_height as Size::Size, which table_row_height returns verbatim, or upstream's Medium 32px where the theme states none (sizing.rs, table_row_height); a Size::Size cell sets no text size and inherits the container's, which geometry::table gives list.item_font, where a Medium cell is text_sm (sizing.rs, table_cell_size); above a text scale of 1 at least list.item_font's scaled line box at defaults.line_height, upstream's own cell padding (sizing.rs, table_cell_padding) and the row's 1px bottom border, which upstream draws inside the row (table/state.rs, border_b_1), so the row holds its text",
    ),
    (
        "list_header_height",
        "list.header_height, the height of a table's header row, which DataTable::header_row_height takes (table/data_table.rs, DataTable::header_row_height); where the theme states none, nothing is set and the header keeps the body rows' height (table/state.rs, header_row_height); above a text scale of 1 at least list.item_font's scaled line box at defaults.line_height, upstream's own cell padding and the header row's 1px bottom border (table/state.rs, border_b_1): a Size::Size header cell sets no text size and inherits the container's, which geometry::table gives list.item_font (sizing.rs, table_cell_size)",
    ),
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
        "toolbar_button_rest",
        "where the theme states toolbar.button_hover_border_color, a transparent fill and a transparent border button.border.line_width wide, and each stated side of button.border.padding less that width: a custom variant draws its colour as the border in every state and lays out no border width (button/button.rs, ButtonCustomVariant and RenderOnce for Button), so this hides the edge at rest and keeps the button's size; nothing where the theme states no outline",
    ),
    (
        "toolbar_button_checked",
        "toolbar.button_checked_background and button.checked_text_color (button.active_text_color where that is unset) on a selected toolbar Button built on variants::toolbar_button; where the theme states no checked fill, toolbar.button_active_background, else button.active_background, with no text colour; nothing where it states none of the three: a custom variant fills a selected Button with its active colour and letters every state in its one foreground (button/button.rs, ButtonCustomVariant and RenderOnce for Button), and the Button replays the refinement over its selected style. Its border colour is toolbar.button_checked_border_color where stated, at toolbar_button_rest's width, and stays under the pointer and pressed, where a selected Button takes no hover or press style",
    ),
    (
        "default_button_rest",
        "button.default_button_background as the fill and button.default_button_border_color (button.border.color where unstated) as the border colour of a dialog's default Button at rest, over variants::default_button, whose rest fill is a fifth of its border colour (button/button.rs, ButtonVariant::bg_color); nothing where the theme states no fill, and the Button is .primary()",
    ),
    (
        "list_item_selected",
        "list.selection_text_color on a selected ListItem's text, over the foreground upstream sets on every row before the refinement -- or geometry::list_item's list.item_font colour: while list.active_highlight is on, gpui-component's default, upstream marks a selected row by its fill alone, painted after the refinement; with it off, it letters the row accent_foreground on accent after the refinement, so this colour does not reach it; and it mutes a row that is not selectable after the refinement too (list/list_item.rs, ListItem::render)",
    ),
    (
        "link",
        "link.font's family, size, weight and style, defaults.line_height as the line height, link.background_color as the fill, which upstream paints none of (gpui-component/link.rs, RenderOnce for Link), and no underline where link.underline_enabled is false: upstream underlines a link at rest before the refinement (gpui-component/link.rs, RenderOnce for Link: text_decoration_1 then refine_style), and a zero thickness paints none",
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

/// [`GEOMETRY_NOTES`] for the builders of `native_theme_gpui::next::geometry`
/// a widget on the 0.8.0 model records, each naming the homes its builder
/// reads: `WidgetInfo::next_geometry` prints `geometry::<name>: <what>`. A
/// builder whose every caller is on the 0.8.0 model has its entry here alone.
pub const NEXT_GEOMETRY_NOTES: &[(&str, &str)] = &[
    (
        "button",
        "button.body(NONE).min_size[1] by the control-height rule (typography.line_height_multiplier as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1; upstream's height where the theme states none), button.body(NONE).min_size[0], the padding sides button.body(NONE) states, its corner radius, stroke width and stroke colour, and button.label(NONE).weight but not its size (spec §9.2)",
    ),
    (
        "button_label",
        "button.label(NONE)'s family, size, weight and style, and typography.line_height_multiplier as the line height, on a label passed as the Button's child: upstream sizes a .label() from the Size enum on the content element -- text_sm at Size::Medium -- which a style on the Button does not reach and a child's own size overrules (sizing.rs, button_text_size; button/button.rs, Button::render); no colour, so the label takes the Button's state colours",
    ),
    (
        "toggle",
        "button.label(NONE)'s family, size, weight and style, and typography.line_height_multiplier as the line height, on the Toggle: upstream sets text_sm on a Size::Medium Toggle before the caller's refinement, so this size wins (button/toggle.rs, Toggle::render); the text alone -- a Toggle is Ghost by default, a flat button, and the model states no flat-button frame, so upstream's own stands",
    ),
    (
        "button_disabled",
        "button.body(DISABLED).fill and button.label(DISABLED).color on a disabled Button, faded by button.disabled_opacity, over upstream's own disabled literals -- input_background() or the variant's token faded, and muted_foreground at 0.5 -- which its disabled style replays the refinement over (button/button.rs, ButtonVariant::disabled and RenderOnce for Button)",
    ),
    (
        "button_checked",
        "button.body(CHECKED).fill and button.label(CHECKED).color on a selected Button, the pressed pair (button.body(PRESSED).fill, button.label(PRESSED).color) where no rule states them for the checked state, over upstream's selected style -- its button_active token and button_foreground -- which the Button replays the refinement over (button/button.rs, ButtonVariant::selected and RenderOnce for Button); where neither fill is stated the refinement sets none, and upstream's selected fill stands",
    ),
    ("icon_size_toolbar", "toolbar.button_icon(NONE).size"),
    ("icon_size_small", "icons.sizes.small"),
    (
        "input",
        "input.field(NONE).min_size[1] by the control-height rule (typography.line_height_multiplier as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1; upstream's height where the theme states none), the padding sides input.field(NONE) states -- which reach the field, because upstream pads it before the refinement (input/input.rs, Input::render: input_px then refine_style) --, its corner radius and stroke width, and input.text(NONE)'s font including its colour -- the root sets none of its own and the editor paints the field's text in the root's, the window's foreground without it (input/input.rs, Input::render; gpui-base/input/base/element.rs, prepaint)",
    ),
    (
        "input_height",
        "input.field(NONE).min_size[1] by the control-height rule geometry::input applies (typography.line_height_multiplier as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), and nothing else of geometry::input",
    ),
    (
        "input_fill",
        "input.field(NONE).fill, or for a disabled field input.field(DISABLED).fill (the fill at rest where no rule states one for the state) and input.text(DISABLED).color, which the editor halves the alpha of as it does any disabled field's text (gpui-base/input/base/element.rs, prepaint), faded by input.disabled_opacity, over upstream's fill -- Theme::input_background, the window's background in a light theme, and input mixed with transparent when disabled (input/input.rs, input_style and Input::render: bg then refine_style) -- which no ThemeColor field can make the platform's",
    ),
    (
        "input_group_button",
        "button.body(NONE).corners alone. gpui-component scales a control's radius with its size, so an in-group button would take radius/2 (input/group.rs, InputGroupButton::render_in_group), where the model records one radius per widget",
    ),
    (
        "text_area",
        "the multi-line field's own padding, the sides text_area.field(NONE) states (docs/platform-facts.md §2.29): the root padded by the rest of each stated side past the Size::Medium editor padding upstream gives a Textarea in render (input/input.rs, Input::render), none below it, an unstated side upstream's; text_area.field(NONE)'s corner radius and stroke width, and input.text(NONE)'s font including its colour at typography.line_height_multiplier; no height: the rows its state holds size it",
    ),
    (
        "checkbox",
        "checkbox.label_gap, checkbox.label(NONE)'s family, size, weight and style but not its colour, and typography.line_height_multiplier as the line height on the row, over upstream's relative(1.); the label column sets its own relative(1.25) (gpui-component/checkbox.rs, RenderOnce for Checkbox)",
    ),
    (
        "radio",
        "checkbox.label_gap, and checkbox.label(NONE)'s font including its colour, which the label takes because upstream sets foreground on the row and mutes the label child instead (platform-facts §2.5: radio metrics are the checkbox's), and typography.line_height_multiplier as the line height on the row, over upstream's relative(1.); the label column sets its own relative(1.25) (gpui-component/radio.rs, RenderOnce for Radio)",
    ),
    (
        "select",
        "combo_box.field(NONE).min_size[1] by the control-height rule (typography.line_height_multiplier as the line height; the stated height itself at a text scale of 1 or less, and a minimum with an automatic height above 1; upstream's height where the theme states none), combo_box.field(NONE).min_size[0] as the least width, the padding sides combo_box.field(NONE) states -- but not the right side where combo_box.arrow_area_width is stated: the platform measures it to an arrow column the trigger does not have, its caret sitting inside the padded row, so upstream's own right padding stands --, its corner radius and stroke width, and combo_box.label(NONE)'s font -- and, unlike geometry::combobox, its colour as well (native-theme-gpui/geometry.rs, select)",
    ),
    (
        "combo_box_fill",
        "combo_box.field(NONE).fill, or for a disabled trigger combo_box.field(DISABLED).fill (the fill at rest where no rule states one for the state) and combo_box.label(DISABLED).color, faded by combo_box.disabled_opacity in place of upstream's own half fade, over upstream's fill -- Theme::input_background, and input mixed with transparent when disabled (input/input.rs, input_style; gpui-component/select.rs, Select::render; gpui-component/combobox.rs, Combobox::render) -- which no ThemeColor field can make the platform's. The disabled text reaches a Combobox's selected text and not a Select's title, which mutes itself (select.rs, SelectState::display_title)",
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

/// [`native_info`] on the 0.8.0 model: `w` with the refinement a builder of
/// `native_theme_gpui::next::geometry` gives for the installed 0.8.0 theme,
/// and `info` given `build`'s geometry line. Where the theme states no value
/// gpui can take at a home the builder reads (unstated, or in a form gpui
/// cannot draw), the widget keeps upstream's geometry and `info` names that
/// home (spec §10, rule 0); before `apply` ran it names no builder.
pub fn next_native_info<W: Styled>(
    w: W,
    cx: &App,
    build: fn(&native_theme::next::ResolvedTheme) -> Mapped<StyleRefinement>,
    name: &'static str,
    info: &mut WidgetInfo,
) -> W {
    refined(w, next_native_value(cx, build, name, info).as_ref())
}

/// What [`next_native_info`] applies, for a builder of any value (a size, a
/// refinement): the value `build` gives for the installed 0.8.0 theme, and
/// `info` given `build`'s geometry line; where the theme leaves a value the
/// builder needs without a value gpui can take, `None`, so the widget keeps
/// upstream's, and `info` says at which home (spec §10, rule 0); `None` before
/// `apply` ran, naming no builder.
pub fn next_native_value<T>(
    cx: &App,
    build: fn(&native_theme::next::ResolvedTheme) -> Mapped<T>,
    name: &'static str,
    info: &mut WidgetInfo,
) -> Option<T> {
    match next_value(cx, build) {
        Some(Ok(value)) => {
            *info = std::mem::take(info).next_geometry(name);
            Some(value)
        }
        Some(Err(unstated)) => {
            *info = std::mem::take(info).next_unmapped(name, unstated);
            None
        }
        None => None,
    }
}

/// What a Widget Info says of the icons gpui-component builds and draws
/// inside a widget on its own, `icons` naming them and where upstream draws
/// them. gpui-component draws each by its asset path, which it hands to the
/// application's icon resolver first (gpui-component icon.rs,
/// `set_icon_resolver`, `resolve_icon_path`); the showcase installs the
/// connector's widget icons there (`ShowcaseWidgetIcons`), which serve the
/// chosen icon theme where they can and say what they serve on the Icons
/// page. Without `svg-rasterize` there are no widget icons, and each is
/// gpui-component's own.
#[cfg(feature = "svg-rasterize")]
pub fn own_icons(icons: &str) -> String {
    let essential = native_theme_gpui::icons::ESSENTIAL_ICONS.len();
    format!(
        "{icons}: drawn by gpui-component inside the widget, each by its asset path, which it first hands to the application's icon resolver (gpui-component/icon.rs, resolve_icon_path). The showcase installs the connector's widget icons there (native-theme-gpui/icons/widget.rs, WidgetIcons): under an icon set of SVGs that has the {essential} glyphs nearly every window needs, these are the chosen set's icons, looked up by name (a freedesktop theme's at 16 px) and drawn in one colour, and one the set lacks is not drawn; under gpui-component's built-in Lucide, under a set that lacks one of the {essential}, and under SF Symbols or Segoe Fluent, they are gpui-component's own, and so is the filled star under Lucide. The Icons page reports which holds"
    )
}

/// [`own_icons`] in a build without `svg-rasterize`.
#[cfg(not(feature = "svg-rasterize"))]
pub fn own_icons(icons: &str) -> String {
    format!(
        "{icons}: drawn by gpui-component inside the widget, each by its asset path, which it first hands to the application's icon resolver (gpui-component/icon.rs, resolve_icon_path). This build has no widget icons (feature svg-rasterize), so the showcase sets no resolver and each path draws gpui-component's own icon, whichever icon theme is chosen"
    )
}

/// A colour of the native theme as gpui paints it: the value of a claim on a
/// colour the showcase hands a widget itself, from the resolved theme, where
/// no `ThemeColor` token carries it.
pub fn stated(color: native_theme::color::Rgba) -> Hsla {
    let [r, g, b, a] = color.to_f32_array();
    Hsla::from(gpui::Rgba { r, g, b, a })
}

/// A colour of the installed 0.8.0 theme as gpui paints it, each channel
/// clamped to [0, 1] as the connector converts one (native-theme-gpui
/// colors.rs, `channels_to_hsla`): the value of a claim on a global colour
/// (`colors.focus_ring`) that a theme-drawn widget paints.
#[cfg(feature = "widgets")]
pub fn next_stated(color: native_theme::next::Color) -> Hsla {
    Hsla::from(gpui::Rgba {
        r: color.r.clamp(0., 1.),
        g: color.g.clamp(0., 1.),
        b: color.b.clamp(0., 1.),
        a: color.a.clamp(0., 1.),
    })
}

/// A size in logical pixels to two decimals, trailing zeros dropped: a rem
/// multiple of a 14.666667px font_size (adwaita's 11 pt at 96 DPI) prints as
/// what it is to the eye, not as float noise.
pub fn px_text(v: f32) -> String {
    let s = format!("{v:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// `font` as `demo::text_in`, the connector's `with_text` (geometry.rs) and
/// its widgets' labels (widgets/mod.rs, `font_text`) draw text in it, for a
/// font line of the showcase's own Widget Info: the family as `status::family_label` names what gpui is asked for
/// (`native_theme_gpui::font_family`), the stated size the text-scaling
/// factor scales, the weight, the style where it is not normal, and the line
/// box.
pub fn drawn_font(font: &native_theme::theme::ResolvedFontSpec) -> String {
    let drawn = native_theme_gpui::font_family(&font.family);
    let style = match font.style {
        native_theme::theme::FontStyle::Normal => "",
        native_theme::theme::FontStyle::Italic => ", italic",
        native_theme::theme::FontStyle::Oblique => ", oblique",
    };
    format!(
        "{}, {}px scaled by the text-scaling factor, weight {}{style}, at defaults.line_height",
        native_theme_showcase_support::status::family_label(&font.family, Some(&drawn)),
        px_text(font.size),
        font.weight
    )
}

/// A rem multiple of the showcase's own as the window lays it out: its size
/// in logical pixels at gpui-component's rem, which is `Theme::font_size`
/// (gpui-component 0.10.0 `root.rs:725`), then the multiple itself.
pub fn rems_text(r: Rems, t: &Theme) -> String {
    format!(
        "{}px ({} rem of the {}px font_size)",
        px_text(r.0 * t.font_size.as_f32()),
        px_text(r.0),
        px_text(t.font_size.as_f32())
    )
}

/// An opacity as a percentage, the way `px_text` prints a size: the prose
/// derived from the value, so it cannot say another.
pub fn percent_text(opacity: f32) -> String {
    format!("{}%", px_text(opacity * 100.))
}

/// `c` as native-theme's `Rgba`, whose `Display` is the colour text the
/// showcase prints, as a theme colour prints: `#rrggbb`, or `#rrggbbaa` where
/// the alpha's byte is not 255 -- a colour painted at 80% printed without
/// its alpha would claim a fill the widget never paints.
pub fn as_rgba(c: Hsla) -> native_theme::color::Rgba {
    let c = gpui::Rgba::from(c);
    native_theme::color::Rgba::from_f32(c.r, c.g, c.b, c.a)
}
