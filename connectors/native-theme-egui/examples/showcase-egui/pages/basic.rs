//! Basic: the controls the three showcases all draw, in the same order, with the same labels,
//! values and states, packed onto one screen in one to five columns, so the gpui, iced and egui
//! captures compare control by control. Each control goes through the seam its palette page
//! gives it; what egui leaves to the call site is applied per call from the theme. Every
//! element of `docs/showcase-elements.toml` the page draws is placed where it is drawn, its
//! parts too, for `--dump-layout` and Widget Info.

use egui::Button;
use egui::class::Classes;
use egui::widget_style::{ButtonStyle, CheckboxStyle, StyleArgs, TextEditStyle, WidgetState};
use native_theme::icons::IconRole;
use native_theme::icons::IconSet;
use native_theme::next::WidgetState as NextState;
use native_theme::next::{ListKind, ListQuery, ListVariant, PartStated, ResolvedTheme};
use native_theme_egui::convert::{to_corner_radius, to_margin};
use native_theme_egui::{Role, RoleVariant, Surface, TextRole, next};
use native_theme_egui_widgets::combo_box::ComboBox;
use native_theme_egui_widgets::expander::Expander;
use native_theme_egui_widgets::fade::fade_as_one;
use native_theme_egui_widgets::links;
use native_theme_egui_widgets::parts::Parts;
use native_theme_egui_widgets::progress_bar::ProgressBar;
use native_theme_egui_widgets::radio_button::RadioButton;
use native_theme_egui_widgets::segmented_control::SegmentedControl;
use native_theme_egui_widgets::slider::Slider;
use native_theme_egui_widgets::spinner::Spinner;
use native_theme_egui_widgets::switch::Switch;

use super::data::TableHeader;
use super::{DemoState, Framed, button_min_size, caption, field_frame, input_min_size};
use crate::app::OldAtlas;
use crate::demo::{self, Registry};

/// The width of the Basic page's text fields, drop-down, number input, slider and progress
/// bar. The model states no such width; it is the Basic page's own, the gpui showcase's and
/// the iced showcase's `BASIC_WIDTH` too, so the three pages lay the same controls out alike.
pub(crate) const BASIC_WIDTH: f32 = 140.0;

/// The width of the Basic page's text area, list, expander, card, separator and table: the
/// Basic page's own, as `BASIC_WIDTH`, and the gpui and iced showcases' `BASIC_WIDE`.
pub(crate) const BASIC_WIDE: f32 = 170.0;

/// The fewest and the most columns the page lays its groups out in.
const MIN_COLUMNS: usize = 1;
const MAX_COLUMNS: usize = 5;

/// The Segmented control's group, by its index in reading order.
const SEGMENTED_GROUP: usize = 10;

/// The drop-down's rows.
const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

/// The progress bar's value: the datum on display, not a style value.
const PROGRESS: f32 = 0.4;

/// The tab bar's tabs, and the segmented control's segments.
const TABS: [&str; 2] = ["One", "Two"];
const SEGMENTS: [&str; 3] = ["Day", "Week", "Month"];

/// The list's rows, and how many of them it shows: data of the page.
const LIST_ROWS: usize = 8;
pub(crate) const LIST_VISIBLE: usize = 3;

/// The text area's text, three lines.
pub(crate) const AREA_TEXT: &str = "Line one\nLine two\nLine three";
const AREA_ROWS: usize = 3;

/// The number input's value and step: data of the page.
pub(crate) const NUMBER: f64 = 42.0;
const NUMBER_STEP: f64 = 1.0;

/// The table's header and rows, and the row shown selected (the second): data of the page.
const TABLE_HEADER: [&str; 2] = ["Name", "Size"];
pub(crate) const TABLE_ROWS: [[&str; 2]; 2] = [["a.txt", "1 KB"], ["b.png", "20 KB"]];
pub(crate) const TABLE_SELECTED: usize = 1;

/// The page's groups in reading order (Basic page v6): the five columns' groups, left to right,
/// each column top to bottom, which the tests check the page against.
#[cfg(test)]
pub(crate) const GROUPS: [&str; 20] = [
    "Buttons",
    "Checkboxes",
    "Radio buttons",
    "Drop-down",
    "Text inputs",
    "Text area",
    "Slider",
    "Switches",
    "Number input",
    "Spinner",
    "Segmented control",
    "Card",
    "Typography",
    "Separator",
    "Progress bar",
    "List",
    "Icons",
    "Tabs",
    "Expander",
    "Table",
];

/// Each column's groups, top to bottom, as indices of the groups in reading order, for the
/// page's five, four, three, two and one columns: the gpui and iced pages' arrangements
/// (docs/showcase-elements.toml, the Basic page).
pub(crate) const FIVE: [&[usize]; 5] = [
    &[0, 1, 2, 3],
    &[4, 5, 6],
    &[7, 8, 9, 10, 11],
    &[12, 13, 14, 15],
    &[16, 17, 18, 19],
];
pub(crate) const FOUR: [&[usize]; 4] = [
    &[0, 1, 2, 3, 18],
    &[4, 5, 6, 16],
    &[7, 8, 9, 10, 11, 17],
    &[12, 13, 14, 15, 19],
];
pub(crate) const THREE: [&[usize]; 3] = [
    &[0, 1, 2, 3, 15, 18],
    &[4, 5, 6, 13, 14, 16, 19],
    &[7, 8, 9, 10, 11, 12, 17],
];
pub(crate) const TWO: [&[usize]; 2] = [
    &[0, 1, 2, 3, 7, 12, 15, 17, 18],
    &[4, 5, 6, 8, 9, 10, 11, 13, 14, 16, 19],
];
pub(crate) const ONE: [&[usize]; 1] = [&[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
]];

/// The groups of each column of a page of `columns` columns.
pub(crate) fn arrangement(columns: usize) -> &'static [&'static [usize]] {
    match columns {
        5 => &FIVE,
        4 => &FOUR,
        3 => &THREE,
        2 => &TWO,
        _ => &ONE,
    }
}

/// Whether a page of `columns` columns lays the Segmented control out in its last column, which
/// the page's right margin follows.
fn segmented_in_last_column(columns: usize) -> bool {
    arrangement(columns)
        .last()
        .is_some_and(|column| column.contains(&SEGMENTED_GROUP))
}

/// What the page's column rule ([`column_count`]) reads of the installed 0.8.0 theme, each
/// length egui's own where the theme states none (spec §10, rule 0): the space egui itself
/// puts at that place, and so too without an installed 0.8.0 theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ColumnRule {
    /// `layout.window_margin`, the content panel's inner margin on each side; where unstated,
    /// egui's own `Frame::central_panel` margin, which the panel then keeps (egui-0.37.0
    /// `src/containers/frame.rs:191-193`, `inner_margin(8)`, the same on every side).
    pub(crate) margin: f32,
    /// `layout.section_gap`, between the columns; where unstated, the gap egui lays columns
    /// out apart, `item_spacing.x` (egui-0.37.0 `src/ui.rs:2665`, `Ui::columns`).
    pub(crate) gap: f32,
    /// `layout.widget_gap`, the gap between neighbouring widgets of a row; where unstated,
    /// egui's own `item_spacing.x`, which the connector then leaves (egui-0.37.0
    /// `src/style.rs:1565`).
    pub(crate) widget_gap: f32,
    /// `scrollbar.overlay_mode`; egui's own `ScrollStyle::floating` without a 0.8.0 theme.
    pub(crate) overlay: bool,
    /// The scroll bar's groove width, `next::scrollbar_groove_width`, egui's own
    /// `ScrollStyle::bar_width` where the theme states none.
    pub(crate) groove: f32,
    /// The widest fixed-width control the theme states on the page: `combo_box.field(NONE)
    /// .min_size[0]`, the two tabs at `tab.tab(NONE).min_size[0]` `tab.item_gap` apart (where
    /// unstated, the page style's `item_spacing.x`, which the tab scope keeps) with room
    /// for both of the selected tab's flares, `2 * tab.active_outward_corner_radius` where the
    /// tab bar draws them (`demo::next_outward_radius`), `BASIC_WIDE` and `BASIC_WIDTH`.
    pub(crate) widest: f32,
}

impl ColumnRule {
    /// The rule of the installed 0.8.0 theme `r`, where `own` is the page's egui style, whose
    /// item spacing, central panel margin and scroll style an unstated length keeps.
    pub(crate) fn of(r: Option<&native_theme::next::ResolvedTheme>, own: &egui::Style) -> Self {
        let metric = |read: fn(&native_theme::next::LayoutMetrics) -> Option<f32>, own: f32| {
            r.and_then(|r| read(&r.layout)).unwrap_or(own)
        };
        let panel = egui::Frame::central_panel(own).inner_margin;
        let rows = own.spacing.item_spacing;
        let scroll = &own.spacing.scroll;
        let tab_min = r
            .and_then(|r| r.tab.tab(NextState::NONE).min_size[0])
            .unwrap_or_default();
        // As the tab bar lays its tabs out (`demo::TabLook::next`): the tab scope's
        // `item_spacing.x`, `tab.item_gap` where stated, else the page style's, which the scope
        // keeps.
        let item_gap = r.and_then(|r| r.tab.item_gap).unwrap_or(rows.x);
        let flare = r.and_then(demo::next_outward_radius).unwrap_or_default();
        let tabs = tab_min + tab_min + item_gap + flare + flare;
        Self {
            margin: metric(|l| l.window_margin, panel.leftf()),
            gap: metric(|l| l.section_gap, rows.x),
            widget_gap: metric(|l| l.widget_gap, rows.x),
            overlay: r.map_or(scroll.floating, |r| r.scrollbar.overlay_mode),
            groove: r
                .and_then(|r| next::scrollbar_groove_width(r).ok())
                .unwrap_or(scroll.bar_width),
            widest: combo_min_width(r)
                .unwrap_or_default()
                .max(tabs)
                .max(BASIC_WIDE)
                .max(BASIC_WIDTH),
        }
    }
}

/// How many columns the page lays its groups out in, from the theme alone (`rule`, each length
/// egui's own where the theme states none), as the gpui and iced pages count them: the most, of five down to one, whose width,
/// `(page - (n - 1) * layout.section_gap) / n`, holds the widest fixed-width control the theme
/// states on the page (`ColumnRule::widest`), where `page` is `content_width` less
/// `layout.window_margin` on both sides and, where `scrollbar.overlay_mode` is false, the groove
/// width a scrolling page keeps free. The column also holds `segmented`, the Segmented
/// control's own width as egui laid it out (`DemoState::segmented_width`), whose segments stay
/// on one row: the control may reach into the section gap after its column, but keeps
/// `layout.widget_gap`, the gap the page's rows keep between neighbouring widgets (Button and
/// Primary), from the next column's left edge (M9-166); in the last column it may reach into
/// the page's right margin, `layout.window_margin`, keeping `layout.widget_gap` from the content
/// panel's edge or its scroll bar: where `scrollbar.overlay_mode` is true the bar floats over
/// that margin, its groove width taken off the room. One where not even one does: the page is
/// then wider than its area.
pub(crate) fn column_count(rule: &ColumnRule, content_width: f32, segmented: f32) -> usize {
    let ColumnRule {
        margin,
        gap,
        widget_gap,
        overlay,
        groove,
        widest,
    } = *rule;
    let gutter = if overlay { 0.0 } else { groove };
    // a floating bar takes no width of the page, and is drawn over its right margin
    let overlay = if overlay { groove } else { 0.0 };
    let page = content_width - margin - margin - gutter;
    (MIN_COLUMNS..=MAX_COLUMNS)
        .rev()
        .find(|&n| {
            let column = (page - (n as f32 - 1.0) * gap) / n as f32;
            // past the column into the section gap, or in the last column into the page's
            // right margin, keeping `layout.widget_gap` from what lies beyond either: the
            // scroll area's edge, or a floating bar drawn over the margin
            let room = if segmented_in_last_column(n) {
                column + margin - overlay - widget_gap
            } else {
                column + gap - widget_gap
            };
            column >= widest && room >= segmented
        })
        .unwrap_or(MIN_COLUMNS)
}

/// One group of the page, by its index in reading order.
fn group(
    ix: usize,
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    match ix {
        0 => buttons(reg, atlas, ui),
        1 => checkboxes(reg, atlas, ui),
        2 => radios(reg, state, ui),
        // The companion crate's drop-down (docs/archive/todo_egui-widgets-spec.md §4.6).
        3 => drop_down(reg, state, atlas, ui),
        4 => text_inputs(reg, state, atlas, ui),
        5 => text_area(reg, state, atlas, ui),
        // The companion crate's slider (docs/archive/todo_egui-widgets-spec.md §4.2).
        6 => slider(reg, state, ui),
        7 => switches(reg, ui),
        8 => number_input(reg, state, atlas, ui),
        9 => spinner(reg, ui),
        10 => segmented(reg, state, ui),
        11 => card(reg, atlas, ui),
        12 => typography(reg, ui),
        13 => separator(reg, ui),
        14 => progress_bar(reg, ui),
        15 => list_group(reg, state, atlas, ui),
        16 => icons_group(reg, state, atlas, ui, chosen),
        17 => tabs(reg, state, atlas, ui),
        18 => expander(reg, ui),
        _ => table_group(reg, atlas, ui),
    }
}

