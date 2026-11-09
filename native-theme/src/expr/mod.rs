//! Expressions (S2.2): a derivation a field states instead of a value,
//! `"= <expression>"` in TOML.
//!
//! [`Expr::parse`] reads the grammar; inside the crate, `Expr::check` types
//! an expression against the field it is stated on when a layer is added, and
//! `TypedExpr::eval` evaluates it where it is used, through an `Env` that
//! answers its references. Evaluation is total: it clamps, floors and zeroes
//! as S2.2's *Totality* says, and typing refuses a call of a platform function
//! not implemented yet, so it gives a value for every typed expression.

#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "typing and evaluation are called by S12's checks and S8's query (M1a tasks 8 to 11); an `allow`, as Rust 1.88 (the MSRV) counts what dead code under an `expect` uses as used, and stable does not"
    )
)]
mod arith;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "typing and evaluation are called by S12's checks and S8's query (M1a tasks 8 to 11)"
    )
)]
mod eval;
mod parse;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "typing and evaluation are called by S12's checks and S8's query (M1a tasks 8 to 11)"
    )
)]
mod ty;

use std::fmt;
use std::sync::LazyLock;

use super::color::Color;
use super::inputs::{KdeSchemeKey, KdeWmKey};
use super::key::Key;
use super::roles::SurfaceSet;
use super::tokens::KdeGroup;
use super::units::{Deg, Ms, Px};
use crate::key;

pub(crate) use eval::{Env, Val, bounded};
pub use parse::{ParseError, ParseErrorKind};
pub(crate) use ty::{Bound, EnumTy, FieldType, SURFACE_SET, Ty, TypeEnv};

/// How deep an expression may nest: calls and `active(…)` inside each other
/// (S2.2, S12).
pub const MAX_DEPTH: usize = 32;

/// How long a chain of references may be (S2.2, S12).
pub(crate) const MAX_CHAIN: usize = 32;

/// A parsed expression: a literal, a reference or a call.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A literal.
    Lit(Literal),
    /// A reference to a role, token, input, field or the query's own operands.
    Ref(Ref),
    /// A call of one of the closed set of functions, with its arguments.
    Call(Func, Vec<Expr>),
}

/// A literal.
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// A colour in one of S2.1's forms written bare: `#rrggbb`, `#rrggbbaa`,
    /// `srgb(r, g, b[, a])`, `display_p3(r, g, b[, a])`.
    Color(Color),
    /// A plain number.
    Number(f32),
    /// `<n>px`: a length, or a font size where the field or parameter is one.
    Px(Px),
    /// `<n>ms`: a duration.
    Ms(Ms),
    /// `<n>deg`: an angle.
    Deg(Deg),
    /// `<n>pt`: a font size in points.
    Pt(f32),
    /// A name: an enum variant or a keyed name.
    Name(Name),
    /// `true` or `false`.
    Bool(bool),
}

/// A name operand.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Name {
    /// An enum variant in snake_case (`inactive`).
    Variant(Key),
    /// A keyed name, `@` and a key (`@primary`).
    Key(Key),
}

/// A reference (S2.2's `ref`); a path is its segments, each a [`Key`].
#[derive(Clone, Debug, PartialEq)]
pub enum Ref {
    /// `role.<set>.<path>`: a typed colour role of a surface set (S3).
    Role {
        /// The surface set.
        set: SurfaceSet,
        /// The role's path in the set (`disabled.foreground`).
        path: Vec<Key>,
    },
    /// `role.global.<path>`: a typed global role (S3).
    GlobalRole(Vec<Key>),
    /// `role.@<key>`: a keyed platform role (S3).
    KeyedRole(Key),
    /// `surface.<role>`: a role of the surface set bound in scope (S3.3).
    Surface(Vec<Key>),
    /// `active(<ref>)`: a role's active value, from the bases of every layer,
    /// before any rule (R5.1).
    Active(Box<Ref>),
    /// `token.<path>`: a token (S4).
    Token(Vec<Key>),
    /// `input.<path>`: an input (S9).
    Input(Vec<Key>),
    /// `base`: the value earlier rules gave in this query (R2.5).
    Base,
    /// `scheme`: the scheme of the colours being resolved (R5.1).
    Scheme,
    /// `font_size`: the part's, in logical px after the point conversion and
    /// text scaling (S9.4).
    FontSize,
    /// `line_height`: the part's, in logical px; under `line_height = auto`
    /// the font's own line spacing.
    LineHeight,
    /// `element.<data>`: the element's data or its own font's metric (S8).
    Element(ElementDatum),
    /// `body_font_height`: the `body` slot's font's height, ascent plus
    /// descent.
    BodyFontHeight,
    /// `part.<path>`: another resolved field of the same part.
    Part(Vec<Key>),
    /// `widget.<part>.<path>`: a resolved field of another part of the same
    /// widget, or that part's font metric.
    Widget {
        /// The part.
        part: Key,
        /// The field's path in that part (`corners`, `char_width`).
        path: Vec<Key>,
    },
    /// `values.<name>`: the element's widget's resolved value.
    Values(Key),
    /// `text_scale`: the text-scaling factor (S9.4).
    TextScale,
}

