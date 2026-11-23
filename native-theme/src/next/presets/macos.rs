//! The `macos-monterey` and `macos-tahoe` presets' builder (spec §6): AppKit
//! from macOS 12 Monterey to 15 Sequoia, and AppKit's Liquid Glass look of
//! macOS 26 Tahoe, light and dark.
//!
//! The two presets share the builder and differ in their constants: each
//! has its own table of colours per scheme ([`SchemeColors`]) and its own
//! generation of metrics, fonts and other constants ([`Generation`]), each
//! constant with the citation its 0.7.2 TOML gives it. The builder runs in
//! three steps: the preset's constants for the scheme resolved for; the
//! colour roles and the globals made from them; the widgets derived from the
//! roles, the globals and the constants. A widget's colour that is a role's
//! colour is taken from the roles, not from a second copy of the constant.
//! This is the static path: the system's settings, the active palette and an
//! explicit accent ([`ResolveInputs`]) are not laid over the constants yet,
//! and the widgets this builder does not state yet are the theme with
//! nothing stated (`nothing_stated`).
//!
//! Defined by the presets (spec §1, "Axis values without data"): the
//! variant `ButtonVariant::Primary` (0.7.2's `primary_*` fields), a list's
//! alternate rows (`ListKind::alternate`), the two orientations of the
//! scroll bar, no control size and the contrast level `Normal` only; a query
//! for anything else returns the part's base style. A maximized or
//! full-screen window has no data of its own: its frame and title bar are
//! the normal window's.
//!
//! No part has an exclusive state group: where two rules can both match,
//! the later one wins (spec §3, "State combinations a source does not
//! state"), and a state without data of its own returns what the query
//! returns without that state. The precedences, each with its source:
//! - the button's body: the pressed fill over the hovered one and the
//!   disabled fill over both, as egui, which 0.7.2's egui connector letters
//!   from these fields, takes a pressed widget's look before a hovered one's
//!   and a widget that takes no input's before both (egui 0.37.0
//!   `src/style.rs`, `Widgets::style`); the primary fill over the hovered and
//!   the pressed ones, as 0.7.2's connectors keep `primary_background` in
//!   every state (egui: "regardless of state"), and the disabled fill over
//!   it; the label's disabled colour over the primary text;
//! - the check box's and the radio button's indicator: the checked fill and
//!   outline over the hovered fill, as 0.7.2's egui widgets draw a checked
//!   box or circle in its checked look whatever the pointer does (the
//!   `Selected` variant "while it is checked":
//!   `connectors/native-theme-egui-widgets/src/radio_button.rs`);
//! - the list's item label: the disabled colour over the selection's text,
//!   by egui's order above;
//! - the scroll bar's handle: the pressed fill over the hovered one, by
//!   egui's order above.
//!
//! An inactive window (`ThemeContext::window_active` false) has the inactive
//! title bar: `macos-monterey`'s own, `macos-tahoe`'s the active one (0.7.2:
//! `window.inactive_title_bar_background` is `window.title_bar_background`
//! where unstated, `resolve_widget_to_widget`).
//!
//! Of the context it acts on the scheme and on the text scale: every font
//! size, converted at the font DPI ([`FONT_DPI`]), times the text scale, not
//! rounded, as macOS draws fonts at fractional sizes (0.7.2 states no
//! whole-pixel rounding for it), and every line height times it too; each
//! `defined_size` stays the unscaled size (spec §6, resolution step 2). The
//! motion preference collapses or scales the motion tokens, of which the
//! presets state none. The contrast, the direction and the accessibility
//! flags change nothing here: the presets state one contrast level, and
//! every per-side value the ported widgets state is the same on the left and
//! on the right.
//!
//! Short source names in the citations, from the TOMLs' headers: "probe" in
//! a `macos-monterey` constant's citation is the OS probe's report on macOS
//! 14.8.9 and 15.7.9, `docs/evidence/os-probe/2026-10-11-run-38129819806/`
//! `os-probe-macos-14/probe.toml` and `os-probe-macos-15/probe.toml`, which
//! report the same value on every line cited; "probe" in a `macos-tahoe`
//! constant's citation, and "probe 26" in either, is its report on macOS
//! 26.6.2, `os-probe-macos-26/probe.toml` of the same run (os-probe-xcode-27,
//! macOS 27.0.1, reports the same colour or size on every line cited). The
//! probe resolves each NSColor per appearance with Reduce Transparency on;
//! a colour with alpha is stated as the API gives it, and its citation gives
//! the composite over the colour it sits on. A `macos-tahoe` value whose
//! citation says "no macOS 26 source" is one no source gives for macOS 26:
//! it carries `macos-monterey`'s value.
//!
//! Sources: `macos-monterey.toml` and `macos-tahoe.toml` of 0.7.2
//! (`src/presets/`), each constant with the citation its TOML gives it
//! (`docs/platform-facts.md` by section, the upstream source or the probe)
//! or `(uncited)`; 0.7.2's inheritance rules (`src/resolve/inheritance.rs`,
//! `src/resolve/validate.rs`, `src/resolve/validate_helpers.rs`) where 0.7.2
//! fills a value the TOML does not state.

use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{Corners, Edges, Stroke};
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::LayoutMetrics;
use crate::next::motion::{CaretSpec, MotionTokens, UiTimings};
use crate::next::paint::Paint;
use crate::next::preset::{Preset, ResolveInputs, nothing_stated};
use crate::next::presets::helpers::{linear_font_px, motion, motion_tokens};
use crate::next::query::{ButtonVariant, Contrast, Orientation, Scheme, ThemeContext, WidgetState};
use crate::next::roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
use crate::next::rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, Patched, RuleBuilder, RuleSelector, TextProp,
};
use crate::next::style::{
    DecorationKind, FocusIndicator, FocusKind, FontSize, FontStyle, IconStyle, PartStyle,
    TextDecoration, TextStated, TextStyle,
};
use crate::next::typography::{TextRendering, Typography};
use crate::next::widgets::{
    ButtonWidget, CheckboxWidget, InputWidget, LinkWidget, ListWidget, RadioWidget, ResolvedTheme,
    ScrollbarWidget, TooltipWidget, WidgetOrigin,
};
use crate::next::window::WindowChrome;

// ---------------------------------------------------------------------------
// The constants
// ---------------------------------------------------------------------------

/// The colours of one preset's scheme, each from its 8-bit sRGB channels
/// ([`srgb8`]) as the preset's TOML states it; `None` where that preset's
/// TOML does not state it.
#[derive(Clone, Copy)]
struct SchemeColors {
    background: Color,
    foreground: Color,
    muted: Color,
    border: Color,
    surface: Color,
    accent: Color,
    accent_foreground: Color,
    shadow: Color,
    link: Color,
    selection_background: Color,
    selection_foreground: Color,
    selection_inactive_background: Color,
    selection_inactive_foreground: Color,
    text_selection_background: Color,
    text_selection_foreground: Color,
    disabled_text: Color,
    focus_ring: Color,
    error: Color,
    warning: Color,
    success: Color,
    info: Color,
    status_foreground: Color,
    button_background: Color,
    button_hover: Color,
    button_pressed: Option<Color>,
    button_disabled: Color,
    button_foreground: Color,
    checkbox_hover: Option<Color>,
    checkbox_unchecked: Color,
    checkbox_unchecked_border: Color,
    input_background: Color,
    caret: Color,
    input_disabled: Option<Color>,
    input_foreground: Color,
    placeholder: Color,
    link_background: Color,
    link_hover_background: Color,
    list_alternate: Color,
    list_background: Color,
    list_grid: Color,
    list_header_background: Color,
    list_header_foreground: Color,
    scrollbar_thumb_pressed: Option<Color>,
    scrollbar_thumb_hover: Option<Color>,
    scrollbar_track: Color,
    tooltip_background: Color,
    tooltip_foreground: Color,
    title_bar_background: Color,
    title_bar_inactive_background: Option<Color>,
}

