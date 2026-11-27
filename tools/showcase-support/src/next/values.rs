//! [`crate::values`] for the 0.8.0 model: the value Widget Info shows for a
//! home (the header of `docs/showcase-elements.toml`, "One entry" and "The
//! Widget Info format"), read from `native_theme::next::ResolvedTheme` and
//! its serialised form (docs/v0.8.0-spec.md §9, "`ResolvedTheme`'s serialised
//! form"; the golden snapshots in `native-theme/tests/snapshots/` show it):
//! globals as their fields, a static widget as its parts, and each part and
//! text style as only the properties the theme states.
//!
//! A home is one of the list's forms. A global, a widget's metric and a
//! static part's property are paths of the serialised form, their segments
//! joined by `.`, each field by its own name (`layout.widget_gap`,
//! `button.icon_text_gap`, `tooltip.panel.fill`). A ruled part's property,
//! `<widget>.<part>(<query>).<property>`, is read from the style the query
//! returns ([`ruled_style`]); `window.<getter>(normal).<property>` from
//! `WindowChrome`'s `frame`, `title_bar` or `title` at
//! `WindowState::default()`; a text part or a global text style named whole
//! (`button.label(NONE)`, `typography.body`) is the whole text style. Where
//! the query has a state, the style is masked first: a property that no rule
//! requiring a state wrote (its `from_state` bit clear) is not stated for that
//! state (spec §2), the sizes by their `MIN_SIZE` and `MAX_SIZE` bits and a
//! text's `defined_size` with its `SIZE_PX`. Such a property that the
//! query's style states (on the part's base or by a rule requiring no state)
//! is its value there marked [`STATED_AT_REST`], `<value> (stated at rest)`:
//! a statement about the theme, which states it and nothing for that state;
//! what a toolkit draws there is the route line's. A text style named whole
//! is shown masked. The style is serialised as the form serialises a part
//! and walked as the rest is; an item of a list is read by its index
//! (`min_size[0]`).
//!
//! The text is the one the three showcases agreed (the list's header, "The
//! Widget Info format"): a colour as `native_theme::next::Color` prints it; a
//! number with the unit of its property's kind ([`unit_of`]: ` px` for a
//! length, ` ms` for a duration, none for an opacity, a weight or a factor);
//! a text style as its family, its size in the unit its source stated
//! (`defined_size`, else `size_px`), its weight, and its style where that is
//! not normal, each part the theme leaves unstated as [`NOT_STATED`]; a
//! boolean, a name or an enum as it prints; rounded corners whose curve is
//! continuous as their radius marked [`DRAWN_CIRCULAR`], `<value> (drawn
//! circular)`, the toolkits' corner being circular (docs/v0.8.0-spec.md §10,
//! rule 2); a part's stroke stated per side that
//! `native_theme::next::Stroke::prevailing` reduces (four sides of one width
//! and solid colours, at least three of them one colour) as the uniform
//! stroke a toolkit with one border colour draws (spec §10, rule 2): its
//! `stroke.color` that colour, its swatch too, marked [`side_not_drawn`],
//! `<colour> (<side> side <colour> not drawn)`, where the fourth side's
//! colour is another, and its `stroke.width` the width, unmarked. This holds
//! for a ruled part, a window part and a static part (`tooltip.panel`), not
//! for a focus indicator (`focus_ring.*`, a part's `focus`); the marker
//! stands before [`STATED_AT_REST`].
//! [`NOT_STATED`] where the theme
//! states nothing: a property whose `stated` bit is clear, a size that is
//! `None`, a global that is `null`, a part that states nothing; a value
//! stated only at rest under a query with a state is marked
//! [`STATED_AT_REST`] (above), never [`NOT_STATED`]. A property stated in a
//! shape the home's path does not address (a stroke of `none` or
//! one per side that does not reduce, read at `stroke.color`) is that stated form's text, never
//! [`NOT_STATED`]. A home of no known name is an error. A text style's family,
//! of a whole text style or a `*.family` home, is the label the showcase's
//! lookup gives it ([`crate::status::DrawnFamily::label`]).

use std::str::FromStr as _;

use native_theme::next::{
    ButtonKind, ButtonQuery, ButtonShape, ButtonVariant, Color, ControlSize, FontStyle,
    GroupPosition, IconStyle, InputQuery, InputVariant, ListKind, ListQuery, ListVariant,
    Orientation, Paint, PartStated, PartStyle, ResolvedTheme, Stroke, StyleQuery, Surface,
    TabQuery, TabVariant, TextStated, TextStyle, WidgetState, WindowState,
};

use crate::values::{LeafValue, NOT_STATED, number};

/// The marker after a value that a query with a state shows as the part
/// states it at rest (module doc): `<value> (stated at rest)`.
pub const STATED_AT_REST: &str = "(stated at rest)";

/// The marker after a rounded corners value whose curve is continuous,
/// which every toolkit the showcases draw with rounds circular
/// (docs/v0.8.0-spec.md §10, rule 2): `<value> (drawn circular)`.
pub const DRAWN_CIRCULAR: &str = "(drawn circular)";

/// The four sides of a per-side stroke, by the names its serialised form
/// gives them.
const SIDES: [&str; 4] = ["top", "right", "bottom", "left"];

/// The marker after a per-side stroke's colour that [`Stroke::prevailing`]
/// reduces to the colour of three sides, naming the fourth side and its
/// colour, which a toolkit with one border colour does not draw
/// (docs/v0.8.0-spec.md §10, rule 2): `<colour> (<side> side <colour> not
/// drawn)`.
#[must_use]
pub fn side_not_drawn(side: &str, colour: Color) -> String {
    format!("({side} side {colour} not drawn)")
}

/// `text` without a [`side_not_drawn`] marker at its end.
#[must_use]
pub fn without_side_not_drawn(text: &str) -> &str {
    text.strip_suffix(" not drawn)")
        .and_then(|marked| marked.rsplit_once(" ("))
        .filter(|(_, marker)| {
            SIDES.iter().any(|side| {
                marker
                    .strip_prefix(side)
                    .is_some_and(|rest| rest.starts_with(" side "))
            })
        })
        .map_or(text, |(value, _)| value)
}

/// A per-side stroke that [`Stroke::prevailing`] reduces: the uniform stroke
/// it is drawn as, and the [`side_not_drawn`] marker of the side whose colour
/// is another, where one is; `None` for every other stroke.
fn reduced(stroke: &Stroke) -> Option<(Stroke, Option<String>)> {
    let Stroke::Complex(sides) = stroke else {
        return None;
    };
    let uniform = stroke.prevailing();
    let Stroke::Uniform { color, .. } = uniform else {
        return None;
    };
    let marker = SIDES
        .iter()
        .zip([&sides.top, &sides.right, &sides.bottom, &sides.left])
        .find_map(|(name, side)| match side.paint {
            Paint::Solid(other) if other != color => Some(side_not_drawn(name, other)),
            _ => None,
        });
    Some((uniform, marker))
}

/// `value`, read at the stroke home `property` of a stroke [`reduced`]
/// gives `marker` for, marked: a colour that is stated carries the marker,
/// a width none.
fn marked_stroke(
    value: LeafValue<Color>,
    property: &str,
    marker: Option<String>,
) -> LeafValue<Color> {
    match marker {
        Some(marker) if property == "stroke.color" && value.text != NOT_STATED => LeafValue {
            text: format!("{} {marker}", value.text),
            colour: value.colour,
        },
        _ => value,
    }
}

/// A text style's font as the serialised form states it, which the family
/// lookup is handed: the family, and the weight and the slant where the
/// theme states them.
#[derive(Clone, Debug, PartialEq)]
pub struct StatedFont {
    /// The family.
    pub family: String,
    /// The weight, where stated.
    pub weight: Option<u16>,
    /// The slant, where stated.
    pub style: Option<FontStyle>,
}

/// The resolved theme as [`home_value`] walks it: `serde_json::to_value`.
pub fn theme_json(resolved: &ResolvedTheme) -> Result<serde_json::Value, String> {
    serde_json::to_value(resolved).map_err(|e| format!("the resolved theme's JSON: {e}"))
}

