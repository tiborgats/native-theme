//! The resolve pass's evaluation (S2.2, S3.3, S8, S9.4). Every number,
//! colour, name and metric here is synthetic, none a platform's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use std::sync::Arc;

use super::{Cx, Refusal};
use crate::next::blocks::part_style::RoleRef;
use crate::next::blocks::text::ResolvedFont;
use crate::next::color::Color;
use crate::next::declare::Declarations;
use crate::next::expr::{Bound, Expr, FieldType, Name, Ty, Val};
use crate::next::key::Key;
use crate::next::layer::Layer;
use crate::next::query::{Element, FontMetrics, Scope, ScopeEntry};
use crate::next::resolve::compose::Target;
use crate::next::resolve::{ReaderState, ResolvedTheme, Stack, WindowConditions, WindowState};
use crate::next::roles::{SurfaceRole, SurfaceSet};
use crate::next::selector::{Direction, InputKind, PartRef, Scheme, Selector, StateFlags};
use crate::next::units::{Px, Size};
use crate::next::widgets::{ButtonPart, TypedPart};

/// Synthetic measurement: each metric a fixed share of the font's size.
struct Metrics;

fn size_of(font: &ResolvedFont) -> f32 {
    match font.size {
        Some(Px(size)) => size,
        None => f32::NAN,
    }
}

impl FontMetrics for Metrics {
    fn char_width(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) / 2.0)
    }
    fn max_char_width(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font))
    }
    fn x_height(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) / 4.0)
    }
    fn height(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) + 3.0)
    }
    fn line_spacing(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) + 5.0)
    }
    fn underline_offset(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) / 8.0)
    }
    fn underline_thickness(&self, _: &ResolvedFont) -> Px {
        Px(1.0)
    }
    fn strikeout_offset(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) / 3.0)
    }
    fn strikeout_thickness(&self, _: &ResolvedFont) -> Px {
        Px(1.5)
    }
}

const BODY: TypedPart = TypedPart::Button(ButtonPart::Body);
const LABEL: TypedPart = TypedPart::Button(ButtonPart::Label);

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: f32::from(r) / 255.0,
        g: f32::from(g) / 255.0,
        b: f32::from(b) / 255.0,
        a: 1.0,
    }
}

fn toml(text: &str) -> Layer {
    Layer::from_toml(text).unwrap_or_else(|e| panic!("{e}"))
}

/// The theme `layers` resolve to, the first the preset's, unvalidated (the
/// evaluation alone is tested), under the light scheme with a stand-in
/// scaling text by `factor`.
fn resolved(layers: &[&str], factor: f32) -> ResolvedTheme {
    let mut layers = layers.iter().map(|text| Arc::new(toml(text)));
    let preset = layers.next().unwrap_or_default();
    let stack = Stack {
        preset: None,
        declarations: Declarations::default(),
        preset_layer: preset,
        reader: Some(super::super::Reader::StandIn {
            state: ReaderState::new(Scheme::Light, factor),
            layer: Arc::default(),
        }),
        layers: layers
            .map(|layer| (crate::next::layer::LayerId::next(), layer))
            .collect(),
        accent: None,
        accent_layer: Arc::default(),
        picked: None,
        default_scheme: Scheme::Light,
        contrast: None,
        flavor: None,
    };
    stack
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme
}

fn scope() -> Scope {
    Scope::root(
        Size {
            width: Px(800.0),
            height: Px(600.0),
        },
        Size {
            width: Px(1920.0),
            height: Px(1080.0),
        },
        Direction::Ltr,
    )
}

/// The base fixture: roles, a body text style, the DPI and linear scaling.
const BASE: &str = r##"
[colors.light.surfaces.window]
foreground = "#102030"
background = "#f0f0f0"
[colors.light.surfaces.button]
foreground = "#405060"
background = "#e0e0e0"
[colors.light.global]
accent = "#3366cc"
[colors.light.keyed]
shade = "#808080"
[inputs.accent]
light = "#aa5500"
[tokens.motion]
speed_factor = 0.5
durations.short_ms = 100
[tokens.focus]
width_px = 2
[tokens.text]
font_dpi = 96
scaling = "linear"
[tokens.text.roles.body.replace]
font = { family = "Synthetic", size_pt = 9, weight = 400, style = "normal", width = "absent", axes = {}, features = {} }
scaling_role = "none"
letter_spacing = "absent"
word_spacing = "absent"
line_height = "auto"
decoration = "absent"
mnemonic_decoration = "absent"
case = "mixed"
elision = "end"
wrap = "word"
max_lines = "none"
fill = "absent"
[widgets.button.body]
min_width_px = 40
padding = { top_px = 6, right_px = 6, bottom_px = 6, left_px = 6 }
[widgets.button.label]
text = "= token.text.roles.body"
[widgets.button.values]
icon_text_gap_px = 4
"##;

