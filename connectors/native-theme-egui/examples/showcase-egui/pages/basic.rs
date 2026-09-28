//! Basic: the controls the three showcases all draw, in the same order, with the same labels,
//! values and states, packed onto one screen in four columns, so the gpui, iced and egui
//! captures compare control by control. Each control goes through the seam its palette page
//! gives it; what egui leaves to the call site is applied per call from the theme.

use egui::Button;
use native_theme_egui::convert::{to_color32, to_corner_radius, to_stroke};
use native_theme_egui::{Role, RoleVariant, Surface, ThemeAtlas, expander_icon, input_frame};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

/// The width of the Basic page's text fields, drop-down, slider and progress bar. The model
/// states no such width; it is the Basic page's own, the gpui showcase's and the iced
/// showcase's `BASIC_WIDTH` too, so the three pages lay the same controls out alike.
pub(crate) const BASIC_WIDTH: f32 = 140.0;

/// The width of the Basic page's text area, list, expander, card and separator: the Basic
/// page's own, as `BASIC_WIDTH`, and the gpui and iced showcases' `BASIC_WIDE`.
pub(crate) const BASIC_WIDE: f32 = 200.0;

/// The page's columns: the Basic page's layout (a datum of the page, not a style value).
const COLUMNS: usize = 4;

/// The drop-down's rows.
const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

/// The progress bar's value: the datum on display, not a style value.
const PROGRESS: f32 = 0.4;

/// The tab bar's tabs, and the segmented control's segments.
const TABS: [&str; 3] = ["One", "Two", "Three"];
const SEGMENTS: [&str; 3] = ["Day", "Week", "Month"];

/// The list's rows, and how many of them it shows: data of the page.
const LIST_ROWS: usize = 8;
const LIST_VISIBLE: usize = 4;

/// The text area's text, three lines.
pub(crate) const AREA_TEXT: &str = "Line one\nLine two\nLine three";
const AREA_ROWS: usize = 3;

/// The groups of the page, column by column, in order (BASIC2's), which the tests check the
/// page against.
#[cfg(test)]
pub(crate) const GROUPS: [&[&str]; COLUMNS] = [
    &["Buttons", "Checkboxes", "Radio buttons", "Switches"],
    &["Text inputs", "Text area", "Drop-down", "Text"],
    &[
        "Slider",
        "Progress bar",
        "Spinner",
        "Tabs",
        "Segmented control",
    ],
    &["List", "Expander", "Card", "Separator"],
];

/// Four columns of equal width, `layout.section_gap` apart (the page's gap where the theme
/// states none: egui's `item_spacing`), each a stack of groups top-aligned: each column is laid
/// out in a `Ui` of its own at its place, so a group wider than its column (a preset's tab
/// minimum width) shows as the overflow it is, and the next column does not move.
pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let gap = atlas
        .layout()
        .section_gap
        .unwrap_or(ui.spacing().item_spacing.x);
    let origin = ui.cursor().min;
    let columns = COLUMNS as f32;
    let width = ((ui.available_width() - gap * (columns - 1.0)) / columns).max(0.0);
    let mut used = egui::Rect::NOTHING;
    for column in 0..COLUMNS {
        let left = origin.x + column as f32 * (width + gap);
        let rect = egui::Rect::from_min_size(
            egui::pos2(left, origin.y),
            egui::vec2(width, ui.available_height()),
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
            _ => column_4(reg, state, atlas, &mut child),
        }
        used = used.union(child.min_rect());
    }
    if used.is_positive() {
        ui.advance_cursor_after_rect(used);
    }
}

