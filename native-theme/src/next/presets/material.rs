//! The `material-3` preset's builder (spec §6): Material Design 3's baseline
//! theme, light and dark.
//!
//! The builder runs in three steps: the preset's constants, one table per
//! scheme for the colours and plain constants for the rest; the colour roles
//! and the globals made from them; the widgets derived from the roles, the
//! globals and the colours. A widget's colour that is a role's colour, the
//! same `md.sys.color` role, is taken from the roles, not from a second copy
//! of the constant. The preset has no reader.
//!
//! The active palette ([`ResolveInputs::palette`]) and then an explicit
//! accent ([`ResolveInputs::accent`]) are laid over the constants (spec §6,
//! resolution step 2, layer 3). An explicit accent, or else a palette's
//! `accent`, is the seed of Material's dynamic scheme ("`material-3` runs
//! `m3_role` HCT"; [`m3_role`]), at its standard contrast level, in the
//! resolved scheme: each `md.sys.color` role the builder reads is the
//! scheme's role of that name ([`lay_seed_over`]), `surface`, `on-surface`,
//! `on-surface-variant`, `outline`, `outline-variant`, `surface-variant`,
//! `primary`, `on-primary`, `secondary`, `on-secondary`,
//! `secondary-container`, `on-secondary-container`, the three surface
//! containers, `inverse-surface`, `inverse-on-surface`, `error` and
//! `on-error`, and so every widget colour the preset takes from them, the
//! accent's fill (`primary`) and its text among them. A colour that is no
//! role of the scheme keeps the preset's: the shadow, Material's `shadow`
//! at the key shadow's opacity (the scheme's `shadow` is its tone 0, black
//! whatever the seed); the warning's and the success's colours, which
//! Material has no role for; the state colours 0.7.2 states without a
//! source (the expander's hover layer, the link's hover layer, pressed and
//! disabled colours, the scroll bar's pressed handle).
//!
//! A palette states the roles its slots name (spec §7, "The roles a palette
//! states"), and each slot whose Material colour is the colour of the
//! slot's role replaces that colour and every widget colour the preset takes
//! from it ([`lay_palette_over`]), over the seed's: `window_bg` is
//! `surface`, the window's background (0.7.2's `defaults.background_color`),
//! so also every panel and bar the preset fills with it, the text field's
//! fill and the list's alternate row; `foreground` is `on-surface`, the
//! window's text (`defaults.text_color`), in which the preset letters its
//! widgets, and the text of the eight surfaces (§7: one text colour);
//! `foreground_muted` is `on-surface-variant` (`defaults.muted_color`);
//! `border` is `outline` (`defaults.border.color`), so also the switch's off
//! thumb and outline and the scroll bar's hovered handle; `accent` and
//! `accent_fg` are `primary` and `on-primary`, laid as given; `error` is
//! `error`; `warning` and `success` are the warning's and the success's
//! colours. A slot with no Material colour of its role states its role
//! alone: `view_bg`, `header_bg`, `sidebar_bg` and `divider`; `surface_bg`,
//! the card's, the dialog's and the popover's, so their panels; `tooltip_bg`,
//! the tooltip's, so its panel; `selection_bg` and `selection_fg`, the
//! selection's, which the unfocused and the text selection take as the
//! preset takes them from the selection, so the list's and the sidebar's
//! selected rows; `link`, `caret` and `info` (the info role's text is
//! `on-primary`, as the preset's is). Under a seed the state colours the
//! preset states as a role at an opacity are derived again from the roles
//! as laid ([`derive_state_layers`]; spec §6: a derivation computes again
//! what reads a colour a slot replaced): `on-surface` at the disabled
//! container, the disabled content and the hover opacities,
//! `surface-container-highest` at the disabled track opacity, and the tonal
//! button's hovered and pressed fills, `on-secondary-container` at the hover
//! and the pressed opacities over `secondary-container`. Every other colour
//! keeps the seed's: the tonal button's fills and text, the segmented
//! control's selected segment, the menu's panel and separator, the outlines
//! in `outline-variant`, the focus ring (`secondary`). An explicit accent
//! over a palette replaces its `accent` and `accent_fg`, which are then the
//! scheme's `primary` and `on-primary` of the explicit seed; the palette's
//! other slots stand (spec §6: a derivation never replaces a colour a
//! palette states in a slot of its own).
//!
//! The colours are the baseline scheme's `md.sys.color` roles (the header of
//! 0.7.2's `material-3.toml`): a state layer, a role at an `md.sys.state`
//! opacity, stated as the layer where it lies over a transparent or a
//! surface container (a hovered list row, a hovered menu item) and composited
//! over its container where 0.7.2 states the result (the tonal button's
//! hovered and pressed fills); a disabled colour as `on-surface` at the
//! disabled opacities, with its alpha.
//!
//! Defined by the preset (spec §1, "Axis values without data"): the variant
//! `ButtonVariant::Primary` (0.7.2's `primary_*` fields: the filled button),
//! a list's alternate rows (`ListKind::alternate`), the two orientations of
//! the scroll bar and of the slider's groove, no control size, and the
//! contrast level `Normal` alone. A query for anything else returns the
//! part's base style. A maximized or full-screen window has no data of its
//! own: its frame and title bar are the normal window's.
//!
//! A part's exclusive state group names only the flags that part has a rule
//! for: a state without data of its own returns what the query returns
//! without that state, so a flag the part does not style removes no other.
//! The groups: the button's body `PRESSED, HOVER`; every other part none.
//!
//! Where two rules can both match, the later one wins (spec §3, "State
//! combinations a source does not state"); the precedences, each with its
//! source:
//! - the button's body: the pressed fill over the hovered fill (0.7.2's
//!   pressed colours win over its hover colours), both over the checked fill,
//!   for which no source the builder reads states a hovered or a pressed look;
//!   the primary fill over all of them, as 0.7.2's connectors keep
//!   `primary_background` in every state; the disabled fill over every other,
//!   the primary one included, as 0.7.2's egui connector fills a disabled
//!   button, primary or not, with `button.disabled_background` where it is
//!   stated (`connectors/native-theme-egui/src/mapping.md`, its rows
//!   `button.disabled_background` and `button.primary_background`). Its
//!   label, in the same order: the checked text, the primary one, the
//!   disabled one over all (`mapping.md`, `button.disabled_text_color`);
//! - the check box's and the radio button's indicator: the checked fill and
//!   outline over the hovered fill, as 0.7.2's egui connector draws a checked
//!   box ("`checkbox.hover_background` is the unchecked box's hover fill, so
//!   the hovered entry holds the checked fill too":
//!   `connectors/native-theme-egui/src/style/variants.rs`); the disabled fill
//!   over both, checked or not (0.7.2's `checkbox.disabled_background`, which
//!   the table of homes compares at `DISABLED | CHECKED` too);
//! - the switch's track: the disabled off track over the checked one and the
//!   disabled on track (`DISABLED | CHECKED`) over every other; its thumb: the
//!   disabled thumb over the checked one (0.7.2's `switch.disabled_*`
//!   colours, stated for the disabled control whatever its value);
//! - the text field's and the combo box's outline: the focused outline over
//!   the hovered one, as Material's focused field keeps its focused outline
//!   under the pointer (`_md-comp-outlined-text-field.scss`, the focus tokens
//!   beside the hover tokens; 0.7.2 states the two apart);
//! - the scroll bar's handle: the pressed colour over the hovered one;
//! - the link's label: the pressed colour, the disabled colour over it;
//! - the list's row highlight, the sidebar's item, the tab and the segmented
//!   control's segment: the selected fill over the hovered fill, as 0.7.2's
//!   egui connector puts the selection's colour in the selection's fill and
//!   the hover colour in the hovered widgets' fill
//!   (`connectors/native-theme-egui/src/mapping.md`) and egui fills a
//!   selected item with the selection's fill over its hovered look (egui
//!   0.37.0 `src/style.rs`, `Style::interact_selectable`). Their labels: the
//!   selected colour over the hovered one, for the same reason, and on the
//!   list the disabled colour over both.
//!
//! The fill under an outline (spec §2, "Fill and stroke";
//! `.superpowers/sdd/v0.8.0-plan/M2/fill-stroke-decision.md`): no source the
//! builder reads gives Material's extents, so every outline is
//! `StrokeAlign::Inside`, the fill under the whole of it.
//!
//! An inactive window (`ThemeContext::window_active` false) has the inactive
//! title bar (`window.inactive_title_bar_background`) and the inactive title
//! colour (`window.inactive_title_bar_text_color`), which 0.7.2 states equal
//! to the active ones: Material has no window-focus state.
//!
//! Of the context it acts on the scheme and on the direction: a layout from
//! right to left places every per-side value on the mirrored side through the
//! shared helper ([`physical_edges`]); the preset states no per-corner
//! radius. The text scale is Android's `FontScaleConverter` (spec §6,
//! resolution step 2; [`android_non_linear`]): every font size, a logical
//! pixel being Android's dp and a size at a scale of one its sp, through the
//! converter of the text scale, not rounded (0.7.2 rounds a size only where
//! the preset states `font_whole_pixels`, which `material-3.toml` does not);
//! a line height through the same converter up to Compose's threshold and
//! above it in proportion to its font's change ([`line_height_px`]); each
//! `defined_size` stays the unscaled size. The motion preference:
//! `MotionPreference::None` sets the dialog's entrance to
//! `DialogEntranceStyle::None` and would collapse every transition, and the
//! preset states no transition and no motion token; no source the builder
//! reads states a policy of Material's for `MotionPreference::Reduced`, which
//! therefore changes nothing. The contrast and the accessibility flags change
//! nothing here.
//!
//! Sources: `material-3.toml` of 0.7.2 (`src/presets/material-3.toml`), each
//! constant with the citation the TOML gives it (`docs/platform-facts.md` by
//! section, the upstream file) or `(uncited)`. The short names its citations
//! use: a token file named without a path (`_md-comp-filled-tonal-button.scss`,
//! `_md-sys-color.scss`, `_md-ref-palette.scss`) is material-web's at
//! cbd34a8921915af94d5ef65c2a69eece41d5b4f3, in `tokens/versions/v0_192/`,
//! the token set its components use (the TOML's header); `latest/sass/` is
//! that commit's `tokens/versions/latest/sass/`, the 34.0.21 set, and
//! `tokens/` a file of `tokens/` itself; a path such as
//! `button/internal/_shared.scss` or `elevation/internal/_elevation.scss` is
//! material-web's at the same commit; `m3_motion_fade_enter.xml` and
//! `MaterialAlertDialog` are material-components-android 1.14.0's
//! (66c334b7946dabf33adfe1a2b7cad6bcaa4ea3ad); a Compose file (`*.kt`) is
//! androidx's at the commit its citation names, `1e677685` being
//! 1e6776853023ebe4266aca2b308305723e3a2d57 (`docs/platform-facts.md`, the
//! Material column's pinned sources). 0.7.2's inheritance rules
//! (`src/resolve/inheritance.rs`, `src/resolve/validate.rs`,
//! `src/resolve/validate_helpers.rs`; `docs/inheritance-rules.toml`) where
//! 0.7.2 fills a value the TOML does not state.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{Corners, Edges, Stroke};
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::{DialogButtonOrder, LayoutMetrics, TabIndicatorSide};
use crate::next::motion::{CaretSpec, DialogEntranceStyle, MotionTokens, UiTimings};
use crate::next::paint::Paint;
use crate::next::palette::Palette;
use crate::next::platform::android_non_linear::{
    PROPORTIONAL_LINE_HEIGHT_ABOVE, android_non_linear, android_non_linear_inverse,
};
use crate::next::platform::m3_role::m3_role;
use crate::next::preset::{Preset, ResolveInputs};
use crate::next::presets::helpers::{
    Motion, dialog_entrance, motion, motion_tokens, physical_edges,
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

/// The colours of one scheme the builder works from: the constants
/// `material-3.toml` states, each colour from its 8-bit sRGB channels
/// ([`srgb8`]). A key 0.7.2 states with the same colour in both schemes as
/// another key's, for the same `md.sys.color` role or the same state layer,
/// shares that key's field; each key's citation stands at the field.
#[derive(Clone, Copy)]
struct SchemeColors {
    surface: Color,
    on_surface: Color,
    on_surface_variant: Color,
    outline: Color,
    outline_variant: Color,
    surface_variant: Color,
    primary: Color,
    on_primary: Color,
    secondary: Color,
    on_secondary: Color,
    secondary_container: Color,
    on_secondary_container: Color,
    surface_container: Color,
    surface_container_high: Color,
    surface_container_highest: Color,
    inverse_surface: Color,
    inverse_on_surface: Color,
    /// `on-surface` at the disabled container opacity, 0.12.
    disabled_container: Color,
    /// `on-surface` at the disabled content opacity, 0.38.
    disabled_content: Color,
    /// `on-surface` at the hover state layer's opacity, 0.08.
    hover_layer: Color,
    /// `surface-container-highest` at the disabled track opacity, 0.12.
    disabled_track: Color,
    button_hover: Color,
    button_pressed: Color,
    expander_hover: Color,
    link_pressed: Color,
    link_disabled: Color,
    link_hover_layer: Color,
    scrollbar_thumb_pressed: Color,
    shadow: Color,
    error: Color,
    on_error: Color,
    warning: Color,
    on_warning: Color,
    success: Color,
    on_success: Color,
}

/// The light scheme's constants.
const LIGHT: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md (§2.1.3): md.sys.color.surface = background,
    // neutral98 (_md-sys-color.scss:120, :84; _md-ref-palette.scss:66)
    // `defaults.surface_color`: surface, neutral98 (_md-sys-color.scss:120; _md-ref-palette.scss:66)
    // `input.background_color`: platform-facts.md (§2.4): the outlined field has no container, the
    // surface shows through
    // `list.alternate_row_background`: platform-facts.md (§2.15): (none), M3 has no row striping:
    // the alternate row is the list's surface (_md-comp-list.scss:39)
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): (none), M3 has no
    // window-focus state; nearest the top app bar's surface and on-surface
    // (_md-comp-top-app-bar-small.scss:34, :38)
    surface: srgb8(0xfef7_ffff),
    // `defaults.text_color`: on-surface, neutral10 (_md-sys-color.scss:102; _md-ref-palette.scss:45)
    // `combo_box.hover_border_color`: platform-facts.md (§2.24): text-field-hover-outline-color =
    // on-surface (_md-comp-outlined-select.scss:168-169)
    // `input.font.color`: platform-facts.md (§2.4): input-text-color = on-surface
    // (_md-comp-outlined-text-field.scss:93); input-text = body-large 16 (:101)
    // `input.hover_border_color`: platform-facts.md (§2.4): hover-outline-color = on-surface
    // (_md-comp-outlined-text-field.scss:87)
    // `list.hover_text_color`: list-item-hover-label-text-color = on-surface (_md-comp-list.scss:81)
    // `menu.hover_text_color`: platform-facts.md (§2.6): list-item-hover-label-text-color =
    // on-surface (latest/sass/_md-comp-menu.scss:108)
    // `popover.font.color`: platform-facts.md (§2.16): ← defaults.font.color, on-surface
    // `tab.hover_text_color`: inactive-hover-label-text-color = on-surface
    // (_md-comp-primary-navigation-tab.scss:95)
    // `window.inactive_title_bar_text_color`: 0.7.2 material-3.toml (uncited)
    on_surface: srgb8(0x1d1b_20ff),
    // `defaults.muted_color`: 0.7.2 material-3.toml (uncited)
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): unselected-outline-color =
    // on-surface-variant (_md-comp-checkbox.scss:118)
    // `dialog.body_font.color`: platform-facts.md (§2.22): supporting-text-color =
    // on-surface-variant (_md-comp-dialog.scss:97)
    // `expander.arrow_color`: 0.7.2 material-3.toml (uncited)
    // `list.header_font.color`: 0.7.2 material-3.toml (uncited)
    // `sidebar.font.color`: platform-facts.md (§2.12): inactive-label-text-color =
    // on-surface-variant (_md-comp-navigation-drawer.scss:106); label-text = label-large, weight
    // 500 (:131)
    // `tab.font.color`: platform-facts.md (§2.11): with-label-text-inactive-label-text-color =
    // on-surface-variant (_md-comp-primary-navigation-tab.scss:97)
    on_surface_variant: srgb8(0x4945_4fff),
    // `defaults.border.color`: 0.7.2 material-3.toml (uncited)
    // `scrollbar.thumb_hover_color`: 0.7.2 material-3.toml (uncited)
    // `switch.unchecked_thumb_background`: platform-facts.md (§2.21): unselected-handle-color =
    // outline (_md-comp-switch.scss:105)
    // `switch.unchecked_track_border_color`: platform-facts.md (§2.21):
    // unselected-track-outline-color = outline #79747e (_md-comp-switch.scss:137), drawn round the
    // unselected track only; hover, focus and pressed the same (platform-facts.md)
    outline: srgb8(0x7974_7eff),
    // `list.grid_color`: platform-facts.md (§2.15): row-item-outline-color = outline-variant,
    // neutral-variant80 (_md-comp-data-table.scss:84; _md-sys-color.scss:109;
    // _md-ref-palette.scss:40)
    // `separator.line_color`: 0.7.2 material-3.toml (uncited)
    outline_variant: srgb8(0xcac4_d0ff),
    // `menu.separator_color`: platform-facts.md (§2.6): divider-color = surface-variant,
    // neutral-variant90 (latest/sass/_md-comp-menu.scss:76; _md-sys-color.scss:129;
    // _md-ref-palette.scss:41)
    surface_variant: srgb8(0xe7e0_ecff),
    // `defaults.accent_color`: md.sys.color.primary, primary40 (_md-sys-color.scss:110,
    // _md-ref-palette.scss:73)
    // `defaults.info_color`, `defaults.link_color`: 0.7.2 material-3.toml (uncited)
    // `checkbox.border.color`: platform-facts.md (§2.5): a checked box has no outline
    // (selected-outline-width 0, _md-comp-checkbox.scss:80), so the line is its fill,
    // selected-container-color primary (:43), and the checked radio's ring is primary
    // (platform-facts.md)
    // `input.caret_color`: platform-facts.md (§2.4): caret-color = primary
    // (_md-comp-outlined-text-field.scss:31)
    // `input.focus_border_color`: 0.7.2 material-3.toml (uncited)
    // `link.hover_text_color`: 0.7.2 material-3.toml (uncited)
    // `slider.thumb_color`: md.comp.slider.handle.color = md.sys.color.primary, primary40
    // (material-web cbd34a8, tokens/versions/v0_192/_md-comp-slider.scss:52, _md-sys-color.scss:110,
    // _md-ref-palette.scss:73)
    // `slider.thumb_hover_color`: 0.7.2 material-3.toml (uncited)
    // `splitter.hover_color`: 0.7.2 material-3.toml (uncited)
    // `switch.hover_checked_background`: selected-hover-track-color = primary
    // (_md-comp-switch.scss:73)
    // `tab.active_indicator_color`: platform-facts.md (§2.11): active-indicator-color = primary,
    // active-indicator-height 3 (_md-comp-primary-navigation-tab.scss:37-38), along the bottom
    // (tabs/internal/_tab.scss:92-102)
    // `tab.active_text_color`: 0.7.2 material-3.toml (uncited)
    primary: srgb8(0x6750_a4ff),
    // `defaults.accent_text_color`, `defaults.info_text_color`: 0.7.2 material-3.toml (uncited)
    // `switch.thumb_background`: platform-facts.md (§2.21): selected-handle-color = on-primary
    // (_md-comp-switch.scss:62)
    on_primary: srgb8(0xffff_ffff),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): md-focus-ring color = secondary,
    // secondary40 (tokens/_md-comp-focus-ring.scss:47; _md-sys-color.scss:115;
    // _md-ref-palette.scss:86)
    // `button.checked_background`: platform-facts.md (§2.3): latest comp.button.tonal
    // selected-container-color = secondary, selected-label-text-color = on-secondary
    // (_md-comp-button-tonal.scss:79, :95)
    secondary: srgb8(0x625b_71ff),
    // `button.checked_text_color`: 0.7.2 material-3.toml (uncited)
    on_secondary: srgb8(0xffff_ffff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): list and menu selection,
    // secondary-container, secondary90 (_md-sys-color.scss:116; _md-ref-palette.scss:91) and
    // on-secondary-container, secondary10 (:98; :82)
    // `button.background_color`: 0.7.2 material-3.toml (uncited)
    // `segmented_control.active_background`: platform-facts.md (§2.25): selected-container-color =
    // secondary-container, selected-label-text-color = on-secondary-container
    // (_md-comp-outlined-segmented-button.scss:69, :83)
    secondary_container: srgb8(0xe8de_f8ff),
    // `defaults.selection_text_color`, `button.font.color`, `button.hover_text_color`,
    // `button.active_text_color`, `segmented_control.active_text_color`: 0.7.2 material-3.toml
    // (uncited)
    on_secondary_container: srgb8(0x1d19_2bff),
    // `menu.background_color`: platform-facts.md (§2.6): container-color = surface-container,
    // neutral94 (_md-comp-menu.scss:31; _md-sys-color.scss:122; _md-ref-palette.scss:63)
    // `popover.background_color`: platform-facts.md (§2.16): comp.menu container-color =
    // surface-container (_md-comp-menu.scss:31)
    surface_container: srgb8(0xf3ed_f7ff),
    // `dialog.background_color`: platform-facts.md (§2.22): container-color =
    // surface-container-high, neutral92 (_md-comp-dialog.scss:73; _md-sys-color.scss:123;
    // _md-ref-palette.scss:62)
    surface_container_high: srgb8(0xece6_f0ff),
    // `card.background_color`: platform-facts.md (§2.26): filled card container-color =
    // surface-container-highest (_md-comp-filled-card.scss:34)
    // `progress_bar.track_color`: platform-facts.md (§2.10): track-color = surface-container-highest
    // (_md-comp-linear-progress-indicator.scss:39)
    // `slider.track_color`: platform-facts.md (§2.9): inactive-track-color =
    // surface-container-highest, neutral90 (_md-comp-slider.scss:62; _md-sys-color.scss:124;
    // _md-ref-palette.scss:61)
    // `switch.hover_unchecked_background`: unselected-hover-track-color = surface-container-highest
    // (_md-comp-switch.scss:116)
    // `switch.unchecked_background`: platform-facts.md (§2.21): unselected-track-color =
    // surface-container-highest (_md-comp-switch.scss:135)
    surface_container_highest: srgb8(0xe6e0_e9ff),
    // `tooltip.background_color`: platform-facts.md (§2.7): container-color = inverse-surface,
    // neutral20 (_md-comp-plain-tooltip.scss:31; _md-sys-color.scss:89; _md-ref-palette.scss:49)
    inverse_surface: srgb8(0x322f_35ff),
    // `tooltip.font.color`: platform-facts.md (§2.7): supporting-text-color = inverse-on-surface,
    // neutral95 (_md-comp-plain-tooltip.scss:33; _md-sys-color.scss:87; _md-ref-palette.scss:64)
    inverse_on_surface: srgb8(0xf5ef_f7ff),
    // `button.disabled_background`: platform-facts.md (§2.3): on-surface at
    // disabled-container-opacity 0.12, label at 0.38 (_md-comp-filled-tonal-button.scss:42-46)
    // `slider.disabled_track_color`: 0.7.2 material-3.toml (uncited)
    // `switch.disabled_checked_background`: disabled-selected-track-color on-surface at
    // disabled-track-opacity 0.12 (_md-comp-switch.scss:35-37)
    // `switch.disabled_unchecked_track_border_color`: platform-facts.md (§2.21):
    // disabled-unselected-track-outline-color = on-surface (_md-comp-switch.scss:48-49) at
    // disabled-track-opacity 0.12 (:37)
    disabled_container: srgb8(0x1d1b_201f),
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): on-surface at
    // disabled-label-text-opacity 0.38 (_md-comp-filled-button.scss:45-46)
    // `button.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    // `checkbox.disabled_background`: platform-facts.md (§2.5): selected-disabled-container-color
    // on-surface at 0.38 (_md-comp-checkbox.scss:44-46)
    // `checkbox.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    // `combo_box.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    // `input.disabled_text_color`: platform-facts.md (§2.4): input text on-surface at 0.38
    // (_md-comp-outlined-text-field.scss:33-34)
    // `list.disabled_text_color`: list-item-disabled-label-text-color = on-surface at 0.38
    // (_md-comp-list.scss:43-45)
    // `menu.disabled_text_color`: platform-facts.md (§2.6): on-surface at 0.38
    // (latest/sass/_md-comp-menu.scss:92, :40)
    // `slider.disabled_fill_color`: platform-facts.md (§2.9): on-surface at 0.38 (active track,
    // handle) and 0.12 (inactive track) (_md-comp-slider.scss:40-47)
    // `slider.disabled_thumb_color`: 0.7.2 material-3.toml (uncited)
    // `switch.disabled_thumb_color`: disabled-unselected-handle-color on-surface at 0.38
    // (_md-comp-switch.scss:38-40)
    disabled_content: srgb8(0x1d1b_2061),
    // `checkbox.hover_background`: unselected-hover-state-layer-color = on-surface at hover opacity
    // 0.08 (_md-comp-checkbox.scss:114-116)
    // `list.hover_background`: platform-facts.md (§2.15): list-item-hover-state-layer-color =
    // on-surface at 0.08 (_md-comp-list.scss:85-87)
    // `menu.hover_background`: platform-facts.md (§2.6): list-item-hover-state-layer-color =
    // on-surface at 0.08 (tokens/versions/latest/sass/_md-comp-menu.scss:112-116)
    // `segmented_control.hover_background`: unselected-hover-state-layer-color = on-surface at 0.08
    // (_md-comp-outlined-segmented-button.scss:99-102)
    // `sidebar.hover_background`: platform-facts.md (§2.12): inactive-hover-state-layer-color =
    // on-surface at 0.08 (_md-comp-navigation-drawer.scss:92, :103)
    // `tab.hover_background`: with-label-text-inactive-hover: on-surface at hover opacity 0.08
    // (_md-comp-primary-navigation-tab.scss:59-61)
    hover_layer: srgb8(0x1d1b_2014),
    // `switch.disabled_unchecked_background`: disabled-unselected-track-color
    // surface-container-highest at disabled-track-opacity 0.12 (_md-comp-switch.scss:37, :46)
    disabled_track: srgb8(0xe6e0_e91f),
    // `button.hover_background`: platform-facts.md (§2.3): on-secondary-container at
    // hover-state-layer-opacity 0.08 over secondary-container (_md-comp-filled-tonal-button.scss:57-60)
    button_hover: srgb8(0xd8ce_e8ff),
    // `button.active_background`: on-secondary-container at pressed-state-layer-opacity 0.12 over
    // secondary-container (_md-comp-filled-tonal-button.scss:85-88; _md-sys-state.scss:20)
    button_pressed: srgb8(0xd0c6_dfff),
    // `expander.hover_background`: 0.7.2 material-3.toml (uncited)
    expander_hover: srgb8(0x1c1b_1f14),
    // `link.active_text_color`: 0.7.2 material-3.toml (uncited)
    link_pressed: srgb8(0x4f3d_8aff),
    // `link.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    link_disabled: srgb8(0xc4c7_c5ff),
    // `link.hover_background`: 0.7.2 material-3.toml (uncited)
    link_hover_layer: srgb8(0x6750_a418),
    // `scrollbar.thumb_active_color`: 0.7.2 material-3.toml (uncited)
    scrollbar_thumb_pressed: srgb8(0x625b_71ff),
    // `defaults.shadow_color`: md.sys.color.shadow #000000 (_md-sys-color.scss:119) as
    // material-web's key shadow renders it, opacity 0.3 (elevation/internal/_elevation.scss:104-105)
    shadow: srgb8(0x0000_004d),
    // `defaults.danger_color`: 0.7.2 material-3.toml (uncited)
    error: srgb8(0xb326_1eff),
    // `defaults.danger_text_color`: 0.7.2 material-3.toml (uncited)
    on_error: srgb8(0xffff_ffff),
    // `defaults.warning_color`: 0.7.2 material-3.toml (uncited)
    warning: srgb8(0x7d57_00ff),
    // `defaults.warning_text_color`: 0.7.2 material-3.toml (uncited)
    on_warning: srgb8(0xffff_ffff),
    // `defaults.success_color`: 0.7.2 material-3.toml (uncited)
    success: srgb8(0x386a_20ff),
    // `defaults.success_text_color`: 0.7.2 material-3.toml (uncited)
    on_success: srgb8(0xffff_ffff),
};