/// `macos-monterey`'s light scheme.
const MONTEREY_LIGHT: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md: windowBackgroundColor, probe
    background: srgb8(0xecec_ecff),
    // `defaults.text_color`: platform-facts.md: labelColor, probe; over background_color #242424
    foreground: srgb8(0x0000_00d8),
    // `defaults.muted_color`: platform-facts.md: secondaryLabelColor, probe; over background_color
    // #767676
    muted: srgb8(0x0000_007f),
    // `defaults.border.color`: platform-facts.md (§2.1.3): separatorColor, probe; over
    // background_color #d5d5d5
    border: srgb8(0x0000_0019),
    // `defaults.surface_color`: platform-facts.md: controlBackgroundColor, probe
    surface: srgb8(0xffff_ffff),
    // `defaults.accent_color`: platform-facts.md (§2.1.3): controlAccentColor, probe (the same in
    // darkAqua)
    accent: srgb8(0x007a_ffff),
    // `defaults.accent_text_color`: platform-facts.md: alternateSelectedControlTextColor, probe
    accent_foreground: srgb8(0xffff_ffff),
    // `defaults.shadow_color`: platform-facts.md: NSShadow.shadowColor's default, black at 1/3
    // (85/255, the nearest 8-bit alpha)
    shadow: srgb8(0x0000_0055),
    // `defaults.link_color`: platform-facts.md: linkColor, probe
    link: srgb8(0x0068_daff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): selection_background,
    // selection_text_color, selection_inactive_background, selection_inactive_text_color,
    // text_selection_background and text_selection_color: selectedContentBackgroundColor,
    // alternateSelectedControlTextColor, unemphasizedSelectedContentBackgroundColor and labelColor
    // on it, selectedTextBackgroundColor and selectedTextColor, probe; labelColor's measured on
    // macOS 15 (os-probe run 42215651196)
    selection_background: srgb8(0x0064_e1ff),
    // `defaults.selection_text_color`: 0.7.2 macos-monterey.toml (uncited)
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.selection_inactive_background`: 0.7.2 macos-monterey.toml (uncited)
    selection_inactive_background: srgb8(0xdcdc_dcff),
    // `defaults.selection_inactive_text_color`: 0.7.2 macos-monterey.toml (uncited)
    selection_inactive_foreground: srgb8(0x0000_00d8),
    // `defaults.text_selection_background`: 0.7.2 macos-monterey.toml (uncited)
    text_selection_background: srgb8(0xb3d7_ffff),
    // `defaults.text_selection_color`: 0.7.2 macos-monterey.toml (uncited)
    text_selection_foreground: srgb8(0x0000_00ff),
    // `defaults.disabled_text_color`: platform-facts.md: disabledControlTextColor, probe; over
    // background_color #b2b2b2
    disabled_text: srgb8(0x0000_003f),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): keyboardFocusIndicatorColor, probe;
    // over background_color #76aaf0
    focus_ring: srgb8(0x0067_f47f),
    // `defaults.danger_color`: platform-facts.md (§2.1.4): systemRedColor, systemOrangeColor,
    // systemGreenColor and systemBlueColor, probe; each text colour is labelColor (probe), the body
    // text beside a status mark, not text on the status fill (footnote 1)
    error: srgb8(0xff3b_30ff),
    // `defaults.warning_color`: 0.7.2 macos-monterey.toml (uncited)
    warning: srgb8(0xff95_00ff),
    // `defaults.success_color`: 0.7.2 macos-monterey.toml (uncited)
    success: srgb8(0x28cd_41ff),
    // `defaults.info_color`: 0.7.2 macos-monterey.toml (uncited)
    info: srgb8(0x007a_ffff),
    // `defaults.{danger,warning,success,info}_text_color`: 0.7.2 macos-monterey.toml (uncited)
    status_foreground: srgb8(0x0000_00d8),
    // `button.background_color`: platform-facts.md (§2.3): controlColor, probe
    button_background: srgb8(0xffff_ffff),
    // `button.hover_background`: Interactive state colors (platform-facts.md section 2.3 macOS)
    button_hover: srgb8(0xe0e0_e5ff),
    // `button.active_background`: soft_option: CoreUI pressed highlight -- slightly darker than
    // hover
    button_pressed: Some(srgb8(0xd6d6_dbff)),
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, white at
    // 128/255 (measured, os-probe run 40348854007,
    // renders/aqua/push_button-regular-disabled@2x.png, not key); over background_color #f6f6f6
    button_disabled: srgb8(0xffff_ff80),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #272727
    button_foreground: srgb8(0x0000_00d8),
    // `checkbox.hover_background`: soft_option: checkbox hover state -- subtle highlight
    checkbox_hover: Some(srgb8(0xe0e0_e5ff)),
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): ← background_color, the unchecked
    // box measured white on macOS 14 and 15 (os-probe run 40348854007,
    // renders/aqua/checkbox-regular-off@2x.png; the radio's the same)
    checkbox_unchecked: srgb8(0xffff_ffff),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): ← border.color, the box's 1 pt
    // edge, #d6d6d6 on the sides and the bottom, darker at the top (#c3c3c3), which one colour
    // cannot carry (measured, os-probe run 40348854007, renders/aqua/checkbox-regular-off@2x.png;
    // the radio's within 4 levels)
    checkbox_unchecked_border: srgb8(0xd6d6_d6ff),
    // `input.background_color`: platform-facts.md (§2.4): textBackgroundColor, probe
    input_background: srgb8(0xffff_ffff),
    // `input.caret_color`: platform-facts.md (§2.4): textInsertionPointColor, probe, the same in
    // darkAqua
    caret: srgb8(0x007a_ffff),
    // `input.disabled_background`: soft_option: disabled text field bg -- surface with reduced
    // alpha
    input_disabled: Some(srgb8(0xffff_ff80)),
    // `input.font.color`: platform-facts.md (§2.4): textColor, probe
    input_foreground: srgb8(0x0000_00ff),
    // `input.placeholder_color`: platform-facts.md: placeholderTextColor, probe; over the field's
    // background_color #c0c0c0
    placeholder: srgb8(0x0000_003f),
    // `link.background_color`: 0.7.2 macos-monterey.toml (uncited)
    link_background: srgb8(0x0000_0000),
    // `link.hover_background`: platform-facts.md (§2.28): macOS hover_background **(none)**,
    // transparent as background_color
    link_hover_background: srgb8(0x0000_0000),
    // `list.alternate_row_background`: platform-facts.md (§2.15):
    // alternatingContentBackgroundColors[1], probe
    list_alternate: srgb8(0xf4f5_f5ff),
    // `list.background_color`: platform-facts.md (§2.15): controlBackgroundColor, probe,
    // alternatingContentBackgroundColors[0]
    list_background: srgb8(0xffff_ffff),
    // `list.grid_color`: platform-facts.md: gridColor, probe
    list_grid: srgb8(0xe6e6_e6ff),
    // `list.header_background`: platform-facts.md (§2.15): the table's own fill in a key window,
    // measured on macOS 14 and 15, which agree (os-probe run 40320895749,
    // os-probe-macos-14/probe.toml)
    list_header_background: srgb8(0xffff_ffff),
    // `list.header_font.color`: platform-facts.md (§2.15): headerTextColor, probe
    list_header_foreground: srgb8(0x0000_00d8),
    // `scrollbar.thumb_active_color`: soft_option: pressed scrollbar thumb -- darker than hover
    scrollbar_thumb_pressed: Some(srgb8(0x5050_5090)),
    // `scrollbar.thumb_hover_color`: Interactive state colors (platform-facts.md section 2.8 macOS)
    // measured Sonoma scrollbar thumb hover -- NOT defaults.muted_color
    scrollbar_thumb_hover: Some(srgb8(0x6060_6080)),
    // `scrollbar.track_color`: platform-facts.md (§2.8): transparent in overlay mode
    scrollbar_track: srgb8(0x0000_0000),
    // `tooltip.background_color`: 0.7.2 macos-monterey.toml (uncited)
    tooltip_background: srgb8(0x2c2c_2eff),
    // `tooltip.font.color`: 0.7.2 macos-monterey.toml (uncited)
    tooltip_foreground: srgb8(0xffff_ffff),
    // `window.title_bar_background`: platform-facts.md (§2.2): a key window's title bar with Reduce
    // Transparency on, measured on macOS 14 and 15, which agree (os-probe run 40320895749,
    // os-probe-macos-14/probe.toml)
    title_bar_background: srgb8(0xffff_ffff),
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): the title bar of a window
    // that is not key while another window of the application is, measured on macOS 14 and 15,
    // which agree (os-probe run 40324458365, os-probe-macos-14/not-key/probe.toml)
    title_bar_inactive_background: Some(srgb8(0xecec_ecff)),
};

/// `macos-monterey`'s dark scheme.
const MONTEREY_DARK: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md: windowBackgroundColor, probe
    background: srgb8(0x3232_32ff),
    // `defaults.text_color`: platform-facts.md: labelColor, probe; over background_color #e0e0e0
    foreground: srgb8(0xffff_ffd8),
    // `defaults.muted_color`: platform-facts.md: secondaryLabelColor, probe; over background_color
    // #a3a3a3
    muted: srgb8(0xffff_ff8c),
    // `defaults.border.color`: platform-facts.md (§2.1.3): separatorColor, probe; over
    // background_color #464646
    border: srgb8(0xffff_ff19),
    // `defaults.surface_color`: platform-facts.md: controlBackgroundColor, probe
    surface: srgb8(0x1e1e_1eff),
    // `defaults.accent_color`: platform-facts.md (§2.1.3): controlAccentColor, probe, the same as
    // in Aqua, as on macOS 26.6.2 and 27.0.1 (os-probe-macos-26 and os-probe-xcode-27, probe.toml)
    accent: srgb8(0x007a_ffff),
    // `defaults.accent_text_color`: platform-facts.md: alternateSelectedControlTextColor, probe
    accent_foreground: srgb8(0xffff_ffff),
    // `defaults.shadow_color`: platform-facts.md: NSShadow.shadowColor's default, black at 1/3
    // (85/255, the nearest 8-bit alpha)
    shadow: srgb8(0x0000_0055),
    // `defaults.link_color`: platform-facts.md: linkColor, probe
    link: srgb8(0x419c_ffff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): selection_background,
    // selection_text_color, selection_inactive_background, selection_inactive_text_color,
    // text_selection_background and text_selection_color: selectedContentBackgroundColor,
    // alternateSelectedControlTextColor, unemphasizedSelectedContentBackgroundColor and labelColor
    // on it, selectedTextBackgroundColor and selectedTextColor, probe; labelColor's measured on
    // macOS 15 (os-probe run 42215651196)
    selection_background: srgb8(0x0059_d1ff),
    // `defaults.selection_text_color`: 0.7.2 macos-monterey.toml (uncited)
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.selection_inactive_background`: 0.7.2 macos-monterey.toml (uncited)
    selection_inactive_background: srgb8(0x4646_46ff),
    // `defaults.selection_inactive_text_color`: 0.7.2 macos-monterey.toml (uncited)
    selection_inactive_foreground: srgb8(0xffff_ffd8),
    // `defaults.text_selection_background`: 0.7.2 macos-monterey.toml (uncited)
    text_selection_background: srgb8(0x3f63_8bff),
    // `defaults.text_selection_color`: 0.7.2 macos-monterey.toml (uncited)
    text_selection_foreground: srgb8(0xffff_ffff),
    // `defaults.disabled_text_color`: platform-facts.md: disabledControlTextColor, probe; over
    // background_color #656565
    disabled_text: srgb8(0xffff_ff3f),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): keyboardFocusIndicatorColor, probe;
    // over background_color #266d98
    focus_ring: srgb8(0x1aa9_ff7f),
    // `defaults.danger_color`: platform-facts.md (§2.1.4): systemRedColor, systemOrangeColor,
    // systemGreenColor and systemBlueColor, probe; each text colour is labelColor (probe), the body
    // text beside a status mark, not text on the status fill (footnote 1)
    error: srgb8(0xff45_3aff),
    // `defaults.warning_color`: 0.7.2 macos-monterey.toml (uncited)
    warning: srgb8(0xff9f_0aff),
    // `defaults.success_color`: 0.7.2 macos-monterey.toml (uncited)
    success: srgb8(0x32d7_4bff),
    // `defaults.info_color`: 0.7.2 macos-monterey.toml (uncited)
    info: srgb8(0x0a84_ffff),
    // `defaults.{danger,warning,success,info}_text_color`: 0.7.2 macos-monterey.toml (uncited)
    status_foreground: srgb8(0xffff_ffd8),
    // `button.background_color`: platform-facts.md (§2.3): controlColor, probe; over the window
    // background #656565
    button_background: srgb8(0xffff_ff3f),
    // `button.hover_background`: Interactive state colors (platform-facts.md section 2.3 macOS)
    button_hover: srgb8(0x4444_46ff),
    // `button.active_background`: soft_option: CoreUI pressed highlight -- brighter than hover
    button_pressed: Some(srgb8(0x4e4e_50ff)),
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, white at
    // 32/255 (measured, os-probe run 40348854007,
    // renders/darkAqua/push_button-regular-disabled@2x.png, not key); over background_color #4c4c4c
    button_disabled: srgb8(0xffff_ff20),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #e7e7e7
    button_foreground: srgb8(0xffff_ffd8),
    // `checkbox.hover_background`: soft_option: checkbox hover state -- subtle highlight
    checkbox_hover: Some(srgb8(0x4444_46ff)),
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): ← background_color, white at alpha
    // 55/255 at the box's centre, a gradient from 40/255 at its top to 71/255 at its bottom, not
    // controlBackgroundColor (measured, os-probe run 40348854007,
    // renders/darkAqua/checkbox-regular-off@2x.png; the radio's the same); over background_color
    // #5e5e5e
    checkbox_unchecked: srgb8(0xffff_ff37),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): ← border.color, none: the fill
    // runs to the box's edge, a faint dark rim outside it (measured, the same render)
    checkbox_unchecked_border: srgb8(0x0000_0000),
    // `input.background_color`: platform-facts.md (§2.4): textBackgroundColor, probe
    input_background: srgb8(0x1e1e_1eff),
    // `input.caret_color`: platform-facts.md (§2.4): textInsertionPointColor, probe
    caret: srgb8(0x007a_ffff),
    // `input.disabled_background`: soft_option, no source: disabled text field bg -- the former
    // dark surface #2d2d2d at alpha 0x80
    input_disabled: Some(srgb8(0x2d2d_2d80)),
    // `input.font.color`: platform-facts.md (§2.4): textColor, probe
    input_foreground: srgb8(0xffff_ffff),
    // `input.placeholder_color`: platform-facts.md: placeholderTextColor, probe; over the field's
    // background_color #565656
    placeholder: srgb8(0xffff_ff3f),
    // `link.background_color`: 0.7.2 macos-monterey.toml (uncited)
    link_background: srgb8(0x0000_0000),
    // `link.hover_background`: platform-facts.md (§2.28): macOS hover_background **(none)**,
    // transparent as background_color
    link_hover_background: srgb8(0x0000_0000),
    // `list.alternate_row_background`: platform-facts.md (§2.15):
    // alternatingContentBackgroundColors[1], probe; over the list's background_color #292929
    list_alternate: srgb8(0xffff_ff0c),
    // `list.background_color`: platform-facts.md (§2.15): controlBackgroundColor, probe,
    // alternatingContentBackgroundColors[0]
    list_background: srgb8(0x1e1e_1eff),
    // `list.grid_color`: platform-facts.md: gridColor, probe
    list_grid: srgb8(0x1a1a_1aff),
    // `list.header_background`: platform-facts.md (§2.15): the table's own fill in a key window,
    // measured on macOS 14 and 15, which agree (os-probe run 40320895749,
    // os-probe-macos-14/probe.toml)
    list_header_background: srgb8(0x1e1e_1eff),
    // `list.header_font.color`: platform-facts.md (§2.15): headerTextColor, probe
    list_header_foreground: srgb8(0xffff_ffff),
    // `scrollbar.thumb_active_color`: soft_option: pressed scrollbar thumb -- brighter than hover
    scrollbar_thumb_pressed: Some(srgb8(0xb0b0_b090)),
    // `scrollbar.thumb_hover_color`: Interactive state colors (platform-facts.md section 2.8 macOS)
    // measured Sonoma scrollbar thumb hover -- NOT defaults.muted_color
    scrollbar_thumb_hover: Some(srgb8(0xa0a0_a080)),
    // `scrollbar.track_color`: platform-facts.md (§2.8): transparent in overlay mode
    scrollbar_track: srgb8(0x0000_0000),
    // `tooltip.background_color`: 0.7.2 macos-monterey.toml (uncited)
    tooltip_background: srgb8(0x3a3a_3cff),
    // `tooltip.font.color`: 0.7.2 macos-monterey.toml (uncited)
    tooltip_foreground: srgb8(0xffff_ffff),
    // `window.title_bar_background`: platform-facts.md (§2.2): a key window's title bar with Reduce
    // Transparency on, measured on macOS 14 and 15, which agree (os-probe run 40320895749,
    // os-probe-macos-14/probe.toml)
    title_bar_background: srgb8(0x2626_26ff),
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): the title bar of a window
    // that is not key while another window of the application is, measured on macOS 14 and 15,
    // which agree (os-probe run 40324458365, os-probe-macos-14/not-key/probe.toml)
    title_bar_inactive_background: Some(srgb8(0x1414_14ff)),
};

