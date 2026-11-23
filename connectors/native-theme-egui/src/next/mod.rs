//! The connector on the 0.8.0 model ([`native_theme::next`]), beside the
//! 0.7.2 one until the 0.8.0 cut removes the latter: what the connector
//! hands to egui, computed from a [`native_theme::next::ResolvedTheme`].
//!
//! Each function here is the twin of the crate root's function of the same
//! name. It takes the 0.8.0 resolved theme, which is already scaled by the
//! text scale the theme was resolved for, so no preferences are passed; and
//! a value whose home the theme may leave unstated is returned as
//! [`Mapped`]: the value, or the [`Unmapped`] source it needs, where the
//! 0.7.2 function, whose fields are always set, returns the value alone.
//! Where the theme states nothing for a value egui has one of its own for,
//! egui's stays (spec §10, rule 0).
//!
//! Each function reads the model where the table of homes
//! (`native-theme/tests/next_homes.toml`) places the 0.7.2 field the old
//! function reads, by the query its row names. `root_parity` proves the
//! outputs equal to the old functions' (docs/v0.8.0-plan.md, section 1,
//! rule 2), and lists where they differ with the reason.
//!
//! The part reads ([`fill_of`], [`stroke_color_of`], ...) give a part's
//! stated values as egui takes them, `None` where the theme states none,
//! for a reader that shows egui's own there.

#[cfg_attr(
    not(test),
    expect(dead_code, reason = "`to_theme`, built next in M4.E2, calls it")
)]
mod build;
#[cfg(test)]
mod parity;
#[cfg(test)]
mod root_parity;
#[cfg(test)]
mod tests;

use std::fmt;
use std::time::Duration;

use egui::{Color32, FontId};
use native_theme::next::{
    CornerCurve, CornerRadius, DialogButtonOrder, FontStyle, Orientation, Paint, PartStated,
    PartStyle, ResolvedTheme, Stroke, StyleQuery, TextStated, TextStyle, WidgetState, WindowState,
};

use crate::accessors::text_size_or_stock;
use crate::{Role, TextRole};

/// A source a value needs that the theme leaves unstated, or states in a
/// form the connector cannot hand to egui (a gradient fill with no fallback
/// colour, a per-side stroke): its place in the 0.8.0 model, as a query and
/// a property (`typography.body.weight`). The 0.7.2 path has no such case,
/// its resolved fields being always set; the 0.8.0 path reports it rather
/// than inventing the value.
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
        write!(f, "the theme states no value egui can take at {}", self.0)
    }
}

impl std::error::Error for Unmapped {}

/// A value of the 0.8.0 mapping, or the source it lacks.
pub type Mapped<T> = Result<T, Unmapped>;

/// `v`, or `Unmapped(home)` where it is `None`.
pub(crate) fn stated<T>(v: Option<T>, home: &'static str) -> Mapped<T> {
    v.ok_or(Unmapped(home))
}

/// A colour of the model as egui's: its 8-bit sRGB channels with straight
/// alpha, premultiplied as [`crate::convert::to_color32`] premultiplies a
/// 0.7.2 colour.
#[must_use]
pub(crate) fn to_color32(c: native_theme::next::Color) -> Color32 {
    let [r, g, b, a] = c.to_srgb8();
    Color32::from_rgba_unmultiplied_const(r, g, b, a)
}

// ---------------------------------------------------------------------------
// A part's and a text style's stated values
// ---------------------------------------------------------------------------

/// A part's fill, where stated: its one colour, or, for a layered paint,
/// the colour a toolkit that paints one colour renders in its place
/// (spec §10, rule 2).
pub(crate) fn fill(part: &PartStyle, home: &'static str) -> Mapped<Color32> {
    stated(fill_of(part), home)
}

/// A part's fill as `fill` reads it, `None` where it is not stated, or
/// stated as a layered paint with no fallback colour: for a reader that
/// shows egui's own there (spec §10, rule 0). A fill stated as none is
/// transparent, and a layered paint gives its fallback colour (spec §10,
/// rule 2).
#[must_use]
pub fn fill_of(part: &PartStyle) -> Option<Color32> {
    if !part.stated.contains(PartStated::FILL) {
        return None;
    }
    match &part.fill {
        Paint::None => Some(Color32::TRANSPARENT),
        Paint::Solid(c) => Some(to_color32(*c)),
        paint @ Paint::Complex(_) => paint.fallback_color().map(to_color32),
    }
}