/// The value of `home` (module doc) in `theme`, whose serialised form is
/// `json` ([`theme_json`]), as Widget Info shows it, or an error where the
/// home names nothing the model has. A text style's family, of a whole text
/// style (`button.label(NONE)`, `typography.body`) or a `*.family` home, is
/// `family`'s label of it: `family` is called with the text style's path
/// (`button.label(NONE)`, `typography.monospace`) and its font
/// ([`StatedFont`]), and gives the label that says what the toolkit draws
/// the family in. The counterpart of [`crate::values::leaf_value`].
pub fn home_value(
    home: &str,
    theme: &ResolvedTheme,
    json: &serde_json::Value,
    family: &dyn Fn(&str, &StatedFont) -> String,
) -> Result<LeafValue<Color>, String> {
    let unknown = || format!("`{home}` is no home of the model");
    let Some((head, after)) = home.split_once('(') else {
        return global_value(home, json, family).ok_or_else(unknown)?;
    };
    let (query, property) = after.split_once(')').ok_or_else(unknown)?;
    let property = match property {
        "" => None,
        dotted => Some(dotted.strip_prefix('.').ok_or_else(unknown)?),
    };
    let (widget, part) = head.split_once('.').ok_or_else(unknown)?;
    let path = format!("{head}({query})");
    let (style, state) = if widget == "window" {
        (window_style(theme, part, query)?, WidgetState::NONE)
    } else {
        ruled_style(theme, widget, part, query)?.ok_or_else(unknown)?
    };
    if state == WidgetState::NONE {
        return style_value(&path, &style, property, family).ok_or_else(unknown)?;
    }
    let masked = style.clone().masked_to_its_states();
    let value = style_value(&path, &masked, property, family).ok_or_else(unknown)??;
    if property.is_none() || value.text != NOT_STATED {
        return Ok(value);
    }
    let rest = style_value(&path, &style, property, family).ok_or_else(unknown)??;
    Ok(if rest.text == NOT_STATED {
        value
    } else {
        LeafValue {
            text: format!("{} {STATED_AT_REST}", rest.text),
            colour: rest.colour,
        }
    })
}

/// A part's style as a query or a getter returns it, by the part's kind.
#[derive(Clone, Debug, PartialEq)]
pub enum Styled {
    /// A part.
    Part(PartStyle),
    /// A text part.
    Text(TextStyle),
    /// An icon part.
    Icon(IconStyle),
}

impl Styled {
    /// The style with every property no rule requiring a state wrote left
    /// unstated (module doc).
    fn masked_to_its_states(self) -> Self {
        match self {
            Self::Part(mut style) => {
                style.stated &= style.from_state;
                if !style.from_state.contains(PartStated::MIN_SIZE) {
                    style.min_size = [None, None];
                }
                if !style.from_state.contains(PartStated::MAX_SIZE) {
                    style.max_size = [None, None];
                }
                Self::Part(style)
            }
            Self::Text(mut style) => {
                style.stated &= style.from_state;
                if !style.stated.contains(TextStated::SIZE_PX) {
                    style.defined_size = None;
                }
                Self::Text(style)
            }
            Self::Icon(mut style) => {
                style.stated &= style.from_state;
                Self::Icon(style)
            }
        }
    }

    /// The style's serialised form, and the properties a home may read of a
    /// part of its kind (the list's header, "A part's property").
    fn json(&self) -> Result<(serde_json::Value, &'static [&'static str]), String> {
        let json = match self {
            Self::Part(style) => serde_json::to_value(style),
            Self::Text(style) => serde_json::to_value(style),
            Self::Icon(style) => serde_json::to_value(style),
        }
        .map_err(|e| format!("a part's JSON: {e}"))?;
        let properties: &[&str] = match self {
            Self::Part(_) => &PART_PROPERTIES,
            Self::Text(_) => &TEXT_PROPERTIES,
            Self::Icon(_) => &ICON_PROPERTIES,
        };
        Ok((json, properties))
    }
}

/// The properties of `PartStyle` a home reads.
const PART_PROPERTIES: [&str; 23] = [
    "fill",
    "stroke.color",
    "stroke.width",
    "corners",
    "shadows",
    "opacity",
    "padding.top",
    "padding.right",
    "padding.bottom",
    "padding.left",
    "margin.top",
    "margin.right",
    "margin.bottom",
    "margin.left",
    "min_size[0]",
    "min_size[1]",
    "max_size[0]",
    "max_size[1]",
    "focus",
    "ripple",
    "transition",
    "cursor",
    "transform",
];

/// The properties of `TextStyle` a home reads.
const TEXT_PROPERTIES: [&str; 12] = [
    "family",
    "color",
    "size_px",
    "defined_size",
    "weight",
    "style",
    "width",
    "line_height",
    "letter_spacing",
    "optical_size",
    "decoration",
    "blend_mode",
];

/// The properties of `IconStyle` a home reads.
const ICON_PROPERTIES: [&str; 7] = [
    "size",
    "color",
    "opacity",
    "weight",
    "rendering_mode",
    "stroke_width",
    "blend_mode",
];

/// Whether `property` is a property of a part, a text or an icon.
fn is_property(property: &str) -> bool {
    [
        PART_PROPERTIES.as_slice(),
        &TEXT_PROPERTIES,
        &ICON_PROPERTIES,
    ]
    .iter()
    .any(|properties| properties.contains(&property))
}

/// `WindowChrome`'s `getter` at `WindowState::default()`, the query written
/// `normal`.
fn window_style(theme: &ResolvedTheme, getter: &str, query: &str) -> Result<Styled, String> {
    if query != "normal" {
        return Err(format!("a window home's query is `normal`, not `{query}`"));
    }
    let normal = WindowState::default();
    Ok(match getter {
        "frame" => Styled::Part(theme.window.frame(normal)),
        "title_bar" => Styled::Part(theme.window.title_bar(normal)),
        "title" => Styled::Text(theme.window.title(normal)),
        other => return Err(format!("`window.{other}` is no getter of `WindowChrome`")),
    })
}