/// The dark scheme's constants.
const DARK: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md (§2.1.3): md.sys.color.surface = background,
    // neutral6 (_md-sys-color.scss:61, :25; _md-ref-palette.scss:56)
    // `defaults.surface_color`: surface, neutral6 (_md-sys-color.scss:61; _md-ref-palette.scss:56)
    // `input.background_color`: platform-facts.md (§2.4): the outlined field has no container, the
    // surface shows through
    // `list.alternate_row_background`: platform-facts.md (§2.15): (none), M3 has no row striping:
    // the alternate row is the list's surface (_md-comp-list.scss:39)
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): (none), M3 has no
    // window-focus state; nearest the top app bar's surface and on-surface
    // (_md-comp-top-app-bar-small.scss:34, :38)
    surface: srgb8(0x1412_18ff),
    // `defaults.text_color`: on-surface, neutral90 (_md-sys-color.scss:43; _md-ref-palette.scss:61)
    // `combo_box.hover_border_color`: platform-facts.md (§2.24): text-field-hover-outline-color =
    // on-surface (_md-comp-outlined-select.scss:168-169)
    // `input.font.color`: platform-facts.md (§2.4): input-text-color = on-surface
    // (_md-comp-outlined-text-field.scss:93); input-text = body-large 16 (:101)
    // `input.hover_border_color`: platform-facts.md (§2.4): hover-outline-color = on-surface
    // (_md-comp-outlined-text-field.scss:87)
    // `list.hover_text_color`: list-item-hover-label-text-color = on-surface (_md-comp-list.scss:81)
    // `menu.hover_text_color`: platform-facts.md (§2.6): list-item-hover-label-text-color =
    // on-surface (latest/sass/_md-comp-menu.scss:108)
    // `popover.font.color`: platform-facts.md (§2.16): ← defaults.font.color, on-surface
    // `tab.hover_text_color`: inactive-hover-label-text-color = on-surface
    // (_md-comp-primary-navigation-tab.scss:95)
    // `window.inactive_title_bar_text_color`: 0.7.2 material-3.toml (uncited)
    on_surface: srgb8(0xe6e0_e9ff),
    // `defaults.muted_color`: 0.7.2 material-3.toml (uncited)
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): unselected-outline-color =
    // on-surface-variant (_md-comp-checkbox.scss:118)
    // `dialog.body_font.color`: platform-facts.md (§2.22): supporting-text-color =
    // on-surface-variant (_md-comp-dialog.scss:97)
    // `expander.arrow_color`: 0.7.2 material-3.toml (uncited)
    // `list.header_font.color`: 0.7.2 material-3.toml (uncited)
    // `sidebar.font.color`: platform-facts.md (§2.12): inactive-label-text-color =
    // on-surface-variant (_md-comp-navigation-drawer.scss:106); label-text = label-large, weight
    // 500 (:131)
    // `tab.font.color`: platform-facts.md (§2.11): with-label-text-inactive-label-text-color =
    // on-surface-variant (_md-comp-primary-navigation-tab.scss:97)
    on_surface_variant: srgb8(0xcac4_d0ff),
    // `defaults.border.color`: 0.7.2 material-3.toml (uncited)
    // `scrollbar.thumb_hover_color`: 0.7.2 material-3.toml (uncited)
    // `switch.unchecked_thumb_background`: platform-facts.md (§2.21): unselected-handle-color =
    // outline (_md-comp-switch.scss:105)
    // `switch.unchecked_track_border_color`: platform-facts.md (§2.21):
    // unselected-track-outline-color = outline #938f99 (_md-comp-switch.scss:137), drawn round the
    // unselected track only; hover, focus and pressed the same (platform-facts.md)
    outline: srgb8(0x938f_99ff),
    // `list.grid_color`: platform-facts.md (§2.15): row-item-outline-color = outline-variant,
    // neutral-variant30 (_md-comp-data-table.scss:84; _md-sys-color.scss:50;
    // _md-ref-palette.scss:35)
    // `separator.line_color`: 0.7.2 material-3.toml (uncited)
    outline_variant: srgb8(0x4945_4fff),
    // `menu.separator_color`: platform-facts.md (§2.6): divider-color = surface-variant,
    // neutral-variant30 (latest/sass/_md-comp-menu.scss:76; _md-sys-color.scss:70;
    // _md-ref-palette.scss:35)
    surface_variant: srgb8(0x4945_4fff),
    // `defaults.accent_color`: md.sys.color.primary, primary80 (_md-sys-color.scss:51,
    // _md-ref-palette.scss:77)
    // `defaults.info_color`, `defaults.link_color`: 0.7.2 material-3.toml (uncited)
    // `checkbox.border.color`: platform-facts.md (§2.5): a checked box has no outline
    // (selected-outline-width 0, _md-comp-checkbox.scss:80), so the line is its fill,
    // selected-container-color primary (:43), and the checked radio's ring is primary
    // (platform-facts.md)
    // `input.caret_color`: platform-facts.md (§2.4): caret-color = primary
    // (_md-comp-outlined-text-field.scss:31)
    // `input.focus_border_color`: 0.7.2 material-3.toml (uncited)
    // `link.hover_text_color`: 0.7.2 material-3.toml (uncited)
    // `slider.thumb_color`: md.comp.slider.handle.color = md.sys.color.primary, primary80
    // (material-web cbd34a8, tokens/versions/v0_192/_md-comp-slider.scss:52, _md-sys-color.scss:51,
    // _md-ref-palette.scss:77)
    // `slider.thumb_hover_color`: 0.7.2 material-3.toml (uncited)
    // `splitter.hover_color`: 0.7.2 material-3.toml (uncited)
    // `switch.hover_checked_background`: selected-hover-track-color = primary
    // (_md-comp-switch.scss:73)
    // `tab.active_indicator_color`: platform-facts.md (§2.11): active-indicator-color = primary,
    // active-indicator-height 3 (_md-comp-primary-navigation-tab.scss:37-38), along the bottom
    // (tabs/internal/_tab.scss:92-102)
    // `tab.active_text_color`: 0.7.2 material-3.toml (uncited)
    primary: srgb8(0xd0bc_ffff),
    // `defaults.accent_text_color`, `defaults.info_text_color`: 0.7.2 material-3.toml (uncited)
    // `switch.thumb_background`: platform-facts.md (§2.21): selected-handle-color = on-primary
    // (_md-comp-switch.scss:62)
    on_primary: srgb8(0x381e_72ff),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): md-focus-ring color = secondary,
    // secondary80 (tokens/_md-comp-focus-ring.scss:47; _md-sys-color.scss:56;
    // _md-ref-palette.scss:90)
    // `button.checked_background`: platform-facts.md (§2.3): latest comp.button.tonal
    // selected-container-color = secondary, selected-label-text-color = on-secondary
    // (_md-comp-button-tonal.scss:79, :95)
    secondary: srgb8(0xccc2_dcff),
    // `button.checked_text_color`: 0.7.2 material-3.toml (uncited)
    on_secondary: srgb8(0x332d_41ff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): list and menu selection,
    // secondary-container, secondary30 (_md-sys-color.scss:57; _md-ref-palette.scss:85) and
    // on-secondary-container, secondary90 (:39; :91)
    // `button.background_color`: 0.7.2 material-3.toml (uncited)
    // `segmented_control.active_background`: platform-facts.md (§2.25): selected-container-color =
    // secondary-container, selected-label-text-color = on-secondary-container
    // (_md-comp-outlined-segmented-button.scss:69, :83)
    secondary_container: srgb8(0x4a44_58ff),
    // `defaults.selection_text_color`, `button.font.color`, `button.hover_text_color`,
    // `button.active_text_color`, `segmented_control.active_text_color`: 0.7.2 material-3.toml
    // (uncited)
    on_secondary_container: srgb8(0xe8de_f8ff),
    // `menu.background_color`: platform-facts.md (§2.6): container-color = surface-container,
    // neutral12 (_md-comp-menu.scss:31; _md-sys-color.scss:63; _md-ref-palette.scss:47)
    // `popover.background_color`: platform-facts.md (§2.16): comp.menu container-color =
    // surface-container (_md-comp-menu.scss:31)
    surface_container: srgb8(0x211f_26ff),
    // `dialog.background_color`: platform-facts.md (§2.22): container-color =
    // surface-container-high, neutral17 (_md-comp-dialog.scss:73; _md-sys-color.scss:64;
    // _md-ref-palette.scss:48)
    surface_container_high: srgb8(0x2b29_30ff),
    // `card.background_color`: platform-facts.md (§2.26): filled card container-color =
    // surface-container-highest (_md-comp-filled-card.scss:34)
    // `progress_bar.track_color`: platform-facts.md (§2.10): track-color = surface-container-highest
    // (_md-comp-linear-progress-indicator.scss:39)
    // `slider.track_color`: platform-facts.md (§2.9): inactive-track-color =
    // surface-container-highest, neutral22 (_md-comp-slider.scss:62; _md-sys-color.scss:65;
    // _md-ref-palette.scss:50)
    // `switch.hover_unchecked_background`: unselected-hover-track-color = surface-container-highest
    // (_md-comp-switch.scss:116)
    // `switch.unchecked_background`: platform-facts.md (§2.21): unselected-track-color =
    // surface-container-highest (_md-comp-switch.scss:135)
    surface_container_highest: srgb8(0x3634_3bff),
    // `tooltip.background_color`: platform-facts.md (§2.7): container-color = inverse-surface,
    // neutral90 (_md-comp-plain-tooltip.scss:31; _md-sys-color.scss:30; _md-ref-palette.scss:61)
    inverse_surface: srgb8(0xe6e0_e9ff),
    // `tooltip.font.color`: platform-facts.md (§2.7): supporting-text-color = inverse-on-surface,
    // neutral20 (_md-comp-plain-tooltip.scss:33; _md-sys-color.scss:28; _md-ref-palette.scss:49)
    inverse_on_surface: srgb8(0x322f_35ff),
    // `button.disabled_background`: platform-facts.md (§2.3): on-surface at
    // disabled-container-opacity 0.12, label at 0.38 (_md-comp-filled-tonal-button.scss:42-46)
    // `slider.disabled_track_color`: 0.7.2 material-3.toml (uncited)
    // `switch.disabled_checked_background`: disabled-selected-track-color on-surface at
    // disabled-track-opacity 0.12 (_md-comp-switch.scss:35-37)
    // `switch.disabled_unchecked_track_border_color`: platform-facts.md (§2.21):
    // disabled-unselected-track-outline-color = on-surface (_md-comp-switch.scss:48-49) at
    // disabled-track-opacity 0.12 (:37)
    disabled_container: srgb8(0xe6e0_e91f),
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): on-surface at
    // disabled-label-text-opacity 0.38 (_md-comp-filled-button.scss:45-46)
    // `button.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    // `checkbox.disabled_background`: platform-facts.md (§2.5): selected-disabled-container-color
    // on-surface at 0.38 (_md-comp-checkbox.scss:44-46)
    // `checkbox.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    // `combo_box.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    // `input.disabled_text_color`: platform-facts.md (§2.4): input text on-surface at 0.38
    // (_md-comp-outlined-text-field.scss:33-34)
    // `list.disabled_text_color`: list-item-disabled-label-text-color = on-surface at 0.38
    // (_md-comp-list.scss:43-45)
    // `menu.disabled_text_color`: platform-facts.md (§2.6): on-surface at 0.38
    // (latest/sass/_md-comp-menu.scss:92, :40)
    // `slider.disabled_fill_color`: platform-facts.md (§2.9): on-surface at 0.38 (active track,
    // handle) and 0.12 (inactive track) (_md-comp-slider.scss:40-47)
    // `slider.disabled_thumb_color`: 0.7.2 material-3.toml (uncited)
    // `switch.disabled_thumb_color`: disabled-unselected-handle-color on-surface at 0.38
    // (_md-comp-switch.scss:38-40)
    disabled_content: srgb8(0xe6e0_e961),
    // `checkbox.hover_background`: unselected-hover-state-layer-color = on-surface at hover opacity
    // 0.08 (_md-comp-checkbox.scss:114-116)
    // `list.hover_background`: platform-facts.md (§2.15): list-item-hover-state-layer-color =
    // on-surface at 0.08 (_md-comp-list.scss:85-87)
    // `menu.hover_background`: platform-facts.md (§2.6): list-item-hover-state-layer-color =
    // on-surface at 0.08 (tokens/versions/latest/sass/_md-comp-menu.scss:112-116)
    // `segmented_control.hover_background`: unselected-hover-state-layer-color = on-surface at 0.08
    // (_md-comp-outlined-segmented-button.scss:99-102)
    // `sidebar.hover_background`: platform-facts.md (§2.12): inactive-hover-state-layer-color =
    // on-surface at 0.08 (_md-comp-navigation-drawer.scss:92, :103)
    // `tab.hover_background`: with-label-text-inactive-hover: on-surface at hover opacity 0.08
    // (_md-comp-primary-navigation-tab.scss:59-61)
    hover_layer: srgb8(0xe6e0_e914),
    // `switch.disabled_unchecked_background`: disabled-unselected-track-color
    // surface-container-highest at disabled-track-opacity 0.12 (_md-comp-switch.scss:37, :46)
    disabled_track: srgb8(0x3634_3b1f),
    // `button.hover_background`: platform-facts.md (§2.3): on-secondary-container at
    // hover-state-layer-opacity 0.08 over secondary-container (_md-comp-filled-tonal-button.scss:57-60)
    button_hover: srgb8(0x5750_65ff),
    // `button.active_background`: on-secondary-container at pressed-state-layer-opacity 0.12 over
    // secondary-container (_md-comp-filled-tonal-button.scss:85-88; _md-sys-state.scss:20)
    button_pressed: srgb8(0x5d56_6bff),
    // `expander.hover_background`: 0.7.2 material-3.toml (uncited)
    expander_hover: srgb8(0xe6e1_e514),
    // `link.active_text_color`: 0.7.2 material-3.toml (uncited)
    link_pressed: srgb8(0xe8de_ffff),
    // `link.disabled_text_color`: 0.7.2 material-3.toml (uncited)
    link_disabled: srgb8(0x4945_4fff),
    // `link.hover_background`: 0.7.2 material-3.toml (uncited)
    link_hover_layer: srgb8(0xd0bc_ff18),
    // `scrollbar.thumb_active_color`: 0.7.2 material-3.toml (uncited)
    scrollbar_thumb_pressed: srgb8(0xaea9_b4ff),
    // `defaults.shadow_color`: md.sys.color.shadow #000000 (_md-sys-color.scss:60) as material-web's
    // key shadow renders it, opacity 0.3 (elevation/internal/_elevation.scss:104-105)
    shadow: srgb8(0x0000_004d),
    // `defaults.danger_color`: 0.7.2 material-3.toml (uncited)
    error: srgb8(0xf2b8_b5ff),
    // `defaults.danger_text_color`: 0.7.2 material-3.toml (uncited)
    on_error: srgb8(0x6014_10ff),
    // `defaults.warning_color`: 0.7.2 material-3.toml (uncited)
    warning: srgb8(0xe6c1_8eff),
    // `defaults.warning_text_color`: 0.7.2 material-3.toml (uncited)
    on_warning: srgb8(0x3e2e_00ff),
    // `defaults.success_color`: 0.7.2 material-3.toml (uncited)
    success: srgb8(0xb1d1_8aff),
    // `defaults.success_text_color`: 0.7.2 material-3.toml (uncited)
    on_success: srgb8(0x1b37_00ff),
};

