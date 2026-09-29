//! The elements the three showcases share, read from
//! `docs/showcase-elements.toml`, and the layout dump that reports where
//! this showcase drew each of them (`--dump-layout`).
//!
//! The list is the one place those elements are named: Widget Info shows an
//! element's name, state and leaves from it, and
//! `scripts/check_showcase_parity.py` compares the gpui, iced and egui dumps
//! and captures element by element against it.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use gpui::{
    App, Bounds, Div, Entity, IntoElement, ParentElement as _, Pixels, Styled as _, Window, canvas,
    div,
};

#[cfg(feature = "widgets")]
use native_theme_gpui::widgets::{Part, PartBounds};

use crate::info::InfoRegistry;

/// One element of `docs/showcase-elements.toml`, as the list's header
/// gives the three showcases to read it: every field of the schema, though
/// the running showcase reads only some (`parent` is the comparator's, `when`
/// the tests').
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ShowcaseElement {
    pub id: String,
    pub name: String,
    pub parent: String,
    pub leaves: Vec<String>,
    pub states: Vec<String>,
    pub part: bool,
    pub when: Option<String>,
}

impl ShowcaseElement {
    /// The Widget Info title: the name, then the state or variant shown.
    pub fn title(&self) -> String {
        match self.states.first() {
            Some(state) => format!("{} · {state}", self.name),
            None => self.name.clone(),
        }
    }
}

const SHOWCASE_ELEMENTS: &str = include_str!("../../../../docs/showcase-elements.toml");

pub fn showcase_elements() -> Result<Vec<ShowcaseElement>, String> {
    let table: toml::Table = toml::from_str(SHOWCASE_ELEMENTS).map_err(|e| e.to_string())?;
    let rows = table
        .get("element")
        .and_then(toml::Value::as_array)
        .ok_or("docs/showcase-elements.toml: no [[element]] entries")?;
    rows.iter()
        .map(|row| {
            let text = |key: &str| {
                row.get(key)
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| format!("an element has no `{key}`"))
            };
            let list = |key: &str| {
                row.get(key)
                    .and_then(toml::Value::as_array)
                    .ok_or_else(|| format!("an element has no `{key}`"))?
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .map(str::to_owned)
                            .ok_or_else(|| format!("`{key}` holds a value that is not a string"))
                    })
                    .collect::<Result<Vec<_>, _>>()
            };
            Ok(ShowcaseElement {
                id: text("id")?,
                name: text("name")?,
                parent: text("parent")?,
                leaves: list("leaves")?,
                states: list("states")?,
                part: row
                    .get("part")
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(false),
                when: row
                    .get("when")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            })
        })
        .collect()
}

/// The list, parsed once. A list that does not parse is reported once on
/// stderr and taken as empty: Widget Info then falls back to each widget's
/// own info, and the dump holds nothing, which the comparator reports.
/// `the_element_list_parses` holds the committed list to parsing.
pub fn elements() -> &'static [ShowcaseElement] {
    static LIST: OnceLock<Vec<ShowcaseElement>> = OnceLock::new();
    LIST.get_or_init(|| {
        showcase_elements().unwrap_or_else(|error| {
            eprintln!("docs/showcase-elements.toml: {error}");
            Vec::new()
        })
    })
}

/// The element of the list with `id`.
pub fn element(id: &str) -> Option<&'static ShowcaseElement> {
    elements().iter().find(|e| e.id == id)
}

