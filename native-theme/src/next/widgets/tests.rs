//! The typed widgets' tables, patches and slots (S5, S6.1, S6.3), against
//! the catalogue's `button` entry. Every number here is synthetic.

use super::*;
use crate::next::blocks::focus::{FocusVisibleWhen, ResolvedFocusVisibleWhen};
use crate::next::blocks::motion::{
    PresentationEvent, PresentationTransition, ResolvedPresentationTransition,
};
use crate::next::blocks::part_style::{PartStyle, PartStylePatch};
use crate::next::blocks::{Apply, Table};
use crate::next::registry::{self, FieldInfo, FieldType, PartInfo};
use crate::next::selector::{WidgetOrPart, WidgetRef};
use crate::next::units::{Ms, Px, Sides};
use crate::next::value::Value;

fn written(field: &FieldInfo) -> &'static str {
    match field.ty {
        FieldType::Leaf(ty) | FieldType::Block { ty, .. } | FieldType::Patch { ty, .. } => ty,
    }
}

fn button() -> &'static registry::WidgetInfo {
    registry::widget("button").unwrap_or_else(|| panic!("button is not in the registry"))
}

fn part(name: &str) -> &'static PartInfo {
    button()
        .part(name)
        .unwrap_or_else(|| panic!("button has no part {name}"))
}

/// The catalogue's `button` entry: each part (its name, whether it may be
/// absent as a whole, its fields, and the fields its `may_be_absent` item
/// names), and each widget value with its type, as `ButtonValues` holds it,
/// and whether it may be absent. `menu_placement` and `scaffold_placement`
/// are deferred to the placement unit (scripts/widget_units.toml).
const PARTS: &[(&str, bool, &[&str], &[&str])] = &[
    (
        "body",
        false,
        &[
            "fill",
            "stroke",
            "corners",
            "shadow",
            "padding",
            "margin",
            "min_width",
            "min_height",
            "max_width",
            "opacity",
            "transform",
            "focus",
            "cursor",
            "touch_target",
            "bindings",
            "transitions",
            "interaction",
        ],
        &["shadow", "cursor"],
    ),
    ("label", false, &["text", "padding"], &[]),
    ("subtitle", true, &["text"], &[]),
    ("icon", true, &["icon", "glyph", "transitions"], &["glyph"]),
    (
        "menu_indicator",
        true,
        &[
            "glyph",
            "min_width",
            "min_height",
            "margin",
            "transform",
            "transitions",
        ],
        &[],
    ),
];

const VALUES: &[(&str, &str, bool)] = &[
    ("icon_text_gap", "Value<Px>", false),
    ("icon_placement", "Value<IconPlacement>", false),
    ("label_position", "Value<ButtonLabelPosition>", true),
    ("subtitle_gap", "Value<Px>", true),
    ("menu_indicator_gap", "Value<Px>", false),
    ("menu_indicator_anchor", "Value<IndicatorAnchor>", false),
    ("menu_trigger", "Value<MenuTrigger>", true),
    ("shows_icon", "Value<bool>", true),
    ("focus_visible_when", "Value<FocusVisibleWhen>", false),
    ("repeat_delay", "Value<Ms>", true),
    ("repeat_interval", "Value<Ms>", true),
    ("back_title_fallback", "Value<bool>", false),
    ("back_display_mode", "Value<BackDisplayMode>", true),
    ("help_tooltip_on_click", "Value<bool>", false),
    ("hit_extension", "Sides<Value<Px>>", false),
    ("display_mode", "Value<PasteDisplayMode>", true),
    ("presentations", "Vec<PresentationTransition>", false),
];