/// `macos-tahoe`'s light scheme.
const TAHOE_LIGHT: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md: windowBackgroundColor, probe
    background: srgb8(0xffff_ffff),
    // `defaults.text_color`: platform-facts.md: labelColor, probe; over background_color #272727
    foreground: srgb8(0x0000_00d8),
    // `defaults.muted_color`: platform-facts.md: secondaryLabelColor, probe; over background_color
    // #808080
    muted: srgb8(0x0000_007f),
    // `defaults.border.color`: platform-facts.md (§2.1.3): separatorColor, probe; over
    // background_color #e6e6e6
    border: srgb8(0x0000_0019),
    // `defaults.surface_color`: platform-facts.md: controlBackgroundColor, probe
    surface: srgb8(0xffff_ffff),
    // `defaults.accent_color`: platform-facts.md (§2.1.3): controlAccentColor, probe (the same in
    // darkAqua)
    accent: srgb8(0x007a_ffff),
    // `defaults.accent_text_color`: platform-facts.md: alternateSelectedControlTextColor, probe
    accent_foreground: srgb8(0xffff_ffff),
    // `defaults.shadow_color`: platform-facts.md: NSShadow.shadowColor's default, black at 1/3
    // (85/255, the nearest 8-bit alpha)
    shadow: srgb8(0x0000_0055),
    // `defaults.link_color`: platform-facts.md: linkColor, probe
    link: srgb8(0x0068_daff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): selection_background,
    // selection_text_color, selection_inactive_background, selection_inactive_text_color,
    // text_selection_background and text_selection_color: selectedContentBackgroundColor,
    // alternateSelectedControlTextColor, unemphasizedSelectedContentBackgroundColor and labelColor
    // on it, selectedTextBackgroundColor and selectedTextColor, probe; labelColor's measured on
    // macOS 26 (os-probe run 41195846264)
    selection_background: srgb8(0x0064_e1ff),
    // `defaults.selection_text_color`: 0.7.2 macos-tahoe.toml (uncited)
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.selection_inactive_background`: 0.7.2 macos-tahoe.toml (uncited)
    selection_inactive_background: srgb8(0xdcdc_dcff),
    // `defaults.selection_inactive_text_color`: 0.7.2 macos-tahoe.toml (uncited)
    selection_inactive_foreground: srgb8(0x0000_00d8),
    // `defaults.text_selection_background`: 0.7.2 macos-tahoe.toml (uncited)
    text_selection_background: srgb8(0xb3d7_ffff),
    // `defaults.text_selection_color`: 0.7.2 macos-tahoe.toml (uncited)
    text_selection_foreground: srgb8(0x0000_00ff),
    // `defaults.disabled_text_color`: platform-facts.md: disabledControlTextColor, probe; over
    // background_color #c0c0c0
    disabled_text: srgb8(0x0000_003f),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): keyboardFocusIndicatorColor, probe;
    // over background_color #80b3fa
    focus_ring: srgb8(0x0067_f47f),
    // `defaults.danger_color`: platform-facts.md (§2.1.4): systemRedColor, systemOrangeColor,
    // systemGreenColor and systemBlueColor, probe; each text colour is labelColor (probe), the body
    // text beside a status mark, not text on the status fill (footnote 1)
    error: srgb8(0xff38_3cff),
    // `defaults.warning_color`: 0.7.2 macos-tahoe.toml (uncited)
    warning: srgb8(0xff8d_28ff),
    // `defaults.success_color`: 0.7.2 macos-tahoe.toml (uncited)
    success: srgb8(0x34c7_59ff),
    // `defaults.info_color`: 0.7.2 macos-tahoe.toml (uncited)
    info: srgb8(0x0088_ffff),
    // `defaults.{danger,warning,success,info}_text_color`: 0.7.2 macos-tahoe.toml (uncited)
    status_foreground: srgb8(0x0000_00d8),
    // `button.background_color`: platform-facts.md (§2.3): drawn in secondarySystemFillColor's
    // value (measured; probe); over background_color #ebebeb
    button_background: srgb8(0x0000_0014),
    // `button.hover_background`: no macOS 26 source (platform-facts.md): macos-monterey's value;
    // the inherited defaults.background_color (inheritance-rules.toml, a wrong safety net) is the
    // window's opaque colour, which would hide the translucent button on hover
    button_hover: srgb8(0xe0e0_e5ff),
    button_pressed: None,
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, black at
    // 10/255 (measured, os-probe run 40348854007, renders/aqua/push_button-regular-disabled@2x.png,
    // not key); over background_color #f5f5f5
    button_disabled: srgb8(0x0000_000a),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #242424
    button_foreground: srgb8(0x0000_00d8),
    checkbox_hover: None,
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): macOS 26: the unchecked box is one
    // fill, systemFillColor's value #00000019 (measured, renders/aqua/checkbox-regular-off@2x.png;
    // probe); the probe's window is never ordered front, so this is the box of a window that is not
    // key
    checkbox_unchecked: srgb8(0x0000_0019),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): macOS 26: none (measured), no
    // edge pixel differs from the fill; the same caveat
    checkbox_unchecked_border: srgb8(0x0000_0000),
    // `input.background_color`: platform-facts.md (§2.4): textBackgroundColor, probe
    input_background: srgb8(0xffff_ffff),
    // `input.caret_color`: platform-facts.md (§2.4): textInsertionPointColor, probe
    caret: srgb8(0x007a_ffff),
    input_disabled: None,
    // `input.font.color`: platform-facts.md (§2.4): textColor, probe
    input_foreground: srgb8(0x0000_00ff),
    // `input.placeholder_color`: platform-facts.md: placeholderTextColor, probe; over the field's
    // background_color #808080
    placeholder: srgb8(0x0000_007f),
    // `link.background_color`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    link_background: srgb8(0x0000_0000),
    // `link.hover_background`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    link_hover_background: srgb8(0x0000_0000),
    // `list.alternate_row_background`: platform-facts.md (§2.15):
    // alternatingContentBackgroundColors[1], probe
    list_alternate: srgb8(0xf4f5_f5ff),
    // `list.background_color`: platform-facts.md (§2.15): controlBackgroundColor, probe,
    // alternatingContentBackgroundColors[0]
    list_background: srgb8(0xffff_ffff),
    // `list.grid_color`: platform-facts.md: gridColor, probe
    list_grid: srgb8(0xe6e6_e6ff),
    // `list.header_background`: platform-facts.md (§2.15): the table's own fill in a key window,
    // measured on macOS 26 and xcode-27's macOS 27, which agree (os-probe run 40320895749,
    // os-probe-macos-26/probe.toml)
    list_header_background: srgb8(0xffff_ffff),
    // `list.header_font.color`: platform-facts.md (§2.15): headerTextColor, probe
    list_header_foreground: srgb8(0x0000_00d8),
    scrollbar_thumb_pressed: None,
    scrollbar_thumb_hover: None,
    // `scrollbar.track_color`: no macOS 26 source (platform-facts.md): macos-monterey's value, the
    // overlay mode's transparent track; the inherited window background would be the wrong kind
    scrollbar_track: srgb8(0x0000_0000),
    // `tooltip.background_color`: no macOS 26 source (platform-facts.md): macos-monterey's value;
    // the inherited window background would be the wrong kind
    tooltip_background: srgb8(0x2c2c_2eff),
    // `tooltip.font.color`: no macOS 26 source (platform-facts.md): macos-monterey's value; the
    // inherited label colour would be the wrong kind
    tooltip_foreground: srgb8(0xffff_ffff),
    // `window.title_bar_background`: platform-facts.md (§2.2): a key window's title bar with Reduce
    // Transparency on, measured on macOS 26 and xcode-27's macOS 27, which agree (os-probe run
    // 40320895749, os-probe-macos-26/probe.toml)
    title_bar_background: srgb8(0xffff_ffff),
    title_bar_inactive_background: None,
};

/// `macos-tahoe`'s dark scheme.
const TAHOE_DARK: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md: windowBackgroundColor, probe
    background: srgb8(0x1e1e_1eff),
    // `defaults.text_color`: platform-facts.md: labelColor, probe; over background_color #dddddd
    foreground: srgb8(0xffff_ffd8),
    // `defaults.muted_color`: platform-facts.md: secondaryLabelColor, probe; over background_color
    // #9a9a9a
    muted: srgb8(0xffff_ff8c),
    // `defaults.border.color`: platform-facts.md (§2.1.3): separatorColor, probe; over
    // background_color #343434
    border: srgb8(0xffff_ff19),
    // `defaults.surface_color`: platform-facts.md: controlBackgroundColor, probe
    surface: srgb8(0x1e1e_1eff),
    // `defaults.accent_color`: platform-facts.md (§2.1.3): controlAccentColor, probe, the same as
    // in Aqua
    accent: srgb8(0x007a_ffff),
    // `defaults.accent_text_color`: platform-facts.md: alternateSelectedControlTextColor, probe
    accent_foreground: srgb8(0xffff_ffff),
    // `defaults.shadow_color`: platform-facts.md: NSShadow.shadowColor's default, black at 1/3
    // (85/255, the nearest 8-bit alpha)
    shadow: srgb8(0x0000_0055),
    // `defaults.link_color`: platform-facts.md: linkColor, probe
    link: srgb8(0x419c_ffff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): selection_background,
    // selection_text_color, selection_inactive_background, selection_inactive_text_color,
    // text_selection_background and text_selection_color: selectedContentBackgroundColor,
    // alternateSelectedControlTextColor, unemphasizedSelectedContentBackgroundColor and labelColor
    // on it, selectedTextBackgroundColor and selectedTextColor, probe; labelColor's measured on
    // macOS 26 (os-probe run 41195846264)
    selection_background: srgb8(0x0059_d1ff),
    // `defaults.selection_text_color`: 0.7.2 macos-tahoe.toml (uncited)
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.selection_inactive_background`: 0.7.2 macos-tahoe.toml (uncited)
    selection_inactive_background: srgb8(0x4646_46ff),
    // `defaults.selection_inactive_text_color`: 0.7.2 macos-tahoe.toml (uncited)
    selection_inactive_foreground: srgb8(0xffff_ffd8),
    // `defaults.text_selection_background`: 0.7.2 macos-tahoe.toml (uncited)
    text_selection_background: srgb8(0x3f63_8bff),
    // `defaults.text_selection_color`: 0.7.2 macos-tahoe.toml (uncited)
    text_selection_foreground: srgb8(0xffff_ffff),
    // `defaults.disabled_text_color`: platform-facts.md: disabledControlTextColor, probe; over
    // background_color #565656
    disabled_text: srgb8(0xffff_ff3f),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): keyboardFocusIndicatorColor, probe;
    // over background_color #1c638e
    focus_ring: srgb8(0x1aa9_ff7f),
    // `defaults.danger_color`: platform-facts.md (§2.1.4): systemRedColor, systemOrangeColor,
    // systemGreenColor and systemBlueColor, probe; each text colour is labelColor (probe), the body
    // text beside a status mark, not text on the status fill (footnote 1)
    error: srgb8(0xff42_45ff),
    // `defaults.warning_color`: 0.7.2 macos-tahoe.toml (uncited)
    warning: srgb8(0xff92_30ff),
    // `defaults.success_color`: 0.7.2 macos-tahoe.toml (uncited)
    success: srgb8(0x30d1_58ff),
    // `defaults.info_color`: 0.7.2 macos-tahoe.toml (uncited)
    info: srgb8(0x0091_ffff),
    // `defaults.{danger,warning,success,info}_text_color`: 0.7.2 macos-tahoe.toml (uncited)
    status_foreground: srgb8(0xffff_ffd8),
    // `button.background_color`: platform-facts.md (§2.3): drawn in secondarySystemFillColor's
    // value (measured; probe); over background_color #303030
    button_background: srgb8(0xffff_ff14),
    // `button.hover_background`: no macOS 26 source (platform-facts.md): macos-monterey's value;
    // the inherited defaults.background_color (inheritance-rules.toml, a wrong safety net) is the
    // window's opaque colour, which would hide the translucent button on hover
    button_hover: srgb8(0x4444_46ff),
    button_pressed: None,
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, white at
    // 10/255 (measured, os-probe run 40348854007,
    // renders/darkAqua/push_button-regular-disabled@2x.png, not key); over background_color #272727
    button_disabled: srgb8(0xffff_ff0a),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #dfdfdf
    button_foreground: srgb8(0xffff_ffd8),
    checkbox_hover: None,
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): macOS 26: the unchecked box is one
    // fill, systemFillColor's value #ffffff19 (measured,
    // renders/darkAqua/checkbox-regular-off@2x.png; probe); the probe's window is never ordered
    // front, so this is the box of a window that is not key
    checkbox_unchecked: srgb8(0xffff_ff19),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): macOS 26: none (measured), no
    // edge pixel differs from the fill; the same caveat
    checkbox_unchecked_border: srgb8(0x0000_0000),
    // `input.background_color`: platform-facts.md (§2.4): textBackgroundColor, probe
    input_background: srgb8(0x1e1e_1eff),
    // `input.caret_color`: platform-facts.md (§2.4): textInsertionPointColor, probe
    caret: srgb8(0x007a_ffff),
    input_disabled: None,
    // `input.font.color`: platform-facts.md (§2.4): textColor, probe
    input_foreground: srgb8(0xffff_ffff),
    // `input.placeholder_color`: platform-facts.md: placeholderTextColor, probe; over the field's
    // background_color #9a9a9a
    placeholder: srgb8(0xffff_ff8c),
    // `link.background_color`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    link_background: srgb8(0x0000_0000),
    // `link.hover_background`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    link_hover_background: srgb8(0x0000_0000),
    // `list.alternate_row_background`: platform-facts.md (§2.15):
    // alternatingContentBackgroundColors[1], probe; over the list's background_color #292929
    list_alternate: srgb8(0xffff_ff0c),
    // `list.background_color`: platform-facts.md (§2.15): controlBackgroundColor, probe,
    // alternatingContentBackgroundColors[0]
    list_background: srgb8(0x1e1e_1eff),
    // `list.grid_color`: platform-facts.md: gridColor, probe
    list_grid: srgb8(0x1a1a_1aff),
    // `list.header_background`: platform-facts.md (§2.15): the table's own fill in a key window,
    // measured on macOS 26 and xcode-27's macOS 27, which agree (os-probe run 40320895749,
    // os-probe-macos-26/probe.toml)
    list_header_background: srgb8(0x1e1e_1eff),
    // `list.header_font.color`: platform-facts.md (§2.15): headerTextColor, probe
    list_header_foreground: srgb8(0xffff_ffff),
    scrollbar_thumb_pressed: None,
    scrollbar_thumb_hover: None,
    // `scrollbar.track_color`: no macOS 26 source (platform-facts.md): macos-monterey's value, the
    // overlay mode's transparent track; the inherited window background would be the wrong kind
    scrollbar_track: srgb8(0x0000_0000),
    // `tooltip.background_color`: no macOS 26 source (platform-facts.md): macos-monterey's value;
    // the inherited window background would be the wrong kind
    tooltip_background: srgb8(0x3a3a_3cff),
    // `tooltip.font.color`: no macOS 26 source (platform-facts.md): macos-monterey's value; the
    // inherited label colour would be the wrong kind
    tooltip_foreground: srgb8(0xffff_ffff),
    // `window.title_bar_background`: platform-facts.md (§2.2): a key window's title bar with Reduce
    // Transparency on, measured on macOS 26 and xcode-27's macOS 27, which agree (os-probe run
    // 40320895749, os-probe-macos-26/probe.toml)
    title_bar_background: srgb8(0x1e1e_1eff),
    title_bar_inactive_background: None,
};

