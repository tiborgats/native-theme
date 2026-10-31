//! S8's query on fixture stacks. Every number, colour, name and metric here
//! is synthetic, none a platform's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use super::{DataNeeds, Element, FontMetrics, RootInputs, Scope, Sibling, Styled, Unavailable};
use crate::next::blocks::outline::ResolvedRadius;
use crate::next::blocks::part_style::{ResolvedPartStyle, RoleRef};
use crate::next::blocks::text::ResolvedFont;
use crate::next::color::Color;
use crate::next::declare::DataName;
use crate::next::expr::ElementDatum;
use crate::next::key::Key;
use crate::next::layer::Layer;
use crate::next::resolve::{ResolvedTheme, Stack, WindowConditions, WindowState};
use crate::next::roles::{SurfaceRole, SurfaceSet};
use crate::next::selector::{
    ControlSize, Density, Direction, InputKind, PartRef, SizeClass, SizeClasses, StateFlag,
    StateFlags, VariantRef, WidgetOrPart,
};
use crate::next::toml::layer::read_preset;
use crate::next::units::{Length, Px, Sides, Size};
use crate::next::widgets::{ButtonPart, ButtonValues, TypedPart, Variant};

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
variants = ["primary", "flat", "destructive", "@solo"]
sizes = ["small", "regular"]
densities = ["compact"]
flags = ["@armed"]
exclusive = [["disabled", "pressed"]]
exclusive_variants = [["primary", "flat"]]
recency = { lift = ["pressed", "focus_visible", "hover"] }
sizes_by_variant = [{ variant = "destructive", sizes = ["large"] }]
default_size = "regular"
default_size_by_variant = [{ variant = "destructive", size = "large" }]
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
transitions = []
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
menu_indicator_gap_px = 2
menu_indicator_anchor = "inline"
menu_trigger = "absent"
shows_icon = true
focus_visible_when = "any_focus"
repeat_delay_ms = "absent"
repeat_interval_ms = "absent"
back_title_fallback = false
back_display_mode = "absent"
help_tooltip_on_click = false
hit_extension = { top_px = 1, right_px = 2, bottom_px = 3, left_px = 4 }
display_mode = "absent"
presentations = []
"##;

fn layer(text: &str) -> Layer {
    Layer::from_toml(text).unwrap_or_else(|e| panic!("{e}"))
}

/// The theme the fixture's preset and `layers` resolve to, under `declare`.
fn theme_with(declare: &str, layers: &[&str]) -> ResolvedTheme {
    let file = read_preset(&format!(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n{declare}"
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    let mut stacked = vec![layer(PRESET)];
    stacked.extend(layers.iter().map(|text| layer(text)));
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
        Some(crate::next::widgets::IconPlacement::Trailing)
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
    let other = [VariantRef::Typed(Variant::Pill)];
    assert_eq!(
        theme.style(&Element::new(BODY, &scope, &Metrics).with_variants(&other)),
        Styled::NotAvailable(Unavailable::Variant)
    );
    let content = [
        VariantRef::Typed(Variant::IconOnly),
        VariantRef::Keyed(key("with_arrow")),
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
                 set = { drawn_as = { widget = \"button\", part = \"body\", variants = [\"flat\"] } }";
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
            variants: vec![VariantRef::Typed(Variant::Flat)],
        }
    );
    assert_eq!(
        theme.style(&Element::new(ICON, &scope, &Metrics)),
        Styled::NotDrawn
    );
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
    let destructive = [VariantRef::Typed(Variant::Destructive)];
    let small = Element::new(BODY, &scope, &Metrics)
        .with_variants(&destructive)
        .with_size(ControlSize::Small);
    assert_eq!(theme.style(&small), Styled::NotAvailable(Unavailable::Size));
    let large = large.with_variants(&destructive);
    assert!(matches!(theme.style(&large), Styled::Style(_)));
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
        VariantRef::Typed(Variant::Flat),
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
                 variants = [\"flat\"] }] }\ntarget = \"button.label\"\n\
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
    let flat = [VariantRef::Typed(Variant::Flat)];
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
    let destructive = [VariantRef::Typed(Variant::Destructive)];
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
    let label = drawn(&theme, &Element::new(LABEL, &compact, &Metrics)).0;
    assert_eq!(label.padding.top, Some(Px(2.0)));
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

/// `data_needs` (S8, S10): the part's declared data, the box where a `Full`
/// corner is read as an operand, and a glyph or icon drawn to its box.
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
            "data = { label = [\"char_width\"], icon = [\"width\", \"height\"] }",
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
        theme.data_needs(PartRef::Component {
            key: key("rule_editor"),
            part: key("row"),
        }),
        DataNeeds::default()
    );
}

/// The returned scope (S8): the entry of the part with its kept variants,
/// flags and size, its bindings, and the element's own density, direction
/// and language passed on; a context matches the entry's widget.
#[test]
fn returned_scope_records_the_entry() {
    let bound = "[widgets.button.body]\nbindings = [{ surface = \"button\" }]";
    let theme = fixture(&[bound]);
    let scope = root(&theme, Direction::Ltr);
    let element = Element::new(BODY, &scope, &Metrics)
        .with_states(
            StateFlags::new()
                .with(StateFlag::Disabled)
                .with(StateFlag::Pressed),
            &[],
        )
        .with_density(Density::Compact);
    let (body, inside) = drawn(&theme, &element);
    let entry = inside.entries.last().unwrap();
    assert_eq!(entry.part, BODY);
    assert_eq!(entry.states, StateFlags::new().with(StateFlag::Disabled));
    assert_eq!(entry.size, Some(ControlSize::Regular));
    assert_eq!(entry.surface, Some(SurfaceSet::Button));
    assert_eq!(inside.density, Some(Density::Compact));
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