/// An `element.` operand (S2.2, S8): the element's data, the root's inputs and
/// the part's own font's metrics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElementDatum {
    /// `width`: the laid-out box's width.
    Width,
    /// `height`: the laid-out box's height.
    Height,
    /// `progress`.
    Progress,
    /// `scroll_offset`.
    ScrollOffset,
    /// `scroll_progress`.
    ScrollProgress,
    /// `item_index`.
    ItemIndex,
    /// `item_count`.
    ItemCount,
    /// `text`.
    Text,
    /// `color`: a colour the connector supplies.
    Color,
    /// `screen_width`, from the root's inputs.
    ScreenWidth,
    /// `screen_height`, from the root's inputs.
    ScreenHeight,
    /// `window_width`, from the root's inputs.
    WindowWidth,
    /// `window_height`, from the root's inputs.
    WindowHeight,
    /// `move_distance`: how far a `Move` presentation moves the element.
    MoveDistance,
    /// `caret_height`: a caret part's own caret, its toolkit's caret
    /// rectangle's height as drawn (S4.2).
    CaretHeight,
    /// `char_width`: the part's font's average character width.
    CharWidth,
    /// `max_char_width`: the part's font's widest glyph.
    MaxCharWidth,
    /// `font_x_height`.
    FontXHeight,
    /// `font_height`: ascent plus descent.
    FontHeight,
    /// `font_underline_offset`.
    FontUnderlineOffset,
    /// `font_underline_thickness`.
    FontUnderlineThickness,
    /// `font_strikeout_offset`.
    FontStrikeoutOffset,
    /// `font_strikeout_thickness`.
    FontStrikeoutThickness,
}

impl ElementDatum {
    /// Every datum, in S2.2's order.
    pub const ALL: [ElementDatum; 23] = [
        ElementDatum::Width,
        ElementDatum::Height,
        ElementDatum::Progress,
        ElementDatum::ScrollOffset,
        ElementDatum::ScrollProgress,
        ElementDatum::ItemIndex,
        ElementDatum::ItemCount,
        ElementDatum::Text,
        ElementDatum::Color,
        ElementDatum::ScreenWidth,
        ElementDatum::ScreenHeight,
        ElementDatum::WindowWidth,
        ElementDatum::WindowHeight,
        ElementDatum::MoveDistance,
        ElementDatum::CaretHeight,
        ElementDatum::CharWidth,
        ElementDatum::MaxCharWidth,
        ElementDatum::FontXHeight,
        ElementDatum::FontHeight,
        ElementDatum::FontUnderlineOffset,
        ElementDatum::FontUnderlineThickness,
        ElementDatum::FontStrikeoutOffset,
        ElementDatum::FontStrikeoutThickness,
    ];

    /// The name after `element.`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            ElementDatum::Width => "width",
            ElementDatum::Height => "height",
            ElementDatum::Progress => "progress",
            ElementDatum::ScrollOffset => "scroll_offset",
            ElementDatum::ScrollProgress => "scroll_progress",
            ElementDatum::ItemIndex => "item_index",
            ElementDatum::ItemCount => "item_count",
            ElementDatum::Text => "text",
            ElementDatum::Color => "color",
            ElementDatum::ScreenWidth => "screen_width",
            ElementDatum::ScreenHeight => "screen_height",
            ElementDatum::WindowWidth => "window_width",
            ElementDatum::WindowHeight => "window_height",
            ElementDatum::MoveDistance => "move_distance",
            ElementDatum::CaretHeight => "caret_height",
            ElementDatum::CharWidth => "char_width",
            ElementDatum::MaxCharWidth => "max_char_width",
            ElementDatum::FontXHeight => "font_x_height",
            ElementDatum::FontHeight => "font_height",
            ElementDatum::FontUnderlineOffset => "font_underline_offset",
            ElementDatum::FontUnderlineThickness => "font_underline_thickness",
            ElementDatum::FontStrikeoutOffset => "font_strikeout_offset",
            ElementDatum::FontStrikeoutThickness => "font_strikeout_thickness",
        }
    }

    /// The datum `name` names.
    #[must_use]
    pub fn from_name(name: &str) -> Option<ElementDatum> {
        ElementDatum::ALL
            .into_iter()
            .find(|datum| datum.name() == name)
    }
}