/// One preset's constants that do not change with the scheme: its metrics,
/// fonts and flags, each as the preset's TOML states it (the light and the
/// dark variant state the same value); `None` where that preset's TOML
/// does not state it.
#[derive(Clone, Copy)]
struct Generation {
    preset: Preset,
    light: SchemeColors,
    dark: SchemeColors,
    icon_set: IconSet,
    icon_theme: &'static str,
    icon_toolbar: f32,
    icon_small: f32,
    icon_large: f32,
    icon_dialog: f32,
    icon_panel: f32,
    font_family: &'static str,
    font_size_pt: f32,
    font_weight: u16,
    mono_family: &'static str,
    mono_size_pt: f32,
    mono_weight: u16,
    line_height: f32,
    corner_radius: f32,
    corner_radius_lg: f32,
    border_width: f32,
    border_opacity: f32,
    shadow_enabled: bool,
    disabled_opacity: f32,
    focus_ring_width: f32,
    focus_ring_offset: f32,
    widget_gap: f32,
    container_margin: Option<f32>,
    window_margin: f32,
    section_gap: Option<f32>,
    caption_size_pt: f32,
    caption_weight: u16,
    section_heading_size_pt: f32,
    section_heading_weight: u16,
    dialog_title_size_pt: f32,
    dialog_title_weight: u16,
    display_size_pt: f32,
    display_weight: u16,
    button_min_width: f32,
    button_min_height: f32,
    button_padding_top: f32,
    button_padding_bottom: f32,
    button_padding_horizontal: f32,
    button_icon_text_gap: f32,
    button_shadow_enabled: Option<bool>,
    checkbox_shadow_enabled: bool,
    checkbox_indicator_width: f32,
    checkbox_label_gap: f32,
    radio_dot_diameter: f32,
    input_corner_radius: Option<f32>,
    input_padding_vertical: f32,
    input_padding_horizontal: f32,
    input_shadow_enabled: bool,
    input_min_height: f32,
    link_underline: bool,
    list_padding_vertical: Option<f32>,
    list_padding_horizontal: Option<f32>,
    list_header_height: f32,
    list_row_height: f32,
    scrollbar_groove_width: f32,
    scrollbar_min_thumb_length: f32,
    scrollbar_overlay: bool,
    scrollbar_thumb_width: f32,
    tooltip_padding_vertical: Option<f32>,
    tooltip_padding_horizontal: Option<f32>,
    tooltip_font_size_pt: f32,
    tooltip_max_width: f32,
    title_bar_font_weight: u16,
}

