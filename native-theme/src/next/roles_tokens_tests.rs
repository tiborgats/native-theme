//! S3's colour roles, S4's tokens and S9's inputs: their tables, marks and
//! merges (S6.1, S6.3). Every number here is synthetic.

use super::blocks::focus::FocusIndicator;
use super::blocks::focus::FocusVisibleWhen;
use super::blocks::icon::IconStyle;
use super::blocks::motion::Timing;
use super::blocks::shadow::ShadowLayer;
use super::blocks::text::TextStyle;
use super::blocks::{Apply, BlockPatch, Merge, MergeConflict, Table};
use super::color::Color;
use super::inputs::{Accent, Contrast, InputBases, InputName, InputValue, KdeSchemeKey, KdeWmKey};
use super::key::Key;
use super::registry::{self, BlockForm, FieldInfo, FieldType, join, leaves};
use super::roles::{
    ColorRolesPatch, DisabledRoles, GlobalRole, GlobalRoles, StatusRoles, SurfaceRole,
    SurfaceRoles, SurfaceRolesPatch, SurfaceSet,
};
use super::selector::Scheme;
use super::tokens::{
    Behavior, BehaviorPatch, Caret, IconSizeTokens, IconTokens, KdeEffects, KdeTokens, KeyRepeat,
    MaterialScheme, MaterialVariant, Mnemonics, PointerTokens, TextRendering, ThemeChange, Tokens,
    TokensPatch, TouchBehavior,
};
use super::units::{Ms, Scalar};
use super::value::{SchemeMap, SurfaceMap, Value};
use crate::key;

/// The blocks of S2 a token holds, which their own tests cover.
fn s2_blocks() -> [(&'static str, &'static [FieldInfo]); 6] {
    [
        ("FocusIndicator", FocusIndicator::FIELDS),
        ("FocusVisibleWhen", FocusVisibleWhen::FIELDS),
        ("IconStyle", IconStyle::FIELDS),
        ("ShadowLayer", ShadowLayer::FIELDS),
        ("TextStyle", TextStyle::FIELDS),
        ("Timing", Timing::FIELDS),
    ]
}

fn written(field: &FieldInfo) -> &'static str {
    match field.ty {
        FieldType::Leaf(ty) | FieldType::Block { ty, .. } => ty,
    }
}

/// Whether two tables hold the same fields, marks aside.
fn same_fields(a: &[FieldInfo], b: &[FieldInfo]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(a, b)| a.path == b.path && written(a) == written(b))
}

/// Every leaf path below `fields`, a field holding one of S2's blocks
/// written `<path>: <block>` and not walked into.
fn token_paths(prefix: &str, fields: &'static [FieldInfo], out: &mut Vec<String>) {
    for field in fields {
        let path = join(prefix, field.path);
        match field.ty {
            FieldType::Block { form, fields, .. } => {
                if let Some((name, _)) = s2_blocks()
                    .into_iter()
                    .find(|(_, table)| same_fields(fields, table))
                {
                    out.push(format!("{path}: {name}"));
                    continue;
                }
                if matches!(form, BlockForm::Enum { unit_variants } if !unit_variants.is_empty()) {
                    out.push(path.clone());
                }
                token_paths(&path, fields, out);
            }
            FieldType::Leaf(_) => out.push(path),
        }
    }
}

fn kde_effects(prefix: &str) -> Vec<String> {
    [
        "intensity",
        "intensity_amount",
        "color",
        "color_amount",
        "color_color",
        "contrast",
        "contrast_amount",
    ]
    .iter()
    .map(|path| join(prefix, path))
    .collect()
}

