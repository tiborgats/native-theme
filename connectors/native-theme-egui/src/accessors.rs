//! Spec §4.7's free accessors: the values no `Style` field carries, and some the `Style` also
//! holds, for the application's own use.

use std::time::Duration;

use egui::FontId;
use native_theme::SystemTheme;
use native_theme::theme::{
    DialogButtonOrder, FontStyle, ResolvedFontSpec, ResolvedTextScaleEntry, ResolvedTheme,
};

use crate::convert::{
    button_padding_outside, clamp_length, finite_or, padding_with_line, to_color32,
    to_corner_radius, to_margin,
};
use crate::{AccessibilityPreferences, Role, TextRole};

/// A text size from the theme times the user's text-scaling factor,
/// [`AccessibilityPreferences::text_scale_factor`]: a factor that is not finite and positive is
/// ignored. The product is returned as it is, even where it is not a positive normal `f32` (a
/// finite factor that overflows it to `+∞`, or one that makes it subnormal or zero): spec §8.6
/// defines this function as the plain product, and every size the atlas writes through it then
/// takes spec §8.5's rule, egui's own size for the slot with a `Note::ValueSanitised`, and the
/// font accessors egui's own size. `native_theme_iced::scaled_text_size` and
/// `native_theme_gpui::scaled_text_size` go through [`AccessibilityPreferences::scaled`]
/// instead, which returns such a size unscaled: those connectors report no sanitised value, so
/// they keep the theme's size in that one place, where this atlas needs the product itself to
/// see the degenerate size and report it.
///
/// For a size no other accessor returns — a widget font read from the `ResolvedTheme`
/// directly (`t.button.font.size`, …) and drawn at a call site. Takes the preferences, not a
/// `&SystemTheme`, so the preset path can pass [`ThemeAtlas::accessibility`](crate::ThemeAtlas::accessibility).
///
/// The product has no upper bound here; the one that matters is the GPU's and the monitor's.
/// epaint panics on a glyph wider than its font atlas (`epaint/src/texture_atlas.rs:226-231`),
/// so every size the atlas hands egui is lowered to
/// [`ThemeAtlas::text_size_cap`](crate::ThemeAtlas::text_size_cap), derived from the pass's
/// `max_texture_side` and `pixels_per_point`: the base styles' at install and before each
/// pass, a role style's when it is handed over; a size from here that the application hands
/// egui itself is its own to clamp to that cap.
#[must_use]
pub fn scaled_text_size(size: f32, prefs: &AccessibilityPreferences) -> f32 {
    size * prefs.text_scale_factor()
}

/// egui's own size for a stock `TextStyle` slot (`egui/src/style.rs:1414-1424`); `None` only
/// for a key those five never lack.
fn stock_text_size(slot: &egui::TextStyle) -> Option<f32> {
    egui::style::default_text_styles()
        .get(slot)
        .map(|id| id.size)
}

/// Spec §8.5's rule for a size this crate hands out as a `FontId`: `scaled` when it is a positive
/// normal `f32`, else egui's own size for `slot` — and the degenerate value itself only where
/// egui's stock styles lack the slot, which they never do (nothing is invented).
pub(crate) fn text_size_or_stock(scaled: f32, slot: &egui::TextStyle) -> f32 {
    if scaled.is_normal() && scaled > 0.0 {
        scaled
    } else {
        stock_text_size(slot).unwrap_or(scaled)
    }
}

// --- colours egui has no slot for --------------------------------------------
// `Visuals` has `warn_fg_color` (egui/src/style.rs:1056) and `error_fg_color` (:1059) and
// nothing else in that family. Each is `to_color32` of its `defaults.*` leaf.
/// `defaults.info_color` as an `egui::Color32`.
#[must_use]
pub fn info_color(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.info_color)
}
/// `defaults.info_text_color` as an `egui::Color32`.
#[must_use]
pub fn info_text_color(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.info_text_color)
}
/// `defaults.warning_text_color` as an `egui::Color32`.
#[must_use]
pub fn warning_text_color(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.warning_text_color)
}
/// `defaults.disabled_text_color` as an `egui::Color32`.
#[must_use]
pub fn disabled_text_color(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.disabled_text_color)
}
/// `defaults.selection_inactive_background` as an `egui::Color32`.
#[must_use]
pub fn selection_inactive_background(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.selection_inactive_background)
}
/// `defaults.selection_inactive_text_color` as an `egui::Color32`: the text colour over the
/// selection of a view without the focus.
#[must_use]
pub fn selection_inactive_text_color(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.selection_inactive_text_color)
}
/// `toolbar.button_checked_background` as an `egui::Color32`: the fill of a
/// checked toolbar button, for `Button::fill` on the one button. egui has no
/// checked state: a selected button takes `visuals.selection`, which also
/// fills selected text in the same scope. Where it is `Some`, letter the
/// button in `button.checked_text_color`, or `button.active_text_color` where
/// that is `None`, read from the `ResolvedTheme`. `None` where the theme
/// states no checked fill: the button is drawn as a selected flat button.
#[must_use]
pub fn toolbar_button_checked_background(t: &ResolvedTheme) -> Option<egui::Color32> {
    t.toolbar.button_checked_background.map(to_color32)
}
/// `toolbar.button_checked_border_color` as an `egui::Color32`: the line of a
/// checked toolbar button at rest, `button.border.line_width` wide; under the
/// pointer and pressed the toolbar's hover outline
/// (`toolbar.button_hover_border_color`) applies. `None`: the checked button
/// keeps its rest stroke.
#[must_use]
pub fn toolbar_button_checked_border_color(t: &ResolvedTheme) -> Option<egui::Color32> {
    t.toolbar.button_checked_border_color.map(to_color32)
}