/// A function of S2.2's closed set: the model's own arithmetic (`alpha`,
/// `with_alpha`, `mix`, `mul`, `div`, `add`, `sub`, `min`, `max`, `floor`,
/// `round`, `pick`) and the platform functions, each implemented to its
/// source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Func {
    /// `alpha(c, a)`: `c` with its alpha multiplied by `a`.
    Alpha,
    /// `with_alpha(c, a)`: `c` with its alpha set to `a`, as QML's
    /// `Qt.alpha` and CSS's relative colour `rgb(from c r g b / a)` set it.
    WithAlpha,
    /// `mix(a, b, t)`: CSS `color-mix(in srgb, a (1−t)·100%, b t·100%)`.
    Mix,
    /// `kde_effects(c, background, group, role_kind)`: KColorScheme's effects.
    KdeEffects,
    /// `kde_selection_inactive(selection, window, selection_background, role_kind)`.
    KdeSelectionInactive,
    /// `kcu_mix(a, b, bias)`: `KColorUtils::mix`.
    KcuMix,
    /// `kcu_tint(a, b, amount)`: `KColorUtils::tint`.
    KcuTint,
    /// `kcu_shade(c, luma, chroma)`: `KColorUtils::shade`.
    KcuShade,
    /// `kde_shade(c, role, contrast)`: KColorScheme's `shade`, by
    /// kdeglobals' contrast step.
    KdeShade,
    /// `adw_standalone(c, scheme)`: libadwaita's standalone accent.
    AdwStandalone,
    /// `m3_role(role)`: material-color-utilities' dynamic scheme role.
    M3Role,
    /// `mul(x, y)`.
    Mul,
    /// `div(x, y)`.
    Div,
    /// `add(x, y)`.
    Add,
    /// `sub(x, y)`.
    Sub,
    /// `min(x, y)`.
    Min,
    /// `max(x, y)`.
    Max,
    /// `floor(x)`: toward negative infinity.
    Floor,
    /// `round(x)`: to the nearest integer, a half away from zero.
    Round,
    /// `round_pt(x)`: a font size rounded to whole points by the font DPI
    /// (S9.4), as `QFont::pointSize()` rounds the application font's.
    RoundPt,
    /// `qt_lighter(c, factor)`: `QColor::lighter`.
    QtLighter,
    /// `qt_darker(c, factor)`: `QColor::darker`.
    QtDarker,
    /// `kcu_overlay(under, over)`: `KColorUtils::overlayColors`.
    KcuOverlay,
    /// `kirigami_tint(c, tint, alpha)`: Kirigami's `ColorUtils.tintWithAlpha`.
    KirigamiTint,
    /// `kirigami_lerp(a, b, balance)`: Kirigami's
    /// `ColorUtils.linearInterpolation`, in HSV.
    KirigamiLerp,
    /// `kirigami_label_icon_size(font_height)`: Kirigami's `units.cpp`.
    KirigamiLabelIconSize,
    /// `kirigami_badge_text(text, background)`: Kirigami's `Badge.qml`.
    KirigamiBadgeText,
    /// `ka_card_border(background, text)`: kirigami-addons' FormCard.
    KaCardBorder,
    /// `ka_banner_line(type_text, frame)`: kirigami-addons' banner.
    KaBannerLine,
    /// `ka_contrast_text(c)`: kirigami-addons' avatar.
    KaContrastText,
    /// `kqc_series_color(seed, count, index)`: KQuickCharts.
    KqcSeriesColor,
    /// `kte_minimap_mark(mark, background)`: KTextEditor.
    KteMinimapMark,
    /// `kte_mark_underlay(hue_from, background, foreground)`: KTextEditor.
    KteMarkUnderlay,
    /// `sf_tracking(font_size, design)`: Apple's tracking tables.
    SfTracking,
    /// `uikit_prominence(c, prominence)`: `UIColor.withProminence(_:)`.
    UikitProminence,
    /// `gtk_contrast_pick(c, on_light, on_dark)`: `gtkcolorswatch.c`.
    GtkContrastPick,
    /// `shadow_extent(shadow, side)`: `gtk_css_shadow_value_get_extents`.
    ShadowExtent,
    /// `gtk_wheel_step(page_size)`: GTK's scroll step.
    GtkWheelStep,
    /// `uifm_scaled(x, text_role)`: `UIFontMetrics.scaledValue(for:)`.
    UifmScaled,
    /// `kde_accent(set, key, group)`: Plasma's `applyScheme()`.
    KdeAccent,
    /// `kde_accent_wm(key)`: `applyScheme()`'s `[WM]` group.
    KdeAccentWm,
    /// `name_index(text, count, method)`: an avatar's name hash.
    NameIndex,
    /// `pick(index, c0, c1, …)`: the colour at `index`.
    Pick,
}