// `link.background_color`: 0.7.2 material-3.toml (uncited)
// `checkbox.unchecked_background`: platform-facts.md (§2.5): comp.checkbox has no unselected
// container colour, the box is transparent (_md-comp-checkbox.scss:118-120)
const TRANSPARENT: Color = srgb8(0x0000_0000);

// `defaults.font.family`: 0.7.2 material-3.toml (uncited)
const FONT_FAMILY: &str = "Roboto";
// `defaults.font.size`, `defaults.font.defined_size`: 0.7.2 material-3.toml (uncited)
const FONT_SIZE_PX: f32 = 14.0;
// `defaults.font.weight`: 0.7.2 material-3.toml (uncited)
const FONT_WEIGHT: u16 = 400;
// `defaults.mono_font.family`: 0.7.2 material-3.toml (uncited)
const MONO_FAMILY: &str = "Roboto Mono";
// `defaults.mono_font.size`, `defaults.mono_font.defined_size`: 0.7.2 material-3.toml (uncited)
const MONO_SIZE_PX: f32 = 14.0;
// `defaults.mono_font.weight`: 0.7.2 material-3.toml (uncited)
const MONO_WEIGHT: u16 = 400;
// `defaults.line_height`: platform-facts.md (§2.1.1): body-medium-line-height 20 /
// body-medium-size 14 (_md-sys-typescale.scss:55-56)
const LINE_HEIGHT: f32 = 1.43;
// `text_scale.caption.size`: platform-facts.md (§2.19): md.sys.typescale.body-small, 12 / 400,
// line 16 (material-web cbd34a8, tokens/versions/v0_192/_md-sys-typescale.scss:72-75)
const CAPTION_SIZE_PX: f32 = 12.0;
// `text_scale.caption.weight`: 0.7.2 material-3.toml (uncited)
const CAPTION_WEIGHT: u16 = 400;
// `text_scale.caption.line_height`: 0.7.2 material-3.toml (uncited)
const CAPTION_LINE_HEIGHT_PX: f32 = 16.0;
// `text_scale.section_heading.size`: 0.7.2 material-3.toml (uncited)
const HEADLINE_SIZE_PX: f32 = 16.0;
// `text_scale.section_heading.weight`: 0.7.2 material-3.toml (uncited)
const HEADLINE_WEIGHT: u16 = 500;
// `text_scale.section_heading.line_height`: 0.7.2 material-3.toml (uncited)
const HEADLINE_LINE_HEIGHT_PX: f32 = 24.0;
// `text_scale.dialog_title.size`: 0.7.2 material-3.toml (uncited)
const TITLE1_SIZE_PX: f32 = 24.0;
// `text_scale.dialog_title.weight`: 0.7.2 material-3.toml (uncited)
const TITLE1_WEIGHT: u16 = 400;
// `text_scale.dialog_title.line_height`: 0.7.2 material-3.toml (uncited)
const TITLE1_LINE_HEIGHT_PX: f32 = 32.0;
// `text_scale.display.size`: 0.7.2 material-3.toml (uncited)
const DISPLAY_SIZE_PX: f32 = 36.0;
// `text_scale.display.weight`: 0.7.2 material-3.toml (uncited)
const DISPLAY_WEIGHT: u16 = 400;
// `text_scale.display.line_height`: 0.7.2 material-3.toml (uncited)
const DISPLAY_LINE_HEIGHT_PX: f32 = 44.0;

