//! Basic: the controls the three showcases all draw, in the same order, with the same labels,
//! values and states, packed onto one screen in five columns, so the gpui, iced and egui
//! captures compare control by control. Each control goes through the seam its palette page
//! gives it; what egui leaves to the call site is applied per call from the theme. Every
//! element of `docs/showcase-elements.toml` the page draws is placed where it is drawn, its
//! parts too, for `--dump-layout` and Widget Info.

use egui::Button;
use egui::widget_style::{Classes, WidgetState};
use native_theme::theme::{IconRole, IconSet};
use native_theme_egui::convert::{to_color32, to_corner_radius, to_margin, to_stroke};
use native_theme_egui::{
    Role, RoleVariant, Surface, TextRole, ThemeAtlas, input_frame, text_area_frame,
};
use native_theme_egui_widgets::combo_box::ComboBox;
use native_theme_egui_widgets::expander::Expander;
use native_theme_egui_widgets::parts::Parts;
use native_theme_egui_widgets::progress_bar::ProgressBar;
use native_theme_egui_widgets::radio_button::RadioButton;
use native_theme_egui_widgets::segmented_control::SegmentedControl;
use native_theme_egui_widgets::slider::Slider;
use native_theme_egui_widgets::spinner::Spinner;
use native_theme_egui_widgets::switch::Switch;
use native_theme_egui_widgets::wrap;

use super::{DemoState, caption};
use crate::demo::{self, Registry};

/// The width of the Basic page's text fields, drop-down, number input, slider and progress
/// bar. The model states no such width; it is the Basic page's own, the gpui showcase's and
/// the iced showcase's `BASIC_WIDTH` too, so the three pages lay the same controls out alike.
pub(crate) const BASIC_WIDTH: f32 = 140.0;

/// The width of the Basic page's text area, list, expander, card, separator and table: the
/// Basic page's own, as `BASIC_WIDTH`, and the gpui and iced showcases' `BASIC_WIDE`.
pub(crate) const BASIC_WIDE: f32 = 170.0;

/// The page's columns: the Basic page's layout (a datum of the page, not a style value).
const COLUMNS: usize = 5;

/// The drop-down's rows.
const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

/// The progress bar's value: the datum on display, not a style value.
const PROGRESS: f32 = 0.4;

/// The tab bar's tabs, and the segmented control's segments.
const TABS: [&str; 2] = ["One", "Two"];
const SEGMENTS: [&str; 3] = ["Day", "Week", "Month"];

/// The list's rows, and how many of them it shows: data of the page.
const LIST_ROWS: usize = 8;
const LIST_VISIBLE: usize = 3;

/// The text area's text, three lines.
pub(crate) const AREA_TEXT: &str = "Line one\nLine two\nLine three";
const AREA_ROWS: usize = 3;

/// The number input's value and step: data of the page.
pub(crate) const NUMBER: f64 = 42.0;
const NUMBER_STEP: f64 = 1.0;

/// The table's header and rows, and the row shown selected (the second): data of the page.
const TABLE_HEADER: [&str; 2] = ["Name", "Size"];
const TABLE_ROWS: [[&str; 2]; 2] = [["a.txt", "1 KB"], ["b.png", "20 KB"]];
const TABLE_SELECTED: usize = 1;

/// The groups of the page, column by column, in order (Basic page v5), which the tests check
/// the page against.
#[cfg(test)]
pub(crate) const GROUPS: [&[&str]; COLUMNS] = [
    &["Buttons", "Checkboxes", "Radio buttons", "Drop-down"],
    &["Text inputs", "Text area", "Slider"],
    &[
        "Switches",
        "Number input",
        "Spinner",
        "Segmented control",
        "Card",
    ],
    &["Typography", "Separator", "Progress bar", "List"],
    &["Icons", "Tabs", "Expander", "Table"],
];