/// A part's fill where a rule states it for the queried state (its
/// `from_state` bit), as [`fill`] reads it; `None` where none does, a soft
/// option's unset value.
pub(crate) fn state_fill(part: &PartStyle, home: &'static str) -> Mapped<Option<Color32>> {
    if part.from_state.contains(PartStated::FILL) {
        fill(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A part's fill as `state_fill` reads it, `None` where no rule states it
/// for the queried state, or it is stated as a layered paint with no
/// fallback colour. The rule is for some flag of the query, not a given one
/// (spec §2, `from_state`).
#[must_use]
pub fn state_fill_of(part: &PartStyle) -> Option<Color32> {
    part.from_state
        .contains(PartStated::FILL)
        .then(|| fill_of(part))
        .flatten()
}

/// A part's stroke colour as stated, where stated as one stroke of one
/// colour, in any alignment; `None` where it is not stated, or stated in a
/// form egui cannot draw (a per-side stroke). This is the theme's colour;
/// where a translucent stroke lies inside or across the box's edge, the
/// connector hands egui it composited over the fill under it.
#[must_use]
pub fn stroke_color_of(part: &PartStyle) -> Option<Color32> {
    match &part.stroke {
        Stroke::Uniform { color, .. } if part.stated.contains(PartStated::STROKE) => {
            Some(to_color32(*color))
        }
        _ => None,
    }
}

/// A part's stroke colour as [`stroke_color_of`] reads it, `None` where no
/// rule states it for the queried state. The rule is for some flag of the
/// query, not a given one (spec §2, `from_state`).
#[must_use]
pub fn state_stroke_color_of(part: &PartStyle) -> Option<Color32> {
    part.from_state
        .contains(PartStated::STROKE)
        .then(|| stroke_color_of(part))
        .flatten()
}

/// A part's stroke width, where stated as one stroke, in any alignment, or
/// as none, which is no width; `None` where it is not stated, or stated in
/// a form egui cannot draw (a per-side stroke).
#[must_use]
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

/// A part's corner radius, where stated as one circular radius, or as
/// square corners, which are no radius; `None` where it is not stated, or
/// stated in a form egui cannot draw as one radius (a pill, per-corner
/// radii, a continuous curve).
#[must_use]
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

/// A part's padding, top, right, bottom, left, each side where stated.
#[must_use]
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
pub(crate) fn text_color(text: &TextStyle, home: &'static str) -> Mapped<Color32> {
    stated(text_color_of(text), home)
}

/// A text style's colour as `text_color` reads it, `None` where it is not
/// stated.
#[must_use]
pub fn text_color_of(text: &TextStyle) -> Option<Color32> {
    text.stated
        .contains(TextStated::COLOR)
        .then(|| to_color32(text.color))
}

/// A text style's colour as `text_color` reads it, `None` where no rule
/// states it for the queried state or it is not stated. The rule is for
/// some flag of the query, not a given one (spec §2, `from_state`).
#[must_use]
pub fn state_text_color_of(text: &TextStyle) -> Option<Color32> {
    text.from_state
        .contains(TextStated::COLOR)
        .then(|| text_color_of(text))
        .flatten()
}

/// A text style's weight, where stated.
fn text_weight(text: &TextStyle, home: &'static str) -> Mapped<u16> {
    stated(
        text.stated
            .contains(TextStated::WEIGHT)
            .then_some(text.weight),
        home,
    )
}

/// A text style's size in logical pixels, where stated.
fn text_size(text: &TextStyle, home: &'static str) -> Mapped<f32> {
    stated(
        text.stated
            .contains(TextStated::SIZE_PX)
            .then_some(text.size_px),
        home,
    )
}

// ---------------------------------------------------------------------------
// Colours egui has no slot for
// ---------------------------------------------------------------------------

/// [`crate::info_color`]: the info colour (`colors.info.fill`).
#[must_use]
pub fn info_color(r: &ResolvedTheme) -> Color32 {
    to_color32(r.colors.info.fill)
}

/// [`crate::info_text_color`]: the text colour on an info fill
/// (`colors.info.foreground`).
#[must_use]
pub fn info_text_color(r: &ResolvedTheme) -> Color32 {
    to_color32(r.colors.info.foreground)
}

/// [`crate::warning_text_color`]: the text colour on a warning fill
/// (`colors.warning.foreground`).
#[must_use]
pub fn warning_text_color(r: &ResolvedTheme) -> Color32 {
    to_color32(r.colors.warning.foreground)
}

/// [`crate::disabled_text_color`]: the text colour of disabled elements
/// (`colors.disabled_text`).
#[must_use]
pub fn disabled_text_color(r: &ResolvedTheme) -> Color32 {
    to_color32(r.colors.disabled_text)
}

/// [`crate::selection_inactive_background`]: the selection's fill in a view
/// without the focus (`colors.selection.unfocused_background`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states none.
pub fn selection_inactive_background(r: &ResolvedTheme) -> Mapped<Color32> {
    stated(
        r.colors.selection.unfocused_background,
        "colors.selection.unfocused_background",
    )
    .map(to_color32)
}

/// [`crate::selection_inactive_text_color`]: the selected text's colour in
/// a view without the focus (`colors.selection.unfocused_foreground`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states none.
pub fn selection_inactive_text_color(r: &ResolvedTheme) -> Mapped<Color32> {
    stated(
        r.colors.selection.unfocused_foreground,
        "colors.selection.unfocused_foreground",
    )
    .map(to_color32)
}

/// [`crate::toolbar_button_checked_background`]: the fill of a checked
/// toolbar button, where a rule states one (`toolbar.button(CHECKED)
/// .fill`); `None` where none does.
///
/// # Errors
///
/// [`Unmapped`] where the fill is stated as a layered paint with no
/// fallback colour.
pub fn toolbar_button_checked_background(r: &ResolvedTheme) -> Mapped<Option<Color32>> {
    state_fill(
        &r.toolbar.button(WidgetState::CHECKED),
        "toolbar.button(CHECKED).fill",
    )
}

/// [`crate::toolbar_button_checked_border_color`]: the stroke colour of a
/// checked toolbar button as stated, where a rule states one
/// (`toolbar.button(CHECKED).stroke.color`); `None` where none does.
///
/// # Errors
///
/// [`Unmapped`] where the stroke is stated in a form egui cannot draw (a
/// per-side stroke).
pub fn toolbar_button_checked_border_color(r: &ResolvedTheme) -> Mapped<Option<Color32>> {
    let part = r.toolbar.button(WidgetState::CHECKED);
    if !part.from_state.contains(PartStated::STROKE) {
        return Ok(None);
    }
    match part.stroke {
        Stroke::None => Ok(None),
        _ => stated(
            stroke_color_of(&part),
            "toolbar.button(CHECKED).stroke.color",
        )
        .map(Some),
    }
}

// ---------------------------------------------------------------------------
// The focus ring
// ---------------------------------------------------------------------------

/// [`crate::focus_ring_color`]: the focus indicator's colour
/// (`colors.focus_ring`).
#[must_use]
pub fn focus_ring_color(r: &ResolvedTheme) -> Color32 {
    to_color32(r.colors.focus_ring)
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
/// focus ring, in logical pixels (`focus_ring.offset`).
#[must_use]
pub fn focus_ring_offset(r: &ResolvedTheme) -> f32 {
    r.focus_ring.offset
}

// ---------------------------------------------------------------------------
// Typography
// ---------------------------------------------------------------------------

/// The typography's style of `role`, or the body's where the theme defines
/// no such style (the table of homes, rows `text_scale.*`,
/// `home_fallback`).
fn text_role_style(r: &ResolvedTheme, role: TextRole) -> &TextStyle {
    let t = &r.typography;
    let style = match role {
        TextRole::Caption => &t.caption,
        TextRole::SectionHeading => &t.headline,
        TextRole::DialogTitle => &t.title1,
        TextRole::Display => &t.display,
    };
    style.as_ref().unwrap_or(&t.body)
}

/// The egui text style whose stock size stands in for a degenerate size of
/// `role`.
fn text_role_slot(role: TextRole) -> egui::TextStyle {
    match role {
        TextRole::Caption => egui::TextStyle::Small,
        TextRole::SectionHeading | TextRole::DialogTitle | TextRole::Display => {
            egui::TextStyle::Heading
        }
    }
}

/// [`crate::text_role_font`]: the proportional font of `role` at its size
/// (`typography.caption`, `.headline`, `.title1`, `.display`, each the
/// body's where the theme defines no such style), egui's own size for the
/// role's slot where the size is not a positive normal number.
#[must_use]
pub fn text_role_font(r: &ResolvedTheme, role: TextRole) -> FontId {
    FontId::proportional(text_size_or_stock(
        text_role_style(r, role).size_px,
        &text_role_slot(role),
    ))
}

/// [`crate::text_role_line_height`]: `role`'s line height in logical
/// pixels.
#[must_use]
pub fn text_role_line_height(r: &ResolvedTheme, role: TextRole) -> f32 {
    text_role_style(r, role).line_height
}

/// [`crate::text_role_weight`]: `role`'s CSS weight.
#[must_use]
pub fn text_role_weight(r: &ResolvedTheme, role: TextRole) -> u16 {
    text_role_style(r, role).weight
}

/// [`crate::line_height`]: the line height multiplier
/// (`typography.line_height_multiplier`).
#[must_use]
pub fn line_height(r: &ResolvedTheme) -> f32 {
    r.typography.line_height_multiplier
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

/// [`crate::mono_font_weight`]: the monospace text's CSS weight
/// (`typography.monospace.weight`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no monospace weight.
pub fn mono_font_weight(r: &ResolvedTheme) -> Mapped<u16> {
    text_weight(&r.typography.monospace, "typography.monospace.weight")
}

/// The text style `role`'s widget draws its text in, and its home: the
/// label, text or body at rest, the body text for a role with no text of
/// its own.
fn role_text(r: &ResolvedTheme, role: Role) -> (TextStyle, &'static str) {
    let none = WidgetState::NONE;
    match role {
        Role::Button => (r.button.label(none), "button.label(NONE)"),
        Role::Input => (r.input.text(none), "input.text(NONE)"),
        Role::Checkbox => (r.checkbox.label(none), "checkbox.label(NONE)"),
        Role::Menu => (r.menu.item_label(none), "menu.item_label(NONE)"),
        Role::Tooltip => (r.tooltip.text.clone(), "tooltip.text"),
        Role::Tab => (r.tab.label(none), "tab.label(NONE)"),
        Role::Sidebar => (r.sidebar.item_label(none), "sidebar.item_label(NONE)"),
        Role::Toolbar => (r.toolbar.button_label(none), "toolbar.button_label(NONE)"),
        Role::StatusBar => (r.status_bar.label.clone(), "status_bar.label"),
        Role::Popover => (r.popover.text.clone(), "popover.text"),
        Role::ComboBox => (r.combo_box.label(none), "combo_box.label(NONE)"),
        Role::SegmentedControl => (
            r.segmented_control.label(none),
            "segmented_control.label(NONE)",
        ),
        Role::Expander => (r.expander.header_label(none), "expander.header_label(NONE)"),
        Role::Link => (r.link.label(none), "link.label(NONE)"),
        Role::List => (r.list.item_label(none), "list.item_label(NONE)"),
        Role::Dialog => (r.dialog.body.clone(), "dialog.body"),
        Role::Window => (
            r.window.title(WindowState::default()),
            "window.title(normal)",
        ),
        Role::Scrollbar
        | Role::Slider
        | Role::ProgressBar
        | Role::Splitter
        | Role::Separator
        | Role::Switch
        | Role::Spinner
        | Role::Card => (r.typography.body.clone(), "typography.body"),
    }
}

/// [`crate::role_font_weight`]: the CSS weight of the text `role`'s widget
/// draws.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no weight for it.
pub fn role_font_weight(r: &ResolvedTheme, role: Role) -> Mapped<u16> {
    let (text, home) = role_text(r, role);
    text.stated
        .contains(TextStated::WEIGHT)
        .then_some(text.weight)
        .ok_or(Unmapped(home))
}

/// [`crate::role_font_is_italic`]: whether the text `role`'s widget draws
/// is stated slanted (italic or oblique) where the body text is not; egui's
/// own upright text where the theme states no slant for either.
#[must_use]
pub fn role_font_is_italic(r: &ResolvedTheme, role: Role) -> bool {
    let slanted =
        |t: &TextStyle| t.stated.contains(TextStated::STYLE) && t.style != FontStyle::Normal;
    slanted(&role_text(r, role).0) && !slanted(&r.typography.body)
}

/// [`crate::window_title_bar_font`]: the font of a window's title
/// (`window.title(normal).size_px`), egui's own `Heading` size where it is
/// not a positive normal number.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no title size.
pub fn window_title_bar_font(r: &ResolvedTheme) -> Mapped<FontId> {
    let size = text_size(
        &r.window.title(WindowState::default()),
        "window.title(normal).size_px",
    )?;
    Ok(FontId::proportional(text_size_or_stock(
        size,
        &egui::TextStyle::Heading,
    )))
}

/// [`crate::window_title_bar_text_color`]: the colour of a window's title
/// (`window.title(normal).color`) in the window state the theme was
/// resolved for: an inactive window's title is read from the theme
/// resolved with `ThemeContext::window_active` false.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no title colour.
pub fn window_title_bar_text_color(r: &ResolvedTheme) -> Mapped<Color32> {
    text_color(
        &r.window.title(WindowState::default()),
        "window.title(normal).color",
    )
}

/// [`crate::list_header_font`]: the font of a column header
/// (`list.header_label(NONE).size_px`), egui's own `Body` size where it is
/// not a positive normal number.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no header size.
pub fn list_header_font(r: &ResolvedTheme) -> Mapped<FontId> {
    let size = text_size(
        &r.list.header_label(WidgetState::NONE),
        "list.header_label(NONE).size_px",
    )?;
    Ok(FontId::proportional(text_size_or_stock(
        size,
        &egui::TextStyle::Body,
    )))
}

/// [`crate::font_family`]: the body text's family (`typography.body.family`).
#[must_use]
pub fn font_family(r: &ResolvedTheme) -> &str {
    &r.typography.body.family
}

/// [`crate::font_size`]: the body text's size in logical pixels
/// (`typography.body.size_px`), egui's own `Body` size where it is not a
/// positive normal number.
#[must_use]
pub fn font_size(r: &ResolvedTheme) -> f32 {
    text_size_or_stock(r.typography.body.size_px, &egui::TextStyle::Body)
}

/// [`crate::mono_font_family`]: the monospace text's family
/// (`typography.monospace.family`).
#[must_use]
pub fn mono_font_family(r: &ResolvedTheme) -> &str {
    &r.typography.monospace.family
}

/// [`crate::mono_font_size`]: the monospace text's size in logical pixels
/// (`typography.monospace.size_px`), egui's own `Monospace` size where it is
/// not a positive normal number.
#[must_use]
pub fn mono_font_size(r: &ResolvedTheme) -> f32 {
    text_size_or_stock(r.typography.monospace.size_px, &egui::TextStyle::Monospace)
}

// ---------------------------------------------------------------------------
// Widgets' values no `Style` field carries
// ---------------------------------------------------------------------------

/// [`crate::dialog_button_order`]: which end of a dialog's button row its
/// affirmative button sits at (`dialog.button_order`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no order.
pub fn dialog_button_order(r: &ResolvedTheme) -> Mapped<DialogButtonOrder> {
    stated(r.dialog.button_order, "dialog.button_order")
}

/// [`crate::list_header_height`]: a column header's height, in logical
/// pixels (`list.header(NONE).min_size[1]`); `None` where the theme states
/// none.
#[must_use]
pub fn list_header_height(r: &ResolvedTheme) -> Option<f32> {
    r.list.header(WidgetState::NONE).min_size[1]
}

/// [`crate::list_header_hover_background`]: the fill of a hovered
/// column-header section, where a rule states one (`list.header(HOVER)
/// .fill`); `None` where none does.
///
/// # Errors
///
/// [`Unmapped`] where the fill is stated as a layered paint with no
/// fallback colour.
pub fn list_header_hover_background(r: &ResolvedTheme) -> Mapped<Option<Color32>> {
    state_fill(
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
pub fn list_header_active_background(r: &ResolvedTheme) -> Mapped<Option<Color32>> {
    state_fill(
        &r.list.header(WidgetState::PRESSED),
        "list.header(PRESSED).fill",
    )
}

/// [`crate::tooltip_show_delay`]: the delay between the pointer coming to
/// rest on an element and its tooltip showing (`timings.tooltip_show_ms`);
/// `None` where the theme states none, egui's own then staying.
#[must_use]
pub fn tooltip_show_delay(r: &ResolvedTheme) -> Option<Duration> {
    r.timings
        .tooltip_show_ms
        .map(|ms| Duration::from_millis(u64::from(ms)))
}

/// [`crate::scrollbar_groove_width`]: the vertical scroll bar's groove
/// width, in logical pixels (`scrollbar.track(NONE; orientation=Vertical)
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

/// [`crate::border_corner_radius`]: the default corner radius
/// (`layout.corner_radius`).
#[must_use]
pub fn border_corner_radius(r: &ResolvedTheme) -> f32 {
    r.layout.corner_radius
}

/// [`crate::border_corner_radius_lg`]: the default large corner radius
/// (`layout.corner_radius_lg`).
#[must_use]
pub fn border_corner_radius_lg(r: &ResolvedTheme) -> f32 {
    r.layout.corner_radius_lg
}

/// [`crate::border_color`]: the border and divider colour
/// (`colors.window.border`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window border colour.
pub fn border_color(r: &ResolvedTheme) -> Mapped<Color32> {
    stated(r.colors.window.border, "colors.window.border").map(to_color32)
}

/// [`crate::disabled_opacity`]: the whole widget's fade on top of its
/// disabled colours (`colors.disabled_opacity`).
#[must_use]
pub fn disabled_opacity(r: &ResolvedTheme) -> f32 {
    r.colors.disabled_opacity
}