// --- focus ring: no egui concept at all ---------------------------------------
// egui promotes a keyboard-focused widget to `widgets.active`
// (`egui/src/widget_style.rs:107-109`; `Widgets::style`, `egui/src/style.rs:1273-1282`) and
// paints no ring. The install plugin paints it from these three leaves (§6.18); writing it
// into `widgets.active.bg_stroke` instead would show it on every mouse press too. These
// accessors are for a ring the application draws on a surface of its own.
/// `defaults.focus_ring_color` as an `egui::Color32`.
#[must_use]
pub fn focus_ring_color(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.focus_ring_color)
}
/// `defaults.focus_ring_width`, the focus ring's outline width in logical pixels.
#[must_use]
pub fn focus_ring_width(t: &ResolvedTheme) -> f32 {
    t.defaults.focus_ring_width
}
/// `defaults.focus_ring_offset`, the gap between a widget's edge and its focus ring, in logical
/// pixels. **May legitimately be negative** (an inset ring: adwaita −2.0, macOS −1.0). Never clamp it
/// to zero (`native-theme/src/resolve/validate_helpers.rs:810-811`).
#[must_use]
pub fn focus_ring_offset(t: &ResolvedTheme) -> f32 {
    t.defaults.focus_ring_offset
}

// --- typography roles egui's five TextStyles cannot hold -----------------------
fn text_role_entry(t: &ResolvedTheme, role: TextRole) -> &ResolvedTextScaleEntry {
    match role {
        TextRole::Caption => &t.text_scale.caption,
        TextRole::SectionHeading => &t.text_scale.section_heading,
        TextRole::DialogTitle => &t.text_scale.dialog_title,
        TextRole::Display => &t.text_scale.display,
    }
}

/// The stock slot a text-scale role falls back to (spec §4.7): `Small` for `Caption`, `Heading`
/// for the other three, the style `Ui::heading` sets (`egui/src/widget_text.rs:233-235`).
fn text_role_slot(role: TextRole) -> egui::TextStyle {
    match role {
        TextRole::Caption => egui::TextStyle::Small,
        TextRole::SectionHeading | TextRole::DialogTitle | TextRole::Display => {
            egui::TextStyle::Heading
        }
    }
}

/// The `egui::FontId` for a text-scale role, its size [`scaled_text_size`] of the role's size —
/// the size the atlas installs — or, where that is not a positive normal `f32`, egui's own size
/// for `TextStyle::Small` (`Caption`) or `TextStyle::Heading` (the other three, the style
/// `Ui::heading` sets, `egui/src/widget_text.rs:233-235`) (spec §8.5). `Caption` and
/// `SectionHeading` are also installed
/// into `TextStyle::Small` and `TextStyle::Heading`; `DialogTitle` and `Display` have no
/// **stock** `TextStyle` slot and are reachable only here, through `RichText::font(..)`
/// (`egui/src/widget_text.rs:190-198`).
///
/// A slot could have been minted — `TextStyle::Name(Arc<str>)` exists (`egui/src/style.rs:94`,
/// stored in `Style::text_styles`, `:289`) — so this is a **declined** candidate, not an egui
/// limitation. It is declined because `TextStyle::resolve` panics on a key absent from
/// `text_styles` (`:112-120`) with no `cfg(debug_assertions)` guard, which would make a `Name`
/// key a live panic in release for any `Style` this crate did not build. See spec §5.8.
///
/// Not bounded above: clamp the size to
/// [`ThemeAtlas::text_size_cap`](crate::ThemeAtlas::text_size_cap) before handing it to egui,
/// which panics on a glyph wider than its font atlas (`epaint/src/texture_atlas.rs:226-231`).
#[must_use]
pub fn text_role_font(
    t: &ResolvedTheme,
    role: TextRole,
    prefs: &AccessibilityPreferences,
) -> FontId {
    let scaled = scaled_text_size(text_role_entry(t, role).size, prefs);
    FontId::proportional(text_size_or_stock(scaled, &text_role_slot(role)))
}

/// The absolute line height in logical pixels for a text-scale role
/// (`ResolvedTextScaleEntry::line_height`, `native-theme/src/model/resolved.rs:49`), scaled by
/// the same factor as [`text_role_font`]'s size, so the line box keeps its ratio to the text.
/// Feed to `RichText::line_height(Some(..))` (`egui/src/widget_text.rs:174`) — the only exact
/// per-role mechanism egui has.
#[must_use]
pub fn text_role_line_height(
    t: &ResolvedTheme,
    role: TextRole,
    prefs: &AccessibilityPreferences,
) -> f32 {
    scaled_text_size(text_role_entry(t, role).line_height, prefs)
}

/// `defaults.line_height`, the dimensionless **multiplier**
/// (`native-theme/src/model/resolved.rs:80-81`). Not the same unit as
/// [`text_role_line_height`].
#[must_use]
pub fn line_height(t: &ResolvedTheme) -> f32 {
    t.defaults.line_height
}
/// `defaults.font.weight`, a CSS weight.
#[must_use]
pub fn font_weight(t: &ResolvedTheme) -> u16 {
    t.defaults.font.weight
}
/// `defaults.mono_font.weight`, a CSS weight.
#[must_use]
pub fn mono_font_weight(t: &ResolvedTheme) -> u16 {
    t.defaults.mono_font.weight
}
/// The CSS weight the theme asks for in a given text-scale role — what to pass to
/// `RichText::variation(egui::epaint::text::Tag::new(b"wght"), w as f32)` at a call site.
#[must_use]
pub fn text_role_weight(t: &ResolvedTheme, role: TextRole) -> u16 {
    text_role_entry(t, role).weight
}

