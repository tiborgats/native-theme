//! The resolve pass's evaluation (S2.2, S3.3, S8, S9.4). Every number,
//! colour, name and metric here is synthetic, none a platform's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use std::sync::Arc;

use super::{Cx, Reached, Refusal};
use crate::blocks::part_style::RoleRef;
use crate::blocks::text::{AxisTag, AxisValue, ResolvedFont};
use crate::color::Color;
use crate::declare::Declarations;
use crate::expr::{Bound, Expr, FieldType, Name, Ty, Val};
use crate::key::Key;
use crate::layer::Layer;
use crate::query::{Element, FontMetrics, Scope, ScopeEntry};
use crate::resolve::compose::Target;
use crate::resolve::{ReaderState, ResolvedTheme, Stack, WindowConditions, WindowState};
use crate::roles::{SurfaceRole, SurfaceSet};
use crate::selector::{Direction, InputKind, PartRef, Scheme, Selector, StateFlags};
use crate::units::{Px, Size};
use crate::widgets::{ButtonPart, TypedPart};

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
    resolved_by(layers, ReaderState::new(Scheme::Light, factor))
}

/// [`resolved`], the stand-in reporting `state`.
fn resolved_by(layers: &[&str], state: ReaderState) -> ResolvedTheme {
    resolved_with(layers, state, Declarations::default())
}

/// [`resolved`] over the declarations `declarations`, a keyed scalar
/// typed by its group's declared unit (S4.6).
fn resolved_over(layers: &[&str], declarations: Declarations) -> ResolvedTheme {
    resolved_with(layers, ReaderState::new(Scheme::Light, 1.0), declarations)
}