/// Buttons, check boxes, radio buttons, switches.
fn column_1(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;
    // `button.min_width`, which egui's `Button` never reads from the style: it raises only its
    // height, to `interact_size.y` (`button.min_height` in the button scope), so the width is
    // the application's, per call (connector spec §5.3, `Button::min_size`).
    let button_min = egui::vec2(t.button.min_width, t.button.min_height);
    let read_min = |i: &mut demo::InstanceInfo| {
        i.read.push(("button.min_width", button_min.x.to_string()));
        i.read.push(("button.min_height", button_min.y.to_string()));
    };

    caption(reg, ui, "Buttons");
    ui.horizontal(|ui| {
        demo::scoped(reg, ui, Role::Button, normal, "button (enabled)", |ui| {
            ui.add(Button::new("Button").min_size(button_min))
        });
        reg.amend_last(read_min);
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (suggested action)",
            |ui| ui.add(Button::new("Primary").selected(true).min_size(button_min)),
        );
        reg.amend_last(read_min);
    });
    ui.horizontal(|ui| {
        demo::scoped(
            reg,
            ui,
            Role::Button,
            RoleVariant::Disabled,
            "button (disabled)",
            |ui| ui.add_enabled(false, Button::new("Disabled").min_size(button_min)),
        );
        reg.amend_last(read_min);
        let owner = demo::scoped(reg, ui, Role::Button, normal, "tooltip button", |ui| {
            ui.add(Button::new("Tooltip").min_size(button_min))
        });
        reg.amend_last(read_min);
        demo::surfaced(
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
                    demo::scoped(reg, ui, Role::Tooltip, normal, "tooltip text", |ui| {
                        ui.label("A tooltip")
                    });
                })
                .map(|out| out.response)
            },
        );
    });

    // Each control shows one state and is held in it: a click changes a copy made for the pass.
    // One per row, each in its own checkbox scope, whose `interact_size.y` is the indicator.
    caption(reg, ui, "Checkboxes");
    for (label, checked, variant, kind) in [
        ("Unchecked", false, normal, "checkbox (unchecked)"),
        ("Checked", true, RoleVariant::Selected, "checkbox (checked)"),
        (
            "Disabled",
            true,
            RoleVariant::Disabled,
            "checkbox (disabled)",
        ),
    ] {
        let mut value = checked;
        let enabled = variant != RoleVariant::Disabled;
        demo::scoped(reg, ui, Role::Checkbox, variant, kind, |ui| {
            ui.add_enabled(enabled, egui::Checkbox::new(&mut value, label))
        });
    }

    // The selected radio button is drawn in `RoleVariant::Selected`, as on the Selection page.
    caption(reg, ui, "Radio buttons");
    for (i, label) in ["Option A", "Option B"].into_iter().enumerate() {
        let selected = state.basic_radio == i;
        let variant = if selected {
            RoleVariant::Selected
        } else {
            normal
        };
        let r = demo::scoped(reg, ui, Role::Checkbox, variant, "RadioButton", |ui| {
            ui.add(egui::RadioButton::new(selected, label))
        });
        if r.clicked() {
            state.basic_radio = i;
        }
    }

    // The connector's switch (connector spec §5.3, `Role::Switch`): a button whose selected flag
    // is the switch's state, its track colours as the button's fill, `switch.track_height` its
    // height and `switch.track_width`, which egui never reads from the style, its minimum width
    // per call. egui has no switch widget: its thumb is the planned widgets crate's
    // (docs/todo_egui-widgets-spec.md §4.1), so the label sits in the track.
    caption(reg, ui, "Switches");
    let track = egui::vec2(t.switch.track_width, t.switch.track_height);
    for (label, on, enabled, kind) in [
        ("Off", false, true, "switch (off)"),
        ("On", true, true, "switch (on)"),
        ("Disabled", true, false, "switch (disabled)"),
    ] {
        let variant = if enabled {
            normal
        } else {
            RoleVariant::Disabled
        };
        demo::scoped(reg, ui, Role::Switch, variant, kind, |ui| {
            ui.add_enabled(enabled, Button::new(label).selected(on).min_size(track))
        });
        reg.amend_last(|i| {
            i.read
                .push(("switch.track_width", t.switch.track_width.to_string()));
        });
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

/// Text inputs, the text area, the drop-down, text.
fn column_2(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;
    let read_min = |i: &mut demo::InstanceInfo| {
        i.read
            .push(("input.min_height", t.input.min_height.to_string()));
    };

    caption(reg, ui, "Text inputs");
    let id = ui.make_persistent_id("basic/placeholder");
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (hint)", |ui| {
        let edit = field(&mut state.basic_hint, id, ui, t).hint_text("Placeholder");
        ui.add(edit)
    });
    reg.amend_last(|i| {
        read_min(i);
        i.notes.push(("hint text", "\"Placeholder\"".to_string()));
    });
    let id = ui.make_persistent_id("basic/filled");
    demo::scoped(
        reg,
        ui,
        Role::Input,
        normal,
        "TextEdit (single line)",
        |ui| {
            let edit = field(&mut state.basic_text, id, ui, t);
            ui.add(edit)
        },
    );
    reg.amend_last(read_min);
    let id = ui.make_persistent_id("basic/disabled");
    let mut disabled = "Disabled".to_string();
    demo::scoped(
        reg,
        ui,
        Role::Input,
        RoleVariant::Disabled,
        "TextEdit (disabled)",
        |ui| {
            let edit = field(&mut disabled, id, ui, t);
            ui.add_enabled(false, edit)
        },
    );
    reg.amend_last(read_min);

    caption(reg, ui, "Text area");
    let id = ui.make_persistent_id("basic/area");
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (multiline)", |ui| {
        ui.add(
            egui::TextEdit::multiline(&mut state.basic_area)
                .desired_width(BASIC_WIDE)
                .desired_rows(AREA_ROWS)
                .id(id)
                .frame(input_frame(ui, id, t)),
        )
    });

    caption(reg, ui, "Drop-down");
    let current = FRUITS.get(state.basic_combo).copied().unwrap_or_default();
    demo::scoped_popup(
        reg,
        ui,
        Role::ComboBox,
        normal,
        "ComboBox",
        |ui, modifier, row, reg| {
            let mut combo = egui::ComboBox::from_id_salt("basic/combo")
                .selected_text(current)
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

    caption(reg, ui, "Text");
    demo::base(reg, ui, "Label (body text)", |ui| ui.label("Body text"));
    // `link.underline_enabled`, which egui's `Link` never reads: it underlines only on hover
    // or focus (`egui/src/widgets/hyperlink.rs:50-54`). An underline in the text's own format
    // is painted at rest, in the text's colour, so the text takes `link.font.color` too, the
    // `hyperlink_color` of the link scope (connector spec §5.3).
    let mut text = egui::RichText::new("Link").color(to_color32(t.link.font.color));
    if t.link.underline_enabled {
        text = text.underline();
    }
    demo::scoped(reg, ui, Role::Link, normal, "Link", |ui| {
        ui.add(egui::Link::new(text))
    });
    reg.amend_last(|i| {
        i.read.push((
            "link.underline_enabled",
            t.link.underline_enabled.to_string(),
        ));
        i.read
            .push(("link.font.color", format!("{:?}", t.link.font.color)));
    });
}

/// The slider, the progress bar, the spinner, a tab bar, the segmented control.
fn column_3(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    caption(reg, ui, "Slider");
    demo::scoped(reg, ui, Role::Slider, normal, "Slider (horizontal)", |ui| {
        ui.spacing_mut().slider_width = BASIC_WIDTH;
        ui.add(egui::Slider::new(&mut state.basic_slider, 0.0..=100.0).show_value(false))
    });
    reg.amend_last(|i| i.notes.push(("range", "0 to 100".to_string())));

    // `progress_bar.border.corner_radius`, which `ProgressBar` takes per call (connector spec
    // §5.5).
    caption(reg, ui, "Progress bar");
    let radius = to_corner_radius(
        egui::CornerRadius::default(),
        t.progress_bar.border.corner_radius,
    );
    demo::scoped(reg, ui, Role::ProgressBar, normal, "ProgressBar", |ui| {
        ui.add(
            egui::ProgressBar::new(PROGRESS)
                .desired_width(BASIC_WIDTH)
                .corner_radius(radius),
        )
    });
    reg.amend_last(|i| {
        i.read.push((
            "progress_bar.border.corner_radius",
            t.progress_bar.border.corner_radius.to_string(),
        ));
    });

    caption(reg, ui, "Spinner");
    demo::scoped(reg, ui, Role::Spinner, normal, "Spinner", |ui| {
        ui.add(egui::Spinner::new())
    });

    caption(reg, ui, "Tabs");
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
        },
        |_, _, _| {},
    );
    if let Some(tab) = picked {
        state.basic_tab = tab;
    }

    // As the Buttons page's (connector spec §5.3): one row in one `Role::SegmentedControl`
    // scope, whose cell carries the segment height, padding and colours and puts
    // `separator_width` between the segments.
    caption(reg, ui, "Segmented control");
    demo::scoped_container(
        reg,
        ui,
        Role::SegmentedControl,
        normal,
        "segmented control",
        |ui, segment, reg| {
            ui.horizontal(|ui| {
                for (i, label) in SEGMENTS.into_iter().enumerate() {
                    let selected = i == state.basic_segment;
                    if segment
                        .add(reg, ui, "segment", |ui| {
                            ui.add(Button::new(label).selected(selected))
                        })
                        .clicked()
                    {
                        state.basic_segment = i;
                    }
                }
            })
            .response
        },
    );
}