/// The font a role's text is set in (spec §4.7's table): the widget's own `font` for the fourteen
/// that state one, `list.item_font`, `dialog.body_font`, `window.title_bar_font`, and
/// `defaults.font` for the eight whose widget states none.
fn role_font(t: &ResolvedTheme, role: Role) -> &ResolvedFontSpec {
    match role {
        Role::Button => &t.button.font,
        Role::Input => &t.input.font,
        Role::Checkbox => &t.checkbox.font,
        Role::Menu => &t.menu.font,
        Role::Tooltip => &t.tooltip.font,
        Role::Tab => &t.tab.font,
        Role::Sidebar => &t.sidebar.font,
        Role::Toolbar => &t.toolbar.font,
        Role::StatusBar => &t.status_bar.font,
        Role::Popover => &t.popover.font,
        Role::ComboBox => &t.combo_box.font,
        Role::SegmentedControl => &t.segmented_control.font,
        Role::Expander => &t.expander.font,
        Role::Link => &t.link.font,
        Role::List => &t.list.item_font,
        Role::Dialog => &t.dialog.body_font,
        Role::Window => &t.window.title_bar_font,
        Role::Scrollbar
        | Role::Slider
        | Role::ProgressBar
        | Role::Splitter
        | Role::Separator
        | Role::Switch
        | Role::Spinner
        | Role::Card => &t.defaults.font,
    }
}

/// The CSS weight of the font text in `role` is set in — for `RichText::variation(
/// egui::epaint::text::Tag::new(b"wght"), w as f32)` (`egui/src/widget_text.rs:202`) at a
/// call site, because a `FontId` carries no weight (`epaint/src/text/fonts.rs:21-28`) and this
/// crate installs one face per family (spec §4.9). The font is the widget's own: `font` for the
/// fourteen roles whose widget states one (`Button`, `Input`, `Checkbox`, `Menu`, `Tooltip`,
/// `Tab`, `Sidebar`, `Toolbar`, `StatusBar`, `Popover`, `ComboBox`, `SegmentedControl`,
/// `Expander`, `Link`), `list.item_font` for `List` (its cells), `dialog.body_font` for
/// `Dialog` (its body), `window.title_bar_font` for `Window` (the one font that widget
/// states), and `defaults.font` for the eight roles whose widget states none (`Scrollbar`,
/// `Slider`, `ProgressBar`, `Splitter`, `Separator`, `Switch`, `Spinner`, `Card`), whose
/// text is the defaults' text (`native-theme/src/model/widgets/mod.rs`, the
/// `#[theme_inherit(font = ..)]` attribute of each widget). A font spec that is no role's
/// font — `list.header_font`, `dialog.title_font` — is read from the `ResolvedTheme`
/// (`t.list.header_font.weight`, …) into the same call.
#[must_use]
pub fn role_font_weight(t: &ResolvedTheme, role: Role) -> u16 {
    role_font(t, role).weight
}

/// Whether text in `role` needs egui's synthetic slant, `RichText::italics`
/// (`egui/src/widget_text.rs:284`), to show the slant the theme asks for: `true` when the
/// role's font (as [`role_font_weight`] chooses it) has a `style` other than
/// `FontStyle::Normal` (`native-theme/src/model/font.rs:59`) and `defaults.font` — the spec
/// whose face the atlas installs as `FontFamily::Proportional` (spec §4.9, §8.2) — has
/// `FontStyle::Normal`. When `defaults.font` is itself slanted, the installed face already
/// is, and a second, synthetic slant would double it, so this returns `false`. egui's skew
/// cannot tell `Italic` from `Oblique`, so both map to the same call (spec §8.4).
#[must_use]
pub fn role_font_is_italic(t: &ResolvedTheme, role: Role) -> bool {
    role_font(t, role).style != FontStyle::Normal && t.defaults.font.style == FontStyle::Normal
}

/// `window.title_bar_font` as an `egui::FontId`: `FontFamily::Proportional` (the never-`Name`
/// invariant, spec §4.9) at [`scaled_text_size`] of its size, or egui's own `TextStyle::Heading`
/// size where that is not a positive normal `f32` — the style a `Window` title falls back to
/// (`egui/src/containers/window.rs:1350`, spec §8.5). For the title atoms of a `Window`,
/// via `RichText::font` (`egui/src/widget_text.rs:190-198`): the title is laid out outside the
/// application's closure (spec §5.2), so no scope reaches it.
///
/// Not bounded above: clamp the size to
/// [`ThemeAtlas::text_size_cap`](crate::ThemeAtlas::text_size_cap) before handing it to egui,
/// which panics on a glyph wider than its font atlas (`epaint/src/texture_atlas.rs:226-231`).
#[must_use]
pub fn window_title_bar_font(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> FontId {
    let scaled = scaled_text_size(t.window.title_bar_font.size, prefs);
    FontId::proportional(text_size_or_stock(scaled, &egui::TextStyle::Heading))
}

/// The title text colour for an active (`true`) or inactive window: `window.title_bar_font.color`
/// or `window.inactive_title_bar_text_color` (`native-theme/src/model/widgets/mod.rs:30`, `:27`).
/// Feed to `RichText::color` (`egui/src/widget_text.rs:320`). egui does not tell the title
/// which of the two a window is in (spec §5.2), so the application passes what it knows.
#[must_use]
pub fn window_title_bar_text_color(t: &ResolvedTheme, active: bool) -> egui::Color32 {
    to_color32(if active {
        t.window.title_bar_font.color
    } else {
        t.window.inactive_title_bar_text_color
    })
}

/// A dialog's default button where the theme gives it a look of its own:
/// the fill, label colour and frame to hand `Button::fill`, `RichText::color`
/// and `Button::stroke` at the call site.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct DefaultButton {
    /// `button.default_button_background`.
    pub fill: egui::Color32,
    /// `button.default_button_text_color`, else `button.font.color`.
    pub text: egui::Color32,
    /// `button.default_button_border_color`, else `button.border.color`, at
    /// `button.border.line_width`.
    pub stroke: egui::Stroke,
    /// The frame under the pointer and while pressed: `button.hover_border_color`,
    /// else `stroke`'s colour, at the same width.
    pub hover_stroke: egui::Stroke,
    /// The frame while it has the keyboard focus: `button.focus_border_color`,
    /// else `hover_stroke`'s colour, at the same width.
    pub focus_stroke: egui::Stroke,
}

