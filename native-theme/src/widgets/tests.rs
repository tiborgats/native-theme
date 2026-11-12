//! The typed widgets' tables, patches and slots (S5, S6.1, S6.3), against
//! the catalogue's `button` entry. Every number here is synthetic.

use super::*;
use crate::blocks::focus::{FocusVisibleWhen, ResolvedFocusVisibleWhen};
use crate::blocks::part_style::{PartStyle, PartStylePatch};
use crate::blocks::{Apply, Table};
use crate::registry::{self, FieldInfo, FieldType, PartInfo};
use crate::selector::{WidgetOrPart, WidgetRef};
use crate::units::{Ms, Px, Sides};
use crate::value::Value;

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
/// are deferred to the placement unit, the values only later names read,
/// `presentations` and the parts' `transitions` to their later units
/// (scripts/widget_units.toml; S6.1, ADD-A147).
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
            "interaction",
        ],
        &["shadow", "cursor"],
    ),
    ("label", false, &["text", "padding"], &[]),
    ("subtitle", true, &["text"], &[]),
    ("icon", true, &["icon", "glyph"], &["glyph"]),
    (
        "menu_indicator",
        true,
        &["glyph", "min_width", "min_height", "margin", "transform"],
        &[],
    ),
];

const VALUES: &[(&str, &str, bool)] = &[
    ("icon_text_gap", "Value<Px>", false),
    ("icon_placement", "Value<IconPlacement>", false),
    ("label_position", "Value<ButtonLabelPosition>", true),
    ("subtitle_gap", "Value<Px>", true),
    ("shows_icon", "Value<bool>", true),
    ("focus_visible_when", "Value<FocusVisibleWhen>", false),
    ("repeat_delay", "Value<Ms>", true),
    ("repeat_interval", "Value<Ms>", true),
    ("hit_extension", "Sides<Value<Px>>", false),
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
    for deferred in [
        "menu_placement",
        "scaffold_placement",
        "menu_indicator_gap",
        "back_title_fallback",
        "presentations",
    ] {
        assert!(
            registry::find(widget.values, deferred).is_none(),
            "{deferred}"
        );
    }

    // The variants of the Depth line alone (S6.1, ADD-A147): no variant of
    // a later unit is a member.
    assert_eq!(
        widget.variants,
        [
            Variant::Primary,
            Variant::Tinted,
            Variant::Toggle,
            Variant::Tool,
            Variant::WithIcon,
        ]
    );
    assert_eq!(widget.content_variants, [(None, "with_icon")]);
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
        opacity: Some(Value::Stated(crate::units::Fraction(opacity))),
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
        menu_indicator: Some(PartPatch::of_presence(PartPresence::Absent)),
        body: Some(PartPatch::of_style(style(0.5))),
        values: Some(values(4.0)),
        ..ButtonPatch::default()
    };
    let (parts, values_patch) = patch.into_parts();
    assert_eq!(
        parts,
        vec![
            (ButtonPart::Body, PartPatch::of_style(style(0.5))),
            (
                ButtonPart::MenuIndicator,
                PartPatch::of_presence(PartPresence::Absent)
            ),
        ]
    );
    assert_eq!(values_patch, Some(values(4.0)));
}

#[test]
fn button_patches_merge_in_order() {
    let mut slots = WidgetSlots::default();
    let base = WidgetsPatch {
        button: Some(Box::new(ButtonPatch {
            body: Some(PartPatch::of_style(PartStylePatch {
                min_width: Some(Value::Stated(Px(10.0))),
                ..style(0.5)
            })),
            label: Some(PartPatch::of_style(style(1.0))),
            values: Some(ButtonValuesPatch {
                repeat_delay: Some(Value::Stated(Ms(300.0))),
                ..values(4.0)
            }),
            ..ButtonPatch::default()
        })),
    };
    assert_eq!(slots.apply(base), Ok(()));
    let later = WidgetsPatch {
        button: Some(Box::new(ButtonPatch {
            body: Some(PartPatch::of_presence(PartPresence::Absent)),
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
        })),
    };
    assert_eq!(slots.apply(later), Ok(()));
    // `Present` after `Absent` draws the part again with the fields
    // stated below the `Absent` and beside it (S6.3).
    assert_eq!(
        slots.apply_part(
            TypedPart::Button(ButtonPart::Body),
            PartPatch {
                presence: Some(PartPresence::Present),
                style: Box::new(style(0.8)),
            }
        ),
        Ok(())
    );
    let drawn_as = PartPatch::of_presence(PartPresence::DrawnAs {
        target: WidgetOrPart::Widget(WidgetRef::Typed(WidgetKind::Button)),
        variants: Vec::new(),
    });
    assert_eq!(
        slots.apply_part(TypedPart::Button(ButtonPart::Subtitle), drawn_as.clone()),
        Ok(())
    );
    assert_eq!(
        slots.apply_values(WidgetValuesPatch::Button(Box::new(ButtonValuesPatch {
            shows_icon: Some(Value::Stated(true)),
            ..ButtonValuesPatch::default()
        }))),
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
            shows_icon: Some(Value::Stated(true)),
            ..values(6.0)
        })
    );
}

