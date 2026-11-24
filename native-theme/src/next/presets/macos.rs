//! The `macos-monterey` and `macos-tahoe` presets' builder (spec §6): AppKit
//! from macOS 12 Monterey to 15 Sequoia, and AppKit's Liquid Glass look of
//! macOS 26 Tahoe, light and dark.
//!
//! The two presets share the builder and differ in their constants: each
//! has its own table of colours per scheme ([`SchemeColors`]) and its own
//! generation of metrics, fonts and other constants ([`Generation`]), each
//! constant with the citation its 0.7.2 TOML gives it. The builder runs in
//! three steps: the preset's constants for the scheme resolved for, with the
//! system's settings ([`ResolveInputs`]) laid over them as the 0.7.2 reader
//! writes them over its preset ([`resolve`]); the colour roles and the
//! globals made from them; the widgets derived from the roles, the globals
//! and the constants. A widget's colour that is a role's colour is taken
//! from the roles, not from a second copy of the constant.
//!
//! The active palette ([`ResolveInputs::palette`]) and then an explicit
//! accent ([`ResolveInputs::accent`]) are laid over the system's settings
//! (spec §6, resolution step 2, layer 3). A palette states the roles its
//! slots name (spec §7, "The roles a palette states"), and each slot whose
//! AppKit colour has the slot's name replaces that colour and every widget
//! colour the preset takes from it ([`lay_palette_over`]): `window_bg` is
//! `windowBackgroundColor`, the window's background (0.7.2's
//! `defaults.background_color`), so also the window's frame and every panel
//! and bar the preset fills with it (the menu's, the menu bar's, the tab
//! bar's and the tabs', the toolbar's, the status bar's, the segmented
//! control's track); `view_bg` is `controlBackgroundColor`, the view's
//! background (`defaults.surface_color`), so also the list's
//! (`list.background_color`, the same colour) and the slider's knob where
//! the scheme states no knob colour; `sidebar_bg` is the sidebar's material, so its pane; `tooltip_bg`
//! is the tooltip's panel; `foreground` is `labelColor`, the window's text,
//! in which the preset letters its widgets, the sidebar's items
//! (`sidebar.font.color`, `labelColor`) and the tooltip's text, and the text
//! of the eight surfaces (§7: one text colour); `foreground_muted` is
//! `secondaryLabelColor`, so the scroll bar's handle and the progress bar's
//! groove; `border` is `separatorColor`, so every outline drawn in the
//! default border and the separator; `link` is `linkColor`; `caret` is
//! `textInsertionPointColor`; `error`, `warning`, `success` and `info` are
//! the status fills; `accent` is written as given as an explicit accent is
//! (below), and `accent_fg` is the text on it; `selection_bg` and
//! `selection_fg` are the selection's, which the text selection takes, and
//! the hovered menu item's text is the selection's text, as it is drawn on
//! the selection. A slot with no AppKit colour of its name states its role
//! alone: `header_bg` and `divider` state their roles (the title bar keeps
//! the preset's colour); `surface_bg` states the card's, the dialog's and
//! the popover's, so their panels. Every other colour keeps the value of the
//! layers below: the control fills with their text (the button's, the text
//! field's, the pop-up button's), the hover fills, the outlines of their own,
//! every disabled colour, the placeholder, the alternate row, the list's
//! header, the title bar, the unfocused selection, the status colours' text.
//!
//! An explicit accent, or else a palette's `accent`, is written as given;
//! no colour is computed from it (spec §6, step 2;
//! `.superpowers/sdd/v0.8.0-plan/M2/units/accent-decision.md`) but
//! `macos-tahoe`'s checked button fills, at rest, pressed and disabled, its
//! measured mixes ([`BUTTON_CHECKED_MIX`], [`BUTTON_CHECKED_PRESSED_MIX`],
//! [`BUTTON_CHECKED_DISABLED_MIX`] at [`BUTTON_CHECKED_DISABLED_ALPHA`]):
//! it is the accent's fill, and so the checked
//! fills, the slider's and the progress bar's fill, `macos-tahoe`'s selected
//! segment and the disabled on switch's track at its generation's alpha; the
//! selection's background; the text selection, which takes the selection's
//! two colours; the focus ring, at alpha 0x7F ([`FOCUS_RING_ALPHA`]); the
//! caret, unless a palette states one ([`lay_accent_over`]). The link, the
//! status colours, the unfocused selection and the text on the accent and on
//! the selection (`alternateSelectedControlTextColor`) keep the value of the
//! layers below,
//! as AppKit keeps them under another accent. An explicit accent over a
//! palette replaces its `accent`, `accent_fg`, `selection_bg` and
//! `selection_fg`; the two texts are then the ones of the layers below the
//! palette, and its `link`, `info` and `caret` stand. Where AppKit draws a
//! variant of the accent this crate does not compute, the result differs
//! from AppKit's; for the red accent (`AppleAccentColor` 0, whose
//! `controlAccentColor` is `#e0383e` in light and `#ff5257` in dark, given as
//! the accent in each; the OS probe's `accent-red` reads, macOS 14, 15 and
//! 26), light / dark: the selection is
//! `#e0383e` / `#ff5257` here, `#c4262b` / `#d13539` in AppKit; the text
//! selection `#e0383e` / `#ff5257` here, `#f6c4c5` / `#8b5759` in AppKit;
//! the focus ring `#e0383e7f` / `#ff52577f` here, `#d321277f` / `#ff7a807f`
//! in AppKit.
//!
//! Defined by the presets (spec §1, "Axis values without data"): the
//! variant `ButtonVariant::Primary` (0.7.2's `primary_*` fields), a list's
//! alternate rows (`ListKind::alternate`), the two orientations of the
//! scroll bar and of the slider's groove, no control size and the contrast
//! level `Normal` only; a query for anything else returns the part's base
//! style. A maximized or full-screen window has no data of its own: its
//! frame and title bar are the normal window's.
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
//!   it; the label's disabled colour over the primary text; on
//!   `macos-tahoe` the checked fill and label over the hovered look, as
//!   hover draws nothing on a macOS 26 push button (`platform-facts.md`
//!   §2.3, `hover_border_color`), and the checked and pressed fill over the
//!   checked one, as AppKit draws a pressed checked toggle in a fill of its
//!   own (`docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`); the
//!   primary look over both, as over the hovered and the pressed ones, and
//!   the disabled look over it by egui's order; the checked and disabled
//!   fill and label over the disabled look, the primary one included, as
//!   AppKit draws a disabled checked toggle in a look of its own (the same
//!   facts), pressed or not, as a disabled and pressed button draws as a
//!   disabled one (the same facts: run 42272240639's disabled-and-pressed
//!   and disabled layers of 402 buttons are the same pixels);
//! - the check box's and the radio button's indicator: the checked fill and
//!   outline over the hovered fill, as 0.7.2's egui widgets draw a checked
//!   box or circle in its checked look whatever the pointer does (the
//!   `Selected` variant "while it is checked":
//!   `connectors/native-theme-egui-widgets/src/radio_button.rs`);
//! - the list's item label: the disabled colour over the selection's text,
//!   by egui's order above;
//! - the scroll bar's handle: the pressed fill over the hovered one, by
//!   egui's order above;
//! - the slider's knob (`macos-monterey`), the menu's item label and the
//!   pop-up button's field and label: the disabled look over the hovered
//!   one, by egui's order above;
//! - the tab, the sidebar's item and the segmented control's segment: the
//!   selected fill over the hovered fill, as 0.7.2's egui connector puts the
//!   selection's colour in the selection's fill and the hover colour in the
//!   hovered widgets' fill (`connectors/native-theme-egui/src/mapping.md`)
//!   and egui fills a selected item with the selection's fill over its
//!   hovered look (egui 0.37.0 `src/style.rs`, `Style::interact_selectable`);
//! - the switch's track: the on track over the hovered off track, the
//!   hovered on track being the on track (0.7.2's
//!   `switch.hover_checked_background` unset, which the table of homes
//!   compares with the on track, `tests/next_homes.toml`); the disabled off
//!   track over both and the disabled on track (`DISABLED | CHECKED`) over
//!   every other, as 0.7.2 states each disabled track apart
//!   (`switch.disabled_unchecked_background`, `disabled_checked_background`).
//!   The knob: the disabled colour over the rest, on or off
//!   (`switch.disabled_thumb_color`, which the table compares in both
//!   states).
//!
//! An inactive window (`ThemeContext::window_active` false) has the inactive
//! title bar: `macos-monterey`'s own, `macos-tahoe`'s the active one (0.7.2:
//! `window.inactive_title_bar_background` is `window.title_bar_background`
//! where unstated, `resolve_widget_to_widget`).
//!
//! Of the context it acts on the scheme and on the text scale: every font
//! size, converted at the font DPI (the system's, else [`FONT_DPI`]), times the text scale, not
//! rounded, as macOS draws fonts at fractional sizes (0.7.2 states no
//! whole-pixel rounding for it), and every line height times it too; each
//! `defined_size` stays the unscaled size (spec §6, resolution step 2). The
//! motion preference collapses or scales the motion tokens, of which the
//! presets state none, and under no motion the dialog's entrance is
//! `DialogEntranceStyle::None` (the presets state no entrance). The
//! direction mirrors the one per-side value the presets state on one side
//! alone, the pop-up button's padding on the side away from its arrow (the
//! left from left to right, the right from right to left); every other
//! per-side value is the same on the left and on the right. The contrast and
//! the accessibility flags change nothing here: the presets state one
//! contrast level.
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

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{Corners, Edges, Stroke};
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::{ArrowSide, DialogButtonOrder, LabelSide, LayoutMetrics};
use crate::next::motion::{CaretSpec, MotionTokens, UiTimings};
use crate::next::paint::Paint;
use crate::next::palette::Palette;
use crate::next::preset::{Preset, ResolveInputs};
use crate::next::presets::helpers::{
    Motion, dialog_entrance, linear_font_px, motion, motion_tokens, physical_edges,
    rest_color_rules,
};
use crate::next::query::{
    ButtonVariant, Contrast, Direction, Orientation, Scheme, ThemeContext, WidgetState,
};
use crate::next::reader::{OsColors, OsFont, SystemOverrides};
use crate::next::roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
use crate::next::rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, Patched, RuleBuilder, RuleSelector, TextProp,
};
use crate::next::style::{
    DecorationKind, FocusIndicator, FocusKind, FocusTrigger, FontSize, FontStyle, IconStyle,
    PartStyle, TextDecoration, TextStated, TextStyle,
};
use crate::next::typography::{TextRendering, Typography};
use crate::next::widgets::{
    ButtonWidget, CardWidget, CheckboxWidget, ComboBoxWidget, DialogWidget, ExpanderWidget,
    InputWidget, LinkWidget, ListWidget, MenuBarWidget, MenuWidget, PopoverWidget,
    ProgressBarWidget, RadioWidget, ResolvedTheme, ScrollbarWidget, SegmentedControlWidget,
    SeparatorWidget, SidebarWidget, SliderWidget, SpinnerWidget, SplitterWidget, StatusBarWidget,
    SwitchWidget, TabWidget, ToolbarWidget, TooltipWidget, WidgetOrigin,
};
use crate::next::window::{EdgeFlags, WindowChrome};

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
    button_pressed: Option<Color>,
    button_disabled: Color,
    button_foreground: Color,
    /// The pressed push button's label colour, `None` where it keeps
    /// [`SchemeColors::button_foreground`].
    button_pressed_foreground: Option<Color>,
    /// The colour a checked toggle's fill mixes into the accent, by
    /// [`BUTTON_CHECKED_MIX`], pressed by [`BUTTON_CHECKED_PRESSED_MIX`] and
    /// disabled by [`BUTTON_CHECKED_DISABLED_MIX`]; `None` where the
    /// generation states no checked fill.
    button_checked_mix_with: Option<Color>,
    /// A checked toggle's label; `None` where the generation states none.
    button_checked_foreground: Option<Color>,
    /// A disabled checked toggle's label; `None` where the generation states
    /// none.
    button_checked_disabled_foreground: Option<Color>,
    checkbox_hover: Option<Color>,
    checkbox_unchecked: Color,
    checkbox_unchecked_border: Color,
    /// The unchecked box's edge pressed; `None` where it is the rest one.
    checkbox_pressed_border: Option<Color>,
    /// The unchecked check box's fill pressed.
    checkbox_pressed: Color,
    /// The checked check box's fill pressed, measured on the default accent;
    /// `None` where no single colour was read or another accent is laid
    /// over ([`drop_accent_measurements`]).
    checkbox_checked_pressed: Option<Color>,
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
    slider_track: Color,
    slider_thumb: Option<Color>,
    slider_disabled_fill: Color,
    slider_disabled_track: Option<Color>,
    slider_disabled_thumb: Option<Color>,
    /// The knob pressed; `None` where it is the rest one.
    slider_thumb_pressed: Option<Color>,
    /// The track's filled part pressed, measured on the default accent;
    /// `None` where it is the rest one or another accent is laid over
    /// ([`drop_accent_measurements`]).
    slider_fill_pressed: Option<Color>,
    spinner: Color,
    splitter_hover: Option<Color>,
    menu_hover_foreground: Color,
    /// An unselected tab pressed.
    tab_pressed: Color,
    sidebar_background: Color,
    sidebar_foreground: Option<Color>,
    /// A pressed source-list row's fill, measured on the default accent;
    /// `None` where it is the selection's or another accent is laid over
    /// ([`drop_accent_measurements`]).
    sidebar_pressed: Option<Color>,
    switch_unchecked: Color,
    switch_thumb: Color,
    /// The off track pressed.
    switch_pressed_unchecked: Color,
    switch_disabled_unchecked: Color,
    switch_disabled_thumb: Option<Color>,
    combo_background: Color,
    combo_foreground: Color,
    /// The pop-up button's bezel pressed.
    combo_pressed: Color,
    combo_disabled: Option<Color>,
    segmented_active: Option<Color>,
    segmented_active_foreground: Option<Color>,
    /// An unselected segment pressed.
    segment_pressed: Color,
    expander_arrow: Option<Color>,
    /// A toolbar button's label pressed.
    toolbar_label_pressed: Color,
    /// The box's outline; `None` where it draws none.
    card_border: Option<Color>,
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
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3),
    // unemphasizedSelectedContentBackgroundColor, the drawn row measured on macOS 15 and 26
    selection_inactive_background: srgb8(0xdcdc_dcff),
    // `defaults.selection_inactive_text_color`: platform-facts.md (§2.1.3), labelColor, the
    // unemphasized row's label measured on macOS 15 and 26
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
    // `button.active_background`: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`,
    // `button.body.fill` states [pressed] (measured, os-probe run 41006842699, macOS 15,
    // layers-key-push.toml `key.push.aqua.pressed.*-plain.fill`)
    button_pressed: Some(srgb8(0xf0f0_f0ff)),
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, white at
    // 128/255 (measured, os-probe run 40348854007,
    // renders/aqua/push_button-regular-disabled@2x.png, not key); over background_color #f6f6f6
    button_disabled: srgb8(0xffff_ff80),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #272727
    button_foreground: srgb8(0x0000_00d8),
    // `button.active_text_color`: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`,
    // `button.label.text.fill` scheme:light states [pressed] (measured, os-probe run 41006842699,
    // macOS 15, layers-key-push.toml `key.push.aqua.pressed.regular-plain.label`), opaque over the
    // pressed fill #f0f0f0
    button_pressed_foreground: Some(srgb8(0x2424_24ff)),
    // `button.checked_background`, `button.checked_text_color`: unstated: a checked toggle in a
    // key window of macOS 12-15 draws a gradient, pending (U141) in the parked facts
    // (`docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`)
    button_checked_mix_with: None,
    button_checked_foreground: None,
    button_checked_disabled_foreground: None,
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
    // `checkbox.indicator(PRESSED).stroke`: the pressed box's outer row #c7c7c7 (top #b3b3b3),
    // at rest #d1d1d1 (#bcbcbc), one row in each (measured, os-probe run 45198750893, macOS 15,
    // `*.pressed.checkbox_off` against `*.normal.checkbox_off` `edge_*`)
    checkbox_pressed_border: Some(srgb8(0xc7c7_c7ff)),
    // `checkbox.indicator(PRESSED).fill`: #f2f2f2, at rest #ffffff (measured, os-probe run
    // 45198750893, macOS 15, `*.pressed.checkbox_off` `fill`)
    checkbox_pressed: srgb8(0xf2f2_f2ff),
    // `checkbox.indicator(CHECKED|PRESSED).fill`: not stated: the pressed checked box is a
    // gradient (its edges #0072eb…, at rest #1485ff…), no single colour (the same run,
    // `*.pressed.checkbox_on` `edge_*`)
    checkbox_checked_pressed: None,
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
    // `slider.track_color`: platform-facts.md (§2.9): black at alpha 13/255 (measured, os-probe run
    // 40348854007, renders/aqua/slider-regular@2x.png; a key window draws #eaeaea over #f6f6f6,
    // os-probe-macos-14/probe.toml), not muted_color; over background_color #e0e0e0
    slider_track: srgb8(0x0000_000d),
    slider_thumb: None,
    // `slider.thumb(PRESSED).fill`: #f0f0f0, at rest #ffffff (measured, os-probe runs 45198750893
    // and 45208260830, macOS 15, `*.pressed.slider_knob` `fill`)
    slider_thumb_pressed: Some(srgb8(0xf0f0_f0ff)),
    // `slider.active_track(PRESSED).fill`: not read on macOS 15
    slider_fill_pressed: None,
    // `slider.disabled_fill_color`: platform-facts.md (§2.9): no value fill when disabled, the bar
    // black at alpha 7/255 on both sides of the knob (measured, os-probe run 40348854007,
    // renders/aqua/slider-regular-disabled@2x.png); over background_color #e6e6e6
    slider_disabled_fill: srgb8(0x0000_0007),
    // `slider.disabled_track_color`: platform-facts.md (§2.9): black at alpha 7/255 (measured, the
    // same render); over background_color #e6e6e6
    slider_disabled_track: Some(srgb8(0x0000_0007)),
    // `slider.disabled_thumb_color`: platform-facts.md (§2.9): white at alpha 128/255 (measured,
    // the same render); over background_color #f6f6f6
    slider_disabled_thumb: Some(srgb8(0xffff_ff80)),
    // `spinner.fill_color`: platform-facts.md (§2.23): black fins at a graded alpha, the leading
    // one 82/255 (measured in the probe's renders/aqua/progress_spinning-regular@2x.png); over the
    // window background #a0a0a0
    spinner: srgb8(0x0000_0052),
    // `splitter.hover_color`: Interactive state colors (platform-facts.md section 2.17 macOS)
    splitter_hover: Some(srgb8(0xd5d5_d5ff)),
    // `menu.hover_text_color`: platform-facts.md: selectedMenuItemTextColor, probe
    menu_hover_foreground: srgb8(0xffff_ffff),
    // `tab.tab(PRESSED).fill`: an unselected tab pressed, #f0f0f0 (measured, os-probe run
    // 45198750893, macOS 15, `*.pressed.tab` `fill`)
    tab_pressed: srgb8(0xf0f0_f0ff),
    // `sidebar.background_color`: platform-facts.md (§2.12): NSVisualEffectView.Material.sidebar's
    // opaque rendering with Reduce Transparency on, in an active view of a key window, measured on
    // macOS 14 and 15, which agree (os-probe run 40320895749, os-probe-macos-14/probe.toml)
    sidebar_background: srgb8(0xf6f6_f6ff),
    // `sidebar.font.color`: platform-facts.md (§2.12): ← defaults.font.color, labelColor (probe)
    sidebar_foreground: Some(srgb8(0x0000_00d8)),
    // `sidebar.item(PRESSED).fill`: the selection's, #0064e1 (measured, os-probe run 45198750893,
    // macOS 15, `*.pressed.source_list_row` `fill`)
    sidebar_pressed: None,
    // `switch.unchecked_background`: platform-facts.md (§2.21): black at alpha 26/255 (measured,
    // renders/aqua/switch-regular-off@2x.png); over background_color #d4d4d4
    switch_unchecked: srgb8(0x0000_001a),
    // `switch.thumb_background`: platform-facts.md (§2.21): white (measured)
    switch_thumb: srgb8(0xffff_ffff),
    // `switch.track(PRESSED).fill`: the off track pressed, black at alpha 41/255, at rest 26/255
    // (measured, os-probe run 45208260830, macOS 15, `*.pressed.switch_off` `common`)
    switch_pressed_unchecked: srgb8(0x0000_0029),
    // `switch.disabled_unchecked_background`: platform-facts.md (§2.21): black at alpha 7/255
    // (measured, os-probe run 40348854007, renders/aqua/switch-regular-disabled-off@2x.png); over
    // background_color #e6e6e6
    switch_disabled_unchecked: srgb8(0x0000_0007),
    // `switch.disabled_thumb_color`: platform-facts.md (§2.21): white at alpha 128/255 (measured,
    // the same render); over background_color #f6f6f6
    switch_disabled_thumb: Some(srgb8(0xffff_ff80)),
    // `combo_box.background_color`: platform-facts.md (§2.24): controlColor, probe
    combo_background: srgb8(0xffff_ffff),
    // `combo_box.font.color`: platform-facts.md (§2.24): controlTextColor, probe; over the fill
    // #272727
    combo_foreground: srgb8(0x0000_00d8),
    // `combo_box.field(PRESSED).fill`: #f0f0f0, at rest #ffffff, read on the highlighted cell, as
    // the mouse-down opens the menu over the button (measured, os-probe run 45208260830, macOS 15,
    // `*.highlighted.popup` `fill`)
    combo_pressed: srgb8(0xf0f0_f0ff),
    // `combo_box.disabled_background`: soft_option: NSPopUpButton disabled -- reduced alpha bg
    combo_disabled: Some(srgb8(0xe8e8_ed80)),
    // `segmented_control.active_background`: Interactive state colors (platform-facts.md section
    // 2.25 macOS) platform-facts.md (§2.25): the selected segment in a key window, measured on
    // macOS 14 and 15, which agree (os-probe run 40320895749, os-probe-macos-14/probe.toml)
    segmented_active: Some(srgb8(0xffff_ffff)),
    // `segmented_control.active_text_color`: platform-facts.md (§2.25): labelColor (probe); a key
    // window's selected label measures within one level of it over the segment on macOS 14 and 15
    // (os-probe run 40320895749, os-probe-macos-14/renders/key_window/aqua.png)
    segmented_active_foreground: Some(srgb8(0x0000_00d8)),
    // `segmented_control.segment(PRESSED).fill`: an unselected segment pressed, #f0f0f0
    // (measured, os-probe run 45198750893, macOS 15, `*.pressed.segment` `fill`)
    segment_pressed: srgb8(0xf0f0_f0ff),
    // `expander.arrow_color`: NSDisclosureButton triangle color -- no source: the former muted
    // colour #86868b, not secondaryLabelColor (defaults.muted_color)
    expander_arrow: Some(srgb8(0x8686_8bff)),
    // `toolbar.button_label(PRESSED).color`: #474747, at rest #737373, opaque over the opaque
    // title bar (measured, os-probe run 45208260830's read of run 45198750893, macOS 15,
    // `*.pressed.toolbar_button` `label`)
    toolbar_label_pressed: srgb8(0x4747_47ff),
    // `card.container.stroke`: NSBox (primary, untitled), black at alpha 19/255, 1 px (measured,
    // os-probe run 45198750893, macOS 15, `*.normal.box` `edge_*`)
    card_border: Some(srgb8(0x0000_0013)),
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
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3),
    // unemphasizedSelectedContentBackgroundColor, the drawn row measured on macOS 15 and 26
    selection_inactive_background: srgb8(0x4646_46ff),
    // `defaults.selection_inactive_text_color`: platform-facts.md (§2.1.3), labelColor, the
    // unemphasized row's label measured on macOS 15 and 26
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
    // `button.active_background`: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`,
    // `button.body.fill` states [pressed] (measured, os-probe run 41006842699, macOS 15,
    // layers-key-push.toml `key.push.darkAqua.pressed.*-plain.fill`)
    button_pressed: Some(srgb8(0xffff_ff5c)),
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, white at
    // 32/255 (measured, os-probe run 40348854007,
    // renders/darkAqua/push_button-regular-disabled@2x.png, not key); over background_color #4c4c4c
    button_disabled: srgb8(0xffff_ff20),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #e7e7e7
    button_foreground: srgb8(0xffff_ffd8),
    // `button.active_text_color`: as at rest: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at
    // `78ee0efd` states no dark pressed `button.label.text.fill` (the pressed capture reads
    // #ffffffd9, the rest #ffffffda; os-probe run 41006842699, macOS 15)
    button_pressed_foreground: None,
    // `button.checked_background`, `button.checked_text_color`: unstated: a checked toggle in a
    // key window of macOS 12-15 draws a gradient, pending (U141) in the parked facts
    // (`docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`)
    button_checked_mix_with: None,
    button_checked_foreground: None,
    button_checked_disabled_foreground: None,
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
    // `checkbox.indicator(PRESSED).stroke`: as at rest: every pressed edge pixel is the pressed
    // fill #ffffff5c (top #ffffff62), so no line of its own shows (measured, os-probe run
    // 45198750893, macOS 15, `*.pressed.checkbox_off` `edge_*`)
    checkbox_pressed_border: None,
    // `checkbox.indicator(PRESSED).fill`: white at alpha 92/255, at rest 41/255 (measured, the
    // same run, `*.pressed.checkbox_off` `fill`)
    checkbox_pressed: srgb8(0xffff_ff5c),
    // `checkbox.indicator(CHECKED|PRESSED).fill`: not stated: a gradient (its edges #2277ed…, at
    // rest #1664da…), no single colour (`*.pressed.checkbox_on` `edge_*`)
    checkbox_checked_pressed: None,
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
    // `slider.track_color`: platform-facts.md (§2.9): white at alpha 26/255 (measured, os-probe run
    // 40348854007, renders/darkAqua/slider-regular@2x.png; a key window draws #3e3e3e over
    // #282828, os-probe-macos-14/probe.toml), not muted_color; over background_color #474747
    slider_track: srgb8(0xffff_ff1a),
    // `slider.thumb_color`: platform-facts.md (§2.9): the knob is white at alpha 128/255
    // (measured, renders/darkAqua/slider-regular@2x.png); over background_color #999999
    slider_thumb: Some(srgb8(0xffff_ff80)),
    // `slider.thumb(PRESSED).fill`: white at alpha 158/255, at rest 128/255 (measured, os-probe
    // runs 45198750893 and 45208260830, macOS 15, `*.pressed.slider_knob` `fill`)
    slider_thumb_pressed: Some(srgb8(0xffff_ff9e)),
    // `slider.active_track(PRESSED).fill`: not read on macOS 15
    slider_fill_pressed: None,
    // `slider.disabled_fill_color`: soft_option interactive state colors (platform-facts.md
    // section 2.9 macOS) platform-facts.md (§2.9): no value fill when disabled, the bar white at
    // alpha 13/255 on both sides of the knob (measured, os-probe run 40348854007,
    // renders/darkAqua/slider-regular-disabled@2x.png); over background_color #3c3c3c
    slider_disabled_fill: srgb8(0xffff_ff0d),
    // `slider.disabled_track_color`: platform-facts.md (§2.9): white at alpha 13/255 (measured, the
    // same render); over background_color #3c3c3c
    slider_disabled_track: Some(srgb8(0xffff_ff0d)),
    // `slider.disabled_thumb_color`: platform-facts.md (§2.9): white at alpha 32/255 (measured, the
    // same render); over background_color #4c4c4c
    slider_disabled_thumb: Some(srgb8(0xffff_ff20)),
    // `spinner.fill_color`: platform-facts.md (§2.23): white fins at a graded alpha, the leading
    // one 82/255 (measured in the probe's renders/darkAqua/progress_spinning-regular@2x.png); over
    // the window background #747474
    spinner: srgb8(0xffff_ff52),
    // `splitter.hover_color`: Interactive state colors (platform-facts.md section 2.17 macOS)
    splitter_hover: Some(srgb8(0x3d3d_3dff)),
    // `menu.hover_text_color`: platform-facts.md: selectedMenuItemTextColor, probe
    menu_hover_foreground: srgb8(0xffff_ffff),
    // `tab.tab(PRESSED).fill`: an unselected tab pressed, white at alpha 100/255 (measured,
    // os-probe run 45198750893, macOS 15, `*.pressed.tab` `fill`)
    tab_pressed: srgb8(0xffff_ff64),
    // `sidebar.background_color`: platform-facts.md (§2.12): NSVisualEffectView.Material.sidebar's
    // opaque rendering with Reduce Transparency on, in an active view of a key window, measured on
    // macOS 14 and 15, which agree (os-probe run 40320895749, os-probe-macos-14/probe.toml)
    sidebar_background: srgb8(0x3838_38ff),
    // `sidebar.font.color`: platform-facts.md (§2.12): ← defaults.font.color, labelColor (probe)
    sidebar_foreground: Some(srgb8(0xffff_ffd8)),
    // `sidebar.item(PRESSED).fill`: the selection's, #0059d1 (measured, os-probe run 45198750893,
    // macOS 15, `*.pressed.source_list_row` `fill`)
    sidebar_pressed: None,
    // `switch.unchecked_background`: platform-facts.md (§2.21): white at alpha 26/255 (measured,
    // renders/darkAqua/switch-regular-off@2x.png); over background_color #474747
    switch_unchecked: srgb8(0xffff_ff1a),
    // `switch.thumb_background`: platform-facts.md (§2.21): white at alpha 191/255 (measured)
    switch_thumb: srgb8(0xffff_ffbf),
    // `switch.track(PRESSED).fill`: the off track pressed, white at alpha 41/255, at rest 26/255
    // (measured, os-probe run 45208260830, macOS 15, `*.pressed.switch_off` `common`)
    switch_pressed_unchecked: srgb8(0xffff_ff29),
    // `switch.disabled_unchecked_background`: platform-facts.md (§2.21): white at alpha 7/255
    // (measured, os-probe run 40348854007, renders/darkAqua/switch-regular-disabled-off@2x.png);
    // over background_color #383838
    switch_disabled_unchecked: srgb8(0xffff_ff07),
    // `switch.disabled_thumb_color`: platform-facts.md (§2.21): white at alpha 64/255 (measured,
    // the same render); over background_color #656565
    switch_disabled_thumb: Some(srgb8(0xffff_ff40)),
    // `combo_box.background_color`: platform-facts.md (§2.24): controlColor, probe; over the window
    // background #656565
    combo_background: srgb8(0xffff_ff3f),
    // `combo_box.font.color`: platform-facts.md (§2.24): controlTextColor, probe; over the fill
    // #e7e7e7
    combo_foreground: srgb8(0xffff_ffd8),
    // `combo_box.field(PRESSED).fill`: white at alpha 92/255, at rest 64/255, read on the
    // highlighted cell (measured, os-probe run 45208260830, macOS 15, `*.highlighted.popup`
    // `fill`)
    combo_pressed: srgb8(0xffff_ff5c),
    // `combo_box.disabled_background`: soft_option: NSPopUpButton disabled -- reduced alpha bg
    combo_disabled: Some(srgb8(0x3a3a_3c80)),
    // `segmented_control.active_background`: Interactive state colors (platform-facts.md section
    // 2.25 macOS) platform-facts.md (§2.25): the selected segment in a key window, measured on
    // macOS 14 and 15, which agree (os-probe run 40320895749, os-probe-macos-14/probe.toml)
    segmented_active: Some(srgb8(0x6666_66ff)),
    // `segmented_control.active_text_color`: platform-facts.md (§2.25): labelColor (probe); a key
    // window's selected label measures within one level of it over the segment on macOS 14 and 15
    // (os-probe run 40320895749, os-probe-macos-14/renders/key_window/darkAqua.png)
    segmented_active_foreground: Some(srgb8(0xffff_ffd8)),
    // `segmented_control.segment(PRESSED).fill`: an unselected segment pressed, white at alpha
    // 100/255 (measured, os-probe run 45198750893, macOS 15, `*.pressed.segment` `fill`)
    segment_pressed: srgb8(0xffff_ff64),
    // `expander.arrow_color`: NSDisclosureButton triangle color -- no source: the former muted
    // colour #98989d, not secondaryLabelColor (defaults.muted_color)
    expander_arrow: Some(srgb8(0x9898_9dff)),
    // `toolbar.button_label(PRESSED).color`: #ececec, at rest #a6a6a6, opaque over the opaque
    // title bar (measured, os-probe run 45208260830's read of run 45198750893, macOS 15,
    // `*.pressed.toolbar_button` `label`)
    toolbar_label_pressed: srgb8(0xecec_ecff),
    // `card.container.stroke`: NSBox (primary, untitled), white at alpha 38/255, 1 px (measured,
    // os-probe run 45198750893, macOS 15, `*.normal.box` `edge_*`)
    card_border: Some(srgb8(0xffff_ff26)),
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
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3),
    // unemphasizedSelectedContentBackgroundColor, the drawn row measured on macOS 15 and 26
    selection_inactive_background: srgb8(0xdcdc_dcff),
    // `defaults.selection_inactive_text_color`: platform-facts.md (§2.1.3), labelColor, the
    // unemphasized row's label measured on macOS 15 and 26
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
    // `button.active_background`: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`,
    // `button.body.fill` states [pressed] (measured, os-probe run 41006842699, macOS 26,
    // layers-key-push.toml `key.push.aqua.pressed.regular-plain.fill`)
    button_pressed: Some(srgb8(0x0000_002a)),
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, black at
    // 10/255 (measured, os-probe run 40348854007, renders/aqua/push_button-regular-disabled@2x.png,
    // not key); over background_color #f5f5f5
    button_disabled: srgb8(0x0000_000a),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #242424
    button_foreground: srgb8(0x0000_00d8),
    // `button.active_text_color`: as at rest: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at
    // `78ee0efd` states one `button.label.text.fill` per scheme for every state but the disabled
    // and default ones (the pressed capture reads #000000d7; os-probe run 41006842699, macOS 26)
    button_pressed_foreground: None,
    // `button.checked_background`: a checked toggle's fill is mix(accent, #000000, 0.025)
    // (measured, os-probe run 41006842699, macOS 26.6.2, both accents at every size: #0077f9 under
    // #007aff, #db373c under #e0383e; `docs/facts/macos/button.toml` on `v0.8.0-rc1` at
    // `78ee0efd`, from the run's `os-probe-macos-26/tiebreak-derivations.toml`,
    // `derivation.checked_fill_light_*`); pressed, mix(accent, #000000, 0.12) (#006be1 under
    // #007aff, #c53236 under #e0383e: `derivation.checked_pressed_fill_light_*`); disabled,
    // alpha(mix(accent, #000000, 0.04), 0.44) (#0076f470 under #007aff, #d6363d71 under #e0383e:
    // `derivation.checked_disabled_fill_light_*`)
    button_checked_mix_with: Some(srgb8(0x0000_00ff)),
    // `button.checked_text_color`: #ffffff (measured, the same run's
    // `os-probe-macos-26/layers-key-push.toml`, `key.push.aqua.normal.*-toggle_on.label`)
    button_checked_foreground: Some(srgb8(0xffff_ffff)),
    // a disabled checked toggle's label: #ffffff20 (measured, the same run's
    // `os-probe-macos-26/layers-key-push.toml`, `key.push.aqua.disabled.*-toggle_on.label`; under
    // the red accent too, `tiebreak-derivations.toml`, `derivation.checked_disabled_label_light_*`)
    button_checked_disabled_foreground: Some(srgb8(0xffff_ff20)),
    checkbox_hover: None,
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): macOS 26: the unchecked box is one
    // fill, systemFillColor's value #00000019 (measured, renders/aqua/checkbox-regular-off@2x.png;
    // probe); the probe's window is never ordered front, so this is the box of a window that is not
    // key
    checkbox_unchecked: srgb8(0x0000_0019),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): macOS 26: none (measured), no
    // edge pixel differs from the fill; the same caveat
    checkbox_unchecked_border: srgb8(0x0000_0000),
    // `checkbox.indicator(PRESSED).stroke`: as at rest: every pressed edge pixel is the fill
    // (measured, os-probe run 45198750893, macOS 26, `*.pressed.checkbox_*` `edge_*`)
    checkbox_pressed_border: None,
    // `checkbox.indicator(PRESSED).fill`: black at alpha 48/255, at rest 25/255 (measured, os-probe
    // run 45198750893, macOS 26, `*.pressed.checkbox_off` `fill`)
    checkbox_pressed: srgb8(0x0000_0030),
    // `checkbox.indicator(CHECKED|PRESSED).fill`: #006be1, at rest #0077f9, on the default accent
    // (measured, the same run, `*.pressed.checkbox_on` `fill`)
    checkbox_checked_pressed: Some(srgb8(0x006b_e1ff)),
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
    // `slider.track_color`: platform-facts.md (§2.9): systemFillColor's value (probe), black at
    // alpha 25/255 (measured, os-probe run 40348854007, renders/aqua/slider-regular@2x.png; a key
    // window draws #e4e4e4 over #ffffff, os-probe-macos-26/probe.toml), not muted_color; over
    // background_color #e6e6e6
    slider_track: srgb8(0x0000_0019),
    slider_thumb: None,
    // `slider.thumb(PRESSED).fill`: as at rest (measured, os-probe run 45217592058, macOS 26,
    // `*.pressed.slider_knob` `fill`)
    slider_thumb_pressed: None,
    // `slider.active_track(PRESSED).fill`: the track's filled part pressed, #006be1, at rest
    // #0077f9, on the default accent (the same run, `*.pressed.slider_knob`, `*.normal.slider_knob`
    // `fill`, `analysis/diff_layers-slider_knob.txt`)
    slider_fill_pressed: Some(srgb8(0x006b_e1ff)),
    // `slider.disabled_fill_color`: platform-facts.md (§2.9): no accent when disabled, the bar left
    // of the knob a grey darker than the track: #dfdfdf over the window's #ffffff in a key window
    // (os-probe run 40348854007, os-probe-macos-26/probe.toml), black at alpha 32/255; over
    // background_color #dfdfdf
    slider_disabled_fill: srgb8(0x0000_0020),
    slider_disabled_track: None,
    slider_disabled_thumb: None,
    // `spinner.fill_color`: platform-facts.md (§2.23): black fins at a graded alpha, the leading
    // one 82/255 (the render equals macOS 14.8.9's byte for byte); over the window background
    // #adadad
    spinner: srgb8(0x0000_0052),
    splitter_hover: None,
    // `menu.hover_text_color`: platform-facts.md: selectedMenuItemTextColor, probe
    menu_hover_foreground: srgb8(0xffff_ffff),
    // `tab.tab(PRESSED).fill`: an unselected tab pressed, black at alpha 63/255 (measured, os-probe
    // run 45198750893, macOS 26, `*.pressed.tab` `fill`)
    tab_pressed: srgb8(0x0000_003f),
    // `sidebar.background_color`: platform-facts.md (§2.12): NSVisualEffectView.Material.sidebar's
    // opaque rendering with Reduce Transparency on, in an active view of a key window, measured on
    // macOS 26 and xcode-27's macOS 27, which agree (os-probe run 40320895749,
    // os-probe-macos-26/probe.toml)
    sidebar_background: srgb8(0xf6f6_f6ff),
    sidebar_foreground: None,
    // `sidebar.item(PRESSED).fill`: #0076fb, on the default accent (measured, os-probe run
    // 45198750893, macOS 26, `*.pressed.source_list_row` `fill`)
    sidebar_pressed: Some(srgb8(0x0076_fbff)),
    // `switch.unchecked_background`: platform-facts.md (§2.21): black at alpha 25/255 (measured),
    // systemFillColor's value (probe); over background_color #e6e6e6
    switch_unchecked: srgb8(0x0000_0019),
    // `switch.thumb_background`: platform-facts.md (§2.21): white (measured)
    switch_thumb: srgb8(0xffff_ffff),
    // `switch.track(PRESSED).fill`: the off track pressed, black at alpha 50/255, at rest 26/255
    // (measured, os-probe run 45217592058, macOS 26, `*.pressed.switch_off` `common`; a near tie
    // with alpha 49/255)
    switch_pressed_unchecked: srgb8(0x0000_0032),
    // `switch.disabled_unchecked_background`: platform-facts.md (§2.21): black at alpha 13/255
    // (measured, os-probe run 40348854007, renders/aqua/switch-regular-disabled-off@2x.png); over
    // background_color #f2f2f2
    switch_disabled_unchecked: srgb8(0x0000_000d),
    switch_disabled_thumb: None,
    // `combo_box.background_color`: platform-facts.md (§2.24): drawn in secondarySystemFillColor's
    // value (measured; probe); over background_color #ebebeb
    combo_background: srgb8(0x0000_0014),
    // `combo_box.font.color`: platform-facts.md (§2.24): labelColor = controlTextColor, probe; over
    // the fill #242424
    combo_foreground: srgb8(0x0000_00d8),
    // `combo_box.field(PRESSED).fill`: black at alpha 42/255, at rest 19/255 (measured, os-probe
    // run 45198750893, macOS 26, `*.pressed.popup` `common`, on the part the open menu leaves
    // uncovered; run 45217592058's `*.highlighted.popup` `fill` the same)
    combo_pressed: srgb8(0x0000_002a),
    combo_disabled: None,
    segmented_active: None,
    segmented_active_foreground: None,
    // `segmented_control.segment(PRESSED).fill`: an unselected segment pressed, black at alpha
    // 63/255 (measured, os-probe run 45198750893, macOS 26, `*.pressed.segment` `fill`)
    segment_pressed: srgb8(0x0000_003f),
    expander_arrow: None,
    // `toolbar.button_label(PRESSED).color`: #414141, at rest #494949, opaque over the opaque
    // title bar (measured, os-probe run 45208260830's read of run 45198750893, macOS 26,
    // `*.pressed.toolbar_button` `label`)
    toolbar_label_pressed: srgb8(0x4141_41ff),
    // `card.container.stroke`: none: NSBox's edge pixels are its fill (measured, os-probe run
    // 45198750893, macOS 26, `*.normal.box` `edge_*`)
    card_border: None,
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
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3),
    // unemphasizedSelectedContentBackgroundColor, the drawn row measured on macOS 15 and 26
    selection_inactive_background: srgb8(0x4646_46ff),
    // `defaults.selection_inactive_text_color`: platform-facts.md (§2.1.3), labelColor, the
    // unemphasized row's label measured on macOS 15 and 26
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
    // `button.active_background`: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`,
    // `button.body.fill` states [pressed] (measured, os-probe run 41006842699, macOS 26,
    // layers-key-push.toml `key.push.darkAqua.pressed.regular-plain.fill`)
    button_pressed: Some(srgb8(0xffff_ff2a)),
    // `button.disabled_background`: platform-facts.md (§2.3): the bezel at half its alpha, white at
    // 10/255 (measured, os-probe run 40348854007,
    // renders/darkAqua/push_button-regular-disabled@2x.png, not key); over background_color #272727
    button_disabled: srgb8(0xffff_ff0a),
    // `button.font.color`: platform-facts.md (§2.3): controlTextColor, probe; over the button's
    // fill #dfdfdf
    button_foreground: srgb8(0xffff_ffd8),
    // `button.active_text_color`: as at rest: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at
    // `78ee0efd` states one `button.label.text.fill` per scheme for every state but the disabled
    // and default ones (the pressed capture reads #ffffffd9; os-probe run 41006842699, macOS 26)
    button_pressed_foreground: None,
    // `button.checked_background`: a checked toggle's fill is mix(accent, #ffffff, 0.025)
    // (measured, os-probe run 41006842699, macOS 26.6.2, both accents at every size: #077dff under
    // #007aff, #ff575b under #ff5257; `docs/facts/macos/button.toml` on `v0.8.0-rc1` at
    // `78ee0efd`, from the run's `os-probe-macos-26/tiebreak-derivations.toml`,
    // `derivation.checked_fill_dark_*`); pressed, mix(accent, #ffffff, 0.12) (#1f8aff under
    // #007aff, #ff676b under #ff5257: `derivation.checked_pressed_fill_dark_*`); disabled,
    // alpha(mix(accent, #ffffff, 0.04), 0.44) (#0982ff70 under #007aff, #ff5b5d70 under #ff5257:
    // `derivation.checked_disabled_fill_dark_*`)
    button_checked_mix_with: Some(srgb8(0xffff_ffff)),
    // `button.checked_text_color`: #ffffff (measured, the same run's
    // `os-probe-macos-26/layers-key-push.toml`, `key.push.darkAqua.normal.*-toggle_on.label`)
    button_checked_foreground: Some(srgb8(0xffff_ffff)),
    // a disabled checked toggle's label: #ffffff20, the parked facts' value for both schemes
    // (measured, the same run's `os-probe-macos-26/layers-key-push.toml`,
    // `key.push.darkAqua.disabled.*-toggle_on.label`, read #fefffe20 or #fdfffe20, within the
    // facts' tolerance of 2 levels; under the red accent too, `tiebreak-derivations.toml`,
    // `derivation.checked_disabled_label_dark_*`)
    button_checked_disabled_foreground: Some(srgb8(0xffff_ff20)),
    checkbox_hover: None,
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): macOS 26: the unchecked box is one
    // fill, systemFillColor's value #ffffff19 (measured,
    // renders/darkAqua/checkbox-regular-off@2x.png; probe); the probe's window is never ordered
    // front, so this is the box of a window that is not key
    checkbox_unchecked: srgb8(0xffff_ff19),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): macOS 26: none (measured), no
    // edge pixel differs from the fill; the same caveat
    checkbox_unchecked_border: srgb8(0x0000_0000),
    // `checkbox.indicator(PRESSED).stroke`: as at rest: every pressed edge pixel is the fill
    // (measured, os-probe run 45198750893, macOS 26, `*.pressed.checkbox_*` `edge_*`)
    checkbox_pressed_border: None,
    // `checkbox.indicator(PRESSED).fill`: white at alpha 48/255, at rest 25/255 (measured, os-probe
    // run 45198750893, macOS 26, `*.pressed.checkbox_off` `fill`)
    checkbox_pressed: srgb8(0xffff_ff30),
    // `checkbox.indicator(CHECKED|PRESSED).fill`: #1f8aff, at rest #077dff, on the default accent
    // (measured, the same run, `*.pressed.checkbox_on` `fill`)
    checkbox_checked_pressed: Some(srgb8(0x1f8a_ffff)),
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
    // `slider.track_color`: platform-facts.md (§2.9): systemFillColor's value (probe), white at
    // alpha 25/255 (measured, os-probe run 40348854007, renders/darkAqua/slider-regular@2x.png; a
    // key window draws #343434 over #1e1e1e, os-probe-macos-26/probe.toml), not muted_color; over
    // background_color #343434
    slider_track: srgb8(0xffff_ff19),
    // `slider.thumb_color`: platform-facts.md (§2.9): white at alpha 217/255 (measured,
    // renders/darkAqua/slider-regular@2x.png); over background_color #dddddd
    slider_thumb: Some(srgb8(0xffff_ffd9)),
    // `slider.thumb(PRESSED).fill`: the knob's own colour as at rest (measured, os-probe run
    // 45217592058, macOS 26, `*.pressed.slider_knob` `fill`)
    slider_thumb_pressed: None,
    // `slider.active_track(PRESSED).fill`: the track's filled part pressed, #1e8aff, at rest
    // #067dff, on the default accent (the same run and keys)
    slider_fill_pressed: Some(srgb8(0x1e8a_ffff)),
    // `slider.disabled_fill_color`: platform-facts.md (§2.9): no accent when disabled, the bar left
    // of the knob a grey lighter than the track: #393939 over the window's #1e1e1e in a key window
    // (os-probe run 40348854007, os-probe-macos-26/probe.toml), white at alpha 31/255; over
    // background_color #393939
    slider_disabled_fill: srgb8(0xffff_ff1f),
    slider_disabled_track: None,
    slider_disabled_thumb: None,
    // `spinner.fill_color`: platform-facts.md (§2.23): white fins at a graded alpha, the leading
    // one 82/255 (the render equals macOS 14.8.9's byte for byte); over the window background
    // #666666
    spinner: srgb8(0xffff_ff52),
    splitter_hover: None,
    // `menu.hover_text_color`: platform-facts.md: selectedMenuItemTextColor, probe
    menu_hover_foreground: srgb8(0xffff_ffff),
    // `tab.tab(PRESSED).fill`: an unselected tab pressed, white at alpha 63/255 (measured, os-probe
    // run 45198750893, macOS 26, `*.pressed.tab` `fill`)
    tab_pressed: srgb8(0xffff_ff3f),
    // `sidebar.background_color`: platform-facts.md (§2.12): NSVisualEffectView.Material.sidebar's
    // opaque rendering with Reduce Transparency on, in an active view of a key window, measured on
    // macOS 26 and xcode-27's macOS 27, which agree (os-probe run 40320895749,
    // os-probe-macos-26/probe.toml)
    sidebar_background: srgb8(0x3838_38ff),
    sidebar_foreground: None,
    // `sidebar.item(PRESSED).fill`: #1c96ff, on the default accent (measured, os-probe run
    // 45198750893, macOS 26, `*.pressed.source_list_row` `fill`)
    sidebar_pressed: Some(srgb8(0x1c96_ffff)),
    // `switch.unchecked_background`: platform-facts.md (§2.21): white at alpha 25/255 (measured),
    // systemFillColor's value (probe); over background_color #343434
    switch_unchecked: srgb8(0xffff_ff19),
    // `switch.thumb_background`: platform-facts.md (§2.21): white at alpha 221/255 (measured,
    // renders/darkAqua/switch-regular-off@2x.png)
    switch_thumb: srgb8(0xffff_ffdd),
    // `switch.track(PRESSED).fill`: the off track pressed, #fafafa at alpha 49/255, at rest #f5f5f5
    // at 26/255 (measured, os-probe run 45217592058, macOS 26, `*.pressed.switch_off` `common`)
    switch_pressed_unchecked: srgb8(0xfafa_fa31),
    // `switch.disabled_unchecked_background`: platform-facts.md (§2.21): white at alpha 13/255
    // (measured, os-probe run 40348854007, renders/darkAqua/switch-regular-disabled-off@2x.png);
    // over background_color #292929
    switch_disabled_unchecked: srgb8(0xffff_ff0d),
    // `switch.disabled_thumb_color`: platform-facts.md (§2.21): white at alpha 219/255, where the
    // enabled knob is 221/255 (measured, the same render); over background_color #dfdfdf
    switch_disabled_thumb: Some(srgb8(0xffff_ffdb)),
    // `combo_box.background_color`: platform-facts.md (§2.24): drawn in secondarySystemFillColor's
    // value (measured; probe); over background_color #303030
    combo_background: srgb8(0xffff_ff14),
    // `combo_box.font.color`: platform-facts.md (§2.24): labelColor = controlTextColor, probe; over
    // the fill #dfdfdf
    combo_foreground: srgb8(0xffff_ffd8),
    // `combo_box.field(PRESSED).fill`: white at alpha 42/255, at rest 19/255 (measured, os-probe
    // run 45198750893, macOS 26, `*.pressed.popup` `common`, on the part the open menu leaves
    // uncovered; run 45217592058's `*.highlighted.popup` `fill` the same)
    combo_pressed: srgb8(0xffff_ff2a),
    combo_disabled: None,
    segmented_active: None,
    segmented_active_foreground: None,
    // `segmented_control.segment(PRESSED).fill`: an unselected segment pressed, white at alpha
    // 63/255 (measured, os-probe run 45198750893, macOS 26, `*.pressed.segment` `fill`)
    segment_pressed: srgb8(0xffff_ff3f),
    expander_arrow: None,
    // `toolbar.button_label(PRESSED).color`: #ebebeb, at rest #e7e7e7, opaque over the opaque
    // title bar (measured, os-probe run 45208260830's read of run 45198750893, macOS 26,
    // `*.pressed.toolbar_button` `label`)
    toolbar_label_pressed: srgb8(0xebeb_ebff),
    // `card.container.stroke`: none: NSBox's edge pixels are its fill (measured, os-probe run
    // 45198750893, macOS 26, `*.normal.box` `edge_*`)
    card_border: None,
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
    /// Whether the push button's bezel is outlined in the default line;
    /// `false` where the measurement reads no stroke.
    button_outlined: bool,
    checkbox_shadow_enabled: bool,
    checkbox_indicator_width: f32,
    checkbox_label_gap: f32,
    radio_dot_diameter: f32,
    input_corner_radius: Option<f32>,
    input_focus_ring_width: f32,
    input_focus_ring_offset: f32,
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
    slider_track_height: f32,
    slider_thumb_diameter: f32,
    slider_tick_length: f32,
    progress_bar_track_height: f32,
    progress_bar_min_width: f32,
    progress_bar_padding_vertical: f32,
    progress_bar_padding_horizontal: f32,
    progress_bar_shadow_enabled: bool,
    splitter_divider: Color,
    splitter_divider_width: f32,
    spinner_diameter: f32,
    spinner_min_diameter: f32,
    spinner_stroke_width: f32,
    menu_row_height: Option<f32>,
    menu_icon_text_gap: f32,
    menu_icon_size: f32,
    menu_padding_vertical: Option<f32>,
    menu_padding_horizontal: Option<f32>,
    tab_min_width: f32,
    tab_min_height: f32,
    tab_padding_vertical: Option<f32>,
    tab_padding_horizontal: Option<f32>,
    toolbar_bar_height: Option<f32>,
    toolbar_item_gap: Option<f32>,
    /// Whether a hovered toolbar button's label is measured as at rest.
    toolbar_label_hover_as_rest: bool,
    toolbar_padding_vertical: Option<f32>,
    toolbar_padding_horizontal: Option<f32>,
    switch_track_width: f32,
    switch_track_height: f32,
    switch_track_radius: f32,
    switch_thumb_diameter: f32,
    switch_disabled_checked_alpha: u8,
    dialog_padding_vertical: Option<f32>,
    dialog_padding_horizontal: Option<f32>,
    dialog_min_width: f32,
    dialog_max_width: f32,
    dialog_min_height: f32,
    dialog_max_height: f32,
    dialog_button_gap: f32,
    dialog_icon_size: f32,
    dialog_button_order: DialogButtonOrder,
    alert_title_size_pt: f32,
    alert_title_weight: u16,
    combo_min_width: f32,
    combo_min_height: f32,
    combo_arrow_size: f32,
    combo_padding_top: f32,
    combo_padding_bottom: f32,
    combo_padding_left: f32,
    combo_shadow_enabled: bool,
    /// Whether the pop-up button's bezel is outlined in the default line;
    /// `false` where the measurement reads no stroke.
    combo_outlined: bool,
    segment_height: f32,
    segment_separator_width: f32,
    segment_padding_vertical: Option<f32>,
    segmented_shadow_enabled: bool,
    /// Whether the track and a segment's edge are the control's bezel line
    /// in the default line; `false` where the measurement reads no stroke.
    segment_outlined: bool,
    card_padding_vertical: f32,
    card_padding_horizontal: f32,
    /// The box's corner radius.
    card_corner_radius: f32,
    /// The box's outline width; `None` where it draws none.
    card_border_width: Option<f32>,
    expander_header_height: f32,
    expander_arrow_size: f32,
    expander_arrow_side: ArrowSide,
    expander_shadow_enabled: bool,
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
    // `defaults.focus_ring_width`: platform-facts.md (§2.1.5): focus_ring_width: 3px (measured)
    focus_ring_width: 3.0,
    // `defaults.focus_ring_offset`: platform-facts.md (§2.1.5): focus_ring_offset: -1px (measured),
    // inset
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
    // `button.border`: 0.7.2's default line; the bezel's stroke on macOS 15 is pending (U135) in
    // `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`
    button_outlined: true,
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
    // The text field's own focus ring, which its mouse-down brings: 4 px wide, from 2 px inside
    // the field's frame to 2 px outside it (offsets -2 … 1), on every side (measured, os-probe run
    // 45231684209, macOS 15, `*.pressed.text_field` `around_*`)
    input_focus_ring_width: 4.0,
    input_focus_ring_offset: -2.0,
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
    // `slider.track_height`: platform-facts.md (§2.9): barRectFlipped 4 high (probe)
    slider_track_height: 4.0,
    // `slider.thumb_diameter`: platform-facts.md (§2.9): NSSlider knob 20 (measured, macOS 14.8.9
    // probe render)
    slider_thumb_diameter: 20.0,
    // `slider.tick_mark_length`: 0.7.2 macos-monterey.toml (uncited)
    slider_tick_length: 8.0,
    // `progress_bar.track_height`: 0.7.2 macos-monterey.toml (uncited)
    progress_bar_track_height: 6.0,
    // `progress_bar.min_width`: 0.7.2 macos-monterey.toml (uncited)
    progress_bar_min_width: 100.0,
    // `progress_bar.border.padding.top`, `progress_bar.border.padding.bottom`: 0.7.2
    // macos-monterey.toml (uncited)
    progress_bar_padding_vertical: 0.0,
    // `progress_bar.border.padding.left`, `progress_bar.border.padding.right`: platform-facts.md
    // (§2.10): 0; 0 (measured), the fill starts in the track's first column and covers its rows
    progress_bar_padding_horizontal: 0.0,
    // `progress_bar.border.shadow_enabled`: platform-facts.md (§2.10): no shadow
    progress_bar_shadow_enabled: false,
    // `splitter.divider_color`: 0.7.2 macos-monterey.toml (uncited)
    splitter_divider: srgb8(0x0000_0000),
    // `splitter.divider_width`: platform-facts.md (§2.17): NSSplitView's default .thick divider,
    // dividerThickness 9 (macOS 14.8.9 probe), dividerColor clear
    splitter_divider_width: 9.0,
    // `spinner.diameter`: 0.7.2 macos-monterey.toml (uncited)
    spinner_diameter: 32.0,
    // `spinner.min_diameter`: 0.7.2 macos-monterey.toml (uncited)
    spinner_min_diameter: 10.0,
    // `spinner.stroke_width`: 0.7.2 macos-monterey.toml (uncited)
    spinner_stroke_width: 2.0,
    // `menu.row_height`: 0.7.2 macos-monterey.toml (uncited)
    menu_row_height: Some(22.0),
    // `menu.icon_text_gap`: 0.7.2 macos-monterey.toml (uncited)
    menu_icon_text_gap: 4.0,
    // `menu.icon_size`: platform-facts.md (§2.6): 16 (cross-project)
    menu_icon_size: 16.0,
    // `menu.border.padding.top`, `menu.border.padding.bottom`: 0.7.2 macos-monterey.toml (uncited)
    menu_padding_vertical: Some(3.0),
    // `menu.border.padding.left`, `menu.border.padding.right`: platform-facts.md (§2.6): NSMenuItem
    // 12; 3 (measured)
    menu_padding_horizontal: Some(12.0),
    // `tab.min_width`: 0.7.2 macos-monterey.toml (uncited)
    tab_min_width: 64.0,
    // `tab.min_height`: 0.7.2 macos-monterey.toml (uncited)
    tab_min_height: 24.0,
    // `tab.border.padding.top`, `tab.border.padding.bottom`: 0.7.2 macos-monterey.toml (uncited)
    tab_padding_vertical: Some(4.0),
    // `tab.border.padding.left`, `tab.border.padding.right`: platform-facts.md (§2.11): NSTabView
    // 12; 4 (measured)
    tab_padding_horizontal: Some(12.0),
    // `toolbar.bar_height`: platform-facts.md (§2.13): NSToolbar 38
    toolbar_bar_height: Some(38.0),
    // `toolbar.item_gap`: platform-facts.md (§2.13): AppKit 8
    toolbar_item_gap: Some(8.0),
    // `toolbar.button_label(HOVER)`: as at rest, #737373 / #a6a6a6 in the toolbar, and translucent
    // and unchanged over the probe's backdrops (measured, os-probe run 45208260830's read of run
    // 45198750893, macOS 15, `*.hovered.toolbar_button` `label`; run 45231684209's toolbar sheet,
    // `*.normal.toolbar_button`, `*.hovered.toolbar_button` `label`)
    toolbar_label_hover_as_rest: true,
    // `toolbar.border.padding.top`, `toolbar.border.padding.bottom`: 0.7.2 macos-monterey.toml
    // (uncited)
    toolbar_padding_vertical: Some(0.0),
    // `toolbar.border.padding.left`, `toolbar.border.padding.right`: platform-facts.md (§2.13):
    // NSToolbar 8 (measured); 0
    toolbar_padding_horizontal: Some(8.0),
    // `switch.track_width`: platform-facts.md (§2.21): NSSwitch intrinsicContentSize 38 x 22 (probe)
    switch_track_width: 38.0,
    // `switch.track_height`: 0.7.2 macos-monterey.toml (uncited)
    switch_track_height: 22.0,
    // `switch.track_radius`: 0.7.2 macos-monterey.toml (uncited)
    switch_track_radius: 11.0,
    // `switch.thumb_diameter`: platform-facts.md (§2.21): the knob's fill, 20 (measured)
    switch_thumb_diameter: 20.0,
    // `switch.disabled_checked_background`: platform-facts.md (§2.21): accent_color at alpha
    // 128/255, as macOS draws the enabled on track at one half (measured in a key window, os-probe
    // run 40348854007, os-probe-macos-14/probe.toml); the macOS reader writes the user's accent at
    // it; over background_color #76b3f6 in light, over background_color #195699 in dark, where the
    // key window drew #1f4e81 (probe), macOS's dark on track being darker than the accent it reads
    switch_disabled_checked_alpha: 0x80,
    // `dialog.border.padding.top`, `dialog.border.padding.bottom`: 0.7.2 macos-monterey.toml
    // (uncited)
    dialog_padding_vertical: Some(20.0),
    // `dialog.border.padding.left`, `dialog.border.padding.right`: platform-facts.md (§2.22): ~20
    // (measured)
    dialog_padding_horizontal: Some(20.0),
    // `dialog.min_width`: 0.7.2 macos-monterey.toml (uncited)
    dialog_min_width: 320.0,
    // `dialog.max_width`: 0.7.2 macos-monterey.toml (uncited)
    dialog_max_width: 560.0,
    // `dialog.min_height`: 0.7.2 macos-monterey.toml (uncited)
    dialog_min_height: 140.0,
    // `dialog.max_height`: 0.7.2 macos-monterey.toml (uncited)
    dialog_max_height: 600.0,
    // `dialog.button_gap`: 0.7.2 macos-monterey.toml (uncited)
    dialog_button_gap: 12.0,
    // `dialog.icon_size`: 0.7.2 macos-monterey.toml (uncited)
    dialog_icon_size: 64.0,
    // `dialog.button_order`: 0.7.2 macos-monterey.toml (uncited)
    dialog_button_order: DialogButtonOrder::PrimaryRight,
    // `dialog.title_font.size`: platform-facts.md (§2.22): the emphasized system font, 13pt Bold
    alert_title_size_pt: 13.0,
    // `dialog.title_font.weight`: 0.7.2 macos-monterey.toml (uncited)
    alert_title_weight: 700,
    // `combo_box.min_width`: 0.7.2 macos-monterey.toml (uncited)
    combo_min_width: 120.0,
    // `combo_box.min_height`: platform-facts.md (§2.24): NSPopUpButton intrinsicContentSize 20
    // (probe)
    combo_min_height: 20.0,
    // `combo_box.arrow_icon_size`: no source: platform-facts.md measures the chevron 6 x 10, which
    // a square icon size cannot carry
    combo_arrow_size: 17.0,
    // `combo_box.border.padding.top`: 0.7.2 macos-monterey.toml (uncited)
    combo_padding_top: 1.0,
    // `combo_box.border.padding.bottom`: 0.7.2 macos-monterey.toml (uncited)
    combo_padding_bottom: 3.0,
    // `combo_box.border.padding.left`: platform-facts.md (§2.24): 8 left, 1 top / 3 bottom (API,
    // the title rect, probe), from the bezel's outer edge; the right side holds the arrow
    combo_padding_left: 8.0,
    // `combo_box.border.shadow_enabled`: platform-facts.md (§2.24): no shadow
    combo_shadow_enabled: false,
    // `combo_box.border`: 0.7.2's default line, as the push button's
    combo_outlined: true,
    // `segmented_control.segment_height`: platform-facts.md (§2.25): intrinsicContentSize 22 high,
    // the alignment rect (probe)
    segment_height: 22.0,
    // `segmented_control.separator_width`: 0.7.2 macos-monterey.toml (uncited)
    segment_separator_width: 1.0,
    // `segmented_control.border.padding.top`, `segmented_control.border.padding.bottom`:
    // platform-facts.md (§2.25): ~3 (measured)
    segment_padding_vertical: Some(3.0),
    // `segmented_control.border.shadow_enabled`: platform-facts.md (§2.25): no shadow
    segmented_shadow_enabled: false,
    // `segmented_control.segment(HOVER).stroke`: the bezel's line, as at rest (os-probe run
    // 45198750893, macOS 15, `*.hovered.segment` differs from the normal capture in 0 pixels)
    segment_outlined: true,
    // `card.border.padding.top`, `card.border.padding.bottom`: 0.7.2 macos-monterey.toml (uncited)
    card_padding_vertical: 5.0,
    // `card.border.padding.left`, `card.border.padding.right`: platform-facts.md (§2.26):
    // NSBox.contentViewMargins, "By default, these are both 5.0" (API); the box's fill and border
    // are not published (§2.26 background_color, ❓), so the card's fill stays inherited
    card_padding_horizontal: 5.0,
    // `card.container.corners`: NSBox's corners, 4.75–5.0 px by the half-coverage edge fit on all
    // four corners (measured, os-probe runs 45198750893 and 45208260830, macOS 15, `*.normal.box`
    // `corner_edge_*`)
    card_corner_radius: 5.0,
    // `card.container.stroke.width`: 1 px (measured, os-probe run 45198750893, macOS 15,
    // `*.normal.box` `edge_*`)
    card_border_width: Some(1.0),
    // `expander.header_height`: 0.7.2 macos-monterey.toml (uncited)
    expander_header_height: 40.0,
    // `expander.arrow_icon_size`: 0.7.2 macos-monterey.toml (uncited)
    expander_arrow_size: 13.0,
    // `expander.arrow_side`: platform-facts.md (§2.27): the HIG's disclosure triangle points in
    // from the leading edge
    expander_arrow_side: ArrowSide::Leading,
    // `expander.border.shadow_enabled`: platform-facts.md (§2.27): no shadow
    expander_shadow_enabled: false,
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
    // `button.border`: no stroke: `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`
    // states `button.body.stroke` absent (measured, os-probe run 41006842699, macOS 26,
    // layers-key-push.toml `key.push.<appearance>.normal.regular-plain.stroke` absent)
    button_outlined: false,
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
    // The text field's own focus ring, which its mouse-down brings: 4 px wide, from 1 px inside
    // the field's frame to 3 px outside it (offsets -1 … 2), on every side (measured, os-probe runs
    // 45231684209 and 45252258079, macOS 26, `*.pressed.text_field` `around_*`)
    input_focus_ring_width: 4.0,
    input_focus_ring_offset: -1.0,
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
    // `slider.track_height`: platform-facts.md (§2.9): barRectFlipped 6 high (probe)
    slider_track_height: 6.0,
    // `slider.thumb_diameter`: platform-facts.md (§2.9): the knob is a 20 x 16 capsule (probe); its
    // 16 across the track (docs/archive/todo_v0.7.0_code-review-findings.md, M2a Steps 8-12)
    slider_thumb_diameter: 16.0,
    // `slider.tick_mark_length`: platform-facts.md (§2.9): rectOfTickMarkAtIndex 2 x 2 (probe)
    slider_tick_length: 2.0,
    // `progress_bar.track_height`: platform-facts.md (§2.10): 8 (measured,
    // renders/aqua/progress_bar-regular@2x.png in light, renders/darkAqua/progress_bar-regular@2x.png
    // in dark)
    progress_bar_track_height: 8.0,
    // `progress_bar.min_width`: no macOS 26 source (platform-facts.md, (none)): macos-monterey's
    // value
    progress_bar_min_width: 100.0,
    // `progress_bar.border.padding.top`, `progress_bar.border.padding.bottom`: 0.7.2
    // macos-tahoe.toml (uncited)
    progress_bar_padding_vertical: 0.0,
    // `progress_bar.border.padding.left`, `progress_bar.border.padding.right`: platform-facts.md
    // (§2.10): 0; 0 (measured, macOS 26.6.2), the fill starts in the track's first column and
    // covers its rows
    progress_bar_padding_horizontal: 0.0,
    // `progress_bar.border.shadow_enabled`: platform-facts.md (§2.10): no shadow (measured)
    progress_bar_shadow_enabled: false,
    // `splitter.divider_color`: 0.7.2 macos-tahoe.toml (uncited)
    splitter_divider: srgb8(0x0000_0000),
    // `splitter.divider_width`: platform-facts.md (§2.17): NSSplitView's default .thick divider,
    // dividerThickness 9 (probe), dividerColor clear
    splitter_divider_width: 9.0,
    // `spinner.diameter`: platform-facts.md (§2.23): sizeToFit 32 regular, 10 mini (probe)
    spinner_diameter: 32.0,
    // `spinner.min_diameter`: 0.7.2 macos-tahoe.toml (uncited)
    spinner_min_diameter: 10.0,
    // `spinner.stroke_width`: no macOS 26 source (platform-facts.md, (none)): macos-monterey's
    // value
    spinner_stroke_width: 2.0,
    menu_row_height: None,
    // `menu.icon_text_gap`: no macOS 26 source (platform-facts.md): macos-monterey's value
    menu_icon_text_gap: 4.0,
    // `menu.icon_size`: no macOS 26 source (platform-facts.md): macos-monterey's value; the
    // inherited toolbar size would be the wrong kind
    menu_icon_size: 16.0,
    menu_padding_vertical: None,
    menu_padding_horizontal: None,
    // `tab.min_width`: no macOS 26 source (platform-facts.md): macos-monterey's values
    tab_min_width: 64.0,
    // `tab.min_height`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no macOS 26
    // source (platform-facts.md): macos-monterey's values
    tab_min_height: 24.0,
    tab_padding_vertical: None,
    tab_padding_horizontal: None,
    toolbar_bar_height: None,
    toolbar_item_gap: None,
    // `toolbar.button_label(HOVER)`: not settled: over the probe's backdrops macOS 26 draws an
    // opaque capsule that hides them, and one translucent label fits every read (os-probe run
    // 45231684209, toolbar sheet, `*.toolbar_button` `label`, `fill`)
    toolbar_label_hover_as_rest: false,
    toolbar_padding_vertical: None,
    toolbar_padding_horizontal: None,
    // `switch.track_width`: platform-facts.md (§2.21): NSSwitch intrinsicContentSize 54 x 24 (probe)
    switch_track_width: 54.0,
    // `switch.track_height`: 0.7.2 macos-tahoe.toml (uncited)
    switch_track_height: 24.0,
    // `switch.track_radius`: platform-facts.md (§2.21): half the track's height, a pill (the
    // render's ends are round)
    switch_track_radius: 12.0,
    // `switch.thumb_diameter`: platform-facts.md (§2.21): the knob is a 32 x 20 capsule (measured);
    // its 20 across the track (docs/archive/todo_v0.7.0_code-review-findings.md, M2a Steps 8-12)
    switch_thumb_diameter: 20.0,
    // `switch.disabled_checked_background`: platform-facts.md (§2.21): accent_color at alpha
    // 115/255, as macOS 26 draws the enabled on track at about 0.45 (measured in a key window,
    // os-probe run 40348854007, os-probe-macos-26/probe.toml); the macOS reader writes the user's
    // accent at it; over background_color #8cc3ff in light, over background_color #104783 in dark
    switch_disabled_checked_alpha: 0x73,
    dialog_padding_vertical: None,
    dialog_padding_horizontal: None,
    // `dialog.min_width`: no macOS 26 source (platform-facts.md): macos-monterey's values
    dialog_min_width: 320.0,
    // `dialog.max_width`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no macOS
    // 26 source (platform-facts.md): macos-monterey's values
    dialog_max_width: 560.0,
    // `dialog.min_height`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    dialog_min_height: 140.0,
    // `dialog.max_height`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    dialog_max_height: 600.0,
    // `dialog.button_gap`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    dialog_button_gap: 12.0,
    // `dialog.icon_size`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no macOS
    // 26 source (platform-facts.md): macos-monterey's values
    dialog_icon_size: 64.0,
    // `dialog.button_order`: platform-facts.md (§2.22): the default button rightmost
    dialog_button_order: DialogButtonOrder::PrimaryRight,
    // `dialog.title_font.size`: no macOS 26 source (platform-facts.md): macos-monterey's values,
    // the emphasized system font, 13pt Bold; the inherited defaults.font would be the regular
    // weight
    alert_title_size_pt: 13.0,
    // `dialog.title_font.weight`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group:
    // no macOS 26 source (platform-facts.md): macos-monterey's values, the emphasized system font,
    // 13pt Bold; the inherited defaults.font would be the regular weight
    alert_title_weight: 700,
    // `combo_box.min_width`: no macOS 26 source (platform-facts.md, (none)): macos-monterey's value
    combo_min_width: 120.0,
    // `combo_box.min_height`: platform-facts.md (§2.24): NSPopUpButton intrinsicContentSize 24
    // (probe)
    combo_min_height: 24.0,
    // `combo_box.arrow_icon_size`: no macOS 26 source (platform-facts.md: the chevron is 7 x 10.5,
    // which a square icon size cannot carry): macos-monterey's value
    combo_arrow_size: 17.0,
    // `combo_box.border.padding.top`: 0.7.2 macos-tahoe.toml (uncited)
    combo_padding_top: 4.0,
    // `combo_box.border.padding.bottom`: 0.7.2 macos-tahoe.toml (uncited)
    combo_padding_bottom: 4.0,
    // `combo_box.border.padding.left`: platform-facts.md (§2.24): the title rect [12, 4, 41, 16] in
    // the 89 x 24 frame (probe), from its outer edge; the right side holds the arrow
    combo_padding_left: 12.0,
    // `combo_box.border.shadow_enabled`: platform-facts.md (§2.24): no shadow (measured)
    combo_shadow_enabled: false,
    // `combo_box.border`: no stroke: at rest the pop-up's edge pixels are its fill #00000013 /
    // #ffffff13 (measured, os-probe run 45198750893, macOS 26, `*.normal.popup` `edge_*`)
    combo_outlined: false,
    // `segmented_control.segment_height`: platform-facts.md (§2.25): intrinsicContentSize 24 high,
    // the alignment rect (probe)
    segment_height: 24.0,
    // `segmented_control.separator_width`: no macOS 26 source (platform-facts.md): macos-monterey's
    // value
    segment_separator_width: 1.0,
    segment_padding_vertical: None,
    // `segmented_control.border.shadow_enabled`: platform-facts.md (§2.25): no shadow (measured)
    segmented_shadow_enabled: false,
    // `segmented_control.segment(HOVER|PRESSED).stroke`: none: at rest, hovered (0 pixels
    // differ) and pressed the segment's top, bottom and left edge pixels are its fill (measured,
    // os-probe run 45198750893, macOS 26, `*.normal.segment`, `*.hovered.segment`,
    // `*.pressed.segment` `edge_*`); the track's too (`segmented_control.track.stroke`): the
    // measured part spans the control's whole height, so its top and bottom edge pixels are the
    // control's, and they are the fill #00000013 / #ffffff13 (`*.normal.segment` `edge_top`,
    // `edge_bottom`; the control's two ends were not read), where 0.7.2 outlines it in the
    // default line
    segment_outlined: false,
    // `card.border.padding.top`, `card.border.padding.bottom`: 0.7.2 macos-tahoe.toml (uncited)
    card_padding_vertical: 5.0,
    // `card.border.padding.left`, `card.border.padding.right`: platform-facts.md (§2.26):
    // NSBox.contentViewMargins, "By default, these are both 5.0" (API); on macOS 26 the same,
    // inferred from the current page, which is not versioned and makes no macOS 26 statement; the
    // box's fill and border are not published (§2.26 background_color, ❓), so the card's fill
    // stays inherited
    card_padding_horizontal: 5.0,
    // `card.container.corners`: NSBox's corners, 12.0–12.25 px by the half-coverage edge fit on
    // all four corners (measured, os-probe runs 45198750893 and 45217592058, macOS 26,
    // `*.normal.box` `corner_edge_*`; no stroke, so the fit's reference is the fill's alpha 8/255)
    card_corner_radius: 12.0,
    // `card.container.stroke`: none drawn (`card_border`)
    card_border_width: None,
    // `expander.header_height`: no macOS 26 source (platform-facts.md): macos-monterey's values
    expander_header_height: 40.0,
    // `expander.arrow_icon_size`: 0.7.2 macos-tahoe.toml (uncited); the TOML's note on the group: no
    // macOS 26 source (platform-facts.md): macos-monterey's values
    expander_arrow_size: 13.0,
    // `expander.arrow_side`: platform-facts.md (§2.27): the HIG's disclosure triangle points in
    // from the leading edge
    expander_arrow_side: ArrowSide::Leading,
    // `expander.border.shadow_enabled`: no macOS 26 source (platform-facts.md): macos-monterey's
    // value; the inherited defaults.border.shadow_enabled would be the wrong kind
    expander_shadow_enabled: false,
};