/// Columns of equal width, as many as [`column_count`] counts from the content panel's width,
/// `layout.section_gap` apart, the gap the count takes, and where the theme states none egui's
/// own gap between columns, `item_spacing.x` (`ColumnRule::gap`); each a stack of groups
/// top-aligned, `layout.section_gap` apart and, where unstated, egui's own gap between the
/// widgets of a top-down `Ui`, `item_spacing.y` (egui-0.37.0 `src/layout.rs:759`,
/// `Layout::advance_after_rects`), each group in a top-down `Ui` of its own with its rows the
/// connector's `item_spacing` apart (`layout.widget_gap`, else egui's own): each column is laid
/// out in a `Ui` of its own at its place, so a group wider than its column shows as the
/// overflow it is, and the next column does not move.
pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let rule = ColumnRule::of(atlas.next_shown(ui.ctx()), ui.style());
    let gap = rule.gap;
    let below = demo::next_metric(ui, |l| l.section_gap).unwrap_or(ui.spacing().item_spacing.y);
    let origin = ui.cursor().min;
    let count = column_count(&rule, state.content_width, state.segmented_width);
    let columns = count as f32;
    let page_width = ui.available_width();
    let width = ((page_width - gap * (columns - 1.0)) / columns).max(0.0);
    let mut used = egui::Rect::NOTHING;
    let pixels = ui.pixels_per_point();
    for (column, members) in arrangement(count).iter().enumerate() {
        // The left edge on a whole pixel, so a one-pixel border a control at the column's start
        // draws is one pixel wide, not two half-covered ones; the column the width the page
        // gives it, fraction and all, as the iced page's, so a row wraps where theirs does.
        let left = origin.x + column as f32 * (width + gap);
        let snap = |x: f32| egui::emath::GuiRounding::round_to_pixels(x, pixels);
        let rect = egui::Rect::from_x_y_ranges(
            snap(left)..=snap(left) + width,
            origin.y..=origin.y + ui.available_height(),
        );
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(("basic column", column))
                .max_rect(rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        // The groups follow one another `below` apart and no more: the column adds no
        // `item_spacing` between them, each group's own `Ui` puts it between its rows, and a
        // group's heading, first in its `Ui`, adds nothing (`caption`).
        let rows = child.spacing().item_spacing;
        child.spacing_mut().item_spacing.y = 0.0;
        for (place, &ix) in members.iter().enumerate() {
            if place > 0 {
                child.add_space(below);
            }
            let _laid = child.scope_builder(
                egui::UiBuilder::new()
                    .id_salt(("basic group", ix))
                    .layout(egui::Layout::top_down(egui::Align::Min)),
                |ui| {
                    ui.spacing_mut().item_spacing = rows;
                    group(ix, reg, state, atlas, ui, chosen);
                },
            );
            #[cfg(test)]
            reg.basic_groups.push((ix, _laid.response.rect));
        }
        let content = child.min_rect();
        reg.place(
            ui,
            &format!("basic.column_{}", column.saturating_add(1)),
            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), content.height())),
        );
        used = used.union(content);
    }
    if used.is_positive() {
        reg.place(
            ui,
            "basic.page",
            egui::Rect::from_min_size(origin, egui::vec2(page_width, used.max.y - origin.y)),
        );
        // The page takes its own width, whatever reaches into its right margin (the Segmented
        // control in the last column may, `column_count`): a wider rectangle would widen the
        // margin's frame, the scroll area and the panel past the window.
        let page = egui::Rect::from_min_max(
            used.min,
            egui::pos2(used.max.x.min(origin.x + page_width), used.max.y),
        );
        ui.advance_cursor_after_rect(page);
    }
}

/// A group's heading, the element `id`.
fn heading(reg: &mut Registry, ui: &mut egui::Ui, id: &str, text: &str) {
    let response = caption(reg, ui, text);
    reg.tag(id, &response);
}

/// `text` as a widget of `ui` lays it out (`egui/src/atomics/atom_kind.rs:134-135`): on one
/// line, in `font`.
fn text_size(
    ui: &egui::Ui,
    text: impl Into<egui::WidgetText>,
    font: egui::FontSelection,
) -> egui::Vec2 {
    text.into()
        .into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font)
        .size()
}

/// `rect` less `margin` on each side, spelled in `f32` as egui's `Rect - MarginF32` computes it
/// (`epaint/src/margin_f32.rs:269-279`).
fn inside(rect: egui::Rect, margin: egui::epaint::MarginF32) -> egui::Rect {
    egui::Rect::from_min_max(
        egui::pos2(rect.min.x + margin.left, rect.min.y + margin.top),
        egui::pos2(rect.max.x - margin.right, rect.max.y - margin.bottom),
    )
}

/// Where a text-only `Button` of `ui` laid its label out: the text in the button style's font,
/// placed in the button less its frame's margin by the layout's alignment
/// (`egui/src/widgets/button.rs:325-367`, `egui/src/widget_style.rs:146-171`,
/// `egui/src/atomics/atom_layout.rs:340-342`, `:619`).
fn button_label(
    ui: &egui::Ui,
    button: &egui::Response,
    text: impl Into<egui::WidgetText>,
) -> egui::Rect {
    let style = ui
        .ctx()
        .get_widget_style::<ButtonStyle>(&StyleArgs {
            classes: &Classes::default(),
            state: WidgetState::Inactive,
            stack: ui.stack(),
            style: ui.style(),
            ctx: ui.ctx(),
        })
        .atom_layout;
    let size = text_size(
        ui,
        text,
        egui::FontSelection::FontId(style.text_style.font_id),
    );
    let inner = inside(button.rect, style.frame.total_margin());
    let layout = ui.layout();
    egui::Align2([layout.horizontal_align(), layout.vertical_align()])
        .align_size_within_rect(size, inner)
}

/// The parts of an egui `Checkbox` of `ui` (`egui/src/widgets/checkbox.rs:76-140`): its box
/// atom `checkbox_size` wide at the start of the row, the box that size square on the row's
/// centre line, the check mark `check_size` square in it (drawn only when checked), the label
/// `icon_spacing` after the atom, centred on the row.
fn checkbox_parts(
    ui: &egui::Ui,
    control: &egui::Response,
    text: egui::RichText,
    checked: bool,
) -> Vec<(&'static str, egui::Rect)> {
    let style = ui.ctx().get_widget_style::<CheckboxStyle>(&StyleArgs {
        classes: &Classes::default(),
        state: WidgetState::Inactive,
        stack: ui.stack(),
        style: ui.style(),
        ctx: ui.ctx(),
    });
    let inner = inside(control.rect, style.atom_layout.frame.total_margin());
    let atom = egui::Rect::from_x_y_ranges(
        inner.left()..=inner.left() + style.checkbox_size,
        inner.y_range(),
    );
    let indicator = egui::Rect::from_center_size(
        egui::pos2(atom.left() + 0.5 * style.checkbox_size, atom.center().y),
        egui::Vec2::splat(style.checkbox_size),
    );
    let size = text_size(ui, text, egui::FontSelection::Default);
    let label = egui::Rect::from_min_size(
        egui::pos2(
            atom.right() + ui.spacing().icon_spacing,
            atom.center().y - 0.5 * size.y,
        ),
        size,
    );
    let mut parts = vec![("indicator", indicator)];
    if checked {
        parts.push((
            "mark",
            egui::Rect::from_center_size(indicator.center(), egui::Vec2::splat(style.check_size)),
        ));
    }
    parts.push(("label", label));
    parts
}

/// A companion-crate widget's `Parts`, each placed as `<id>.<element part>`: `names` pairs the
/// widget's part with the element list's.
fn place_parts(
    reg: &mut Registry,
    ui: &egui::Ui,
    id: &str,
    response: &egui::Response,
    names: &[(&str, &str)],
) {
    let Some(parts) = Parts::of(response) else {
        return;
    };
    for (part, element) in names {
        if let Some(rect) = parts.get(part) {
            reg.place(ui, &format!("{id}.{element}"), rect);
        }
    }
}

