//! The connector on the 0.8.0 model ([`native_theme::next`]), beside the
//! 0.7.2 one until the 0.8.0 cut removes the latter: what the connector
//! hands to gpui, computed from a [`native_theme::next::ResolvedTheme`].
//!
//! [`to_theme`] builds the whole gpui-component theme, as [`crate::to_theme`]
//! does; [`geometry`], [`variants`] and [`base_layer`] hold one function for
//! each public function of [`crate::geometry`], [`crate::variants`] and
//! [`crate::base_layer`] that reads the native theme, of the same name. Each
//! takes the 0.8.0 resolved theme, which carries its name, its scheme and
//! the context it was resolved for (text scale, reduced transparency), so no
//! name, mode or accessibility preferences are passed; and each returns
//! [`Mapped`]: the value, or the [`Unmapped`] source it needs that the theme
//! leaves unstated, where the 0.7.2 function, whose fields are always set,
//! returns the value alone. Where the theme states nothing for a value the
//! toolkit has one of its own for, the toolkit's stays (spec §10, rule 0).
//!
//! Each function reads the model where the table of homes
//! (`native-theme/tests/next_homes.toml`) places the 0.7.2 field the old
//! mapping reads, by the query its row names, and hands the values to the
//! function the old mapping hands its own reads to, so the two share
//! everything after the read. `parity` proves the leaves equal to the old
//! mapping's (docs/v0.8.0-plan.md, section 1, rule 2), and lists where they
//! differ with the reason.
//!
//! A 0.7.2 field that is a soft option of a state (`button.active_background`,
//! `scrollbar.thumb_active_color`, `button.checked_background`, ...) is unset
//! where the theme states nothing for the state; a query at a state the theme
//! gives no rule returns the part without that state instead. Such a field is
//! read here only where the property's `from_state` bit is set at the state
//! (spec §2), and is `None` otherwise, so the old path's own fallback for the
//! unset option is taken from both models.

pub mod base_layer;
pub(crate) mod builders;
pub mod geometry;
#[cfg(test)]
pub(crate) mod parity;
pub mod variants;

use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use gpui::{App, Hsla, SharedString};
use gpui_base::ResizableTheme;
use gpui_base::input::CursorBlink;
use gpui_component::theme::{Theme as GpuiTheme, ThemeColor, ThemeConfig, ThemeMode};
use native_theme::next::{
    ButtonQuery, Color, Contrast, CornerCurve, CornerRadius, DialogButtonOrder, FocusTrigger,
    FontStyle, IconSizes, ListQuery, MotionPreference, Orientation, Paint, PartStated, PartStyle,
    ResolvedTheme, Stroke, StyleQuery, TextStated, TextStyle, WidgetState, WindowState,
};

use crate::base_layer::{ScrollbarGeometry, ScrollbarSource, geometry_of, resizable_theme_of};
use crate::colors::{
    CappedNatives, SliceColors, WidgetColors, capped_natives_of, channels_to_hsla, focus_band,
    theme_color_of,
};
use crate::config::{apply_config_scalars, highlight_style_of, theme_color_to_config_colors};
use crate::geometry::Font;
use crate::{AccessibilityPreferences, ThemeScalars, apply_scalars, caret_blink_of, font_family};

/// A source a value needs that the theme leaves unstated, or states in a
/// form the connector cannot hand to gpui (a gradient fill, a per-side
/// stroke, a pill): its place in the 0.8.0 model, as a query and a property
/// (`button.body(NONE).stroke.width`). The 0.7.2 path has no such case, its
/// resolved fields being always set; the 0.8.0 path reports it rather than
/// inventing the value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unmapped(pub(crate) &'static str);

impl Unmapped {
    /// The source's place in the 0.8.0 model.
    #[must_use]
    pub fn home(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Unmapped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the theme states no value gpui can take at {}", self.0)
    }
}

impl std::error::Error for Unmapped {}

/// A value of the 0.8.0 mapping, or the source it lacks.
pub type Mapped<T> = Result<T, Unmapped>;

/// A colour of the model as gpui's: the connector's conversion
/// ([`channels_to_hsla`]).
pub(crate) fn hsla(c: Color) -> Hsla {
    channels_to_hsla([c.r, c.g, c.b, c.a])
}

/// `v`, or `Unmapped(home)` where it is `None`.
pub(crate) fn stated<T>(v: Option<T>, home: &'static str) -> Mapped<T> {
    v.ok_or(Unmapped(home))
}

/// A part's fill, where stated as one colour.
pub(crate) fn fill(part: &PartStyle, home: &'static str) -> Mapped<Hsla> {
    stated(fill_of(part), home)
}

/// A part's fill as `fill` reads it, `None` where it is not stated, or
/// stated as a layered paint with no fallback colour: for a reader that
/// shows the toolkit's own there (spec §10, rule 0), as the showcase's
/// Widget Info does. A fill stated as none is transparent, and a layered
/// paint gives its fallback colour (spec §10, rule 2).
pub fn fill_of(part: &PartStyle) -> Option<Hsla> {
    fill_color_of(part).map(hsla)
}

/// A part's stated fill as one colour of the model, as [`fill_of`] reads it.
fn fill_color_of(part: &PartStyle) -> Option<Color> {
    if !part.stated.contains(PartStated::FILL) {
        return None;
    }
    match &part.fill {
        Paint::None => Some(Color::TRANSPARENT),
        Paint::Solid(c) => Some(*c),
        paint @ Paint::Complex(_) => paint.fallback_color(),
    }
}

/// A part's stroke colour, where stated as one stroke of one colour, in any
/// alignment; stated as none, transparent, no line (spec §10, rule 2), as
/// [`fill_color_of`] reads a fill stated as none: a builder draws it at the
/// width [`stroke_width`] gives none, 0, and a colour token, which has no
/// width to say "none" by, is transparent.
///
/// gpui paints a quad's border over its background, inside its bounds, so
/// the fill runs under the whole border: `StrokeAlign::Inside` (spec §2,
/// below `PartStyle`), which it draws from the stated colours. Under an
/// opaque stroke the three alignments paint the same picture, so `Center`
/// and `Outside` are drawn exactly from the stated colours too. A
/// translucent `Center` or `Outside` stroke shows the fill under the whole
/// band where the theme has it end at the band's centre line or inner edge;
/// no border colour can take the fill away there, and each builder gives a
/// part as one style refinement, which gpui paints as one quad, so gpui's
/// extent stays and the connector's README records the limit (spec §10,
/// rule 4).
pub(crate) fn stroke_color(part: &PartStyle, home: &'static str) -> Mapped<Hsla> {
    if part.stated.contains(PartStated::STROKE) && matches!(part.stroke, Stroke::None) {
        return Ok(hsla(Color::TRANSPARENT));
    }
    stated(stroke_color_of(part), home)
}

/// A part's stroke colour as `stroke_color` reads one stroke, `None` where
/// it is not stated, stated as none (no line: [`stroke_width_of`] gives 0),
/// or stated in a form gpui cannot draw (not one stroke of one colour).
pub fn stroke_color_of(part: &PartStyle) -> Option<Hsla> {
    match &part.stroke {
        Stroke::Uniform { color, .. } if part.stated.contains(PartStated::STROKE) => {
            Some(hsla(*color))
        }
        _ => None,
    }
}

/// A part's stroke width, where stated as one stroke, in any alignment (the
/// band lies inside the part's box in each, spec §2), or as none, which is
/// no width.
pub(crate) fn stroke_width(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    stated(stroke_width_of(part), home)
}

/// A part's stroke width as `stroke_width` reads it, `None` where it is
/// not stated, or stated in a form gpui cannot draw (not one stroke, nor
/// none).
pub fn stroke_width_of(part: &PartStyle) -> Option<f32> {
    if !part.stated.contains(PartStated::STROKE) {
        return None;
    }
    match &part.stroke {
        Stroke::None => Some(0.0),
        Stroke::Uniform { width, .. } => Some(*width),
        _ => None,
    }
}

/// A part's corner radius, where stated as one circular radius, or as square
/// corners, which are no radius.
pub(crate) fn radius(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    stated(radius_of(part), home)
}

/// A part's corner radius as `radius` reads it, `None` where it is not
/// stated, or stated in a form gpui cannot draw (not one circular radius,
/// nor square corners: pill, per-corner or concentric corners, a continuous
/// curve).
pub fn radius_of(part: &PartStyle) -> Option<f32> {
    if !part.stated.contains(PartStated::CORNERS) {
        return None;
    }
    match (part.corners.radius, part.corners.curve) {
        (CornerRadius::None, _) => Some(0.0),
        (CornerRadius::Uniform(r), CornerCurve::Circular) => Some(r),
        _ => None,
    }
}

/// A part's padding, top, right, bottom, left, each side where stated, as
/// 0.7.2's per-side padding.
pub fn padding_of(part: &PartStyle) -> [Option<f32>; 4] {
    let side = |bit: PartStated, v: f32| part.stated.contains(bit).then_some(v);
    let p = &part.padding;
    [
        side(PartStated::PADDING_TOP, p.top),
        side(PartStated::PADDING_RIGHT, p.right),
        side(PartStated::PADDING_BOTTOM, p.bottom),
        side(PartStated::PADDING_LEFT, p.left),
    ]
}

/// A text style's colour, where stated.
pub(crate) fn text_color(text: &TextStyle, home: &'static str) -> Mapped<Hsla> {
    stated(text_color_of(text), home)
}

/// A text style's colour as `text_color` reads it, `None` where it is not
/// stated.
pub fn text_color_of(text: &TextStyle) -> Option<Hsla> {
    text.stated
        .contains(TextStated::COLOR)
        .then(|| hsla(text.color))
}

