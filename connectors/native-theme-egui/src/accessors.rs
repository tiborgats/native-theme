//! §4.7's free accessors: the values no `Style` field carries.

use egui::FontId;
use native_theme::SystemTheme;
use native_theme::theme::{
    DialogButtonOrder, FontStyle, ResolvedFontSpec, ResolvedTextScaleEntry, ResolvedTheme,
};

use crate::convert::{
    clamp_length, finite_or, i8_from_f32_saturating, padding_with_border, to_color32,
    to_corner_radius, to_margin,
};
use crate::{AccessibilityPreferences, Role, TextRole};

/// A text size from the theme times the user's text-scaling factor; a factor that is not
/// finite and positive is ignored. Same signature and semantics as
/// `native_theme_iced::scaled_text_size` (`connectors/native-theme-iced/src/lib.rs:480`) and
/// `native_theme_gpui::scaled_text_size` (`connectors/native-theme-gpui/src/lib.rs:444`).
///
/// For a size no accessor above returns — a widget font read from the `ResolvedTheme`
/// directly (`t.button.font.size`, …) and drawn at a call site. Takes the preferences, not a
/// `&SystemTheme`, so the preset path can pass [`ThemeAtlas::accessibility`](crate::ThemeAtlas::accessibility).
#[must_use]
pub fn scaled_text_size(size: f32, prefs: &AccessibilityPreferences) -> f32 {
    size * text_scale_factor(prefs)
}

/// The text-scaling multiplier: the factor when it is finite and positive, else `1.0`, the
/// multiplicative identity (§6.17) — the rule both siblings apply, iced's own
/// `text_scale_factor` being `connectors/native-theme-iced/src/lib.rs:486-489`.
fn text_scale_factor(prefs: &AccessibilityPreferences) -> f32 {
    let s = prefs.text_scaling_factor;
    if s.is_finite() && s > 0.0 { s } else { 1.0 }
}

/// egui's own size for a stock `TextStyle` slot (`egui/src/style.rs:1414-1424`); `None` only
/// for a key those five never lack.
fn stock_text_size(slot: &egui::TextStyle) -> Option<f32> {
    egui::style::default_text_styles()
        .get(slot)
        .map(|id| id.size)
}

/// §8.5's rule for a size this crate hands out as a `FontId`: `scaled` when it is a positive
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
/// **May legitimately be negative** (an inset ring: adwaita −2.0, macOS −1.0). Never clamp it
/// to zero (`native-theme/src/resolve/validate_helpers.rs:708-709`).
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