/// A text-only button in the button role's `variant`, the element `id`, its label placed as
/// `<id>.label`: `make` builds it from its label `text`, one line box tall (`demo::next_lined`), at
/// least `min` large.
fn text_button(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    (id, kind, variant): (&str, &'static str, RoleVariant),
    (text, min): (&str, egui::Vec2),
    make: impl FnOnce(egui::RichText) -> Button<'static>,
    enabled: bool,
) -> egui::Response {
    wrap_before(ui, id);
    let mut label = None;
    let response = demo::scoped(reg, ui, Role::Button, variant, kind, |ui| {
        // The label on one line, as the other two showcases set it: in a wrapping row egui
        // would wrap it to the room left (`TextWrapMode::Wrap`), where the row wraps before the
        // button instead (`wrap_before`).
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
        let text = demo::next_lined(ui, text, egui::TextStyle::Button);
        let r = ui.add_enabled(enabled, make(text.clone()).min_size(min));
        label = Some(button_label(ui, &r, text));
        r
    });
    reg.tag(id, &response);
    remember_width(ui, id, response.rect);
    if let Some(label) = label {
        reg.place(ui, &format!("{id}.label"), label);
    }
    response
}

/// In a wrapping row (`Ui::horizontal_wrapped`), starts a new line before the item `key` where
/// the width it took in the last pass no longer fits beside what the line holds. An item added
/// through a scope — every role scope, `Ui::scope` — is allocated after it is laid out and
/// never wraps by itself (`egui/src/ui.rs:2203-2215`, `scope_dyn`: the child takes the room
/// before the wrap, and the row only advances past it); a first pass, with no width yet, lays
/// the line out as it comes.
fn wrap_before(ui: &mut egui::Ui, key: &str) {
    let id = egui::Id::unique(("basic wrap", key));
    let Some(width) = ui.ctx().data(|d| d.get_temp::<f32>(id)) else {
        return;
    };
    let line_start = ui.max_rect().left();
    let at = ui.cursor().left();
    if at > line_start + 0.5 && at + width > ui.max_rect().right() + 1e-3 {
        // A new line `item_spacing.y` under this one and as tall, inside the row's room: `end_row`
        // opens it with no height below the room so far (`egui/src/layout.rs:769-778`), and a
        // scope takes the room between the line's top and the room's bottom
        // (`available_from_cursor_max_rect`, `:446-450`), which would lift its item.
        let height = ui.cursor().height();
        ui.end_row();
        ui.set_row_height(height);
        let bottom = ui.cursor().bottom();
        ui.expand_to_include_y(bottom);
    }
}

/// The width of the item `key`, laid out at `rect`, for [`wrap_before`] in the next pass.
fn remember_width(ui: &egui::Ui, key: &str, rect: egui::Rect) {
    let id = egui::Id::unique(("basic wrap", key));
    ui.ctx().data_mut(|d| d.insert_temp(id, rect.width()));
}

/// Buttons (the toggle buttons and a dialog's default button their third row), check boxes,
/// radio buttons, the drop-down.
/// The Buttons group: two rows of two buttons and a third of three, each row wrapping onto a
/// further line where the column is too narrow for it (`horizontal_wrapped`), as the gpui
/// (`flex_wrap`) and iced (`Row::wrap`) pages' rows wrap.
fn buttons(reg: &mut Registry, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let normal = RoleVariant::Normal;
    // `button.body(NONE).min_size[0]`, which egui's `Button` never reads from the style: it
    // raises only its height, to `interact_size.y`, the button scope's, so the width is the
    // application's, per call (connector spec §5.3, `Button::min_size`); egui's own, none, where
    // the theme states none.
    let min = button_min_size(r);
    let plain = |text: egui::RichText| Button::new(text);
    let primary = |text: egui::RichText| Button::new(text).selected(true);

    heading(reg, ui, "basic.buttons.heading", "Buttons");
    ui.horizontal_wrapped(|ui| {
        let kind = ("basic.buttons.default", "button (enabled)", normal);
        text_button(reg, ui, kind, ("Button", min), plain, true);
        let kind = ("basic.buttons.primary", "button (suggested action)", normal);
        text_button(reg, ui, kind, ("Primary", min), primary, true);
    });
    ui.horizontal_wrapped(|ui| {
        let kind = (
            "basic.buttons.disabled",
            "button (disabled)",
            RoleVariant::Disabled,
        );
        text_button(reg, ui, kind, ("Disabled", min), plain, false);
        let kind = ("basic.buttons.tooltip", "tooltip button", normal);
        let owner = text_button(reg, ui, kind, ("Tooltip", min), plain, true);
        let bubble = demo::surfaced(
            reg,
            ui,
            Surface::Tooltip,
            false,
            Some((Role::Tooltip, normal)),
            "tooltip",
            |_, chrome, reg| {
                let mut tip = egui::Tooltip::for_enabled(&owner);
                tip.popup = tip.popup.frame(chrome.frame);
                if let Some(modifier) = chrome.modifier {
                    tip.popup = tip.popup.style(modifier);
                }
                tip.show(|ui| {
                    let text = demo::scoped(reg, ui, Role::Tooltip, normal, "tooltip text", |ui| {
                        ui.label(demo::next_lined(ui, "A tooltip", egui::TextStyle::Body))
                    });
                    reg.tag("basic.buttons.tooltip.bubble.text", &text);
                })
                .map(|out| out.response)
            },
        );
        if let Some(bubble) = bubble {
            reg.tag("basic.buttons.tooltip.bubble", &bubble);
        }
    });
    // A toggle button: `Off` a button at rest, `On` the same button held on: egui's own selected
    // button (`Button::selected`), with the checked colours [`toggle_on`] reads laid over it per
    // call where the theme states them (spec §10, rule 0).
    let on = toggle_on(r);
    let (on_fill, on_text) = (on.fill, on.text);
    // A dialog's default button, the one Enter activates: where the theme gives it a look of its
    // own, per call, since `Button::fill` and `Button::stroke` override egui's on-hover effects,
    // in the state of its previous response, kept in the page's memory (focus before hover, as
    // egui's `Widgets::style` tests them); pressed, no fill and no label colour, so the button
    // role's pressed pair shows. Where the theme gives it none, the primary button.
    let state_key = egui::Id::unique(("basic state", "basic.buttons.dialog_default"));
    let (hovered, pressed, focused) = ui
        .ctx()
        .data(|d| d.get_temp::<(bool, bool, bool)>(state_key))
        .unwrap_or_default();
    // `None`, the primary button, also where the theme states the look in a form egui cannot
    // draw (`next::Unmapped`).
    let look = r.and_then(|r| next::default_button(r).ok().flatten());
    ui.horizontal_wrapped(|ui| {
        let kind = ("basic.buttons.toggle_off", "toggle button (off)", normal);
        text_button(reg, ui, kind, ("Off", min), plain, true);
        let kind = ("basic.buttons.toggle_on", "toggle button (on)", normal);
        let held = |text: egui::RichText| {
            let text = match on_text {
                Some(color) => text.color(color),
                None => text,
            };
            let button = Button::new(text).selected(true);
            match on_fill {
                Some(fill) => button.fill(fill),
                None => button,
            }
        };
        text_button(reg, ui, kind, ("On", min), held, true);
        let kind = (
            "basic.buttons.dialog_default",
            "button (dialog default)",
            normal,
        );
        let response = match look {
            Some(d) => {
                let own = move |text: egui::RichText| {
                    if pressed {
                        return Button::new(text).stroke(d.hover_stroke);
                    }
                    let stroke = if focused {
                        d.focus_stroke
                    } else if hovered {
                        d.hover_stroke
                    } else {
                        d.stroke
                    };
                    Button::new(text.color(d.text)).fill(d.fill).stroke(stroke)
                };
                text_button(reg, ui, kind, ("OK", min), own, true)
            }
            None => text_button(reg, ui, kind, ("OK", min), primary, true),
        };
        let now = (
            response.hovered(),
            response.is_pointer_button_down_on(),
            response.has_focus(),
        );
        if now != (hovered, pressed, focused) {
            // drawn from the previous response: one more pass shows the state it is in now
            ui.ctx().request_repaint();
        }
        ui.ctx().data_mut(|d| d.insert_temp(state_key, now));
    });
    // Where the theme states no checked colour, what the toggle is drawn in instead.
    if on.fill.is_none() {
        reg.note_element(
            "basic.buttons.toggle_on",
            "fill while on",
            &format!(
                "not stated by the theme (button.body(CHECKED).fill): {}",
                ToggleOn::OWN_FILL
            ),
        );
    }
    if on.text.is_none() {
        reg.note_element(
            "basic.buttons.toggle_on",
            "text colour while on",
            &format!(
                "not stated by the theme (button.label(CHECKED).color): {}",
                ToggleOn::OWN_TEXT
            ),
        );
    }
}

/// The Basic page's toggle button while on: the colours a rule of the installed 0.8.0 theme
/// states for the checked state, each `None` where none does, and so egui's own selected
/// button's.
#[derive(Debug, PartialEq)]
pub(crate) struct ToggleOn {
    pub(crate) fill: Option<egui::Color32>,
    pub(crate) text: Option<egui::Color32>,
}

impl ToggleOn {
    /// What an unstated checked fill is drawn in.
    pub(crate) const OWN_FILL: &'static str =
        "egui's own selected button fill (Button::selected: visuals.selection.bg_fill)";
    /// What an unstated checked text colour is drawn in.
    pub(crate) const OWN_TEXT: &'static str =
        "egui's own selected button text (Button::selected: visuals.selection.stroke)";
}

/// The toggle button's colours while on, of the installed 0.8.0 theme `r`
/// (docs/platform-facts.md §2.3), each asked with the checked flag alone: the fill a rule states
/// for `button.body(CHECKED)`, the text colour a rule states for `button.label(CHECKED)`. Neither
/// falls back to another state's: what the theme leaves unstated, and both without an installed
/// 0.8.0 theme, is egui's own way of drawing a button that is on, its selected button
/// (`Button::selected`, egui-0.37.0 `src/theme/default_style.rs`: `visuals.selection`'s fill
/// and stroke), as the connector themes it (spec §10, rules 0 and 0a).
pub(crate) fn toggle_on(r: Option<&native_theme::next::ResolvedTheme>) -> ToggleOn {
    ToggleOn {
        fill: r.and_then(|r| next::state_fill_of(&r.button.body(NextState::CHECKED))),
        text: r.and_then(|r| next::state_text_color_of(&r.button.label(NextState::CHECKED))),
    }
}

/// The Checkboxes group.
fn checkboxes(reg: &mut Registry, atlas: &OldAtlas, ui: &mut egui::Ui) {
    // The page's ground the disabled box fades over: `colors.window.background` of the installed
    // 0.8.0 theme, egui's own panel fill where it states none or none is installed.
    let ground = atlas
        .next_shown(ui.ctx())
        .and_then(|r| r.colors.window.background)
        .map_or(ui.visuals().panel_fill, demo::next_color32);
    let normal = RoleVariant::Normal;
    // Each control shows one state and is held in it: a click changes a copy made for the pass.
    // One per row, each in its own checkbox scope, whose `interact_size.y` is the indicator.
    // The disabled box is checked: the `SelectedDisabled` cell, the checked box with the
    // theme's disabled fill where it states one, faded as one by its `disabled_alpha`,
    // `checkbox.disabled_opacity` (docs/platform-facts.md §2.1.6).
    heading(reg, ui, "basic.checkboxes.heading", "Checkboxes");
    for (label, checked, variant, enabled, kind, id) in [
        (
            "Unchecked",
            false,
            normal,
            true,
            "checkbox (unchecked)",
            "basic.checkboxes.unchecked",
        ),
        (
            "Checked",
            true,
            RoleVariant::Selected,
            true,
            "checkbox (checked)",
            "basic.checkboxes.checked",
        ),
        (
            "Disabled",
            true,
            RoleVariant::SelectedDisabled,
            false,
            "checkbox (disabled)",
            "basic.checkboxes.disabled",
        ),
    ] {
        let mut value = checked;
        let mut parts = Vec::new();
        let response = demo::scoped(reg, ui, Role::Checkbox, variant, kind, |ui| {
            let text = demo::next_lined(ui, label, egui::TextStyle::Body);
            let r = if !enabled {
                ui.scope(|ui| {
                    // The check box faded as one over the page (the companion crate's rule,
                    // which its switch and radio button follow too).
                    fade_as_one(ui, ground);
                    ui.add(egui::Checkbox::new(&mut value, text.clone()))
                })
                .inner
            } else {
                ui.add(egui::Checkbox::new(&mut value, text.clone()))
            };
            parts = checkbox_parts(ui, &r, text, checked);
            r
        });
        reg.tag(id, &response);
        for (part, rect) in parts {
            reg.place(ui, &format!("{id}.{part}"), rect);
        }
    }
}

/// The Radio buttons group: the companion crate's radio button
/// (docs/archive/todo_egui-widgets-spec.md §4.8): egui's, in `RoleVariant::Selected` while selected as
/// on the Selection page, its dot `checkbox.radio_dot_diameter` across where the theme states
/// one.
fn radios(reg: &mut Registry, state: &mut DemoState, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.radios.heading", "Radio buttons");
    for (i, (label, id)) in [
        ("Option A", "basic.radios.option_a"),
        ("Option B", "basic.radios.option_b"),
    ]
    .into_iter()
    .enumerate()
    {
        let selected = state.basic_radio == i;
        let variant = if selected {
            RoleVariant::Selected
        } else {
            normal
        };
        let r = demo::widget(reg, ui, Role::Checkbox, variant, "RadioButton", |ui| {
            let text = demo::next_lined(ui, label, egui::TextStyle::Body);
            ui.add(RadioButton::new(selected, text))
        });
        reg.tag(id, &r);
        let names = [
            ("indicator", "indicator"),
            ("dot", "dot"),
            ("label", "label"),
        ];
        place_parts(reg, ui, id, &r, &names);
        if r.clicked() {
            state.basic_radio = i;
        }
    }
}

/// The Switches group: the companion crate's switch (docs/archive/todo_egui-widgets-spec.md §4.1),
/// `switch.*`'s track and thumb, which no egui widget draws, the label after it;
/// `.enabled(false)` is the platform's disabled switch. Each is held in its state, as the check
/// boxes are.
fn switches(reg: &mut Registry, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.switches.heading", "Switches");
    for (label, on, enabled, kind, id) in [
        ("Off", false, true, "switch (off)", "basic.switches.off"),
        ("On", true, true, "switch (on)", "basic.switches.on"),
        (
            "Disabled",
            true,
            false,
            "switch (disabled)",
            "basic.switches.disabled",
        ),
    ] {
        let variant = if enabled {
            normal
        } else {
            RoleVariant::Disabled
        };
        let mut value = on;
        let r = demo::widget(reg, ui, Role::Switch, variant, kind, |ui| {
            let text = demo::next_lined(ui, label, egui::TextStyle::Body);
            ui.add(Switch::new(&mut value).label(text).enabled(enabled))
        });
        reg.tag(id, &r);
        let names = [("track", "track"), ("thumb", "thumb"), ("label", "label")];
        place_parts(reg, ui, id, &r, &names);
    }
}

/// A text field of the Basic page, built inside its `Role::Input` scope (`ui` is the scope's), so
/// its frame is the style it paints in: the disabled field takes the disabled cell's fill.
/// `input.field(NONE).min_size[1]` is a per-instance `min_size`, which a `TextEdit` never reads
/// from the style: it is one row plus its margin tall, at least its `min_size` (connector spec
/// §5.4); the row is centred in the height the minimum adds, as the platform centres a field's
/// text. `frame` is [`field_frame`]'s, egui's own where it is `None`.
fn field<'a>(
    text: &'a mut String,
    id: egui::Id,
    r: Option<&native_theme::next::ResolvedTheme>,
    frame: Option<egui::Frame>,
) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .desired_width(BASIC_WIDTH)
        .min_size(egui::vec2(BASIC_WIDTH, input_min_size(r).y))
        .align(egui::Align2::LEFT_CENTER)
        .id(id)
        .framed(frame)
}

/// A `TextEdit` of `ui` lays each row out `row_height + extra_text_line_spacing` tall
/// (`egui/src/widgets/text_edit/builder.rs:488-490`, `:512-514`); the connector writes the
/// spacing only where the line box, the font's size times the installed 0.8.0 theme's line
/// height (`next::line_height`), is taller than the font's own row (§6.15). Where the row is
/// taller — a face whose row exceeds the theme's line box — the spacing is the negative
/// difference, written per field, so each row is one line box tall, as a label's is
/// (`demo::next_lined`). Without an installed 0.8.0 theme, egui's own spacing.
fn line_box_rows(ui: &mut egui::Ui) {
    let Some(line_height) = demo::next_value(ui, next::line_height) else {
        return;
    };
    let font = egui::FontSelection::Default.resolve(ui.style());
    let row = ui.fonts_mut(|f| f.row_height(&font));
    ui.spacing_mut().extra_text_line_spacing = font.size * line_height - row;
}

/// The frame a `TextEdit` of `ui` draws in: `frame` where the page hands it one, else egui's own,
/// the `TextEditStyle` of the scope at rest (`egui/src/widgets/text_edit/builder.rs`, which
/// takes its `atom_layout.frame` where it is given none).
fn drawn_frame(ui: &egui::Ui, frame: Option<egui::Frame>) -> egui::Frame {
    frame.unwrap_or_else(|| {
        ui.ctx()
            .get_widget_style::<TextEditStyle>(&StyleArgs {
                classes: &Classes::default(),
                state: WidgetState::Inactive,
                stack: ui.stack(),
                style: ui.style(),
                ctx: ui.ctx(),
            })
            .atom_layout
            .frame
    })
}