// `defaults.border.corner_radius`: 0.7.2 material-3.toml (uncited)
const CORNER_RADIUS: f32 = 12.0;
// `defaults.border.corner_radius_lg`: 0.7.2 material-3.toml (uncited)
const CORNER_RADIUS_LG: f32 = 16.0;
// `defaults.border.line_width`: 0.7.2 material-3.toml (uncited)
const BORDER_WIDTH: f32 = 1.0;
// `defaults.border.opacity`: 0.7.2 material-3.toml (uncited)
const BORDER_OPACITY: f32 = 0.12;
// `defaults.border.shadow_enabled`: 0.7.2 material-3.toml (uncited)
const SHADOW_ENABLED: bool = true;
// `defaults.disabled_opacity`: 1.0: Material 3 dims a disabled component by its colours alone, the
// container at disabled-container-opacity 0.12 and the label at disabled-label-text-opacity 0.38
// of on-surface (material-web cbd34a8, tokens/versions/v0_192/_md-comp-filled-button.scss:42-46;
// button/internal/_shared.scss:141-150), which this preset's disabled colours carry (#…1f, #…61)
const DISABLED_OPACITY: f32 = 1.0;
// `defaults.focus_ring_width`: platform-facts.md (§2.1.5): md-focus-ring width 3, outward-offset 2
// (tokens/_md-comp-focus-ring.scss:50, :52)
const FOCUS_RING_WIDTH: f32 = 3.0;
// `defaults.focus_ring_offset`: 0.7.2 material-3.toml (uncited)
const FOCUS_RING_OFFSET: f32 = 2.0;
// `defaults.focus_ring_keyboard_only`: platform-facts.md (§2.1.5): focus_ring_keyboard_only:
// Compose shows focus indication in keyboard input mode only (AndroidComposeView.android.kt:3807-3809,
// androidx e4d933c9)
const FOCUS_RING_KEYBOARD_ONLY: bool = true;

// `Resolved.icon_set`: 0.7.2 material-3.toml (uncited)
const ICON_SET: IconSet = IconSet::Material;
// `Resolved.icon_theme`: 0.7.2 material-3.toml (uncited)
const ICON_THEME: &str = "material";
// `defaults.icon_sizes.toolbar`: 0.7.2 material-3.toml (uncited)
const ICON_TOOLBAR: f32 = 24.0;
// `defaults.icon_sizes.small`: 0.7.2 material-3.toml (uncited)
const ICON_SMALL: f32 = 16.0;
// `defaults.icon_sizes.large`: 0.7.2 material-3.toml (uncited)
const ICON_LARGE: f32 = 32.0;
// `defaults.icon_sizes.dialog`: platform-facts.md (§2.1.8): comp.dialog with-icon-icon-size 24
// (_md-comp-dialog.scss:122)
const ICON_DIALOG: f32 = 24.0;
// `defaults.icon_sizes.panel`: platform-facts.md (§2.1.8): comp.navigation-drawer icon-size 24
// (_md-comp-navigation-drawer.scss:94)
const ICON_PANEL: f32 = 24.0;

// `layout.widget_gap`: 0.7.2 material-3.toml (uncited)
const WIDGET_GAP: f32 = 6.0;
// `layout.container_margin`: 0.7.2 material-3.toml (uncited)
const CONTAINER_MARGIN: f32 = 6.0;
// `layout.window_margin`: 0.7.2 material-3.toml (uncited)
const WINDOW_MARGIN: f32 = 10.0;
// `layout.section_gap`: 0.7.2 material-3.toml (uncited)
const SECTION_GAP: f32 = 18.0;

// `tooltip.show_delay`: platform-facts.md (§2.7): 0, shown on pointer enter (Compose
// BasicTooltip.kt:271-276; Tooltip.kt:1075-1099)
const TOOLTIP_SHOW_MS: u32 = 0;
// `input.caret_blink_on`: platform-facts.md (§2.4): Compose shows the cursor 500 ms
// (CursorAnimationState.kt:76-83 at androidx e4d933c9)
const CARET_BLINK_ON_MS: u32 = 500;
// `input.caret_blink_off`: platform-facts.md (§2.4): and hides it 500 ms
const CARET_BLINK_OFF_MS: u32 = 500;

// `button.min_width`: 0.7.2 material-3.toml (uncited)
const BUTTON_MIN_WIDTH: f32 = 64.0;
// `button.min_height`: 0.7.2 material-3.toml (uncited)
const BUTTON_MIN_HEIGHT: f32 = 40.0;
// `button.icon_text_gap`: 0.7.2 material-3.toml (uncited)
const BUTTON_ICON_TEXT_GAP: f32 = 8.0;
// `button.font.weight`: platform-facts.md (§2.3): label-text = label-large, weight 500
// (_md-comp-filled-tonal-button.scss:80; _md-sys-typescale.scss:195)
const BUTTON_FONT_WEIGHT: u16 = 500;
// `button.border.corner_radius`: platform-facts.md (§2.3): container-shape corner-full, a pill:
// half the 40 container-height (_md-comp-filled-tonal-button.scss:39, :41)
const BUTTON_CORNER_RADIUS: f32 = 20.0;
// `button.border.line_width`: platform-facts.md (§2.3): the tonal button has no outline token
// (_md-comp-filled-tonal-button.scss:37-99)
const BUTTON_BORDER_WIDTH: f32 = 0.0;
// `button.border.padding.top`, `button.border.padding.bottom`: 0.7.2 material-3.toml (uncited)
const BUTTON_PADDING_VERTICAL: f32 = 10.0;
// `button.border.padding.left`, `button.border.padding.right`: platform-facts.md (§2.3):
// material-web leading-space / trailing-space 24 (cbd34a8, tokens/_md-comp-filled-tonal-button.scss:96-105);
// padding-block (container-height 40 − line 20) / 2 = 10 (button/internal/_shared.scss:31-32)
const BUTTON_PADDING_HORIZONTAL: f32 = 24.0;

// `checkbox.indicator_width`: 0.7.2 material-3.toml (uncited)
const CHECKBOX_INDICATOR_SIZE: f32 = 18.0;
// `checkbox.radio_indicator_width`: platform-facts.md (§2.5): comp.radio-button icon-size 20
// (material-web cbd34a8, tokens/versions/v0_192/_md-comp-radio-button.scss:34)
const RADIO_INDICATOR_SIZE: f32 = 20.0;
// `checkbox.radio_dot_diameter`: platform-facts.md (§2.5): material-web radio.ts:119, the inner
// circle r = 5 (cbd34a8)
const RADIO_DOT_SIZE: f32 = 10.0;
// `checkbox.label_gap`: 0.7.2 material-3.toml (uncited)
const CHECKBOX_LABEL_GAP: f32 = 8.0;
// `checkbox.border.corner_radius`: platform-facts.md (§2.5): container-shape 2
// (_md-comp-checkbox.scss:31)
const CHECKBOX_CORNER_RADIUS: f32 = 2.0;
// `checkbox.border.line_width`: platform-facts.md (§2.5): unselected-outline-width 2
// (_md-comp-checkbox.scss:120)
const CHECKBOX_BORDER_WIDTH: f32 = 2.0;
// `checkbox.check_mark_stroke_width`: platform-facts.md (§2.5): material-web $_mark-stroke: 2px
// (cbd34a8, checkbox/internal/_checkbox.scss:18-19)
const CHECK_MARK_STROKE_WIDTH: f32 = 2.0;

// `list.border.padding.left`, `list.border.padding.right`: platform-facts.md (§2.15): md.comp.list
// list-item-leading-space / list-item-trailing-space 16 (material-web cbd34a8,
// tokens/versions/v0_192/_md-comp-list.scss:152, :225)
const LIST_ROW_PADDING_HORIZONTAL: f32 = 16.0;
// `list.header_height`: platform-facts.md (§2.15): md.comp.data-table.header.container.height 56
// (_md-comp-data-table.scss:50)
const LIST_HEADER_HEIGHT: f32 = 56.0;
// `list.header_font.weight`: platform-facts.md (§2.15): header-headline = title-small, weight 500;
// header-headline-color = on-surface-variant (_md-comp-data-table.scss:51, :61)
const LIST_HEADER_WEIGHT: u16 = 500;

// `scrollbar.groove_width`: 0.7.2 material-3.toml (uncited)
const SCROLLBAR_GROOVE_WIDTH: f32 = 12.0;
// `scrollbar.min_thumb_length`: 0.7.2 material-3.toml (uncited)
const SCROLLBAR_MIN_THUMB_LENGTH: f32 = 30.0;
// `scrollbar.thumb_width`: 0.7.2 material-3.toml (uncited)
const SCROLLBAR_THUMB_WIDTH: f32 = 4.0;
// `scrollbar.overlay_mode`: 0.7.2 material-3.toml (uncited)
const SCROLLBAR_OVERLAY: bool = false;

// `tooltip.font.size`, `tooltip.font.defined_size`: platform-facts.md (§2.7): supporting-text-size =
// body-small-size 12 (_md-comp-plain-tooltip.scss:39)
const TOOLTIP_FONT_SIZE_PX: f32 = 12.0;
// `tooltip.border.corner_radius`: platform-facts.md (§2.7): container-shape corner-extra-small 4
// (_md-comp-plain-tooltip.scss:32)
const TOOLTIP_CORNER_RADIUS: f32 = 4.0;
// `tooltip.border.line_width`: platform-facts.md (§2.7): no outline token
const TOOLTIP_BORDER_WIDTH: f32 = 0.0;
// `tooltip.border.padding.top`, `tooltip.border.padding.bottom`: 0.7.2 material-3.toml (uncited)
const TOOLTIP_PADDING_VERTICAL: f32 = 4.0;
// `tooltip.border.padding.left`, `tooltip.border.padding.right`: platform-facts.md (§2.7): Compose
// PlainTooltipHorizontalPadding 8, PlainTooltipVerticalPadding 4 (androidx 1e677685,
// Tooltip.kt:1454-1459)
const TOOLTIP_PADDING_HORIZONTAL: f32 = 8.0;
// `tooltip.max_width`: platform-facts.md (§2.7): Compose plainTooltipMaxWidth 200 (androidx
// 1e677685, Tooltip.kt:535)
const TOOLTIP_MAX_WIDTH: f32 = 200.0;

// `input.min_height`: 0.7.2 material-3.toml (uncited)
const INPUT_MIN_HEIGHT: f32 = 56.0;
// `input.font.size`, `input.font.defined_size`: 0.7.2 material-3.toml (uncited)
const INPUT_FONT_SIZE_PX: f32 = 16.0;
// `input.border.corner_radius`: platform-facts.md (§2.4): container-shape corner-extra-small 4
// (_md-comp-outlined-text-field.scss:32)
const INPUT_CORNER_RADIUS: f32 = 4.0;
// `input.border.padding.top`, `input.border.padding.bottom`: 0.7.2 material-3.toml (uncited)
const INPUT_PADDING_VERTICAL: f32 = 15.0;
// `input.border.padding.left`, `input.border.padding.right`: platform-facts.md (§2.4):
// material-web outlined field leading-space / trailing-space 16, top-space / bottom-space 16 from
// the field's edge (cbd34a8, tokens/_md-comp-outlined-text-field.scss:139-142), less the 1 outline
// laid over it (field/internal/_outlined-field.scss:50-59, v0_192 outline-width :146)
const INPUT_PADDING_HORIZONTAL: f32 = 15.0;
// `text_area.border.padding.top`, `text_area.border.padding.bottom`: 0.7.2 material-3.toml (uncited)
const TEXT_AREA_PADDING_VERTICAL: f32 = 15.0;
// `text_area.border.padding.left`, `text_area.border.padding.right`: platform-facts.md (§2.29): the
// outlined field's 16; 16 from its edge (material-web cbd34a8,
// tokens/_md-comp-outlined-text-field.scss:139-142), less the 1 outline laid over it
const TEXT_AREA_PADDING_HORIZONTAL: f32 = 15.0;

// `link.underline_enabled`: 0.7.2 material-3.toml (uncited)
const LINK_DECORATION: DecorationKind = DecorationKind::None;

// `slider.track_height`: 0.7.2 material-3.toml (uncited)
const SLIDER_TRACK_HEIGHT: f32 = 4.0;
// `slider.thumb_diameter`: 0.7.2 material-3.toml (uncited)
const SLIDER_THUMB_DIAMETER: f32 = 20.0;
// `slider.tick_mark_length`: platform-facts.md (§2.9): with-tick-marks-container-size 2
// (_md-comp-slider.scss:108)
const SLIDER_TICK_LENGTH: f32 = 2.0;

// `progress_bar.track_height`: 0.7.2 material-3.toml (uncited)
const PROGRESS_BAR_TRACK_HEIGHT: f32 = 4.0;
// `progress_bar.min_width`: 0.7.2 material-3.toml (uncited)
const PROGRESS_BAR_MIN_WIDTH: f32 = 100.0;
// `progress_bar.border.corner_radius`: platform-facts.md (§2.10): track-shape /
// active-indicator-shape corner-none (_md-comp-linear-progress-indicator.scss:30, :41)
const PROGRESS_BAR_CORNER_RADIUS: f32 = 0.0;
// `progress_bar.border.line_width`: 0.7.2 material-3.toml (uncited)
const PROGRESS_BAR_BORDER_WIDTH: f32 = 0.0;

// `splitter.divider_width`: 0.7.2 material-3.toml (uncited)
const SPLITTER_DIVIDER_WIDTH: f32 = 4.0;

