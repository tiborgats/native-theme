//! The `adwaita` preset's builder (spec §6): GNOME's libadwaita, light and
//! dark.
//!
//! The builder runs in three steps: the preset's constants, one table per
//! scheme for the colours and plain constants for the rest; the colour roles
//! and the globals made from them; the widgets derived from the roles, the
//! globals and the colours. A widget's colour that is a role's colour is
//! taken from the roles, not from a second copy of the constant. The system's
//! settings ([`ResolveInputs::system`]) are laid over the constants where
//! given, and what 0.7.2's portal reader derives from them the builder
//! derives (spec §8, "The GNOME part"): from the accent, the accent's fill,
//! the selection, the tab's indicator, the info status and the focus ring
//! (libadwaita's standalone colour, through the `adw_standalone` kernel);
//! from the UI font, the body and the type scale libadwaita sizes as a share
//! of it; the monospace and the title bar fonts, the overlay scroll bars, the
//! icon theme and the caret's blink as given.
//!
//! The active palette ([`ResolveInputs::palette`]) and then an explicit
//! accent ([`ResolveInputs::accent`]) are laid over the system's settings
//! (spec §6, resolution step 2, layer 3). A palette states the roles its
//! slots name (spec §7, "The roles a palette states"), and each slot whose
//! libadwaita colour has the slot's name replaces that colour and every
//! widget colour the preset takes from it (libadwaita 1.10.0
//! `src/stylesheet/_colors.scss:12-71`): `window_bg` is `window_bg_color`,
//! so also the inactive title bar (`headerbar_backdrop_color`,
//! `_colors.scss:43`); `view_bg` is `view_bg_color`; `header_bg` is
//! `headerbar_bg_color`, the active title bar; `sidebar_bg` is
//! `sidebar_bg_color`; `surface_bg` is `card_bg_color`, `dialog_bg_color`
//! and `popover_bg_color`, the card's, the dialog's and the popover's
//! panels; `foreground` is every `*_fg_color` of those surfaces, the window
//! text the preset letters its widgets in, and the tooltip's text with
//! `tooltip_bg` (§7: one text colour for the eight surfaces); `accent` is
//! `accent_bg_color`, the accent's fill and the tab's active indicator, from
//! which the focus ring is derived again through `adw_standalone`; `error`,
//! `warning` and `success` are `destructive_bg_color`, `warning_bg_color`
//! and `success_bg_color`. A slot with no libadwaita colour of its name
//! states its role alone (§7): `foreground_muted`, `border` (the default
//! border, and so every outline drawn in it), `divider`, `accent_fg`,
//! `selection_bg`, `selection_fg`, `link`, `caret` and `info`; the
//! selection's slots stand over the accent the preset would take them from
//! (spec §6: a derivation never replaces a colour a palette states in a slot
//! of its own), and the unfocused and the text selection, which the preset
//! takes from the selection, follow them, as the disabled text follows the
//! window text. Every other colour keeps the preset's value: the fills the
//! preset states as libadwaita's translucent layers already laid over its
//! own window (the buttons', the entries', the hover fills), the
//! placeholder, the switch's and the slider's colours, the status colours'
//! text. An explicit accent is libadwaita's accent as the system's accent
//! is ([`lay_accent_over`]), the later statement of layer 3: it replaces the
//! palette's `accent`, `selection_bg`, `info`, and with the accent its text
//! (`accent_fg`, `selection_fg`), which are then the preset's.
//!
//! Defined by the preset (spec §1, "Axis values without data"): the variant
//! `ButtonVariant::Primary` (0.7.2's `primary_*` fields), a list's alternate
//! rows (`ListKind::alternate`), the two orientations of the scroll bar and
//! of the slider's groove, no control size, and the contrast levels `Normal`
//! and `High`. `High` states one value of its own, the focus ring's alpha 0.8,
//! which the focused text field's outline takes too
//! ([`FOCUS_RING_ALPHA_HIGH`]); every other value at `High` is `Normal`'s, as
//! no source the builder reads states it. A query for anything else returns
//! the part's base style. A maximized or full-screen window has no data of
//! its own: its frame and title bar are the normal window's.
//!
//! A part's exclusive state group names only the flags that part has a rule
//! for: a state without data of its own returns what the query returns
//! without that state, so a flag the part does not style removes no other.
//! The groups: the button's body and the scroll bar's handle `PRESSED,
//! HOVER`; every other part none.
//!
//! Where two rules can both match, the later one wins (spec §3, "State
//! combinations a source does not state"); the precedences, each with its
//! source:
//! - the button's body: the pressed fill over the hovered fill (0.7.2's
//!   pressed colours win over its hover colours), both over the checked fill,
//!   for which no source the builder reads states a hovered or a pressed look;
//!   the primary fill over all of them, as 0.7.2's connectors keep
//!   `primary_background` in every state. Its label: the disabled colour over
//!   the primary one;
//! - the check box's and the radio button's indicator: the checked fill and
//!   outline over the hovered fill, as 0.7.2's egui connector draws a checked
//!   box ("`checkbox.hover_background` is the unchecked box's hover fill, so
//!   the hovered entry holds the checked fill too":
//!   `connectors/native-theme-egui/src/style/variants.rs`);
//! - the switch's track: the hovered on track (`HOVER | CHECKED`), whose
//!   colour 0.7.2 states apart (`switch.hover_checked_background`), over the
//!   hovered and the checked track, its rule last;
//! - the list's row highlight, the sidebar's item, the tab and the segmented
//!   control's segment: the selected fill over the hovered fill, as 0.7.2's
//!   egui connector puts the selection's colour in the selection's fill and
//!   the hover colour in the hovered widgets' fill
//!   (`connectors/native-theme-egui/src/mapping.md`) and egui fills a
//!   selected item with the selection's fill over its hovered look (egui
//!   0.37.0 `src/style.rs`, `Style::interact_selectable`). The list row's
//!   label: the disabled colour over the selection's;
//! - the toolbar's button: the checked fill over the hovered and the pressed
//!   fills, as 0.7.2's gpui connector draws a toolbar button that is on (a
//!   selected button takes no hover or press style:
//!   `connectors/native-theme-gpui/src/geometry.rs`,
//!   `toolbar_button_checked`), and the pressed fill over the hovered one.
//!
//! An inactive window (`ThemeContext::window_active` false) has the inactive
//! title bar (`window.inactive_title_bar_background`) and, as 0.7.2 resolves
//! `window.inactive_title_bar_text_color`, the active title's colour.
//!
//! Of the context it acts on the scheme, on the contrast (`High`, above) and
//! on the direction: the preset states every per-side value with equal left
//! and right sides, so a layout from right to left is the left-to-right one,
//! its per-side values placed by the shared helper ([`physical_edges`]); it
//! states no per-corner radius. The text scale is linear (spec §6,
//! resolution step 2): every font size, converted at the font DPI, times the
//! text scale, not rounded (0.7.2 rounds a size only where the preset states
//! `font_whole_pixels`, which `adwaita.toml` does not), and every line height
//! times it too; each `defined_size` stays the unscaled size. The motion
//! preference: `MotionPreference::None` sets the dialog's entrance to
//! `DialogEntranceStyle::None` and would collapse every transition, and the
//! preset states no transition and no motion token; no source the builder
//! reads states a policy of GNOME's for `MotionPreference::Reduced`, which
//! therefore changes nothing. The accessibility flags change nothing here.
//!
//! Sources: `adwaita.toml` of 0.7.2 (`src/presets/adwaita.toml`), each
//! constant with the citation the TOML gives it (`docs/platform-facts.md` by
//! section, the upstream file at its pinned version) or `(uncited)`; a value
//! the TOML marks ❓ has no source, as its header says. An upstream file a
//! citation names without a version (`_buttons.scss:28-33`) is that of the
//! versions the TOML's header names: libadwaita 1.10.0 (c9ab1186, GNOME 51),
//! GTK 4.24.1 and gsettings-desktop-schemas 51.0. 0.7.2's inheritance rules
//! (`src/resolve/inheritance.rs`, `src/resolve/validate.rs`,
//! `src/resolve/validate_helpers.rs`; `docs/inheritance-rules.toml`) where
//! 0.7.2 fills a value the TOML does not state; 0.7.2's GNOME reader
//! (`src/gnome/accent.rs`, `focus_ring`) for the focus ring's alpha under high
//! contrast.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{Corners, Edges, Stroke};
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::{
    ArrowSide, DialogButtonOrder, LabelSide, LayoutMetrics, TabIndicatorSide,
};
use crate::next::motion::{CaretSpec, DialogEntranceStyle, MotionTokens, UiTimings};
use crate::next::paint::Paint;
use crate::next::palette::Palette;
use crate::next::platform::adw_standalone::adw_standalone;
use crate::next::preset::{Preset, ResolveInputs};
use crate::next::presets::helpers::{
    Motion, dialog_entrance, linear_font_px, motion, motion_tokens, physical_edges,
    rest_color_rules,
};
use crate::next::query::{
    ButtonVariant, Contrast, Direction, Orientation, Scheme, ThemeContext, WidgetState,
};
use crate::next::reader::{OsFont, SystemOverrides};
use crate::next::roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
use crate::next::rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, Patched, RuleBuilder, RuleSelector, RuleSet,
    TextProp,
};
use crate::next::style::{
    DecorationKind, FocusIndicator, FocusKind, FontSize, FontStyle, IconStyle, PartStyle,
    TextDecoration, TextStated, TextStyle,
};
use crate::next::typography::{TextRendering, Typography};
use crate::next::widgets::{
    ButtonWidget, CardWidget, CheckboxWidget, ComboBoxWidget, DialogWidget, ExpanderWidget,
    InputWidget, LinkWidget, ListWidget, MenuBarWidget, MenuWidget, PopoverWidget,
    ProgressBarWidget, RadioWidget, ResolvedTheme, ScrollbarWidget, SegmentedControlWidget,
    SeparatorWidget, SidebarWidget, SliderWidget, SpinnerWidget, SplitterWidget, StatusBarWidget,
    SwitchWidget, TabWidget, ToolbarWidget, TooltipWidget, WidgetOrigin,
};
use crate::next::window::WindowChrome;

// ---------------------------------------------------------------------------
// The constants
// ---------------------------------------------------------------------------

/// The colours of one scheme the builder works from: the constants
/// `adwaita.toml` states, each colour from its 8-bit sRGB channels
/// ([`srgb8`]). A key 0.7.2 states with the same colour in both schemes as
/// another key's, for the same libadwaita colour, shares that key's field;
/// each key's citation stands at the field.
#[derive(Clone, Copy)]
struct SchemeColors {
    window_background: Color,
    window_foreground: Color,
    /// libadwaita's `window_fg_color`, which `view_fg_color` and
    /// `card_fg_color` equal: a list row's text.
    row_foreground: Color,
    window_foreground_muted: Color,
    window_border: Color,
    view_background: Color,
    accent: Color,
    accent_foreground: Color,
    focus_ring: Color,
    link: Color,
    shadow: Color,
    error: Color,
    error_foreground: Color,
    warning: Color,
    warning_foreground: Color,
    success: Color,
    success_foreground: Color,
    button_background: Color,
    button_hover: Color,
    button_pressed: Color,
    checkbox_background: Color,
    button_active_layer: Color,
    trough_active: Color,
    trough_hover: Color,
    view_active: Color,
    active_layer: Color,
    hover_layer: Color,
    frame: Color,
    row_hover: Color,
    list_alternate_row: Color,
    placeholder: Color,
    menu_hover: Color,
    popover_background: Color,
    scrollbar_thumb_hover: Color,
    scrollbar_thumb_pressed: Color,
    segment_selected: Color,
    segment_selected_foreground: Color,
    sidebar_background: Color,
    sidebar_hover: Color,
    slider_thumb: Color,
    slider_thumb_hover: Color,
    /// The paned separator's line, in every state.
    splitter_line: Color,
    switch_hover_off: Color,
    switch_hover_on: Color,
    switch_thumb: Color,
    /// The switch's thumb when off, where stated; unstated, 0.7.2's
    /// `switch.unchecked_thumb_background` is `thumb_background`.
    switch_thumb_off: Option<Color>,
    tooltip_background: Color,
    tooltip_foreground: Color,
    toolbar_button_hover: Color,
    toolbar_button_pressed: Color,
    toolbar_button_checked: Color,
    /// The active window's title bar, where stated; unstated, 0.7.2's
    /// `window.title_bar_background` is `defaults.surface_color`.
    title_bar_background: Option<Color>,
    /// The card's fill, where stated; unstated, 0.7.2's
    /// `card.background_color` is `defaults.surface_color`.
    card_background: Option<Color>,
}