/// `Some` where the theme states `button.default_button_background` (KDE), `None` where the
/// default button is the primary one (macOS, Windows): draw it then as a selected button in the
/// `Role::Button` scope, which takes `button.primary_background` and `primary_text_color`.
/// egui's `Button::fill` and `Button::stroke` override every on-hover effect ("Note that this
/// will override any on-hover effects", `egui/src/widgets/button.rs:153-167` at egui 0.37.0),
/// so one call draws one look in every state, and the states are the application's, from the
/// button's previous response: at rest `fill`, `text` and `stroke`; while it was hovered, the
/// same with `hover_stroke`; while it had the keyboard focus and was not pressed, the same with
/// `focus_stroke`, since egui gives a focused button its `active` look
/// (`egui/src/widget_style/mod.rs:212-222` at egui 0.37.0), which `Button::stroke` overrides as
/// well; while it was pressed, `hover_stroke` with neither `fill` nor `text`, so that egui draws
/// the plain button's pressed fill and label, which the `Role::Button` scope gives it, as Breeze
/// draws a pressed default button (docs/platform-facts.md §2.3). A non-finite
/// `button.border.line_width` gives a frame 0 wide, as [`text_area_frame`]'s border does.
#[must_use]
pub fn default_button(t: &ResolvedTheme) -> Option<DefaultButton> {
    let b = &t.button;
    let fill = b.default_button_background?;
    Some(default_button_from(&DefaultButtonSource {
        fill: to_color32(fill),
        text: to_color32(b.default_button_text_color.unwrap_or(b.font.color)),
        width: b.border.line_width,
        frame: to_color32(b.default_button_border_color.unwrap_or(b.border.color)),
        hover: b.hover_border_color.map(to_color32),
        focus: b.focus_border_color.map(to_color32),
    }))
}

/// What [`default_button`] reads, from either model: the default button's fill, label colour,
/// frame width and colour, and the hovered and focused frame colours where the theme states
/// them.
pub(crate) struct DefaultButtonSource {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub width: f32,
    pub frame: egui::Color32,
    pub hover: Option<egui::Color32>,
    pub focus: Option<egui::Color32>,
}

/// [`DefaultButton`] from its source: the hovered frame `frame`'s colour where the theme
/// states none, the focused one the hovered one's, each at the frame's width, 0 where it is
/// not finite.
pub(crate) fn default_button_from(src: &DefaultButtonSource) -> DefaultButton {
    let width = clamp_length(finite_or(src.width, 0.0));
    let hover = src.hover.unwrap_or(src.frame);
    let focus = src.focus.unwrap_or(hover);
    DefaultButton {
        fill: src.fill,
        text: src.text,
        stroke: egui::Stroke::new(width, src.frame),
        hover_stroke: egui::Stroke::new(width, hover),
        focus_stroke: egui::Stroke::new(width, focus),
    }
}

// --- geometry with no Style sink ----------------------------------------------
/// The «input» cell's `spacing.button_padding` as the margin egui's own `TextEdit` frame takes
/// from it, `Margin::from(Vec2)` (`egui-0.37.0/src/theme/default_style.rs`,
/// `StyleProvider<TextEditStyle>`): `input.border` under spec §5's pair rule over the base style's
/// `button_padding`, which is `button.border`'s under the same rule over egui's own
/// `Spacing::default().button_padding` (`egui-0.37.0/src/style.rs:1568`), as the atlas writes
/// the two cells ([`convert::button_padding_outside`](crate::convert::button_padding_outside)).
fn input_cell_margin(button: &Padded, input: &Padded) -> egui::Margin {
    let base = button_padding_outside(
        egui::style::Spacing::default().button_padding,
        &button.outside(),
    );
    egui::Margin::from(button_padding_outside(base, &input.outside()))
}

/// A widget's padding as the field margins read it, from either model: its four sides, top,
/// right, bottom, left, each `None` where unstated, and its border's line width, `None` where
/// the theme states no stroke.
pub(crate) struct Padded {
    pub sides: [Option<f32>; 4],
    pub line_width: Option<f32>,
}

impl Padded {
    /// A 0.7.2 border's padding and line width.
    pub(crate) fn old(border: &native_theme::theme::ResolvedWidgetBorder) -> Self {
        let p = &border.padding;
        Self {
            sides: [p.top, p.right, p.bottom, p.left],
            line_width: Some(border.line_width),
        }
    }

    /// The padding measured from the outside of the border
    /// ([`padding_with_line`]); a stroke the theme states none of adds no width.
    fn outside(&self) -> native_theme::theme::ResolvedPadding {
        let [top, right, bottom, left] = self.sides;
        let padding = native_theme::theme::ResolvedPadding {
            top,
            right,
            bottom,
            left,
        };
        match self.line_width {
            Some(width) => padding_with_line(&padding, width),
            None => padding_with_line(&padding, 0.0),
        }
    }
}

/// [`input_margin`] from the button's and the single-line field's paddings.
pub(crate) fn input_margin_from(button: &Padded, input: &Padded) -> egui::Margin {
    to_margin(input_cell_margin(button, input), &input.outside())
}

/// [`text_area_margin`] from [`input_margin`] and the multi-line field's padding.
pub(crate) fn text_area_margin_from(input: egui::Margin, text_area: &Padded) -> egui::Margin {
    to_margin(input, &text_area.outside())
}

