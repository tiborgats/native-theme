//! [`crate::values`] for the 0.8.0 model: the value Widget Info shows for a
//! leaf, read from `native_theme::next::ResolvedTheme`'s serialised form
//! (docs/v0.8.0-spec.md §9, "`ResolvedTheme`'s serialised form"; the golden
//! snapshots in `native-theme/tests/snapshots/` show it): globals as their
//! fields, a static widget as its parts, a ruled widget's part as
//! `{ exclusive, base, rules }`, and each part and text style as only the
//! properties the theme states.
//!
//! A leaf is a path of that form, its segments joined by `.`: a field by its
//! own name, or by its name without a `_px` or `_ms` unit the leaf adds for
//! the text (`layout.widget_gap_px` is `layout.widget_gap`, shown with
//! ` px`); a ruled part's property by the part's name, read from its base,
//! before any rule (`button.body.fill` is `button.body.base.fill`); an item
//! of a list by its index (`button.body.min_size.0`). The 0.8.0 theme holds
//! its layout, icon set and icon theme (`layout.*`, `icons.set`,
//! `icons.theme`), so no leaf is read from beside it.
//!
//! The text is the one the three showcases agreed (the list's header, "The
//! Widget Info format"), as [`crate::values`] gives it: a colour as
//! `native_theme::next::Color` prints it; a number with ` px` or ` ms` where
//! the leaf ends in that unit; a text style as its family, its size in the
//! unit its source stated (`defined_size`, else `size_px`), its weight, and
//! its style where that is not normal, each part the theme leaves unstated
//! as [`NOT_STATED`]; a boolean, a name or an enum as it prints;
//! [`NOT_STATED`] for what the form does not hold, a part that states
//! nothing included. A text style's family, of a whole text style or a
//! `*.family` leaf, is the label the showcase's lookup gives it
//! ([`crate::status::DrawnFamily::label`]).

use std::str::FromStr as _;

use native_theme::next::{Color, FontStyle, ResolvedTheme};

use crate::values::{LeafValue, NOT_STATED, number, unit};

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

/// The resolved theme as [`leaf_value`] walks it: `serde_json::to_value`.
pub fn theme_json(resolved: &ResolvedTheme) -> Result<serde_json::Value, String> {
    serde_json::to_value(resolved).map_err(|e| format!("the resolved theme's JSON: {e}"))
}

/// The value of `leaf`, a path of the serialised form (module doc), in
/// `resolved`, the resolved theme's JSON ([`theme_json`]), as Widget Info
/// shows it, or [`NOT_STATED`] where the JSON holds none. A text style's
/// family, of a whole text style (`button.label`) or a `*.family` leaf
/// (`typography.body.family`), is `family`'s label of it: `family` is called
/// with the text style's path (`button.label`, `typography.monospace`) and
/// its font ([`StatedFont`]), and gives the label that says what the toolkit
/// draws the family in. The counterpart of both [`crate::values::leaf_value`]
/// and [`crate::values::value_at`].
pub fn leaf_value(
    leaf: &str,
    resolved: &serde_json::Value,
    family: &dyn Fn(&str, &StatedFont) -> String,
) -> LeafValue<Color> {
    let Some(value) = walk(resolved, leaf) else {
        return LeafValue::text(NOT_STATED.to_string());
    };
    if value.is_string()
        && let Some(path) = leaf.strip_suffix(".family")
        && let Some(label) = walk(resolved, path).and_then(|text| family_of(path, text, family))
    {
        return LeafValue::text(label);
    }
    value_of(leaf, value, family)
}

/// The value at `leaf` in the JSON `at`, one [`step`] per segment, a ruled
/// part reached whole as its base.
fn walk<'a>(at: &'a serde_json::Value, leaf: &str) -> Option<&'a serde_json::Value> {
    leaf.split('.')
        .try_fold(at, |at, segment| step(at, segment))
        .map(base)
}

/// One step down a path of the serialised form: an item of a list by its
/// index; a field by its own name, or by the name without its `_px` or
/// `_ms` unit; else that field of a ruled part's base.
fn step<'a>(at: &'a serde_json::Value, segment: &str) -> Option<&'a serde_json::Value> {
    if let Some(items) = at.as_array() {
        return segment
            .parse::<usize>()
            .ok()
            .and_then(|index| items.get(index));
    }
    let field = |at: &'a serde_json::Value| {
        at.get(segment).or_else(|| {
            segment
                .strip_suffix("_px")
                .or_else(|| segment.strip_suffix("_ms"))
                .and_then(|bare| at.get(bare))
        })
    };
    field(at).or_else(|| at.get("base").and_then(field))
}