/// The light scheme's constants.
const LIGHT: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md (§2.1.3): window_bg_color #fafafb (libadwaita
    // 1.10.0 _colors.scss:32)
    // `tab.active_background`: 0.7.2 adwaita.toml (uncited)
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): headerbar:backdrop fills
    // --headerbar-backdrop-color = @window_bg_color #fafafb (libadwaita 1.10.0
    // _header-bar.scss:41-42, _colors.scss:43, :32)
    window_background: srgb8(0xfafa_fbff),
    // `defaults.text_color`: ❓ platform-facts.md (§2.1.3): no source for #2e3436 at libadwaita
    // 1.10.0 (docs/todo.md, "Presets: colours with no reproducible source")
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): the normal text colour;
    // libadwaita dims a disabled widget by opacity alone
    // `input.caret_color`: platform-facts.md (§2.4): caret-color currentColor, the entry's text
    // colour, input.font.color (libadwaita 1.10.0 _entries.scss:11)
    // `button.font.color`, `button.hover_text_color`, `button.active_text_color`,
    // `button.checked_text_color`, `input.font.color`, `list.hover_text_color`,
    // `menu.hover_text_color`, `popover.font.color`, `sidebar.font.color`,
    // `tab.active_text_color`, `tab.hover_text_color`, `window.title_bar_font.color`,
    // `expander.arrow_color`: 0.7.2 adwaita.toml (uncited)
    window_foreground: srgb8(0x2e34_36ff),
    // `switch.label`: window_fg_color, view_fg_color and card_fg_color RGB(0 0 6 / 80%)
    // (_colors.scss:33, :37, :62 at libadwaita 1.10.0); 0.8 × 255 = 204 = 0xcc
    row_foreground: srgb8(0x0000_06cc),
    // `defaults.muted_color`: ❓ platform-facts.md (§2.1.3): no source for #929292 at libadwaita
    // 1.10.0 (docs/todo.md, "Presets: colours with no reproducible source")
    window_foreground_muted: srgb8(0x9292_92ff),
    // `defaults.border.color`: platform-facts.md (§2.1.3): $border_color, window fg RGB(0 0 6 /
    // 80%) at --border-opacity 15% over #fafafb (libadwaita 1.10.0 _colors.scss:32-33, :257,
    // :263, :318)
    window_border: srgb8(0xdcdc_deff),
    // `defaults.surface_color`: platform-facts.md (§2.1.3): view_bg_color #ffffff (libadwaita
    // 1.10.0 _colors.scss:36)
    // `list.header_background`: platform-facts.md (§2.15): the header's buttons are transparent
    // (_column-view.scss:12-13) over the column view's .view fill, --view-bg-color #ffffff
    // (_column-view.scss:72-76, _views.scss:1-4, _colors.scss:36, libadwaita 1.10.0)
    view_background: srgb8(0xffff_ffff),
    // `defaults.accent_color`: platform-facts.md (§2.1.3): the portal accent; libadwaita's
    // default, blue #3584e4 (libadwaita 1.10.0 adw-accent-color.c:16, :53-54)
    // `defaults.selection_background`: platform-facts.md (§2.1.3): theme_selected_bg_color =
    // accent_bg_color, #3584e4 (libadwaita 1.10.0 _compat-colors.scss:9)
    // `defaults.info_color`: platform-facts.md (§2.1.4): libadwaita 1.10.0 names no info colour;
    // the deprecated GtkInfoBar draws its info type from --accent-bg-color
    // (_deprecated.scss:174), the default accent #3584e4 (adw-accent-color.c:53-54)
    // `tab.active_indicator_color`: platform-facts.md (§2.11): a checked notebook tab is inset 0
    // -4px var(--accent-bg-color) (_notebook.scss:19)
    accent: srgb8(0x3584_e4ff),
    // `defaults.accent_text_color`: platform-facts.md (§2.1.3): accent_fg_color white (libadwaita
    // 1.10.0 adw-style-manager.c:161)
    // `defaults.selection_text_color`: platform-facts.md (§2.1.3): theme_selected_fg_color =
    // accent_fg_color, white (libadwaita 1.10.0 _compat-colors.scss:10, adw-style-manager.c:161)
    // `defaults.info_text_color`: platform-facts.md (§2.1.4): accent_fg_color white, the text on
    // the accent (libadwaita 1.10.0 adw-style-manager.c:161)
    accent_foreground: srgb8(0xffff_ffff),
    // `defaults.focus_ring_color`: platform-facts.md §2.1.5 focus_ring_color: $focus_border_color,
    // the standalone accent #0461be at 50 % (libadwaita 1.10.0 _drawing.scss:20, _colors.scss:260,
    // :307, :146; adw-accent-color.c:130-157)
    // `input.focus_border_color`: platform-facts.md §2.4 focus_border_color: $focus_border_color,
    // the focus ring's colour (_entries.scss:17, _colors.scss:307)
    focus_ring: srgb8(0x0461_be80),
    // `defaults.link_color`: ❓ platform-facts.md (§2.1.3): no source for #1b6acb at libadwaita
    // 1.10.0 (docs/todo.md, "Presets: colours with no reproducible source")
    // `link.disabled_text_color`: platform-facts.md (§2.1.6): the enabled link colour, at
    // disabled_opacity
    // `link.hover_text_color`, `link.active_text_color`: 0.7.2 adwaita.toml (uncited)
    link: srgb8(0x1b6a_cbff),
    // `defaults.shadow_color`: ❓ platform-facts.md (§2.1.3): no source for #00000018 at
    // libadwaita 1.10.0 (docs/todo.md, "Presets: colours with no reproducible source")
    shadow: srgb8(0x0000_0018),
    // `defaults.danger_color`: platform-facts.md (§2.1.4): destructive_bg_color @red_3 #e01b24
    // (libadwaita 1.10.0 _colors.scss:12, _palette.scss:23)
    error: srgb8(0xe01b_24ff),
    // `defaults.danger_text_color`: platform-facts.md (§2.1.4): destructive_fg_color white
    // (libadwaita 1.10.0 _colors.scss:13)
    error_foreground: srgb8(0xffff_ffff),
    // `defaults.warning_color`: platform-facts.md (§2.1.4): warning_bg_color @yellow_5 #e5a50a
    // (libadwaita 1.10.0 _colors.scss:19, _palette.scss:15)
    warning: srgb8(0xe5a5_0aff),
    // `defaults.warning_text_color`: ❓ platform-facts.md (§2.1.4): no source for #2e3436 at
    // libadwaita 1.10.0 (docs/todo.md, "Presets: colours with no reproducible source")
    warning_foreground: srgb8(0x2e34_36ff),
    // `defaults.success_color`: platform-facts.md (§2.1.4): success_bg_color @green_4 #2ec27e
    // (libadwaita 1.10.0 _colors.scss:16, _palette.scss:9)
    success: srgb8(0x2ec2_7eff),
    // `defaults.success_text_color`: platform-facts.md (§2.1.4): success_fg_color white
    // (libadwaita 1.10.0 _colors.scss:17)
    success_foreground: srgb8(0xffff_ffff),
    // `button.background_color`: platform-facts.md §2.3 background_color: a raised button fills
    // with $button_color, currentColor 10 %, over the window, #e6e6e7 (libadwaita 1.10.0
    // _buttons.scss:1, :28-29, _colors.scss:32-33, :95-96)
    // `combo_box.background_color`: platform-facts.md §2.24 background_color: the trigger button's
    // $button_color over the window, #e6e6e7 (libadwaita 1.10.0 _buttons.scss:28-29;
    // _dropdowns.scss:1-12 adds no fill)
    // `input.background_color`: platform-facts.md §2.4 background_color: an entry fills with
    // $button_color, currentColor 10 %, over the window, #e6e6e7 (libadwaita 1.10.0
    // _entries.scss:9, _buttons.scss:1, _colors.scss:32-33, :95-96)
    // `segmented_control.background_color`: platform-facts.md §2.25 background_color:
    // AdwToggleGroup's $button_color, currentColor 10 % (_toggle-group.scss:12, _buttons.scss:1),
    // the window fg RGB(0 0 6 / 80%) so 8 % of #000006, laid over the window's #fafafb
    // (libadwaita 1.10.0 _colors.scss:32-33)
    button_background: srgb8(0xe6e6_e7ff),
    // `button.hover_background`, `checkbox.hover_background`, `combo_box.hover_background`,
    // `segmented_control.hover_background`: 0.7.2 adwaita.toml (uncited)
    button_hover: srgb8(0xd5d5_d5ff),
    // `button.active_background`: 0.7.2 adwaita.toml (uncited)
    // `button.checked_background`: platform-facts.md (§2.3): $button_checked_color is the pressed
    // fill's 30 % currentColor (_buttons.scss:3-6); no :checked rule sets a label colour
    button_pressed: srgb8(0xc8c8_c8ff),
    // `checkbox.unchecked_background`: no disabled_background / disabled_text_color:
    // platform-facts.md (§2.1.6), a disabled check is its enabled self at disabled_opacity
    checkbox_background: srgb8(0xffff_ffff),
    // a pressed drop-down's trigger, a raised button: :active, $button_active_color, the window
    // text rgb(0 0 6 / 80%) at 30 %, a layer (_buttons.scss:3, :39-42, _dropdowns.scss:1-12,
    // _colors.scss:33 at libadwaita 1.10.0)
    button_active_layer: srgb8(0x0000_063d),
    // a pressed unchecked check box's and off switch's fill: $trough_active_color, the text at
    // 25 % (_checks.scss:42-44, _switch.scss:9, _colors.scss:282)
    trough_active: srgb8(0x0000_0633),
    // a hovered unchecked check's or radio's outline, an inset 2px box-shadow: $trough_hover_color,
    // the text rgb(0 0 6 / 80%) at 20 %, rgb(0 0 6 / 16%) (_checks.scss:20-35, _colors.scss:281,
    // :33 at libadwaita 1.10.0)
    trough_hover: srgb8(0x0000_0629),
    // a pressed activatable row's fill, a list's and a boxed list's (the expander's header):
    // $view_active_color and %boxed_list_row's :active, the text at 8 % (_lists.scss:75-78,
    // :352-360, :524-526, _colors.scss:275)
    view_active: srgb8(0x0000_0610),
    // a pressed toggle's and sidebar row's fill: $active_color, the text at 16 %
    // (_toggle-group.scss:69-70, _sidebars.scss:126-129, _colors.scss:269, :49)
    active_layer: srgb8(0x0000_0621),
    // `tab.hover_background`: a notebook tab hovered or pressed, `&:hover, &:active {
    // background-color: $hover_color }`, the text at 7 % (_notebook.scss:127-129,
    // _colors.scss:268)
    hover_layer: srgb8(0x0000_060e),
    // `checkbox.unchecked_border_color`, `separator.line_color`, `splitter.hover_color`,
    // `switch.unchecked_background`: 0.7.2 adwaita.toml (uncited)
    frame: srgb8(0xd5d5_d5ff),
    // `list.hover_background`, `expander.hover_background`: 0.7.2 adwaita.toml (uncited)
    row_hover: srgb8(0xebeb_edff),
    // `list.alternate_row_background`: 0.7.2 adwaita.toml (uncited)
    list_alternate_row: srgb8(0xf5f5_f5ff),
    // `input.placeholder_color`: 0.7.2 adwaita.toml (uncited)
    placeholder: srgb8(0x9295_95ff),
    // `menu.hover_background`: 0.7.2 adwaita.toml (uncited)
    menu_hover: srgb8(0xe8e8_e8ff),
    // `popover.background_color`: 0.7.2 adwaita.toml (uncited)
    popover_background: srgb8(0xffff_ffff),
    // `scrollbar.thumb_hover_color`: 0.7.2 adwaita.toml (uncited)
    scrollbar_thumb_hover: srgb8(0x6e6e_6eff),
    // `scrollbar.thumb_active_color`: 0.7.2 adwaita.toml (uncited)
    scrollbar_thumb_pressed: srgb8(0x4e4e_4eff),
    // `segmented_control.active_background`: platform-facts.md (§2.25): AdwToggleGroup's checked
    // toggle, --active-toggle-bg-color #ffffff and --active-toggle-fg-color RGB(0 0 6 / 80%) over
    // it (libadwaita 1.10.0 _toggle-group.scss:79-81, _colors.scss:238-239)
    segment_selected: srgb8(0xffff_ffff),
    // `segmented_control.active_text_color`: 0.7.2 adwaita.toml (uncited)
    segment_selected_foreground: srgb8(0x3333_38ff),
    // `sidebar.background_color`: 0.7.2 adwaita.toml (uncited)
    sidebar_background: srgb8(0xebeb_edff),
    // `sidebar.hover_background`: 0.7.2 adwaita.toml (uncited)
    sidebar_hover: srgb8(0xe0e0_e2ff),
    // `slider.thumb_color`: platform-facts.md §2.9 thumb_color: $slider_color, white 80 % over the
    // view's #ffffff (libadwaita 1.10.0 _scale.scss:50, _colors.scss:288, :36)
    slider_thumb: srgb8(0xffff_ffff),
    // `slider.thumb_hover_color`, `slider.thumb(PRESSED).fill`: $slider_hover_color, white, under
    // `&:hover, &:active` (libadwaita 1.10.0 _scale.scss:62-76, _colors.scss:289), where 0.7.2
    // adwaita.toml states #ebebeb (uncited)
    slider_thumb_hover: srgb8(0xffff_ffff),
    // `splitter.handle` in every state: the separator's `box-shadow: inset 1px 0 $border_color`
    // with no :hover or :active rule (libadwaita 1.10.0 _paned.scss:1-36); $border_color is
    // currentColor at --border-opacity 15 % (_colors.scss:263, :257), currentColor rgb(0 0 6 / 80
    // %) (_colors.scss:33), so rgb(0 0 6 / 12 %), 8-bit rounded; platform-facts.md (§2.17)
    // `hover_color`
    splitter_line: srgb8(0x0000_061f),
    // `switch.hover_unchecked_background`: 0.7.2 adwaita.toml (uncited)
    switch_hover_off: srgb8(0xc8c8_c8ff),
    // `switch.hover_checked_background`: 0.7.2 adwaita.toml (uncited)
    switch_hover_on: srgb8(0x2e78_ccff),
    // `switch.thumb_background`: 0.7.2 adwaita.toml (uncited)
    switch_thumb: srgb8(0xffff_ffff),
    switch_thumb_off: None,
    // `tooltip.background_color`: 0.7.2 adwaita.toml (uncited)
    tooltip_background: srgb8(0x2e34_36ff),
    // `tooltip.font.color`: 0.7.2 adwaita.toml (uncited)
    tooltip_foreground: srgb8(0xffff_ffff),
    // `toolbar.button_hover_background`: platform-facts.md (§2.13): the toolbar look of an image,
    // image-text or arrow button (_toolbars.scss:2-8): :hover, $hover_color, the window text
    // #000006cc at 7 %, a layer (_colors.scss:268, _buttons.scss:250-251 at libadwaita 1.10.0)
    toolbar_button_hover: srgb8(0x0000_060e),
    // `toolbar.button_active_background`: platform-facts.md (§2.13): :active, $active_color, the
    // window text at 16 % (_colors.scss:269, _buttons.scss:254-256)
    toolbar_button_pressed: srgb8(0x0000_0621),
    // `toolbar.button_checked_background`: platform-facts.md (§2.13): :checked, the window text at
    // 10 % (_colors.scss:270, _buttons.scss:259-260)
    toolbar_button_checked: srgb8(0x0000_0614),
    title_bar_background: None,
    card_background: None,
};