#[test]
fn button_parts_match_catalogue() {
    let widget = button();
    assert_eq!(widget.kind, WidgetKind::Button);
    let names: Vec<_> = widget.parts.iter().map(|p| p.name).collect();
    let expected: Vec<_> = PARTS.iter().map(|(name, ..)| *name).collect();
    assert_eq!(names, expected);
    for (name, may_be_absent, fields, absent) in PARTS {
        let part = part(name);
        assert_eq!(part.root, *name == "body", "{name}");
        assert!(!part.element, "{name}");
        assert_eq!(part.parent, None, "{name}");
        assert_eq!(part.may_be_absent, *may_be_absent, "{name}");
        let paths: Vec<_> = part.fields.iter().map(|f| f.path).collect();
        assert_eq!(paths, *fields, "{name}");
        for field in part.fields {
            let style = registry::find(PartStyle::FIELDS, field.path)
                .unwrap_or_else(|| panic!("{name}.{} is no PartStyle field", field.path));
            assert_eq!(written(field), written(style), "{name}.{}", field.path);
            assert_eq!(field.auto, style.auto, "{name}.{}", field.path);
            assert_eq!(
                field.may_be_absent,
                style.may_be_absent || absent.contains(&field.path),
                "{name}.{}",
                field.path
            );
        }
    }
    // The catalogue's field marks name drawing fields, which S6.3 marks on
    // every part already.
    for path in ["shadow", "cursor", "glyph"] {
        assert!(
            registry::find(PartStyle::FIELDS, path).is_some_and(|f| f.may_be_absent),
            "{path}"
        );
    }

    let values: Vec<_> = widget
        .values
        .iter()
        .map(|f| (f.path, written(f), f.may_be_absent))
        .collect();
    assert_eq!(values, VALUES);
    let hit = registry::find(widget.values, "hit_extension.top");
    assert_eq!(hit.map(|f| f.may_be_absent), Some(true));
    assert!(widget.values.iter().all(|f| !f.auto));
    for deferred in ["menu_placement", "scaffold_placement"] {
        assert!(
            registry::find(widget.values, deferred).is_none(),
            "{deferred}"
        );
    }

    assert_eq!(widget.variants.first(), Some(&Variant::Primary));
    for variant in [
        Variant::Flat,
        Variant::Toggle,
        Variant::Menu,
        Variant::IconOnly,
        Variant::WithIcon,
        Variant::Close,
        Variant::ClearField,
        Variant::Route,
    ] {
        assert!(widget.variants.contains(&variant), "{variant:?}");
    }
    assert_eq!(widget.variants.len(), 34);
}

#[test]
fn registry_has_button_body_fill() {
    let fill = registry::find(part("body").fields, "fill");
    assert_eq!(
        fill.map(|f| (written(f), f.may_be_absent, f.auto)),
        Some(("Value<Paint>", true, false))
    );
    assert!(registry::find(part("body").fields, "fill.layers.blend").is_some());
    assert!(registry::find(part("label").fields, "fill").is_none());
    assert!(registry::find(part("label").fields, "text.font.size").is_some());
    assert!(registry::find(part("body").fields, "max_width").is_some_and(|f| f.auto));
    assert!(registry::find(button().values, "icon_text_gap").is_some());
    assert!(registry::widget("slider").is_none());
}

fn style(opacity: f32) -> PartStylePatch {
    PartStylePatch {
        opacity: Some(Value::Stated(crate::next::units::Fraction(opacity))),
        ..PartStylePatch::default()
    }
}

fn values(gap: f32) -> ButtonValuesPatch {
    ButtonValuesPatch {
        icon_text_gap: Some(Value::Stated(Px(gap))),
        ..ButtonValuesPatch::default()
    }
}

#[test]
fn button_patch_splits_into_its_parts_in_catalogue_order() {
    let patch = ButtonPatch {
        menu_indicator: Some(PartPatch::Absent),
        body: Some(PartPatch::Style(style(0.5))),
        values: Some(values(4.0)),
        ..ButtonPatch::default()
    };
    let (parts, values_patch) = patch.into_parts();
    assert_eq!(
        parts,
        vec![
            (ButtonPart::Body, PartPatch::Style(style(0.5))),
            (ButtonPart::MenuIndicator, PartPatch::Absent),
        ]
    );
    assert_eq!(values_patch, Some(values(4.0)));
}