/// Where a `TextEdit` of `ui` laid `text` out: at the start of its frame's content, on the
/// row's centre line for a single-line field, at its top for a multi-line one
/// (`egui/src/widgets/text_edit/builder.rs`), in the field's font, each row one line box tall
/// (`line_box_rows`).
fn field_text(
    ui: &egui::Ui,
    field: &egui::Response,
    frame: &egui::Frame,
    text: &str,
    centred: bool,
) -> egui::Rect {
    let inner = inside(field.rect, frame.total_margin());
    let size = text_size(
        ui,
        demo::next_lined(ui, text, egui::TextStyle::Body),
        egui::FontSelection::Default,
    );
    let top = if centred {
        inner.center().y - 0.5 * size.y
    } else {
        inner.top()
    };
    egui::Rect::from_min_size(egui::pos2(inner.left(), top), size)
}

/// A single-line field of the Basic page in `variant` of the input role, the element `id`: its
/// text placed as `<id>.text`, `shown` the text it holds (or its hint). `add` builds the field
/// in the frame [`field_frame`] gives in the scope, `next::input_frame` of the installed 0.8.0
/// theme `r`.
fn text_field(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    (id, kind, variant): (&str, &'static str, RoleVariant),
    r: Option<&native_theme::next::ResolvedTheme>,
    shown: &str,
    add: impl FnOnce(&mut egui::Ui, egui::Id, Option<egui::Frame>) -> egui::Response,
) -> egui::Response {
    let edit_id = ui.make_persistent_id(id);
    let mut text = None;
    let response = demo::scoped(reg, ui, Role::Input, variant, kind, |ui| {
        line_box_rows(ui);
        let frame = field_frame(ui, edit_id, r, next::input_frame);
        let field = add(ui, edit_id, frame).accessible_name(kind);
        let frame = drawn_frame(ui, frame);
        text = Some(field_text(ui, &field, &frame, shown, true));
        field
    });
    reg.tag(id, &response);
    if let Some(text) = text {
        reg.place(ui, &format!("{id}.text"), text);
    }
    response
}

/// Text inputs (the focused one their fourth row), the text area, the slider.
/// The Text inputs group.
fn text_inputs(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let normal = RoleVariant::Normal;

    heading(reg, ui, "basic.text_inputs.heading", "Text inputs");
    let kind = ("basic.text_inputs.placeholder", "TextEdit (hint)", normal);
    text_field(reg, ui, kind, r, "Placeholder", |ui, id, frame| {
        let hint = demo::next_lined(ui, "Placeholder", egui::TextStyle::Body);
        ui.add(field(&mut state.basic_hint, id, r, frame).hint_text(hint))
    });
    let kind = ("basic.text_inputs.filled", "TextEdit (single line)", normal);
    let shown = state.basic_text.clone();
    text_field(reg, ui, kind, r, &shown, |ui, id, frame| {
        ui.add(field(&mut state.basic_text, id, r, frame))
    });
    let kind = (
        "basic.text_inputs.disabled",
        "TextEdit (disabled)",
        RoleVariant::Disabled,
    );
    let mut disabled = "Disabled".to_string();
    text_field(reg, ui, kind, r, "Disabled", |ui, id, frame| {
        ui.add_enabled(false, field(&mut disabled, id, r, frame))
    });
    // A text field holding the keyboard focus from the start, so its focus border shows.
    let kind = ("basic.text_inputs.focused", "TextEdit (focused)", normal);
    let shown = state.basic_focused.clone();
    let focused = text_field(reg, ui, kind, r, &shown, |ui, id, frame| {
        ui.add(field(&mut state.basic_focused, id, r, frame))
    });
    if !state.basic_focus_given {
        focused.request_focus();
        state.basic_focus_given = true;
    }
}

/// The Text area group: its frame `next::text_area_frame` of the installed 0.8.0 theme
/// ([`field_frame`]), egui's own where the theme states none egui can take.
fn text_area(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.text_area.heading", "Text area");
    let id = ui.make_persistent_id("basic/area");
    let mut text = None;
    let kind = "TextEdit (multiline)";
    let area = demo::scoped(reg, ui, Role::Input, normal, kind, |ui| {
        line_box_rows(ui);
        let frame = field_frame(ui, id, r, next::text_area_frame);
        let field = ui
            .add(
                egui::TextEdit::multiline(&mut state.basic_area)
                    .desired_width(BASIC_WIDE)
                    .desired_rows(AREA_ROWS)
                    .id(id)
                    .framed(frame),
            )
            .accessible_name(kind);
        let frame = drawn_frame(ui, frame);
        text = Some(field_text(ui, &field, &frame, AREA_TEXT, false));
        field
    });
    reg.tag("basic.text_area.field", &area);
    if let Some(text) = text {
        reg.place(ui, "basic.text_area.field.text", text);
    }
}

/// The Slider group: the companion crate's slider (docs/archive/todo_egui-widgets-spec.md §4.2),
/// `slider.*`'s rail, trailing fill and a knob in `slider.thumb_color`, which egui paints in
/// the rail's colour.
fn slider(reg: &mut Registry, state: &mut DemoState, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.slider.heading", "Slider");
    let kind = "Slider (horizontal)";
    let slider = demo::widget(reg, ui, Role::Slider, normal, kind, |ui| {
        ui.scope(|ui| {
            ui.spacing_mut().slider_width = BASIC_WIDTH;
            ui.add(Slider::new(&mut state.basic_slider, 0.0..=100.0))
                .accessible_name(kind)
        })
        .inner
    });
    reg.tag("basic.slider.control", &slider);
    let names = [("track", "track"), ("fill", "fill"), ("thumb", "thumb")];
    place_parts(reg, ui, "basic.slider.control", &slider, &names);
}

/// The Progress bar group: the companion crate's progress bar
/// (docs/archive/todo_egui-widgets-spec.md §4.9), egui's, rounded `progress_bar.border.corner_radius`
/// and outlined as `progress_bar.border` states, which egui's `ProgressBar` does not draw.
fn progress_bar(reg: &mut Registry, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.progress_bar.heading", "Progress bar");
    let bar = demo::scoped(reg, ui, Role::ProgressBar, normal, "ProgressBar", |ui| {
        ui.add(ProgressBar::new(PROGRESS).desired_width(BASIC_WIDTH))
    });
    reg.tag("basic.progress_bar.bar", &bar);
    place_parts(reg, ui, "basic.progress_bar.bar", &bar, &[("fill", "fill")]);
}

/// `combo_box.field(NONE).min_size[0]` of the installed 0.8.0 theme `r`, the drop-down's least
/// width; `None` where it states none or none is installed.
fn combo_min_width(r: Option<&native_theme::next::ResolvedTheme>) -> Option<f32> {
    r.and_then(|r| {
        let [width, _] = r.combo_box.field(NextState::NONE).min_size;
        width
    })
}

/// The Drop-down group: the companion crate's drop-down (docs/archive/todo_egui-widgets-spec.md §4.6),
/// egui's own `ComboBox` in the combo-box scope, as tall as its text and padding make it, at
/// least `combo_box.field(NONE).min_size[1]`, where egui's square arrow box would make it taller.
fn drop_down(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.drop_down.heading", "Drop-down");
    let current = FRUITS.get(state.basic_combo).copied().unwrap_or_default();
    // `BASIC_WIDTH`, and at least `combo_box.field(NONE).min_size[0]` of the installed 0.8.0
    // theme, which egui's `ComboBox` never reads from the style: the width is the application's,
    // per call, as the gpui page's select's refinement and the iced page's pick list take it;
    // egui's own least width, none, where the theme states none.
    let width = BASIC_WIDTH.max(combo_min_width(r).unwrap_or_default());
    let combo = demo::scoped_popup(
        reg,
        ui,
        Role::ComboBox,
        normal,
        "ComboBox",
        |ui, modifier, row, reg| {
            let mut combo = ComboBox::from_id_salt("basic/combo")
                .selected_text(demo::next_lined(ui, current, egui::TextStyle::Button))
                .width(width);
            if let Some(modifier) = modifier {
                combo = combo.popup_style(modifier);
            }
            combo
                .show_ui(ui, |ui| {
                    for (i, fruit) in FRUITS.into_iter().enumerate() {
                        row.add(reg, ui, "ComboBox row", |ui| {
                            ui.selectable_value(&mut state.basic_combo, i, fruit)
                        });
                    }
                })
                .response
                .accessible_name("ComboBox")
        },
    );
    reg.tag("basic.drop_down.trigger", &combo);
    let names = [("text", "text"), ("arrow", "arrow")];
    place_parts(reg, ui, "basic.drop_down.trigger", &combo, &names);
}

/// The switches, the number input, the spinner, the segmented control, a card.
/// The Number input group.
fn number_input(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let [_, height] = atlas
        .next_shown(ui.ctx())
        .map_or([None, None], |r| r.input.field(NextState::NONE).min_size);
    let normal = RoleVariant::Normal;

    // A number input: egui's `DragValue`, the number field egui has, in the input role's
    // scope, `BASIC_WIDTH` wide and `input.field(NONE).min_size[1]` tall (its `interact_size`,
    // which `DragValue` sizes its button by, `egui/src/widgets/drag_value.rs`), the scope's own
    // height where the installed 0.8.0 theme states none, stepping by 1. egui draws it as a
    // button whose text a drag or a click edits: no step buttons.
    heading(reg, ui, "basic.number_input.heading", "Number input");
    let shown = format!("{}", state.basic_number);
    let mut text = None;
    let number = demo::scoped(reg, ui, Role::Input, normal, "DragValue", |ui| {
        let interact = &mut ui.spacing_mut().interact_size;
        interact.x = BASIC_WIDTH;
        if let Some(height) = height {
            interact.y = height;
        }
        // The field's frame as a text field's: `DragValue` is a `Button`, which pads by the
        // scope's `button_padding` with its stroke inside (`egui-0.37.0/src/theme/default_style.rs`,
        // `StyleProvider<ButtonStyle>`), as a plain `TextEdit` in the scope does
        // (`StyleProvider<TextEditStyle>`): the «input» cell's, `input.border.padding` under
        // the pair rule. A `Button` fills with the state's `weak_bg_fill`, where a `TextEdit`
        // fills with `text_edit_bg_color`.
        let fill = ui.visuals().text_edit_bg_color();
        let widgets = &mut ui.visuals_mut().widgets;
        for state in [
            &mut widgets.inactive,
            &mut widgets.hovered,
            &mut widgets.active,
        ] {
            state.weak_bg_fill = fill;
        }
        let r = ui
            .add(egui::DragValue::new(&mut state.basic_number).speed(NUMBER_STEP))
            .accessible_name("DragValue");
        text = Some(button_label(ui, &r, &shown));
        r
    });
    reg.tag("basic.number_input.field", &number);
    if let Some(text) = text {
        reg.place(ui, "basic.number_input.field.text", text);
    }
}

/// The Segmented control group: the companion crate's segmented control
/// (docs/archive/todo_egui-widgets-spec.md §4.4): one control of joined buttons in one
/// `Role::SegmentedControl` scope, whose cell carries the segment height, padding and colours;
/// one outline in `border`, `separator_width` dividers between the segments; a radio group. Its
/// segments stay on one row at their own widths, as the platforms' segmented controls do; the
/// page's column count gives the control a column it overruns by at most the section gap less
/// `layout.widget_gap` ([`column_count`], M9-166).
fn segmented(reg: &mut Registry, state: &mut DemoState, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.segmented.heading", "Segmented control");
    let control = demo::widget(
        reg,
        ui,
        Role::SegmentedControl,
        normal,
        "segmented control",
        |ui| {
            let segments = SEGMENTS.map(|s| demo::next_lined(ui, s, egui::TextStyle::Button));
            ui.add(SegmentedControl::new(&mut state.basic_segment, segments))
        },
    );
    // Its width for the next pass's column count ([`column_count`]): on one row its width does
    // not depend on the column, so a pass that learns a new width is run again with it.
    let width = control.rect.width();
    if (width - state.segmented_width).abs() > 0.01 {
        state.segmented_width = width;
        ui.ctx().request_discard("the segmented control's width");
        ui.ctx().request_repaint();
    }
    reg.tag("basic.segmented.control", &control);
    if let Some(parts) = Parts::of(&control) {
        for (part, id) in [
            ("segment_0", "basic.segmented.day"),
            ("divider_0", "basic.segmented.divider_1"),
            ("segment_1", "basic.segmented.week"),
            ("divider_1", "basic.segmented.divider_2"),
            ("segment_2", "basic.segmented.month"),
        ] {
            if let Some(rect) = parts.get(part) {
                reg.place(ui, id, rect);
            }
        }
    }
}

/// The Tabs group: a tab bar of two tabs, the first selected, on the installed 0.8.0 theme
/// (`demo::TabLook::next`).
fn tabs(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    heading(reg, ui, "basic.tabs.heading", "Tabs");
    let tabs: Vec<(usize, &'static str)> = TABS.into_iter().enumerate().collect();
    let picked = demo::tab_bar(
        reg,
        ui,
        &demo::TabLook::next(atlas.next_shown(ui.ctx())),
        demo::TabBar {
            kind: "TabBar · Basic",
            tab_kind: "Tab · Basic",
            tabs: &tabs,
            current: state.basic_tab,
            margin: None,
            scroll: false,
            full_width: false,
            element: "basic.tabs.bar",
            tab_elements: &["basic.tabs.one", "basic.tabs.two"],
        },
        |_, _, _| {},
    );
    if let Some(tab) = picked {
        state.basic_tab = tab;
    }
}

/// The Spinner group: the companion crate's spinner (docs/archive/todo_egui-widgets-spec.md §4.3),
/// the icon set's loading indicator at `spinner.diameter`, or an arc at
/// `spinner.stroke_width`, which egui's `Spinner` hardcodes.
fn spinner(reg: &mut Registry, ui: &mut egui::Ui) {
    heading(reg, ui, "basic.spinner.heading", "Spinner");
    let spinner = demo::widget(
        reg,
        ui,
        Role::Spinner,
        RoleVariant::Normal,
        "Spinner",
        |ui| ui.add(Spinner::new()),
    );
    reg.tag("basic.spinner.indicator", &spinner);
}

/// One line of the Typography group in `role` (the base style's `Body`, `typography.body`, for
/// `None`), the element `id`.
fn typography_line(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Option<TextRole>,
    text: &str,
    id: &str,
) {
    let line = demo::base(reg, ui, "Label (typography)", |ui| match role {
        Some(role) => ui.label(demo::next_role_text(ui, role, text)),
        None => ui.label(demo::next_lined(ui, text, egui::TextStyle::Body)),
    });
    reg.tag(id, &line);
}

/// The Typography group.
fn typography(reg: &mut Registry, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;

    // One line in each of the theme's text roles: the text-scale roles through
    // `demo::next_role_text`, the body in the base style's `Body` (`typography.body`), the link
    // in the link role, the monospace line in the base style's `Monospace`
    // (`typography.monospace`).
    heading(reg, ui, "basic.typography.heading", "Typography");
    typography_line(
        reg,
        ui,
        Some(TextRole::Caption),
        "Caption",
        "basic.typography.caption",
    );
    typography_line(reg, ui, None, "Body", "basic.typography.body");
    // The companion crate's link (docs/archive/todo_egui-widgets-spec.md §4.5): egui's `Link` in the
    // link scope, its text in `link.*`'s rest, hover, pressed and disabled colours and
    // underlined at rest where `link.underline_enabled` says so, which egui's `Link` never
    // reads: it underlines only on hover or focus (`egui/src/widgets/hyperlink.rs:50-54`).
    let link = demo::widget(reg, ui, Role::Link, normal, "Link", |ui| {
        let link = links::link(ui, demo::next_lined(ui, "Link", egui::TextStyle::Body));
        ui.add(link)
    });
    reg.tag("basic.typography.link", &link);
    for (role, text, id) in [
        (
            TextRole::SectionHeading,
            "Section heading",
            "basic.typography.section_heading",
        ),
        (
            TextRole::DialogTitle,
            "Dialog title",
            "basic.typography.dialog_title",
        ),
    ] {
        typography_line(reg, ui, Some(role), text, id);
    }
    // The line box of the monospace font's size at the theme's line height
    // (`demo::next_lined`), as a body line's is.
    let mono = demo::base(reg, ui, "Label (monospace)", |ui| {
        ui.label(demo::next_lined(ui, "Monospace", egui::TextStyle::Monospace).monospace())
    });
    reg.tag("basic.typography.monospace", &mono);
}

/// The Separator group.
fn separator(reg: &mut Registry, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.separator.heading", "Separator");
    ui.scope(|ui| {
        ui.set_max_width(BASIC_WIDE);
        let mut width = 0.0;
        let line = demo::scoped(
            reg,
            ui,
            Role::Separator,
            normal,
            "Separator (horizontal)",
            |ui| {
                width = ui.visuals().widgets.noninteractive.bg_stroke.width;
                // The line alone: the group's spacing is the room around it.
                ui.add(egui::Separator::default().horizontal().spacing(width))
            },
        );
        reg.name("basic.separator.line", &line);
        reg.place(
            ui,
            "basic.separator.line",
            crate::chrome::separator_line(&line, width),
        );
    });
}

/// The List group.
fn list_group(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    heading(reg, ui, "basic.list.heading", "List");
    list(reg, state, atlas, ui);
}

/// The outline a rule of the installed 0.8.0 theme `r` states for a toolbar button in `state`
/// (`toolbar.button(<state>).stroke`, its `from_state` bit set), one stroke of one colour; none
/// where its width is not positive; `None` where no rule states one, or in a form egui cannot
/// draw.
fn tool_pen(r: &ResolvedTheme, state: NextState) -> Option<egui::Stroke> {
    let part = r.toolbar.button(state);
    if !part.from_state.contains(PartStated::STROKE) {
        return None;
    }
    match (
        next::stroke_width_of(&part),
        next::state_stroke_color_of(&part),
    ) {
        (Some(width), _) if width <= 0.0 => Some(egui::Stroke::NONE),
        (Some(width), Some(colour)) => Some(egui::Stroke::new(width, colour)),
        _ => None,
    }
}

/// The Icons group's first row: four icon-only tool buttons, as the toolbar's
/// (`crate::chrome`'s toolbar): the chosen set's Copy, Paste, Delete and Edit at
/// `toolbar.button_icon(NONE).size` of the installed 0.8.0 theme ([`demo::tool_icon_size`]; where it
/// states none, each button's label in place of its icon, which its Widget Info names), in the
/// toolbar's scope, its items the scope's `item_spacing.x` apart (`toolbar.item_gap`, else the
/// connector's `layout.widget_gap`, else egui's own), each a tool button
/// ([`demo::next_tool_button`]). Edit is an edit-mode switch (`DemoState::basic_edit`), on at
/// start: while on it is egui's selected button (`Button::selected`) in the colours of
/// [`edit_look`], each one the theme leaves unstated egui's own, which its Widget Info names; and
/// lined in the pen a rule states for the checked state at rest (`toolbar.button(CHECKED)`),
/// under the pointer (`CHECKED | HOVER`) and pressed (`CHECKED | PRESSED`), each unstated one the
/// scope's own ([`tool_pen`]).
fn icon_buttons(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let r = atlas.next_shown(ui.ctx());
    let size = demo::tool_icon_size(r);
    let padding = demo::tool_padding(r);
    let (set, icon_theme) = chosen;
    let image_at = |ui: &egui::Ui, role| {
        size.and_then(|size| demo::role_image(ui, role, *set, icon_theme.as_deref(), size))
    };
    let unsized_note = |reg: &mut Registry| {
        if size.is_none() {
            reg.note_last(
                "icon size",
                "not stated by the theme (toolbar.button_icon(NONE).size): the button's label \
                 in place of its icon",
            );
        }
    };
    demo::scoped_container(
        reg,
        ui,
        Role::Toolbar,
        RoleVariant::Normal,
        "icon buttons",
        |ui, bar, reg| {
            // A line as tall as its buttons, and the lines as far apart as the buttons, as the
            // gpui (`flex_wrap` under one gap) and iced (`Row::wrap`) rows lay theirs out: the
            // toolbar scope's `interact_size.y` is `toolbar.bar_height`, a bar's, which this row
            // of tool buttons is not, and a wrapping row takes its first line's height from it
            // when it opens (`egui/src/ui.rs:2376-2379`).
            ui.spacing_mut().interact_size.y = 0.0;
            // Wrapping onto a further line where the column is too narrow for the four.
            ui.horizontal_wrapped(|ui| {
                let spacing = ui.spacing_mut();
                spacing.item_spacing.y = spacing.item_spacing.x;
                for (role, label, id) in [
                    (IconRole::ActionCopy, "Copy", "basic.icons.copy"),
                    (IconRole::ActionPaste, "Paste", "basic.icons.paste"),
                    (IconRole::ActionDelete, "Delete", "basic.icons.delete"),
                ] {
                    let image = image_at(ui, role);
                    let drawn = image.is_some();
                    wrap_before(ui, id);
                    let response = ui
                        .scope(|ui| {
                            // One line, as `text_button`'s label.
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                            demo::next_tool_button(ui, padding);
                            bar.add(reg, ui, "icon button", |ui| {
                                let button = match image {
                                    Some(image) => Button::image(image),
                                    None => Button::new(label),
                                };
                                let r = ui.add(button);
                                r.widget_info(|| {
                                    egui::WidgetInfo::labeled(egui::Role::Button, true, label)
                                });
                                r
                            })
                        })
                        .inner;
                    reg.ghost_last();
                    unsized_note(reg);
                    reg.tag(id, &response);
                    remember_width(ui, id, response.rect);
                    if let Some(size) = size.filter(|_| drawn) {
                        crate::chrome::place_icon(reg, ui, id, &response, size);
                    }
                }
                // The edit-mode switch: the one toolbar button that is ever checked.
                let id = "basic.icons.edit";
                let on = state.basic_edit;
                let look = edit_look(r);
                let mut drawn = false;
                wrap_before(ui, id);
                let response = ui
                    .scope(|ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        demo::next_tool_button(ui, padding);
                        // E9: while on, the pens a rule states for the checked state at rest,
                        // under the pointer and pressed. `Button::fill` leaves the stroke to the
                        // scope; only `Button::stroke` overrides it
                        // (`egui-0.37.0/src/widgets/button.rs:161`).
                        if let Some(r) = r.filter(|_| on) {
                            let w = &mut ui.visuals_mut().widgets;
                            for (entry, state) in [
                                (&mut w.inactive, NextState::CHECKED),
                                (&mut w.hovered, NextState::CHECKED.union(NextState::HOVER)),
                                (&mut w.active, NextState::CHECKED.union(NextState::PRESSED)),
                            ] {
                                if let Some(pen) = tool_pen(r, state) {
                                    entry.bg_stroke = pen;
                                }
                            }
                        }
                        // The text while on, egui's selected button's where unstated, before the
                        // icon is loaded in the scope's text colour.
                        if on {
                            let own = ui.visuals().selection.stroke.color;
                            ui.visuals_mut().override_text_color = Some(look.text.unwrap_or(own));
                        }
                        let image = image_at(ui, IconRole::ActionEdit);
                        drawn = image.is_some();
                        let kind = if on {
                            "icon button, checked"
                        } else {
                            "icon button"
                        };
                        bar.add(reg, ui, kind, |ui| {
                            let button = match image {
                                Some(image) => Button::image(image),
                                None => Button::new("Edit"),
                            }
                            .selected(on);
                            let button = match look.fill.filter(|_| on) {
                                Some(fill) => button.fill(fill),
                                None => button,
                            };
                            let r = ui.add(button);
                            r.widget_info(|| {
                                egui::WidgetInfo::selected(egui::Role::Button, true, on, "Edit")
                            });
                            r
                        })
                    })
                    .inner;
                if response.clicked() {
                    state.basic_edit = !on;
                }
                // Not at rest transparent while it is on.
                if !on {
                    reg.ghost_last();
                }
                unsized_note(reg);
                if on {
                    for (what, home, unstated, own) in [
                        (
                            "fill while on",
                            "toolbar.button(CHECKED).fill",
                            look.fill.is_none(),
                            ToggleOn::OWN_FILL,
                        ),
                        (
                            "text colour while on",
                            "button.label(CHECKED).color",
                            look.text.is_none(),
                            ToggleOn::OWN_TEXT,
                        ),
                    ] {
                        if unstated {
                            reg.note_last(
                                what,
                                &format!("not stated by the theme ({home}): {own}"),
                            );
                        }
                    }
                }
                reg.tag(id, &response);
                remember_width(ui, id, response.rect);
                if let Some(size) = size.filter(|_| drawn) {
                    crate::chrome::place_icon(reg, ui, id, &response, size);
                }
            })
            .response
        },
    );
}

/// The Basic page's edit-mode switch's colours while on, of the installed 0.8.0 theme `r`, each
/// asked with the checked flag alone and `None` where no rule states it: the fill
/// `toolbar.button(CHECKED).fill`, the toolbar's checked fill, and the text colour
/// `button.label(CHECKED).color`, the checked button text (the table of homes'
/// `toolbar.button_checked_background` and `button.checked_text_color`). Neither falls back to
/// another state's: what the theme leaves unstated, and both without an installed 0.8.0 theme,
/// is egui's own selected button's, as the toggle's ([`toggle_on`], spec §10, rules 0 and 0a).
pub(crate) fn edit_look(r: Option<&ResolvedTheme>) -> ToggleOn {
    ToggleOn {
        fill: r.and_then(|r| next::state_fill_of(&r.toolbar.button(NextState::CHECKED))),
        text: r.and_then(|r| next::state_text_color_of(&r.button.label(NextState::CHECKED))),
    }
}

/// The Icons group's second row: the chosen set's open folder at the theme's three icon sizes,
/// `icons.sizes`' small, toolbar and large of the installed 0.8.0 theme `r`, never another set's;
/// a size the theme states none of is not drawn, and a line names it.
fn icons(
    reg: &mut Registry,
    r: Option<&native_theme::next::ResolvedTheme>,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    use native_theme_egui::icons::IconContext;
    let (set, icon_theme) = chosen;
    let size = |context| r.and_then(|r| next::icon_size(r, context).ok());
    let sizes = [
        (size(IconContext::Small), "small", "basic.icons.small"),
        (size(IconContext::Toolbar), "toolbar", "basic.icons.toolbar"),
        (size(IconContext::Large), "large", "basic.icons.large"),
    ];
    let unstated: Vec<&str> = sizes
        .iter()
        .filter(|(size, _, _)| size.is_none())
        .map(|(_, name, _)| *name)
        .collect();
    ui.scope(|ui| {
        // One row, as tall as the largest icon, each icon centred on it: `ui.horizontal`
        // centres on the row's `interact_size.y` (`egui/src/ui.rs:2376-2379`).
        ui.spacing_mut().interact_size.y = sizes
            .iter()
            .filter_map(|(size, _, _)| *size)
            .fold(0.0, f32::max);
        // Wrapping onto a further line where the column is too narrow for the three.
        ui.horizontal_wrapped(|ui| {
            for (size, _, id) in sizes {
                let Some(size) = size else {
                    continue;
                };
                let Some(image) =
                    demo::role_image(ui, IconRole::FolderOpen, *set, icon_theme.as_deref(), size)
                else {
                    continue;
                };
                wrap_before(ui, id);
                // Drawn in the base style it is recorded with: the row's own height above is
                // the row's alone, not the icon's.
                let icon = ui
                    .scope(|ui| {
                        *ui.style_mut() = ui.ctx().global_style().as_ref().clone();
                        demo::base(reg, ui, "Image · icon", |ui| ui.add(image))
                    })
                    .inner;
                reg.tag(id, &icon);
                remember_width(ui, id, icon.rect);
            }
        });
    });
    if !unstated.is_empty() {
        let names = unstated.join(", ");
        demo::base(reg, ui, "Label · no icon size", |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "The theme states no icons.sizes {names}: those icons are not drawn"
                ))
                .small()
                .weak(),
            )
        });
    }
}