/// The font DPI a point size converts at: 72, "where a point is a pixel"
/// (0.7.2's `ResolutionContext::from_system` on macOS, `src/resolve/context.rs`;
/// spec §4, `Typography::font_dpi`: "72.0 Apple").
const FONT_DPI: f32 = 72.0;

/// The focus ring's alpha over an accent laid as given, 0x7F of 255: the
/// alpha of `keyboardFocusIndicatorColor` under both accents the OS probe
/// measured (the default and `AppleAccentColor` 0), in both appearances, on
/// macOS 14, 15 and 26 (spec §6, step 2; `docs/evidence/os-probe/`
/// `2026-10-26-run-40348854007`, `os-probe-macos-{14,15,26}/probe.toml` and
/// `os-probe-macos-{14,15,26}/accent-red/probe.toml`).
const FOCUS_RING_ALPHA: u8 = 0x7f;

/// The weight of the colour `macos-tahoe` mixes into the accent for a checked
/// toggle's fill ([`SchemeColors::button_checked_mix_with`]): 0.025, in sRGB,
/// the mix the parked facts measured under the default and the red accent, in
/// both appearances and at every size (spec §8, "Derived by the macOS
/// builders"; `docs/facts/macos/button.toml` on `v0.8.0-rc1` at `78ee0efd`,
/// os-probe run 41006842699, `os-probe-macos-26/tiebreak-derivations.toml`,
/// `derivation.checked_fill_*`).
const BUTTON_CHECKED_MIX: f32 = 0.025;