/// The dark scheme's constants.
const DARK: SchemeColors = SchemeColors {
    // `defaults.background_color`, `tab.active_background`: 0.7.2 adwaita.toml (uncited)
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): headerbar:backdrop fills
    // --headerbar-backdrop-color = @window_bg_color #222226 (libadwaita 1.10.0
    // _header-bar.scss:41-42, _colors.scss:43, :95)
    window_background: srgb8(0x2222_26ff),
    // `defaults.text_color`: 0.7.2 adwaita.toml (uncited)
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): the normal text colour;
    // libadwaita dims a disabled widget by opacity alone
    // `input.caret_color`: platform-facts.md (§2.4): caret-color currentColor, the entry's text
    // colour, input.font.color (libadwaita 1.10.0 _entries.scss:11)
    // `button.font.color`, `button.hover_text_color`, `button.active_text_color`,
    // `button.checked_text_color`, `input.font.color`, `list.hover_text_color`,
    // `menu.hover_text_color`, `popover.font.color`, `sidebar.font.color`,
    // `tab.active_text_color`, `tab.hover_text_color`, `window.title_bar_font.color`,
    // `expander.arrow_color`: 0.7.2 adwaita.toml (uncited)
    window_foreground: srgb8(0xffff_ffff),
    // `switch.label`: window_fg_color, view_fg_color and card_fg_color white (_colors.scss:96,
    // :100, :124 at libadwaita 1.10.0)
    row_foreground: srgb8(0xffff_ffff),
    // `defaults.muted_color`: 0.7.2 adwaita.toml (uncited)
    window_foreground_muted: srgb8(0x9292_92ff),
    // `defaults.border.color`: platform-facts.md (§2.1.3): $border_color, white at
    // --border-opacity 15% over #222226 (libadwaita 1.10.0 _colors.scss:95-96, :257, :263, :318)
    window_border: srgb8(0x4343_47ff),
    // `defaults.surface_color`: platform-facts.md (§2.1.3): view_bg_color #1d1d20 (libadwaita
    // 1.10.0 _colors.scss:99)
    // `list.header_background`: platform-facts.md (§2.15): the header's buttons are transparent
    // (_column-view.scss:12-13) over the column view's .view fill, --view-bg-color #1d1d20
    // (_column-view.scss:72-76, _views.scss:1-4, _colors.scss:99, libadwaita 1.10.0)
    view_background: srgb8(0x1d1d_20ff),
    // `defaults.accent_color`, `defaults.selection_background`, `defaults.info_color`: 0.7.2
    // adwaita.toml (uncited)
    // `tab.active_indicator_color`: platform-facts.md (§2.11): a checked notebook tab is inset 0
    // -4px var(--accent-bg-color) (_notebook.scss:19)
    accent: srgb8(0x3584_e4ff),
    // `defaults.accent_text_color`, `defaults.selection_text_color`, `defaults.info_text_color`:
    // 0.7.2 adwaita.toml (uncited)
    accent_foreground: srgb8(0xffff_ffff),
    // `defaults.focus_ring_color`: platform-facts.md §2.1.5 focus_ring_color: $focus_border_color,
    // the standalone accent #81d0ff at 50 % (libadwaita 1.10.0 _drawing.scss:20, _colors.scss:260,
    // :307, :151; adw-accent-color.c:130-157)
    // `input.focus_border_color`: platform-facts.md §2.4 focus_border_color: $focus_border_color,
    // the focus ring's colour (_entries.scss:17, _colors.scss:307)
    focus_ring: srgb8(0x81d0_ff80),
    // `defaults.link_color`: 0.7.2 adwaita.toml (uncited)
    // `link.disabled_text_color`: platform-facts.md (§2.1.6): the enabled link colour, at
    // disabled_opacity
    // `link.hover_text_color`, `link.active_text_color`: 0.7.2 adwaita.toml (uncited)
    link: srgb8(0x5b9b_f8ff),
    // `defaults.shadow_color`: 0.7.2 adwaita.toml (uncited)
    shadow: srgb8(0x0000_0040),
    // `defaults.danger_color`: 0.7.2 adwaita.toml (uncited)
    error: srgb8(0xc01c_28ff),
    // `defaults.danger_text_color`: 0.7.2 adwaita.toml (uncited)
    error_foreground: srgb8(0xffff_ffff),
    // `defaults.warning_color`: 0.7.2 adwaita.toml (uncited)
    warning: srgb8(0xcd93_09ff),
    // `defaults.warning_text_color`: ❓ platform-facts.md (§2.1.4): no source for #ffffff at
    // libadwaita 1.10.0, whose warning_fg_color is RGB(0 0 0 / 80%) in both schemes
    // (_colors.scss:20; the dark block, :82-143, does not redefine it) (docs/todo.md, "Presets:
    // colours with no reproducible source")
    warning_foreground: srgb8(0xffff_ffff),
    // `defaults.success_color`: 0.7.2 adwaita.toml (uncited)
    success: srgb8(0x26a2_69ff),
    // `defaults.success_text_color`: 0.7.2 adwaita.toml (uncited)
    success_foreground: srgb8(0xffff_ffff),
    // `button.background_color`: platform-facts.md §2.3 background_color: a raised button fills
    // with $button_color, currentColor 10 %, over the window, #38383c (libadwaita 1.10.0
    // _buttons.scss:1, :28-29, _colors.scss:32-33, :95-96)
    // `combo_box.background_color`: platform-facts.md §2.24 background_color: the trigger button's
    // $button_color over the window, #38383c (libadwaita 1.10.0 _buttons.scss:28-29;
    // _dropdowns.scss:1-12 adds no fill)
    // `input.background_color`: platform-facts.md §2.4 background_color: an entry fills with
    // $button_color, currentColor 10 %, over the window, #38383c (libadwaita 1.10.0
    // _entries.scss:9, _buttons.scss:1, _colors.scss:32-33, :95-96)
    // `segmented_control.background_color`: platform-facts.md §2.25 background_color:
    // AdwToggleGroup's $button_color, currentColor 10 % (_toggle-group.scss:12, _buttons.scss:1),
    // the window fg white at 10 %, laid over the window's #222226 (libadwaita 1.10.0
    // _colors.scss:95-96)
    button_background: srgb8(0x3838_3cff),
    // `button.hover_background`, `checkbox.hover_background`, `combo_box.hover_background`,
    // `segmented_control.hover_background`: 0.7.2 adwaita.toml (uncited)
    button_hover: srgb8(0x4646_49ff),
    // `button.active_background`: 0.7.2 adwaita.toml (uncited)
    // `button.checked_background`: platform-facts.md (§2.3): $button_checked_color is the pressed
    // fill's 30 % currentColor (_buttons.scss:3-6); no :checked rule sets a label colour
    button_pressed: srgb8(0x2e2e_32ff),
    // `checkbox.unchecked_background`: no disabled_background / disabled_text_color:
    // platform-facts.md (§2.1.6), a disabled check is its enabled self at disabled_opacity
    checkbox_background: srgb8(0x3030_34ff),
    // a pressed drop-down's trigger, a raised button: :active, $button_active_color, the window
    // text white at 30 %, a layer (_buttons.scss:3, :39-42, _dropdowns.scss:1-12, _colors.scss:96
    // at libadwaita 1.10.0)
    button_active_layer: Color::new(1.0, 1.0, 1.0, 0.3),
    // a pressed unchecked check box's and off switch's fill: $trough_active_color, the text at
    // 25 % (_checks.scss:42-44, _switch.scss:9, _colors.scss:282)
    trough_active: srgb8(0xffff_ff40),
    // a hovered unchecked check's or radio's outline, an inset 2px box-shadow: $trough_hover_color,
    // the text white at 20 % (_checks.scss:20-35, _colors.scss:281, :96 at libadwaita 1.10.0)
    trough_hover: srgb8(0xffff_ff33),
    // a pressed activatable row's fill, a list's and a boxed list's (the expander's header):
    // $view_active_color and %boxed_list_row's :active, the text at 8 % (_lists.scss:75-78,
    // :352-360, :524-526, _colors.scss:275)
    view_active: srgb8(0xffff_ff14),
    // a pressed toggle's and sidebar row's fill: $active_color, the text at 16 %
    // (_toggle-group.scss:69-70, _sidebars.scss:126-129, _colors.scss:269, :111)
    active_layer: srgb8(0xffff_ff29),
    // `tab.hover_background`: a notebook tab hovered or pressed, `&:hover, &:active {
    // background-color: $hover_color }`, the text at 7 % (_notebook.scss:127-129,
    // _colors.scss:268)
    hover_layer: srgb8(0xffff_ff12),
    // `checkbox.unchecked_border_color`, `separator.line_color`, `splitter.hover_color`,
    // `switch.unchecked_background`: 0.7.2 adwaita.toml (uncited)
    frame: srgb8(0x4a4a_4eff),
    // `list.hover_background`, `expander.hover_background`: 0.7.2 adwaita.toml (uncited)
    row_hover: srgb8(0x3030_34ff),
    // `list.alternate_row_background`: 0.7.2 adwaita.toml (uncited)
    list_alternate_row: srgb8(0x2a2a_2eff),
    // `input.placeholder_color`: 0.7.2 adwaita.toml (uncited)
    placeholder: srgb8(0x8b8b_8bff),
    // `menu.hover_background`: 0.7.2 adwaita.toml (uncited)
    menu_hover: srgb8(0x3a3a_3eff),
    // `popover.background_color`: 0.7.2 adwaita.toml (uncited)
    popover_background: srgb8(0x3636_3aff),
    // `scrollbar.thumb_hover_color`: 0.7.2 adwaita.toml (uncited)
    scrollbar_thumb_hover: srgb8(0xa0a0_a0ff),
    // `scrollbar.thumb_active_color`: 0.7.2 adwaita.toml (uncited)
    scrollbar_thumb_pressed: srgb8(0xc0c0_c0ff),
    // `segmented_control.active_background`: platform-facts.md (§2.25): AdwToggleGroup's checked
    // toggle, --active-toggle-bg-color white at 20 % over the group's #38383c, and
    // --active-toggle-fg-color #ffffff (libadwaita 1.10.0 _toggle-group.scss:79-81,
    // _colors.scss:245-248)
    segment_selected: srgb8(0x6060_63ff),
    // `segmented_control.active_text_color`: 0.7.2 adwaita.toml (uncited)
    segment_selected_foreground: srgb8(0xffff_ffff),
    // `sidebar.background_color`: 0.7.2 adwaita.toml (uncited)
    sidebar_background: srgb8(0x2e2e_32ff),
    // `sidebar.hover_background`: 0.7.2 adwaita.toml (uncited)
    sidebar_hover: srgb8(0x3636_3aff),
    // `slider.thumb_color`: platform-facts.md §2.9 thumb_color: $slider_color, white 80 % over the
    // view's #1d1d20 (libadwaita 1.10.0 _scale.scss:50, _colors.scss:288, :99)
    slider_thumb: srgb8(0xd2d2_d2ff),
    // `slider.thumb_hover_color`, `slider.thumb(PRESSED).fill`: $slider_hover_color, white, under
    // `&:hover, &:active` (libadwaita 1.10.0 _scale.scss:62-76, _colors.scss:289), where 0.7.2
    // adwaita.toml states #3a3a3e (uncited)
    slider_thumb_hover: srgb8(0xffff_ffff),
    // `splitter.handle` in every state: the separator's $border_color line, as the light scheme's
    // (libadwaita 1.10.0 _paned.scss:1-36), currentColor #ffffff (_colors.scss:96) at 15 %
    // (:263, :318); platform-facts.md (§2.17) `hover_color`
    splitter_line: srgb8(0xffff_ff26),
    // `switch.hover_unchecked_background`: 0.7.2 adwaita.toml (uncited)
    switch_hover_off: srgb8(0x5656_5aff),
    // `switch.hover_checked_background`: 0.7.2 adwaita.toml (uncited)
    switch_hover_on: srgb8(0x2e78_ccff),
    // `switch.thumb_background`: 0.7.2 adwaita.toml (uncited)
    switch_thumb: srgb8(0xffff_ffff),
    // `switch.unchecked_thumb_background`: platform-facts.md (§2.21): $slider_color, white 80% over
    // view_bg_color #1d1d20; checked is white
    switch_thumb_off: Some(srgb8(0xd2d2_d2ff)),
    // `tooltip.background_color`: 0.7.2 adwaita.toml (uncited)
    tooltip_background: srgb8(0x3a3a_3eff),
    // `tooltip.font.color`: 0.7.2 adwaita.toml (uncited)
    tooltip_foreground: srgb8(0xffff_ffff),
    // `toolbar.button_hover_background`: platform-facts.md (§2.13): the toolbar look of an image,
    // image-text or arrow button (_toolbars.scss:2-8): :hover, $hover_color, the window text
    // #ffffff at 7 %, a layer (_colors.scss:268, _buttons.scss:250-251 at libadwaita 1.10.0)
    toolbar_button_hover: srgb8(0xffff_ff12),
    // `toolbar.button_active_background`: platform-facts.md (§2.13): :active, $active_color, the
    // window text at 16 % (_colors.scss:269, _buttons.scss:254-256)
    toolbar_button_pressed: srgb8(0xffff_ff29),
    // `toolbar.button_checked_background`: platform-facts.md (§2.13): :checked, the window text at
    // 10 % (_colors.scss:270, _buttons.scss:259-260)
    toolbar_button_checked: srgb8(0xffff_ff1a),
    // `window.title_bar_background`: platform-facts.md (§2.2): headerbar fills
    // --headerbar-bg-color = headerbar_bg_color #2e2e32 (libadwaita 1.10.0 _header-bar.scss:3,
    // _colors.scss:103)
    title_bar_background: Some(srgb8(0x2e2e_32ff)),
    // `card.background_color`: platform-facts.md (§2.26): .card fills var(--card-bg-color) =
    // card_bg_color RGB(255 255 255 / 8%), alpha 0x14 (libadwaita 1.10.0 _misc.scss:203,
    // _colors.scss:123)
    card_background: Some(srgb8(0xffff_ff14)),
};

/// The popover's outline colour, in both schemes.
// `popover.border.color`: platform-facts.md (§2.16): the outline is the box shadow 0 0 0 1px
// RGB(0 0 0 / 5%), light and dark (libadwaita 1.10.0 _popovers.scss:19)
const POPOVER_BORDER: Color = srgb8(0x0000_000d);

/// The expander's outline colour, in both schemes.
// `expander.border.color`: platform-facts.md §2.27 border.color, border.line_width: the
// boxed-list card's outline, 0 0 0 1px RGB(0 0 6 / 3%), in both modes (libadwaita 1.10.0
// _misc.scss:202-208); the shadow is the card's too (§2.27 border.shadow_enabled)
const EXPANDER_BORDER: Color = srgb8(0x0000_0608);

/// A link's background, at rest and under the pointer: transparent.
// `link.background_color`: 0.7.2 adwaita.toml (uncited)
// `link.hover_background`: platform-facts.md (§2.28): GNOME hover_background **(none)**,
// transparent as background_color
const LINK_BACKGROUND: Color = srgb8(0x0000_0000);

/// The UI font's family.
// `defaults.font.family`: 0.7.2 adwaita.toml (uncited)
const FONT_FAMILY: &str = "Adwaita Sans";

/// The UI font's size, in points.
// `defaults.font.size`, `defaults.font.defined_size`: 0.7.2 adwaita.toml (uncited)
const FONT_SIZE_PT: f32 = 11.0;

/// The UI font's weight.
// `defaults.font.weight`: 0.7.2 adwaita.toml (uncited)
const FONT_WEIGHT: u16 = 400;

/// The monospace font's family.
// `defaults.mono_font.family`: 0.7.2 adwaita.toml (uncited)
const MONO_FAMILY: &str = "Adwaita Mono";

/// The monospace font's size, in points.
// `defaults.mono_font.size`, `defaults.mono_font.defined_size`: 0.7.2 adwaita.toml (uncited)
const MONO_SIZE_PT: f32 = 11.0;

/// The monospace font's weight.
// `defaults.mono_font.weight`: 0.7.2 adwaita.toml (uncited)
const MONO_WEIGHT: u16 = 400;

/// The line height, a multiple of the font size.
// `defaults.line_height`: 0.7.2 adwaita.toml (uncited)
const LINE_HEIGHT: f32 = 1.21;

/// The caption's size, in points; its weight is the UI font's (0.7.2's
/// `resolve_text_scale_entry`).
// `text_scale.caption.size`: 0.7.2 adwaita.toml (uncited)
const CAPTION_SIZE_PT: f32 = 9.0;

/// The section heading's weight; its size is the UI font's (0.7.2's
/// `resolve_text_scale_entry`).
// `text_scale.section_heading.weight`: 0.7.2 adwaita.toml (uncited)
const HEADLINE_WEIGHT: u16 = 700;

/// The dialog title's text style's size, in points.
// `text_scale.dialog_title.size`: 0.7.2 adwaita.toml (uncited)
const TITLE1_SIZE_PT: f32 = 15.0;

/// The dialog title's text style's weight.
// `text_scale.dialog_title.weight`: 0.7.2 adwaita.toml (uncited)
const TITLE1_WEIGHT: u16 = 800;

/// The display style's size, in points.
// `text_scale.display.size`: 0.7.2 adwaita.toml (uncited)
const DISPLAY_SIZE_PT: f32 = 20.0;

/// The display style's weight.
// `text_scale.display.weight`: 0.7.2 adwaita.toml (uncited)
const DISPLAY_WEIGHT: u16 = 800;

/// The default corner radius.
// `defaults.border.corner_radius`: 0.7.2 adwaita.toml (uncited)
const CORNER_RADIUS: f32 = 9.0;

/// The large corner radius.
// `defaults.border.corner_radius_lg`: 0.7.2 adwaita.toml (uncited)
const CORNER_RADIUS_LG: f32 = 15.0;

/// The default border width.
// `defaults.border.line_width`: 0.7.2 adwaita.toml (uncited)
const BORDER_WIDTH: f32 = 1.0;

/// The border opacity.
// `defaults.border.opacity`: 0.7.2 adwaita.toml (uncited)
const BORDER_OPACITY: f32 = 0.15;

/// Whether surfaces cast shadows.
// `defaults.border.shadow_enabled`: 0.7.2 adwaita.toml (uncited)
const SHADOW_ENABLED: bool = true;

/// The opacity of a disabled control.
// `defaults.disabled_opacity`: platform-facts.md (§2.1.6): --disabled-opacity 50%, the only
// disabled mechanism
const DISABLED_OPACITY: f32 = 0.5;

/// The focus ring's width.
// `defaults.focus_ring_width`: 0.7.2 adwaita.toml (uncited)
const FOCUS_RING_WIDTH: f32 = 2.0;

/// The focus ring's offset from the control's edge.
// `defaults.focus_ring_offset`: 0.7.2 adwaita.toml (uncited)
const FOCUS_RING_OFFSET: f32 = -2.0;

/// The focus ring's alpha: libadwaita's `$focus_border_color`, the
/// standalone accent at 50 % (libadwaita 1.10.0 `_colors.scss:260`, `:307`;
/// `_drawing.scss:20`), as 0.7.2's GNOME reader states it for the system's
/// accent (`src/gnome/accent.rs`, `focus_ring`).
const FOCUS_RING_ALPHA: f32 = 0.5;

/// The focus ring's alpha under `Contrast::High`: libadwaita's
/// `$focus_border_color_hc`, the standalone accent at 80 % where it draws
/// the ring at 50 % otherwise (libadwaita 1.10.0 `_colors.scss:261`, `:308`;
/// `_drawing.scss:24-26`), as 0.7.2's GNOME reader states it
/// (`src/gnome/accent.rs`, `focus_ring`). It applies to the default accent
/// too: the ring is `var(--accent-color)`, the standalone colour of whatever
/// `--accent-bg-color` is (libadwaita 1.10.0 `_colors.scss:166-170`).
const FOCUS_RING_ALPHA_HIGH: f32 = 0.8;

/// The icon set.
// `Resolved.icon_set`: 0.7.2 adwaita.toml (uncited)
const ICON_SET: IconSet = IconSet::Freedesktop;

/// The icon theme.
// `Resolved.icon_theme`: 0.7.2 adwaita.toml (uncited)
const ICON_THEME: &str = "Adwaita";

/// A toolbar icon's size.
// `defaults.icon_sizes.toolbar`: 0.7.2 adwaita.toml (uncited)
const ICON_TOOLBAR: f32 = 16.0;

/// A small icon's size.
// `defaults.icon_sizes.small`: 0.7.2 adwaita.toml (uncited)
const ICON_SMALL: f32 = 16.0;

/// A large icon's size.
// `defaults.icon_sizes.large`: 0.7.2 adwaita.toml (uncited)
const ICON_LARGE: f32 = 32.0;

/// A dialog icon's size.
// `defaults.icon_sizes.dialog`: 0.7.2 adwaita.toml (uncited)
const ICON_DIALOG: f32 = 22.0;

/// A panel icon's size.
// `defaults.icon_sizes.panel`: 0.7.2 adwaita.toml (uncited)
const ICON_PANEL: f32 = 20.0;

/// The gap between widgets.
// `layout.widget_gap`: 0.7.2 adwaita.toml (uncited)
const WIDGET_GAP: f32 = 6.0;

/// A container's margin.
// `layout.container_margin`: 0.7.2 adwaita.toml (uncited)
const CONTAINER_MARGIN: f32 = 12.0;

/// A window's margin.
// `layout.window_margin`: 0.7.2 adwaita.toml (uncited)
const WINDOW_MARGIN: f32 = 12.0;

/// The gap between sections.
// `layout.section_gap`: 0.7.2 adwaita.toml (uncited)
const SECTION_GAP: f32 = 18.0;

/// The delay before a tooltip shows.
// `tooltip.show_delay`: platform-facts.md (§2.7): GTK HOVER_TIMEOUT 500 (gtktooltip.c:71-73)
const TOOLTIP_SHOW_MS: u32 = 500;

/// The caret's visible half of a blink.
// `input.caret_blink_on`: platform-facts.md (§2.4): half cursor-blink-time, 1200 by default
// (org.gnome.desktop.interface.gschema.xml.in:70-77 at gsettings-desktop-schemas 51.0;
// gtktext.c:6866-6873 at GTK 4.24.1)
const CARET_BLINK_ON_MS: u32 = 600;

/// The caret's hidden half of a blink.
// `input.caret_blink_off`: platform-facts.md (§2.4): the other half, hidden
const CARET_BLINK_OFF_MS: u32 = 600;

/// How long the caret blinks before it stays visible.
// `input.caret_blink_timeout`: platform-facts.md (§2.4): cursor-blink-timeout, 10 s by default
// (org.gnome.desktop.interface.gschema.xml.in:78-85 at gsettings-desktop-schemas 51.0)
const CARET_BLINK_TIMEOUT_MS: u32 = 10_000;

/// The push button's least width.
// `button.min_width`: 0.7.2 adwaita.toml (uncited)
const BUTTON_MIN_WIDTH: f32 = 64.0;

/// The push button's least height.
// `button.min_height`: 0.7.2 adwaita.toml (uncited)
const BUTTON_MIN_HEIGHT: f32 = 34.0;

/// The push button's padding left and right.
// `button.border.padding.left`, `button.border.padding.right`: platform-facts.md (§2.3):
// Adwaita CSS, a label-only button (.text-button, _buttons.scss:72-75) 17; 5
const BUTTON_PADDING_HORIZONTAL: f32 = 17.0;

/// The push button's padding above and below.
// `button.border.padding.top`, `button.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const BUTTON_PADDING_VERTICAL: f32 = 5.0;