/// The elements of the list that a widget reporting itself under an info id
/// (`InfoExt::info`) is, as (info id, element id): the widget's info is then
/// shown in the list's form and its bounds are dumped under the element's id.
/// `every_listed_id_is_in_the_list` holds the second column to the list.
const LISTED: &[(&str, &str)] = &[
    ("chrome-menu-bar", "chrome.menu_bar"),
    ("chrome-toolbar", "chrome.toolbar"),
    (
        "chrome-button-toolbar-command-palette",
        "chrome.toolbar.command_palette",
    ),
    (
        "chrome-button-toolbar-reload-theme",
        "chrome.toolbar.reload_theme",
    ),
    (
        "chrome-button-toolbar-preferences",
        "chrome.toolbar.preferences",
    ),
    ("chrome-side-panel", "chrome.side_panel"),
    ("chrome-theme-settings", "chrome.side_panel.settings"),
    (
        "chrome-label-theme",
        "chrome.side_panel.settings.theme_label",
    ),
    ("chrome-settings-preset", "chrome.side_panel.settings.theme"),
    ("chrome-label-mode", "chrome.side_panel.settings.mode_label"),
    (
        "chrome-settings-color-mode",
        "chrome.side_panel.settings.mode",
    ),
    (
        "chrome-label-icon-theme",
        "chrome.side_panel.settings.icon_theme_label",
    ),
    (
        "chrome-settings-icon-theme",
        "chrome.side_panel.settings.icon_theme",
    ),
    ("chrome-side-panel-separator", "chrome.side_panel.separator"),
    ("chrome-inspector-tabs", "chrome.side_panel.inspector_tabs"),
    ("chrome-page-tabs", "chrome.page_tabs"),
    ("chrome-status-bar", "chrome.status_bar"),
    (
        "chrome-button-status-toggle-side-panel",
        "chrome.status_bar.toggle",
    ),
    ("basic-heading-buttons", "basic.buttons.heading"),
    ("basic-button", "basic.buttons.default"),
    ("basic-button-primary", "basic.buttons.primary"),
    ("basic-button-disabled", "basic.buttons.disabled"),
    ("basic-button-tooltip", "basic.buttons.tooltip"),
    ("basic-heading-checkboxes", "basic.checkboxes.heading"),
    ("basic-checkbox-unchecked", "basic.checkboxes.unchecked"),
    ("basic-checkbox-checked", "basic.checkboxes.checked"),
    ("basic-checkbox-disabled", "basic.checkboxes.disabled"),
    ("basic-heading-radio", "basic.radios.heading"),
    ("basic-radio-0", "basic.radios.option_a"),
    ("basic-radio-1", "basic.radios.option_b"),
    ("basic-heading-switches", "basic.switches.heading"),
    ("basic-switch-off", "basic.switches.off"),
    ("basic-switch-on", "basic.switches.on"),
    ("basic-switch-disabled", "basic.switches.disabled"),
    ("basic-heading-toggle", "basic.toggle_buttons.heading"),
    ("basic-toggle-off", "basic.toggle_buttons.off"),
    ("basic-toggle-on", "basic.toggle_buttons.on"),
    ("basic-heading-icon-buttons", "basic.icon_buttons.heading"),
    ("basic-icon-copy", "basic.icon_buttons.copy"),
    ("basic-icon-paste", "basic.icon_buttons.paste"),
    ("basic-icon-delete", "basic.icon_buttons.delete"),
    ("basic-heading-inputs", "basic.text_inputs.heading"),
    ("basic-input-placeholder", "basic.text_inputs.placeholder"),
    ("basic-input-filled", "basic.text_inputs.filled"),
    ("basic-input-disabled", "basic.text_inputs.disabled"),
    ("basic-heading-textarea", "basic.text_area.heading"),
    ("basic-textarea", "basic.text_area.field"),
    ("basic-heading-select", "basic.drop_down.heading"),
    ("basic-select", "basic.drop_down.trigger"),
    ("basic-heading-text", "basic.text.heading"),
    ("basic-body-text", "basic.text.body"),
    ("basic-link", "basic.text.link"),
    ("basic-heading-number", "basic.number_input.heading"),
    ("basic-number", "basic.number_input.field"),
    ("basic-heading-focused", "basic.focused_input.heading"),
    ("basic-input-focused", "basic.focused_input.field"),
    ("basic-heading-slider", "basic.slider.heading"),
    ("basic-slider", "basic.slider.control"),
    ("basic-heading-progress", "basic.progress_bar.heading"),
    ("basic-progress", "basic.progress_bar.bar"),
    ("basic-heading-spinner", "basic.spinner.heading"),
    ("basic-spinner", "basic.spinner.indicator"),
    ("basic-heading-tabs", "basic.tabs.heading"),
    ("basic-tabs", "basic.tabs.bar"),
    ("basic-heading-segmented", "basic.segmented.heading"),
    ("basic-segmented", "basic.segmented.control"),
    ("basic-heading-typography", "basic.typography.heading"),
    ("basic-type-caption", "basic.typography.caption"),
    ("basic-type-body", "basic.typography.body"),
    (
        "basic-type-section-heading",
        "basic.typography.section_heading",
    ),
    ("basic-type-dialog-title", "basic.typography.dialog_title"),
    ("basic-type-display", "basic.typography.display"),
    ("basic-type-monospace", "basic.typography.monospace"),
    ("basic-heading-list", "basic.list.heading"),
    ("basic-list", "basic.list.frame"),
    ("basic-list-row-0", "basic.list.row_1"),
    ("basic-list-row-1", "basic.list.row_2"),
    ("basic-list-row-2", "basic.list.row_3"),
    ("basic-list-row-3", "basic.list.row_4"),
    ("basic-heading-expander", "basic.expander.heading"),
    ("basic-expander-body", "basic.expander.details.body"),
    ("basic-heading-card", "basic.card.heading"),
    ("basic-card", "basic.card.frame"),
    ("basic-card-text", "basic.card.text"),
    ("basic-heading-separator", "basic.separator.heading"),
    ("basic-separator", "basic.separator.line"),
    ("basic-heading-table", "basic.table.heading"),
    ("basic-table", "basic.table.frame"),
    ("basic-table-header", "basic.table.header"),
    ("basic-table-row-0", "basic.table.row_1"),
    ("basic-table-row-1", "basic.table.row_2"),
    ("basic-table-row-2", "basic.table.row_3"),
    ("basic-heading-icons", "basic.icons.heading"),
    ("basic-icon-small", "basic.icons.small"),
    ("basic-icon-toolbar", "basic.icons.toolbar"),
    ("basic-icon-large", "basic.icons.large"),
];