/// The style of `widget`'s ruled part `part` for `query` (the list's header,
/// `<widget>.<part>(<query>)`) as the query returns it, unmasked, and the
/// query's state, by which [`home_value`] masks it (module doc); `None` where
/// the widget has no such ruled part, an error where the query does not
/// parse or has an axis the widget's query lacks.
pub fn ruled_style(
    theme: &ResolvedTheme,
    widget: &str,
    part: &str,
    query: &str,
) -> Result<Option<(Styled, WidgetState)>, String> {
    let q = Query::parse(query)?;
    let style = match (widget, part) {
        ("button", "body") => Styled::Part(theme.button.body(q.button()?)),
        ("button", "label") => Styled::Text(theme.button.label(q.button()?)),
        ("button", "icon") => Styled::Icon(theme.button.icon(q.button()?)),
        ("checkbox", "indicator") => Styled::Part(theme.checkbox.indicator(q.style()?)),
        ("checkbox", "checkmark") => Styled::Icon(theme.checkbox.checkmark(q.style()?)),
        ("checkbox", "label") => Styled::Text(theme.checkbox.label(q.style()?)),
        ("radio", "indicator") => Styled::Part(theme.radio.indicator(q.style()?)),
        ("radio", "dot") => Styled::Part(theme.radio.dot(q.style()?)),
        ("radio", "label") => Styled::Text(theme.radio.label(q.style()?)),
        ("scrollbar", "track") => Styled::Part(theme.scrollbar.track(q.style()?)),
        ("scrollbar", "thumb") => Styled::Part(theme.scrollbar.thumb(q.style()?)),
        ("list", "container") => Styled::Part(theme.list.container(q.list()?)),
        ("list", "row") => Styled::Part(theme.list.row(q.list()?)),
        ("list", "highlight") => Styled::Part(theme.list.highlight(q.list()?)),
        ("list", "item_label") => Styled::Text(theme.list.item_label(q.list()?)),
        ("list", "header") => Styled::Part(theme.list.header(q.list()?)),
        ("list", "header_label") => Styled::Text(theme.list.header_label(q.list()?)),
        ("list", "header_separator") => Styled::Part(theme.list.header_separator(q.list()?)),
        ("input", "field") => Styled::Part(theme.input.field(q.input()?)),
        ("input", "text") => Styled::Text(theme.input.text(q.input()?)),
        ("input", "placeholder") => Styled::Text(theme.input.placeholder(q.input()?)),
        ("input", "helper_text") => Styled::Text(theme.input.helper_text(q.input()?)),
        ("text_area", "field") => Styled::Part(theme.text_area.field(q.input()?)),
        ("text_area", "text") => Styled::Text(theme.text_area.text(q.input()?)),
        ("text_area", "placeholder") => Styled::Text(theme.text_area.placeholder(q.input()?)),
        ("text_area", "helper_text") => Styled::Text(theme.text_area.helper_text(q.input()?)),
        ("link", "container") => Styled::Part(theme.link.container(q.style()?)),
        ("link", "label") => Styled::Text(theme.link.label(q.style()?)),
        ("slider", "track") => Styled::Part(theme.slider.track(q.style()?)),
        ("slider", "active_track") => Styled::Part(theme.slider.active_track(q.style()?)),
        ("slider", "thumb") => Styled::Part(theme.slider.thumb(q.style()?)),
        ("slider", "tick") => Styled::Part(theme.slider.tick(q.style()?)),
        ("progress_bar", "track") => Styled::Part(theme.progress_bar.track(q.style()?)),
        ("progress_bar", "fill") => Styled::Part(theme.progress_bar.fill(q.style()?)),
        ("splitter", "handle") => Styled::Part(theme.splitter.handle(q.style()?)),
        ("tab", "bar") => Styled::Part(theme.tab.bar(q.tab()?)),
        ("tab", "tab") => Styled::Part(theme.tab.tab(q.tab()?)),
        ("tab", "label") => Styled::Text(theme.tab.label(q.tab()?)),
        ("tab", "indicator") => Styled::Part(theme.tab.indicator(q.tab()?)),
        ("tab", "close_button") => Styled::Part(theme.tab.close_button(q.tab()?)),
        ("tab", "close_icon") => Styled::Icon(theme.tab.close_icon(q.tab()?)),
        ("menu", "panel") => Styled::Part(theme.menu.panel(q.style()?)),
        ("menu", "item") => Styled::Part(theme.menu.item(q.style()?)),
        ("menu", "item_label") => Styled::Text(theme.menu.item_label(q.style()?)),
        ("menu", "shortcut") => Styled::Text(theme.menu.shortcut(q.style()?)),
        ("menu", "icon") => Styled::Icon(theme.menu.icon(q.style()?)),
        ("menu", "submenu_arrow") => Styled::Icon(theme.menu.submenu_arrow(q.style()?)),
        ("menu", "separator") => Styled::Part(theme.menu.separator(q.style()?)),
        ("menu_bar", "bar") => Styled::Part(theme.menu_bar.bar(q.style()?)),
        ("menu_bar", "item") => Styled::Part(theme.menu_bar.item(q.style()?)),
        ("menu_bar", "item_label") => Styled::Text(theme.menu_bar.item_label(q.style()?)),
        ("sidebar", "container") => Styled::Part(theme.sidebar.container(q.style()?)),
        ("sidebar", "item") => Styled::Part(theme.sidebar.item(q.style()?)),
        ("sidebar", "item_label") => Styled::Text(theme.sidebar.item_label(q.style()?)),
        ("sidebar", "item_icon") => Styled::Icon(theme.sidebar.item_icon(q.style()?)),
        ("sidebar", "section_header") => Styled::Text(theme.sidebar.section_header(q.style()?)),
        ("toolbar", "bar") => Styled::Part(theme.toolbar.bar(q.style()?)),
        ("toolbar", "button") => Styled::Part(theme.toolbar.button(q.style()?)),
        ("toolbar", "button_label") => Styled::Text(theme.toolbar.button_label(q.style()?)),
        ("toolbar", "button_icon") => Styled::Icon(theme.toolbar.button_icon(q.style()?)),
        ("toolbar", "separator") => Styled::Part(theme.toolbar.separator(q.style()?)),
        ("switch", "track") => Styled::Part(theme.switch.track(q.style()?)),
        ("switch", "thumb") => Styled::Part(theme.switch.thumb(q.style()?)),
        ("switch", "label") => Styled::Text(theme.switch.label(q.style()?)),
        ("combo_box", "field") => Styled::Part(theme.combo_box.field(q.style()?)),
        ("combo_box", "label") => Styled::Text(theme.combo_box.label(q.style()?)),
        ("combo_box", "chevron") => Styled::Icon(theme.combo_box.chevron(q.style()?)),
        ("segmented_control", "track") => Styled::Part(theme.segmented_control.track(q.style()?)),
        ("segmented_control", "segment") => {
            Styled::Part(theme.segmented_control.segment(q.style()?))
        }
        ("segmented_control", "label") => Styled::Text(theme.segmented_control.label(q.style()?)),
        ("segmented_control", "separator") => {
            Styled::Part(theme.segmented_control.separator(q.style()?))
        }
        ("card", "container") => Styled::Part(theme.card.container(q.style()?)),
        ("expander", "container") => Styled::Part(theme.expander.container(q.style()?)),
        ("expander", "header") => Styled::Part(theme.expander.header(q.style()?)),
        ("expander", "header_label") => Styled::Text(theme.expander.header_label(q.style()?)),
        ("expander", "arrow") => Styled::Icon(theme.expander.arrow(q.style()?)),
        _ => return Ok(None),
    };
    Ok(Some((style, q.state)))
}

/// `property` of `style`, the part at `path` (`button.label(NONE)`), or the
/// whole text style where `property` is `None`; `None` where the part's kind
/// has no such property or, named whole, is no text.
fn style_value(
    path: &str,
    style: &Styled,
    property: Option<&str>,
    family: &dyn Fn(&str, &StatedFont) -> String,
) -> Option<Result<LeafValue<Color>, String>> {
    if let (Styled::Part(part), Some(property @ ("stroke.color" | "stroke.width"))) =
        (style, property)
        && let Some((uniform, marker)) = reduced(&part.stroke)
    {
        let mut drawn = part.clone();
        drawn.stroke = uniform;
        return style_value(path, &Styled::Part(drawn), Some(property), family)
            .map(|value| value.map(|value| marked_stroke(value, property, marker)));
    }
    let (json, properties) = match style.json() {
        Ok(json) => json,
        Err(e) => return Some(Err(e)),
    };
    let label = || family_of(path, &json, family);
    let Some(property) = property else {
        return matches!(style, Styled::Text(_)).then(|| {
            Ok(LeafValue::text(match json.as_object() {
                Some(text) if !text.is_empty() => text_style(text, label()),
                _ => NOT_STATED.to_string(),
            }))
        });
    };
    if !properties.contains(&property) {
        return None;
    }
    let segments = segments(property);
    Some(match walk(&json, &segments) {
        Walk::Found(value, in_list) => found_value(&segments, value, in_list, label),
        Walk::Shape(value) => Ok(shape_value(value)),
        Walk::Missing(_) => Ok(LeafValue::text(NOT_STATED.to_string())),
    })
}

/// A global, a metric or a static part's property at `home`, a path of the
/// serialised form `json`; `None` where the form has no such field.
fn global_value(
    home: &str,
    json: &serde_json::Value,
    family: &dyn Fn(&str, &StatedFont) -> String,
) -> Option<Result<LeafValue<Color>, String>> {
    let segments = segments(home);
    if let Some(value) = reduced_global_stroke(&segments, json) {
        return Some(value);
    }
    Some(match walk(json, &segments) {
        Walk::Found(value, in_list) => {
            let label = || match segments.split_last() {
                Some((&"family", parent)) if value.is_string() => match walk(json, parent) {
                    Walk::Found(text, _) => family_of(&parent.join("."), text, family),
                    Walk::Shape(_) | Walk::Missing(_) => None,
                },
                _ => family_of(home, value, family),
            };
            found_value(&segments, value, in_list, label)
        }
        Walk::Shape(value) => Ok(shape_value(value)),
        Walk::Missing(at) => {
            let rest = segments.get(at..).unwrap_or_default();
            if at >= 2 && is_property(&property_path(rest)) {
                Ok(LeafValue::text(NOT_STATED.to_string()))
            } else {
                return None;
            }
        }
    })
}

/// A static part's stroke colour or width at `segments` of the serialised
/// form `json` (`tooltip.panel.stroke.color`), where the stroke there,
/// deserialised, is one [`reduced`] reduces: the uniform stroke's value,
/// marked as [`marked_stroke`] marks it; `None` for every other home, a
/// focus indicator's among them (`focus_ring.*`, a part's `focus`), which no
/// connector reduces.
fn reduced_global_stroke(
    segments: &[&str],
    json: &serde_json::Value,
) -> Option<Result<LeafValue<Color>, String>> {
    let (&property, parent) = segments.split_last()?;
    if !matches!(property, "color" | "width")
        || parent.last() != Some(&"stroke")
        || segments.first() == Some(&"focus_ring")
        || parent.contains(&"focus")
    {
        return None;
    }
    let Walk::Found(stated, _) = walk(json, parent) else {
        return None;
    };
    let (uniform, marker) = reduced(&serde_json::from_value(stated.clone()).ok()?)?;
    let drawn = match serde_json::to_value(&uniform) {
        Ok(drawn) => drawn,
        Err(e) => return Some(Err(format!("a stroke's JSON: {e}"))),
    };
    let Walk::Found(value, in_list) = walk(&drawn, &[property]) else {
        return None;
    };
    Some(
        found_value(segments, value, in_list, || None)
            .map(|value| marked_stroke(value, &format!("stroke.{property}"), marker)),
    )
}

