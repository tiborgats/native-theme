//! The `adwaita` preset's builder (spec §6): GNOME's libadwaita, light and
//! dark.
//!
//! The builder runs in three steps: the preset's constants, one table per
//! scheme for the colours and plain constants for the rest; the colour roles
//! and the globals made from them; the widgets derived from the roles, the
//! globals and the colours. A widget's colour that is a role's colour is
//! taken from the roles, not from a second copy of the constant. The system's
//! settings ([`ResolveInputs::system`]), the active palette
//! ([`ResolveInputs::palette`]) and an explicit accent
//! ([`ResolveInputs::accent`]) are not read yet: what 0.7.2's portal reader
//! derives from the system is the next task of the plan's GNOME unit.
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
use crate::next::layout::{ArrowSide, DialogButtonOrder, LayoutMetrics, TabIndicatorSide};
use crate::next::motion::{CaretSpec, DialogEntranceStyle, MotionTokens, UiTimings};
use crate::next::paint::Paint;
use crate::next::preset::{Preset, ResolveInputs};
use crate::next::presets::helpers::{
    Motion, dialog_entrance, linear_font_px, motion, motion_tokens, physical_edges,
};
use crate::next::query::{
    ButtonVariant, Contrast, Direction, Orientation, Scheme, ThemeContext, WidgetState,
};
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
    // `checkbox.unchecked_border_color`, `separator.line_color`, `splitter.hover_color`,
    // `switch.unchecked_background`: 0.7.2 adwaita.toml (uncited)
    frame: srgb8(0xd5d5_d5ff),
    // `list.hover_background`, `expander.hover_background`: 0.7.2 adwaita.toml (uncited)
    row_hover: srgb8(0xebeb_edff),
    // `list.alternate_row_background`: 0.7.2 adwaita.toml (uncited)
    list_alternate_row: srgb8(0xf5f5_f5ff),
    // `input.placeholder_color`: 0.7.2 adwaita.toml (uncited)
    placeholder: srgb8(0x9295_95ff),
    // `menu.hover_background`, `tab.hover_background`: 0.7.2 adwaita.toml (uncited)
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
    // `slider.thumb_hover_color`: 0.7.2 adwaita.toml (uncited)
    slider_thumb_hover: srgb8(0xebeb_ebff),
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
    // `checkbox.unchecked_border_color`, `separator.line_color`, `splitter.hover_color`,
    // `switch.unchecked_background`: 0.7.2 adwaita.toml (uncited)
    frame: srgb8(0x4a4a_4eff),
    // `list.hover_background`, `expander.hover_background`: 0.7.2 adwaita.toml (uncited)
    row_hover: srgb8(0x3030_34ff),
    // `list.alternate_row_background`: 0.7.2 adwaita.toml (uncited)
    list_alternate_row: srgb8(0x2a2a_2eff),
    // `input.placeholder_color`: 0.7.2 adwaita.toml (uncited)
    placeholder: srgb8(0x8b8b_8bff),
    // `menu.hover_background`, `tab.hover_background`: 0.7.2 adwaita.toml (uncited)
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
    // `slider.thumb_hover_color`: 0.7.2 adwaita.toml (uncited)
    slider_thumb_hover: srgb8(0x3a3a_3eff),
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

/// The focus ring's alpha under `Contrast::High`: libadwaita's
/// `$focus_border_color_hc`, the standalone accent at 80 % where it draws
/// the ring at 50 % otherwise (libadwaita 1.10.0 `_colors.scss:261`, `:308`;
/// `_drawing.scss:24-26`), as 0.7.2's GNOME reader states it
/// (`src/gnome/accent.rs`, `focus_ring`).
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

// ---------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------

/// The contrast levels the preset defines (spec §1, "Axis values without
/// data"): `Normal`, and `High` for the focus ring's alpha, as the module
/// documentation says.
pub(crate) const CONTRAST_LEVELS: &[Contrast] = &[Contrast::Normal, Contrast::High];