/// The parts of a theme-drawn control (`native_theme_gpui::widgets`) that
/// are elements of the list, by the control's info id, as (part, element
/// id).
#[cfg(feature = "widgets")]
type ControlParts = (&'static str, &'static [(Part, &'static str)]);

#[cfg(feature = "widgets")]
const PARTS: &[ControlParts] = &[
    (
        "basic-checkbox-unchecked",
        &[
            (Part::Indicator, "basic.checkboxes.unchecked.indicator"),
            (Part::Label, "basic.checkboxes.unchecked.label"),
        ],
    ),
    (
        "basic-checkbox-checked",
        &[
            (Part::Indicator, "basic.checkboxes.checked.indicator"),
            (Part::Mark, "basic.checkboxes.checked.mark"),
            (Part::Label, "basic.checkboxes.checked.label"),
        ],
    ),
    (
        "basic-checkbox-disabled",
        &[
            (Part::Indicator, "basic.checkboxes.disabled.indicator"),
            (Part::Mark, "basic.checkboxes.disabled.mark"),
            (Part::Label, "basic.checkboxes.disabled.label"),
        ],
    ),
    (
        "basic-radio-0",
        &[
            (Part::Indicator, "basic.radios.option_a.indicator"),
            (Part::Mark, "basic.radios.option_a.dot"),
            (Part::Label, "basic.radios.option_a.label"),
        ],
    ),
    (
        "basic-radio-1",
        &[
            (Part::Indicator, "basic.radios.option_b.indicator"),
            (Part::Label, "basic.radios.option_b.label"),
        ],
    ),
    (
        "basic-switch-off",
        &[
            (Part::Track, "basic.switches.off.track"),
            (Part::Thumb, "basic.switches.off.thumb"),
            (Part::Label, "basic.switches.off.label"),
        ],
    ),
    (
        "basic-switch-on",
        &[
            (Part::Track, "basic.switches.on.track"),
            (Part::Thumb, "basic.switches.on.thumb"),
            (Part::Label, "basic.switches.on.label"),
        ],
    ),
    (
        "basic-switch-disabled",
        &[
            (Part::Track, "basic.switches.disabled.track"),
            (Part::Thumb, "basic.switches.disabled.thumb"),
            (Part::Label, "basic.switches.disabled.label"),
        ],
    ),
    (
        "basic-slider",
        &[
            (Part::Track, "basic.slider.control.track"),
            (Part::Fill, "basic.slider.control.fill"),
            (Part::Thumb, "basic.slider.control.thumb"),
        ],
    ),
    (
        "basic-progress",
        &[(Part::Fill, "basic.progress_bar.bar.fill")],
    ),
    (
        "basic-tabs",
        &[
            (Part::Tab(0), "basic.tabs.one"),
            (Part::Tab(1), "basic.tabs.two"),
            (Part::Tab(2), "basic.tabs.three"),
        ],
    ),
    (
        "chrome-page-tabs",
        &[
            (Part::Tab(0), "chrome.page_tabs.basic"),
            (Part::Tab(1), "chrome.page_tabs.buttons"),
        ],
    ),
    (
        "chrome-inspector-tabs",
        &[
            (Part::Tab(0), "chrome.side_panel.inspector_tabs.widget"),
            (Part::Tab(1), "chrome.side_panel.inspector_tabs.theme"),
        ],
    ),
];

/// What records the parts of the control reporting itself as `info_id` as
/// elements of the list ([`PARTS`]), or `None` where none of its parts is
/// one.
#[cfg(feature = "widgets")]
pub fn part_observer(ui: &Entity<InfoRegistry>, info_id: &str) -> Option<PartBounds> {
    let &(_, parts) = PARTS.iter().find(|(id, _)| *id == info_id)?;
    let ui = ui.clone();
    Some(std::rc::Rc::new(move |part, bounds, cx: &mut App| {
        if let Some(&(_, id)) = parts.iter().find(|(p, _)| *p == part) {
            ui.update(cx, |r, _| r.record_layout(id, bounds));
        }
    }))
}

/// The Buttons whose labels are elements of the list, as (Button id,
/// label's element id).
const LABELS: &[(&str, &str)] = &[
    ("basic-button", "basic.buttons.default.label"),
    ("basic-button-primary", "basic.buttons.primary.label"),
    ("basic-button-disabled", "basic.buttons.disabled.label"),
    ("basic-button-tooltip", "basic.buttons.tooltip.label"),
    ("basic-toggle-off", "basic.toggle_buttons.off.label"),
    ("basic-toggle-on", "basic.toggle_buttons.on.label"),
];

/// The element of the list the label of the Button `button_id` is.
pub fn label_of(button_id: &str) -> Option<&'static str> {
    LABELS
        .iter()
        .find(|(id, _)| *id == button_id)
        .map(|(_, element)| *element)
}

/// The icon-only Buttons whose icons are elements of the list, as (Button
/// id, icon's element id).
const ICONS: &[(&str, &str)] = &[
    (
        "toolbar-command-palette",
        "chrome.toolbar.command_palette.icon",
    ),
    ("toolbar-reload-theme", "chrome.toolbar.reload_theme.icon"),
    ("toolbar-preferences", "chrome.toolbar.preferences.icon"),
    ("status-toggle-side-panel", "chrome.status_bar.toggle.icon"),
];

/// The element of the list the icon of the Button `button_id` is.
pub fn icon_of(button_id: &str) -> Option<&'static str> {
    ICONS
        .iter()
        .find(|(id, _)| *id == button_id)
        .map(|(_, element)| *element)
}

/// Every element id this module's tables name, for the test that holds them
/// to the list.
#[cfg(test)]
pub fn named_ids() -> Vec<&'static str> {
    let mut ids: Vec<&'static str> = LISTED.iter().map(|(_, id)| *id).collect();
    ids.extend(LABELS.iter().map(|(_, id)| *id));
    ids.extend(ICONS.iter().map(|(_, id)| *id));
    #[cfg(feature = "widgets")]
    ids.extend(
        PARTS
            .iter()
            .flat_map(|(_, parts)| parts.iter().map(|(_, id)| *id)),
    );
    for (segment, divider) in SEGMENTS {
        ids.push(segment);
        ids.extend(divider);
    }
    ids.extend(EXPANDER_ITEMS.iter().flatten());
    ids
}