impl Func {
    /// Every function, in S2.2's table's order.
    pub const ALL: [Func; 43] = [
        Func::Alpha,
        Func::WithAlpha,
        Func::Mix,
        Func::KdeEffects,
        Func::KdeSelectionInactive,
        Func::KcuMix,
        Func::KcuTint,
        Func::KcuShade,
        Func::KdeShade,
        Func::AdwStandalone,
        Func::M3Role,
        Func::Mul,
        Func::Div,
        Func::Add,
        Func::Sub,
        Func::Min,
        Func::Max,
        Func::Floor,
        Func::Round,
        Func::RoundPt,
        Func::QtLighter,
        Func::QtDarker,
        Func::KcuOverlay,
        Func::KirigamiTint,
        Func::KirigamiLerp,
        Func::KirigamiLabelIconSize,
        Func::KirigamiBadgeText,
        Func::KaCardBorder,
        Func::KaBannerLine,
        Func::KaContrastText,
        Func::KqcSeriesColor,
        Func::KteMinimapMark,
        Func::KteMarkUnderlay,
        Func::SfTracking,
        Func::UikitProminence,
        Func::GtkContrastPick,
        Func::ShadowExtent,
        Func::GtkWheelStep,
        Func::UifmScaled,
        Func::KdeAccent,
        Func::KdeAccentWm,
        Func::NameIndex,
        Func::Pick,
    ];

    /// The function's name in expressions.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Func::Alpha => "alpha",
            Func::WithAlpha => "with_alpha",
            Func::Mix => "mix",
            Func::KdeEffects => "kde_effects",
            Func::KdeSelectionInactive => "kde_selection_inactive",
            Func::KcuMix => "kcu_mix",
            Func::KcuTint => "kcu_tint",
            Func::KcuShade => "kcu_shade",
            Func::KdeShade => "kde_shade",
            Func::AdwStandalone => "adw_standalone",
            Func::M3Role => "m3_role",
            Func::Mul => "mul",
            Func::Div => "div",
            Func::Add => "add",
            Func::Sub => "sub",
            Func::Min => "min",
            Func::Max => "max",
            Func::Floor => "floor",
            Func::Round => "round",
            Func::RoundPt => "round_pt",
            Func::QtLighter => "qt_lighter",
            Func::QtDarker => "qt_darker",
            Func::KcuOverlay => "kcu_overlay",
            Func::KirigamiTint => "kirigami_tint",
            Func::KirigamiLerp => "kirigami_lerp",
            Func::KirigamiLabelIconSize => "kirigami_label_icon_size",
            Func::KirigamiBadgeText => "kirigami_badge_text",
            Func::KaCardBorder => "ka_card_border",
            Func::KaBannerLine => "ka_banner_line",
            Func::KaContrastText => "ka_contrast_text",
            Func::KqcSeriesColor => "kqc_series_color",
            Func::KteMinimapMark => "kte_minimap_mark",
            Func::KteMarkUnderlay => "kte_mark_underlay",
            Func::SfTracking => "sf_tracking",
            Func::UikitProminence => "uikit_prominence",
            Func::GtkContrastPick => "gtk_contrast_pick",
            Func::ShadowExtent => "shadow_extent",
            Func::GtkWheelStep => "gtk_wheel_step",
            Func::UifmScaled => "uifm_scaled",
            Func::KdeAccent => "kde_accent",
            Func::KdeAccentWm => "kde_accent_wm",
            Func::NameIndex => "name_index",
            Func::Pick => "pick",
        }
    }

    /// The function `name` names.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Func> {
        Func::ALL.into_iter().find(|func| func.name() == name)
    }

    /// The tokens, inputs and scheme a call of the function with the
    /// arguments `args` reads itself, listed as its operands (S2.2):
    /// `kde_effects`' group's ([`kde_group_tokens`]; both groups' where its
    /// `group` is no literal), `kde_selection_inactive`'s
    /// `token.kde.inactive_change_selection_color`, `m3_role`'s
    /// ([`m3_role_operands`]), `kde_accent`'s for its set and key
    /// ([`kde_accent_operands`]; every set's and key's where either is no
    /// literal, none for a set outside KDE's seven) and `kde_accent_wm`'s
    /// for its key ([`kde_accent_wm_operands`]; every key's where it is no
    /// literal); none for any other.
    pub(crate) fn token_operands(self, args: &[Expr]) -> &'static [Ref] {
        let groups = KDE_EFFECT_FIELDS.len().saturating_mul(2).saturating_add(1);
        match self {
            Func::KdeEffects => match args.get(2) {
                Some(Expr::Lit(Literal::Name(Name::Variant(name)))) => {
                    <KdeGroup as crate::toml::read::Named>::from_name(name.as_str())
                        .map_or(&[], kde_group_tokens)
                }
                _ => kde_tokens(0, groups),
            },
            Func::KdeSelectionInactive => kde_selection_tokens(),
            Func::M3Role => m3_role_operands(),
            Func::KdeAccent => match (variant(args.first()), variant(args.get(1))) {
                (Some(set), Some(key)) => SurfaceSet::from_name(set)
                    .zip(scheme_key(key))
                    .map_or(&[], |(set, key)| kde_accent_operands(set, key)),
                _ => KDE_ACCENT_OPERANDS.every.as_slice(),
            },
            Func::KdeAccentWm => match variant(args.first()) {
                Some(key) => wm_key(key).map_or(&[], kde_accent_wm_operands),
                None => KDE_ACCENT_WM_OPERANDS.every.as_slice(),
            },
            _ => &[],
        }
    }
}