/// Every token path S4 states, down to S2's blocks; `Behavior.scroll` is
/// staged with the `scroll_view` unit (S6.1).
fn token_spec_paths() -> Vec<String> {
    let mut out: Vec<String> = [
        "motion.durations",
        "motion.timings: Timing",
        "motion.speed_factor",
        "theme_change",
        "theme_change.crossfade.duration",
        "theme_change.crossfade.timing: Timing",
        "theme_change.suspend.duration",
        "elevation: ShadowLayer",
    ]
    .map(str::to_owned)
    .to_vec();
    for slot in [
        "body",
        "body_emphasized",
        "caption",
        "caption_emphasized",
        "caption2",
        "footnote",
        "subheadline",
        "callout",
        "headline",
        "title1",
        "title2",
        "title3",
        "large_title",
        "display_large",
        "display_medium",
        "display_small",
        "label",
        "monospace",
        "numeric",
    ] {
        out.push(format!("text.roles.{slot}: TextStyle"));
    }
    out.extend(
        [
            "text.keyed: TextStyle",
            "text.fallbacks.families",
            "text.fallbacks.emoji_family",
            "text.fallbacks.per_language",
            "text.fallbacks.per_family",
            "text.rendering.engine",
            "text.rendering.antialiasing",
            "text.rendering.hinting",
            "text.rendering.subpixel_order",
            "text.rendering.lcd_filter",
            "text.rendering.gamma",
            "text.rendering.contrast",
            "text.scaling",
            "text.font_dpi",
            "icons.set",
            "icons.theme",
            "icons.theme_fallback",
            "icons.sizes.toolbar: IconStyle",
            "icons.sizes.small: IconStyle",
            "icons.sizes.large: IconStyle",
            "icons.sizes.dialog: IconStyle",
            "icons.sizes.panel: IconStyle",
            "icons.sizes.keyed: IconStyle",
            "icons.mirroring",
            "focus: FocusIndicator",
            "caret.color",
            "caret.width",
            "caret.blink_on",
            "caret.blink_off",
            "caret.blink_timeout",
            "caret.blink_style",
            "caret.secondary",
            "caret.shown_in_read_only",
            "caret.solid_while_typing",
            "behavior.double_click_time",
            "behavior.double_click_distance",
            "behavior.drag_threshold",
            "behavior.hover_open_delay",
            "behavior.key_repeat.delay",
            "behavior.key_repeat.interval",
            "behavior.scroll_lines_per_step",
            "behavior.scroll_px_per_step",
            "behavior.hide_pointer_while_typing",
            "behavior.hide_pointer_delay",
            "behavior.hide_pointer_fade",
            "behavior.long_press",
            "behavior.spring_loading",
            "behavior.drag_time",
            "behavior.single_click_activates",
            "behavior.double_click_height",
            "behavior.touch.slop",
            "behavior.touch.paging_slop",
            "behavior.touch.double_tap_time",
            "behavior.touch.double_tap_slop",
            "behavior.touch.delay",
            "behavior.mnemonics",
            "behavior.mnemonics.on_alt.delay",
            "behavior.focus_visible_when: FocusVisibleWhen",
            "size_classes.horizontal",
            "size_classes.vertical",
            "params",
            "pointer.theme",
            "pointer.size",
            "pointer.color",
            "pointer.drag_cursors",
        ]
        .map(str::to_owned),
    );
    out.extend(kde_effects("kde.disabled"));
    out.extend(kde_effects("kde.inactive"));
    out.extend(
        [
            "kde.inactive_enabled",
            "kde.inactive_change_selection_color",
            "kde.tint_factor",
            "kde.titlebar_is_accent_colored",
            "material.variant",
            "material.spec_version",
            "material.contrast_level",
        ]
        .map(str::to_owned),
    );
    out
}

#[test]
fn tokens_registry_has_slice_groups() {
    let mut found = Vec::new();
    token_paths("", registry::tokens(), &mut found);
    let expected = token_spec_paths();
    for (index, (found, expected)) in found.iter().zip(&expected).enumerate() {
        assert_eq!(found, expected, "path {index}");
    }
    assert_eq!(found.len(), expected.len());
    for path in [
        "params.opacity.disabled",
        "params.color.contrast",
        "motion.durations.button",
        "elevation.1.blur",
        "text.keyed.body.font.size",
        "text.roles.body.font.size",
        "icons.sizes.keyed.main_toolbar.size",
        "size_classes.compact.horizontal",
        "behavior.focus_visible_when.keyboard.timeout",
        "theme_change.instant",
    ] {
        assert!(registry::find(registry::tokens(), path).is_some(), "{path}");
    }
    for path in [
        "behavior.scroll",
        "params",
        "caret.shape",
        "icons.sizes.keyed",
    ] {
        let found = registry::find(registry::tokens(), path);
        assert_eq!(
            found.is_some(),
            matches!(path, "params" | "icons.sizes.keyed"),
            "{path}"
        );
    }
}

