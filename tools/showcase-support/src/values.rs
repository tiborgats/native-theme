//! The value Widget Info shows for a leaf of `docs/showcase-elements.toml`,
//! as the resolved theme holds it, in the text the three showcases agreed
//! (the list's header, "The Widget Info format"): a colour as native-theme's
//! `Rgba` prints it; a `_px` length as its number and ` px`; a `_ms`
//! duration as its number and ` ms`; a font as its family, its size in the
//! unit its source stated, and its weight, with its style where that is not
//! normal; a text-scale entry as its size in pixels and its weight; any
//! other number as itself; a boolean, a name or an enum as it prints;
//! [`NOT_STATED`] for what the theme leaves unstated.

use std::str::FromStr as _;

use native_theme::color::Rgba;
use native_theme::theme::{IconSet, LayoutTheme, ResolvedTheme};

/// What Widget Info shows for a value the theme leaves unstated.
pub const NOT_STATED: &str = "not stated";

/// A leaf's value: its text, and its colour where it is one, which the
/// showcase draws as the row's swatch in its own colour type.
#[derive(Clone, Debug, PartialEq)]
pub struct LeafValue {
    pub text: String,
    pub colour: Option<Rgba>,
}

impl LeafValue {
    fn text(text: String) -> Self {
        Self { text, colour: None }
    }
}

/// The resolved theme as [`leaf_value`] walks it: `serde_json::to_value`.
pub fn theme_json(resolved: &ResolvedTheme) -> Result<serde_json::Value, String> {
    serde_json::to_value(resolved).map_err(|e| format!("the resolved theme's JSON: {e}"))
}

/// The value of `leaf`, a path of `docs/property-registry.toml`, from
/// `resolved`, the resolved theme's JSON ([`theme_json`]), but for the
/// leaves the resolved theme does not hold: a `layout.*` leaf from `layout`,
/// `theme_variant.icon_set` from `icon_set`, the set the theme states, and
/// `defaults.icon_theme` from `icon_theme`, the icon theme the showcase
/// resolved for the mode it shows (the variant's, else the theme's, else the
/// system's; native-theme's `Resolved::icon_theme`).
pub fn leaf_value(
    leaf: &str,
    resolved: &serde_json::Value,
    layout: &LayoutTheme,
    icon_set: Option<IconSet>,
    icon_theme: Option<&str>,
) -> LeafValue {
    let stated =
        |value: Option<String>| LeafValue::text(value.unwrap_or_else(|| NOT_STATED.to_string()));
    if let Some(field) = leaf.strip_prefix("layout.") {
        let value = match field {
            "widget_gap_px" => layout.widget_gap,
            "container_margin_px" => layout.container_margin,
            "window_margin_px" => layout.window_margin,
            "section_gap_px" => layout.section_gap,
            _ => None,
        };
        return stated(value.map(|v| format!("{v} px")));
    }
    match leaf {
        "theme_variant.icon_set" => return stated(icon_set.map(|set| set.to_string())),
        "defaults.icon_theme" => return stated(icon_theme.map(str::to_string)),
        _ => {}
    }
    let value = leaf
        .split('.')
        .try_fold(resolved, |at, segment| step(at, segment));
    match value {
        Some(value) => value_of(leaf, value),
        None => LeafValue::text(NOT_STATED.to_string()),
    }
}

/// One step down a registry path in the resolved theme's JSON: a field by
/// its own name, or by the name without its `_px` or `_ms` unit, a border
/// side (`padding_top_px`) under `padding`.
fn step<'a>(at: &'a serde_json::Value, segment: &str) -> Option<&'a serde_json::Value> {
    if let Some(found) = at.get(segment) {
        return Some(found);
    }
    let bare = segment
        .strip_suffix("_px")
        .or_else(|| segment.strip_suffix("_ms"))
        .unwrap_or(segment);
    if let Some(side) = bare.strip_prefix("padding_") {
        return at.get("padding").and_then(|padding| padding.get(side));
    }
    at.get(bare)
}

