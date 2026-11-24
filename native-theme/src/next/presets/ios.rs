//! The `ios-13` preset's builder (spec §6): iOS's system look, light and
//! dark, as UIKit's colours and WebKit's drawing of iOS controls give it.
//!
//! The builder runs in three steps: the preset's constants, one table per
//! scheme for the colours and plain constants for the rest; the colour roles
//! and the globals made from them; the widgets derived from the roles, the
//! globals and the colours. A widget's colour that is a role's colour is
//! taken from the roles, not from a second copy of the constant. The preset
//! has no reader.
//!
//! The active palette ([`ResolveInputs::palette`]) and then an explicit
//! accent ([`ResolveInputs::accent`]) are laid over the constants (spec §6,
//! resolution step 2, layer 3). The accent is UIKit's tint, which its
//! controls draw as given: no colour is computed from it. An explicit
//! accent, or else a palette's `accent`, is written as given to the accent's
//! fill, and so to every widget colour the preset takes from it (the checked
//! box and radio button, the primary button, the slider's and the progress
//! bar's fill, the spinner, the selected segment, the selected tab's text,
//! the hovered and the focused text field's outline, the hovered splitter),
//! and to the selection's background, which the text selection takes with
//! the selection's text, and the unfocused selection too, which 0.7.2 states
//! as the selection's (`.superpowers/sdd/v0.8.0-plan/M2/units/accent-decision.md`,
//! iOS). The focus ring, the link, the info role, the caret and every text
//! on the accent keep the preset's: WebKit draws the focus ring, a button's
//! and a select's text in `systemBlue`, a system colour, not the tint
//! (`button.font.color`, `combo_box.font.color`, `defaults.focus_ring_color`
//! below), and the disabled slider's fill keeps the preset's constant, as
//! the checked button's colours keep theirs, measured under the default tint
//! alone ([`button`]).
//!
//! A palette states the roles its slots name (spec §7, "The roles a palette
//! states"), and each slot whose colour is the colour of the slot's role
//! replaces that colour and every widget colour the preset takes from it
//! ([`lay_palette_over`]): `window_bg` is `systemBackground`, the window's
//! background (0.7.2's `defaults.background_color`), so also the text
//! field's fill, the selected tab and every panel and bar the preset fills
//! with it; `view_bg` is `secondarySystemBackground`, the view's
//! (`defaults.surface_color`), so also the list's alternate row, the
//! inactive title bar, the list's header and the slider's thumb;
//! `sidebar_bg` the sidebar's background; `tooltip_bg` the tooltip's;
//! `foreground` is `label`, the window's text, in which the preset letters
//! its widgets, and the text of the eight surfaces (§7: one text colour);
//! `foreground_muted` is `secondaryLabel`; `border` is the separator
//! (`defaults.border.color`), so also the separator's line and the check
//! box's unchecked outline; `accent` is laid as above and `accent_fg` is the
//! text on it; `selection_bg` and `selection_fg` are the selection's, which
//! the text selection takes and the hovered menu item; `link`, `caret`,
//! `error`, `warning`, `success` and `info` state their roles alone (the
//! switch's on track keeps `systemGreen`, the switch's own colour). A slot
//! with no colour of its role in the preset states its role alone:
//! `header_bg`, `divider` and `surface_bg`, the card's, the dialog's and the
//! popover's, so their panels. An explicit accent over a palette replaces
//! its `accent`, `accent_fg`, `selection_bg` and `selection_fg` (spec §6,
//! step 2): the accent's text and the selection's text are then the
//! preset's; the palette's other slots stand.
//!
//! Defined by the preset (spec §1, "Axis values without data"): the variant
//! `ButtonVariant::Primary` (0.7.2's `primary_*` fields, which it fills from
//! the accent), a list's alternate rows (`ListKind::alternate`), the two
//! orientations of the scroll bar, no control size, and the contrast level
//! `Normal` alone. A query for anything else returns the part's base style.
//! A maximized or full-screen window has no data of its own.
//!
//! Synthesized (spec §5, `WidgetOrigin::Synthesized`): the tooltip, the
//! status bar, the splitter and the menu bar, desktop controls iOS does not
//! have, built from the preset's own colours, type and corners as 0.7.2
//! states them. Every other widget is `Native`.
//!
//! A part's exclusive state group names only the flags that part has a rule
//! for: a state without data of its own returns what the query returns
//! without that state. The groups: the button's body `PRESSED, HOVER`; every
//! other part none.
//!
//! Where two rules can both match, the later one wins (spec §3, "State
//! combinations a source does not state"); the precedences, each with its
//! source:
//! - the button's body: the pressed fill over the hovered fill (0.7.2's
//!   pressed colours win over its hover colours); the primary fill over both,
//!   as 0.7.2's connectors keep `primary_background` in every state; the
//!   disabled fill over every other, the primary one included, as 0.7.2's
//!   egui connector fills a disabled button with `button.disabled_background`
//!   where it is stated (`connectors/native-theme-egui/src/mapping.md`, its
//!   rows `button.disabled_background` and `button.primary_background`); the
//!   checked fill over the hovered and the pressed ones, and the checked and
//!   pressed fill over it, as UIKit draws a selected toggle, highlighted or
//!   not, in fills of its own (`docs/facts/ios/button.toml` on `v0.8.0-rc1`
//!   at `78ee0efd`), under the primary and the disabled fills. Its label: the
//!   checked colour over the pressed one, and the checked and pressed colour
//!   over it, as UIKit draws a highlighted selected toggle's title in a
//!   colour of its own (the same facts), the primary text over them, the
//!   disabled text over every other (`mapping.md`,
//!   `button.disabled_text_color`);
//! - the check box's and the radio button's indicator: the checked fill and
//!   outline over the hovered fill, as 0.7.2's egui connector draws a checked
//!   box ("`checkbox.hover_background` is the unchecked box's hover fill, so
//!   the hovered entry holds the checked fill too":
//!   `connectors/native-theme-egui/src/style/variants.rs`); the disabled fill
//!   over both, checked or not;
//! - the combo box's field: the disabled fill over the hovered one;
//! - the switch's track: the hovered on track over the on and the hovered
//!   off track, the disabled tracks over every other, the disabled on track
//!   (`DISABLED | CHECKED`) over the disabled off one (0.7.2's
//!   `switch.disabled_*` colours, stated for the disabled control);
//! - the text field's outline: the focused outline over the hovered one;
//!   the disabled fill over the resting one;
//! - the scroll bar's handle: the pressed colour over the hovered one;
//! - the link's label: the pressed colour, the disabled colour over it;
//! - the list's row highlight, the sidebar's item, the tab and the segmented
//!   control's segment: the selected fill over the hovered fill, as 0.7.2's
//!   egui connector puts the selection's colour in the selection's fill and
//!   the hover colour in the hovered widgets' fill
//!   (`connectors/native-theme-egui/src/mapping.md`) and egui fills a
//!   selected item with the selection's fill over its hovered look (egui
//!   0.37.0 `src/style.rs`, `Style::interact_selectable`). Their labels: the
//!   selected colour, and on the list the disabled colour over it; the menu
//!   item's label: the disabled colour over the hovered one.
//!
//! The fill under an outline (spec §2, "Fill and stroke";
//! `.superpowers/sdd/v0.8.0-plan/M2/fill-stroke-decision.md`): no source the
//! builder reads gives iOS's extents, so every outline is
//! `StrokeAlign::Inside`, the fill under the whole of it.
//!
//! An inactive window (`ThemeContext::window_active` false) has the inactive
//! title bar and title colour, which 0.7.2 states equal to the active ones
//! (`window.inactive_title_bar_*`).
//!
//! Of the context it acts on the scheme and the text scale; the preset
//! states no per-side value that differs on the left and the right and no
//! per-corner radius, so a layout from right to left changes nothing. The
//! text scale is the nearest Dynamic Type category (spec §6, resolution step
//! 2; [`dynamic_type`]): the category whose Body size, over the Body size of
//! the default category `Large`, is nearest the text scale (the smaller
//! category where two are equally near; at 1, `Large` itself), and each font
//! size scaled by its text style's row of the table, the category's size
//! over the default's: the caption's, the first title's, the display's and
//! the headline's by Caption 1, Title 1, Large Title and Headline, every
//! other font, the UI font and the monospace font and so every widget's, by
//! Body (the default text style, UIKit's `UIFontMetrics.default`); each
//! text-scale entry's line height by its style's leading, the category's
//! over the default's; the body's line height the multiplier times its
//! scaled size (0.7.2's rule of the multiplier); each `defined_size` stays
//! the unscaled size. Sizes convert at [`FONT_DPI`] first. The motion
//! preference: `MotionPreference::None` would collapse every transition, and
//! the preset states no transition, no motion token and no dialog entrance;
//! no source the builder reads states a policy of iOS's for
//! `MotionPreference::Reduced`, which therefore changes nothing. The
//! contrast and the accessibility flags change nothing here.
//!
//! Sources: `ios-13.toml` of 0.7.2 (`src/presets/ios-13.toml`), each
//! constant with the citation the TOML gives it or `(uncited)`; the TOML's
//! header says that a value without a comment has no primary source. The
//! short names its citations use (the TOML's header):
//! - HIG color 2025-06-07:
//!   <https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json>
//!   (the iOS system colours before the June 2025 update);
//! - HIG typography 2025-05-23:
//!   <https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json>
//!   ("iOS, iPadOS Dynamic Type sizes", Large (default); the copy read
//!   2026-10-11 gives the same values);
//! - HIG accessibility 2024-11-14:
//!   <https://web.archive.org/web/20241114144601/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/accessibility.json>;
//! - ios-18 expected.txt: WebKit de320ba3ff90229f983988d42d4b0f83315459da
//!   `LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt`
//!   (UIKit's iOS 18 colours);
//! - html.css, RenderThemeIOS.mm: WebKit
//!   31adf9d1605fad1fb548e86d36384161a2a060b4 (tag WebKit-7621.2.5.14.9,
//!   before the iOS 26 form-control refresh), `Source/WebCore/css/html.css`
//!   and `Source/WebCore/rendering/ios/RenderThemeIOS.mm`;
//! - RenderThemeCocoa.mm: WebKit de320ba3ff90229f983988d42d4b0f83315459da
//!   `Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm`;
//! - (WebKit): marks a value read from WebKit's drawing of iOS controls
//!   (`docs/platform-facts.md`), not from UIKit;
//! - (UIKit): marks a value read from UIKit's headers, at the SDK the
//!   citation names;
//! - `behavior_values.toml`: the measurement the caret's blink-on citation
//!   names in full (`docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15/behavior_values.toml`).
//!
//! 0.7.2's inheritance rules (`src/resolve/inheritance.rs`,
//! `src/resolve/validate.rs`, `src/resolve/validate_helpers.rs`;
//! `docs/inheritance-rules.toml`) where 0.7.2 fills a value the TOML does not
//! state. The Dynamic Type table is the HIG's
//! (`next::platform::dynamic_type`).

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{Corners, Stroke};
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::{DialogButtonOrder, LabelSide, LayoutMetrics};
use crate::next::motion::{CaretSpec, MotionTokens, UiTimings};
use crate::next::paint::Paint;
use crate::next::palette::Palette;
use crate::next::platform::dynamic_type::{
    CATEGORIES, ContentSizeCategory, DEFAULT_CATEGORY, TextRole, dynamic_type,
};
use crate::next::preset::{Preset, ResolveInputs};
use crate::next::presets::helpers::{Motion, dialog_entrance, motion, motion_tokens};
use crate::next::query::{ButtonVariant, Contrast, Orientation, Scheme, ThemeContext, WidgetState};
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
use crate::next::window::WindowChrome;

// ---------------------------------------------------------------------------
// The constants
// ---------------------------------------------------------------------------