/// A literal variant argument's name.
fn variant(arg: Option<&Expr>) -> Option<&str> {
    match arg {
        Some(Expr::Lit(Literal::Name(Name::Variant(name)))) => Some(name.as_str()),
        _ => None,
    }
}

/// The scheme key `name` names.
pub(crate) fn scheme_key(name: &str) -> Option<KdeSchemeKey> {
    KdeSchemeKey::ALL.into_iter().find(|key| key.name() == name)
}

/// The `[WM]` key `name` names.
pub(crate) fn wm_key(name: &str) -> Option<KdeWmKey> {
    KdeWmKey::ALL.into_iter().find(|key| key.name() == name)
}

/// KDE's seven colour sets, `kde_accent`'s `set` (S2.2).
pub(crate) const KDE_ACCENT_SETS: [SurfaceSet; 7] = [
    SurfaceSet::Window,
    SurfaceSet::View,
    SurfaceSet::Button,
    SurfaceSet::Selection,
    SurfaceSet::Tooltip,
    SurfaceSet::Complementary,
    SurfaceSet::Header,
];

/// The operands of a KDE accent function: each call's, and every call's
/// together, each once.
struct AccentOperands {
    calls: Vec<Vec<Ref>>,
    every: Vec<Ref>,
}

impl AccentOperands {
    fn of(calls: Vec<Vec<Ref>>) -> AccentOperands {
        let mut every: Vec<Ref> = Vec::new();
        for reference in calls.iter().flatten() {
            if !every.contains(reference) {
                every.push(reference.clone());
            }
        }
        AccentOperands { calls, every }
    }
}