/// `input.border.padding` as the margin of a single-line `TextEdit`, for `TextEdit::margin`
/// (`egui-0.37.0/src/widgets/text_edit/builder.rs`, `TextEdit::margin`) or inside
/// [`input_frame`]; no `Style` field holds it per side. Each side the theme states is that side
/// plus the border's line width, exactly per side, because egui paints the field's stroke
/// inside this margin where the platform's padding lies inside the border (spec §7.2): a
/// margin handed to `TextEdit::margin` becomes the frame's inner margin less the stroke's width
/// (`egui-0.37.0/src/widgets/text_edit/builder.rs:693-704`), and [`input_frame`] takes the
/// stroke out of it the same way. A side the theme leaves `None` is the «input» cell's
/// `spacing.button_padding` component, the padding egui's own `TextEdit` frame takes in that
/// scope (`egui-0.37.0/src/theme/default_style.rs`, `StyleProvider<TextEditStyle>`): on an
/// axis whose two sides are unstated, the base style's — the button's padding, else egui's
/// own (`egui-0.37.0/src/style.rs:1568`) — so a field drawn with this margin and one drawn
/// plain in the scope agree there, while the application's
/// [`Builder::style_patch`](crate::Builder::style_patch) leaves `button_padding` alone: the
/// accessors read only the theme. Built with
/// [`convert::to_margin`](crate::convert::to_margin), so a side saturates at the `i8` bound of
/// `epaint::Margin`.
#[must_use]
pub fn input_margin(t: &ResolvedTheme) -> egui::Margin {
    input_margin_from(
        &Padded::old(&t.button.border),
        &Padded::old(&t.input.border),
    )
}

/// The frame of a single-line `TextEdit`, for `TextEdit::frame`
/// (`egui-0.37.0/src/widgets/text_edit/builder.rs`, `TextEdit::frame`), which takes a frame it
/// is handed in place of the theme's (`:693-694`, 0.37.0): egui's own frame for a mutable text,
/// `StyleProvider<TextEditStyle>`'s (`egui-0.37.0/src/theme/default_style.rs`), but for its
/// inner margin, [`input_margin`] where egui's is the scope's `button_padding`, and its focus
/// stroke. egui outlines a field in the `Active` state of the previous pass's response —
/// pressed, focused or clicked (`egui-0.37.0/src/widget_style/mod.rs`,
/// `Response::widget_state`) — in `visuals.selection.stroke`, the selected text's colour, which
/// the `Role::Input` cell holds (spec §5.4); this frame outlines a focused field in
/// `input.focus_border_color` at the width of the state's own `bg_stroke`. Called inside the
/// `Role::Input` scope with the id handed to `TextEdit::id`:
/// `TextEdit::singleline(&mut s).id(id).frame(input_frame(ui, id, &t))`. Beside a frame,
/// `TextEdit::margin` is unused (`egui-0.37.0/src/widgets/text_edit/builder.rs:693-704`).
///
/// Computed from `ui`'s style. The state is `ui.style().interact(&r)`
/// (`egui/src/style.rs:355-357`) of `r = ui.ctx().read_response(id)`
/// (`egui/src/context.rs:1350-1355`) — this pass's interaction, before the field is added, the
/// entry `Response::widget_state` picks too (`egui-0.37.0/src/widget_style/mod.rs`) — or,
/// where there is none yet, `widgets.active` when focused (below) and `widgets.inactive`
/// otherwise — the entries `Widgets::style` gives a focused response and one neither hovered,
/// pressed nor focused (`egui/src/style.rs:1273-1282`), where egui takes `Inactive` for a
/// field with no response yet (`egui-0.37.0/src/theme/mod.rs`, `Ui::widget_style`); focused is
/// `Response::has_focus`'s own body (`egui/src/response.rs:349-351`),
/// `ui.ctx().input(|i| i.focused)` and `ui.ctx().memory(|m| m.has_focus(id))`, whether or not
/// `read_response` returned a response (a field focused before its first pass). Focus moved
/// while the field is added — by Tab (`egui/src/context.rs:1269-1270`,
/// `egui/src/memory/mod.rs:659-668`) or by `Memory::request_focus` after this call — shows in
/// the next pass: a key press or a click is an input event, after which egui runs that pass
/// itself (`egui/src/input_state/mod.rs:655-663`, `egui/src/context.rs:469`, `:537-538`); a
/// focus moved with no input event shows in the next pass that runs, which the application can
/// request at once with `Context::request_repaint` (`egui/src/context.rs:1821`).
/// Fill `ui.visuals().text_edit_bg_color()`; corner radius the state's; stroke the state's
/// `bg_stroke`, or, focused, `input.focus_border_color` at the state's own `bg_stroke.width`
/// (not egui's `selection.stroke.width`: a width that changed with the state could change the
/// frame's whole-point margin, and the field's size, on focus) — or the resting
/// `widgets.inactive.bg_stroke` where that `soft_option` is `None`, the border not changing on
/// focus (spec §5.4). The stroke and the state's `expansion` go on through egui's own
/// `Frame::apply_stroke_and_expansion_without_layout_shift`
/// (`egui-0.37.0/src/containers/frame.rs:313-323`), as egui's frame does: inner margin
/// [`input_margin`] plus `round(expansion − stroke.width)`, outer margin `−round(expansion)`,
/// each side saturating at the `i8` bound (`epaint-0.37.0/src/margin.rs`, `Margin`'s `Add` and
/// `Sub`), so the function is total.
///
/// egui lays the field out by its frame's total margin, `Frame::total_margin`
/// (`egui-0.37.0/src/containers/frame.rs:362-366`), whether the frame is its own or this one
/// (`egui-0.37.0/src/widgets/text_edit/builder.rs:809`), and no shorter than the scope's
/// `interact_size.y`, `TextEditStyle`'s least size (`StyleProvider<TextEditStyle>`,
/// `builder.rs:546`, 0.37.0). The total margin is [`input_margin`] wherever
/// `expansion − stroke.width` is whole; with a half-point stroke (`macos-monterey`,
/// `macos-tahoe`, `ios-13`, spec §5.4) it is half a point less on every side, in egui's own
/// frame and this one alike.
#[must_use = "hand the frame to TextEdit::frame"]
pub fn input_frame(ui: &egui::Ui, id: egui::Id, t: &ResolvedTheme) -> egui::Frame {
    field_frame(
        ui,
        id,
        &FieldFrameSource {
            margin: input_margin(t),
            focus: t.input.focus_border_color.map(to_color32),
            own: None,
        },
    )
}

