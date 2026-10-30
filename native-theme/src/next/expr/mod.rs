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
    expect(
        dead_code,
        reason = "typing and evaluation are called by S12's checks and S8's query (M1a tasks 8 to 11)"
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

use super::color::Color;
use super::key::Key;
use super::roles::SurfaceSet;
use super::units::{Deg, Ms, Px};

pub use parse::{ParseError, ParseErrorKind};

/// How deep an expression may nest: calls and `active(…)` inside each other
/// (S2.2, S12).
pub const MAX_DEPTH: usize = 32;

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
    pub const ALL: [ElementDatum; 22] = [
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

/// A function of S2.2's closed set: the model's own arithmetic (`alpha`, `mix`,
/// `mul`, `div`, `add`, `sub`, `min`, `max`, `floor`, `round`, `pick`) and the platform
/// functions, each implemented to its source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Func {
    /// `alpha(c, a)`: `c` with its alpha multiplied by `a`.
    Alpha,
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
    /// `qt_lighter(c, factor)`: `QColor::lighter`.
    QtLighter,
    /// `qt_darker(c, factor)`: `QColor::darker`.
    QtDarker,
    /// `kcu_overlay(under, over)`: `KColorUtils::overlayColors`.
    KcuOverlay,
    /// `kirigami_tint(c, tint, alpha)`: Kirigami's `ColorUtils.tintWithAlpha`.
    KirigamiTint,
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
    pub const ALL: [Func; 39] = [
        Func::Alpha,
        Func::Mix,
        Func::KdeEffects,
        Func::KdeSelectionInactive,
        Func::KcuMix,
        Func::KcuTint,
        Func::KcuShade,
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
        Func::QtLighter,
        Func::QtDarker,
        Func::KcuOverlay,
        Func::KirigamiTint,
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
            Func::Mix => "mix",
            Func::KdeEffects => "kde_effects",
            Func::KdeSelectionInactive => "kde_selection_inactive",
            Func::KcuMix => "kcu_mix",
            Func::KcuTint => "kcu_tint",
            Func::KcuShade => "kcu_shade",
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
            Func::QtLighter => "qt_lighter",
            Func::QtDarker => "qt_darker",
            Func::KcuOverlay => "kcu_overlay",
            Func::KirigamiTint => "kirigami_tint",
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
mod tests {
    use super::*;

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