/// `f` over a context evaluating `BODY` for an element of synthetic data,
/// in `scope`.
fn with_body<R>(theme: &ResolvedTheme, scope: &Scope, f: impl FnOnce(&mut Cx<'_>) -> R) -> R {
    let part = PartRef::Typed(BODY);
    let mut element = Element::new(part, scope, &Metrics);
    element.data.width = Px(120.0);
    element.data.height = Px(30.0);
    element.data.text = "Synthetic";
    element.data.item_index = 4;
    element.data.color = rgb(1, 2, 3);
    let all: &dyn Fn(&Selector) -> bool = &|_| true;
    let mut cx = Cx::new(theme, scope, Some(&element), all, Target::Part(BODY));
    cx.chain_part = Some(BODY);
    f(&mut cx)
}

fn reference(text: &str) -> crate::next::expr::Ref {
    match Expr::parse(text).unwrap() {
        Expr::Ref(reference) => reference,
        other => panic!("{other:?}"),
    }
}

/// The operand `text`, a reference, gives the body.
fn operand(theme: &ResolvedTheme, text: &str) -> Option<Val> {
    let scope = scope();
    with_body(theme, &scope, |cx| {
        cx.operand_of(&reference(text), Target::Part(BODY))
    })
}

fn length(px: f32) -> Option<Val> {
    Some(Val::Length(Px(px)))
}

/// One operand of each of S2.2's kinds, read as the body's query reads it:
/// roles, `surface.`, `active(…)`, tokens, inputs, `scheme`, `font_size`
/// and `line_height` (the part's text's, converted and scaled), the
/// element's data, the root's sizes, the font metrics, `body_font_height`,
/// `part.`, `widget.<part>.`, `values.`, `text_scale`.
#[test]
fn operands_of_each_kind() {
    let theme = resolved(&[BASE], 1.5);
    let color = |c| Some(Val::Color(c));
    assert_eq!(
        operand(&theme, "role.window.foreground"),
        color(rgb(0x10, 0x20, 0x30))
    );
    assert_eq!(
        operand(&theme, "role.global.accent"),
        color(rgb(0x33, 0x66, 0xcc))
    );
    assert_eq!(operand(&theme, "role.@shade"), color(rgb(0x80, 0x80, 0x80)));
    assert_eq!(
        operand(&theme, "surface.foreground"),
        color(rgb(0x10, 0x20, 0x30))
    );
    assert_eq!(
        operand(&theme, "active(surface.background)"),
        color(rgb(0xf0, 0xf0, 0xf0))
    );
    assert_eq!(
        operand(&theme, "input.accent"),
        color(rgb(0xaa, 0x55, 0x00))
    );
    assert_eq!(
        operand(&theme, "token.motion.speed_factor"),
        Some(Val::Number(0.5))
    );
    assert_eq!(
        operand(&theme, "token.motion.durations.short"),
        Some(Val::Duration(crate::next::units::Ms(100.0)))
    );
    assert_eq!(operand(&theme, "token.focus.width"), length(2.0));
    assert_eq!(
        operand(&theme, "scheme"),
        Some(Val::Name(Name::Variant(Key::new("light").unwrap())))
    );
    // The body has no text: `font_size` is none there; the label's is the
    // body slot's 9pt at 96 DPI, 12px, scaled by 1.5.
    assert_eq!(operand(&theme, "font_size"), None);
    assert_eq!(
        operand(&theme, "widget.label.font_size"),
        Some(Val::FontSize(Px(18.0)))
    );
    // `auto`: the font's line spacing, 18 + 5.
    assert_eq!(operand(&theme, "widget.label.line_height"), length(23.0));
    assert_eq!(operand(&theme, "widget.label.char_width"), length(9.0));
    assert_eq!(operand(&theme, "element.width"), length(120.0));
    assert_eq!(operand(&theme, "element.height"), length(30.0));
    assert_eq!(
        operand(&theme, "element.item_index"),
        Some(Val::Number(4.0))
    );
    assert_eq!(
        operand(&theme, "element.item_count"),
        Some(Val::Number(1.0))
    );
    assert_eq!(
        operand(&theme, "element.text"),
        Some(Val::Text("Synthetic".to_owned()))
    );
    assert_eq!(operand(&theme, "element.color"), color(rgb(1, 2, 3)));
    assert_eq!(operand(&theme, "element.window_width"), length(800.0));
    assert_eq!(operand(&theme, "element.screen_height"), length(1080.0));
    // The body slot's font, 9pt: 12px, scaled 18; its height 18 + 3.
    assert_eq!(operand(&theme, "body_font_height"), length(21.0));
    assert_eq!(operand(&theme, "part.min_width"), length(40.0));
    assert_eq!(operand(&theme, "part.padding.left"), length(6.0));
    assert_eq!(operand(&theme, "values.icon_text_gap"), length(4.0));
    assert_eq!(operand(&theme, "text_scale"), Some(Val::Number(1.5)));
}

/// `base`: the value the steps before this one gave (S2.2).
#[test]
fn base_reads_the_steps_below() {
    let ruled = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                 set = { min_width_px = \"= add(base, 8px)\" }";
    let theme = resolved(&[BASE, ruled], 1.0);
    assert_eq!(operand(&theme, "part.min_width"), length(48.0));
}

/// The call `text` typed as `ty` on the body, its value.
fn call(theme: &ResolvedTheme, text: &str, ty: Ty) -> Option<Val> {
    let scope = scope();
    let expr = Expr::parse(text).unwrap();
    with_body(theme, &scope, |cx| {
        cx.call(
            &expr,
            &FieldType::new(ty, Bound::Unbounded),
            Target::Part(BODY),
            |_| None,
        )
    })
}

/// Every function `next` implements, over operands read through the
/// scope: the model's arithmetic and the platform functions.
#[test]
fn functions_evaluate_over_read_operands() {
    let theme = resolved(&[BASE], 1.0);
    let length = |px| Some(Val::Length(Px(px)));
    assert_eq!(
        call(&theme, "add(part.min_width, 2px)", Ty::Length),
        length(42.0)
    );
    assert_eq!(
        call(&theme, "sub(part.min_width, token.focus.width)", Ty::Length),
        length(38.0)
    );
    assert_eq!(
        call(
            &theme,
            "mul(part.min_width, token.motion.speed_factor)",
            Ty::Length
        ),
        length(20.0)
    );
    assert_eq!(
        call(&theme, "div(part.min_width, 3)", Ty::Length),
        length(40.0 / 3.0)
    );
    assert_eq!(
        call(&theme, "min(part.min_width, element.width)", Ty::Length),
        length(40.0)
    );
    assert_eq!(
        call(&theme, "max(part.min_width, element.width)", Ty::Length),
        length(120.0)
    );
    assert_eq!(
        call(&theme, "floor(div(part.min_width, 3))", Ty::Length),
        length(13.0)
    );
    assert_eq!(
        call(&theme, "round(div(part.min_width, 3))", Ty::Length),
        length(13.0)
    );
    let accent = rgb(0x33, 0x66, 0xcc);
    assert_eq!(
        call(&theme, "alpha(role.global.accent, 0.5)", Ty::Color),
        Some(Val::Color(Color { a: 0.5, ..accent }))
    );
    assert_eq!(
        call(&theme, "with_alpha(role.global.accent, 0.25)", Ty::Color),
        Some(Val::Color(Color { a: 0.25, ..accent }))
    );
    // Halfway between two opaque colours, in sRGB.
    let mixed = call(
        &theme,
        "mix(role.window.foreground, role.window.background, 0.5)",
        Ty::Color,
    );
    let (fg, bg) = (rgb(0x10, 0x20, 0x30), rgb(0xf0, 0xf0, 0xf0));
    let Some(Val::Color(mixed)) = mixed else {
        panic!("{mixed:?}")
    };
    assert!((mixed.r - (fg.r + bg.r) / 2.0).abs() < 1e-6, "{mixed:?}");
    assert!((mixed.b - (fg.b + bg.b) / 2.0).abs() < 1e-6, "{mixed:?}");
    // The item index 4 picks the second of three, its remainder by three 1.
    assert_eq!(
        call(
            &theme,
            "pick(element.item_index, #ff0000, #00ff00, #0000ff)",
            Ty::Color
        ),
        Some(Val::Color(rgb(0, 255, 0)))
    );
    // The platform functions, each as `next::platform` computes it for the
    // same operands.
    let channels = |c: Color| [c.r, c.g, c.b, c.a];
    let colour = |[r, g, b, a]: [f32; 4]| Color { r, g, b, a };
    let lerp = colour(
        crate::next::platform::kirigami_lerp::kirigami_lerp(channels(fg), channels(bg), 0.5)
            .unwrap(),
    );
    assert_eq!(
        call(
            &theme,
            "kirigami_lerp(role.window.foreground, role.window.background, 0.5)",
            Ty::Color
        ),
        Some(Val::Color(lerp))
    );
    let shade = colour(crate::next::platform::kde_shade::kde_shade(
        channels(bg),
        crate::next::platform::kde_shade::KdeShadeRole::Dark,
        4.0,
    ));
    assert_eq!(
        call(
            &theme,
            "kde_shade(role.window.background, dark, 4)",
            Ty::Color
        ),
        Some(Val::Color(shade))
    );
}

/// S3.3's two steps: `surface.x` finds the nearest `Role { Surface(x) }` or
/// `Surface(S)` binding, the part's own innermost, a `Surface(S)` answer
/// then `role.S.x` walked again from the element outward, so a nearer
/// `Role { Set(S, x) }` binding counts; with no binding `role.window.x`.
#[test]
fn bindings_follow_s3_3_in_two_steps() {
    let theme = resolved(&[BASE], 1.0);
    let foreground = SurfaceRole::from_path("foreground").unwrap();
    let entry = |surface, roles| ScopeEntry {
        part: PartRef::Typed(LABEL),
        variants: Vec::new(),
        states: StateFlags::new(),
        size: None,
        surface,
        roles,
    };
    let colour = |scope: &Scope, text: &str| {
        with_body(&theme, scope, |cx| {
            cx.operand_of(&reference(text), Target::Part(BODY))
        })
    };
    let color = |c| Some(Val::Color(c));
    // No binding: the window set's.
    assert_eq!(
        colour(&scope(), "surface.foreground"),
        color(rgb(0x10, 0x20, 0x30))
    );
    // An ancestor binds the button set: `role.button.foreground`.
    let button = scope().entered(entry(Some(SurfaceSet::Button), Vec::new()));
    assert_eq!(
        colour(&button, "surface.foreground"),
        color(rgb(0x40, 0x50, 0x60))
    );
    // A nearer ancestor binds `role.button.foreground` itself: step 2 finds
    // it from the element outward.
    let bound = button.entered(entry(
        None,
        vec![(
            RoleRef::Set(SurfaceSet::Button, foreground),
            Some(rgb(9, 9, 9)),
        )],
    ));
    assert_eq!(colour(&bound, "surface.foreground"), color(rgb(9, 9, 9)));
    // A `Role { Surface(x) }` binding answers step 1 directly, but never
    // step 2.
    let surface_role = scope().entered(entry(
        None,
        vec![(RoleRef::Surface(foreground), Some(rgb(7, 7, 7)))],
    ));
    assert_eq!(
        colour(&surface_role, "surface.foreground"),
        color(rgb(7, 7, 7))
    );
    assert_eq!(
        colour(&surface_role, "role.window.foreground"),
        color(rgb(0x10, 0x20, 0x30))
    );
    // `active(…)` follows the `Surface(S)` binding, not the role binding.
    assert_eq!(
        colour(&bound, "active(surface.foreground)"),
        color(rgb(0x40, 0x50, 0x60))
    );
    // The part's own `Surface(window)` binding is innermost: it decides over
    // the ancestors' button set.
    let own = resolved(
        &[
            BASE,
            "[[widgets.button.body.bindings]]\nsurface = \"window\"",
        ],
        1.0,
    );
    let colour = with_body(&own, &button, |cx| {
        cx.part(BODY);
        cx.operand_of(&reference("surface.foreground"), Target::Part(BODY))
    });
    assert_eq!(colour, color(rgb(0x10, 0x20, 0x30)));
}

/// A reference leading back to the place being evaluated, and a chain of
/// references longer than S2.2's 32, are refused at evaluation as S12
/// refuses them: the value is not drawn, the refusal recorded.
#[test]
fn cycles_and_too_deep_chains_refused() {
    let cycle = "[widgets.button.values]\nicon_text_gap_px = \"= values.subtitle_gap\"\n\
                 subtitle_gap_px = \"= values.icon_text_gap\"";
    let theme = resolved(&[BASE, cycle], 1.0);
    let scope = scope();
    let (value, refusals) = with_body(&theme, &scope, |cx| {
        let value = cx.operand_of(&reference("values.icon_text_gap"), Target::Part(BODY));
        (value, cx.refusals.clone())
    });
    assert_eq!(value, None);
    assert_eq!(
        refusals,
        [Refusal::Cycle {
            target: Target::Values(crate::next::widgets::WidgetKind::Button),
            path: "icon_text_gap".to_owned(),
        }]
    );
    // Thirty-three durations, each the next's reference, the last stated.
    let mut chain = "[tokens.motion.durations]\n".to_owned();
    for index in 0..33 {
        chain.push_str(&format!(
            "d{index}_ms = \"= token.motion.durations.d{}\"\n",
            index + 1
        ));
    }
    chain.push_str("d33_ms = 7\n");
    let theme = resolved(&[BASE, &chain], 1.0);
    let long = |start: usize| {
        with_body(&theme, &scope, |cx| {
            let text = format!("token.motion.durations.d{start}");
            let value = cx.operand_of(&reference(&text), Target::Part(BODY));
            (value, cx.refusals.clone())
        })
    };
    // From d2, 32 references reach the stated d33.
    assert_eq!(long(2).0, Some(Val::Duration(crate::next::units::Ms(7.0))));
    // From d1, 33: refused.
    let (value, refusals) = long(1);
    assert_eq!(value, None);
    assert!(
        matches!(refusals.as_slice(), [Refusal::TooDeep { .. }]),
        "{refusals:?}"
    );
}

/// A whole synthetic text style 10px tall, under `table`.
fn text_style(table: &str) -> String {
    format!(
        "[{table}.replace]\nfont = {{ family = \"Synthetic\", size_px = 10, weight = 400, \
         style = \"normal\", width = \"absent\", axes = {{}}, features = {{}} }}\n\
         scaling_role = \"none\"\nletter_spacing = \"absent\"\nword_spacing = \"absent\"\n\
         line_height = \"auto\"\ndecoration = \"absent\"\nmnemonic_decoration = \"absent\"\n\
         case = \"mixed\"\nelision = \"end\"\nwrap = \"word\"\nmax_lines = \"none\"\n\
         fill = \"absent\"\n"
    )
}

/// The `body` slot's font size, evaluated.
fn body_size(theme: &ResolvedTheme) -> Option<Px> {
    let scope = scope();
    with_body(theme, &scope, |cx| {
        let path = ["text", "roles", "body"].map(str::to_owned).to_vec();
        cx.follow(Target::Tokens, path, false, |cx, folds| {
            cx.block::<crate::next::blocks::text::TextStyle>(folds)
        })
    })
    .and_then(|style| style.font.size)
}

/// A `Fields` patch over a reference composes lazily (S2.2), evaluated: the
/// referenced style's bases (the preset's and a later layer's), then the
/// patch stated in a base (a reader's font patch on the slot), before every
/// rule (a preset's `Tokens` rule on the referenced style still reaches
/// the slot).
#[test]
fn fields_patch_composes_lazily_before_rules() {
    let preset = format!(
        "{}[tokens.text.roles]\nbody = \"= token.text.keyed.x\"\n",
        text_style("tokens.text.keyed.x")
    );
    let reader = "[tokens.text.roles.body.font]\nsize_px = 14";
    let later = "[tokens.text.keyed.x.font]\nsize_px = 16";
    // The referenced style's later base comes before the slot's patch.
    assert_eq!(
        body_size(&resolved(&[&preset, reader, later], 1.0)),
        Some(Px(14.0))
    );
    // A rule on the referenced style comes after every base patch.
    let ruled = format!(
        "{preset}[[rules]]\nwhen = {{ conditions = [\"scheme:light\"] }}\ntarget = \"tokens\"\n\
         set = {{ text.keyed.x.font.size_px = 12 }}\n"
    );
    assert_eq!(
        body_size(&resolved(&[&ruled, reader, later], 1.0)),
        Some(Px(12.0))
    );
    // A rule stated before the reference's own rule's place, on the slot,
    // comes before it; one after, after it.
    let slot_rule = |size| {
        format!(
            "[[rules]]\nwhen = {{ conditions = [\"scheme:light\"] }}\ntarget = \"tokens\"\n\
             set = {{ text.roles.body.font.size_px = {size} }}\n"
        )
    };
    let after = format!("{ruled}{}", slot_rule(18));
    assert_eq!(
        body_size(&resolved(&[&after, reader, later], 1.0)),
        Some(Px(18.0))
    );
    let before = format!(
        "{preset}{}[[rules]]\nwhen = {{ conditions = [\"scheme:light\"] }}\n\
         target = \"tokens\"\nset = {{ text.keyed.x.font.size_px = 12 }}\n",
        slot_rule(18)
    );
    assert_eq!(
        body_size(&resolved(&[&before, reader, later], 1.0)),
        Some(Px(12.0))
    );
}

/// The whole button of a fixture stack: the preset's parts and values, a
/// later layer's patch on the label's referenced text style, a rule. Every
/// field the catalogue lists for each part, compared with what the fixture
/// gives by hand (S8): a part's field its entry does not list is not drawn,
/// a list it does not list empty.
#[test]
fn whole_button_resolves_field_by_field() {
    use crate::next::blocks::focus::ResolvedFocusVisibleWhen;
    use crate::next::blocks::outline::{CornerCurve, ResolvedCorners, ResolvedRadius};
    use crate::next::blocks::paint::{
        BlendMode, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer,
    };
    use crate::next::blocks::part_style::CursorShape;
    use crate::next::blocks::part_style::ResolvedBinding;
    use crate::next::blocks::transform::ResolvedTransform;
    use crate::next::units::{Length, Ms, Sides};
    use crate::next::widgets::{ButtonValues, IconPlacement, IndicatorAnchor, WidgetKind};
    let parts = r##"
[widgets.button.body]
fill = "= surface.background"
stroke = "absent"
corners_px = 4
corners.curve = "circular"
shadow = "absent"
margin = { top_px = "absent", right_px = "absent", bottom_px = "absent", left_px = "absent" }
min_height_px = "auto"
max_width_px = "auto"
opacity = "= sub(1, token.motion.speed_factor)"
transform = { scale_x = 1, scale_y = 1, rotate_deg = 0, translate_x_px = 0, translate_y_px = "= div(token.focus.width, 2)", origin = [0.5, 0.5] }
focus = "absent"
cursor = "pointer"
touch_target_px = "absent"
transitions = []
interaction = "absent"
[[widgets.button.body.bindings]]
surface = "button"
[widgets.button.label]
padding = { top_px = 1, right_px = 1, bottom_px = 1, left_px = "= div(font_size, 2)" }
[widgets.button.subtitle]
absent = true
[widgets.button.icon]
absent = true
[widgets.button.menu_indicator]
absent = true
[widgets.button.values]
icon_placement = "leading"
label_position = "absent"
subtitle_gap_px = "absent"
menu_indicator_gap_px = 2
menu_indicator_anchor = "inline"
menu_trigger = "absent"
shows_icon = true
focus_visible_when = "any_focus"
repeat_delay_ms = "absent"
repeat_interval_ms = "= token.motion.durations.short"
back_title_fallback = false
back_display_mode = "absent"
help_tooltip_on_click = false
hit_extension = { top_px = "absent", right_px = "absent", bottom_px = "absent", left_px = "absent" }
display_mode = "absent"
presentations = []
"##;
    let later = "[tokens.text.roles.body.font]\nweight = 700";
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                set = { min_width_px = \"= add(base, 8px)\" }";
    let theme = resolved(&[BASE, parts, later, rule], 1.5);
    let scope = scope();
    let style = |part: TypedPart, hover: bool| {
        let element = Element::new(PartRef::Typed(part), &scope, &Metrics);
        let applies: &dyn Fn(&Selector) -> bool = &move |_| hover;
        let evaluated = theme.evaluate_part(part, &element, applies);
        assert_eq!(evaluated.refusals, []);
        evaluated.value
    };
    // The body, at rest and hovered.
    let body = style(BODY, false).expect("the body is drawn");
    let layer = |c| ResolvedPaintLayer {
        kind: ResolvedPaintKind::Color(Some(c)),
        blend: BlendMode::Normal,
    };
    // Its own `Surface(button)` binding: the button set's background.
    assert_eq!(
        body.fill,
        Some(ResolvedPaint {
            layers: vec![layer(rgb(0xe0, 0xe0, 0xe0))]
        })
    );
    assert_eq!(body.stroke, None);
    let four = ResolvedRadius::Px(Some(Px(4.0)));
    assert_eq!(
        body.corners,
        Some(ResolvedCorners {
            top_left: four.clone(),
            top_right: four.clone(),
            bottom_right: four.clone(),
            bottom_left: four,
            curve: CornerCurve::Circular,
        })
    );
    assert_eq!(body.shadow, None);
    let six = Some(Px(6.0));
    assert_eq!(
        body.padding,
        Sides {
            top: six,
            right: six,
            bottom: six,
            left: six
        }
    );
    assert_eq!(
        body.margin,
        Sides {
            top: None,
            right: None,
            bottom: None,
            left: None
        }
    );
    assert_eq!(body.min_width, Some(Length::Px(40.0)));
    assert_eq!(body.min_height, Some(Length::Auto));
    assert_eq!(body.max_width, Some(Length::Auto));
    // 1 - 0.5.
    assert_eq!(body.opacity, Some(crate::next::units::Fraction(0.5)));
    assert_eq!(
        body.transform,
        Some(ResolvedTransform {
            scale_x: Some(1.0),
            scale_y: Some(1.0),
            rotate: Some(crate::next::units::Deg(0.0)),
            translate_x: Some(Px(0.0)),
            // 2px / 2.
            translate_y: Some(Px(1.0)),
            origin: (0.5, 0.5),
        })
    );
    assert_eq!(body.focus, None);
    assert_eq!(body.cursor, Some(CursorShape::Pointer));
    assert_eq!(body.touch_target, None);
    assert_eq!(
        body.bindings,
        [ResolvedBinding::Surface(SurfaceSet::Button)]
    );
    assert_eq!(body.transitions, []);
    assert_eq!(body.interaction, None);
    // Fields the body's entry does not list.
    assert_eq!(
        (body.text, body.icon, body.glyph, body.shape),
        (None, None, None, None)
    );
    // Hovered: the rule over the base, 40 + 8.
    let hovered = style(BODY, true).expect("the body is drawn");
    assert_eq!(hovered.min_width, Some(Length::Px(48.0)));
    // The label: the body slot's style by reference, the later layer's
    // weight over it; 9pt at 96 DPI is 12px, scaled by 1.5: 18px.
    let label = style(LABEL, false).expect("the label is drawn");
    let text = label.text.expect("the label has text");
    assert_eq!(text.font.family, Some("Synthetic".to_owned()));
    assert_eq!(text.font.size, Some(Px(18.0)));
    assert_eq!(text.font.weight, Some(700));
    assert_eq!(text.line_height, Some(Length::Auto));
    assert_eq!(text.letter_spacing, None);
    assert_eq!(text.fill, None);
    // `div(font_size, 2)`: the scaled size, 18 / 2.
    let one = Some(Px(1.0));
    assert_eq!(
        label.padding,
        Sides {
            top: one,
            right: one,
            bottom: one,
            left: Some(Px(9.0))
        }
    );
    // The parts the preset states absent are not drawn.
    for part in [
        ButtonPart::Subtitle,
        ButtonPart::Icon,
        ButtonPart::MenuIndicator,
    ] {
        assert_eq!(style(TypedPart::Button(part), false), None, "{part:?}");
    }
    // The values.
    let element = Element::new(PartRef::Typed(BODY), &scope, &Metrics);
    let values = theme
        .evaluate_values::<ButtonValues>(WidgetKind::Button, &element, &|_| true)
        .value
        .expect("the values resolve");
    assert_eq!(values.icon_text_gap, Some(Px(4.0)));
    assert_eq!(values.icon_placement, Some(IconPlacement::Leading));
    assert_eq!(values.label_position, None);
    assert_eq!(values.subtitle_gap, None);
    assert_eq!(values.menu_indicator_gap, Some(Px(2.0)));
    assert_eq!(values.menu_indicator_anchor, Some(IndicatorAnchor::Inline));
    assert_eq!(values.menu_trigger, None);
    assert_eq!(values.shows_icon, Some(true));
    assert_eq!(
        values.focus_visible_when,
        Some(ResolvedFocusVisibleWhen::AnyFocus)
    );
    assert_eq!(values.repeat_delay, None);
    assert_eq!(values.repeat_interval, Some(Ms(100.0)));
    assert_eq!(values.back_title_fallback, Some(false));
    assert_eq!(values.back_display_mode, None);
    assert_eq!(values.help_tooltip_on_click, Some(false));
    assert_eq!(
        values.hit_extension,
        Sides {
            top: None,
            right: None,
            bottom: None,
            left: None
        }
    );
    assert_eq!(values.display_mode, None);
    assert_eq!(values.presentations, []);
    // A role read in the scope.
    let accent = theme.evaluate_role(&scope, &reference("role.global.accent"));
    assert_eq!(accent.value, Some(rgb(0x33, 0x66, 0xcc)));
}

/// S9.4: points convert first, then text scales, then lengths derive: a
/// font size stated over another style's reads it converted, not scaled,
/// and is scaled once; a line height in points is converted and scaled
/// under `Linear`; a spacing in pixels is scaled and keeps its sign; one
/// stated on `font_size` follows the scaled font and is not scaled again.
#[test]
fn text_scales_once_after_conversion() {
    let style = text_style("tokens.text.keyed.y");
    let patch = "[tokens.text.keyed.y]\nfont.size = \"= mul(token.text.roles.body.font.size, 2)\"\n\
                 line_height_pt = 10\nletter_spacing_px = -1\n\
                 word_spacing = \"= mul(font_size, 0.5)\"\n\
                 [widgets.button.label]\ntext = \"= token.text.keyed.y\"";
    let theme = resolved(&[BASE, &style, patch], 1.5);
    let scope = scope();
    let element = Element::new(PartRef::Typed(LABEL), &scope, &Metrics);
    let text = theme
        .evaluate_part(LABEL, &element, &|_| true)
        .value
        .and_then(|label| label.text)
        .expect("the label has text");
    // The body slot's 9pt is 12px, doubled 24, scaled once: 36.
    assert_eq!(text.font.size, Some(Px(36.0)));
    // 10pt at 96 DPI, scaled.
    assert_eq!(
        text.line_height,
        Some(crate::next::units::Length::Px(10.0 * 96.0 / 72.0 * 1.5))
    );
    assert_eq!(text.letter_spacing, Some(Px(-1.5)));
    assert_eq!(text.word_spacing, Some(Px(18.0)));
}

/// Task 11a's review: an input reading itself is a cycle, refused at
/// evaluation as any other (S12), not an unbounded recursion.
#[test]
fn input_cycle_refused() {
    let cycle = "[inputs.accent]\nlight = \"= input.accent\"\n\
                 [colors.light.global]\naccent = \"= input.accent\"";
    let theme = resolved(&[BASE, cycle], 1.0);
    let scope = scope();
    let found = theme.evaluate_role(&scope, &reference("role.global.accent"));
    assert_eq!(found.value, None);
    assert!(
        matches!(found.refusals.as_slice(), [Refusal::Cycle { .. }]),
        "{:?}",
        found.refusals
    );
}

/// Task 11a's review: `active(…)` reads every place it reaches from the
/// bases alone, before any rule, and follows no `Role` binding (S2.2).
#[test]
fn active_reads_the_bases_throughout() {
    let layers = "[colors.light.surfaces.window]\nbackground = \"= role.global.accent\"\n\
                  [[rules]]\nwhen = { conditions = [\"scheme:light\"] }\ntarget = \"colors\"\n\
                  set = { global.accent = \"#ff0000\" }";
    let theme = resolved(&[BASE, layers], 1.0);
    assert_eq!(
        operand(&theme, "active(role.window.background)"),
        Some(Val::Color(rgb(0x33, 0x66, 0xcc)))
    );
    // Without `active`, the rule reaches it.
    assert_eq!(
        operand(&theme, "role.window.background"),
        Some(Val::Color(rgb(0xff, 0, 0)))
    );
}

/// Task 11a's review: the DPI and the scaling are read outside a chain of
/// references, so a chain of 32 ending in a call is not refused.
#[test]
fn sizes_read_outside_the_chain() {
    let mut chain = "[tokens.motion.durations]\n".to_owned();
    for index in 2..33 {
        chain.push_str(&format!(
            "d{index}_ms = \"= token.motion.durations.d{}\"\n",
            index + 1
        ));
    }
    chain.push_str("d33_ms = \"= add(7ms, 0ms)\"\n");
    let theme = resolved(&[BASE, &chain], 1.0);
    let scope = scope();
    let (value, refusals) = with_body(&theme, &scope, |cx| {
        let value = cx.operand_of(&reference("token.motion.durations.d2"), Target::Part(BODY));
        (value, cx.refusals.clone())
    });
    assert_eq!(value, Some(Val::Duration(crate::next::units::Ms(7.0))));
    assert_eq!(refusals, []);
}

/// Task 11a's review: a `Font.size` over a `Font.size` over a stated size
/// reads both converted and not yet scaled, and is scaled once (S9.4).
#[test]
fn nested_font_sizes_scale_once() {
    let a = text_style("tokens.text.keyed.a");
    let b = text_style("tokens.text.keyed.b");
    let c = text_style("tokens.text.keyed.c");
    let patch = "[tokens.text.keyed.b]\nfont.size = \"= mul(token.text.keyed.a.font.size, 2)\"\n\
                 [tokens.text.keyed.c]\nfont.size = \"= mul(token.text.keyed.b.font.size, 1.5)\"\n\
                 [widgets.button.label]\ntext = \"= token.text.keyed.c\"";
    let theme = resolved(&[BASE, &format!("{a}{b}{c}"), patch], 1.5);
    let scope = scope();
    let element = Element::new(PartRef::Typed(LABEL), &scope, &Metrics);
    let size = theme
        .evaluate_part(LABEL, &element, &|_| true)
        .value
        .and_then(|label| label.text)
        .and_then(|text| text.font.size);
    // 10px, doubled, times 1.5, scaled once by 1.5.
    assert_eq!(size, Some(Px(10.0 * 2.0 * 1.5 * 1.5)));
}

/// Task 11a's review: a call stated on a keyed scalar is typed by its
/// group's declared unit (S2.2, S4.6) and evaluates, as an operand too.
#[test]
fn keyed_scalar_calls_typed_by_their_unit() {
    let params = "[tokens.params.layout]\nk0 = \"= add(2px, 2px)\"";
    let mut theme = resolved(&[BASE, params], 1.0);
    theme.declarations = crate::next::toml::layer::read_preset(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\n\
         params = { layout = { unit = \"px\", keys = [\"k0\"] } }\n",
    )
    .unwrap()
    .declarations;
    assert_eq!(operand(&theme, "token.params.layout.k0"), length(4.0));
}

/// Task 11a's review: a corner operand is a length (S8): a `Px` corner its
/// radius, a `Full` one half the smaller side of the element's box (120 by
/// 30 here), in a call as a plain reference alike.
#[test]
fn corner_operands_are_lengths() {
    let px = "[widgets.button.body]\ncorners_px = 4";
    let theme = resolved(&[BASE, px], 1.0);
    assert_eq!(operand(&theme, "part.corners.top_left"), length(4.0));
    assert_eq!(
        call(&theme, "add(part.corners.top_left, 1px)", Ty::Length),
        length(5.0)
    );
    let full = "[widgets.button.body.corners]\ntop_left = { full = { maximum_px = \"absent\" } }";
    let theme = resolved(&[BASE, full], 1.0);
    assert_eq!(operand(&theme, "part.corners.top_left"), length(15.0));
}

/// Task 11a's review: a field read below a reference composes lazily as
/// the block does (S2.2): the slot's size read directly is the referenced
/// style's rule's, over the reader's base patch on the slot, as the slot
/// itself resolves.
#[test]
fn a_field_below_a_reference_composes_lazily() {
    let preset = format!(
        "{}[tokens.text.roles]\nbody = \"= token.text.keyed.x\"\n\
         [[rules]]\nwhen = {{ conditions = [\"scheme:light\"] }}\ntarget = \"tokens\"\n\
         set = {{ text.keyed.x.font.size_px = 12 }}\n",
        text_style("tokens.text.keyed.x")
    );
    let reader = "[tokens.text.roles.body.font]\nsize_px = 14";
    let theme = resolved(&[&preset, reader], 1.0);
    assert_eq!(body_size(&theme), Some(Px(12.0)));
    assert_eq!(
        operand(&theme, "token.text.roles.body.font.size"),
        Some(Val::FontSize(Px(12.0)))
    );
    // A later base stating the slot whole replaces the reference.
    let whole = text_style("tokens.text.roles.body");
    let theme = resolved(&[&preset, reader, &whole], 1.0);
    assert_eq!(
        operand(&theme, "token.text.roles.body.font.size"),
        Some(Val::FontSize(Px(10.0)))
    );
}

/// Task 11a's review: a plain reference to a place-less value is limited as
/// a call's result is (S2.2's *Totality*): a fraction clamped, a non-finite
/// number zero; and a font size read from a length is that length.
#[test]
fn plain_references_are_total() {
    let theme = resolved(
        &[
            BASE,
            "[widgets.button.body]\nopacity = \"= element.progress\"\n\
             [widgets.button.label.text]\nletter_spacing = \"= token.focus.width\"",
        ],
        1.5,
    );
    let scope = scope();
    let opacity = |progress: f32| {
        let mut element = Element::new(PartRef::Typed(BODY), &scope, &Metrics);
        element.data.progress = crate::next::units::Fraction(progress);
        theme
            .evaluate_part(BODY, &element, &|_| true)
            .value
            .and_then(|body| body.opacity)
    };
    assert_eq!(opacity(1.5), Some(crate::next::units::Fraction(1.0)));
    assert_eq!(opacity(f32::NAN), Some(crate::next::units::Fraction(0.0)));
    let element = Element::new(PartRef::Typed(LABEL), &scope, &Metrics);
    let spacing = theme
        .evaluate_part(LABEL, &element, &|_| true)
        .value
        .and_then(|label| label.text)
        .and_then(|text| text.letter_spacing);
    assert_eq!(spacing, Some(Px(2.0)));
}

/// `widget.<part>` is evaluated in the scope that part is itself queried
/// in (S2.2): the bindings of the parts enclosing it and its own, not the
/// querying part's. The label reads the body's fill as the body draws it,
/// under the body's own `Surface(button)` binding.
#[test]
fn widget_part_reads_that_parts_bindings() {
    let bound = "[widgets.button.body]\nfill = \"= surface.background\"\n\
                 bindings = [{ surface = \"button\" }]";
    let theme = resolved(&[BASE, bound], 1.0);
    let scope = scope();
    let element = Element::new(PartRef::Typed(LABEL), &scope, &Metrics);
    let all: &dyn Fn(&Selector) -> bool = &|_| true;
    let mut cx = Cx::new(&theme, &scope, Some(&element), all, Target::Part(LABEL));
    cx.chain_part = Some(LABEL);
    let fill = cx.operand_of(&reference("widget.body.fill"), Target::Part(LABEL));
    let Some(Val::Paint(paint)) = fill else {
        panic!("a paint: {fill:?}");
    };
    assert_eq!(
        format!("{:?}", paint.layers),
        format!(
            "[ResolvedPaintLayer {{ kind: Color(Some({:?})), blend: Normal }}]",
            rgb(0xe0, 0xe0, 0xe0)
        )
    );
}

/// Task 11b's review: a plain reference to element data is total in an
/// `auto` length and in a paint too (S2.2's *Totality*): a negative length
/// where the field cannot be negative zero, a non-finite datum zero.
#[test]
fn plain_references_are_total_in_auto_lengths_and_paints() {
    let theme = resolved(
        &[
            BASE,
            "[widgets.button.body]\nmin_width_px = \"= element.scroll_offset\"\n\
             fill = \"= element.color\"",
        ],
        1.0,
    );
    let scope = scope();
    let body = |offset: f32, red: f32| {
        let mut element = Element::new(PartRef::Typed(BODY), &scope, &Metrics);
        element.data.scroll_offset = Px(offset);
        element.data.color = Color {
            r: red,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        };
        theme
            .evaluate_part(BODY, &element, &|_| true)
            .value
            .expect("the body is drawn")
    };
    use crate::next::units::Length;
    assert_eq!(body(-3.0, 0.5).min_width, Some(Length::Px(0.0)));
    assert_eq!(body(f32::NAN, 0.5).min_width, Some(Length::Px(0.0)));
    assert_eq!(body(7.0, 0.5).min_width, Some(Length::Px(7.0)));
    let fill = body(0.0, f32::NAN).fill.expect("a fill");
    assert_eq!(
        format!("{:?}", fill.layers),
        format!(
            "[ResolvedPaintLayer {{ kind: Color(Some({:?})), blend: Normal }}]",
            Color {
                r: 0.0,
                g: 0.5,
                b: 0.5,
                a: 1.0
            }
        )
    );
}

/// Task 11b's review: the DPI and the scaling, read once, are read from
/// the bases and the rules even where an `active(…)` reads first (S2.2,
/// S9.4): the rule's linear scaling scales the body slot's 12px by 1.5.
#[test]
fn sizes_read_outside_active() {
    let base = BASE.replace("scaling = \"linear\"", "scaling = \"android_non_linear\"");
    let layers = "[colors.light.surfaces.window]\nbackground = \"= mix(#ffffff, #000000, 0.5)\"\n\
                  [[rules]]\nwhen = { conditions = [\"scheme:light\"] }\ntarget = \"tokens\"\n\
                  set = { text.scaling = \"linear\" }";
    let theme = resolved(&[&base, layers], 1.5);
    let scope = scope();
    let size = with_body(&theme, &scope, |cx| {
        let _ = cx.operand_of(
            &reference("active(role.window.background)"),
            Target::Part(BODY),
        );
        cx.operand_of(
            &reference("token.text.roles.body.font.size"),
            Target::Part(BODY),
        )
    });
    assert_eq!(size, Some(Val::FontSize(Px(18.0))));
}

/// Task 11a's review: a text style's font and line height are resolved once
/// for its frame and taken from there, so what their evaluation refuses is
/// recorded once.
#[test]
fn text_style_font_resolved_once() {
    let cycle =
        "[tokens.text.roles.body.font]\nsize = \"= mul(token.text.roles.body.font.size, 1)\"";
    let theme = resolved(&[BASE, cycle], 1.0);
    let scope = scope();
    let element = Element::new(PartRef::Typed(LABEL), &scope, &Metrics);
    let evaluated = theme.evaluate_part(LABEL, &element, &|_| true);
    assert_eq!(evaluated.refusals.len(), 1, "{:?}", evaluated.refusals);
}