/// The colours of one scheme the builder works from: the constants
/// `ios-13.toml` states, each colour from its 8-bit sRGB channels
/// ([`srgb8`]). A key 0.7.2 states with the same colour in both schemes as
/// another key's, for the same system colour or the same use, shares that
/// key's field; each key's citation stands at the field.
#[derive(Clone, Copy)]
struct SchemeColors {
    /// `systemBackground`.
    background: Color,
    /// `secondarySystemBackground`.
    surface: Color,
    sidebar: Color,
    /// The popover's panel and the unchecked box.
    elevated: Color,
    /// `label`.
    label: Color,
    /// `secondaryLabel`.
    muted: Color,
    separator: Color,
    accent: Color,
    on_accent: Color,
    /// `systemBlue` where WebKit draws it, not the tint.
    system_blue: Color,
    link: Color,
    selection_background: Color,
    selection_foreground: Color,
    /// `tertiaryLabel`.
    tertiary_label: Color,
    placeholder: Color,
    /// `opaqueSecondaryFill`.
    secondary_fill: Color,
    control_hover: Color,
    button_pressed: Color,
    /// A checked toggle's fill under the default tint.
    button_checked: Color,
    /// A checked toggle's fill while pressed, under the default tint.
    button_checked_pressed: Color,
    /// A checked toggle's label under the default tint.
    button_checked_foreground: Color,
    /// A checked toggle's label while pressed, under the default tint.
    button_checked_pressed_foreground: Color,
    disabled_fill: Color,
    disabled_label: Color,
    row_hover: Color,
    input_disabled: Color,
    gray: Color,
    scrollbar_hover: Color,
    link_pressed: Color,
    link_hover_layer: Color,
    /// `opaqueFill`.
    track: Color,
    slider_disabled_fill: Color,
    slider_disabled_track: Color,
    slider_thumb_hover: Color,
    /// `systemGreen`.
    system_green: Color,
    switch_off: Color,
    switch_hover_off: Color,
    switch_hover_on: Color,
    switch_disabled_off: Color,
    switch_disabled_on: Color,
    white: Color,
    warning_foreground: Color,
    tooltip_background: Color,
    shadow: Color,
    /// `systemRed`.
    danger: Color,
    /// `systemOrange`.
    warning: Color,
}