/// The list, the expander, a card, a separator.
fn column_4(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    caption(reg, ui, "List");
    list(reg, state, t, ui);

    // The expander's arrow in `expander.arrow_color` at `expander.arrow_icon_size`
    // (`expander_icon`), its header `expander.header_height` tall (the expander scope's
    // `interact_size.y`).
    caption(reg, ui, "Expander");
    ui.scope(|ui| {
        ui.set_max_width(BASIC_WIDE);
        for (title, body, open, kind) in [
            (
                "Details",
                "Expanded content",
                true,
                "CollapsingHeader (expanded)",
            ),
            (
                "More",
                "More content",
                false,
                "CollapsingHeader (collapsed)",
            ),
        ] {
            let mut shown = None;
            demo::scoped(reg, ui, Role::Expander, normal, kind, |ui| {
                let out = egui::CollapsingHeader::new(title)
                    .id_salt(("basic/expander", title))
                    .default_open(open)
                    .icon(expander_icon(t))
                    .show(ui, |ui| ui.label(body));
                shown = out.body_returned;
                out.header_response
            });
            // The body's label, laid out in the expander's scope, recorded once it is drawn.
            if let Some(label) = shown {
                reg.record(
                    &label,
                    demo::info(
                        "expander body",
                        vec![demo::Seam::Role(Role::Expander, normal)],
                    ),
                    false,
                );
            }
        }
    });

    // The card surface's frame (`card.background_color`, `card.border.*`), `BASIC_WIDE` across
    // its border.
    caption(reg, ui, "Card");
    ui.scope(|ui| {
        ui.set_max_width(BASIC_WIDE);
        demo::framed(
            reg,
            ui,
            Surface::Card,
            Some((Role::Card, normal)),
            "card",
            |ui, reg| {
                // The room inside the frame: `BASIC_WIDE` less its margins and border.
                ui.set_min_width(ui.available_width());
                demo::scoped(reg, ui, Role::Card, normal, "card label", |ui| {
                    ui.label("Card content")
                });
            },
        );
    });

    caption(reg, ui, "Separator");
    ui.scope(|ui| {
        ui.set_max_width(BASIC_WIDE);
        demo::scoped(
            reg,
            ui,
            Role::Separator,
            normal,
            "Separator (horizontal)",
            |ui| ui.add(egui::Separator::default().horizontal()),
        );
    });
}