/// The element of the list the widget reporting itself as `info_id` is.
pub fn listed_for(info_id: &str) -> Option<&'static str> {
    LISTED
        .iter()
        .find(|(id, _)| *id == info_id)
        .map(|(_, element)| *element)
}

/// Records the bounds of the box it is laid out over as the element `id` of
/// the list: an absolute child pinned over its parent, so its bounds are the
/// parent's padding box. For a part inside a box the showcase builds itself,
/// with no border of its own.
pub fn record(ui: &Entity<InfoRegistry>, id: &'static str) -> impl IntoElement {
    let ui = ui.clone();
    canvas(
        move |bounds, _window: &mut Window, cx: &mut App| {
            ui.update(cx, |r, _| r.record_layout(id, bounds));
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// The segments of the Basic page's segmented control, by index, and the
/// dividing line before each after the first, as elements of the list.
const SEGMENTS: [(&str, Option<&str>); 3] = [
    ("basic.segmented.day", None),
    ("basic.segmented.week", Some("basic.segmented.divider_1")),
    ("basic.segmented.month", Some("basic.segmented.divider_2")),
];

/// The elements of the list the segment `ix` of the segmented control `id`
/// is, and the line before it, where they are ones.
pub fn segment_of(id: &str, ix: usize) -> Option<(&'static str, Option<&'static str>)> {
    (id == "basic-segmented")
        .then(|| SEGMENTS.get(ix).copied())
        .flatten()
}

/// The elements of the list an item of the Basic page's expander is, by
/// its index: the item, its header row, its arrow and its title.
const EXPANDER_ITEMS: [[&str; 4]; 2] = [
    [
        "basic.expander.details",
        "basic.expander.details.header",
        "basic.expander.details.arrow",
        "basic.expander.details.title",
    ],
    [
        "basic.expander.more",
        "basic.expander.more.header",
        "basic.expander.more.arrow",
        "basic.expander.more.title",
    ],
];

/// The elements of the list the item `ix` of the expander `id` is, as
/// (item, header, arrow, title), where it is one.
pub fn expander_item(id: &str, ix: usize) -> Option<[&'static str; 4]> {
    (id == "basic-expander")
        .then(|| EXPANDER_ITEMS.get(ix).copied())
        .flatten()
}

/// A wrapper that records the bounds of the element it holds -- its border
/// box, as gpui laid it out -- as the element `id` of the list. The wrapper
/// is a plain box: where its parent does not stretch it, it takes the
/// element's size and changes nothing of the layout.
pub trait ListedExt: IntoElement + Sized {
    fn listed(self, ui: &Entity<InfoRegistry>, id: &'static str) -> Div {
        let ui = ui.clone();
        div()
            .on_children_prepainted(move |bounds, _window, cx| {
                if let Some(bounds) = bounds.first() {
                    ui.update(cx, |r, _| r.record_layout(id, *bounds));
                }
            })
            .child(self)
    }
}

impl<E: IntoElement> ListedExt for E {}

/// The layout dump (`--dump-layout`): the showcase kind, the theme, the
/// scale and one rectangle per element of the list drawn in the last frame,
/// in logical pixels of the window's content (docs/showcase-elements.toml,
/// "The layout dump").
pub fn dump_json(
    preset: &str,
    variant: &str,
    scale: f32,
    drawn: &BTreeMap<&'static str, Bounds<Pixels>>,
) -> serde_json::Value {
    let rects: serde_json::Map<String, serde_json::Value> = drawn
        .iter()
        .map(|(id, b)| {
            (
                (*id).to_string(),
                serde_json::json!({
                    "x": b.origin.x.as_f32(),
                    "y": b.origin.y.as_f32(),
                    "w": b.size.width.as_f32(),
                    "h": b.size.height.as_f32(),
                }),
            )
        })
        .collect();
    serde_json::json!({
        "kind": "gpui",
        "preset": preset,
        "variant": variant,
        "scale": scale,
        "elements": rects,
    })
}