#[test]
fn every_s3_role_path_resolves_in_the_registry() {
    for set in SurfaceSet::NAMES {
        for role in SurfaceRole::ALL {
            let path = format!("surfaces.{set}.{}", role.path());
            let field = registry::find(registry::colors(), &path);
            assert!(
                field.is_some_and(|f| written(f) == "Value<Color>"),
                "{path}"
            );
            assert_eq!(SurfaceRole::from_path(role.path()), Some(*role));
        }
    }
    for role in GlobalRole::ALL {
        let path = format!("global.{}", role.path());
        let field = registry::find(registry::colors(), &path);
        assert!(
            field.is_some_and(|f| written(f) == "Value<Color>"),
            "{path}"
        );
        assert_eq!(GlobalRole::from_path(role.path()), Some(*role));
    }
    for path in [
        "global.accent_ramp.light1",
        "keyed.shade_light",
        "keyed.wm_active_background",
    ] {
        assert!(registry::find(registry::colors(), path).is_some(), "{path}");
    }
    for path in [
        "surfaces.background",
        "global.status.error",
        "global.accent.x",
        "role",
    ] {
        let field = registry::find(registry::colors(), path);
        assert!(field.is_none_or(|f| written(f) != "Value<Color>"), "{path}");
    }

    let mut surface = Vec::new();
    leaves("", SurfaceRoles::FIELDS, &mut surface);
    let roles: Vec<_> = SurfaceRole::ALL.iter().map(|r| r.path()).collect();
    assert_eq!(surface, roles);
    let mut global = Vec::new();
    leaves("", GlobalRoles::FIELDS, &mut global);
    global.retain(|path| path != "accent_ramp");
    let roles: Vec<_> = GlobalRole::ALL.iter().map(|r| r.path()).collect();
    assert_eq!(global, roles);
    let mut disabled = Vec::new();
    leaves("disabled", DisabledRoles::FIELDS, &mut disabled);
    let active: Vec<_> = SurfaceRole::ALL
        .iter()
        .map(|r| r.path())
        .filter(|p| !p.starts_with("disabled."))
        .map(|p| format!("disabled.{p}"))
        .collect();
    assert_eq!(disabled, active);
    let mut statuses = Vec::new();
    leaves("", StatusRoles::FIELDS, &mut statuses);
    assert_eq!(statuses.len(), 30);
    assert_eq!(statuses.first().map(String::as_str), Some("success.fill"));
    assert_eq!(
        statuses.last().map(String::as_str),
        Some("destructive.background_solid")
    );
}

/// Holds the KDE and Material fields S4.8 and S9 state as their types.
fn kde_and_material_types(tokens: Tokens, kde: KdeTokens, effects: KdeEffects, bases: InputBases) {
    let _: Value<KdeTokens> = tokens.kde;
    let _: Value<MaterialScheme> = tokens.material;
    let _: KdeEffects = kde.disabled;
    let _: KdeEffects = kde.inactive;
    let _: Value<bool> = kde.inactive_enabled;
    let _: Value<bool> = kde.inactive_change_selection_color;
    let _: Option<Value<f32>> = kde.tint_factor;
    let _: Value<bool> = kde.titlebar_is_accent_colored;
    let _: super::tokens::KdeIntensityEffect = effects.intensity;
    let _: Value<f32> = effects.intensity_amount;
    let _: super::tokens::KdeColorEffect = effects.color;
    let _: Value<f32> = effects.color_amount;
    let _: Value<Color> = effects.color_color;
    let _: super::tokens::KdeContrastEffect = effects.contrast;
    let _: Value<f32> = effects.contrast_amount;
    let _: Vec<(InputName, InputValue)> = bases.entries;
    let _: SchemeMap<Vec<SurfaceSet>> = bases.kde_groups;
    let _: SchemeMap<Vec<SurfaceSet>> = bases.kde_inactive_groups;
    let _: SchemeMap<Vec<InputName>> = bases.kde_unset_keys;
}