/// The weight of the colour `macos-tahoe` mixes into the accent for a
/// pressed checked toggle's fill: 0.12, in sRGB, as [`BUTTON_CHECKED_MIX`]
/// is measured (the same facts and run,
/// `derivation.checked_pressed_fill_*`).
const BUTTON_CHECKED_PRESSED_MIX: f32 = 0.12;

/// The weight of the colour `macos-tahoe` mixes into the accent for a
/// disabled checked toggle's fill, which is then at alpha
/// [`BUTTON_CHECKED_DISABLED_ALPHA`]: 0.04, in sRGB, as
/// [`BUTTON_CHECKED_MIX`] is measured (the same facts and run,
/// `derivation.checked_disabled_fill_*`).
const BUTTON_CHECKED_DISABLED_MIX: f32 = 0.04;

/// The alpha of a disabled checked toggle's fill on `macos-tahoe`: 0.44
/// (the same facts and run, `derivation.checked_disabled_fill_*`).
const BUTTON_CHECKED_DISABLED_ALPHA: f32 = 0.44;

/// The space between a switch's label and its track, on macOS 15 and 26: 8
/// (measured, os-probe run 45210835145, SwiftUI `Toggle` with
/// `.toggleStyle(.switch)`, `switch-label.toml` `[[item]] toggle_off`: the
/// item's fitting width 33 + 8 + the switch's; `measure.toml`
/// `layout_gap_host_pt` 8.0 on macOS 15).
const SWITCH_LABEL_GAP: f32 = 8.0;

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