/// The theme of `adwaita` for `ctx.scheme` and `ctx.contrast`, from the
/// preset's constants.
// `inputs` is not read yet: the system's settings, the palette's and the
// accent's derivations (`adw_standalone`, spec §6, resolution step 2; spec
// §8, "The GNOME part") are the next task of the plan's GNOME unit.
pub(crate) fn resolve(ctx: &ThemeContext, _inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let c = match ctx.scheme {
        Scheme::Light => &LIGHT,
        Scheme::Dark => &DARK,
    };
    let colors = roles(c, ctx.contrast);
    let typography = typography(c, ctx.text_scale);
    let units = FontUnits {
        dpi: typography.font_dpi,
        text_scale: ctx.text_scale,
    };
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
            theme: Some(Arc::from(ICON_THEME)),
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
            blink_on_ms: Some(CARET_BLINK_ON_MS),
            blink_off_ms: Some(CARET_BLINK_OFF_MS),
            blink_timeout_ms: Some(CARET_BLINK_TIMEOUT_MS),
            shape: None,
        },
        elevations: Arc::from([]),
        button: button(c, &colors, &typography, &layout),
        checkbox: checkbox(c, &colors, &typography, &layout),
        radio: radio(c, &colors, &typography, &layout),
        scrollbar: scrollbar(c, &colors),
        list: list(c, &colors, &typography, &layout),
        tab: tab(c, &colors, &typography),
        menu: menu(c, &colors, &typography, ctx.direction),
        menu_bar: menu_bar(&colors),
        sidebar: sidebar(c, &colors, &typography, &layout, ctx.direction),
        toolbar: toolbar(c, &colors, &typography, &layout),
        status_bar: status_bar(&colors, &typography, &layout, ctx.direction),
        tooltip: tooltip(&colors, &typography, &layout),
        window: window(c, &colors, &typography, &layout, ctx.window_active),
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
        splitter: splitter(c, &colors),
        separator: separator(c, &layout),
        spinner: spinner(&colors),
        popover: popover(c, &typography, &layout),
        switch: switch(c, &colors),
        dialog: dialog(&colors, &typography, units, &layout, ctx.direction, motion),
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
fn roles(c: &SchemeColors, contrast: Contrast) -> ColorRoles {
    let status = |fill, foreground| StatusColors {
        fill,
        foreground,
        standalone: None,
        origin: RoleOrigin::Native,
    };
    let unstated = SurfaceColors::default();
    ColorRoles {
        window: SurfaceColors::new(
            Some(c.window_background),
            Some(c.window_foreground),
            Some(c.window_foreground_muted),
            Some(c.window_border),
            None,
        ),
        view: SurfaceColors::new(Some(c.view_background), None, None, None, None),
        header: unstated,
        sidebar: unstated,
        card: unstated,
        dialog: unstated,
        popover: unstated,
        tooltip: SurfaceColors::new(
            Some(c.tooltip_background),
            Some(c.tooltip_foreground),
            None,
            None,
            None,
        ),
        osd: unstated,
        selection: SelectionColors::new(
            c.accent,
            c.accent_foreground,
            Some(c.accent),
            Some(c.accent_foreground),
        ),
        text_selection: SelectionColors::new(c.accent, c.accent_foreground, None, None),
        find_highlight: None,
        scrim: None,
        accent: AccentColors {
            fill: c.accent,
            foreground: c.accent_foreground,
            standalone: None,
        },
        focus_ring: match contrast {
            Contrast::High => translucent(c.focus_ring, FOCUS_RING_ALPHA_HIGH),
            Contrast::Normal | Contrast::Increased => c.focus_ring,
        },
        caret: c.window_foreground,
        link: c.link,
        // 0.7.2: `link.visited_text_color` is `defaults.link_color` where unstated.
        link_visited: None,
        disabled_text: c.window_foreground,
        shadow: c.shadow,
        disabled_opacity: DISABLED_OPACITY,
        error: status(c.error, c.error_foreground),
        warning: status(c.warning, c.warning_foreground),
        success: status(c.success, c.success_foreground),
        info: status(c.accent, c.accent_foreground),
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
/// (spec §4). Sizes convert at [`FONT_DPI`] and scale by `text_scale` (spec
/// §6, resolution step 2); each keeps its unscaled size as its
/// `defined_size`.
fn typography(c: &SchemeColors, text_scale: f32) -> Typography {
    let units = FontUnits {
        dpi: FONT_DPI,
        text_scale,
    };
    let text = c.window_foreground;
    let body_size = FontSize::Pt(FONT_SIZE_PT);
    let mut body = font(FONT_FAMILY, body_size, FONT_WEIGHT, text, units);
    body.line_height = line_height_px(body_size, units);
    body.stated |= TextStated::LINE_HEIGHT;
    Typography {
        display: Some(text_scale_entry(
            FontSize::Pt(DISPLAY_SIZE_PT),
            DISPLAY_WEIGHT,
            units,
        )),
        title1: Some(text_scale_entry(
            FontSize::Pt(TITLE1_SIZE_PT),
            TITLE1_WEIGHT,
            units,
        )),
        title2: None,
        title3: None,
        title4: None,
        headline: Some(text_scale_entry(body_size, HEADLINE_WEIGHT, units)),
        body,
        callout: None,
        subheadline: None,
        caption: Some(text_scale_entry(
            FontSize::Pt(CAPTION_SIZE_PT),
            FONT_WEIGHT,
            units,
        )),
        monospace: font(
            MONO_FAMILY,
            FontSize::Pt(MONO_SIZE_PT),
            MONO_WEIGHT,
            text,
            units,
        ),
        line_height_multiplier: LINE_HEIGHT,
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
        .rule(primary(), [fill(roles.accent.fill)])
        .build();
    let label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
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
/// pointer the hover fill; checked, the accent (0.7.2:
/// `checkbox.checked_background` is `defaults.accent_color` where unstated)
/// in the default border colour (`checkbox.border.color`). No disabled fill:
/// a disabled check is its enabled self at the disabled opacity
/// (`checkbox.unchecked_background`'s citation).
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
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill)]
                .into_iter()
                .chain(border(roles, layout.border_width)),
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
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base([
            IconProp::Color(roles.accent.foreground),
            IconProp::StrokeWidth(Some(CHECK_MARK_STROKE_WIDTH)),
        ])
        .build();
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator(c, roles, layout)
            .base([corners(CHECKBOX_CORNER_RADIUS)])
            .build(),
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
    let dot = RuleBuilder::new(PartStyle::default())
        .base([
            fill(roles.accent.foreground),
            PartProp::MinSize([Some(RADIO_DOT_SIZE); 2]),
            PartProp::MaxSize([Some(RADIO_DOT_SIZE); 2]),
        ])
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
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    let item_label = RuleBuilder::new(label(&typography.body, None))
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
/// the bar; the hovered and the pressed handle's own colours.
fn scrollbar(c: &SchemeColors, roles: &ColorRoles) -> ScrollbarWidget {
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
        SCROLLBAR_OVERLAY,
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
/// `window.title_bar_font.color` where unstated).
fn window(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
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
        .rule(
            RuleSelector::on(WidgetState::FOCUSED),
            [stroke(INPUT_BORDER_WIDTH, roles.focus_ring)],
        )
        .build();
    let text = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
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
    let label = RuleBuilder::new(label(&typography.body, Some(roles.link)))
        .base([TextProp::Decoration(TextDecoration {
            kind: LINK_DECORATION,
            color: None,
            thickness: None,
        })])
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
        .rule(
            RuleSelector::on(WidgetState::HOVER),
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

/// The splitter: its handle in the default border colour (0.7.2:
/// `splitter.divider_color` is `defaults.border.color` where unstated), its
/// hover colour under the pointer.
fn splitter(c: &SchemeColors, roles: &ColorRoles) -> SplitterWidget {
    let handle = RuleBuilder::new(PartStyle::default())
        .base(roles.window.border.map(fill))
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.frame)])
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
    let item_label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
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
/// padded, its hover fill, the selected tab's own fill
/// (`tab.active_background`); the window's text, which a hovered and a
/// selected tab keep (`tab.hover_text_color`, `tab.active_text_color`); the
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
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.menu_hover)])
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
    TabWidget::new(
        WidgetOrigin::Native,
        bar,
        tab,
        RuleBuilder::new(label(&typography.body, Some(c.window_foreground))).build(),
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
    ToolbarWidget::new(
        WidgetOrigin::Native,
        bar,
        button,
        RuleBuilder::new(label(&typography.body, None)).build(),
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
    let item = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.sidebar_hover)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    let item_label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
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
/// draws no outline").
fn switch(c: &SchemeColors, roles: &ColorRoles) -> SwitchWidget {
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
        .build();
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.switch_thumb_off.unwrap_or(c.switch_thumb)),
            PartProp::Stroke(Stroke::None),
            PartProp::MinSize([Some(SWITCH_THUMB_DIAMETER); 2]),
            PartProp::MaxSize([Some(SWITCH_THUMB_DIAMETER); 2]),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.switch_thumb)],
        )
        .build();
    SwitchWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        RuleBuilder::new(TextStyle::default()).build(),
        None,
        None,
        // 0.7.2: `switch.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The dialog: its panel in the window's background (0.7.2's
