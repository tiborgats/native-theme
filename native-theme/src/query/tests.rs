//! S8's query on fixture stacks. Every number, colour, name and metric here
//! is synthetic, none a platform's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use super::{DataNeeds, Element, FontMetrics, RootInputs, Scope, Sibling, Styled, Unavailable};
use crate::blocks::motion::{ResolvedKeyframe, ResolvedTransitionKind};
use crate::blocks::outline::ResolvedRadius;
use crate::blocks::part_style::{ResolvedPartStyle, ResolvedPartStylePatch, RoleRef};
use crate::blocks::text::ResolvedFont;
use crate::color::Color;
use crate::declare::DataName;
use crate::expr::ElementDatum;
use crate::key::Key;
use crate::layer::Layer;
use crate::resolve::{ResolvedTheme, Stack, WindowConditions, WindowState};
use crate::roles::{SurfaceRole, SurfaceSet};
use crate::selector::{
    ControlSize, Density, Direction, InputKind, PartRef, SizeClass, SizeClasses, StateFlag,
    StateFlags, VariantRef, WidgetOrPart,
};
use crate::toml::layer::read_preset;
use crate::units::{Deg, Fraction, Length, Px, Sides, Size};
use crate::widgets::{ButtonPart, ButtonValues, TypedPart, Variant};

/// Synthetic measurement: each metric the font's size.
struct Metrics;