/// The light scheme's constants.
const LIGHT: SchemeColors = SchemeColors {
    // `defaults.background_color`: systemBackground (ios-18 expected.txt:7)
    // `input.background_color`: (WebKit) Canvas = systemBackground (html.css:463;
    // RenderThemeIOS.mm:1024; ios-18 expected.txt:7)
    // `tab.active_background`: 0.7.2 ios-13.toml (uncited)
    background: srgb8(0xffff_ffff),
    // `defaults.surface_color`: secondarySystemBackground (ios-18 expected.txt:133)
    // `list.alternate_row_background`, `window.inactive_title_bar_background`: 0.7.2 ios-13.toml
    // (uncited)
    surface: srgb8(0xf2f2_f7ff),
    // `sidebar.background_color`: 0.7.2 ios-13.toml (uncited)
    sidebar: srgb8(0xf2f2_f7ff),
    // `popover.background_color`, `checkbox.unchecked_background`: 0.7.2 ios-13.toml (uncited)
    elevated: srgb8(0xffff_ffff),
    // `defaults.text_color`: label (ios-18 expected.txt:73)
    // `input.font.color`, `popover.font.color`, `sidebar.font.color`: label (ios-18
    // expected.txt:73)
    // `list.hover_text_color`, `tab.hover_text_color`, `window.inactive_title_bar_text_color`:
    // 0.7.2 ios-13.toml (uncited)
    label: srgb8(0x0000_00ff),
    // `defaults.muted_color`: secondaryLabel rgba(60,60,67,0.6) (ios-18 expected.txt:145), the
    // macOS column's muted_color role (docs/platform-facts.md); #8a8a8e over white
    muted: srgb8(0x3c3c_4399),
    // `defaults.border.color`, `separator.line_color`: separator rgba(60,60,67,0.29) over white
    // (ios-18 expected.txt:157)
    // `checkbox.unchecked_border_color`: 0.7.2 ios-13.toml (uncited)
    separator: srgb8(0xc6c6_c8ff),
    // `defaults.accent_color`: systemBlue (HIG color 2025-06-07, Blue; ios-18 expected.txt:13)
    // `input.focus_border_color`, `input.hover_border_color`,
    // `segmented_control.active_background`, `splitter.hover_color`, `tab.active_text_color`:
    // 0.7.2 ios-13.toml (uncited)
    accent: srgb8(0x007a_ffff),
    // `defaults.accent_text_color`, `segmented_control.active_text_color`: 0.7.2 ios-13.toml
    // (uncited)
    on_accent: srgb8(0xffff_ffff),
    // `button.font.color`: (WebKit) systemBlue, an iOS button's text (html.css:891; ios-18
    // expected.txt:13)
    // `button.active_text_color`: (WebKit) systemBlue, a pressed iOS button keeps its blue text
    // (html.css:901-906; ios-18 expected.txt:13)
    // `combo_box.font.color`: (WebKit) systemBlue, an iOS select's text (html.css:1099; ios-18
    // expected.txt:13)
    // `defaults.focus_ring_color`: (WebKit) systemBlue, WebKit's iOS focus ring
    // (RenderThemeIOS.mm:936-943; ios-18 expected.txt:13)
    // `defaults.info_color`: systemBlue, the macOS column's info_color role
    // (docs/platform-facts.md)
    system_blue: srgb8(0x007a_ffff),
    // `defaults.link_color`, `link.hover_text_color`: 0.7.2 ios-13.toml (uncited)
    link: srgb8(0x007a_ffff),
    // `defaults.selection_background`, `menu.hover_background`: 0.7.2 ios-13.toml (uncited)
    selection_background: srgb8(0x007a_ffff),
    // `defaults.selection_text_color`, `menu.hover_text_color`: 0.7.2 ios-13.toml (uncited)
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.disabled_text_color`: (WebKit) tertiaryLabel rgba(60,60,67,0.298), the text of a
    // disabled button or select (html.css:996-1003; ios-18 expected.txt:181); #c5c5c7 over white
    // `button.disabled_text_color`: (WebKit) tertiaryLabel, a disabled button's text
    // (html.css:996-1003; ios-18 expected.txt:181)
    // `combo_box.disabled_text_color`: (WebKit) tertiaryLabel, a disabled select's text
    // (html.css:996-1003; ios-18 expected.txt:181)
    tertiary_label: srgb8(0x3c3c_434c),
    // `input.placeholder_color`: placeholderText, "The color for placeholder text in controls or
    // text views" (UIKit https://developer.apple.com/tutorials/data/documentation/uikit/uicolor/placeholdertext.json,
    // read 2026-10-11), rgba(60,60,67,0.298) (ios-18 expected.txt:109); #c5c5c7 over white
    placeholder: srgb8(0x3c3c_434c),
    // `button.background_color`: (WebKit) opaqueSecondaryFill, an iOS button's fill (html.css:895;
    // ios-18 expected.txt:85)
    // `combo_box.background_color`: (WebKit) opaqueSecondaryFill, an iOS select's fill
    // (html.css:1106; ios-18 expected.txt:85)
    secondary_fill: srgb8(0xe9e9_eaff),
    // `button.hover_background`, `checkbox.hover_background`, `combo_box.hover_background`,
    // `segmented_control.hover_background`: 0.7.2 ios-13.toml (uncited)
    control_hover: srgb8(0xd1d1_d6ff),
    // `button.active_background`: 0.7.2 ios-13.toml (uncited)
    button_pressed: srgb8(0xc7c7_ccff),
    // `button.checked_background`: a toggle's selected fill, the default tint at 0x2e (measured,
    // iOS 18.6 on the iPhone 16 Pro simulator, ios-host run 41020880059,
    // `ios-host-macos-15/analysis.toml`, `cells.light.gray.*.selected.fill`, the plain style the
    // same; `docs/facts/ios/button.toml` on `v0.8.0-rc1` at `78ee0efd`)
    button_checked: srgb8(0x007a_ff2e),
    // the same, pressed: the default tint at 0x1e (the same run,
    // `cells.light.gray.*.selected_highlighted.fill`, the plain style the same)
    button_checked_pressed: srgb8(0x007a_ff1e),
    // `button.checked_text_color`: the default tint (the same run,
    // `cells.light.gray.*.selected.label_fill`, the plain style the same)
    button_checked_foreground: srgb8(0x007a_ffff),
    // the same, pressed: the default tint at 0xbf (the same run,
    // `cells.light.gray.*.selected_highlighted.label_fill`, the plain style the same)
    button_checked_pressed_foreground: srgb8(0x007a_ffbf),
    // `button.disabled_background`, `checkbox.disabled_background`,
    // `combo_box.disabled_background`: 0.7.2 ios-13.toml (uncited)
    disabled_fill: srgb8(0xe5e5_ea80),
    // `checkbox.disabled_text_color`, `input.disabled_text_color`, `link.disabled_text_color`,
    // `list.disabled_text_color`, `menu.disabled_text_color`: 0.7.2 ios-13.toml (uncited)
    disabled_label: srgb8(0xc7c7_ccff),
    // `expander.hover_background`, `list.hover_background`, `sidebar.hover_background`,
    // `tab.hover_background`, `slider.disabled_thumb_color`: 0.7.2 ios-13.toml (uncited)
    row_hover: srgb8(0xe5e5_eaff),
    // `input.disabled_background`: 0.7.2 ios-13.toml (uncited)
    input_disabled: srgb8(0xf2f2_f7ff),
    // `expander.arrow_color`, `scrollbar.thumb_active_color`: 0.7.2 ios-13.toml (uncited)
    gray: srgb8(0x8e8e_93ff),
    // `scrollbar.thumb_hover_color`: 0.7.2 ios-13.toml (uncited)
    scrollbar_hover: srgb8(0xa0a0_a5ff),
    // `link.active_text_color`: 0.7.2 ios-13.toml (uncited)
    link_pressed: srgb8(0x0056_b3ff),
    // `link.hover_background`: 0.7.2 ios-13.toml (uncited)
    link_hover_layer: srgb8(0x007a_ff18),
    // `progress_bar.track_color`: (WebKit) opaqueFill, the progress bar's track
    // (RenderThemeIOS.mm:752; ios-18 expected.txt:79)
    // `slider.track_color`: (WebKit) opaqueFill, the range control's track
    // (RenderThemeIOS.mm:623; ios-18 expected.txt:79)
    track: srgb8(0xe4e4_e5ff),
    // `slider.disabled_fill_color`: 0.7.2 ios-13.toml (uncited)
    slider_disabled_fill: srgb8(0x007a_ff80),
    // `slider.disabled_track_color`: 0.7.2 ios-13.toml (uncited)
    slider_disabled_track: srgb8(0xc7c7_ccff),
    // `slider.thumb_hover_color`: 0.7.2 ios-13.toml (uncited)
    slider_thumb_hover: srgb8(0xf0f0_f0ff),
    // `defaults.success_color`: systemGreen (HIG color 2025-06-07, Green; ios-18 expected.txt:43)
    // `switch.checked_background`: (WebKit) the on track is systemGreen on iOS
    // (RenderThemeCocoa.mm:271-275, :333; ios-18 expected.txt:43)
    system_green: srgb8(0x34c7_59ff),
    // `switch.unchecked_background`: (WebKit) the off track, rgba(120,120,128,41/255)
    // (RenderThemeCocoa.mm:277); #e9e9eb over white
    switch_off: srgb8(0x7878_8029),
    // `switch.hover_unchecked_background`: 0.7.2 ios-13.toml (uncited)
    switch_hover_off: srgb8(0xb0b0_b5ff),
    // `switch.hover_checked_background`: 0.7.2 ios-13.toml (uncited)
    switch_hover_on: srgb8(0x2d8a_47ff),
    // `switch.disabled_unchecked_background`: 0.7.2 ios-13.toml (uncited)
    switch_disabled_off: srgb8(0x7878_8010),
    // `switch.disabled_checked_background`: (WebKit) a disabled switch's track is its track colour
    // with the alpha multiplied by 0.4 (RenderThemeCocoa.mm:816-817, the painter of :271-279): on
    // 255 x 0.4 = 102 (0x66), #aee9bd over white; off 41 x 0.4 = 16.4, rounded to 16 (0x10),
    // #f7f7f7 over white
    switch_disabled_on: srgb8(0x34c7_5966),
    // `switch.thumb_background`: (WebKit) white (RenderThemeCocoa.mm:763)
    // `switch.disabled_thumb_color`: (WebKit) the thumb is white, disabled too
    // (RenderThemeCocoa.mm:763)
    // `defaults.danger_text_color`, `defaults.info_text_color`, `defaults.success_text_color`,
    // `tooltip.font.color`: 0.7.2 ios-13.toml (uncited)
    white: srgb8(0xffff_ffff),
    // `defaults.warning_text_color`: 0.7.2 ios-13.toml (uncited)
    warning_foreground: srgb8(0x0000_00ff),
    // `tooltip.background_color`: 0.7.2 ios-13.toml (uncited)
    tooltip_background: srgb8(0x2c2c_2eff),
    // `defaults.shadow_color`: 0.7.2 ios-13.toml (uncited)
    shadow: srgb8(0x0000_0018),
    // `defaults.danger_color`: systemRed (HIG color 2025-06-07, Red; ios-18 expected.txt:127)
    danger: srgb8(0xff3b_30ff),
    // `defaults.warning_color`: systemOrange (HIG color 2025-06-07, Orange; ios-18 expected.txt:97)
    warning: srgb8(0xff95_00ff),
};

/// The dark scheme's constants.
const DARK: SchemeColors = SchemeColors {
    // `defaults.background_color`: systemBackground (ios-18 expected.txt:9)
    // `input.background_color`: (WebKit) Canvas = systemBackground (html.css:463;
    // RenderThemeIOS.mm:1024; ios-18 expected.txt:9)
    // `tab.active_background`: 0.7.2 ios-13.toml (uncited)
    background: srgb8(0x0000_00ff),
    // `defaults.surface_color`: secondarySystemBackground (ios-18 expected.txt:135)
    // `list.alternate_row_background`, `window.inactive_title_bar_background`: 0.7.2 ios-13.toml
    // (uncited)
    surface: srgb8(0x1c1c_1eff),
    // `sidebar.background_color`: 0.7.2 ios-13.toml (uncited)
    sidebar: srgb8(0x1c1c_1eff),
    // `popover.background_color`, `checkbox.unchecked_background`: 0.7.2 ios-13.toml (uncited)
    elevated: srgb8(0x1c1c_1eff),
    // `defaults.text_color`: label (ios-18 expected.txt:75)
    // `input.font.color`, `popover.font.color`, `sidebar.font.color`: label (ios-18
    // expected.txt:75)
    // `list.hover_text_color`, `tab.hover_text_color`, `window.inactive_title_bar_text_color`:
    // 0.7.2 ios-13.toml (uncited)
    label: srgb8(0xffff_ffff),
    // `defaults.muted_color`: secondaryLabel rgba(235,235,245,0.6) (ios-18 expected.txt:147), the
    // macOS column's muted_color role (docs/platform-facts.md); #8d8d93 over black
    muted: srgb8(0xebeb_f599),
    // `defaults.border.color`, `separator.line_color`, `checkbox.unchecked_border_color`: 0.7.2
    // ios-13.toml (uncited)
    separator: srgb8(0x3838_3aff),
    // `defaults.accent_color`: systemBlue (HIG color 2025-06-07, Blue; ios-18 expected.txt:15)
    // `input.focus_border_color`, `input.hover_border_color`,
    // `segmented_control.active_background`, `splitter.hover_color`, `tab.active_text_color`:
    // 0.7.2 ios-13.toml (uncited)
    accent: srgb8(0x0a84_ffff),
    // `defaults.accent_text_color`, `segmented_control.active_text_color`: 0.7.2 ios-13.toml
    // (uncited)
    on_accent: srgb8(0xffff_ffff),
    // `button.font.color`: (WebKit) systemBlue, an iOS button's text (html.css:891; ios-18
    // expected.txt:15)
    // `button.active_text_color`: (WebKit) systemBlue, a pressed iOS button keeps its blue text
    // (html.css:901-906; ios-18 expected.txt:15)
    // `combo_box.font.color`: (WebKit) systemBlue, an iOS select's text (html.css:1099; ios-18
    // expected.txt:15)
    // `defaults.focus_ring_color`: (WebKit) systemBlue, WebKit's iOS focus ring
    // (RenderThemeIOS.mm:936-943; ios-18 expected.txt:15)
    // `defaults.info_color`: systemBlue, the macOS column's info_color role
    // (docs/platform-facts.md)
    system_blue: srgb8(0x0a84_ffff),
    // `defaults.link_color`, `link.hover_text_color`: 0.7.2 ios-13.toml (uncited)
    link: srgb8(0x0a84_ffff),
    // `defaults.selection_background`, `menu.hover_background`: 0.7.2 ios-13.toml (uncited)
    selection_background: srgb8(0x0a84_ffff),
    // `defaults.selection_text_color`, `menu.hover_text_color`: 0.7.2 ios-13.toml (uncited)
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.disabled_text_color`: (WebKit) tertiaryLabel rgba(235,235,245,0.298), the text of
    // a disabled button or select (html.css:996-1003; ios-18 expected.txt:183); #464649 over black
    // `button.disabled_text_color`: (WebKit) tertiaryLabel, a disabled button's text
    // (html.css:996-1003; ios-18 expected.txt:183)
    // `combo_box.disabled_text_color`: (WebKit) tertiaryLabel, a disabled select's text
    // (html.css:996-1003; ios-18 expected.txt:183)
    tertiary_label: srgb8(0xebeb_f54c),
    // `input.placeholder_color`: placeholderText, "The color for placeholder text in controls or
    // text views" (UIKit https://developer.apple.com/tutorials/data/documentation/uikit/uicolor/placeholdertext.json,
    // read 2026-10-11), rgba(235,235,245,0.298) (ios-18 expected.txt:111); #464649 over black
    placeholder: srgb8(0xebeb_f54c),
    // `button.background_color`: (WebKit) opaqueSecondaryFill, an iOS button's fill (html.css:895;
    // ios-18 expected.txt:87)
    // `combo_box.background_color`: (WebKit) opaqueSecondaryFill, an iOS select's fill
    // (html.css:1106; ios-18 expected.txt:87)
    secondary_fill: srgb8(0x2626_29ff),
    // `button.hover_background`, `checkbox.hover_background`, `combo_box.hover_background`,
    // `segmented_control.hover_background`: 0.7.2 ios-13.toml (uncited)
    control_hover: srgb8(0x3a3a_3cff),
    // `button.active_background`: 0.7.2 ios-13.toml (uncited)
    button_pressed: srgb8(0x4848_4aff),
    // `button.checked_background`: a toggle's selected fill, the default tint at 0x40 (measured,
    // iOS 18.6 on the iPhone 16 Pro simulator, ios-host run 41020880059,
    // `ios-host-macos-15/analysis.toml`, `cells.dark.gray.*.selected.fill`, the plain style the
    // same; `docs/facts/ios/button.toml` on `v0.8.0-rc1` at `78ee0efd`)
    button_checked: srgb8(0x0a84_ff40),
    // the same, pressed: the default tint at 0x2d (the same run,
    // `cells.dark.gray.*.selected_highlighted.fill`, the plain style the same)
    button_checked_pressed: srgb8(0x0a84_ff2d),
    // `button.checked_text_color`: the default tint (the same run,
    // `cells.dark.gray.*.selected.label_fill`, the plain style the same)
    button_checked_foreground: srgb8(0x0a84_ffff),
    // the same, pressed: #2390ff (the same run,
    // `cells.dark.gray.*.selected_highlighted.label_fill`, the plain style the same)
    button_checked_pressed_foreground: srgb8(0x2390_ffff),
    // `button.disabled_background`, `checkbox.disabled_background`,
    // `combo_box.disabled_background`: 0.7.2 ios-13.toml (uncited)
    disabled_fill: srgb8(0x2c2c_2e80),
    // `checkbox.disabled_text_color`, `input.disabled_text_color`, `link.disabled_text_color`,
    // `list.disabled_text_color`, `menu.disabled_text_color`: 0.7.2 ios-13.toml (uncited)
    disabled_label: srgb8(0x3a3a_3cff),
    // `expander.hover_background`, `list.hover_background`, `sidebar.hover_background`,
    // `tab.hover_background`, `slider.disabled_thumb_color`: 0.7.2 ios-13.toml (uncited)
    row_hover: srgb8(0x2c2c_2eff),
    // `input.disabled_background`: 0.7.2 ios-13.toml (uncited)
    input_disabled: srgb8(0x2c2c_2eff),
    // `expander.arrow_color`, `scrollbar.thumb_active_color`: 0.7.2 ios-13.toml (uncited)
    gray: srgb8(0x8e8e_93ff),
    // `scrollbar.thumb_hover_color`: 0.7.2 ios-13.toml (uncited)
    scrollbar_hover: srgb8(0x6363_66ff),
    // `link.active_text_color`: 0.7.2 ios-13.toml (uncited)
    link_pressed: srgb8(0x409c_ffff),
    // `link.hover_background`: 0.7.2 ios-13.toml (uncited)
    link_hover_layer: srgb8(0x0a84_ff18),
    // `progress_bar.track_color`: (WebKit) opaqueFill, the progress bar's track
    // (RenderThemeIOS.mm:752; ios-18 expected.txt:81)
    // `slider.track_color`: (WebKit) opaqueFill, the range control's track
    // (RenderThemeIOS.mm:623; ios-18 expected.txt:81)
    track: srgb8(0x2b2b_2eff),
    // `slider.disabled_fill_color`: 0.7.2 ios-13.toml (uncited)
    slider_disabled_fill: srgb8(0x0a84_ff80),
    // `slider.disabled_track_color`: 0.7.2 ios-13.toml (uncited)
    slider_disabled_track: srgb8(0x3838_3aff),
    // `slider.thumb_hover_color`: 0.7.2 ios-13.toml (uncited)
    slider_thumb_hover: srgb8(0x3a3a_3cff),
    // `defaults.success_color`: systemGreen (HIG color 2025-06-07, Green; ios-18 expected.txt:45)
    // `switch.checked_background`: (WebKit) the on track is systemGreen on iOS
    // (RenderThemeCocoa.mm:271-275, :333; ios-18 expected.txt:45)
    system_green: srgb8(0x30d1_58ff),
    // `switch.unchecked_background`: (WebKit) the off track, rgba(120,120,128,82/255)
    // (RenderThemeCocoa.mm:279); #272729 over black
    switch_off: srgb8(0x7878_8052),
    // `switch.hover_unchecked_background`: 0.7.2 ios-13.toml (uncited)
    switch_hover_off: srgb8(0x4848_4aff),
    // `switch.hover_checked_background`: 0.7.2 ios-13.toml (uncited)
    switch_hover_on: srgb8(0x2581_3cff),
    // `switch.disabled_unchecked_background`: 0.7.2 ios-13.toml (uncited)
    switch_disabled_off: srgb8(0x7878_8021),
    // `switch.disabled_checked_background`: (WebKit) a disabled switch's track is its track colour
    // with the alpha multiplied by 0.4 (RenderThemeCocoa.mm:816-817, the painter of :271-279): on
    // 255 x 0.4 = 102 (0x66), #135423 over black; off 82 x 0.4 = 32.8, rounded to 33 (0x21),
    // #101011 over black
    switch_disabled_on: srgb8(0x30d1_5866),
    // `switch.thumb_background`: (WebKit) white (RenderThemeCocoa.mm:763)
    // `switch.disabled_thumb_color`: (WebKit) the thumb is white, disabled too
    // (RenderThemeCocoa.mm:763)
    // `defaults.danger_text_color`, `defaults.info_text_color`, `defaults.success_text_color`,
    // `tooltip.font.color`: 0.7.2 ios-13.toml (uncited)
    white: srgb8(0xffff_ffff),
    // `defaults.warning_text_color`: 0.7.2 ios-13.toml (uncited)
    warning_foreground: srgb8(0x0000_00ff),
    // `tooltip.background_color`: 0.7.2 ios-13.toml (uncited)
    tooltip_background: srgb8(0x2c2c_2eff),
    // `defaults.shadow_color`: 0.7.2 ios-13.toml (uncited)
    shadow: srgb8(0x0000_0040),
    // `defaults.danger_color`: systemRed (HIG color 2025-06-07, Red; ios-18 expected.txt:129)
    danger: srgb8(0xff45_3aff),
    // `defaults.warning_color`: systemOrange (HIG color 2025-06-07, Orange; ios-18 expected.txt:99)
    warning: srgb8(0xff9f_0aff),
};