/// The segments of a path: `.`-separated, an item of a list by its index
/// (`min_size[0]` is `min_size`, `0`).
fn segments(path: &str) -> Vec<&str> {
    path.split('.')
        .flat_map(|segment| match segment.split_once('[') {
            Some((name, index)) => match index.strip_suffix(']') {
                Some(index) => vec![name, index],
                None => vec![segment],
            },
            None => vec![segment],
        })
        .collect()
}

/// `segments` written back as a property is (`min_size[0]`).
fn property_path(segments: &[&str]) -> String {
    let mut path = String::new();
    for segment in segments {
        if segment.parse::<usize>().is_ok() {
            path.push_str(&format!("[{segment}]"));
        } else {
            if !path.is_empty() {
                path.push('.');
            }
            path.push_str(segment);
        }
    }
    path
}

/// Where a walk down a path of the serialised form ends.
enum Walk<'a> {
    /// The value at the path, and whether it is an item of a list.
    Found(&'a serde_json::Value, bool),
    /// The path goes past a stated value of a shape it does not address (a
    /// stroke of `none` read at `stroke.color`, a stroke per side): that
    /// value.
    Shape(&'a serde_json::Value),
    /// The form holds nothing at the segment of this index.
    Missing(usize),
}

/// The value at `segments` in the JSON `at`: a field by its name, an item of
/// a list by its index.
fn walk<'a>(at: &'a serde_json::Value, segments: &[&str]) -> Walk<'a> {
    let mut at = at;
    let mut in_list = false;
    let mut previous = None;
    for (index, segment) in segments.iter().enumerate() {
        let next = match at {
            serde_json::Value::Array(items) => {
                segment.parse::<usize>().ok().and_then(|i| items.get(i))
            }
            serde_json::Value::Object(fields) => fields.get(*segment),
            _ => None,
        };
        let Some(next) = next else {
            return match at {
                serde_json::Value::String(_)
                | serde_json::Value::Number(_)
                | serde_json::Value::Bool(_)
                    if index > 0 =>
                {
                    Walk::Shape(at)
                }
                serde_json::Value::Object(_) if previous == Some("stroke") => Walk::Shape(at),
                _ => Walk::Missing(index),
            };
        };
        in_list = at.is_array();
        previous = Some(*segment);
        at = next;
    }
    Walk::Found(at, in_list)
}

/// A stated value of a shape the home's path does not address, as it
/// prints: a name as itself, anything else as its JSON.
fn shape_value(value: &serde_json::Value) -> LeafValue<Color> {
    LeafValue::text(match value.as_str() {
        Some(name) => name.to_string(),
        None => value.to_string(),
    })
}

/// `value`, found at `segments`, as Widget Info shows it: an item `none` of
/// a list (a size's unstated dimension) is [`NOT_STATED`], a family string
/// is `label`'s where it gives one; else [`value_of`].
fn found_value(
    segments: &[&str],
    value: &serde_json::Value,
    in_list: bool,
    label: impl Fn() -> Option<String>,
) -> Result<LeafValue<Color>, String> {
    if in_list && value.as_str() == Some("none") {
        return Ok(LeafValue::text(NOT_STATED.to_string()));
    }
    if value.is_string()
        && segments.last() == Some(&"family")
        && let Some(label) = label()
    {
        return Ok(LeafValue::text(label));
    }
    let label = if value.is_object() { label() } else { None };
    value_of(segments, value, label)
}

/// The properties of a text style that no part and no global group of the
/// serialised form has.
const TEXT_ONLY: [&str; 6] = [
    "family",
    "size_px",
    "defined_size",
    "weight",
    "style",
    "line_height",
];

/// Whether `text`, an object of the serialised form, is a text style: it
/// states one of [`TEXT_ONLY`].
fn is_text(text: &serde_json::Map<String, serde_json::Value>) -> bool {
    TEXT_ONLY.iter().any(|key| text.contains_key(*key))
}

/// `family`'s label of the family of `text`, the JSON of the text style at
/// `path`; `None` where `text` is not a text style stating a family.
fn family_of(
    path: &str,
    text: &serde_json::Value,
    family: &dyn Fn(&str, &StatedFont) -> String,
) -> Option<String> {
    let font = StatedFont {
        family: text.get("family")?.as_str()?.to_string(),
        weight: text
            .get("weight")
            .and_then(serde_json::Value::as_u64)
            .and_then(|weight| u16::try_from(weight).ok()),
        style: text
            .get("style")
            .and_then(|style| serde_json::from_value(style.clone()).ok()),
    };
    Some(family(path, &font))
}

/// `value`, the value at `segments` of the serialised form, as Widget Info
/// shows it (module doc); a text style's family as `label` names it where it
/// is given. An error for a number whose property has no known unit.
fn value_of(
    segments: &[&str],
    value: &serde_json::Value,
    label: Option<String>,
) -> Result<LeafValue<Color>, String> {
    Ok(match value {
        serde_json::Value::Null => LeafValue::text(NOT_STATED.to_string()),
        serde_json::Value::Bool(b) => LeafValue::text(b.to_string()),
        serde_json::Value::Number(_) => {
            let v = number(Some(value));
            match unit_of(segments)? {
                Some(unit) => LeafValue::text(format!("{v} {unit}")),
                None => LeafValue::text(v),
            }
        }
        serde_json::Value::String(s) => match Color::from_str(s) {
            Ok(colour) if is_colour_text(s) => LeafValue {
                text: colour.to_string(),
                colour: Some(colour),
            },
            _ => LeafValue::text(s.clone()),
        },
        serde_json::Value::Object(o) if o.is_empty() => LeafValue::text(NOT_STATED.to_string()),
        serde_json::Value::Object(o) if is_text(o) => LeafValue::text(text_style(o, label)),
        serde_json::Value::Object(o) => match continuous_radius(segments, o) {
            Some(radius) => LeafValue::text(format!(
                "{} {DRAWN_CIRCULAR}",
                value_of(segments, radius, None)?.text
            )),
            None => LeafValue::text(value.to_string()),
        },
        other => LeafValue::text(other.to_string()),
    })
}

/// The radius of `corners`, the serialised form's `{ radius, curve }` at
/// `segments`, where the property is `corners`, the curve is continuous and
/// the radius rounds a corner: not `none` nor a zero radius, which round
/// nothing, and not a concentric one (`{ inset, min_radius }`), which no
/// toolkit the showcases draw with takes (it needs the container's radius),
/// so none draws it circular; `None` otherwise.
fn continuous_radius<'a>(
    segments: &[&str],
    corners: &'a serde_json::Map<String, serde_json::Value>,
) -> Option<&'a serde_json::Value> {
    let radius = corners.get("radius")?;
    let rounds = |v: &serde_json::Value| v.as_f64().is_some_and(|r| r != 0.0);
    let rounds_a_corner = match radius {
        serde_json::Value::String(name) => name != "none",
        serde_json::Value::Number(_) => rounds(radius),
        serde_json::Value::Object(per_corner) => {
            !per_corner.contains_key("inset") && per_corner.values().any(rounds)
        }
        _ => false,
    };
    (segments.last() == Some(&"corners")
        && corners.get("curve").and_then(serde_json::Value::as_str) == Some("continuous")
        && rounds_a_corner)
        .then_some(radius)
}

/// The properties and fields measured in logical pixels (docs/v0.8.0-spec.md
/// §§2, 4-5; `PartStyle`, `TextStyle`, `IconStyle` and the widgets' metrics
/// document each "in logical pixels"), by their last segment.
const LENGTHS: [&str; 33] = [
    "corners",
    "min_size",
    "max_size",
    "size",
    "size_px",
    "line_height",
    "letter_spacing",
    "stroke_width",
    "offset",
    "thickness",
    "diameter",
    "min_diameter",
    "inset",
    "margin",
    "icon_text_gap",
    "label_gap",
    "item_gap",
    "button_gap",
    "arrow_gap",
    "widget_gap",
    "section_gap",
    "container_margin",
    "window_margin",
    "corner_radius",
    "corner_radius_lg",
    "border_width",
    "min_thumb_length",
    "tick_length",
    "divider_width",
    "separator_width",
    "arrow_area_width",
    "content_indent",
    "active_outward_corner_radius",
];

