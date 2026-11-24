//! Basic: the controls the three showcases all draw, in the same order, with the same labels,
//! values and states, packed onto one screen in one to five columns, so the gpui, iced and egui
//! captures compare control by control. Each control goes through the seam its palette page
//! gives it; what egui leaves to the call site is applied per call from the theme. Every
//! element of `docs/showcase-elements.toml` the page draws is placed where it is drawn, its
//! parts too, for `--dump-layout` and Widget Info.

use egui::Button;
use egui::class::Classes;
use egui::widget_style::{ButtonStyle, CheckboxStyle, StyleArgs, TextEditStyle, WidgetState};
use native_theme::AccessibilityPreferences;
use native_theme::icons::IconRole;
use native_theme::icons::IconSet;
use native_theme::next::WidgetState as NextState;
use native_theme_egui::convert::{to_color32, to_corner_radius, to_margin, to_stroke};
use native_theme_egui::{
    Role, RoleVariant, Surface, TextRole, list_header_active_background, list_header_font,
    list_header_height, list_header_hover_background, next, selection_inactive_background,
    selection_inactive_text_color,
};
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
use native_theme_showcase_support::rows;

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

/// How many columns the page lays its groups out in, from the theme alone, as the gpui and
/// iced pages count them: the most, of five down to one, whose width,
/// `(page - (n - 1) * layout.section_gap) / n`, holds the widest fixed-width control the theme
/// states on the page — `combo_box.min_width`, the two tabs at `tab.min_width` `tab.item_gap`
/// apart with room for both of the selected tab's flares, `2 * tab.active_outward_corner_radius`
/// where `demo::tab_bar` draws them (`demo::outward_radius`), `BASIC_WIDE` and `BASIC_WIDTH` —
/// where `page` is `content_width` less `layout.window_margin` on both sides and, where
/// `scrollbar.overlay_mode` is false, the `scrollbar.groove_width` a scrolling page keeps free.
/// The column also holds `segmented`, the Segmented control's own width as egui laid it out
/// (`DemoState::segmented_width`), whose segments stay on one row: the control may reach into the
/// section gap after its column, but keeps `layout.widget_gap`, the gap the page's rows keep
/// between neighbouring widgets (Button and Primary), from the next column's left edge (M9-166);
/// in the last column it may reach into the page's right margin, `layout.window_margin`, keeping
/// `layout.widget_gap` from the content panel's edge or its scroll bar: where
/// `scrollbar.overlay_mode` is true the bar floats over that margin, its `scrollbar.groove_width`
/// taken off the room. One where not even one
/// does: the page is then wider than its area.
pub(crate) fn column_count(
    t: &native_theme_egui::ResolvedTheme,
    layout: &native_theme::theme::LayoutTheme,
    content_width: f32,
    segmented: f32,
) -> usize {
    let margin = layout.window_margin.unwrap_or_default();
    let gap = layout.section_gap.unwrap_or_default();
    let bar = &t.scrollbar;
    let gutter = if bar.overlay_mode {
        0.0
    } else {
        bar.groove_width
    };
    // a floating bar takes no width of the page, and is drawn over its right margin
    let overlay = if bar.overlay_mode {
        bar.groove_width
    } else {
        0.0
    };
    let page = content_width - margin - margin - gutter;
    let flare = demo::outward_radius(t).unwrap_or_default();
    let tabs =
        t.tab.min_width + t.tab.min_width + t.tab.item_gap.unwrap_or_default() + flare + flare;
    let widest = t
        .combo_box
        .min_width
        .max(tabs)
        .max(BASIC_WIDE)
        .max(BASIC_WIDTH);
    let widget_gap = layout.widget_gap.unwrap_or_default();
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
    let t = atlas.old_resolved(ui.ctx().theme());
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
        17 => tabs(reg, state, t, ui),
        18 => expander(reg, ui),
        _ => table_group(reg, atlas, ui),
    }
}