/// No colour: `link.background_color`, `#00000000` (0.7.2 ios-13.toml (uncited)).
const TRANSPARENT: Color = srgb8(0x0000_0000);

// `defaults.font.family`: SF Pro, Body: Regular 17 pt (HIG typography 2025-05-23)
const FONT_FAMILY: &str = "SF Pro";
// `defaults.font.size`, `defaults.font.defined_size`: 0.7.2 ios-13.toml (uncited)
const FONT_SIZE_PT: f32 = 17.0;
// `defaults.font.weight`: 0.7.2 ios-13.toml (uncited)
const FONT_WEIGHT: u16 = 400;
// `defaults.mono_font.family`: 0.7.2 ios-13.toml (uncited)
const MONO_FAMILY: &str = "SF Mono";
// `defaults.mono_font.size`, `defaults.mono_font.defined_size`: 0.7.2 ios-13.toml (uncited)
const MONO_SIZE_PT: f32 = 17.0;
// `defaults.mono_font.weight`: 0.7.2 ios-13.toml (uncited)
const MONO_WEIGHT: u16 = 400;
// `defaults.line_height`: docs/platform-facts.md (§2.1.1): 1.19, SF Pro's font metrics,
// (1950+494+0)/2048, as macos-monterey.toml states it; SF Pro is the system font in iOS and iPadOS
// (HIG typography 2025-05-23)
const LINE_HEIGHT: f32 = 1.19;
// `text_scale.caption.size`: Caption 1: Regular 12 pt, leading 16 (HIG typography 2025-05-23)
const CAPTION_SIZE_PT: f32 = 12.0;
// `text_scale.caption.weight`: 0.7.2 ios-13.toml (uncited)
const CAPTION_WEIGHT: u16 = 400;
// `text_scale.caption.line_height`: 0.7.2 ios-13.toml (uncited)
const CAPTION_LINE_HEIGHT_PT: f32 = 16.0;
// `text_scale.section_heading.size`: Headline: Semibold 17 pt, leading 22 (HIG typography
// 2025-05-23); the macOS column's role, .headline (docs/platform-facts.md)
const HEADLINE_SIZE_PT: f32 = 17.0;
// `text_scale.section_heading.weight`: 0.7.2 ios-13.toml (uncited)
const HEADLINE_WEIGHT: u16 = 600;
// `text_scale.section_heading.line_height`: 0.7.2 ios-13.toml (uncited)
const HEADLINE_LINE_HEIGHT_PT: f32 = 22.0;
// `text_scale.dialog_title.size`: Title 1: Regular 28 pt, leading 34 (HIG typography 2025-05-23);
// the macOS column's role, .title1 (docs/platform-facts.md)
const TITLE1_SIZE_PT: f32 = 28.0;
// `text_scale.dialog_title.weight`: 0.7.2 ios-13.toml (uncited)
const TITLE1_WEIGHT: u16 = 400;
// `text_scale.dialog_title.line_height`: 0.7.2 ios-13.toml (uncited)
const TITLE1_LINE_HEIGHT_PT: f32 = 34.0;
// `text_scale.display.size`: Large Title: Regular 34 pt, leading 41 (HIG typography 2025-05-23);
// the macOS column's role, .largeTitle (docs/platform-facts.md)
const DISPLAY_SIZE_PT: f32 = 34.0;
// `text_scale.display.weight`: 0.7.2 ios-13.toml (uncited)
const DISPLAY_WEIGHT: u16 = 400;
// `text_scale.display.line_height`: 0.7.2 ios-13.toml (uncited)
const DISPLAY_LINE_HEIGHT_PT: f32 = 41.0;

// `defaults.border.corner_radius`: 0.7.2 ios-13.toml (uncited)
const CORNER_RADIUS: f32 = 10.0;
// `defaults.border.corner_radius_lg`: 0.7.2 ios-13.toml (uncited)
const CORNER_RADIUS_LG: f32 = 13.0;
// `defaults.border.line_width`: 0.7.2 ios-13.toml (uncited)
const BORDER_WIDTH: f32 = 0.5;
// `defaults.border.opacity`: the text colour's share that gives color over the background
// (docs/platform-facts.md): black over white at 57/255 = 0.224 (red, green) and 55/255 = 0.216
// (blue) (light); white over black at 56/255 = 0.220 (red, green) and 58/255 = 0.227 (blue)
// (dark)
const BORDER_OPACITY: f32 = 0.22;
// `defaults.border.shadow_enabled`: 0.7.2 ios-13.toml (uncited)
const SHADOW_ENABLED: bool = true;
// `defaults.disabled_opacity`: 0.7.2 ios-13.toml (uncited)
const DISABLED_OPACITY: f32 = 0.3;
// (WebKit) pressedStateOpacity, "the opacity applied to UIKit controls" (RenderThemeIOS.mm:830-831),
// by which a pressed check box's or radio button's fill and mark (:1390-1393, :1416-1417) and a
// pressed select's background and text (:862-870) are multiplied
const PRESSED_STATE_OPACITY: f32 = 0.75;
// `button.border.line_width`: (WebKit) an iOS button's `border: 1px solid
// -webkit-control-background` (html.css:889)
const BUTTON_BORDER_WIDTH: f32 = 1.0;
// `defaults.focus_ring_width`: 0.7.2 ios-13.toml (uncited)
const FOCUS_RING_WIDTH: f32 = 2.0;
// `defaults.focus_ring_offset`: 0.7.2 ios-13.toml (uncited)
const FOCUS_RING_OFFSET: f32 = 1.0;
// `defaults.focus_ring_keyboard_only`: (UIKit) the focus effect applies whenever the item is
// focused (UIView.h:166-167, Xcode 16.4 iPhoneSimulator SDK)
const FOCUS_RING_KEYBOARD_ONLY: bool = false;

// `icon_set`: SF Symbols, the system symbol set (HIG
// https://developer.apple.com/tutorials/data/design/human-interface-guidelines/sf-symbols.json,
// read 2026-10-11)
const ICON_SET: IconSet = IconSet::SfSymbols;
// `icon_theme`: 0.7.2 ios-13.toml (uncited)
const ICON_THEME: &str = "sf-symbols";
// `defaults.icon_sizes.toolbar`: 0.7.2 ios-13.toml (uncited)
const ICON_TOOLBAR: f32 = 24.0;
// `defaults.icon_sizes.small`: 0.7.2 ios-13.toml (uncited)
const ICON_SMALL: f32 = 16.0;
// `defaults.icon_sizes.large`: 0.7.2 ios-13.toml (uncited)
const ICON_LARGE: f32 = 32.0;
// `defaults.icon_sizes.dialog`: 0.7.2 ios-13.toml (uncited)
const ICON_DIALOG: f32 = 22.0;
// `defaults.icon_sizes.panel`: 0.7.2 ios-13.toml (uncited)
const ICON_PANEL: f32 = 20.0;

// `layout.widget_gap`: 0.7.2 ios-13.toml (uncited)
const WIDGET_GAP: f32 = 8.0;
// `layout.container_margin`: the default layout margins of most views, 8 pt (UIKit
// https://developer.apple.com/tutorials/data/documentation/uikit/uiview/directionallayoutmargins.json,
// read 2026-10-11)
const CONTAINER_MARGIN: f32 = 8.0;
// `layout.window_margin`: 0.7.2 ios-13.toml (uncited)
const WINDOW_MARGIN: f32 = 20.0;
// `layout.section_gap`: 0.7.2 ios-13.toml (uncited)
const SECTION_GAP: f32 = 20.0;

// `input.caret_blink_on`: measured on iOS 18.6, which no platform-facts column covers: the caret's
// blink animation shows it, then fades it out, for 0.65 s of each 1 s cycle
// (docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15/behavior_values.toml)
const CARET_BLINK_ON_MS: u32 = 650;
// `input.caret_blink_off`: hidden, then fading in, for the other 0.35 s (behavior_values.toml)
const CARET_BLINK_OFF_MS: u32 = 350;

// `button.min_width`: 0.7.2 ios-13.toml (uncited)
const BUTTON_MIN_WIDTH: f32 = 64.0;
// `button.min_height`, `combo_box.min_height`, `input.min_height`: the HIG's minimum hit target,
// 44 x 44 pt (HIG accessibility 2024-11-14), used as the drawn min_height
const MIN_HIT_TARGET: f32 = 44.0;
// `button.icon_text_gap`: 0.7.2 ios-13.toml (uncited)
const BUTTON_ICON_TEXT_GAP: f32 = 6.0;

// `checkbox.indicator_width`: 0.7.2 ios-13.toml (uncited)
const CHECKBOX_INDICATOR_SIZE: f32 = 22.0;
// (WebKit) the empty box's outline as it shows: stroked `checkboxRadioBorderWidth * 2` = 3 on its
// edge and clipped to it, so 1.5 inside (RenderThemeIOS.mm:1354, :1484-1491 at WebKit 31adf9d)
const CHECKBOX_OUTLINE_WIDTH: f32 = 1.5;
// `checkbox.label_gap`: 0.7.2 ios-13.toml (uncited)
const CHECKBOX_LABEL_GAP: f32 = 8.0;

// `combo_box.min_width`: 0.7.2 ios-13.toml (uncited)
const COMBO_BOX_MIN_WIDTH: f32 = 120.0;
// `combo_box.arrow_icon_size`: 0.7.2 ios-13.toml (uncited)
const COMBO_BOX_ARROW_SIZE: f32 = 12.0;

// `dialog.min_width`: 0.7.2 ios-13.toml (uncited)
const DIALOG_MIN_WIDTH: f32 = 270.0;
// `dialog.max_width`: 0.7.2 ios-13.toml (uncited)
const DIALOG_MAX_WIDTH: f32 = 560.0;
// `dialog.min_height`: 0.7.2 ios-13.toml (uncited)
const DIALOG_MIN_HEIGHT: f32 = 140.0;
// `dialog.max_height`: 0.7.2 ios-13.toml (uncited)
const DIALOG_MAX_HEIGHT: f32 = 600.0;
// `dialog.button_gap`: 0.7.2 ios-13.toml (uncited)
const DIALOG_BUTTON_GAP: f32 = 8.0;
// `dialog.icon_size`: 0.7.2 ios-13.toml (uncited)
const DIALOG_ICON_SIZE: f32 = 32.0;
// `dialog.button_order`: "Always place the default button on the trailing side" (HIG
// https://developer.apple.com/tutorials/data/design/human-interface-guidelines/alerts.json, read
// 2026-10-11)
const DIALOG_BUTTON_ORDER: DialogButtonOrder = DialogButtonOrder::PrimaryRight;

// `expander.header_height`: 0.7.2 ios-13.toml (uncited)
const EXPANDER_HEADER_HEIGHT: f32 = 44.0;
// `expander.arrow_icon_size`: 0.7.2 ios-13.toml (uncited)
const EXPANDER_ARROW_SIZE: f32 = 12.0;