/// The properties and fields shown without a unit: an opacity, a weight or
/// width class, a factor, a dots-per-inch count.
const UNITLESS: [&str; 8] = [
    "opacity",
    "disabled_opacity",
    "border_opacity",
    "weight",
    "width",
    "line_height_multiplier",
    "text_scale",
    "font_dpi",
];

/// The unit a number at `segments` is shown in, by its property's kind: `ms`
/// for a field named `_ms`, `px` for one named `_px`; a font's source size
/// in the unit it names (`defined_size.pt`); `px` for an icon size
/// (`icons.sizes.*`), a padding's or a margin's side, a stroke's width, a
/// caret's width, the radius of corners in the serialised form's `{ radius,
/// curve }` (`CornerRadius`, in logical pixels) and every name in
/// `LENGTHS`; none for the names in `UNITLESS`. An error for any other name.
pub fn unit_of(segments: &[&str]) -> Result<Option<&'static str>, String> {
    let named: Vec<&str> = segments
        .iter()
        .copied()
        .filter(|segment| segment.parse::<usize>().is_err())
        .collect();
    let (last, parent) = match named.as_slice() {
        [.., parent, last] => (*last, Some(*parent)),
        [last] => (*last, None),
        [] => return Err("a number at no path has no unit".to_string()),
    };
    Ok(match (parent, last) {
        (_, name) if name.ends_with("_ms") => Some("ms"),
        (_, name) if name.ends_with("_px") => Some("px"),
        (Some("defined_size"), "pt") => Some("pt"),
        (Some("defined_size"), "px") => Some("px"),
        (Some("sizes"), _) => Some("px"),
        (Some("padding" | "margin"), "top" | "right" | "bottom" | "left") => Some("px"),
        (Some("stroke" | "caret"), "width") => Some("px"),
        (Some("corners"), "radius") => Some("px"),
        (_, name) if LENGTHS.contains(&name) => Some("px"),
        (_, name) if UNITLESS.contains(&name) => None,
        _ => {
            return Err(format!(
                "no unit is known for `{}`",
                property_path(segments)
            ));
        }
    })
}

/// Whether `s` is in one of the text forms the serialised form writes a
/// colour in (`#rrggbb`, `#rrggbbaa`, `srgb(...)`), not a name that `Color`
/// would read too.
fn is_colour_text(s: &str) -> bool {
    s.starts_with('#') || s.starts_with("srgb(") || s.starts_with("p3(")
}

/// A text style: its family, as `label` names it where it is given, else as
/// stated; its size as the source stated it (`defined_size`, in points or
/// pixels; `size_px` where the source stated none); its weight; and its
/// style where it is not normal. A part the style does not state is
/// [`NOT_STATED`].
fn text_style(text: &serde_json::Map<String, serde_json::Value>, label: Option<String>) -> String {
    let family = label.unwrap_or_else(|| {
        text.get("family")
            .and_then(serde_json::Value::as_str)
            .unwrap_or(NOT_STATED)
            .to_string()
    });
    let defined = text.get("defined_size").and_then(|size| {
        ["pt", "px"]
            .into_iter()
            .find_map(|unit| size.get(unit).map(|v| (v, unit)))
    });
    let size = match (defined, text.get("size_px")) {
        (Some((v, unit)), _) => format!("{} {unit}", number(Some(v))),
        (None, Some(v)) => format!("{} px", number(Some(v))),
        (None, None) => NOT_STATED.to_string(),
    };
    let weight = number(text.get("weight"));
    match text.get("style").and_then(serde_json::Value::as_str) {
        Some(style) if style != "normal" => format!("{family} {size} {weight} {style}"),
        _ => format!("{family} {size} {weight}"),
    }
}

// ---------------------------------------------------------------------------
// The query of a home (ported from native-theme's
// `src/next/parity/compare.rs`, `Query`)
// ---------------------------------------------------------------------------

/// `WidgetState`'s bits by name (spec §3).
const STATE_NAMES: [(&str, WidgetState); 19] = [
    ("HOVER", WidgetState::HOVER),
    ("PRESSED", WidgetState::PRESSED),
    ("FOCUSED", WidgetState::FOCUSED),
    ("FOCUS_WITHIN", WidgetState::FOCUS_WITHIN),
    ("FOCUS_VISIBLE", WidgetState::FOCUS_VISIBLE),
    ("UNFOCUSED_VIEW", WidgetState::UNFOCUSED_VIEW),
    ("DISABLED", WidgetState::DISABLED),
    ("CHECKED", WidgetState::CHECKED),
    ("SELECTED", WidgetState::SELECTED),
    ("INDETERMINATE", WidgetState::INDETERMINATE),
    ("OPEN", WidgetState::OPEN),
    ("DEFAULT", WidgetState::DEFAULT),
    ("INVALID", WidgetState::INVALID),
    ("READ_ONLY", WidgetState::READ_ONLY),
    ("VISITED", WidgetState::VISITED),
    ("DRAGGED", WidgetState::DRAGGED),
    ("DROP_TARGET", WidgetState::DROP_TARGET),
    ("SCROLLED", WidgetState::SCROLLED),
    ("HOVER_WITHIN", WidgetState::HOVER_WITHIN),
];

/// The value of `name` in `values`, or an error naming the axis.
fn named<T: Copy>(axis: &str, name: &str, values: &[(&str, T)]) -> Result<T, String> {
    values
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, v)| *v)
        .ok_or_else(|| format!("`{name}` is no value of `{axis}`"))
}

/// A home's query: `<states>[; <axis>=<value>, …]`, its variant axes kept as
/// written until the widget's query type is known.
#[derive(Default)]
struct Query<'a> {
    state: WidgetState,
    size: ControlSize,
    surface: Surface,
    position: GroupPosition,
    orientation: Orientation,
    variant: Option<&'a str>,
    shape: Option<&'a str>,
    alternate: Option<&'a str>,
}

impl<'a> Query<'a> {
    fn parse(text: &'a str) -> Result<Self, String> {
        let (states, axes) = match text.split_once("; ") {
            Some((states, axes)) => (states, Some(axes)),
            None => (text, None),
        };
        let mut q = Query::default();
        if states != "NONE" {
            for name in states.split('|') {
                q.state |= named("state", name, &STATE_NAMES)?;
            }
        }
        for axis in axes.into_iter().flat_map(|a| a.split(", ")) {
            let (name, value) = axis
                .split_once('=')
                .ok_or_else(|| format!("an axis is `<name>=<value>`, not `{axis}`"))?;
            match name {
                "size" => {
                    q.size = named(
                        name,
                        value,
                        &[
                            ("Mini", ControlSize::Mini),
                            ("Small", ControlSize::Small),
                            ("Regular", ControlSize::Regular),
                            ("Large", ControlSize::Large),
                            ("ExtraLarge", ControlSize::ExtraLarge),
                        ],
                    )?;
                }
                "surface" => {
                    q.surface = named(
                        name,
                        value,
                        &[
                            ("Window", Surface::Window),
                            ("View", Surface::View),
                            ("HeaderBar", Surface::HeaderBar),
                            ("Sidebar", Surface::Sidebar),
                            ("Card", Surface::Card),
                            ("Dialog", Surface::Dialog),
                            ("Popover", Surface::Popover),
                            ("Tooltip", Surface::Tooltip),
                            ("Osd", Surface::Osd),
                        ],
                    )?;
                }
                "position" => {
                    q.position = named(
                        name,
                        value,
                        &[
                            ("Single", GroupPosition::Single),
                            ("First", GroupPosition::First),
                            ("Middle", GroupPosition::Middle),
                            ("Last", GroupPosition::Last),
                        ],
                    )?;
                }
                "orientation" => {
                    q.orientation = named(
                        name,
                        value,
                        &[
                            ("Horizontal", Orientation::Horizontal),
                            ("Vertical", Orientation::Vertical),
                        ],
                    )?;
                }
                "variant" => q.variant = Some(value),
                "shape" => q.shape = Some(value),
                "alternate" => q.alternate = Some(value),
                other => return Err(format!("no axis `{other}`")),
            }
        }
        Ok(q)
    }

    fn base<V: Default>(&self) -> StyleQuery<V> {
        StyleQuery::new(self.state)
            .size(self.size)
            .surface(self.surface)
            .position(self.position)
            .orientation(self.orientation)
    }

