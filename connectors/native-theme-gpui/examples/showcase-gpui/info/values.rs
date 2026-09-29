//! The value Widget Info shows for a leaf of docs/showcase-elements.toml,
//! as the resolved theme holds it, in the text the three showcases agreed
//! (docs/showcase-elements.toml, "The Widget Info format"): a colour as
//! native-theme's `Rgba` prints it; a `_px` length as its number and
//! ` px`; a font as its family, its size in the unit its source stated, and
//! its weight, with its style where that is not normal; a text-scale entry
//! as its size in pixels and its weight; any other number as itself; a
//! boolean, a name or an enum as it prints; `not stated` for what the theme
//! leaves unstated.

use std::str::FromStr as _;

use gpui::Hsla;
use native_theme::color::Rgba;
use native_theme::theme::{IconSet, LayoutTheme};

/// A leaf's value: its text, and its colour where it is one.
#[derive(Clone, Debug, PartialEq)]
pub struct LeafValue {
    pub text: String,
    pub colour: Option<Hsla>,
}

/// What Widget Info shows for a value the theme leaves unstated.
pub const NOT_STATED: &str = "not stated";

/// The value of `leaf`, a path of docs/property-registry.toml: from
/// `resolved`, the resolved theme as `serde_json::to_value` gives it; a
/// `layout.*` leaf from `layout`; `theme_variant.icon_set` from `icon_set`,
/// the set the theme states.
pub fn leaf_value(
    leaf: &str,
    resolved: &serde_json::Value,
    layout: &LayoutTheme,
    icon_set: Option<IconSet>,
) -> LeafValue {
    let text = |text: String| LeafValue { text, colour: None };
    if let Some(field) = leaf.strip_prefix("layout.") {
        let value = match field {
            "widget_gap_px" => layout.widget_gap,
            "container_margin_px" => layout.container_margin,
            "window_margin_px" => layout.window_margin,
            "section_gap_px" => layout.section_gap,
            _ => None,
        };
        return text(value.map_or_else(|| NOT_STATED.to_string(), |v| format!("{v} px")));
    }
    if leaf == "theme_variant.icon_set" {
        return text(icon_set.map_or_else(|| NOT_STATED.to_string(), |set| set.to_string()));
    }
    let value = leaf
        .split('.')
        .try_fold(resolved, |at, segment| step(at, segment));
    match value {
        Some(value) => shown(leaf, value),
        None => text(NOT_STATED.to_string()),
    }
}

/// One step down a registry path in the resolved theme's JSON: a field by
/// its own name, or by the name without its `_px` unit, a border side
/// (`padding_top_px`) under `padding`.
fn step<'a>(at: &'a serde_json::Value, segment: &str) -> Option<&'a serde_json::Value> {
    if let Some(found) = at.get(segment) {
        return Some(found);
    }
    let bare = segment.strip_suffix("_px").unwrap_or(segment);
    if let Some(side) = bare.strip_prefix("padding_") {
        return at.get("padding").and_then(|padding| padding.get(side));
    }
    at.get(bare)
}

/// `value`, the value of `leaf`, as Widget Info shows it.
fn shown(leaf: &str, value: &serde_json::Value) -> LeafValue {
    let text = |text: String| LeafValue { text, colour: None };
    match value {
        serde_json::Value::Null => text(NOT_STATED.to_string()),
        serde_json::Value::Bool(b) => text(b.to_string()),
        serde_json::Value::Number(_) => {
            let v = number(Some(value));
            if leaf.ends_with("_px") {
                text(format!("{v} px"))
            } else {
                text(v)
            }
        }
        serde_json::Value::String(s) => match Rgba::from_str(s) {
            Ok(colour) if s.starts_with('#') => LeafValue {
                text: colour.to_string(),
                colour: Some(super::stated(colour)),
            },
            _ => text(s.clone()),
        },
        serde_json::Value::Object(o) if o.contains_key("family") => text(font_text(o)),
        serde_json::Value::Object(o) if o.contains_key("line_height") => {
            let size = number(o.get("size"));
            let weight = number(o.get("weight"));
            text(format!("{size} px {weight}"))
        }
        other => text(other.to_string()),
    }
}

/// A number of the theme's JSON as the f32 the theme holds, printed.
fn number(value: Option<&serde_json::Value>) -> String {
    value
        .and_then(serde_json::Value::as_f64)
        .map_or_else(|| NOT_STATED.to_string(), |v| format!("{}", v as f32))
}

/// A resolved font: its family, its size as the source stated it
/// (`defined_size`, in points or pixels; the resolved pixels where the
/// source stated none), its weight, and its style where it is not normal.
fn font_text(font: &serde_json::Map<String, serde_json::Value>) -> String {
    let family = font
        .get("family")
        .and_then(serde_json::Value::as_str)
        .unwrap_or(NOT_STATED);
    let defined = font.get("defined_size").and_then(|size| {
        [("Pt", "pt"), ("Px", "px")]
            .into_iter()
            .find_map(|(key, unit)| size.get(key).map(|v| (v, unit)))
    });
    let size = match defined {
        Some((v, unit)) => format!("{} {unit}", number(Some(v))),
        None => format!("{} px", number(font.get("size"))),
    };
    let weight = number(font.get("weight"));
    match font.get("style").and_then(serde_json::Value::as_str) {
        Some(style) if style != "normal" => format!("{family} {size} {weight} {style}"),
        _ => format!("{family} {size} {weight}"),
    }
}