/// `text_area.border.padding` as the margin of a multi-line `TextEdit`, as [`input_margin`] is
/// the single-line field's: each side the theme states plus `text_area.border.line_width`,
/// exactly per side, and [`input_margin`]'s side on a side it leaves `None`. A platform pads
/// its multi-line field apart from its single-line one (`docs/platform-facts.md` §2.29:
/// Breeze's QTextEdit 5 where its line edit is 7 / 6, GTK's text view 0 where its entry is
/// 9 / 0).
#[must_use]
pub fn text_area_margin(t: &ResolvedTheme) -> egui::Margin {
    text_area_margin_from(input_margin(t), &Padded::old(&t.text_area.border))
}

/// [`input_frame`] for a multi-line `TextEdit`, called in the same `Role::Input` scope with
/// the id handed to `TextEdit::id`:
/// `TextEdit::multiline(&mut s).id(id).frame(text_area_frame(ui, id, &t))`. The frame is the
/// single-line field's, state for state, but for `text_area.border`: its inner margin is
/// [`text_area_margin`] where [`input_frame`]'s is [`input_margin`], its stroke is
/// `text_area.border.line_width` wide, its corners `text_area.border.corner_radius`, and at
/// rest -- the scope's `inactive` or `noninteractive` entry -- its stroke is
/// `text_area.border.color`. Those inherit `input.border`'s (`docs/platform-facts.md` §2.29),
/// so on a theme that states none of its own the two fields are framed alike; under the
/// pointer and focused the stroke takes the input's hover and focus colours, which the text
/// area shares.
#[must_use = "hand the frame to TextEdit::frame"]
pub fn text_area_frame(ui: &egui::Ui, id: egui::Id, t: &ResolvedTheme) -> egui::Frame {
    let b = &t.text_area.border;
    field_frame(
        ui,
        id,
        &FieldFrameSource {
            margin: text_area_margin(t),
            focus: t.input.focus_border_color.map(to_color32),
            own: Some(OwnBorder {
                width: Some(b.line_width),
                color: Some(to_color32(b.color)),
                radius: Some(b.corner_radius),
            }),
        },
    )
}

/// What [`input_frame`] and [`text_area_frame`] read, from either model: the margin before
/// the state's `expansion − stroke.width`, the focused stroke's colour where the theme states
/// one, and the field's own border, where it has one.
pub(crate) struct FieldFrameSource {
    pub margin: egui::Margin,
    pub focus: Option<egui::Color32>,
    pub own: Option<OwnBorder>,
}

/// A field's own border: its width in every state, its resting colour and its corner radius,
/// each `None` where the theme states none, the scope's then standing.
pub(crate) struct OwnBorder {
    pub width: Option<f32>,
    pub color: Option<egui::Color32>,
    pub radius: Option<f32>,
}

/// The frame [`input_frame`] and [`text_area_frame`] build from their source.
pub(crate) fn field_frame(ui: &egui::Ui, id: egui::Id, src: &FieldFrameSource) -> egui::Frame {
    let ctx = ui.ctx();
    // This pass's interaction, before the field is added (`egui/src/context.rs:1350-1355`).
    let response = ctx.read_response(id);
    // `Response::has_focus`'s own body (`egui/src/response.rs:349-351`), whether or not a
    // response exists yet; two accessors in sequence, never one inside the other (§10.3).
    let focused = ctx.input(|i| i.focused) && ctx.memory(|m| m.has_focus(id));
    let style = ui.style();
    let state: &egui::style::WidgetVisuals = match &response {
        Some(r) => style.interact(r),
        // the entries `Widgets::style` gives a focused response and one neither hovered,
        // pressed nor focused (`egui/src/style.rs:1273-1282`)
        None if focused => &style.visuals.widgets.active,
        None => &style.visuals.widgets.inactive,
    };
    let stroke = if focused {
        match src.focus {
            // at the state's own width, so the frame's total margin never changes on focus
            Some(color) => egui::Stroke::new(state.bg_stroke.width, color),
            // the resting stroke where the soft option is `None`: the border does not change
            None => style.visuals.widgets.inactive.bg_stroke,
        }
    } else {
        state.bg_stroke
    };
    // `own`'s width in every state, its colour at rest: the entry `style.interact` gives a
    // response neither hovered, pressed nor focused, or a non-interactive one
    // (`egui/src/style.rs:1273-1282`).
    let resting = !focused
        && (std::ptr::eq(state, &style.visuals.widgets.inactive)
            || std::ptr::eq(state, &style.visuals.widgets.noninteractive));
    let (stroke, corner_radius) = match &src.own {
        Some(b) => (
            egui::Stroke::new(
                b.width
                    .map_or(stroke.width, |w| clamp_length(finite_or(w, 0.0))),
                match b.color {
                    Some(color) if resting => color,
                    _ => stroke.color,
                },
            ),
            b.radius.map_or(state.corner_radius, |r| {
                to_corner_radius(state.corner_radius, r)
            }),
        ),
        None => (stroke, state.corner_radius),
    };
    // egui's own frame for a mutable text, built as `StyleProvider<TextEditStyle>` builds it
    // (`egui-0.37.0/src/theme/default_style.rs`)
    egui::Frame {
        fill: style.visuals.text_edit_bg_color(),
        corner_radius,
        inner_margin: src.margin,
        ..Default::default()
    }
    .apply_stroke_and_expansion_without_layout_shift(stroke, state.expansion)
}