/// Five columns of equal width, `layout.section_gap` apart (the page's gap where the theme
/// states none: egui's `item_spacing`), each a stack of groups top-aligned: each column is laid
/// out in a `Ui` of its own at its place, so a group wider than its column (a preset's tab
/// minimum width) shows as the overflow it is, and the next column does not move.
pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let gap = atlas
        .layout()
        .section_gap
        .unwrap_or(ui.spacing().item_spacing.x);
    let origin = ui.cursor().min;
    let columns = COLUMNS as f32;
    let page_width = ui.available_width();
    let width = ((page_width - gap * (columns - 1.0)) / columns).max(0.0);
    let mut used = egui::Rect::NOTHING;
    let pixels = ui.pixels_per_point();
    for column in 0..COLUMNS {
        // Both edges on whole pixels, so a one-pixel border the column draws is one pixel wide,
        // not two half-covered ones, and the column is the room between the edges it paints.
        let left = origin.x + column as f32 * (width + gap);
        let snap = |x: f32| egui::emath::GuiRounding::round_to_pixels(x, pixels);
        let rect = egui::Rect::from_x_y_ranges(
            snap(left)..=snap(left + width),
            origin.y..=origin.y + ui.available_height(),
        );
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(("basic column", column))
                .max_rect(rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        match column {
            0 => column_1(reg, state, atlas, &mut child),
            1 => column_2(reg, state, atlas, &mut child),
            2 => column_3(reg, state, atlas, &mut child),
            3 => column_4(reg, state, atlas, &mut child),
            _ => column_5(reg, state, atlas, &mut child, chosen),
        }
        let content = child.min_rect();
        reg.place(
            ui,
            &format!("basic.column_{}", column + 1),
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
        ui.advance_cursor_after_rect(used);
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
        .style()
        .button_style(&Classes::default(), WidgetState::Inactive);
    let size = text_size(
        ui,
        text,
        egui::FontSelection::FontId(style.text_style.font_id),
    );
    let inner = button.rect - style.frame.total_margin();
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
    let style = ui
        .style()
        .checkbox_style(&Classes::default(), WidgetState::Inactive);
    let inner = control.rect - style.frame.total_margin();
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

/// Disables `ui` for a check box faded as one, as the platform fades a disabled widget where the
/// theme states no disabled colours: `filter: Opacity(var(--disabled-opacity))` on the whole
/// widget, which keeps its normal colours (docs/platform-facts.md §2.1.6, GNOME), so its mark is
/// drawn on its fill and the pair fades over the page. egui fades each shape on its own
/// (`Ui::disable` multiplies the painter's opacity by `disabled_alpha`,
/// `egui/src/ui.rs:497-503`), which would fade the mark over the faded fill; here each colour
/// the box paints is composed over what lies under it (its fill over `ground`, the border and
/// the mark over the fill, the label over `ground`) and faded by `disabled_alpha` over `ground`,
/// and the painter keeps its opacity.
fn fade_as_one(ui: &mut egui::Ui, ground: egui::Color32) {
    let alpha = ui.visuals().disabled_alpha();
    let opacity = ui.opacity();
    let fade = |colour: egui::Color32| ground.lerp_to_gamma(colour, alpha);
    let text = ui.visuals().override_text_color;
    let visuals = ui.visuals_mut();
    let widgets = &mut visuals.widgets;
    for cell in [&mut widgets.noninteractive, &mut widgets.inactive] {
        let fill = ground.blend(cell.bg_fill);
        cell.bg_fill = fade(fill);
        cell.bg_stroke.color = fade(fill.blend(cell.bg_stroke.color));
        cell.fg_stroke.color = fade(fill.blend(cell.fg_stroke.color));
    }
    visuals.override_text_color = text.map(|colour| fade(ground.blend(colour)));
    ui.disable();
    ui.set_opacity(opacity);
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
    let mut label = None;
    let response = demo::scoped(reg, ui, Role::Button, variant, kind, |ui| {
        let text = demo::lined(ui, text, egui::TextStyle::Button);
        let r = ui.add_enabled(enabled, make(text.clone()).min_size(min));
        label = Some(button_label(ui, &r, text));
        r
    });
    reg.amend_last(|i| {
        i.read.push(("button.min_width", min.x.to_string()));
        i.read.push(("button.min_height", min.y.to_string()));
    });
    reg.tag(id, &response);
    if let Some(label) = label {
        reg.place(ui, &format!("{id}.label"), label);
    }
    response
}

/// Buttons (the toggle button their third row), check boxes, radio buttons, the drop-down.
fn column_1(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;
    // `button.min_width`, which egui's `Button` never reads from the style: it raises only its
    // height, to `interact_size.y` (`button.min_height` in the button scope), so the width is
    // the application's, per call (connector spec §5.3, `Button::min_size`).
    let min = egui::vec2(t.button.min_width, t.button.min_height);
    let plain = |text: egui::RichText| Button::new(text);

    heading(reg, ui, "basic.buttons.heading", "Buttons");
    ui.horizontal(|ui| {
        let kind = ("basic.buttons.default", "button (enabled)", normal);
        text_button(reg, ui, kind, ("Button", min), plain, true);
        let kind = ("basic.buttons.primary", "button (suggested action)", normal);
        let primary = |text: egui::RichText| Button::new(text).selected(true);
        text_button(reg, ui, kind, ("Primary", min), primary, true);
    });
    ui.horizontal(|ui| {
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
                        ui.label(demo::lined(ui, "A tooltip", egui::TextStyle::Body))
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
    // A toggle button: `Off` a button at rest, `On` the same button held on, in the button
    // role's pressed colours, `button.active_background` (its hover fill where it states none,
    // §6.4) and `button.active_text_color` — the model has no checked-button colour — per call:
    // egui's `selected` flag would take the suggested action's `selection` colours.
    let on_fill = to_color32(
        t.button
            .active_background
            .unwrap_or(t.button.hover_background),
    );
    let on_text = to_color32(t.button.active_text_color);
    ui.horizontal(|ui| {
        let kind = ("basic.buttons.toggle_off", "toggle button (off)", normal);
        text_button(reg, ui, kind, ("Off", min), plain, true);
        let kind = ("basic.buttons.toggle_on", "toggle button (on)", normal);
        let held = |text: egui::RichText| Button::new(text.color(on_text)).fill(on_fill);
        text_button(reg, ui, kind, ("On", min), held, true);
    });

    // Each control shows one state and is held in it: a click changes a copy made for the pass.
    // One per row, each in its own checkbox scope, whose `interact_size.y` is the indicator.
    // The disabled box is checked. Where the theme states a disabled fill it is the `Disabled`
    // cell's; where it states none the platform dims by opacity alone
    // (docs/platform-facts.md §2.1.6), so the box is the checked (`Selected`) cell's, faded by
    // its `disabled_alpha`, `checkbox.disabled_opacity`: egui's cells are one variant each.
    heading(reg, ui, "basic.checkboxes.heading", "Checkboxes");
    let disabled_variant = if t.checkbox.disabled_background.is_some() {
        RoleVariant::Disabled
    } else {
        RoleVariant::Selected
    };
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
            disabled_variant,
            false,
            "checkbox (disabled)",
            "basic.checkboxes.disabled",
        ),
    ] {
        let mut value = checked;
        let mut parts = Vec::new();
        let response = demo::scoped(reg, ui, Role::Checkbox, variant, kind, |ui| {
            // A checked box is bordered in `checkbox.border.color`, disabled too: egui's
            // `Disabled` cell serves a checked and an unchecked box alike and carries the
            // unchecked border, so the disabled checked box takes the checked one per call.
            if checked && !enabled && variant == RoleVariant::Disabled {
                let border = to_color32(t.checkbox.border.color);
                let widgets = &mut ui.visuals_mut().widgets;
                widgets.noninteractive.bg_stroke.color = border;
                widgets.inactive.bg_stroke.color = border;
            }
            let text = demo::lined(ui, label, egui::TextStyle::Body);
            let r = if !enabled && variant == RoleVariant::Selected {
                let ground = to_color32(t.defaults.background_color);
                ui.scope(|ui| {
                    fade_as_one(ui, ground);
                    ui.add(egui::Checkbox::new(&mut value, text.clone()))
                })
                .inner
            } else {
                ui.add_enabled(enabled, egui::Checkbox::new(&mut value, text.clone()))
            };
            parts = checkbox_parts(ui, &r, text, checked);
            r
        });
        reg.tag(id, &response);
        for (part, rect) in parts {
            reg.place(ui, &format!("{id}.{part}"), rect);
        }
    }

    // The companion crate's radio button (docs/todo_egui-widgets-spec.md §4.8): egui's, in
    // `RoleVariant::Selected` while selected as on the Selection page, its dot
    // `checkbox.radio_dot_diameter` across where the theme states one.
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
            let text = demo::lined(ui, label, egui::TextStyle::Body);
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

    // The companion crate's drop-down (docs/todo_egui-widgets-spec.md §4.6).
    drop_down(reg, state, ui);
}

/// The Switches group: the companion crate's switch (docs/todo_egui-widgets-spec.md §4.1),
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
            let text = demo::lined(ui, label, egui::TextStyle::Body);
            ui.add(Switch::new(&mut value).label(text).enabled(enabled))
        });
        reg.tag(id, &r);
        let names = [("track", "track"), ("thumb", "thumb"), ("label", "label")];
        place_parts(reg, ui, id, &r, &names);
    }
}