/// Columns of equal width, as many as [`column_count`] counts from the content panel's width,
/// `layout.section_gap` apart, the gap the count takes, and none where the theme states none,
/// as the gpui and iced pages lay theirs out (`with_gap(.., section_gap)`, `spacing(0)` where
/// unstated); each a stack of groups top-aligned, `layout.section_gap` apart and none where
/// unstated too, each group in a top-down `Ui` of its own with its rows the connector's
/// `item_spacing` apart (`layout.widget_gap`): each column is laid out in a `Ui` of its own at
/// its place, so a group wider than its column shows as the overflow it is, and the next column
/// does not move.
pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let gap = atlas.layout().section_gap.unwrap_or_default();
    let origin = ui.cursor().min;
    let count = column_count(
        atlas.old_resolved(ui.ctx().theme()),
        atlas.layout(),
        state.content_width,
        state.segmented_width,
    );
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
        // The groups follow one another `gap` apart and no more: the column adds no
        // `item_spacing` between them, each group's own `Ui` puts it between its rows, and a
        // group's heading, first in its `Ui`, adds nothing (`caption`).
        let rows = child.spacing().item_spacing;
        child.spacing_mut().item_spacing.y = 0.0;
        for (place, &ix) in members.iter().enumerate() {
            if place > 0 {
                child.add_space(gap);
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
/// `<id>.label`: `make` builds it from its label `text`, one line box tall (`demo::lined`), at
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
    // A toggle button: `Off` a button at rest, `On` the same button held on, in the colours
    // [`toggle_on`] picks, per call: egui's `selected` flag would take the suggested action's
    // `selection` colours.
    let on = toggle_on(r, &ui.visuals().widgets.active);
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
        let held = |text: egui::RichText| Button::new(text.color(on_text)).fill(on_fill);
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
    if let Some(from) = on.fill_from {
        reg.note_element(
            "basic.buttons.toggle_on",
            "fill while on",
            &format!("not stated by the theme (button.body(CHECKED).fill): {from}"),
        );
    }
    if let Some(from) = on.text_from {
        reg.note_element(
            "basic.buttons.toggle_on",
            "text colour while on",
            &format!("not stated by the theme (button.label(CHECKED).color): {from}"),
        );
    }
}

/// The Basic page's toggle button while on, each colour with what it is read from where that is
/// not the checked state's own home.
#[derive(Debug, PartialEq)]
pub(crate) struct ToggleOn {
    pub(crate) fill: egui::Color32,
    pub(crate) fill_from: Option<&'static str>,
    pub(crate) text: egui::Color32,
    pub(crate) text_from: Option<&'static str>,
}

/// The toggle button's colours while on, of the installed 0.8.0 theme `r`
/// (docs/platform-facts.md §2.3), each asked with one state flag: the fill a rule states for
/// `button.body(CHECKED)`, else the pressed one, `button.body(PRESSED)`, else the hovered one,
/// `button.body(HOVER)`, as 0.7.2 fell back from `button.checked_background`; the text colour a
/// rule states for `button.label(CHECKED)`, else `button.label(PRESSED)`'s. Each the theme leaves
/// unstated, and both without an installed 0.8.0 theme, egui's own held-down button's, `own`
/// (`widgets.active`): its `weak_bg_fill`, a `Button`'s fill, and its `fg_stroke` colour.
pub(crate) fn toggle_on(
    r: Option<&native_theme::next::ResolvedTheme>,
    own: &egui::style::WidgetVisuals,
) -> ToggleOn {
    use NextState as S;
    let first = |homes: &[(S, &'static str)], read: &dyn Fn(S) -> Option<egui::Color32>| {
        homes
            .iter()
            .enumerate()
            .find_map(|(i, (state, home))| read(*state).map(|c| (c, (i > 0).then_some(*home))))
    };
    let (fill, fill_from) = r
        .and_then(|r| {
            first(
                &[
                    (S::CHECKED, "button.body(CHECKED).fill"),
                    (S::PRESSED, "button.body(PRESSED).fill"),
                    (S::HOVER, "button.body(HOVER).fill"),
                ],
                &|state| next::state_fill_of(&r.button.body(state)),
            )
        })
        .unwrap_or((
            own.weak_bg_fill,
            Some("egui's own held-down button fill (widgets.active.weak_bg_fill)"),
        ));
    let (text, text_from) = r
        .and_then(|r| {
            first(
                &[
                    (S::CHECKED, "button.label(CHECKED).color"),
                    (S::PRESSED, "button.label(PRESSED).color"),
                ],
                &|state| next::state_text_color_of(&r.button.label(state)),
            )
        })
        .unwrap_or((
            own.fg_stroke.color,
            Some("egui's own held-down button text (widgets.active.fg_stroke)"),
        ));
    ToggleOn {
        fill,
        fill_from,
        text,
        text_from,
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

/// The Tabs group: a tab bar of two tabs, the first selected.
fn tabs(
    reg: &mut Registry,
    state: &mut DemoState,
    t: &native_theme_egui::ResolvedTheme,
    ui: &mut egui::Ui,
) {
    heading(reg, ui, "basic.tabs.heading", "Tabs");
    let tabs: Vec<(usize, &'static str)> = TABS.into_iter().enumerate().collect();
    let picked = demo::tab_bar(
        reg,
        ui,
        t,
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

/// The Icons group's first row: four icon-only tool buttons, as the toolbar's
/// (`crate::chrome`'s toolbar): the chosen set's Copy, Paste, Delete and Edit at
/// `toolbar.icon_size`, in the toolbar's scope, Ghost. Edit is an edit-mode switch
/// (`DemoState::basic_edit`), on at start: while on it is filled with the toolbar's checked
/// fill (`toolbar_button_checked_background`), lettered in the checked text, or, where the theme
/// states no checked fill, with the selected flat button's fill, `widgets.active.weak_bg_fill`;
/// and lined at rest in the checked pen (`toolbar_button_checked_border_color`,
/// `button.border.line_width` wide) where the theme states it, under the pointer and pressed in
/// the toolbar's hover outline, or that pen where the theme states no hover outline (E9).
fn icon_buttons(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let t = atlas.old_resolved(ui.ctx().theme());
    let size = t.toolbar.icon_size;
    let (set, icon_theme) = chosen;
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
                demo::toolbar_gap(ui, t, atlas.layout());
                let spacing = ui.spacing_mut();
                spacing.item_spacing.y = spacing.item_spacing.x;
                for (role, label, id) in [
                    (IconRole::ActionCopy, "Copy", "basic.icons.copy"),
                    (IconRole::ActionPaste, "Paste", "basic.icons.paste"),
                    (IconRole::ActionDelete, "Delete", "basic.icons.delete"),
                ] {
                    let image = demo::role_image(ui, role, *set, icon_theme.as_deref(), size);
                    let drawn = image.is_some();
                    wrap_before(ui, id);
                    let response = ui
                        .scope(|ui| {
                            // One line, as `text_button`'s label.
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                            demo::tool_button(ui, &t.button.border.padding, demo::toolbar_edge(t));
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
                    reg.tag(id, &response);
                    remember_width(ui, id, response.rect);
                    if drawn {
                        crate::chrome::place_icon(reg, ui, id, &response, size);
                    }
                }
                // The edit-mode switch: the one toolbar button that is ever checked.
                let id = "basic.icons.edit";
                let on = state.basic_edit;
                let mut drawn = false;
                wrap_before(ui, id);
                let response = ui
                    .scope(|ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        demo::tool_button(ui, &t.button.border.padding, demo::toolbar_edge(t));
                        // E9: a checked button's pen at rest; under the pointer and pressed the
                        // toolbar's hover outline where the theme states one, else the pen (S8).
                        // `Button::fill` leaves the stroke to the scope; only `Button::stroke`
                        // overrides it (`egui-0.37.0/src/widgets/button.rs:161`).
                        if let Some(pen) =
                            native_theme_egui::toolbar_button_checked_border_color(t).filter(|_| on)
                        {
                            let line = egui::Stroke::new(t.button.border.line_width, pen);
                            let w = &mut ui.visuals_mut().widgets;
                            w.inactive.bg_stroke = line;
                            if demo::toolbar_edge(t).is_none() {
                                (w.hovered.bg_stroke, w.active.bg_stroke) = (line, line);
                            }
                        }
                        let (fill, text) = edit_look(t, ui.visuals(), on);
                        // The checked text, before the icon is loaded in the scope's text
                        // colour.
                        if text.is_some() {
                            ui.visuals_mut().override_text_color = text;
                        }
                        let image = demo::role_image(
                            ui,
                            IconRole::ActionEdit,
                            *set,
                            icon_theme.as_deref(),
                            size,
                        );
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
                            };
                            let button = match fill {
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
                reg.tag(id, &response);
                remember_width(ui, id, response.rect);
                if drawn {
                    crate::chrome::place_icon(reg, ui, id, &response, size);
                }
            })
            .response
        },
    );
}

/// The fill handed to the Basic page's edit-mode switch and the text colour its scope takes,
/// while `on`: the toolbar's checked fill (`toolbar_button_checked_background`) and the checked
/// text, `button.checked_text_color` (`button.active_text_color` where that is unset), where the
/// theme states the fill; else the selected flat button's fill, `visuals`'
/// `widgets.active.weak_bg_fill`, and the scope's own text. Neither while off.
pub(crate) fn edit_look(
    t: &native_theme_egui::ResolvedTheme,
    visuals: &egui::Visuals,
    on: bool,
) -> (Option<egui::Color32>, Option<egui::Color32>) {
    if !on {
        return (None, None);
    }
    match native_theme_egui::toolbar_button_checked_background(t) {
        Some(fill) => (
            Some(fill),
            Some(to_color32(
                t.button
                    .checked_text_color
                    .unwrap_or(t.button.active_text_color),
            )),
        ),
        None => (Some(visuals.widgets.active.weak_bg_fill), None),
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

/// The frame the list and the table draw: `list.background_color`, `list.border`'s colour,
/// width and radius, no padding of its own (a row pads its text).
fn list_frame(l: &native_theme::theme::ResolvedListTheme) -> egui::Frame {
    egui::Frame::NONE
        .fill(to_color32(l.background_color))
        .stroke(to_stroke(
            egui::Stroke::NONE,
            l.border.color,
            l.border.line_width,
        ))
        .corner_radius(to_corner_radius(
            egui::CornerRadius::default(),
            l.border.corner_radius,
        ))
}

/// The radius of `list_frame`'s inside, `list.border.corner_radius` less `list.border.line_width`
/// (egui lays a frame's content inside its stroke, and the frame has no margin), on the corners
/// `[nw, ne, sw, se]` marks, square on the others: a fill where a row meets the frame keeps
/// inside the frame's arcs.
fn inner_corners(
    l: &native_theme::theme::ResolvedListTheme,
    [nw, ne, sw, se]: [bool; 4],
) -> egui::CornerRadius {
    let radius = to_corner_radius(
        egui::CornerRadius::ZERO,
        (l.border.corner_radius - l.border.line_width).max(0.0),
    );
    let at = |rounded: bool, r: u8| if rounded { r } else { 0 };
    egui::CornerRadius {
        nw: at(nw, radius.nw),
        ne: at(ne, radius.ne),
        sw: at(sw, radius.sw),
        se: at(se, radius.se),
    }
}

/// A list or table row's height and its text's inset: `stated` where the theme states the
/// row's height (`list.row_height`, `list.header_height`), grown above a text scale of 1 to
/// hold the row's line, its padding and `rule`, the width of a line drawn inside it, as gpui's
/// table and list rows grow and the iced showcase's (`rows::scaled_height`, over `size`, the
/// font's stated size, and `prefs`); else as tall as its content — one line of `font`, the
/// font as drawn, `defaults.line_height` of its size, and `list.border.padding` above and below
/// it, the platform's "sizes to content" (docs/platform-facts.md §2.15) — its text
/// `list.border.padding.left` in. A side the theme leaves unstated is `own`'s: for the list what
/// egui gives a selectable row there, the scope's `button_padding`
/// (`egui/src/widgets/button.rs:333-337`); for the table nothing, as the three showcases lay
/// their tables out.
fn row_metrics(
    t: &native_theme_egui::ResolvedTheme,
    prefs: &AccessibilityPreferences,
    (stated, size): (Option<f32>, f32),
    font: &egui::FontId,
    own: egui::Vec2,
    rule: f32,
) -> (f32, f32, f32) {
    let padding = &t.list.border.padding;
    let line_height = t.defaults.line_height;
    let line = font.size * line_height;
    let padding_y = padding.top.unwrap_or(own.y) + padding.bottom.unwrap_or(own.y);
    let height = stated.map_or(line + padding_y, |h| {
        rows::scaled_height(h, prefs, size, line_height, padding_y, rule)
    });
    // On egui's layout grid (`emath::GUI_ROUNDING`), as `allocate_exact_size` lays each row out,
    // so `n` rows are `n` row heights tall.
    (
        egui::emath::GuiRounding::round_ui(height),
        padding.left.unwrap_or(own.x),
        line,
    )
}

/// The list: `LIST_ROWS` selectable rows in a `ScrollArea` `LIST_VISIBLE` rows tall, framed by
/// `list_frame`, in one `Role::List` scope (its selection colours and item font). egui has no
/// list widget, and the leaves egui never reads from the style are per call (connector spec
/// §5.4, T18(a)): each row is painted here, `row_metrics` tall and inset, filled with
/// `list.selection_background` in `list.selection_text_color` while selected (the unfocused
/// pair while the window is not active, `selection_pair`) and with `list.hover_background` in
/// `list.hover_text_color` while hovered, outlined there in `list.hover_border_color` at
/// `defaults.border.line_width` where the theme states it, the fill rounded where it meets the
/// frame (`inner_corners`).
fn list(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let t = atlas.old_resolved(ui.ctx().theme());
    let l = &t.list;
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
                t,
                atlas.accessibility(),
                (l.row_height, l.item_font.size),
                &font,
                ui.spacing().button_padding,
                0.0,
            );
            let frame = list_frame(l);
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
                                let r = rows.add(reg, ui, "List row", |ui| {
                                    let (row, label) = list_row(
                                        ui,
                                        t,
                                        edges,
                                        &text,
                                        selected,
                                        (row_height, inset, line),
                                        &font,
                                    );
                                    labels_drawn.push(label);
                                    row
                                });
                                if r.clicked() {
                                    state.basic_list = i;
                                }
                                rows_drawn.push(r);
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

/// One list row across the list, `height` tall, its text `inset` in, one `line` tall and
/// centred vertically. Its fill covers what the list's view shows of it, rounded at a corner
/// where that meets a corner of `edges`, the frame's inside. Its response, and its text's
/// rectangle. A selected row takes [`selection_pair`] in the window's state.
fn list_row(
    ui: &mut egui::Ui,
    t: &native_theme_egui::ResolvedTheme,
    edges: egui::Rect,
    text: &str,
    selected: bool,
    (height, inset, line): (f32, f32, f32),
    font: &egui::FontId,
) -> (egui::Response, egui::Rect) {
    let l = &t.list;
    let size = egui::vec2(ui.available_width(), height);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    response.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, selected, text));
    let (fill, colour) = if selected {
        let (fill, colour) = selection_pair(t, ui.ctx().input(|input| input.focused));
        (Some(fill), colour)
    } else if response.hovered() {
        (
            Some(to_color32(l.hover_background)),
            to_color32(l.hover_text_color),
        )
    } else {
        (None, to_color32(l.item_font.color))
    };
    if let Some(fill) = fill {
        let view = ui.clip_rect();
        let shown = egui::Rect::from_x_y_ranges(
            rect.x_range(),
            rect.top().max(view.top())..=rect.bottom().min(view.bottom()),
        );
        if shown.is_positive() {
            // The scroll area lays the rows out on whole pixels and then on egui's layout grid
            // (`egui/src/containers/scroll_area.rs:797-803`; `emath/src/gui_rounding.rs:58-70`),
            // which moves each edge by at most half a pixel and half a grid step from the frame's
            // inside: half a point under a 0.5 pt line at one pixel per point. A row is far
            // taller, and a solid bar's groove far wider, than that.
            let off = 0.5 / ui.pixels_per_point() + 0.5 * egui::emath::GUI_ROUNDING;
            let meets = |a: f32, b: f32| (a - b).abs() <= off;
            let top = meets(shown.top(), edges.top());
            let bottom = meets(shown.bottom(), edges.bottom());
            let left = meets(shown.left(), edges.left());
            let right = meets(shown.right(), edges.right());
            let corners = inner_corners(
                l,
                [top && left, top && right, bottom && left, bottom && right],
            );
            ui.painter().rect_filled(shown, corners, fill);
            // A hovered, unselected row's outline on the row's shape, inside it, as Breeze's
            // line is (`egui-0.37.0/src/painter.rs:406-414`, `epaint-0.37.0/src/stroke.rs:102-111`),
            // clipped to the view as the fill is: Breeze clips the row to the view, so a row
            // partly scrolled out draws no line along the view's edge. The line is on the fill's
            // shape, its arcs on the fill's; on a side the view cuts the clip stops one line
            // width short of the fill, so the line along that edge is not drawn.
            if let Some(edge) = l
                .hover_border_color
                .filter(|_| !selected && response.hovered())
            {
                let stroke = to_stroke(egui::Stroke::NONE, edge, t.defaults.border.line_width);
                let short = |cut: bool| if cut { stroke.width } else { 0.0 };
                let clip = egui::Rect::from_x_y_ranges(
                    shown.x_range(),
                    shown.top() + short(rect.top() < shown.top())
                        ..=shown.bottom() - short(rect.bottom() > shown.bottom()),
                );
                ui.painter().with_clip_rect(clip).rect_stroke(
                    shown,
                    corners,
                    stroke,
                    egui::StrokeKind::Inside,
                );
            }
        }
    }
    let label = paint_cell_text(ui, text, font, colour, (rect, inset, line));
    (response, label)
}

/// A selected list or table row's fill and text colour: `defaults.selection_inactive_background`
/// lettered `defaults.selection_inactive_text_color` while the window is not active
/// (`window_active` false, egui's `InputState::focused`) where the theme's inactive fill
/// differs from `defaults.selection_background`; `list.selection_background` and
/// `list.selection_text_color` otherwise.
pub(crate) fn selection_pair(
    t: &native_theme_egui::ResolvedTheme,
    window_active: bool,
) -> (egui::Color32, egui::Color32) {
    let d = &t.defaults;
    if !window_active && d.selection_inactive_background != d.selection_background {
        (
            selection_inactive_background(t),
            selection_inactive_text_color(t),
        )
    } else {
        (
            to_color32(t.list.selection_background),
            to_color32(t.list.selection_text_color),
        )
    }
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

/// A table header section's fill: pressed in `list.header_active_background`, else in the
/// hovered fill; hovered in `list.header_hover_background`; `None` at rest and where the theme
/// states neither fill.
pub(crate) fn header_section_fill(
    t: &native_theme_egui::ResolvedTheme,
    hovered: bool,
    pressed: bool,
) -> Option<egui::Color32> {
    if pressed {
        list_header_active_background(t).or(list_header_hover_background(t))
    } else if hovered {
        list_header_hover_background(t)
    } else {
        None
    }
}

/// The table: a header and two rows, the second selected, `BASIC_WIDE` across the list's
/// frame (`list_frame`), the two columns half the width inside it each, in one `Role::List`
/// scope. egui has no table the theme reaches (egui_extras' `TableBuilder` paints only its
/// stripes), so it is painted here as the list's rows are: the header in
/// `list.header_background` and `list.header_font` at its weight and scaled size
/// (`list_header_font`), `list_header_height` tall where the theme states it, else a row
/// (`row_metrics`, of the header font), the rows `row_metrics` tall in
/// `list.item_font`, the selected one in `list.selection_background` and
/// `list.selection_text_color` (the unfocused pair while the window is not active,
/// `selection_pair`), the header's fill rounded at the top and the last row's at the bottom,
/// where they meet the frame (`inner_corners`); each header section hovered in
/// `list.header_hover_background` and pressed in `list.header_active_background`, else the
/// hovered fill, where the theme states them (`header_section_fill`), rounded where the section
/// meets the frame's top corner; a vertical
/// `list.grid_color` line between the columns through the header and the rows and a horizontal
/// one under the header, each `separator.line_width` wide, inside the cell it closes, the
/// header's share of them in `list.header_separator_color` where the theme states it (E7).
fn table(reg: &mut Registry, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let t = atlas.old_resolved(ui.ctx().theme());
    let prefs = atlas.accessibility();
    let l = &t.list;
    let mut places: Vec<(String, egui::Rect)> = Vec::new();
    let frame_response = demo::scoped_container(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "Table",
        |ui, _rows, _reg| {
            let item_font = egui::TextStyle::Body.resolve(ui.style());
            let header_font = egui::FontId::new(
                list_header_font(t, prefs).size,
                demo::weighted_family(ui, l.header_font.weight, t.defaults.font.weight),
            );
            let grid = to_color32(l.grid_color);
            let grid_width = t.separator.line_width;
            let (row_height, inset, line) = row_metrics(
                t,
                prefs,
                (l.row_height, l.item_font.size),
                &item_font,
                egui::Vec2::ZERO,
                0.0,
            );
            // `list.header_height` where the theme states it, else the header is a row; the
            // grid's line under it lies inside it
            let (header_height, _, header_line) = row_metrics(
                t,
                prefs,
                (list_header_height(t).or(l.row_height), l.header_font.size),
                &header_font,
                egui::Vec2::ZERO,
                grid_width,
            );
            let frame = list_frame(l);
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
                    painter.rect_filled(
                        header,
                        inner_corners(l, [true, true, false, false]),
                        to_color32(l.header_background),
                    );
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
                            t,
                            response.hovered(),
                            response.is_pointer_button_down_on(),
                        );
                        if let Some(fill) = fill {
                            painter.rect_filled(
                                cell,
                                inner_corners(l, [i == 0, i == 1, false, false]),
                                fill,
                            );
                        }
                        paint_cell_text(
                            ui,
                            text,
                            &header_font,
                            to_color32(l.header_font.color),
                            (cell, inset, header_line),
                        );
                        places.push((id.to_string(), cell));
                    }
                    for (i, cells) in TABLE_ROWS.iter().enumerate() {
                        let top = header.bottom() + i as f32 * row_height;
                        let row = egui::Rect::from_min_size(
                            egui::pos2(rect.left(), top),
                            egui::vec2(inner, row_height),
                        );
                        let (fill, colour) = if i == TABLE_SELECTED {
                            let window_active = ui.ctx().input(|input| input.focused);
                            let (fill, colour) = selection_pair(t, window_active);
                            (Some(fill), colour)
                        } else {
                            (None, to_color32(l.item_font.color))
                        };
                        if let Some(fill) = fill {
                            let last = i.saturating_add(1) == TABLE_ROWS.len();
                            painter.rect_filled(
                                row,
                                inner_corners(l, [false, false, last, last]),
                                fill,
                            );
                        }
                        for (j, text) in cells.iter().enumerate() {
                            let cell = if j == 0 {
                                egui::Rect::from_x_y_ranges(row.left()..=mid, row.y_range())
                            } else {
                                egui::Rect::from_x_y_ranges(mid..=row.right(), row.y_range())
                            };
                            paint_cell_text(ui, text, &item_font, colour, (cell, inset, line));
                        }
                        places.push((format!("basic.table.row_{}", i.saturating_add(1)), row));
                    }
                    // The grid: under the header, and between the columns from top to bottom,
                    // each inside the cell it closes. Over the header, in
                    // `list.header_separator_color` where the theme states it (E7), Breeze's
                    // translucent section line laid over the header's fill: the column line is
                    // split at the line under the header, so no pixel takes that colour twice.
                    let stroke = egui::Stroke::new(grid_width, grid);
                    let x = mid - 0.5 * grid_width;
                    match l.header_separator_color {
                        Some(c) => {
                            let head = egui::Stroke::new(grid_width, to_color32(c));
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
    reg.tag("basic.table.frame", &frame_response);
    for (id, rect) in places {
        reg.place(ui, &id, rect);
    }
}