/// `value`, the value of `leaf` in the resolved theme's JSON, as Widget Info
/// shows it: the text [`leaf_value`] gives a leaf it found. For a showcase
/// that walks a JSON tree of its own to a leaf (the egui showcase's
/// `mapping.toml` rows, whose keys carry no unit), so its rows read as the
/// listed ones do.
#[must_use]
pub fn value_of(leaf: &str, value: &serde_json::Value) -> LeafValue {
    match value {
        serde_json::Value::Null => LeafValue::text(NOT_STATED.to_string()),
        serde_json::Value::Bool(b) => LeafValue::text(b.to_string()),
        serde_json::Value::Number(_) => {
            let v = number(Some(value));
            match unit(leaf) {
                Some(unit) => LeafValue::text(format!("{v} {unit}")),
                None => LeafValue::text(v),
            }
        }
        serde_json::Value::String(s) => match Rgba::from_str(s) {
            Ok(colour) if s.starts_with('#') => LeafValue {
                text: colour.to_string(),
                colour: Some(colour),
            },
            _ => LeafValue::text(s.clone()),
        },
        serde_json::Value::Object(o) if o.contains_key("family") => LeafValue::text(font_text(o)),
        serde_json::Value::Object(o) if o.contains_key("line_height") => {
            let size = number(o.get("size"));
            let weight = number(o.get("weight"));
            LeafValue::text(format!("{size} px {weight}"))
        }
        other => LeafValue::text(other.to_string()),
    }
}