/// The keys of a ruled part; `exclusive` and `rules` are left out where
/// empty.
const RULED: [&str; 3] = ["exclusive", "base", "rules"];

/// A ruled part (`{ exclusive, base, rules }`) as its base; any other value
/// as itself.
fn base(value: &serde_json::Value) -> &serde_json::Value {
    match (value.as_object(), value.get("base")) {
        (Some(part), Some(base)) if part.keys().all(|key| RULED.contains(&key.as_str())) => base,
        _ => value,
    }
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

/// `value`, the value of `leaf` in the serialised form, as Widget Info shows
/// it (module doc).
fn value_of(
    leaf: &str,
    value: &serde_json::Value,
    family: &dyn Fn(&str, &StatedFont) -> String,
) -> LeafValue<Color> {
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
        serde_json::Value::String(s) => match Color::from_str(s) {
            Ok(colour) if is_colour_text(s) => LeafValue {
                text: colour.to_string(),
                colour: Some(colour),
            },
            _ => LeafValue::text(s.clone()),
        },
        serde_json::Value::Object(o) if o.is_empty() => LeafValue::text(NOT_STATED.to_string()),
        serde_json::Value::Object(o) if is_text(o) => {
            LeafValue::text(text_style(o, family_of(leaf, value, family)))
        }
        other => LeafValue::text(other.to_string()),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    use native_theme::next::{
        FontSize, Paint, Preset, TextStated, TextStyle, Theme, ThemeContext, WidgetState,
    };

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    /// A lookup that names every family as stated.
    fn as_stated(_: &str, font: &StatedFont) -> String {
        font.family.clone()
    }

    /// kde-breeze light, resolved for the light scheme's default context,
    /// and its JSON.
    fn kde_breeze_light() -> std::result::Result<(ResolvedTheme, serde_json::Value), String> {
        let resolved = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        let json = theme_json(&resolved)?;
        Ok((resolved, json))
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

    /// A leaf's value reads as the showcases agreed: a text style as family,
    /// stated size and weight; a colour as `Color` prints it, with its
    /// swatch, a ruled part's property read from its base; a length with
    /// ` px`, a duration with ` ms`; a name as it prints; the layout and the
    /// icons from the theme itself.
    #[test]
    fn a_leaf_value_reads_as_the_showcases_agreed() -> Result {
        let (r, json) = kde_breeze_light()?;
        let value = |leaf: &str| leaf_value(leaf, &json, &as_stated);
        let label = r.button.label(WidgetState::NONE);
        assert_eq!(value("button.label").text, text_of(&label));
        let colour = value("button.label.color");
        assert_eq!(colour.text, label.color.to_string());
        assert_eq!(colour.colour, Some(label.color));
        let body = r.button.body(WidgetState::NONE);
        let Paint::Solid(fill) = body.fill else {
            return Err("kde-breeze's button body has no solid fill".into());
        };
        assert_eq!(value("button.body.fill").colour, Some(fill));
        assert_eq!(value("button.body.base.fill").colour, Some(fill));
        assert_eq!(
            value("button.body.padding.top_px").text,
            format!("{} px", body.padding.top)
        );
        let headline = r
            .typography
            .headline
            .as_ref()
            .ok_or("kde-breeze states no headline")?;
        assert_eq!(value("typography.headline").text, text_of(headline));
        assert_eq!(value("typography.body").text, text_of(&r.typography.body));
        let gap = r
            .layout
            .widget_gap
            .ok_or("kde-breeze states no widget gap")?;
        assert_eq!(value("layout.widget_gap_px").text, format!("{gap} px"));
        let delay = r
            .timings
            .tooltip_show_ms
            .ok_or("kde-breeze states no tooltip delay")?;
        assert_eq!(value("timings.tooltip_show_ms").text, format!("{delay} ms"));
        let theme = r
            .icons
            .theme
            .as_deref()
            .ok_or("kde-breeze names no icon theme")?;
        assert_eq!(value("icons.theme").text, theme);
        assert_eq!(value("icons.set").text, "freedesktop");
        let small = r
            .icons
            .sizes
            .small
            .ok_or("kde-breeze states no small icon size")?;
        assert_eq!(value("icons.sizes.small_px").text, format!("{small} px"));
        assert_eq!(value("context.scheme").text, "light");
        Ok(())
    }

    /// What the form does not hold reads [`NOT_STATED`]: a missing leaf, a
    /// null, a part that states nothing, a property its base leaves out, an
    /// index past a list's end.
    #[test]
    fn what_the_form_does_not_hold_is_not_stated() -> Result {
        let (_, json) = kde_breeze_light()?;
        let value = |leaf: &str| leaf_value(leaf, &json, &as_stated).text;
        assert_eq!(value("button.no_such_leaf"), NOT_STATED);
        assert_eq!(value("colors.header.background"), NOT_STATED);
        assert_eq!(value("button.icon"), NOT_STATED);
        assert_eq!(value("button.icon.color"), NOT_STATED);
        assert_eq!(value("button.body.min_size.2"), NOT_STATED);
        Ok(())
    }

    /// A text style that states no family or size reads as one, its
    /// unstated parts [`NOT_STATED`]; an object that is no text style, a
    /// stroke, prints as its JSON.
    #[test]
    fn an_object_reads_as_a_text_style_only_where_it_is_one() {
        let json = serde_json::json!({
            "label": {"text": {"weight": 700}},
            "body": {"stroke": {"width": 1.0, "color": "#000000"}},
        });
        let value = |leaf: &str| leaf_value(leaf, &json, &as_stated).text;
        assert_eq!(
            value("label.text"),
            format!("{NOT_STATED} {NOT_STATED} 700")
        );
        assert_eq!(
            value("body.stroke"),
            serde_json::json!({"width": 1.0, "color": "#000000"}).to_string()
        );
    }

    /// A text style's family, of a whole text style or a `*.family` leaf, is
    /// the lookup's label, the lookup handed the style's path and font; the
    /// style's other leaves read as stated.
    #[test]
    fn a_text_family_is_the_lookups_label() -> Result {
        let (r, json) = kde_breeze_light()?;
        let asked = std::cell::RefCell::new(Vec::new());
        let label = |path: &str, font: &StatedFont| {
            asked.borrow_mut().push((path.to_string(), font.clone()));
            format!("{} (drawn in Theme Sans)", font.family)
        };
        let value = |leaf: &str| leaf_value(leaf, &json, &label).text;
        let (body, mono) = (&r.typography.body, &r.typography.monospace);
        assert_eq!(
            value("typography.body.family"),
            format!("{} (drawn in Theme Sans)", body.family)
        );
        let stated_text = leaf_value("typography.monospace", &json, &as_stated).text;
        assert_eq!(
            value("typography.monospace"),
            stated_text.replacen(
                &*mono.family,
                &format!("{} (drawn in Theme Sans)", mono.family),
                1
            )
        );
        assert_eq!(value("typography.body.color"), body.color.to_string());
        let font = |text: &native_theme::next::TextStyle| StatedFont {
            family: text.family.to_string(),
            weight: Some(text.weight),
            style: Some(text.style),
        };
        assert_eq!(
            *asked.borrow(),
            [
                ("typography.body".to_string(), font(body)),
                ("typography.monospace".to_string(), font(mono)),
            ]
        );
        Ok(())
    }

    /// A text style names its style where it is not normal and its pixel
    /// size where its source stated none; a part it leaves unstated reads
    /// [`NOT_STATED`]; a colour that is not exact in 8-bit sRGB keeps the
    /// text `Color` prints it in.
    #[test]
    fn a_text_style_shows_what_it_states() -> Result {
        let json = serde_json::json!({
            "label": {
                "italic": {"family": "Noto Sans", "size_px": 13.0, "weight": 400, "style": "italic"},
                "sizeless": {"family": "Noto Sans"},
            },
            "colour": "srgb(0.5, 0.25, 0.125, 1)",
        });
        let value = |leaf: &str| leaf_value(leaf, &json, &as_stated);
        assert_eq!(value("label.italic").text, "Noto Sans 13 px 400 italic");
        assert_eq!(
            value("label.sizeless").text,
            format!("Noto Sans {NOT_STATED} {NOT_STATED}")
        );
        let colour = Color::from_str("srgb(0.5, 0.25, 0.125, 1)")?;
        assert_eq!(value("colour").colour, Some(colour));
        assert_eq!(value("colour").text, colour.to_string());
        Ok(())
    }
}