/// `macos-monterey`'s constants.
const MONTEREY: Generation = Generation {
    preset: Preset::MacosMonterey,
    light: MONTEREY_LIGHT,
    dark: MONTEREY_DARK,
    // `Resolved.icon_set`: 0.7.2 macos-monterey.toml (uncited)
    icon_set: IconSet::SfSymbols,
    // `Resolved.icon_theme`: 0.7.2 macos-monterey.toml (uncited)
    icon_theme: "sf-symbols",
    // `defaults.icon_sizes.toolbar`: toolbar icon_sizes = 24 matches the small toolbar icon mode.
    // Large mode uses 32.
    icon_toolbar: 24.0,
    // `defaults.icon_sizes.small`: 0.7.2 macos-monterey.toml (uncited)
    icon_small: 16.0,
    // `defaults.icon_sizes.large`: 0.7.2 macos-monterey.toml (uncited)
    icon_large: 32.0,
    // `defaults.icon_sizes.dialog`: 0.7.2 macos-monterey.toml (uncited)
    icon_dialog: 22.0,
    // `defaults.icon_sizes.panel`: 0.7.2 macos-monterey.toml (uncited)
    icon_panel: 20.0,
    // `defaults.font.family`: 0.7.2 macos-monterey.toml (uncited)
    font_family: "SF Pro",
    // `defaults.font.size`: 0.7.2 macos-monterey.toml (uncited)
    font_size_pt: 13.0,
    // `defaults.font.weight`: 0.7.2 macos-monterey.toml (uncited)
    font_weight: 400,
    // `defaults.mono_font.family`: 0.7.2 macos-monterey.toml (uncited)
    mono_family: "SF Mono",
    // `defaults.mono_font.size`: 0.7.2 macos-monterey.toml (uncited)
    mono_size_pt: 13.0,
    // `defaults.mono_font.weight`: 0.7.2 macos-monterey.toml (uncited)
    mono_weight: 400,
    // `defaults.line_height`: 0.7.2 macos-monterey.toml (uncited)
    line_height: 1.19,
    // `defaults.border.corner_radius`: 0.7.2 macos-monterey.toml (uncited)
    corner_radius: 5.0,
    // `defaults.border.corner_radius_lg`: 0.7.2 macos-monterey.toml (uncited)
    corner_radius_lg: 10.0,
    // `defaults.border.line_width`: 0.7.2 macos-monterey.toml (uncited)
    border_width: 0.5,
    // `defaults.border.opacity`: platform-facts.md (§2.1.6): separatorColor's alpha, 25/255
    // (probe), already in color
    border_opacity: 0.098,
    // `defaults.border.shadow_enabled`: 0.7.2 macos-monterey.toml (uncited)
    shadow_enabled: true,
    // `defaults.disabled_opacity`: platform-facts.md (§2.1.6): 1.0, macOS dims a disabled control
    // by colour (disabled_text_color)
    disabled_opacity: 1.0,
    // `defaults.focus_ring_width`: 0.7.2 macos-monterey.toml (uncited)
    focus_ring_width: 3.0,
    // `defaults.focus_ring_offset`: 0.7.2 macos-monterey.toml (uncited)
    focus_ring_offset: -1.0,
    // `layout.widget_gap`: 0.7.2 macos-monterey.toml (uncited)
    widget_gap: 8.0,
    // `layout.container_margin`: 0.7.2 macos-monterey.toml (uncited)
    container_margin: Some(8.0),
    // `layout.window_margin`: 0.7.2 macos-monterey.toml (uncited)
    window_margin: 20.0,
    // `layout.section_gap`: 0.7.2 macos-monterey.toml (uncited)
    section_gap: Some(20.0),
    // `text_scale.caption.size`: 0.7.2 macos-monterey.toml (uncited)
    caption_size_pt: 10.0,
    // `text_scale.caption.weight`: 0.7.2 macos-monterey.toml (uncited)
    caption_weight: 400,
    // `text_scale.section_heading.size`: 0.7.2 macos-monterey.toml (uncited)
    section_heading_size_pt: 13.0,
    // `text_scale.section_heading.weight`: 0.7.2 macos-monterey.toml (uncited)
    section_heading_weight: 700,
    // `text_scale.dialog_title.size`: 0.7.2 macos-monterey.toml (uncited)
    dialog_title_size_pt: 22.0,
    // `text_scale.dialog_title.weight`: 0.7.2 macos-monterey.toml (uncited)
    dialog_title_weight: 400,
    // `text_scale.display.size`: 0.7.2 macos-monterey.toml (uncited)
    display_size_pt: 26.0,
    // `text_scale.display.weight`: 0.7.2 macos-monterey.toml (uncited)
    display_weight: 400,
    // `button.min_width`: 0.7.2 macos-monterey.toml (uncited)
    button_min_width: 64.0,
    // `button.min_height`: platform-facts.md (§2.3): NSButton intrinsicContentSize 20 (macOS 14.8.9
    // probe)
    button_min_height: 20.0,
    // `button.border.padding.top`: 0.7.2 macos-monterey.toml (uncited)
    button_padding_top: 1.0,
    // `button.border.padding.bottom`: 0.7.2 macos-monterey.toml (uncited)
    button_padding_bottom: 3.0,
    // `button.border.padding.left`, `button.border.padding.right`: platform-facts.md (§2.3):
    // NSButton ~8 (WebKit); 1 top / 3 bottom (API, macOS 14.8.9 probe)
    button_padding_horizontal: 8.0,
    // `button.icon_text_gap`: 0.7.2 macos-monterey.toml (uncited)
    button_icon_text_gap: 4.0,
    button_shadow_enabled: None,
    // `checkbox.border.shadow_enabled`: platform-facts.md (§2.5): no shadow
    checkbox_shadow_enabled: false,
    // `checkbox.indicator_width`: platform-facts.md (§2.5): imageRectForBounds 14 x 14 (probe); the
    // title rect starts 6 after it (probe)
    checkbox_indicator_width: 14.0,
    // `checkbox.label_gap`: 0.7.2 macos-monterey.toml (uncited)
    checkbox_label_gap: 6.0,
    // `checkbox.radio_dot_diameter`: platform-facts.md (§2.5): the selected radio's dot, 6
    // (measured, os-probe run 40348854007, os-probe-macos-14/renders/aqua/radio-regular-on@2x.png,
    // in a window that is not key)
    radio_dot_diameter: 6.0,
    input_corner_radius: None,
    // `input.border.padding.top`, `input.border.padding.bottom`: 0.7.2 macos-monterey.toml
    // (uncited)
    input_padding_vertical: 2.0,
    // `input.border.padding.left`, `input.border.padding.right`: platform-facts.md (§2.4): the
    // title rect [2, 2, 156, 17] in the 160 x 21 field (probe), from its outer edge
    input_padding_horizontal: 2.0,
    // `input.border.shadow_enabled`: platform-facts.md (§2.4): no shadow
    input_shadow_enabled: false,
    // `input.min_height`: platform-facts.md (§2.4): NSTextField intrinsicContentSize 21 (probe)
    input_min_height: 21.0,
    // `link.underline_enabled`: 0.7.2 macos-monterey.toml (uncited)
    link_underline: true,
    // `list.border.padding.top`, `list.border.padding.bottom`: 0.7.2 macos-monterey.toml (uncited)
    list_padding_vertical: Some(4.0),
    // `list.border.padding.left`, `list.border.padding.right`: platform-facts.md (§2.15):
    // NSTableView 4; 4 (measured)
    list_padding_horizontal: Some(4.0),
    // `list.header_height`: platform-facts.md (§2.15): NSTableHeaderView height 28, .fullWidth,
    // macOS 14.8.9 and 15.7.9 (probe)
    list_header_height: 28.0,
    // `list.row_height`: 0.7.2 macos-monterey.toml (uncited)
    list_row_height: 24.0,
    // `scrollbar.groove_width`: platform-facts.md (§2.8): scrollerWidth overlay 16 (legacy 15),
    // macOS 14.8.9; overlay_mode below
    scrollbar_groove_width: 16.0,
    // `scrollbar.min_thumb_length`: no source for the overlay mode overlay_mode states:
    // platform-facts.md's 40 (measured) is the legacy mode's; the field is required
    scrollbar_min_thumb_length: 40.0,
    // `scrollbar.overlay_mode`: 0.7.2 macos-monterey.toml (uncited)
    scrollbar_overlay: true,
    // `scrollbar.thumb_width`: 0.7.2 macos-monterey.toml (uncited)
    scrollbar_thumb_width: 7.0,
    // `tooltip.border.padding.top`, `tooltip.border.padding.bottom`: 0.7.2 macos-monterey.toml
    // (uncited)
    tooltip_padding_vertical: Some(4.0),
    // `tooltip.border.padding.left`, `tooltip.border.padding.right`: platform-facts.md (§2.7):
    // NSToolTipManager 4; 4
    tooltip_padding_horizontal: Some(4.0),
    // `tooltip.font.size`: platform-facts.md (§1.1.1) (§2.7): toolTipsFontOfSize: 11, probe
    tooltip_font_size_pt: 11.0,
    // `tooltip.max_width`: 0.7.2 macos-monterey.toml (uncited)
    tooltip_max_width: 300.0,
    // `window.title_bar_font.weight`: 0.7.2 macos-monterey.toml (uncited)
    title_bar_font_weight: 700,
};