/// The stock slot a text-scale role falls back to (§4.7): `Small` for `Caption`, `Heading`
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
/// `Ui::heading` sets, `egui/src/widget_text.rs:233-235`) (§8.5). `Caption` and
/// `SectionHeading` are also installed
/// into `TextStyle::Small` and `TextStyle::Heading`; `DialogTitle` and `Display` have no
/// **stock** `TextStyle` slot and are reachable only here, through `RichText::font(..)`
/// (`egui/src/widget_text.rs:190-198`).
///
/// A slot could have been minted — `TextStyle::Name(Arc<str>)` exists (`egui/src/style.rs:94`,
/// stored in `Style::text_styles`, `:289`) — so this is a **declined** candidate, not an egui
/// limitation. It is declined because `TextStyle::resolve` panics on a key absent from
/// `text_styles` (`:112-120`) with no `cfg(debug_assertions)` guard, which would make a `Name`
/// key a live panic in release for any `Style` this crate did not build. See §5.8.
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
/// (`ResolvedTextScaleEntry::line_height`, `native-theme/src/model/resolved.rs:47`), scaled by
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
/// (`native-theme/src/model/resolved.rs:76-77`). Not the same unit as
/// [`text_role_line_height`].
#[must_use]
pub fn line_height_multiplier(t: &ResolvedTheme) -> f32 {
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

/// The font a role's text is set in (§4.7's table): the widget's own `font` for the fourteen
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
/// crate installs one face per family (§4.9). The font is the widget's own: `font` for the
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
/// whose face the atlas installs as `FontFamily::Proportional` (§4.9, §8.2) — has
/// `FontStyle::Normal`. When `defaults.font` is itself slanted, the installed face already
/// is, and a second, synthetic slant would double it, so this returns `false`. egui's skew
/// cannot tell `Italic` from `Oblique`, so both map to the same call (§8.4).
#[must_use]
pub fn role_font_is_italic(t: &ResolvedTheme, role: Role) -> bool {
    role_font(t, role).style != FontStyle::Normal && t.defaults.font.style == FontStyle::Normal
}

/// `window.title_bar_font` as an `egui::FontId`: `FontFamily::Proportional` (the never-`Name`
/// invariant, §4.9) at [`scaled_text_size`] of its size, or egui's own `TextStyle::Heading`
/// size where that is not a positive normal `f32` — the style a `Window` title falls back to
/// (`egui/src/containers/window.rs:1350`, §8.5). For the title atoms of a `Window`,
/// via `RichText::font` (`egui/src/widget_text.rs:190-198`): the title is laid out outside the
/// application's closure (§5.2), so no scope reaches it.
#[must_use]
pub fn window_title_bar_font(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> FontId {
    let scaled = scaled_text_size(t.window.title_bar_font.size, prefs);
    FontId::proportional(text_size_or_stock(scaled, &egui::TextStyle::Heading))
}

/// The title text colour for an active (`true`) or inactive window: `window.title_bar_font.color`
/// or `window.inactive_title_bar_text_color` (`native-theme/src/model/widgets/mod.rs:25`).
/// Feed to `RichText::color` (`egui/src/widget_text.rs:320`). egui does not tell the title
/// which of the two a window is in (§5.2), so the application passes what it knows.
#[must_use]
pub fn window_title_bar_text_color(t: &ResolvedTheme, active: bool) -> egui::Color32 {
    to_color32(if active {
        t.window.title_bar_font.color
    } else {
        t.window.inactive_title_bar_text_color
    })
}

// --- geometry with no Style sink ----------------------------------------------
/// egui's own `TextEdit` margin (`egui/src/widgets/text_edit/builder.rs:136`).
const TEXT_EDIT_MARGIN: egui::Margin = egui::Margin::symmetric(4, 2);

/// `input.border.padding` as the margin to hand `TextEdit::margin`
/// (`egui/src/widgets/text_edit/builder.rs:313`), the one place a `TextEdit` takes its inner
/// padding — no `Style` field holds it. Each side the theme states is that side plus the
/// border's line width, because `TextEdit` paints its stroke inside this margin (`:763-767`)
/// where the platform's padding lies inside the border (§7.2); a side the theme leaves
/// `None` keeps egui's own default, `Margin::symmetric(4, 2)`
/// (`egui/src/widgets/text_edit/builder.rs:136`, a crate constant naming that line). Built
/// with [`convert::to_margin`](crate::convert::to_margin), so a side saturates at the `i8` bound of `epaint::Margin`.
#[must_use]
pub fn input_margin(t: &ResolvedTheme) -> egui::Margin {
    to_margin(TEXT_EDIT_MARGIN, &padding_with_border(&t.input.border))
}

/// The frame a `TextEdit` paints, for `TextEdit::frame`
/// (`egui/src/widgets/text_edit/builder.rs:304-309`), which paints a frame it is handed as
/// given (`:712-713`, `:734-735`): egui's own frame for a mutable text (`:737-750`,
/// `:759-768`), except that a focused field's stroke takes its colour from
/// `input.focus_border_color` where egui's takes `visuals.selection.stroke` (`:742-747`) —
/// the selected text's colour, which the `Role::Input` cell holds (§5.4) — at the width of
/// the state's own `bg_stroke`. Called inside the `Role::Input` scope with the id handed to
/// `TextEdit::id` (`:168`):
/// `TextEdit::singleline(&mut s).id(id).frame(input_frame(ui, id, &t))`. The frame's inner
/// margin is [`input_margin`]; beside a frame, `TextEdit::margin` is unused (`:712-713`).
///
/// Computed from `ui`'s style exactly as egui computes it. The state is
/// `ui.style().interact(&r)` (`egui/src/style.rs:355-357`) of
/// `r = ui.ctx().read_response(id)` (`egui/src/context.rs:1350-1355`) — this pass's
/// interaction, before the field is added — or, where there is none yet, `widgets.active`
/// when focused (below) and `widgets.inactive` otherwise — the entries `Widgets::style` gives
/// a focused response and one neither hovered, pressed nor focused
/// (`egui/src/style.rs:1273-1282`); focused is `Response::has_focus`'s own body
/// (`egui/src/response.rs:349-351`), `ui.ctx().input(|i| i.focused)` and
/// `ui.ctx().memory(|m| m.has_focus(id))`, whether or not `read_response` returned a response
/// (a field focused before its first pass). Focus moved while the field is added — by Tab
/// (`egui/src/context.rs:1269-1270`, `egui/src/memory/mod.rs:659-668`) or by
/// `Memory::request_focus` after this call — shows in the next pass: a key press or a click is
/// an input event, after which egui runs that pass itself (`egui/src/input_state/mod.rs:655-663`,
/// `egui/src/context.rs:469`, `:537-538`); a focus moved with no input event shows in the next
/// pass that runs, which the application can request at once with `Context::request_repaint`
/// (`egui/src/context.rs:1821`).
/// Fill `ui.visuals().text_edit_bg_color()` (`egui/src/widgets/text_edit/builder.rs:739`);
/// corner radius the state's (`:744`, `:749`); stroke the state's `bg_stroke` (`:749`), or,
/// focused, `input.focus_border_color` at the state's own `bg_stroke.width` (not egui's
/// `selection.stroke.width`: a width that changed with the state would change the frame's
/// total margin, and the field's size, on focus) — or the
/// resting `widgets.inactive.bg_stroke` where that `soft_option` is `None`, the border not
/// changing on focus (§5.4); inner margin [`input_margin`] plus `expansion − stroke.width`
/// on every side, outer margin `−expansion` (`:763-767`). Both narrowings to `i8` go through
/// `i8_from_f32_saturating` (§7.2) where egui casts with `as`
/// (`egui/src/widgets/text_edit/builder.rs:765`, `:767`), and the sum is `epaint::Margin`'s
/// saturating `Add` (`epaint/src/margin.rs:123-134`), so the function is total; the two agree
/// bit for bit wherever `expansion` is whole, as it is in every scope but `Role::Slider`'s
/// (§6.6).
///
/// `TextEdit` lays a custom frame out by the frame's own total margin
/// (`egui/src/widgets/text_edit/builder.rs:715`, `:726-729`; `Frame::total_margin`,
/// `egui/src/containers/frame.rs:327-331`), where it lays its stock frame out by the margin
/// alone (`egui/src/widgets/text_edit/builder.rs:713`); the two agree whenever
/// `expansion − stroke.width` is whole. With a half-point stroke (`macos-sonoma`, `ios`,
/// §5.4) the total margin is half a point less than egui's stock one in every state, so the
/// field is one point shorter than egui's own and keeps that size whether focused or not.
#[must_use = "hand the frame to TextEdit::frame"]
pub fn input_frame(ui: &egui::Ui, id: egui::Id, t: &ResolvedTheme) -> egui::Frame {
    field_frame(ui, id, t, input_margin(t), None)
}

/// `text_area.border.padding` as the margin of a multi-line `TextEdit`, as [`input_margin`] is
/// the single-line field's: each side the theme states plus `text_area.border.line_width`,
/// egui's own `Margin::symmetric(4, 2)` on a side it leaves `None`. A platform pads its
/// multi-line field apart from its single-line one (`docs/platform-facts.md` §2.29: Breeze's
/// QTextEdit 5 where its line edit is 7 / 6, GTK's text view 0 where its entry is 9 / 0).
#[must_use]
pub fn text_area_margin(t: &ResolvedTheme) -> egui::Margin {
    to_margin(TEXT_EDIT_MARGIN, &padding_with_border(&t.text_area.border))
}

/// [`input_frame`] for a multi-line `TextEdit`, called in the same `Role::Input` scope with
/// the id handed to `TextEdit::id`:
/// `TextEdit::multiline(&mut s).id(id).frame(text_area_frame(ui, id, &t))`. The frame is the
/// single-line field's, state for state, but for `text_area.border`: its inner margin is
/// [`text_area_margin`] where [`input_frame`]'s is [`input_margin`], its stroke is
/// `text_area.border.line_width` wide, its corners `text_area.border.corner_radius`, and at
/// rest -- the scope's `inactive` or `noninteractive` entry -- its stroke is
/// `text_area.border.color`. Those inherit `input.border`'s (§2.29), so on a theme that states
/// none of its own the two fields are framed alike; under the pointer and focused the stroke
/// takes the input's hover and focus colours, which the text area shares.
#[must_use = "hand the frame to TextEdit::frame"]
pub fn text_area_frame(ui: &egui::Ui, id: egui::Id, t: &ResolvedTheme) -> egui::Frame {
    field_frame(ui, id, t, text_area_margin(t), Some(&t.text_area.border))
}

/// The frame [`input_frame`] and [`text_area_frame`] build: `base` its margin before the
/// state's `expansion − stroke.width`, and `own`, where given, the border whose width,
/// radius and resting colour replace the scope's.
fn field_frame(
    ui: &egui::Ui,
    id: egui::Id,
    t: &ResolvedTheme,
    base: egui::Margin,
    own: Option<&native_theme::theme::ResolvedWidgetBorder>,
) -> egui::Frame {
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
        match t.input.focus_border_color {
            // at the state's own width, so the frame's total margin never changes on focus
            Some(color) => egui::Stroke::new(state.bg_stroke.width, to_color32(color)),
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
    let (stroke, corner_radius) = match own {
        Some(b) => (
            egui::Stroke::new(
                clamp_length(finite_or(b.line_width, 0.0)),
                if resting {
                    to_color32(b.color)
                } else {
                    stroke.color
                },
            ),
            to_corner_radius(state.corner_radius, b.corner_radius),
        ),
        None => (stroke, state.corner_radius),
    };
    // egui's `Margin::same((expansion - stroke.width).round() as i8)` and
    // `Margin::same(-(expansion as i8))` (`egui/src/widgets/text_edit/builder.rs:765`, `:767`),
    // each through the saturating narrowing of §7.2. egui adds the first to the margin with
    // `Margin + Margin`, which saturates side by side (`epaint/src/margin.rs:123-134`); the
    // sum is spelled out with `i8::saturating_add`, the same arithmetic, because the
    // strict-panic set's `arithmetic_side_effects` rejects the operator on a non-primitive type.
    let grow = i8_from_f32_saturating(state.expansion - stroke.width);
    let inner = egui::Margin {
        left: base.left.saturating_add(grow),
        right: base.right.saturating_add(grow),
        top: base.top.saturating_add(grow),
        bottom: base.bottom.saturating_add(grow),
    };
    let outer = egui::Margin::same(i8_from_f32_saturating(-state.expansion));
    egui::Frame::new()
        .fill(style.visuals.text_edit_bg_color())
        .corner_radius(corner_radius)
        .inner_margin(inner)
        .outer_margin(outer)
        .stroke(stroke)
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
    // §6.19: the soft option decides whether egui's own arrow colour stands
    let arrow = t.expander.arrow_color.map(to_color32);
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

/// `list.header_font` (`native-theme/src/model/widgets/mod.rs:535`) as an `egui::FontId`:
/// `FontFamily::Proportional` (the never-`Name` invariant, §4.9) at [`scaled_text_size`] of
/// its size, or egui's own `TextStyle::Body` size where that is not a positive normal `f32`
/// (§8.5). Feed to `RichText::font(..)`
/// (`egui/src/widget_text.rs:190-198`); the header's colour and weight are
/// `t.list.header_font.color` and `.weight` (§5.4).
#[must_use]
pub fn list_header_font(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> FontId {
    let scaled = scaled_text_size(t.list.header_font.size, prefs);
    FontId::proportional(text_size_or_stock(scaled, &egui::TextStyle::Body))
}

/// `dialog.button_order`: the order in which to add a dialog's buttons (§5.2).
#[must_use]
pub fn dialog_button_order(t: &ResolvedTheme) -> DialogButtonOrder {
    t.dialog.button_order
}
/// `scrollbar.groove_width`, in logical pixels.
#[must_use]
pub fn scrollbar_width(t: &ResolvedTheme) -> f32 {
    t.scrollbar.groove_width
}
/// `defaults.border.corner_radius`, in logical pixels.
#[must_use]
pub fn border_radius(t: &ResolvedTheme) -> f32 {
    t.defaults.border.corner_radius
}
/// `defaults.border.corner_radius_lg`, in logical pixels.
#[must_use]
pub fn border_radius_lg(t: &ResolvedTheme) -> f32 {
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
/// (§8.5). Same signature as `native_theme_iced::font_size`
/// (`connectors/native-theme-iced/src/lib.rs:438-443`).
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
/// normal `f32` (§8.5).
#[must_use]
pub fn mono_font_size(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32 {
    text_size_or_stock(
        scaled_text_size(t.defaults.mono_font.size, prefs),
        &egui::TextStyle::Monospace,
    )
}

// --- accessibility: `&SystemTheme`, never `&ResolvedTheme` ---------------------
// `AccessibilityPreferences` lives on `SystemTheme` (`native-theme/src/lib.rs:490`) and is
// deliberately absent from `ResolutionContext` (`native-theme/src/resolve/context.rs:20-23`).
// The bare `&SystemTheme` in these four signatures is resolved by a crate-private
// `use native_theme::SystemTheme;`, NOT by a root re-export — §4.1 removes that one on purpose.
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

/// `accessibility.text_scaling_factor` as the OS reported it (1.0 = no scaling), unfiltered;
/// [`scaled_text_size`] is what ignores a factor that is not finite and positive.
///
/// The atlas applies it to text sizes ([`Builder::accessibility`](crate::Builder::accessibility)). egui has no equivalent of
/// its own: no `egui` or `eframe` 0.36.2 source reads an OS text-scaling preference, and
/// `Context::set_zoom_factor` (`egui/src/context.rs:2337`) is not one — it sets
/// `pixels_per_point = zoom_factor * native_pixels_per_point` (`:455`) and so scales every
/// length, strokes and icons included, where the preference asks for larger text
/// (`native-theme/src/lib.rs:251-253`). Do **not** pre-multiply any `Style` value by
/// `pixels_per_point` or `zoom_factor` (§2, *Lengths*).
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
        let prefs = |f: f32| AccessibilityPreferences {
            text_scaling_factor: f,
            ..AccessibilityPreferences::default()
        };
        assert_eq!(scaled_text_size(10.0, &prefs(1.5)), 15.0);
        for f in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(scaled_text_size(10.0, &prefs(f)), 10.0, "{f}");
        }
    }
}