/// The push button's outline width.
// `button.border.line_width`: platform-facts.md §2.3 border.line_width: 0 at rest, a
// $button_color fill with no border; the inset 1px line only under prefers-contrast: more
// (_buttons.scss:28-33)
const BUTTON_BORDER_WIDTH: f32 = 0.0;

/// The gap between a button's icon and its text.
// `button.icon_text_gap`: 0.7.2 adwaita.toml (uncited)
const BUTTON_ICON_TEXT_GAP: f32 = 8.0;

/// The size of a check box's box and of a radio button's circle.
// `checkbox.indicator_width`: platform-facts.md (§2.5): min-width 14 plus padding 3 on each side
const CHECKBOX_INDICATOR_SIZE: f32 = 20.0;

/// The check box's corner radius.
// `checkbox.border.corner_radius`: platform-facts.md §2.5 border.corner_radius: $check_radius 6
// (libadwaita 1.10.0 _checks.scss:1, :67-68)
const CHECKBOX_CORNER_RADIUS: f32 = 6.0;

/// The indicator's padding left and right.
// `checkbox.border.padding.left`, `checkbox.border.padding.right`: platform-facts.md (§2.5):
// Adwaita CSS check { padding: 3px }
const CHECKBOX_PADDING_HORIZONTAL: f32 = 3.0;

/// The indicator's padding above and below.
// `checkbox.border.padding.top`, `checkbox.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const CHECKBOX_PADDING_VERTICAL: f32 = 3.0;

/// The radio button's dot.
// `checkbox.radio_dot_diameter`: platform-facts.md (§2.5): bullet.svg, a circle of radius 4
const RADIO_DOT_SIZE: f32 = 8.0;

/// The check mark's stroke width.
// `checkbox.check_mark_stroke_width`: platform-facts.md (§2.5): check.svg (libadwaita 1.10.0), a
// stroke 2 wide
const CHECK_MARK_STROKE_WIDTH: f32 = 2.0;

/// The gap between the indicator and its label.
// `checkbox.label_gap`: 0.7.2 adwaita.toml (uncited)
const CHECKBOX_LABEL_GAP: f32 = 8.0;

/// A list row's padding left and right (spec §5, `L4`: GNOME's list padding
/// cell is the row's own padding, "plain list=2 (`padding: 2px`)":
/// platform-facts.md (§2.15)).
// `list.border.padding.left`, `list.border.padding.right`: platform-facts.md (§2.15): Adwaita CSS
// plain list context, 2
const LIST_ROW_PADDING_HORIZONTAL: f32 = 2.0;

/// A list row's padding above and below.
// `list.border.padding.top`, `list.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const LIST_ROW_PADDING_VERTICAL: f32 = 2.0;

/// A list header's height.
// `list.header_height`: platform-facts.md (§2.15): the column title button's min-height 24 +
// padding 3 + 3 (_column-view.scss:12-35; _buttons.scss:18-22)
const LIST_HEADER_HEIGHT: f32 = 30.0;

/// The scroll bar's thickness.
// `scrollbar.groove_width`: 0.7.2 adwaita.toml (uncited)
const SCROLLBAR_GROOVE_WIDTH: f32 = 12.0;

/// The handle's thickness.
// `scrollbar.thumb_width`: 0.7.2 adwaita.toml (uncited)
const SCROLLBAR_THUMB_WIDTH: f32 = 8.0;

/// The handle's least length.
// `scrollbar.min_thumb_length`: 0.7.2 adwaita.toml (uncited)
const SCROLLBAR_MIN_THUMB_LENGTH: f32 = 40.0;

/// Whether the scroll bar overlays the content.
// `scrollbar.overlay_mode`: platform-facts.md (§2.8): gsettings overlay-scrolling, default true
// (gsettings-desktop-schemas 51.0 org.gnome.desktop.interface.gschema.xml.in:261-262)
const SCROLLBAR_OVERLAY: bool = true;

/// The tooltip's padding left and right.
// `tooltip.border.padding.left`, `tooltip.border.padding.right`: platform-facts.md (§2.7): Adwaita
// CSS 10; 6
const TOOLTIP_PADDING_HORIZONTAL: f32 = 10.0;

/// The tooltip's padding above and below.
// `tooltip.border.padding.top`, `tooltip.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const TOOLTIP_PADDING_VERTICAL: f32 = 6.0;

/// The tooltip's greatest width.
// `tooltip.max_width`: 0.7.2 adwaita.toml (uncited)
const TOOLTIP_MAX_WIDTH: f32 = 360.0;

/// The text field's least height.
// `input.min_height`: 0.7.2 adwaita.toml (uncited)
const INPUT_MIN_HEIGHT: f32 = 34.0;

/// The text field's padding left and right.
// `input.border.padding.left`, `input.border.padding.right`: platform-facts.md (§2.4): Adwaita CSS
// 9; 0 (CSS sets no vertical padding)
const INPUT_PADDING_HORIZONTAL: f32 = 9.0;

/// The text field's padding above and below.
// `input.border.padding.top`, `input.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const INPUT_PADDING_VERTICAL: f32 = 0.0;

/// The text field's outline width, in every state: 0.7.2 has one line width
/// for its border.
// `input.border.line_width`: platform-facts.md §2.4 border.line_width: 0 at rest, a $button_color
// fill with no border; the inset 1px line only under prefers-contrast: more (_entries.scss:9-15)
const INPUT_BORDER_WIDTH: f32 = 0.0;

/// The text area's padding left and right.
// `text_area.border.padding.left`, `text_area.border.padding.right`: platform-facts.md (§2.29):
// textview sets no padding, GtkTextView's margins default to 0
const TEXT_AREA_PADDING_HORIZONTAL: f32 = 0.0;

/// The text area's padding above and below.
// `text_area.border.padding.top`, `text_area.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const TEXT_AREA_PADDING_VERTICAL: f32 = 0.0;

/// The link's decoration.
// `link.underline_enabled`: 0.7.2 adwaita.toml (uncited)
const LINK_DECORATION: DecorationKind = DecorationKind::Underline;

/// The slider's groove thickness.
// `slider.track_height`: 0.7.2 adwaita.toml (uncited)
const SLIDER_TRACK_HEIGHT: f32 = 10.0;

/// The slider's handle diameter.
// `slider.thumb_diameter`: 0.7.2 adwaita.toml (uncited)
const SLIDER_THUMB_DIAMETER: f32 = 20.0;

/// A tick mark's length.
// `slider.tick_mark_length`: 0.7.2 adwaita.toml (uncited)
const SLIDER_TICK_LENGTH: f32 = 4.0;

/// The progress bar's groove thickness.
// `progress_bar.track_height`: 0.7.2 adwaita.toml (uncited)
const PROGRESS_BAR_TRACK_HEIGHT: f32 = 8.0;

/// The progress bar's least length.
// `progress_bar.min_width`: 0.7.2 adwaita.toml (uncited)
const PROGRESS_BAR_MIN_WIDTH: f32 = 80.0;

/// The progress bar's outline width.
// `progress_bar.border.line_width`: platform-facts.md (§2.10): no border (an inset 1px shadow
// only under prefers-contrast: more)
const PROGRESS_BAR_BORDER_WIDTH: f32 = 0.0;

/// The progress bar's padding left and right.
// `progress_bar.border.padding.left`, `progress_bar.border.padding.right`: platform-facts.md
// (§2.10): Adwaita CSS 0; 0, the progress fills the trough (_scale.scss:12-17,
// _progress-bar.scss:6, :27-31)
const PROGRESS_BAR_PADDING_HORIZONTAL: f32 = 0.0;

/// The progress bar's padding above and below.
// `progress_bar.border.padding.top`, `progress_bar.border.padding.bottom`: 0.7.2 adwaita.toml
// (uncited)
const PROGRESS_BAR_PADDING_VERTICAL: f32 = 0.0;

/// The splitter's divider width.
// `splitter.divider_width`: 0.7.2 adwaita.toml (uncited)
const SPLITTER_DIVIDER_WIDTH: f32 = 1.0;

/// The spinner's diameter.
// `spinner.diameter`: 0.7.2 adwaita.toml (uncited)
const SPINNER_DIAMETER: f32 = 16.0;

/// The spinner's least diameter.
// `spinner.min_diameter`: 0.7.2 adwaita.toml (uncited)
const SPINNER_MIN_DIAMETER: f32 = 16.0;

/// The spinner's stroke width.
// `spinner.stroke_width`: 0.7.2 adwaita.toml (uncited)
const SPINNER_STROKE_WIDTH: f32 = 2.0;

/// A menu row's height, exact.
// `menu.row_height`: 0.7.2 adwaita.toml (uncited)
const MENU_ROW_HEIGHT: f32 = 32.0;

/// The gap between a menu item's icon and its text.
// `menu.icon_text_gap`: 0.7.2 adwaita.toml (uncited)
const MENU_ICON_TEXT_GAP: f32 = 8.0;

/// A menu item's padding left and right.
// `menu.border.padding.left`, `menu.border.padding.right`: platform-facts.md (§2.6): Adwaita CSS
// 12 ($menu_padding); 0
const MENU_ITEM_PADDING_HORIZONTAL: f32 = 12.0;

/// A menu item's padding above and below.
// `menu.border.padding.top`, `menu.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const MENU_ITEM_PADDING_VERTICAL: f32 = 0.0;

/// A tab's least width.
// `tab.min_width`: 0.7.2 adwaita.toml (uncited)
const TAB_MIN_WIDTH: f32 = 64.0;

/// A tab's least height.
// `tab.min_height`: 0.7.2 adwaita.toml (uncited)
const TAB_MIN_HEIGHT: f32 = 30.0;

/// The gap between tabs.
// `tab.item_gap`: platform-facts.md (§2.11): GtkNotebook places each tab at the previous one's
// end (gtknotebook.c:5294)
const TAB_ITEM_GAP: f32 = 0.0;

/// A tab's padding left and right.
// `tab.border.padding.left`, `tab.border.padding.right`: platform-facts.md (§2.11): Adwaita CSS
// 12; 3
const TAB_PADDING_HORIZONTAL: f32 = 12.0;

/// A tab's padding above and below.
// `tab.border.padding.top`, `tab.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const TAB_PADDING_VERTICAL: f32 = 3.0;

/// The selected tab's indicator thickness.
// `tab.active_indicator_width`: 0.7.2 adwaita.toml (uncited)
const TAB_INDICATOR_WIDTH: f32 = 4.0;

/// The side of the tab the indicator lies along.
// `tab.active_indicator_side`: 0.7.2 adwaita.toml (uncited)
const TAB_INDICATOR_SIDE: TabIndicatorSide = TabIndicatorSide::Bottom;

/// The gap between toolbar items.
// `toolbar.item_gap`: platform-facts.md (§2.13): Adwaita CSS .toolbar border-spacing 6
const TOOLBAR_ITEM_GAP: f32 = 6.0;

/// The toolbar's outline width.
// `toolbar.border.line_width`: platform-facts.md §2.13 border.line_width, border.shadow_enabled:
// .toolbar sets no border and casts no shadow (_toolbars.scss:1-110)
const TOOLBAR_BORDER_WIDTH: f32 = 0.0;

/// The toolbar's padding left and right.
// `toolbar.border.padding.left`, `toolbar.border.padding.right`: platform-facts.md (§2.13):
// Adwaita CSS .toolbar { padding: 6px }
const TOOLBAR_PADDING_HORIZONTAL: f32 = 6.0;

/// The toolbar's padding above and below.
// `toolbar.border.padding.top`, `toolbar.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const TOOLBAR_PADDING_VERTICAL: f32 = 6.0;

/// The status bar's padding left and right.
// `status_bar.border.padding.left`, `status_bar.border.padding.right`: platform-facts.md (§2.14):
// Adwaita CSS statusbar { padding: 6px 10px }
const STATUS_BAR_PADDING_HORIZONTAL: f32 = 10.0;

/// The status bar's padding above and below.
// `status_bar.border.padding.top`, `status_bar.border.padding.bottom`: 0.7.2 adwaita.toml
// (uncited)
const STATUS_BAR_PADDING_VERTICAL: f32 = 6.0;

/// The sidebar's padding above.
// `sidebar.border.padding.top`: platform-facts.md (§2.12): 6 top / 4 bottom, `padding-top:
// $menu_margin; padding-bottom: $menu_margin - 2px`, the last row's 2px bottom margin making up
// the difference (_sidebars.scss:149-150, :193)
const SIDEBAR_PADDING_TOP: f32 = 6.0;

/// The sidebar's padding left and right.
// `sidebar.border.padding.left`, `sidebar.border.padding.right`: platform-facts.md (§2.12): 6,
// `.navigation-sidebar > row { margin: 0 $menu_margin 2px }`, $menu_margin = 6; the list itself
// has no side padding (libadwaita 1.10.0 _sidebars.scss:189-193, _common.scss:11)
const SIDEBAR_PADDING_HORIZONTAL: f32 = 6.0;

/// The sidebar's padding below.
// `sidebar.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const SIDEBAR_PADDING_BOTTOM: f32 = 4.0;

/// The popover's outline width.
// `popover.border.line_width`: platform-facts.md (§2.16): 1px; drawn as a border it puts the
// content 9px from the edge, as the 1px outline outside libadwaita's 8px padding does
const POPOVER_BORDER_WIDTH: f32 = 1.0;

/// The popover's padding left and right.
// `popover.border.padding.left`, `popover.border.padding.right`: platform-facts.md (§2.16):
// Adwaita CSS popover > contents { padding: 8px }
const POPOVER_PADDING_HORIZONTAL: f32 = 8.0;

/// The popover's padding above and below.
// `popover.border.padding.top`, `popover.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const POPOVER_PADDING_VERTICAL: f32 = 8.0;

/// The switch's track width.
// `switch.track_width`: 0.7.2 adwaita.toml (uncited)
const SWITCH_TRACK_WIDTH: f32 = 46.0;

/// The switch's track height.
// `switch.track_height`: 0.7.2 adwaita.toml (uncited)
const SWITCH_TRACK_HEIGHT: f32 = 26.0;

/// The switch's track corner radius.
// `switch.track_radius`: 0.7.2 adwaita.toml (uncited)
const SWITCH_TRACK_RADIUS: f32 = 14.0;

/// The switch's thumb diameter, on and off (0.7.2:
/// `switch.unchecked_thumb_diameter` is `thumb_diameter` where unstated).
// `switch.thumb_diameter`: 0.7.2 adwaita.toml (uncited)
const SWITCH_THUMB_DIAMETER: f32 = 20.0;

/// The least gap between a switch row's title and its switch, which the
/// title's expanding box widens to the row's free width.
// `switch.label_gap`: the header box's `border-spacing: 6px` (_lists.scss:132), between the
// title box, `hexpand` True (adw-action-row.ui:40-43), and the suffixes box, the switch's
// (adw-action-row.ui:91-94; adw-switch-row.c:161), at libadwaita 1.10.0
const SWITCH_LABEL_GAP: f32 = 6.0;

/// The dialog's least width.
// `dialog.min_width`: 0.7.2 adwaita.toml (uncited)
const DIALOG_MIN_WIDTH: f32 = 300.0;

/// The dialog's greatest width.
// `dialog.max_width`: 0.7.2 adwaita.toml (uncited)
const DIALOG_MAX_WIDTH: f32 = 372.0;

/// The dialog's least height.
// `dialog.min_height`: 0.7.2 adwaita.toml (uncited)
const DIALOG_MIN_HEIGHT: f32 = 140.0;

/// The dialog's greatest height.
// `dialog.max_height`: 0.7.2 adwaita.toml (uncited)
const DIALOG_MAX_HEIGHT: f32 = 600.0;

/// The gap between the dialog's buttons.
// `dialog.button_gap`: 0.7.2 adwaita.toml (uncited)
const DIALOG_BUTTON_GAP: f32 = 12.0;

/// The dialog's icon size.
// `dialog.icon_size`: 0.7.2 adwaita.toml (uncited)
const DIALOG_ICON_SIZE: f32 = 32.0;

/// The order of the dialog's buttons.
// `dialog.button_order`: 0.7.2 adwaita.toml (uncited)
const DIALOG_BUTTON_ORDER: DialogButtonOrder = DialogButtonOrder::PrimaryRight;

/// The dialog's entrance.
// `dialog.entrance`: platform-facts.md (§2.22): the floating AdwDialog fades in while scaling 0.8
// → 1 (adw-floating-sheet.c:22, :34-35, :220-226)
const DIALOG_ENTRANCE: DialogEntranceStyle = DialogEntranceStyle::ScaleFade;

/// The dialog's padding above.
// `dialog.border.padding.top`: 0.7.2 adwaita.toml (uncited)
const DIALOG_PADDING_TOP: f32 = 32.0;

/// The dialog's padding left and right.
// `dialog.border.padding.left`, `dialog.border.padding.right`: platform-facts.md (§2.22): 24; 32
// top / 24 bottom
const DIALOG_PADDING_HORIZONTAL: f32 = 24.0;

/// The dialog's padding below.
// `dialog.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const DIALOG_PADDING_BOTTOM: f32 = 24.0;

/// The dialog's corner radius.
// `dialog.border.corner_radius`: 0.7.2 adwaita.toml (uncited)
const DIALOG_CORNER_RADIUS: f32 = 18.0;

/// The dialog title's size, in points.
// `dialog.title_font.size`, `dialog.title_font.defined_size`: platform-facts.md (§2.22): .title-2,
// ≈15pt ExtraBold
const DIALOG_TITLE_SIZE_PT: f32 = 15.0;

