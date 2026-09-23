// Border specification sub-structs for defaults-level and widget-level border properties

use crate::Rgba;
use native_theme_derive::ThemeFields;
use serde::{Deserialize, Serialize};

/// Defaults-level border specification: color, geometry, and opacity.
///
/// Used on [`ThemeDefaults`](crate::model::ThemeDefaults) for global border
/// properties that are inherited by per-widget borders.
///
/// **No padding fields:** Padding lives exclusively on [`WidgetBorderSpec`]
/// because padding is a widget-level layout concern, not a global default.
/// This split (Phase 79, BORDER-01) eliminated the former
/// "derives-from-presence" rule where the resolver would fill padding with
/// `0.0` based on whether `line_width` or `corner_radius` was set -- a
/// confusing proxy heuristic that is no longer needed.
///
/// All fields are optional to support partial overlays -- a DefaultsBorderSpec
/// with only `color` set will only override the color when merged.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ThemeFields)]
#[serde(default)]
pub struct DefaultsBorderSpec {
    /// Border color.
    pub color: Option<Rgba>,
    /// Corner radius in logical pixels.
    #[serde(rename = "corner_radius_px")]
    pub corner_radius: Option<f32>,
    /// Large corner radius in logical pixels (defaults only).
    #[serde(rename = "corner_radius_lg_px")]
    pub corner_radius_lg: Option<f32>,
    /// Border stroke width in logical pixels.
    #[serde(rename = "line_width_px")]
    pub line_width: Option<f32>,
    /// Border alpha multiplier 0.0-1.0 (defaults only).
    pub opacity: Option<f32>,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: Option<bool>,
}

impl_merge!(DefaultsBorderSpec {
    option { color, corner_radius, corner_radius_lg, line_width, opacity, shadow_enabled }
});

/// Widget-level border specification: color, geometry, and padding.
///
/// Used on per-widget structs for border properties specific to individual
/// widgets. Unlike [`DefaultsBorderSpec`], includes the four padding sides
/// (widget-level layout) but omits `corner_radius_lg` and `opacity`
/// (defaults-only geometry).
///
/// Padding fields are widget-only because different widgets need different
/// internal padding even when sharing the same border geometry from defaults.
///
/// **Padding is per side.** A platform states padding side by side, and some
/// sides differ (Windows' input is 10 left / 6 right, docs/platform-facts.md
/// §2.4), so the model stores the four sides and nothing else. `None` means
/// the platform states no value for that side; `Some(0.0)` means it states
/// zero.
///
/// **TOML.** The sides are `padding_top_px`, `padding_right_px`,
/// `padding_bottom_px` and `padding_left_px`. `padding_horizontal_px` and
/// `padding_vertical_px` are parse-time shorthand that set both sides of
/// their axis. A table that states an axis key together with one of that
/// axis's sides is a parse error naming both keys. Serialisation writes
/// sides only.
///
/// All fields are optional to support partial overlays, and they merge per
/// field, the overlay winning, so a reader's left side over a preset's
/// shorthand replaces the left side alone.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ThemeFields)]
#[serde(try_from = "WidgetBorderSpecRaw", into = "WidgetBorderSpecRaw")]
// serde reads and writes this struct through WidgetBorderSpecRaw, which also
// knows the two shorthand keys. The ThemeFields derive's introspection path
// cannot see the proxy, so declare the wire-format field names explicitly
// here, as FontSpec does. Keep in sync with WidgetBorderSpecRaw below.
#[theme_layer(
    fields = "color, corner_radius_px, line_width_px, shadow_enabled, padding_top_px, padding_right_px, padding_bottom_px, padding_left_px, padding_horizontal_px, padding_vertical_px"
)]
pub struct WidgetBorderSpec {
    /// Border color.
    pub color: Option<Rgba>,
    /// Corner radius in logical pixels.
    pub corner_radius: Option<f32>,
    /// Border stroke width in logical pixels.
    pub line_width: Option<f32>,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: Option<bool>,
    /// Padding inside the border above the content, in logical pixels.
    pub padding_top: Option<f32>,
    /// Padding inside the border right of the content, in logical pixels.
    pub padding_right: Option<f32>,
    /// Padding inside the border below the content, in logical pixels.
    pub padding_bottom: Option<f32>,
    /// Padding inside the border left of the content, in logical pixels.
    pub padding_left: Option<f32>,
}