/// `dialog.background_color` where unstated, `resolve_safety_nets`) with the
/// default border (`resolve_border_inheritance`), its own corners, padding
/// and size limits; its title in the UI font at its own size and weight, its
/// body in the UI font (0.7.2: `dialog.title_font` and `body_font` are
/// `defaults.font` where unstated).
fn dialog(
    roles: &ColorRoles,
    typography: &Typography,
    units: FontUnits,
    layout: &LayoutMetrics,
    direction: Direction,
    motion: Motion,
) -> DialogWidget {
    let mut panel = PartStyle::default();
    for prop in roles
        .window
        .background
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
    let title_size = FontSize::Pt(DIALOG_TITLE_SIZE_PT);
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
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.button_hover)])
        .build();
    let label = RuleBuilder::new(label(&typography.body, None))
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
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.button_hover)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(c.segment_selected)],
        )
        .build();
    let label = RuleBuilder::new(label(&typography.body, None))
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
/// `card.background_color` is `defaults.surface_color` where unstated), and
/// its corners.
fn card(c: &SchemeColors, roles: &ColorRoles) -> CardWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(c.card_background.or(roles.view.background).map(fill))
        .base([corners(CARD_CORNER_RADIUS)])
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
        .build();
    let arrow = RuleBuilder::new(IconStyle::default())
        .base([
            IconProp::Size(EXPANDER_ARROW_SIZE),
            IconProp::Color(c.window_foreground),
        ])
        .build();
    ExpanderWidget::new(
        WidgetOrigin::Native,
        container,
        header,
        RuleBuilder::new(label(&typography.body, None)).build(),
        arrow,
        Some(EXPANDER_ARROW_SIDE),
        Some(EXPANDER_ARROW_GAP),
        Some(EXPANDER_CONTENT_INDENT),
        Some(EXPANDER_FRAME_ENABLED),
    )
}

#[cfg(test)]
mod tests;