/// The expander's arrow, in `expander.arrow_color`, for `CollapsingHeader::icon`
/// (`egui/src/containers/collapsing_header.rs:480`). egui's own arrow,
/// `egui::collapsing_header::paint_default_icon` (`:336`), fills with the `fg_stroke.color`
/// of the state it is in (`:337`, `:351-355`), which is the header text's colour too. The
/// closure sets that colour to `expander.arrow_color` in all five `widgets` states of a copy
/// of the `Ui`'s style, calls `paint_default_icon` — egui's shape, egui's rotation — and
/// restores the `Ui`'s previous `Arc<Style>` with `Ui::set_style` (`egui/src/ui.rs:387`),
/// because it is handed the header's parent `Ui` (`:592`), whose later widgets must not see
/// the change. Owns one `Color32`, so it is `'static` as `icon` requires.
#[must_use = "hand the closure to CollapsingHeader::icon"]
pub fn expander_icon(t: &ResolvedTheme) -> impl Fn(&mut egui::Ui, f32, &egui::Response) + 'static {
    expander_icon_from(expander_arrow(t))
}

/// The arrow's colour [`expander_icon`] paints in, `expander.arrow_color`; `None` where the
/// theme states none.
pub(crate) fn expander_arrow(t: &ResolvedTheme) -> Option<egui::Color32> {
    t.expander.arrow_color.map(to_color32)
}

/// [`expander_icon`]'s closure, painting in `arrow`, or egui's own arrow where it is `None`
/// (spec §6.19: the soft option decides whether egui's own arrow colour stands).
pub(crate) fn expander_icon_from(
    arrow: Option<egui::Color32>,
) -> impl Fn(&mut egui::Ui, f32, &egui::Response) + 'static {
    move |ui: &mut egui::Ui, openness: f32, response: &egui::Response| match arrow {
        None => egui::collapsing_header::paint_default_icon(ui, openness, response),
        Some(color) => {
            let saved = std::sync::Arc::clone(ui.style());
            let widgets = &mut ui.style_mut().visuals.widgets;
            for state in [
                &mut widgets.noninteractive,
                &mut widgets.inactive,
                &mut widgets.hovered,
                &mut widgets.active,
                &mut widgets.open,
            ] {
                state.fg_stroke.color = color;
            }
            egui::collapsing_header::paint_default_icon(ui, openness, response);
            ui.set_style(saved);
        }
    }
}