// `link.underline_enabled`: 0.7.2 ios-13.toml (uncited)
const LINK_DECORATION: DecorationKind = DecorationKind::None;

// `menu.icon_text_gap`: 0.7.2 ios-13.toml (uncited)
const MENU_ICON_TEXT_GAP: f32 = 8.0;

// `progress_bar.min_width`: 0.7.2 ios-13.toml (uncited)
const PROGRESS_BAR_MIN_WIDTH: f32 = 100.0;
// `progress_bar.track_height`: (WebKit) barBlockSize (RenderThemeIOS.mm:718)
const PROGRESS_BAR_TRACK_HEIGHT: f32 = 4.0;

// `scrollbar.groove_width`: 0.7.2 ios-13.toml (uncited)
const SCROLLBAR_GROOVE_WIDTH: f32 = 6.0;
// `scrollbar.min_thumb_length`: 0.7.2 ios-13.toml (uncited)
const SCROLLBAR_MIN_THUMB_LENGTH: f32 = 30.0;
// `scrollbar.thumb_width`: 0.7.2 ios-13.toml (uncited)
const SCROLLBAR_THUMB_WIDTH: f32 = 3.0;
// `scrollbar.overlay_mode`: a translucent scroll indicator shown while scrolling (HIG
// https://developer.apple.com/tutorials/data/design/human-interface-guidelines/scroll-views.json,
// read 2026-10-11)
const SCROLLBAR_OVERLAY: bool = true;

// `segmented_control.segment_height`: 0.7.2 ios-13.toml (uncited)
const SEGMENT_HEIGHT: f32 = 32.0;
// `segmented_control.separator_width`: 0.7.2 ios-13.toml (uncited)
const SEGMENT_SEPARATOR_WIDTH: f32 = 1.0;

// `slider.track_height`: (WebKit) kTrackThickness, the track of WebKit's HTML range control
// (RenderThemeIOS.mm:560)
const SLIDER_TRACK_HEIGHT: f32 = 4.0;
// `slider.thumb_diameter`: no UIKit source. The same range control's thumb is 16
// (kDefaultSliderThumbSize, RenderThemeIOS.mm:562, :657-661), which M2c Step 6 did not take as
// UISlider's (M2c-21)
const SLIDER_THUMB_DIAMETER: f32 = 28.0;
// `slider.tick_mark_length`: no UIKit source. The same range control's ticks are 1 x 3, under a
// WebKit comment that says the values need correcting (RenderThemeIOS.mm:796-800)
const SLIDER_TICK_LENGTH: f32 = 4.0;

// `spinner.diameter`: 0.7.2 ios-13.toml (uncited)
const SPINNER_DIAMETER: f32 = 24.0;
// `spinner.min_diameter`: 0.7.2 ios-13.toml (uncited)
const SPINNER_MIN_DIAMETER: f32 = 16.0;
// `spinner.stroke_width`: 0.7.2 ios-13.toml (uncited)
const SPINNER_STROKE_WIDTH: f32 = 2.0;

// `splitter.divider_width`: 0.7.2 ios-13.toml (uncited)
const SPLITTER_DIVIDER_WIDTH: f32 = 1.0;

// `switch.track_width`: (WebKit) 51 x 31 (RenderThemeIOS.mm:668-669; RenderThemeCocoa.mm:216-218)
const SWITCH_TRACK_WIDTH: f32 = 51.0;
// `switch.track_height`: 0.7.2 ios-13.toml (uncited)
const SWITCH_TRACK_HEIGHT: f32 = 31.0;
// `switch.track_radius`: 0.7.2 ios-13.toml (uncited)
const SWITCH_TRACK_RADIUS: f32 = 15.5;
// `switch.thumb_diameter`: (WebKit) the track height less a 2 pt margin on each side
// (RenderThemeCocoa.mm:770-771)
const SWITCH_THUMB_DIAMETER: f32 = 27.0;

// `tab.min_width`: 0.7.2 ios-13.toml (uncited)
const TAB_MIN_WIDTH: f32 = 64.0;
// `tab.min_height`: 0.7.2 ios-13.toml (uncited)
const TAB_MIN_HEIGHT: f32 = 49.0;

// `tooltip.max_width`: 0.7.2 ios-13.toml (uncited)
const TOOLTIP_MAX_WIDTH: f32 = 300.0;

/// The state precedence of a part with rules for pressed and hover: pressed
/// over hover.
const PRESSED_HOVER: &[&[WidgetState]] = &[&[WidgetState::PRESSED, WidgetState::HOVER]];

/// The font DPI a point size converts at: 72, "where a point is a pixel"
/// (spec §4, `Typography::font_dpi`: "72.0 Apple"; 0.7.2's
/// `ResolutionContext::from_system` on macOS, `src/resolve/context.rs`).
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

/// The contrast levels the preset defines (spec §1, "Axis values without
/// data"): `Normal` alone, as the module documentation says.
pub(crate) const CONTRAST_LEVELS: &[Contrast] = &[Contrast::Normal];