// `spinner.diameter`: platform-facts.md (§2.23): comp.circular-progress-indicator size 48
// (_md-comp-circular-progress-indicator.scss:39)
const SPINNER_DIAMETER: f32 = 48.0;
// `spinner.min_diameter`: 0.7.2 material-3.toml (uncited)
const SPINNER_MIN_DIAMETER: f32 = 16.0;
// `spinner.stroke_width`: platform-facts.md (§2.23): active-indicator-width 4
// (_md-comp-circular-progress-indicator.scss:30)
const SPINNER_STROKE_WIDTH: f32 = 4.0;

// `menu.icon_text_gap`: platform-facts.md (§2.6): md-menu-item gap: 16px (material-web cbd34a8,
// menu/internal/menuitem/_menu-item.scss:39)
const MENU_ICON_TEXT_GAP: f32 = 16.0;
// `menu.row_height`: platform-facts.md (§2.6): md.comp.menu list-item-container-height 48
// (material-web cbd34a8, tokens/versions/latest/sass/_md-comp-menu.scss:36;
// v0_192/_md-comp-outlined-select.scss:47)
const MENU_ROW_HEIGHT: f32 = 48.0;
// `menu.border.corner_radius`: platform-facts.md (§2.6): container-shape corner-extra-small 4
// (_md-comp-menu.scss:34)
const MENU_CORNER_RADIUS: f32 = 4.0;
// `menu.border.padding.left`, `menu.border.padding.right`: platform-facts.md (§2.6): md-menu-item
// takes the list item's leading-space / trailing-space 16 (material-web cbd34a8,
// tokens/versions/v0_192/_md-comp-list.scss:152, :225; tokens/_md-comp-menu-item.scss:93)
const MENU_ITEM_PADDING_HORIZONTAL: f32 = 16.0;
// `menu.font.weight`: platform-facts.md (§2.6): list-item-label-text = label-large, weight 500
// (latest/sass/_md-comp-menu.scss:140)
const MENU_FONT_WEIGHT: u16 = 500;

// `tab.min_width`: platform-facts.md (§2.11): fixed tabs share the width; scrollable tabs, Compose
// ScrollableTabRowMinTabWidth 90 (androidx 1e677685, TabRow.kt:981)
const TAB_MIN_WIDTH: f32 = 90.0;
// `tab.min_height`: 0.7.2 material-3.toml (uncited)
const TAB_MIN_HEIGHT: f32 = 48.0;
// `tab.border.padding.left`, `tab.border.padding.right`: platform-facts.md (§2.11): material-web tab
// padding: 0 16px (cbd34a8, tabs/internal/_tab.scss:23)
const TAB_PADDING_HORIZONTAL: f32 = 16.0;
// `tab.font.weight`: platform-facts.md (§2.11): with-label-text-label-text = title-small, weight
// 500 (_md-comp-primary-navigation-tab.scss:121)
const TAB_FONT_WEIGHT: u16 = 500;
// `tab.active_indicator_width`: 0.7.2 material-3.toml (uncited)
const TAB_INDICATOR_WIDTH: f32 = 3.0;
// `tab.active_indicator_side`: 0.7.2 material-3.toml (uncited)
const TAB_INDICATOR_SIDE: TabIndicatorSide = TabIndicatorSide::Bottom;

// `sidebar.border.padding.left`, `sidebar.border.padding.right`: platform-facts.md (§2.12): (impl)
// Compose 12, NavigationDrawerItemDefaults.ItemPadding = PaddingValues(horizontal = 12.dp)
// (androidx 1e677685, compose/NavigationDrawer.kt:1239-1243); the vertical is app-defined (:1381),
// so it is unstated
const SIDEBAR_PADDING_HORIZONTAL: f32 = 12.0;
// `sidebar.font.weight`: 0.7.2 material-3.toml (uncited)
const SIDEBAR_FONT_WEIGHT: u16 = 500;

// `toolbar.bar_height`: platform-facts.md (§2.13): md.comp.top-app-bar-small container-height 64
// (material-web cbd34a8, tokens/versions/v0_192/_md-comp-top-app-bar-small.scss:36)
const TOOLBAR_BAR_HEIGHT: f32 = 64.0;
// `toolbar.border.corner_radius`: platform-facts.md (§2.13): the small top app bar:
// container-shape corner-none, container-elevation level0, no outline
// (_md-comp-top-app-bar-small.scss:35-37)
const TOOLBAR_CORNER_RADIUS: f32 = 0.0;
// `toolbar.border.line_width`: 0.7.2 material-3.toml (uncited)
const TOOLBAR_BORDER_WIDTH: f32 = 0.0;
// `toolbar.font.size`, `toolbar.font.defined_size`: platform-facts.md (§2.13): headline-size =
// title-large-size 22 (_md-comp-top-app-bar-small.scss:42)
const TOOLBAR_FONT_SIZE_PX: f32 = 22.0;

// `popover.border.corner_radius`: platform-facts.md (§2.16): the menu's corner-extra-small 4
// (_md-comp-menu.scss:34)
const POPOVER_CORNER_RADIUS: f32 = 4.0;
// `popover.border.line_width`: platform-facts.md (§2.16): no outline
const POPOVER_BORDER_WIDTH: f32 = 0.0;
// `popover.border.padding.top`, `popover.border.padding.bottom`: 0.7.2 material-3.toml (uncited)
const POPOVER_PADDING_VERTICAL: f32 = 8.0;
// `popover.border.padding.left`, `popover.border.padding.right`: platform-facts.md (§2.16): the
// menu container pads 8 top and bottom, nothing at the sides (material-web cbd34a8,
// tokens/_md-comp-menu.scss:53-54)
const POPOVER_PADDING_HORIZONTAL: f32 = 0.0;

// `switch.track_width`: 0.7.2 material-3.toml (uncited)
const SWITCH_TRACK_WIDTH: f32 = 52.0;
// `switch.track_height`: 0.7.2 material-3.toml (uncited)
const SWITCH_TRACK_HEIGHT: f32 = 32.0;
// `switch.track_radius`: 0.7.2 material-3.toml (uncited)
const SWITCH_TRACK_RADIUS: f32 = 16.0;
// `switch.thumb_diameter`: 0.7.2 material-3.toml (uncited)
const SWITCH_THUMB_DIAMETER: f32 = 24.0;
// `switch.unchecked_thumb_diameter`: platform-facts.md (§2.21): unselected-handle-width / height 16
// (_md-comp-switch.scss:106-107)
const SWITCH_UNCHECKED_THUMB_DIAMETER: f32 = 16.0;
// `switch.track_border_line_width`: platform-facts.md (§2.21): track-outline-width 2
// (_md-comp-switch.scss:90), unselected only, so checked_track_border_color stays unset
// (platform-facts.md)
const SWITCH_TRACK_BORDER_WIDTH: f32 = 2.0;

// `dialog.min_width`: 0.7.2 material-3.toml (uncited)
const DIALOG_MIN_WIDTH: f32 = 280.0;
// `dialog.max_width`: 0.7.2 material-3.toml (uncited)
const DIALOG_MAX_WIDTH: f32 = 560.0;
// `dialog.min_height`: 0.7.2 material-3.toml (uncited)
const DIALOG_MIN_HEIGHT: f32 = 140.0;
// `dialog.max_height`: platform-facts.md (§2.22): material-web min(560px, calc(100% - 48px))
// (cbd34a8, dialog/internal/_dialog.scss:39)
const DIALOG_MAX_HEIGHT: f32 = 560.0;
// `dialog.button_gap`: 0.7.2 material-3.toml (uncited)
const DIALOG_BUTTON_GAP: f32 = 8.0;
// `dialog.icon_size`: platform-facts.md (§2.22): with-icon-icon-size 24 (_md-comp-dialog.scss:122)
const DIALOG_ICON_SIZE: f32 = 24.0;
// `dialog.button_order`: 0.7.2 material-3.toml (uncited)
const DIALOG_BUTTON_ORDER: DialogButtonOrder = DialogButtonOrder::PrimaryRight;
// `dialog.entrance`: platform-facts.md (§2.22): MaterialAlertDialog.Material3.Animation, alpha 0 →
// 1 with scale 0.8 → 1 (m3_motion_fade_enter.xml:17-31)
const DIALOG_ENTRANCE: DialogEntranceStyle = DialogEntranceStyle::ScaleFade;
// `dialog.title_font.size`, `dialog.title_font.defined_size`: platform-facts.md (§2.22):
// md.comp.dialog headline-size = headline-small-size 24, headline-small-weight 400 (material-web
// cbd34a8, tokens/versions/v0_192/_md-comp-dialog.scss:80, :95)
const DIALOG_TITLE_SIZE_PX: f32 = 24.0;
// `dialog.title_font.weight`: 0.7.2 material-3.toml (uncited)
const DIALOG_TITLE_WEIGHT: u16 = 400;
// `dialog.border.corner_radius`: platform-facts.md (§2.22): container-shape corner-extra-large 28
// (_md-comp-dialog.scss:75)
const DIALOG_CORNER_RADIUS: f32 = 28.0;
// `dialog.border.line_width`: platform-facts.md (§2.22): no outline token
const DIALOG_BORDER_WIDTH: f32 = 0.0;
// `dialog.border.padding.top`, `dialog.border.padding.bottom`: 0.7.2 material-3.toml (uncited)
const DIALOG_PADDING_VERTICAL: f32 = 24.0;
// `dialog.border.padding.left`, `dialog.border.padding.right`: platform-facts.md (§2.22):
// material-web dialog headline padding 24px 24px 0, actions 16px 24px 24px (cbd34a8,
// dialog/internal/_dialog.scss:111, :218)
const DIALOG_PADDING_HORIZONTAL: f32 = 24.0;

// `combo_box.min_height`: platform-facts.md (§2.24): the text field's height, container-height 56
// (tokens/versions/latest/sass/_md-comp-outlined-text-field.scss:26)
const COMBO_BOX_MIN_HEIGHT: f32 = 56.0;
// `combo_box.min_width`: platform-facts.md (§2.24): md-outlined-select min-width: 210px
// (material-web cbd34a8, select/internal/_shared.scss:19)
const COMBO_BOX_MIN_WIDTH: f32 = 210.0;
// `combo_box.arrow_icon_size`: platform-facts.md (§2.24): text-field-trailing-icon-size 24
// (_md-comp-outlined-select.scss:261)
const COMBO_BOX_ARROW_SIZE: f32 = 24.0;
// `combo_box.font.size`, `combo_box.font.defined_size`: platform-facts.md (§2.24):
// text-field-input-text = body-large 16 (_md-comp-outlined-select.scss:184)
const COMBO_BOX_FONT_SIZE_PX: f32 = 16.0;
// `combo_box.border.corner_radius`: platform-facts.md (§2.24): text-field-container-shape
// corner-extra-small 4 (_md-comp-outlined-select.scss:84)
const COMBO_BOX_CORNER_RADIUS: f32 = 4.0;
// `combo_box.border.padding.top`, `combo_box.border.padding.bottom`: 0.7.2 material-3.toml
// (uncited)
const COMBO_BOX_PADDING_VERTICAL: f32 = 15.0;
// `combo_box.border.padding.right`: 0.7.2 material-3.toml (uncited)
const COMBO_BOX_PADDING_RIGHT: f32 = 11.0;
// `combo_box.border.padding.left`: platform-facts.md (§2.24): material-web outlined field 16
// leading, 12 trailing (the arrow's with-trailing-content-trailing-space), 16 top and bottom from
// the field's edge (cbd34a8, tokens/_md-comp-outlined-field.scss:222, :240-241, :235, :127), less
// the 1 outline laid over it (v0_192 _md-comp-outlined-select.scss:234)
const COMBO_BOX_PADDING_LEFT: f32 = 15.0;

// `segmented_control.segment_height`: 0.7.2 material-3.toml (uncited)
const SEGMENT_HEIGHT: f32 = 40.0;
// `segmented_control.separator_width`: 0.7.2 material-3.toml (uncited)
const SEGMENT_SEPARATOR_WIDTH: f32 = 1.0;
// `segmented_control.border.corner_radius`: platform-facts.md (§2.25): shape corner-full, a pill:
// half the 40 container-height (_md-comp-outlined-segmented-button.scss:34, :93)
const SEGMENT_CORNER_RADIUS: f32 = 20.0;
// `segmented_control.border.padding.left`, `segmented_control.border.padding.right`:
// platform-facts.md (§2.25): material-web labs outlined segmented button spacing-leading /
// spacing-trailing 12 (cbd34a8, labs/segmentedbutton/internal/_outlined-segmented-button.scss:22-23)
const SEGMENT_PADDING_HORIZONTAL: f32 = 12.0;
// `segmented_control.font.weight`: platform-facts.md (§2.25): label-text = label-large, weight 500
// (_md-comp-outlined-segmented-button.scss:63)
const SEGMENT_FONT_WEIGHT: u16 = 500;

// `card.border.corner_radius`: platform-facts.md (§2.26): container-shape corner-medium 12
// (_md-comp-filled-card.scss:38)
const CARD_CORNER_RADIUS: f32 = 12.0;

// `expander.header_height`: 0.7.2 material-3.toml (uncited)
const EXPANDER_HEADER_HEIGHT: f32 = 48.0;
// `expander.arrow_icon_size`: 0.7.2 material-3.toml (uncited)
const EXPANDER_ARROW_SIZE: f32 = 12.0;

/// The state precedence of a part with rules for pressed and hover: pressed
/// over hover.
const PRESSED_HOVER: &[&[WidgetState]] = &[&[WidgetState::PRESSED, WidgetState::HOVER]];

/// The contrast level the dynamic scheme of a seed is resolved at
/// ([`m3_role`]): 0, the scheme's standard level, at the one contrast level
/// the preset defines ([`CONTRAST_LEVELS`]).
const M3_CONTRAST_LEVEL: f32 = 0.0;

/// `md.sys.state`'s hover state layer opacity, 0.08
/// (`_md-comp-filled-tonal-button.scss:57-60`; `_md-comp-list.scss:85-87`).
const HOVER_STATE_LAYER_OPACITY: f32 = 0.08;

/// `md.sys.state`'s pressed state layer opacity, 0.12
/// (`_md-comp-filled-tonal-button.scss:85-88`; `_md-sys-state.scss:20`).
const PRESSED_STATE_LAYER_OPACITY: f32 = 0.12;

/// The disabled container's opacity of `on-surface`, 0.12
/// (`_md-comp-filled-tonal-button.scss:42-46`).
const DISABLED_CONTAINER_OPACITY: f32 = 0.12;

/// The disabled content's opacity of `on-surface`, 0.38
/// (`_md-comp-filled-button.scss:45-46`).
const DISABLED_CONTENT_OPACITY: f32 = 0.38;

/// The switch's disabled track opacity, 0.12 (`_md-comp-switch.scss:37`).
const DISABLED_TRACK_OPACITY: f32 = 0.12;

/// The font DPI a size converts at: 0.7.2's on every desktop but macOS
/// (`ResolutionContext::from_system`, `src/resolve/context.rs`; spec §4,
/// `Typography::font_dpi`). `material-3.toml` states every size in logical
/// pixels, which the DPI does not convert.
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

// ---------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------

/// The contrast levels the preset defines (spec §1, "Axis values without
/// data"): `Normal` alone, as the module documentation says.
pub(crate) const CONTRAST_LEVELS: &[Contrast] = &[Contrast::Normal];