/// The Icons group: the icon buttons, then the icons at the theme's three sizes.
fn icons_group(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    heading(reg, ui, "basic.icons.heading", "Icons");
    icon_buttons(reg, state, atlas, ui, chosen);
    icons(reg, atlas.next_shown(ui.ctx()), ui, chosen);
}

/// The Expander group.
fn expander(reg: &mut Registry, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    // The companion crate's expander (docs/archive/todo_egui-widgets-spec.md §4.10): the arrow in
    // `expander.arrow_color` at `expander.arrow_icon_size` on `expander.arrow_side`,
    // `arrow_gap` from the title, the body `content_indent` in, framed as `frame_enabled`
    // states; the header `expander.header_height` tall (the expander scope's
    // `interact_size.y`) at a text scale of 1 or less, its title one line, and above 1 grown to
    // hold its scaled line box and padding, the one rule every stated row height takes (M9-150).
    heading(reg, ui, "basic.expander.heading", "Expander");
    ui.scope(|ui| {
        ui.set_max_width(BASIC_WIDE);
        for (title, body, open, kind, id) in [
            (
                "Details",
                "Expanded content",
                true,
                "Expander (expanded)",
                "basic.expander.details",
            ),
            (
                "More",
                "More content",
                false,
                "Expander (collapsed)",
                "basic.expander.more",
            ),
        ] {
            let mut shown = None;
            let header = demo::scoped(reg, ui, Role::Expander, normal, kind, |ui| {
                let out = Expander::new(demo::next_lined(ui, title, egui::TextStyle::Button))
                    .id_salt(("basic/expander", title))
                    .default_open(open)
                    .show(ui, |ui| {
                        ui.label(demo::next_lined(ui, body, egui::TextStyle::Body))
                    });
                shown = out.body_returned;
                out.header_response
            });
            reg.name(id, &header);
            // The expander: its frame where `frame_enabled` draws one, else its header and, while
            // open, its body's text, `content_indent` in.
            let frame = Parts::of(&header).and_then(|parts| parts.get("frame"));
            let whole = frame.unwrap_or_else(|| {
                shown
                    .as_ref()
                    .map_or(header.rect, |label| header.rect.union(label.rect))
            });
            reg.place(ui, id, whole);
            let names = [("header", "header"), ("arrow", "arrow"), ("title", "title")];
            place_parts(reg, ui, id, &header, &names);
            // The body's label, laid out in the expander's scope, recorded once it is drawn.
            if let Some(label) = shown {
                reg.place(ui, &format!("{id}.body"), label.rect);
                reg.record(
                    &label,
                    demo::info(
                        "expander body",
                        vec![demo::Seam::Role(Role::Expander, normal)],
                    ),
                    false,
                );
                reg.name(id, &label);
            }
        }
    });
}