/// `list.header_font` (`native-theme/src/model/widgets/mod.rs:646`) as an `egui::FontId`:
/// `FontFamily::Proportional` (the never-`Name` invariant, spec §4.9) at [`scaled_text_size`] of
/// its size, or egui's own `TextStyle::Body` size where that is not a positive normal `f32`
/// (spec §8.5). Feed to `RichText::font(..)`
/// (`egui/src/widget_text.rs:190-198`); the header's colour and weight are
/// `t.list.header_font.color` and `.weight` (spec §5.4).
///
/// Not bounded above: clamp the size to
/// [`ThemeAtlas::text_size_cap`](crate::ThemeAtlas::text_size_cap) before handing it to egui,
/// which panics on a glyph wider than its font atlas (`epaint/src/texture_atlas.rs:226-231`).
#[must_use]
pub fn list_header_font(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> FontId {
    let scaled = scaled_text_size(t.list.header_font.size, prefs);
    FontId::proportional(text_size_or_stock(scaled, &egui::TextStyle::Body))
}

/// `dialog.button_order`: the order in which to add a dialog's buttons (spec §5.2).
#[must_use]
pub fn dialog_button_order(t: &ResolvedTheme) -> DialogButtonOrder {
    t.dialog.button_order
}
/// `list.header_height`, the height of a table's header row in logical pixels, for
/// `TableBuilder::header(height, ..)` (`egui_extras/src/table.rs:452`), which no `Style` field
/// reaches; `None` where the theme states none, and the showcase's tables then make the header
/// as tall as a row.
#[must_use]
pub fn list_header_height(t: &ResolvedTheme) -> Option<f32> {
    t.list.header_height
}
/// `list.header_hover_background` as an `egui::Color32`: the fill of a hovered column-header
/// section, where the platform states one (`None`: it keeps `list.header_background`).
/// egui_extras' header row has no hover (`egui_extras/src/table.rs:490-506`); for a header
/// the application paints.
#[must_use]
pub fn list_header_hover_background(t: &ResolvedTheme) -> Option<egui::Color32> {
    t.list.header_hover_background.map(to_color32)
}
/// `list.header_active_background` as an `egui::Color32`: the fill of a pressed column-header
/// section, where the platform states one (`None`: the hovered fill, else
/// `list.header_background`). As [`list_header_hover_background`].
#[must_use]
pub fn list_header_active_background(t: &ResolvedTheme) -> Option<egui::Color32> {
    t.list.header_active_background.map(to_color32)
}
/// `tooltip.show_delay` as a `Duration`, or `None` where the theme states none. The atlas
/// already writes it, in seconds, into `Style::interaction.tooltip_delay`
/// (`egui/src/style.rs:928`), which egui's own tooltips wait for
/// (`egui/src/containers/tooltip.rs:238`); this is for a delay the application times itself.
#[must_use]
pub fn tooltip_show_delay(t: &ResolvedTheme) -> Option<Duration> {
    t.tooltip
        .show_delay
        .map(|ms| Duration::from_millis(u64::from(ms)))
}
/// `scrollbar.groove_width`, in logical pixels.
#[must_use]
pub fn scrollbar_groove_width(t: &ResolvedTheme) -> f32 {
    t.scrollbar.groove_width
}
/// `defaults.border.corner_radius`, in logical pixels.
#[must_use]
pub fn border_corner_radius(t: &ResolvedTheme) -> f32 {
    t.defaults.border.corner_radius
}
/// `defaults.border.corner_radius_lg`, in logical pixels.
#[must_use]
pub fn border_corner_radius_lg(t: &ResolvedTheme) -> f32 {
    t.defaults.border.corner_radius_lg
}
/// `defaults.border.color` as an `egui::Color32`: the final line colour, into which the model
/// has already folded `defaults.border.opacity`, so nothing multiplies it again.
#[must_use]
pub fn border_color(t: &ResolvedTheme) -> egui::Color32 {
    to_color32(t.defaults.border.color)
}
/// `defaults.disabled_opacity`, the opacity for disabled controls.
#[must_use]
pub fn disabled_opacity(t: &ResolvedTheme) -> f32 {
    t.defaults.disabled_opacity
}
/// `defaults.font.family`, the family name the theme states.
#[must_use]
pub fn font_family(t: &ResolvedTheme) -> &str {
    &t.defaults.font.family
}

/// [`scaled_text_size`] of `defaults.font.size` — the `TextStyle::Body` size the atlas
/// installs, or egui's own `TextStyle::Body` size where that is not a positive normal `f32`
/// (spec §8.5). Same signature as `native_theme_iced::font_size`
/// (`connectors/native-theme-iced/src/lib.rs:1799-1804`).
#[must_use]
pub fn font_size(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32 {
    text_size_or_stock(
        scaled_text_size(t.defaults.font.size, prefs),
        &egui::TextStyle::Body,
    )
}

/// `defaults.mono_font.family`, the family name the theme states.
#[must_use]
pub fn mono_font_family(t: &ResolvedTheme) -> &str {
    &t.defaults.mono_font.family
}

/// [`scaled_text_size`] of `defaults.mono_font.size` — the `TextStyle::Monospace` size the
/// atlas installs, or egui's own `TextStyle::Monospace` size where that is not a positive
/// normal `f32` (spec §8.5).
#[must_use]
pub fn mono_font_size(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32 {
    text_size_or_stock(
        scaled_text_size(t.defaults.mono_font.size, prefs),
        &egui::TextStyle::Monospace,
    )
}

// --- accessibility: `&SystemTheme`, never `&ResolvedTheme` ---------------------
// `AccessibilityPreferences` lives on `SystemTheme` (`native-theme/src/lib.rs:716`) and is
// deliberately absent from `ResolutionContext` (`native-theme/src/resolve/context.rs:20-23`).
// The bare `&SystemTheme` in these four signatures is resolved by a crate-private
// `use native_theme::SystemTheme;`, NOT by a root re-export — §4.1 removes that one on purpose.
/// `accessibility.reduce_motion` of the `SystemTheme`: whether the OS asks for reduced motion.
/// The atlas already forwards it to egui's own animations ([`Builder::accessibility`](crate::Builder::accessibility)); this
/// is for the application's, e.g. the `reduced_motion` argument of
/// [`icons::animated_frame_index`](crate::icons::animated_frame_index) and [`icons::spin_angle`](crate::icons::spin_angle).
#[must_use]
pub fn is_reduced_motion(sys: &SystemTheme) -> bool {
    sys.accessibility.reduce_motion
}
/// `accessibility.high_contrast` of the `SystemTheme`: whether the OS asks for high contrast.
#[must_use]
pub fn is_high_contrast(sys: &SystemTheme) -> bool {
    sys.accessibility.high_contrast
}
/// `accessibility.reduce_transparency` of the `SystemTheme`: whether the OS asks for less transparency.
#[must_use]
pub fn is_reduced_transparency(sys: &SystemTheme) -> bool {
    sys.accessibility.reduce_transparency
}

/// `accessibility.text_scaling_factor`, the factor of the user's text-size preference the
/// toolkit has not applied (1.0 = no scaling), unfiltered; [`scaled_text_size`] is what ignores
/// a factor that is not finite and positive.
///
/// The atlas applies it to text sizes ([`Builder::accessibility`](crate::Builder::accessibility)).
/// On a GNOME or Budgie X11 session it is 1.0: eframe's `native_pixels_per_point` is winit's
/// scale factor (egui-winit 0.36.2 `src/lib.rs:54-58`, `:1376`), which on X11, while
/// `WINIT_X11_SCALE_FACTOR` is unset, is `Xft.dpi` over 96 (winit 0.30.13
/// `src/platform_impl/linux/x11/util/randr.rs:112-154`), and gnome-settings-daemon writes
/// `Xft.dpi` with the factor in it, so egui scales every length there, text included.
/// Elsewhere egui has no equivalent of its own: `Context::set_zoom_factor`
/// (`egui/src/context.rs:2337`) is not one — it sets
/// `pixels_per_point = zoom_factor * native_pixels_per_point` (`:455`) and so scales every
/// length, strokes and icons included, where the preference asks for larger text
/// (`native-theme/src/lib.rs:416-418`). Do **not** pre-multiply any `Style` value by
/// `pixels_per_point` or `zoom_factor` (spec §2, *Lengths*).
#[must_use]
pub fn text_scaling_factor(sys: &SystemTheme) -> f32 {
    sys.accessibility.text_scaling_factor
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;

    /// §4.7, the siblings' rule: the factor applies when finite and positive, else the size
    /// is returned unchanged (T13 (d) repeats it).
    #[test]
    fn scaled_text_size_ignores_a_factor_that_is_not_finite_and_positive() {
        let prefs = |f: f32| {
            let mut prefs = AccessibilityPreferences::default();
            prefs.text_scaling_factor = f;
            prefs
        };
        assert_eq!(scaled_text_size(10.0, &prefs(1.5)), 15.0);
        for f in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(scaled_text_size(10.0, &prefs(f)), 10.0, "{f}");
        }
    }
}