/// The list: `LIST_ROWS` selectable rows in a `ScrollArea` `LIST_VISIBLE` rows tall, framed in
/// `list.background_color` and `list.border`'s colour, width and radius, in one `Role::List`
/// scope (its selection colours and item font). egui has no list widget, and the leaves egui
/// never reads from the style are per call (connector spec §5.4, T18(a)): each row is painted
/// here, `list.row_height` tall where the theme states it and else as tall as its content —
/// one line of the item font and `list.border.padding` above and below it, the platform's "sizes
/// to content" (docs/platform-facts.md §2.15) — its text `list.border.padding.left` in, filled
/// with `list.selection_background` in `list.selection_text_color` while selected and with
/// `list.hover_background` in `list.hover_text_color` while hovered.
fn list(
    reg: &mut Registry,
    state: &mut DemoState,
    t: &native_theme_egui::ResolvedTheme,
    ui: &mut egui::Ui,
) {
    let l = &t.list;
    demo::scoped_container(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "List",
        |ui, rows, reg| {
            let font = egui::TextStyle::Body.resolve(ui.style());
            let padding = &l.border.padding;
            let content = ui.fonts_mut(|f| f.row_height(&font))
                + padding.top.unwrap_or_default()
                + padding.bottom.unwrap_or_default();
            let row_height = l.row_height.unwrap_or(content);
            let inset = padding.left.unwrap_or_default();
            let line = to_stroke(egui::Stroke::NONE, l.border.color, l.border.line_width);
            let frame = egui::Frame::NONE
                .fill(to_color32(l.background_color))
                .stroke(line)
                .corner_radius(to_corner_radius(
                    egui::CornerRadius::default(),
                    l.border.corner_radius,
                ));
            let inner = (BASIC_WIDE - frame.total_margin().sum().x).max(0.0);
            let visible = row_height * LIST_VISIBLE as f32;
            frame
                .show(ui, |ui| {
                    ui.set_width(inner);
                    egui::ScrollArea::vertical()
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
                                    list_row(ui, l, &text, selected, (row_height, inset), &font)
                                });
                                if r.clicked() {
                                    state.basic_list = i;
                                }
                            }
                        });
                })
                .response
        },
    );
    reg.amend_last(|i| {
        i.read
            .push(("list.row_height", format!("{:?}", l.row_height)));
        i.read
            .push(("list.background_color", l.background_color.to_string()));
        i.read
            .push(("list.border.color", l.border.color.to_string()));
        i.read
            .push(("list.border.padding", format!("{:?}", l.border.padding)));
        i.read
            .push(("list.hover_text_color", format!("{:?}", l.hover_text_color)));
    });
}

/// One list row across the list, `height` tall, its text `inset` in and centred vertically.
fn list_row(
    ui: &mut egui::Ui,
    l: &native_theme::theme::ResolvedListTheme,
    text: &str,
    selected: bool,
    (height, inset): (f32, f32),
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
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_string(), font.clone(), to_color32(colour));
    let at = egui::pos2(rect.left() + inset, rect.center().y - galley.size().y / 2.0);
    ui.painter().galley(at, galley, to_color32(colour));
    response
}