/// `macos-tahoe`'s constants.
const TAHOE: Generation = Generation {
    preset: Preset::MacosTahoe,
    light: TAHOE_LIGHT,
    dark: TAHOE_DARK,
    // `Resolved.icon_set`: 0.7.2 macos-tahoe.toml (uncited)
    icon_set: IconSet::SfSymbols,
    // `Resolved.icon_theme`: 0.7.2 macos-tahoe.toml (uncited)
    icon_theme: "sf-symbols",
    // `defaults.icon_sizes.toolbar`: platform-facts.md (§2.1.8): 32 regular / 24 small;
    // macos-monterey's choice of the small mode
    icon_toolbar: 24.0,
    // `defaults.icon_sizes.small`: no macOS 26 source (platform-facts.md): macos-monterey's values
    icon_small: 16.0,
    // `defaults.icon_sizes.large`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group:
    // no macOS 26 source (platform-facts.md): macos-monterey's values
    icon_large: 32.0,
    // `defaults.icon_sizes.dialog`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group:
    // no macOS 26 source (platform-facts.md): macos-monterey's values
    icon_dialog: 22.0,
    // `defaults.icon_sizes.panel`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group:
    // no macOS 26 source (platform-facts.md): macos-monterey's values
    icon_panel: 20.0,
    // `defaults.font.family`: platform-facts.md (§2.1.1): the system font, 13pt, weight trait 0
    // (probe)
    font_family: "SF Pro",
    // `defaults.font.size`: 0.7.2 macos-tahoe.toml (uncited)
    font_size_pt: 13.0,
    // `defaults.font.weight`: 0.7.2 macos-tahoe.toml (uncited)
    font_weight: 400,
    // `defaults.mono_font.family`: platform-facts.md (§2.1.2): 13pt, weight trait 0 (probe); the
    // family name is the same open question as macos-monterey's (platform-facts.md)
    mono_family: "SF Mono",
    // `defaults.mono_font.size`: 0.7.2 macos-tahoe.toml (uncited)
    mono_size_pt: 13.0,
    // `defaults.mono_font.weight`: 0.7.2 macos-tahoe.toml (uncited)
    mono_weight: 400,
    // `defaults.line_height`: platform-facts.md (§2.1.1): SF Pro's sTypo metrics, unchanged in SF
    // Pro 22.0d5e4
    line_height: 1.19,
    // `defaults.border.corner_radius`: platform-facts.md (§2.1.6): 6, the push button's and
    // pop-up's (WebKit's vector drawing)
    corner_radius: 6.0,
    // `defaults.border.corner_radius_lg`: no macOS 26 source (platform-facts.md: Tahoe's window
    // radii vary by style, no API): macos-monterey's value; Tahoe's window radii are larger per
    // third-party measurements (platform-facts.md), not adopted
    corner_radius_lg: 10.0,
    // `defaults.border.line_width`: no macOS 26 source (platform-facts.md): macos-monterey's value
    border_width: 0.5,
    // `defaults.border.opacity`: platform-facts.md (§2.1.6): separatorColor's alpha, 25/255
    // (probe), already in color
    border_opacity: 0.098,
    // `defaults.border.shadow_enabled`: no macOS 26 source (platform-facts.md): macos-monterey's
    // value
    shadow_enabled: true,
    // `defaults.disabled_opacity`: platform-facts.md (§2.1.6): 1.0, macOS dims a disabled control
    // by colour (disabled_text_color)
    disabled_opacity: 1.0,
    // `defaults.focus_ring_width`: platform-facts.md (§2.1.5): 3, WebKit's unconditional constant
    focus_ring_width: 3.0,
    // `defaults.focus_ring_offset`: no macOS 26 source (platform-facts.md): macos-monterey's value
    focus_ring_offset: -1.0,
    // `layout.widget_gap`: platform-facts.md (§2.20): NSStackView's default spacing 8
    widget_gap: 8.0,
    container_margin: None,
    // `layout.window_margin`: platform-facts.md (§2.20): 20, §1.1.5's content margin (the Auto
    // Layout Guide)
    window_margin: 20.0,
    section_gap: None,
    // `text_scale.caption.size`: platform-facts.md (§2.19): the HIG's macOS text styles (probe)
    caption_size_pt: 10.0,
    // `text_scale.caption.weight`: 0.7.2 macos-tahoe.toml (uncited)
    caption_weight: 400,
    // `text_scale.section_heading.size`: 0.7.2 macos-tahoe.toml (uncited)
    section_heading_size_pt: 13.0,
    // `text_scale.section_heading.weight`: 0.7.2 macos-tahoe.toml (uncited)
    section_heading_weight: 700,
    // `text_scale.dialog_title.size`: 0.7.2 macos-tahoe.toml (uncited)
    dialog_title_size_pt: 22.0,
    // `text_scale.dialog_title.weight`: 0.7.2 macos-tahoe.toml (uncited)
    dialog_title_weight: 400,
    // `text_scale.display.size`: 0.7.2 macos-tahoe.toml (uncited)
    display_size_pt: 26.0,
    // `text_scale.display.weight`: 0.7.2 macos-tahoe.toml (uncited)
    display_weight: 400,
    // `button.min_width`: no macOS 26 source (platform-facts.md, (none)): macos-monterey's value
    button_min_width: 64.0,
    // `button.min_height`: platform-facts.md (§2.3): NSButton intrinsicContentSize 24 (probe)
    button_min_height: 24.0,
    // `button.border.padding.top`: 0.7.2 macos-tahoe.toml (uncited)
    button_padding_top: 4.0,
    // `button.border.padding.bottom`: 0.7.2 macos-tahoe.toml (uncited)
    button_padding_bottom: 4.0,
    // `button.border.padding.left`, `button.border.padding.right`: platform-facts.md (§2.3): the
    // title rect [12, 4, 41, 16] in the 65 x 24 bezel (probe), from its outer edge
    button_padding_horizontal: 12.0,
    // `button.icon_text_gap`: no macOS 26 source (platform-facts.md): macos-monterey's value
    button_icon_text_gap: 4.0,
    // `button.border.shadow_enabled`: platform-facts.md (§2.3): no shadow (measured: nothing drawn
    // outside the bezel)
    button_shadow_enabled: Some(false),
    // `checkbox.border.shadow_enabled`: platform-facts.md (§2.5): no shadow (measured)
    checkbox_shadow_enabled: false,
    // `checkbox.indicator_width`: platform-facts.md (§2.5): imageRectForBounds 16 x 16 (probe)
    checkbox_indicator_width: 16.0,
    // `checkbox.label_gap`: platform-facts.md (§2.5): the title rect starts 6 after the image rect
    // (probe)
    checkbox_label_gap: 6.0,
    // `checkbox.radio_dot_diameter`: platform-facts.md (§2.5): macOS 26: the selected radio's dot,
    // 5 (measured, os-probe run 40348854007,
    // os-probe-macos-26/renders/aqua/radio-regular-on@2x.png, in a window that is not key)
    radio_dot_diameter: 5.0,
    // `input.border.corner_radius`: platform-facts.md (§1.1.3): the text field's radius, 5
    // (WebKit's vector drawing)
    input_corner_radius: Some(5.0),
    // `input.border.padding.top`, `input.border.padding.bottom`: 0.7.2 macos-tahoe.toml (uncited)
    input_padding_vertical: 4.0,
    // `input.border.padding.left`, `input.border.padding.right`: platform-facts.md (§2.4): the
    // title rect [4, 4, 152, 16] in the 160 x 24 field (probe), from its outer edge
    input_padding_horizontal: 4.0,
    // `input.border.shadow_enabled`: platform-facts.md (§2.4): no drop shadow (measured)
    input_shadow_enabled: false,
    // `input.min_height`: platform-facts.md (§2.4): NSTextField intrinsicContentSize 24 (probe)
    input_min_height: 24.0,
    // `link.underline_enabled`: no macOS 26 source (platform-facts.md): macos-monterey's values
    link_underline: true,
    list_padding_vertical: None,
    list_padding_horizontal: None,
    // `list.header_height`: platform-facts.md (§2.15): NSTableHeaderView height 28, .fullWidth,
    // macOS 26 (probe 26)
    list_header_height: 28.0,
    // `list.row_height`: platform-facts.md (§2.15): NSTableView rowHeight 24 (probe)
    list_row_height: 24.0,
    // `scrollbar.groove_width`: platform-facts.md (§2.8): scrollerWidth 17 in both modes (probe)
    scrollbar_groove_width: 17.0,
    // `scrollbar.min_thumb_length`: no macOS 26 source (platform-facts.md): macos-monterey's value
    scrollbar_min_thumb_length: 40.0,
    // `scrollbar.overlay_mode`: platform-facts.md (§2.8): NSScroller.preferredScrollerStyle,
    // overlay on the probe's runner (probe)
    scrollbar_overlay: true,
    // `scrollbar.thumb_width`: no macOS 26 source (platform-facts.md): macos-monterey's value
    scrollbar_thumb_width: 7.0,
    tooltip_padding_vertical: None,
    tooltip_padding_horizontal: None,
    // `tooltip.font.size`: platform-facts.md (§2.7): toolTipsFontOfSize: 11, probe
    tooltip_font_size_pt: 11.0,
    // `tooltip.max_width`: no macOS 26 source (platform-facts.md): macos-monterey's value
    tooltip_max_width: 300.0,
    // `window.title_bar_font.weight`: platform-facts.md (§2.2): titleBarFontOfSize:, weight trait
    // 0.4 (probe)
    title_bar_font_weight: 700,
};

/// The font DPI a point size converts at: 72, "where a point is a pixel"
/// (0.7.2's `ResolutionContext::from_system` on macOS, `src/resolve/context.rs`;
/// spec §4, `Typography::font_dpi`: "72.0 Apple").
const FONT_DPI: f32 = 72.0;

/// The colour of `rgba`, 8-bit sRGB `0xRRGGBBAA`, each channel over 255 as
/// [`Color::from_srgb8`] takes it.
const fn srgb8(rgba: u32) -> Color {
    const SCALE: f32 = u8::MAX as f32;
    let [r, g, b, a] = rgba.to_be_bytes();
    Color::new(
        r as f32 / SCALE,
        g as f32 / SCALE,
        b as f32 / SCALE,
        a as f32 / SCALE,
    )
}

// ---------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------

/// The contrast levels both presets define (spec §1, "Axis values without
/// data"): `Normal` only, as the module documentation says.
pub(crate) const CONTRAST_LEVELS: &[Contrast] = &[Contrast::Normal];

/// The theme of `macos-monterey` for `ctx.scheme`, from the preset's
/// constants (the static path: `inputs` is not applied yet).
pub(crate) fn resolve_monterey(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    resolve(&MONTEREY, ctx, inputs)
}

/// The theme of `macos-tahoe` for `ctx.scheme`, from the preset's constants
/// (the static path: `inputs` is not applied yet).
pub(crate) fn resolve_tahoe(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    resolve(&TAHOE, ctx, inputs)
}

/// The theme of the preset of `g` for `ctx.scheme`. The widgets this builder
/// does not state yet are the theme with nothing stated's.
fn resolve(g: &Generation, ctx: &ThemeContext, _inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let c = match ctx.scheme {
        Scheme::Light => &g.light,
        Scheme::Dark => &g.dark,
    };
    let colors = roles(c, g);
    let typography = typography(g, &colors, ctx.text_scale);
    let layout = LayoutMetrics {
        widget_gap: Some(g.widget_gap),
        container_margin: g.container_margin,
        window_margin: Some(g.window_margin),
        section_gap: g.section_gap,
        corner_radius: g.corner_radius,
        corner_radius_lg: g.corner_radius_lg,
        border_width: g.border_width,
        border_opacity: g.border_opacity,
        shadow_enabled: g.shadow_enabled,
        cursor_size_px: None,
        cursor_theme: None,
    };
    let base = nothing_stated(g.preset, ctx.clone());
    ResolvedTheme {
        icons: IconThemeSpec {
            set: g.icon_set,
            theme: Some(Arc::from(g.icon_theme)),
            // 0.7.2's `Resolved::icon_theme_explicit`: the TOML states the theme
            // (`src/model/resolved.rs`).
            theme_explicit: true,
            sizes: IconSizes::new(
                Some(g.icon_toolbar),
                Some(g.icon_small),
                Some(g.icon_large),
                Some(g.icon_dialog),
                Some(g.icon_panel),
            ),
        },
        timings: UiTimings::default(),
        motion_tokens: motion_tokens(MotionTokens::default(), motion(ctx.motion, None)),
        // 0.7.2's `defaults.focus_ring_*`: a ring; `focus_ring_keyboard_only` unstated.
        focus_ring: FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(g.focus_ring_width, colors.focus_ring),
            Stroke::None,
            g.focus_ring_offset,
            None,
            None,
        ),
        caret: CaretSpec::default(),
        elevations: Arc::from([]),
        button: button(c, g, &colors, &typography, &layout),
        checkbox: checkbox(c, g, &colors, &typography, &layout),
        radio: radio(c, g, &colors, &typography, &layout),
        scrollbar: scrollbar(c, g, &colors),
        list: list(c, g, &colors, &typography, &layout),
        tooltip: tooltip(c, g, &layout, ctx.text_scale),
        window: window(c, g, &colors, &layout, ctx),
        input: input(
            c,
            g,
            &colors,
            &typography,
            &layout,
            Some(Edges::symmetric(
                g.input_padding_vertical,
                g.input_padding_horizontal,
            )),
            Some(g.input_min_height),
        ),
        text_area: input(c, g, &colors, &typography, &layout, None, None),
        link: link(c, g, &colors, &typography),
        colors,
        typography,
        layout,
        ..base
    }
}