/// The dialog title's weight.
// `dialog.title_font.weight`: 0.7.2 adwaita.toml (uncited)
const DIALOG_TITLE_WEIGHT: u16 = 800;

/// The window title's weight.
// `window.title_bar_font.weight`: 0.7.2 adwaita.toml (uncited)
const WINDOW_TITLE_WEIGHT: u16 = 700;

/// The combo box's least width.
// `combo_box.min_width`: 0.7.2 adwaita.toml (uncited)
const COMBO_BOX_MIN_WIDTH: f32 = 120.0;

/// The combo box's least height.
// `combo_box.min_height`: 0.7.2 adwaita.toml (uncited)
const COMBO_BOX_MIN_HEIGHT: f32 = 34.0;

/// The combo box's arrow size.
// `combo_box.arrow_icon_size`: 0.7.2 adwaita.toml (uncited)
const COMBO_BOX_ARROW_SIZE: f32 = 16.0;

/// The combo box's outline width.
// `combo_box.border.line_width`: platform-facts.md §2.24 border.line_width: 0 at rest, the
// button's (§2.3); the inset 1px line only under prefers-contrast: more (_buttons.scss:28-33)
const COMBO_BOX_BORDER_WIDTH: f32 = 0.0;

/// The combo box's padding left and right.
// `combo_box.border.padding.left`, `combo_box.border.padding.right`: platform-facts.md (§2.24):
// Adwaita CSS base button rule, as the dropdown's button has a box child: 10; 5
const COMBO_BOX_PADDING_HORIZONTAL: f32 = 10.0;

/// The combo box's padding above and below.
// `combo_box.border.padding.top`, `combo_box.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const COMBO_BOX_PADDING_VERTICAL: f32 = 5.0;

/// A segment's height.
// `segmented_control.segment_height`: 0.7.2 adwaita.toml (uncited)
const SEGMENT_HEIGHT: f32 = 28.0;

/// The separator between segments.
// `segmented_control.separator_width`: 0.7.2 adwaita.toml (uncited)
const SEGMENT_SEPARATOR_WIDTH: f32 = 1.0;

/// A segment's padding left and right.
// `segmented_control.border.padding.left`, `segmented_control.border.padding.right`:
// platform-facts.md (§2.25): AdwToggleGroup text toggle, Adwaita CSS 11 − 3 = 8; 0 (no rule pads a
// toggle vertically)
const SEGMENT_PADDING_HORIZONTAL: f32 = 8.0;

/// A segment's padding above and below.
// `segmented_control.border.padding.top`, `segmented_control.border.padding.bottom`: 0.7.2
// adwaita.toml (uncited)
const SEGMENT_PADDING_VERTICAL: f32 = 0.0;

/// The card's corner radius.
// `card.border.corner_radius`: 0.7.2 adwaita.toml (uncited)
const CARD_CORNER_RADIUS: f32 = 12.0;

/// The expander header's least height.
// `expander.header_height`: 0.7.2 adwaita.toml (uncited)
const EXPANDER_HEADER_HEIGHT: f32 = 50.0;

/// The expander's arrow size.
// `expander.arrow_icon_size`: 0.7.2 adwaita.toml (uncited)
const EXPANDER_ARROW_SIZE: f32 = 16.0;

/// The side of the header the expander's arrow is on.
// `expander.arrow_side`: platform-facts.md (§2.27): AdwExpanderRow, the arrow at the row's end, 3 +
// 6 from the content before it, the nested list unindented, the whole a boxed-list card
const EXPANDER_ARROW_SIDE: ArrowSide = ArrowSide::Trailing;

/// The gap between the expander's arrow and its title.
// `expander.arrow_gap`: 0.7.2 adwaita.toml (uncited)
const EXPANDER_ARROW_GAP: f32 = 9.0;

/// The indent of the expander's content.
// `expander.content_indent`: 0.7.2 adwaita.toml (uncited)
const EXPANDER_CONTENT_INDENT: f32 = 0.0;

/// Whether an expander draws its frame.
// `expander.frame_enabled`: 0.7.2 adwaita.toml (uncited)
const EXPANDER_FRAME_ENABLED: bool = true;

/// The expander's outline width.
// `expander.border.line_width`: 0.7.2 adwaita.toml (uncited)
const EXPANDER_BORDER_WIDTH: f32 = 1.0;

/// The expander's corner radius.
// `expander.border.corner_radius`: platform-facts.md §2.27 border.corner_radius: $card_radius 12 at
// the boxed list's outer corners, 0 between rows (_lists.scss:533-553, _common.scss:9); one radius
// carries a lone collapsed row, which is first and last
const EXPANDER_CORNER_RADIUS: f32 = 12.0;

/// The expander header's padding left and right.
// `expander.border.padding.left`, `expander.border.padding.right`: platform-facts.md (§2.27):
// AdwExpanderRow header, Adwaita CSS row 2 + header margin 12 = 14; row 2 + title margin 6 = 8
const EXPANDER_PADDING_HORIZONTAL: f32 = 14.0;

/// The expander header's padding above and below.
// `expander.border.padding.top`, `expander.border.padding.bottom`: 0.7.2 adwaita.toml (uncited)
const EXPANDER_PADDING_VERTICAL: f32 = 8.0;

/// The state precedence of a part with rules for pressed and hover but none
/// for disabled: pressed over hover.
const PRESSED_HOVER: &[&[WidgetState]] = &[&[WidgetState::PRESSED, WidgetState::HOVER]];

/// The font DPI a point size converts at: 0.7.2's on every desktop but
/// macOS (`ResolutionContext::from_system`, `src/resolve/context.rs`; spec
/// §4, `Typography::font_dpi`).
const FONT_DPI: f32 = 96.0;

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

/// `c` at `alpha`, held in 8 bits, as 0.7.2 holds every colour
/// (`Rgba::from_f32`: each channel clamped, times 255, rounded).
fn translucent(c: Color, alpha: f32) -> Color {
    let [r, g, b, a] = c.with_alpha(alpha).to_srgb8();
    Color::from_srgb8(r, g, b, a)
}

/// libadwaita's `.caption`: 82 % of the base font, platform-facts.md
/// (§1.4.1), the size of `typography.caption` under the system's UI font (spec
/// §8, "The GNOME part").
const CAPTION_FACTOR: f32 = 0.82;

/// libadwaita's `.title-2`: 136 % of the base font, platform-facts.md
/// (§1.4.1), the size of `typography.title1` and of the dialog's title under
/// the system's UI font.
const TITLE_2_FACTOR: f32 = 1.36;

/// libadwaita's `.title-1`: 181 % of the base font, platform-facts.md
/// (§1.4.1), the size of `typography.display` under the system's UI font.
const TITLE_1_FACTOR: f32 = 1.81;

/// `size` times `factor`, in `size`'s unit.
fn scaled(size: FontSize, factor: f32) -> FontSize {
    match size {
        FontSize::Pt(v) => FontSize::Pt(v * factor),
        FontSize::Px(v) => FontSize::Px(v * factor),
    }
}

/// A title's size: the system's UI font size `ui_size` times `factor`, in
/// its unit, where given, else the preset's `preset_pt` points (spec §8,
/// "The GNOME part"; 0.7.2's `gnome::apply_type_scale`).
fn title_size(ui_size: Option<FontSize>, factor: f32, preset_pt: f32) -> FontSize {
    ui_size.map_or(FontSize::Pt(preset_pt), |size| scaled(size, factor))
}

/// Lays the system's accent `accent` over the scheme's colours `c` for
/// `scheme` (spec §8, "The GNOME part", *Derived by the `adwaita`
/// builder*; 0.7.2's `gnome::apply_gnome_accent`): it is the accent's fill,
/// the selection's background, the tab's active indicator and the info
/// status's fill, and the focus ring's colour, which the focused text
/// field's outline takes too, is libadwaita's standalone colour of the
/// accent for `scheme` ([`adw_standalone`]), clamped and held in 8 bits as
/// 0.7.2 holds it, at [`FOCUS_RING_ALPHA`] ([`roles`] takes it to
/// [`FOCUS_RING_ALPHA_HIGH`] under `Contrast::High`).
fn lay_accent_over(c: &mut SchemeColors, accent: Color, scheme: Scheme) {
    c.accent = accent;
    let [r, g, b, a] = adw_standalone([accent.r, accent.g, accent.b, accent.a], scheme);
    c.focus_ring = translucent(Color::new(r, g, b, a), FOCUS_RING_ALPHA);
}

/// Lays the palette `p` over the scheme's colours `c` for `scheme`: each
/// slot whose libadwaita colour has its name replaces that colour, and the
/// accent is laid as the system's is ([`lay_accent_over`]), so the focus
/// ring is derived again from it, as the module documentation lists; the
/// slots that state only their roles are read by [`roles`].
fn lay_palette_over(c: &mut SchemeColors, p: &Palette, scheme: Scheme) {
    c.window_background = p.window_bg;
    c.view_background = p.view_bg;
    c.title_bar_background = Some(p.header_bg);
    c.sidebar_background = p.sidebar_bg;
    c.card_background = Some(p.surface_bg);
    c.popover_background = p.surface_bg;
    c.tooltip_background = p.tooltip_bg;
    c.window_foreground = p.foreground;
    c.row_foreground = p.foreground;
    c.tooltip_foreground = p.foreground;
    c.window_foreground_muted = p.foreground_muted;
    c.window_border = p.border;
    c.link = p.link;
    c.error = p.error;
    c.warning = p.warning;
    c.success = p.success;
    lay_accent_over(c, p.accent, scheme);
}

// ---------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------

/// The contrast levels the preset defines (spec §1, "Axis values without
/// data"): `Normal`, and `High` for the focus ring's alpha, as the module
/// documentation says.
pub(crate) const CONTRAST_LEVELS: &[Contrast] = &[Contrast::Normal, Contrast::High];

/// The theme of `adwaita` for `ctx.scheme` and `ctx.contrast`, from the
/// preset's constants with the system's settings `inputs.system` laid over
/// them (spec §8, "The GNOME part"): the accent ([`lay_accent_over`]), the
/// fonts ([`typography`]; the title bar's, [`window`]), the overlay scroll
/// bars, the icon theme and the caret's blink; then the active palette
/// ([`lay_palette_over`]) and the explicit accent, each over the layer below
/// it (spec §6, resolution step 2).
pub(crate) fn resolve(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let system = inputs.system;
    let mut scheme_colors = match ctx.scheme {
        Scheme::Light => LIGHT,
        Scheme::Dark => DARK,
    };
    if let Some(accent) = system.and_then(|s| s.accent) {
        lay_accent_over(&mut scheme_colors, accent, ctx.scheme);
    }
    if let Some(p) = inputs.palette {
        lay_palette_over(&mut scheme_colors, p, ctx.scheme);
    }
    if let Some(accent) = inputs.accent {
        lay_accent_over(&mut scheme_colors, accent, ctx.scheme);
    }
    let c = &scheme_colors;
    let colors = roles(c, ctx.contrast, inputs.palette, inputs.accent.is_some());
    let typography = typography(c, system, ctx.text_scale);
    let units = FontUnits {
        dpi: typography.font_dpi,
        text_scale: ctx.text_scale,
    };
    let ui_size = system.and_then(|s| s.ui_font.as_ref()?.size);
    // The system's icon theme, unless the system was read in the other
    // scheme (spec §6, resolution step 2); the sizes stay the preset's.
    let icon_theme = system
        .filter(|s| s.scheme.is_none_or(|scheme| scheme == ctx.scheme))
        .and_then(|s| s.icon_theme.clone())
        .unwrap_or_else(|| Arc::from(ICON_THEME));
    // No source the builder reads states a policy of GNOME's for reduced motion: `Reduced`
    // runs as `Full`.
    let motion = motion(ctx.motion, None);
    let layout = LayoutMetrics {
        widget_gap: Some(WIDGET_GAP),
        container_margin: Some(CONTAINER_MARGIN),
        window_margin: Some(WINDOW_MARGIN),
        section_gap: Some(SECTION_GAP),
        corner_radius: CORNER_RADIUS,
        corner_radius_lg: CORNER_RADIUS_LG,
        border_width: BORDER_WIDTH,
        border_opacity: BORDER_OPACITY,
        shadow_enabled: SHADOW_ENABLED,
        cursor_size_px: None,
        cursor_theme: None,
    };
    ResolvedTheme {
        name: Arc::from(Preset::Adwaita.display_name()),
        preset: Preset::Adwaita,
        context: ctx.clone(),
        icons: IconThemeSpec {
            set: ICON_SET,
            theme: Some(icon_theme),
            // 0.7.2's `Resolved::icon_theme_explicit`: the TOML states the theme
            // (`src/model/resolved.rs`).
            theme_explicit: true,
            sizes: IconSizes::new(
                Some(ICON_TOOLBAR),
                Some(ICON_SMALL),
                Some(ICON_LARGE),
                Some(ICON_DIALOG),
                Some(ICON_PANEL),
            ),
        },
        timings: UiTimings {
            tooltip_show_ms: Some(TOOLTIP_SHOW_MS),
            tooltip_hide_ms: None,
            tooltip_reshow_ms: None,
            double_click_ms: None,
            drag_threshold_px: None,
            scrollbar_auto_hide_ms: None,
            menu_hover_open_ms: None,
            key_repeat_delay_ms: None,
            key_repeat_interval_ms: None,
        },
        motion_tokens: motion_tokens(
            MotionTokens {
                fast: None,
                standard: None,
                emphasized: None,
                spring_spatial: None,
                spring_effects: None,
                theme_change: None,
            },
            motion,
        ),
        // 0.7.2's `defaults.focus_ring_*`: a ring; `focus_ring_keyboard_only` unstated.
        focus_ring: FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(FOCUS_RING_WIDTH, colors.focus_ring),
            Stroke::None,
            FOCUS_RING_OFFSET,
            None,
            None,
        ),
        caret: CaretSpec {
            width: None,
            blink_on_ms: system
                .and_then(|s| s.caret_blink_on_ms)
                .or(Some(CARET_BLINK_ON_MS)),
            blink_off_ms: system
                .and_then(|s| s.caret_blink_off_ms)
                .or(Some(CARET_BLINK_OFF_MS)),
            blink_timeout_ms: system
                .and_then(|s| s.caret_blink_timeout_ms)
                .or(Some(CARET_BLINK_TIMEOUT_MS)),
            shape: None,
        },
        elevations: Arc::from([]),
        button: button(c, &colors, &typography, &layout),
        checkbox: checkbox(c, &colors, &typography, &layout),
        radio: radio(c, &colors, &typography, &layout),
        scrollbar: scrollbar(
            c,
            &colors,
            system
                .and_then(|s| s.overlay_scrollbars)
                .unwrap_or(SCROLLBAR_OVERLAY),
        ),
        list: list(c, &colors, &typography, &layout),
        tab: tab(c, &colors, &typography),
        menu: menu(c, &colors, &typography, ctx.direction),
        menu_bar: menu_bar(&colors),
        sidebar: sidebar(c, &colors, &typography, &layout, ctx.direction),
        toolbar: toolbar(c, &colors, &typography, &layout),
        status_bar: status_bar(&colors, &typography, &layout, ctx.direction),
        tooltip: tooltip(&colors, &typography, &layout),
        window: window(
            c,
            &colors,
            &typography,
            system.and_then(|s| s.title_bar_font.as_ref()),
            units,
            &layout,
            ctx.window_active,
        ),
        input: input(
            c,
            &colors,
            &typography,
            &layout,
            Edges::symmetric(INPUT_PADDING_VERTICAL, INPUT_PADDING_HORIZONTAL),
            Some(INPUT_MIN_HEIGHT),
        ),
        text_area: input(
            c,
            &colors,
            &typography,
            &layout,
            Edges::symmetric(TEXT_AREA_PADDING_VERTICAL, TEXT_AREA_PADDING_HORIZONTAL),
            None,
        ),
        link: link(&colors, &typography),
        slider: slider(c, &colors),
        progress_bar: progress_bar(&colors, &layout),
        splitter: splitter(c),
        separator: separator(c, &layout),
        spinner: spinner(&colors),
        popover: popover(c, &typography, &layout),
        switch: switch(c, &colors, &typography),
        dialog: dialog(
            &colors,
            &typography,
            title_size(ui_size, TITLE_2_FACTOR, DIALOG_TITLE_SIZE_PT),
            units,
            &layout,
            ctx.direction,
            motion,
        ),
        combo_box: combo_box(c, &colors, &typography, &layout, ctx.direction),
        segmented_control: segmented_control(c, &colors, &typography, &layout, ctx.direction),
        card: card(c, &colors),
        expander: expander(c, &typography, ctx.direction),
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
        colors,
        typography,
        layout,
    }
}