/// Serde proxy for [`WidgetBorderSpec`]: the four side keys and the two axis
/// shorthands.
#[serde_with::skip_serializing_none]
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct WidgetBorderSpecRaw {
    color: Option<Rgba>,
    corner_radius_px: Option<f32>,
    line_width_px: Option<f32>,
    shadow_enabled: Option<bool>,
    padding_top_px: Option<f32>,
    padding_right_px: Option<f32>,
    padding_bottom_px: Option<f32>,
    padding_left_px: Option<f32>,
    padding_horizontal_px: Option<f32>,
    padding_vertical_px: Option<f32>,
}

/// Expand one axis shorthand into its two sides, rejecting a table that also
/// states either side.
fn expand_axis(
    axis: (&str, Option<f32>),
    first: (&str, Option<f32>),
    second: (&str, Option<f32>),
) -> Result<(Option<f32>, Option<f32>), String> {
    let Some(v) = axis.1 else {
        return Ok((first.1, second.1));
    };
    for side in [first, second] {
        if side.1.is_some() {
            return Err(format!(
                "border: set `{}` or `{}`, not both",
                axis.0, side.0
            ));
        }
    }
    Ok((Some(v), Some(v)))
}

impl TryFrom<WidgetBorderSpecRaw> for WidgetBorderSpec {
    type Error = String;
    fn try_from(raw: WidgetBorderSpecRaw) -> Result<Self, Self::Error> {
        let (padding_left, padding_right) = expand_axis(
            ("padding_horizontal_px", raw.padding_horizontal_px),
            ("padding_left_px", raw.padding_left_px),
            ("padding_right_px", raw.padding_right_px),
        )?;
        let (padding_top, padding_bottom) = expand_axis(
            ("padding_vertical_px", raw.padding_vertical_px),
            ("padding_top_px", raw.padding_top_px),
            ("padding_bottom_px", raw.padding_bottom_px),
        )?;
        Ok(WidgetBorderSpec {
            color: raw.color,
            corner_radius: raw.corner_radius_px,
            line_width: raw.line_width_px,
            shadow_enabled: raw.shadow_enabled,
            padding_top,
            padding_right,
            padding_bottom,
            padding_left,
        })
    }
}

impl From<WidgetBorderSpec> for WidgetBorderSpecRaw {
    fn from(b: WidgetBorderSpec) -> Self {
        WidgetBorderSpecRaw {
            color: b.color,
            corner_radius_px: b.corner_radius,
            line_width_px: b.line_width,
            shadow_enabled: b.shadow_enabled,
            padding_top_px: b.padding_top,
            padding_right_px: b.padding_right,
            padding_bottom_px: b.padding_bottom,
            padding_left_px: b.padding_left,
            padding_horizontal_px: None,
            padding_vertical_px: None,
        }
    }
}

impl_merge!(WidgetBorderSpec {
    option {
        color, corner_radius, line_width, shadow_enabled,
        padding_top, padding_right, padding_bottom, padding_left
    }
});

/// A widget's resolved padding, one field per side.
///
/// `None` means the theme states no value for that side, so a connector
/// leaves the toolkit's own padding in place there; `Some(0.0)` means the
/// theme states zero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResolvedPadding {
    /// Padding above the content, in logical pixels.
    pub top: Option<f32>,
    /// Padding right of the content, in logical pixels.
    pub right: Option<f32>,
    /// Padding below the content, in logical pixels.
    pub bottom: Option<f32>,
    /// Padding left of the content, in logical pixels.
    pub left: Option<f32>,
}

/// The resolved `defaults.border`: the global border geometry and colour
/// that widget borders inherit.
///
/// Phase 93-01 (G1): no `Default` derive. It is constructed from a fully
/// populated unresolved source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedDefaultsBorder {
    /// Border color.
    pub color: Rgba,
    /// Corner radius in logical pixels.
    pub corner_radius: f32,
    /// Large corner radius in logical pixels.
    pub corner_radius_lg: f32,
    /// Border stroke width in logical pixels.
    pub line_width: f32,
    /// Border alpha multiplier 0.0-1.0.
    pub opacity: f32,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: bool,
}