/// The theme of `material-3` for `ctx.scheme`, from the preset's constants
/// with the dynamic scheme of the seed, an explicit accent or else the
/// active palette's `accent`, laid over them ([`lay_seed_over`]), then the
/// palette ([`lay_palette_over`]), and under a seed the state colours derived
/// again ([`derive_state_layers`]), as the module documentation says (spec
/// §6, resolution step 2). The preset has no reader.
pub(crate) fn resolve(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let mut scheme_colors = match ctx.scheme {
        Scheme::Light => LIGHT,
        Scheme::Dark => DARK,
    };
    let seed = inputs.accent.or(inputs.palette.map(|p| p.accent));
    if let Some(seed) = seed {
        lay_seed_over(&mut scheme_colors, seed, ctx.scheme);
    }
    if let Some(p) = inputs.palette {
        lay_palette_over(&mut scheme_colors, p, inputs.accent.is_some());
    }
    if seed.is_some() {
        derive_state_layers(&mut scheme_colors);
    }
    let c = &scheme_colors;
    let colors = roles(c, inputs.palette);
    let typography = typography(c, ctx.text_scale);
    let units = FontUnits {
        dpi: typography.font_dpi,
        text_scale: ctx.text_scale,
    };
    // No source the builder reads states a policy of Material's for reduced motion: `Reduced`
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
    let direction = ctx.direction;
    ResolvedTheme {
        name: Arc::from(Preset::Material3.display_name()),
        preset: Preset::Material3,
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
        // 0.7.2's `defaults.focus_ring_*`: a ring outside the part.
        focus_ring: FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(FOCUS_RING_WIDTH, colors.focus_ring),
            Stroke::None,
            FOCUS_RING_OFFSET,
            None,
            Some(if FOCUS_RING_KEYBOARD_ONLY {
                FocusTrigger::KeyboardOnly
            } else {
                FocusTrigger::Always
            }),
        ),
        caret: CaretSpec {
            width: None,
            blink_on_ms: Some(CARET_BLINK_ON_MS),
            blink_off_ms: Some(CARET_BLINK_OFF_MS),
            blink_timeout_ms: None,
            shape: None,
        },
        elevations: Arc::from([]),
        button: button(c, &colors, &typography),
        checkbox: checkbox(c, &colors, &typography),
        radio: radio(c, &colors, &typography),
        scrollbar: scrollbar(c, &colors),
        list: list(c, &colors, &typography, &layout),
        tab: tab(c, &colors, &typography),
        menu: menu(c, &colors, &typography),
        menu_bar: menu_bar(&colors),
        sidebar: sidebar(c, &colors, &typography, &layout),
        toolbar: toolbar(&colors, &typography, units),
        status_bar: status_bar(&colors, &typography, &layout),
        tooltip: tooltip(&colors, &typography, units),
        window: window(&colors, &typography, &layout),
        input: input(
            c,
            &colors,
            &typography,
            &layout,
            units,
            Edges::symmetric(INPUT_PADDING_VERTICAL, INPUT_PADDING_HORIZONTAL),
            Some(INPUT_MIN_HEIGHT),
        ),
        text_area: input(
            c,
            &colors,
            &typography,
            &layout,
            units,
            Edges::symmetric(TEXT_AREA_PADDING_VERTICAL, TEXT_AREA_PADDING_HORIZONTAL),
            None,
        ),
        link: link(c, &colors, &typography),
        slider: slider(c, &colors),
        progress_bar: progress_bar(c, &colors),
        splitter: splitter(&colors),
        separator: separator(c, &layout),
        spinner: spinner(&colors),
        popover: popover(c, &colors, &typography),
        switch: switch(c, &colors),
        dialog: dialog(c, &colors, &typography, units, motion),
        combo_box: combo_box(c, &colors, &typography, &layout, units, direction),
        segmented_control: segmented_control(c, &colors, &typography, &layout),
        card: card(c, &colors),
        expander: expander(c, &colors, &typography, &layout),
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
        colors,
        typography,
        layout,
    }
}

/// The colour roles of one scheme from its colours `c`. The window's surface
/// is Material's `surface`, lettered in `on-surface`, its muted text
/// `on-surface-variant`, its outline `outline`; the view's surface the same
/// `surface` (`defaults.surface_color`); the tooltip's the inverse surface.
/// The selection is the secondary container, lettered in its text (0.7.2:
/// `selection_inactive_background` and `selection_inactive_text_color` are
/// the selection's, and the text selection's colours the selection's, where
/// unstated, `src/resolve/inheritance.rs`, `resolve_defaults_internal`); the
/// caret is `primary` (`input.caret_color`). The status roles' origins are
/// Material's (spec §4, `RoleOrigin`): the error role its own, the other
/// three harmonized.
///
/// The active palette `palette`, whose colours `c` holds already, states
/// the roles its slots name (spec §7, "The roles a palette states"): the
/// view's, the header's, the sidebar's, the card's, the dialog's, the
/// popover's and the tooltip's surfaces with its one text colour, the
/// window's divider, the selection (the unfocused and the text selection
/// with it), the link, the caret, the info role's fill, and the four status
/// colours' origin (`Custom`).
fn roles(c: &SchemeColors, palette: Option<&Palette>) -> ColorRoles {
    let origin = |own| {
        if palette.is_some() {
            RoleOrigin::Custom
        } else {
            own
        }
    };
    let status = |fill, foreground, own| StatusColors {
        fill,
        foreground,
        standalone: None,
        origin: origin(own),
    };
    let unstated = SurfaceColors::default();
    let surface = |background: fn(&Palette) -> Color| {
        palette.map_or(unstated, |p| {
            SurfaceColors::new(Some(background(p)), Some(p.foreground), None, None, None)
        })
    };
    let (selection_background, selection_foreground) = palette
        .map_or((c.secondary_container, c.on_secondary_container), |p| {
            (p.selection_bg, p.selection_fg)
        });
    ColorRoles {
        window: SurfaceColors::new(
            Some(c.surface),
            Some(c.on_surface),
            Some(c.on_surface_variant),
            Some(c.outline),
            palette.map(|p| p.divider),
        ),
        view: SurfaceColors::new(
            Some(palette.map_or(c.surface, |p| p.view_bg)),
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
        tooltip: palette.map_or(
            SurfaceColors::new(
                Some(c.inverse_surface),
                Some(c.inverse_on_surface),
                None,
                None,
                None,
            ),
            |p| SurfaceColors::new(Some(p.tooltip_bg), Some(p.foreground), None, None, None),
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
            fill: c.primary,
            foreground: c.on_primary,
            standalone: None,
        },
        focus_ring: c.secondary,
        caret: palette.map_or(c.primary, |p| p.caret),
        link: palette.map_or(c.primary, |p| p.link),
        // 0.7.2: `link.visited_text_color` is `defaults.link_color` where unstated.
        link_visited: None,
        disabled_text: c.disabled_content,
        shadow: c.shadow,
        disabled_opacity: DISABLED_OPACITY,
        error: status(c.error, c.on_error, RoleOrigin::Native),
        warning: status(c.warning, c.on_warning, RoleOrigin::Harmonized),
        success: status(c.success, c.on_success, RoleOrigin::Harmonized),
        info: status(
            palette.map_or(c.primary, |p| p.info),
            c.on_primary,
            RoleOrigin::Harmonized,
        ),
    }
}

/// The dynamic scheme of the seed `seed` in `scheme` laid over `c`
/// ([`m3_role`], at [`M3_CONTRAST_LEVEL`]): each field of `c` that is an
/// `md.sys.color` role takes the scheme's role of its name.
fn lay_seed_over(c: &mut SchemeColors, seed: Color, scheme: Scheme) {
    let seed = [seed.r, seed.g, seed.b, seed.a];
    for (slot, role) in [
        (&mut c.surface, "surface"),
        (&mut c.on_surface, "on_surface"),
        (&mut c.on_surface_variant, "on_surface_variant"),
        (&mut c.outline, "outline"),
        (&mut c.outline_variant, "outline_variant"),
        (&mut c.surface_variant, "surface_variant"),
        (&mut c.primary, "primary"),
        (&mut c.on_primary, "on_primary"),
        (&mut c.secondary, "secondary"),
        (&mut c.on_secondary, "on_secondary"),
        (&mut c.secondary_container, "secondary_container"),
        (&mut c.on_secondary_container, "on_secondary_container"),
        (&mut c.surface_container, "surface_container"),
        (&mut c.surface_container_high, "surface_container_high"),
        (
            &mut c.surface_container_highest,
            "surface_container_highest",
        ),
        (&mut c.inverse_surface, "inverse_surface"),
        (&mut c.inverse_on_surface, "inverse_on_surface"),
        (&mut c.error, "error"),
        (&mut c.on_error, "on_error"),
    ] {
        if let Some([r, g, b, a]) = m3_role(role, seed, M3_CONTRAST_LEVEL, scheme) {
            *slot = Color::new(r, g, b, a);
        }
    }
}

/// Lays the palette `p` over `c`: each slot whose Material colour is the
/// colour of the slot's role replaces that colour, as the module
/// documentation lists; `accent` and `accent_fg` unless an explicit accent
/// (`explicit_accent`) replaces them. The slots that state only their roles
/// are read by [`roles`].
fn lay_palette_over(c: &mut SchemeColors, p: &Palette, explicit_accent: bool) {
    c.surface = p.window_bg;
    c.on_surface = p.foreground;
    c.on_surface_variant = p.foreground_muted;
    c.outline = p.border;
    c.error = p.error;
    c.warning = p.warning;
    c.success = p.success;
    if !explicit_accent {
        c.primary = p.accent;
        c.on_primary = p.accent_fg;
    }
}

/// The state colours of `c` that are a role at an `md.sys.state` or a
/// disabled opacity, derived again from the roles of `c`: `on-surface` at
/// the disabled container, the disabled content and the hover opacities;
/// `surface-container-highest` at the disabled track opacity; the tonal
/// button's hovered and pressed fills, `on-secondary-container` at the hover
/// and the pressed opacities composited over `secondary-container`.
fn derive_state_layers(c: &mut SchemeColors) {
    c.disabled_container = c.on_surface.mul_alpha(DISABLED_CONTAINER_OPACITY);
    c.disabled_content = c.on_surface.mul_alpha(DISABLED_CONTENT_OPACITY);
    c.hover_layer = c.on_surface.mul_alpha(HOVER_STATE_LAYER_OPACITY);
    c.disabled_track = c
        .surface_container_highest
        .mul_alpha(DISABLED_TRACK_OPACITY);
    c.button_hover = c
        .on_secondary_container
        .mul_alpha(HOVER_STATE_LAYER_OPACITY)
        .composite_over(c.secondary_container);
    c.button_pressed = c
        .on_secondary_container
        .mul_alpha(PRESSED_STATE_LAYER_OPACITY)
        .composite_over(c.secondary_container);
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

/// A font size in logical pixels: converted at the font DPI, then through
/// Android's converter of the text scale (spec §6, resolution step 2), not
/// rounded (0.7.2 rounds only where `font_whole_pixels` is stated,
/// `src/resolve/validate.rs`).
fn font_px(size: FontSize, units: FontUnits) -> f32 {
    android_non_linear(size.to_logical_px(units.dpi), units.text_scale)
}

/// A line height `line_height` of a font of `font_size_px` logical pixels,
/// already scaled: through Android's converter of the text scale up to
/// Compose's threshold, and above it the unscaled line height times the
/// font's change, its scaled size over the size the converter takes back
/// from it (`compose/ui/ui-text` `SpannableExtensions.android.kt:224-250`,
/// [`PROPORTIONAL_LINE_HEIGHT_ABOVE`]).
fn line_height_px(line_height: FontSize, font_size_px: f32, units: FontUnits) -> f32 {
    let px = line_height.to_logical_px(units.dpi);
    if f64::from(units.text_scale) > PROPORTIONAL_LINE_HEIGHT_ABOVE {
        px / android_non_linear_inverse(font_size_px, units.text_scale) * font_size_px
    } else {
        android_non_linear(px, units.text_scale)
    }
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
/// stated, nothing else (0.7.2's `TextScaleEntry`); the TOML states each
/// entry's line height in pixels.
fn text_scale_entry(
    size: FontSize,
    weight: u16,
    line_height_px_unscaled: f32,
    units: FontUnits,
) -> TextStyle {
    let size_px = font_px(size, units);
    TextStyle {
        size_px,
        defined_size: Some(size),
        weight,
        line_height: line_height_px(FontSize::Px(line_height_px_unscaled), size_px, units),
        stated: TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::LINE_HEIGHT,
        ..TextStyle::default()
    }
}

/// The text styles. The fonts' colour is the window's foreground (0.7.2:
/// `defaults.font.color` is `defaults.text_color`, and
/// `defaults.mono_font`'s is `defaults.font`'s, where unstated:
/// `resolve_defaults_internal`); the section heading is `headline`, the
/// dialog title `title1` and the display style `display` (spec §4). The
/// body's line height is the multiplier times its size, in its unit (spec
/// §4; 0.7.2's `resolve_text_scale_entry`, then its conversion in
/// `validate`). Sizes convert at [`FONT_DPI`] and scale by `text_scale`
/// (spec §6, resolution step 2); each keeps its unscaled size as its
/// `defined_size`.
fn typography(c: &SchemeColors, text_scale: f32) -> Typography {
    let units = FontUnits {
        dpi: FONT_DPI,
        text_scale,
    };
    let text = c.on_surface;
    let body_size = FontSize::Px(FONT_SIZE_PX);
    let mut body = font(FONT_FAMILY, body_size, FONT_WEIGHT, text, units);
    body.line_height = line_height_px(
        FontSize::Px(LINE_HEIGHT * FONT_SIZE_PX),
        body.size_px,
        units,
    );
    body.stated |= TextStated::LINE_HEIGHT;
    Typography {
        display: Some(text_scale_entry(
            FontSize::Px(DISPLAY_SIZE_PX),
            DISPLAY_WEIGHT,
            DISPLAY_LINE_HEIGHT_PX,
            units,
        )),
        title1: Some(text_scale_entry(
            FontSize::Px(TITLE1_SIZE_PX),
            TITLE1_WEIGHT,
            TITLE1_LINE_HEIGHT_PX,
            units,
        )),
        title2: None,
        title3: None,
        title4: None,
        headline: Some(text_scale_entry(
            FontSize::Px(HEADLINE_SIZE_PX),
            HEADLINE_WEIGHT,
            HEADLINE_LINE_HEIGHT_PX,
            units,
        )),
        body,
        callout: None,
        subheadline: None,
        caption: Some(text_scale_entry(
            FontSize::Px(CAPTION_SIZE_PX),
            CAPTION_WEIGHT,
            CAPTION_LINE_HEIGHT_PX,
            units,
        )),
        monospace: font(
            MONO_FAMILY,
            FontSize::Px(MONO_SIZE_PX),
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

/// A part's outline in `color`, inside its edge, `width` wide, the fill under
/// the whole of it.
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

/// A part's padding on its left and right sides alone, `horizontal` each:
/// the preset states no vertical padding for it.
fn padding_sides(horizontal: f32) -> PartProp {
    PartProp::PaddingSides(
        Edges::symmetric(0.0, horizontal),
        EdgeFlags::LEFT | EdgeFlags::RIGHT,
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

/// `label` at the size `size`, which it keeps as its `defined_size`.
fn sized(mut label: TextStyle, size: FontSize, units: FontUnits) -> TextStyle {
    label.size_px = font_px(size, units);
    label.defined_size = Some(size);
    label
}

/// `label` at the weight `weight`.
fn weighted(mut label: TextStyle, weight: u16) -> TextStyle {
    label.weight = weight;
    label
}

/// The push button, Material's filled tonal button: the secondary container
/// with no outline, in the default border colour (0.7.2's
/// `resolve_border_inheritance`), its pill corners, no shadow, padded, of
/// least sizes; its hovered and pressed fills (the state layers composited
/// over the container), checked the secondary colour; the primary variant,
/// the filled button, in the accent (0.7.2: `primary_background` is
/// `defaults.accent_color` where unstated); disabled, the disabled container.
/// Its label is the secondary container's text at its own weight, checked the
/// secondary colour's text, primary the accent's (0.7.2: `primary_text_color`
/// is `defaults.accent_text_color` where unstated), disabled the disabled
/// text.
fn button(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> ButtonWidget {
    let primary = || {
        RuleSelector::on(WidgetState::NONE).matcher(ButtonMatcher {
            variant: Some(ButtonVariant::Primary),
            shape: None,
        })
    };
    let body = RuleBuilder::with_exclusive(PartStyle::default(), PRESSED_HOVER)
        .base([fill(c.secondary_container)])
        .base(border(roles, BUTTON_BORDER_WIDTH))
        .base([
            corners(BUTTON_CORNER_RADIUS),
            // `button.border.shadow_enabled`: platform-facts.md (§2.3): container-elevation level0
            // (_md-comp-filled-tonal-button.scss:38)
            PartProp::Shadows(None),
            PartProp::Padding(Edges::symmetric(
                BUTTON_PADDING_VERTICAL,
                BUTTON_PADDING_HORIZONTAL,
            )),
            PartProp::MinSize([Some(BUTTON_MIN_WIDTH), Some(BUTTON_MIN_HEIGHT)]),
        ])
        .rule(RuleSelector::on(WidgetState::CHECKED), [fill(c.secondary)])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.button_hover)])
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.button_pressed)],
        )
        .rule(primary(), [fill(roles.accent.fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_container)],
        )
        .build();
    // `button.hover_text_color`, `active_text_color`: 0.7.2 material-3.toml (uncited), the rest
    // colour, each stated for its state (spec §2, `from_state`), before the rules of the states
    // that replace it.
    let label = RuleBuilder::new(weighted(
        label(&typography.body, Some(c.on_secondary_container)),
        BUTTON_FONT_WEIGHT,
    ))
    .rules(
        [
            RuleSelector::on(WidgetState::HOVER),
            RuleSelector::on(WidgetState::PRESSED),
        ],
        [TextProp::Color(c.on_secondary_container)],
    )
    .rule(
        RuleSelector::on(WidgetState::CHECKED),
        [TextProp::Color(c.on_secondary)],
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
/// `checkbox.*` serves both, `size` across: transparent in its unchecked
/// outline at its own width, no shadow; under the pointer the hover layer;
/// checked, the accent's fill (0.7.2: `checkbox.checked_background` is
/// `defaults.accent_color` where unstated) in its own outline colour; disabled,
/// the disabled fill, checked or not.
fn indicator(c: &SchemeColors, roles: &ColorRoles, size: f32) -> RuleBuilder<PartStyle, PartProp> {
    RuleBuilder::new(PartStyle::default())
        .base([
            fill(TRANSPARENT),
            stroke(CHECKBOX_BORDER_WIDTH, c.on_surface_variant),
            // `checkbox.border.shadow_enabled`: platform-facts.md (§2.5): no elevation token
            PartProp::Shadows(None),
            PartProp::MinSize([Some(size); 2]),
            PartProp::MaxSize([Some(size); 2]),
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.hover_layer)])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [
                fill(roles.accent.fill),
                stroke(CHECKBOX_BORDER_WIDTH, c.primary),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_content)],
        )
}

/// The label of a check box or a radio button: the UI font in its colour;
/// disabled, the disabled text's colour.
fn check_label(roles: &ColorRoles, typography: &Typography) -> RuleSet<TextStyle, TextProp> {
    RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(roles.disabled_text)],
        )
        .build()
}