/// The theme of `macos-monterey` for `ctx.scheme` ([`resolve`]).
pub(crate) fn resolve_monterey(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    resolve(&MONTEREY, ctx, inputs)
}

/// The theme of `macos-tahoe` for `ctx.scheme` ([`resolve`]).
pub(crate) fn resolve_tahoe(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    resolve(&TAHOE, ctx, inputs)
}

/// The theme of the preset of `g` for `ctx.scheme`, from the preset's
/// constants with the system's settings `inputs.system` laid over them, as
/// the 0.7.2 reader writes them over the preset (spec §8, "The macOS part";
/// 0.7.2's `macos::reader_result`): the colours of the entry for the
/// resolved scheme and contrast ([`lay_system_over`]), the title bar's text
/// among them; the fonts ([`laid_over`]); the scroller's style and width;
/// the font DPI. The disabled on switch's track is the accent at the
/// alpha of `g`'s own generation ([`switch`]). Then the active palette
/// ([`lay_palette_over`]) and the explicit accent ([`lay_accent_over`]),
/// each over the layer below it (spec §6, resolution step 2).
fn resolve(g: &Generation, ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let system = inputs.system;
    let mut scheme_colors = match ctx.scheme {
        Scheme::Light => g.light,
        Scheme::Dark => g.dark,
    };
    let os = system.and_then(|s| s.colors_for(ctx.scheme, ctx.contrast));
    if let Some(os) = os {
        lay_system_over(&mut scheme_colors, os);
    }
    let explicit_accent = inputs.accent.is_some();
    if let Some(p) = inputs.palette {
        lay_palette_over(&mut scheme_colors, p, explicit_accent);
    }
    if let Some(accent) = inputs.accent {
        lay_accent_over(&mut scheme_colors, accent, inputs.palette.is_none());
    }
    let title_bar_foreground = os
        .and_then(|os| os.surfaces.as_ref())
        .and_then(|s| s.title_bar)
        .and_then(|s| s.foreground);
    let c = &scheme_colors;
    let colors = roles(c, g, inputs.palette);
    let units = FontUnits {
        dpi: system.and_then(|s| s.font_dpi).unwrap_or(FONT_DPI),
        text_scale: ctx.text_scale,
    };
    let typography = typography(g, &colors, system, units);
    let fonts = SystemFonts::of(system);
    let motion = motion(ctx.motion, None);
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
    ResolvedTheme {
        name: Arc::from(g.preset.display_name()),
        preset: g.preset,
        context: ctx.clone(),
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
        motion_tokens: motion_tokens(MotionTokens::default(), motion),
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
        scrollbar: scrollbar(
            c,
            g,
            &colors,
            system
                .and_then(|s| s.scrollbar_width_px)
                .unwrap_or(g.scrollbar_groove_width),
            system
                .and_then(|s| s.overlay_scrollbars)
                .unwrap_or(g.scrollbar_overlay),
        ),
        list: list(c, g, &colors, &typography, &layout),
        tooltip: tooltip(c, g, &layout, &typography, fonts.tooltip, units),
        window: window(
            c,
            laid_over(
                resized(
                    label(
                        &typography.body,
                        title_bar_foreground.or(colors.window.foreground),
                    ),
                    None,
                    g.title_bar_font_weight,
                    units,
                ),
                fonts.title_bar,
                units,
            ),
            &layout,
            ctx,
        ),
        input: input(c, g, &colors, &typography, &layout, true),
        text_area: input(c, g, &colors, &typography, &layout, false),
        link: link(c, g, &colors, &typography),
        slider: slider(c, g, &colors),
        progress_bar: progress_bar(g, &colors, &layout),
        splitter: splitter(c, g),
        separator: separator(c, &layout),
        spinner: spinner(c, g),
        menu: menu(c, g, &colors, &typography, fonts.menu, units),
        menu_bar: menu_bar(c),
        tab: tab(c, g, &colors, &typography),
        sidebar: sidebar(c, &colors, &typography, &layout),
        toolbar: toolbar(c, g, &typography, &layout),
        status_bar: status_bar(c, &typography, &layout),
        popover: popover(c, &colors, &typography, &layout),
        switch: switch(c, g, &colors, &typography),
        dialog: dialog(c, g, &colors, &typography, &layout, motion, units),
        combo_box: combo_box(c, g, &colors, &typography, &layout, ctx.direction),
        segmented_control: segmented_control(c, g, &colors, &typography, &layout),
        card: card(c, g, &colors),
        expander: expander(c, g, &typography, &layout),
        colors,
        typography,
        layout,
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
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
///
/// The active palette `palette`, whose colours `c` holds already, states
/// the roles its slots name (spec §7, "The roles a palette states"): the
/// view's, the header's, the sidebar's, the card's, the dialog's, the
/// popover's and the tooltip's surfaces with its one text colour, the
/// window's divider, and the four status colours' origin (`Custom`).
fn roles(c: &SchemeColors, g: &Generation, palette: Option<&Palette>) -> ColorRoles {
    let origin = if palette.is_some() {
        RoleOrigin::Custom
    } else {
        RoleOrigin::SystemColor
    };
    let status = |fill| StatusColors {
        fill,
        foreground: c.status_foreground,
        standalone: None,
        origin,
    };
    let unstated = SurfaceColors::default();
    let surface = |background: fn(&Palette) -> Color| {
        palette.map_or(unstated, |p| {
            SurfaceColors::new(Some(background(p)), Some(p.foreground), None, None, None)
        })
    };
    ColorRoles {
        window: SurfaceColors::new(
            Some(c.background),
            Some(c.foreground),
            Some(c.muted),
            Some(c.border),
            palette.map(|p| p.divider),
        ),
        view: SurfaceColors::new(
            Some(c.surface),
            palette.map(|p| p.foreground),
            None,
            None,
            None,
        ),
        header: surface(|p| p.header_bg),
        sidebar: surface(|p| p.sidebar_bg),
        card: surface(|p| p.surface_bg),
        dialog: surface(|p| p.surface_bg),
        popover: surface(|p| p.surface_bg),
        tooltip: surface(|p| p.tooltip_bg),
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

/// The colours of the system's entry `os` laid over the preset's colours
/// `c`, each where `os` states it, as the 0.7.2 reader writes its reads over
/// the preset (spec §8, "The macOS part"; 0.7.2's `macos::defaults_of` and
/// `apply_per_widget`): the window's background, text, muted text and
/// border (`labelColor` also the status roles' text and the unfocused
/// selection's text, as the 0.7.2 reader copies it); the view's background;
/// the selection's background, text (also the accent's text, one read that
/// 0.7.2 writes to both) and unfocused background; the text selection; the
/// accent; the focus ring; the link; the four status colours; the disabled
/// text; the placeholder; the caret; the alternate row; the table header's
/// text; the grid. A stated unfocused selection text replaces the copy.
fn lay_system_over(c: &mut SchemeColors, os: &OsColors) {
    fn set(slot: &mut Color, value: Option<Color>) {
        if let Some(value) = value {
            *slot = value;
        }
    }
    let surfaces = os.surfaces.as_ref();
    if let Some(window) = surfaces.and_then(|s| s.window) {
        set(&mut c.background, window.background);
        set(&mut c.foreground, window.foreground);
        set(&mut c.status_foreground, window.foreground);
        set(&mut c.selection_inactive_foreground, window.foreground);
        set(&mut c.muted, window.foreground_muted);
        set(&mut c.border, window.border);
    }
    if let Some(view) = surfaces.and_then(|s| s.view) {
        set(&mut c.surface, view.background);
    }
    if let Some(selection) = os.selection {
        set(&mut c.selection_background, selection.background);
        set(&mut c.selection_foreground, selection.foreground);
        set(&mut c.accent_foreground, selection.foreground);
        set(
            &mut c.selection_inactive_background,
            selection.unfocused_background,
        );
        set(
            &mut c.selection_inactive_foreground,
            selection.unfocused_foreground,
        );
    }
    if let Some(text_selection) = os.text_selection {
        set(&mut c.text_selection_background, text_selection.background);
        set(&mut c.text_selection_foreground, text_selection.foreground);
    }
    if os.accent.is_some_and(|accent| accent != c.accent) {
        drop_accent_measurements(c);
    }
    set(&mut c.accent, os.accent);
    set(&mut c.focus_ring, os.focus_ring);
    set(&mut c.link, os.link);
    if let Some([error, warning, success, info]) = os.status {
        set(&mut c.error, error);
        set(&mut c.warning, warning);
        set(&mut c.success, success);
        set(&mut c.info, info);
    }
    set(&mut c.disabled_text, os.disabled_text);
    set(&mut c.placeholder, os.placeholder);
    set(&mut c.caret, os.caret);
    set(&mut c.list_alternate, os.alternate_row_background);
    set(&mut c.list_header_foreground, os.list_header_foreground);
    set(&mut c.list_grid, os.grid);
}

/// Lays the palette `p` over `c`: each slot whose AppKit colour has its name
/// replaces that colour and the widget colours the preset takes from it, as
/// the module documentation lists; the caret is the palette's; the accent's
/// slots are laid as given ([`lay_accent_over`]) with the palette's selection
/// over the accent's, which the text selection and the hovered menu item's
/// text take, unless an explicit accent (`explicit_accent`) replaces them.
/// The slots that state only their roles are read by [`roles`].
fn lay_palette_over(c: &mut SchemeColors, p: &Palette, explicit_accent: bool) {
    c.background = p.window_bg;
    c.surface = p.view_bg;
    c.list_background = p.view_bg;
    c.sidebar_background = p.sidebar_bg;
    c.tooltip_background = p.tooltip_bg;
    c.foreground = p.foreground;
    c.sidebar_foreground = Some(p.foreground);
    c.tooltip_foreground = p.foreground;
    c.muted = p.foreground_muted;
    c.border = p.border;
    c.link = p.link;
    c.caret = p.caret;
    c.error = p.error;
    c.warning = p.warning;
    c.success = p.success;
    c.info = p.info;
    if explicit_accent {
        return;
    }
    lay_accent_over(c, p.accent, false);
    c.accent_foreground = p.accent_fg;
    c.selection_background = p.selection_bg;
    c.selection_foreground = p.selection_fg;
    c.text_selection_background = p.selection_bg;
    c.text_selection_foreground = p.selection_fg;
    c.menu_hover_foreground = p.selection_fg;
}

/// Lays the accent `accent` over `c` as given (spec §6, step 2): the
/// accent's fill, the selection's background, the text selection, which
/// takes the selection's two colours, the focus ring at
/// [`FOCUS_RING_ALPHA`], and, where `caret`, the caret. The text on the
/// accent and on the selection keeps the value of the layers below. An
/// accent other than the preset's drops the pressed colours measured on the
/// preset's ([`drop_accent_measurements`]).
fn lay_accent_over(c: &mut SchemeColors, accent: Color, caret: bool) {
    if accent != c.accent {
        drop_accent_measurements(c);
    }
    c.accent = accent;
    c.selection_background = accent;
    c.text_selection_background = accent;
    c.text_selection_foreground = c.selection_foreground;
    c.focus_ring = accent.with_alpha(f32::from(FOCUS_RING_ALPHA) / f32::from(u8::MAX));
    if caret {
        c.caret = accent;
    }
}

/// Drops the pressed colours the probe measured on the default accent (the
/// checked box's, the slider's filled track's, the source-list row's): under
/// the default accent the first two equal, within one level, the pressed
/// checked button's mix of the accent by [`BUTTON_CHECKED_PRESSED_MIX`]
/// (#006be1 light; #1f8aff and #1e8aff dark), the row's does not (#0076fb /
/// #1c96ff); whether any of them follows that mix, or another function of
/// the accent, under another accent is not measured, so they are dropped,
/// not derived: the checked box and the slider keep their rest fill pressed
/// and the row takes the selection's. Once dropped, they stay dropped: a
/// later layer that lays the preset's accent back (a palette's or the
/// explicit accent over a system accent) compares it with the accent then in
/// force, not the preset's, and does not restore them, which leaves the
/// unmeasured rest fill rather than a colour measured for another accent.
fn drop_accent_measurements(c: &mut SchemeColors) {
    c.checkbox_checked_pressed = None;
    c.slider_fill_pressed = None;
    c.sidebar_pressed = None;
}

/// The fonts the system states for widgets of their own: the menu's
/// (`menu_font`), the tooltip's (`tooltip_font`) and the window title's
/// (`title_bar_font`).
#[derive(Clone, Copy, Default)]
struct SystemFonts<'a> {
    /// The menu's.
    menu: Option<&'a OsFont>,
    /// The tooltip's.
    tooltip: Option<&'a OsFont>,
    /// The window title's.
    title_bar: Option<&'a OsFont>,
}

impl<'a> SystemFonts<'a> {
    /// The fonts `system` states.
    fn of(system: Option<&'a SystemOverrides>) -> Self {
        let Some(s) = system else {
            return Self::default();
        };
        Self {
            menu: s.menu_font.as_ref(),
            tooltip: s.tooltip_font.as_ref(),
            title_bar: s.title_bar_font.as_ref(),
        }
    }
}

/// `base` with the system's font `os` laid over it, where given: each of
/// its family, size, weight and style that `os` states replaces `base`'s,
/// as 0.7.2 merges a reader's font over the preset's (`Theme::merge`).
fn laid_over(base: TextStyle, os: Option<&OsFont>, units: FontUnits) -> TextStyle {
    let mut font = base;
    let Some(os) = os else {
        return font;
    };
    if let Some(family) = &os.family {
        font.family = family.clone();
    }
    if let Some(size) = os.size {
        font.size_px = font_px(size, units);
        font.defined_size = Some(size);
    }
    if let Some(weight) = os.weight {
        font.weight = weight;
    }
    if let Some(style) = os.style {
        font.style = style;
    }
    font
}

/// `text` at the size `size`, where given, and the weight `weight`: a
/// widget's font that states its size or weight and takes its other fields
/// from the UI font (0.7.2's `resolve_font_inheritance`).
fn resized(
    mut text: TextStyle,
    size: Option<FontSize>,
    weight: u16,
    units: FontUnits,
) -> TextStyle {
    if let Some(size) = size {
        text.size_px = font_px(size, units);
        text.defined_size = Some(size);
    }
    text.weight = weight;
    text
}

/// How a font size becomes logical pixels: the font DPI and the text scale
/// (spec §6, resolution step 2).
#[derive(Clone, Copy)]
struct FontUnits {
    /// The font DPI: the system's (`SystemOverrides::font_dpi`), else
    /// [`FONT_DPI`].
    dpi: f32,
    /// The text scale, `effective.text_scale`.
    text_scale: f32,
}

/// A font size in logical pixels: converted at the font DPI, times the text
/// scale, not rounded (spec §6, resolution step 2).
fn font_px(size: FontSize, units: FontUnits) -> f32 {
    linear_font_px(size, units.dpi, units.text_scale, None)
}

/// The line height of a style of `size` at the multiplier `multiplier`, in
/// logical pixels: the multiplier times the size, in its unit, converted at
/// the font DPI (0.7.2's `resolve_text_scale_entry`,
/// `src/resolve/inheritance.rs`, then its conversion in `validate`; spec §4,
/// `Typography`), times the text scale.
fn line_height_px(size: FontSize, multiplier: f32, units: FontUnits) -> f32 {
    let size = match size {
        FontSize::Pt(v) => FontSize::Pt(v * multiplier),
        FontSize::Px(v) => FontSize::Px(v * multiplier),
    };
    font_px(size, units)
}

/// A font as 0.7.2 resolves it: its family, size, weight and style stated,
/// the style normal, which no macOS TOML states (`src/resolve/validate_helpers.rs`,
/// `require_font`); and `color`, where given.
fn font(
    family: &str,
    size: FontSize,
    weight: u16,
    color: Option<Color>,
    units: FontUnits,
) -> TextStyle {
    let mut font = TextStyle {
        family: Arc::from(family),
        size_px: font_px(size, units),
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
fn text_scale_entry(g: &Generation, size_pt: f32, weight: u16, units: FontUnits) -> TextStyle {
    let size = FontSize::Pt(size_pt);
    TextStyle {
        size_px: font_px(size, units),
        defined_size: Some(size),
        weight,
        line_height: line_height_px(size, g.line_height, units),
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
/// display (`display`). Sizes convert at the font DPI and scale by the text
/// scale (`units`); each keeps its unscaled size as its `defined_size`. The
/// system's UI font (`ui_font`, 0.7.2's `defaults.font` from
/// `NSFont.systemFontOfSize`) is laid over the body, and so its size gives
/// the body's line height; the system's monospaced font (`monospace_font`,
/// `defaults.mono_font`) over the monospace.
fn typography(
    g: &Generation,
    colors: &ColorRoles,
    system: Option<&SystemOverrides>,
    units: FontUnits,
) -> Typography {
    let ui = system.and_then(|s| s.ui_font.as_ref());
    let mono = system.and_then(|s| s.monospace_font.as_ref());
    let body_size = FontSize::Pt(g.font_size_pt);
    let text = colors.window.foreground;
    let mut body = laid_over(
        font(g.font_family, body_size, g.font_weight, text, units),
        ui,
        units,
    );
    let line_size = ui.and_then(|f| f.size).unwrap_or(body_size);
    body.line_height = line_height_px(line_size, g.line_height, units);
    body.stated |= TextStated::LINE_HEIGHT;
    // The monospace font's line height, which no source states: the multiplier times its size
    // (spec §4, `Typography`: `body` and `monospace` follow a slot's rule).
    let mono_size = FontSize::Pt(g.mono_size_pt);
    let mut monospace = laid_over(
        font(g.mono_family, mono_size, g.mono_weight, text, units),
        mono,
        units,
    );
    monospace.line_height = line_height_px(
        mono.and_then(|f| f.size).unwrap_or(mono_size),
        g.line_height,
        units,
    );
    monospace.stated |= TextStated::LINE_HEIGHT;
    let entry = |size, weight| Some(text_scale_entry(g, size, weight, units));
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
        monospace,
        line_height_multiplier: g.line_height,
        fallbacks: Arc::from([]),
        emoji_family: None,
        rendering: TextRendering::default(),
        font_dpi: units.dpi,
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

/// The push button: its fill in the default line where the generation draws
/// one ([`Generation::button_outlined`]; 0.7.2: `button.border` is
/// `defaults.border` where unstated) and radius, with its padding and least
/// size; hovered as at rest, the measured pressed fill and the disabled fill;
/// checked (`macos-tahoe` only), the accent in force mixed with
/// [`SchemeColors::button_checked_mix_with`] by [`BUTTON_CHECKED_MIX`], with
/// its own label colour, pressed by [`BUTTON_CHECKED_PRESSED_MIX`], disabled
/// by [`BUTTON_CHECKED_DISABLED_MIX`] at [`BUTTON_CHECKED_DISABLED_ALPHA`]
/// with its own disabled label colour; the primary variant in the accent
/// with its text (0.7.2: `primary_background` and `primary_text_color` are
/// `defaults.accent_color` and `accent_text_color` where unstated). Its text
/// keeps its colour hovered, takes
/// [`SchemeColors::button_pressed_foreground`] pressed where stated, and is
/// the disabled text when disabled (`defaults.disabled_text_color`).
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
    let stroke = if g.button_outlined {
        outline(layout, c.border)
    } else {
        PartProp::Stroke(Stroke::None)
    };
    // Hovered, the bezel as at rest: hover draws nothing on a push button, every hovered push
    // button differs from its normal capture in 0 pixels (`docs/facts/macos/state-models.toml` on
    // `v0.8.0-rc1`, `hover` not an axis; os-probe runs 41006842699, macOS 15, and 40969040283,
    // macOS 26, button-pass/probe.toml `…hovered.regular-*.diff_from_normal`). Pressed, the
    // measured fill; where no stroke is drawn, none pressed either (layers-key-push.toml
    // `key.push.<appearance>.pressed.regular-plain.stroke` absent, run 41006842699, macOS 26);
    // where one is, the edge as at rest (macOS 15: the pressed bezel's outermost pixels are the
    // rest ones byte for byte, os-probe run 45198750893 `*.pressed.push` against
    // `*.normal.push`).
    let pressed = c
        .button_pressed
        .map(|pressed| [fill(pressed), stroke.clone()]);
    let body = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.button_background),
            stroke.clone(),
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
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.button_background), stroke],
        )
        .rules(
            pressed
                .as_ref()
                .map(|_| RuleSelector::on(WidgetState::PRESSED)),
            pressed.into_iter().flatten(),
        )
        .rules(
            c.button_checked_mix_with
                .map(|_| RuleSelector::on(WidgetState::CHECKED)),
            c.button_checked_mix_with
                .map(|with| fill(roles.accent.fill.mix_srgb(with, BUTTON_CHECKED_MIX))),
        )
        .rules(
            c.button_checked_mix_with
                .map(|_| RuleSelector::on(WidgetState::CHECKED | WidgetState::PRESSED)),
            c.button_checked_mix_with
                .map(|with| fill(roles.accent.fill.mix_srgb(with, BUTTON_CHECKED_PRESSED_MIX))),
        )
        .rule(primary(), [fill(roles.accent.fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.button_disabled)],
        )
        .rules(
            c.button_checked_mix_with
                .map(|_| RuleSelector::on(WidgetState::CHECKED | WidgetState::DISABLED)),
            c.button_checked_mix_with.map(|with| {
                fill(
                    roles
                        .accent
                        .fill
                        .mix_srgb(with, BUTTON_CHECKED_DISABLED_MIX)
                        .with_alpha(BUTTON_CHECKED_DISABLED_ALPHA),
                )
            }),
        )
        .build();
    // Hovered, the label as at rest (the measurement above); pressed, the measured label where
    // the generation states one, else as at rest.
    let label = RuleBuilder::new(label(&typography.body, Some(c.button_foreground)))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.button_foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(
                c.button_pressed_foreground.unwrap_or(c.button_foreground),
            )],
        )
        .rules(
            c.button_checked_foreground
                .map(|_| RuleSelector::on(WidgetState::CHECKED)),
            c.button_checked_foreground.map(TextProp::Color),
        )
        .rule(primary(), [TextProp::Color(roles.accent.foreground)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .rules(
            c.button_checked_disabled_foreground
                .map(|_| RuleSelector::on(WidgetState::CHECKED | WidgetState::DISABLED)),
            c.button_checked_disabled_foreground.map(TextProp::Color),
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
/// shadow, of an exact size; hovered and pressed, the fills `states` gives;
/// checked, the accent (0.7.2: `checkbox.checked_background` is
/// `defaults.accent_color` where unstated) in the default line (0.7.2:
/// `checkbox.border.color` is `defaults.border.color` where unstated).
fn indicator(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    layout: &LayoutMetrics,
    states: IndicatorFills,
) -> RuleBuilder<PartStyle, PartProp> {
    let size = Some(g.checkbox_indicator_width);
    // Hovered, the edge as at rest (os-probe run 45198750893, macOS 15 and 26,
    // `*.hovered.checkbox_off` and `…checkbox_on` differ from the normal capture in 0 pixels);
    // pressed, the measured edge where the generation states one, else as at rest (macOS 26:
    // every pressed edge pixel is the fill, `*.pressed.checkbox_*` `edge_*`).
    let rest_edge = outline(layout, c.checkbox_unchecked_border);
    let pressed_edge = c
        .checkbox_pressed_border
        .map_or_else(|| rest_edge.clone(), |color| outline(layout, color));
    RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.checkbox_unchecked),
            rest_edge.clone(),
            PartProp::MinSize([size; 2]),
            PartProp::MaxSize([size; 2]),
        ])
        .base(no_shadow(g.checkbox_shadow_enabled))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            states.hover.map(fill).into_iter().chain([rest_edge]),
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            states.pressed.map(fill).into_iter().chain([pressed_edge]),
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill), outline(layout, c.border)],
        )
        .rules(
            states
                .checked_pressed
                .map(|_| RuleSelector::on(WidgetState::CHECKED | WidgetState::PRESSED)),
            states.checked_pressed.map(fill),
        )
}