#[test]
fn an_absent_part_keeps_its_style_until_brought_back() {
    let mut slot = PartSlot::default();
    assert_eq!(slot.apply(PartPatch::of_style(style(0.3))), Ok(()));
    assert_eq!(
        slot.apply(PartPatch::of_presence(PartPresence::Absent)),
        Ok(())
    );
    assert_eq!(
        slot,
        PartSlot {
            shown: Shown::Absent,
            style: Some(style(0.3)),
        }
    );
    // A patch stating no presence sets fields only (S6.3).
    assert_eq!(slot.apply(PartPatch::default()), Ok(()));
    assert_eq!(slot.shown, Shown::Absent);
    assert_eq!(
        slot.apply(PartPatch::of_presence(PartPresence::Present)),
        Ok(())
    );
    assert_eq!(slot.shown, Shown::Drawn);
    assert_eq!(slot.style, Some(style(0.3)));
}

#[test]
fn button_values_resolve_by_type() {
    let resolved = ResolvedButtonValues {
        icon_text_gap: Some(Px(4.0)),
        icon_placement: Some(IconPlacement::Leading),
        label_position: None,
        subtitle_gap: None,
        shows_icon: Some(true),
        focus_visible_when: Some(ResolvedFocusVisibleWhen::AnyFocus),
        repeat_delay: Some(Ms(300.0)),
        repeat_interval: None,
        hit_extension: Sides {
            top: Some(Px(1.0)),
            right: None,
            bottom: None,
            left: None,
        },
    };
    let _: Option<ButtonLabelPosition> = resolved.label_position;
    assert_eq!(resolved.icon_text_gap, Some(Px(4.0)));

    // Created where every value left unset may be `Absent` (S6.1), those
    // values then `Absent`; a value left unset that may not be is the merge
    // conflict.
    let stated = ButtonValuesPatch {
        icon_placement: Some(Value::Stated(IconPlacement::Leading)),
        focus_visible_when: Some(Value::Stated(FocusVisibleWhen::Never)),
        ..values(4.0)
    };
    let created = <ButtonValues as crate::blocks::Create>::create(stated.clone());
    assert_eq!(
        created.map(|v| (v.icon_text_gap, v.label_position, v.hit_extension.top)),
        Ok((Value::Stated(Px(4.0)), Value::Absent, Value::Absent))
    );
    let created = <ButtonValues as crate::blocks::Create>::create(ButtonValuesPatch {
        icon_placement: None,
        ..stated
    });
    assert_eq!(
        created.map(|_| ()),
        Err(crate::blocks::MergeConflict {
            path: "icon_placement".to_owned(),
            below: crate::blocks::Below::Missing,
        })
    );
}

#[test]
fn a_deferred_value_is_no_value_of_the_model() {
    // The values only later names read leave the values struct until their
    // unit (S6.1, ADD-A147): a layer stating one is refused.
    let error =
        crate::Layer::from_toml("[widgets.button.values]\nmenu_indicator_anchor = \"inline_top\"");
    assert!(error.is_err(), "{error:?}");
}

#[test]
fn a_drawn_as_part_keeps_its_style_until_brought_back() {
    // `Present` after a `DrawnAs` draws the part again as after `Absent`,
    // the fields stated before the `DrawnAs` kept (S6.3, ADD-A126); a
    // patch of fields alone leaves it drawn as the other widget.
    let mut slot = PartSlot::default();
    assert_eq!(slot.apply(PartPatch::of_style(style(0.3))), Ok(()));
    let drawn_as = PartPatch::of_presence(PartPresence::DrawnAs {
        target: WidgetOrPart::Widget(WidgetRef::Typed(WidgetKind::Button)),
        variants: Vec::new(),
    });
    assert_eq!(slot.apply(drawn_as), Ok(()));
    assert_eq!(slot.style, Some(style(0.3)));
    let width = PartStylePatch {
        min_width: Some(Value::Stated(Px(8.0))),
        ..PartStylePatch::default()
    };
    assert_eq!(slot.apply(PartPatch::of_style(width)), Ok(()));
    assert!(matches!(slot.shown, Shown::DrawnAs { .. }));
    assert_eq!(
        slot.apply(PartPatch::of_presence(PartPresence::Present)),
        Ok(())
    );
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
    // The content variants at depth (S6.1, ADD-A147): the others are the
    // content unit's (scripts/widget_units.toml).
    assert_eq!(button().content_variants, [(None, "with_icon")]);
}