/// `input.kde_scheme.<path>`.
fn scheme_input(path: &[&'static str]) -> Ref {
    let mut segments = vec![key!("kde_scheme")];
    segments.extend(path.iter().copied().filter_map(Key::from_static));
    Ref::Input(segments)
}

/// What both KDE accent functions read besides a key's scheme inputs
/// (S2.2), in this order: the accent `input.accent` first, then the lists
/// `input.kde_scheme.groups`, `.inactive_groups` (`kde_accent`'s alone) and
/// `.unset_keys`, `token.kde.tint_factor` and
/// `token.kde.titlebar_is_accent_colored` last.
fn accent_operands(inputs: Vec<Ref>, inactive_groups: bool) -> Vec<Ref> {
    let kde = |field: Key| Ref::Token(vec![key!("kde"), field]);
    let mut out = vec![Ref::Input(vec![key!("accent")])];
    out.extend(inputs);
    out.push(scheme_input(&["groups"]));
    if inactive_groups {
        out.push(scheme_input(&["inactive_groups"]));
    }
    out.push(scheme_input(&["unset_keys"]));
    out.push(kde(key!("tint_factor")));
    out.push(kde(key!("titlebar_is_accent_colored")));
    out
}

/// What `kde_accent` reads itself, per set of [`KDE_ACCENT_SETS`] and key
/// of [`KdeSchemeKey::ALL`] in their order.
static KDE_ACCENT_OPERANDS: LazyLock<AccentOperands> = LazyLock::new(|| {
    let mut calls = Vec::new();
    for set in KDE_ACCENT_SETS {
        for key in KdeSchemeKey::ALL {
            let (set, key) = (set.name(), key.name());
            let inputs = vec![
                scheme_input(&[set, key]),
                scheme_input(&[set, "inactive", key]),
                scheme_input(&[set, "background_normal"]),
                scheme_input(&["window", key]),
                scheme_input(&["window", "background_normal"]),
                scheme_input(&["view", "background_normal"]),
            ];
            calls.push(accent_operands(inputs, true));
        }
    }
    AccentOperands::of(calls)
});

/// What `kde_accent(set, key, …)` reads itself (S2.2), its group reading
/// the same, in this order: `input.accent`; the scheme inputs
/// `input.kde_scheme.<set>.<key>`, `.<set>.inactive.<key>`,
/// `.<set>.background_normal`, `.window.<key>`, `.window.background_normal`
/// and `.view.background_normal`; the lists `input.kde_scheme.groups`,
/// `.inactive_groups` and `.unset_keys`; `token.kde.tint_factor` and
/// `.titlebar_is_accent_colored`. None for a set outside KDE's seven.
pub(crate) fn kde_accent_operands(set: SurfaceSet, key: KdeSchemeKey) -> &'static [Ref] {
    let set = KDE_ACCENT_SETS.iter().position(|held| *held == set);
    let key = KdeSchemeKey::ALL.iter().position(|held| *held == key);
    set.zip(key)
        .and_then(|(set, key)| {
            set.checked_mul(KdeSchemeKey::ALL.len())
                .and_then(|at| at.checked_add(key))
        })
        .and_then(|at| KDE_ACCENT_OPERANDS.calls.get(at))
        .map_or(&[], Vec::as_slice)
}

/// What `kde_accent_wm` reads itself, per key of [`KdeWmKey::ALL`] in its
/// order.
static KDE_ACCENT_WM_OPERANDS: LazyLock<AccentOperands> = LazyLock::new(|| {
    let calls = KdeWmKey::ALL
        .into_iter()
        .map(|key| {
            let inputs = vec![
                scheme_input(&["wm", key.name()]),
                scheme_input(&["window", "background_normal"]),
            ];
            accent_operands(inputs, false)
        })
        .collect();
    AccentOperands::of(calls)
});

/// What `kde_accent_wm(key)` reads itself (S2.2), in this order:
/// `input.accent`; the scheme inputs `input.kde_scheme.wm.<key>` and
/// `.window.background_normal`; the lists `input.kde_scheme.groups` and
/// `.unset_keys`; `token.kde.tint_factor` and `.titlebar_is_accent_colored`.
pub(crate) fn kde_accent_wm_operands(key: KdeWmKey) -> &'static [Ref] {
    KdeWmKey::ALL
        .iter()
        .position(|held| *held == key)
        .and_then(|at| KDE_ACCENT_WM_OPERANDS.calls.get(at))
        .map_or(&[], Vec::as_slice)
}

/// Whether `reference` is one of KDE's three scheme lists (S9.1), which a
/// KDE accent function reads itself: no input a preset declares.
pub(crate) fn is_kde_list(reference: &Ref) -> bool {
    matches!(reference, Ref::Input(path) if matches!(
        path.as_slice(),
        [scheme, list] if scheme.as_str() == "kde_scheme"
            && matches!(list.as_str(), "groups" | "inactive_groups" | "unset_keys")
    ))
}

/// What `m3_role` reads itself (S2.2), in this order: the seed
/// `input.accent`, `token.material.variant`, `.spec_version` and
/// `.contrast_level`, and the scheme of the colours being resolved.
pub(crate) fn m3_role_operands() -> &'static [Ref] {
    static OPERANDS: LazyLock<[Ref; 5]> = LazyLock::new(|| {
        let material = |field: Key| Ref::Token(vec![key!("material"), field]);
        [
            Ref::Input(vec![key!("accent")]),
            material(key!("variant")),
            material(key!("spec_version")),
            material(key!("contrast_level")),
            Ref::Scheme,
        ]
    });
    OPERANDS.as_slice()
}