/// The check box: [`indicator`] with its own corners, a check mark in the
/// accent's text colour (0.7.2: `checkbox.indicator_color` is
/// `defaults.accent_text_color` where unstated) at its own stroke width,
/// [`check_label`].
fn checkbox(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> CheckboxWidget {
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base([
            IconProp::Color(roles.accent.foreground),
            IconProp::StrokeWidth(Some(CHECK_MARK_STROKE_WIDTH)),
        ])
        .build();
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator(c, roles, CHECKBOX_INDICATOR_SIZE)
            .base([corners(CHECKBOX_CORNER_RADIUS)])
            .build(),
        checkmark,
        check_label(roles, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The radio button: [`indicator`] at the radio's own size (0.7.2 states no
/// corners for it), a dot in the accent's text colour
/// (`checkbox.indicator_color`), [`check_label`].
fn radio(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> RadioWidget {
    let dot = RuleBuilder::new(PartStyle::default())
        .base([
            fill(roles.accent.foreground),
            PartProp::MinSize([Some(RADIO_DOT_SIZE); 2]),
            PartProp::MaxSize([Some(RADIO_DOT_SIZE); 2]),
        ])
        .build();
    RadioWidget::new(
        WidgetOrigin::Native,
        indicator(c, roles, RADIO_INDICATOR_SIZE).build(),
        dot,
        check_label(roles, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The list or table view: the window's background (0.7.2's
/// `list.background_color` where unstated, `resolve_safety_nets`) in the
/// default border, its colour, radius and width (`resolve_border_inheritance`;
/// the default shadow, which is on); rows padded on their sides (spec §5,
/// `L4`: Material's list item pads its content), with the alternate row's
/// fill; the highlight's hover layer and the selection's fill; the window's
/// text, which a hovered row keeps (`list.hover_text_color`), the selection's
/// text on a selected row, the disabled text's colour on a disabled row;
/// headers in the window's background, of a least height, their text in its
/// own colour and weight; the grid in its own colour. 0.7.2 states no row
/// height and no header separator.
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
        .base([padding_sides(LIST_ROW_PADDING_HORIZONTAL)])
        .rule(alternate, [fill(c.surface)])
        .build();
    let highlight = RuleBuilder::new(PartStyle::default())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.hover_layer)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    // `list.hover_text_color`: list-item-hover-label-text-color = on-surface
    // (_md-comp-list.scss:81), the rest colour, stated for `HOVER` (spec §2, `from_state`), before
    // the rules of the states that replace it.
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
        .base(roles.view.background.map(fill))
        .base([PartProp::MinSize([None, Some(LIST_HEADER_HEIGHT)])])
        .build();
    let header_label = RuleBuilder::new(weighted(
        label(&typography.body, Some(c.on_surface_variant)),
        LIST_HEADER_WEIGHT,
    ))
    .build();
    ListWidget::new(
        WidgetOrigin::Native,
        container,
        row,
        highlight,
        item_label,
        header,
        header_label,
        RuleBuilder::new(PartStyle::default()).build(),
        c.outline_variant,
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

/// The scroll bar: a groove in the window's background (0.7.2:
/// `scrollbar.track_color` is `defaults.background_color` where unstated,
/// `resolve_safety_nets`) and a handle in the muted foreground (0.7.2:
/// `scrollbar.thumb_color` is `defaults.muted_color` where unstated), each of
/// an exact thickness across the bar; the hovered and the pressed handle's
/// own colours.
fn scrollbar(c: &SchemeColors, roles: &ColorRoles) -> ScrollbarWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base(roles.window.background.map(fill)),
        SCROLLBAR_GROOVE_WIDTH,
    )
    .build();
    let thumb = across(
        RuleBuilder::new(PartStyle::default()).base(roles.window.foreground_muted.map(fill)),
        SCROLLBAR_THUMB_WIDTH,
    )
    .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.outline)])
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

/// The tooltip: the tooltip's colours, no outline, in the default border
/// colour (`resolve_border_inheritance`), its own corners, no shadow, padded
/// and of a greatest width; its text in the UI font at its own size.
fn tooltip(roles: &ColorRoles, typography: &Typography, units: FontUnits) -> TooltipWidget {
    let mut panel = PartStyle::default();
    for prop in roles
        .tooltip
        .background
        .map(fill)
        .into_iter()
        .chain(border(roles, TOOLTIP_BORDER_WIDTH))
        .chain([
            corners(TOOLTIP_CORNER_RADIUS),
            // `tooltip.border.shadow_enabled`: platform-facts.md (§2.7): no elevation token;
            // Compose PlainTooltip shadowElevation = 0.dp (Tooltip.kt:350-351)
            PartProp::Shadows(None),
            PartProp::Padding(Edges::symmetric(
                TOOLTIP_PADDING_VERTICAL,
                TOOLTIP_PADDING_HORIZONTAL,
            )),
            PartProp::MaxSize([Some(TOOLTIP_MAX_WIDTH), None]),
        ])
    {
        panel.apply(&prop);
    }
    TooltipWidget::new(
        WidgetOrigin::Native,
        panel,
        sized(
            label(&typography.body, roles.tooltip.foreground),
            FontSize::Px(TOOLTIP_FONT_SIZE_PX),
            units,
        ),
    )
}

/// The window: its frame in the window's background (0.7.2:
/// `window.background_color` is `defaults.background_color` where unstated)
/// with the default border, its colour and width, the large corners
/// (`resolve_border_inheritance`; the default shadow, which is on); the title
/// bar, active or not, in the view's background (0.7.2:
/// `window.title_bar_background` is `defaults.surface_color` where unstated);
/// the title in the UI font, in the window's text, active or not.
fn window(roles: &ColorRoles, typography: &Typography, layout: &LayoutMetrics) -> WindowChrome {
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
    let mut title_bar = PartStyle::default();
    if let Some(prop) = roles.view.background.map(fill) {
        title_bar.apply(&prop);
    }
    WindowChrome::new(
        WidgetOrigin::Native,
        None,
        frame.clone(),
        frame,
        title_bar.clone(),
        title_bar,
        label(&typography.body, None),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
    )
}

/// The text field, and with `padding` and no least height the text area
/// (0.7.2's `text_area.border` takes the field's colour, radius, width and
/// shadow where unstated, `resolve_widget_to_widget`): the surface in the
/// default border, its colour and width (`resolve_border_inheritance`), its
/// own corners, no shadow; hovered, the outline in the window's text;
/// focused, in the accent; the window's text at its own size, its own
/// disabled colour; the placeholder in the muted text (0.7.2:
/// `input.placeholder_color` is `defaults.muted_color` where unstated).
fn input(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    units: FontUnits,
    padding: Edges,
    min_height: Option<f32>,
) -> InputWidget {
    let field = RuleBuilder::new(PartStyle::default())
        .base([fill(c.surface)])
        .base(border(roles, layout.border_width))
        .base([
            corners(INPUT_CORNER_RADIUS),
            // `input.border.shadow_enabled`: platform-facts.md (§2.4): the text fields have no
            // elevation token
            PartProp::Shadows(None),
            PartProp::Padding(padding),
        ])
        .base(min_height.map(|height| PartProp::MinSize([None, Some(height)])))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [stroke(layout.border_width, c.on_surface)],
        )
        .rule(
            RuleSelector::on(WidgetState::FOCUSED),
            [stroke(layout.border_width, c.primary)],
        )
        .build();
    let text = RuleBuilder::new(sized(
        label(&typography.body, Some(c.on_surface)),
        FontSize::Px(INPUT_FONT_SIZE_PX),
        units,
    ))
    .rule(
        RuleSelector::on(WidgetState::DISABLED),
        [TextProp::Color(roles.disabled_text)],
    )
    .build();
    let placeholder = RuleBuilder::new(TextStyle::default())
        .base(roles.window.foreground_muted.map(TextProp::Color))
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

/// The link: a transparent background, under the pointer its hover layer;
/// the link colour (0.7.2: `link.font.color` and `visited_text_color` are
/// `defaults.link_color` where unstated), which a hovered link keeps
/// (`link.hover_text_color`), not underlined; its pressed and disabled
/// colours.
fn link(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> LinkWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(TRANSPARENT)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.link_hover_layer)],
        )
        .build();
    let label = RuleBuilder::new(label(&typography.body, Some(roles.link)))
        .base([TextProp::Decoration(TextDecoration {
            kind: LINK_DECORATION,
            color: None,
            thickness: None,
        })])
        // `link.hover_text_color`: 0.7.2 material-3.toml (uncited), the rest colour, stated for
        // `HOVER` (spec §2, `from_state`), before the rules of the states that replace it.
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(roles.link)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(c.link_pressed)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.link_disabled)],
        )
        .build();
    LinkWidget::new(WidgetOrigin::Native, container, label)
}

/// The slider: a groove in `surface-container-highest` of an exact
/// thickness, its filled part in the accent (0.7.2: `slider.fill_color` is
/// `defaults.accent_color` where unstated), each with its disabled colour;
/// the handle in its own colour, which it keeps under the pointer
/// (`slider.thumb_hover_color`), of an exact diameter, its disabled colour.
fn slider(c: &SchemeColors, roles: &ColorRoles) -> SliderWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base([fill(c.surface_container_highest)]),
        SLIDER_TRACK_HEIGHT,
    )
    .rule(
        RuleSelector::on(WidgetState::DISABLED),
        [fill(c.disabled_container)],
    )
    .build();
    let active_track = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.accent.fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_content)],
        )
        .build();
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.primary),
            PartProp::MinSize([Some(SLIDER_THUMB_DIAMETER); 2]),
            PartProp::MaxSize([Some(SLIDER_THUMB_DIAMETER); 2]),
        ])
        // `slider.thumb_hover_color`: 0.7.2 material-3.toml (uncited), the rest colour, stated for
        // `HOVER` (spec §2, `from_state`), before the disabled rule.
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.primary)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_content)],
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

/// The progress bar: a groove in `surface-container-highest` with no
/// outline, in the default border colour, its own corners, no shadow, of an
/// exact height and a least length; the progress in the accent (0.7.2:
/// `progress_bar.fill_color` is `defaults.accent_color` where unstated).
fn progress_bar(c: &SchemeColors, roles: &ColorRoles) -> ProgressBarWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base([fill(c.surface_container_highest)])
        .base(border(roles, PROGRESS_BAR_BORDER_WIDTH))
        .base([
            corners(PROGRESS_BAR_CORNER_RADIUS),
            // `progress_bar.border.shadow_enabled`: platform-facts.md (§2.10): no elevation token,
            // no outline
            PartProp::Shadows(None),
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

/// The separator: its own colour at the default border width (0.7.2:
/// `separator.line_width` is `defaults.border.line_width` where unstated).
fn separator(c: &SchemeColors, layout: &LayoutMetrics) -> SeparatorWidget {
    SeparatorWidget::new(
        WidgetOrigin::Native,
        c.outline_variant,
        layout.border_width,
        None,
        None,
    )
}

/// The splitter: its handle in the default border colour (0.7.2:
/// `splitter.divider_color` is `defaults.border.color` where unstated), under
/// the pointer in the accent.
fn splitter(roles: &ColorRoles) -> SplitterWidget {
    let handle = RuleBuilder::new(PartStyle::default())
        .base(roles.window.border.map(fill))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(roles.accent.fill)],
        )
        .build();
    SplitterWidget::new(WidgetOrigin::Native, handle, SPLITTER_DIVIDER_WIDTH)
}