    /// The query of a widget without variant axes.
    fn style(&self) -> Result<StyleQuery, String> {
        if self.variant.is_some() || self.shape.is_some() || self.alternate.is_some() {
            return Err("the widget's query has no variant axis".to_owned());
        }
        Ok(self.base())
    }

    fn button(&self) -> Result<ButtonQuery, String> {
        if self.alternate.is_some() {
            return Err("a button's query has no axis `alternate`".to_owned());
        }
        let mut kind = ButtonKind::default();
        if let Some(v) = self.variant {
            kind.variant = named(
                "variant",
                v,
                &[
                    ("Standard", ButtonVariant::Standard),
                    ("Primary", ButtonVariant::Primary),
                    ("Secondary", ButtonVariant::Secondary),
                    ("Destructive", ButtonVariant::Destructive),
                    ("Outlined", ButtonVariant::Outlined),
                    ("Flat", ButtonVariant::Flat),
                    ("Glass", ButtonVariant::Glass),
                    ("Link", ButtonVariant::Link),
                ],
            )?;
        }
        if let Some(s) = self.shape {
            kind.shape = named(
                "shape",
                s,
                &[
                    ("Default", ButtonShape::Default),
                    ("Pill", ButtonShape::Pill),
                    ("Circle", ButtonShape::Circle),
                    ("Square", ButtonShape::Square),
                ],
            )?;
        }
        Ok(self.base().variant(kind))
    }

    fn input(&self) -> Result<InputQuery, String> {
        if self.shape.is_some() || self.alternate.is_some() {
            return Err("an input's query has no axis but `variant`".to_owned());
        }
        let mut variant = InputVariant::default();
        if let Some(v) = self.variant {
            variant = named(
                "variant",
                v,
                &[
                    ("Standard", InputVariant::Standard),
                    ("Outlined", InputVariant::Outlined),
                    ("Filled", InputVariant::Filled),
                    ("Search", InputVariant::Search),
                    ("Borderless", InputVariant::Borderless),
                ],
            )?;
        }
        Ok(self.base().variant(variant))
    }

    fn list(&self) -> Result<ListQuery, String> {
        if self.shape.is_some() {
            return Err("a list's query has no axis `shape`".to_owned());
        }
        let mut kind = ListKind::default();
        if let Some(v) = self.variant {
            kind.variant = named(
                "variant",
                v,
                &[
                    ("Plain", ListVariant::Plain),
                    ("InsetGrouped", ListVariant::InsetGrouped),
                    ("Sidebar", ListVariant::Sidebar),
                    ("Table", ListVariant::Table),
                ],
            )?;
        }
        if let Some(a) = self.alternate {
            kind.alternate = named("alternate", a, &[("true", true), ("false", false)])?;
        }
        Ok(self.base().variant(kind))
    }