/// A chain of `count` keyed durations, `token.params.chain.d<n>`, each the
/// next's reference from `d<first>`, the last `d<count>` stated `last`
/// (synthetic), and its group's declaration.
fn duration_chain(first: usize, count: usize, last: &str) -> (String, Declarations) {
    let mut chain = "[tokens.params.chain]\n".to_owned();
    for index in first..count {
        chain.push_str(&format!(
            "d{index} = \"= token.params.chain.d{}\"\n",
            index + 1
        ));
    }
    chain.push_str(&format!("d{count} = {last}\n"));
    let keys: Vec<String> = (first..=count).map(|n| format!("\"d{n}\"")).collect();
    let declare = format!(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\nparams = {{ chain = {{ \
         unit = \"ms\", keys = [{}] }} }}",
        keys.join(", ")
    );
    let declarations = crate::toml::layer::read_preset(&declare)
        .map(|file| file.declarations)
        .unwrap_or_default();
    (chain, declarations)
}

/// [`resolved_by`] over the declarations `declarations`.
fn resolved_with(layers: &[&str], state: ReaderState, declarations: Declarations) -> ResolvedTheme {
    let mut layers = layers.iter().map(|text| Arc::new(toml(text)));
    let preset = layers.next().unwrap_or_default();
    let stack = Stack {
        preset: None,
        declarations,
        preset_layer: preset,
        derivation: None,
        reader_left_out: None,
        reader: Some(super::super::Reader::StandIn {
            state,
            layer: Arc::default(),
        }),
        layers: layers
            .map(|layer| (crate::layer::LayerId::next(), layer))
            .collect(),
        palette: None,
        port_left_out: None,
        accent: None,
        accent_layer: Arc::default(),
        picked: None,
        default_scheme: Scheme::Light,
        contrast: None,
        flavor: None,
        pending: Vec::new(),
        synthetic: None,
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
[tokens.focus]
width_px = 2
[tokens.text]
font_dpi = 96
scaling = "linear"
[tokens.text.roles.body.replace]
font = { family = "Synthetic", size_pt = 9, weight = 400, style = "normal", width = "absent", axes = {}, features = {} }
scaling_role = "none"
letter_spacing_px = 0
word_spacing_px = 0
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
repeat_delay_ms = 100
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

fn reference(text: &str) -> crate::expr::Ref {
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
        operand(&theme, "values.repeat_delay"),
        Some(Val::Duration(crate::units::Ms(100.0)))
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

/// The body's focus offset on `side`, evaluated as its `Value<Px>` field.
fn focus_offset(theme: &ResolvedTheme, side: &str) -> Option<Px> {
    let scope = scope();
    with_body(theme, &scope, |cx| {
        let path = ["focus", "offset", side].map(str::to_owned).to_vec();
        cx.follow(Target::Part(BODY), path, false, |cx, folds| {
            cx.value::<Px>(folds, true)
        })
    })
}

/// A length's plain reference to a text style's font size or line height
/// (S2.2) gives that place's value converted and scaled (S9.4), as the
/// same reference read as a call's operand does: the label's 9pt at 96 DPI,
/// 12px, and its 14px line height, each scaled by 1.5.
#[test]
fn a_length_reads_a_font_size_plainly() {
    let offsets = "[widgets.button.label.text]\nline_height_px = 14\n\
                   [widgets.button.body.focus.offset]\n\
                   top_px = \"= widget.label.font_size\"\n\
                   bottom_px = \"= add(widget.label.font_size, 0px)\"\n\
                   left_px = \"= widget.label.line_height\"\n\
                   right_px = \"= add(widget.label.line_height, 0px)\"";
    let theme = resolved(&[BASE, offsets], 1.5);
    assert_eq!(focus_offset(&theme, "bottom"), Some(Px(18.0)));
    assert_eq!(focus_offset(&theme, "right"), Some(Px(21.0)));
    assert_eq!(focus_offset(&theme, "left"), Some(Px(21.0)));
    assert_eq!(focus_offset(&theme, "top"), Some(Px(18.0)));
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

/// Every function the crate implements, over operands read through the
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
    // The platform functions, each as `platform` computes it for the
    // same operands.
    let channels = |c: Color| [c.r, c.g, c.b, c.a];
    let colour = |[r, g, b, a]: [f32; 4]| Color { r, g, b, a };
    let lerp = colour(
        crate::platform::kirigami_lerp::kirigami_lerp(channels(fg), channels(bg), 0.5).unwrap(),
    );
    assert_eq!(
        call(
            &theme,
            "kirigami_lerp(role.window.foreground, role.window.background, 0.5)",
            Ty::Color
        ),
        Some(Val::Color(lerp))
    );
    let shade = colour(crate::platform::kde_shade::kde_shade(
        channels(bg),
        crate::platform::kde_shade::KdeShadeRole::Dark,
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

/// Synthetic KDE tokens (S4.8): each effect and amount a value no other
/// group shares, the inactive effects on, the selection switch `change`.
fn kde_tokens(change: bool) -> String {
    format!(
        r##"[tokens.kde]
disabled = {{ intensity = "darken", intensity_amount = 0.3, color = "fade", color_amount = 0.1, color_color = "#204060", contrast = "tint", contrast_amount = 0.5 }}
inactive = {{ intensity = "shade", intensity_amount = 0.2, color = "tint", color_amount = 0.4, color_color = "#605040", contrast = "fade", contrast_amount = 0.6 }}
inactive_enabled = true
inactive_change_selection_color = {change}
tint_factor = "none"
titlebar_is_accent_colored = false
"##
    )
}

/// `kde_effects` and `kde_selection_inactive` read `token.kde` themselves
/// (S2.2, S4.8): each call is its function over the explicit operands and
/// the tokens as the theme resolves them, a group's effects its own.
#[test]
fn kde_functions_read_their_tokens() {
    use crate::platform::kde_effects::{KdeGroupEffects, kde_effects};
    use crate::platform::kde_selection_inactive::kde_selection_inactive;
    use crate::tokens::{
        KdeColorEffect, KdeContrastEffect, KdeGroup, KdeIntensityEffect, KdeRoleKind,
    };
    let channels = |c: Color| [c.r, c.g, c.b, c.a];
    let colour = |[r, g, b, a]: [f32; 4]| Some(Val::Color(Color { r, g, b, a }));
    let (fg, bg) = (
        channels(rgb(0x10, 0x20, 0x30)),
        channels(rgb(0xf0, 0xf0, 0xf0)),
    );
    let button = channels(rgb(0xe0, 0xe0, 0xe0));
    let kde = kde_tokens(true);
    let theme = resolved(&[BASE, &kde], 1.0);
    assert_eq!(
        operand(&theme, "token.kde.inactive.intensity"),
        Some(Val::Name(Name::Variant(Key::new("shade").unwrap())))
    );
    let inactive = KdeGroupEffects {
        intensity: KdeIntensityEffect::Shade,
        intensity_amount: 0.2,
        color: KdeColorEffect::Tint,
        color_amount: 0.4,
        color_color: channels(rgb(0x60, 0x50, 0x40)),
        contrast: KdeContrastEffect::Fade,
        contrast_amount: 0.6,
    };
    let disabled = KdeGroupEffects {
        intensity: KdeIntensityEffect::Darken,
        intensity_amount: 0.3,
        color: KdeColorEffect::Fade,
        color_amount: 0.1,
        color_color: channels(rgb(0x20, 0x40, 0x60)),
        contrast: KdeContrastEffect::Tint,
        contrast_amount: 0.5,
    };
    let effects = |group, kind, effects: &KdeGroupEffects| {
        colour(kde_effects(fg, bg, group, kind, effects, true))
    };
    assert_eq!(
        call(
            &theme,
            "kde_effects(role.window.foreground, role.window.background, inactive, foreground)",
            Ty::Color
        ),
        effects(KdeGroup::Inactive, KdeRoleKind::Foreground, &inactive)
    );
    assert_eq!(
        call(
            &theme,
            "kde_effects(role.window.foreground, role.window.background, disabled, background)",
            Ty::Color
        ),
        effects(KdeGroup::Disabled, KdeRoleKind::Background, &disabled)
    );
    let selection = "kde_selection_inactive(role.window.foreground, role.button.background, \
                     role.window.background, background)";
    assert_eq!(
        call(&theme, selection, Ty::Color),
        colour(kde_selection_inactive(
            fg,
            button,
            bg,
            KdeRoleKind::Background,
            true
        ))
    );
    let unchanged = kde_tokens(false);
    let theme = resolved(&[BASE, &unchanged], 1.0);
    assert_eq!(
        call(&theme, selection, Ty::Color),
        colour(kde_selection_inactive(
            fg,
            button,
            bg,
            KdeRoleKind::Background,
            false
        ))
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
        synthetic: false,
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
            target: Target::Values(crate::widgets::WidgetKind::Button),
            path: "icon_text_gap".to_owned(),
        }]
    );
    // Thirty-three keyed durations, each the next's reference, the last
    // stated (the motion tokens' keyed durations are staged, S6.1).
    let (chain, declarations) = duration_chain(0, 33, "\"= 7ms\"");
    let theme = resolved_over(&[BASE, &chain], declarations);
    let long = |start: usize| {
        with_body(&theme, &scope, |cx| {
            let text = format!("token.params.chain.d{start}");
            let value = cx.operand_of(&reference(&text), Target::Part(BODY));
            (value, cx.refusals.clone())
        })
    };
    // From d2, 32 references reach the stated d33.
    assert_eq!(long(2).0, Some(Val::Duration(crate::units::Ms(7.0))));
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
         scaling_role = \"none\"\nletter_spacing_px = 0\nword_spacing_px = 0\n\
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
            cx.block::<crate::blocks::text::TextStyle>(folds)
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
    use crate::blocks::focus::ResolvedFocusVisibleWhen;
    use crate::blocks::outline::{CornerCurve, ResolvedCorners, ResolvedRadius};
    use crate::blocks::paint::{BlendMode, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer};
    use crate::blocks::part_style::CursorShape;
    use crate::blocks::part_style::ResolvedBinding;
    use crate::blocks::transform::ResolvedTransform;
    use crate::units::{Length, Ms, Sides};
    use crate::widgets::{ButtonValues, IconPlacement, WidgetKind};
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
shows_icon = true
focus_visible_when = "any_focus"
repeat_delay_ms = "absent"
repeat_interval_ms = "= mul(token.motion.speed_factor, 200ms)"
hit_extension = { top_px = "absent", right_px = "absent", bottom_px = "absent", left_px = "absent" }
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
    assert_eq!(body.opacity, Some(crate::units::Fraction(0.5)));
    assert_eq!(
        body.transform,
        Some(ResolvedTransform {
            scale_x: Some(1.0),
            scale_y: Some(1.0),
            rotate: Some(crate::units::Deg(0.0)),
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
    assert_eq!(text.letter_spacing, Some(Px(0.0)));
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
    assert_eq!(values.shows_icon, Some(true));
    assert_eq!(
        values.focus_visible_when,
        Some(ResolvedFocusVisibleWhen::AnyFocus)
    );
    assert_eq!(values.repeat_delay, None);
    assert_eq!(values.repeat_interval, Some(Ms(100.0)));
    assert_eq!(
        values.hit_extension,
        Sides {
            top: None,
            right: None,
            bottom: None,
            left: None
        }
    );
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
        Some(crate::units::Length::Px(10.0 * 96.0 / 72.0 * 1.5))
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

/// An input reading a role's active value, read by that role's own rule,
/// is no cycle (S12, ADD-A144: a port's inactive input under the inactive
/// rule): `active(…)` reads the role's bases alone, which do not read the
/// rule. One the role's bases read back is.
#[test]
fn an_input_reading_its_roles_active_value_under_its_rule() {
    let input = "[inputs.accent]\nlight = \"= active(role.global.accent)\"\n";
    let rule = "[[rules]]\nwhen = { conditions = [\"scheme:light\"] }\ntarget = \"colors\"\n\
                set = { global.accent = \"= mix(input.accent, #000000, 0.5)\" }";
    let theme = resolved(&[BASE, &format!("{input}{rule}")], 1.0);
    let found = theme.evaluate_role(&scope(), &reference("role.global.accent"));
    assert_eq!(found.refusals, []);
    assert!(found.value.is_some(), "{:?}", found.value);
    let read_back = format!("{input}[colors.light.global]\naccent = \"= input.accent\"");
    let theme = resolved(&[BASE, &read_back], 1.0);
    let found = theme.evaluate_role(&scope(), &reference("role.global.accent"));
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
    let (chain, declarations) = duration_chain(2, 33, "\"= add(7ms, 0ms)\"");
    let theme = resolved_over(&[BASE, &chain], declarations);
    let scope = scope();
    let (value, refusals) = with_body(&theme, &scope, |cx| {
        let value = cx.operand_of(&reference("token.params.chain.d2"), Target::Part(BODY));
        (value, cx.refusals.clone())
    });
    assert_eq!(value, Some(Val::Duration(crate::units::Ms(7.0))));
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
    theme.declarations = crate::toml::layer::read_preset(
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
        element.data.progress = crate::units::Fraction(progress);
        theme
            .evaluate_part(BODY, &element, &|_| true)
            .value
            .and_then(|body| body.opacity)
    };
    assert_eq!(opacity(1.5), Some(crate::units::Fraction(1.0)));
    assert_eq!(opacity(f32::NAN), Some(crate::units::Fraction(0.0)));
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
    use crate::units::Length;
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

/// Task 11e's review: a call on a material's parameter evaluates by its
/// declared unit, as validation types it (S2.2, S10).
#[test]
fn material_params_typed_by_their_unit() {
    use crate::blocks::paint::ResolvedPaintKind;
    use crate::units::Scalar;
    let parts = r##"
[widgets.button.body]
fill = { layers = [{ kind = { material = { identity = "mica", blur_radius_px = "absent", tint = "absent", saturation = "absent", luminosity = "absent", noise_opacity = "absent", fallback = "#000000", params = { depth = "= add(1px, 2px)" } } }, blend = "normal" }] }
"##;
    let mut theme = resolved(&[BASE, parts], 1.0);
    theme.declarations = crate::toml::layer::read_preset(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\nmaterials = [\"mica\"]\n\
         material_params = { depth = \"px\" }\n",
    )
    .unwrap()
    .declarations;
    let scope = scope();
    let element = Element::new(PartRef::Typed(BODY), &scope, &Metrics);
    let body = theme
        .evaluate_part(BODY, &element, &|_| true)
        .value
        .unwrap();
    let layer = body.fill.unwrap().layers.into_iter().next().unwrap();
    let ResolvedPaintKind::Material(material) = layer.kind else {
        panic!("a material layer: {layer:?}");
    };
    assert_eq!(
        material.params,
        [(Key::new("depth").unwrap(), Some(Scalar::Px(Px(3.0))))]
    );
}

/// Task 11e's review: a call on a keyed transition's parameter evaluates by
/// its declared unit, as validation types it (S2.2, S2.7, S10).
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn keyed_transition_params_typed_by_their_unit() {
    use crate::blocks::motion::ResolvedTransitionKind;
    use crate::units::Scalar;
    let parts = r##"
[widgets.button.body]
transitions = [{ properties = ["fill"], kind = { keyed = { key = "bounce", params = { amplitude = "= add(2px, 2px)" } } }, duration_ms = 100, delay_ms = 0, timing = "linear", engine = "gtk_css" }]
"##;
    let mut theme = resolved(&[BASE, parts], 1.0);
    theme.declarations = crate::toml::layer::read_preset(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\n\
         transition_kinds = { bounce = { amplitude = \"px\" } }\n",
    )
    .unwrap()
    .declarations;
    let scope = scope();
    let element = Element::new(PartRef::Typed(BODY), &scope, &Metrics);
    let body = theme
        .evaluate_part(BODY, &element, &|_| true)
        .value
        .unwrap();
    let transition = body.transitions.first().unwrap();
    let ResolvedTransitionKind::Keyed { params, .. } = &transition.kind else {
        panic!("a keyed transition: {transition:?}");
    };
    assert_eq!(
        params,
        &[(Key::new("amplitude").unwrap(), Some(Scalar::Px(Px(4.0))))]
    );
}

/// A `Difference` layer resolves as any layer, its blend kept, over the
/// layers beneath it in stated order (S2.3; ADD-A134): WinUI's caret, a
/// white layer of `token.caret.color` blended by `difference`.
#[test]
fn difference_layer_resolves_with_its_blend() {
    use crate::blocks::paint::{BlendMode, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer};
    let over = "[widgets.button.body]\nfill = { layers = [{ kind = { color = \"#102030\" }, \
                blend = \"normal\" }, { kind = { color = \"#ffffff\" }, blend = \"difference\" }] }";
    let theme = resolved(&[BASE, over], 1.0);
    let scope = scope();
    let element = Element::new(PartRef::Typed(BODY), &scope, &Metrics);
    let body = theme
        .evaluate_part(BODY, &element, &|_| true)
        .value
        .unwrap();
    let layer = |color, blend| ResolvedPaintLayer {
        kind: ResolvedPaintKind::Color(Some(color)),
        blend,
    };
    assert_eq!(
        body.fill,
        Some(ResolvedPaint {
            layers: vec![
                layer(rgb(0x10, 0x20, 0x30), BlendMode::Normal),
                layer(rgb(0xff, 0xff, 0xff), BlendMode::Difference),
            ]
        })
    );
}

/// A shadow layer resolves with its `falloff` kept, each layer its own
/// (S2.5; ADD-A134): a stack mixing the two falloffs is stated layer by
/// layer.
#[test]
fn shadow_layers_resolve_with_their_falloffs() {
    use crate::blocks::shadow::ShadowFalloff;
    let over = "[widgets.button.body]\nshadow = { layers = [{ x_px = 0, y_px = 1, blur_px = 2, \
                spread_px = 0, color = \"#00000026\", inset = false, falloff = \"gaussian\" }, \
                { x_px = 0, y_px = 2, blur_px = 8, spread_px = 0, color = \"#00000040\", \
                inset = false, falloff = \"kirigami_smoothstep\" }] }";
    let theme = resolved(&[BASE, over], 1.0);
    let scope = scope();
    let element = Element::new(PartRef::Typed(BODY), &scope, &Metrics);
    let body = theme
        .evaluate_part(BODY, &element, &|_| true)
        .value
        .unwrap();
    let falloffs: Option<Vec<ShadowFalloff>> = body
        .shadow
        .map(|layers| layers.iter().map(|layer| layer.falloff).collect());
    assert_eq!(
        falloffs,
        Some(vec![
            ShadowFalloff::Gaussian,
            ShadowFalloff::KirigamiSmoothstep
        ])
    );
}

/// A synthetic layer's step decides whether a part is drawn only where it
/// states a presence that changes what the steps before it make the part
/// (S6.3, S17, ADD-A128, ADD-A126): a step of fields alone decides
/// nothing, over a part a lower step leaves absent too; `absent = false`
/// over an absent part decides, and over a part drawn as itself decides
/// nothing; an `Absent` decides; a later step of another layer decides
/// over it.
#[test]
fn synthetic_steps_decide_presence_only_by_changing_it() {
    let style = "[widgets.button.body]\nopacity = 1\n";
    let absent = "[widgets.button.body]\nabsent = true\n";
    let present = "[widgets.button.body]\nabsent = false\n";
    // The stack's places: the preset, the stand-in, then the layers.
    let decides = |layers: &[&str], synthetic: usize| {
        let mut theme = resolved(layers, 1.0);
        theme.synthetic = vec![synthetic];
        theme.shown_by_synthetic(BODY, |_| true)
    };
    assert!(!decides(&[BASE, style], 2));
    assert!(!decides(&[BASE, absent, style], 3));
    assert!(decides(&[BASE, absent, present], 3));
    assert!(!decides(&[BASE, present], 2));
    assert!(decides(&[BASE, absent], 2));
    assert!(!decides(&[BASE, absent, present], 2));
}

/// What evaluation records it read of the synthetic route's layers (S17,
/// ADD-A128), a place by the field evaluated outside every reference: a
/// step deciding a reference's place itself, directly; a reference through
/// a function giving one value for many (`floor`); a role read through a
/// binding whose value reads one; a font size converted by a listed DPI,
/// through the text style a reference brings. A field reading none of them
/// is recorded at no place; outside the synthetic route nothing is.
#[test]
fn reads_of_synthetic_layers_are_recorded() {
    let synthetic = "[tokens.motion]\nspeed_factor = 2\n[tokens.text]\nfont_dpi = 72\n\
                     [colors.light.global]\naccent = \"#010203\"\n";
    let reading = "[widgets.button.body]\nopacity = \"= floor(token.motion.speed_factor)\"\n\
                   bindings = [{ role = { role = { surface = \"foreground\" }, value = \
                   \"= role.global.accent\" } }]\nfill = \"= surface.foreground\"\n";
    let plain = resolved(&[BASE, synthetic, reading], 1.0);
    let mut theme = plain.clone();
    // The stack's places: the preset, the stand-in, then the synthetic
    // layer.
    theme.synthetic = vec![2];
    let scope = scope();
    let field = |path: &str| -> Vec<String> { path.split('.').map(str::to_owned).collect() };
    let direct = with_body(&theme, &scope, |cx| {
        cx.operand_of(&reference("token.motion.speed_factor"), Target::Part(BODY));
        cx.reached().to_vec()
    });
    assert!(
        direct
            .iter()
            .any(|held| held.direct && held.meets(Target::Tokens, &field("motion.speed_factor"))),
        "{direct:?}"
    );
    let body = with_body(&theme, &scope, |cx| {
        cx.part(BODY);
        cx.reached().to_vec()
    });
    let reaches = |reached: &[Reached], target, path: &str| {
        reached.iter().any(|held| held.meets(target, &field(path)))
    };
    assert!(reaches(&body, Target::Part(BODY), "opacity"), "{body:?}");
    assert!(reaches(&body, Target::Part(BODY), "fill"), "{body:?}");
    assert!(!reaches(&body, Target::Part(BODY), "min_width"), "{body:?}");
    assert!(
        body.iter()
            .filter(|held| held.meets(Target::Part(BODY), &field("opacity")))
            .all(|held| !held.direct),
        "a reference's operand is no step of the field itself: {body:?}"
    );
    let label = Element::new(PartRef::Typed(LABEL), &scope, &Metrics);
    let read = theme.evaluate_in(
        &scope,
        Some(&label),
        &|_| true,
        Target::Part(LABEL),
        None,
        |cx| {
            cx.part(LABEL);
            cx.reached().to_vec()
        },
    );
    assert!(
        reaches(&read, Target::Part(LABEL), "text.font.size"),
        "{read:?}"
    );
    let none = with_body(&plain, &scope, |cx| {
        cx.part(BODY);
        cx.reached().to_vec()
    });
    assert_eq!(none, Vec::new());
}

/// Each place evaluation reads a synthetic layer's step through records it
/// at the field evaluated (S17's synthetic route, ADD-A128): a plain field,
/// an `Option`'s `None`, a keyed list, an `Option` block, a map
/// stated empty, a surface map stated empty, the text scaling a font size
/// is scaled by, the text frame's `font_size`, `line_height` and font
/// metrics, an enclosing part's bindings (the icon's, read from the body
/// through `widget.icon.…`) and an outer scope entry whose bindings read
/// one. A synthetic layer is the stack's third place: the preset, the
/// stand-in, then it.
#[test]
fn each_recording_site_records_a_synthetic_read() {
    use crate::resolve::evaluate::Resolve;
    use crate::roles::SurfaceRoles;
    use crate::tokens::{TextScaling, TextTokens};
    let scope = scope();
    let field = |path: &str| -> Vec<String> { path.split('.').map(str::to_owned).collect() };
    let theme = |layers: &[&str]| {
        let mut theme = resolved(layers, 1.0);
        theme.synthetic = vec![2];
        theme
    };
    // A token's field resolved alone, by the function its type takes.
    let token = |synthetic: &str, path: &str, read: &dyn Fn(&mut Cx<'_>, Vec<String>)| {
        let theme = theme(&[BASE, synthetic]);
        let reached = theme.evaluate_in(&scope, None, &|_| true, Target::Tokens, None, |cx| {
            read(cx, field(path));
            cx.reached().to_vec()
        });
        reached
            .iter()
            .any(|held| held.meets(Target::Tokens, &field(path)))
    };
    assert!(
        token(
            "[tokens.text]\nscaling = \"linear\"\n",
            "text.scaling",
            &|cx, path| {
                cx.follow(Target::Tokens, path, false, |cx, folds| {
                    cx.plain::<TextScaling>(folds)
                });
            }
        ),
        "plain"
    );
    assert!(
        token(
            "[tokens.size_classes.compact]\nhorizontal_px = \"none\"\nvertical_px = \"none\"\n",
            "size_classes.compact.horizontal",
            &|cx, path| {
                cx.follow(Target::Tokens, path, false, |cx, folds| {
                    cx.optional::<Px>(folds, false)
                });
            }
        ),
        "an Option's None"
    );
    assert!(
        token("[tokens.text]\nkeyed = {}\n", "text.keyed", &|cx, _| {
            cx.follow(Target::Tokens, field("text"), false, |cx, folds| {
                TextTokens::resolve(folds, cx)
            });
        }),
        "a map"
    );
    let surfaces = theme(&[BASE, "[colors.light]\nsurfaces = {}\n"]);
    let reached = surfaces.evaluate_in(&scope, None, &|_| true, Target::Roles, None, |cx| {
        cx.follow(Target::Roles, field("surfaces"), false, |cx, folds| {
            cx.surfaces::<SurfaceRoles>(folds)
        });
        cx.reached().to_vec()
    });
    assert!(
        reached
            .iter()
            .any(|held| held.meets(Target::Roles, &field("surfaces"))),
        "a surface map"
    );
    let part = |layers: &[&str], scope: &Scope, part: TypedPart, path: &str| {
        let theme = theme(layers);
        let element = Element::new(PartRef::Typed(part), scope, &Metrics);
        let reached = theme.evaluate_in(
            scope,
            Some(&element),
            &|_| true,
            Target::Part(part),
            None,
            |cx| {
                cx.part(part);
                cx.reached().to_vec()
            },
        );
        reached
            .iter()
            .any(|held| held.meets(Target::Part(part), &field(path)))
    };
    let body = |synthetic: &str, path: &str| part(&[BASE, synthetic], &scope, BODY, path);
    let interaction = "[widgets.button.body.interaction]\nstate_layer = { color = \
                       \"#000000\", alpha = 0.1 }\nripple = { effect = \"absent\", color = \
                       \"absent\", alpha = \"absent\", bounded = true, max_radius_px = \
                       \"absent\", padding_px = \"absent\", indication_box = \"none\" }\n";
    assert!(
        part(
            &[&format!("{BASE}{interaction}"), interaction],
            &scope,
            BODY,
            "interaction.ripple.indication_box"
        ),
        "an Option block"
    );
    assert!(
        body("[widgets.button.body]\nbindings = []\n", "bindings"),
        "a keyed list"
    );
    let scaling = "[tokens.text]\nscaling = \"linear\"\n";
    assert!(
        part(&[BASE, scaling], &scope, LABEL, "text.font.size"),
        "the scaling factor"
    );
    let dpi = "[tokens.text]\nfont_dpi = 72\n";
    let frame = "[widgets.button.label.text]\nletter_spacing = \"= mul(font_size, 0.1)\"\n\
                 word_spacing = \"= mul(line_height, 0.1)\"\n\
                 [widgets.button.label]\npadding.top_px = \"= element.char_width\"\n";
    for path in ["text.letter_spacing", "text.word_spacing", "padding.top"] {
        assert!(part(&[BASE, dpi, frame], &scope, LABEL, path), "{path}");
    }
    let bound = "[widgets.button.body]\nbindings = [{ role = { role = { surface = \"foreground\" }, \
                 value = \"= role.global.accent\" } }]\n[colors.light.global]\naccent = \"#010203\"\n";
    let reading = "[widgets.button.icon.icon]\nfill = \"= surface.foreground\"\n\
                   [widgets.button.body]\nfill = \"= widget.icon.icon.fill\"\n";
    assert!(
        part(&[BASE, bound, reading], &scope, BODY, "fill"),
        "an enclosing part's binding"
    );
    let outer = scope.entered(ScopeEntry {
        part: PartRef::Typed(LABEL),
        variants: Vec::new(),
        states: StateFlags::new(),
        size: None,
        surface: None,
        roles: vec![(
            RoleRef::Surface(SurfaceRole::from_path("foreground").unwrap()),
            Some(rgb(1, 2, 3)),
        )],
        synthetic: true,
    });
    let reading = "[widgets.button.body]\nfill = \"= surface.foreground\"\n";
    assert!(
        part(&[BASE, "", reading], &outer, BODY, "fill"),
        "an outer scope's binding"
    );
    assert!(!part(&[BASE, "", reading], &scope, BODY, "fill"));
}

/// A synthetic layer's step holding a map merged by key reaches the map's
/// place and the entries it states, and no other (S6.1, S17 *Where a
/// listed path is found*, ADD-A144): stating a keyed role and a surface
/// set's role beside the preset's, it reaches neither the preset's keyed
/// role nor the preset's other set, decides the entry it states, and
/// decides neither map, which the preset's steps hold too. A synthetic
/// layer is the stack's third place: the preset, the stand-in, then it.
#[test]
fn a_synthetic_step_holding_a_map_reaches_only_what_it_states() {
    use crate::resolve::evaluate::Resolve;
    use crate::roles::ColorRoles;
    let field = |path: &str| -> Vec<String> { path.split('.').map(str::to_owned).collect() };
    let synthetic = "[colors.light.keyed]\nglow = \"#010203\"\n\
                     [colors.light.surfaces.window]\nneutral = \"#010203\"\n";
    let mut theme = resolved(&[BASE, synthetic], 1.0);
    theme.synthetic = vec![2];
    let scope = scope();
    let roles = theme.evaluate_in(&scope, None, &|_| true, Target::Roles, None, |cx| {
        let folds = cx.roots(Target::Roles);
        ColorRoles::resolve(&folds, cx);
        cx.reached().to_vec()
    });
    let reaches = |reached: &[Reached], target, path: &str| {
        reached.iter().any(|held| held.meets(target, &field(path)))
    };
    let decides = |reached: &[Reached], target, path: &str| {
        let path = field(path);
        reached
            .iter()
            .any(|held| held.meets(target, &path) && held.decides(&path))
    };
    assert!(!reaches(&roles, Target::Roles, "keyed.shade"), "{roles:?}");
    assert!(decides(&roles, Target::Roles, "keyed.glow"), "{roles:?}");
    assert!(reaches(&roles, Target::Roles, "keyed"), "{roles:?}");
    assert!(!decides(&roles, Target::Roles, "keyed"), "{roles:?}");
    assert!(
        !reaches(&roles, Target::Roles, "surfaces.button.foreground"),
        "{roles:?}"
    );
    assert!(
        !reaches(&roles, Target::Roles, "surfaces.window.foreground"),
        "{roles:?}"
    );
    assert!(
        decides(&roles, Target::Roles, "surfaces.window.neutral"),
        "{roles:?}"
    );
    assert!(reaches(&roles, Target::Roles, "surfaces"), "{roles:?}");
    assert!(!decides(&roles, Target::Roles, "surfaces"), "{roles:?}");
}

/// The label's text style, resolved over `layers` (the first the preset's).
fn label_text(layers: &[&str]) -> crate::blocks::text::ResolvedTextStyle {
    label_text_by(layers, ReaderState::new(Scheme::Light, 1.0))
}

/// [`label_text`], the stand-in reporting `state`.
fn label_text_by(layers: &[&str], state: ReaderState) -> crate::blocks::text::ResolvedTextStyle {
    let theme = resolved_by(layers, state);
    let scope = scope();
    let element = Element::new(PartRef::Typed(LABEL), &scope, &Metrics);
    theme
        .evaluate_part(LABEL, &element, &|_| true)
        .value
        .and_then(|label| label.text)
        .expect("the label has text")
}

/// `round_pt` rounds by S9.4's DPI in force: the reader's where it reports
/// one, else the preset's `token.text.font_dpi` (S2.2, S9.4; ADD-A131). A
/// body size of 14 px is 10.5 pt at the token's 96 dpi, rounded to 11 pt,
/// 14.667 px; at the reader's 120 dpi it is 8.4 pt, rounded to 8 pt,
/// 13.333 px. Synthetic values.
#[test]
fn round_pt_rounds_by_the_dpi_in_force() {
    let body = "[tokens.text.roles.body.font]\nsize_px = 14\n\
                [widgets.button.label]\n\
                text.font.size = \"= round_pt(token.text.roles.body.font.size)\"\n";
    let size = |dpi: Option<f32>| {
        let mut state = ReaderState::new(Scheme::Light, 1.0);
        state.font_dpi = dpi;
        size_of(&label_text_by(&[BASE, body], state).font)
    };
    let token = size(None);
    assert!((token - 14.0 * 11.0 / 10.5).abs() < 1e-3, "{token}");
    let reader = size(Some(120.0));
    assert!((reader - 8.0 * 120.0 / 72.0).abs() < 1e-3, "{reader}");
}

/// A `Value<Option<T>>`'s `"none"` is the option's own `None`, stated, and
/// `"absent"` the value's absence (S6.1, S11): read as `Stated(None)` and
/// `Absent`, resolved as `Some(None)` and `None`, and told apart by the
/// gate's tree (S17).
#[test]
fn value_option_none_is_not_absent() {
    use crate::blocks::BlockPatch;
    use crate::blocks::part_style::PartStylePatch;
    use crate::blocks::text::{TextRole, TextStylePatch, Wrap};
    use crate::gate::{Tolerance, Tree};
    use crate::value::Value;

    let read = |set: &str| -> TextStylePatch {
        let label = toml(&format!("[widgets.button.label.text]\n{set}"))
            .bases
            .widgets
            .button
            .and_then(|button| button.label);
        match label.map(|patch| *patch.style) {
            Some(PartStylePatch {
                text: Some(BlockPatch::Fields(text)),
                ..
            }) => text,
            other => panic!("{other:?}"),
        }
    };
    let none = read("max_lines = \"none\"\nscaling_role = \"none\"");
    assert_eq!(none.max_lines, Some(Value::Stated(None)));
    assert_eq!(none.scaling_role, Some(Value::Stated(None)));
    let absent = read("max_lines = \"absent\"\nscaling_role = \"absent\"");
    assert_eq!(absent.max_lines, Some(Value::Absent));
    assert_eq!(absent.scaling_role, Some(Value::Absent));
    let stated = read("max_lines = 2\nscaling_role = \"caption\"");
    assert_eq!(stated.max_lines, Some(Value::Stated(Some(2))));
    assert_eq!(
        stated.scaling_role,
        Some(Value::Stated(Some(TextRole::Caption)))
    );

    // BASE's body slot states both `"none"`; the label references it.
    let stated_none = label_text(&[BASE]);
    assert_eq!(stated_none.max_lines, Some(None));
    assert_eq!(stated_none.scaling_role, Some(None));
    let absent = "[tokens.text.roles.body]\nmax_lines = \"absent\"\nscaling_role = \"absent\"";
    let left_absent = label_text(&[BASE, absent]);
    assert_eq!(left_absent.max_lines, None);
    assert_eq!(left_absent.scaling_role, None);

    let tree = |value: &dyn std::fmt::Debug| Tree::of(value).expect("a derived Debug form");
    assert!(
        tree(&stated_none.max_lines)
            .differs(&tree(&left_absent.max_lines), &Tolerance::quote())
            .is_some()
    );
    assert_eq!(
        tree(&stated_none.max_lines).differs(&tree(&Some(None::<u32>)), &Tolerance::quote()),
        None
    );
    assert_eq!(
        tree(&Some(Some(2_u32))).differs(&tree(&Some(2_u32)), &Tolerance::quote()),
        None
    );
    // A unit variant named `None` reads as an option's `None` does: stated,
    // it differs from the absence.
    assert!(
        tree(&Some(Wrap::None))
            .differs(&tree(&None::<Wrap>), &Tolerance::quote())
            .is_some()
    );
}

/// `TextStyle`'s `scaling_role`, `case`, `elision`, `wrap` and `max_lines`
/// are `Value`s (S2.8): stated, or a part's leaf referencing its style's,
/// each resolves to the value it gives.
#[test]
fn text_style_value_leaves_resolve() {
    use crate::blocks::text::{ResolvedElision, TextCase, TextRole, Wrap};

    let label = text_style("widgets.button.label.text");
    let stated = label_text(&[BASE, &label]);
    assert_eq!(stated.case, Some(TextCase::Mixed));
    assert_eq!(stated.elision, Some(ResolvedElision::End));
    assert_eq!(stated.wrap, Some(Wrap::Word));
    assert_eq!(stated.max_lines, Some(None));
    assert_eq!(stated.scaling_role, Some(None));

    let slot = "[tokens.text.roles.body]\ncase = \"upper\"\nelision = \"middle\"\n\
                wrap = \"char\"\nmax_lines = 2\nscaling_role = \"caption\"";
    let references = "[widgets.button.label.text]\n\
                      case = \"= token.text.roles.body.case\"\n\
                      elision = \"= token.text.roles.body.elision\"\n\
                      wrap = \"= token.text.roles.body.wrap\"\n\
                      max_lines = \"= token.text.roles.body.max_lines\"\n\
                      scaling_role = \"= token.text.roles.body.scaling_role\"";
    let referenced = label_text(&[BASE, &label, slot, references]);
    assert_eq!(referenced.case, Some(TextCase::Upper));
    assert_eq!(referenced.elision, Some(ResolvedElision::Middle));
    assert_eq!(referenced.wrap, Some(Wrap::Char));
    assert_eq!(referenced.max_lines, Some(Some(2)));
    assert_eq!(referenced.scaling_role, Some(Some(TextRole::Caption)));

    let word_char = "[tokens.text.roles.body]\nwrap = \"word_char\"";
    let referenced = label_text(&[BASE, &label, slot, references, word_char]);
    assert_eq!(referenced.wrap, Some(Wrap::WordChar));
}

/// `(tag, AxisValue::Fixed(value))` pairs, synthetic.
fn axes(pairs: &[(&str, f32)]) -> Vec<(AxisTag, AxisValue)> {
    pairs
        .iter()
        .map(|(tag, value)| (AxisTag::new(tag).unwrap(), AxisValue::Fixed(*value)))
        .collect()
}

/// The body slot's font axes stated as `table` in a layer's bases.
fn body_axes(table: &str) -> String {
    format!("[tokens.text.roles.body.font]\naxes = {table}\n")
}

/// The label's font axes, resolved over `layers`.
fn label_axes(layers: &[&str]) -> Option<Vec<(AxisTag, AxisValue)>> {
    label_text(layers).font.axes
}

/// A `Value` of a map merges by key where both are stated (S6.1): a later
/// layer's entry replaces the earlier entry of its key and adds the others,
/// patches merged as applied; a rule's map merges too.
#[test]
fn value_map_merges_by_key() {
    use crate::blocks::Merge;
    use crate::blocks::text::FontPatch;
    use crate::value::Value;

    let first = body_axes("{ GRAD = { fixed = 1 }, opsz = \"follows_size\" }");
    let second = body_axes("{ GRAD = { fixed = 2 }, XTRA = { fixed = 3 } }");
    let opsz = (AxisTag::new("opsz").unwrap(), AxisValue::FollowsSize);
    let merged = |xtra| {
        let mut out = axes(&[("GRAD", 2.0)]);
        out.push(opsz);
        out.extend(axes(&[("XTRA", xtra)]));
        out
    };
    assert_eq!(label_axes(&[BASE, &first, &second]), Some(merged(3.0)));

    let rule = "[[rules]]\nwhen = { conditions = [\"scheme:light\"] }\ntarget = \"tokens\"\n\
                set = { text.roles.body.font.axes = { XTRA = { fixed = 4 } } }\n";
    let ruled = format!("{BASE}{rule}");
    assert_eq!(label_axes(&[&ruled, &first, &second]), Some(merged(4.0)));

    let patch = |map: Vec<_>| FontPatch {
        axes: Some(Value::Stated(map)),
        ..FontPatch::default()
    };
    let mut earlier = patch(axes(&[("GRAD", 1.0), ("XTRA", 1.0)]));
    earlier
        .merge(patch(axes(&[("XTRA", 3.0), ("YOPQ", 5.0)])))
        .unwrap();
    assert_eq!(
        earlier.axes,
        Some(Value::Stated(axes(&[
            ("GRAD", 1.0),
            ("XTRA", 3.0),
            ("YOPQ", 5.0)
        ])))
    );
    let mut value = Value::Stated(axes(&[("GRAD", 1.0)]));
    crate::blocks::apply_value_map(&mut value, Value::Stated(axes(&[("XTRA", 2.0)]))).unwrap();
    assert_eq!(value, Value::Stated(axes(&[("GRAD", 1.0), ("XTRA", 2.0)])));
}

/// `{}` in a patch on a `Value` of a map is no change (S6.1, S11): no patch
/// on the map itself removes an entry; in the preset's bases, and where
/// nothing lies below, it is the empty map, not nothing.
#[test]
fn value_map_empty_patch_is_no_change() {
    use crate::value::Value;

    let stated = body_axes("{ GRAD = { fixed = 1 } }");
    let empty = body_axes("{}");
    assert_eq!(
        label_axes(&[BASE, &stated, &empty]),
        Some(axes(&[("GRAD", 1.0)]))
    );
    assert_eq!(label_axes(&[BASE]), Some(Vec::new()));
    assert_eq!(label_axes(&[BASE, &empty]), Some(Vec::new()));

    let mut value = Value::Stated(axes(&[("GRAD", 1.0)]));
    crate::blocks::apply_value_map(&mut value, Value::Stated(Vec::new())).unwrap();
    assert_eq!(value, Value::Stated(axes(&[("GRAD", 1.0)])));
    // A stated map over `Absent` changes nothing, as a `Fields` patch there
    // does (S6.1, ADD-A126).
    let mut value = Value::Absent;
    crate::blocks::apply_value_map(&mut value, Value::Stated(axes(&[("GRAD", 1.0)]))).unwrap();
    assert_eq!(value, Value::Absent);
    let mut slot = Some(Value::Stated(axes(&[("GRAD", 1.0)])));
    crate::blocks::merge_slot_value_map(&mut slot, Some(Value::Stated(Vec::new()))).unwrap();
    assert_eq!(slot, Some(Value::Stated(axes(&[("GRAD", 1.0)]))));
}

/// A reference replaces a `Value` of a map whole, as a whole-block `Set` of
/// the text style holding it does (S6.1): no entry stated below it is kept.
#[test]
fn value_map_reference_replaces_whole() {
    use crate::blocks::MergeConflict;
    use crate::value::Value;

    let preset = format!("{BASE}{}", text_style("tokens.text.keyed.x"));
    let x = "[tokens.text.keyed.x.font]\naxes = { XTRA = { fixed = 3 } }\n";
    let stated = body_axes("{ GRAD = { fixed = 1 } }");
    let referenced = body_axes("\"= token.text.keyed.x.font.axes\"");
    assert_eq!(
        label_axes(&[&preset, x, &stated, &referenced]),
        Some(axes(&[("XTRA", 3.0)]))
    );
    let whole = text_style("tokens.text.roles.body");
    assert_eq!(label_axes(&[&preset, &stated, &whole]), Some(Vec::new()));

    let reference = Value::Expr(Expr::parse("token.text.keyed.x.font.axes").unwrap());
    let mut value = Value::Stated(axes(&[("GRAD", 1.0)]));
    crate::blocks::apply_value_map(&mut value, reference.clone()).unwrap();
    assert_eq!(value, reference);
    // A stated map over the reference is kept on it, composed lazily.
    assert_eq!(
        crate::blocks::apply_value_map(&mut value, Value::Stated(axes(&[("GRAD", 1.0)]))),
        Err(MergeConflict::new(crate::blocks::Below::Reference))
    );
}

/// A stated map over a reference composes lazily (S2.2, S6.1): the
/// referenced map's own bases first, a later layer's included, then the
/// stated map, before every rule; a rule on the referenced map still
/// reaches the slot.
#[test]
fn value_map_over_reference_composes_lazily() {
    let preset = format!("{BASE}{}", text_style("tokens.text.keyed.x"));
    let referenced = body_axes("\"= token.text.keyed.x.font.axes\"");
    let stated = body_axes("{ GRAD = { fixed = 1 }, XTRA = { fixed = 1 } }");
    let later =
        "[tokens.text.keyed.x.font]\naxes = { XTRA = { fixed = 3 }, YOPQ = { fixed = 5 } }\n";
    assert_eq!(
        label_axes(&[&preset, &referenced, &stated, later]),
        Some(axes(&[("XTRA", 1.0), ("YOPQ", 5.0), ("GRAD", 1.0)]))
    );
    let rule = "[[rules]]\nwhen = { conditions = [\"scheme:light\"] }\ntarget = \"tokens\"\n\
                set = { text.keyed.x.font.axes = { XTRA = { fixed = 7 } } }\n";
    let ruled = format!("{preset}{rule}");
    assert_eq!(
        label_axes(&[&ruled, &referenced, &stated, later]),
        Some(axes(&[("XTRA", 7.0), ("YOPQ", 5.0), ("GRAD", 1.0)]))
    );
}

/// The T6 review's M3: the patches stated in rules over a reference each
/// apply at its rule's position (S2.2): a part rule on the label's axes and
/// a `Tokens` rule on the referenced style's, stating one axis, the later
/// rule's value is the label's, in either order.
#[test]
fn value_map_rules_over_reference_by_position() {
    let preset = format!("{BASE}{}", text_style("tokens.text.keyed.x"));
    let referenced = body_axes("\"= token.text.keyed.x.font.axes\"");
    let part = "[[rules]]\ntarget = \"button.label\"\n\
                set = { text.font.axes = { GRAD = { fixed = 2 } } }\n";
    let tokens = "[[rules]]\ntarget = \"tokens\"\n\
                  set = { text.keyed.x.font.axes = { GRAD = { fixed = 7 } } }\n";
    assert_eq!(
        label_axes(&[&preset, &referenced, &format!("{tokens}{part}")]),
        Some(axes(&[("GRAD", 2.0)]))
    );
    assert_eq!(
        label_axes(&[&preset, &referenced, &format!("{part}{tokens}")]),
        Some(axes(&[("GRAD", 7.0)]))
    );
}

/// A `Value` struct block nothing states resolves `None`, as a `Value`
/// nothing states does (S6.1), not a block of `None` fields: a drawn part
/// whose table leaves its `text` out has no text. `Cx::value_block` gives
/// it; `Cx::block` over no step would build a `TextStyle` of `None` leaves,
/// every one of its leaves being a `Value` or a block of them.
#[test]
fn value_block_nothing_states_is_none() {
    use crate::blocks::text::TextStyle;

    const SUBTITLE: TypedPart = TypedPart::Button(ButtonPart::Subtitle);
    let preset = format!("{BASE}[widgets.button.subtitle]\n");
    let theme = resolved(&[&preset], 1.0);
    let scope = scope();
    let element = Element::new(PartRef::Typed(SUBTITLE), &scope, &Metrics);
    let subtitle = theme.evaluate_part(SUBTITLE, &element, &|_| true).value;
    assert_eq!(subtitle.map(|style| style.text), Some(None));

    let (value_block, block) = with_body(&theme, &scope, |cx| {
        let path = vec!["text".to_owned()];
        cx.follow(Target::Part(SUBTITLE), path, false, |cx, folds| {
            assert!(folds.is_empty(), "nothing states the subtitle's text");
            Some((
                cx.value_block::<TextStyle>(folds),
                cx.block::<TextStyle>(folds),
            ))
        })
        .unwrap()
    });
    assert_eq!(value_block, None);
    assert!(
        block.is_some(),
        "a block of `None` leaves, which no `Value` resolves to"
    );
}

/// A keyed style `y` the label's text references under the text scaling
/// `kind` at 72 DPI, `fields` its patch's TOML lines (its font size, role,
/// line height and letter spacing).
fn scaled_style(kind: &str, fields: &str) -> [String; 2] {
    [
        format!(
            "{}[tokens.text]\nscaling = \"{kind}\"\nfont_dpi = 72\n\
             [widgets.button.label]\ntext = \"= token.text.keyed.y\"\n",
            text_style("tokens.text.keyed.y")
        ),
        format!("[tokens.text.keyed.y]\n{fields}"),
    ]
}

/// The label's text under the stand-in `factor` and `category`.
fn scaled_text(
    style: &[String; 2],
    factor: f32,
    category: Option<crate::resolve::ContentSizeCategory>,
) -> crate::blocks::text::ResolvedTextStyle {
    let mut state = ReaderState::new(Scheme::Light, factor);
    state.content_size_category = category;
    label_text_by(&[BASE, &style[0], &style[1]], state)
}

/// S9.4's `AndroidNonLinear`: a font size and a spacing go through
/// Android's curve, each as a value of its own; a line height in px above
/// Compose's threshold (1.05) keeps its proportion to its font's change,
/// at or below it goes through the curve itself. Synthetic sizes.
#[test]
fn android_non_linear_scales_by_the_curve() {
    use crate::platform::android_non_linear::{android_non_linear, android_non_linear_inverse};
    use crate::units::Length;

    let style = scaled_style(
        "android_non_linear",
        "font.size_px = 14\nline_height_px = 20\nletter_spacing_px = -0.5",
    );
    let text = scaled_text(&style, 1.3, None);
    let size = android_non_linear(14.0, 1.3);
    assert_eq!(text.font.size, Some(Px(size)));
    // 1.3's table maps 14 sp to 18.8 dp: large sizes scale less.
    assert_eq!(size, 18.8);
    assert_eq!(
        text.line_height,
        Some(Length::Px(
            20.0 / android_non_linear_inverse(size, 1.3) * size
        ))
    );
    assert_eq!(text.letter_spacing, Some(Px(android_non_linear(-0.5, 1.3))));

    let text = scaled_text(&style, 1.04, None);
    assert_eq!(text.font.size, Some(Px(android_non_linear(14.0, 1.04))));
    assert_eq!(
        text.line_height,
        Some(Length::Px(android_non_linear(20.0, 1.04)))
    );

    // Below the curves a size scales by the factor, at one not at all.
    let text = scaled_text(&style, 1.0, None);
    assert_eq!(text.font.size, Some(Px(14.0)));
    assert_eq!(text.line_height, Some(Length::Px(20.0)));
}

/// S9.4: under `AndroidNonLinear` a font size stated over another style's
/// reads it unscaled and goes through the curve once, as its own size; a
/// font size read as an operand is its style's, through the curve.
#[test]
fn android_non_linear_scales_derived_sizes_once() {
    use crate::platform::android_non_linear::android_non_linear;

    let style = scaled_style(
        "android_non_linear",
        "font.size = \"= mul(token.text.roles.body.font.size, 2)\"",
    );
    let text = scaled_text(&style, 1.5, None);
    // The body slot's 9pt at the fixture's 72 DPI, doubled, then the curve.
    assert_eq!(text.font.size, Some(Px(android_non_linear(18.0, 1.5))));

    let theme = resolved(&[BASE, &style[0], &style[1]], 1.5);
    assert_eq!(
        operand(&theme, "token.text.keyed.y.font.size"),
        Some(Val::FontSize(Px(android_non_linear(18.0, 1.5))))
    );
}

/// S9.4's `WinUi`: the font size alone goes through WinUI's curve; a line
/// height and a spacing stated in px hold, and a spacing stated on
/// `font_size` follows the scaled size. Synthetic sizes.
#[test]
fn win_ui_scales_the_font_size_alone() {
    use crate::platform::win_ui::win_ui;
    use crate::units::Length;

    let style = scaled_style(
        "win_ui",
        "font.size_px = 10\nline_height_px = 20\nletter_spacing_px = -0.5\n\
         word_spacing = \"= mul(font_size, 0.5)\"",
    );
    let text = scaled_text(&style, 1.5, None);
    let size = win_ui(10.0, 1.5);
    assert_eq!(text.font.size, Some(Px(size)));
    // Small sizes grow more than the factor: 10 px is near 15.87 at 150 %.
    assert!(size > 15.8);
    assert_eq!(text.line_height, Some(Length::Px(20.0)));
    assert_eq!(text.letter_spacing, Some(Px(-0.5)));
    assert_eq!(text.word_spacing, Some(Px(size * 0.5)));

    let text = scaled_text(&style, 1.0, None);
    assert_eq!(text.font.size, Some(Px(10.0)));
}

/// S9.4: under `WinUi` a font size stated over another style's reads it
/// unscaled and goes through the curve once; a font size read as an
/// operand is its style's, through the curve; a point is not scaled.
#[test]
fn win_ui_scales_derived_sizes_once() {
    use crate::platform::win_ui::win_ui;

    let style = scaled_style(
        "win_ui",
        "font.size = \"= mul(token.text.roles.body.font.size, 2)\"",
    );
    let text = scaled_text(&style, 1.5, None);
    // The body slot's 9pt at the fixture's 72 DPI, doubled, then the curve.
    assert_eq!(text.font.size, Some(Px(win_ui(18.0, 1.5))));

    let theme = resolved(&[BASE, &style[0], &style[1]], 1.5);
    assert_eq!(
        operand(&theme, "token.text.keyed.y.font.size"),
        Some(Val::FontSize(Px(win_ui(18.0, 1.5))))
    );
    assert_eq!(theme.px_per_point(&scope()), Some(1.0));
}

/// S9.4's `DynamicType`: at the default category the preset's sizes hold;
/// at another, a size scales by its `scaling_role`'s row of Apple's table
/// (`body`'s where it states none), the row's own leading becomes the
/// category's, and the factor is not read.
#[test]
fn dynamic_type_scales_by_the_table() {
    use crate::platform::dynamic_type::dynamic_type;
    use crate::resolve::ContentSizeCategory;
    use crate::units::Length;

    let xl = Some(ContentSizeCategory::ExtraLarge);
    // The table's body row at the default category, 17 pt on 22 pt.
    let style = scaled_style(
        "dynamic_type",
        "font.size_px = 17\nline_height_px = 22\nscaling_role = \"body\"",
    );
    let text = scaled_text(&style, 2.0, None);
    assert_eq!(text.font.size, Some(Px(17.0)));
    assert_eq!(text.line_height, Some(Length::Px(22.0)));
    let row = dynamic_type(
        crate::blocks::text::TextRole::Body,
        ContentSizeCategory::ExtraLarge,
        72.0,
    )
    .expect("the body row");
    let text = scaled_text(&style, 2.0, xl);
    assert_eq!(text.font.size, Some(Px(row.font_size)));
    assert_eq!(text.line_height, Some(Length::Px(row.line_height)));
    // No role: `body`'s row.
    let none = scaled_style("dynamic_type", "font.size_px = 17\nline_height_px = 22");
    assert_eq!(
        scaled_text(&none, 1.0, xl).font.size,
        Some(Px(row.font_size))
    );

    // Another size follows the row's ratio; a line height other than the
    // row's has no rule yet, nor has a spacing in px; one on `font_size`
    // follows the scaled font.
    let custom = scaled_style(
        "dynamic_type",
        "font.size_px = 34\nline_height_px = 23\nletter_spacing_px = 1\nscaling_role = \"body\"",
    );
    let text = scaled_text(&custom, 1.0, xl);
    assert_eq!(text.font.size, Some(Px(34.0 * row.font_size / 17.0)));
    assert_eq!(text.line_height, None);
    assert_eq!(text.letter_spacing, None);
    let following = scaled_style(
        "dynamic_type",
        "font.size_px = 17\nscaling_role = \"body\"\n\
         line_height = \"= mul(font_size, 1.5)\"\nletter_spacing = \"= mul(font_size, 0.5)\"",
    );
    let text = scaled_text(&following, 1.0, xl);
    assert_eq!(text.line_height, Some(Length::Px(row.font_size * 1.5)));
    assert_eq!(text.letter_spacing, Some(Px(row.font_size * 0.5)));

    // A role without a row scales nowhere but at the default category.
    let display = scaled_style(
        "dynamic_type",
        "font.size_px = 17\nline_height_px = 22\nscaling_role = \"display_large\"",
    );
    assert_eq!(scaled_text(&display, 1.0, xl).font.size, None);
    assert_eq!(scaled_text(&display, 1.0, None).font.size, Some(Px(17.0)));
}

/// S9.4: under `DynamicType` a size stated over another style's is scaled
/// once, by its own style's row; read as an operand, a style's size and line
/// height follow their style's row.
#[test]
fn dynamic_type_scales_by_the_own_row() {
    use crate::blocks::text::TextRole;
    use crate::platform::dynamic_type::dynamic_type;
    use crate::resolve::ContentSizeCategory;

    let category = ContentSizeCategory::AccessibilityMedium;
    let row = |role| dynamic_type(role, category, 72.0).expect("a row");
    let default = |role| dynamic_type(role, ContentSizeCategory::Large, 72.0).expect("a row");
    let style = scaled_style(
        "dynamic_type",
        "font.size = \"= add(token.text.roles.body.font.size, 19px)\"\n\
         line_height_px = 34\nscaling_role = \"title1\"",
    );
    let mut state = ReaderState::new(Scheme::Light, 1.0);
    state.content_size_category = Some(category);
    let theme = resolved_by(&[BASE, &style[0], &style[1]], state);
    let size = 28.0 * row(TextRole::Title1).font_size / default(TextRole::Title1).font_size;
    // The body slot's 9pt and 19px: 28px, title 1's size at the default.
    assert_eq!(
        operand(&theme, "token.text.keyed.y.font.size"),
        Some(Val::FontSize(Px(size)))
    );
    assert_eq!(
        operand(&theme, "token.text.keyed.y.line_height"),
        Some(Val::FontSize(Px(row(TextRole::Title1).line_height)))
    );
}