/// The colour roles of one scheme from its colours `c`, at `contrast`. The
/// window's surface is libadwaita's window colours, the view's its view
/// background, the tooltip's the tooltip's own colours; the selection and the
/// accent are the accent (0.7.2: `selection_inactive_background` and
/// `selection_inactive_text_color` are the selection's where unstated, and
/// the text selection's colours are the selection's,
/// `src/resolve/inheritance.rs`, `resolve_defaults_internal`); the caret and
/// the disabled text are the window's text (`input.caret_color`,
/// `defaults.disabled_text_color`); the info status is the accent. Under
/// `Contrast::High` the focus ring is its colour at [`FOCUS_RING_ALPHA_HIGH`].
///
/// The active palette `palette`, whose colours `c` holds already, states
/// the roles its slots name (spec §7, "The roles a palette states"): the
/// view's, the header's, the sidebar's, the card's, the dialog's and the
/// popover's surfaces with its one text colour, the window's divider, the
/// caret, the four status colours' origin, and, unless an explicit accent
/// (`explicit_accent`) replaces them, the accent's text, the selection's
/// pair, which the unfocused and the text selection take too, and the info
/// status's fill.
fn roles(
    c: &SchemeColors,
    contrast: Contrast,
    palette: Option<&Palette>,
    explicit_accent: bool,
) -> ColorRoles {
    let origin = if palette.is_some() {
        RoleOrigin::Custom
    } else {
        RoleOrigin::Native
    };
    let status = |fill, foreground| StatusColors {
        fill,
        foreground,
        standalone: None,
        origin,
    };
    let unstated = SurfaceColors::default();
    let surface = |background: fn(&Palette) -> Color| {
        palette.map_or(unstated, |p| {
            SurfaceColors::new(Some(background(p)), Some(p.foreground), None, None, None)
        })
    };
    let accent_slots = palette.filter(|_| !explicit_accent);
    let (selection_background, selection_foreground) = accent_slots
        .map_or((c.accent, c.accent_foreground), |p| {
            (p.selection_bg, p.selection_fg)
        });
    ColorRoles {
        window: SurfaceColors::new(
            Some(c.window_background),
            Some(c.window_foreground),
            Some(c.window_foreground_muted),
            Some(c.window_border),
            palette.map(|p| p.divider),
        ),
        view: SurfaceColors::new(
            Some(c.view_background),
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
        tooltip: SurfaceColors::new(
            Some(c.tooltip_background),
            Some(c.tooltip_foreground),
            None,
            None,
            None,
        ),
        osd: unstated,
        selection: SelectionColors::new(
            selection_background,
            selection_foreground,
            Some(selection_background),
            Some(selection_foreground),
        ),
        text_selection: SelectionColors::new(
            selection_background,
            selection_foreground,
            None,
            None,
        ),
        find_highlight: None,
        scrim: None,
        accent: AccentColors {
            fill: c.accent,
            foreground: accent_slots.map_or(c.accent_foreground, |p| p.accent_fg),
            standalone: None,
        },
        focus_ring: match contrast {
            Contrast::High => translucent(c.focus_ring, FOCUS_RING_ALPHA_HIGH),
            Contrast::Normal | Contrast::Increased => c.focus_ring,
        },
        caret: palette.map_or(c.window_foreground, |p| p.caret),
        link: c.link,
        // 0.7.2: `link.visited_text_color` is `defaults.link_color` where unstated.
        link_visited: None,
        disabled_text: c.window_foreground,
        shadow: c.shadow,
        disabled_opacity: DISABLED_OPACITY,
        error: status(c.error, c.error_foreground),
        warning: status(c.warning, c.warning_foreground),
        success: status(c.success, c.success_foreground),
        info: status(
            accent_slots.map_or(c.accent, |p| p.info),
            c.accent_foreground,
        ),
    }
}

/// How a font size becomes logical pixels: the font DPI and the text scale
/// (spec §6, resolution step 2).
#[derive(Clone, Copy)]
struct FontUnits {
    /// The font DPI.
    dpi: f32,
    /// The text scale, `effective.text_scale`.
    text_scale: f32,
}

/// A font size in logical pixels: converted at the font DPI, times the text
/// scale, linear on GNOME (spec §6, resolution step 2), not rounded (0.7.2
/// rounds only where `font_whole_pixels` is stated, `src/resolve/validate.rs`).
fn font_px(size: FontSize, units: FontUnits) -> f32 {
    linear_font_px(size, units.dpi, units.text_scale, None)
}

/// The line height of a style of `size`, in logical pixels: the multiplier
/// times the size, in its unit, converted at the font DPI (0.7.2's
/// `resolve_text_scale_entry`, `src/resolve/inheritance.rs`, then its
/// conversion in `validate`; spec §4, `Typography`), times the text scale.
fn line_height_px(size: FontSize, units: FontUnits) -> f32 {
    let size = match size {
        FontSize::Pt(v) => FontSize::Pt(LINE_HEIGHT * v),
        FontSize::Px(v) => FontSize::Px(LINE_HEIGHT * v),
    };
    font_px(size, units)
}

/// A font as 0.7.2 resolves it: its family, size, weight and normal style
/// stated (`src/resolve/validate_helpers.rs`, `require_font`), in `color`.
fn font(family: &str, size: FontSize, weight: u16, color: Color, units: FontUnits) -> TextStyle {
    TextStyle {
        family: Arc::from(family),
        size_px: font_px(size, units),
        defined_size: Some(size),
        weight,
        style: FontStyle::Normal,
        color,
        stated: TextStated::FAMILY
            | TextStated::SIZE_PX
            | TextStated::WEIGHT
            | TextStated::STYLE
            | TextStated::COLOR,
        ..TextStyle::default()
    }
}

/// A text-scale entry as 0.7.2 resolves it: its size, weight and line height
/// stated, nothing else (0.7.2's `TextScaleEntry`).
fn text_scale_entry(size: FontSize, weight: u16, units: FontUnits) -> TextStyle {
    TextStyle {
        size_px: font_px(size, units),
        defined_size: Some(size),
        weight,
        line_height: line_height_px(size, units),
        stated: TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::LINE_HEIGHT,
        ..TextStyle::default()
    }
}

/// The text styles. The fonts' colour is the window's foreground (0.7.2:
/// `defaults.font.color` is `defaults.text_color`, and
/// `defaults.mono_font`'s is `defaults.font`'s, where unstated:
/// `resolve_defaults_internal`). Each text-scale entry takes the UI font's
/// size or weight where the TOML states none (0.7.2's
/// `resolve_text_scale_entry`): the caption its weight, the section heading
/// (`headline`) its size; the dialog title is `title1` and the display style
/// `display` (spec §4). The body's line height follows the text-scale rule
/// (spec §4). Sizes convert at the system's font DPI where given, else at
/// [`FONT_DPI`], and scale by `text_scale` (spec §6, resolution step 2); each
/// keeps its unscaled size as its `defined_size`.
///
/// The system's fonts in `system` are laid over the preset's (spec §8, "The
/// GNOME part"; 0.7.2's `gnome::build_gnome_variant_pure`, merged over the
/// preset): the UI font over the body, and with it what takes the body's
/// size or weight; the monospace font over the monospace style. Where the UI
/// font's size is given, the caption, `title1` and `display` are that size
/// times [`CAPTION_FACTOR`], [`TITLE_2_FACTOR`] and [`TITLE_1_FACTOR`], in its
/// unit, their weights the preset's ([`title_size`]).
fn typography(c: &SchemeColors, system: Option<&SystemOverrides>, text_scale: f32) -> Typography {
    let dpi = system.and_then(|s| s.font_dpi).unwrap_or(FONT_DPI);
    let units = FontUnits { dpi, text_scale };
    let text = c.window_foreground;
    let ui = system.and_then(|s| s.ui_font.as_ref());
    let ui_size = ui.and_then(|f| f.size);
    let body_size = ui_size.unwrap_or(FontSize::Pt(FONT_SIZE_PT));
    let mut body = laid_over(
        font(
            FONT_FAMILY,
            FontSize::Pt(FONT_SIZE_PT),
            FONT_WEIGHT,
            text,
            units,
        ),
        ui,
        units,
    );
    body.line_height = line_height_px(body_size, units);
    body.stated |= TextStated::LINE_HEIGHT;
    // The monospace font's line height, which no source states: the multiplier times its size
    // (spec §4, `Typography`: `body` and `monospace` follow a slot's rule).
    let mono = system.and_then(|s| s.monospace_font.as_ref());
    let mono_size = mono
        .and_then(|f| f.size)
        .unwrap_or(FontSize::Pt(MONO_SIZE_PT));
    let mut monospace = laid_over(
        font(
            MONO_FAMILY,
            FontSize::Pt(MONO_SIZE_PT),
            MONO_WEIGHT,
            text,
            units,
        ),
        mono,
        units,
    );
    monospace.line_height = line_height_px(mono_size, units);
    monospace.stated |= TextStated::LINE_HEIGHT;
    Typography {
        display: Some(text_scale_entry(
            title_size(ui_size, TITLE_1_FACTOR, DISPLAY_SIZE_PT),
            DISPLAY_WEIGHT,
            units,
        )),
        title1: Some(text_scale_entry(
            title_size(ui_size, TITLE_2_FACTOR, TITLE1_SIZE_PT),
            TITLE1_WEIGHT,
            units,
        )),
        title2: None,
        title3: None,
        title4: None,
        headline: Some(text_scale_entry(body_size, HEADLINE_WEIGHT, units)),
        caption: Some(text_scale_entry(
            title_size(ui_size, CAPTION_FACTOR, CAPTION_SIZE_PT),
            body.weight,
            units,
        )),
        body,
        callout: None,
        subheadline: None,
        monospace,
        line_height_multiplier: LINE_HEIGHT,
        fallbacks: Arc::from([]),
        emoji_family: None,
        rendering: TextRendering::default(),
        font_dpi: dpi,
        font_whole_pixels: None,
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

// ---------------------------------------------------------------------------
// The widgets
// ---------------------------------------------------------------------------

/// A part's fill of one colour.
fn fill(color: Color) -> PartProp {
    PartProp::Fill(Paint::Solid(color))
}

/// A part's outline in `color`, inside its edge, `width` wide.
fn stroke(width: f32, color: Color) -> PartProp {
    PartProp::Stroke(Stroke::uniform(width, color))
}

/// A part's outline in the default border colour at `width` (0.7.2: a
/// widget's `border.color` is `defaults.border.color` where unstated,
/// `resolve_border_inheritance`).
fn border(roles: &ColorRoles, width: f32) -> Option<PartProp> {
    roles.window.border.map(|color| stroke(width, color))
}

/// A part's corners at `radius`.
fn corners(radius: f32) -> PartProp {
    PartProp::Corners(Corners::uniform(radius))
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

/// The push button: `$button_color` with no outline (`button.border.line_width`
/// 0, in the default border colour, 0.7.2's `button.border.color`), the
/// default corners, its hovered, pressed and checked fills; the primary
/// variant filled with the accent and lettered in its text (0.7.2:
/// `primary_background` and `primary_text_color` are `defaults.accent_color`
/// and `defaults.accent_text_color` where unstated). Its label is the window's
/// text in every state, the disabled one the disabled text's colour (0.7.2:
/// `button.disabled_text_color` is `defaults.disabled_text_color` where
/// unstated).
fn button(
    c: &SchemeColors,
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
    let body = RuleBuilder::with_exclusive(PartStyle::default(), PRESSED_HOVER)
        .base([fill(c.button_background)])
        .base(border(roles, BUTTON_BORDER_WIDTH))
        .base([
            corners(layout.corner_radius),
            PartProp::Padding(Edges::symmetric(
                BUTTON_PADDING_VERTICAL,
                BUTTON_PADDING_HORIZONTAL,
            )),
            PartProp::MinSize([Some(BUTTON_MIN_WIDTH), Some(BUTTON_MIN_HEIGHT)]),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.button_pressed)],
        )
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.button_hover)])
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.button_pressed)],
        )
        // Hovered and pressed, the outline as at rest: a raised button's :hover and :active set
        // only its background (_buttons.scss:28-42 at libadwaita 1.10.0), stated for each state
        // (spec §2, `from_state`).
        .rules(
            border(roles, BUTTON_BORDER_WIDTH)
                .into_iter()
                .flat_map(|_| {
                    [
                        RuleSelector::on(WidgetState::HOVER),
                        RuleSelector::on(WidgetState::PRESSED),
                    ]
                }),
            border(roles, BUTTON_BORDER_WIDTH),
        )
        .rule(primary(), [fill(roles.accent.fill)])
        .build();
    // `button.hover_text_color`, `active_text_color`: 0.7.2 adwaita.toml (uncited);
    // `checked_text_color`: platform-facts.md (§2.3), "no :checked rule sets a label colour"; each
    // stated for its state (spec §2, `from_state`).
    let label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
                RuleSelector::on(WidgetState::CHECKED),
            ],
            [TextProp::Color(c.window_foreground)],
        )
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
        BUTTON_ICON_TEXT_GAP,
        // 0.7.2: `button.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The box of a check box or the circle of a radio button, which 0.7.2's
/// `checkbox.*` serves both: its unchecked fill in its outline at the default
/// border width (0.7.2: `checkbox.border.line_width` is
/// `defaults.border.line_width` where unstated), no shadow, padded; under the
/// pointer the hover fill and, unchecked, the hover outline; checked, the accent (0.7.2:
/// `checkbox.checked_background` is `defaults.accent_color` where unstated)
/// with no outline (`box-shadow: none`, _checks.scss:52-56 at libadwaita
/// 1.10.0; the outline is the inset box-shadow, :27). No disabled fill: a
/// disabled check is its enabled self at the disabled opacity
/// (`checkbox.unchecked_background`'s citation; `check:disabled, radio:disabled`
/// change no colour, _checks.scss:62-64).
fn indicator(
    c: &SchemeColors,
    roles: &ColorRoles,
    layout: &LayoutMetrics,
) -> RuleBuilder<PartStyle, PartProp> {
    RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.checkbox_background),
            stroke(layout.border_width, c.frame),
            // `checkbox.border.shadow_enabled`: platform-facts.md §2.5 border.shadow_enabled: no
            // shadow
            PartProp::Shadows(None),
            PartProp::Padding(Edges::symmetric(
                CHECKBOX_PADDING_VERTICAL,
                CHECKBOX_PADDING_HORIZONTAL,
            )),
            PartProp::MinSize([Some(CHECKBOX_INDICATOR_SIZE); 2]),
            PartProp::MaxSize([Some(CHECKBOX_INDICATOR_SIZE); 2]),
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.button_hover)])
        // An unchecked check or radio under the pointer: its outline $trough_hover_color
        // (_checks.scss:20-35 at libadwaita 1.10.0); a checked one keeps its checked look.
        .rule(
            RuleSelector::on(WidgetState::HOVER).except(WidgetState::CHECKED),
            [stroke(layout.border_width, c.trough_hover)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill), PartProp::Stroke(Stroke::None)],
        )
}

/// The label of a check box or a radio button: the UI font in its colour;
/// disabled, the disabled text's colour (0.7.2: `checkbox.disabled_text_color`
/// is `defaults.disabled_text_color` where unstated).
fn check_label(roles: &ColorRoles, typography: &Typography) -> RuleSet<TextStyle, TextProp> {
    RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build()
}

/// The check box: [`indicator`] with its corners, a check mark in the
/// accent's text colour (0.7.2: `checkbox.indicator_color` is
/// `defaults.accent_text_color` where unstated), [`check_label`].
fn checkbox(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> CheckboxWidget {
    // Hovered, pressed and disabled, the mark as at rest: a checked or mixed check's `color:
    // var(--accent-fg-color)` holds in every state, :hover and :active lay a layer over the box,
    // not the mark, and :disabled changes no colour, only the control's opacity
    // (_checks.scss:52-64, :70, :80 at libadwaita 1.10.0).
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base([
            IconProp::Color(roles.accent.foreground),
            IconProp::StrokeWidth(Some(CHECK_MARK_STROKE_WIDTH)),
        ])
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
                RuleSelector::on(WidgetState::DISABLED),
            ],
            [IconProp::Color(roles.accent.foreground)],
        )
        .build();
    // An unchecked box pressed: $trough_active_color and no outline (_checks.scss:42-44). A
    // checked box keeps its checked look. A mixed box is drawn as a checked one, the accent with
    // no outline, in every state (`&:checked, &:indeterminate`, _checks.scss:52-56).
    let indicator = indicator(c, roles, layout)
        .base([corners(CHECKBOX_CORNER_RADIUS)])
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(WidgetState::CHECKED),
            [fill(c.trough_active), PartProp::Stroke(Stroke::None)],
        )
        .rule(
            RuleSelector::on(WidgetState::INDETERMINATE),
            [fill(roles.accent.fill), PartProp::Stroke(Stroke::None)],
        )
        .build();
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator,
        checkmark,
        check_label(roles, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The radio button: [`indicator`] (0.7.2 states no corners for it), a dot
/// in the accent's text colour (`checkbox.indicator_color`), [`check_label`].
fn radio(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> RadioWidget {
    // Disabled, the dot as at rest: `:disabled` changes no colour, only the control's opacity
    // (_checks.scss:52-55, :62-64, :76 at libadwaita 1.10.0).
    let dot = RuleBuilder::new(PartStyle::default())
        .base([
            fill(roles.accent.foreground),
            PartProp::MinSize([Some(RADIO_DOT_SIZE); 2]),
            PartProp::MaxSize([Some(RADIO_DOT_SIZE); 2]),
        ])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(roles.accent.foreground)],
        )
        .build();
    RadioWidget::new(
        WidgetOrigin::Native,
        indicator(c, roles, layout).build(),
        dot,
        check_label(roles, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The list or table view: the window's background (0.7.2's
/// `list.background_color` where unstated, `resolve_safety_nets`) in the
/// default frame and corners (`resolve_border_inheritance`); rows padded
/// (spec §5, `L4`) with the alternate row's fill; the highlight's hovered and
/// selected fills (0.7.2: `list.selection_background` and
/// `selection_text_color` are the selection's where unstated); the window's
/// text, which a hovered row keeps (`list.hover_text_color`), the selection's
/// text on a selected row and the disabled text's colour on a disabled one
/// (0.7.2: `list.disabled_text_color` is `defaults.disabled_text_color` where
/// unstated); headers in the view's background, of a stated height; the grid
/// in the default border colour (0.7.2: `list.grid_color` is
/// `defaults.border.color` where unstated). 0.7.2 states no row height
/// ("sizes to content") and no header separator.
fn list(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ListWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, layout.border_width))
        .base([corners(layout.corner_radius)])
        .build();
    let alternate = RuleSelector::on(WidgetState::NONE).matcher(ListMatcher {
        variant: None,
        alternate: Some(true),
    });
    let row = RuleBuilder::new(PartStyle::default())
        .base([PartProp::Padding(Edges::symmetric(
            LIST_ROW_PADDING_VERTICAL,
            LIST_ROW_PADDING_HORIZONTAL,
        ))])
        .rule(alternate, [fill(c.list_alternate_row)])
        .build();
    let highlight = RuleBuilder::new(PartStyle::default())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.row_hover)])
        // `row.activatable:active`: $view_active_color (_lists.scss:75-78 at libadwaita 1.10.0).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.view_active)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    // `list.hover_text_color`: 0.7.2 adwaita.toml (uncited), stated for `HOVER` (spec §2,
    // `from_state`), before the rules of the states that replace it.
    let item_label = RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(typography.body.color)],
        )
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
            fill(c.view_background),
            PartProp::MinSize([None, Some(LIST_HEADER_HEIGHT)]),
        ])
        .build();
    ListWidget::new(
        WidgetOrigin::Native,
        container,
        row,
        highlight,
        item_label,
        header,
        RuleBuilder::new(label(&typography.body, None)).build(),
        RuleBuilder::new(PartStyle::default()).build(),
        c.window_border,
    )
}