fn material_types(material: MaterialScheme, value: InputValue, accent: Accent) {
    let _: MaterialVariant = material.variant;
    let _: Key = material.spec_version;
    let _: Value<f32> = material.contrast_level;
    let _: SchemeMap<Value<Color>> = value.per_scheme;
    let _: Vec<(Contrast, SchemeMap<Value<Color>>)> = value.per_contrast;
    let _: InputValue = accent.accent;
    let _: Vec<(InputName, InputValue)> = accent.derived;
}

#[test]
fn kde_and_material_fields_present_as_types() {
    let _ = kde_and_material_types as fn(Tokens, KdeTokens, KdeEffects, InputBases);
    let _ = material_types as fn(MaterialScheme, InputValue, Accent);
    assert_eq!(
        KdeSchemeKey::ALL.map(KdeSchemeKey::name),
        KdeSchemeKey::NAMES
    );
    assert_eq!(KdeWmKey::ALL.map(KdeWmKey::name), KdeWmKey::NAMES);
    let table: Vec<_> = KdeSchemeKey::ALL
        .iter()
        .map(|key| (key.name(), key.role().path()))
        .collect();
    assert_eq!(
        table,
        [
            ("background_normal", "background"),
            ("background_alternate", "background_alternate"),
            ("foreground_normal", "foreground"),
            ("foreground_inactive", "foreground_inactive"),
            ("foreground_active", "foreground_active"),
            ("foreground_link", "link"),
            ("foreground_visited", "link_visited"),
            ("foreground_negative", "negative"),
            ("foreground_neutral", "neutral"),
            ("foreground_positive", "positive"),
            ("decoration_focus", "decoration_focus"),
            ("decoration_hover", "decoration_hover"),
        ]
    );
    for path in [
        "kde.disabled.intensity",
        "kde.inactive.color_color",
        "kde.tint_factor",
        "material.variant",
        "material.contrast_level",
    ] {
        assert!(registry::find(registry::tokens(), path).is_some(), "{path}");
    }
    let bases = InputBases::default();
    assert_eq!(bases.kde_groups.get(Scheme::Light), None);
    assert!(bases.entries.is_empty());
    let accent = Accent::new(InputValue::new(SchemeMap::one(Scheme::Dark, Value::Absent)));
    assert!(accent.derived.is_empty() && accent.accent.per_contrast.is_empty());
}

#[test]
fn input_names_are_paths_of_keys() {
    for path in [
        "accent",
        "accent_ramp.light1",
        "kde_scheme.view.inactive.foreground_visited",
    ] {
        assert_eq!(
            InputName::new(path).map(|n| n.to_string()),
            Ok(path.to_owned())
        );
    }
    assert_eq!(
        InputName::new("accent_ramp.light1").map(|n| n.segments().to_vec()),
        Ok(vec![key!("accent_ramp"), key!("light1")])
    );
    for path in ["", "accent.", ".accent", "Accent", "accent..ramp"] {
        assert!(InputName::new(path).is_err(), "{path}");
    }
}

fn marked(fields: &'static [FieldInfo], path: &str) -> bool {
    registry::find(fields, path).is_some_and(|f| f.may_be_absent)
}