/// The theme of `ios-13` for `ctx.scheme`, from the preset's constants with
/// the active palette laid over them ([`lay_palette_over`]), then an
/// explicit accent ([`lay_accent_over`]), as the module documentation says
/// (spec §6, resolution step 2). The preset has no reader.
pub(crate) fn resolve(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let mut scheme_colors = match ctx.scheme {
        Scheme::Light => LIGHT,
        Scheme::Dark => DARK,
    };
    if let Some(p) = inputs.palette {
        lay_palette_over(&mut scheme_colors, p, inputs.accent.is_some());
    }
    if let Some(accent) = inputs.accent {
        lay_accent_over(&mut scheme_colors, accent);
    }
    let c = &scheme_colors;
    let colors = roles(c, inputs.palette);
    let units = FontUnits {
        dpi: FONT_DPI,
        category: nearest_category(ctx.text_scale),
    };
    let typography = typography(&colors, units);
    // No source the builder reads states a policy of iOS's for reduced motion: `Reduced` runs
    // as `Full`.
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
        name: Arc::from(Preset::Ios13.display_name()),
        preset: Preset::Ios13,
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
            tooltip_show_ms: None,
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
        button: button(c, &colors, &typography, &layout),
        checkbox: checkbox(c, &colors, &typography, &layout),
        radio: radio(c, &colors, &typography),
        scrollbar: scrollbar(c, &colors),
        list: list(c, &colors, &typography, &layout),
        tab: tab(c, &colors, &typography),
        menu: menu(c, &colors, &typography),
        menu_bar: menu_bar(&colors),
        sidebar: sidebar(c, &colors, &typography, &layout),
        toolbar: toolbar(&colors, &typography, &layout),
        status_bar: status_bar(&colors, &typography, &layout),
        tooltip: tooltip(c, &colors, &typography, &layout),
        window: window(&colors, &typography, &layout),
        input: input(c, &colors, &typography, &layout, true),
        text_area: input(c, &colors, &typography, &layout, false),
        link: link(c, &colors, &typography),
        slider: slider(c, &colors),
        progress_bar: progress_bar(c, &colors, &layout),
        splitter: splitter(&colors),
        separator: separator(&colors, &layout),
        spinner: spinner(&colors),
        popover: popover(c, &colors, &typography, &layout),
        switch: switch(c, &colors),
        dialog: dialog(&colors, &typography, &layout, motion),
        combo_box: combo_box(c, &colors, &typography, &layout),
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
/// is `systemBackground`, lettered in `label`, its muted text
/// `secondaryLabel`, its outline the separator; the view's surface
/// `secondarySystemBackground` (`defaults.surface_color`); the tooltip's its
/// own colours. The selection, lettered in its text, is also the unfocused
/// and the text selection (0.7.2: `selection_inactive_background`,
/// `selection_inactive_text_color` and the text selection's colours are the
/// selection's where unstated, `src/resolve/inheritance.rs`,
/// `resolve_defaults_internal`); the caret is the window's text (0.7.2:
/// `input.caret_color` is `defaults.text_color` where unstated,
/// `docs/inheritance-rules.toml`, `per_platform`); the focus ring and the
/// info role WebKit's `systemBlue`. The status roles are Apple's system
/// colours (spec §4, `RoleOrigin::SystemColor`).
///
/// The active palette `palette`, whose colours `c` holds already, states
/// the roles its slots name (spec §7, "The roles a palette states"): the
/// view's, the header's, the sidebar's, the card's, the dialog's, the
/// popover's and the tooltip's surfaces with its one text colour, the
/// window's divider, the link, the caret, the four status colours and their
/// origin (`Custom`).
fn roles(c: &SchemeColors, palette: Option<&Palette>) -> ColorRoles {
    let origin = if palette.is_some() {
        RoleOrigin::Custom
    } else {
        RoleOrigin::SystemColor
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
    let selection = SelectionColors::new(
        c.selection_background,
        c.selection_foreground,
        Some(c.selection_background),
        Some(c.selection_foreground),
    );
    ColorRoles {
        window: SurfaceColors::new(
            Some(c.background),
            Some(c.label),
            Some(c.muted),
            Some(c.separator),
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
        tooltip: palette.map_or(
            SurfaceColors::new(Some(c.tooltip_background), Some(c.white), None, None, None),
            |p| SurfaceColors::new(Some(p.tooltip_bg), Some(p.foreground), None, None, None),
        ),
        osd: unstated,
        selection,
        text_selection: SelectionColors::new(
            c.selection_background,
            c.selection_foreground,
            None,
            None,
        ),
        find_highlight: None,
        scrim: None,
        accent: AccentColors {
            fill: c.accent,
            foreground: c.on_accent,
            standalone: None,
        },
        focus_ring: c.system_blue,
        caret: palette.map_or(c.label, |p| p.caret),
        link: palette.map_or(c.link, |p| p.link),
        // 0.7.2: `link.visited_text_color` is `defaults.link_color` where unstated.
        link_visited: None,
        disabled_text: c.tertiary_label,
        shadow: c.shadow,
        disabled_opacity: DISABLED_OPACITY,
        error: status(palette.map_or(c.danger, |p| p.error), c.white),
        warning: status(
            palette.map_or(c.warning, |p| p.warning),
            c.warning_foreground,
        ),
        success: status(palette.map_or(c.system_green, |p| p.success), c.white),
        info: status(palette.map_or(c.system_blue, |p| p.info), c.white),
    }
}

/// The palette `p` laid over `c` (spec §6, resolution step 2; §7): each slot
/// whose colour is the colour of the slot's role replaces that colour, and
/// so every widget colour the preset takes from it, as the module
/// documentation lists; the accent is laid as given ([`lay_accent_over`]),
/// with the palette's text on it and its selection over the accent's,
/// unless an explicit accent (`explicit_accent`) replaces the four. The
/// slots that state only their roles are read by [`roles`].
fn lay_palette_over(c: &mut SchemeColors, p: &Palette, explicit_accent: bool) {
    c.background = p.window_bg;
    c.surface = p.view_bg;
    c.sidebar = p.sidebar_bg;
    c.tooltip_background = p.tooltip_bg;
    c.label = p.foreground;
    c.muted = p.foreground_muted;
    c.separator = p.border;
    if explicit_accent {
        return;
    }
    lay_accent_over(c, p.accent);
    c.on_accent = p.accent_fg;
    c.selection_background = p.selection_bg;
    c.selection_foreground = p.selection_fg;
}

/// Lays the accent `accent` over `c` as given (spec §6, step 2): the
/// accent's fill and the selection's background, which the text selection
/// takes. The text on the accent and on the selection keeps the value of the
/// layers below.
fn lay_accent_over(c: &mut SchemeColors, accent: Color) {
    c.accent = accent;
    c.selection_background = accent;
}

/// How a font size becomes logical pixels: the font DPI and the Dynamic Type
/// category of the text scale (spec §6, resolution step 2).
#[derive(Clone, Copy)]
struct FontUnits {
    /// The font DPI.
    dpi: f32,
    /// The Dynamic Type category nearest the text scale
    /// ([`nearest_category`]).
    category: ContentSizeCategory,
}

/// The Dynamic Type category nearest the text scale `text_scale`: the one
/// whose Body size over the default category's Body size is nearest it, the
/// smaller of two equally near; `Large`, the default, at 1.
fn nearest_category(text_scale: f32) -> ContentSizeCategory {
    let body = |category| dynamic_type(TextRole::Body, category, FONT_DPI);
    let Some(default) = body(DEFAULT_CATEGORY) else {
        return DEFAULT_CATEGORY;
    };
    let distance = |category| {
        body(category).map_or(f64::INFINITY, |at| {
            (f64::from(at.font_size) / f64::from(default.font_size) - f64::from(text_scale)).abs()
        })
    };
    CATEGORIES
        .into_iter()
        .min_by(|a, b| distance(*a).total_cmp(&distance(*b)))
        .unwrap_or(DEFAULT_CATEGORY)
}

/// `px`, a length of the text style `role` at the default category, at the
/// category of `units`: times the category's row over the default's, its
/// size where `leading` is false, else its leading (spec §6, resolution step
/// 2; rc1's `DynamicType` scaling, `src/resolve/evaluate.rs`); unchanged at
/// the default category. The table has a row for every [`TextRole`] at the
/// constant [`FONT_DPI`].
fn dynamic_px(px: f32, role: TextRole, leading: bool, units: FontUnits) -> f32 {
    if units.category == DEFAULT_CATEGORY {
        return px;
    }
    let rows = dynamic_type(role, units.category, FONT_DPI).zip(dynamic_type(
        role,
        DEFAULT_CATEGORY,
        FONT_DPI,
    ));
    rows.map_or(px, |(at, default)| {
        let (at, default) = if leading {
            (at.line_height, default.line_height)
        } else {
            (at.font_size, default.font_size)
        };
        (f64::from(px) * f64::from(at) / f64::from(default)) as f32
    })
}

/// A font size of the text style `role` in logical pixels: converted at the
/// font DPI, then scaled by the category ([`dynamic_px`]), not rounded
/// (0.7.2 rounds only where `font_whole_pixels` is stated,
/// `src/resolve/validate.rs`).
fn font_px(size: FontSize, role: TextRole, units: FontUnits) -> f32 {
    dynamic_px(size.to_logical_px(units.dpi), role, false, units)
}

/// A font as 0.7.2 resolves it: its family, size, weight and normal style
/// stated (`src/resolve/validate_helpers.rs`, `require_font`), in `color`.
fn font(family: &str, size_pt: f32, weight: u16, color: Color, units: FontUnits) -> TextStyle {
    let size = FontSize::Pt(size_pt);
    TextStyle {
        family: Arc::from(family),
        size_px: font_px(size, TextRole::Body, units),
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

/// A text-scale entry of the text style `role` as 0.7.2 resolves it: its
/// size, weight and line height stated, nothing else (0.7.2's
/// `TextScaleEntry`); the TOML states each entry's line height in points.
fn text_scale_entry(
    role: TextRole,
    size_pt: f32,
    weight: u16,
    line_height_pt: f32,
    units: FontUnits,
) -> TextStyle {
    let size = FontSize::Pt(size_pt);
    TextStyle {
        size_px: font_px(size, role, units),
        defined_size: Some(size),
        weight,
        line_height: dynamic_px(
            FontSize::Pt(line_height_pt).to_logical_px(units.dpi),
            role,
            true,
            units,
        ),
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
/// `validate`). Sizes convert at [`FONT_DPI`] and scale by the Dynamic Type
/// category (`units`); each keeps its unscaled size as its `defined_size`.
fn typography(colors: &ColorRoles, units: FontUnits) -> Typography {
    let text = colors.window.foreground.unwrap_or_default();
    let mut body = font(FONT_FAMILY, FONT_SIZE_PT, FONT_WEIGHT, text, units);
    body.line_height = font_px(
        FontSize::Pt(LINE_HEIGHT * FONT_SIZE_PT),
        TextRole::Body,
        units,
    );
    body.stated |= TextStated::LINE_HEIGHT;
    // The monospace font's line height, which no source states: the multiplier times its size
    // (spec §4, `Typography`: `body` and `monospace` follow a slot's rule).
    let mut monospace = font(MONO_FAMILY, MONO_SIZE_PT, MONO_WEIGHT, text, units);
    monospace.line_height = font_px(
        FontSize::Pt(LINE_HEIGHT * MONO_SIZE_PT),
        TextRole::Body,
        units,
    );
    monospace.stated |= TextStated::LINE_HEIGHT;
    Typography {
        display: Some(text_scale_entry(
            TextRole::LargeTitle,
            DISPLAY_SIZE_PT,
            DISPLAY_WEIGHT,
            DISPLAY_LINE_HEIGHT_PT,
            units,
        )),
        title1: Some(text_scale_entry(
            TextRole::Title1,
            TITLE1_SIZE_PT,
            TITLE1_WEIGHT,
            TITLE1_LINE_HEIGHT_PT,
            units,
        )),
        title2: None,
        title3: None,
        title4: None,
        headline: Some(text_scale_entry(
            TextRole::Headline,
            HEADLINE_SIZE_PT,
            HEADLINE_WEIGHT,
            HEADLINE_LINE_HEIGHT_PT,
            units,
        )),
        body,
        callout: None,
        subheadline: None,
        caption: Some(text_scale_entry(
            TextRole::Caption,
            CAPTION_SIZE_PT,
            CAPTION_WEIGHT,
            CAPTION_LINE_HEIGHT_PT,
            units,
        )),
        monospace,
        line_height_multiplier: LINE_HEIGHT,
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
/// where unstated, `resolve_border_inheritance`), the fill under the whole of
/// it.
fn outline(layout: &LayoutMetrics, color: Color) -> PartProp {
    PartProp::Stroke(Stroke::uniform(layout.border_width, color))
}

/// A part's outline in the default border colour (0.7.2: a widget's
/// `border.color` is `defaults.border.color` where unstated,
/// `resolve_border_inheritance`).
fn border(roles: &ColorRoles, layout: &LayoutMetrics) -> Option<PartProp> {
    roles.window.border.map(|color| outline(layout, color))
}

/// A part's corners at `radius`.
fn corners(radius: f32) -> PartProp {
    PartProp::Corners(Corners::uniform(radius))
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

/// The push button, WebKit's iOS button: `opaqueSecondaryFill` in WebKit's
/// border, 1px of `systemBackground`, in every state, at the default
/// radius, of least sizes; hovered and pressed its own fills; checked, and
/// checked and pressed, UIKit's selected toggle's fills, with its labels; the
/// primary variant in the accent (0.7.2: `primary_background` is
/// `defaults.accent_color` where unstated); disabled, its disabled fill. Its
/// label `systemBlue`, which a hovered and a pressed button keep, primary the accent's
/// text (0.7.2: `primary_text_color` is `defaults.accent_text_color` where
/// unstated), disabled the disabled text. 0.7.2 states no checked colours
/// and no padding. The checked colours are the measured ones under UIKit's
/// default tint, the one tint the measurement drew (`docs/facts/ios/button.toml`
/// on `v0.8.0-rc1` at `78ee0efd`): they do not follow the accent, and under
/// another accent a checked button keeps the default tint's blue.
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
    // `button.border`: (WebKit) `border: 1px solid -webkit-control-background` (html.css:889),
    // systemBackground (RenderThemeIOS.mm:1027), which no :hover rule and no :active one
    // (html.css:901-906) changes, nor adjustButtonLikeControlStyle, which changes an active
    // button's text and background alone (RenderThemeIOS.mm:862-870): stated for `HOVER` and
    // `PRESSED` too (spec §2, `from_state`).
    let rest_border = PartProp::Stroke(Stroke::uniform(BUTTON_BORDER_WIDTH, c.background));
    let body = RuleBuilder::with_exclusive(PartStyle::default(), PRESSED_HOVER)
        .base([fill(c.secondary_fill), rest_border.clone()])
        .base([
            corners(layout.corner_radius),
            PartProp::MinSize([Some(BUTTON_MIN_WIDTH), Some(MIN_HIT_TARGET)]),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.control_hover), rest_border.clone()],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.button_pressed), rest_border],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.button_checked)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED | WidgetState::PRESSED),
            [fill(c.button_checked_pressed)],
        )
        .rule(primary(), [fill(roles.accent.fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_fill)],
        )
        .build();
    // `button.active_text_color`: (WebKit) systemBlue, a pressed iOS button keeps its blue text
    // (html.css:901-906), stated for `PRESSED` (spec §2, `from_state`).
    // A hovered button's label keeps it too: html.css has no :hover rule for a button and
    // RenderThemeIOS's adjustment reads only `element.active()` (RenderThemeIOS.mm:862).
    let label = RuleBuilder::new(label(&typography.body, Some(c.system_blue)))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.system_blue)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(c.system_blue)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [TextProp::Color(c.button_checked_foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED | WidgetState::PRESSED),
            [TextProp::Color(c.button_checked_pressed_foreground)],
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
/// `checkbox.*` serves both, `CHECKBOX_INDICATOR_SIZE` across (0.7.2:
/// `radio_indicator_width` is `indicator_width` where unstated): its
/// unchecked fill in WebKit's outline, no shadow; under the pointer its
/// hover fill; checked, the accent (0.7.2: `checkbox.checked_background` is
/// `defaults.accent_color` where unstated) with no outline; disabled, its
/// disabled fill, checked or not.
fn indicator(c: &SchemeColors, roles: &ColorRoles) -> RuleBuilder<PartStyle, PartProp> {
    let size = [Some(CHECKBOX_INDICATOR_SIZE); 2];
    // (WebKit) checkboxRadioBorderColor: secondaryLabel (ios.rs's `muted`), read for Enabled and
    // Pressed only, so hovered as at rest; pressed, at pressedStateOpacity
    // (RenderThemeIOS.mm:1357-1368 at WebKit 31adf9d). paintCheckbox strokes only the empty box
    // and fills a checked one with no stroke (:1483-1491, :1502), where 0.7.2 outlined both in
    // its uncited default line and separator.
    let rest_edge = PartProp::Stroke(Stroke::uniform(CHECKBOX_OUTLINE_WIDTH, c.muted));
    let pressed_edge = PartProp::Stroke(Stroke::uniform(
        CHECKBOX_OUTLINE_WIDTH,
        c.muted.mul_alpha(PRESSED_STATE_OPACITY),
    ));
    let no_edge = PartProp::Stroke(Stroke::None);
    RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.elevated),
            rest_edge.clone(),
            // `checkbox.border.shadow_enabled`: docs/inheritance-rules.toml ([wrong_safety_nets]):
            // the defaults' shadow is a wrong safety net for this control, and the preset must say
            // so
            PartProp::Shadows(None),
            PartProp::MinSize(size),
            PartProp::MaxSize(size),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.control_hover), rest_edge],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(roles.accent.fill), no_edge.clone()],
        )
        // (WebKit) checkboxRadioBackgroundColor: pressed, the enabled background at
        // PRESSED_STATE_OPACITY, -webkit-control-background (systemBackground) unchecked, the tint
        // checked (RenderThemeIOS.mm:1390-1393; the colour map :1027), its gradient dropped
        // (:1399-1401).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [
                fill(c.background.mul_alpha(PRESSED_STATE_OPACITY)),
                pressed_edge,
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::CHECKED),
            [
                fill(roles.accent.fill.mul_alpha(PRESSED_STATE_OPACITY)),
                no_edge,
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_fill)],
        )
}

/// The label of a check box or a radio button: the UI font in its colour;
/// disabled, its own disabled colour.
fn check_label(c: &SchemeColors, typography: &Typography) -> RuleSet<TextStyle, TextProp> {
    RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.disabled_label)],
        )
        .build()
}