/// `KdeEffects`' fields as `token.kde.<group>.<field>` names them, in the
/// order [`kde_group_tokens`] lists them.
pub(crate) const KDE_EFFECT_FIELDS: [&str; 7] = [
    "intensity",
    "intensity_amount",
    "color",
    "color_amount",
    "color_color",
    "contrast",
    "contrast_amount",
];

/// The tokens the KDE functions read themselves (S2.2, S4.8): the inactive
/// group's effects in [`KDE_EFFECT_FIELDS`]' order and
/// `token.kde.inactive_enabled`, the disabled group's effects, then
/// `token.kde.inactive_change_selection_color`.
static KDE_TOKENS: LazyLock<Vec<Ref>> = LazyLock::new(|| {
    let token = |path: Vec<Key>| Ref::Token([vec![key!("kde")], path].concat());
    let effects = |group: Key| {
        KDE_EFFECT_FIELDS
            .into_iter()
            .filter_map(Key::from_static)
            .map(move |field| token(vec![group.clone(), field]))
    };
    let mut out: Vec<Ref> = effects(key!("inactive")).collect();
    out.push(token(vec![key!("inactive_enabled")]));
    out.extend(effects(key!("disabled")));
    out.push(token(vec![key!("inactive_change_selection_color")]));
    out
});

/// [`KDE_TOKENS`]' entries `from..to`.
fn kde_tokens(from: usize, to: usize) -> &'static [Ref] {
    KDE_TOKENS.get(from..to).unwrap_or_default()
}

/// The token `kde_selection_inactive` reads itself (S2.2, S4.8):
/// `token.kde.inactive_change_selection_color`.
pub(crate) fn kde_selection_tokens() -> &'static [Ref] {
    let at = KDE_EFFECT_FIELDS.len().saturating_mul(2).saturating_add(1);
    kde_tokens(at, at.saturating_add(1))
}

/// The tokens `kde_effects` reads itself for `group` (S2.2, S4.8): the
/// group's effects, `token.kde.<group>.<field>` in [`KDE_EFFECT_FIELDS`]'
/// order, and for the inactive group `token.kde.inactive_enabled` after
/// them; none for the active group, which has no effects.
pub(crate) fn kde_group_tokens(group: KdeGroup) -> &'static [Ref] {
    let effects = KDE_EFFECT_FIELDS.len();
    match group {
        KdeGroup::Active => &[],
        KdeGroup::Inactive => kde_tokens(0, effects.saturating_add(1)),
        KdeGroup::Disabled => kde_tokens(
            effects.saturating_add(1),
            effects.saturating_mul(2).saturating_add(1),
        ),
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Variant(name) => write!(f, "{name}"),
            Name::Key(key) => write!(f, "@{key}"),
        }
    }
}

/// Writes a path's segments joined by `.`.
fn write_path(f: &mut fmt::Formatter<'_>, path: &[Key]) -> fmt::Result {
    let mut first = true;
    for segment in path {
        if !first {
            f.write_str(".")?;
        }
        first = false;
        write!(f, "{segment}")?;
    }
    Ok(())
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ref::Role { set, path } => {
                write!(f, "role.{}.", set.name())?;
                write_path(f, path)
            }
            Ref::GlobalRole(path) => {
                f.write_str("role.global.")?;
                write_path(f, path)
            }
            Ref::KeyedRole(key) => write!(f, "role.@{key}"),
            Ref::Surface(path) => {
                f.write_str("surface.")?;
                write_path(f, path)
            }
            Ref::Active(inner) => write!(f, "active({inner})"),
            Ref::Token(path) => {
                f.write_str("token.")?;
                write_path(f, path)
            }
            Ref::Input(path) => {
                f.write_str("input.")?;
                write_path(f, path)
            }
            Ref::Base => f.write_str("base"),
            Ref::Scheme => f.write_str("scheme"),
            Ref::FontSize => f.write_str("font_size"),
            Ref::LineHeight => f.write_str("line_height"),
            Ref::Element(datum) => write!(f, "element.{}", datum.name()),
            Ref::BodyFontHeight => f.write_str("body_font_height"),
            Ref::Part(path) => {
                f.write_str("part.")?;
                write_path(f, path)
            }
            Ref::Widget { part, path } => {
                write!(f, "widget.{part}.")?;
                write_path(f, path)
            }
            Ref::Values(name) => write!(f, "values.{name}"),
            Ref::TextScale => f.write_str("text_scale"),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Color(c) => write!(f, "srgb({}, {}, {}, {})", c.r, c.g, c.b, c.a),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Px(Px(n)) => write!(f, "{n}px"),
            Literal::Ms(Ms(n)) => write!(f, "{n}ms"),
            Literal::Deg(Deg(n)) => write!(f, "{n}deg"),
            Literal::Pt(n) => write!(f, "{n}pt"),
            Literal::Name(name) => write!(f, "{name}"),
            Literal::Bool(b) => write!(f, "{b}"),
        }
    }
}