/// The fills [`indicator`] states for the hovered, the pressed and the
/// checked pressed box or circle; `None` where it states none.
#[derive(Clone, Copy)]
struct IndicatorFills {
    hover: Option<Color>,
    pressed: Option<Color>,
    checked_pressed: Option<Color>,
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
    // Hovered and pressed, the mark as at rest (os-probe run 45198750893, macOS 15 and 26:
    // `*.hovered.checkbox_on` differs from the normal capture in 0 pixels; `*.pressed.checkbox_on`
    // `ink` the rest one, macOS 15's dark #dfebfc against #dde8f9 at rest, the core of a mark over
    // a fill that the press changes).
    let mark = IconProp::Color(roles.accent.foreground);
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base([mark.clone()])
        .rules(
            [WidgetState::HOVER, WidgetState::PRESSED].map(RuleSelector::on),
            [mark],
        )
        .build();
    // Hovered, the box's fill as at rest (`*.hovered.checkbox_off`, `…checkbox_on` differ in 0
    // pixels), where 0.7.2 macos-monterey.toml states an uncited hover fill (#e0e0e5 / #444446);
    // pressed, the measured fills.
    let states = IndicatorFills {
        hover: Some(c.checkbox_unchecked),
        pressed: Some(c.checkbox_pressed),
        checked_pressed: c.checkbox_checked_pressed,
    };
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator(c, g, roles, layout, states)
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
    // The radio's hovered fill is 0.7.2's (`checkbox.hover_background`, which serves both); its
    // states were not measured.
    let states = IndicatorFills {
        hover: c.checkbox_hover,
        pressed: None,
        checked_pressed: None,
    };
    RadioWidget::new(
        WidgetOrigin::Native,
        indicator(c, g, roles, layout, states).build(),
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
    // A selected row in a view without the focus is drawn unemphasized: in a window whose table is
    // not the first responder the row is filled with `unemphasizedSelectedContentBackgroundColor`
    // and its label is `labelColor`, as outside the selection, #dcdcdc / #464646 under #000000d8 /
    // #ffffffd8, measured on macOS 15 and 26 (platform-facts.md (§2.1.3),
    // `selection_inactive_background` and `selection_inactive_text_color`); stated for
    // `SELECTED | UNFOCUSED_VIEW` (spec §3, §5), after the selection's rules.
    let unfocused_selected = WidgetState::SELECTED.union(WidgetState::UNFOCUSED_VIEW);
    // Hovered, transparent: the hovered row differs from its normal capture in 0 pixels; pressed,
    // the selection's, as the mouse-down selects the row (os-probe run 45198750893, macOS 15 and
    // 26, `*.hovered.table_row`, `*.pressed.table_row` `fill` #0064e1 / #0059d1).
    let highlight = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(Color::TRANSPARENT)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(roles.selection.background)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .rules(
            roles
                .selection
                .unfocused_background
                .map(|_| RuleSelector::on(unfocused_selected)),
            roles.selection.unfocused_background.map(fill),
        )
        .build();
    let item_label = RuleBuilder::new(label(&typography.body, roles.window.foreground))
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(roles.selection.foreground)],
        )
        .rules(
            roles
                .selection
                .unfocused_foreground
                .map(|_| RuleSelector::on(unfocused_selected)),
            roles.selection.unfocused_foreground.map(TextProp::Color),
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
/// `macos-monterey`'s pressed one. The groove is `groove_width` thick and
/// the bar overlays the content where `overlay` (the system's scroller
/// width and style, else the preset's).
fn scrollbar(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    groove_width: f32,
    overlay: bool,
) -> ScrollbarWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base([fill(c.scrollbar_track)]),
        groove_width,
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
        overlay,
    )
}