/// The parts `builder` sizes across their orientation: `thickness` wide on a
/// vertical bar and tall on a horizontal one.
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

/// The scroll bar: a groove in the window's background (0.7.2's
/// `scrollbar.track_color` where unstated, `resolve_safety_nets`) and a
/// handle in the muted foreground (0.7.2: `scrollbar.thumb_color` is
/// `defaults.muted_color` where unstated), each of an exact thickness across
/// the bar; the hovered and the pressed handle's own colours. It overlays the
/// content where `overlay` (the system's setting, else the preset's).
fn scrollbar(c: &SchemeColors, roles: &ColorRoles, overlay: bool) -> ScrollbarWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base(roles.window.background.map(fill)),
        SCROLLBAR_GROOVE_WIDTH,
    )
    .build();
    let thumb = across(
        RuleBuilder::with_exclusive(PartStyle::default(), PRESSED_HOVER)
            .base(roles.window.foreground_muted.map(fill)),
        SCROLLBAR_THUMB_WIDTH,
    )
    .rule(
        RuleSelector::on(WidgetState::HOVER),
        [fill(c.scrollbar_thumb_hover)],
    )
    .rule(
        RuleSelector::on(WidgetState::PRESSED),
        [fill(c.scrollbar_thumb_pressed)],
    )
    .build();
    ScrollbarWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        SCROLLBAR_MIN_THUMB_LENGTH,
        overlay,
    )
}

/// The tooltip: the tooltip's colours, the default corners and border
/// (`resolve_border_inheritance`), padded and of a greatest width.
fn tooltip(roles: &ColorRoles, typography: &Typography, layout: &LayoutMetrics) -> TooltipWidget {
    let mut panel = PartStyle::default();
    for prop in roles
        .tooltip
        .background
        .map(fill)
        .into_iter()
        .chain([
            corners(layout.corner_radius),
            PartProp::Padding(Edges::symmetric(
                TOOLTIP_PADDING_VERTICAL,
                TOOLTIP_PADDING_HORIZONTAL,
            )),
            PartProp::MaxSize([Some(TOOLTIP_MAX_WIDTH), None]),
        ])
        .chain(border(roles, layout.border_width))
    {
        panel.apply(&prop);
    }
    TooltipWidget::new(
        WidgetOrigin::Native,
        panel,
        label(&typography.body, roles.tooltip.foreground),
    )
}

/// The window: its frame in the window's background (0.7.2:
/// `window.background_color` is `defaults.background_color` where unstated)
/// with the default border and the large corners
/// (`resolve_border_inheritance`); the active title bar in its own colour,
/// else the view's background (0.7.2: `window.title_bar_background` is
/// `defaults.surface_color` where unstated); the inactive one in its own
/// colour; the title in the UI font at its weight, in the window's text,
/// active or not (0.7.2: `window.inactive_title_bar_text_color` is
/// `window.title_bar_font.color` where unstated), with the system's title
/// bar font `title_font` laid over it where given ([`laid_over`]; 0.7.2's
/// reader states `window.title_bar_font`).
fn window(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    title_font: Option<&OsFont>,
    units: FontUnits,
    layout: &LayoutMetrics,
    active: bool,
) -> WindowChrome {
    let mut frame = PartStyle::default();
    for prop in roles
        .window
        .background
        .map(fill)
        .into_iter()
        .chain(border(roles, layout.border_width))
        .chain([corners(layout.corner_radius_lg)])
    {
        frame.apply(&prop);
    }
    let title_bar_fill = if active {
        c.title_bar_background.or(roles.view.background)
    } else {
        Some(c.window_background)
    };
    let mut title_bar = PartStyle::default();
    if let Some(color) = title_bar_fill {
        title_bar.apply(&fill(color));
    }
    let mut title = label(&typography.body, Some(c.window_foreground));
    title.weight = WINDOW_TITLE_WEIGHT;
    let title = laid_over(title, title_font, units);
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

/// The text field, and with `padding` and no least height the text area
/// (0.7.2's `text_area.border` takes the field's colour, radius, width and
/// shadow where unstated, `resolve_widget_to_widget`): `$button_color` with
/// no outline at rest, in the default border colour; the default corners; no
/// shadow; focused, the outline in the focus ring's colour
/// (`input.focus_border_color`); the window's text, the disabled text's
/// colour when disabled (0.7.2: `input.disabled_text_color` is
/// `defaults.disabled_text_color` where unstated); the placeholder's own
/// colour.
fn input(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    padding: Edges,
    min_height: Option<f32>,
) -> InputWidget {
    let field = RuleBuilder::new(PartStyle::default())
        .base([fill(c.button_background)])
        .base(border(roles, INPUT_BORDER_WIDTH))
        .base([
            corners(layout.corner_radius),
            // `input.border.shadow_enabled`: platform-facts.md §2.4 border.shadow_enabled: no shadow
            PartProp::Shadows(None),
            PartProp::Padding(padding),
        ])
        .base(min_height.map(|height| PartProp::MinSize([None, Some(height)])))
        // Hovered and pressed, the outline as at rest: an entry has no :hover or :active rule
        // (_entries.scss:1-60 at libadwaita 1.10.0); the click focuses it, and its focus ring is
        // FOCUSED's (:focus-within, _entries.scss:17; _drawing.scss:12-22).
        .rules(
            border(roles, INPUT_BORDER_WIDTH).into_iter().flat_map(|_| {
                [
                    RuleSelector::on(WidgetState::HOVER),
                    RuleSelector::on(WidgetState::PRESSED),
                ]
            }),
            border(roles, INPUT_BORDER_WIDTH),
        )
        .rule(
            RuleSelector::on(WidgetState::FOCUSED),
            [stroke(INPUT_BORDER_WIDTH, roles.focus_ring)],
        )
        .build();
    // Hovered and pressed, the text as at rest: an entry has no :hover or :active rule of its
    // own, only its icons' opacity (_entries.scss:1-60 at libadwaita 1.10.0).
    let text = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
            ],
            [TextProp::Color(c.window_foreground)],
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
        // 0.7.2: `input.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The link: a transparent background, at rest and under the pointer; the
/// link colour (0.7.2: `link.font.color` and `visited_text_color` are
/// `defaults.link_color` where unstated) in every state, underlined.
fn link(roles: &ColorRoles, typography: &Typography) -> LinkWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(LINK_BACKGROUND)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(LINK_BACKGROUND)],
        )
        .build();
    // `link.hover_text_color`, `active_text_color`: 0.7.2 adwaita.toml (uncited);
    // `disabled_text_color`: platform-facts.md (§2.1.6), the enabled link colour, at
    // `disabled_opacity`; each stated for its state (spec §2, `from_state`).
    let label = RuleBuilder::new(label(&typography.body, Some(roles.link)))
        .base([TextProp::Decoration(TextDecoration {
            kind: LINK_DECORATION,
            color: None,
            thickness: None,
        })])
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
                RuleSelector::on(WidgetState::DISABLED),
            ],
            [TextProp::Color(roles.link)],
        )
        .build();
    LinkWidget::new(WidgetOrigin::Native, container, label)
}

/// The slider: a groove in the muted foreground (0.7.2: `slider.track_color`
/// is `defaults.muted_color` where unstated) of an exact thickness, its
/// filled part in the accent (`slider.fill_color` is `defaults.accent_color`
/// where unstated), the handle in `$slider_color` and its hover colour.
fn slider(c: &SchemeColors, roles: &ColorRoles) -> SliderWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base(roles.window.foreground_muted.map(fill)),
        SLIDER_TRACK_HEIGHT,
    )
    .build();
    let active_track = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.accent.fill)])
        .build();
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.slider_thumb),
            PartProp::MinSize([Some(SLIDER_THUMB_DIAMETER); 2]),
            PartProp::MaxSize([Some(SLIDER_THUMB_DIAMETER); 2]),
        ])
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
            ],
            [fill(c.slider_thumb_hover)],
        )
        .build();
    SliderWidget::new(
        WidgetOrigin::Native,
        track,
        active_track,
        thumb,
        RuleBuilder::new(PartStyle::default()).build(),
        SLIDER_TICK_LENGTH,
        // 0.7.2: `slider.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The progress bar: a groove in the muted foreground (0.7.2:
/// `progress_bar.track_color` is `defaults.muted_color` where unstated) with
/// no outline, in the default border colour, the default corners, unpadded,
/// of an exact height and a least length; the progress in the accent
/// (`progress_bar.fill_color` is `defaults.accent_color` where unstated).
fn progress_bar(roles: &ColorRoles, layout: &LayoutMetrics) -> ProgressBarWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base(roles.window.foreground_muted.map(fill))
        .base(border(roles, PROGRESS_BAR_BORDER_WIDTH))
        .base([
            corners(layout.corner_radius),
            PartProp::Padding(Edges::symmetric(
                PROGRESS_BAR_PADDING_VERTICAL,
                PROGRESS_BAR_PADDING_HORIZONTAL,
            )),
            PartProp::MinSize([
                Some(PROGRESS_BAR_MIN_WIDTH),
                Some(PROGRESS_BAR_TRACK_HEIGHT),
            ]),
            PartProp::MaxSize([None, Some(PROGRESS_BAR_TRACK_HEIGHT)]),
        ])
        .build();
    let progress = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.accent.fill)])
        .build();
    ProgressBarWidget::new(WidgetOrigin::Native, track, progress, None)
}

/// The separator: its line colour, at the default border width (0.7.2:
/// `separator.line_width` is `defaults.border.line_width` where unstated).
fn separator(c: &SchemeColors, layout: &LayoutMetrics) -> SeparatorWidget {
    SeparatorWidget::new(
        WidgetOrigin::Native,
        c.frame,
        layout.border_width,
        None,
        None,
    )
}

/// The splitter: its handle in the separator's translucent line in every
/// state, where 0.7.2 took the default border colour
/// (`splitter.divider_color` is `defaults.border.color` where unstated) and
/// an opaque hover colour.
fn splitter(c: &SchemeColors) -> SplitterWidget {
    let line = fill(c.splitter_line);
    let handle = RuleBuilder::new(PartStyle::default())
        .base([line.clone()])
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
            ],
            [line],
        )
        .build();
    SplitterWidget::new(WidgetOrigin::Native, handle, SPLITTER_DIVIDER_WIDTH)
}

/// The spinner, in the accent (0.7.2's `spinner.fill_color` where unstated,
/// `resolve_safety_nets`).
fn spinner(roles: &ColorRoles) -> SpinnerWidget {
    SpinnerWidget::new(
        WidgetOrigin::Native,
        roles.accent.fill,
        SPINNER_DIAMETER,
        SPINNER_MIN_DIAMETER,
        SPINNER_STROKE_WIDTH,
        None,
    )
}

/// The menu: its panel in the window's background (0.7.2:
/// `menu.background_color` is `defaults.background_color` where unstated);
/// items of an exact height, padded, their hover fill; the window's text,
/// which a hovered item keeps (`menu.hover_text_color`), the disabled text's
/// colour when disabled (0.7.2: `menu.disabled_text_color` is
/// `defaults.disabled_text_color` where unstated); icons at the toolbar's size
/// (0.7.2: `menu.icon_size` is `defaults.icon_sizes.toolbar` where
/// unstated); separators in the default border colour (0.7.2:
/// `menu.separator_color` is `defaults.border.color` where unstated).
fn menu(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    direction: Direction,
) -> MenuWidget {
    let panel = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .build();
    let item = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::MinSize([None, Some(MENU_ROW_HEIGHT)]),
            PartProp::MaxSize([None, Some(MENU_ROW_HEIGHT)]),
            PartProp::Padding(physical_edges(
                Edges::symmetric(MENU_ITEM_PADDING_VERTICAL, MENU_ITEM_PADDING_HORIZONTAL),
                direction,
            )),
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.menu_hover)])
        .build();
    // `menu.hover_text_color`: 0.7.2 adwaita.toml (uncited), stated for `HOVER` (spec §2,
    // `from_state`), before the disabled rule.
    let item_label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.window_foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    let icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(ICON_TOOLBAR)])
        .build();
    let separator = RuleBuilder::new(PartStyle::default())
        .base(roles.window.border.map(fill))
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
        MENU_ICON_TEXT_GAP,
    )
}

/// The menu bar: the toolbar's background (0.7.2's
/// `toolbar.background_color`, `defaults.background_color` where unstated);
/// 0.7.2 states nothing of its titles.
fn menu_bar(roles: &ColorRoles) -> MenuBarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
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
/// `defaults.background_color` where unstated), a tab of least sizes,
/// padded, no outline, selected or not, its hover fill, the selected tab's
/// own fill (`tab.active_background`); the window's text, which a hovered and
/// a selected tab keep (`tab.hover_text_color`, `tab.active_text_color`); the
/// selected tab's indicator in the accent along its bottom.
fn tab(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> TabWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .build();
    let tab = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base([
            PartProp::MinSize([Some(TAB_MIN_WIDTH), Some(TAB_MIN_HEIGHT)]),
            PartProp::Padding(Edges::symmetric(
                TAB_PADDING_VERTICAL,
                TAB_PADDING_HORIZONTAL,
            )),
            // `tab.border.color`, `border.line_width`: platform-facts.md (§2.11): Notebook and
            // AdwTabBar draw no outline on a tab, line width 0
            PartProp::Stroke(Stroke::None),
        ])
        // Hovered and pressed, no outline, as at rest: `&:hover, &:active` sets only the
        // background (_notebook.scss:127-129 at libadwaita 1.10.0); the hovered tab's 4px
        // $border_color line on its page side (:17) is the indicator's place, not an outline.
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
            ],
            [fill(c.hover_layer)],
        )
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER).except(WidgetState::SELECTED),
                RuleSelector::on(WidgetState::PRESSED).except(WidgetState::SELECTED),
            ],
            [PartProp::Stroke(Stroke::None)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(c.window_background)],
        )
        .build();
    let indicator = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [
                fill(c.accent),
                PartProp::MinSize([None, Some(TAB_INDICATOR_WIDTH)]),
                PartProp::MaxSize([None, Some(TAB_INDICATOR_WIDTH)]),
            ],
        )
        .build();
    // `tab.active_text_color`, `hover_text_color`: 0.7.2 adwaita.toml (uncited), each stated for
    // its state (spec §2, `from_state`); pressed the same: a notebook tab's :hover and :active
    // set only background-color (_notebook.scss:127-129 at libadwaita 1.10.0).
    let tab_label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
                RuleSelector::on(WidgetState::SELECTED),
            ],
            [TextProp::Color(c.window_foreground)],
        )
        .build();
    TabWidget::new(
        WidgetOrigin::Native,
        bar,
        tab,
        tab_label,
        indicator,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        Some(TAB_ITEM_GAP),
        Some(TAB_INDICATOR_SIDE),
        None,
    )
}