#[test]
fn button_patches_merge_in_order() {
    let mut slots = WidgetSlots::default();
    let base = WidgetsPatch {
        button: Some(ButtonPatch {
            body: Some(PartPatch::Style(PartStylePatch {
                min_width: Some(Value::Stated(Px(10.0))),
                ..style(0.5)
            })),
            label: Some(PartPatch::Style(style(1.0))),
            values: Some(ButtonValuesPatch {
                repeat_delay: Some(Value::Stated(Ms(300.0))),
                ..values(4.0)
            }),
            ..ButtonPatch::default()
        }),
    };
    assert_eq!(slots.apply(base), Ok(()));
    let later = WidgetsPatch {
        button: Some(ButtonPatch {
            body: Some(PartPatch::Absent),
            values: Some(ButtonValuesPatch {
                hit_extension: Some(Sides {
                    top: Some(Value::Absent),
                    right: None,
                    bottom: None,
                    left: None,
                }),
                ..values(6.0)
            }),
            ..ButtonPatch::default()
        }),
    };
    assert_eq!(slots.apply(later), Ok(()));
    // A style after `Absent` brings the part back with the fields stated
    // below the `Absent` (S6.3).
    assert_eq!(
        slots.apply_part(
            TypedPart::Button(ButtonPart::Body),
            PartPatch::Style(style(0.8))
        ),
        Ok(())
    );
    let drawn_as = PartPatch::DrawnAs {
        target: WidgetOrPart::Widget(WidgetRef::Typed(WidgetKind::Button)),
        variants: Vec::new(),
    };
    assert_eq!(
        slots.apply_part(TypedPart::Button(ButtonPart::Subtitle), drawn_as.clone()),
        Ok(())
    );
    assert_eq!(
        slots.apply_values(WidgetValuesPatch::Button(ButtonValuesPatch {
            presentations: Some(vec![presentation(PresentationEvent::Appear)]),
            ..ButtonValuesPatch::default()
        })),
        Ok(())
    );

    let body = PartSlot {
        shown: Shown::Drawn,
        style: Some(PartStylePatch {
            min_width: Some(Value::Stated(Px(10.0))),
            ..style(0.8)
        }),
    };
    let label = PartSlot {
        shown: Shown::Drawn,
        style: Some(style(1.0)),
    };
    let subtitle = PartSlot {
        shown: Shown::DrawnAs {
            target: WidgetOrPart::Widget(WidgetRef::Typed(WidgetKind::Button)),
            variants: Vec::new(),
        },
        style: None,
    };
    assert_eq!(
        slots.button.parts,
        vec![
            (ButtonPart::Body, body),
            (ButtonPart::Label, label),
            (ButtonPart::Subtitle, subtitle),
        ]
    );
    assert_eq!(
        slots.button.values,
        Some(ButtonValuesPatch {
            repeat_delay: Some(Value::Stated(Ms(300.0))),
            hit_extension: Some(Sides {
                top: Some(Value::Absent),
                right: None,
                bottom: None,
                left: None,
            }),
            presentations: Some(vec![presentation(PresentationEvent::Appear)]),
            ..values(6.0)
        })
    );
}

#[test]
fn an_absent_part_keeps_its_style_until_brought_back() {
    let mut slot = PartSlot::default();
    assert_eq!(slot.apply(PartPatch::Style(style(0.3))), Ok(()));
    assert_eq!(slot.apply(PartPatch::Absent), Ok(()));
    assert_eq!(
        slot,
        PartSlot {
            shown: Shown::Absent,
            style: Some(style(0.3)),
        }
    );
    assert_eq!(
        slot.apply(PartPatch::Style(PartStylePatch::default())),
        Ok(())
    );
    assert_eq!(slot.shown, Shown::Drawn);
    assert_eq!(slot.style, Some(style(0.3)));
}

fn presentation(event: PresentationEvent) -> PresentationTransition {
    PresentationTransition {
        event,
        steps: Vec::new(),
    }
}