/// The check box: [`indicator`] at the default radius (0.7.2's
/// `resolve_border_inheritance`), a check mark in the accent's text colour
/// (0.7.2: `checkbox.indicator_color` is `defaults.accent_text_color` where
/// unstated), [`check_label`].
fn checkbox(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> CheckboxWidget {
    // (WebKit) checkboxRadioIndicatorColor reads Enabled and Pressed alone: hovered, the mark as at
    // rest; pressed, at PRESSED_STATE_OPACITY (RenderThemeIOS.mm:1410-1420).
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Color(roles.accent.foreground)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [IconProp::Color(roles.accent.foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [IconProp::Color(
                roles.accent.foreground.mul_alpha(PRESSED_STATE_OPACITY),
            )],
        )
        .build();
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator(c, roles)
            .base([corners(layout.corner_radius)])
            .build(),
        checkmark,
        check_label(c, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The radio button: [`indicator`] (0.7.2 states no corners for it), a dot
/// in the accent's text colour (`checkbox.indicator_color`; 0.7.2 states no
/// dot size), [`check_label`].
fn radio(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> RadioWidget {
    let dot = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.accent.foreground)])
        .build();
    RadioWidget::new(
        WidgetOrigin::Native,
        indicator(c, roles).build(),
        dot,
        check_label(c, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The list or table view: the window's background (0.7.2's
/// `list.background_color` where unstated) in the default border at the
/// default radius (`resolve_border_inheritance`; the default shadow, which
/// is on); the alternate row's fill; the highlight's hover fill and the
/// selection's fill; the window's text, which a hovered row keeps
/// (`list.hover_text_color`), the selection's text on a selected row, its
/// disabled colour on a disabled row; headers in the view's background
/// (0.7.2: `list.header_background` is `defaults.surface_color` where
/// unstated); the grid in the default border colour (`list.grid_color`
/// likewise). 0.7.2 states no row height, padding or header height.
fn list(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ListWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, layout))
        .base([corners(layout.corner_radius)])
        .build();
    let alternate = RuleSelector::on(WidgetState::NONE).matcher(ListMatcher {
        variant: None,
        alternate: Some(true),
    });
    let row = RuleBuilder::new(PartStyle::default())
        .rule(alternate, [fill(c.surface)])
        .build();
    let highlight = RuleBuilder::new(PartStyle::default())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.row_hover)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    // `list.hover_text_color`: 0.7.2 ios-13.toml (uncited), the rest colour, stated for `HOVER`
    // (spec §2, `from_state`), before the rules of the states that replace it.
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
            [TextProp::Color(c.disabled_label)],
        )
        .build();
    let header = RuleBuilder::new(PartStyle::default())
        .base(roles.view.background.map(fill))
        .build();
    let header_label = RuleBuilder::new(label(&typography.body, None)).build();
    ListWidget::new(
        WidgetOrigin::Native,
        container,
        row,
        highlight,
        item_label,
        header,
        header_label,
        RuleBuilder::new(PartStyle::default()).build(),
        c.separator,
    )
}

/// The scroll bar: a groove in the window's background (0.7.2:
/// `scrollbar.track_color` is `defaults.background_color` where unstated)
/// and a handle in the muted foreground (0.7.2: `scrollbar.thumb_color` is
/// `defaults.muted_color` where unstated), each of an exact thickness across
/// the bar; the hovered and the pressed handle's own colours.
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
    .rule(
        RuleSelector::on(WidgetState::HOVER),
        [fill(c.scrollbar_hover)],
    )
    .rule(RuleSelector::on(WidgetState::PRESSED), [fill(c.gray)])
    .build();
    ScrollbarWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        SCROLLBAR_MIN_THUMB_LENGTH,
        SCROLLBAR_OVERLAY,
    )
}

/// The tooltip, synthesized: the tooltip's colours in the default border at
/// the default radius (`resolve_border_inheritance`; the default shadow), of
/// a greatest width; its text in the UI font.
fn tooltip(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> TooltipWidget {
    let mut panel = PartStyle::default();
    for prop in [fill(
        roles.tooltip.background.unwrap_or(c.tooltip_background),
    )]
    .into_iter()
    .chain(border(roles, layout))
    .chain([
        corners(layout.corner_radius),
        PartProp::MaxSize([Some(TOOLTIP_MAX_WIDTH), None]),
    ]) {
        panel.apply(&prop);
    }
    TooltipWidget::new(
        WidgetOrigin::Synthesized,
        panel,
        label(&typography.body, roles.tooltip.foreground),
    )
}

/// The window's chrome: the frame in the window's background (0.7.2:
/// `window.background_color` is `defaults.background_color` where
/// unstated) in the default border, at the large radius; the title bar, its
/// inactive one the same, in the view's background (0.7.2:
/// `window.title_bar_background` is `defaults.surface_color` where
/// unstated, and the inactive one is stated equal); the title in the UI
/// font and the window's text.
fn window(roles: &ColorRoles, typography: &Typography, layout: &LayoutMetrics) -> WindowChrome {
    let mut frame = PartStyle::default();
    for prop in roles
        .window
        .background
        .map(fill)
        .into_iter()
        .chain(border(roles, layout))
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

/// The text field (`field`) or the text area: the window's background in
/// the default border at the default radius, no shadow (the text area
/// takes the field's border and shadow: 0.7.2's `text_area.border.*` is
/// `input.border.*` where unstated), the text field of the HIG's least
/// height; the hovered and the focused outline the
/// accent's, disabled its own fill. The text in the window's text,
/// disabled its own colour; the placeholder `placeholderText`.
fn input(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    text_field: bool,
) -> InputWidget {
    let field = RuleBuilder::new(PartStyle::default())
        .base([fill(c.background)])
        .base(border(roles, layout))
        .base([
            corners(layout.corner_radius),
            // `input.border.shadow_enabled`: docs/inheritance-rules.toml ([wrong_safety_nets]): the
            // defaults' shadow is a wrong safety net for this control, and the preset must say so
            PartProp::Shadows(None),
        ])
        .base(text_field.then_some(PartProp::MinSize([None, Some(MIN_HIT_TARGET)])))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [outline(layout, roles.accent.fill)],
        )
        // (WebKit) a pressed field keeps its rest border: html.css has no :active rule for it
        // (its border html.css:466) and paintTextFieldDecorations reads no state
        // (RenderThemeIOS.mm:271-292); stated for `PRESSED` (spec §2, `from_state`).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            border(roles, layout),
        )
        .rule(
            RuleSelector::on(WidgetState::FOCUSED),
            [outline(layout, roles.accent.fill)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.input_disabled)],
        )
        .build();
    // (WebKit) a hovered field's text as at rest: html.css has no :hover rule for an input and
    // RenderThemeIOS reads no hover state.
    let text = RuleBuilder::new(label(&typography.body, Some(c.label)))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.label)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.disabled_label)],
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

/// The link: transparent, under the pointer its hover layer; its label in
/// the link's colour (0.7.2: `link.font.color` is `defaults.link_color`
/// where unstated), which a hovered link keeps, not underlined; pressed and
/// disabled its own colours.
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
        // `link.hover_text_color`: 0.7.2 ios-13.toml (uncited), the rest colour, stated for
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
            [TextProp::Color(c.disabled_label)],
        )
        .build();
    LinkWidget::new(WidgetOrigin::Native, container, label)
}

/// The slider: the groove `opaqueFill`, of an exact thickness across it,
/// disabled its own colour; the filled part in the accent (0.7.2:
/// `slider.fill_color` is `defaults.accent_color` where unstated), disabled
/// its own colour; the handle in the view's background (0.7.2:
/// `slider.thumb_color` is `defaults.surface_color` where unstated), of an
/// exact size, hovered and disabled its own colours.
fn slider(c: &SchemeColors, roles: &ColorRoles) -> SliderWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base([fill(c.track)]),
        SLIDER_TRACK_HEIGHT,
    )
    .rule(
        RuleSelector::on(WidgetState::DISABLED),
        [fill(c.slider_disabled_track)],
    )
    .build();
    let active_track = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.accent.fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.slider_disabled_fill)],
        )
        .build();
    let thumb_size = [Some(SLIDER_THUMB_DIAMETER); 2];
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.surface),
            PartProp::MinSize(thumb_size),
            PartProp::MaxSize(thumb_size),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.slider_thumb_hover)],
        )
        .rule(RuleSelector::on(WidgetState::DISABLED), [fill(c.row_hover)])
        .build();
    SliderWidget::new(
        WidgetOrigin::Native,
        track,
        active_track,
        thumb,
        RuleBuilder::new(PartStyle::default()).build(),
        SLIDER_TICK_LENGTH,
        Some(roles.disabled_opacity),
    )
}

/// The progress bar: the track `opaqueFill` in the default border at the
/// default radius, no shadow, of a least width and an exact height; the
/// progress in the accent (0.7.2: `progress_bar.fill_color` is
/// `defaults.accent_color` where unstated).
fn progress_bar(c: &SchemeColors, roles: &ColorRoles, layout: &LayoutMetrics) -> ProgressBarWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base([fill(c.track)])
        .base(border(roles, layout))
        .base([
            corners(layout.corner_radius),
            // `progress_bar.border.shadow_enabled`: docs/inheritance-rules.toml
            // ([wrong_safety_nets]): the defaults' shadow is a wrong safety net for this control,
            // and the preset must say so
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

/// The separator: the default border's colour and width (0.7.2:
/// `separator.line_width` is `defaults.border.line_width` where unstated).
fn separator(roles: &ColorRoles, layout: &LayoutMetrics) -> SeparatorWidget {
    SeparatorWidget::new(
        WidgetOrigin::Native,
        roles.window.border.unwrap_or_default(),
        layout.border_width,
        None,
        None,
    )
}

/// The splitter, synthesized: the handle in the default border's colour
/// (0.7.2: `splitter.divider_color` is `defaults.border.color` where
/// unstated), under the pointer the accent, of its own width.
fn splitter(roles: &ColorRoles) -> SplitterWidget {
    let handle = RuleBuilder::new(PartStyle::default())
        .base(roles.window.border.map(fill))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(roles.accent.fill)],
        )
        .build();
    SplitterWidget::new(WidgetOrigin::Synthesized, handle, SPLITTER_DIVIDER_WIDTH)
}

/// The spinner: the accent (0.7.2: `spinner.fill_color` is
/// `defaults.accent_color` where unstated), at its sizes and stroke.
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

/// The menu: the panel in the window's background (0.7.2:
/// `menu.background_color` is `defaults.background_color` where unstated),
/// its corners unstated (no source gives iOS's menu container radius:
/// platform-facts.md (§2.6) has no iOS column, 0.7.2 states none); a hovered
/// item in the selection's colours (0.7.2: `menu.hover_*` stated equal to the
/// selection's), a disabled item's text its own colour; icons at the
/// toolbar's size (0.7.2: `menu.icon_size` is `defaults.icon_sizes.toolbar`
/// where unstated); the separator in the default border colour
/// (`menu.separator_color` likewise).
fn menu(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> MenuWidget {
    let panel = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .build();
    let item = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(roles.selection.background)],
        )
        .build();
    let item_label = RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(roles.selection.foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.disabled_label)],
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

/// The menu bar, synthesized: the window's background.
fn menu_bar(roles: &ColorRoles) -> MenuBarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .build();
    MenuBarWidget::new(
        WidgetOrigin::Synthesized,
        bar,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        None,
    )
}