/// The tooltip: its fill in the default line and radius (0.7.2:
/// `tooltip.border` is `defaults.border` where unstated) with
/// `macos-monterey`'s padding and the preset's greatest width; its text in
/// the UI font's family, weight and style at the tooltip's size (0.7.2:
/// `tooltip.font`'s unstated fields are `defaults.font`'s) in its colour,
/// with the system's tooltip font `os` laid over it.
fn tooltip(
    c: &SchemeColors,
    g: &Generation,
    layout: &LayoutMetrics,
    typography: &Typography,
    os: Option<&OsFont>,
    units: FontUnits,
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
    let body = &typography.body;
    TooltipWidget::new(
        WidgetOrigin::Native,
        panel,
        laid_over(
            resized(
                label(body, Some(c.tooltip_foreground)),
                Some(FontSize::Pt(g.tooltip_font_size_pt)),
                body.weight,
                units,
            ),
            os,
            units,
        ),
    )
}

/// The window's chrome, in the active window or, where `ctx.window_active`
/// is false, in an inactive one: the frame in the window's background in the
/// default line with the large radius (0.7.2: `window.background_color` is
/// `defaults.background_color` and `window.border` `defaults.border` with
/// `corner_radius_lg` where unstated); the title bar's fill, inactive where
/// the preset states one (else the active one, 0.7.2's
/// `resolve_widget_to_widget`); the title `title`, active or not (0.7.2:
/// `inactive_title_bar_text_color` is `title_bar_font.color` where
/// unstated): the UI font at the title bar's weight (0.7.2:
/// `window.title_bar_font` is `defaults.font` where unstated) in the
/// system's title bar text, else the window's, with the system's title-bar
/// font laid over it. A maximized or full-screen window has no data of its
/// own and takes the normal window's styles; no caption button is stated.
fn window(
    c: &SchemeColors,
    title: TextStyle,
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
        title,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
    )
}