/// The colour roles of one scheme from its colours `c`: the window's
/// surface (0.7.2's `defaults.background_color`, `text_color`, `muted_color`
/// and `border.color`), the view's background (`defaults.surface_color`),
/// the selections, the accent, the focus ring, the caret
/// (`input.caret_color`), the link (0.7.2: `link.visited_text_color` is
/// `defaults.link_color` where unstated, so no visited colour of its own)
/// and the status colours with their text, Apple's system colours
/// (`RoleOrigin::SystemColor`).
fn roles(c: &SchemeColors, g: &Generation) -> ColorRoles {
    let status = |fill| StatusColors {
        fill,
        foreground: c.status_foreground,
        standalone: None,
        origin: RoleOrigin::SystemColor,
    };
    let unstated = SurfaceColors::default();
    ColorRoles {
        window: SurfaceColors::new(
            Some(c.background),
            Some(c.foreground),
            Some(c.muted),
            Some(c.border),
            None,
        ),
        view: SurfaceColors::new(Some(c.surface), None, None, None, None),
        header: unstated,
        sidebar: unstated,
        card: unstated,
        dialog: unstated,
        popover: unstated,
        tooltip: unstated,
        osd: unstated,
        selection: SelectionColors::new(
            c.selection_background,
            c.selection_foreground,
            Some(c.selection_inactive_background),
            Some(c.selection_inactive_foreground),
        ),
        text_selection: SelectionColors::new(
            c.text_selection_background,
            c.text_selection_foreground,
            None,
            None,
        ),
        find_highlight: None,
        scrim: None,
        accent: AccentColors {
            fill: c.accent,
            foreground: c.accent_foreground,
            standalone: None,
        },
        focus_ring: c.focus_ring,
        caret: c.caret,
        link: c.link,
        link_visited: None,
        disabled_text: c.disabled_text,
        shadow: c.shadow,
        disabled_opacity: g.disabled_opacity,
        error: status(c.error),
        warning: status(c.warning),
        success: status(c.success),
        info: status(c.info),
    }
}

/// A font size in logical pixels: converted at [`FONT_DPI`], times the text
/// scale, not rounded (spec §6, resolution step 2).
fn font_px(size: FontSize, text_scale: f32) -> f32 {
    linear_font_px(size, FONT_DPI, text_scale, None)
}

/// The line height of a style of `size` at the multiplier `multiplier`, in
/// logical pixels: the multiplier times the size, in its unit, converted at
/// the font DPI (0.7.2's `resolve_text_scale_entry`,
/// `src/resolve/inheritance.rs`, then its conversion in `validate`; spec §4,
/// `Typography`), times the text scale.
fn line_height_px(size: FontSize, multiplier: f32, text_scale: f32) -> f32 {
    let size = match size {
        FontSize::Pt(v) => FontSize::Pt(v * multiplier),
        FontSize::Px(v) => FontSize::Px(v * multiplier),
    };
    linear_font_px(size, FONT_DPI, text_scale, None)
}

/// A font as 0.7.2 resolves it: its family, size, weight and style stated,
/// the style normal, which no macOS TOML states (`src/resolve/validate_helpers.rs`,
/// `require_font`); and `color`, where given.
fn font(
    family: &str,
    size: FontSize,
    weight: u16,
    color: Option<Color>,
    text_scale: f32,
) -> TextStyle {
    let mut font = TextStyle {
        family: Arc::from(family),
        size_px: font_px(size, text_scale),
        defined_size: Some(size),
        weight,
        style: FontStyle::Normal,
        stated: TextStated::FAMILY | TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::STYLE,
        ..TextStyle::default()
    };
    if let Some(color) = color {
        font.color = color;
        font.stated |= TextStated::COLOR;
    }
    font
}

/// A text-scale entry as 0.7.2 resolves it: its size, weight and line height
/// stated, nothing else (0.7.2's `TextScaleEntry`).
fn text_scale_entry(g: &Generation, size_pt: f32, weight: u16, text_scale: f32) -> TextStyle {
    let size = FontSize::Pt(size_pt);
    TextStyle {
        size_px: font_px(size, text_scale),
        defined_size: Some(size),
        weight,
        line_height: line_height_px(size, g.line_height, text_scale),
        stated: TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::LINE_HEIGHT,
        ..TextStyle::default()
    }
}

/// The text styles: the UI and monospace fonts in the window's text colour
/// (0.7.2: `defaults.font.color` is `defaults.text_color`, and
/// `defaults.mono_font`'s is `defaults.font`'s, where unstated:
/// `resolve_defaults_internal`), the body's line height by the text-scale
/// rule (spec §4); 0.7.2's text scale as the caption (`caption`), the
/// headline (`section_heading`), the first title (`dialog_title`) and the
/// display (`display`). Sizes convert at [`FONT_DPI`] and scale by
/// `text_scale`; each keeps its unscaled size as its `defined_size`.
fn typography(g: &Generation, colors: &ColorRoles, text_scale: f32) -> Typography {
    let body_size = FontSize::Pt(g.font_size_pt);
    let text = colors.window.foreground;
    let mut body = font(g.font_family, body_size, g.font_weight, text, text_scale);
    body.line_height = line_height_px(body_size, g.line_height, text_scale);
    body.stated |= TextStated::LINE_HEIGHT;
    let entry = |size, weight| Some(text_scale_entry(g, size, weight, text_scale));
    Typography {
        display: entry(g.display_size_pt, g.display_weight),
        title1: entry(g.dialog_title_size_pt, g.dialog_title_weight),
        title2: None,
        title3: None,
        title4: None,
        headline: entry(g.section_heading_size_pt, g.section_heading_weight),
        body,
        callout: None,
        subheadline: None,
        caption: entry(g.caption_size_pt, g.caption_weight),
        monospace: font(
            g.mono_family,
            FontSize::Pt(g.mono_size_pt),
            g.mono_weight,
            text,
            text_scale,
        ),
        line_height_multiplier: g.line_height,
        fallbacks: Arc::from([]),
        emoji_family: None,
        rendering: TextRendering::default(),
        font_dpi: FONT_DPI,
        font_whole_pixels: None,
    }
}

// ---------------------------------------------------------------------------
// The widgets
// ---------------------------------------------------------------------------

/// A part's fill of one colour.
fn fill(color: Color) -> PartProp {
    PartProp::Fill(Paint::Solid(color))
}

/// A part's outline in `color`, inside its edge, at the default border width
/// (0.7.2: a widget's `border.line_width` is `defaults.border.line_width`
/// where unstated, `resolve_border_inheritance`).
fn outline(layout: &LayoutMetrics, color: Color) -> PartProp {
    PartProp::Stroke(Stroke::uniform(layout.border_width, color))
}

/// A part's corners at the default radius (0.7.2: a widget's
/// `border.corner_radius` is `defaults.border.corner_radius` where unstated,
/// `resolve_border_inheritance`).
fn corners(layout: &LayoutMetrics) -> PartProp {
    PartProp::Corners(Corners::uniform(layout.corner_radius))
}

/// No shadow where the TOML states `shadow_enabled = false`; nothing where
/// it states `true`, as 0.7.2 gives a shadow no geometry (spec §5, notes to
/// the table).
fn no_shadow(enabled: bool) -> Option<PartProp> {
    (!enabled).then_some(PartProp::Shadows(None))
}

/// `builder` with the exact `thickness` across its part: the width of a
/// vertical part, the height of a horizontal one.
fn across(
    builder: RuleBuilder<PartStyle, PartProp>,
    thickness: f32,
) -> RuleBuilder<PartStyle, PartProp> {
    builder
        .rule(
            RuleSelector::on(WidgetState::NONE).orientation(Orientation::Vertical),
            [
                PartProp::MinSize([Some(thickness), None]),
                PartProp::MaxSize([Some(thickness), None]),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::NONE).orientation(Orientation::Horizontal),
            [
                PartProp::MinSize([None, Some(thickness)]),
                PartProp::MaxSize([None, Some(thickness)]),
            ],
        )
}

/// A widget's label: the UI font (0.7.2: a widget's font is `defaults.font`
/// where unstated, `resolve_font_inheritance`) without its line height, which
/// no 0.7.2 widget font has, in `color` where given, else in the UI font's.
fn label(body: &TextStyle, color: Option<Color>) -> TextStyle {
    let mut label = body.clone();
    label.line_height = TextStyle::default().line_height;
    label.stated = label.stated.difference(TextStated::LINE_HEIGHT);
    if let Some(color) = color {
        label.color = color;
        label.stated |= TextStated::COLOR;
    }
    label
}

/// The push button: its fill in the default line and radius (0.7.2:
/// `button.border` is `defaults.border` where unstated) with its padding and
/// least size; hovered, pressed (`macos-monterey` only) and disabled fills;
/// the primary variant in the accent with its text (0.7.2:
/// `primary_background` and `primary_text_color` are
/// `defaults.accent_color` and `accent_text_color` where unstated). Its text
/// keeps its colour hovered and pressed (0.7.2: `hover_text_color` and
/// `active_text_color` are `button.font.color` where unstated), and is the
/// disabled text when disabled (`defaults.disabled_text_color`).
fn button(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ButtonWidget {
    let primary = || {
        RuleSelector::on(WidgetState::NONE).matcher(ButtonMatcher {
            variant: Some(ButtonVariant::Primary),
            shape: None,
        })
    };
    let body = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.button_background),
            outline(layout, c.border),
            corners(layout),
            PartProp::Padding(Edges::new(
                g.button_padding_top,
                g.button_padding_horizontal,
                g.button_padding_bottom,
                g.button_padding_horizontal,
            )),
            PartProp::MinSize([Some(g.button_min_width), Some(g.button_min_height)]),
        ])
        .base(g.button_shadow_enabled.and_then(no_shadow))
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.button_hover)])
        .rules(
            c.button_pressed
                .map(|_| RuleSelector::on(WidgetState::PRESSED)),
            c.button_pressed.map(fill),
        )
        .rule(primary(), [fill(roles.accent.fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.button_disabled)],
        )
        .build();
    let label = RuleBuilder::new(label(&typography.body, Some(c.button_foreground)))
        .rule(primary(), [TextProp::Color(roles.accent.foreground)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    ButtonWidget::new(
        WidgetOrigin::Native,
        body,
        label,
        RuleBuilder::new(IconStyle::default()).build(),
        g.button_icon_text_gap,
        // 0.7.2: `button.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The box of a check box or the circle of a radio button, which 0.7.2's
/// `checkbox.*` serves both: the unchecked fill in the unchecked outline, no
/// shadow, of an exact size; hovered, `macos-monterey`'s hover fill;
/// checked, the accent (0.7.2: `checkbox.checked_background` is
/// `defaults.accent_color` where unstated) in the default line (0.7.2:
/// `checkbox.border.color` is `defaults.border.color` where unstated).
fn indicator(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    layout: &LayoutMetrics,
) -> RuleBuilder<PartStyle, PartProp> {
    let size = Some(g.checkbox_indicator_width);
    RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.checkbox_unchecked),
            outline(layout, c.checkbox_unchecked_border),
            PartProp::MinSize([size; 2]),
            PartProp::MaxSize([size; 2]),
        ])
        .base(no_shadow(g.checkbox_shadow_enabled))
        .rules(
            c.checkbox_hover
                .map(|_| RuleSelector::on(WidgetState::HOVER)),
            c.checkbox_hover.map(fill),
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill), outline(layout, c.border)],
        )
}