/// The tab bar and its tabs: both in the window's background (0.7.2:
/// `tab.bar_background` and `tab.background_color` are
/// `defaults.background_color` where unstated), the tab of least sizes,
/// hovered its own fill, selected its own; the label in the window's text,
/// which a hovered tab keeps, selected the accent. 0.7.2 states no
/// indicator.
fn tab(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> TabWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .build();
    let tab = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base([PartProp::MinSize([
            Some(TAB_MIN_WIDTH),
            Some(TAB_MIN_HEIGHT),
        ])])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.row_hover)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(c.background)],
        )
        .build();
    // `tab.hover_text_color`: 0.7.2 ios-13.toml (uncited), the rest colour, stated for `HOVER`
    // (spec §2, `from_state`), before the selected rule.
    let label = RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(typography.body.color)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(roles.accent.fill)],
        )
        .build();
    TabWidget::new(
        WidgetOrigin::Native,
        bar,
        tab,
        label,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        None,
        None,
        None,
    )
}

/// The toolbar: the window's background (0.7.2: `toolbar.background_color`
/// is `defaults.background_color` where unstated) in the default border at
/// the default radius (the default shadow); its text the UI font; its
/// buttons' icons at the toolbar's size (0.7.2: `toolbar.icon_size` is
/// `defaults.icon_sizes.toolbar` where unstated).
fn toolbar(roles: &ColorRoles, typography: &Typography, layout: &LayoutMetrics) -> ToolbarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, layout))
        .base([corners(layout.corner_radius)])
        .build();
    let button_icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(ICON_TOOLBAR)])
        .build();
    ToolbarWidget::new(
        WidgetOrigin::Native,
        bar,
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(label(&typography.body, None)).build(),
        button_icon,
        RuleBuilder::new(PartStyle::default()).build(),
        None,
    )
}

/// The status bar, synthesized: the window's background (0.7.2:
/// `status_bar.background_color` is `defaults.background_color` where
/// unstated) in the default border; its text the UI font.
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
        .chain(border(roles, layout))
    {
        bar.apply(&prop);
    }
    StatusBarWidget::new(
        WidgetOrigin::Synthesized,
        bar,
        label(&typography.body, None),
        PartStyle::default(),
    )
}

/// The sidebar: its own background in the default border; an item hovered
/// its own fill, selected the selection's (0.7.2:
/// `sidebar.selection_background` is `defaults.selection_background` where
/// unstated); the label in the window's text, selected the selection's
/// (`sidebar.selection_text_color` likewise).
fn sidebar(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SidebarWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(c.sidebar)])
        .base(border(roles, layout))
        .build();
    let item = RuleBuilder::new(PartStyle::default())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.row_hover)])
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background)],
        )
        .build();
    let item_label = RuleBuilder::new(label(&typography.body, Some(c.label)))
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

/// The popover: its own background, or the popover's role where a palette
/// states it, in the default border at the large radius (0.7.2:
/// `popover.border.corner_radius` is `defaults.border.corner_radius_lg`
/// where unstated; the default shadow); its text the window's.
fn popover(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> PopoverWidget {
    let mut panel = PartStyle::default();
    for prop in [fill(roles.popover.background.unwrap_or(c.elevated))]
        .into_iter()
        .chain(border(roles, layout))
        .chain([corners(layout.corner_radius_lg)])
    {
        panel.apply(&prop);
    }
    PopoverWidget::new(
        WidgetOrigin::Native,
        panel,
        PartStyle::default(),
        label(&typography.body, Some(c.label)),
    )
}

/// The switch: the off track at its radius and exact size, no outline
/// (0.7.2's `switch.*track_border_*` and `thumb_border_*` are `None`, no
/// outline, where unstated); hovered its own fill; on `systemGreen`, hovered on
/// its own; disabled off and disabled on their own fills. The thumb white,
/// disabled too, of an exact size on or off (0.7.2 states no unchecked
/// size).
fn switch(c: &SchemeColors, roles: &ColorRoles) -> SwitchWidget {
    let size = [Some(SWITCH_TRACK_WIDTH), Some(SWITCH_TRACK_HEIGHT)];
    let track = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.switch_off),
            PartProp::Stroke(Stroke::None),
            corners(SWITCH_TRACK_RADIUS),
            PartProp::MinSize(size),
            PartProp::MaxSize(size),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.switch_hover_off)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.system_green)],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED | WidgetState::HOVER),
            [fill(c.switch_hover_on)],
        )
        // (WebKit) a pressed off track as at rest: switchTrackColor reads on or off, the contrast,
        // the scheme and the animation's progress, no pressed state (RenderThemeCocoa.mm:258-338);
        // the pressed overlay is macOS's alone (:467-472, :691-694).
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(WidgetState::CHECKED),
            [fill(c.switch_off)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.switch_disabled_off)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED | WidgetState::CHECKED),
            [fill(c.switch_disabled_on)],
        )
        .build();
    let thumb_size = [Some(SWITCH_THUMB_DIAMETER); 2];
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.white),
            PartProp::Stroke(Stroke::None),
            PartProp::MinSize(thumb_size),
            PartProp::MaxSize(thumb_size),
        ])
        // The on thumb, `switch.thumb_background`: (WebKit) white (RenderThemeCocoa.mm:763), and
        // `thumb_diameter` (the constant's citation); `disabled_thumb_color`: (WebKit) the thumb is
        // white, disabled too (RenderThemeCocoa.mm:763); each stated for its state (spec §2,
        // `from_state`).
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [
                fill(c.white),
                PartProp::MinSize(thumb_size),
                PartProp::MaxSize(thumb_size),
            ],
        )
        .rule(RuleSelector::on(WidgetState::DISABLED), [fill(c.white)])
        .build();
    SwitchWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        // The label's colours and font: no source (iOS's switch draws no label; the list row's
        // text is unpublished, "system default styling").
        RuleBuilder::new(TextStyle::default()).build(),
        // `label_side`, `label_gap_stretches`: "In iOS, iPadOS, watchOS, and tvOS, the label and
        // switch fill as much horizontal space as the toggle's parent offers by aligning the
        // label's leading edge and the switch's trailing edge with the containing view's
        // respective leading and trailing edges" (Apple, SwiftUI `ToggleStyle.switch`,
        // developer.apple.com, read 2026-11-23). `label_gap`: unmeasured (the iOS host).
        Some(LabelSide::Leading),
        None,
        Some(true),
        None,
        Some(roles.disabled_opacity),
    )
}

/// The dialog: the window's background, or the dialog's role where a
/// palette states it (0.7.2: `dialog.background_color` is
/// `defaults.background_color` where unstated), in the default border at
/// the large radius (the default shadow), of least and greatest sizes; its
/// title and body in the UI font; its icon, its buttons' gap and order. 0.7.2
/// states no entrance.
fn dialog(
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    motion: Motion,
) -> DialogWidget {
    let mut panel = PartStyle::default();
    for prop in roles
        .dialog
        .background
        .or(roles.window.background)
        .map(fill)
        .into_iter()
        .chain(border(roles, layout))
        .chain([
            corners(layout.corner_radius_lg),
            PartProp::MinSize([Some(DIALOG_MIN_WIDTH), Some(DIALOG_MIN_HEIGHT)]),
            PartProp::MaxSize([Some(DIALOG_MAX_WIDTH), Some(DIALOG_MAX_HEIGHT)]),
        ])
    {
        panel.apply(&prop);
    }
    let mut icon = IconStyle::default();
    icon.apply(&IconProp::Size(DIALOG_ICON_SIZE));
    DialogWidget::new(
        WidgetOrigin::Native,
        panel,
        label(&typography.body, None),
        label(&typography.body, None),
        icon,
        DIALOG_BUTTON_GAP,
        Some(DIALOG_BUTTON_ORDER),
        dialog_entrance(None, motion),
    )
}

/// The combo box, WebKit's iOS select: `opaqueSecondaryFill` in the default
/// border at the default radius, no shadow, of least sizes; hovered and
/// disabled its own fills. Its label `systemBlue`, disabled the disabled
/// text; the chevron at its size.
fn combo_box(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ComboBoxWidget {
    // (WebKit) the select's `border: 1px solid -webkit-control-background` (html.css:1101 at
    // WebKit 31adf9d), where 0.7.2 took its uncited default line; no :hover or :active rule
    // for select, and adjustButtonLikeControlStyle changes an active control's text and
    // background only (RenderThemeIOS.mm:459, :862-870), so hovered and pressed as at rest.
    let edge = PartProp::Stroke(Stroke::uniform(BUTTON_BORDER_WIDTH, c.background));
    let field = RuleBuilder::new(PartStyle::default())
        .base([fill(c.secondary_fill), edge.clone()])
        .base([
            corners(layout.corner_radius),
            // `combo_box.border.shadow_enabled`: docs/inheritance-rules.toml ([wrong_safety_nets]):
            // the defaults' shadow is a wrong safety net for this control, and the preset must say
            // so
            PartProp::Shadows(None),
            PartProp::MinSize([Some(COMBO_BOX_MIN_WIDTH), Some(MIN_HIT_TARGET)]),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.control_hover), edge.clone()],
        )
        // (WebKit) a pressed select: adjustMenuListButtonStyle takes adjustButtonLikeControlStyle
        // (RenderThemeIOS.mm:459), which multiplies an active control's background and text by
        // PRESSED_STATE_OPACITY (:862-870).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [
                fill(c.secondary_fill.mul_alpha(PRESSED_STATE_OPACITY)),
                edge,
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.disabled_fill)],
        )
        .build();
    // Hovered, the label as at rest: the select's `color: -apple-system-blue` (html.css:1099),
    // no :hover rule, adjustButtonLikeControlStyle reads only `active()` (RenderThemeIOS.mm:862).
    let label = RuleBuilder::new(label(&typography.body, Some(c.system_blue)))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.system_blue)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(
                c.system_blue.mul_alpha(PRESSED_STATE_OPACITY),
            )],
        )
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
        Some(roles.disabled_opacity),
    )
}

/// The segmented control: the track in the window's background (0.7.2:
/// `segmented_control.background_color` is `defaults.background_color`
/// where unstated) in the default border at the default radius, no shadow;
/// a segment of least height, hovered its own fill, selected the accent;
/// the label in the window's text, selected the accent's text.
fn segmented_control(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SegmentedControlWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, layout))
        .base([
            corners(layout.corner_radius),
            // `segmented_control.border.shadow_enabled`: docs/inheritance-rules.toml
            // ([wrong_safety_nets]): the defaults' shadow is a wrong safety net for this control,
            // and the preset must say so
            PartProp::Shadows(None),
        ])
        .build();
    let segment = RuleBuilder::new(PartStyle::default())
        .base([PartProp::MinSize([None, Some(SEGMENT_HEIGHT)])])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.control_hover)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.accent.fill)],
        )
        .build();
    let label = RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(roles.accent.foreground)],
        )
        .build();
    SegmentedControlWidget::new(
        WidgetOrigin::Native,
        track,
        segment,
        label,
        RuleBuilder::new(PartStyle::default()).build(),
        SEGMENT_SEPARATOR_WIDTH,
        Some(roles.disabled_opacity),
    )
}

/// The card: the view's background, or the card's role where a palette
/// states it (0.7.2: `card.background_color` is `defaults.surface_color`
/// where unstated); 0.7.2 leaves its outline and its corners unset.
fn card(c: &SchemeColors, roles: &ColorRoles) -> CardWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(roles.card.background.unwrap_or(c.surface))])
        .build();
    CardWidget::new(WidgetOrigin::Native, container)
}

/// The expander: the default border at the default radius, no shadow; the
/// header of least height, hovered its own fill; its text the UI font; the
/// arrow at its size and colour.
fn expander(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ExpanderWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(border(roles, layout))
        .base([
            corners(layout.corner_radius),
            // `expander.border.shadow_enabled`: docs/inheritance-rules.toml ([wrong_safety_nets]):
            // the defaults' shadow is a wrong safety net for this control, and the preset must say
            // so
            PartProp::Shadows(None),
        ])
        .build();
    let header = RuleBuilder::new(PartStyle::default())
        .base([PartProp::MinSize([None, Some(EXPANDER_HEADER_HEIGHT)])])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.row_hover)])
        .build();
    let arrow = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(EXPANDER_ARROW_SIZE), IconProp::Color(c.gray)])
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