/// A part's fill where a rule states it for the queried state (its
/// `from_state` bit), as [`fill`] reads it; `None` where none does, a soft
/// option's unset value.
pub(crate) fn state_fill(part: &PartStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    state_option(
        state_fill_of(part),
        part.from_state.contains(PartStated::FILL),
        home,
    )
}

/// A state's soft option as a `Mapped` form reads it through its `_of`
/// read: the value where it gives one; where it gives none, unset if no
/// rule states the property for the queried state (`ruled` false), and
/// the home lacked if one does, in a form gpui cannot take.
fn state_option(value: Option<Hsla>, ruled: bool, home: &'static str) -> Mapped<Option<Hsla>> {
    match value {
        Some(value) => Ok(Some(value)),
        None if ruled => Err(Unmapped(home)),
        None => Ok(None),
    }
}

/// A part's fill as `state_fill` reads it, `None` where no rule states it
/// for the queried state, or it is stated in a form gpui cannot paint (not
/// one colour). The rule is for some flag of the query, not a given one
/// (spec §2, `from_state`): a reader that keeps the toolkit's own treatment
/// of one flag over a value stated for another queries with that flag
/// alone.
pub fn state_fill_of(part: &PartStyle) -> Option<Hsla> {
    part.from_state
        .contains(PartStated::FILL)
        .then(|| fill_of(part))
        .flatten()
}

/// A part's stroke colour as `state_stroke_color` reads it, a stroke stated
/// as none transparent, no line (spec §10, rule 2); `None` where no rule
/// states it for the queried state, or it is stated in a form gpui cannot
/// draw (not one stroke of one colour, nor none). The rule is for some flag
/// of the query, not a given one (spec §2, `from_state`): a reader that
/// keeps the toolkit's own treatment of one flag over a value stated for
/// another queries with that flag alone.
pub fn state_stroke_color_of(part: &PartStyle) -> Option<Hsla> {
    if !part.from_state.contains(PartStated::STROKE) {
        return None;
    }
    match part.stroke {
        Stroke::None if part.stated.contains(PartStated::STROKE) => Some(hsla(Color::TRANSPARENT)),
        _ => stroke_color_of(part),
    }
}

/// A text style's colour as `state_text_color` reads it, `None` where no
/// rule states it for the queried state or it is not stated. The rule is
/// for some flag of the query, not a given one (spec §2, `from_state`): a
/// reader that keeps the toolkit's own treatment of one flag over a value
/// stated for another queries with that flag alone.
pub fn state_text_color_of(text: &TextStyle) -> Option<Hsla> {
    text.from_state
        .contains(TextStated::COLOR)
        .then(|| text_color_of(text))
        .flatten()
}