/// The Table group.
fn table_group(reg: &mut Registry, atlas: &OldAtlas, ui: &mut egui::Ui) {
    heading(reg, ui, "basic.table.heading", "Table");
    table(reg, atlas, ui);
}

/// The Card group: the card surface's frame (`card.container(NONE)`),
/// `BASIC_WIDE` across its border.
fn card(reg: &mut Registry, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.card.heading", "Card");
    // A side of `card.container(NONE)`'s padding the installed 0.8.0 theme leaves unstated is
    // `layout.container_margin`, inside its border (R11, D-card: KDE and macOS state no card
    // padding, the card pads by the container margin), where the surface's frame would keep
    // egui's own; the border is the stated stroke width, else the frame's own. Without an
    // installed 0.8.0 theme, or a container margin, the surface's frame as it is.
    let pad_unstated = |frame: &mut egui::Frame| {
        let Some((margin, card)) = r.and_then(|r| {
            Some((
                r.layout.container_margin?,
                r.card.container(NextState::NONE),
            ))
        }) else {
            return;
        };
        let width = next::stroke_width_of(&card).unwrap_or(frame.stroke.width);
        let [top, right, bottom, left] =
            next::padding_of(&card).map(|stated| stated.is_none().then_some(margin + width));
        let unstated = native_theme::theme::ResolvedPadding {
            top,
            right,
            bottom,
            left,
        };
        frame.inner_margin = to_margin(frame.inner_margin, &unstated);
    };
    ui.scope(|ui| {
        ui.set_max_width(BASIC_WIDE);
        let card = demo::framed_with(
            reg,
            ui,
            (Surface::Card, pad_unstated),
            Some((Role::Card, normal)),
            "card",
            |ui, reg| {
                // The room inside the frame: `BASIC_WIDE` less its margins and border.
                ui.set_min_width(ui.available_width());
                let text = demo::scoped(reg, ui, Role::Card, normal, "card label", |ui| {
                    ui.label(demo::next_lined(ui, "Card content", egui::TextStyle::Body))
                });
                reg.tag("basic.card.text", &text);
            },
        );
        reg.tag("basic.card.frame", &card.response);
    });
}

/// The query of a row of a list of `variant`, the Basic list's `Plain` or the table's `Table`,
/// in `state`: a row of either is never an alternate one, as neither is striped.
fn list_query(variant: ListVariant, state: NextState) -> ListQuery {
    ListQuery::new(state).variant(ListKind {
        variant,
        alternate: false,
    })
}

/// The frame a list of `variant` draws, of the installed 0.8.0 theme `r`:
/// `list.container(NONE)`'s fill, stroke and corners, each where the theme states it, else
/// egui's own, a frame with none (`Frame::NONE`), and all of them without an installed 0.8.0
/// theme; no padding of its own (a row pads its text). With it, the radius of its inside, its
/// corners' radius less its stroke's width (egui lays a frame's content inside its stroke, and
/// the frame has no margin), none where that is not a positive length.
fn list_frame(r: Option<&ResolvedTheme>, variant: ListVariant) -> (egui::Frame, f32) {
    let mut frame = egui::Frame::NONE;
    let own = f32::from(frame.corner_radius.nw);
    let Some(r) = r else {
        return (frame, own);
    };
    let part = r.list.container(list_query(variant, NextState::NONE));
    if let Some(fill) = next::fill_of(&part) {
        frame = frame.fill(fill);
    }
    let width = next::stroke_width_of(&part);
    if let (Some(width), Some(colour)) = (width, next::stroke_color_of(&part)) {
        frame = frame.stroke(egui::Stroke::new(width, colour));
    }
    let radius = next::radius_of(&part).unwrap_or(own);
    frame = frame.corner_radius(to_corner_radius(frame.corner_radius, radius));
    (frame, (radius - frame.stroke.width).max(own))
}

/// `inner`, the radius of [`list_frame`]'s inside, on the corners `[nw, ne, sw, se]` marks,
/// square on the others: a fill where a row meets the frame keeps inside the frame's arcs.
fn inner_corners(inner: f32, [nw, ne, sw, se]: [bool; 4]) -> egui::CornerRadius {
    let radius = to_corner_radius(egui::CornerRadius::ZERO, inner);
    let at = |rounded: bool, r: u8| if rounded { r } else { 0 };
    egui::CornerRadius {
        nw: at(nw, radius.nw),
        ne: at(ne, radius.ne),
        sw: at(sw, radius.sw),
        se: at(se, radius.se),
    }
}

/// A list or table row's height, its text's inset and its line box, of the installed 0.8.0
/// theme `r`, for a body row or, `header`, the table's header, of a list of `variant`: the
/// stated height, `list.row(NONE).min_size[1]` (the header's `list.header(NONE).min_size[1]`,
/// else the row's, as a header the theme gives no height is a row), grown above a text scale of
/// 1 to hold the row's line, its padding and `rule`, the width of a line drawn inside it
/// (`demo::next_grown`, as gpui's table and list rows grow); else as tall as its content, the
/// platform's "sizes to content" (docs/platform-facts.md §2.15): one line box of `font`, the
/// font as drawn, in `list.item_label(NONE)` (`list.header_label(NONE)`), `demo::next_line_box`,
/// and the part's padding above and below it, its text the part's left padding in. A side the
/// theme leaves unstated is `own`'s: for the list what egui gives a selectable row there, the
/// scope's `button_padding` (`egui/src/widgets/button.rs:333-337`); for the table nothing, as
/// the three showcases lay their tables out. Without an installed 0.8.0 theme, egui's own line,
/// the font's row height, padded by `own`.
fn row_metrics(
    ui: &egui::Ui,
    r: Option<&ResolvedTheme>,
    (variant, header): (ListVariant, bool),
    font: &egui::FontId,
    own: egui::Vec2,
    rule: f32,
) -> (f32, f32, f32) {
    let (height, inset, line) = match r {
        None => {
            let line = ui.ctx().fonts_mut(|f| f.row_height(font));
            (line + own.y + own.y, own.x, line)
        }
        Some(r) => {
            let l = &r.list;
            let q = list_query(variant, NextState::NONE);
            let [_, row_height] = l.row(q).min_size;
            let (part, label, stated) = if header {
                let part = l.header(q);
                let stated = part.min_size[1].or(row_height);
                (part, l.header_label(q), stated)
            } else {
                (l.row(q), l.item_label(q), row_height)
            };
            let [top, _, bottom, left] = next::padding_of(&part);
            let padding_y = top.unwrap_or(own.y) + bottom.unwrap_or(own.y);
            let line = demo::next_line_box(r, &label, font.size);
            let height = match stated {
                Some(h) => {
                    let at_size = demo::next_line_box(r, &label, label.size_px);
                    demo::next_grown(r, h, at_size, padding_y, rule)
                }
                None => line + padding_y,
            };
            (height, left.unwrap_or(own.x), line)
        }
    };
    // On egui's layout grid (`emath::GUI_ROUNDING`), as `allocate_exact_size` lays each row out,
    // so `n` rows are `n` row heights tall.
    (egui::emath::GuiRounding::round_ui(height), inset, line)
}

/// egui's own selectable row in `ui`, the list's scope, as `Button::selectable` draws one: the
/// widget visuals of the row's interaction, `response`'s (`Widgets::style`; a painted row
/// without one, the table's, at rest), with `selection.bg_fill` and `selection.stroke` while
/// `selected` (egui-0.37.0 `src/style.rs`, `Style::interact_selectable`, as
/// `src/theme/default_style.rs` lays them on a selected button); its frame, the fill and
/// stroke, hidden at rest while not selected (`Button::frame_when_inactive(selected)`,
/// `src/widgets/button.rs`). Its fill, stroke (none where egui paints nothing of it) and text
/// colour.
fn own_row(
    ui: &egui::Ui,
    selected: bool,
    response: Option<&egui::Response>,
) -> (Option<egui::Color32>, egui::Stroke, egui::Color32) {
    let visuals = &ui.style().visuals;
    let widgets = &visuals.widgets;
    let at = response.map_or(&widgets.inactive, |response| widgets.style(response));
    let mut v = *at;
    if selected {
        v.weak_bg_fill = visuals.selection.bg_fill;
        v.fg_stroke = visuals.selection.stroke;
    }
    if !selected && std::ptr::eq(at, &widgets.inactive) {
        (None, egui::Stroke::NONE, v.text_color())
    } else {
        // a line egui paints nothing of (no width, or transparent) is no line
        let line = if v.bg_stroke.width > 0.0 && v.bg_stroke.color.a() > 0 {
            v.bg_stroke
        } else {
            egui::Stroke::NONE
        };
        (Some(v.weak_bg_fill), line, v.text_color())
    }
}