/// The label of a check box or a radio button: the UI font in its colour;
/// disabled, the disabled text (0.7.2: `checkbox.disabled_text_color` is
/// `defaults.disabled_text_color` where unstated).
fn check_label(roles: &ColorRoles, typography: &Typography) -> RuleBuilder<TextStyle, TextProp> {
    RuleBuilder::new(label(&typography.body, None)).rule(
        RuleSelector::on(WidgetState::DISABLED),
        [TextProp::Color(roles.disabled_text)],
    )
}

/// The check box: [`indicator`] with the default corners, a check mark in
/// the accent's text (0.7.2: `checkbox.indicator_color` is
/// `defaults.accent_text_color` where unstated), [`check_label`].
fn checkbox(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> CheckboxWidget {
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Color(roles.accent.foreground)])
        .build();
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator(c, g, roles, layout)
            .base([corners(layout)])
            .build(),
        checkmark,
        check_label(roles, typography).build(),
        g.checkbox_label_gap,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The radio button: [`indicator`] (0.7.2 states no corners for it), a dot
/// of the preset's diameter in the accent's text (`checkbox.indicator_color`),
/// [`check_label`].
fn radio(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> RadioWidget {
    let dot_size = Some(g.radio_dot_diameter);
    let dot = RuleBuilder::new(PartStyle::default())
        .base([
            fill(roles.accent.foreground),
            PartProp::MinSize([dot_size; 2]),
            PartProp::MaxSize([dot_size; 2]),
        ])
        .build();
    RadioWidget::new(
        WidgetOrigin::Native,
        indicator(c, g, roles, layout).build(),
        dot,
        check_label(roles, typography).build(),
        g.checkbox_label_gap,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The list or table view: the list's own background in the default line and
/// radius (0.7.2: `list.border` is `defaults.border` where unstated); rows of
/// the preset's height with `macos-monterey`'s padding and the alternate
/// row's fill; the selection's colours (0.7.2: `list.selection_background`
/// and `selection_text_color` are the selection's where unstated); the item
/// text in the window's text (0.7.2: `list.item_font` is `defaults.font`
/// where unstated), which a hovered row keeps (`hover_text_color`:
/// `item_font.color` where unstated), the disabled text when disabled;
/// headers of the preset's height in their own fill and text. No hover fill
/// (0.7.2: `list.hover_background` unset is the container's fill, "no tint")
/// and no header separator are stated.
fn list(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ListWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.list_background),
            outline(layout, c.border),
            corners(layout),
        ])
        .build();
    let alternate = RuleSelector::on(WidgetState::NONE).matcher(ListMatcher {
        variant: None,
        alternate: Some(true),
    });
    let padding = g
        .list_padding_vertical
        .zip(g.list_padding_horizontal)
        .map(|(v, h)| PartProp::Padding(Edges::symmetric(v, h)));
    let row = RuleBuilder::new(PartStyle::default())
        .base([PartProp::MinSize([None, Some(g.list_row_height)])])
        .base(padding)
        .rule(alternate, [fill(c.list_alternate)])
        .build();
    let highlight = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    let item_label = RuleBuilder::new(label(&typography.body, roles.window.foreground))
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(roles.selection.foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    let header = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.list_header_background),
            PartProp::MinSize([None, Some(g.list_header_height)]),
        ])
        .build();
    let header_label =
        RuleBuilder::new(label(&typography.body, Some(c.list_header_foreground))).build();
    ListWidget::new(
        WidgetOrigin::Native,
        container,
        row,
        highlight,
        item_label,
        header,
        header_label,
        RuleBuilder::new(PartStyle::default()).build(),
        c.list_grid,
    )
}

/// The scroll bar: a groove of an exact thickness in the track's colour and
/// a handle of an exact thickness in the window's muted text (0.7.2:
/// `scrollbar.thumb_color` is `defaults.muted_color` where unstated), with
/// the hovered handle's fill (`macos-tahoe`: `thumb_hover_color` is
/// `defaults.muted_color` where unstated, so no rule) and
/// `macos-monterey`'s pressed one.
fn scrollbar(c: &SchemeColors, g: &Generation, roles: &ColorRoles) -> ScrollbarWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base([fill(c.scrollbar_track)]),
        g.scrollbar_groove_width,
    )
    .build();
    let thumb = across(
        RuleBuilder::new(PartStyle::default()).base(roles.window.foreground_muted.map(fill)),
        g.scrollbar_thumb_width,
    )
    .rules(
        c.scrollbar_thumb_hover
            .map(|_| RuleSelector::on(WidgetState::HOVER)),
        c.scrollbar_thumb_hover.map(fill),
    )
    .rules(
        c.scrollbar_thumb_pressed
            .map(|_| RuleSelector::on(WidgetState::PRESSED)),
        c.scrollbar_thumb_pressed.map(fill),
    )
    .build();
    ScrollbarWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        g.scrollbar_min_thumb_length,
        g.scrollbar_overlay,
    )
}

/// The tooltip: its fill in the default line and radius (0.7.2:
/// `tooltip.border` is `defaults.border` where unstated) with
/// `macos-monterey`'s padding and the preset's greatest width; its text in
/// the UI font's family, weight and style at the tooltip's size (0.7.2:
/// `tooltip.font`'s unstated fields are `defaults.font`'s) in its colour.
fn tooltip(
    c: &SchemeColors,
    g: &Generation,
    layout: &LayoutMetrics,
    text_scale: f32,
) -> TooltipWidget {
    let mut panel = PartStyle::default();
    let padding = g
        .tooltip_padding_vertical
        .zip(g.tooltip_padding_horizontal)
        .map(|(v, h)| PartProp::Padding(Edges::symmetric(v, h)));
    for prop in [
        fill(c.tooltip_background),
        outline(layout, c.border),
        corners(layout),
        PartProp::MaxSize([Some(g.tooltip_max_width), None]),
    ]
    .into_iter()
    .chain(padding)
    {
        panel.apply(&prop);
    }
    TooltipWidget::new(
        WidgetOrigin::Native,
        panel,
        font(
            g.font_family,
            FontSize::Pt(g.tooltip_font_size_pt),
            g.font_weight,
            Some(c.tooltip_foreground),
            text_scale,
        ),
    )
}

/// The window's chrome, in the active window or, where `ctx.window_active`
/// is false, in an inactive one: the frame in the window's background in the
/// default line with the large radius (0.7.2: `window.background_color` is
/// `defaults.background_color` and `window.border` `defaults.border` with
/// `corner_radius_lg` where unstated); the title bar's fill, inactive where
/// the preset states one (else the active one, 0.7.2's
/// `resolve_widget_to_widget`); the title in the UI font at the title bar's
/// weight (0.7.2: `window.title_bar_font` is `defaults.font` where unstated)
/// in the window's text, active or not (0.7.2:
/// `inactive_title_bar_text_color` is `title_bar_font.color` where
/// unstated). A maximized or full-screen window has no data of its own and
/// takes the normal window's styles; no caption button is stated.
fn window(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    layout: &LayoutMetrics,
    ctx: &ThemeContext,
) -> WindowChrome {
    let mut frame = PartStyle::default();
    for prop in [
        fill(c.background),
        outline(layout, c.border),
        PartProp::Corners(Corners::uniform(layout.corner_radius_lg)),
    ] {
        frame.apply(&prop);
    }
    let title_bar_fill = if ctx.window_active {
        c.title_bar_background
    } else {
        c.title_bar_inactive_background
            .unwrap_or(c.title_bar_background)
    };
    let mut title_bar = PartStyle::default();
    title_bar.apply(&fill(title_bar_fill));
    WindowChrome::new(
        WidgetOrigin::Native,
        None,
        frame.clone(),
        frame,
        title_bar.clone(),
        title_bar,
        font(
            g.font_family,
            FontSize::Pt(g.font_size_pt),
            g.title_bar_font_weight,
            roles.window.foreground,
            ctx.text_scale,
        ),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
    )
}

/// A text field, single-line (`input`) or multi-line (`text_area`), with its
/// own `padding` and least height `min_height` where given: the field's
/// fill in the default line (0.7.2: `input.border` is `defaults.border`
/// where unstated, and `text_area.border`'s colour, radius, width and shadow
/// are `input.border`'s) at `macos-tahoe`'s own radius, else the default
/// one, with no shadow; its text in its colour, the placeholder in its own;
/// disabled, `macos-monterey`'s disabled fill and the disabled text (0.7.2:
/// `input.disabled_text_color` is `defaults.disabled_text_color` and
/// `disabled_opacity` `defaults.disabled_opacity` where unstated).
fn input(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    padding: Option<Edges>,
    min_height: Option<f32>,
) -> InputWidget {
    let radius = g.input_corner_radius.unwrap_or(layout.corner_radius);
    let field = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.input_background),
            outline(layout, c.border),
            PartProp::Corners(Corners::uniform(radius)),
        ])
        .base(no_shadow(g.input_shadow_enabled))
        .base(padding.map(PartProp::Padding))
        .base(min_height.map(|height| PartProp::MinSize([None, Some(height)])))
        .rules(
            c.input_disabled
                .map(|_| RuleSelector::on(WidgetState::DISABLED)),
            c.input_disabled.map(fill),
        )
        .build();
    let text = RuleBuilder::new(label(&typography.body, Some(c.input_foreground)))
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    let placeholder = RuleBuilder::new(TextStyle::default())
        .base([TextProp::Color(c.placeholder)])
        .build();
    InputWidget::new(
        WidgetOrigin::Native,
        field,
        text,
        placeholder,
        RuleBuilder::new(TextStyle::default()).build(),
        Some(roles.disabled_opacity),
    )
}

/// The hyperlink: its background, at rest and hovered; the UI font in the
/// link colour, underlined (0.7.2: `link.font` is `defaults.font` with
/// `defaults.link_color` where unstated), the same colour visited, hovered
/// and pressed (0.7.2: `visited_text_color` is `defaults.link_color`, and
/// `hover_text_color` and `active_text_color` `link.font.color`, where
/// unstated, so no rule), the disabled text when disabled
/// (`defaults.disabled_text_color`).
fn link(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
) -> LinkWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(c.link_background)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.link_hover_background)],
        )
        .build();
    let label = RuleBuilder::new(label(&typography.body, Some(roles.link)))
        .base(
            g.link_underline
                .then_some(TextProp::Decoration(TextDecoration {
                    kind: DecorationKind::Underline,
                    color: None,
                    thickness: None,
                })),
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    LinkWidget::new(WidgetOrigin::Native, container, label)
}