impl FontMetrics for Metrics {
    fn char_width(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn max_char_width(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn x_height(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn height(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn line_spacing(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn underline_offset(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn underline_thickness(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn strikeout_offset(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
    fn strikeout_thickness(&self, font: &ResolvedFont) -> Px {
        font.size.unwrap_or(Px(0.0))
    }
}

const BODY: PartRef = PartRef::Typed(TypedPart::Button(ButtonPart::Body));
const LABEL: PartRef = PartRef::Typed(TypedPart::Button(ButtonPart::Label));
const ICON: PartRef = PartRef::Typed(TypedPart::Button(ButtonPart::Icon));
const SUBTITLE: PartRef = PartRef::Typed(TypedPart::Button(ButtonPart::Subtitle));

/// The fixture's declarations: synthetic variants, sizes, flags, groups and
/// data.
const DECLARE: &str = r#"
[declare]
surface_sets = ["window", "button"]
[declare.widget.button]
variants = ["primary", "tool", "tinted", "@solo"]
sizes = ["small", "regular"]
densities = ["compact"]
flags = ["@armed"]
exclusive = [["disabled", "pressed"]]
exclusive_variants = [["primary", "tool"]]
recency = { lift = ["pressed", "focus_visible", "hover"] }
sizes_by_variant = [{ variant = "tinted", sizes = ["large"], default_size = "large" }]
default_size = "regular"
data = { label = ["char_width"] }
"#;

/// The fixture's preset layer: every part the button draws stated.
const PRESET: &str = r##"
[colors.light.surfaces.window]
foreground = "#102030"
background = "#f0f0f0"
[colors.light.surfaces.button]
foreground = "#405060"
background = "#e0e0e0"
[tokens.size_classes.compact]
horizontal_px = 0
[tokens.size_classes.medium]
horizontal_px = 600
[tokens.size_classes.expanded]
horizontal_px = 840
[tokens.size_classes.regular]
vertical_px = 400
[widgets.button.body]
fill = "= surface.background"
stroke = "absent"
corners_px = 4
corners.curve = "circular"
shadow = "absent"
padding = { top_px = 1, right_px = 2, bottom_px = 3, left_px = 4 }
margin = { top_px = 0, right_px = 5, bottom_px = 0, left_px = 7 }
min_width_px = 40
min_height_px = "auto"
max_width_px = "auto"
opacity = 1
transform = { scale_x = 1, scale_y = 1, rotate_deg = 0, translate_x_px = 5, translate_y_px = 3, origin = [0.25, 0.5] }
focus = "absent"
cursor = "pointer"
touch_target_px = "absent"
interaction = "absent"
[widgets.button.label]
padding = { top_px = 0, right_px = 0, bottom_px = 0, left_px = 0 }
text = "absent"
[widgets.button.subtitle]
absent = true
[widgets.button.icon]
absent = true
[widgets.button.menu_indicator]
absent = true
[widgets.button.values]
icon_text_gap_px = 4
icon_placement = "leading"
label_position = "absent"
subtitle_gap_px = "absent"
shows_icon = true
focus_visible_when = "any_focus"
repeat_delay_ms = "absent"
repeat_interval_ms = "absent"
hit_extension = { top_px = 1, right_px = 2, bottom_px = 3, left_px = 4 }
"##;

fn layer(text: &str) -> Layer {
    Layer::from_toml(text).unwrap_or_else(|e| panic!("{e}"))
}

/// The theme the fixture's preset and `layers` resolve to, under `declare`.
fn theme_with(declare: &str, layers: &[&str]) -> ResolvedTheme {
    theme_of(declare, layers.iter().map(|text| layer(text)).collect())
}

/// The theme the fixture's preset and `layers`, built in Rust, resolve to,
/// under `declare`.
fn theme_of(declare: &str, layers: Vec<Layer>) -> ResolvedTheme {
    let file = read_preset(&format!(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n{declare}"
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    let mut stacked = vec![layer(PRESET)];
    stacked.extend(layers);
    Stack::unvalidated(file.declarations, stacked, None)
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme
}

fn fixture(layers: &[&str]) -> ResolvedTheme {
    theme_with(DECLARE, layers)
}

fn size(width: f32, height: f32) -> Size {
    Size {
        width: Px(width),
        height: Px(height),
    }
}

fn root(theme: &ResolvedTheme, direction: Direction) -> Scope {
    theme.root_scope(&RootInputs::new(
        size(800.0, 600.0),
        size(1920.0, 1080.0),
        direction,
    ))
}

/// The style `element` resolves to, which must be drawn.
fn drawn(theme: &ResolvedTheme, element: &Element<'_>) -> (ResolvedPartStyle, Scope) {
    match theme.style(element) {
        Styled::Style(style) => style,
        other => panic!("not drawn: {other:?}"),
    }
}

fn min_width(theme: &ResolvedTheme, element: &Element<'_>) -> Option<Length> {
    drawn(theme, element).0.min_width
}

fn key(name: &str) -> Key {
    Key::new(name).unwrap()
}

/// `rtl_mirrors_sides_not_corners` (S2.4): under right-to-left, the element's
/// own direction or its scope's, every `Sides` is mirrored and the
/// transform's `translate_x` negated; the corners, the translation's `y` and
/// the origin stay; a widget's values mirror their `Sides` and alignments
/// too.
#[test]
fn rtl_mirrors_sides_not_corners() {
    let theme = fixture(&["[widgets.button.body]\ncorners.top_left_px = 9"]);
    let ltr = root(&theme, Direction::Ltr);
    let rtl = root(&theme, Direction::Rtl);
    let stated = drawn(&theme, &Element::new(BODY, &ltr, &Metrics)).0;
    assert_eq!(
        stated.padding,
        Sides {
            top: Some(Px(1.0)),
            right: Some(Px(2.0)),
            bottom: Some(Px(3.0)),
            left: Some(Px(4.0)),
        }
    );
    let by_scope = drawn(&theme, &Element::new(BODY, &rtl, &Metrics)).0;
    let by_element = drawn(
        &theme,
        &Element::new(BODY, &ltr, &Metrics).with_direction(Direction::Rtl),
    )
    .0;
    for mirrored in [by_scope, by_element] {
        assert_eq!(
            mirrored.padding,
            Sides {
                top: Some(Px(1.0)),
                right: Some(Px(4.0)),
                bottom: Some(Px(3.0)),
                left: Some(Px(2.0)),
            }
        );
        assert_eq!(mirrored.margin.left, Some(Px(5.0)));
        assert_eq!(mirrored.margin.right, Some(Px(7.0)));
        let transform = mirrored.transform.unwrap();
        assert_eq!(transform.translate_x, Some(Px(-5.0)));
        assert_eq!(transform.translate_y, Some(Px(3.0)));
        assert_eq!(transform.origin, (0.25, 0.5));
        assert_eq!(mirrored.corners, stated.corners);
        let corners = mirrored.corners.unwrap();
        assert_eq!(corners.top_left, ResolvedRadius::Px(Some(Px(9.0))));
        assert_eq!(corners.top_right, ResolvedRadius::Px(Some(Px(4.0))));
    }
    let Styled::Style(values) = theme.values::<ButtonValues>(&Element::new(BODY, &rtl, &Metrics))
    else {
        panic!("the values resolve");
    };
    assert_eq!(values.hit_extension.left, Some(Px(2.0)));
    assert_eq!(values.hit_extension.right, Some(Px(4.0)));
    // Task 11e's review: an alignment's start and end swap too, so a
    // leading image lies on the physical right.
    assert_eq!(
        values.icon_placement,
        Some(crate::widgets::IconPlacement::Trailing)
    );
}

/// `later_names_answer_later` (S8, S10's *Later names*; ADD-A147): an
/// element holding a variant its widget lists under `later` is answered
/// `NotAvailable(Later)`, and, while the sizes unit is not enabled, one
/// asking a size it takes other than its default in force, or a declared
/// density; its default size, and a size it does not take, as before.
#[test]
#[expect(
    clippy::assertions_on_constants,
    reason = "the sizes unit's flag is a generated constant the answers depend on"
)]
fn later_names_answer_later() {
    assert!(!crate::widgets::SIZES_UNIT, "the sizes unit is not enabled");
    let declare = format!("{DECLARE}later = {{ variants = [\"@solo\"] }}\n");
    let theme = theme_with(&declare, &[]);
    let scope = root(&theme, Direction::Ltr);
    let element = || Element::new(BODY, &scope, &Metrics);
    let solo = [VariantRef::Keyed(key("solo"))];
    assert_eq!(
        theme.style(&element().with_variants(&solo)),
        Styled::NotAvailable(Unavailable::Later)
    );
    assert_eq!(
        theme.style(&element().with_size(ControlSize::Small)),
        Styled::NotAvailable(Unavailable::Later)
    );
    assert!(matches!(
        theme.style(&element().with_size(ControlSize::Regular)),
        Styled::Style(_)
    ));
    assert_eq!(
        theme.style(&element().with_size(ControlSize::Large)),
        Styled::NotAvailable(Unavailable::Size)
    );
    let tinted = [VariantRef::Typed(Variant::Tinted)];
    assert!(matches!(
        theme.style(
            &element()
                .with_variants(&tinted)
                .with_size(ControlSize::Large)
        ),
        Styled::Style(_)
    ));
    assert_eq!(
        theme.style(&element().with_density(Density::Compact)),
        Styled::NotAvailable(Unavailable::Later)
    );
    // A content variant the connector sets is refused too, where `later`
    // lists it, and kept where only `variants` does.
    let content = [VariantRef::Typed(Variant::WithIcon)];
    let declared = DECLARE.replace(
        "variants = [\"primary\", \"tool\", \"tinted\", \"@solo\"]",
        "variants = [\"primary\", \"tool\", \"tinted\", \"@solo\", \"with_icon\"]",
    );
    let kept = theme_with(&declared, &[]);
    let scope = root(&kept, Direction::Ltr);
    assert!(matches!(
        kept.style(&Element::new(BODY, &scope, &Metrics).with_variants(&content)),
        Styled::Style(_)
    ));
    let listed = theme_with(
        &format!("{declared}later = {{ variants = [\"with_icon\"] }}\n"),
        &[],
    );
    let scope = root(&listed, Direction::Ltr);
    assert_eq!(
        listed.style(&Element::new(BODY, &scope, &Metrics).with_variants(&content)),
        Styled::NotAvailable(Unavailable::Later)
    );
    // An unavailable widget's answer comes first.
    let unavailable = theme_with(&format!("{declare}available = false\n"), &[]);
    let scope = root(&unavailable, Direction::Ltr);
    assert_eq!(
        unavailable.style(&Element::new(BODY, &scope, &Metrics).with_variants(&solo)),
        Styled::NotAvailable(Unavailable::Widget)
    );
}

/// `undeclared_variant_not_available` (S8): a variant the application asks
/// for that the widget does not declare is not available; a content
/// variant the connector sets, typed or keyed, is ignored where undeclared,
/// and kept where declared.
#[test]
fn undeclared_variant_not_available() {
    let theme = fixture(&[]);
    let scope = root(&theme, Direction::Ltr);
    let asked = [VariantRef::Keyed(key("nope"))];
    assert_eq!(
        theme.style(&Element::new(BODY, &scope, &Metrics).with_variants(&asked)),
        Styled::NotAvailable(Unavailable::Variant)
    );
    // `vertical`-like: a name another widget has is asked for here.
    let other = [VariantRef::Typed(Variant::Toggle)];
    assert_eq!(
        theme.style(&Element::new(BODY, &scope, &Metrics).with_variants(&other)),
        Styled::NotAvailable(Unavailable::Variant)
    );
    let content = [
        VariantRef::Typed(Variant::WithIcon),
        VariantRef::Typed(Variant::Primary),
    ];
    let (_, scope_out) = drawn(
        &theme,
        &Element::new(BODY, &scope, &Metrics).with_variants(&content),
    );
    let entry = scope_out.entries.last().unwrap();
    assert_eq!(entry.variants, [VariantRef::Typed(Variant::Primary)]);
}

/// `absent_part_not_drawn` (S6.3, S8): a part whose last kept patch makes it
/// `Absent` is not drawn, a rule's included; one a rule draws as another
/// widget's part is `DrawnAs`.
#[test]
fn absent_part_not_drawn() {
    let rules = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.label\"\n\
                 set = { absent = true }\n\
                 [[rules]]\nwhen = { states = [\"focus_visible\"] }\ntarget = \"button.icon\"\n\
                 set = { drawn_as = { widget = \"button\", part = \"body\", variants = [\"tool\"] } }";
    let theme = fixture(&[rules]);
    let scope = root(&theme, Direction::Ltr);
    assert_eq!(
        theme.style(&Element::new(SUBTITLE, &scope, &Metrics)),
        Styled::NotDrawn
    );
    let at_rest = Element::new(LABEL, &scope, &Metrics);
    assert!(matches!(theme.style(&at_rest), Styled::Style(_)));
    let hovered = Element::new(LABEL, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
    assert_eq!(theme.style(&hovered), Styled::NotDrawn);
    let focused = Element::new(ICON, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::FocusVisible), &[]);
    assert_eq!(
        theme.style(&focused),
        Styled::DrawnAs {
            target: WidgetOrPart::Part(BODY),
            variants: vec![VariantRef::Typed(Variant::Tool)],
        }
    );
    assert_eq!(
        theme.style(&Element::new(ICON, &scope, &Metrics)),
        Styled::NotDrawn
    );
}

/// `field_rule_does_not_redraw_absent_part` (S6.3, ADD-A126): a later rule
/// patching a field of a part a condition made absent, or drawn as another
/// widget, states no presence and leaves the part as it was (an
/// application's colour `@ {hover}`).
#[test]
fn field_rule_does_not_redraw_absent_part() {
    let rules = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.label\"\n\
                 set = { absent = true }\n\
                 [[rules]]\nwhen = { states = [\"focus_visible\"] }\ntarget = \"button.icon\"\n\
                 set = { drawn_as = { widget = \"button\", part = \"body\", variants = [] } }";
    let fields = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.label\"\n\
                  set = { padding = { top_px = 2 } }\n\
                  [[rules]]\nwhen = { states = [\"focus_visible\"] }\ntarget = \"button.icon\"\n\
                  set = { icon.size_px = 16 }";
    let theme = fixture(&[rules, fields]);
    let scope = root(&theme, Direction::Ltr);
    let hovered = Element::new(LABEL, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
    assert_eq!(theme.style(&hovered), Styled::NotDrawn);
    let focused = Element::new(ICON, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::FocusVisible), &[]);
    assert_eq!(
        theme.style(&focused),
        Styled::DrawnAs {
            target: WidgetOrPart::Part(BODY),
            variants: Vec::new(),
        }
    );
}

/// `absent_false_draws_part_again` (S6.3, S11, ADD-A126, ADD-A131): a rule
/// stating `absent = false` draws a part a broader rule made absent, with
/// the fields beside it over those beneath.
#[test]
fn absent_false_draws_part_again() {
    let rules = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.label\"\n\
                 set = { absent = true }\n\
                 [[rules]]\nwhen = { states = [\"pressed\"] }\ntarget = \"button.label\"\n\
                 set = { absent = false, padding = { top_px = 3 } }";
    let theme = fixture(&[rules]);
    let scope = root(&theme, Direction::Ltr);
    let hovered = Element::new(LABEL, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
    assert_eq!(theme.style(&hovered), Styled::NotDrawn);
    let pressed = Element::new(LABEL, &scope, &Metrics).with_states(
        StateFlags::new()
            .with(StateFlag::Hover)
            .with(StateFlag::Pressed),
        &[],
    );
    let (label, _) = drawn(&theme, &pressed);
    assert_eq!(label.padding.top, Some(Px(3.0)));
    assert_eq!(label.padding.left, Some(Px(0.0)));
}

/// `fields_before_absent_kept_beneath` (S6.1, S6.3, ADD-A131): the fields a
/// patch states before an `Absent`, and beside it, stay beneath it, so a
/// later `absent = false` draws the part with them.
#[test]
fn fields_before_absent_kept_beneath() {
    let rules = "[[rules]]\ntarget = \"button.label\"\nset = { padding = { top_px = 6 } }\n\
                 [[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.label\"\n\
                 set = { absent = true, padding = { left_px = 8 } }\n\
                 [[rules]]\nwhen = { states = [\"pressed\"] }\ntarget = \"button.label\"\n\
                 set = { absent = false }";
    let theme = fixture(&[rules]);
    let scope = root(&theme, Direction::Ltr);
    let hovered = Element::new(LABEL, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
    assert_eq!(theme.style(&hovered), Styled::NotDrawn);
    let pressed = Element::new(LABEL, &scope, &Metrics).with_states(
        StateFlags::new()
            .with(StateFlag::Hover)
            .with(StateFlag::Pressed),
        &[],
    );
    let (label, _) = drawn(&theme, &pressed);
    assert_eq!(label.padding.top, Some(Px(6.0)));
    assert_eq!(label.padding.left, Some(Px(8.0)));
}

/// The theme of the fixture's preset and `layers`, each added as an
/// application layer, so validated (S12).
fn validated(layers: &[&str]) -> ResolvedTheme {
    let file = read_preset(&format!(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n{DECLARE}"
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    let mut stack = Stack::unvalidated(file.declarations, vec![layer(PRESET)], None);
    for text in layers {
        stack
            .add_layer(layer(text))
            .unwrap_or_else(|e| panic!("{e}"));
    }
    stack
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme
}

/// `fields_patch_over_absent_leaves_absent` (S2.2, S6.3, S12, ADD-A126): a
/// rule's `Fields` patch over the focus indicator the base states `absent`
/// validates, its expression reading an absent operand not evaluated
/// there, and sets nothing: the indicator resolves `None`.
#[test]
fn fields_patch_over_absent_leaves_absent() {
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                set = { focus.kind = \"ring\", focus.width_px = \"= add(part.touch_target, 1px)\" }";
    let theme = validated(&[rule]);
    let scope = root(&theme, Direction::Ltr);
    let hovered = Element::new(BODY, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
    let (body, _) = drawn(&theme, &hovered);
    assert_eq!(body.focus, None);

    // The menu's check glyph outside its checked states (S6.3): a glyph's
    // leaf patched over the glyph a layer states absent.
    let icon = "[widgets.button.icon]\nabsent = false\nicon = { replace = { size_px = 16, fill = \
                \"#000000\", rendering_mode = \"from_icon\", weight = 400, symbolic_colors = {}, axes = \
                {}, symbol_variants = [], scale = \"absent\", state_effect = \"absent\" } }\n\
                glyph = \"absent\"";
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.icon\"\n\
                set = { glyph.size_px = 4 }";
    let theme = validated(&[icon, rule]);
    let scope = root(&theme, Direction::Ltr);
    let hovered = Element::new(ICON, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
    let (icon, _) = drawn(&theme, &hovered);
    assert_eq!(icon.glyph, None);
}

/// `append_over_absent_paint_draws_nothing` (S6.1, S6.3, ADD-A126): a
/// rule's `Append` over a fill a layer states `absent` validates and
/// appends to nothing: the fill resolves `None`.
#[test]
fn append_over_absent_paint_draws_nothing() {
    let absent = "[widgets.button.body]\nfill = \"absent\"";
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                set = { fill = { append = [\"#ff0000\"] } }";
    let theme = validated(&[absent, rule]);
    let scope = root(&theme, Direction::Ltr);
    let hovered = Element::new(BODY, &scope, &Metrics)
        .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
    let (body, _) = drawn(&theme, &hovered);
    assert_eq!(body.fill, None);
}

/// `scope_bindings_once` (S3.3, S8): a part's binding is evaluated once, in
/// the scope outside it, and its children read the bound colour through
/// the returned scope as it is: a binding darkening `surface.background`
/// darkens it once, for the part and for every part inside it.
#[test]
fn scope_bindings_once() {
    let bound = "[widgets.button.body]\nbindings = [{ role = { role = { surface = \"background\" }, \
                 value = \"= mix(surface.background, #000000, 0.5)\" } }]";
    let theme = fixture(&[bound]);
    let scope = root(&theme, Direction::Ltr);
    let background = RoleRef::Surface(SurfaceRole::Background);
    let window = 240.0 / 255.0;
    let once = Color {
        r: window / 2.0,
        g: window / 2.0,
        b: window / 2.0,
        a: 1.0,
    };
    assert_eq!(
        theme.role(&scope, background.clone()).map(|c| c.r),
        Some(window)
    );
    let (body, inside) = drawn(&theme, &Element::new(BODY, &scope, &Metrics));
    let fill = body.fill.unwrap();
    assert_eq!(fill.layers.len(), 1);
    assert_eq!(
        format!("{:?}", fill.layers.first().unwrap().kind),
        format!("Color(Some({once:?}))")
    );
    assert_eq!(theme.role(&inside, background.clone()), Some(once));
    let (_, label_scope) = drawn(&theme, &Element::new(LABEL, &inside, &Metrics));
    assert_eq!(theme.role(&label_scope, background), Some(once));
    // A role the binding does not bind reads the colour roles.
    assert_eq!(
        theme.role(
            &inside,
            RoleRef::Set(SurfaceSet::Button, SurfaceRole::Background)
        ),
        Some(Color {
            r: 224.0 / 255.0,
            g: 224.0 / 255.0,
            b: 224.0 / 255.0,
            a: 1.0
        })
    );
}

/// `NotAvailable(Widget)` (S8, S10): a widget the preset declares not
/// available, a component no preset declares, another widget's element
/// for `values`.
#[test]
fn not_available_widget() {
    let declare = "[declare.widget.button]\navailable = false";
    let theme = theme_with(declare, &[]);
    let scope = root(&theme, Direction::Ltr);
    assert_eq!(
        theme.style(&Element::new(BODY, &scope, &Metrics)),
        Styled::NotAvailable(Unavailable::Widget)
    );
    let theme = fixture(&[]);
    let component = PartRef::Component {
        key: key("rule_editor"),
        part: key("row"),
    };
    assert_eq!(
        theme.style(&Element::new(component.clone(), &scope, &Metrics)),
        Styled::NotAvailable(Unavailable::Widget)
    );
    assert_eq!(
        theme.values::<ButtonValues>(&Element::new(component, &scope, &Metrics)),
        Styled::NotAvailable(Unavailable::Widget)
    );
}

/// `NotAvailable(Size)` (S8, S10): a size the element does not take; a
/// variant's sizes replacing the widget's for an element holding it.
#[test]
fn not_available_size() {
    let theme = fixture(&[]);
    let scope = root(&theme, Direction::Ltr);
    let large = Element::new(BODY, &scope, &Metrics).with_size(ControlSize::Large);
    assert_eq!(theme.style(&large), Styled::NotAvailable(Unavailable::Size));
    let destructive = [VariantRef::Typed(Variant::Tinted)];
    let small = Element::new(BODY, &scope, &Metrics)
        .with_variants(&destructive)
        .with_size(ControlSize::Small);
    assert_eq!(theme.style(&small), Styled::NotAvailable(Unavailable::Size));
    let large = large.with_variants(&destructive);
    assert!(matches!(theme.style(&large), Styled::Style(_)));
}

/// S10's sizes and defaults (ADD-A131): an element takes its variant
/// entry's sizes, else the widget's, less those its context entry leaves
/// out; its default is the context entry's, else the variant entry's,
/// else the widget's; an element taking no size resolves with none, and a
/// size it is given is not available.
#[test]
fn sizes_by_variant_then_context() {
    let declare = "[declare.widget.button]\nvariants = [\"primary\", \"tool\"]\n\
                   sizes = [\"small\", \"regular\"]\ndefault_size = \"regular\"\n\
                   sizes_by_variant = [{ variant = \"primary\", sizes = [\"regular\", \"large\"], \
                   default_size = \"large\" }, { variant = \"tool\", sizes = [] }]\n\
                   sizes_by_context = [{ context = { widget = \"button\", part = \"body\" }, \
                   sizes = [\"regular\", \"large\"], default_size = \"regular\" }]";
    let theme = theme_with(declare, &[]);
    let root = root(&theme, Direction::Ltr);
    let primary = [VariantRef::Typed(Variant::Primary)];
    let flat = [VariantRef::Typed(Variant::Tool)];
    let size_of = |element: &Element<'_>| match theme.style(element) {
        Styled::Style((_, scope)) => Ok(scope.entries.last().and_then(|entry| entry.size)),
        Styled::NotAvailable(why) => Err(why),
        other => panic!("{other:?}"),
    };
    let body = |scope| Element::new(BODY, scope, &Metrics);
    assert_eq!(size_of(&body(&root)), Ok(Some(ControlSize::Regular)));
    assert_eq!(
        size_of(&body(&root).with_variants(&primary)),
        Ok(Some(ControlSize::Large))
    );
    assert_eq!(
        size_of(
            &body(&root)
                .with_variants(&primary)
                .with_size(ControlSize::Small)
        ),
        Err(Unavailable::Size)
    );
    assert_eq!(size_of(&body(&root).with_variants(&flat)), Ok(None));
    assert_eq!(
        size_of(
            &body(&root)
                .with_variants(&flat)
                .with_size(ControlSize::Regular)
        ),
        Err(Unavailable::Size)
    );
    // Inside another button's body: the context entry keeps `regular` of
    // the widget's sizes, and its default comes before the variant's.
    let (_, outer) = drawn(&theme, &body(&root));
    assert_eq!(size_of(&body(&outer)), Ok(Some(ControlSize::Regular)));
    assert_eq!(
        size_of(&body(&outer).with_size(ControlSize::Small)),
        Err(Unavailable::Size)
    );
    assert_eq!(
        size_of(&body(&outer).with_variants(&primary)),
        Ok(Some(ControlSize::Regular))
    );
    // A size the element takes that is not its default in force: `Later`
    // while the sizes unit is not enabled (S8, S10; ADD-A147).
    assert_eq!(
        size_of(
            &body(&outer)
                .with_variants(&primary)
                .with_size(ControlSize::Large)
        ),
        Err(Unavailable::Later)
    );
}

/// A pending default size (S10's *A pending default*): the lookup stops at
/// it, so an element that states no size and whose default is pending is
/// not available, as the synthetic theme answers it (S8, S17); one stating
/// a size it takes is no default in force, so `Later` while the sizes
/// unit is not enabled (S8, S10; ADD-A147); and another variant's default
/// stands.
#[test]
fn pending_default_size_not_available() {
    let declare = "[declare.widget.button]\nvariants = [\"primary\"]\n\
                   sizes = [\"small\", \"regular\"]\ndefault_size = \"regular\"\n\
                   sizes_by_variant = [{ variant = \"primary\", sizes = [\"regular\", \"large\"] }]\n\
                   [pending]\n\
                   \"declare.widget.button.sizes_by_variant.primary.default_size\" = \"U1\"";
    let theme = theme_with(declare, &[]);
    let scope = root(&theme, Direction::Ltr);
    let primary = [VariantRef::Typed(Variant::Primary)];
    let body = Element::new(BODY, &scope, &Metrics);
    assert_eq!(
        theme.style(&body.with_variants(&primary)),
        Styled::NotAvailable(Unavailable::Size)
    );
    let large = Element::new(BODY, &scope, &Metrics)
        .with_variants(&primary)
        .with_size(ControlSize::Large);
    assert_eq!(
        theme.style(&large),
        Styled::NotAvailable(Unavailable::Later)
    );
    assert!(matches!(
        theme.style(&Element::new(BODY, &scope, &Metrics)),
        Styled::Style(_)
    ));
}

/// `NotAvailable(Variant)` (S8, S10): none of the forms the widget requires,
/// or two variants of one exclusive group.
#[test]
fn not_available_variant() {
    let required = format!("{DECLARE}requires_variants = [\"primary\", \"@solo\"]");
    let theme = theme_with(&required, &[]);
    let scope = root(&theme, Direction::Ltr);
    assert_eq!(
        theme.style(&Element::new(BODY, &scope, &Metrics)),
        Styled::NotAvailable(Unavailable::Variant)
    );
    let solo = [VariantRef::Keyed(key("solo"))];
    assert!(matches!(
        theme.style(&Element::new(BODY, &scope, &Metrics).with_variants(&solo)),
        Styled::Style(_)
    ));
    let theme = fixture(&[]);
    let both = [
        VariantRef::Typed(Variant::Primary),
        VariantRef::Typed(Variant::Tool),
    ];
    assert_eq!(
        theme.style(&Element::new(BODY, &scope, &Metrics).with_variants(&both)),
        Styled::NotAvailable(Unavailable::Variant)
    );
}

/// The context (S8): an enclosing button matches `context button`, the
/// button the part belongs to does not; the nearest of the widgets named.
#[test]
fn context_skips_the_elements_own_parts() {
    let rules = "[[rules]]\nwhen = { context = [{ widget = \"button\" }] }\ntarget = \"button.body\"\n\
                 set = { min_width_px = 99 }\n\
                 [[rules]]\nwhen = { context = [{ scope = { nearest = [\"button\"] }, widget = \"button\", \
                 variants = [\"tool\"] }] }\ntarget = \"button.label\"\n\
                 set = { padding.top_px = 9 }";
    let theme = fixture(&[rules]);
    let scope = root(&theme, Direction::Ltr);
    let (_, outer) = drawn(&theme, &Element::new(BODY, &scope, &Metrics));
    assert_eq!(
        min_width(&theme, &Element::new(BODY, &scope, &Metrics)),
        Some(Length::Px(40.0))
    );
    assert_eq!(
        min_width(&theme, &Element::new(BODY, &outer, &Metrics)),
        Some(Length::Px(99.0))
    );
    // The label's own button is flat; it is skipped, and the enclosing one
    // is not.
    let flat = [VariantRef::Typed(Variant::Tool)];
    let (_, own) = drawn(
        &theme,
        &Element::new(BODY, &scope, &Metrics).with_variants(&flat),
    );
    let label = drawn(&theme, &Element::new(LABEL, &own, &Metrics)).0;
    assert_eq!(label.padding.top, Some(Px(0.0)));
    let (_, flat_outer) = drawn(
        &theme,
        &Element::new(BODY, &scope, &Metrics).with_variants(&flat),
    );
    let (_, inner) = drawn(&theme, &Element::new(BODY, &flat_outer, &Metrics));
    let label = drawn(&theme, &Element::new(LABEL, &inner, &Metrics)).0;
    assert_eq!(label.padding.top, Some(Px(9.0)));
}

/// The position (S8): counted among the siblings the rule names, an index
/// outside the siblings as having none.
#[test]
fn position_counts_among_the_siblings() {
    let rules = "[[rules]]\nwhen = { position = { first = true } }\ntarget = \"button.body\"\n\
                 set = { min_width_px = 50 }\n\
                 [[rules]]\nwhen = { position = { parity = \"even\" } }\ntarget = \"button.body\"\n\
                 set = { min_width_px = 60 }\n\
                 [[rules]]\nwhen = { position = { among = \"any\", previous = { widget = \"button\", \
                 part = \"label\" } } }\ntarget = \"button.body\"\nset = { min_width_px = 70 }";
    let theme = fixture(&[rules]);
    let scope = root(&theme, Direction::Ltr);
    let none: &[VariantRef] = &[];
    let siblings = [
        Sibling::new(BODY, none, StateFlags::new()),
        Sibling::new(LABEL, none, StateFlags::new()),
        Sibling::new(BODY, none, StateFlags::new()),
    ];
    let at = |index| {
        min_width(
            &theme,
            &Element::new(BODY, &scope, &Metrics).with_siblings(&siblings, index),
        )
    };
    assert_eq!(at(0), Some(Length::Px(50.0)));
    // The second body: even among the bodies, after a label among any.
    assert_eq!(at(2), Some(Length::Px(70.0)));
    assert_eq!(at(9), Some(Length::Px(50.0)));
    let two = [
        Sibling::new(BODY, none, StateFlags::new()),
        Sibling::new(BODY, none, StateFlags::new()),
    ];
    assert_eq!(
        min_width(
            &theme,
            &Element::new(BODY, &scope, &Metrics).with_siblings(&two, 1)
        ),
        Some(Length::Px(60.0))
    );
}

/// The flags (S7.2, S8, S10): the exclusive groups' drop, keyed flags the
/// widget does not declare dropped; a recency set's latest by the
/// connector's order, else by the set's stated order.
#[test]
fn flags_dropped_and_latest_by_order() {
    let rules = "[[rules]]\nwhen = { states = [\"pressed\"] }\ntarget = \"button.body\"\n\
                 set = { min_width_px = 50 }\n\
                 [[rules]]\nwhen = { latest = { lift = \"hover\" } }\ntarget = \"button.body\"\n\
                 set = { min_width_px = 60 }";
    let theme = fixture(&[rules]);
    let scope = root(&theme, Direction::Ltr);
    let states = StateFlags::new()
        .with(StateFlag::Disabled)
        .with(StateFlag::Pressed)
        .with(StateFlag::Widget(key("armed")))
        .with(StateFlag::Widget(key("nope")));
    let element = Element::new(BODY, &scope, &Metrics).with_states(states, &[]);
    assert_eq!(
        theme.kept_states(&element),
        StateFlags::new()
            .with(StateFlag::Disabled)
            .with(StateFlag::Widget(key("armed")))
    );
    assert_eq!(min_width(&theme, &element), Some(Length::Px(40.0)));
    let both = StateFlags::new()
        .with(StateFlag::Hover)
        .with(StateFlag::FocusVisible);
    let with_order = |order: &[StateFlag]| {
        min_width(
            &theme,
            &Element::new(BODY, &scope, &Metrics).with_states(both.clone(), order),
        )
    };
    assert_eq!(
        with_order(&[StateFlag::FocusVisible, StateFlag::Hover]),
        Some(Length::Px(60.0))
    );
    assert_eq!(
        with_order(&[StateFlag::Hover, StateFlag::FocusVisible]),
        Some(Length::Px(40.0))
    );
    // Listed twice: by its last place.
    assert_eq!(
        with_order(&[StateFlag::Hover, StateFlag::FocusVisible, StateFlag::Hover]),
        Some(Length::Px(60.0))
    );
    // Unreported: the stated order, focus before hover.
    assert_eq!(with_order(&[]), Some(Length::Px(40.0)));
    // A flag the drop removed counts for nothing.
    let pressed = StateFlags::new()
        .with(StateFlag::Hover)
        .with(StateFlag::Disabled)
        .with(StateFlag::Pressed);
    assert_eq!(
        min_width(
            &theme,
            &Element::new(BODY, &scope, &Metrics)
                .with_states(pressed, &[StateFlag::Hover, StateFlag::Pressed])
        ),
        Some(Length::Px(60.0))
    );
}

/// The sizes, density and direction (S7.3, S8): the default size by
/// variant, a density from the scope that the widget declares, one it does
/// not ignored, the element's own direction.
#[test]
fn size_density_and_direction_match() {
    let rules = "[[rules]]\nwhen = { sizes = [\"large\"] }\ntarget = \"button.body\"\n\
                 set = { min_width_px = 50 }\n\
                 [[rules]]\nwhen = { densities = [\"compact\"] }\ntarget = \"button.label\"\n\
                 set = { padding.top_px = 2 }\n\
                 [[rules]]\nwhen = { direction = \"rtl\" }\ntarget = \"button.label\"\n\
                 set = { padding.bottom_px = 3 }";
    let theme = fixture(&[rules]);
    let destructive = [VariantRef::Typed(Variant::Tinted)];
    let scope = root(&theme, Direction::Ltr);
    let (_, large) = drawn(
        &theme,
        &Element::new(BODY, &scope, &Metrics).with_variants(&destructive),
    );
    assert_eq!(
        min_width(
            &theme,
            &Element::new(BODY, &scope, &Metrics).with_variants(&destructive)
        ),
        Some(Length::Px(50.0))
    );
    assert_eq!(large.entries.last().unwrap().size, Some(ControlSize::Large));
    let mut inputs = RootInputs::new(size(800.0, 600.0), size(1920.0, 1080.0), Direction::Ltr);
    inputs.density = Some(Density::Compact);
    let compact = theme.root_scope(&inputs);
    // A declared density in force from the scope, while the sizes unit is
    // not enabled, is the sizes unit's (S8, S10; ADD-A147); an undeclared one
    // is ignored.
    assert_eq!(
        theme.style(&Element::new(LABEL, &compact, &Metrics)),
        Styled::NotAvailable(Unavailable::Later)
    );
    inputs.density = Some(Density::Keyed(key("dense")));
    let dense = theme.root_scope(&inputs);
    let label = drawn(&theme, &Element::new(LABEL, &dense, &Metrics)).0;
    assert_eq!(label.padding.top, Some(Px(0.0)));
    let label = drawn(
        &theme,
        &Element::new(LABEL, &scope, &Metrics).with_direction(Direction::Rtl),
    );
    assert_eq!(label.0.padding.bottom, Some(Px(3.0)));
    assert_eq!(label.1.direction, Direction::Rtl);
}

/// The size classes from the thresholds (R16): the largest bound not above
/// the window's size on each axis.
#[test]
fn size_classes_from_thresholds() {
    let theme = fixture(&[]);
    assert_eq!(
        theme.size_classes(size(700.0, 300.0)),
        SizeClasses {
            horizontal: Some(SizeClass::Medium),
            vertical: None,
        }
    );
    assert_eq!(
        theme.size_classes(size(900.0, 400.0)),
        SizeClasses {
            horizontal: Some(SizeClass::Expanded),
            vertical: Some(SizeClass::Regular),
        }
    );
}

/// `data_needs` (S8, S10): the part's declared data, the widget values'
/// with an element's head (task 11e's review), and a glyph or icon drawn to
/// its box; a `Full` corner operand adds nothing beyond the declaration,
/// which S12 requires to hold the box.
#[test]
fn data_needs_declared_full_corner_and_box() {
    let theme = fixture(&[]);
    assert_eq!(
        theme.data_needs(LABEL),
        DataNeeds {
            data: vec![DataName::Element(ElementDatum::CharWidth)],
            drawn_to_box: false,
        }
    );
    let full = "[widgets.button.body]\ncorners.top_left = { full = { maximum_px = \"absent\" } }\n\
                [widgets.button.label]\npadding.left_px = \"= widget.body.corners.top_left\"\n\
                [widgets.button.icon]\nicon.size_px = \"= min(element.width, element.height)\"";
    let theme = theme_with(
        &DECLARE.replace(
            "data = { label = [\"char_width\"] }",
            "data = { label = [\"char_width\", \"width\", \"height\"], icon = [\"width\", \
             \"height\"], values = [\"move_distance\"] }",
        ),
        &[full],
    );
    let needs = theme.data_needs(LABEL);
    assert_eq!(
        needs.data,
        [
            DataName::Element(ElementDatum::CharWidth),
            DataName::Element(ElementDatum::Width),
            DataName::Element(ElementDatum::Height),
        ]
    );
    assert!(!needs.drawn_to_box);
    assert!(theme.data_needs(ICON).drawn_to_box);
    assert_eq!(
        theme.data_needs(BODY).data,
        [DataName::Element(ElementDatum::MoveDistance)]
    );
    assert_eq!(
        theme.data_needs(PartRef::Component {
            key: key("rule_editor"),
            part: key("row"),
        }),
        DataNeeds::default()
    );
}

/// `element.caret_height` (S2.2, S8; ADD-A130): a part declaring it needs
/// it, and the query reads it from the element's data, not another datum.
#[test]
fn element_caret_height_evaluates() {
    let theme = theme_with(
        &DECLARE.replace(
            "data = { label = [\"char_width\"] }",
            "data = { label = [\"caret_height\"] }",
        ),
        &["[widgets.button.label]\npadding.top_px = \"= element.caret_height\""],
    );
    assert_eq!(
        theme.data_needs(LABEL).data,
        [DataName::Element(ElementDatum::CaretHeight)]
    );
    let scope = root(&theme, Direction::Ltr);
    let element = Element::new(LABEL, &scope, &Metrics).with_data(super::ElementData {
        caret_height: Px(17.0),
        move_distance: Px(3.0),
        ..super::ElementData::default()
    });
    let (label, _) = drawn(&theme, &element);
    assert_eq!(label.padding.top, Some(Px(17.0)));
}

/// The returned scope (S8): the entry of the part with its kept variants,
/// flags and size, its bindings, and the element's own direction and
/// language passed on; a context matches the entry's widget. A declared
/// density is the sizes unit's: while it is not enabled the query answers
/// `Later` for an element holding one (S8, S10; ADD-A147).
#[test]
fn returned_scope_records_the_entry() {
    let bound = "[widgets.button.body]\nbindings = [{ surface = \"button\" }]";
    let theme = fixture(&[bound]);
    let scope = root(&theme, Direction::Ltr);
    let element = Element::new(BODY, &scope, &Metrics).with_states(
        StateFlags::new()
            .with(StateFlag::Disabled)
            .with(StateFlag::Pressed),
        &[],
    );
    assert_eq!(
        theme.style(&element.clone().with_density(Density::Compact)),
        Styled::NotAvailable(Unavailable::Later)
    );
    let (body, inside) = drawn(&theme, &element);
    let entry = inside.entries.last().unwrap();
    assert_eq!(entry.part, BODY);
    assert_eq!(entry.states, StateFlags::new().with(StateFlag::Disabled));
    assert_eq!(entry.size, Some(ControlSize::Regular));
    assert_eq!(entry.surface, Some(SurfaceSet::Button));
    assert_eq!(
        body.fill.unwrap().layers.len(),
        1,
        "the body's fill reads its own binding"
    );
    assert_eq!(
        theme.role(&inside, RoleRef::Surface(SurfaceRole::Foreground)),
        Some(Color {
            r: 0x40 as f32 / 255.0,
            g: 0x50 as f32 / 255.0,
            b: 0x60 as f32 / 255.0,
            a: 1.0
        })
    );
}

/// Task 11e's review: the root scope's container is the display, at the
/// connector's radii where it reports them, else the `display` tokens'
/// (S4.6, S8); a part whose entry lists `corners` is the container of the
/// parts it encloses, with its stated corners.
#[test]
fn scope_records_the_container() {
    use super::Container;
    let declare = format!(
        "{DECLARE}[declare.params.display]\nunit = \"px\"\nkeys = [\"corner_radius_top_left\", \
         \"corner_radius_top_right\", \"corner_radius_bottom_right\", \
         \"corner_radius_bottom_left\"]\n"
    );
    let tokens = "[tokens.params.display]\ncorner_radius_top_left = 9\n\
                  corner_radius_top_right = 9\ncorner_radius_bottom_right = 9\n\
                  corner_radius_bottom_left = 9";
    let theme = theme_with(&declare, &[tokens]);
    let mut inputs = RootInputs::new(size(800.0, 600.0), size(1920.0, 1080.0), Direction::Ltr);
    assert_eq!(
        theme.root_scope(&inputs).container,
        Container::Display([Some(Px(9.0)); 4])
    );
    inputs.display_corner_radius = Some([Px(39.0); 4]);
    let scope = theme.root_scope(&inputs);
    assert_eq!(scope.container, Container::Display([Some(Px(39.0)); 4]));
    assert_eq!(
        fixture(&[])
            .root_scope(&RootInputs::new(
                size(800.0, 600.0),
                size(1920.0, 1080.0),
                Direction::Ltr
            ))
            .container,
        Container::Display([None; 4])
    );
    let element = Element::new(BODY, &scope, &Metrics);
    let (body, inside) = drawn(&theme, &element);
    assert_eq!(
        inside.container,
        Container::Part {
            part: BODY,
            corners: body.corners
        }
    );
}

/// `PartStyle.shown` (S2.7, S2.11; ADD-A126): an `Option<Value<bool>>`
/// resolving to `Option<bool>`, `None` where the part does not list it
/// (drawn), the stated flag where a style states it; spelled `shown =
/// true|false`, here in a keyframe's patch, which patches any field.
#[test]
fn part_shown_resolves_option() {
    use crate::blocks::part_style::PartStylePatch;
    use crate::value::Value;
    use crate::widgets::{ButtonPatch, PartPatch};
    let theme = fixture(&[]);
    let scope = root(&theme, Direction::Ltr);
    let element = Element::new(BODY, &scope, &Metrics);
    let shown: Option<bool> = drawn(&theme, &element).0.shown;
    assert_eq!(shown, None);
    for flag in [false, true] {
        let mut hidden = Layer::default();
        hidden.bases.widgets.button = Some(Box::new(ButtonPatch {
            body: Some(PartPatch::of_style(PartStylePatch {
                shown: Some(Some(Value::Stated(flag))),
                ..PartStylePatch::default()
            })),
            ..ButtonPatch::default()
        }));
        let theme = theme_of(DECLARE, vec![hidden]);
        let scope = root(&theme, Direction::Ltr);
        let element = Element::new(BODY, &scope, &Metrics);
        assert_eq!(drawn(&theme, &element).0.shown, Some(flag));
    }
    // A keyframe's patch states `shown` too, read by a loop's own reader:
    // `IconStyle.animation` is staged with the motion unit (S6.1).
    let shown = crate::toml::read::read_as::<crate::blocks::motion::Loop>(
        "{ period_ms = 100, timing = \"linear\", keyframes = [{ at = 0, timing = \"absent\", \
         patch = { shown = false } }] }",
        crate::toml::read::Spelling::Bare,
        "widgets.button.icon.icon.animation",
    )
    .map(|looped| {
        looped
            .keyframes
            .into_iter()
            .next()
            .map(|keyframe| keyframe.patch.shown)
    });
    assert_eq!(shown, Ok(Some(Some(Some(Value::Stated(false))))));
}

/// `ResolvedPartStyle.breeze_tests` (S2.7, S6.1, S8; ADD-A126): the Breeze
/// tests of every rule that can apply to the element, whatever its flags,
/// in rank order: as they first appear in the part's base transitions
/// (each step's list, then its focus indicator's transition), then a test
/// only a rule names, in the rules' order; a test stated twice once, a
/// stated zero's none, a rule the element's variants exclude left out.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn breeze_tests_in_rank_order() {
    use crate::blocks::motion::BreezeTest;
    let breeze = |group: &str, duration: u32, any: &str, none: &str| {
        format!(
            "{{ properties = [\"{group}\"], kind = \"interpolate\", duration_ms = {duration}, \
             delay_ms = 0, timing = {}, engine = {{ breeze = {{ any = [{any}], none = [{none}], \
             conditions = [] }} }} }}",
            if duration == 0 {
                "\"absent\""
            } else {
                "\"linear\""
            }
        )
    };
    let rule = |when: &str, item: String| {
        format!(
            "[[rules]]\nwhen = {{ {when} }}\ntarget = \"button.body\"\nset = {{ transitions = [{item}] }}\n"
        )
    };
    let stated = format!(
        "[widgets.button.body]\ntransitions = [{}, {}, {}]\n[widgets.button.body.focus]\n\
         transition = {{ properties = [], kind = \"interpolate\", duration_ms = 100, delay_ms = \
         0, timing = \"linear\", engine = {{ breeze = {{ any = [\"focused\"], none = [], \
         conditions = [] }} }}, from = \"none\" }}\n{}{}{}{}",
        breeze("fill", 100, "\"hover\"", ""),
        breeze("stroke", 100, "\"focus_visible\"", "\"disabled\""),
        breeze("opacity", 100, "\"hover\"", ""),
        rule(
            "variants = [\"tool\"]",
            breeze("fill", 100, "\"checked\"", "")
        ),
        rule(
            "states = [\"pressed\"]",
            breeze("fill", 100, "\"pressed\"", "")
        ),
        rule("states = [\"disabled\"]", breeze("fill", 0, "", "")),
        rule(
            "states = [\"hover\"]",
            breeze("stroke", 100, "\"hover\"", "")
        ),
    );
    let theme = fixture(&[&stated]);
    let scope = root(&theme, Direction::Ltr);
    let test = |any: StateFlag, none: &[StateFlag]| BreezeTest {
        any: vec![any],
        none: none.to_vec(),
        conditions: Vec::new(),
    };
    let ranked = [
        test(StateFlag::Hover, &[]),
        test(StateFlag::FocusVisible, &[StateFlag::Disabled]),
        test(StateFlag::Focused, &[]),
        test(StateFlag::Pressed, &[]),
    ];
    let element = Element::new(BODY, &scope, &Metrics);
    assert_eq!(drawn(&theme, &element).0.breeze_tests, ranked);
    let flat = [VariantRef::Typed(Variant::Tool)];
    let element = Element::new(BODY, &scope, &Metrics).with_variants(&flat);
    let mut with_flat = ranked.to_vec();
    with_flat.insert(3, test(StateFlag::Checked, &[]));
    assert_eq!(drawn(&theme, &element).0.breeze_tests, with_flat);
    let element = Element::new(LABEL, &scope, &Metrics);
    assert_eq!(drawn(&theme, &element).0.breeze_tests, []);
}

/// A `Keyframes` transition's keyframes resolve over the part as a loop's
/// do, each field it states `Some`, and mirror with the layout (S2.4, S2.7;
/// ADD-A134): the body's padding at the keyframe its start one in
/// right-to-left.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn keyframes_transition_resolves_and_mirrors() {
    use crate::blocks::motion::ResolvedEngine;
    let theme = fixture(&[
        "[widgets.button.body]\ntransitions = [{ properties = [\"geometry\", \"opacity\"], kind = \
         { keyframes = { keyframes = [{ at = 0, timing = \"absent\", patch = { padding.left_px = \
         10, opacity = 0 } }] } }, duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = \
         \"css_animation\" }]",
    ]);
    let keyframes = |direction: Direction| {
        let scope = root(&theme, direction);
        let style = drawn(&theme, &Element::new(BODY, &scope, &Metrics)).0;
        let transition = style.transitions.first().cloned().unwrap();
        assert_eq!(transition.engine, ResolvedEngine::CssAnimation);
        match transition.kind {
            ResolvedTransitionKind::Keyframes { keyframes } => keyframes,
            other => panic!("{other:?}"),
        }
    };
    let ltr = keyframes(Direction::Ltr);
    let patch = &ltr.first().unwrap().patch;
    assert_eq!(patch.opacity, Some(Some(Fraction(0.0))));
    let padding = patch.padding.unwrap();
    assert_eq!(
        (padding.left, padding.right),
        (Some(Px(10.0)), Some(Px(2.0)))
    );
    let rtl = keyframes(Direction::Rtl);
    let padding = rtl.first().unwrap().patch.padding.unwrap();
    assert_eq!(
        (padding.left, padding.right),
        (Some(Px(2.0)), Some(Px(10.0)))
    );
}

/// The button whose body lists one `Keyframes` transition of `keyframes`
/// (synthetic: a duration of no platform's), over the fixture's body, its
/// stroke stated with its outset absent, and `layers`.
fn body_keyframes_theme(keyframes: &str, layers: &[&str]) -> ResolvedTheme {
    let body = format!(
        "[widgets.button.body]\nstroke = {{ replace = {{ width_px = 1, paint = \"#000000\", \
         dash = \"solid\", snap = \"to_device_pixel\", position = \"inside\", outset_px = \
         \"absent\", fill_clip = \"box\" }} }}\ntransitions = [{{ properties = [\"fill\", \
         \"stroke\", \"opacity\", \"transform\", \"geometry\", \"state_layer\"], kind = {{ \
         keyframes = {{ keyframes = [{keyframes}] }} }}, duration_ms = 100, delay_ms = 0, \
         timing = \"linear\", engine = \"css_animation\" }}]"
    );
    let mut stacked = vec![body.as_str()];
    stacked.extend(layers);
    fixture(&stacked)
}

/// The keyframes of the body's `Keyframes` transition, as the query
/// answers them.
fn body_keyframes(theme: &ResolvedTheme) -> Vec<ResolvedKeyframe> {
    let scope = root(theme, Direction::Ltr);
    let kind = drawn(theme, &Element::new(BODY, &scope, &Metrics))
        .0
        .transitions
        .into_iter()
        .next()
        .map(|transition| transition.kind);
    match kind {
        Some(ResolvedTransitionKind::Keyframes { keyframes }) => keyframes,
        _ => Vec::new(),
    }
}

/// The one keyframe's patch on the body.
fn on_body(patch: &str) -> Option<ResolvedPartStylePatch> {
    let theme = body_keyframes_theme(
        &format!("{{ at = 0, timing = \"absent\", patch = {{ {patch} }} }}"),
        &[],
    );
    body_keyframes(&theme)
        .into_iter()
        .next()
        .map(|keyframe| keyframe.patch)
}

/// S2.7: a keyframe's `Fields` patch of a block resolves over the part's
/// resolved block, its `Some` that block with the keyframe's fields
/// applied, at any depth.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn keyframe_fields_resolve_over_the_resolved_block() {
    let patch = on_body("transform.rotate_deg = 90, stroke.top.width_px = 3").unwrap_or_default();
    let transform = patch.transform.flatten();
    assert_eq!(transform.as_ref().and_then(|t| t.rotate), Some(Deg(90.0)));
    assert_eq!(transform.as_ref().and_then(|t| t.scale_x), Some(1.0));
    assert_eq!(
        transform.as_ref().and_then(|t| t.translate_x),
        Some(Px(5.0))
    );
    let stroke = patch.stroke.flatten();
    assert_eq!(stroke.as_ref().and_then(|s| s.top.width), Some(Px(3.0)));
    assert_eq!(stroke.as_ref().and_then(|s| s.right.width), Some(Px(1.0)));
    assert_eq!(patch.opacity, None, "a field the keyframe does not set");
}

/// S2.7: `base` in a keyframe is the field's own resolved value on the part
/// before the keyframe applies.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn keyframe_base_is_the_parts_resolved_value() {
    let patch = on_body("min_width_px = \"= add(base, 2px)\", opacity = \"= mul(base, 0.5)\"")
        .unwrap_or_default();
    assert_eq!(patch.min_width, Some(Some(Length::Px(42.0))));
    assert_eq!(patch.opacity, Some(Some(Fraction(0.5))));
}

/// S2.7: a keyframe's `Append` appends over the part's resolved paint.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn keyframe_append_over_the_parts_paint() {
    let patch = on_body("fill = { append = [\"#ff0000\"] }").unwrap_or_default();
    let layers = patch.fill.flatten().map(|fill| fill.layers.len());
    assert_eq!(layers, Some(2), "the body's own layer, then the keyframe's");
}

/// S2.7: any keyframe patch of a field that resolves `Absent` on the part,
/// at any depth, is `None`, the field staying absent: a `Set`, a `Fields`
/// patch and an `Append` alike, and a field within a block the keyframe
/// sets.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn keyframe_over_an_absent_field_is_none() {
    let patch = on_body(
        "focus = \"= token.focus\", interaction.state_layer.alpha = 0.5, stroke.outset_px = 2, \
         stroke.top.width_px = 3",
    )
    .unwrap_or_default();
    assert_eq!(patch.focus, None, "a Set over an absent focus indicator");
    assert_eq!(patch.interaction, None, "a Fields patch over absent");
    let stroke = patch.stroke.flatten();
    assert_eq!(
        stroke.as_ref().map(|s| s.outset),
        Some(None),
        "absent within the stroke"
    );
    assert_eq!(stroke.as_ref().and_then(|s| s.top.width), Some(Px(3.0)));
    let theme = body_keyframes_theme(
        "{ at = 0, timing = \"absent\", patch = { fill = { append = [\"#ff0000\"] } } }",
        &["[widgets.button.body]\nfill = \"absent\""],
    );
    let patch = body_keyframes(&theme)
        .into_iter()
        .next()
        .map(|keyframe| keyframe.patch);
    assert_eq!(
        patch.map(|patch| patch.fill),
        Some(None),
        "an Append over absent"
    );
}

/// S2.7: a keyframe over a part field set by a reference applies after
/// every rule that patches the referenced value: its step sorts after
/// every step of the theme where the reference's steps are composed with
/// the patches after it (R2.4).
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn keyframe_over_a_referenced_field_follows_every_rule() {
    let tokens = "[tokens.focus]\nkind = \"ring\"\nwidth_px = 2\npaint = \"#3584e4\"\ndash = \
                  \"solid\"\noffset = { top_px = 0, right_px = 0, bottom_px = 0, left_px = 0 }\n\
                  corners = { top_left = \"follow_part\", top_right = \"follow_part\", \
                  bottom_right = \"follow_part\", bottom_left = \"follow_part\", curve = \
                  \"circular\" }\nshadow = \"absent\"\ntransition = \"absent\"\n\
                  [widgets.button.body]\nfocus = \"= token.focus\"\n[[rules]]\ntarget = \
                  \"button.body\"\nset = { focus.width_px = 3 }";
    let theme = body_keyframes_theme(
        "{ at = 0, timing = \"absent\", patch = { focus.width_px = \"= add(base, 1px)\" } }",
        &[tokens],
    );
    let scope = root(&theme, Direction::Ltr);
    let focus = drawn(&theme, &Element::new(BODY, &scope, &Metrics)).0.focus;
    assert_eq!(
        focus.and_then(|focus| focus.width),
        Some(Px(3.0)),
        "the rule's"
    );
    let width = body_keyframes(&theme)
        .into_iter()
        .next()
        .and_then(|keyframe| keyframe.patch.focus)
        .flatten()
        .and_then(|focus| focus.width);
    assert_eq!(width, Some(Px(4.0)), "over the rule's 3 px");
}