/// One property of a row in a state: `stated`, what a rule states for the state, where one
/// does; else egui's own row's value in the state (`own.1`) where it differs from egui's own
/// row at rest (`own.0`), and `rest`, the row's value at rest, where egui makes no change
/// (spec §10, rule 0).
fn in_state<T: PartialEq>(stated: Option<T>, rest: T, own: (T, T)) -> T {
    match stated {
        Some(value) => value,
        None if own.1 == own.0 => rest,
        None => own.1,
    }
}

/// What a list or table row draws, of the installed 0.8.0 theme: the row's own fill under its
/// highlight, `list.row(<state>).fill` where the theme states it (`None` for none, egui's own
/// selectable row filling nothing under its frame); its highlight's fill (`None` for none) and
/// stroke, its text colour, the highlight's margin, top, right, bottom and left, each where
/// stated, and its corners' radius where stated; and the properties egui's own selectable row
/// gives it, for its Widget Info.
pub(crate) struct RowLook {
    pub(crate) row: Option<egui::Color32>,
    pub(crate) fill: Option<egui::Color32>,
    pub(crate) stroke: egui::Stroke,
    pub(crate) text: egui::Color32,
    pub(crate) margin: [Option<f32>; 4],
    pub(crate) corners: Option<f32>,
    pub(crate) own: Vec<&'static str>,
}

/// What a row's *Not themeable* line names, where egui's own row gives it a property.
pub(crate) const OWN_ROW: &str = "highlight where unstated";

impl RowLook {
    /// The look of a row of a list of `variant` in `ui`, its `Role::List` scope, of the
    /// installed 0.8.0 theme `r` (spec §5, the list's `row` and `highlight`): in every state the
    /// row's fill `list.row(<state>).fill` where the theme states it, under the highlight; not
    /// selected and not under
    /// the pointer, no highlight, its text `list.item_label(<view>).color`, else egui's own;
    /// selected or under the pointer, `list.highlight(<state>)`'s fill and stroke and
    /// `list.item_label(<state>).color` where a rule states them for the state (spec §2,
    /// `from_state`; the state `SELECTED` while `selected`, `HOVER` too while `response` is
    /// hovered, `UNFOCUSED_VIEW` too while the window is not active, egui's
    /// `InputState::focused`, as every view of an inactive window lacks the keyboard focus),
    /// each one left unstated egui's own selectable row's ([`own_row`]) where egui changes it
    /// from its row at rest, else the row's at rest ([`in_state`]); the margin and corners of
    /// the highlight in that state. Without an installed 0.8.0 theme, egui's own row.
    pub(crate) fn of(
        ui: &egui::Ui,
        r: Option<&ResolvedTheme>,
        variant: ListVariant,
        selected: bool,
        response: Option<&egui::Response>,
    ) -> Self {
        let rest = own_row(ui, false, None);
        let own = own_row(ui, selected, response);
        let Some(r) = r else {
            return Self {
                row: None,
                fill: own.0,
                stroke: own.1,
                text: own.2,
                margin: [None; 4],
                corners: None,
                own: vec!["fill", "stroke", "text colour"],
            };
        };
        let l = &r.list;
        let view = if ui.ctx().input(|input| input.focused) {
            NextState::NONE
        } else {
            NextState::UNFOCUSED_VIEW
        };
        let label = next::text_color_of(&l.item_label(list_query(variant, view)));
        let hovered = response.is_some_and(egui::Response::hovered);
        let mut state = view;
        if selected {
            state = state.union(NextState::SELECTED);
        }
        if hovered {
            state = state.union(NextState::HOVER);
        }
        let q = list_query(variant, state);
        let row = next::fill_of(&l.row(q));
        let part = l.highlight(q);
        let margin = {
            let side = |bit: PartStated, v: f32| part.stated.contains(bit).then_some(v);
            let m = &part.margin;
            [
                side(PartStated::MARGIN_TOP, m.top),
                side(PartStated::MARGIN_RIGHT, m.right),
                side(PartStated::MARGIN_BOTTOM, m.bottom),
                side(PartStated::MARGIN_LEFT, m.left),
            ]
        };
        let corners = next::radius_of(&part);
        if !selected && !hovered {
            return Self {
                row,
                fill: None,
                stroke: egui::Stroke::NONE,
                text: label.unwrap_or(rest.2),
                margin,
                corners,
                own: if label.is_some() {
                    Vec::new()
                } else {
                    vec!["text colour"]
                },
            };
        }
        let fill = next::state_fill_of(&part);
        let stroke = if part.from_state.contains(PartStated::STROKE) {
            match (
                next::stroke_width_of(&part),
                next::state_stroke_color_of(&part),
            ) {
                (Some(width), _) if width <= 0.0 => Some(egui::Stroke::NONE),
                (Some(width), Some(colour)) => Some(egui::Stroke::new(width, colour)),
                _ => None,
            }
        } else {
            None
        };
        let text = next::state_text_color_of(&l.item_label(q));
        let mut taken = Vec::new();
        for (name, unstated, changed) in [
            ("fill", fill.is_none(), rest.0 != own.0),
            ("stroke", stroke.is_none(), rest.1 != own.1),
            ("text colour", text.is_none(), rest.2 != own.2),
        ] {
            if unstated && changed {
                taken.push(name);
            }
        }
        Self {
            row,
            fill: in_state(fill.map(Some), None, (rest.0, own.0)),
            stroke: in_state(stroke, egui::Stroke::NONE, (rest.1, own.1)),
            text: in_state(text, label.unwrap_or(rest.2), (rest.2, own.2)),
            margin,
            corners,
            own: taken,
        }
    }

    /// `row`, a row's rectangle, inset by the highlight's margin on each side the theme
    /// states it, as wide and as tall as `row` on a side it leaves unstated, as egui's own
    /// selectable row fills its whole rectangle; no less than empty.
    pub(crate) fn highlight_box(&self, row: egui::Rect) -> egui::Rect {
        let [top, right, bottom, left] = self.margin;
        let mut inset = row;
        if let Some(top) = top {
            inset.min.y += top;
        }
        if let Some(right) = right {
            inset.max.x -= right;
        }
        if let Some(bottom) = bottom {
            inset.max.y -= bottom;
        }
        if let Some(left) = left {
            inset.min.x += left;
        }
        inset.max = inset.max.max(inset.min);
        inset
    }

    /// The *Not themeable* line naming what egui's own row gives this one; `None` where the
    /// theme states all of it.
    fn own_note(&self) -> Option<String> {
        (!self.own.is_empty()).then(|| {
            format!(
                "{}: egui's own selectable row's, Style::interact_selectable \
                 (egui-0.37.0 src/style.rs), where egui changes it from the row at rest",
                self.own.join(", ")
            )
        })
    }
}

/// The list: `LIST_ROWS` selectable rows in a `ScrollArea` `LIST_VISIBLE` rows tall, framed by
/// `list_frame`, in one `Role::List` scope (its selection colours and item font), of the
/// installed 0.8.0 theme. egui has no list widget, and the leaves egui never reads from the
/// style are per call (connector spec §5.4, T18(a)): each row is painted here, `row_metrics`
/// tall and inset, in its `RowLook` (spec §5, the list's `row` and `highlight`: the row's fill
/// where stated, then its highlight inset by its margin), a property the theme leaves unstated
/// egui's own selectable
/// row's, which the row's Widget Info names.
fn list(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let mut rows_drawn: Vec<egui::Response> = Vec::new();
    let mut labels_drawn: Vec<egui::Rect> = Vec::new();
    let mut bar = None;
    let frame_response = demo::scoped_container(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "List",
        |ui, rows, reg| {
            let font = egui::TextStyle::Body.resolve(ui.style());
            let (row_height, inset, line) = row_metrics(
                ui,
                r,
                (ListVariant::Plain, false),
                &font,
                ui.spacing().button_padding,
                0.0,
            );
            let (frame, inner_radius) = list_frame(r, ListVariant::Plain);
            let inner = (BASIC_WIDE - frame.total_margin().sum().x).max(0.0);
            let visible = row_height * LIST_VISIBLE as f32;
            frame
                .show(ui, |ui| {
                    ui.set_width(inner);
                    // No fade over the rows at the list's edges: egui fades a scroll area's
                    // content toward an edge with more beyond it (`ScrollFadeStyle`, strength
                    // 0.5 over 20 points, `egui/src/style.rs:779-802`,
                    // `egui/src/containers/scroll_area.rs:1562-1568`), which repaints the rows'
                    // `list.item_font.color`; the theme states no such effect.
                    ui.spacing_mut().scroll.fade.strength = 0.0;
                    let room = ui.max_rect();
                    // The frame's inside: the scroll area fills it, `visible` tall.
                    let edges = egui::Rect::from_min_size(room.min, egui::vec2(inner, visible));
                    let area = egui::ScrollArea::vertical()
                        .id_salt("basic/list")
                        .auto_shrink([false, false])
                        .min_scrolled_height(visible)
                        .max_height(visible)
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for i in 0..LIST_ROWS {
                                let text = format!("Item {}", i.saturating_add(1));
                                let selected = state.basic_list == i;
                                let mut note = None;
                                let row = rows.add(reg, ui, "List row", |ui| {
                                    let (row, label, own) = list_row(
                                        ui,
                                        r,
                                        (edges, inner_radius),
                                        &text,
                                        selected,
                                        (row_height, inset, line),
                                        &font,
                                    );
                                    labels_drawn.push(label);
                                    note = own;
                                    row
                                });
                                if let Some(note) = note {
                                    reg.note_last(OWN_ROW, &note);
                                }
                                if row.clicked() {
                                    state.basic_list = i;
                                }
                                rows_drawn.push(row);
                            }
                        });
                    bar = scroll_bar(ui, &area, room);
                })
                .response
        },
    );
    if let Some((groove, thumb)) = bar {
        reg.place(ui, "basic.list.scrollbar", groove);
        reg.place(ui, "basic.list.scrollbar.thumb", thumb);
    }
    reg.tag("basic.list.frame", &frame_response);
    for (i, (row, label)) in rows_drawn
        .iter()
        .zip(&labels_drawn)
        .take(LIST_VISIBLE)
        .enumerate()
    {
        let id = format!("basic.list.row_{}", i.saturating_add(1));
        reg.place(ui, &id, row.rect);
        reg.place(ui, &format!("{id}.text"), *label);
    }
}

/// Where a vertical `ScrollArea` of `ui` whose output is `area`, laid out in `room`, painted its
/// solid bar at rest (`egui/src/containers/scroll_area.rs:1294-1400`): the groove from the
/// content's right edge to `room`'s, as tall as the content's view, and the thumb across the
/// groove less the bar's inner and outer margins, as long as the view's share of the content
/// and at least `handle_min_length`, at the top while nothing is scrolled. `None` for a
/// floating bar, which is not shown at rest.
fn scroll_bar(
    ui: &egui::Ui,
    area: &egui::scroll_area::ScrollAreaOutput<()>,
    room: egui::Rect,
) -> Option<(egui::Rect, egui::Rect)> {
    let style = &ui.spacing().scroll;
    if style.floating {
        return None;
    }
    let view = area.inner_rect;
    let groove = egui::Rect::from_x_y_ranges(view.right()..=room.right(), view.y_range());
    let cross = (view.right() + style.bar_inner_margin)..=(room.right() - style.bar_outer_margin);
    let content = area.content_size.y;
    let share = if content > 0.0 {
        view.height() * (view.height() / content).min(1.0)
    } else {
        view.height()
    };
    let length = share.max(style.handle_min_length);
    let offset = area.state.clamped_offset().y;
    let travel = (view.height() - length).max(0.0);
    let scrollable = (content - view.height()).max(0.0);
    let start = if scrollable > 0.0 {
        view.top() + travel * (offset / scrollable).clamp(0.0, 1.0)
    } else {
        view.top()
    };
    let thumb = egui::Rect::from_x_y_ranges(cross, start..=start + length);
    Some((groove, thumb))
}

/// What the list's view, `ui`'s clip, shows of `b`, a box across a row, rounded by `inner`, the
/// frame's inner radius, at each corner where it meets a corner of `edges`, the frame's inside,
/// square elsewhere; `None` where the view shows none of it.
fn in_view(
    ui: &egui::Ui,
    b: egui::Rect,
    (edges, inner): (egui::Rect, f32),
) -> Option<(egui::Rect, egui::CornerRadius)> {
    let view = ui.clip_rect();
    let shown = egui::Rect::from_x_y_ranges(
        b.x_range(),
        b.top().max(view.top())..=b.bottom().min(view.bottom()),
    );
    if !shown.is_positive() {
        return None;
    }
    // The scroll area lays the rows out on whole pixels and then on egui's layout grid
    // (`egui/src/containers/scroll_area.rs:797-803`; `emath/src/gui_rounding.rs:58-70`), which
    // moves each edge by at most half a pixel and half a grid step from the frame's inside: half
    // a point under a 0.5 pt line at one pixel per point. A row is far taller, and a solid bar's
    // groove far wider, than that.
    let off = 0.5 / ui.pixels_per_point() + 0.5 * egui::emath::GUI_ROUNDING;
    let meets = |a: f32, b: f32| (a - b).abs() <= off;
    let top = meets(shown.top(), edges.top());
    let bottom = meets(shown.bottom(), edges.bottom());
    let left = meets(shown.left(), edges.left());
    let right = meets(shown.right(), edges.right());
    let corners = inner_corners(
        inner,
        [top && left, top && right, bottom && left, bottom && right],
    );
    Some((shown, corners))
}