/// A text field, single-line (`input`, `single_line`) or multi-line
/// (`text_area`): the field's fill in the default line (0.7.2: `input.border`
/// is `defaults.border` where unstated, and `text_area.border`'s colour,
/// radius, width and shadow are `input.border`'s) at `macos-tahoe`'s own
/// radius, else the default one, with no shadow; its text in its colour, the
/// placeholder in its own; disabled, `macos-monterey`'s disabled fill and the
/// disabled text (0.7.2: `input.disabled_text_color` is
/// `defaults.disabled_text_color` and `disabled_opacity`
/// `defaults.disabled_opacity` where unstated). The single-line field has
/// its own padding, least height and focus ring, the ring the measured
/// NSTextField draws once it is focused, by the pointer too
/// ([`FocusTrigger::Always`]); the text area keeps the global ring.
fn input(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    single_line: bool,
) -> InputWidget {
    let padding =
        single_line.then(|| Edges::symmetric(g.input_padding_vertical, g.input_padding_horizontal));
    let min_height = single_line.then_some(g.input_min_height);
    // The ring of the field's mouse-down, in the focus ring's colour: measured L #0066f57f,
    // D #1aabff7f on both generations, within 2 levels of keyboardFocusIndicatorColor (os-probe
    // run 45231684209, macOS 15 and 26, `*.pressed.text_field` `around_*`).
    let focus_ring = single_line.then(|| {
        FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(g.input_focus_ring_width, roles.focus_ring),
            Stroke::None,
            g.input_focus_ring_offset,
            None,
            Some(FocusTrigger::Always),
        )
    });
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
        .base(focus_ring.map(|ring| PartProp::Focus(Some(Arc::new(ring)))))
        // Hovered, the edge as at rest: the hovered text field differs from its normal capture
        // in 0 pixels (os-probe run 45198750893, macOS 15 and 26, `*.hovered.text_field`).
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [outline(layout, c.border)],
        )
        // Pressed, the fill and the edge as at rest under the focus ring the press brings (os-probe
        // run 45231684209, macOS 15 and 26, `*.pressed.text_field` against `*.normal.text_field`
        // `around_*`, `fill`).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.input_background), outline(layout, c.border)],
        )
        .rules(
            c.input_disabled
                .map(|_| RuleSelector::on(WidgetState::DISABLED)),
            c.input_disabled.map(fill),
        )
        .build();
    // Hovered and pressed, the text as at rest (`*.hovered.text_field` differs in 0 pixels; the
    // pressed field's label within 1 level of the rest one, run 45231684209).
    let text = RuleBuilder::new(label(&typography.body, Some(c.input_foreground)))
        .rules(
            [WidgetState::HOVER, WidgetState::PRESSED].map(RuleSelector::on),
            [TextProp::Color(c.input_foreground)],
        )
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

/// The padding of `vertical` above and below and `horizontal` on each side,
/// where the preset states both.
fn symmetric_padding(vertical: Option<f32>, horizontal: Option<f32>) -> Option<PartProp> {
    vertical
        .zip(horizontal)
        .map(|(v, h)| PartProp::Padding(Edges::symmetric(v, h)))
}

/// `builder` with the exact `size` in both dimensions.
fn square(
    builder: RuleBuilder<PartStyle, PartProp>,
    size: f32,
) -> RuleBuilder<PartStyle, PartProp> {
    builder.base([
        PartProp::MinSize([Some(size); 2]),
        PartProp::MaxSize([Some(size); 2]),
    ])
}

/// The slider: a groove of an exact thickness in its colour, the value's
/// part in the accent (0.7.2: `slider.fill_color` is `defaults.accent_color`
/// where unstated), a knob of an exact diameter in its colour where the
/// scheme states one, else the view's background (0.7.2:
/// `slider.thumb_color` is `defaults.surface_color` where unstated), with
/// `macos-monterey`'s hovered knob; disabled, the value's part in its
/// colour and `macos-monterey`'s groove and knob.
fn slider(c: &SchemeColors, g: &Generation, roles: &ColorRoles) -> SliderWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base([fill(c.slider_track)]),
        g.slider_track_height,
    )
    .rules(
        c.slider_disabled_track
            .map(|_| RuleSelector::on(WidgetState::DISABLED)),
        c.slider_disabled_track.map(fill),
    )
    .build();
    // Pressed, the filled part as measured where the generation states it (`macos-tahoe`).
    let active_track = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.accent.fill)])
        .rules(
            c.slider_fill_pressed
                .map(|_| RuleSelector::on(WidgetState::PRESSED)),
            c.slider_fill_pressed.map(fill),
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.slider_disabled_fill)],
        )
        .build();
    // Hovered, the knob as at rest (macOS 15: os-probe run 45198750893, macOS 26: run 45217592058,
    // `*.hovered.slider_knob` differs from the normal capture in 0 pixels), where 0.7.2
    // macos-monterey.toml states an uncited hover fill (#e0e0e0); pressed, the measured fill, else
    // as at rest.
    let rest_thumb = c.slider_thumb.or(roles.view.background);
    let pressed_thumb = c.slider_thumb_pressed.or(rest_thumb);
    let thumb = square(
        RuleBuilder::new(PartStyle::default()).base(rest_thumb.map(fill)),
        g.slider_thumb_diameter,
    )
    .rules(
        rest_thumb.map(|_| RuleSelector::on(WidgetState::HOVER).except(WidgetState::DISABLED)),
        rest_thumb.map(fill),
    )
    .rules(
        pressed_thumb.map(|_| RuleSelector::on(WidgetState::PRESSED).except(WidgetState::DISABLED)),
        pressed_thumb.map(fill),
    )
    .rules(
        c.slider_disabled_thumb
            .map(|_| RuleSelector::on(WidgetState::DISABLED)),
        c.slider_disabled_thumb.map(fill),
    )
    .build();
    SliderWidget::new(
        WidgetOrigin::Native,
        track,
        active_track,
        thumb,
        RuleBuilder::new(PartStyle::default()).build(),
        g.slider_tick_length,
        // 0.7.2: `slider.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The progress bar: a groove in the window's muted text (0.7.2:
/// `progress_bar.track_color` is `defaults.muted_color` where unstated) in
/// the default line and radius (0.7.2: `progress_bar.border` is
/// `defaults.border` where unstated), no shadow, unpadded, of an exact height
/// and a least length; the progress in the accent (`fill_color`:
/// `defaults.accent_color` where unstated).
fn progress_bar(g: &Generation, roles: &ColorRoles, layout: &LayoutMetrics) -> ProgressBarWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base(roles.window.foreground_muted.map(fill))
        .base(roles.window.border.map(|border| outline(layout, border)))
        .base([
            corners(layout),
            PartProp::Padding(Edges::symmetric(
                g.progress_bar_padding_vertical,
                g.progress_bar_padding_horizontal,
            )),
            PartProp::MinSize([
                Some(g.progress_bar_min_width),
                Some(g.progress_bar_track_height),
            ]),
            PartProp::MaxSize([None, Some(g.progress_bar_track_height)]),
        ])
        .base(no_shadow(g.progress_bar_shadow_enabled))
        .build();
    let progress = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.accent.fill)])
        .build();
    ProgressBarWidget::new(WidgetOrigin::Native, track, progress, None)
}

/// The split view's divider: its handle in its colour, hovered in
/// `macos-monterey`'s hover colour, else as at rest; pressed, as at rest.
fn splitter(c: &SchemeColors, g: &Generation) -> SplitterWidget {
    // `macos-tahoe` hovered (os-probe run 45198750893, macOS 26, `*.hovered.split_divider`
    // differs from the normal capture in 0 pixels) and both pressed (run 45231684209, macOS 15
    // and 26, `*.pressed.split_divider` differs in 0 pixels) draw the divider as at rest.
    let handle = RuleBuilder::new(PartStyle::default())
        .base([fill(g.splitter_divider)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.splitter_hover.unwrap_or(g.splitter_divider))],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(g.splitter_divider)],
        )
        .build();
    SplitterWidget::new(WidgetOrigin::Native, handle, g.splitter_divider_width)
}

/// The separator: the default border colour at the default border width
/// (0.7.2: `separator.line_color` and `line_width` are
/// `defaults.border.color` and `line_width` where unstated).
fn separator(c: &SchemeColors, layout: &LayoutMetrics) -> SeparatorWidget {
    SeparatorWidget::new(
        WidgetOrigin::Native,
        c.border,
        layout.border_width,
        None,
        None,
    )
}

/// The spinner, in its colour, of its sizes.
fn spinner(c: &SchemeColors, g: &Generation) -> SpinnerWidget {
    SpinnerWidget::new(
        WidgetOrigin::Native,
        c.spinner,
        g.spinner_diameter,
        g.spinner_min_diameter,
        g.spinner_stroke_width,
        None,
    )
}

/// The menu: its panel in the window's background (0.7.2:
/// `menu.background_color` is `defaults.background_color` where unstated);
/// items of `macos-monterey`'s exact height and padding, hovered in the
/// selection (`menu.hover_background`: `defaults.selection_background` where
/// unstated); the UI font in the window's text (0.7.2: `menu.font` is
/// `defaults.font` where unstated) with the system's menu font `os` laid over
/// it, hovered in its own colour, the disabled text when disabled
/// (`defaults.disabled_text_color`); icons at their size;
/// separators in the default border colour (`menu.separator_color`:
/// `defaults.border.color` where unstated). 0.7.2 states no outline,
/// corners or shadow of the panel.
fn menu(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    os: Option<&OsFont>,
    units: FontUnits,
) -> MenuWidget {
    let panel = RuleBuilder::new(PartStyle::default())
        .base([fill(c.background)])
        .build();
    let item = RuleBuilder::new(PartStyle::default())
        .base(symmetric_padding(
            g.menu_padding_vertical,
            g.menu_padding_horizontal,
        ))
        .base(
            g.menu_row_height
                .map(|h| PartProp::MinSize([None, Some(h)])),
        )
        .base(
            g.menu_row_height
                .map(|h| PartProp::MaxSize([None, Some(h)])),
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(roles.selection.background)],
        )
        .build();
    let item_label = RuleBuilder::new(laid_over(label(&typography.body, None), os, units))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.menu_hover_foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    let icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(g.menu_icon_size)])
        .build();
    let separator = RuleBuilder::new(PartStyle::default())
        .base([fill(c.border)])
        .build();
    MenuWidget::new(
        WidgetOrigin::Native,
        panel,
        item,
        item_label,
        RuleBuilder::new(TextStyle::default()).build(),
        icon,
        RuleBuilder::new(IconStyle::default()).build(),
        separator,
        g.menu_icon_text_gap,
    )
}

/// The menu bar: the toolbar's background (0.7.2's
/// `toolbar.background_color`, `defaults.background_color` where unstated);
/// 0.7.2 states nothing of its titles.
fn menu_bar(c: &SchemeColors) -> MenuBarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base([fill(c.background)])
        .build();
    MenuBarWidget::new(
        WidgetOrigin::Native,
        bar,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        None,
    )
}

/// The tab bar: the bar and the tabs in the window's background (0.7.2:
/// `tab.bar_background` and `tab.background_color` are
/// `defaults.background_color` where unstated), a tab of least sizes with
/// `macos-monterey`'s padding and hover fill; the selected tab in the
/// window's background again (`tab.active_background`:
/// `defaults.background_color` where unstated); the UI font in the window's
/// text, which a hovered and a selected tab keep (0.7.2: `tab.font` is
/// `defaults.font`, `hover_text_color` `tab.font.color` and
/// `active_text_color` `defaults.text_color` where unstated, so no rule).
/// 0.7.2 states no outline, corners, shadow or indicator.
fn tab(c: &SchemeColors, g: &Generation, roles: &ColorRoles, typography: &Typography) -> TabWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base([fill(c.background)])
        .build();
    let tab = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.background),
            PartProp::MinSize([Some(g.tab_min_width), Some(g.tab_min_height)]),
        ])
        .base(symmetric_padding(
            g.tab_padding_vertical,
            g.tab_padding_horizontal,
        ))
        // Hovered, the fill as at rest (the label's measurement below), where 0.7.2
        // macos-monterey.toml states an uncited hover fill (#e8e8ed / #3a3a3c); an unselected tab
        // pressed, the measured fill.
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.background)])
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(WidgetState::SELECTED),
            [fill(c.tab_pressed)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            roles.window.background.map(fill),
        )
        .build();
    // Hovered and pressed, the label as at rest: the hovered tab differs from its normal capture
    // in 0 pixels (os-probe run 45198750893, macOS 15 and 26, `*.hovered.tab`); pressed, labelColor
    // (macOS 15: #242424 over the opaque pressed fill #f0f0f0, which labelColor over it gives;
    // macOS 26: #000000d8 / #ffffffd8, `*.pressed.tab` `label`, the reads of os-probe run
    // 45208260830).
    let rest_label = label(&typography.body, None);
    let rest_color = rest_label
        .stated
        .contains(TextStated::COLOR)
        .then_some(rest_label.color);
    TabWidget::new(
        WidgetOrigin::Native,
        bar,
        tab,
        RuleBuilder::new(rest_label)
            .rules(
                rest_color
                    .into_iter()
                    .flat_map(|_| [WidgetState::HOVER, WidgetState::PRESSED].map(RuleSelector::on)),
                rest_color.map(TextProp::Color),
            )
            .build(),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        None,
        None,
        None,
    )
}

/// The sidebar: its pane in its own background in the default border colour
/// and width (0.7.2's partial border inheritance: colour and width only); a
/// selected item in the selection's colours (0.7.2:
/// `sidebar.selection_background` and `selection_text_color` are the
/// selection's where unstated); the UI font (0.7.2: `sidebar.font` is
/// `defaults.font` where unstated) in `macos-monterey`'s own colour, which its
/// TOML states as labelColor's constant and so keeps under the system's
/// colours, else in the window's text. No hover fill (0.7.2:
/// `sidebar.hover_background` unset is the pane's fill, "no tint"); a
/// pressed item in its measured fill and the selection's text.
fn sidebar(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SidebarWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(c.sidebar_background), outline(layout, c.border)])
        .build();
    // Hovered, transparent: the hovered source-list row differs from its normal capture in 0 pixels
    // (os-probe run 45198750893, macOS 15 and 26, `*.hovered.source_list_row`); pressed, the
    // generation's measured fill, else the selection's (`*.pressed.source_list_row` `fill`).
    let item = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(Color::TRANSPARENT)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(
                c.sidebar_pressed.unwrap_or(roles.selection.background),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    // Hovered, the label as at rest (the same capture); pressed, the selection's text, white over
    // the pressed fill (`*.pressed.source_list_row` `label`, the reads of os-probe run
    // 45208260830).
    let rest_label = label(&typography.body, c.sidebar_foreground);
    let (selectors, color) = rest_color_rules(&rest_label, [WidgetState::HOVER]);
    let item_label = RuleBuilder::new(rest_label)
        .rules(selectors, color)
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(roles.selection.foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(roles.selection.foreground)],
        )
        .build();
    SidebarWidget::new(
        WidgetOrigin::Native,
        container,
        item,
        item_label,
        RuleBuilder::new(IconStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        None,
    )
}

/// The toolbar: the window's background (0.7.2: `toolbar.background_color`
/// is `defaults.background_color` where unstated) in the default line and
/// radius (0.7.2: `toolbar.border` is `defaults.border` where unstated) with
/// `macos-monterey`'s padding, least height and item gap; its buttons'
/// labels in the UI font (`toolbar.font`: `defaults.font` where unstated),
/// their icons at the toolbar's size (`toolbar.icon_size`:
/// `defaults.icon_sizes.toolbar` where unstated). 0.7.2 states no button
/// fill.
fn toolbar(
    c: &SchemeColors,
    g: &Generation,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ToolbarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.background),
            outline(layout, c.border),
            corners(layout),
        ])
        .base(symmetric_padding(
            g.toolbar_padding_vertical,
            g.toolbar_padding_horizontal,
        ))
        .base(
            g.toolbar_bar_height
                .map(|h| PartProp::MinSize([None, Some(h)])),
        )
        .build();
    let button_icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(g.icon_toolbar)])
        .build();
    // Hovered, the label as at rest where the generation measured it so; pressed, the measured
    // label.
    let rest_label = label(&typography.body, None);
    let (hover, hover_color) = rest_color_rules(&rest_label, [WidgetState::HOVER]);
    let button_label = RuleBuilder::new(rest_label)
        .rules(
            hover.into_iter().filter(|_| g.toolbar_label_hover_as_rest),
            hover_color,
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(c.toolbar_label_pressed)],
        )
        .build();
    ToolbarWidget::new(
        WidgetOrigin::Native,
        bar,
        RuleBuilder::new(PartStyle::default()).build(),
        button_label,
        button_icon,
        RuleBuilder::new(PartStyle::default()).build(),
        g.toolbar_item_gap,
    )
}