/// A widget's resolved border: its colour, geometry and padding.
///
/// Phase 93-01 (G1): no `Default` derive. Any "zero" instance is a
/// placeholder sentinel built manually (see
/// `resolve::validate_helpers::resolved_widget_border_sentinel`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedWidgetBorder {
    /// Border color.
    pub color: Rgba,
    /// Corner radius in logical pixels.
    pub corner_radius: f32,
    /// Border stroke width in logical pixels.
    pub line_width: f32,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: bool,
    /// Padding inside the border, per side; a side the theme does not state
    /// is `None`.
    #[serde(default)]
    pub padding: ResolvedPadding,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === DefaultsBorderSpec tests ===

    #[test]
    fn defaults_border_spec_default_is_empty() {
        assert!(DefaultsBorderSpec::default().is_empty());
    }

    #[test]
    fn defaults_border_spec_not_empty_when_color_set() {
        let bs = DefaultsBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            ..Default::default()
        };
        assert!(!bs.is_empty());
    }

    #[test]
    fn defaults_border_spec_toml_round_trip_full() {
        let bs = DefaultsBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            corner_radius: Some(4.0),
            corner_radius_lg: Some(8.0),
            line_width: Some(1.0),
            opacity: Some(0.15),
            shadow_enabled: Some(true),
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: DefaultsBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
    }

    #[test]
    fn defaults_border_spec_toml_round_trip_partial() {
        let bs = DefaultsBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(8.0),
            corner_radius_lg: None,
            line_width: None,
            opacity: None,
            shadow_enabled: None,
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: DefaultsBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
        assert!(deserialized.corner_radius_lg.is_none());
        assert!(deserialized.line_width.is_none());
        assert!(deserialized.opacity.is_none());
        assert!(deserialized.shadow_enabled.is_none());
    }

    #[test]
    fn defaults_border_spec_merge_overlay_wins() {
        let mut base = DefaultsBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(4.0),
            ..Default::default()
        };
        let overlay = DefaultsBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.color, Some(Rgba::rgb(200, 200, 200)));
        // base corner_radius preserved since overlay corner_radius is None
        assert_eq!(base.corner_radius, Some(4.0));
    }

    // === WidgetBorderSpec tests ===

    #[test]
    fn widget_border_spec_default_is_empty() {
        assert!(WidgetBorderSpec::default().is_empty());
    }

    #[test]
    fn widget_border_spec_not_empty_when_color_set() {
        let bs = WidgetBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            ..Default::default()
        };
        assert!(!bs.is_empty());
    }

    #[test]
    fn widget_border_spec_toml_round_trip_full() {
        let bs = WidgetBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            corner_radius: Some(4.0),
            line_width: Some(1.0),
            shadow_enabled: Some(true),
            padding_top: Some(5.0),
            padding_right: Some(6.0),
            padding_bottom: Some(7.0),
            padding_left: Some(8.0),
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: WidgetBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
    }

    #[test]
    fn widget_border_spec_toml_round_trip_partial() {
        let bs = WidgetBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(8.0),
            line_width: None,
            shadow_enabled: None,
            padding_top: None,
            padding_right: None,
            padding_bottom: None,
            padding_left: None,
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: WidgetBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
        assert!(deserialized.line_width.is_none());
        assert!(deserialized.shadow_enabled.is_none());
        assert!(deserialized.padding_top.is_none());
        assert!(deserialized.padding_right.is_none());
        assert!(deserialized.padding_bottom.is_none());
        assert!(deserialized.padding_left.is_none());
    }

    #[test]
    fn widget_border_spec_merge_overlay_wins() {
        let mut base = WidgetBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(4.0),
            ..Default::default()
        };
        let overlay = WidgetBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.color, Some(Rgba::rgb(200, 200, 200)));
        // base corner_radius preserved since overlay corner_radius is None
        assert_eq!(base.corner_radius, Some(4.0));
    }

    // === Per-side padding and its TOML shorthand ===

    #[test]
    fn padding_horizontal_shorthand_sets_left_and_right_only() {
        let bs: WidgetBorderSpec = toml::from_str("padding_horizontal_px = 10").unwrap();
        assert_eq!(bs.padding_left, Some(10.0));
        assert_eq!(bs.padding_right, Some(10.0));
        assert_eq!(bs.padding_top, None);
        assert_eq!(bs.padding_bottom, None);
    }

    #[test]
    fn padding_vertical_shorthand_states_a_zero() {
        let bs: WidgetBorderSpec = toml::from_str("padding_vertical_px = 0.0").unwrap();
        assert_eq!(bs.padding_top, Some(0.0));
        assert_eq!(bs.padding_bottom, Some(0.0));
        assert_eq!(bs.padding_left, None);
        assert_eq!(bs.padding_right, None);
    }

    #[test]
    fn an_axis_key_with_one_of_its_sides_is_rejected_naming_the_table_and_both_keys() {
        let src = "[light.button.border]\npadding_horizontal_px = 10\npadding_left_px = 4\n";
        let err = toml::from_str::<toml::Table>(src)
            .ok()
            .and_then(|t| t.get("light").cloned())
            .and_then(|l| l.get("button").cloned())
            .and_then(|b| b.get("border").cloned())
            .map(|border| border.try_into::<WidgetBorderSpec>())
            .expect("the table parses as TOML")
            .expect_err("an axis key with one of its sides must be rejected")
            .to_string();
        assert!(err.contains("padding_horizontal_px"), "{err}");
        assert!(err.contains("padding_left_px"), "{err}");

        // Through a whole theme, where the error also names the table.
        let theme = format!("name = \"T\"\n{src}");
        let err = crate::Theme::from_toml(&theme)
            .expect_err("the theme must not load")
            .to_string();
        assert!(err.contains("padding_horizontal_px"), "{err}");
        assert!(err.contains("padding_left_px"), "{err}");
        assert!(err.contains("light.button.border"), "{err}");
    }

    #[test]
    fn the_vertical_axis_key_with_a_side_is_rejected_too() {
        let err =
            toml::from_str::<WidgetBorderSpec>("padding_vertical_px = 2\npadding_bottom_px = 3")
                .expect_err("rejected")
                .to_string();
        assert!(err.contains("padding_vertical_px"), "{err}");
        assert!(err.contains("padding_bottom_px"), "{err}");
    }

    #[test]
    fn serialisation_writes_sides_never_the_shorthand() {
        let bs: WidgetBorderSpec =
            toml::from_str("padding_horizontal_px = 10\npadding_top_px = 3").unwrap();
        let out = toml::to_string(&bs).unwrap();
        assert!(out.contains("padding_left_px = 10"), "{out}");
        assert!(out.contains("padding_right_px = 10"), "{out}");
        assert!(out.contains("padding_top_px = 3"), "{out}");
        assert!(!out.contains("padding_horizontal_px"), "{out}");
        assert!(!out.contains("padding_bottom_px"), "{out}");
        let back: WidgetBorderSpec = toml::from_str(&out).unwrap();
        assert_eq!(back, bs);
    }

    #[test]
    fn a_readers_left_side_over_a_presets_shorthand_wins_for_left_only() {
        let mut preset: WidgetBorderSpec = toml::from_str("padding_horizontal_px = 10").unwrap();
        let reader = WidgetBorderSpec {
            padding_left: Some(4.0),
            ..Default::default()
        };
        preset.merge(&reader);
        assert_eq!(preset.padding_left, Some(4.0));
        assert_eq!(preset.padding_right, Some(10.0));
        assert_eq!(preset.padding_top, None);
        assert_eq!(preset.padding_bottom, None);
    }

    // === Resolved border tests ===

    #[test]
    fn resolved_border_struct_literals_compile() {
        // Field-name compile guard: any rename/remove breaks these literals.
        let defaults = ResolvedDefaultsBorder {
            color: Rgba::new(0, 0, 0, 0),
            corner_radius: 0.0,
            corner_radius_lg: 0.0,
            line_width: 0.0,
            opacity: 0.0,
            shadow_enabled: false,
        };
        assert_eq!(defaults.corner_radius_lg, 0.0);
        let widget = ResolvedWidgetBorder {
            color: Rgba::new(0, 0, 0, 0),
            corner_radius: 0.0,
            line_width: 0.0,
            shadow_enabled: false,
            padding: ResolvedPadding {
                top: None,
                right: Some(0.0),
                bottom: None,
                left: Some(4.0),
            },
        };
        assert_eq!(widget.padding.left, Some(4.0));
        assert_eq!(widget.padding.top, None);
        assert_eq!(ResolvedPadding::default().right, None);
    }

    /// A resolved border serialised without its padding (by a version that
    /// had none, or by hand) deserialises with every side unstated.
    #[test]
    fn resolved_widget_border_without_padding_deserialises_unstated() {
        let src =
            "color = \"#000000\"\ncorner_radius = 2.0\nline_width = 1.0\nshadow_enabled = false\n";
        let border: ResolvedWidgetBorder = toml::from_str(src).unwrap();
        assert_eq!(border.padding, ResolvedPadding::default());
    }
}