/// One list row across the list, `height` tall, its text `inset` in, one `line` tall and
/// centred vertically, in its [`RowLook`]: first the row's own fill where the theme states one,
/// across the row as the view shows it ([`in_view`]); then its highlight, the row inset by the
/// highlight's margin ([`RowLook::highlight_box`]); where the highlight's corners are stated,
/// the whole box is filled and outlined, rounded by them, and the scroll area's clip, the view,
/// cuts it, as Breeze clips a row to the view; elsewhere what the view shows of the box is
/// ([`in_view`]). Its response, its text's rectangle, and its *Not themeable* line where egui's
/// own row gives it a property.
fn list_row(
    ui: &mut egui::Ui,
    r: Option<&ResolvedTheme>,
    (edges, inner): (egui::Rect, f32),
    text: &str,
    selected: bool,
    (height, inset, line): (f32, f32, f32),
    font: &egui::FontId,
) -> (egui::Response, egui::Rect, Option<String>) {
    let size = egui::vec2(ui.available_width(), height);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    response.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, selected, text));
    let look = RowLook::of(ui, r, ListVariant::Plain, selected, Some(&response));
    // The row first, square across its whole rectangle as Breeze paints it, then its highlight.
    if let Some(fill) = look.row
        && let Some((shown, corners)) = in_view(ui, rect, (edges, inner))
    {
        ui.painter().rect_filled(shown, corners, fill);
    }
    let lit = look.highlight_box(rect);
    let drawn = look.fill.is_some() || look.stroke.width > 0.0;
    if drawn && let Some(radius) = look.corners {
        let corners = to_corner_radius(egui::CornerRadius::ZERO, radius);
        if let Some(fill) = look.fill {
            ui.painter().rect_filled(lit, corners, fill);
        }
        if look.stroke.width > 0.0 {
            ui.painter()
                .rect_stroke(lit, corners, look.stroke, egui::StrokeKind::Inside);
        }
    } else if drawn && let Some((shown, corners)) = in_view(ui, lit, (edges, inner)) {
        if let Some(fill) = look.fill {
            ui.painter().rect_filled(shown, corners, fill);
        }
        // The row's outline on the fill's shape, inside it, as Breeze's line is
        // (`egui-0.37.0/src/painter.rs:406-414`, `epaint-0.37.0/src/stroke.rs:102-111`), clipped
        // to the view as the fill is: a row partly scrolled out draws no line along the view's
        // edge. The line is on the fill's shape, its arcs on the fill's; on a side the view cuts
        // the clip stops one line width short of the fill, so the line along that edge is not
        // drawn.
        let stroke = look.stroke;
        if stroke.width > 0.0 {
            let short = |cut: bool| if cut { stroke.width } else { 0.0 };
            let clip = egui::Rect::from_x_y_ranges(
                shown.x_range(),
                shown.top() + short(lit.top() < shown.top())
                    ..=shown.bottom() - short(lit.bottom() > shown.bottom()),
            );
            ui.painter().with_clip_rect(clip).rect_stroke(
                shown,
                corners,
                stroke,
                egui::StrokeKind::Inside,
            );
        }
    }
    let label = paint_cell_text(ui, text, font, look.text, (rect, inset, line));
    (response, label, look.own_note())
}

/// `text` painted in `font` and `colour`, one `line` tall, `inset` in from `rect`'s left and
/// centred on its height; its rectangle.
fn paint_cell_text(
    ui: &egui::Ui,
    text: &str,
    font: &egui::FontId,
    colour: egui::Color32,
    (rect, inset, line): (egui::Rect, f32, f32),
) -> egui::Rect {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), font.clone(), colour);
    if let Some(section) = job.sections.first_mut() {
        section.format.line_height = Some(line);
    }
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    let at = egui::pos2(rect.left() + inset, rect.center().y - 0.5 * line);
    let cell = egui::Rect::from_min_size(at, egui::vec2(galley.size().x, line));
    ui.painter().galley(at, galley, colour);
    cell
}

/// A table header section's fill, of the installed 0.8.0 theme `r`: pressed, which needs the
/// pointer on it, `list.header(HOVER | PRESSED).fill`, hovered `list.header(HOVER).fill`, each
/// where a rule states it for the state (spec §2, `from_state`), of the table
/// (`ListVariant::Table`); `None` at rest, where no rule states it, and without an installed
/// 0.8.0 theme: egui's own header has no hover (`egui_extras/src/table.rs:490-506`).
pub(crate) fn header_section_fill(
    r: Option<&ResolvedTheme>,
    hovered: bool,
    pressed: bool,
) -> Option<egui::Color32> {
    let state = if pressed {
        NextState::HOVER.union(NextState::PRESSED)
    } else if hovered {
        NextState::HOVER
    } else {
        return None;
    };
    next::state_fill_of(&r?.list.header(list_query(ListVariant::Table, state)))
}

/// The table: a header and two rows, the second selected, `BASIC_WIDE` across the list's
/// frame (`list_frame`), the two columns half the width inside it each, in one `Role::List`
/// scope, of the installed 0.8.0 theme, a table's (`ListVariant::Table`). egui has no table the
/// theme reaches (egui_extras' `TableBuilder` paints only its stripes), so it is painted here as
/// the list's rows are: the header in `list.header(NONE).fill` where stated, its titles in
/// `list.header_label(NONE)` (the Data page's `TableHeader`), `row_metrics` tall and inset, the
/// rows `row_metrics` tall, each in its `RowLook` (the row's own fill where stated, then over it
/// the selected one's highlight inset by its margin and, where stated, rounded by its corners),
/// the header's fill rounded at the top and
/// the last row's highlight at the bottom where they meet the frame and the highlight states no
/// corners (`inner_corners`); each header section hovered and pressed in its fill where a rule
/// states it (`header_section_fill`), rounded where the section meets the frame's top corner; a
/// vertical `list.grid_color` line between the columns through the header and the rows and a
/// horizontal one under the header, each `separator.thickness` wide, inside the cell it closes,
/// the header's share of them in `list.header_separator(NONE).fill` where the theme states it
/// (E7). Without an installed 0.8.0 theme, egui's own: no fills and no grid.
fn table(reg: &mut Registry, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let table = ListVariant::Table;
    let mut places: Vec<(String, egui::Rect)> = Vec::new();
    let mut own_notes: Vec<String> = Vec::new();
    let frame_response = demo::scoped_container(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "Table",
        |ui, _rows, _reg| {
            let item_font = egui::TextStyle::Body.resolve(ui.style());
            let head = TableHeader::of(ui, r);
            let rest = list_query(table, NextState::NONE);
            let grid = r.map(|r| {
                egui::Stroke::new(r.separator.thickness, demo::next_color32(r.list.grid_color))
            });
            let grid_width = grid.map_or(egui::Stroke::NONE.width, |s| s.width);
            let header_fill = r.and_then(|r| next::fill_of(&r.list.header(rest)));
            let header_line_colour = r.and_then(|r| next::fill_of(&r.list.header_separator(rest)));
            let (row_height, inset, line) =
                row_metrics(ui, r, (table, false), &item_font, egui::Vec2::ZERO, 0.0);
            // the grid's line under the header lies inside it
            let (header_height, header_inset, header_line) = row_metrics(
                ui,
                r,
                (table, true),
                &head.font,
                egui::Vec2::ZERO,
                grid_width,
            );
            let (frame, inner_radius) = list_frame(r, table);
            let inner = (BASIC_WIDE - frame.total_margin().sum().x).max(0.0);
            frame
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    let rows = 1.0 + TABLE_ROWS.len() as f32;
                    let height = header_height + (rows - 1.0) * row_height;
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(inner, height), egui::Sense::hover());
                    let mid = rect.left() + 0.5 * inner;
                    let painter = ui.painter();
                    let header =
                        egui::Rect::from_min_size(rect.min, egui::vec2(inner, header_height));
                    if let Some(fill) = header_fill {
                        painter.rect_filled(
                            header,
                            inner_corners(inner_radius, [true, true, false, false]),
                            fill,
                        );
                    }
                    places.push(("basic.table.header".to_string(), header));
                    for (i, (text, id)) in TABLE_HEADER
                        .into_iter()
                        .zip(["basic.table.header.name", "basic.table.header.size"])
                        .enumerate()
                    {
                        let cell = if i == 0 {
                            egui::Rect::from_x_y_ranges(header.left()..=mid, header.y_range())
                        } else {
                            egui::Rect::from_x_y_ranges(mid..=header.right(), header.y_range())
                        };
                        // egui_extras' header row has no hover
                        // (`egui_extras/src/table.rs:490-506`): each section is filled here
                        // (`header_section_fill`); it takes no focus, as iced's and gpui's do not
                        let response = ui.interact(
                            cell,
                            ui.scope_id().with(("table-head", i)),
                            egui::Sense::CLICK,
                        );
                        let fill = header_section_fill(
                            r,
                            response.hovered(),
                            response.is_pointer_button_down_on(),
                        );
                        if let Some(fill) = fill {
                            painter.rect_filled(
                                cell,
                                inner_corners(inner_radius, [i == 0, i == 1, false, false]),
                                fill,
                            );
                        }
                        paint_cell_text(
                            ui,
                            text,
                            &head.font,
                            head.color,
                            (cell, header_inset, header_line),
                        );
                        places.push((id.to_string(), cell));
                    }
                    for (i, cells) in TABLE_ROWS.iter().enumerate() {
                        let top = header.bottom() + i as f32 * row_height;
                        let row = egui::Rect::from_min_size(
                            egui::pos2(rect.left(), top),
                            egui::vec2(inner, row_height),
                        );
                        let look = RowLook::of(ui, r, table, i == TABLE_SELECTED, None);
                        if let Some(note) = look.own_note()
                            && !own_notes.contains(&note)
                        {
                            own_notes.push(note);
                        }
                        let last = i.saturating_add(1) == TABLE_ROWS.len();
                        // the row first, across the whole of it, then its highlight
                        if let Some(fill) = look.row {
                            painter.rect_filled(
                                row,
                                inner_corners(inner_radius, [false, false, last, last]),
                                fill,
                            );
                        }
                        let lit = look.highlight_box(row);
                        let corners = look.corners.map_or_else(
                            || inner_corners(inner_radius, [false, false, last, last]),
                            |radius| to_corner_radius(egui::CornerRadius::ZERO, radius),
                        );
                        if let Some(fill) = look.fill {
                            painter.rect_filled(lit, corners, fill);
                        }
                        if look.stroke.width > 0.0 {
                            painter.rect_stroke(
                                lit,
                                corners,
                                look.stroke,
                                egui::StrokeKind::Inside,
                            );
                        }
                        for (j, text) in cells.iter().enumerate() {
                            let cell = if j == 0 {
                                egui::Rect::from_x_y_ranges(row.left()..=mid, row.y_range())
                            } else {
                                egui::Rect::from_x_y_ranges(mid..=row.right(), row.y_range())
                            };
                            paint_cell_text(ui, text, &item_font, look.text, (cell, inset, line));
                        }
                        places.push((format!("basic.table.row_{}", i.saturating_add(1)), row));
                    }
                    // The grid: under the header, and between the columns from top to bottom,
                    // each inside the cell it closes. Over the header, in
                    // `list.header_separator(NONE).fill` where the theme states it (E7), Breeze's
                    // translucent section line laid over the header's fill: the column line is
                    // split at the line under the header, so no pixel takes that colour twice.
                    let Some(stroke) = grid else {
                        return;
                    };
                    let x = mid - 0.5 * grid_width;
                    match header_line_colour {
                        Some(c) => {
                            let head = egui::Stroke::new(grid_width, c);
                            painter.hline(
                                header.x_range(),
                                header.bottom() - 0.5 * grid_width,
                                head,
                            );
                            painter.vline(x, header.top()..=header.bottom() - grid_width, head);
                            painter.vline(x, header.bottom()..=rect.bottom(), stroke);
                        }
                        None => {
                            painter.hline(
                                header.x_range(),
                                header.bottom() - 0.5 * grid_width,
                                stroke,
                            );
                            painter.vline(x, rect.y_range(), stroke);
                        }
                    }
                })
                .response
        },
    );
    // The rows are painted, not recorded: the table's record names what egui's own row gives
    // them, as each list row's does.
    for note in &own_notes {
        reg.note_last(OWN_ROW, note);
    }
    reg.tag("basic.table.frame", &frame_response);
    for (id, rect) in places {
        reg.place(ui, &id, rect);
    }
}
