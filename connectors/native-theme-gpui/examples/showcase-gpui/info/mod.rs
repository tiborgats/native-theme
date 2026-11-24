//! What the inspector shows for one widget instance (spec §3).

use gpui::{App, Hsla, Rems, StyleRefinement, Styled};
use gpui_component::theme::Theme;
use native_theme_gpui::next::{Mapped, Unmapped};
/// The reads a claim in the 0.8.0 form makes of its home's property: the
/// connector's own (`native_theme_gpui::next`), each `None` where the theme
/// states no value gpui can take there, unstated or in a form gpui cannot
/// draw, so the panel shows the toolkit's own there (spec §10, rule 0).
pub use native_theme_gpui::next::{
    radius_of, state_stroke_color_of, stroke_color_of, stroke_width_of,
};

use crate::support::{next_value, refined};

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
    /// The "geometry" line for the `next::geometry` builder named `builder`,
    /// from [`NEXT_GEOMETRY_NOTES`]; the only way such a line is written.
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
    /// The "variant" line for the `next::variants` builder named `builder`
    /// where the installed 0.8.0 theme states the value at `unstated`'s home
    /// in a form gpui cannot take, which the builder needs: the widget takes
    /// `variants::ghost_button`, the variant the builder starts from.
    pub fn next_variant_unmapped(self, builder: &'static str, unstated: Unmapped) -> Self {
        self.config(
            "variant",
            format!(
                "native_theme_gpui::variants::ghost_button: the theme states no value gpui can take at {}, which next::variants::{builder} needs",
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

/// What each builder of `native_theme_gpui::next::geometry` a widget records
/// carries, the value builders included, each naming the homes its builder
/// reads: `WidgetInfo::next_geometry` prints `geometry::<name>: <what>`.
/// `every_geometry_builder_has_a_note` holds the table to the builders.
pub const NEXT_GEOMETRY_NOTES: &[(&str, &str)] = &[
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
    (
        "scrollbar_gutter",
        "the vertical track's width, scrollbar.track(NONE; orientation=Vertical).min_size[0], as right padding where scrollbar.overlay_mode is false, and nothing where it is true, because gpui-component overlays the bar on the scroll area instead of putting it beside the content",
    ),
    (
        "group_box_content",
        "the padding sides card.container(NONE) states, its corner radius and its stroke where it states them, and card.container(NONE).fill -- refined last, so the radius, the edge and the fill drawn are the card's, not upstream's radius or the Fill variant's group_box token (group_box.rs, GroupBox); where the theme states no corners or no stroke, the refinement sets none, and the GroupBox keeps upstream's",
    ),
    (
        "accordion_title",
        "expander.header(NONE).min_size[1] as the title row's height, and expander.header_label(NONE).color over the foreground upstream sets on an open item's title row first; a closed one's sets none (gpui-component/accordion.rs, AccordionItem::render)",
    ),
    (
        "accordion",
        "expander.container(NONE).stroke.color on the Accordion's card and on each item's line under it, over the border token upstream draws both in first (gpui-component/accordion.rs, RenderOnce for Accordion and AccordionItem::render), and a width of 0 where the stroke is stated as none. A stated visible width is not set: the one refinement lands on the Accordion, edged on every side, and on each AccordionItem, edged only at the bottom and not the last (gpui-component/accordion.rs:108-114, :380-384), so a width on every side would frame each item, and upstream's border_1 and border_b_1 stand (native-theme-gpui next/builders.rs, accordion; the crate README, Limits)",
    ),
    (
        "accordion_hover",
        "expander.header(HOVER).fill as an item's title row fill under the pointer, through AccordionItem::hover, where a rule states it for HOVER; nothing where none does, so the row stays at rest (gpui-component/accordion.rs, AccordionItem::hover)",
    ),
    (
        "title_bar",
        "window.title(normal)'s family, size, weight and style, and its colour, and typography.line_height_multiplier as the line height -- upstream sets no text colour on the bar, so the colour displaces nothing (title_bar.rs, RenderOnce for TitleBar). The window controls set foreground on their own elements, out of its reach (title_bar.rs, RenderOnce for ControlIcon)",
    ),
    (
        "title_bar_inactive",
        "window.title_bar(normal).fill of the theme resolved for an inactive window, drawn as upstream's gradient is from title_bar and the background, window.frame(normal).fill here, and its window.title(normal).color over next::geometry::title_bar's text, for a TitleBar while Window::is_window_active is false: TitleBar reads no window activation, painting every bar alike (title_bar.rs, default_title_bar_background and RenderOnce for TitleBar)",
    ),
    (
        "editor",
        "typography.line_height_multiplier as the line height, over the relative(1.5) upstream's Editor sets on its Input first: the Editor refines the caller's style last (input/editor.rs, Editor::render). Its font is the mono font the theme sets, mono_font_family at mono_font_size, typography.monospace's family and size (native-theme-gpui next/mod.rs, theme_scalars)",
    ),
    (
        "link",
        "link.label(NONE)'s family, size, weight and style, typography.line_height_multiplier as the line height, link.container(NONE).fill as the fill, which upstream paints none of (gpui-component/link.rs, RenderOnce for Link), and no underline where link.label(NONE).decoration is none: upstream underlines a link at rest before the refinement (gpui-component/link.rs, RenderOnce for Link: text_decoration_1 then refine_style), and a zero thickness paints none",
    ),
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
    (
        "default_button_rest",
        "button.body(DEFAULT).fill as the fill and button.body(DEFAULT).stroke.color (button.body(NONE).stroke.color where no rule states it for DEFAULT) as the border colour of a dialog's default Button at rest, over next::variants::default_button, whose rest fill is a fifth of its border colour (button/button.rs, ButtonVariant::bg_color); nothing where no rule states the default body's fill, and the Button is .primary()",
    ),
    ("icon_size_toolbar", "toolbar.button_icon(NONE).size"),
    (
        "toolbar_button_rest",
        "where a rule states toolbar.button(HOVER).stroke.color, a transparent fill and a transparent border button.body(NONE).stroke.width wide, and each padding side button.body(NONE) states less that width: a custom variant draws its colour as the border in every state and lays out no border width (button/button.rs, ButtonCustomVariant and RenderOnce for Button), so this hides the edge at rest and keeps the button's size; nothing where no rule states the outline",
    ),
    (
        "toolbar_button_checked",
        "toolbar.button(CHECKED).fill and button.label(CHECKED).color (button.label(PRESSED).color where no rule states that for the checked state) on a selected toolbar Button built on next::variants::toolbar_button; where no rule states the checked fill, toolbar.button(PRESSED).fill, else button.body(PRESSED).fill, with no text colour; nothing where it states none of the three: a custom variant fills a selected Button with its active colour and letters every state in its one foreground (button/button.rs, ButtonCustomVariant and RenderOnce for Button), and the Button replays the refinement over its selected style. Its border colour is toolbar.button(CHECKED).stroke.color where a rule states it, at toolbar_button_rest's width, and stays under the pointer and pressed, where a selected Button takes no hover or press style",
    ),
    ("icon_size_small", "icons.sizes.small"),
    ("icon_size_dialog", "icons.sizes.dialog"),
    (
        "menu_item",
        "menu.item(NONE).max_size[1], where the theme states one, by the control-height rule (typography.line_height_multiplier as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), else menu.item(NONE).min_size[1] as a least height the content grows past, the padding sides menu.item(NONE) states, menu.icon_text_gap, and menu.item_label(NONE)'s family, size, weight and style but not its colour",
    ),
    (
        "status_bar",
        "the padding sides status_bar.bar states, and status_bar.label's family, size, weight and style, and its colour, which upstream would otherwise paint with muted_foreground (status_bar.rs, StatusBar::render: text_color then refine_style), and typography.line_height_multiplier as the line height",
    ),
    (
        "toolbar",
        "toolbar.bar(NONE).min_size[1] as the minimum height where the theme states one, toolbar.item_gap where it states one, the padding sides toolbar.bar(NONE) states, its fill, the bottom edge in its stroke, toolbar.button_label(NONE)'s family, size, weight and style, and its colour -- upstream's Toolbar sets no colour (gpui-component/toolbar.rs, RenderOnce for Toolbar) --, and typography.line_height_multiplier as the line height",
    ),
    (
        "popover",
        "the padding sides popover.panel states, its corner radius, and its edge, popover.panel.stroke.color at popover.panel.stroke.width, inside the shadow ring popover_style draws; an unstated side keeps upstream's p_3 (gpui-component/popover.rs, Popover::render)",
    ),
    (
        "dialog",
        "the padding sides dialog.panel states (an unstated side keeps upstream's 16px; upstream also spaces the sections by max(top, 8px) and a DialogContent by the bottom padding), dialog.panel.min_size[0] as the least width, which the width upstream sets after the refinement keeps, min_size[1], max_size[1], its corner radius, and dialog.panel.fill and dialog.panel.stroke.color at dialog.panel.stroke.width over the window's background and border upstream paints the surface with first; the max_size[1] does not reach upstream's Dialog, which sets its own max_h after the refinement (dialog/dialog.rs, RenderOnce for Dialog)",
    ),
    (
        "dialog_entrance",
        "dialog.entrance as the nearest entrance gpui-component draws, through Dialog::entrance and AlertDialog::entrance: DialogEntrance::Fade, a fade in place, for scale_fade, fade_rise and fade, and DialogEntrance::None for none. scale_fade loses its scale, since gpui-component has no scaling entrance, and fade_rise its rise, since FadeSlide settles from just above the dialog's place instead (dialog/entrance.rs, DialogEntrance; dialog/dialog.rs, Dialog::entrance)",
    ),
    (
        "dialog_footer",
        "dialog.button_gap between the buttons (dialog/footer.rs, DialogFooter::render)",
    ),
    (
        "dialog_title",
        "dialog.title's family, size, weight and style, and its colour, which displaces nothing, upstream setting none on the title, and typography.line_height_multiplier as the line height, which replaces upstream's relative(1.25) (dialog/title.rs, RenderOnce for DialogTitle: line_height then refine_style)",
    ),
    (
        "dialog_description",
        "dialog.body's family, size, weight and style, and its colour, which upstream would otherwise paint with muted_foreground (dialog/description.rs, DialogDescription::render), and typography.line_height_multiplier as the line height",
    ),
    (
        "dialog_max_width",
        "dialog.panel.max_size[0], which caps the width: a Dialog's through Dialog::max_w (dialog/dialog.rs, Dialog::max_w), and an AlertDialog's, which has no max_w of its own, through Styled::max_w, landing on the surface with the rest of the refinement (dialog/alert_dialog.rs, AlertDialog; dialog/dialog.rs, Dialog::render); nothing where the theme states none",
    ),
    (
        "progress",
        "progress_bar.track(NONE).min_size[1] as the height, its corners' radius, and min_size[0] as the least width",
    ),
    ("spinner_size", "spinner.diameter"),
    ("icon_size_large", "icons.sizes.large"),
    ("icon_size_panel", "icons.sizes.panel"),
    (
        "tooltip",
        "the padding sides tooltip.panel states, its corner radius, tooltip.text's font — including its colour, which upstream would otherwise paint with popover_foreground —, typography.line_height_multiplier as the line height (upstream sets none, so wrapped text was spaced by gpui's phi), tooltip.panel.fill and tooltip.panel.stroke.color, which upstream would otherwise paint with the popover's fill and the window's border, the edge tooltip.panel.stroke.width wide in place of upstream's border_1, and shadow_md's two layers in colors.shadow where tooltip.panel states no shadows, no shadow where it states none, in place of upstream's shadow_md (gpui-component/tooltip.rs, Tooltip::render: bg, text_color, border_1, border_color and shadow_md, then refine_style)",
    ),
    (
        "tooltip_content",
        "tooltip.panel.max_size[0] less the bubble's left and right padding -- the theme's where stated, upstream's px_2 (0.5 rem at typography.body.size_px) where not (gpui-component/tooltip.rs, Tooltip::render) -- and its edge, tooltip.panel.stroke.width on each side, which geometry::tooltip draws in place of upstream's border_1 (gpui-component/tooltip.rs, Tooltip::render), as the max width of the element passed to Tooltip::element, which the text wraps at",
    ),
    (
        "list",
        "list.container(NONE)'s stroke width, stroke colour and corner radius, a clip to that radius, and list.container(NONE).fill as the fill: neither List nor Tree paints one, nor a ListItem at rest (list/list_item.rs, ListItem::render)",
    ),
    (
        "list_item",
        "list.row(NONE).min_size[1], where the theme states one, by the control-height rule (typography.line_height_multiplier as the line height; the stated height at a text scale of 1 or less, a minimum with an automatic height above 1), the padding sides list.row(NONE) states, each with the margin list.highlight(NONE) states on that side, and list.item_label(NONE)'s font including its colour -- upstream labels the row with foreground before applying it (list/list_item.rs, ListItem::render)",
    ),
    (
        "list_item_selected",
        "list.item_label(SELECTED).color on a selected ListItem's text, over the foreground upstream sets on every row before the refinement -- or geometry::list_item's list.item_label(NONE) colour: while list.active_highlight is on, gpui-component's default, upstream marks a selected row by its fill alone, painted after the refinement; with it off, it letters the row accent_foreground on accent after the refinement, so this colour does not reach it; and it mutes a row that is not selectable after the refinement too (list/list_item.rs, ListItem::render)",
    ),
    (
        "table",
        "list.item_label(NONE)'s family, size, weight and style, and typography.line_height_multiplier as the line height, on the table root (table/table.rs, Table::render: text_sm then refine_style)",
    ),
    (
        "data_table_size",
        "list.row(NONE).min_size[1] as Size::Size, which table_row_height returns verbatim, or upstream's Medium 32px where the theme states none (sizing.rs, table_row_height); a Size::Size cell sets no text size and inherits the container's, which geometry::table gives list.item_label(NONE), where a Medium cell is text_sm (sizing.rs, table_cell_size); above a text scale of 1 at least list.item_label(NONE)'s scaled line box at typography.line_height_multiplier, upstream's own cell padding (sizing.rs, table_cell_padding) and the row's 1px bottom border, which upstream draws inside the row (table/state.rs, border_b_1), so the row holds its text",
    ),
    (
        "list_header_height",
        "list.header(NONE).min_size[1], the height of a table's header row, which DataTable::header_row_height takes (table/data_table.rs, DataTable::header_row_height); where the theme states none, nothing is set and the header keeps the body rows' height (table/state.rs, header_row_height); above a text scale of 1 at least list.item_label(NONE)'s scaled line box at typography.line_height_multiplier, upstream's own cell padding and the header row's 1px bottom border (table/state.rs, border_b_1): a Size::Size header cell sets no text size and inherits the container's, which geometry::table gives list.item_label(NONE) (sizing.rs, table_cell_size)",
    ),
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
        "combobox",
        "combo_box.field(NONE).min_size[1] by the control-height rule (typography.line_height_multiplier as the line height; the stated height itself at a text scale of 1 or less, and a minimum with an automatic height above 1; upstream's height where the theme states none), combo_box.field(NONE).min_size[0] as the least width, the padding sides combo_box.field(NONE) states -- but not the right side where combo_box.arrow_area_width is stated: the platform measures it to an arrow column the trigger does not have, its caret sitting inside the padded row, so upstream's own right padding stands --, its corner radius and stroke width, and combo_box.label(NONE)'s family, size, weight and style but not its colour (native-theme-gpui/geometry.rs, combobox)",
    ),
    (
        "combo_box_fill",
        "combo_box.field(NONE).fill, or for a disabled trigger combo_box.field(DISABLED).fill (the fill at rest where no rule states one for the state) and combo_box.label(DISABLED).color, faded by combo_box.disabled_opacity in place of upstream's own half fade, over upstream's fill -- Theme::input_background, and input mixed with transparent when disabled (input/input.rs, input_style; gpui-component/select.rs, Select::render; gpui-component/combobox.rs, Combobox::render) -- which no ThemeColor field can make the platform's. The disabled text reaches a Combobox's selected text and not a Select's title, which mutes itself (select.rs, SelectState::display_title)",
    ),
];

/// `w` with the refinement a builder of
/// `native_theme_gpui::next::geometry` gives for the installed 0.8.0 theme,
/// and `info` given `build`'s geometry line; `name` is `build`'s name, which
/// `native_info_names_the_builder_it_applies` holds to it. Where the theme states no value
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
/// (`colors.focus_ring`) that a theme-drawn widget paints, or that the
/// showcase hands a widget itself (`separator.color`).
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

/// An icon's colour of the installed 0.8.0 theme as gpui paints it
/// ([`next_stated`]), `None` where the theme states none and the widget
/// keeps the toolkit's own (spec §10, rule 0).
pub fn icon_color_of(icon: &native_theme::next::IconStyle) -> Option<Hsla> {
    icon.stated
        .contains(native_theme::next::IconStated::COLOR)
        .then(|| next_stated(icon.color))
}

/// The line box of a font line where a theme-drawn widget's label draws the
/// font (native-theme-gpui/widgets/mod.rs, `LabelFont::next`) or a 0.8.0
/// builder sets it; text the showcase letters itself takes
/// [`typography::line_box_text`].
pub const LINE_AT_THE_MULTIPLIER: &str = "at typography.line_height_multiplier";

/// A font of the installed 0.8.0 theme for its info's font line: the family
/// as `status::family_label` names what gpui is asked for, the size the
/// theme's text scale has already scaled, the weight, the slant where it is
/// not upright, then `line`, how its line box is set
/// ([`LINE_AT_THE_MULTIPLIER`] or [`typography::line_box_text`]).
pub fn next_drawn_font(text: &native_theme::next::TextStyle, line: &str) -> String {
    let drawn = native_theme_gpui::font_family(&text.family);
    let style = match text.style {
        native_theme::next::FontStyle::Italic => ", italic",
        native_theme::next::FontStyle::Oblique => ", oblique",
        _ => "",
    };
    format!(
        "{}, {}px, weight {}{style}, {line}",
        native_theme_showcase_support::status::family_label(&text.family, Some(&drawn)),
        px_text(text.size_px),
        text.weight
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