/// The unit a leaf's number is shown in: `px` for a `_px` leaf, `ms` for a
/// `_ms` one.
fn unit(leaf: &str) -> Option<&'static str> {
    if leaf.ends_with("_px") {
        Some("px")
    } else if leaf.ends_with("_ms") {
        Some("ms")
    } else {
        None
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    use native_theme::ResolutionContext;
    use native_theme::theme::{ColorMode, FontSize, Theme};

    /// kde-breeze light, resolved for tests: the theme, its resolution and
    /// its JSON.
    fn kde_breeze_light() -> (Theme, native_theme::Resolved, serde_json::Value) {
        let theme = Theme::preset("kde-breeze").expect("the preset loads");
        let resolved = theme
            .resolve_with(ColorMode::Light, &ResolutionContext::for_tests())
            .expect("the preset resolves");
        let json = theme_json(&resolved.variant).expect("the resolved theme serialises");
        (theme, resolved, json)
    }

    /// A leaf's value reads as the three showcases agreed: a colour as
    /// `Rgba` prints it, a length with ` px`, a font as family, stated size
    /// and weight, a text-scale entry as size and weight, a number as
    /// itself, the icon set and icon theme as the showcase hands them in.
    #[test]
    fn a_leaf_value_reads_as_the_showcases_agreed() {
        let (theme, all, json) = kde_breeze_light();
        let layout = &theme.layout;
        let resolved = &all.variant;
        let icon_theme = all.icon_theme.as_deref();
        let value = |leaf: &str| leaf_value(leaf, &json, layout, None, icon_theme).text;
        let font = &resolved.button.font;
        let stated = match font.defined_size {
            Some(FontSize::Pt(v)) => format!("{v} pt"),
            Some(FontSize::Px(v)) => format!("{v} px"),
            None => format!("{} px", font.size),
        };
        assert_eq!(
            value("button.font"),
            format!("{} {stated} {}", font.family, font.weight)
        );
        assert_eq!(
            value("button.font.color"),
            resolved.button.font.color.to_string()
        );
        assert_eq!(
            value("defaults.line_height"),
            format!("{}", resolved.defaults.line_height)
        );
        assert_eq!(
            value("button.border.corner_radius_px"),
            format!("{} px", resolved.button.border.corner_radius)
        );
        assert_eq!(
            value("defaults.icon_sizes.small_px"),
            format!("{} px", resolved.defaults.icon_sizes.small)
        );
        assert_eq!(
            value("text_scale.section_heading"),
            format!(
                "{} px {}",
                resolved.text_scale.section_heading.size,
                resolved.text_scale.section_heading.weight
            )
        );
        assert_eq!(value("theme_variant.icon_set"), NOT_STATED);
        assert_eq!(
            leaf_value(
                "theme_variant.icon_set",
                &json,
                layout,
                Some(all.icon_set),
                None
            )
            .text,
            all.icon_set.to_string()
        );
        // The variant's own icon theme, as the preset states it.
        let breeze = theme
            .light
            .as_ref()
            .and_then(|light| light.defaults.icon_theme.as_deref())
            .expect("kde-breeze's light variant states an icon theme");
        assert_eq!(icon_theme, Some(breeze));
        assert_eq!(value("defaults.icon_theme"), breeze);
        assert_eq!(
            leaf_value("defaults.icon_theme", &json, layout, None, None).text,
            NOT_STATED
        );
        match layout.widget_gap {
            Some(gap) => assert_eq!(value("layout.widget_gap_px"), format!("{gap} px")),
            None => assert_eq!(value("layout.widget_gap_px"), NOT_STATED),
        }
        match resolved.button.border.padding.top {
            Some(top) => assert_eq!(value("button.border.padding_top_px"), format!("{top} px")),
            None => assert_eq!(value("button.border.padding_top_px"), NOT_STATED),
        }
        assert_eq!(
            leaf_value("button.background_color", &json, layout, None, None).colour,
            Some(resolved.button.background_color),
            "a colour leaf has no swatch, or not its colour"
        );
        for e in crate::elements::elements() {
            for leaf in &e.leaves {
                let shown = value(leaf);
                assert!(
                    !shown.starts_with('{') && !shown.starts_with('['),
                    "{} {leaf} shows raw JSON: {shown}",
                    e.id
                );
            }
        }
    }

    /// A `_ms` leaf reads as a `_px` one does: `tooltip.show_delay_ms` is the
    /// resolved theme's `tooltip.show_delay`, with ` ms`.
    #[test]
    fn a_ms_leaf_reads_its_value() {
        let (theme, all, json) = kde_breeze_light();
        let delay = all
            .variant
            .tooltip
            .show_delay
            .expect("kde-breeze states a tooltip delay");
        assert_eq!(
            leaf_value("tooltip.show_delay_ms", &json, &theme.layout, None, None).text,
            format!("{delay} ms")
        );
    }

    /// A value another walk reached reads as `leaf_value`'s own: a colour as
    /// `Rgba` prints it, with its swatch; a number under a key with no unit
    /// as itself, under a `_px` one with ` px`.
    #[test]
    fn a_value_reached_elsewhere_reads_as_a_leafs() {
        let colour = value_of("button.background_color", &serde_json::json!("#336699"));
        assert_eq!(colour.text, "#336699");
        assert_eq!(colour.colour, Rgba::from_str("#336699").ok());
        assert_eq!(
            value_of("button.border.corner_radius", &serde_json::json!(4.0)).text,
            "4"
        );
        assert_eq!(
            value_of("button.border.corner_radius_px", &serde_json::json!(4.0)).text,
            "4 px"
        );
        assert_eq!(
            value_of("button.border.padding.top", &serde_json::Value::Null).text,
            NOT_STATED
        );
    }

    /// A font's style is named where it is not normal; a leaf the theme
    /// leaves unstated, or no leaf of the theme, reads [`NOT_STATED`].
    #[test]
    fn a_font_names_its_style_and_a_missing_leaf_is_not_stated() {
        let font = serde_json::json!({
            "family": "Noto Sans",
            "size": 13.333_333,
            "defined_size": {"Pt": 10.0},
            "weight": 400,
            "style": "italic",
        });
        let json = serde_json::json!({"label": {"font": font, "gap": null}});
        let layout = LayoutTheme::default();
        let value = |leaf: &str| leaf_value(leaf, &json, &layout, None, None).text;
        assert_eq!(value("label.font"), "Noto Sans 10 pt 400 italic");
        assert_eq!(value("label.gap_px"), NOT_STATED);
        assert_eq!(value("label.no_such_leaf"), NOT_STATED);
        assert_eq!(value("layout.section_gap_px"), NOT_STATED);
    }
}