/// The toolbar: the window's background (0.7.2: `toolbar.background_color`
/// is `defaults.background_color` where unstated) with no outline, in the
/// default border colour, the default corners, no shadow, padded; its
/// buttons' hovered, pressed and checked layers; icons at the toolbar's size
/// (0.7.2: `toolbar.icon_size` is `defaults.icon_sizes.toolbar` where
/// unstated).
fn toolbar(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ToolbarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, TOOLBAR_BORDER_WIDTH))
        .base([
            corners(layout.corner_radius),
            // `toolbar.border.shadow_enabled`: 0.7.2 adwaita.toml (uncited)
            PartProp::Shadows(None),
            PartProp::Padding(Edges::symmetric(
                TOOLBAR_PADDING_VERTICAL,
                TOOLBAR_PADDING_HORIZONTAL,
            )),
        ])
        .build();
    let button = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.toolbar_button_hover)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.toolbar_button_pressed)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.toolbar_button_checked)],
        )
        .build();
    let button_icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(ICON_TOOLBAR)])
        .build();
    // Hovered and pressed, the label as at rest: a raised button's :hover and :active
    // (_buttons.scss:35-42 at libadwaita 1.10.0) and a flat one's (:250-257; a toolbar flattens
    // its arrow, image and image-text buttons, _toolbars.scss:1-8) set only the background.
    let button_label = label(&typography.body, None);
    let (selectors, color) =
        rest_color_rules(&button_label, [WidgetState::HOVER, WidgetState::PRESSED]);
    ToolbarWidget::new(
        WidgetOrigin::Native,
        bar,
        button,
        RuleBuilder::new(button_label)
            .rules(selectors, color)
            .build(),
        button_icon,
        RuleBuilder::new(PartStyle::default()).build(),
        Some(TOOLBAR_ITEM_GAP),
    )
}

/// The status bar: the window's background (0.7.2:
/// `status_bar.background_color` is `defaults.background_color` where
/// unstated) in the default border colour and width (0.7.2's partial border
/// inheritance: colour and width only), padded.
fn status_bar(
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
) -> StatusBarWidget {
    let mut bar = PartStyle::default();
    for prop in roles
        .window
        .background
        .map(fill)
        .into_iter()
        .chain(border(roles, layout.border_width))
        .chain([PartProp::Padding(physical_edges(
            Edges::symmetric(STATUS_BAR_PADDING_VERTICAL, STATUS_BAR_PADDING_HORIZONTAL),
            direction,
        ))])
    {
        bar.apply(&prop);
    }
    StatusBarWidget::new(
        WidgetOrigin::Native,
        bar,
        label(&typography.body, None),
        PartStyle::default(),
    )
}

/// The sidebar: its own background in the default border colour and width
/// (0.7.2's partial border inheritance: colour and width only), padded; an
/// item's hover fill and the selection's fill (0.7.2:
/// `sidebar.selection_background` and `selection_text_color` are the
/// selection's where unstated); the window's text, the selection's on a
/// selected item.
fn sidebar(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
) -> SidebarWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(c.sidebar_background)])
        .base(border(roles, layout.border_width))
        .base([PartProp::Padding(physical_edges(
            Edges::new(
                SIDEBAR_PADDING_TOP,
                SIDEBAR_PADDING_HORIZONTAL,
                SIDEBAR_PADDING_BOTTOM,
                SIDEBAR_PADDING_HORIZONTAL,
            ),
            direction,
        ))])
        .build();
    // Pressed: `.navigation-sidebar row.activatable:active`, $active_color of the sidebar's text
    // (_sidebars.scss:126-129 at libadwaita 1.10.0).
    let item = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.sidebar_hover)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.active_layer)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    // Hovered and pressed, the label as at rest: the row's :hover and :active set only
    // background-color (_sidebars.scss:126-132).
    let item_label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
            ],
            [TextProp::Color(c.window_foreground)],
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

/// The popover: its background, outline and padding, the large corners
/// (0.7.2: `popover.border.corner_radius` is `defaults.border.corner_radius_lg`
/// where unstated, `resolve_border_inheritance`); its text in the window's.
fn popover(c: &SchemeColors, typography: &Typography, layout: &LayoutMetrics) -> PopoverWidget {
    let mut panel = PartStyle::default();
    for prop in [
        fill(c.popover_background),
        stroke(POPOVER_BORDER_WIDTH, POPOVER_BORDER),
        corners(layout.corner_radius_lg),
        PartProp::Padding(Edges::symmetric(
            POPOVER_PADDING_VERTICAL,
            POPOVER_PADDING_HORIZONTAL,
        )),
    ] {
        panel.apply(&prop);
    }
    PopoverWidget::new(
        WidgetOrigin::Native,
        panel,
        PartStyle::default(),
        label(&typography.body, Some(c.window_foreground)),
    )
}

/// The switch: a track of exact sizes and corners, off and on (0.7.2:
/// `switch.checked_background` is `defaults.accent_color` where unstated),
/// with their hover colours; a thumb of an exact diameter, its colour when
/// off where stated, else its colour when on (0.7.2's fallback of
/// `unchecked_thumb_background`). 0.7.2 states no outline for either ("`None`
/// draws no outline"). Its label [`switch_label`], `AdwSwitchRow`'s title,
/// before the track, at least [`SWITCH_LABEL_GAP`] from it, the title taking
/// the row's free width.
fn switch(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> SwitchWidget {
    let size = [Some(SWITCH_TRACK_WIDTH), Some(SWITCH_TRACK_HEIGHT)];
    let track = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.frame),
            PartProp::Stroke(Stroke::None),
            corners(SWITCH_TRACK_RADIUS),
            PartProp::MinSize(size),
            PartProp::MaxSize(size),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill)],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.switch_hover_off)],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::CHECKED),
            [fill(c.switch_hover_on)],
        )
        // `switch:active`: $trough_active_color (_switch.scss:9 at libadwaita 1.10.0); a checked
        // switch keeps its checked look.
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(WidgetState::CHECKED),
            [fill(c.trough_active)],
        )
        .build();
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.switch_thumb_off.unwrap_or(c.switch_thumb)),
            PartProp::Stroke(Stroke::None),
            PartProp::MinSize([Some(SWITCH_THUMB_DIAMETER); 2]),
            PartProp::MaxSize([Some(SWITCH_THUMB_DIAMETER); 2]),
        ])
        // The on thumb's diameter, `switch.thumb_diameter` (the constant's citation), stated for
        // `CHECKED` (spec §2, `from_state`).
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [
                fill(c.switch_thumb),
                PartProp::MinSize([Some(SWITCH_THUMB_DIAMETER); 2]),
                PartProp::MaxSize([Some(SWITCH_THUMB_DIAMETER); 2]),
            ],
        )
        .build();
    SwitchWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        switch_label(c, typography),
        // `label_side`, `label_gap_stretches`: libadwaita's labelled switch is `AdwSwitchRow`, its
        // title first and expanding (adw-action-row.ui:40-43), the switch a suffix at the row's
        // end (adw-switch-row.c:161; adw-action-row.ui:91-94, at libadwaita 1.10.0).
        Some(LabelSide::Leading),
        Some(SWITCH_LABEL_GAP),
        Some(true),
        None,
        // 0.7.2: `switch.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The switch's label, `AdwSwitchRow`'s title: the row's text,
/// `view_fg_color` in a list and `card_fg_color` in a boxed list
/// (_lists.scss:5-7, :521; _misc.scss:202-204 at libadwaita 1.10.0), in the
/// body font, whose family, size and weight its rule leaves (:146-151); a
/// hovered, pressed or on row changes only its background and the switch's
/// (:75-78, :592-601; _switch.scss:8-9, :36-55; adw-switch-row.c:217-227);
/// insensitive, on or off, the title fades to [`DISABLED_OPACITY`]
/// (`label:disabled { filter: Opacity(var(--disabled-opacity)) }`,
/// _labels.scss:8-10; _colors.scss:320).
fn switch_label(c: &SchemeColors, typography: &Typography) -> RuleSet<TextStyle, TextProp> {
    let same = [
        RuleSelector::on(WidgetState::HOVER),
        RuleSelector::on(WidgetState::PRESSED),
        RuleSelector::on(WidgetState::CHECKED),
    ];
    RuleBuilder::new(label(&typography.body, Some(c.row_foreground)))
        .rules(same, [TextProp::Color(c.row_foreground)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(
                c.row_foreground.mul_alpha(DISABLED_OPACITY),
            )],
        )
        .build()
}

/// The dialog: its panel in the dialog's background where a palette states
/// it (`dialog_bg_color`), else the window's (0.7.2's
/// `dialog.background_color` where unstated, `resolve_safety_nets`) with the
/// default border (`resolve_border_inheritance`), its own corners, padding
/// and size limits; its title in the UI font at `title_size` (the preset's,
/// or derived from the system's UI font, [`title_size`]) and its own weight,
/// its body in the UI font (0.7.2: `dialog.title_font` and `body_font` are
/// `defaults.font` where unstated).
fn dialog(
    roles: &ColorRoles,
    typography: &Typography,
    title_size: FontSize,
    units: FontUnits,
    layout: &LayoutMetrics,
    direction: Direction,
    motion: Motion,
) -> DialogWidget {
    let mut panel = PartStyle::default();
    for prop in roles
        .dialog
        .background
        .or(roles.window.background)
        .map(fill)
        .into_iter()
        .chain(border(roles, layout.border_width))
        .chain([
            corners(DIALOG_CORNER_RADIUS),
            PartProp::Padding(physical_edges(
                Edges::new(
                    DIALOG_PADDING_TOP,
                    DIALOG_PADDING_HORIZONTAL,
                    DIALOG_PADDING_BOTTOM,
                    DIALOG_PADDING_HORIZONTAL,
                ),
                direction,
            )),
            PartProp::MinSize([Some(DIALOG_MIN_WIDTH), Some(DIALOG_MIN_HEIGHT)]),
            PartProp::MaxSize([Some(DIALOG_MAX_WIDTH), Some(DIALOG_MAX_HEIGHT)]),
        ])
    {
        panel.apply(&prop);
    }
    let mut title = label(&typography.body, None);
    title.size_px = font_px(title_size, units);
    title.defined_size = Some(title_size);
    title.weight = DIALOG_TITLE_WEIGHT;
    let mut icon = IconStyle::default();
    icon.apply(&IconProp::Size(DIALOG_ICON_SIZE));
    DialogWidget::new(
        WidgetOrigin::Native,
        panel,
        title,
        label(&typography.body, None),
        icon,
        DIALOG_BUTTON_GAP,
        Some(DIALOG_BUTTON_ORDER),
        dialog_entrance(Some(DIALOG_ENTRANCE), motion),
    )
}

/// The combo box: `$button_color` with no outline, in the default border
/// colour, the default corners, no shadow, padded, of least sizes, its hover
/// fill; the window's text, the disabled text's colour when disabled (0.7.2:
/// `combo_box.disabled_text_color` is `defaults.disabled_text_color` where
/// unstated); its arrow's size.
fn combo_box(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
) -> ComboBoxWidget {
    let field = RuleBuilder::new(PartStyle::default())
        .base([fill(c.button_background)])
        .base(border(roles, COMBO_BOX_BORDER_WIDTH))
        .base([
            corners(layout.corner_radius),
            // `combo_box.border.shadow_enabled`: platform-facts.md §2.24 border.shadow_enabled: no
            // shadow
            PartProp::Shadows(None),
            PartProp::Padding(physical_edges(
                Edges::symmetric(COMBO_BOX_PADDING_VERTICAL, COMBO_BOX_PADDING_HORIZONTAL),
                direction,
            )),
            PartProp::MinSize([Some(COMBO_BOX_MIN_WIDTH), Some(COMBO_BOX_MIN_HEIGHT)]),
        ])
        // Hovered and pressed, the outline as at rest: the trigger is a raised button, whose
        // :hover and :active set only its background (_buttons.scss:35-42; _dropdowns.scss:1-12
        // adds no outline, at libadwaita 1.10.0).
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            std::iter::once(fill(c.button_hover)).chain(border(roles, COMBO_BOX_BORDER_WIDTH)),
        )
        // Pressed: the trigger is a raised button, `&:active { background-color:
        // $button_active_color }` (_buttons.scss:39-42; _dropdowns.scss:1-12 adds no fill, at
        // libadwaita 1.10.0).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            std::iter::once(fill(c.button_active_layer))
                .chain(border(roles, COMBO_BOX_BORDER_WIDTH)),
        )
        .build();
    // Hovered and pressed, the label as at rest: no button rule sets `color` under :hover or
    // :active (_buttons.scss:28-56).
    let rest_label = label(&typography.body, None);
    let (selectors, color) =
        rest_color_rules(&rest_label, [WidgetState::HOVER, WidgetState::PRESSED]);
    let label = RuleBuilder::new(rest_label)
        .rules(selectors, color)
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build();
    let chevron = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(COMBO_BOX_ARROW_SIZE)])
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

/// The segmented control: a track of `$button_color` in the default border
/// (`resolve_border_inheritance`), the default corners, no shadow; segments
/// of a least height, padded, their hover fill and the selected one's own
/// fill and text.
fn segmented_control(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
) -> SegmentedControlWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base([fill(c.button_background)])
        .base(border(roles, layout.border_width))
        .base([
            corners(layout.corner_radius),
            // `segmented_control.border.shadow_enabled`: platform-facts.md §2.25
            // border.shadow_enabled: none
            PartProp::Shadows(None),
        ])
        .build();
    let segment = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::MinSize([None, Some(SEGMENT_HEIGHT)]),
            PartProp::Padding(physical_edges(
                Edges::symmetric(SEGMENT_PADDING_VERTICAL, SEGMENT_PADDING_HORIZONTAL),
                direction,
            )),
        ])
        // Hovered and pressed, no outline: a toggle's :hover, :active and .keyboard-activating
        // set only its background (_toggle-group.scss:65-71 at libadwaita 1.10.0), its inset
        // line is drawn only under prefers-contrast: more (:73-77).
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER).except(WidgetState::SELECTED),
                RuleSelector::on(WidgetState::PRESSED).except(WidgetState::SELECTED),
            ],
            [PartProp::Stroke(Stroke::None)],
        )
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.button_hover)])
        // Pressed: AdwToggleGroup's toggle :active, $active_color (_toggle-group.scss:69-70 at
        // libadwaita 1.10.0).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.active_layer)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(c.segment_selected)],
        )
        .build();
    // Hovered and pressed, the label as at rest: the toggle sets `color` only when :checked
    // (_toggle-group.scss:65-81).
    let rest_label = label(&typography.body, None);
    let (selectors, color) =
        rest_color_rules(&rest_label, [WidgetState::HOVER, WidgetState::PRESSED]);
    let label = RuleBuilder::new(rest_label)
        .rules(selectors, color)
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(c.segment_selected_foreground)],
        )
        .build();
    SegmentedControlWidget::new(
        WidgetOrigin::Native,
        track,
        segment,
        label,
        RuleBuilder::new(PartStyle::default()).build(),
        SEGMENT_SEPARATOR_WIDTH,
        // 0.7.2: `segmented_control.disabled_opacity` is `defaults.disabled_opacity` where
        // unstated.
        Some(roles.disabled_opacity),
    )
}

/// The card: its fill where stated, else the view's background (0.7.2:
/// `card.background_color` is `defaults.surface_color` where unstated), its
/// corners, and no outline.
fn card(c: &SchemeColors, roles: &ColorRoles) -> CardWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(c.card_background.or(roles.view.background).map(fill))
        // platform-facts.md (§2.26) border.color, border.line_width: none; `.card` sets no border,
        // its outline is the first of its box shadows (_misc.scss:201-208 at libadwaita 1.10.0)
        .base([corners(CARD_CORNER_RADIUS), PartProp::Stroke(Stroke::None)])
        .build();
    CardWidget::new(WidgetOrigin::Native, container)
}

/// The expander: the boxed list's outline and corners; a header of a least
/// height, padded, its hover fill; the arrow's size and colour, side and gap;
/// the content unindented; the frame drawn.
fn expander(c: &SchemeColors, typography: &Typography, direction: Direction) -> ExpanderWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([
            stroke(EXPANDER_BORDER_WIDTH, EXPANDER_BORDER),
            corners(EXPANDER_CORNER_RADIUS),
        ])
        .build();
    let header = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::MinSize([None, Some(EXPANDER_HEADER_HEIGHT)]),
            PartProp::Padding(physical_edges(
                Edges::symmetric(EXPANDER_PADDING_VERTICAL, EXPANDER_PADDING_HORIZONTAL),
                direction,
            )),
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.row_hover)])
        // Pressed: the header is a %boxed_list_row (_lists.scss:524-526), whose activatable
        // :active fill is the text at 8 % (_lists.scss:352-360 at libadwaita 1.10.0).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.view_active)],
        )
        .build();
    let arrow = RuleBuilder::new(IconStyle::default())
        .base([
            IconProp::Size(EXPANDER_ARROW_SIZE),
            IconProp::Color(c.window_foreground),
        ])
        .build();
    // Hovered and pressed, the title as at rest: %boxed_list_row's :hover and :active set only
    // background-color (_lists.scss:352-360).
    let title = label(&typography.body, None);
    let (selectors, color) = rest_color_rules(&title, [WidgetState::HOVER, WidgetState::PRESSED]);
    ExpanderWidget::new(
        WidgetOrigin::Native,
        container,
        header,
        RuleBuilder::new(title).rules(selectors, color).build(),
        arrow,
        Some(EXPANDER_ARROW_SIDE),
        Some(EXPANDER_ARROW_GAP),
        Some(EXPANDER_CONTENT_INDENT),
        Some(EXPANDER_FRAME_ENABLED),
    )
}

#[cfg(all(test, feature = "serde"))]
mod snapshots;
#[cfg(test)]
mod tests;