/// A part's fill where the theme states it, as [`fill`] reads it; `None`
/// where it states none, and the toolkit's own stays (spec §10, rule 0).
pub(crate) fn stated_fill(part: &PartStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    if part.stated.contains(PartStated::FILL) {
        fill(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A part's stroke colour where a rule states it for the queried state, as
/// [`stroke_color`] reads it, a stroke stated as none transparent; `None`
/// where none does.
pub(crate) fn state_stroke_color(part: &PartStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    if part.from_state.contains(PartStated::STROKE) {
        stroke_color(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A text style's colour where a rule states it for the queried state, as
/// [`text_color`] reads it; `None` where none does.
fn state_text_color(text: &TextStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    state_option(
        state_text_color_of(text),
        text.from_state.contains(TextStated::COLOR),
        home,
    )
}

/// A text style's size, where stated.
fn text_size(text: &TextStyle, home: &'static str) -> Mapped<f32> {
    if text.stated.contains(TextStated::SIZE_PX) {
        Ok(text.size_px)
    } else {
        Err(Unmapped(home))
    }
}

/// A slant of the model as gpui's.
pub(crate) fn font_style(style: FontStyle) -> Option<gpui::FontStyle> {
    match style {
        FontStyle::Normal => Some(gpui::FontStyle::Normal),
        FontStyle::Italic => Some(gpui::FontStyle::Italic),
        FontStyle::Oblique => Some(gpui::FontStyle::Oblique),
        _ => None,
    }
}

/// A text style as the builders' [`Font`], where its family, size, weight,
/// slant and colour are stated: for the builders that set the colour. The
/// size is the model's, which the theme's text scale has already scaled.
fn font<'a>(text: &'a TextStyle, home: &'static str) -> Mapped<Font<'a>> {
    font_of(text, home, TextStated::COLOR)
}

/// A text style as the builders' [`Font`], where its family, size, weight
/// and slant are stated: for the builders that hand it to
/// `geometry::text`, which sets no colour and does not read the font's. The
/// colour carried is the style's, stated or not.
pub(crate) fn uncoloured_font<'a>(text: &'a TextStyle, home: &'static str) -> Mapped<Font<'a>> {
    font_of(text, home, TextStated::default())
}

/// Where `text`, read at its home `home`, gives no font gpui can letter a
/// label in, that home: its family, size, weight or slant unstated, or a
/// slant gpui has no form for; `None` where it gives one. The check a
/// theme-drawn widget's label font passes (`widgets`, `LabelFont::next`), for
/// a reader that shows gpui-component's own control in its place.
#[must_use]
pub fn label_font_unmapped(text: &TextStyle, home: &'static str) -> Option<Unmapped> {
    uncoloured_font(text, home).err()
}

/// [`font`] and [`uncoloured_font`]: the four properties every builder reads
/// and `also`.
fn font_of<'a>(text: &'a TextStyle, home: &'static str, also: TextStated) -> Mapped<Font<'a>> {
    let needed =
        TextStated::FAMILY | TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::STYLE | also;
    if !text.stated.contains(needed) {
        return Err(Unmapped(home));
    }
    Ok(Font {
        family: &text.family,
        size: text.size_px,
        weight: text.weight,
        style: stated(font_style(text.style), home)?,
        color: hsla(text.color),
    })
}

/// Whether gpui-component's focus band is on for any focus: the focus
/// indicator's stroke width (no stroke is no width) and its trigger
/// (0.7.2's `focus_ring_width` and `focus_ring_keyboard_only`).
fn focus_band_of(r: &ResolvedTheme) -> Mapped<bool> {
    let ring = &r.focus_ring;
    let width = match &ring.stroke {
        Stroke::None => 0.0,
        Stroke::Uniform { width, .. } => *width,
        _ => return Err(Unmapped("focus_ring.stroke.width")),
    };
    Ok(focus_band(
        width,
        ring.trigger == Some(FocusTrigger::KeyboardOnly),
    ))
}

/// The scroll bar's thumb at rest: the handle of a view without the focus,
/// where 0.7.2's `scrollbar.thumb_color` lives on kde-breeze (spec §5, S3);
/// a theme with no rule for that state returns the plain thumb, the field's
/// home elsewhere.
///
/// That makes `thumb(UNFOCUSED_VIEW)` the one query that returns 0.7.2's
/// `thumb_color`, the plain handle, on every preset, whichever of the two
/// the preset states it at: kde-breeze's rest state is the handle of the
/// view that has the focus, and the plain handle is its `UNFOCUSED_VIEW`
/// rule; a preset without an `UNFOCUSED_VIEW` rule returns its base there.
/// When the widgets are ported, the connector sets the bit from the view's
/// real focus instead.
fn thumb_at_rest(r: &ResolvedTheme) -> Mapped<Hsla> {
    fill(
        &r.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW),
        "scrollbar.thumb(UNFOCUSED_VIEW).fill",
    )
}

/// The opacity of `accent` gpui-component gives `list_hover` where a
/// `ThemeConfig` states none, its own hovered row; `table_hover` falls back
/// to `list_hover` (gpui-component 0.10.0 `src/theme/schema.rs`,
/// `ThemeColor::apply_config`: `list_hover`'s fallback
/// `self.accent.opacity(0.6)`).
pub(crate) const UPSTREAM_LIST_HOVER_ACCENT_OPACITY: f32 = 0.6;

/// A hovered list row's fill, `list.highlight(HOVER).fill`, which the theme
/// colours `list_hover` and `table_hover` with; where the theme states none,
/// gpui-component's own hovered row, `accent` at
/// [`UPSTREAM_LIST_HOVER_ACCENT_OPACITY`], so the toolkit's own stays (spec
/// §10, rule 0). `accent` is the menu's hovered item, which the old mapping
/// writes there (`colors::assign_core`).
fn list_row_hover(r: &ResolvedTheme) -> Mapped<Hsla> {
    let home = "list.highlight(HOVER).fill";
    match stated_fill(&r.list.highlight(WidgetState::HOVER), home)? {
        Some(hover) => Ok(hover),
        None => Ok(menu_hover_fill(r)?.opacity(UPSTREAM_LIST_HOVER_ACCENT_OPACITY)),
    }
}

/// gpui-component's own colours of the theme's scheme, its default theme's
/// (`ThemeColor::light()` and `dark()`, gpui-component 0.10.0
/// `theme/theme_color.rs:552-561`, `theme/default-theme.json`): what a
/// widget shows in a state the theme states nothing for (spec §10, rule 0).
fn upstream_colors(r: &ResolvedTheme) -> std::sync::Arc<ThemeColor> {
    if r.is_dark() {
        ThemeColor::dark()
    } else {
        ThemeColor::light()
    }
}

/// A hovered menu item's fill, `accent`, which gpui-component paints a
/// hovered item in over its unfilled rest (gpui-component 0.10.0
/// `menu/menu_item.rs:134-137`): `menu.item(HOVER).fill` where a rule states
/// it for `HOVER` (spec §2, `from_state`), else gpui-component's own `accent`
/// (spec §10, rule 0).
fn menu_hover_fill(r: &ResolvedTheme) -> Mapped<Hsla> {
    state_fill_or(
        &r.menu.item(WidgetState::HOVER),
        "menu.item(HOVER).fill",
        upstream_colors(r).accent,
    )
}

/// A hovered menu item's label, `accent_foreground`, which gpui-component
/// letters a hovered item in (gpui-component 0.10.0
/// `menu/menu_item.rs:122, 134-137`): `menu.item_label(HOVER).color` where a
/// rule states it for `HOVER` (spec §2, `from_state`); where a rule states
/// the item's fill (`item`) for `HOVER` and none its label, the label at
/// rest, `rest` (spec §10, rule 0a: a label is chosen for the fill it lies
/// on); else gpui-component's own `accent_foreground`, `upstream`, which goes
/// with its own `accent` fill (spec §10, rule 0).
fn menu_hover_label(
    item: &PartStyle,
    label: &TextStyle,
    rest: &TextStyle,
    upstream: Hsla,
) -> Mapped<Hsla> {
    if let Some(stated) = state_text_color(label, "menu.item_label(HOVER).color")? {
        return Ok(stated);
    }
    if state_fill(item, "menu.item(HOVER).fill")?.is_some() {
        return text_color(rest, "menu.item_label(NONE).color");
    }
    Ok(upstream)
}

/// A hovered push button's fill, `button_hover` and `button_secondary_hover`
/// ([`button_states`]), which gpui-component paints a hovered button in
/// (gpui-component 0.10.0 `button/button.rs:1194`,
/// `theme/schema.rs:854`): `button.body(HOVER).fill` where a rule states it
/// for `HOVER` (spec §2, `from_state`), else gpui-component's own treatment
/// (spec §10, rule 0): its theme's `secondary_hover` where that theme turns a
/// button on hover (its `secondary_hover` is not its `secondary`), else the
/// button at rest, `rest`, as that theme leaves it.
fn button_hover_fill(part: &PartStyle, rest: Hsla, upstream: &ThemeColor) -> Mapped<Hsla> {
    let own = if upstream.secondary_hover == upstream.secondary {
        rest
    } else {
        upstream.secondary_hover
    };
    state_fill_or(part, "button.body(HOVER).fill", own)
}

/// A pressed push button's fill, `button_active` and
/// `button_secondary_active` ([`button_states`]), which gpui-component paints
/// a pressed button in (gpui-component 0.10.0 `button/button.rs:1278`,
/// `theme/schema.rs:855`): `button.body(PRESSED).fill` where a rule states it
/// for `PRESSED` (spec §2, `from_state`), else gpui-component's own treatment
/// (spec §10, rule 0): its theme's `secondary_active` where that theme turns
/// a button pressed (its `secondary_active` is not its `secondary`), else the
/// button at rest, `rest`, as that theme leaves it. Always `Some`: the 0.7.2
/// model's `None`, the connector's darkening of the rest fill
/// (`colors::assign_secondary`), is not the 0.8.0 model's.
fn button_pressed_fill(
    part: &PartStyle,
    rest: Hsla,
    upstream: &ThemeColor,
) -> Mapped<Option<Hsla>> {
    let own = if upstream.secondary_active == upstream.secondary {
        rest
    } else {
        upstream.secondary_active
    };
    state_fill_or(part, "button.body(PRESSED).fill", own).map(Some)
}

/// A part's fill where a rule states it for the queried state
/// ([`state_fill`]), else `upstream`, gpui-component's own (spec §10,
/// rule 0).
fn state_fill_or(part: &PartStyle, home: &'static str, upstream: Hsla) -> Mapped<Hsla> {
    Ok(state_fill(part, home)?.unwrap_or(upstream))
}

/// The slice's colours of a 0.8.0 resolved theme.
pub(crate) fn slice_colors(r: &ResolvedTheme) -> Mapped<SliceColors> {
    let c = &r.colors;
    let b = &r.button;
    let l = &r.list;
    let s = &r.scrollbar;
    let secondary = fill(&b.body(WidgetState::NONE), "button.body(NONE).fill")?;
    Ok(SliceColors {
        fg: hsla(stated(c.window.foreground, "colors.window.foreground")?),
        accent: hsla(c.accent.fill),
        border: hsla(stated(c.window.border, "colors.window.border")?),
        muted_fg: hsla(stated(
            c.window.foreground_muted,
            "colors.window.foreground_muted",
        )?),
        primary: fill(
            &b.body(ButtonQuery::primary()),
            "button.body(NONE; variant=Primary).fill",
        )?,
        primary_fg: text_color(
            &b.label(ButtonQuery::primary()),
            "button.label(NONE; variant=Primary).color",
        )?,
        secondary,
        secondary_fg: text_color(&b.label(WidgetState::NONE), "button.label(NONE).color")?,
        danger: hsla(c.error.fill),
        danger_fg: hsla(c.error.foreground),
        success: hsla(c.success.fill),
        success_fg: hsla(c.success.foreground),
        warning: hsla(c.warning.fill),
        warning_fg: hsla(c.warning.foreground),
        info: hsla(c.info.fill),
        info_fg: hsla(c.info.foreground),
        focus_ring: hsla(c.focus_ring),
        focus_band: focus_band_of(r)?,
        list_bg: fill(&l.container(WidgetState::NONE), "list.container(NONE).fill")?,
        list_grid: hsla(l.grid_color),
        list_header_bg: fill(&l.header(WidgetState::NONE), "list.header(NONE).fill")?,
        list_header_fg: text_color(
            &l.header_label(WidgetState::NONE),
            "list.header_label(NONE).color",
        )?,
        alternate_row: fill(
            &l.row(ListQuery::new(WidgetState::NONE).alternate(true)),
            "list.row(NONE; alternate=true).fill",
        )?,
        scrollbar_track: fill(&s.track(WidgetState::NONE), "scrollbar.track(NONE).fill")?,
        scrollbar_thumb: thumb_at_rest(r)?,
        scrollbar_thumb_hover: fill(&s.thumb(WidgetState::HOVER), "scrollbar.thumb(HOVER).fill")?,
        button_hover_bg: button_hover_fill(
            &b.body(WidgetState::HOVER),
            secondary,
            &upstream_colors(r),
        )?,
        button_active_bg: button_pressed_fill(
            &b.body(WidgetState::PRESSED),
            secondary,
            &upstream_colors(r),
        )?,
        list_hover_bg: list_row_hover(r)?,
        list_selection_bg: fill(
            &l.highlight(WidgetState::SELECTED),
            "list.highlight(SELECTED).fill",
        )?,
        shadow: hsla(c.shadow),
    })
}

/// A window's frame, normal: not maximized, full screen or tiled.
fn normal_window() -> WindowState {
    WindowState::default()
}

/// The colours outside the slice of a 0.8.0 resolved theme.
pub(crate) fn widget_colors(r: &ResolvedTheme) -> Mapped<WidgetColors> {
    let frame = r.window.frame(normal_window());
    let sidebar = r.sidebar.container(WidgetState::NONE);
    let tab = &r.tab;
    let link = &r.link;
    Ok(WidgetColors {
        bg: fill(&frame, "window.frame(normal).fill")?,
        menu_hover_bg: menu_hover_fill(r)?,
        menu_hover_fg: menu_hover_label(
            &r.menu.item(WidgetState::HOVER),
            &r.menu.item_label(WidgetState::HOVER),
            &r.menu.item_label(WidgetState::NONE),
            upstream_colors(r).accent_foreground,
        )?,
        sidebar_selection_bg: fill(
            &r.sidebar.item(WidgetState::SELECTED),
            "sidebar.item(SELECTED).fill",
        )?,
        sidebar_selection_fg: text_color(
            &r.sidebar.item_label(WidgetState::SELECTED),
            "sidebar.item_label(SELECTED).color",
        )?,
        selection: hsla(r.colors.text_selection.background),
        link: text_color(&link.label(WidgetState::NONE), "link.label(NONE).color")?,
        input: stroke_color(
            &r.input.field(WidgetState::NONE),
            "input.field(NONE).stroke.color",
        )?,
        sidebar: fill(&sidebar, "sidebar.container(NONE).fill")?,
        sidebar_fg: text_color(
            &r.sidebar.item_label(WidgetState::NONE),
            "sidebar.item_label(NONE).color",
        )?,
        sidebar_border: stroke_color(&sidebar, "sidebar.container(NONE).stroke.color")?,
        popover: fill(&r.popover.panel, "popover.panel.fill")?,
        popover_fg: text_color(&r.popover.text, "popover.text.color")?,
        tab_bg: fill(&tab.tab(WidgetState::NONE), "tab.tab(NONE).fill")?,
        tab_active_bg: fill(&tab.tab(WidgetState::SELECTED), "tab.tab(SELECTED).fill")?,
        tab_active_fg: text_color(
            &tab.label(WidgetState::SELECTED),
            "tab.label(SELECTED).color",
        )?,
        tab_bar_bg: fill(&tab.bar(WidgetState::NONE), "tab.bar(NONE).fill")?,
        segmented_bg: fill(
            &r.segmented_control.track(WidgetState::NONE),
            "segmented_control.track(NONE).fill",
        )?,
        tab_fg: text_color(&tab.label(WidgetState::NONE), "tab.label(NONE).color")?,
        title_bar_bg: fill(
            &r.window.title_bar(normal_window()),
            "window.title_bar(normal).fill",
        )?,
        window_border: stroke_color(&frame, "window.frame(normal).stroke.color")?,
        slider_fill: fill(
            &r.slider.active_track(WidgetState::NONE),
            "slider.active_track(NONE).fill",
        )?,
        slider_thumb: fill(
            &r.slider.thumb(WidgetState::NONE),
            "slider.thumb(NONE).fill",
        )?,
        switch_unchecked: fill(
            &r.switch.track(WidgetState::NONE),
            "switch.track(NONE).fill",
        )?,
        switch_thumb: fill(
            &r.switch.thumb(WidgetState::CHECKED),
            "switch.thumb(CHECKED).fill",
        )?,
        progress_fill: fill(
            &r.progress_bar.fill(WidgetState::NONE),
            "progress_bar.fill(NONE).fill",
        )?,
        input_caret: hsla(r.colors.caret),
        // A link button's text turns from `link` to `link_hover` and
        // `link_active` (gpui-component 0.10.0 `button/button.rs:1240, 1316`),
        // which gpui-component's own theme leaves at `link`
        // (`theme/schema.rs:977-978`): unstated for the state, the label's
        // rest colour is gpui-component's own treatment (spec §10, rule 0).
        link_hover_fg: text_color(&link.label(WidgetState::HOVER), "link.label(HOVER).color")?,
        link_active_fg: text_color(
            &link.label(WidgetState::PRESSED),
            "link.label(PRESSED).color",
        )?,
        separator: hsla(r.separator.color),
        status_bar: fill(&r.status_bar.bar, "status_bar.bar.fill")?,
        status_bar_border: stroke_color(&r.status_bar.bar, "status_bar.bar.stroke.color")?,
    })
}

/// The `ThemeColor` fields, written by the old mapping's [`theme_color_of`],
/// but for the push button's hovered and pressed slots ([`button_states`])
/// and the flat button's ([`flat_button_states`]).
pub(crate) fn theme_color(r: &ResolvedTheme) -> Mapped<ThemeColor> {
    let mut tc = theme_color_of(
        &slice_colors(r)?,
        &widget_colors(r)?,
        r.is_dark(),
        r.context.reduced_transparency,
    );
    button_states(&mut tc);
    let (hover, pressed) = flat_button_states(r)?;
    tc.secondary_hover = hover;
    tc.secondary_active = pressed;
    Ok(tc)
}

/// The flat button's hovered and pressed fills, `secondary_hover` and
/// `secondary_active`, which gpui-component paints the controls unfilled at
/// rest in: the ghost button (`variants::ghost_button`; gpui-component
/// 0.10.0 `button/button.rs:984, 987, 1346`), the title bar's buttons
/// (`title_bar.rs:184, 193`), the Outline tab (`tab/tab.rs:234`), the
/// calendar's days (`time/calendar.rs:184`), the stepper's trigger
/// (`stepper/trigger.rs:130-131`), the dock's tab panel
/// (`dock/tab_panel.rs:545-546`) and a pressed table cell
/// (`table/state.rs:1916`). Each is `button.body(<state>; variant=Flat).fill`
/// where a rule states it for that state alone (spec §2, `from_state`),
/// else gpui-component's own (spec §10, rule 0). A rule that names no variant
/// is the flat button's too (spec §2), so a theme that states no flat rule
/// gives it the push button's stated fills.
fn flat_button_states(r: &ResolvedTheme) -> Mapped<(Hsla, Hsla)> {
    let b = &r.button;
    let upstream = upstream_colors(r);
    let flat = ButtonQuery::flat();
    Ok((
        state_fill_or(
            &b.body(flat.state(WidgetState::HOVER)),
            "button.body(HOVER; variant=Flat).fill",
            upstream.secondary_hover,
        )?,
        state_fill_or(
            &b.body(flat.state(WidgetState::PRESSED)),
            "button.body(PRESSED; variant=Flat).fill",
            upstream.secondary_active,
        )?,
    ))
}

/// The filled buttons' hovered and pressed slots, `button_hover`,
/// `button_active` and their `button_secondary_*` pair: the push button's
/// own fill of the state, which the read gives `secondary_hover` and
/// `secondary_active` before [`flat_button_states`] replaces them. The 0.8.0
/// model states each state's whole fill, any state layer pre-baked into it
/// (spec §9, step 2), and gpui-component paints it in place of the rest fill,
/// over what lies under the button as the rest fill is; the old mapping
/// composites the 0.7.2 model's state layer over the rest fill instead
/// (`colors::assign_buttons`, spec §3.2, C17), which drew a hovered button
/// whose hover is its translucent rest fill darker than at rest.
fn button_states(tc: &mut ThemeColor) {
    tc.button_hover = tc.secondary_hover;
    tc.button_active = tc.secondary_active;
    tc.button_secondary_hover = tc.secondary_hover;
    tc.button_secondary_active = tc.secondary_active;
}

/// The global theme's values beside its colours. The sizes are the model's,
/// which its text scale has already scaled. The window's corner radius is
/// its frame's; the caret's blink, `None` where the theme states none, the
/// caret's; the highlighter style fills the code editor from the text
/// field's fill, over the window's.
pub(crate) fn theme_scalars(r: &ResolvedTheme) -> Mapped<ThemeScalars> {
    let frame = r.window.frame(normal_window());
    let t = &r.typography;
    let body = &t.body;
    let mono = &t.monospace;
    if !body.stated.contains(TextStated::FAMILY) {
        return Err(Unmapped("typography.body.family"));
    }
    if !mono.stated.contains(TextStated::FAMILY) {
        return Err(Unmapped("typography.monospace.family"));
    }
    Ok(ThemeScalars {
        is_dark: r.is_dark(),
        font_family: font_family(&body.family),
        font_size: text_size(body, "typography.body.size_px")?,
        mono_font_family: font_family(&mono.family),
        mono_font_size: text_size(mono, "typography.monospace.size_px")?,
        radius: r.layout.corner_radius,
        radius_lg: r.layout.corner_radius_lg,
        shadow: r.layout.shadow_enabled,
        focus_band: focus_band_of(r)?,
        overlay_scrollbars: r.scrollbar.overlay_mode,
        window_radius: radius(&frame, "window.frame(normal).corners")?,
        cursor_blink: caret_blink_of(
            r.caret.blink_on_ms,
            r.caret.blink_off_ms,
            r.caret.blink_timeout_ms,
        ),
        highlight: highlight_style_of(
            fill(&r.input.field(WidgetState::NONE), "input.field(NONE).fill")?,
            fill(&frame, "window.frame(normal).fill")?,
            theme_mode(r),
        ),
    })
}

/// gpui-component's mode of the theme's scheme.
fn theme_mode(r: &ResolvedTheme) -> ThemeMode {
    if r.is_dark() {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

/// The gpui-component theme of `r`, as [`crate::to_theme`] builds it from a
/// 0.7.2 resolved theme.
///
/// It sets all 140 `ThemeColor` fields (and the `tokens` gpui-component
/// derives from them), `mode` (the theme's scheme), `font_family`,
/// `font_size`, `mono_font_family`, `mono_font_size`, `radius`, `radius_lg`,
/// `window_radius` (the window frame's corners), `shadow`, `focus_ring`,
/// `scrollbar_mode`, `highlight_theme` and, where the theme states a caret
/// blink, `cursor_blink`, and stores a `ThemeConfig` of the theme's name and
/// the same values in the slot of its mode, for gpui-component's own mode
/// switches; the other mode's slot stays `ThemeConfig::default()`. The sizes
/// are the model's, which the theme's text scale has already scaled, and the
/// colours follow the context's reduced transparency. `transparent`,
/// `notification`, `list`, `sheet` and `motion` stay gpui-component's
/// defaults, as [`crate::to_theme`] leaves them, and so does `cursor_blink`
/// where the theme states no blink.
///
/// # Errors
///
/// [`Unmapped`] where the theme leaves a value the theme needs unstated, or
/// states it in a form gpui cannot take.
pub fn to_theme(r: &ResolvedTheme) -> Mapped<GpuiTheme> {
    let tc = theme_color(r)?;
    let scalars = theme_scalars(r)?;
    let mut theme = GpuiTheme::from(&tc);
    apply_scalars(&mut theme, &scalars);
    let mut config = ThemeConfig::default();
    config.name = SharedString::from(r.name.to_string());
    config.mode = theme.mode;
    apply_config_scalars(&mut config, &scalars);
    config.colors = theme_color_to_config_colors(&tc);
    if r.is_dark() {
        theme.dark_theme = Rc::new(config);
    } else {
        theme.light_theme = Rc::new(config);
    }
    Ok(theme)
}

// ---------------------------------------------------------------------------
// Installation
// ---------------------------------------------------------------------------

/// What [`apply`] installs from a 0.8.0 resolved theme beside the styled
/// theme, computed when it stores the theme in [`crate::NativeTheme`], for
/// the connector's observer to restore after an upstream rebuild: what
/// [`crate::apply`] computes from a stored 0.7.2 theme at each restore.
pub(crate) struct Installed {
    pub(crate) resolved: ResolvedTheme,
    /// The `ThemeConfig` [`to_theme`] stores in the slot of the theme's mode.
    pub(crate) config: Rc<ThemeConfig>,
    pub(crate) scrollbar: ScrollbarGeometry,
    pub(crate) resizable: ResizableTheme,
    /// The styled theme's colours, whose 12 base-palette fields the observer
    /// restores.
    pub(crate) palette: ThemeColor,
    pub(crate) capped: CappedNatives,
    pub(crate) caret_blink: Option<CursorBlink>,
    pub(crate) tooltip_delay: Option<Duration>,
    /// Whether the platform shows its focus ring for keyboard focus only,
    /// and the ring has a width.
    pub(crate) keyboard_focus_band: bool,
}

impl fmt::Debug for Installed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Installed")
            .field("resolved", &self.resolved)
            .finish_non_exhaustive()
    }
}

impl Installed {
    /// What [`apply`] installs from `r`, whose styled theme is `theme`
    /// ([`to_theme`] of `r`).
    fn of(r: &ResolvedTheme, theme: &GpuiTheme) -> Mapped<Self> {
        let config = if r.is_dark() {
            theme.dark_theme.clone()
        } else {
            theme.light_theme.clone()
        };
        Ok(Self {
            resolved: r.clone(),
            config,
            scrollbar: scrollbar_geometry(r)?,
            resizable: resizable_theme(r)?,
            palette: theme.colors,
            capped: capped_natives_of(
                widget_colors(r)?.selection,
                slice_colors(r)?.list_selection_bg,
            ),
            caret_blink: caret_blink_of(
                r.caret.blink_on_ms,
                r.caret.blink_off_ms,
                r.caret.blink_timeout_ms,
            ),
            tooltip_delay: tooltip_show_delay(r),
            keyboard_focus_band: r.focus_ring.trigger == Some(FocusTrigger::KeyboardOnly)
                && focus_ring_width(r)? > 0.0,
        })
    }
}

/// The accessibility preferences a 0.8.0 theme was resolved for, as 0.7.2's
/// (docs/v0.8.0-spec.md §5, the table of homes: `reduce_motion` is a motion
/// other than `Full`, `high_contrast` a contrast other than `Normal`).
fn accessibility_of(r: &ResolvedTheme) -> AccessibilityPreferences {
    let ctx = &r.context;
    let mut prefs = AccessibilityPreferences::DEFAULT.clone();
    prefs.text_scaling_factor = ctx.text_scale;
    prefs.reduce_motion = ctx.motion != MotionPreference::Full;
    prefs.high_contrast = ctx.contrast != Contrast::Normal;
    prefs.reduce_transparency = ctx.reduced_transparency;
    prefs
}

/// [`crate::apply`] for a 0.8.0 resolved theme: installs [`to_theme`] of `r`
/// as gpui-component's global theme with the native base-layer overrides,
/// and keeps them installed across upstream rebuilds, in the ten steps
/// [`crate::apply`] lists, each from `r` where [`crate::apply`] reads its
/// 0.7.2 theme.
///
/// The theme carries the context it was resolved for, so no accessibility
/// preferences are passed: reduced motion is requested where its motion is
/// other than `Full`, and [`crate::NativeTheme::accessibility`] holds the
/// context as 0.7.2's preferences. Text scaling and reduced transparency are
/// in the resolved values already; to follow a change of the context,
/// resolve the theme again and call `apply` with it.
/// [`crate::apply_accessibility`] stores the preferences it is given and
/// forwards their reduced motion, but rebuilds no 0.8.0 theme.
/// [`crate::NativeTheme::next_resolved`] returns the stored theme, and
/// [`crate::NativeTheme::resolved`] and [`crate::ActiveNativeTheme::native`],
/// which give a 0.7.2 theme, return `None` for it.
///
/// # Errors
///
/// [`Unmapped`] where the theme leaves a value the connector installs
/// unstated, or states it in a form gpui cannot take; nothing is installed
/// then.
pub fn apply(r: &ResolvedTheme, cx: &mut App) -> Mapped<()> {
    let theme = to_theme(r)?;
    let installed = crate::Variant::Next(Box::new(Installed::of(r, &theme)?));
    let (light, dark) = if r.is_dark() {
        (None, Some(installed))
    } else {
        (Some(installed), None)
    };
    crate::apply_inner(theme, light, dark, &accessibility_of(r), cx);
    Ok(())
}

// ---------------------------------------------------------------------------
// The crate root's accessors, each read at its home in the table of homes
// ---------------------------------------------------------------------------

/// [`crate::is_dark_resolved`]: whether the window's background lightness is
/// below 0.5 (`colors.window.background`). [`ResolvedTheme::is_dark`] gives
/// the scheme the theme was resolved for.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window background.
pub fn is_dark_resolved(r: &ResolvedTheme) -> Mapped<bool> {
    let background = stated(r.colors.window.background, "colors.window.background")?;
    Ok(hsla(background).l < 0.5)
}

/// [`crate::border_line_width`]: the border width (`layout.border_width`).
#[must_use]
pub fn border_line_width(r: &ResolvedTheme) -> f32 {
    r.layout.border_width
}

/// [`crate::disabled_opacity`]: the whole widget's fade on top of its
/// disabled colours (`colors.disabled_opacity`); 1.0 where the platform dims
/// by colour alone.
#[must_use]
pub fn disabled_opacity(r: &ResolvedTheme) -> f32 {
    r.colors.disabled_opacity
}

/// [`crate::border_opacity`]: the share of the text colour already folded
/// into every stated border colour (`layout.border_opacity`).
#[must_use]
pub fn border_opacity(r: &ResolvedTheme) -> f32 {
    r.layout.border_opacity
}

/// [`crate::border_shadow_enabled`]: whether drop shadows are enabled
/// (`layout.shadow_enabled`).
#[must_use]
pub fn border_shadow_enabled(r: &ResolvedTheme) -> bool {
    r.layout.shadow_enabled
}

/// [`crate::icon_sizes`]: the per-context icon sizes (`icons.sizes`), each
/// `None` where the theme states none.
#[must_use]
pub fn icon_sizes(r: &ResolvedTheme) -> &IconSizes {
    &r.icons.sizes
}

/// The four text roles of 0.7.2's `text_scale`, each the typography's style
/// at its home, or the body's where the theme defines no such role (the
/// table of homes, rows `text_scale.*`, `home_fallback`).
#[derive(Clone, Copy, Debug)]
pub struct TextScale<'a> {
    /// `typography.caption`.
    pub caption: &'a TextStyle,
    /// `typography.headline`.
    pub section_heading: &'a TextStyle,
    /// `typography.title1`.
    pub dialog_title: &'a TextStyle,
    /// `typography.display`.
    pub display: &'a TextStyle,
}

/// [`crate::text_scale`]: the four text roles 0.7.2 names.
#[must_use]
pub fn text_scale(r: &ResolvedTheme) -> TextScale<'_> {
    let t = &r.typography;
    TextScale {
        caption: t.caption.as_ref().unwrap_or(&t.body),
        section_heading: t.headline.as_ref().unwrap_or(&t.body),
        dialog_title: t.title1.as_ref().unwrap_or(&t.body),
        display: t.display.as_ref().unwrap_or(&t.body),
    }
}

/// [`crate::line_height`]: the line height multiplier
/// (`typography.line_height_multiplier`).
#[must_use]
pub fn line_height(r: &ResolvedTheme) -> f32 {
    r.typography.line_height_multiplier
}

/// A text style's weight, where stated.
fn text_weight(text: &TextStyle, home: &'static str) -> Mapped<u16> {
    if text.stated.contains(TextStated::WEIGHT) {
        Ok(text.weight)
    } else {
        Err(Unmapped(home))
    }
}

/// [`crate::font_weight`]: the body text's CSS weight
/// (`typography.body.weight`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no body weight.
pub fn font_weight(r: &ResolvedTheme) -> Mapped<u16> {
    text_weight(&r.typography.body, "typography.body.weight")
}

/// Where the theme gives no body text a label can take its size and weight
/// from, the home it leaves unstated, `typography.body.size_px` or
/// `typography.body.weight`; `None` where it states both. The check a
/// theme-drawn widget's body-text label passes (`widgets`,
/// `ThemeSource::body_text`).
#[must_use]
pub fn body_text_unmapped(r: &ResolvedTheme) -> Option<Unmapped> {
    if !r.typography.body.stated.contains(TextStated::SIZE_PX) {
        return Some(Unmapped("typography.body.size_px"));
    }
    font_weight(r).err()
}

/// [`crate::mono_font_weight`]: the monospace text's CSS weight
/// (`typography.monospace.weight`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no monospace weight.
pub fn mono_font_weight(r: &ResolvedTheme) -> Mapped<u16> {
    text_weight(&r.typography.monospace, "typography.monospace.weight")
}

/// [`crate::dialog_button_order`]: which end of a dialog's button row its
/// primary button sits at (`dialog.button_order`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no order.
pub fn dialog_button_order(r: &ResolvedTheme) -> Mapped<DialogButtonOrder> {
    stated(r.dialog.button_order, "dialog.button_order")
}

/// [`crate::dialog_button_gap`]: the gap between a dialog's buttons, in
/// logical pixels (`dialog.button_gap`).
#[must_use]
pub fn dialog_button_gap(r: &ResolvedTheme) -> f32 {
    r.dialog.button_gap
}

/// [`crate::scrollbar_groove_width`]: the vertical scroll bar's groove width,
/// in logical pixels (`scrollbar.track(NONE; orientation=Vertical)
/// .min_size[0]`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no groove width.
pub fn scrollbar_groove_width(r: &ResolvedTheme) -> Mapped<f32> {
    let vertical = StyleQuery::<()>::new(WidgetState::NONE).orientation(Orientation::Vertical);
    stated(
        r.scrollbar.track(vertical).min_size[0],
        "scrollbar.track(NONE; orientation=Vertical).min_size[0]",
    )
}

/// [`crate::selection_text_color`]: the text colour of selected content
/// (`colors.selection.foreground`).
#[must_use]
pub fn selection_text_color(r: &ResolvedTheme) -> Color {
    r.colors.selection.foreground
}

/// [`crate::selection_inactive_background`]: the selection's fill in a view
/// without the focus (`colors.selection.unfocused_background`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states none.
pub fn selection_inactive_background(r: &ResolvedTheme) -> Mapped<Color> {
    stated(
        r.colors.selection.unfocused_background,
        "colors.selection.unfocused_background",
    )
}

/// [`crate::selection_inactive_text_color`]: the text colour over the
/// selection of a view without the focus
/// (`colors.selection.unfocused_foreground`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states none.
pub fn selection_inactive_text_color(r: &ResolvedTheme) -> Mapped<Color> {
    stated(
        r.colors.selection.unfocused_foreground,
        "colors.selection.unfocused_foreground",
    )
}

/// A part's fill where a rule states it for the queried state, as a colour
/// of the model; `None` where none does. A fill stated as none is
/// transparent, and a layered paint gives its fallback colour (spec §10,
/// rule 2).
fn state_fill_color(part: &PartStyle, home: &'static str) -> Mapped<Option<Color>> {
    if !part.from_state.contains(PartStated::FILL) {
        return Ok(None);
    }
    fill_color_of(part).map(Some).ok_or(Unmapped(home))
}

/// [`crate::list_header_hover_background`]: the fill of a hovered
/// column-header section, where a rule states one (`list.header(HOVER)
/// .fill`); `None` where none does.
///
/// # Errors
///
/// [`Unmapped`] where the fill is stated as a layered paint with no
/// fallback colour.
pub fn list_header_hover_background(r: &ResolvedTheme) -> Mapped<Option<Color>> {
    state_fill_color(
        &r.list.header(WidgetState::HOVER),
        "list.header(HOVER).fill",
    )
}

/// [`crate::list_header_active_background`]: the fill of a pressed
/// column-header section, where a rule states one (`list.header(PRESSED)
/// .fill`); `None` where none does.
///
/// # Errors
///
/// [`Unmapped`] where the fill is stated as a layered paint with no
/// fallback colour.
pub fn list_header_active_background(r: &ResolvedTheme) -> Mapped<Option<Color>> {
    state_fill_color(
        &r.list.header(WidgetState::PRESSED),
        "list.header(PRESSED).fill",
    )
}

/// [`crate::disabled_text_color`]: the text colour of disabled elements
/// (`colors.disabled_text`).
#[must_use]
pub fn disabled_text_color(r: &ResolvedTheme) -> Color {
    r.colors.disabled_text
}

/// [`crate::focus_ring_width`]: the focus ring's stroke width, in logical
/// pixels (`focus_ring.stroke.width`); 0 where the ring has no stroke.
///
/// # Errors
///
/// [`Unmapped`] where the ring's stroke is per side.
pub fn focus_ring_width(r: &ResolvedTheme) -> Mapped<f32> {
    match &r.focus_ring.stroke {
        Stroke::None => Ok(0.0),
        Stroke::Uniform { width, .. } => Ok(*width),
        _ => Err(Unmapped("focus_ring.stroke.width")),
    }
}

/// [`crate::focus_ring_offset`]: the gap between an element's edge and its
/// focus ring (`focus_ring.offset`).
#[must_use]
pub fn focus_ring_offset(r: &ResolvedTheme) -> f32 {
    r.focus_ring.offset
}

/// [`crate::tooltip_show_delay`]: the delay between the pointer coming to
/// rest on an element and its tooltip showing (`timings.tooltip_show_ms`);
/// `None` where the theme states none.
#[must_use]
pub fn tooltip_show_delay(r: &ResolvedTheme) -> Option<Duration> {
    r.timings
        .tooltip_show_ms
        .map(|ms| Duration::from_millis(u64::from(ms)))
}

/// The scroll bar's values for gpui-base.
pub(crate) fn scrollbar_geometry(r: &ResolvedTheme) -> Mapped<ScrollbarGeometry> {
    let s = &r.scrollbar;
    let vertical = StyleQuery::<()>::new(WidgetState::NONE).orientation(Orientation::Vertical);
    Ok(geometry_of(&ScrollbarSource {
        groove_width: stated(
            s.track(vertical).min_size[0],
            "scrollbar.track(NONE; orientation=Vertical).min_size[0]",
        )?,
        thumb_width: stated(
            s.thumb(vertical).min_size[0],
            "scrollbar.thumb(NONE; orientation=Vertical).min_size[0]",
        )?,
        corner_radius: r.layout.corner_radius,
        min_thumb_length: s.min_thumb_length,
        track: fill(&s.track(WidgetState::NONE), "scrollbar.track(NONE).fill")?,
        border: hsla(stated(r.colors.window.border, "colors.window.border")?),
        thumb: thumb_at_rest(r)?,
        thumb_hover: fill(&s.thumb(WidgetState::HOVER), "scrollbar.thumb(HOVER).fill")?,
        thumb_active: state_fill(
            &s.thumb(WidgetState::PRESSED),
            "scrollbar.thumb(PRESSED).fill",
        )?,
    }))
}

/// The resize handles' colours for gpui-base: the splitter's handle at rest
/// and under the pointer, whose fill a theme without a rule for `HOVER`
/// keeps from rest.
pub(crate) fn resizable_theme(r: &ResolvedTheme) -> Mapped<ResizableTheme> {
    let s = &r.splitter;
    Ok(resizable_theme_of(
        fill(&s.handle(WidgetState::NONE), "splitter.handle(NONE).fill")?,
        fill(&s.handle(WidgetState::HOVER), "splitter.handle(HOVER).fill")?,
    ))
}

#[cfg(test)]
mod tests {
    use gpui::{StyleRefinement, Styled as _};
    use native_theme::next::StrokeAlign;

    use super::*;

    const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

    /// A part whose fill is `c`, stated, and from a state rule where
    /// `for_state`.
    fn filled(c: Color, for_state: bool) -> PartStyle {
        let mut part = PartStyle::default();
        part.fill = Paint::Solid(c);
        part.stated = PartStated::FILL;
        if for_state {
            part.from_state = PartStated::FILL;
        }
        part
    }

    /// Spec §10 rule 2, for every fill read: a fill stated as none is
    /// transparent, a layered paint its fallback colour -- the top opaque
    /// layer's -- and one with no colour to fall back on (gradients only) is
    /// the home lacked; an unstated fill is none of these.
    #[test]
    fn a_none_fill_is_transparent_and_a_layered_one_its_fallback() -> Mapped<()> {
        use native_theme::next::{BlendMode, PaintLayer};

        let painted = |paint: Paint, for_state: bool| {
            let mut part = filled(RED, for_state);
            part.fill = paint;
            part
        };
        let solid = |color| PaintLayer::Solid {
            color,
            blend: BlendMode::Normal,
        };
        let layered = Paint::layers(vec![solid(GREEN), solid(BLUE)]);
        let gradient = Paint::layers(vec![PaintLayer::LinearGradient {
            angle_deg: 90.0,
            stops: Vec::new().into(),
            blend: BlendMode::Normal,
        }]);
        let transparent = hsla(Color::TRANSPARENT);

        assert_eq!(fill(&painted(Paint::None, false), "none")?, transparent);
        assert_eq!(
            fill(&painted(layered.clone(), false), "layered")?,
            hsla(BLUE)
        );
        assert_eq!(
            fill(&painted(gradient.clone(), false), "gradient"),
            Err(Unmapped("gradient"))
        );
        assert_eq!(
            state_fill(&painted(Paint::None, true), "none")?,
            Some(transparent)
        );
        assert_eq!(
            state_fill(&painted(layered.clone(), true), "layered")?,
            Some(hsla(BLUE))
        );
        assert_eq!(
            state_fill(&painted(gradient, true), "gradient"),
            Err(Unmapped("gradient"))
        );
        assert_eq!(state_fill(&painted(Paint::None, false), "none")?, None);
        assert_eq!(
            stated_fill(&painted(layered.clone(), false), "layered")?,
            Some(hsla(BLUE))
        );
        assert_eq!(
            state_fill_color(&painted(Paint::None, true), "none")?,
            Some(Color::TRANSPARENT)
        );
        assert_eq!(
            state_fill_color(&painted(layered, true), "layered")?,
            Some(BLUE)
        );
        let mut unstated = painted(Paint::None, false);
        unstated.stated = PartStated::default();
        assert_eq!(fill_of(&unstated), None);
        assert_eq!(stated_fill(&unstated, "unstated")?, None);
        Ok(())
    }

    /// `button.active_background`: the pressed body's fill without a rule
    /// for the state is unset, so [`button_pressed_fill`] takes
    /// gpui-component's own treatment.
    #[test]
    fn a_pressed_button_fill_without_a_state_rule_is_unset() -> Mapped<()> {
        assert_eq!(state_fill(&filled(RED, false), "pressed")?, None);
        assert_eq!(state_fill(&filled(RED, true), "pressed")?, Some(hsla(RED)));
        Ok(())
    }

    /// `scrollbar.thumb_active_color`: without a rule for the state the
    /// pressed thumb is the hovered one, 0.7.2's fallback.
    #[test]
    fn a_pressed_thumb_without_a_state_rule_falls_back_to_the_hover_colour() -> Mapped<()> {
        let source = |pressed: &PartStyle| -> Mapped<ScrollbarSource> {
            Ok(ScrollbarSource {
                groove_width: 10.0,
                thumb_width: 6.0,
                corner_radius: 3.0,
                min_thumb_length: 20.0,
                track: hsla(BLUE),
                border: hsla(BLUE),
                thumb: hsla(BLUE),
                thumb_hover: hsla(GREEN),
                thumb_active: state_fill(pressed, "scrollbar.thumb(PRESSED).fill")?,
            })
        };
        let unset = geometry_of(&source(&filled(RED, false))?);
        assert_eq!(unset.thumb_active, hsla(GREEN));
        let stated = geometry_of(&source(&filled(RED, true))?);
        assert_eq!(stated.thumb_active, hsla(RED));
        Ok(())
    }

    /// `button.checked_text_color` and `button.checked_background`: without
    /// a rule for `CHECKED` the pressed pair stands in.
    #[test]
    fn a_checked_pair_without_a_state_rule_is_the_pressed_pair() -> Mapped<()> {
        let mut label = TextStyle::default();
        label.color = RED;
        label.stated = TextStated::COLOR;
        assert_eq!(state_text_color(&label, "checked label")?, None);
        let checked = crate::geometry::button_checked_from(
            state_text_color(&label, "checked label")?,
            hsla(GREEN),
            state_fill(&filled(RED, false), "checked body")?,
            state_fill(&filled(BLUE, true), "pressed body")?,
        );
        assert_eq!(
            checked,
            StyleRefinement::default()
                .text_color(hsla(GREEN))
                .bg(hsla(BLUE))
        );
        label.from_state = TextStated::COLOR;
        assert_eq!(state_text_color(&label, "checked label")?, Some(hsla(RED)));
        Ok(())
    }

    /// A part stroked in `color`, `width` wide, aligned `align`, stated.
    fn stroked(width: f32, color: Color, align: StrokeAlign) -> PartStyle {
        let mut part = PartStyle::default();
        part.stroke = Stroke::uniform_aligned(width, color, align);
        part.stated = PartStated::STROKE;
        part
    }

    /// A stroke inside the fill's edge: gpui's own extent, the stated
    /// colour and width, opaque or translucent.
    #[test]
    fn an_inside_stroke_is_drawn_as_stated() -> Mapped<()> {
        for color in [RED, RED.with_alpha(0.25)] {
            let part = stroked(1.0, color, StrokeAlign::Inside);
            assert_eq!(stroke_color(&part, "inside")?, hsla(color));
            assert_eq!(stroke_width(&part, "inside")?, 1.0);
        }
        Ok(())
    }

    /// A stroke centred on the fill's edge: exact from the stated colour
    /// where opaque; translucent, the stated colour over gpui's whole fill,
    /// the limit the README records.
    #[test]
    fn a_centred_stroke_is_drawn_in_its_stated_colour() -> Mapped<()> {
        for color in [GREEN, GREEN.with_alpha(0.2)] {
            let part = stroked(1.001, color, StrokeAlign::Center);
            assert_eq!(stroke_color(&part, "center")?, hsla(color));
            assert_eq!(stroke_width(&part, "center")?, 1.001);
        }
        Ok(())
    }

    /// A stroke around the fill: as the centred one.
    #[test]
    fn an_outside_stroke_is_drawn_in_its_stated_colour() -> Mapped<()> {
        for color in [BLUE, BLUE.with_alpha(0.5)] {
            let part = stroked(2.0, color, StrokeAlign::Outside);
            assert_eq!(stroke_color(&part, "outside")?, hsla(color));
            assert_eq!(stroke_width(&part, "outside")?, 2.0);
        }
        Ok(())
    }

    /// An unstated stroke stays unmapped in every alignment, and a stated
    /// absent one is no width, its colour transparent.
    #[test]
    fn an_unstated_stroke_is_unmapped_in_every_alignment() {
        for align in [
            StrokeAlign::Inside,
            StrokeAlign::Center,
            StrokeAlign::Outside,
        ] {
            let mut part = stroked(1.0, RED, align);
            part.stated = PartStated::default();
            assert_eq!(stroke_color(&part, "home"), Err(Unmapped("home")));
            assert_eq!(stroke_width(&part, "home"), Err(Unmapped("home")));
        }
        let mut none = PartStyle::default();
        none.stated = PartStated::STROKE;
        assert_eq!(stroke_width(&none, "home"), Ok(0.0));
        assert_eq!(stroke_color(&none, "home"), Ok(hsla(Color::TRANSPARENT)));
    }

    /// Spec §10 rule 2, for every stroke colour a builder or a colour token
    /// reads: a stroke stated as none is transparent, no line; one stroke its
    /// colour; an unstated stroke, or one stated per side, the home lacked.
    #[test]
    fn a_none_stroke_is_transparent() -> Mapped<()> {
        let mut none = PartStyle::default();
        none.stated = PartStated::STROKE;
        assert_eq!(stroke_color(&none, "home")?, hsla(Color::TRANSPARENT));
        assert_eq!(stroke_color_of(&none), None);
        assert_eq!(state_stroke_color(&none, "home")?, None);
        assert_eq!(state_stroke_color_of(&none), None);
        none.from_state = PartStated::STROKE;
        assert_eq!(
            state_stroke_color(&none, "home")?,
            Some(hsla(Color::TRANSPARENT))
        );
        assert_eq!(state_stroke_color_of(&none), Some(hsla(Color::TRANSPARENT)));
        let part = stroked(1.0, RED, StrokeAlign::Inside);
        assert_eq!(stroke_color(&part, "home")?, hsla(RED));
        let mut unstated = part.clone();
        unstated.stated = PartStated::default();
        assert_eq!(stroke_color(&unstated, "home"), Err(Unmapped("home")));
        Ok(())
    }

    /// A theme that states the field's and the sidebar's strokes as none
    /// installs: gpui-component's `input` and `sidebar_border` tokens are
    /// transparent, where they took kde-breeze's stated lines before the
    /// patch.
    #[test]
    fn a_theme_stroking_its_field_as_none_maps_whole() -> Mapped<()> {
        use native_theme::next::{
            InputRule, PartPatch, Preset, Scheme, SidebarRule, Theme, ThemeContext, ThemePatch,
        };
        let mut r = Theme::preset(Preset::KdeBreeze)
            .resolve(&ThemeContext::default().with_scheme(Scheme::Light));
        let before = widget_colors(&r)?;
        let clear = hsla(Color::TRANSPARENT);
        assert!(
            before.input != clear && before.sidebar_border != clear,
            "kde-breeze light's field or sidebar is edged in no colour, so the test could not tell the patch's effect"
        );
        r.apply_patch(
            &ThemePatch::new()
                .input(InputRule::new().field(PartPatch::new().stroke(Stroke::None)))
                .sidebar(SidebarRule::new().container(PartPatch::new().stroke(Stroke::None))),
        );
        let after = widget_colors(&r)?;
        assert_eq!(after.input, clear);
        assert_eq!(after.sidebar_border, clear);
        to_theme(&r).map(|_| ())
    }

    /// `button.hover_border_color`: without a rule for `HOVER` it is unset,
    /// and no bordered button is built.
    #[test]
    fn a_hover_stroke_without_a_state_rule_is_unset() -> Mapped<()> {
        let mut hovered = PartStyle::default();
        hovered.stroke = Stroke::uniform(1.0, RED);
        hovered.stated = PartStated::STROKE;
        assert_eq!(state_stroke_color(&hovered, "hover")?, None);
        hovered.from_state = PartStated::STROKE;
        assert_eq!(state_stroke_color(&hovered, "hover")?, Some(hsla(RED)));
        Ok(())
    }

    /// Spec §9, step 2, on the macOS presets, whose push button draws
    /// nothing on hover (the hovered fill is the rest fill): in both schemes
    /// the filled buttons' hovered slots are the rest fill and their pressed
    /// slots the stated pressed fill, translucent ones included, never the
    /// state's fill composited over the rest fill.
    #[test]
    fn a_hovered_macos_button_is_filled_as_at_rest() -> Mapped<()> {
        use native_theme::next::{Preset, Scheme, Theme, ThemeContext};
        let mut layered_would_differ = 0;
        for preset in [Preset::MacosMonterey, Preset::MacosTahoe] {
            for scheme in [Scheme::Light, Scheme::Dark] {
                let r = Theme::preset(preset).resolve(&ThemeContext::default().with_scheme(scheme));
                let b = &r.button;
                let rest = fill(&b.body(WidgetState::NONE), "button.body(NONE).fill")?;
                let pressed = state_fill(&b.body(WidgetState::PRESSED), "button.body(PRESSED)")?
                    .ok_or(Unmapped("button.body(PRESSED).fill, for the state"))?;
                if crate::colors::over(rest, rest) != rest {
                    layered_would_differ += 1;
                }
                let tc = theme_color(&r)?;
                assert_eq!(
                    [tc.button, tc.button_hover, tc.button_secondary_hover],
                    [rest; 3],
                    "{preset:?} {scheme:?}"
                );
                assert_eq!(
                    [tc.button_active, tc.button_secondary_active],
                    [pressed; 2],
                    "{preset:?} {scheme:?}"
                );
            }
        }
        assert!(
            layered_would_differ > 0,
            "no macOS scheme's rest fill is translucent, so the test cannot tell a whole fill from a layered one"
        );
        Ok(())
    }

    /// Spec §10 rule 0 for a hovered menu item: its fill and label where a
    /// rule states them for `HOVER`, else gpui-component's own `accent` and
    /// `accent_foreground`, not the item at rest; a patched theme's stated
    /// hover reaches `accent`.
    #[test]
    fn an_unstated_menu_hover_is_gpui_components_own() -> Mapped<()> {
        use native_theme::next::{
            MenuRule, PartPatch, Preset, Scheme, Theme, ThemeContext, ThemePatch,
        };
        let upstream = hsla(BLUE);
        assert_eq!(
            state_fill_or(&filled(GREEN, false), "item", upstream)?,
            upstream
        );
        assert_eq!(
            state_fill_or(&filled(GREEN, true), "item", upstream)?,
            hsla(GREEN)
        );
        let mut label = TextStyle::default();
        label.color = GREEN;
        label.stated = TextStated::COLOR;
        let unfilled = filled(GREEN, false);
        assert_eq!(
            menu_hover_label(&unfilled, &label, &label, upstream)?,
            upstream
        );
        label.from_state = TextStated::COLOR;
        assert_eq!(
            menu_hover_label(&unfilled, &label, &label, upstream)?,
            hsla(GREEN)
        );

        for scheme in [Scheme::Light, Scheme::Dark] {
            let mut r = Theme::preset(Preset::KdeBreeze)
                .resolve(&ThemeContext::default().with_scheme(scheme));
            r.apply_patch(
                &ThemePatch::new().menu(
                    MenuRule::new()
                        .states(WidgetState::HOVER)
                        .item(PartPatch::new().fill(Paint::Solid(RED))),
                ),
            );
            assert_eq!(widget_colors(&r)?.menu_hover_bg, hsla(RED), "{scheme:?}");
            assert_eq!(theme_color(&r)?.accent, hsla(RED), "{scheme:?}");
        }
        Ok(())
    }

    /// Spec §10 rule 0a for a hovered menu item's label: a label is chosen
    /// for the fill it lies on, so where a rule states the item's fill for
    /// `HOVER` and none its label, the label is the one at rest, not
    /// gpui-component's `accent_foreground`, which goes with its own `accent`
    /// fill; a stated label is taken as stated, and with neither stated the
    /// label is gpui-component's own.
    #[test]
    fn a_stated_menu_hover_fill_keeps_the_label_at_rest() -> Mapped<()> {
        let upstream = hsla(BLUE);
        let mut rest = TextStyle::default();
        rest.color = GREEN;
        rest.stated = TextStated::COLOR;
        let unstated = rest.clone();
        let mut stated = rest.clone();
        stated.color = RED;
        stated.from_state = TextStated::COLOR;
        let fill = filled(RED, true);
        let own = filled(RED, false);
        assert_eq!(
            menu_hover_label(&fill, &unstated, &rest, upstream)?,
            hsla(GREEN)
        );
        assert_eq!(
            menu_hover_label(&fill, &stated, &rest, upstream)?,
            hsla(RED)
        );
        assert_eq!(
            menu_hover_label(&own, &unstated, &rest, upstream)?,
            upstream
        );
        Ok(())
    }

    /// Spec §10 rule 0 for a hovered push button: its fill where a rule
    /// states it for `HOVER`, else gpui-component's own `secondary_hover`
    /// where its theme turns a button on hover, else the button at rest; a
    /// patched theme's stated hover reaches `secondary_hover` and the
    /// button's hovered slots.
    #[test]
    fn an_unstated_button_hover_is_gpui_components_own() -> Mapped<()> {
        use native_theme::next::{
            ButtonRule, PartPatch, Preset, Scheme, Theme, ThemeContext, ThemePatch,
        };
        let rest = hsla(GREEN);
        let mut turning = *ThemeColor::light();
        turning.secondary = hsla(RED);
        turning.secondary_hover = hsla(BLUE);
        let mut keeping = turning;
        keeping.secondary_hover = keeping.secondary;
        let unstated = filled(GREEN, false);
        assert_eq!(button_hover_fill(&unstated, rest, &turning)?, hsla(BLUE));
        assert_eq!(button_hover_fill(&unstated, rest, &keeping)?, rest);
        let stated = filled(RED, true);
        for upstream in [&turning, &keeping] {
            assert_eq!(button_hover_fill(&stated, rest, upstream)?, hsla(RED));
        }

        for scheme in [Scheme::Light, Scheme::Dark] {
            let mut r = Theme::preset(Preset::KdeBreeze)
                .resolve(&ThemeContext::default().with_scheme(scheme));
            r.apply_patch(
                &ThemePatch::new().button(
                    ButtonRule::new()
                        .states(WidgetState::HOVER)
                        .body(PartPatch::new().fill(Paint::Solid(BLUE))),
                ),
            );
            assert_eq!(slice_colors(&r)?.button_hover_bg, hsla(BLUE), "{scheme:?}");
            let tc = theme_color(&r)?;
            assert_eq!(tc.secondary_hover, hsla(BLUE), "{scheme:?}");
            assert_eq!(tc.button_hover, hsla(BLUE), "{scheme:?}");
        }
        Ok(())
    }

    /// Spec §10 rule 0 for a pressed push button: its fill where a rule
    /// states it for `PRESSED`, else gpui-component's own `secondary_active`
    /// where its theme turns a button pressed, else the button at rest, not
    /// the connector's darkening of the rest fill. Every preset states its
    /// push button's pressed fill, so none reaches the unstated case; on any
    /// that would, the pressed slots are gpui-component's own, and a patched
    /// theme's stated pressed fill reaches them.
    #[test]
    fn an_unstated_button_press_is_gpui_components_own() -> Mapped<()> {
        use native_theme::next::{
            ButtonRule, PartPatch, Preset, Scheme, Theme, ThemeContext, ThemePatch,
        };
        let rest = hsla(GREEN);
        let mut turning = *ThemeColor::light();
        turning.secondary = hsla(RED);
        turning.secondary_active = hsla(BLUE);
        let mut keeping = turning;
        keeping.secondary_active = keeping.secondary;
        let unstated = filled(GREEN, false);
        assert_eq!(
            button_pressed_fill(&unstated, rest, &turning)?,
            Some(hsla(BLUE))
        );
        assert_eq!(button_pressed_fill(&unstated, rest, &keeping)?, Some(rest));
        let stated = filled(RED, true);
        for upstream in [&turning, &keeping] {
            assert_eq!(
                button_pressed_fill(&stated, rest, upstream)?,
                Some(hsla(RED))
            );
        }

        for preset in Preset::ALL {
            for scheme in [Scheme::Light, Scheme::Dark] {
                let mut r =
                    Theme::preset(*preset).resolve(&ThemeContext::default().with_scheme(scheme));
                let b = &r.button;
                if state_fill(&b.body(WidgetState::PRESSED), "button.body(PRESSED).fill")?.is_some()
                {
                    continue;
                }
                let rest = fill(&b.body(WidgetState::NONE), "button.body(NONE).fill")?;
                let upstream = upstream_colors(&r);
                let own = if upstream.secondary_active == upstream.secondary {
                    rest
                } else {
                    upstream.secondary_active
                };
                let tc = theme_color(&r)?;
                assert_eq!(
                    [tc.button_active, tc.button_secondary_active],
                    [own; 2],
                    "{preset:?} {scheme:?}"
                );
                r.apply_patch(
                    &ThemePatch::new().button(
                        ButtonRule::new()
                            .states(WidgetState::PRESSED)
                            .body(PartPatch::new().fill(Paint::Solid(BLUE))),
                    ),
                );
                let tc = theme_color(&r)?;
                assert_eq!(
                    [tc.button_active, tc.button_secondary_active],
                    [hsla(BLUE); 2],
                    "{preset:?} {scheme:?}"
                );
            }
        }
        Ok(())
    }

    /// The controls unfilled at rest read the flat button's hovered and
    /// pressed fills, `secondary_hover` and `secondary_active`: a flat rule
    /// reaches them and not the push button's slots; where no rule states
    /// the flat button's state, gpui-component's own (spec §10, rule 0).
    #[test]
    fn the_unfilled_controls_take_the_flat_buttons_states() -> Mapped<()> {
        use native_theme::next::{
            ButtonRule, ButtonVariant, PartPatch, Preset, Scheme, Theme, ThemeContext, ThemePatch,
        };
        for preset in Preset::ALL {
            for scheme in [Scheme::Light, Scheme::Dark] {
                let mut r =
                    Theme::preset(*preset).resolve(&ThemeContext::default().with_scheme(scheme));
                let b = &r.button;
                let upstream = upstream_colors(&r);
                let flat = ButtonQuery::flat();
                let tc = theme_color(&r)?;
                for (state, own, slot) in [
                    (
                        WidgetState::HOVER,
                        upstream.secondary_hover,
                        tc.secondary_hover,
                    ),
                    (
                        WidgetState::PRESSED,
                        upstream.secondary_active,
                        tc.secondary_active,
                    ),
                ] {
                    let stated = state_fill(&b.body(flat.state(state)), "flat")?;
                    assert_eq!(
                        slot,
                        stated.unwrap_or(own),
                        "{preset:?} {scheme:?} {state:?}"
                    );
                }
                let push = [tc.button_hover, tc.button_active];
                r.apply_patch(
                    &ThemePatch::new()
                        .button(
                            ButtonRule::new()
                                .variant(ButtonVariant::Flat)
                                .states(WidgetState::HOVER)
                                .body(PartPatch::new().fill(Paint::Solid(BLUE))),
                        )
                        .button(
                            ButtonRule::new()
                                .variant(ButtonVariant::Flat)
                                .states(WidgetState::PRESSED)
                                .body(PartPatch::new().fill(Paint::Solid(RED))),
                        ),
                );
                let tc = theme_color(&r)?;
                assert_eq!(
                    [tc.secondary_hover, tc.secondary_active],
                    [hsla(BLUE), hsla(RED)],
                    "{preset:?} {scheme:?}"
                );
                assert_eq!(
                    [tc.button_hover, tc.button_active],
                    push,
                    "{preset:?} {scheme:?}"
                );
            }
        }
        Ok(())
    }

    /// A hovered fill a rule states, opaque or translucent, is the hovered
    /// button's whole fill, over macos-tahoe light's translucent rest fill.
    #[test]
    fn a_stated_hover_fill_is_the_hovered_buttons_whole_fill() -> Mapped<()> {
        use native_theme::next::{
            ButtonRule, PartPatch, Preset, Scheme, Theme, ThemeContext, ThemePatch,
        };
        let mut r = Theme::preset(Preset::MacosTahoe)
            .resolve(&ThemeContext::default().with_scheme(Scheme::Light));
        let rest = fill(&r.button.body(WidgetState::NONE), "button.body(NONE).fill")?;
        assert!(
            rest.a < 1.0,
            "macos-tahoe light's button is opaque: {rest:?}"
        );
        for hover in [RED, BLUE.with_alpha(0.5)] {
            r.apply_patch(
                &ThemePatch::new().button(
                    ButtonRule::new()
                        .states(WidgetState::HOVER)
                        .body(PartPatch::new().fill(Paint::Solid(hover))),
                ),
            );
            let tc = theme_color(&r)?;
            assert_eq!(tc.button, rest);
            assert_eq!(
                [tc.button_hover, tc.button_secondary_hover],
                [hsla(hover); 2],
                "{hover:?}"
            );
        }
        Ok(())
    }
}