/// The expression in the grammar's spelling, without TOML's leading `=`;
/// [`Expr::parse`] reads it back to the same expression.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Lit(literal) => write!(f, "{literal}"),
            Expr::Ref(reference) => write!(f, "{reference}"),
            Expr::Call(func, args) => {
                write!(f, "{func}(")?;
                let mut first = true;
                for arg in args {
                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;
                    write!(f, "{arg}")?;
                }
                f.write_str(")")
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// The KDE functions' tokens name `KdeEffects`' fields in its order,
    /// each group's whole, and the two switches (S2.2, S4.8).
    #[test]
    fn kde_tokens_name_the_kde_fields() {
        use crate::blocks::Table;
        let fields: Vec<&str> = crate::tokens::KdeEffects::FIELDS
            .iter()
            .map(|field| field.path)
            .collect();
        assert_eq!(fields, KDE_EFFECT_FIELDS);
        let named = |tokens: &[Ref]| -> Vec<String> { tokens.iter().map(Ref::to_string).collect() };
        let group = |group: &str, then: &[&str]| -> Vec<String> {
            KDE_EFFECT_FIELDS
                .iter()
                .map(|field| format!("token.kde.{group}.{field}"))
                .chain(then.iter().map(|name| format!("token.kde.{name}")))
                .collect()
        };
        assert_eq!(
            named(kde_group_tokens(KdeGroup::Inactive)),
            group("inactive", &["inactive_enabled"])
        );
        assert_eq!(
            named(kde_group_tokens(KdeGroup::Disabled)),
            group("disabled", &[])
        );
        assert!(kde_group_tokens(KdeGroup::Active).is_empty());
        let call = |text: &str| match Expr::parse(text).unwrap() {
            Expr::Call(func, args) => named(func.token_operands(&args)),
            other => panic!("{other:?}"),
        };
        assert_eq!(
            call("kde_effects(#000000, #ffffff, disabled, foreground)"),
            group("disabled", &[])
        );
        assert_eq!(
            call("kde_effects(#000000, #ffffff, token.group, foreground)"),
            [
                group("inactive", &["inactive_enabled"]),
                group("disabled", &[])
            ]
            .concat()
        );
        assert_eq!(
            call("kde_selection_inactive(#000000, #ffffff, #000000, background)"),
            ["token.kde.inactive_change_selection_color"]
        );
        assert!(call("kcu_mix(#000000, #ffffff, 0.5)").is_empty());
    }

    /// `m3_role` lists the seed, `token.material`'s three fields and the
    /// scheme as its operands (S2.2), whatever role it names.
    #[test]
    fn m3_role_operands_name_its_reads() {
        let Expr::Call(func, args) = Expr::parse("m3_role(@primary)").unwrap() else {
            panic!("a call");
        };
        let named: Vec<String> = func
            .token_operands(&args)
            .iter()
            .map(Ref::to_string)
            .collect();
        assert_eq!(
            named,
            [
                "input.accent",
                "token.material.variant",
                "token.material.spec_version",
                "token.material.contrast_level",
                "scheme"
            ]
        );
        let fields: Vec<&str> = <crate::tokens::MaterialScheme as crate::blocks::Table>::FIELDS
            .iter()
            .map(|field| field.path)
            .collect();
        assert_eq!(fields, ["variant", "spec_version", "contrast_level"]);
    }

    #[test]
    fn names_round_trip() {
        for func in Func::ALL {
            assert_eq!(Func::from_name(func.name()), Some(func));
        }
        for datum in ElementDatum::ALL {
            assert_eq!(ElementDatum::from_name(datum.name()), Some(datum));
        }
    }
}