/// The spinner, in the accent (0.7.2: `spinner.fill_color` is
/// `defaults.accent_color` where unstated, `resolve_safety_nets`).
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

/// The menu: its panel in the surface container, with its own corners (spec
/// §5: `material-3`'s `menu.border.corner_radius` is the container's; the
/// default shadow, which is on); items padded on their sides, of an exact
/// height, their hover layer; the window's text at its own weight, which a
/// hovered item keeps (`menu.hover_text_color`), the disabled text's colour
/// when disabled; icons at the toolbar's size (0.7.2: `menu.icon_size` is
/// `defaults.icon_sizes.toolbar` where unstated); separators in their own
/// colour. 0.7.2 states no pressed item and no item outline or corners.
fn menu(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> MenuWidget {
    let panel = RuleBuilder::new(PartStyle::default())
        .base([fill(c.surface_container), corners(MENU_CORNER_RADIUS)])
        .build();
    let item = RuleBuilder::new(PartStyle::default())
        .base([
            padding_sides(MENU_ITEM_PADDING_HORIZONTAL),
            PartProp::MaxSize([None, Some(MENU_ROW_HEIGHT)]),
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.hover_layer)])
        .build();
    // `menu.hover_text_color`: platform-facts.md (§2.6), list-item-hover-label-text-color =
    // on-surface, the rest colour, stated for `HOVER` (spec §2, `from_state`), before the disabled
    // rule.
    let item_label = RuleBuilder::new(weighted(label(&typography.body, None), MENU_FONT_WEIGHT))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(typography.body.color)],
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
        .base([fill(c.surface_variant)])
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

/// The tab bar: the bar and the tabs in the window's background, the
/// selected one too (0.7.2: `tab.bar_background`, `tab.background_color` and
/// `tab.active_background` are `defaults.background_color` where unstated),
/// a tab of least sizes, padded on its sides, its hover layer; the muted text
/// at its own weight, the window's text on a hovered tab, the accent on the
/// selected one; the selected tab's indicator in the accent, of an exact
/// thickness, along its bottom.
fn tab(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> TabWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .build();
    let tab = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base([
            PartProp::MinSize([Some(TAB_MIN_WIDTH), Some(TAB_MIN_HEIGHT)]),
            padding_sides(TAB_PADDING_HORIZONTAL),
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.hover_layer)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            roles.window.background.map(fill),
        )
        .build();
    let label = RuleBuilder::new(weighted(
        label(&typography.body, Some(c.on_surface_variant)),
        TAB_FONT_WEIGHT,
    ))
    .rule(
        RuleSelector::on(WidgetState::HOVER),
        [TextProp::Color(c.on_surface)],
    )
    .rule(
        RuleSelector::on(WidgetState::SELECTED),
        [TextProp::Color(c.primary)],
    )
    .build();
    let indicator = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [
                fill(c.primary),
                PartProp::MinSize([None, Some(TAB_INDICATOR_WIDTH)]),
                PartProp::MaxSize([None, Some(TAB_INDICATOR_WIDTH)]),
            ],
        )
        .build();
    TabWidget::new(
        WidgetOrigin::Native,
        bar,
        tab,
        label,
        indicator,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        None,
        Some(TAB_INDICATOR_SIDE),
        None,
    )
}

/// The toolbar: the window's background (0.7.2: `toolbar.background_color`
/// is `defaults.background_color` where unstated) with no outline, in the
/// default border colour, its own corners, no shadow, of a least height; its
/// buttons' text in the UI font at its own size; icons at the toolbar's size
/// (0.7.2: `toolbar.icon_size` is `defaults.icon_sizes.toolbar` where
/// unstated). 0.7.2 states no padding and no button state.
fn toolbar(roles: &ColorRoles, typography: &Typography, units: FontUnits) -> ToolbarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, TOOLBAR_BORDER_WIDTH))
        .base([
            corners(TOOLBAR_CORNER_RADIUS),
            // `toolbar.border.shadow_enabled`: 0.7.2 material-3.toml (uncited)
            PartProp::Shadows(None),
            PartProp::MinSize([None, Some(TOOLBAR_BAR_HEIGHT)]),
        ])
        .build();
    let button_icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(ICON_TOOLBAR)])
        .build();
    ToolbarWidget::new(
        WidgetOrigin::Native,
        bar,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(sized(
            label(&typography.body, None),
            FontSize::Px(TOOLBAR_FONT_SIZE_PX),
            units,
        ))
        .build(),
        button_icon,
        RuleBuilder::new(PartStyle::default()).build(),
        None,
    )
}

/// The status bar: the window's background (0.7.2:
/// `status_bar.background_color` is `defaults.background_color` where
/// unstated) in the default border colour and width (0.7.2's partial border
/// inheritance: colour and width only). 0.7.2 states no padding.
fn status_bar(
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> StatusBarWidget {
    let mut bar = PartStyle::default();
    for prop in roles
        .window
        .background
        .map(fill)
        .into_iter()
        .chain(border(roles, layout.border_width))
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

/// The sidebar, Material's navigation drawer: the window's background (0.7.2:
/// `sidebar.background_color` is `defaults.background_color` where unstated)
/// in the default border colour and width (0.7.2's partial border
/// inheritance: colour and width only), padded on its sides alone (the
/// vertical is the app's); an item's hover layer and the selection's fill;
/// the muted text at its own weight, the selection's text on a selected item.
fn sidebar(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SidebarWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, layout.border_width))
        .base([padding_sides(SIDEBAR_PADDING_HORIZONTAL)])
        .build();
    let item = RuleBuilder::new(PartStyle::default())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.hover_layer)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    let item_label = RuleBuilder::new(weighted(
        label(&typography.body, Some(c.on_surface_variant)),
        SIDEBAR_FONT_WEIGHT,
    ))
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

/// The popover: the surface container, or the popover's background where a
/// palette states it, with no outline, in the default border colour, its own
/// corners (`resolve_border_inheritance`; the default shadow, which is on),
/// padded above and below; its text in the window's.
fn popover(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> PopoverWidget {
    let mut panel = PartStyle::default();
    for prop in [fill(
        roles.popover.background.unwrap_or(c.surface_container),
    )]
    .into_iter()
    .chain(border(roles, POPOVER_BORDER_WIDTH))
    .chain([
        corners(POPOVER_CORNER_RADIUS),
        PartProp::Padding(Edges::symmetric(
            POPOVER_PADDING_VERTICAL,
            POPOVER_PADDING_HORIZONTAL,
        )),
    ]) {
        panel.apply(&prop);
    }
    PopoverWidget::new(
        WidgetOrigin::Native,
        panel,
        PartStyle::default(),
        label(&typography.body, Some(c.on_surface)),
    )
}

/// The switch: a track of exact sizes and corners, off in its fill and
/// outline, on in the accent (0.7.2: `switch.checked_background` is
/// `defaults.accent_color` where unstated) with no outline (0.7.2's
/// `checked_track_border_color` unset: "`None` draws no outline"), the
/// hovered tracks as the resting ones (`switch.hover_unchecked_background`,
/// `switch.hover_checked_background`), and disabled, off in its fill and
/// outline and on in its fill with no outline; a thumb with no outline (0.7.2
/// states none for it), off in the outline colour and smaller, on in the
/// accent's text and larger, disabled in its own colour.
fn switch(c: &SchemeColors, roles: &ColorRoles) -> SwitchWidget {
    let size = [Some(SWITCH_TRACK_WIDTH), Some(SWITCH_TRACK_HEIGHT)];
    let track = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.surface_container_highest),
            stroke(SWITCH_TRACK_BORDER_WIDTH, c.outline),
            corners(SWITCH_TRACK_RADIUS),
            PartProp::MinSize(size),
            PartProp::MaxSize(size),
        ])
        // `switch.hover_unchecked_background`: unselected-hover-track-color =
        // surface-container-highest (_md-comp-switch.scss:116), the rest fill, stated for `HOVER`
        // (spec §2, `from_state`), before the rules of the states that replace it.
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.surface_container_highest)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill), PartProp::Stroke(Stroke::None)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [
                fill(c.disabled_track),
                stroke(SWITCH_TRACK_BORDER_WIDTH, c.disabled_container),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED | WidgetState::CHECKED),
            [fill(c.disabled_container), PartProp::Stroke(Stroke::None)],
        )
        .build();
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.outline),
            PartProp::Stroke(Stroke::None),
            PartProp::MinSize([Some(SWITCH_UNCHECKED_THUMB_DIAMETER); 2]),
            PartProp::MaxSize([Some(SWITCH_UNCHECKED_THUMB_DIAMETER); 2]),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [
                fill(roles.accent.foreground),
                PartProp::MinSize([Some(SWITCH_THUMB_DIAMETER); 2]),
                PartProp::MaxSize([Some(SWITCH_THUMB_DIAMETER); 2]),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_content)],
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

/// The dialog: its panel in the high surface container, or in the dialog's
/// background where a palette states it, with no outline, in
/// the default border colour, its own corners (`resolve_border_inheritance`;
/// the default shadow, which is on), its padding and size limits; its title
/// in the UI font at its own size and weight, its body in the UI font in the
/// muted text (0.7.2: `dialog.title_font` and `body_font` are
/// `defaults.font` where unstated).
fn dialog(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    units: FontUnits,
    motion: Motion,
) -> DialogWidget {
    let mut panel = PartStyle::default();
    for prop in [fill(
        roles.dialog.background.unwrap_or(c.surface_container_high),
    )]
    .into_iter()
    .chain(border(roles, DIALOG_BORDER_WIDTH))
    .chain([
        corners(DIALOG_CORNER_RADIUS),
        PartProp::Padding(Edges::symmetric(
            DIALOG_PADDING_VERTICAL,
            DIALOG_PADDING_HORIZONTAL,
        )),
        PartProp::MinSize([Some(DIALOG_MIN_WIDTH), Some(DIALOG_MIN_HEIGHT)]),
        PartProp::MaxSize([Some(DIALOG_MAX_WIDTH), Some(DIALOG_MAX_HEIGHT)]),
    ]) {
        panel.apply(&prop);
    }
    let title = weighted(
        sized(
            label(&typography.body, None),
            FontSize::Px(DIALOG_TITLE_SIZE_PX),
            units,
        ),
        DIALOG_TITLE_WEIGHT,
    );
    let mut icon = IconStyle::default();
    icon.apply(&IconProp::Size(DIALOG_ICON_SIZE));
    DialogWidget::new(
        WidgetOrigin::Native,
        panel,
        title,
        label(&typography.body, Some(c.on_surface_variant)),
        icon,
        DIALOG_BUTTON_GAP,
        Some(DIALOG_BUTTON_ORDER),
        dialog_entrance(Some(DIALOG_ENTRANCE), motion),
    )
}

/// The combo box, Material's outlined select: the window's background (0.7.2:
/// `combo_box.background_color` is `defaults.background_color` where
/// unstated) in the default border, its colour and width
/// (`resolve_border_inheritance`), its own corners, no shadow, padded, of
/// least sizes; hovered, the outline in the window's text; the window's text
/// at its own size, the disabled text's colour when disabled; its arrow's
/// size.
fn combo_box(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    units: FontUnits,
    direction: Direction,
) -> ComboBoxWidget {
    let field = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, layout.border_width))
        .base([
            corners(COMBO_BOX_CORNER_RADIUS),
            // `combo_box.border.shadow_enabled`: platform-facts.md (§2.24): the field has no
            // elevation
            PartProp::Shadows(None),
            PartProp::Padding(physical_edges(
                Edges::new(
                    COMBO_BOX_PADDING_VERTICAL,
                    COMBO_BOX_PADDING_RIGHT,
                    COMBO_BOX_PADDING_VERTICAL,
                    COMBO_BOX_PADDING_LEFT,
                ),
                direction,
            )),
            PartProp::MinSize([Some(COMBO_BOX_MIN_WIDTH), Some(COMBO_BOX_MIN_HEIGHT)]),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [stroke(layout.border_width, c.on_surface)],
        )
        .build();
    let label = RuleBuilder::new(sized(
        label(&typography.body, None),
        FontSize::Px(COMBO_BOX_FONT_SIZE_PX),
        units,
    ))
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

/// The segmented control, Material's outlined segmented button: a track in
/// the window's background (0.7.2: `segmented_control.background_color` is
/// `defaults.background_color` where unstated) in the default border, its
/// colour and width (`resolve_border_inheritance`), its pill corners, no
/// shadow; segments of a least height, padded on their sides, their hover
/// layer and the selected one's fill; the UI font at its own weight, the
/// selected segment's text in its own colour.
fn segmented_control(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SegmentedControlWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, layout.border_width))
        .base([
            corners(SEGMENT_CORNER_RADIUS),
            // `segmented_control.border.shadow_enabled`: platform-facts.md (§2.25): no elevation
            // token
            PartProp::Shadows(None),
        ])
        .build();
    let segment = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::MinSize([None, Some(SEGMENT_HEIGHT)]),
            padding_sides(SEGMENT_PADDING_HORIZONTAL),
        ])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.hover_layer)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(c.secondary_container)],
        )
        .build();
    let label = RuleBuilder::new(weighted(label(&typography.body, None), SEGMENT_FONT_WEIGHT))
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(c.on_secondary_container)],
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

/// The card, Material's filled card: `surface-container-highest`, or the
/// card's background where a palette states it, with its own corners. 0.7.2
/// states no outline, shadow or padding for it.
fn card(c: &SchemeColors, roles: &ColorRoles) -> CardWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([
            fill(roles.card.background.unwrap_or(c.surface_container_highest)),
            corners(CARD_CORNER_RADIUS),
        ])
        .build();
    CardWidget::new(WidgetOrigin::Native, container)
}

/// The expander: the default border, its colour, radius and width
/// (`resolve_border_inheritance`; the default shadow, which is on); a header
/// of a least height, its hover layer; the arrow's size and colour. 0.7.2
/// states no padding, arrow side or gap, indent or frame for it.
fn expander(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ExpanderWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(border(roles, layout.border_width))
        .base([corners(layout.corner_radius)])
        .build();
    let header = RuleBuilder::new(PartStyle::default())
        .base([PartProp::MinSize([None, Some(EXPANDER_HEADER_HEIGHT)])])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.expander_hover)],
        )
        .build();
    let arrow = RuleBuilder::new(IconStyle::default())
        .base([
            IconProp::Size(EXPANDER_ARROW_SIZE),
            IconProp::Color(c.on_surface_variant),
        ])
        .build();
    ExpanderWidget::new(
        WidgetOrigin::Native,
        container,
        header,
        RuleBuilder::new(label(&typography.body, None)).build(),
        arrow,
        None,
        None,
        None,
        None,
    )
}

#[cfg(all(test, feature = "serde"))]
mod snapshots;
#[cfg(test)]
mod tests;