#[test]
fn button_values_resolve_by_type() {
    let resolved = ResolvedButtonValues {
        icon_text_gap: Some(Px(4.0)),
        icon_placement: Some(IconPlacement::Leading),
        label_position: None,
        subtitle_gap: None,
        menu_indicator_gap: Some(Px(2.0)),
        menu_indicator_anchor: Some(IndicatorAnchor::Inline),
        menu_trigger: Some(MenuTrigger::LongPressOrSecondaryClick),
        shows_icon: Some(true),
        focus_visible_when: Some(ResolvedFocusVisibleWhen::AnyFocus),
        repeat_delay: Some(Ms(300.0)),
        repeat_interval: None,
        back_title_fallback: Some(false),
        back_display_mode: Some(BackDisplayMode::Minimal),
        help_tooltip_on_click: Some(false),
        hit_extension: Sides {
            top: Some(Px(1.0)),
            right: None,
            bottom: None,
            left: None,
        },
        display_mode: Some(PasteDisplayMode::IconAndLabel),
        presentations: Vec::<ResolvedPresentationTransition>::new(),
    };
    let _: Option<ButtonLabelPosition> = resolved.label_position;
    assert_eq!(resolved.icon_text_gap, Some(Px(4.0)));

    // Created where every value left unset may be `Absent` (S6.1), those
    // values then `Absent`; a value left unset that may not be is the merge
    // conflict.
    let stated = ButtonValuesPatch {
        icon_placement: Some(Value::Stated(IconPlacement::Leading)),
        menu_indicator_gap: Some(Value::Stated(Px(2.0))),
        menu_indicator_anchor: Some(Value::Stated(IndicatorAnchor::Inline)),
        focus_visible_when: Some(Value::Stated(FocusVisibleWhen::Never)),
        back_title_fallback: Some(Value::Stated(false)),
        help_tooltip_on_click: Some(Value::Stated(false)),
        presentations: Some(Vec::new()),
        ..values(4.0)
    };
    let created = <ButtonValues as crate::next::blocks::Create>::create(stated.clone());
    assert_eq!(
        created.map(|v| (v.icon_text_gap, v.label_position, v.hit_extension.top)),
        Ok((Value::Stated(Px(4.0)), Value::Absent, Value::Absent))
    );
    let created = <ButtonValues as crate::next::blocks::Create>::create(ButtonValuesPatch {
        back_title_fallback: None,
        ..stated
    });
    assert_eq!(
        created.map(|_| ()),
        Err(crate::next::blocks::MergeConflict {
            path: "back_title_fallback".to_owned(),
            below: crate::next::blocks::Below::Missing,
        })
    );
}

#[test]
fn indicator_anchor_inline_top_reads() {
    // ADD-A123: an indicator inline with the content, at its top.
    let layer = crate::next::Layer::from_toml(
        "[widgets.button.values]\nmenu_indicator_anchor = \"inline_top\"",
    )
    .unwrap_or_else(|error| panic!("{error}"));
    let anchor = layer
        .bases
        .widgets
        .button
        .and_then(|button| button.values)
        .and_then(|values| values.menu_indicator_anchor);
    assert_eq!(anchor, Some(Value::Stated(IndicatorAnchor::InlineTop)));
}

#[test]
fn a_drawn_as_part_keeps_its_style_until_brought_back() {
    // ADD-A123: a `Style` after a `DrawnAs` brings the part back as after
    // `Absent`, the fields stated before the `DrawnAs` kept.
    let mut slot = PartSlot::default();
    assert_eq!(slot.apply(PartPatch::Style(style(0.3))), Ok(()));
    let drawn_as = PartPatch::DrawnAs {
        target: WidgetOrPart::Widget(WidgetRef::Typed(WidgetKind::Button)),
        variants: Vec::new(),
    };
    assert_eq!(slot.apply(drawn_as), Ok(()));
    assert_eq!(slot.style, Some(style(0.3)));
    let width = PartStylePatch {
        min_width: Some(Value::Stated(Px(8.0))),
        ..PartStylePatch::default()
    };
    assert_eq!(slot.apply(PartPatch::Style(width)), Ok(()));
    assert_eq!(
        slot,
        PartSlot {
            shown: Shown::Drawn,
            style: Some(PartStylePatch {
                min_width: Some(Value::Stated(Px(8.0))),
                ..style(0.3)
            }),
        }
    );
}

/// The button's content variants are the catalogue's pairs (*Entry
/// format*), generated: typed, and GTK's keyed ones with their `@`.
#[test]
fn button_content_variants_match_catalogue() {
    assert_eq!(
        button().content_variants,
        [
            (None, "icon_only"),
            (None, "with_icon"),
            (None, "@custom_child"),
            (None, "@arrow_only"),
            (None, "@with_arrow")
        ]
    );
}