    fn tab(&self) -> Result<TabQuery, String> {
        if self.shape.is_some() || self.alternate.is_some() {
            return Err("a tab's query has no axis but `variant`".to_owned());
        }
        let mut variant = TabVariant::default();
        if let Some(v) = self.variant {
            variant = named(
                "variant",
                v,
                &[
                    ("Standard", TabVariant::Standard),
                    ("Document", TabVariant::Document),
                    ("Pill", TabVariant::Pill),
                    ("Segmented", TabVariant::Segmented),
                ],
            )?;
        }
        Ok(self.base().variant(variant))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    use native_theme::next::{FontSize, Paint, Preset, Theme, ThemeContext};

    type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// A lookup that names every family as stated.
    fn as_stated(_: &str, font: &StatedFont) -> String {
        font.family.clone()
    }

    /// kde-breeze light, resolved for the light scheme's default context,
    /// and its JSON.
    fn kde_breeze_light() -> Result<(ResolvedTheme, serde_json::Value)> {
        let resolved = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        let json = theme_json(&resolved)?;
        Ok((resolved, json))
    }

    /// The text of `home` on kde-breeze light, each family as stated.
    fn text(theme: &ResolvedTheme, json: &serde_json::Value, home: &str) -> Result<String> {
        Ok(home_value(home, theme, json, &as_stated)?.text)
    }

    /// A text style's text as Widget Info shows it, each part from the
    /// model where the style states it: family, size in the unit its source
    /// stated, weight.
    fn text_of(text: &TextStyle) -> String {
        let stated = |bit| text.stated.contains(bit);
        let family = if stated(TextStated::FAMILY) {
            text.family.to_string()
        } else {
            NOT_STATED.to_string()
        };
        let size = match (stated(TextStated::SIZE_PX), text.defined_size) {
            (false, _) => NOT_STATED.to_string(),
            (true, Some(FontSize::Pt(v))) => format!("{v} pt"),
            (true, Some(FontSize::Px(v))) => format!("{v} px"),
            (true, None) => format!("{} px", text.size_px),
        };
        let weight = if stated(TextStated::WEIGHT) {
            text.weight.to_string()
        } else {
            NOT_STATED.to_string()
        };
        format!("{family} {size} {weight}")
    }

    /// A home's value reads as the showcases agreed: a text part or a global
    /// text style named whole as family, stated size and weight; a colour as
    /// `Color` prints it, with its swatch; a length with ` px`, a duration
    /// with ` ms`, an opacity bare; a name as it prints; a size's unset
    /// dimension not stated; the layout, the icons and the window's getters
    /// from the theme itself.
    #[test]
    fn a_home_value_reads_as_the_showcases_agreed() -> Result {
        let (r, json) = kde_breeze_light()?;
        let value = |home: &str| text(&r, &json, home);
        let label = r.button.label(WidgetState::NONE);
        assert_eq!(value("button.label(NONE)")?, text_of(&label));
        let colour = home_value("button.label(NONE).color", &r, &json, &as_stated)?;
        assert_eq!(colour.text, label.color.to_string());
        assert_eq!(colour.colour, Some(label.color));
        let body = r.button.body(WidgetState::NONE);
        let Paint::Solid(fill) = body.fill else {
            return Err("kde-breeze's button body has no solid fill".into());
        };
        let fill_value = home_value("button.body(NONE).fill", &r, &json, &as_stated)?;
        assert_eq!(fill_value.colour, Some(fill));
        assert_eq!(
            value("button.body(NONE).padding.top")?,
            format!("{} px", body.padding.top)
        );
        let [min_width, _] = body.min_size;
        let min_width = min_width.ok_or("kde-breeze states no button width")?;
        assert_eq!(
            value("button.body(NONE).min_size[0]")?,
            format!("{min_width} px")
        );
        assert_eq!(value("button.body(NONE).min_size[1]")?, NOT_STATED);
        let headline = r
            .typography
            .headline
            .as_ref()
            .ok_or("kde-breeze states no headline")?;
        assert_eq!(value("typography.headline")?, text_of(headline));
        assert_eq!(value("typography.body")?, text_of(&r.typography.body));
        let gap = r
            .layout
            .widget_gap
            .ok_or("kde-breeze states no widget gap")?;
        assert_eq!(value("layout.widget_gap")?, format!("{gap} px"));
        let delay = r
            .timings
            .tooltip_show_ms
            .ok_or("kde-breeze states no tooltip delay")?;
        assert_eq!(value("timings.tooltip_show_ms")?, format!("{delay} ms"));
        let theme = r
            .icons
            .theme
            .as_deref()
            .ok_or("kde-breeze names no icon theme")?;
        assert_eq!(value("icons.theme")?, theme);
        assert_eq!(value("icons.set")?, "freedesktop");
        let small = r
            .icons
            .sizes
            .small
            .ok_or("kde-breeze states no small icon size")?;
        assert_eq!(value("icons.sizes.small")?, format!("{small} px"));
        assert_eq!(value("context.scheme")?, "light");
        assert_eq!(
            value("button.icon_text_gap")?,
            format!("{} px", r.button.icon_text_gap)
        );
        let opacity = r
            .button
            .disabled_opacity
            .ok_or("kde-breeze states no disabled opacity")?;
        assert_eq!(value("button.disabled_opacity")?, format!("{opacity}"));
        let frame = r.window.frame(WindowState::default());
        let Paint::Solid(frame_fill) = frame.fill else {
            return Err("kde-breeze's window frame has no solid fill".into());
        };
        assert_eq!(value("window.frame(normal).fill")?, frame_fill.to_string());
        assert_eq!(
            value("window.title(normal)")?,
            text_of(&r.window.title(WindowState::default()))
        );
        Ok(())
    }

    /// Under a query with a state, a property no rule requiring a state
    /// wrote is its value at rest, marked as stated at rest, a size
    /// included; one such a rule wrote reads as the query returns it; a text
    /// style named whole is masked. At the rest state nothing is masked or
    /// marked.
    #[test]
    fn a_state_shows_only_what_its_rules_wrote() -> Result {
        let (r, json) = kde_breeze_light()?;
        let value = |home: &str| text(&r, &json, home);
        let at_rest =
            |home: &str| -> Result<String> { Ok(format!("{} {STATED_AT_REST}", value(home)?)) };
        let hover = r.button.body(WidgetState::HOVER);
        assert!(hover.from_state.contains(PartStated::FILL));
        assert!(!hover.from_state.contains(PartStated::CORNERS));
        assert!(!hover.from_state.contains(PartStated::MIN_SIZE));
        let Paint::Solid(fill) = hover.fill else {
            return Err("kde-breeze's hovered button has no solid fill".into());
        };
        assert_eq!(value("button.body(HOVER).fill")?, fill.to_string());
        assert_ne!(value("button.body(NONE).corners")?, NOT_STATED);
        assert!(!value("button.body(NONE).corners")?.contains(STATED_AT_REST));
        assert_eq!(
            value("button.body(HOVER).corners")?,
            at_rest("button.body(NONE).corners")?
        );
        assert_ne!(value("button.body(NONE).min_size[0]")?, NOT_STATED);
        assert_eq!(
            value("button.body(HOVER).min_size[0]")?,
            at_rest("button.body(NONE).min_size[0]")?
        );
        let label = r.button.label(WidgetState::HOVER);
        assert!(label.from_state.contains(TextStated::COLOR));
        assert!(!label.from_state.contains(TextStated::SIZE_PX));
        assert_eq!(value("button.label(HOVER).color")?, label.color.to_string());
        assert_eq!(
            value("button.label(HOVER).size_px")?,
            at_rest("button.label(NONE).size_px")?
        );
        assert_eq!(
            value("button.label(HOVER)")?,
            format!("{NOT_STATED} {NOT_STATED} {NOT_STATED}")
        );
        Ok(())
    }

    /// On a theme patched to state a button body's opacity on its base and
    /// again by a rule for the hovered state: queried pressed (no rule for
    /// that state writes it), the opacity is the base's, marked as stated at
    /// rest; queried hovered, the rule's, unmarked; at rest, the base's,
    /// unmarked. A property the theme does not state stays not stated under
    /// a state.
    #[test]
    fn a_value_stated_only_at_rest_is_marked() -> Result {
        use native_theme::next::{ButtonRule, PartPatch, ThemePatch};

        let mut r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        r.apply_patch(
            &ThemePatch::new()
                .button(ButtonRule::new().body(PartPatch::new().opacity(0.25)))
                .button(
                    ButtonRule::new()
                        .states(WidgetState::HOVER)
                        .body(PartPatch::new().opacity(0.75)),
                ),
        );
        let json = theme_json(&r)?;
        let value = |home: &str| text(&r, &json, home);
        let pressed = r.button.body(WidgetState::PRESSED);
        assert!(pressed.stated.contains(PartStated::OPACITY));
        assert!(!pressed.from_state.contains(PartStated::OPACITY));
        assert!(
            r.button
                .body(WidgetState::HOVER)
                .from_state
                .contains(PartStated::OPACITY)
        );
        assert_eq!(
            value("button.body(PRESSED).opacity")?,
            format!("0.25 {STATED_AT_REST}")
        );
        assert_eq!(value("button.body(HOVER).opacity")?, "0.75");
        assert_eq!(value("button.body(NONE).opacity")?, "0.25");
        assert!(!pressed.stated.contains(PartStated::TRANSFORM));
        assert_eq!(value("button.body(PRESSED).transform")?, NOT_STATED);
        Ok(())
    }

    /// Rounded corners whose curve is continuous read as their radius marked
    /// drawn circular, a ruled part's and the focus ring's, at rest and
    /// stated at rest under a state; square corners and a circular curve are
    /// not marked.
    #[test]
    fn continuous_corners_are_marked_drawn_circular() -> Result {
        use native_theme::next::{
            ButtonRule, CornerCurve, CornerRadius, Corners, PartPatch, ThemePatch,
        };

        let mut r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        r.apply_patch(
            &ThemePatch::new()
                .button(ButtonRule::new().body(PartPatch::new().corners(Corners::continuous(4.0)))),
        );
        r.focus_ring.corners = Some(Corners {
            radius: CornerRadius::Pill,
            curve: CornerCurve::Continuous,
        });
        let json = theme_json(&r)?;
        let value = |home: &str| text(&r, &json, home);
        assert_eq!(
            value("button.body(NONE).corners")?,
            format!("4 px {DRAWN_CIRCULAR}")
        );
        assert_eq!(
            value("button.body(HOVER).corners")?,
            format!("4 px {DRAWN_CIRCULAR} {STATED_AT_REST}")
        );
        assert_eq!(
            value("focus_ring.corners")?,
            format!("pill {DRAWN_CIRCULAR}")
        );

        r.apply_patch(
            &ThemePatch::new().button(ButtonRule::new().body(PartPatch::new().corners(Corners {
                radius: CornerRadius::None,
                curve: CornerCurve::Continuous,
            }))),
        );
        r.focus_ring.corners = Some(Corners::uniform(3.0));
        let json = theme_json(&r)?;
        let value = |home: &str| text(&r, &json, home);
        assert!(
            !value("button.body(NONE).corners")?.contains(DRAWN_CIRCULAR),
            "square corners have no curve"
        );
        assert_eq!(value("focus_ring.corners")?, "3 px");

        // A zero radius rounds nothing, and a concentric one no toolkit
        // takes: neither is drawn circular.
        for radius in [
            CornerRadius::Uniform(0.0),
            CornerRadius::Concentric {
                inset: 2.0,
                min_radius: 1.0,
            },
        ] {
            r.focus_ring.corners = Some(Corners {
                radius,
                curve: CornerCurve::Continuous,
            });
            let json = theme_json(&r)?;
            let shown = text(&r, &json, "focus_ring.corners")?;
            assert!(!shown.contains(DRAWN_CIRCULAR), "{radius:?}: {shown}");
        }
        Ok(())
    }

    /// A property stated in a shape the path does not address reads as that
    /// form, never as not stated: a stroke per side, a stroke of none.
    #[test]
    fn a_stated_shape_the_path_misses_reads_as_stated() -> Result {
        let (r, json) = kde_breeze_light()?;
        let value = |home: &str| text(&r, &json, home);
        let vertical = value("scrollbar.track(NONE; orientation=Vertical).stroke.color")?;
        assert!(
            vertical.starts_with('{') && vertical.contains("\"left\""),
            "{vertical}"
        );
        assert_eq!(value("focus_ring.inner_stroke.color")?, "none");
        Ok(())
    }

    /// The four sides of a per-side stroke 1 px wide, `odd` on the bottom
    /// and `main` on the other three.
    fn three_and_one(main: Color, odd: Color) -> Stroke {
        use native_theme::next::{StrokeAlign, StrokeSide};
        let side = |color| StrokeSide {
            width: 1.0,
            paint: Paint::Solid(color),
        };
        Stroke::sides(
            side(main),
            side(main),
            side(odd),
            side(main),
            StrokeAlign::Inside,
        )
    }

    /// A per-side stroke `Stroke::prevailing` reduces reads as the uniform
    /// stroke it is drawn as: a 3 + 1 its three sides' colour and swatch,
    /// marked with the fourth side's colour as not drawn, its width unmarked,
    /// on a ruled part and on a static part, the marker before the stated at
    /// rest one; four equal sides unmarked; a focus indicator's, the global
    /// one and a static part's own, keeps its stated form; the marker strips
    /// off.
    #[test]
    fn a_reduced_per_side_stroke_is_marked() -> Result {
        use native_theme::next::{ButtonRule, PartPatch, ThemePatch};

        let main = Color::rgb(0.2, 0.4, 0.6);
        let odd = Color::BLACK.with_alpha(0.5);
        let marked = format!("{main} {}", side_not_drawn("bottom", odd));
        let mut r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        r.apply_patch(
            &ThemePatch::new()
                .button(ButtonRule::new().body(PartPatch::new().stroke(three_and_one(main, odd)))),
        );
        r.tooltip.panel.stroke = three_and_one(main, odd);
        r.tooltip.panel.stated |= PartStated::STROKE;
        r.focus_ring.stroke = three_and_one(main, odd);
        r.tooltip.panel.focus = Some(std::sync::Arc::new(r.focus_ring.clone()));
        r.tooltip.panel.stated |= PartStated::FOCUS;
        let json = theme_json(&r)?;
        let value = |home: &str| text(&r, &json, home);
        let colour = home_value("button.body(NONE).stroke.color", &r, &json, &as_stated)?;
        assert_eq!(colour.text, marked);
        assert_eq!(colour.colour, Some(main));
        assert_eq!(value("button.body(NONE).stroke.width")?, "1 px");
        let (_, unruled) = [
            (WidgetState::PRESSED, "PRESSED"),
            (WidgetState::DISABLED, "DISABLED"),
            (WidgetState::CHECKED, "CHECKED"),
            (WidgetState::HOVER, "HOVER"),
        ]
        .into_iter()
        .find(|(state, _)| {
            !r.button
                .body(*state)
                .from_state
                .contains(PartStated::STROKE)
        })
        .ok_or("kde-breeze states the button's stroke for every state tried")?;
        assert_eq!(
            value(&format!("button.body({unruled}).stroke.color"))?,
            format!("{marked} {STATED_AT_REST}")
        );
        assert_eq!(
            value(&format!("button.body({unruled}).stroke.width"))?,
            format!("1 px {STATED_AT_REST}")
        );
        let panel = home_value("tooltip.panel.stroke.color", &r, &json, &as_stated)?;
        assert_eq!(panel.text, marked);
        assert_eq!(panel.colour, Some(main));
        assert_eq!(value("tooltip.panel.stroke.width")?, "1 px");
        for home in [
            "focus_ring.stroke.color",
            "tooltip.panel.focus.stroke.color",
        ] {
            let ring = value(home)?;
            assert!(
                ring.starts_with('{') && ring.contains("\"bottom\""),
                "{home}: {ring}"
            );
        }
        assert_eq!(without_side_not_drawn(&marked), main.to_string());
        let circular = format!("4 px {DRAWN_CIRCULAR}");
        assert_eq!(without_side_not_drawn(&circular), circular);

        r.apply_patch(
            &ThemePatch::new()
                .button(ButtonRule::new().body(PartPatch::new().stroke(three_and_one(main, main)))),
        );
        let json = theme_json(&r)?;
        assert_eq!(
            text(&r, &json, "button.body(NONE).stroke.color")?,
            main.to_string()
        );
        assert_eq!(text(&r, &json, "button.body(NONE).stroke.width")?, "1 px");
        Ok(())
    }

    /// A home of no known name is an error: an unknown widget, part, state,
    /// axis, property, global or window getter. A static part's property it
    /// leaves unstated, and a global that is null, are not stated.
    #[test]
    fn an_unknown_home_is_an_error() -> Result {
        let (r, json) = kde_breeze_light()?;
        for home in [
            "button.no_such_part(NONE).fill",
            "no_such_widget.body(NONE).fill",
            "button.body(NONE).no_such_property",
            "button.body(NO_SUCH_STATE).fill",
            "button.body(NONE; no_such_axis=1).fill",
            "button.body(NONE).fill.extra",
            "button.body(NONE",
            "layout.no_such_gap",
            "no_such_group",
            "window.no_such_getter(normal).fill",
            "window.frame(maximized).fill",
            "button.body(NONE).min_size",
        ] {
            assert!(
                home_value(home, &r, &json, &as_stated).is_err(),
                "`{home}` evaluates"
            );
        }
        assert_eq!(text(&r, &json, "tooltip.panel.shadows")?, NOT_STATED);
        assert_eq!(text(&r, &json, "timings.tooltip_hide_ms")?, NOT_STATED);
        Ok(())
    }

    /// A text style's family, of a whole text style or a `*.family` home,
    /// is the lookup's label, the lookup handed the style's path and font.
    #[test]
    fn a_text_family_is_the_lookups_label() -> Result {
        let (r, json) = kde_breeze_light()?;
        let asked = std::cell::RefCell::new(Vec::new());
        let label = |path: &str, font: &StatedFont| {
            asked.borrow_mut().push(path.to_string());
            format!("{} (drawn in Theme Sans)", font.family)
        };
        let value = |home: &str| home_value(home, &r, &json, &label).map(|v| v.text);
        let body = &r.typography.body;
        assert_eq!(
            value("typography.body.family")?,
            format!("{} (drawn in Theme Sans)", body.family)
        );
        let button = r.button.label(WidgetState::NONE);
        assert_eq!(
            value("button.label(NONE).family")?,
            format!("{} (drawn in Theme Sans)", button.family)
        );
        assert!(
            value("button.label(NONE)")?
                .starts_with(&format!("{} (drawn in Theme Sans) ", button.family))
        );
        assert_eq!(value("typography.body.color")?, body.color.to_string());
        assert_eq!(
            *asked.borrow(),
            [
                "typography.body",
                "button.label(NONE)",
                "button.label(NONE)"
            ]
        );
        Ok(())
    }

    /// The presets the tests below resolve.
    const PRESETS: [Preset; 7] = [
        Preset::KdeBreeze,
        Preset::Adwaita,
        Preset::Windows11,
        Preset::MacosTahoe,
        Preset::MacosMonterey,
        Preset::Material3,
        Preset::Ios13,
    ];

    /// Each preset's resolved theme, light and dark, with its JSON.
    fn resolved() -> Result<Vec<(Preset, ResolvedTheme, serde_json::Value)>> {
        let mut all = Vec::new();
        for preset in PRESETS {
            for context in [ThemeContext::light(), ThemeContext::dark()] {
                let theme = Theme::preset(preset).resolve(&context);
                let json = theme_json(&theme)?;
                all.push((preset, theme, json));
            }
        }
        Ok(all)
    }

    /// Whether `value` is a ruled part in the serialised form.
    fn is_ruled(value: &serde_json::Value) -> bool {
        value.as_object().is_some_and(|part| {
            part.contains_key("base")
                && part
                    .keys()
                    .all(|key| ["exclusive", "base", "rules"].contains(&key.as_str()))
        })
    }

    /// Every ruled part of a widget in a preset's serialised form is in
    /// [`ruled_style`]'s dispatch, but the window's: its homes are
    /// `WindowChrome`'s getters, and its caption parts take a
    /// `CaptionButtonKind` no home's query has an axis for.
    #[test]
    fn every_ruled_part_is_in_the_dispatch() -> Result {
        let mut missing = BTreeSet::new();
        for (preset, theme, json) in resolved()? {
            let widgets = json.as_object().ok_or("the theme is no object")?;
            for (widget, parts) in widgets.iter().filter(|(widget, _)| *widget != "window") {
                let Some(parts) = parts.as_object() else {
                    continue;
                };
                for (part, _) in parts.iter().filter(|(_, value)| is_ruled(value)) {
                    if !matches!(ruled_style(&theme, widget, part, "NONE"), Ok(Some(_))) {
                        missing.insert(format!("{preset:?}: {widget}.{part}"));
                    }
                }
            }
        }
        assert!(
            missing.is_empty(),
            "ruled parts not dispatched: {missing:#?}"
        );
        Ok(())
    }

    /// The numbers' paths in `value`, at `path`, outside ruled parts.
    fn numbers(path: &mut Vec<String>, value: &serde_json::Value, out: &mut BTreeSet<Vec<String>>) {
        match value {
            serde_json::Value::Number(_) => {
                out.insert(path.clone());
            }
            serde_json::Value::Object(fields) if !is_ruled(value) => {
                for (key, field) in fields {
                    path.push(key.clone());
                    numbers(path, field, out);
                    path.pop();
                }
            }
            serde_json::Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    path.push(index.to_string());
                    numbers(path, item, out);
                    path.pop();
                }
            }
            _ => {}
        }
    }

    /// [`unit_of`] classifies every number of a global, a metric and a
    /// static part in each preset's serialised form.
    #[test]
    fn every_number_has_a_unit() -> Result {
        let mut paths = BTreeSet::new();
        for (_, _, json) in resolved()? {
            numbers(&mut Vec::new(), &json, &mut paths);
        }
        assert!(!paths.is_empty());
        let unclassified: Vec<String> = paths
            .iter()
            .filter_map(|path| {
                let segments: Vec<&str> = path.iter().map(String::as_str).collect();
                unit_of(&segments).err()
            })
            .collect();
        assert!(unclassified.is_empty(), "{unclassified:#?}");
        Ok(())
    }
}