fn value_fields(fields: &'static [FieldInfo]) -> Vec<&'static str> {
    fields
        .iter()
        .filter(|f| written(f).starts_with("Value<"))
        .map(|f| f.path)
        .collect()
}

#[test]
fn text_rendering_fields_may_be_absent() {
    assert_eq!(TextRendering::FIELDS.len(), 7);
    for field in TextRendering::FIELDS {
        assert!(field.may_be_absent, "{}", field.path);
    }
}

#[test]
fn caret_secondary_read_only_and_typing_may_be_absent() {
    for path in ["secondary", "shown_in_read_only", "solid_while_typing"] {
        assert!(marked(Caret::FIELDS, path), "{path}");
    }
    for path in ["color", "width", "blink_on", "blink_off", "blink_timeout"] {
        assert!(!marked(Caret::FIELDS, path), "{path}");
    }
}

#[test]
fn behavior_values_may_be_absent() {
    let values = value_fields(Behavior::FIELDS);
    assert_eq!(values.len(), 15);
    for path in values {
        assert!(marked(Behavior::FIELDS, path), "{path}");
    }
    for path in [
        "key_repeat",
        "key_repeat.delay",
        "key_repeat.interval",
        "mnemonics.on_alt.delay",
    ] {
        assert!(!marked(Behavior::FIELDS, path), "{path}");
    }
}

#[test]
fn touch_behavior_values_may_be_absent() {
    assert_eq!(TouchBehavior::FIELDS.len(), 5);
    for field in TouchBehavior::FIELDS {
        assert!(field.may_be_absent, "{}", field.path);
    }
}

#[test]
fn pointer_values_may_be_absent() {
    for path in ["theme", "size", "color"] {
        assert!(marked(PointerTokens::FIELDS, path), "{path}");
    }
    assert!(!marked(PointerTokens::FIELDS, "drag_cursors"));
}

#[test]
fn icon_size_context_fields_but_size_may_be_absent() {
    let values = value_fields(IconStyle::FIELDS);
    for context in [
        "toolbar",
        "small",
        "large",
        "dialog",
        "panel",
        "keyed.symbol",
    ] {
        for path in &values {
            let path = format!("{context}.{path}");
            assert_eq!(
                marked(IconSizeTokens::FIELDS, &path),
                !path.ends_with(".size"),
                "{path}"
            );
        }
        for list in ["axes", "symbol_variants", "status_colors"] {
            assert!(
                !marked(IconSizeTokens::FIELDS, &format!("{context}.{list}")),
                "{list}"
            );
        }
        assert!(!marked(
            IconSizeTokens::FIELDS,
            &format!("{context}.fill.layers.blend")
        ));
    }
    assert!(!marked(IconStyle::FIELDS, "rendering_mode"));
}

#[test]
fn icon_theme_fallback_may_be_absent() {
    assert!(marked(IconTokens::FIELDS, "theme_fallback"));
}

#[test]
fn absent_tokens_by_the_catalogue_table() {
    for path in [
        "kde",
        "material",
        "icons.theme",
        "icons.sizes.toolbar",
        "icons.sizes.small",
        "icons.sizes.large",
        "icons.sizes.dialog",
        "icons.sizes.panel",
    ] {
        assert!(marked(registry::tokens(), path), "{path}");
    }
    assert!(!marked(registry::tokens(), "icons.set"));
}

#[test]
fn kde_tokens_are_stated_whole() {
    let mut paths = Vec::new();
    leaves("", KdeTokens::FIELDS, &mut paths);
    assert_eq!(paths.len(), 18);
    for path in paths {
        assert!(!marked(KdeTokens::FIELDS, &path), "{path}");
    }
    for field in MaterialScheme::FIELDS {
        assert!(!field.may_be_absent, "{}", field.path);
    }
}

#[test]
fn surface_roles_marks_by_the_catalogue() {
    for path in [
        "background",
        "foreground",
        "foreground_inactive",
        "link",
        "negative",
        "border",
    ] {
        assert!(!marked(SurfaceRoles::FIELDS, path), "{path}");
    }
    let absent: Vec<_> = SurfaceRoles::FIELDS
        .iter()
        .filter(|f| f.may_be_absent)
        .map(|f| f.path)
        .collect();
    assert_eq!(
        absent,
        [
            "background_alternate",
            "foreground_active",
            "link_visited",
            "neutral",
            "positive",
            "decoration_focus",
            "decoration_hover",
            "shade",
            "shade_darker",
            "background_active",
            "background_link",
            "background_visited",
            "background_negative",
            "background_neutral",
            "background_positive",
        ]
    );
}

#[test]
fn disabled_roles_may_all_be_absent() {
    assert_eq!(DisabledRoles::FIELDS.len(), 21);
    for field in DisabledRoles::FIELDS {
        assert!(field.may_be_absent, "{}", field.path);
    }
    assert!(!marked(SurfaceRoles::FIELDS, "disabled"));
}

#[test]
fn status_roles_may_be_absent_but_error_fill() {
    let mut paths = Vec::new();
    leaves("", StatusRoles::FIELDS, &mut paths);
    for path in paths {
        assert_eq!(
            marked(StatusRoles::FIELDS, &path),
            path != "error.fill",
            "{path}"
        );
    }
}

#[test]
fn global_roles_marks_by_the_catalogue() {
    let absent: Vec<_> = GlobalRoles::FIELDS
        .iter()
        .filter(|f| f.may_be_absent)
        .map(|f| f.path)
        .collect();
    assert_eq!(
        absent,
        [
            "shadow_reference",
            "selection_text_foreground",
            "selection_text_background_unfocused",
            "selection_text_foreground_unfocused",
            "find_highlight",
            "drop_highlight",
            "scrim",
        ]
    );
}

fn color(r: f32) -> Value<Color> {
    Value::Stated(Color {
        r,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    })
}

fn surface_roles(r: f32) -> SurfaceRoles {
    let disabled = DisabledRoles {
        background: color(r),
        background_alternate: Value::Absent,
        foreground: color(r),
        foreground_inactive: Value::Absent,
        foreground_active: Value::Absent,
        link: Value::Absent,
        link_visited: Value::Absent,
        negative: Value::Absent,
        neutral: Value::Absent,
        positive: Value::Absent,
        decoration_focus: Value::Absent,
        decoration_hover: Value::Absent,
        shade: Value::Absent,
        shade_darker: Value::Absent,
        border: Value::Absent,
        background_active: Value::Absent,
        background_link: Value::Absent,
        background_visited: Value::Absent,
        background_negative: Value::Absent,
        background_neutral: Value::Absent,
        background_positive: Value::Absent,
    };
    SurfaceRoles {
        background: color(r),
        background_alternate: Value::Absent,
        foreground: color(r),
        foreground_inactive: color(r),
        foreground_active: Value::Absent,
        link: color(r),
        link_visited: Value::Absent,
        negative: color(r),
        neutral: Value::Absent,
        positive: Value::Absent,
        decoration_focus: Value::Absent,
        decoration_hover: Value::Absent,
        shade: Value::Absent,
        shade_darker: Value::Absent,
        border: color(r),
        background_active: Value::Absent,
        background_link: Value::Absent,
        background_visited: Value::Absent,
        background_negative: Value::Absent,
        background_neutral: Value::Absent,
        background_positive: Value::Absent,
        disabled,
    }
}

fn surfaces(
    entries: Vec<(SurfaceSet, SurfaceRolesPatch)>,
) -> Option<SurfaceMap<SurfaceRolesPatch>> {
    let mut map = SurfaceMap::default();
    for (set, patch) in entries {
        map.insert(set, patch);
    }
    Some(map)
}

#[test]
fn surface_sets_patch_set_by_set_and_field_by_field() {
    let mut earlier = ColorRolesPatch {
        surfaces: surfaces(vec![(
            SurfaceSet::Window,
            SurfaceRolesPatch {
                background: Some(color(0.1)),
                foreground: Some(color(0.2)),
                ..SurfaceRolesPatch::default()
            },
        )]),
        ..ColorRolesPatch::default()
    };
    let later = ColorRolesPatch {
        surfaces: surfaces(vec![
            (
                SurfaceSet::Window,
                SurfaceRolesPatch {
                    background: Some(color(0.3)),
                    ..SurfaceRolesPatch::default()
                },
            ),
            (
                SurfaceSet::View,
                SurfaceRolesPatch {
                    link: Some(color(0.4)),
                    ..SurfaceRolesPatch::default()
                },
            ),
        ]),
        ..ColorRolesPatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    let merged = earlier.surfaces.unwrap_or_default();
    let window = merged.get(SurfaceSet::Window);
    assert_eq!(window.and_then(|w| w.background.clone()), Some(color(0.3)));
    assert_eq!(window.and_then(|w| w.foreground.clone()), Some(color(0.2)));
    assert_eq!(
        merged.get(SurfaceSet::View).and_then(|v| v.link.clone()),
        Some(color(0.4))
    );

    let mut value = SurfaceMap::default();
    value.insert(SurfaceSet::Window, surface_roles(0.5));
    let patch = surfaces(vec![(
        SurfaceSet::Window,
        SurfaceRolesPatch {
            neutral: Some(color(0.6)),
            ..SurfaceRolesPatch::default()
        },
    )])
    .unwrap_or_default();
    let mut colors = super::roles::ColorRoles {
        surfaces: value,
        global: global_roles(),
        keyed: Vec::new(),
    };
    let over = ColorRolesPatch {
        surfaces: Some(patch.clone()),
        ..ColorRolesPatch::default()
    };
    assert_eq!(colors.apply(over), Ok(()));
    let window = colors.surfaces.get(SurfaceSet::Window);
    assert_eq!(window.map(|w| w.neutral.clone()), Some(color(0.6)));
    assert_eq!(window.map(|w| w.background.clone()), Some(color(0.5)));

    let mut missing = patch;
    missing.insert(SurfaceSet::Card, SurfaceRolesPatch::default());
    let refused = colors.apply(ColorRolesPatch {
        surfaces: Some(missing),
        ..ColorRolesPatch::default()
    });
    let conflict: Result<(), MergeConflict> = Err(MergeConflict {
        path: "surfaces.card".to_owned(),
        below: super::blocks::Below::Missing,
    });
    assert_eq!(refused, conflict);
}

fn global_roles() -> GlobalRoles {
    let status = |r| super::roles::StatusColors {
        fill: color(r),
        foreground: Value::Absent,
        standalone: Value::Absent,
        background: Value::Absent,
        background_solid: Value::Absent,
    };
    GlobalRoles {
        accent: color(0.1),
        accent_foreground: color(0.2),
        accent_standalone: color(0.1),
        accent_ramp: Vec::new(),
        status: StatusRoles {
            success: status(0.3),
            warning: status(0.3),
            error: status(0.3),
            info: status(0.3),
            neutral: status(0.3),
            destructive: status(0.3),
        },
        shadow_reference: Value::Absent,
        selection_text_background: color(0.4),
        selection_text_foreground: Value::Absent,
        selection_text_background_unfocused: Value::Absent,
        selection_text_foreground_unfocused: Value::Absent,
        find_highlight: Value::Absent,
        drop_highlight: Value::Absent,
        scrim: color(0.0),
    }
}

#[test]
fn status_roles_patch_field_by_field() {
    let mut global = global_roles();
    let patch = super::roles::GlobalRolesPatch {
        status: Some(super::roles::StatusRolesPatch {
            error: Some(super::roles::StatusColorsPatch {
                standalone: Some(color(0.7)),
                ..Default::default()
            }),
            ..Default::default()
        }),
        accent_ramp: Some(vec![(key!("light1"), color(0.8))]),
        ..Default::default()
    };
    assert_eq!(global.apply(patch), Ok(()));
    assert_eq!(global.status.error.standalone, color(0.7));
    assert_eq!(global.status.error.fill, color(0.3));
    assert_eq!(global.accent_ramp, [(key!("light1"), color(0.8))]);
}

fn scalar(n: f32) -> Value<Scalar> {
    Value::Stated(Scalar::Number(n))
}

#[test]
fn params_merge_by_group_then_by_key() {
    let mut earlier = TokensPatch {
        params: Some(vec![
            (
                key!("opacity"),
                vec![(key!("disabled"), scalar(0.5)), (key!("dim"), scalar(0.6))],
            ),
            (key!("color"), vec![(key!("contrast"), scalar(4.0))]),
        ]),
        ..TokensPatch::default()
    };
    let later = TokensPatch {
        params: Some(vec![
            (key!("opacity"), vec![(key!("disabled"), scalar(0.3))]),
            (key!("layout"), vec![(key!("grid_unit"), scalar(18.0))]),
        ]),
        ..TokensPatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    assert_eq!(
        earlier.params,
        Some(vec![
            (
                key!("opacity"),
                vec![(key!("disabled"), scalar(0.3)), (key!("dim"), scalar(0.6))]
            ),
            (key!("color"), vec![(key!("contrast"), scalar(4.0))]),
            (key!("layout"), vec![(key!("grid_unit"), scalar(18.0))]),
        ])
    );
}

fn ms(n: f32) -> Value<Ms> {
    Value::Stated(Ms(n))
}

#[test]
fn behavior_enum_and_optional_fields_are_set_whole() {
    let mut earlier = BehaviorPatch {
        key_repeat: Some(Some(KeyRepeat::new(ms(500.0), ms(30.0)))),
        mnemonics: Some(Mnemonics::OnAlt { delay: ms(100.0) }),
        focus_visible_when: Some(Value::Stated(FocusVisibleWhen::AnyFocus)),
        ..BehaviorPatch::default()
    };
    let later = BehaviorPatch {
        key_repeat: Some(None),
        mnemonics: Some(Mnemonics::Always),
        long_press: Some(Value::Absent),
        ..BehaviorPatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    assert_eq!(earlier.key_repeat, Some(None));
    assert_eq!(earlier.mnemonics, Some(Mnemonics::Always));
    assert_eq!(
        earlier.focus_visible_when,
        Some(Value::Stated(FocusVisibleWhen::AnyFocus))
    );
    assert_eq!(earlier.long_press, Some(Value::Absent));
}

#[test]
fn kde_tokens_patch_by_set_or_fields() {
    let effects = KdeEffects {
        intensity: super::tokens::KdeIntensityEffect::Shade,
        intensity_amount: Value::Stated(0.1),
        color: super::tokens::KdeColorEffect::Desaturate,
        color_amount: Value::Stated(0.2),
        color_color: color(0.3),
        contrast: super::tokens::KdeContrastEffect::Fade,
        contrast_amount: Value::Stated(0.4),
    };
    let kde = KdeTokens {
        disabled: effects.clone(),
        inactive: effects,
        inactive_enabled: Value::Stated(false),
        inactive_change_selection_color: Value::Stated(true),
        tint_factor: None,
        titlebar_is_accent_colored: Value::Stated(false),
    };
    let mut earlier = TokensPatch {
        kde: Some(BlockPatch::Set(Value::Stated(kde))),
        theme_change: Some(ThemeChange::Instant),
        ..TokensPatch::default()
    };
    let later = TokensPatch {
        kde: Some(BlockPatch::Fields(super::tokens::KdeTokensPatch {
            tint_factor: Some(Some(Value::Stated(0.5))),
            inactive: Some(super::tokens::KdeEffectsPatch {
                contrast_amount: Some(Value::Stated(0.6)),
                ..Default::default()
            }),
            ..Default::default()
        })),
        theme_change: Some(ThemeChange::Suspend {
            duration: ms(200.0),
        }),
        ..TokensPatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    let Some(BlockPatch::Set(Value::Stated(kde))) = earlier.kde else {
        panic!("a Fields patch over set KDE tokens applies to them");
    };
    assert_eq!(kde.tint_factor, Some(Value::Stated(0.5)));
    assert_eq!(kde.inactive.contrast_amount, Value::Stated(0.6));
    assert_eq!(kde.disabled.contrast_amount, Value::Stated(0.4));
    assert_eq!(
        earlier.theme_change,
        Some(ThemeChange::Suspend {
            duration: ms(200.0)
        })
    );

    let mut absent = TokensPatch {
        material: Some(BlockPatch::Set(Value::Absent)),
        ..TokensPatch::default()
    };
    let fields = TokensPatch {
        material: Some(BlockPatch::Fields(
            super::tokens::MaterialSchemePatch::default(),
        )),
        ..TokensPatch::default()
    };
    assert!(absent.merge(fields).is_err());
}