/// The status bar: the window's background (0.7.2:
/// `status_bar.background_color` is `defaults.background_color` where
/// unstated) in the default border colour and width (0.7.2's partial border
/// inheritance: colour and width only); its text in the UI font
/// (`status_bar.font`: `defaults.font` where unstated).
fn status_bar(
    c: &SchemeColors,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> StatusBarWidget {
    let mut bar = PartStyle::default();
    for prop in [fill(c.background), outline(layout, c.border)] {
        bar.apply(&prop);
    }
    StatusBarWidget::new(
        WidgetOrigin::Native,
        bar,
        label(&typography.body, None),
        PartStyle::default(),
    )
}

/// The popover: the popover's background where a palette states it, else
/// the window's (0.7.2: `popover.background_color` is
/// `defaults.background_color` where unstated, `resolve_safety_nets`) in
/// the default line with the large radius (0.7.2: `popover.border` is
/// `defaults.border` with `corner_radius_lg` where unstated); its text in the
/// UI font (`popover.font`: `defaults.font` where unstated).
fn popover(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> PopoverWidget {
    let mut panel = PartStyle::default();
    for prop in [
        fill(roles.popover.background.unwrap_or(c.background)),
        outline(layout, c.border),
        PartProp::Corners(Corners::uniform(layout.corner_radius_lg)),
    ] {
        panel.apply(&prop);
    }
    PopoverWidget::new(
        WidgetOrigin::Native,
        panel,
        PartStyle::default(),
        label(&typography.body, None),
    )
}

/// The switch: a track of exact sizes and corners with no outline (0.7.2's
/// track outlines unset: "`None` draws no outline"), off in its fill,
/// `macos-monterey`'s hovered off fill, on in the accent (0.7.2:
/// `switch.checked_background` is `defaults.accent_color` where unstated),
/// whether hovered or not (`hover_checked_background` unset: the enabled
/// colour); disabled, off in its fill and on in the accent at the preset's
/// alpha; a knob of an exact diameter, on and off (0.7.2:
/// `unchecked_thumb_diameter` and `unchecked_thumb_background` are
/// `thumb_diameter` and `thumb_background` where unset), in its colour with
/// no outline, disabled in the preset's disabled colour where the scheme
/// states one.
fn switch(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
) -> SwitchWidget {
    let size = [Some(g.switch_track_width), Some(g.switch_track_height)];
    let disabled_on = roles
        .accent
        .fill
        .with_alpha(f32::from(g.switch_disabled_checked_alpha) / f32::from(u8::MAX));
    let track = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.switch_unchecked),
            PartProp::Stroke(Stroke::None),
            PartProp::Corners(Corners::uniform(g.switch_track_radius)),
            PartProp::MinSize(size),
            PartProp::MaxSize(size),
        ])
        // Hovered, the off track as at rest (os-probe run 45208260830, macOS 15, and 45217592058,
        // macOS 26, `*.hovered.switch_off` differs from the normal capture in 0 pixels), where
        // 0.7.2 macos-monterey.toml states an uncited hover fill (#c8c8cd / #4a4a4c); pressed, the
        // measured fill.
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.switch_unchecked)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.switch_pressed_unchecked)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.switch_disabled_unchecked)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED | WidgetState::CHECKED),
            [fill(disabled_on)],
        )
        .build();
    let thumb = square(
        RuleBuilder::new(PartStyle::default())
            .base([fill(c.switch_thumb), PartProp::Stroke(Stroke::None)]),
        g.switch_thumb_diameter,
    )
    // The on thumb, `switch.thumb_background`: platform-facts.md (§2.21), white, in the dark scheme
    // at an alpha (measured), and `thumb_diameter` (the generation's citation), stated for
    // `CHECKED` (spec §2, `from_state`), before the disabled rule.
    .rule(
        RuleSelector::on(WidgetState::CHECKED),
        [
            fill(c.switch_thumb),
            PartProp::MinSize([Some(g.switch_thumb_diameter); 2]),
            PartProp::MaxSize([Some(g.switch_thumb_diameter); 2]),
        ],
    )
    .rules(
        c.switch_disabled_thumb
            .map(|_| RuleSelector::on(WidgetState::DISABLED)),
        c.switch_disabled_thumb.map(fill),
    )
    .build();
    // The label: a text label in labelColor (platform-facts.md (§2.1.1) `color`), the system font
    // at its size and weight (platform-facts.md (§2.1.1) `family`, `size`, `weight`; AppKit's
    // `NSTextField(labelWithString:)` "uses the system default font"). It leads and keeps to the
    // switch: "A toggle style that displays a leading label and a trailing switch", "In macOS, the
    // style uses a minimum of horizontal space by aligning the trailing edge of the label with the
    // leading edge of the switch" (Apple, SwiftUI `ToggleStyle.switch`, developer.apple.com, read
    // 2026-11-23). Hovered, pressed, on and disabled, labelColor as at rest, 13 px, 8 px before
    // the switch (measured, os-probe run 45210835145, SwiftUI `Toggle` with `.toggleStyle(.switch)`,
    // macOS 15 and 26: `switch-label-macos-{15,26}/measure.toml` `*.hovered_toggle_*`,
    // `*.pressed_toggle_*` `label_changed_from_rest_px` 0, `*.normal.toggle_on`,
    // `*.normal.toggle_*_disabled` `same_as` / `within_3_levels_of` the 13 pt labelColor
    // references; `switch-label.toml` `[[item]] toggle_off` fitting width 33 + 8 + the switch).
    let rest_label = label(&typography.body, None);
    let (selectors, color) = rest_color_rules(
        &rest_label,
        [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
            WidgetState::DISABLED,
            WidgetState::CHECKED | WidgetState::DISABLED,
        ],
    );
    SwitchWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        RuleBuilder::new(rest_label).rules(selectors, color).build(),
        Some(LabelSide::Leading),
        Some(SWITCH_LABEL_GAP),
        Some(false),
        None,
        // 0.7.2: `switch.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The alert: its panel in the dialog's background where a palette states
/// it, else in the window's (0.7.2:
/// `dialog.background_color` is `defaults.background_color` where unstated,
/// `resolve_safety_nets`) in the default line with the large radius (0.7.2:
/// `dialog.border` is `defaults.border` with `corner_radius_lg` where
/// unstated), `macos-monterey`'s padding, its size limits; its title in the
/// UI font at its own size and weight, its body in the UI font (0.7.2:
/// `dialog.title_font` and `body_font` are `defaults.font` where unstated);
/// its icon's size, button gap and order. No entrance is stated; no motion
/// sets `DialogEntranceStyle::None` (spec §6, resolution step 2).
fn dialog(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    motion: Motion,
    units: FontUnits,
) -> DialogWidget {
    let mut panel = PartStyle::default();
    for prop in [
        fill(roles.dialog.background.unwrap_or(c.background)),
        outline(layout, c.border),
        PartProp::Corners(Corners::uniform(layout.corner_radius_lg)),
        PartProp::MinSize([Some(g.dialog_min_width), Some(g.dialog_min_height)]),
        PartProp::MaxSize([Some(g.dialog_max_width), Some(g.dialog_max_height)]),
    ]
    .into_iter()
    .chain(symmetric_padding(
        g.dialog_padding_vertical,
        g.dialog_padding_horizontal,
    )) {
        panel.apply(&prop);
    }
    let mut icon = IconStyle::default();
    icon.apply(&IconProp::Size(g.dialog_icon_size));
    DialogWidget::new(
        WidgetOrigin::Native,
        panel,
        resized(
            label(&typography.body, roles.window.foreground),
            Some(FontSize::Pt(g.alert_title_size_pt)),
            g.alert_title_weight,
            units,
        ),
        label(&typography.body, None),
        icon,
        g.dialog_button_gap,
        Some(g.dialog_button_order),
        dialog_entrance(None, motion),
    )
}

/// The pop-up button: its fill in the default line and radius (0.7.2:
/// `combo_box.border` is `defaults.border` where unstated), no shadow, of
/// least sizes, padded above, below and on the left, the side away from the
/// arrow (the left one mirrored from right to left); hovered as at rest;
/// `macos-monterey`'s disabled fill; the UI font in its colour (0.7.2:
/// `combo_box.font` is `defaults.font` where unstated), the disabled text
/// when disabled (`combo_box.disabled_text_color`:
/// `defaults.disabled_text_color` where unstated); the arrow's size.
fn combo_box(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
) -> ComboBoxWidget {
    let start = match direction {
        Direction::Ltr => EdgeFlags::LEFT,
        Direction::Rtl => EdgeFlags::RIGHT,
    };
    let stroke = if g.combo_outlined {
        outline(layout, c.border)
    } else {
        PartProp::Stroke(Stroke::None)
    };
    // Hovered and pressed, the edge as at rest: the hovered pop-up differs from its normal
    // capture in 0 pixels, the highlighted one's edge pixels are the rest ones (macOS 15), the
    // pressed one's uncovered edge is its fill (macOS 26) (os-probe runs 45198750893 and
    // 45208260830, `*.hovered.popup`, `*.highlighted.popup`, `*.pressed.popup` `edge_*`).
    // Hovered, the fill as at rest too (`*.hovered.popup` differs in 0 pixels, macOS 15 and 26),
    // where 0.7.2 macos-monterey.toml states an uncited hover fill (#e0e0e5 / #444446).
    let field = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.combo_background),
            stroke.clone(),
            corners(layout),
            PartProp::PaddingSides(
                physical_edges(
                    Edges::new(
                        g.combo_padding_top,
                        g.combo_padding_left,
                        g.combo_padding_bottom,
                        g.combo_padding_left,
                    ),
                    direction,
                ),
                EdgeFlags::TOP | EdgeFlags::BOTTOM | start,
            ),
            PartProp::MinSize([Some(g.combo_min_width), Some(g.combo_min_height)]),
        ])
        .base(no_shadow(g.combo_shadow_enabled))
        .rule(
            RuleSelector::on(WidgetState::HOVER).except(WidgetState::DISABLED),
            [fill(c.combo_background), stroke.clone()],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(WidgetState::DISABLED),
            [fill(c.combo_pressed), stroke],
        )
        .rules(
            c.combo_disabled
                .map(|_| RuleSelector::on(WidgetState::DISABLED)),
            c.combo_disabled.map(fill),
        )
        .build();
    // Hovered, the label as at rest (`*.hovered.popup` differs in 0 pixels, the same runs);
    // pressed too: the highlighted cell's label is labelColor within 1 level (macOS 15: #242424 /
    // #ffffffd9 over the opaque pressed fill, which #000000d9 over it gives; macOS 26: #000000d7 /
    // #ffffffd9, os-probe runs 45208260830 and 45217592058, `*.highlighted.popup` `label`).
    let label = RuleBuilder::new(label(&typography.body, Some(c.combo_foreground)))
        .rules(
            [WidgetState::HOVER, WidgetState::PRESSED].map(RuleSelector::on),
            [TextProp::Color(c.combo_foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    let chevron = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(g.combo_arrow_size)])
        .build();
    ComboBoxWidget::new(
        WidgetOrigin::Native,
        field,
        label,
        chevron,
        None,
        // 0.7.2: `combo_box.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The segmented control: a track in the window's background (0.7.2:
/// `segmented_control.background_color` is `defaults.background_color`
/// where unstated) in the default line (`macos-tahoe`: none, measured) and
/// radius (`segmented_control.border`: `defaults.border` where unstated), no
/// shadow; segments of a least height, `macos-monterey`'s padding above and
/// below, an unselected one pressed in its measured fill, the selected one in `macos-monterey`'s own fill,
/// else the accent (`active_background`: `defaults.accent_color` where
/// unstated); the UI font in the window's text (`segmented_control.font`:
/// `defaults.font` where unstated), the selected one's in
/// `macos-monterey`'s own colour, else the accent's text
/// (`active_text_color`: `defaults.accent_text_color` where unstated).
fn segmented_control(
    c: &SchemeColors,
    g: &Generation,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SegmentedControlWidget {
    // A pressed segment's stroke where it is outlined: not stated, as macOS 15 draws it as a
    // raised bezel whose top and bottom rows change apart and whose sides were not read
    // (os-probe run 45198750893, `*.pressed.segment` `edge_top`, `edge_bottom`).
    let segment_edge = if g.segment_outlined {
        outline(layout, c.border)
    } else {
        PartProp::Stroke(Stroke::None)
    };
    let track = RuleBuilder::new(PartStyle::default())
        .base([fill(c.background), segment_edge.clone(), corners(layout)])
        .base(no_shadow(g.segmented_shadow_enabled))
        .build();
    let segment = RuleBuilder::new(PartStyle::default())
        .base([PartProp::MinSize([None, Some(g.segment_height)])])
        .base(
            g.segment_padding_vertical
                .map(|v| PartProp::PaddingSides(Edges::all(v), EdgeFlags::TOP | EdgeFlags::BOTTOM)),
        )
        // Pressed, an unselected segment's measured fill. Hovered, no fill of its own: the
        // hovered segment differs from its normal capture in 0 pixels (the label's measurement
        // below), where 0.7.2 macos-monterey.toml states an uncited hover fill (#e0e0e5 /
        // #444446).
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(WidgetState::SELECTED),
            [fill(c.segment_pressed)],
        )
        .rules(
            std::iter::once(RuleSelector::on(WidgetState::HOVER).except(WidgetState::SELECTED))
                .chain(
                    (!g.segment_outlined).then(|| {
                        RuleSelector::on(WidgetState::PRESSED).except(WidgetState::SELECTED)
                    }),
                ),
            [segment_edge],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(c.segmented_active.unwrap_or(roles.accent.fill))],
        )
        .build();
    // Hovered, the label as at rest (`*.hovered.segment` differs in 0 pixels, macOS 15 and 26);
    // pressed too (macOS 15: #242424 over the opaque pressed fill #f0f0f0, which labelColor over
    // it gives, dark #ffffffd9 as at rest; macOS 26: #000000d8 / #ffffffd8, `*.pressed.segment`
    // `label`, the reads of os-probe run 45208260830).
    let rest_label = label(&typography.body, None);
    let (selectors, color) =
        rest_color_rules(&rest_label, [WidgetState::HOVER, WidgetState::PRESSED]);
    let segment_label = RuleBuilder::new(rest_label)
        .rules(selectors, color)
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(
                c.segmented_active_foreground
                    .unwrap_or(roles.accent.foreground),
            )],
        )
        .build();
    SegmentedControlWidget::new(
        WidgetOrigin::Native,
        track,
        segment,
        segment_label,
        RuleBuilder::new(PartStyle::default()).build(),
        g.segment_separator_width,
        // 0.7.2: `segmented_control.disabled_opacity` is `defaults.disabled_opacity` where
        // unstated.
        Some(roles.disabled_opacity),
    )
}

/// The box: the view's background where a palette states no card's (0.7.2:
/// `card.background_color` is `defaults.surface_color` where unstated),
/// padded, in its measured outline and corners (0.7.2 states no outline,
/// corners or shadow of it).
fn card(c: &SchemeColors, g: &Generation, roles: &ColorRoles) -> CardWidget {
    let stroke = match (c.card_border, g.card_border_width) {
        (Some(color), Some(width)) => Stroke::uniform(width, color),
        _ => Stroke::None,
    };
    let container = RuleBuilder::new(PartStyle::default())
        .base(roles.card.background.or(roles.view.background).map(fill))
        .base([
            PartProp::Padding(Edges::symmetric(
                g.card_padding_vertical,
                g.card_padding_horizontal,
            )),
            PartProp::Stroke(stroke),
            PartProp::Corners(Corners::uniform(g.card_corner_radius)),
        ])
        .build();
    CardWidget::new(WidgetOrigin::Native, container)
}

/// The disclosure group: the default line and radius (0.7.2:
/// `expander.border` is `defaults.border` where unstated), no shadow; a
/// header of a least height with no fill; its title in the UI font
/// (`expander.font`: `defaults.font` where unstated); the arrow's size,
/// `macos-monterey`'s colour, and its side.
fn expander(
    c: &SchemeColors,
    g: &Generation,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ExpanderWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([outline(layout, c.border), corners(layout)])
        .base(no_shadow(g.expander_shadow_enabled))
        .build();
    // SwiftUI's `DisclosureGroup`, collapsed, draws no fill behind its title (its label's core
    // pixel lies on the window's background), at rest, hovered or pressed, and its title in
    // labelColor at 13 px as at rest: 0 of its pixels change hovered or pressed, at the chevron or
    // at the label (os-probe run 45214606115, macOS 15 and 26,
    // `switch-label-macos-{15,26}/measure.toml` `*.normal.disclosure` `label_ink`,
    // `label_within_3_levels_of`, `*.hovered_disclosure_*`, `*.pressed_disclosure_*`
    // `label_changed_from_rest_px`), where 0.7.2 macos-monterey.toml states an uncited hover fill
    // (#e0e0e5 / #444446). Hovered, its chevron as at rest (`chevron_changed_from_rest_px` 0); its
    // pressed chevron, a composite read nearest secondaryLabelColor in light and unchanged in
    // dark, is not stated.
    let no_fill = fill(Color::TRANSPARENT);
    let header = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::MinSize([None, Some(g.expander_header_height)]),
            no_fill.clone(),
        ])
        .rules(
            [WidgetState::HOVER, WidgetState::PRESSED].map(RuleSelector::on),
            [no_fill],
        )
        .build();
    let rest_label = label(&typography.body, None);
    let (selectors, color) =
        rest_color_rules(&rest_label, [WidgetState::HOVER, WidgetState::PRESSED]);
    let arrow = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(g.expander_arrow_size)])
        .base(c.expander_arrow.map(IconProp::Color))
        .rules(
            c.expander_arrow
                .map(|_| RuleSelector::on(WidgetState::HOVER)),
            c.expander_arrow.map(IconProp::Color),
        )
        .build();
    ExpanderWidget::new(
        WidgetOrigin::Native,
        container,
        header,
        RuleBuilder::new(rest_label).rules(selectors, color).build(),
        arrow,
        Some(g.expander_arrow_side),
        None,
        None,
        None,
    )
}

#[cfg(all(test, feature = "serde"))]
mod snapshots;
#[cfg(test)]
mod tests;