/// A text field of the Basic page, built inside its `Role::Input` scope (`ui` is the scope's), so
/// its frame is the style it paints in: the disabled field takes the disabled cell's
/// `input.disabled_background`. `input.min_height` is a per-instance `min_size`, which a
/// `TextEdit` never reads from the style: it is one row plus its margin tall, at least its
/// `min_size` (connector spec §5.4); the row is centred in the height the minimum adds, as the
/// platform centres a field's text.
fn field<'a>(
    text: &'a mut String,
    id: egui::Id,
    ui: &egui::Ui,
    t: &native_theme_egui::ResolvedTheme,
) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .desired_width(BASIC_WIDTH)
        .min_size(egui::vec2(BASIC_WIDTH, t.input.min_height))
        .vertical_align(egui::Align::Center)
        .id(id)
        .frame(input_frame(ui, id, t))
}

/// A `TextEdit` of `ui` lays each row out `row_height + extra_text_line_spacing` tall
/// (`egui/src/widgets/text_edit/builder.rs:488-490`, `:512-514`); the connector writes the
/// spacing only where the line box, the font's size times `defaults.line_height`, is taller
/// than the font's own row (§6.15). Where the row is taller — a face whose row exceeds the
/// theme's line box, Noto Sans (fontconfig's substitute for Inter) under catppuccin-mocha's 1.2
/// — the spacing is the negative difference, written per field, so each row is one line box
/// tall, as a label's is (`demo::lined`).
fn line_box_rows(ui: &mut egui::Ui, t: &native_theme_egui::ResolvedTheme) {
    let font = egui::FontSelection::Default.resolve(ui.style());
    let row = ui.fonts_mut(|f| f.row_height(&font));
    ui.spacing_mut().extra_text_line_spacing = font.size * t.defaults.line_height - row;
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
    let inner = field.rect - frame.total_margin();
    let size = text_size(
        ui,
        demo::lined(ui, text, egui::TextStyle::Body),
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
/// text placed as `<id>.text`, `shown` the text it holds (or its hint).
fn text_field(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    (id, kind, variant): (&str, &'static str, RoleVariant),
    t: &native_theme_egui::ResolvedTheme,
    shown: &str,
    add: impl FnOnce(&mut egui::Ui, egui::Id) -> egui::Response,
) -> egui::Response {
    let edit_id = ui.make_persistent_id(id);
    let mut text = None;
    let response = demo::scoped(reg, ui, Role::Input, variant, kind, |ui| {
        line_box_rows(ui, t);
        let r = add(ui, edit_id);
        text = Some(field_text(
            ui,
            &r,
            &input_frame(ui, edit_id, t),
            shown,
            true,
        ));
        r
    });
    reg.amend_last(|i| {
        i.read
            .push(("input.min_height", t.input.min_height.to_string()));
    });
    reg.tag(id, &response);
    if let Some(text) = text {
        reg.place(ui, &format!("{id}.text"), text);
    }
    response
}

/// Text inputs (the focused one their fourth row), the text area, the slider.
fn column_2(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    heading(reg, ui, "basic.text_inputs.heading", "Text inputs");
    let kind = ("basic.text_inputs.placeholder", "TextEdit (hint)", normal);
    text_field(reg, ui, kind, t, "Placeholder", |ui, id| {
        let hint = demo::lined(ui, "Placeholder", egui::TextStyle::Body);
        ui.add(field(&mut state.basic_hint, id, ui, t).hint_text(hint))
    });
    reg.amend_last(|i| i.notes.push(("hint text", "\"Placeholder\"".to_string())));
    let kind = ("basic.text_inputs.filled", "TextEdit (single line)", normal);
    let shown = state.basic_text.clone();
    text_field(reg, ui, kind, t, &shown, |ui, id| {
        ui.add(field(&mut state.basic_text, id, ui, t))
    });
    let kind = (
        "basic.text_inputs.disabled",
        "TextEdit (disabled)",
        RoleVariant::Disabled,
    );
    let mut disabled = "Disabled".to_string();
    text_field(reg, ui, kind, t, "Disabled", |ui, id| {
        ui.add_enabled(false, field(&mut disabled, id, ui, t))
    });
    // A text field holding the keyboard focus from the start, so its focus border shows.
    let kind = ("basic.text_inputs.focused", "TextEdit (focused)", normal);
    let shown = state.basic_focused.clone();
    let focused = text_field(reg, ui, kind, t, &shown, |ui, id| {
        ui.add(field(&mut state.basic_focused, id, ui, t))
    });
    if !state.basic_focus_given {
        focused.request_focus();
        state.basic_focus_given = true;
    }

    heading(reg, ui, "basic.text_area.heading", "Text area");
    let id = ui.make_persistent_id("basic/area");
    let mut text = None;
    let area = demo::scoped(reg, ui, Role::Input, normal, "TextEdit (multiline)", |ui| {
        line_box_rows(ui, t);
        let frame = text_area_frame(ui, id, t);
        let r = ui.add(
            egui::TextEdit::multiline(&mut state.basic_area)
                .desired_width(BASIC_WIDE)
                .desired_rows(AREA_ROWS)
                .id(id)
                .frame(frame),
        );
        text = Some(field_text(ui, &r, &frame, AREA_TEXT, false));
        r
    });
    reg.tag("basic.text_area.field", &area);
    if let Some(text) = text {
        reg.place(ui, "basic.text_area.field.text", text);
    }

    // The companion crate's slider (docs/todo_egui-widgets-spec.md §4.2).
    slider(reg, state, ui);
}

/// The Slider group: the companion crate's slider (docs/todo_egui-widgets-spec.md §4.2),
/// `slider.*`'s rail, trailing fill and a knob in `slider.thumb_color`, which egui paints in
/// the rail's colour.
fn slider(reg: &mut Registry, state: &mut DemoState, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.slider.heading", "Slider");
    let slider = demo::widget(reg, ui, Role::Slider, normal, "Slider (horizontal)", |ui| {
        ui.scope(|ui| {
            ui.spacing_mut().slider_width = BASIC_WIDTH;
            ui.add(Slider::new(&mut state.basic_slider, 0.0..=100.0))
        })
        .inner
    });
    reg.amend_last(|i| i.notes.push(("range", "0 to 100".to_string())));
    reg.tag("basic.slider.control", &slider);
    let names = [("track", "track"), ("fill", "fill"), ("thumb", "thumb")];
    place_parts(reg, ui, "basic.slider.control", &slider, &names);
}

/// The Progress bar group: the companion crate's progress bar
/// (docs/todo_egui-widgets-spec.md §4.9), egui's, rounded `progress_bar.border.corner_radius`
/// and outlined as `progress_bar.border` states, which egui's `ProgressBar` does not draw.
fn progress_bar(reg: &mut Registry, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.progress_bar.heading", "Progress bar");
    let bar = demo::scoped(reg, ui, Role::ProgressBar, normal, "ProgressBar", |ui| {
        ui.add(ProgressBar::new(PROGRESS).desired_width(BASIC_WIDTH))
    });
    reg.amend_last(|i| {
        i.read.push((
            "progress_bar.border.corner_radius",
            t.progress_bar.border.corner_radius.to_string(),
        ));
        i.read.push((
            "progress_bar.border.line_width",
            t.progress_bar.border.line_width.to_string(),
        ));
        i.read.push((
            "progress_bar.border.color",
            format!("{:?}", t.progress_bar.border.color),
        ));
    });
    reg.tag("basic.progress_bar.bar", &bar);
    place_parts(reg, ui, "basic.progress_bar.bar", &bar, &[("fill", "fill")]);
}

/// The Drop-down group: the companion crate's drop-down (docs/todo_egui-widgets-spec.md §4.6),
/// egui's own `ComboBox` in the combo-box scope, as tall as its text and padding make it, at
/// least `combo_box.min_height`, where egui's square arrow box would make it taller.
fn drop_down(reg: &mut Registry, state: &mut DemoState, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.drop_down.heading", "Drop-down");
    let current = FRUITS.get(state.basic_combo).copied().unwrap_or_default();
    let combo = demo::scoped_popup(
        reg,
        ui,
        Role::ComboBox,
        normal,
        "ComboBox",
        |ui, modifier, row, reg| {
            let mut combo = ComboBox::from_id_salt("basic/combo")
                .selected_text(demo::lined(ui, current, egui::TextStyle::Button))
                .width(BASIC_WIDTH);
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
        },
    );
    reg.tag("basic.drop_down.trigger", &combo);
    let names = [("text", "text"), ("arrow", "arrow")];
    place_parts(reg, ui, "basic.drop_down.trigger", &combo, &names);
}

/// The switches, the number input, the spinner, the segmented control, a card.
fn column_3(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    switches(reg, ui);

    // A number input: egui's `DragValue`, the number field egui has, in the input role's
    // scope, `BASIC_WIDTH` wide and `input.min_height` tall (its `interact_size`, which
    // `DragValue` sizes its button by, `egui/src/widgets/drag_value.rs`), stepping by 1. egui
    // draws it as a button whose text a drag or a click edits: no step buttons.
    heading(reg, ui, "basic.number_input.heading", "Number input");
    let shown = format!("{}", state.basic_number);
    let mut text = None;
    let number = demo::scoped(reg, ui, Role::Input, normal, "DragValue", |ui| {
        ui.spacing_mut().interact_size = egui::vec2(BASIC_WIDTH, t.input.min_height);
        // The field's frame as a text field's (`input_frame`): `DragValue` is a `Button`, which
        // fills with the state's `weak_bg_fill` and pads by `button_padding` less its stroke
        // (`egui/src/widget_style.rs:151-165`), where a `TextEdit` fills with
        // `text_edit_bg_color` and pads by `input_margin`; a `Vec2` pads a pair of sides alike,
        // so each pair is their mean.
        let margin = native_theme_egui::input_margin(t);
        let pair = |a: i8, b: i8| f32::midpoint(f32::from(a), f32::from(b));
        ui.spacing_mut().button_padding = egui::vec2(
            pair(margin.left, margin.right),
            pair(margin.top, margin.bottom),
        );
        let fill = ui.visuals().text_edit_bg_color();
        let widgets = &mut ui.visuals_mut().widgets;
        for state in [
            &mut widgets.inactive,
            &mut widgets.hovered,
            &mut widgets.active,
        ] {
            state.weak_bg_fill = fill;
        }
        let r = ui.add(egui::DragValue::new(&mut state.basic_number).speed(NUMBER_STEP));
        text = Some(button_label(ui, &r, &shown));
        r
    });
    reg.amend_last(|i| i.notes.push(("step", format!("{NUMBER_STEP}"))));
    reg.tag("basic.number_input.field", &number);
    if let Some(text) = text {
        reg.place(ui, "basic.number_input.field.text", text);
    }

    spinner(reg, ui);

    // The companion crate's segmented control (docs/todo_egui-widgets-spec.md §4.4): one
    // control of joined buttons in one `Role::SegmentedControl` scope, whose cell carries the
    // segment height, padding and colours; one outline in `border`, `separator_width`
    // dividers between the segments; a radio group.
    heading(reg, ui, "basic.segmented.heading", "Segmented control");
    let control = demo::widget(
        reg,
        ui,
        Role::SegmentedControl,
        normal,
        "segmented control",
        |ui| {
            let segments = SEGMENTS.map(|s| demo::lined(ui, s, egui::TextStyle::Button));
            ui.add(SegmentedControl::new(&mut state.basic_segment, segments))
        },
    );
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

    card(reg, atlas, ui);
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

/// The Spinner group: the companion crate's spinner (docs/todo_egui-widgets-spec.md §4.3),
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

/// One line of the Typography group in `role` (the base style's `Body`, `defaults.font`, for
/// `None`), the element `id`.
fn typography_line(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Option<TextRole>,
    text: &str,
    id: &str,
) {
    let line = demo::base(reg, ui, "Label (typography)", |ui| match role {
        Some(role) => ui.label(demo::role_text(ui, role, text)),
        None => ui.label(demo::lined(ui, text, egui::TextStyle::Body)),
    });
    reg.tag(id, &line);
}

/// Typography, a separator, the progress bar, the list.
fn column_4(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    // One line in each of the theme's text roles: the text-scale roles through
    // `demo::role_text`, the body in the base style's `Body` (`defaults.font`), the link in the
    // link role, the monospace line in the base style's `Monospace` (`defaults.mono_font`).
    heading(reg, ui, "basic.typography.heading", "Typography");
    typography_line(
        reg,
        ui,
        Some(TextRole::Caption),
        "Caption",
        "basic.typography.caption",
    );
    typography_line(reg, ui, None, "Body", "basic.typography.body");
    // The companion crate's link (docs/todo_egui-widgets-spec.md §4.5): egui's `Link` in the
    // link scope, its text in `link.*`'s rest, hover, pressed and disabled colours and
    // underlined at rest where `link.underline_enabled` says so, which egui's `Link` never
    // reads: it underlines only on hover or focus (`egui/src/widgets/hyperlink.rs:50-54`).
    let link = demo::widget(reg, ui, Role::Link, normal, "Link", |ui| {
        let link = wrap::link(ui, demo::lined(ui, "Link", egui::TextStyle::Body));
        ui.add(link)
    });
    reg.amend_last(|i| {
        i.read.push((
            "link.underline_enabled",
            t.link.underline_enabled.to_string(),
        ));
        i.read
            .push(("link.font.color", format!("{:?}", t.link.font.color)));
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
        (TextRole::Display, "Display", "basic.typography.display"),
    ] {
        typography_line(reg, ui, Some(role), text, id);
    }
    // The line box `defaults.line_height` of the monospace font's size, as a body line's is.
    let mono = demo::base(reg, ui, "Label (monospace)", |ui| {
        let size = egui::TextStyle::Monospace.resolve(ui.style()).size;
        ui.label(
            egui::RichText::new("Monospace")
                .monospace()
                .line_height(Some(size * t.defaults.line_height)),
        )
    });
    reg.tag("basic.typography.monospace", &mono);

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

    progress_bar(reg, atlas, ui);

    heading(reg, ui, "basic.list.heading", "List");
    list(reg, state, t, ui);
}

/// The Icons group's first row: three icon-only tool buttons, as the toolbar's
/// (`crate::chrome`'s toolbar): the chosen set's Copy, Paste and Delete at
/// `toolbar.icon_size`, in the toolbar's scope, Ghost.
fn icon_buttons(
    reg: &mut Registry,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let size = t.toolbar.icon_size;
    let (set, icon_theme) = chosen;
    demo::scoped_container(
        reg,
        ui,
        Role::Toolbar,
        RoleVariant::Normal,
        "icon buttons",
        |ui, bar, reg| {
            ui.horizontal(|ui| {
                demo::toolbar_gap(ui, t, atlas.layout());
                for (role, label, id) in [
                    (IconRole::ActionCopy, "Copy", "basic.icons.copy"),
                    (IconRole::ActionPaste, "Paste", "basic.icons.paste"),
                    (IconRole::ActionDelete, "Delete", "basic.icons.delete"),
                ] {
                    let image = demo::role_image(ui, role, *set, icon_theme.as_deref(), size);
                    let drawn = image.is_some();
                    let response = ui
                        .scope(|ui| {
                            demo::tool_button(ui, &t.button.border.padding);
                            bar.add(reg, ui, "icon button", |ui| {
                                let button = match image {
                                    Some(image) => Button::image(image),
                                    None => Button::new(label),
                                };
                                let r = ui.add(button);
                                r.widget_info(|| {
                                    egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label)
                                });
                                r
                            })
                        })
                        .inner;
                    reg.ghost_last();
                    reg.tag(id, &response);
                    if drawn {
                        crate::chrome::place_icon(reg, ui, id, &response, size);
                    }
                }
            })
            .response
        },
    );
}

/// The Icons group's second row: the chosen set's open folder at the theme's three icon sizes,
/// `defaults.icon_sizes`' small, toolbar and large, never another set's.
fn icons(
    reg: &mut Registry,
    t: &native_theme_egui::ResolvedTheme,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let (set, icon_theme) = chosen;
    let sizes = &t.defaults.icon_sizes;
    ui.scope(|ui| {
        // One row, as tall as the largest icon, each icon centred on it: `ui.horizontal`
        // centres on the row's `interact_size.y` (`egui/src/ui.rs:2376-2379`).
        ui.spacing_mut().interact_size.y = sizes.small.max(sizes.toolbar).max(sizes.large);
        ui.horizontal(|ui| {
            for (size, id) in [
                (sizes.small, "basic.icons.small"),
                (sizes.toolbar, "basic.icons.toolbar"),
                (sizes.large, "basic.icons.large"),
            ] {
                let Some(image) =
                    demo::role_image(ui, IconRole::FolderOpen, *set, icon_theme.as_deref(), size)
                else {
                    continue;
                };
                let icon = demo::base(reg, ui, "Image · icon", |ui| ui.add(image));
                reg.amend_last(|i| i.read.push(("size", format!("{size}"))));
                reg.tag(id, &icon);
            }
        });
    });
}

/// Icons (the icon buttons their first row), a tab bar, the expander, a table.
fn column_5(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    heading(reg, ui, "basic.icons.heading", "Icons");
    icon_buttons(reg, atlas, ui, chosen);
    icons(reg, t, ui, chosen);

    tabs(reg, state, t, ui);

    // The companion crate's expander (docs/todo_egui-widgets-spec.md §4.10): the arrow in
    // `expander.arrow_color` at `expander.arrow_icon_size` on `expander.arrow_side`,
    // `arrow_gap` from the title, the body `content_indent` in, framed as `frame_enabled`
    // states; the header `expander.header_height` tall (the expander scope's
    // `interact_size.y`).
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
                let out = Expander::new(demo::lined(ui, title, egui::TextStyle::Button))
                    .id_salt(("basic/expander", title))
                    .default_open(open)
                    .show(ui, |ui| {
                        ui.label(demo::lined(ui, body, egui::TextStyle::Body))
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

    heading(reg, ui, "basic.table.heading", "Table");
    table(reg, t, ui);
}

/// The Card group: the card surface's frame (`card.background_color`, `card.border.*`),
/// `BASIC_WIDE` across its border.
fn card(reg: &mut Registry, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;
    heading(reg, ui, "basic.card.heading", "Card");
    // A side of the card's padding the theme leaves unstated is the container margin, inside
    // its border (R11, D-card: KDE and macOS state no card padding, the card pads by the
    // container margin), where the surface's frame would keep egui's own.
    let margin = atlas.layout().container_margin;
    let b = &t.card.border;
    let pad_unstated = |frame: &mut egui::Frame| {
        let Some(margin) = margin else {
            return;
        };
        let side = |stated: Option<f32>| stated.is_none().then_some(margin + b.line_width);
        let unstated = native_theme::theme::ResolvedPadding {
            top: side(b.padding.top),
            right: side(b.padding.right),
            bottom: side(b.padding.bottom),
            left: side(b.padding.left),
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
                    ui.label(demo::lined(ui, "Card content", egui::TextStyle::Body))
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

/// A list row's height and its text's inset: `list.row_height` where the theme states it, else
/// as tall as its content — one line of the item font, `defaults.line_height` of its size, and
/// `list.border.padding` above and below it, the platform's "sizes to content"
/// (docs/platform-facts.md §2.15) — its text `list.border.padding.left` in. A side the theme
/// leaves unstated is `own`'s: for the list what egui gives a selectable row there, the scope's
/// `button_padding` (`egui/src/widgets/button.rs:333-337`); for the table nothing, as the three
/// showcases lay their tables out.
fn row_metrics(
    t: &native_theme_egui::ResolvedTheme,
    font: &egui::FontId,
    own: egui::Vec2,
) -> (f32, f32, f32) {
    let l = &t.list;
    let padding = &l.border.padding;
    let line = font.size * t.defaults.line_height;
    let content = line + padding.top.unwrap_or(own.y) + padding.bottom.unwrap_or(own.y);
    // On egui's layout grid (`emath::GUI_ROUNDING`), as `allocate_exact_size` lays each row out,
    // so `n` rows are `n` row heights tall.
    (
        egui::emath::GuiRounding::round_ui(l.row_height.unwrap_or(content)),
        padding.left.unwrap_or(own.x),
        line,
    )
}

/// The list: `LIST_ROWS` selectable rows in a `ScrollArea` `LIST_VISIBLE` rows tall, framed by
/// `list_frame`, in one `Role::List` scope (its selection colours and item font). egui has no
/// list widget, and the leaves egui never reads from the style are per call (connector spec
/// §5.4, T18(a)): each row is painted here, `row_metrics` tall and inset, filled with
/// `list.selection_background` in `list.selection_text_color` while selected and with
/// `list.hover_background` in `list.hover_text_color` while hovered.
fn list(
    reg: &mut Registry,
    state: &mut DemoState,
    t: &native_theme_egui::ResolvedTheme,
    ui: &mut egui::Ui,
) {
    let l = &t.list;
    let mut rows_drawn: Vec<egui::Response> = Vec::new();
    let mut bar = None;
    let frame_response = demo::scoped_container(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "List",
        |ui, rows, reg| {
            let font = egui::TextStyle::Body.resolve(ui.style());
            let (row_height, inset, line) = row_metrics(t, &font, ui.spacing().button_padding);
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
                    let area = egui::ScrollArea::vertical()
                        .id_salt("basic/list")
                        .auto_shrink([false, false])
                        .min_scrolled_height(visible)
                        .max_height(visible)
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for i in 0..LIST_ROWS {
                                let text = format!("Item {}", i + 1);
                                let selected = state.basic_list == i;
                                let r = rows.add(reg, ui, "List row", |ui| {
                                    list_row(
                                        ui,
                                        l,
                                        &text,
                                        selected,
                                        (row_height, inset, line),
                                        &font,
                                    )
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
    reg.amend_last(|i| {
        i.read
            .push(("list.row_height", format!("{:?}", l.row_height)));
        i.read
            .push(("list.border.padding", format!("{:?}", l.border.padding)));
    });
    reg.tag("basic.list.frame", &frame_response);
    for (i, row) in rows_drawn.iter().take(LIST_VISIBLE).enumerate() {
        reg.place(ui, &format!("basic.list.row_{}", i + 1), row.rect);
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
    let offset = area.state.offset.y;
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
/// centred vertically.
fn list_row(
    ui: &mut egui::Ui,
    l: &native_theme::theme::ResolvedListTheme,
    text: &str,
    selected: bool,
    (height, inset, line): (f32, f32, f32),
    font: &egui::FontId,
) -> egui::Response {
    let size = egui::vec2(ui.available_width(), height);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::SelectableLabel, true, selected, text)
    });
    let (fill, colour) = if selected {
        (Some(l.selection_background), l.selection_text_color)
    } else if response.hovered() {
        (Some(l.hover_background), l.hover_text_color)
    } else {
        (None, l.item_font.color)
    };
    if let Some(fill) = fill {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::ZERO, to_color32(fill));
    }
    paint_cell_text(ui, text, font, to_color32(colour), (rect, inset, line));
    response
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

/// The table: a header and two rows, the second selected, `BASIC_WIDE` across the list's
/// frame (`list_frame`), the two columns half the width inside it each, in one `Role::List`
/// scope. egui has no table the theme reaches (egui_extras' `TableBuilder` paints only its
/// stripes), so it is painted here as the list's rows are: the header in
/// `list.header_background` and `list.header_font`, the rows `row_metrics` tall in
/// `list.item_font`, the selected one in `list.selection_background` and
/// `list.selection_text_color`; a vertical
/// `list.grid_color` line between the columns through the header and the rows and a horizontal
/// one under the header, each `separator.line_width` wide, inside the cell it closes.
fn table(reg: &mut Registry, t: &native_theme_egui::ResolvedTheme, ui: &mut egui::Ui) {
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
                l.header_font.size,
                demo::weighted_family(ui, l.header_font.weight, t.defaults.font.weight),
            );
            let (row_height, inset, line) = row_metrics(t, &item_font, egui::Vec2::ZERO);
            let (header_height, _, header_line) = row_metrics(t, &header_font, egui::Vec2::ZERO);
            let grid = to_color32(l.grid_color);
            let grid_width = t.separator.line_width;
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
                        egui::CornerRadius::ZERO,
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
                            (Some(l.selection_background), l.selection_text_color)
                        } else {
                            (None, l.item_font.color)
                        };
                        if let Some(fill) = fill {
                            painter.rect_filled(row, egui::CornerRadius::ZERO, to_color32(fill));
                        }
                        for (j, text) in cells.iter().enumerate() {
                            let cell = if j == 0 {
                                egui::Rect::from_x_y_ranges(row.left()..=mid, row.y_range())
                            } else {
                                egui::Rect::from_x_y_ranges(mid..=row.right(), row.y_range())
                            };
                            paint_cell_text(
                                ui,
                                text,
                                &item_font,
                                to_color32(colour),
                                (cell, inset, line),
                            );
                        }
                        places.push((format!("basic.table.row_{}", i + 1), row));
                    }
                    // The grid: under the header, and between the columns from top to bottom,
                    // each inside the cell it closes.
                    let stroke = egui::Stroke::new(grid_width, grid);
                    painter.hline(header.x_range(), header.bottom() - 0.5 * grid_width, stroke);
                    painter.vline(mid - 0.5 * grid_width, rect.y_range(), stroke);
                })
                .response
        },
    );
    reg.tag("basic.table.frame", &frame_response);
    for (id, rect) in places {
        reg.place(ui, &id, rect);
    }
}
