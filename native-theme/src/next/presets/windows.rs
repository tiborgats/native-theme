//! The `windows-11` preset's builder (spec §6): Windows 11's Fluent Design
//! (WinUI 3), light and dark.
//!
//! The builder runs in three steps: the preset's constants, one table per
//! scheme for the colours and plain constants for the rest; the colour roles
//! and the globals made from them; the widgets derived from the roles, the
//! globals and the colours. A widget's colour that is a role's colour, the
//! same WinUI brush, is taken from the roles, not from a second copy of the
//! constant. The system's settings ([`ResolveInputs::system`]) are laid over
//! the constants as the 0.7.2 reader writes them (spec §8, "The Windows
//! part", *Derived by the `windows-11` builder*; see [`resolve`]).
//!
//! The active palette ([`ResolveInputs::palette`]) and then an explicit
//! accent ([`ResolveInputs::accent`]) are laid over the system's settings
//! (spec §6, resolution step 2, layer 3), over the high-contrast colours too
//! where they are kept there. A palette states the roles its slots name
//! (spec §7, "The roles a palette states"), and each slot whose WinUI colour
//! has the slot's name replaces that colour and every widget colour the
//! preset takes from it ([`lay_palette_over`]): `window_bg` is the window's
//! background, so also the title bar's, the list's header and the dialog's
//! panel; `view_bg` is the view's (`CardBackgroundFillColorDefault`, 0.7.2's
//! `defaults.surface_color`); `sidebar_bg` is the sidebar's; `foreground` is
//! the window's text, in which the preset letters its widgets, and the text
//! of the eight surfaces (§7: one text colour); `foreground_muted` and
//! `border` are the window's, so every outline drawn in the default border
//! and the separator; `link` is the link, whose hovered and pressed colours,
//! shades of another colour, are then not stated (spec §6, step 2); `error`,
//! `warning`, `success` and `info` are the status fills; `accent` is the
//! accent, written as given as an explicit accent is (below), and `accent_fg`
//! the text on it; `selection_bg` and `selection_fg` are the selection's,
//! which the text selection takes. A slot with no WinUI colour of its name
//! states its role alone, and a widget that draws that role takes it:
//! `header_bg`, `divider` and `caret` state their roles; `tooltip_bg` the
//! tooltip's, so its panel (the preset's `AcrylicInAppFillColorDefault` is
//! the popover's too); `surface_bg` the card's, the dialog's and the
//! popover's, so their panels. Every other colour keeps the value of the
//! layers below: the control fills, strokes and hover layers, every disabled
//! colour, the focus ring, the status colours' text.
//!
//! An explicit accent, or else a palette's `accent`, is written as given;
//! no colour is computed from it (spec §6, step 2;
//! `.superpowers/sdd/v0.8.0-plan/M2/units/accent-decision.md`): it is the
//! accent, the accent's fill of the resolved scheme (WinUI's
//! `AccentFillColorDefault`: the primary and checked fills, the slider's,
//! the progress bar's and the spinner's fill and, outside high contrast, the
//! focused text field's outline), the informational fill and the selection,
//! and `SystemOverrides::windows_accent_ramp` is not read for them. Under an
//! explicit accent the text on the accent's fill is the resolved scheme's
//! `TextOnAccentFillColorPrimary` where its contrast ratio on the accent is
//! at least 3, else the other scheme's ([`text_on_accent`]); the link and its
//! hovered and pressed colours, WinUI's text shades, keep the value of the
//! layers below. An explicit accent over a palette replaces its `accent`,
//! `accent_fg`, `info`, `selection_bg` and `selection_fg`; the selection's
//! text is then the one of the layers below the palette.
//!
//! The colours are what WinUI 3 draws (the header of 0.7.2's
//! `windows-11.toml`): a brush with alpha as the resource gives it (ARGB
//! `#E4000000` is `0x0000_00e4` here); a hover or pressed fill that replaces
//! the rest fill (the button's hovered and pressed fills, the combo box's
//! hover, the switch's hovered off track) flattened over the window's
//! background; one that replaces a transparent rest fill (a list row, a
//! sidebar item, a segment) stated as the brush, a layer.
//!
//! Defined by the preset (spec §1, "Axis values without data"): the variants
//! `ButtonVariant::Primary` (0.7.2's `primary_*` fields) and
//! `ButtonVariant::Flat` (WinUI's subtle button), a list's alternate
//! rows (`ListKind::alternate`), the two orientations of the scroll bar and
//! of the slider's groove, no control size, and the contrast level `Normal`
//! alone. A query for anything else returns the part's base style. A
//! maximized or full-screen window has no data of its own: its frame and
//! title bar are the normal window's.
//!
//! A part's exclusive state group names only the flags that part has a rule
//! for: a state without data of its own returns what the query returns
//! without that state, so a flag the part does not style removes no other.
//! The groups: the button's body and the link's label `PRESSED, HOVER`;
//! every other part none.
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
//!   label, in the same order: the checked text, the pressed one over it, the
//!   primary one, the disabled one over all (`mapping.md`,
//!   `button.disabled_text_color`);
//! - the check box's and the radio button's indicator: the checked fill and
//!   outline over the hovered fill, as 0.7.2's egui connector draws a checked
//!   box ("`checkbox.hover_background` is the unchecked box's hover fill, so
//!   the hovered entry holds the checked fill too":
//!   `connectors/native-theme-egui/src/style/variants.rs`), and the mixed
//!   box's over the hovered and the pressed unchecked looks, as the checked
//!   one's (WinUI states the same brushes for both,
//!   `CheckBox_themeresources.xaml`); the disabled fill over all of them,
//!   checked or not (0.7.2's `checkbox.disabled_background`, which the table
//!   of homes compares at `DISABLED | CHECKED` too);
//! - the switch's track: the hovered on track (`HOVER | CHECKED`), whose
//!   colour 0.7.2 states apart (`switch.hover_checked_background`), over the
//!   hovered and the checked track; the disabled off track over the hovered
//!   one and the disabled on track (`DISABLED | CHECKED`) over every other,
//!   as WinUI swaps a disabled control to its `*Disabled` brushes
//!   (platform-facts.md (§2.1.6), `disabled_opacity`);
//! - the link's label: the pressed colour over the hovered one, the disabled
//!   colour over both (WinUI's `*Disabled` brushes, as above);
//! - the list's row highlight, the sidebar's item, the tab and the segmented
//!   control's segment: the selected fill over the hovered fill, as 0.7.2's
//!   egui connector puts the selection's colour in the selection's fill and
//!   the hover colour in the hovered widgets' fill
//!   (`connectors/native-theme-egui/src/mapping.md`) and egui fills a
//!   selected item with the selection's fill over its hovered look (egui
//!   0.37.0 `src/style.rs`, `Style::interact_selectable`). The tab's label:
//!   the selected colour over the hovered one, for the same reason;
//! - the toolbar's button: the checked fill over the hovered and the pressed
//!   fills, as 0.7.2's gpui connector draws a toolbar button that is on (a
//!   selected button takes no hover or press style:
//!   `connectors/native-theme-gpui/src/geometry.rs`,
//!   `toolbar_button_checked`), and the pressed fill over the hovered one.
//!
//! The fill under an outline (spec §2, "Fill and stroke";
//! `.superpowers/sdd/v0.8.0-plan/M2/fill-stroke-decision.md`): WinUI's
//! `DefaultButtonStyle` and `DefaultTextBoxStyle` set
//! `BackgroundSizing="InnerBorderEdge"`, the fill ending at the outline's
//! inner edge, and `AccentButtonStyle` `"OuterBorderEdge"`, the fill under
//! the whole outline (microsoft-ui-xaml 8463f45,
//! `controls/dev/CommonStyles/Button_themeresources.xaml`,
//! `TextBox_themeresources.xaml`): the push button's and the text field's
//! outlines are `StrokeAlign::Outside`, the primary button's `Inside`. No
//! source the builder reads gives another control's `BackgroundSizing`:
//! their outlines stay `Inside`.
//!
//! An inactive window (`ThemeContext::window_active` false) has the inactive
//! title bar (`window.inactive_title_bar_background`) and the inactive title
//! colour (`window.inactive_title_bar_text_color`).
//!
//! Of the context it acts on the scheme and on the direction: a layout from
//! right to left places every per-side value on the mirrored side through the
//! shared helper ([`physical_edges`]); the preset states no per-corner
//! radius. The text scale is linear (spec §6, resolution step 2): every font
//! size, converted at the font DPI, times the text scale, not rounded (0.7.2
//! rounds a size only where the preset states `font_whole_pixels`, which
//! `windows-11.toml` does not), and every line height times it too; each
//! `defined_size` stays the unscaled size. The motion preference:
//! `MotionPreference::None` sets the dialog's entrance to
//! `DialogEntranceStyle::None` and would collapse every transition, and the
//! preset states no transition and no motion token; no source the builder
//! reads states a policy of Windows' for `MotionPreference::Reduced`, which
//! therefore changes nothing. The contrast changes the theme only where the
//! system's settings make `Contrast::High` available (their high-contrast
//! colours, spec §6, resolution step 1): the high-contrast mapping of
//! [`HighContrast`]. The accessibility flags change nothing here.
//!
//! Sources: `windows-11.toml` of 0.7.2 (`src/presets/windows-11.toml`), each
//! constant with the citation the TOML gives it (`docs/platform-facts.md` by
//! section, the upstream file) or `(uncited)`; a value the TOML marks ❓ has
//! no source. The short names its citations use: an upstream file named
//! without a repository (`Button_themeresources.xaml:36`,
//! `Common_themeresources_any.xaml:125`) is microsoft-ui-xaml's at 8463f45
//! (`controls/dev/`), the version the TOML's header names (the same files in
//! WinAppSDK 2.5.1 and 1.8.12), unless the citation names microsoft-ui-xaml
//! 2.5.1 or the Windows Community Toolkit (`Segmented.xaml` at
//! v8.2.251219); "OS probe" and `probe.toml` are the OS-reported values on
//! Windows 11 25H2 (26200) and Windows Server 2025 (26100),
//! `docs/evidence/os-probe/2026-10-11-run-38129819806`; `M2b-21` is that
//! finding of `docs/archive/todo_v0.7.0_code-review-findings.md`;
//! `soft_option` marks a field 0.7.2 lets stay unset
//! (`docs/todo_v0.8.0_model-completeness-rationale.md`). 0.7.2's inheritance
//! rules (`src/resolve/inheritance.rs`, `src/resolve/validate.rs`,
//! `src/resolve/validate_helpers.rs`; `docs/inheritance-rules.toml`) where
//! 0.7.2 fills a value the TOML does not state.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{Corners, Edges, Stroke, StrokeAlign};
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::{ArrowSide, DialogButtonOrder, LabelSide, LayoutMetrics};
use crate::next::motion::{CaretSpec, DialogEntranceStyle, MotionTokens, UiTimings};
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
use crate::next::reader::{OsFont, SystemOverrides};
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
/// `windows-11.toml` states, each colour from its 8-bit sRGB channels
/// ([`srgb8`]). A key 0.7.2 states with the same colour in both schemes as
/// another key's, for the same WinUI brush, shares that key's field; each
/// key's citation stands at the field.
#[derive(Clone, Copy)]
struct SchemeColors {
    window_background: Color,
    window_foreground: Color,
    window_foreground_muted: Color,
    window_border: Color,
    disabled_text: Color,
    view_background: Color,
    accent: Color,
    accent_foreground: Color,
    accent_fill: Color,
    accent_fill_disabled: Color,
    selection_foreground: Color,
    focus_ring: Color,
    focus_ring_inner: Color,
    link: Color,
    link_hover: Color,
    link_disabled: Color,
    shadow: Color,
    error: Color,
    warning: Color,
    success: Color,
    info: Color,
    status_foreground: Color,
    control_fill: Color,
    control_fill_hover: Color,
    control_fill_pressed: Color,
    control_fill_disabled: Color,
    control_alt_fill: Color,
    control_stroke: Color,
    strong_stroke: Color,
    strong_stroke_disabled: Color,
    strong_fill: Color,
    strong_fill_disabled: Color,
    subtle_fill: Color,
    subtle_fill_pressed: Color,
    alt_fill_quarternary: Color,
    accent_text_secondary: Color,
    accent_text_disabled: Color,
    tab_pressed: Color,
    checkbox_hover: Color,
    tab_hover: Color,
    tab_selected: Color,
    list_alternate_row: Color,
    input_disabled_text: Color,
    acrylic_fill: Color,
    menu_background: Color,
    flyout_stroke: Color,
    /// The scroll bar's handle, where stated; unstated, 0.7.2's
    /// `scrollbar.thumb_color` is `defaults.muted_color`.
    scrollbar_thumb: Option<Color>,
    scrollbar_thumb_hover: Color,
    slider_thumb: Color,
    switch_hover_off: Color,
    /// The hovered on switch's fill where a system colour states it (high
    /// contrast's `COLOR_BTNFACE`); unstated, [`switch_hover_on`] derives it
    /// from the accent's fill.
    switch_hover_on: Option<Color>,
    switch_off_disabled: Color,
    sidebar_background: Color,
    inactive_title: Color,
}

/// The light scheme's constants.
const LIGHT: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md (§2.1.3): Mica's solid fallback,
    // SolidBackgroundFillColorBase #F3F3F3
    // `list.header_background`: platform-facts.md (§2.15): ≈ defaults.background_color
    // `window.title_bar_background`: platform-facts.md (§2.2): the title bar's default background is
    // Mica, the TitleBar control transparent over it; Mica's solid fallback
    // SolidBackgroundFillColorBase #F3F3F3 (background_color)
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): Mica shows that fallback when
    // the window deactivates
    // `dialog.background_color`: platform-facts.md (§2.22): ContentDialogBackground =
    // SolidBackgroundFillColorBase #F3F3F3
    window_background: srgb8(0xf3f3_f3ff),
    // `defaults.text_color`: platform-facts.md: TextFillColorPrimary #E4000000; over
    // background_color #1a1a1a
    // `button.font.color`: platform-facts.md (§2.3): ButtonForeground = TextFillColorPrimary
    // #E4000000
    // `button.hover_text_color`: ButtonForegroundPointerOver = TextFillColorPrimary #E4000000
    // (Button_themeresources.xaml:133 at microsoft-ui-xaml 8463f45)
    // `combo_box.font.color`: platform-facts.md (§2.24): ComboBoxForeground = TextFillColorPrimary
    // #E4000000
    // `input.font.color`: platform-facts.md (§2.4): TextControlForeground = TextFillColorPrimary
    // #E4000000
    // `list.hover_text_color`: ListViewItemForegroundPointerOver = TextFillColorPrimary #E4000000
    // (ListViewItem_themeresources.xaml:176 at microsoft-ui-xaml 8463f45)
    // `list.selection_text_color`: 0.7.2 windows-11.toml (uncited)
    // `menu.font.color`: platform-facts.md (§2.6): MenuFlyoutItemForeground = TextFillColorPrimary
    // #E4000000
    // `menu.hover_text_color`: MenuFlyoutItemForegroundPointerOver = TextFillColorPrimary
    // #E4000000; disabled TextFillColorDisabled (MenuFlyout_themeresources.xaml:178, :180 at
    // microsoft-ui-xaml 8463f45)
    // `popover.font.color`: platform-facts.md (§2.16): ← defaults.font.color
    // `segmented_control.active_text_color`: 0.7.2 windows-11.toml (uncited)
    // `sidebar.font.color`: NavigationViewItemForeground = TextFillColorPrimary #E4000000
    // (NavigationView_themeresources.xaml:87 at microsoft-ui-xaml 8463f45)
    // `sidebar.selection_text_color`: 0.7.2 windows-11.toml (uncited)
    // `tab.active_text_color`: TabViewItemHeaderForegroundSelected = TextFillColorPrimary
    // #E4000000 (TabView_themeresources.xaml:14 at microsoft-ui-xaml 8463f45)
    // `tooltip.font.color`: platform-facts.md (§2.7): ToolTipForegroundBrush = TextFillColorPrimary
    // #E4000000
    // `window.title_bar_font.color`: platform-facts.md (§2.2): TitleBarForegroundBrush =
    // TextFillColorPrimary #E4000000
    // `expander.arrow_color`: ExpanderChevronForeground = TextFillColorPrimary #E4000000
    // (Expander_themeresources.xaml:42 at microsoft-ui-xaml 8463f45)
    window_foreground: srgb8(0x0000_00e4),
    // `defaults.muted_color`: platform-facts.md: TextFillColorSecondary #9E000000; over
    // background_color #5c5c5c
    // `button.active_text_color`: ButtonForegroundPressed = TextFillColorSecondary #9E000000
    // (Button_themeresources.xaml:134)
    // `input.placeholder_color`: platform-facts.md (§2.4): TextControlPlaceholderForeground =
    // TextFillColorSecondary #9E000000
    // `tab.font.color`: platform-facts.md (§2.11): an unselected tab's label,
    // TabViewItemHeaderForeground = TextFillColorSecondary #9E000000, which the header
    // ContentPresenter takes as a ThemeResource (TabView.xaml:575; TabView_themeresources.xaml:12)
    // `tab.hover_text_color`: TabViewItemHeaderForegroundPointerOver = TextFillColorSecondary
    // #9E000000 (TabView_themeresources.xaml:15)
    // `switch.unchecked_thumb_background`: platform-facts.md (§2.21): ToggleSwitchKnobFillOff =
    // TextFillColorSecondary #9E000000
    window_foreground_muted: srgb8(0x0000_009e),
    // `defaults.border.color`: platform-facts.md (§2.1.3, §2.1.6): CardStrokeColorDefault
    // #0F000000, its alpha 15/255 the opacity below; over background_color #e5e5e5
    // `card.border.color`: platform-facts.md (§2.26): CardStrokeColorDefault #0F000000
    // (Common_themeresources_any.xaml:250 at microsoft-ui-xaml 8463f45)
    // `tab.border.color`: platform-facts.md (§2.11): the selected tab's CardStrokeColorDefault
    // #0F000000 (Common_themeresources_any.xaml:250 at microsoft-ui-xaml 8463f45), top and sides
    // `input.hover_border_color`: TextControlBorderBrushPointerOver is the rest brush,
    // TextControlElevationBorderBrush (TextBox_themeresources.xaml:135, :134 at microsoft-ui-xaml
    // 8463f45), so the input's rest border, defaults.border.color
    window_border: srgb8(0x0000_000f),
    // `defaults.disabled_text_color`: platform-facts.md: TextFillColorDisabled #5C000000; over
    // background_color #9b9b9b
    // `button.disabled_text_color`: ButtonForegroundDisabled = TextFillColorDisabled #5C000000
    // (Button_themeresources.xaml:135)
    // `checkbox.disabled_text_color`: Interactive state colors (platform-facts.md section 2.5
    // Windows) CheckBoxForegroundUncheckedDisabled = TextFillColorDisabled #5C000000
    // (CheckBox_themeresources.xaml:184 at microsoft-ui-xaml 8463f45)
    // `combo_box.disabled_text_color`: Interactive state colors (platform-facts.md section 2.24
    // Windows) ComboBoxForegroundDisabled = TextFillColorDisabled #5C000000
    // (ComboBox_themeresources.xaml:253 at microsoft-ui-xaml 8463f45)
    // `list.disabled_text_color`: defaults.disabled_text_color: ListView has no disabled
    // foreground; a disabled row fades by ListViewItemDisabledThemeOpacity 0.3
    // (platform-facts.md), which a colour cannot carry
    // `menu.disabled_text_color`: 0.7.2 windows-11.toml (uncited)
    disabled_text: srgb8(0x0000_005c),
    // `defaults.surface_color`: platform-facts.md: CardBackgroundFillColorDefault #B3FFFFFF; over
    // background_color #fbfbfb
    view_background: srgb8(0xffff_ffb3),
    // `defaults.accent_color`: 0.7.2 windows-11.toml (uncited)
    accent: srgb8(0x0078_d4ff),
    // `defaults.accent_text_color`, `button.checked_text_color`: 0.7.2 windows-11.toml (uncited)
    // `switch.thumb_background`: platform-facts.md (§2.21): ToggleSwitchKnobFillOn =
    // TextOnAccentFillColorPrimary #FFFFFF
    accent_foreground: srgb8(0xffff_ffff),
    // `button.primary_background`: platform-facts.md (§2.3): AccentButtonBackground =
    // AccentFillColorDefault = SystemAccentColorDark1 (Button_themeresources.xaml:103,
    // Common_themeresources_any.xaml:329 at microsoft-ui-xaml 8463f45), UISettings AccentDark1
    // #0067c0 for the accent above (OS probe, Windows 11 25H2, probe.toml); primary_text_color
    // stays ← accent_text_color
    // `button.checked_background`: platform-facts.md (§2.3): ToggleButtonBackgroundChecked =
    // AccentFillColorDefault = SystemAccentColorDark1, UISettings AccentDark1 #0067c0 for the
    // accent above (OS probe, Windows 11 25H2); ToggleButtonForegroundChecked =
    // TextOnAccentFillColorPrimary #FFFFFF
    // `checkbox.checked_background`: platform-facts.md (§2.5): CheckBoxCheckBackgroundFillChecked =
    // AccentFillColorDefault = SystemAccentColorDark1 (CheckBox_themeresources.xaml:233,
    // Common_themeresources_any.xaml:329 at microsoft-ui-xaml 8463f45), UISettings AccentDark1
    // #0067c0 for the accent above (OS probe, Windows 11 25H2, probe.toml); the radio's
    // RadioButtonOuterEllipseCheckedFill is the same brush (RadioButton_themeresources.xaml:146)
    // `progress_bar.fill_color`: platform-facts.md (§2.10): ProgressBarForeground =
    // AccentFillColorDefault = SystemAccentColorDark1 (ProgressBar_themeresources.xaml:22,
    // Common_themeresources_any.xaml:329 at microsoft-ui-xaml 8463f45), UISettings AccentDark1
    // #0067c0 for the accent above (OS probe, Windows 11 25H2, probe.toml)
    // `slider.fill_color`: platform-facts.md (§2.9): SliderTrackValueFill = AccentFillColorDefault =
    // SystemAccentColorDark1 (Slider_themeresources.xaml:128, Common_themeresources_any.xaml:329 at
    // microsoft-ui-xaml 8463f45), UISettings AccentDark1 #0067c0 for the accent above (OS probe,
    // Windows 11 25H2, probe.toml)
    // `spinner.fill_color`: platform-facts.md (§2.23): ProgressRingForegroundThemeBrush =
    // AccentFillColorDefault, UISettings AccentDark1 #0067c0 for the accent above (OS probe, Windows
    // 11 25H2, probe.toml)
    // `switch.checked_background`: platform-facts.md (§2.21): ToggleSwitchFillOn =
    // AccentFillColorDefault = SystemAccentColorDark1 (ToggleSwitch_themeresources.xaml:143,
    // Common_themeresources_any.xaml:329 at microsoft-ui-xaml 8463f45), UISettings AccentDark1
    // #0067c0 for the accent above (OS probe, Windows 11 25H2, probe.toml)
    // `toolbar.button_checked_background`: platform-facts.md (§2.13):
    // AppBarToggleButtonBackgroundChecked = AccentFillColorDefault = SystemAccentColorDark1,
    // #0067c0 for the accent above (AppBarToggleButton_themeresources.xaml:9)
    // `input.focus_border_color`: TextControlBorderBrushFocused =
    // TextControlElevationBorderFocusedBrush, a 2px bottom line in SystemAccentColorDark1 over
    // ControlStrokeColorDefault (TextBox_themeresources.xaml:136, :164-173 at microsoft-ui-xaml
    // 8463f45), UISettings AccentDark1 for the accent above (OS probe, Windows 11 25H2)
    // `link.active_text_color`: 0.7.2 windows-11.toml (uncited)
    accent_fill: srgb8(0x0067_c0ff),
    // `slider.disabled_fill_color`: SliderTrackValueFillDisabled = AccentFillColorDisabled
    // #37000000 (Slider_themeresources.xaml:131; Common_themeresources_any.xaml:242 at
    // microsoft-ui-xaml 8463f45)
    // `switch.disabled_checked_background`: ToggleSwitchFillOnDisabled = AccentFillColorDisabled
    // #37000000 (ToggleSwitch_themeresources.xaml:146; Common_themeresources_any.xaml:242 at
    // microsoft-ui-xaml 8463f45)
    // a disabled checked or mixed box's fill and a disabled checked radio's circle, fill and
    // stroke: CheckBoxCheckBackgroundFillCheckedDisabled, …FillIndeterminateDisabled,
    // RadioButtonOuterEllipseCheckedFillDisabled, …CheckedStrokeDisabled =
    // AccentFillColorDisabledBrush (CheckBox_themeresources.xaml:236, :240;
    // RadioButton_themeresources.xaml:149, :145 at microsoft-ui-xaml 8463f45)
    accent_fill_disabled: srgb8(0x0000_0037),
    // `defaults.selection_text_color`: selection_background ← accent_color: platform-facts.md
    // (§2.1.3), a text box's TextControlSelectionHighlightColor =
    // AccentFillColorSelectedTextBackground = SystemAccentColor platform-facts.md:
    // COLOR_HIGHLIGHTTEXT, #ffffff on Windows 11 25H2 (OS probe); WinUI states no selected-text
    // foreground
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): SystemControlFocusVisualPrimaryBrush
    // = FocusStrokeColorOuter #E4000000
    focus_ring: srgb8(0x0000_00e4),
    // platform-facts.md (§2.1.5): focus_ring_color: the 1px inner ring FocusStrokeColorInner
    // #B3FFFFFF
    focus_ring_inner: srgb8(0xffff_ffb3),
    // `defaults.link_color`: platform-facts.md: HyperlinkForeground = SystemAccentColorDark2,
    // UISettings AccentDark2 for the accent above (OS probe, Windows 11 25H2)
    link: srgb8(0x003e_92ff),
    // `link.hover_text_color`: Interactive state colors (platform-facts.md section 2.28 Windows)
    // HyperlinkForegroundPointerOver = SystemAccentColorDark3, HyperlinkForegroundPressed =
    // SystemAccentColorDark1 (Hyperlink_themeresources.xaml:16-17,
    // Common_themeresources_any.xaml:298-299 at microsoft-ui-xaml 8463f45), UISettings AccentDark3
    // / AccentDark1 (OS probe, Windows 11 25H2)
    link_hover: srgb8(0x001a_68ff),
    // `link.disabled_text_color`: HyperlinkButtonForegroundDisabled = AccentTextFillColorDisabled
    // #5C000000 (HyperlinkButton_themeresources.xaml:23)
    link_disabled: srgb8(0x0000_005c),
    // `defaults.shadow_color`: no source ❓: platform-facts.md gives Windows the ambient layer
    // alone, 14% (#00000024) on Fluent 2's low ramp (command dropdowns, tooltips) and 20%
    // (#00000033) on the high one (dialogs); the connectors draw popup and window shadows in this
    // one colour, so no layer is the source
    shadow: srgb8(0x0000_0018),
    // `defaults.danger_color`: 0.7.2 windows-11.toml (uncited)
    error: srgb8(0xc42b_1cff),
    // `defaults.warning_color`: 0.7.2 windows-11.toml (uncited)
    warning: srgb8(0x9d5d_00ff),
    // `defaults.success_color`: 0.7.2 windows-11.toml (uncited)
    success: srgb8(0x0f7b_0fff),
    // `defaults.info_color`: 0.7.2 windows-11.toml (uncited)
    info: srgb8(0x0078_d4ff),
    // `defaults.danger_text_color`: platform-facts.md (§2.1.4): InfoBar *SeverityIconForeground =
    // TextFillColorInverse #FFFFFF
    // `defaults.warning_text_color`, `defaults.success_text_color`, `defaults.info_text_color`:
    // 0.7.2 windows-11.toml (uncited)
    status_foreground: srgb8(0xffff_ffff),
    // `button.background_color`: platform-facts.md (§2.3): ButtonBackground =
    // ControlFillColorDefault #B3FFFFFF
    // `combo_box.background_color`: platform-facts.md (§2.24): ComboBoxBackground =
    // ControlFillColorDefault #B3FFFFFF
    // `input.background_color`: platform-facts.md (§2.4): TextControlBackground =
    // ControlFillColorDefault #B3FFFFFF
    // `segmented_control.active_background`: platform-facts.md (§2.25): the selected item,
    // ControlFillColorDefault lettered TextFillColorPrimary #B3FFFFFF and #E4000000
    // (SegmentedItem.xaml:130-131, :154-155; Common_themeresources_any.xaml:219, :209); its accent
    // pill has no field
    control_fill: srgb8(0xffff_ffb3),
    // `button.hover_background`: Interactive state colors (platform-facts.md section 2.3 Windows)
    // platform-facts.md (§2.3): ButtonBackgroundPointerOver = ControlFillColorSecondary #80F9F9F9
    // replaces the rest fill; flattened over background_color #f3f3f3: 249 × 128/255 + 243 ×
    // 127/255 = 246.0 → #f6f6f6
    // `combo_box.hover_background`: ComboBoxBackgroundPointerOver = ControlFillColorSecondary
    // #80F9F9F9 (ComboBox_themeresources.xaml:243; Common_themeresources_any.xaml:220 at
    // microsoft-ui-xaml 8463f45) replaces the rest fill; flattened as button.hover_background
    // (platform-facts.md)
    control_fill_hover: srgb8(0xf6f6_f6ff),
    // `button.active_background`: platform-facts.md (§2.3): ButtonBackgroundPressed =
    // ControlFillColorTertiary #4DF9F9F9, flattened as the hover: 249 × 77/255 + 243 × 178/255 =
    // 244.8 → #f5f5f5
    control_fill_pressed: srgb8(0xf5f5_f5ff),
    // `button.disabled_background`: soft_option: Fluent ControlFillColorDisabled
    // `checkbox.disabled_background`: soft_option: Fluent checkbox disabled fill
    // `combo_box.disabled_background`: soft_option: Fluent ComboBox disabled
    // `input.disabled_background`: soft_option: Fluent TextBox disabled bg
    control_fill_disabled: srgb8(0xf9f9_f900),
    // `checkbox.unchecked_background`: platform-facts.md (§2.5):
    // CheckBoxCheckBackgroundFillUnchecked = ControlAltFillColorSecondary #06000000
    // `switch.unchecked_background`: platform-facts.md (§2.21): ToggleSwitchFillOff =
    // ControlAltFillColorSecondary #06000000, inside ToggleSwitchStrokeOff
    // (unchecked_track_border_color)
    // `segmented_control.background_color`: platform-facts.md (§2.25): the Windows Community Toolkit
    // Segmented's group, ControlAltFillColorSecondary #06000000 (Segmented.xaml:19-20 at
    // v8.2.251219; Common_themeresources_any.xaml:234 at microsoft-ui-xaml 8463f45)
    control_alt_fill: srgb8(0x0000_0006),
    // `segmented_control.border.color`: platform-facts.md (§2.25): ControlStrokeColorDefault
    // (#0F000000, Segmented.xaml:21-22; Common_themeresources_any.xaml:243); its width and radius,
    // 1 and 4, are the defaults' (platform-facts.md)
    control_stroke: srgb8(0x0000_000f),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5):
    // CheckBoxCheckBackgroundStrokeUnchecked = ControlStrongStrokeColorDefault #72000000
    // `progress_bar.track_color`: platform-facts.md (§2.10): ProgressBarBackground =
    // ControlStrongStrokeColorDefault #72000000
    // `switch.unchecked_track_border_color`: platform-facts.md (§2.21): ToggleSwitchStrokeOff =
    // ControlStrongStrokeColorDefault #72000000 (ToggleSwitch_themeresources.xaml:139;
    // Common_themeresources_any.xaml:252), the off track's OuterBorder stroke; its hover and
    // pressed strokes are the same (platform-facts.md)
    strong_stroke: srgb8(0x0000_0072),
    // `switch.disabled_unchecked_track_border_color`: platform-facts.md (§2.21):
    // ToggleSwitchStrokeOffDisabled = ControlStrongStrokeColorDisabled #37000000
    // (ToggleSwitch_themeresources.xaml:142; Common_themeresources_any.xaml:253)
    // a disabled checked or mixed box's stroke: CheckBoxCheckBackgroundStrokeCheckedDisabled,
    // …StrokeIndeterminateDisabled = ControlStrongStrokeColorDisabledBrush
    // (CheckBox_themeresources.xaml:224, :228, set at :465, :578 at microsoft-ui-xaml 8463f45)
    strong_stroke_disabled: srgb8(0x0000_0037),
    // `slider.track_color`: platform-facts.md (§2.9): SliderTrackFill =
    // ControlStrongFillColorDefault #72000000; the outer thumb, SliderOuterThumbBackground =
    // ControlSolidFillColorDefault #FFFFFF
    strong_fill: srgb8(0x0000_0072),
    // `slider.disabled_track_color`: SliderTrackFillDisabled = ControlStrongFillColorDisabled
    // #51000000 (Slider_themeresources.xaml:127; Common_themeresources_any.xaml:227 at
    // microsoft-ui-xaml 8463f45)
    strong_fill_disabled: srgb8(0x0000_0051),
    // `list.hover_background`: Interactive state colors (platform-facts.md section 2.15 Windows)
    // platform-facts.md (§2.15): ListViewItemBackgroundPointerOver = SubtleFillColorSecondary
    // #09000000
    // `list.selection_background`: platform-facts.md (§2.15): ListViewItemBackgroundSelected =
    // SubtleFillColorSecondary #09000000; ListViewItemForegroundSelected = TextFillColorPrimary
    // #E4000000
    // `menu.hover_background`: Interactive state colors (platform-facts.md section 2.6 Windows)
    // platform-facts.md (§2.6): MenuFlyoutItemBackgroundPointerOver = SubtleFillColorSecondary
    // #09000000
    // `sidebar.hover_background`: Interactive state colors (platform-facts.md section 2.12 Windows)
    // platform-facts.md (§2.12): NavigationViewItemBackgroundPointerOver = SubtleFillColorSecondary
    // #09000000
    // `sidebar.selection_background`: platform-facts.md (§2.12): NavigationViewItemBackgroundSelected
    // = SubtleFillColorSecondary #09000000; NavigationViewItemForegroundSelected =
    // TextFillColorPrimary #E4000000
    // `segmented_control.hover_background`: platform-facts.md (§2.25): SubtleFillColorSecondary laid
    // over the group's fill, the item's rest fill being transparent: a layer, as the list's and the
    // sidebar's hovers (#09000000, SegmentedItem.xaml:128-129; Common_themeresources_any.xaml:230)
    // `toolbar.button_hover_background`: platform-facts.md (§2.13): AppBarButtonBackgroundPointerOver
    // = SubtleFillColorSecondary (AppBarButton_themeresources.xaml:6 at microsoft-ui-xaml 8463f45)
    subtle_fill: srgb8(0x0000_0009),
    // `toolbar.button_active_background`: platform-facts.md (§2.13): AppBarButtonBackgroundPressed
    // = SubtleFillColorTertiary (AppBarButton_themeresources.xaml:7)
    subtle_fill_pressed: srgb8(0x0000_0006),
    // a pressed unchecked check box's fill, CheckBoxCheckBackgroundFillUncheckedPressed
    // (CheckBox_themeresources.xaml:231), and a pressed off switch's track, ToggleSwitchFillOffPressed
    // (ToggleSwitch_themeresources.xaml:137): ControlAltFillColorQuarternary #18000000
    // (Common_themeresources_any.xaml:236 at microsoft-ui-xaml 8463f45)
    alt_fill_quarternary: srgb8(0x0000_0018),
    // a pressed checked box's mark, CheckBoxCheckGlyphForegroundCheckedPressed =
    // TextOnAccentFillColorSecondary #B3FFFFFF (CheckBox_themeresources.xaml:247;
    // Common_themeresources_any.xaml:217)
    accent_text_secondary: srgb8(0xffff_ffb3),
    // a disabled checked or mixed box's mark, CheckBoxCheckGlyphForegroundCheckedDisabled and
    // …IndeterminateDisabled = TextOnAccentFillColorDisabledBrush (CheckBox_themeresources.xaml:248,
    // :252, set at :471, :587 at microsoft-ui-xaml 8463f45): TextOnAccentFillColorDisabled #FFFFFF
    // (Common_themeresources_any.xaml:218)
    accent_text_disabled: srgb8(0xffff_ffff),
    // a pressed tab, TabViewItemHeaderBackgroundPressed = LayerOnMicaBaseAltFillColorDefault
    // #B3FFFFFF (TabView_themeresources.xaml:10; TabView.xaml:327;
    // Common_themeresources_any.xaml:268)
    tab_pressed: srgb8(0xffff_ffb3),
    // `checkbox.hover_background`: CheckBoxCheckBackgroundFillUncheckedPointerOver =
    // ControlAltFillColorTertiaryBrush (CheckBox_themeresources.xaml:230), and the radio button's
    // RadioButtonOuterEllipseFillPointerOver the same brush (RadioButton_themeresources.xaml:139):
    // ControlAltFillColorTertiary #0F000000 (Common_themeresources_any.xaml:321, :235 at
    // microsoft-ui-xaml 8463f45), the whole hovered fill in place of the rest one (spec §9, step 2)
    checkbox_hover: srgb8(0x0000_000f),
    // `tab.hover_background`: soft_option: TabViewItemHeaderBackgroundPointerOver =
    // LayerOnMicaBaseAltFillColorSecondary #0A000000 (TabView_themeresources.xaml:9;
    // Common_themeresources_any.xaml:269), over a transparent rest fill
    tab_hover: srgb8(0x0000_000a),
    // `tab.active_background`: Interactive state colors (platform-facts.md section 2.11 Windows)
    // platform-facts.md (§2.11): TabViewItemHeaderBackgroundSelected =
    // SolidBackgroundFillColorTertiary #F9F9F9 (TabView_themeresources.xaml:7;
    // Common_themeresources_any.xaml:274 at microsoft-ui-xaml 8463f45)
    tab_selected: srgb8(0xf9f9_f9ff),
    // `list.alternate_row_background`: platform-facts.md (§2.15): the preset's value, no WinUI3
    // source ❓
    list_alternate_row: srgb8(0xf9f9_f9ff),
    // `input.disabled_text_color`: Interactive state colors (platform-facts.md section 2.4 Windows)
    // TextControlForegroundDisabled = TemporaryTextFillColorDisabled #5C010101
    // (TextBox_themeresources.xaml:141, :129 at microsoft-ui-xaml 8463f45)
    input_disabled_text: srgb8(0x0101_015c),
    // `tooltip.background_color`: platform-facts.md (§2.7): ToolTipBackgroundBrush =
    // AcrylicInAppFillColorDefault, its FallbackColor #F9F9F9
    // `popover.background_color`: platform-facts.md (§2.16): AcrylicInAppFillColorDefault's
    // FallbackColor #F9F9F9
    acrylic_fill: srgb8(0xf9f9_f9ff),
    // `menu.background_color`: platform-facts.md (§2.6): MenuFlyoutPresenterBackground is
    // transparent over the presenter's DesktopAcrylicBackdrop,
    // AcrylicBackgroundFillColorDefaultBackdrop; the brush of that name falls back to #F9F9F9; the
    // backdrop's own fallback colour is not published ❓
    menu_background: srgb8(0xf9f9_f9ff),
    // `popover.border.color`: platform-facts.md (§2.16): FlyoutBorderThemeBrush =
    // SurfaceStrokeColorFlyout #0F000000 (FlyoutPresenter_themeresources.xaml:16, :6, :27;
    // Common_themeresources_any.xaml:255)
    flyout_stroke: srgb8(0x0000_000f),
    // `scrollbar.thumb_color`: platform-facts.md (§2.8): measured
    scrollbar_thumb: Some(srgb8(0xc2c2_c2ff)),
    // `scrollbar.thumb_hover_color`: Interactive state colors (platform-facts.md section 2.8
    // Windows) measured Windows scrollbar thumb hover
    scrollbar_thumb_hover: srgb8(0xa0a0_a0ff),
    // `slider.thumb_color`: 0.7.2 windows-11.toml (uncited)
    slider_thumb: srgb8(0xffff_ffff),
    // `switch.hover_unchecked_background`: ToggleSwitchFillOffPointerOver =
    // ControlAltFillColorTertiary #0F000000 (ToggleSwitch_themeresources.xaml:136;
    // Common_themeresources_any.xaml:235 at microsoft-ui-xaml 8463f45) replaces the rest fill;
    // flattened over background_color: 243 × 240/255 = 228.7 → #e5e5e5
    switch_hover_off: srgb8(0xe5e5_e5ff),
    // `switch.hover_checked_background`: platform-facts.md (§2.21): ToggleSwitchFillOnPointerOver =
    // AccentFillColorSecondaryBrush (ToggleSwitch_themeresources.xaml:144), SystemAccentColorDark1
    // at opacity 0.9 (Common_themeresources_any.xaml:330 at microsoft-ui-xaml 8463f45), the whole
    // hovered fill in place of the rest one (spec §9, step 2): the accent's fill at
    // ACCENT_FILL_SECONDARY_OPACITY, derived by `switch_hover_on`
    switch_hover_on: None,
    // `switch.disabled_unchecked_background`: ToggleSwitchFillOffDisabled =
    // ControlAltFillColorDisabled #00FFFFFF (ToggleSwitch_themeresources.xaml:138;
    // Common_themeresources_any.xaml:237)
    switch_off_disabled: srgb8(0xffff_ff00),
    // `sidebar.background_color`: platform-facts.md (§2.12): the docked pane,
    // NavigationViewExpandedPaneBackground = SolidBackgroundFillColorTransparent #00F3F3F3, so the
    // window shows through
    sidebar_background: srgb8(0xf3f3_f300),
    // `window.inactive_title_bar_text_color`: platform-facts.md (§2.2):
    // TitleBarDeactivatedForegroundBrush = TextFillColorTertiary #72000000
    inactive_title: srgb8(0x0000_0072),
};

/// The dark scheme's constants.
const DARK: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md (§2.1.3): Mica's solid fallback,
    // SolidBackgroundFillColorBase #202020
    // `list.header_background`: platform-facts.md (§2.15): ≈ defaults.background_color
    // `window.title_bar_background`: platform-facts.md (§2.2): the title bar's default background is
    // Mica, the TitleBar control transparent over it; Mica's solid fallback
    // SolidBackgroundFillColorBase #202020 (background_color)
    // `window.inactive_title_bar_background`: platform-facts.md (§2.2): Mica shows that fallback when
    // the window deactivates
    // `dialog.background_color`: platform-facts.md (§2.22): ContentDialogBackground =
    // SolidBackgroundFillColorBase #202020
    window_background: srgb8(0x2020_20ff),
    // `defaults.text_color`: platform-facts.md: TextFillColorPrimary #FFFFFF
    // `button.font.color`: platform-facts.md (§2.3): ButtonForeground = TextFillColorPrimary #FFFFFF
    // `button.hover_text_color`: ButtonForegroundPointerOver = TextFillColorPrimary #FFFFFF
    // (Button_themeresources.xaml:35 at microsoft-ui-xaml 8463f45)
    // `combo_box.font.color`: platform-facts.md (§2.24): ComboBoxForeground = TextFillColorPrimary
    // #FFFFFF
    // `input.font.color`: platform-facts.md (§2.4): TextControlForeground = TextFillColorPrimary
    // #FFFFFF
    // `list.hover_text_color`: ListViewItemForegroundPointerOver = TextFillColorPrimary #FFFFFF
    // (ListViewItem_themeresources.xaml:24 at microsoft-ui-xaml 8463f45)
    // `list.selection_text_color`: 0.7.2 windows-11.toml (uncited)
    // `menu.font.color`: platform-facts.md (§2.6): MenuFlyoutItemForeground = TextFillColorPrimary
    // #FFFFFF
    // `menu.hover_text_color`: MenuFlyoutItemForegroundPointerOver = TextFillColorPrimary #FFFFFF;
    // disabled TextFillColorDisabled (MenuFlyout_themeresources.xaml:12, :14 at microsoft-ui-xaml
    // 8463f45)
    // `popover.font.color`, `segmented_control.active_text_color`, `sidebar.selection_text_color`:
    // 0.7.2 windows-11.toml (uncited)
    // `sidebar.font.color`: NavigationViewItemForeground = TextFillColorPrimary #FFFFFF
    // (NavigationView_themeresources.xaml:21 at microsoft-ui-xaml 8463f45)
    // `tab.active_text_color`: TabViewItemHeaderForegroundSelected = TextFillColorPrimary #FFFFFF
    // (TabView_themeresources.xaml:92 at microsoft-ui-xaml 8463f45)
    // `tooltip.font.color`: platform-facts.md (§2.7): ToolTipForegroundBrush = TextFillColorPrimary
    // #FFFFFF
    // `window.title_bar_font.color`: platform-facts.md (§2.2): TitleBarForegroundBrush =
    // TextFillColorPrimary #FFFFFF
    // `expander.arrow_color`: ExpanderChevronForeground = TextFillColorPrimary #FFFFFF
    // (Expander_themeresources.xaml:18 at microsoft-ui-xaml 8463f45)
    window_foreground: srgb8(0xffff_ffff),
    // `defaults.muted_color`: platform-facts.md: TextFillColorSecondary #C5FFFFFF; over
    // background_color #cccccc
    // `button.active_text_color`: ButtonForegroundPressed = TextFillColorSecondary #C5FFFFFF
    // (Button_themeresources.xaml:36)
    // `input.placeholder_color`: platform-facts.md (§2.4): TextControlPlaceholderForeground =
    // TextFillColorSecondary #C5FFFFFF
    // `tab.font.color`: platform-facts.md (§2.11): an unselected tab's label,
    // TabViewItemHeaderForeground = TextFillColorSecondary #C5FFFFFF, which the header
    // ContentPresenter takes as a ThemeResource (TabView.xaml:575; TabView_themeresources.xaml:90)
    // `tab.hover_text_color`: TabViewItemHeaderForegroundPointerOver = TextFillColorSecondary
    // #C5FFFFFF (TabView_themeresources.xaml:93)
    // `switch.unchecked_thumb_background`: platform-facts.md (§2.21): ToggleSwitchKnobFillOff =
    // TextFillColorSecondary #C5FFFFFF
    window_foreground_muted: srgb8(0xffff_ffc5),
    // `defaults.border.color`: platform-facts.md (§2.1.3, §2.1.6): CardStrokeColorDefault
    // #19000000, its alpha 25/255 the opacity below; over background_color #1d1d1d
    // `card.border.color`: platform-facts.md (§2.26): CardStrokeColorDefault #19000000
    // (Common_themeresources_any.xaml:46 at microsoft-ui-xaml 8463f45)
    // `tab.border.color`: platform-facts.md (§2.11): the selected tab's CardStrokeColorDefault
    // #19000000 (Common_themeresources_any.xaml:46 at microsoft-ui-xaml 8463f45), top and sides
    // `input.hover_border_color`: TextControlBorderBrushPointerOver is the rest brush,
    // TextControlElevationBorderBrush (TextBox_themeresources.xaml:28, :27 at microsoft-ui-xaml
    // 8463f45), so the input's rest border, defaults.border.color
    window_border: srgb8(0x0000_0019),
    // `defaults.disabled_text_color`: platform-facts.md: TextFillColorDisabled #5DFFFFFF; over
    // background_color #717171
    // `button.disabled_text_color`: ButtonForegroundDisabled = TextFillColorDisabled #5DFFFFFF
    // (Button_themeresources.xaml:37)
    // `checkbox.disabled_text_color`: Interactive state colors (platform-facts.md section 2.5
    // Windows) CheckBoxForegroundUncheckedDisabled = TextFillColorDisabled #5DFFFFFF
    // (CheckBox_themeresources.xaml:8 at microsoft-ui-xaml 8463f45)
    // `combo_box.disabled_text_color`: Interactive state colors (platform-facts.md section 2.24
    // Windows) ComboBoxForegroundDisabled = TextFillColorDisabled #5DFFFFFF
    // (ComboBox_themeresources.xaml:43 at microsoft-ui-xaml 8463f45)
    // `list.disabled_text_color`: defaults.disabled_text_color: ListView has no disabled
    // foreground; a disabled row fades by ListViewItemDisabledThemeOpacity 0.3
    // (platform-facts.md), which a colour cannot carry
    // `menu.disabled_text_color`: 0.7.2 windows-11.toml (uncited)
    disabled_text: srgb8(0xffff_ff5d),
    // `defaults.surface_color`: platform-facts.md: CardBackgroundFillColorDefault #0DFFFFFF; over
    // background_color #2b2b2b
    view_background: srgb8(0xffff_ff0d),
    // `defaults.accent_color`: 0.7.2 windows-11.toml (uncited)
    accent: srgb8(0x0078_d4ff),
    // `defaults.accent_text_color`, `button.checked_text_color`: 0.7.2 windows-11.toml (uncited)
    // `switch.thumb_background`: platform-facts.md (§2.21): ToggleSwitchKnobFillOn =
    // TextOnAccentFillColorPrimary #000000
    accent_foreground: srgb8(0x0000_00ff),
    // `button.primary_background`: platform-facts.md (§2.3): AccentButtonBackground =
    // AccentFillColorDefault = SystemAccentColorLight2 (Button_themeresources.xaml:5,
    // Common_themeresources_any.xaml:125 at microsoft-ui-xaml 8463f45), UISettings AccentLight2
    // #4cc2ff for the accent above (OS probe, Windows 11 25H2, probe.toml); primary_text_color
    // stays ← accent_text_color
    // `button.checked_background`: platform-facts.md (§2.3): ToggleButtonBackgroundChecked =
    // AccentFillColorDefault = SystemAccentColorLight2, UISettings AccentLight2 #4cc2ff for the
    // accent above (OS probe, Windows 11 25H2); ToggleButtonForegroundChecked =
    // TextOnAccentFillColorPrimary #000000
    // `checkbox.checked_background`: platform-facts.md (§2.5): CheckBoxCheckBackgroundFillChecked =
    // AccentFillColorDefault = SystemAccentColorLight2 (CheckBox_themeresources.xaml:57,
    // Common_themeresources_any.xaml:125 at microsoft-ui-xaml 8463f45), UISettings AccentLight2
    // #4cc2ff for the accent above (OS probe, Windows 11 25H2, probe.toml); the radio's
    // RadioButtonOuterEllipseCheckedFill is the same brush (RadioButton_themeresources.xaml:30)
    // `progress_bar.fill_color`: platform-facts.md (§2.10): ProgressBarForeground =
    // AccentFillColorDefault = SystemAccentColorLight2 (ProgressBar_themeresources.xaml:6,
    // Common_themeresources_any.xaml:125 at microsoft-ui-xaml 8463f45), UISettings AccentLight2
    // #4cc2ff for the accent above (OS probe, Windows 11 25H2, probe.toml)
    // `slider.fill_color`: platform-facts.md (§2.9): SliderTrackValueFill = AccentFillColorDefault =
    // SystemAccentColorLight2 (Slider_themeresources.xaml:24, Common_themeresources_any.xaml:125 at
    // microsoft-ui-xaml 8463f45), UISettings AccentLight2 #4cc2ff for the accent above (OS probe,
    // Windows 11 25H2, probe.toml)
    // `spinner.fill_color`: platform-facts.md (§2.23): ProgressRingForegroundThemeBrush =
    // AccentFillColorDefault, UISettings AccentLight2 #4cc2ff for the accent above (OS probe,
    // Windows 11 25H2, probe.toml)
    // `switch.checked_background`: platform-facts.md (§2.21): ToggleSwitchFillOn =
    // AccentFillColorDefault = SystemAccentColorLight2 (ToggleSwitch_themeresources.xaml:23,
    // Common_themeresources_any.xaml:125 at microsoft-ui-xaml 8463f45), UISettings AccentLight2
    // #4cc2ff for the accent above (OS probe, Windows 11 25H2, probe.toml)
    // `toolbar.button_checked_background`: platform-facts.md (§2.13):
    // AppBarToggleButtonBackgroundChecked = AccentFillColorDefault = SystemAccentColorLight2,
    // #4cc2ff for the accent above (AppBarToggleButton_themeresources.xaml:9)
    // `input.focus_border_color`: TextControlBorderBrushFocused =
    // TextControlElevationBorderFocusedBrush, a 2px bottom line in SystemAccentColorLight2 over
    // ControlStrokeColorDefault (TextBox_themeresources.xaml:29, :57-66 at microsoft-ui-xaml
    // 8463f45), UISettings AccentLight2 for the accent above (OS probe, Windows 11 25H2)
    // `link.active_text_color`: 0.7.2 windows-11.toml (uncited)
    accent_fill: srgb8(0x4cc2_ffff),
    // `slider.disabled_fill_color`: SliderTrackValueFillDisabled = AccentFillColorDisabled
    // #28FFFFFF (Slider_themeresources.xaml:27; Common_themeresources_any.xaml:38 at
    // microsoft-ui-xaml 8463f45)
    // `switch.disabled_checked_background`: ToggleSwitchFillOnDisabled = AccentFillColorDisabled
    // #28FFFFFF (ToggleSwitch_themeresources.xaml:26; Common_themeresources_any.xaml:38 at
    // microsoft-ui-xaml 8463f45)
    // a disabled checked or mixed box's fill and a disabled checked radio's circle, fill and
    // stroke: CheckBoxCheckBackgroundFillCheckedDisabled, …FillIndeterminateDisabled,
    // RadioButtonOuterEllipseCheckedFillDisabled, …CheckedStrokeDisabled =
    // AccentFillColorDisabledBrush (CheckBox_themeresources.xaml:60, :64;
    // RadioButton_themeresources.xaml:33, :29 at microsoft-ui-xaml 8463f45)
    accent_fill_disabled: srgb8(0xffff_ff28),
    // `defaults.selection_text_color`: selection_background ← accent_color: platform-facts.md
    // (§2.1.3), a text box's TextControlSelectionHighlightColor =
    // AccentFillColorSelectedTextBackground = SystemAccentColor no source ❓: platform-facts.md's
    // COLOR_HIGHLIGHTTEXT was read in light mode only (OS probe); WinUI states no selected-text
    // foreground; the light value stays
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): SystemControlFocusVisualPrimaryBrush
    // = FocusStrokeColorOuter #FFFFFF
    focus_ring: srgb8(0xffff_ffff),
    // platform-facts.md (§2.1.5): focus_ring_color: the 1px inner ring FocusStrokeColorInner
    // #B3000000
    focus_ring_inner: srgb8(0x0000_00b3),
    // `defaults.link_color`: platform-facts.md: HyperlinkForeground = SystemAccentColorLight3,
    // UISettings AccentLight3 for the accent above (OS probe, Windows 11 25H2)
    link: srgb8(0x99eb_ffff),
    // `link.hover_text_color`: Interactive state colors (platform-facts.md section 2.28 Windows)
    // HyperlinkForegroundPointerOver = SystemAccentColorLight3, HyperlinkForegroundPressed =
    // SystemAccentColorLight2 (Hyperlink_themeresources.xaml:6-7,
    // Common_themeresources_any.xaml:94-95 at microsoft-ui-xaml 8463f45), UISettings AccentLight3 /
    // AccentLight2 (OS probe, Windows 11 25H2)
    link_hover: srgb8(0x99eb_ffff),
    // `link.disabled_text_color`: HyperlinkButtonForegroundDisabled = AccentTextFillColorDisabled
    // #5DFFFFFF (HyperlinkButton_themeresources.xaml:8)
    link_disabled: srgb8(0xffff_ff5d),
    // `defaults.shadow_color`: no source ❓: as the light variant; platform-facts.md's dark ambient
    // layer is 14% (#00000024) low and 20% (#00000033) high
    shadow: srgb8(0x0000_0040),
    // `defaults.danger_color`: 0.7.2 windows-11.toml (uncited)
    error: srgb8(0xff99_a4ff),
    // `defaults.warning_color`: 0.7.2 windows-11.toml (uncited)
    warning: srgb8(0xfce1_00ff),
    // `defaults.success_color`: 0.7.2 windows-11.toml (uncited)
    success: srgb8(0x6ccb_5fff),
    // `defaults.info_color`: platform-facts.md: SystemFillColorAttention = SystemAccentColorLight2,
    // UISettings AccentLight2 for the accent above (OS probe, Windows 11 25H2)
    info: srgb8(0x4cc2_ffff),
    // `defaults.danger_text_color`: platform-facts.md (§2.1.4): InfoBar *SeverityIconForeground =
    // TextFillColorInverse #E4000000
    // `defaults.warning_text_color`, `defaults.success_text_color`, `defaults.info_text_color`:
    // 0.7.2 windows-11.toml (uncited)
    status_foreground: srgb8(0x0000_00e4),
    // `button.background_color`: platform-facts.md (§2.3): ButtonBackground =
    // ControlFillColorDefault #0FFFFFFF
    // `combo_box.background_color`: platform-facts.md (§2.24): ComboBoxBackground =
    // ControlFillColorDefault #0FFFFFFF
    // `input.background_color`: platform-facts.md (§2.4): TextControlBackground =
    // ControlFillColorDefault #0FFFFFFF
    // `segmented_control.active_background`: platform-facts.md (§2.25): the selected item,
    // ControlFillColorDefault lettered TextFillColorPrimary #0FFFFFFF and #FFFFFF
    // (SegmentedItem.xaml:13-14, :36-37; Common_themeresources_any.xaml:15, :5); its accent pill
    // has no field
    control_fill: srgb8(0xffff_ff0f),
    // `button.hover_background`: Interactive state colors (platform-facts.md section 2.3 Windows)
    // platform-facts.md (§2.3): ButtonBackgroundPointerOver = ControlFillColorSecondary #15FFFFFF
    // replaces the rest fill; flattened over background_color #202020: 255 × 21/255 + 32 × 234/255
    // = 50.4 → #323232
    // `combo_box.hover_background`: ComboBoxBackgroundPointerOver = ControlFillColorSecondary
    // #15FFFFFF (ComboBox_themeresources.xaml:33; Common_themeresources_any.xaml:16 at
    // microsoft-ui-xaml 8463f45) replaces the rest fill; flattened as button.hover_background
    // (platform-facts.md)
    control_fill_hover: srgb8(0x3232_32ff),
    // `button.active_background`: platform-facts.md (§2.3): ButtonBackgroundPressed =
    // ControlFillColorTertiary #08FFFFFF, flattened as the hover: 255 × 8/255 + 32 × 247/255 = 39.0
    // → #272727
    control_fill_pressed: srgb8(0x2727_27ff),
    // `button.disabled_background`: soft_option: Fluent ControlFillColorDisabled
    // `checkbox.disabled_background`: soft_option: Fluent checkbox disabled fill
    // `combo_box.disabled_background`: soft_option: Fluent ComboBox disabled
    // `input.disabled_background`: soft_option: Fluent TextBox disabled bg
    control_fill_disabled: srgb8(0x3333_3300),
    // `checkbox.unchecked_background`: platform-facts.md (§2.5):
    // CheckBoxCheckBackgroundFillUnchecked = ControlAltFillColorSecondary #19000000
    // `switch.unchecked_background`: platform-facts.md (§2.21): ToggleSwitchFillOff =
    // ControlAltFillColorSecondary #19000000, inside ToggleSwitchStrokeOff
    // (unchecked_track_border_color)
    // `segmented_control.background_color`: platform-facts.md (§2.25): the Windows Community Toolkit
    // Segmented's group, ControlAltFillColorSecondary #19000000 (Segmented.xaml:12-13 at
    // v8.2.251219; Common_themeresources_any.xaml:30 at microsoft-ui-xaml 8463f45)
    control_alt_fill: srgb8(0x0000_0019),
    // `segmented_control.border.color`: platform-facts.md (§2.25): ControlStrokeColorDefault
    // (#12FFFFFF, Segmented.xaml:14-15; Common_themeresources_any.xaml:39); its width and radius,
    // 1 and 4, are the defaults' (platform-facts.md)
    control_stroke: srgb8(0xffff_ff12),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5):
    // CheckBoxCheckBackgroundStrokeUnchecked = ControlStrongStrokeColorDefault #8BFFFFFF
    // `progress_bar.track_color`: platform-facts.md (§2.10): ProgressBarBackground =
    // ControlStrongStrokeColorDefault #8BFFFFFF
    // `switch.unchecked_track_border_color`: platform-facts.md (§2.21): ToggleSwitchStrokeOff =
    // ControlStrongStrokeColorDefault #8BFFFFFF (ToggleSwitch_themeresources.xaml:19;
    // Common_themeresources_any.xaml:48), the off track's OuterBorder stroke; its hover and pressed
    // strokes are the same (platform-facts.md)
    strong_stroke: srgb8(0xffff_ff8b),
    // `switch.disabled_unchecked_track_border_color`: platform-facts.md (§2.21):
    // ToggleSwitchStrokeOffDisabled = ControlStrongStrokeColorDisabled #28FFFFFF
    // (ToggleSwitch_themeresources.xaml:22; Common_themeresources_any.xaml:49)
    // a disabled checked or mixed box's stroke: CheckBoxCheckBackgroundStrokeCheckedDisabled,
    // …StrokeIndeterminateDisabled = ControlStrongStrokeColorDisabledBrush
    // (CheckBox_themeresources.xaml:48, :52, set at :465, :578 at microsoft-ui-xaml 8463f45)
    strong_stroke_disabled: srgb8(0xffff_ff28),
    // `slider.track_color`: platform-facts.md (§2.9): SliderTrackFill =
    // ControlStrongFillColorDefault #8BFFFFFF; the outer thumb, SliderOuterThumbBackground =
    // ControlSolidFillColorDefault #454545
    strong_fill: srgb8(0xffff_ff8b),
    // `slider.disabled_track_color`: SliderTrackFillDisabled = ControlStrongFillColorDisabled
    // #3FFFFFFF (Slider_themeresources.xaml:23; Common_themeresources_any.xaml:23 at
    // microsoft-ui-xaml 8463f45)
    strong_fill_disabled: srgb8(0xffff_ff3f),
    // `list.hover_background`: Interactive state colors (platform-facts.md section 2.15 Windows)
    // platform-facts.md (§2.15): ListViewItemBackgroundPointerOver = SubtleFillColorSecondary
    // #0FFFFFFF
    // `list.selection_background`: platform-facts.md (§2.15): ListViewItemBackgroundSelected =
    // SubtleFillColorSecondary #0FFFFFFF; ListViewItemForegroundSelected = TextFillColorPrimary
    // #FFFFFF
    // `menu.hover_background`: Interactive state colors (platform-facts.md section 2.6 Windows)
    // platform-facts.md (§2.6): MenuFlyoutItemBackgroundPointerOver = SubtleFillColorSecondary
    // #0FFFFFFF
    // `sidebar.hover_background`: Interactive state colors (platform-facts.md section 2.12 Windows)
    // platform-facts.md (§2.12): NavigationViewItemBackgroundPointerOver = SubtleFillColorSecondary
    // #0FFFFFFF
    // `sidebar.selection_background`: platform-facts.md (§2.12): NavigationViewItemBackgroundSelected
    // = SubtleFillColorSecondary #0FFFFFFF; NavigationViewItemForegroundSelected =
    // TextFillColorPrimary #FFFFFF
    // `segmented_control.hover_background`: platform-facts.md (§2.25): SubtleFillColorSecondary laid
    // over the group's fill, the item's rest fill being transparent: a layer, as the list's and the
    // sidebar's hovers (#0FFFFFFF, SegmentedItem.xaml:11-12; Common_themeresources_any.xaml:26)
    // `toolbar.button_hover_background`: platform-facts.md (§2.13): AppBarButtonBackgroundPointerOver
    // = SubtleFillColorSecondary (AppBarButton_themeresources.xaml:6 at microsoft-ui-xaml 8463f45)
    subtle_fill: srgb8(0xffff_ff0f),
    // `toolbar.button_active_background`: platform-facts.md (§2.13): AppBarButtonBackgroundPressed
    // = SubtleFillColorTertiary (AppBarButton_themeresources.xaml:7)
    subtle_fill_pressed: srgb8(0xffff_ff0a),
    // a pressed unchecked check box's fill, CheckBoxCheckBackgroundFillUncheckedPressed
    // (CheckBox_themeresources.xaml:55), and a pressed off switch's track, ToggleSwitchFillOffPressed
    // (ToggleSwitch_themeresources.xaml:17): ControlAltFillColorQuarternary #12FFFFFF
    // (Common_themeresources_any.xaml:32 at microsoft-ui-xaml 8463f45)
    alt_fill_quarternary: srgb8(0xffff_ff12),
    // a pressed checked box's mark, CheckBoxCheckGlyphForegroundCheckedPressed =
    // TextOnAccentFillColorSecondary #80000000 (CheckBox_themeresources.xaml:71;
    // Common_themeresources_any.xaml:13)
    accent_text_secondary: srgb8(0x0000_0080),
    // a disabled checked or mixed box's mark, CheckBoxCheckGlyphForegroundCheckedDisabled and
    // …IndeterminateDisabled = TextOnAccentFillColorDisabledBrush (CheckBox_themeresources.xaml:72,
    // :76, set at :471, :587 at microsoft-ui-xaml 8463f45): TextOnAccentFillColorDisabled
    // #87FFFFFF (Common_themeresources_any.xaml:14)
    accent_text_disabled: srgb8(0xffff_ff87),
    // a pressed tab, TabViewItemHeaderBackgroundPressed = LayerOnMicaBaseAltFillColorDefault
    // #733A3A3A (TabView_themeresources.xaml:88; TabView.xaml:327;
    // Common_themeresources_any.xaml:64)
    tab_pressed: srgb8(0x3a3a_3a73),
    // `checkbox.hover_background`: CheckBoxCheckBackgroundFillUncheckedPointerOver =
    // ControlAltFillColorTertiaryBrush (CheckBox_themeresources.xaml:54), and the radio button's
    // RadioButtonOuterEllipseFillPointerOver the same brush (RadioButton_themeresources.xaml:23):
    // ControlAltFillColorTertiary #0BFFFFFF (Common_themeresources_any.xaml:117, :31 at
    // microsoft-ui-xaml 8463f45), the whole hovered fill in place of the rest one (spec §9, step 2)
    checkbox_hover: srgb8(0xffff_ff0b),
    // `tab.hover_background`: soft_option: TabViewItemHeaderBackgroundPointerOver =
    // LayerOnMicaBaseAltFillColorSecondary #0FFFFFFF (TabView_themeresources.xaml:87;
    // Common_themeresources_any.xaml:65), over a transparent rest fill
    tab_hover: srgb8(0xffff_ff0f),
    // `tab.active_background`: Interactive state colors (platform-facts.md section 2.11 Windows)
    // platform-facts.md (§2.11): TabViewItemHeaderBackgroundSelected =
    // SolidBackgroundFillColorTertiary #282828 (TabView_themeresources.xaml:85;
    // Common_themeresources_any.xaml:70 at microsoft-ui-xaml 8463f45)
    tab_selected: srgb8(0x2828_28ff),
    // `list.alternate_row_background`: platform-facts.md (§2.15): the preset's value, no WinUI3
    // source ❓
    list_alternate_row: srgb8(0x2626_26ff),
    // `input.disabled_text_color`: Interactive state colors (platform-facts.md section 2.4 Windows)
    // TextControlForegroundDisabled = TemporaryTextFillColorDisabled #5DFEFEFE
    // (TextBox_themeresources.xaml:34, :22 at microsoft-ui-xaml 8463f45)
    input_disabled_text: srgb8(0xfefe_fe5d),
    // `tooltip.background_color`: platform-facts.md (§2.7): ToolTipBackgroundBrush =
    // AcrylicInAppFillColorDefault, its FallbackColor #2C2C2C
    // `popover.background_color`: platform-facts.md (§2.16): AcrylicInAppFillColorDefault's
    // FallbackColor #2C2C2C
    acrylic_fill: srgb8(0x2c2c_2cff),
    // `menu.background_color`: platform-facts.md (§2.6): MenuFlyoutPresenterBackground is
    // transparent over the presenter's DesktopAcrylicBackdrop,
    // AcrylicBackgroundFillColorDefaultBackdrop; the brush of that name falls back to #2C2C2C; the
    // backdrop's own fallback colour is not published ❓
    menu_background: srgb8(0x2c2c_2cff),
    // `popover.border.color`: platform-facts.md (§2.16): FlyoutBorderThemeBrush =
    // SurfaceStrokeColorFlyout #33000000 (FlyoutPresenter_themeresources.xaml:16, :6, :27;
    // Common_themeresources_any.xaml:51)
    flyout_stroke: srgb8(0x0000_0033),
    // Dark states no thumb colour of its own (0.7.2 takes the muted colour).
    scrollbar_thumb: None,
    // `scrollbar.thumb_hover_color`: thumb_color no source ❓: platform-facts.md's measured #c2c2c2
    // names no mode, and no dark measurement exists (docs/todo.md, "the scrollbar thumb colour
    // platform-facts measures") Interactive state colors (platform-facts.md section 2.8 Windows)
    // measured Windows scrollbar thumb hover
    scrollbar_thumb_hover: srgb8(0xa0a0_a0ff),
    // `slider.thumb_color`: 0.7.2 windows-11.toml (uncited)
    slider_thumb: srgb8(0x4545_45ff),
    // `switch.hover_unchecked_background`: ToggleSwitchFillOffPointerOver =
    // ControlAltFillColorTertiary #0BFFFFFF (ToggleSwitch_themeresources.xaml:16;
    // Common_themeresources_any.xaml:31 at microsoft-ui-xaml 8463f45) replaces the rest fill;
    // flattened over background_color: 255 × 11/255 + 32 × 244/255 = 41.6 → #2a2a2a
    switch_hover_off: srgb8(0x2a2a_2aff),
    // `switch.hover_checked_background`: platform-facts.md (§2.21): ToggleSwitchFillOnPointerOver =
    // AccentFillColorSecondaryBrush (ToggleSwitch_themeresources.xaml:24), SystemAccentColorLight2
    // at opacity 0.9 (Common_themeresources_any.xaml:126 at microsoft-ui-xaml 8463f45), the whole
    // hovered fill in place of the rest one (spec §9, step 2): the accent's fill at
    // ACCENT_FILL_SECONDARY_OPACITY, derived by `switch_hover_on`
    switch_hover_on: None,
    // `switch.disabled_unchecked_background`: ToggleSwitchFillOffDisabled =
    // ControlAltFillColorDisabled #00FFFFFF (ToggleSwitch_themeresources.xaml:18;
    // Common_themeresources_any.xaml:33)
    switch_off_disabled: srgb8(0xffff_ff00),
    // `sidebar.background_color`: platform-facts.md (§2.12): the docked pane,
    // NavigationViewExpandedPaneBackground = SolidBackgroundFillColorTransparent #00202020, so the
    // window shows through
    sidebar_background: srgb8(0x2020_2000),
    // `window.inactive_title_bar_text_color`: platform-facts.md (§2.2):
    // TitleBarDeactivatedForegroundBrush = TextFillColorTertiary #87FFFFFF
    inactive_title: srgb8(0xffff_ff87),
};

// SubtleFillColorTransparent, #00FFFFFF (Common_themeresources_any.xaml:229, :25 at
// microsoft-ui-xaml 8463f45): `scrollbar.track_color`, platform-facts.md (§2.8):
// ScrollBarBackground = SubtleFillColorTransparent #00FFFFFF; the subtle (flat) button's fill at
// rest and disabled, SubtleButtonBackground and SubtleButtonBackgroundDisabled =
// SubtleFillColorTransparentBrush (Button_themeresources.xaml:17, :20 at microsoft-ui-xaml
// 8463f45; `docs/facts/windows/button.toml` on `v0.8.0-rc1` at `78ee0efd`, row W1-021)
const SUBTLE_FILL_TRANSPARENT: Color = srgb8(0xffff_ff00);
// `link.background_color`: 0.7.2 windows-11.toml (uncited)
// `link.hover_background`: platform-facts.md (§2.28): an inline Hyperlink has no background and
// changes only its foreground
const LINK_BACKGROUND: Color = srgb8(0x0000_0000);

// `defaults.font.family`: 0.7.2 windows-11.toml (uncited)
const FONT_FAMILY: &str = "Segoe UI";
// `defaults.font.size`, `defaults.font.defined_size`: platform-facts.md (§2.1.1): WinUI3
// ControlContentThemeFontSize 14px; lfMessageFont is 9pt
const FONT_SIZE_PT: f32 = 10.5;
// `defaults.font.weight`: 0.7.2 windows-11.toml (uncited)
const FONT_WEIGHT: u16 = 400;
// `defaults.mono_font.family`: 0.7.2 windows-11.toml (uncited)
const MONO_FAMILY: &str = "Cascadia Mono";
// `defaults.mono_font.size`, `defaults.mono_font.defined_size`: 0.7.2 windows-11.toml (uncited)
const MONO_SIZE_PT: f32 = 10.5;
// `defaults.mono_font.weight`: 0.7.2 windows-11.toml (uncited)
const MONO_WEIGHT: u16 = 400;
// `defaults.line_height`: 0.7.2 windows-11.toml (uncited)
const LINE_HEIGHT: f32 = 1.43;
// `text_scale.caption.size`: 0.7.2 windows-11.toml (uncited)
const CAPTION_SIZE_PT: f32 = 9.0;
// `text_scale.caption.line_height`: platform-facts.md (§1.2.1): the Fluent type ramp's Caption
// 12px, line height 16px
const CAPTION_LINE_HEIGHT_PX: f32 = 16.0;
// `text_scale.section_heading.size`: 0.7.2 windows-11.toml (uncited)
const HEADLINE_SIZE_PT: f32 = 15.0;
// `text_scale.section_heading.weight`: 0.7.2 windows-11.toml (uncited)
const HEADLINE_WEIGHT: u16 = 600;
// `text_scale.section_heading.line_height`: platform-facts.md (§1.2.1): the Fluent type ramp's
// Subtitle 20px, line height 28px
const HEADLINE_LINE_HEIGHT_PX: f32 = 28.0;
// `text_scale.dialog_title.size`: 0.7.2 windows-11.toml (uncited)
const TITLE1_SIZE_PT: f32 = 21.0;
// `text_scale.dialog_title.weight`: 0.7.2 windows-11.toml (uncited)
const TITLE1_WEIGHT: u16 = 600;
// `text_scale.dialog_title.line_height`: platform-facts.md (§1.2.1): the Fluent type ramp's Title
// 28px, line height 36px
const TITLE1_LINE_HEIGHT_PX: f32 = 36.0;
// `text_scale.display.size`: 0.7.2 windows-11.toml (uncited)
const DISPLAY_SIZE_PT: f32 = 51.0;
// `text_scale.display.weight`: 0.7.2 windows-11.toml (uncited)
const DISPLAY_WEIGHT: u16 = 600;
// `text_scale.display.line_height`: platform-facts.md (§1.2.1): the Fluent type ramp's Display
// 68px, line height 92px
const DISPLAY_LINE_HEIGHT_PX: f32 = 92.0;

// `defaults.border.corner_radius`: 0.7.2 windows-11.toml (uncited)
const CORNER_RADIUS: f32 = 4.0;
// `defaults.border.corner_radius_lg`: 0.7.2 windows-11.toml (uncited)
const CORNER_RADIUS_LG: f32 = 8.0;
// `defaults.border.line_width`: 0.7.2 windows-11.toml (uncited)
const BORDER_WIDTH: f32 = 1.0;
// `defaults.border.opacity`: 0.7.2 windows-11.toml (uncited)
const BORDER_OPACITY_LIGHT: f32 = 0.059;
// `defaults.border.opacity`: 0.7.2 windows-11.toml (uncited)
const BORDER_OPACITY_DARK: f32 = 0.098;
// `defaults.border.shadow_enabled`: 0.7.2 windows-11.toml (uncited)
const SHADOW_ENABLED: bool = true;
// `defaults.disabled_opacity`: platform-facts.md (§2.1.6): 1.0, WinUI 3 dims a disabled control by
// its *Disabled brushes alone
const DISABLED_OPACITY: f32 = 1.0;
// AccentFillColorSecondaryBrush's opacity over AccentFillColorDefault's colour, SystemAccentColorDark1
// in light and SystemAccentColorLight2 in dark (Common_themeresources_any.xaml:330, :329 and :126,
// :125 at microsoft-ui-xaml 8463f45)
const ACCENT_FILL_SECONDARY_OPACITY: f32 = 0.9;
// AccentFillColorTertiaryBrush's opacity over the same colour (Common_themeresources_any.xaml:331,
// :127 at microsoft-ui-xaml 8463f45)
const ACCENT_FILL_TERTIARY_OPACITY: f32 = 0.8;
// `defaults.focus_ring_width`: platform-facts.md (§2.1.5): focus_ring_width: the Fluent focus
// visual's 2px outer (primary) ring
const FOCUS_RING_WIDTH: f32 = 2.0;
// platform-facts.md (§2.1.5): focus_ring_offset: the 1px inner (secondary) ring inside the outer
// one; ListViewItem's FocusVisualSecondaryThickness 1 (ListViewItem_themeresources.xaml:252 at
// microsoft-ui-xaml 8463f45)
const FOCUS_RING_INNER_WIDTH: f32 = 1.0;
// The list row's ring: ListViewItem's FocusVisualMargin 1 (platform-facts.md (§2.1.5):
// focus_ring_offset; ListViewItem_themeresources.xaml:248 at microsoft-ui-xaml 8463f45) puts the
// outer ring's outer edge 1 px inside the row, and its FocusVisualPrimaryThickness 2 (:250) its
// inner edge 3 px inside
const LIST_ROW_FOCUS_RING_OFFSET: f32 = -3.0;
// `defaults.focus_ring_offset`: platform-facts.md (§2.1.5): Button FocusVisualMargin -3 around a
// 2px outer and a 1px inner ring
const FOCUS_RING_OFFSET: f32 = 1.0;
// `defaults.focus_ring_keyboard_only`: platform-facts.md (§2.1.5): focus_ring_keyboard_only: the
// focus visual is drawn only in the Keyboard focus state (focusmgr.cpp:2731-2733,
// microsoft-ui-xaml 2.5.1)
const FOCUS_RING_KEYBOARD_ONLY: bool = true;

// `Resolved.icon_set`: 0.7.2 windows-11.toml (uncited)
const ICON_SET: IconSet = IconSet::SegoeFluent;
// `Resolved.icon_theme`: 0.7.2 windows-11.toml (uncited)
const ICON_THEME: &str = "segoe-fluent";
// `defaults.icon_sizes.toolbar`: platform-facts.md (§2.1.8): AppBarButtonContentHeight = 16
const ICON_TOOLBAR: f32 = 16.0;
// `defaults.icon_sizes.small`: 0.7.2 windows-11.toml (uncited)
const ICON_SMALL: f32 = 16.0;
// `defaults.icon_sizes.large`: 0.7.2 windows-11.toml (uncited)
const ICON_LARGE: f32 = 32.0;
// `defaults.icon_sizes.dialog`: 0.7.2 windows-11.toml (uncited)
const ICON_DIALOG: f32 = 22.0;
// `defaults.icon_sizes.panel`: 0.7.2 windows-11.toml (uncited)
const ICON_PANEL: f32 = 20.0;

// `layout.widget_gap`: 0.7.2 windows-11.toml (uncited)
const WIDGET_GAP: f32 = 6.0;
// `layout.container_margin`: 0.7.2 windows-11.toml (uncited)
const CONTAINER_MARGIN: f32 = 6.0;
// `layout.window_margin`: 0.7.2 windows-11.toml (uncited)
const WINDOW_MARGIN: f32 = 10.0;
// `layout.section_gap`: 0.7.2 windows-11.toml (uncited)
const SECTION_GAP: f32 = 18.0;

// `tooltip.show_delay`: platform-facts.md (§2.7): 2 × SPI_GETMOUSEHOVERTIME, 400 on the OS probe's
// Windows images (ToolTipService_Partial.cpp:1731-1783)
const TOOLTIP_SHOW_MS: u32 = 800;
// `input.caret_blink_on`: platform-facts.md (§2.4): GetCaretBlinkTime 530 on the WinUI host's
// Windows 11 image; WinUI shows the caret one blink time (TextBoxHelpers.cpp:55-89 at
// microsoft-ui-xaml 2.5.1)
const CARET_BLINK_ON_MS: u32 = 530;
// `input.caret_blink_off`: platform-facts.md (§2.4): and hides it one blink time
const CARET_BLINK_OFF_MS: u32 = 530;

// `button.min_width`: 0.7.2 windows-11.toml (uncited)
const BUTTON_MIN_WIDTH: f32 = 64.0;
// `button.min_height`: 0.7.2 windows-11.toml (uncited)
const BUTTON_MIN_HEIGHT: f32 = 32.0;
// `button.icon_text_gap`: 0.7.2 windows-11.toml (uncited)
const BUTTON_ICON_TEXT_GAP: f32 = 8.0;
// `button.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const BUTTON_PADDING_TOP: f32 = 5.0;
// `button.border.padding.left`, `button.border.padding.right`: platform-facts.md (§2.3): WinUI3
// ButtonPadding=11,5,11,6
const BUTTON_PADDING_HORIZONTAL: f32 = 11.0;
// `button.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const BUTTON_PADDING_BOTTOM: f32 = 6.0;

// `checkbox.indicator_width`: 0.7.2 windows-11.toml (uncited)
const CHECKBOX_INDICATOR_SIZE: f32 = 20.0;
// `checkbox.radio_dot_diameter`: platform-facts.md (§2.5): RadioButtonCheckGlyphSize = 12
const RADIO_DOT_SIZE: f32 = 12.0;
// `checkbox.label_gap`: 0.7.2 windows-11.toml (uncited)
const CHECKBOX_LABEL_GAP: f32 = 8.0;

// `list.row_height`: 0.7.2 windows-11.toml (uncited)
const LIST_ROW_HEIGHT: f32 = 40.0;
// `list.border.padding.top`, `list.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const LIST_ROW_PADDING_VERTICAL: f32 = 0.0;
// `list.border.padding.right`: 0.7.2 windows-11.toml (uncited)
const LIST_ROW_PADDING_RIGHT: f32 = 12.0;
// `list.border.padding.left`: platform-facts.md (§2.15): WinUI3 DefaultListViewItemStyle
// Padding=16,0,12,0
const LIST_ROW_PADDING_LEFT: f32 = 16.0;

// `scrollbar.groove_width`: 0.7.2 windows-11.toml (uncited)
const SCROLLBAR_GROOVE_WIDTH: f32 = 17.0;
// `scrollbar.min_thumb_length`: 0.7.2 windows-11.toml (uncited)
const SCROLLBAR_MIN_THUMB_LENGTH: f32 = 17.0;
// `scrollbar.thumb_width`: 0.7.2 windows-11.toml (uncited)
const SCROLLBAR_THUMB_WIDTH: f32 = 6.0;
// `scrollbar.overlay_mode`: platform-facts.md (§2.8): AutoHideScrollBars reads true on the OS
// probe, but which scroll bars follow it has no source ❓, so false stays (M2b-21)
const SCROLLBAR_OVERLAY: bool = false;

// `tooltip.font.size`, `tooltip.font.defined_size`: platform-facts.md (§2.7):
// ToolTipContentThemeFontSize 12 (9pt at 96 DPI); lfStatusFont -12 (OS probe)
const TOOLTIP_FONT_SIZE_PT: f32 = 9.0;
// `tooltip.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const TOOLTIP_PADDING_TOP: f32 = 6.0;
// `tooltip.border.padding.left`, `tooltip.border.padding.right`: platform-facts.md (§2.7): WinUI3
// ToolTipBorderPadding=9,6,9,8
const TOOLTIP_PADDING_HORIZONTAL: f32 = 9.0;
// `tooltip.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const TOOLTIP_PADDING_BOTTOM: f32 = 8.0;
// `tooltip.max_width`: 0.7.2 windows-11.toml (uncited)
const TOOLTIP_MAX_WIDTH: f32 = 320.0;

// `input.min_height`: 0.7.2 windows-11.toml (uncited)
const INPUT_MIN_HEIGHT: f32 = 32.0;
// `input.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const INPUT_PADDING_TOP: f32 = 5.0;
// `input.border.padding.right`: platform-facts.md (§2.4): WinUI3 TextControlThemePadding=10,5,6,6
const INPUT_PADDING_RIGHT: f32 = 6.0;
// `input.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const INPUT_PADDING_BOTTOM: f32 = 6.0;
// `input.border.padding.left`: 0.7.2 windows-11.toml (uncited)
const INPUT_PADDING_LEFT: f32 = 10.0;
// `text_area.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const TEXT_AREA_PADDING_TOP: f32 = 5.0;
// `text_area.border.padding.right`: platform-facts.md (§2.29): the same TextBox,
// TextControlThemePadding=10,5,6,6
const TEXT_AREA_PADDING_RIGHT: f32 = 6.0;
// `text_area.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const TEXT_AREA_PADDING_BOTTOM: f32 = 6.0;
// `text_area.border.padding.left`: 0.7.2 windows-11.toml (uncited)
const TEXT_AREA_PADDING_LEFT: f32 = 10.0;

// `link.underline_enabled`: platform-facts.md (§2.28): an inline Hyperlink is underlined at rest
const LINK_DECORATION: DecorationKind = DecorationKind::Underline;

// `slider.track_height`: 0.7.2 windows-11.toml (uncited)
const SLIDER_TRACK_HEIGHT: f32 = 4.0;
// `slider.thumb_diameter`: 0.7.2 windows-11.toml (uncited)
const SLIDER_THUMB_DIAMETER: f32 = 18.0;
// `slider.tick_mark_length`: 0.7.2 windows-11.toml (uncited)
const SLIDER_TICK_LENGTH: f32 = 4.0;

// `progress_bar.track_height`: 0.7.2 windows-11.toml (uncited)
const PROGRESS_BAR_TRACK_HEIGHT: f32 = 1.0;
// `progress_bar.min_width`: 0.7.2 windows-11.toml (uncited)
const PROGRESS_BAR_MIN_WIDTH: f32 = 100.0;
// `progress_bar.border.corner_radius`: platform-facts.md (§2.10): the control's CornerRadius,
// ProgressBarCornerRadius = 1.5
const PROGRESS_BAR_CORNER_RADIUS: f32 = 1.5;
// `progress_bar.border.line_width`: platform-facts.md (§2.10): ProgressBarBorderThemeThickness = 0
// (1 in high contrast)
const PROGRESS_BAR_BORDER_WIDTH: f32 = 0.0;
// `progress_bar.border.padding.left`, `progress_bar.border.padding.right`: platform-facts.md
// (§2.10): 0; 0, ProgressBarRoot pads by the control's Padding, which the style leaves at
// Control.Padding's default 0 (ProgressBar.xaml:155, :3-12)
const PROGRESS_BAR_PADDING_HORIZONTAL: f32 = 0.0;
// `progress_bar.border.padding.top`, `progress_bar.border.padding.bottom`: 0.7.2 windows-11.toml
// (uncited)
const PROGRESS_BAR_PADDING_VERTICAL: f32 = 0.0;

// `splitter.divider_width`: 0.7.2 windows-11.toml (uncited)
const SPLITTER_DIVIDER_WIDTH: f32 = 1.0;

// `spinner.diameter`: 0.7.2 windows-11.toml (uncited)
const SPINNER_DIAMETER: f32 = 32.0;
// `spinner.min_diameter`: 0.7.2 windows-11.toml (uncited)
const SPINNER_MIN_DIAMETER: f32 = 16.0;
// `spinner.stroke_width`: 0.7.2 windows-11.toml (uncited)
const SPINNER_STROKE_WIDTH: f32 = 4.0;

// `menu.icon_text_gap`: 0.7.2 windows-11.toml (uncited)
const MENU_ICON_TEXT_GAP: f32 = 12.0;
// `menu.icon_size`: platform-facts.md (§2.6): SM_CXSMICON 16
const MENU_ICON_SIZE: f32 = 16.0;
// `menu.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const MENU_ITEM_PADDING_TOP: f32 = 4.0;
// `menu.border.padding.left`, `menu.border.padding.right`: platform-facts.md (§2.6): WinUI3 11;
// mouse context 4 top / 5 bottom
const MENU_ITEM_PADDING_HORIZONTAL: f32 = 11.0;
// `menu.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const MENU_ITEM_PADDING_BOTTOM: f32 = 5.0;

// `tab.min_width`: platform-facts.md (§2.11): TabWidthMode Equal clamps a tab to
// TabViewItemMinWidth 100
const TAB_MIN_WIDTH: f32 = 100.0;
// `tab.min_height`: 0.7.2 windows-11.toml (uncited)
const TAB_MIN_HEIGHT: f32 = 32.0;
// `tab.item_gap`: platform-facts.md (§2.11): TabView stacks its items with no margin
// (TabView.xaml:76-82, :292-300)
const TAB_ITEM_GAP: f32 = 0.0;
// `tab.active_outward_corner_radius`: platform-facts.md (§2.11): the selected tab's bottom corners
// curve out by 4 (TabViewItem.cpp:106 at microsoft-ui-xaml 8463f45)
const TAB_OUTWARD_CORNER_RADIUS: f32 = 4.0;
// `tab.border.corner_radius`: 0.7.2 windows-11.toml (uncited)
const TAB_CORNER_RADIUS: f32 = 8.0;
// `tab.border.line_width`: platform-facts.md (§2.11): 1, OverlayCornerRadius 8 on the top corners,
// no shadow
const TAB_BORDER_WIDTH: f32 = 1.0;
// `tab.border.padding.top`, `tab.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const TAB_PADDING_VERTICAL: f32 = 3.0;
// `tab.border.padding.left`, `tab.border.padding.right`: platform-facts.md (§2.11): WinUI3
// without-close-button context, TabViewItemHeaderPaddingWithoutCloseButton=8,3,8,3
const TAB_PADDING_HORIZONTAL: f32 = 8.0;

// `sidebar.border.padding.left`, `sidebar.border.padding.right`: platform-facts.md (§2.12):
// (derived) 3, the pane's PaneContentGrid sits NavigationViewPaneContentGridMargin -1,3 inside it
// and each item lays its fill NavigationViewItemButtonMargin 4,2 inside that: -1 + 4
// (microsoft-ui-xaml 8463f45, NavigationView.xaml:291, NavigationView_themeresources.xaml:233,
// :228, :451); the vertical has no one value (:1381), so it is unstated
const SIDEBAR_PADDING_HORIZONTAL: f32 = 3.0;

// `toolbar.bar_height`: platform-facts.md (§2.13): WinUI3 CommandBar compact (default) mode,
// AppBarThemeCompactHeight = 48
const TOOLBAR_BAR_HEIGHT: f32 = 48.0;
// `toolbar.item_gap`: platform-facts.md (§2.13): WinUI3 0
const TOOLBAR_ITEM_GAP: f32 = 0.0;
// `toolbar.border.corner_radius`: 0.7.2 windows-11.toml (uncited)
const TOOLBAR_CORNER_RADIUS: f32 = 0.0;
// `toolbar.border.line_width`: platform-facts.md §2.13 border.line_width, border.corner_radius: 0
// at rest, a closed CommandBar's ContentRoot has no border and its OpenBorder is collapsed;
// ControlCornerRadius reaches ContentRoot only open (CommandBar_themeresources.xaml:795, :842,
// :559-560)
const TOOLBAR_BORDER_WIDTH: f32 = 0.0;
// `toolbar.border.padding.top`, `toolbar.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const TOOLBAR_PADDING_VERTICAL: f32 = 0.0;
// `toolbar.border.padding.right`: platform-facts.md (§2.13): WinUI3 CommandBar Padding=4,0,0,0
const TOOLBAR_PADDING_RIGHT: f32 = 0.0;
// `toolbar.border.padding.left`: 0.7.2 windows-11.toml (uncited)
const TOOLBAR_PADDING_LEFT: f32 = 4.0;

// `popover.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const POPOVER_PADDING_TOP: f32 = 15.0;
// `popover.border.padding.left`, `popover.border.padding.right`: platform-facts.md (§2.16): WinUI3
// FlyoutContentPadding=16,15,16,17
const POPOVER_PADDING_HORIZONTAL: f32 = 16.0;
// `popover.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const POPOVER_PADDING_BOTTOM: f32 = 17.0;

// `switch.track_width`: 0.7.2 windows-11.toml (uncited)
const SWITCH_TRACK_WIDTH: f32 = 40.0;
// `switch.track_height`: 0.7.2 windows-11.toml (uncited)
const SWITCH_TRACK_HEIGHT: f32 = 20.0;
// `switch.track_radius`: 0.7.2 windows-11.toml (uncited)
const SWITCH_TRACK_RADIUS: f32 = 10.0;
// `switch.thumb_diameter`: 0.7.2 windows-11.toml (uncited)
const SWITCH_THUMB_DIAMETER: f32 = 12.0;
// `switch.track_border_line_width`: platform-facts.md (§2.21): ToggleSwitchOuterBorderStrokeThickness
// 1; the on track's stroke is drawn at ToggleSwitchOnStrokeThickness 0, so
// checked_track_border_color stays unset (platform-facts.md)
const SWITCH_TRACK_BORDER_WIDTH: f32 = 1.0;
// `switch.label_gap`: the template's columns, the switch in column 0, then
// `<ColumnDefinition Width="12" MaxWidth="12" />`, the On and Off content in column 2
// (ToggleSwitch_themeresources.xaml:499-503, :505-507 at microsoft-ui-xaml 8463f45)
const SWITCH_LABEL_GAP: f32 = 12.0;

// `dialog.min_width`: 0.7.2 windows-11.toml (uncited)
const DIALOG_MIN_WIDTH: f32 = 320.0;
// `dialog.max_width`: 0.7.2 windows-11.toml (uncited)
const DIALOG_MAX_WIDTH: f32 = 548.0;
// `dialog.min_height`: 0.7.2 windows-11.toml (uncited)
const DIALOG_MIN_HEIGHT: f32 = 184.0;
// `dialog.max_height`: 0.7.2 windows-11.toml (uncited)
const DIALOG_MAX_HEIGHT: f32 = 756.0;
// `dialog.button_gap`: 0.7.2 windows-11.toml (uncited)
const DIALOG_BUTTON_GAP: f32 = 8.0;
// `dialog.icon_size`: 0.7.2 windows-11.toml (uncited)
const DIALOG_ICON_SIZE: f32 = 32.0;
// `dialog.button_order`: Windows uses primary-leftmost per MS Common Buttons guideline (Win7) and
// modern WinUI 3 ContentDialog. See docs/platform-facts.md.
const DIALOG_BUTTON_ORDER: DialogButtonOrder = DialogButtonOrder::PrimaryLeft;
// `dialog.entrance`: platform-facts.md (§2.22): ContentDialog scales 1.05 → 1 while fading in
// (ContentDialog_themeresources.xaml:96-114)
const DIALOG_ENTRANCE: DialogEntranceStyle = DialogEntranceStyle::ScaleFade;
// `dialog.title_font.size`, `dialog.title_font.defined_size`: 0.7.2 windows-11.toml (uncited)
const DIALOG_TITLE_SIZE_PT: f32 = 15.0;
// `dialog.title_font.weight`: 0.7.2 windows-11.toml (uncited)
const DIALOG_TITLE_WEIGHT: u16 = 600;
// `dialog.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const DIALOG_PADDING_TOP: f32 = 24.0;
// `dialog.border.padding.left`, `dialog.border.padding.right`: platform-facts.md (§2.22): WinUI3
// ContentDialogPadding=24
const DIALOG_PADDING_HORIZONTAL: f32 = 24.0;
// `dialog.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const DIALOG_PADDING_BOTTOM: f32 = 24.0;

// `combo_box.min_height`: 0.7.2 windows-11.toml (uncited)
const COMBO_BOX_MIN_HEIGHT: f32 = 32.0;
// `combo_box.min_width`: 0.7.2 windows-11.toml (uncited)
const COMBO_BOX_MIN_WIDTH: f32 = 64.0;
// `combo_box.arrow_icon_size`: 0.7.2 windows-11.toml (uncited)
const COMBO_BOX_ARROW_SIZE: f32 = 12.0;
// `combo_box.arrow_area_width`: 0.7.2 windows-11.toml (uncited)
const COMBO_BOX_ARROW_AREA_WIDTH: f32 = 38.0;
// `combo_box.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const COMBO_BOX_PADDING_TOP: f32 = 5.0;
// `combo_box.border.padding.right`: 0.7.2 windows-11.toml (uncited)
const COMBO_BOX_PADDING_RIGHT: f32 = 0.0;
// `combo_box.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const COMBO_BOX_PADDING_BOTTOM: f32 = 7.0;
// `combo_box.border.padding.left`: platform-facts.md (§2.24): WinUI3 ComboBoxPadding=12,5,0,7; 0
// right, where the text column meets the 38px arrow column
const COMBO_BOX_PADDING_LEFT: f32 = 12.0;

// `segmented_control.segment_height`: segment_height no source: platform-facts.md gives the
// Toolkit's item no height of its own, its label's line in a 5 / 6 margin; 28 kept
const SEGMENT_HEIGHT: f32 = 28.0;
// `segmented_control.separator_width`: platform-facts.md (§2.25): no separator line, the items 1
// apart on the group's fill
const SEGMENT_SEPARATOR_WIDTH: f32 = 0.0;
// `segmented_control.disabled_opacity`: platform-facts.md (§2.25): a disabled item fades to
// SegmentedItemDisabledOpacity
const SEGMENT_DISABLED_OPACITY: f32 = 0.55;
// `segmented_control.border.padding.top`: 0.7.2 windows-11.toml (uncited)
const SEGMENT_PADDING_TOP: f32 = 5.0;
// `segmented_control.border.padding.left`, `segmented_control.border.padding.right`:
// platform-facts.md (§2.25): an item's content margin, 11 across, 5 above and 6 below
// (SegmentedItem.xaml:629, :649)
const SEGMENT_PADDING_HORIZONTAL: f32 = 11.0;
// `segmented_control.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const SEGMENT_PADDING_BOTTOM: f32 = 6.0;

// `card.border.corner_radius`: 0.7.2 windows-11.toml (uncited)
const CARD_CORNER_RADIUS: f32 = 8.0;
// `card.border.line_width`: platform-facts.md (§2.26): 1px, OverlayCornerRadius 8, border only
const CARD_BORDER_WIDTH: f32 = 1.0;
// `card.border.padding.left`, `card.border.padding.right`: platform-facts.md (§2.26): 12
// (convention)
const CARD_PADDING_HORIZONTAL: f32 = 12.0;
// `card.border.padding.top`, `card.border.padding.bottom`: 0.7.2 windows-11.toml (uncited)
const CARD_PADDING_VERTICAL: f32 = 12.0;

// `expander.header_height`: 0.7.2 windows-11.toml (uncited)
const EXPANDER_HEADER_HEIGHT: f32 = 48.0;
// `expander.arrow_icon_size`: 0.7.2 windows-11.toml (uncited)
const EXPANDER_ARROW_SIZE: f32 = 12.0;
// `expander.arrow_side`: platform-facts.md (§2.27): the chevron at the row's end, its 12px glyph 20
// + (32 - 12) / 2 from the content, the content padded 16, header and content framed
const EXPANDER_ARROW_SIDE: ArrowSide = ArrowSide::Trailing;
// `expander.arrow_gap`: 0.7.2 windows-11.toml (uncited)
const EXPANDER_ARROW_GAP: f32 = 30.0;
// `expander.content_indent`: 0.7.2 windows-11.toml (uncited)
const EXPANDER_CONTENT_INDENT: f32 = 16.0;
// `expander.frame_enabled`: 0.7.2 windows-11.toml (uncited)
const EXPANDER_FRAME_ENABLED: bool = true;
// `expander.border.padding.top`, `expander.border.padding.bottom`: 0.7.2 windows-11.toml
// (uncited)
const EXPANDER_PADDING_VERTICAL: f32 = 0.0;
// `expander.border.padding.right`: platform-facts.md (§2.27): WinUI3 header context,
// ExpanderHeaderPadding=16,0,0,0
const EXPANDER_PADDING_RIGHT: f32 = 0.0;
// `expander.border.padding.left`: 0.7.2 windows-11.toml (uncited)
const EXPANDER_PADDING_LEFT: f32 = 16.0;

/// The state precedence of a part with rules for pressed and hover: pressed
/// over hover.
const PRESSED_HOVER: &[&[WidgetState]] = &[&[WidgetState::PRESSED, WidgetState::HOVER]];

/// The font DPI a point size converts at: 0.7.2's on every desktop but
/// macOS (`ResolutionContext::from_system`, `src/resolve/context.rs`; spec
/// §4, `Typography::font_dpi`).
const FONT_DPI: f32 = 96.0;

/// The default border's opacity under high contrast: 1, the border being
/// `CardStrokeColorDefaultBrush` = `SystemColorWindowTextColor`, the text
/// colour itself (`Common_themeresources_any.xaml:464`; platform-facts.md
/// (§2.1.3), `border.color`; 0.7.2's `windows::apply_sys_colors`).
const HIGH_CONTRAST_BORDER_OPACITY: f32 = 1.0;

/// The progress bar's outline under high contrast:
/// `ProgressBarBorderThemeThickness` is 1 in `HighContrast`
/// (`ProgressBar_themeresources.xaml:13`; platform-facts.md (§2.10),
/// `border.line_width`; 0.7.2's `windows::apply_sys_colors`).
const HIGH_CONTRAST_PROGRESS_BAR_BORDER_WIDTH: f32 = 1.0;

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

/// The least contrast ratio of the text on an explicit accent's fill in the
/// scheme's own `TextOnAccentFillColorPrimary` ([`text_on_accent`]): the
/// minimum of spec §11, item 5 (WCAG 2.2's for large text and user-interface
/// components, SC 1.4.3 and 1.4.11).
const MIN_TEXT_ON_ACCENT_CONTRAST: f32 = 3.0;

/// The theme of `windows-11` for `ctx.scheme`, from the preset's constants
/// with the system's settings `inputs.system` laid over them, as the 0.7.2
/// reader writes them over the preset (spec §8, "The Windows part"; 0.7.2's
/// `windows::build_theme` and `apply_sys_colors`): each setting applies in
/// whichever scheme is resolved (spec §8, "Every reader"); the accent's
/// values of [`lay_accent_over`]; the fonts, the tooltip's `tooltip_font`,
/// else `status_bar_font`; the scroll bar's sizes, the border's width, the
/// icons' small and large sizes and the caret's times; and, where
/// `Contrast::High` is resolved, the high-contrast mapping of
/// [`HighContrast`] in place of the accent's values; then the active palette
/// ([`lay_palette_over`]) and the explicit accent ([`lay_explicit_accent_over`]),
/// each over the layer below it (spec §6, resolution step 2).
pub(crate) fn resolve(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let system = inputs.system;
    let (preset_colors, preset_border_opacity) = match ctx.scheme {
        Scheme::Light => (LIGHT, BORDER_OPACITY_LIGHT),
        Scheme::Dark => (DARK, BORDER_OPACITY_DARK),
    };
    let high_contrast = HighContrast::of(system, ctx);
    let mut hc = high_contrast.unwrap_or_default();
    let mut scheme_colors = preset_colors;
    let mut link_pressed = preset_colors.accent_fill;
    if let Some(system) = system {
        link_pressed = lay_accent_over(
            &mut scheme_colors,
            system,
            ctx.scheme,
            high_contrast.is_some(),
        );
    }
    hc.lay_over(&mut scheme_colors);
    hc.focused_outline = high_contrast.map(|_| scheme_colors.accent_fill);
    let explicit_accent = inputs.accent.is_some();
    if let Some(p) = inputs.palette {
        lay_palette_over(&mut scheme_colors, p, explicit_accent);
    }
    if let Some(accent) = inputs.accent {
        lay_explicit_accent_over(&mut scheme_colors, accent, ctx.scheme);
    }
    let c = &scheme_colors;
    let colors = roles(c, &hc, inputs.palette, explicit_accent);
    // A palette's link has no hovered or pressed colour of its own.
    let link_states = inputs
        .palette
        .is_none()
        .then_some((c.link_hover, link_pressed));
    let text = colors.window.foreground.unwrap_or(c.window_foreground);
    let typography = typography(text, system, ctx.text_scale);
    let units = FontUnits {
        dpi: typography.font_dpi,
        text_scale: ctx.text_scale,
    };
    // No source the builder reads states a policy of Windows' for reduced motion: `Reduced`
    // runs as `Full`.
    let motion = motion(ctx.motion, None);
    let layout = LayoutMetrics {
        widget_gap: Some(WIDGET_GAP),
        container_margin: Some(CONTAINER_MARGIN),
        window_margin: Some(WINDOW_MARGIN),
        section_gap: Some(SECTION_GAP),
        corner_radius: CORNER_RADIUS,
        corner_radius_lg: CORNER_RADIUS_LG,
        border_width: system
            .and_then(|s| s.border_width_px)
            .unwrap_or(BORDER_WIDTH),
        border_opacity: if hc.window_text.is_some() {
            HIGH_CONTRAST_BORDER_OPACITY
        } else {
            preset_border_opacity
        },
        shadow_enabled: SHADOW_ENABLED,
        cursor_size_px: None,
        cursor_theme: None,
    };
    let fonts = SystemFonts::of(system);
    let direction = ctx.direction;
    ResolvedTheme {
        name: Arc::from(Preset::Windows11.display_name()),
        preset: Preset::Windows11,
        context: ctx.clone(),
        icons: IconThemeSpec {
            set: ICON_SET,
            theme: Some(Arc::from(ICON_THEME)),
            // 0.7.2's `Resolved::icon_theme_explicit`: the TOML states the theme
            // (`src/model/resolved.rs`).
            theme_explicit: true,
            // The sizes the system states, each in place of the preset's (Windows:
            // `SM_CXSMICON` and `SM_CXICON`, 0.7.2's `defaults.icon_sizes.small` and
            // `large`), in every scheme.
            sizes: {
                let os = system.and_then(|s| s.icon_sizes);
                let size = |pick: fn(&IconSizes) -> Option<f32>, preset| {
                    Some(os.as_ref().and_then(pick).unwrap_or(preset))
                };
                IconSizes::new(
                    size(|s| s.toolbar, ICON_TOOLBAR),
                    size(|s| s.small, ICON_SMALL),
                    size(|s| s.large, ICON_LARGE),
                    size(|s| s.dialog, ICON_DIALOG),
                    size(|s| s.panel, ICON_PANEL),
                )
            },
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
        // 0.7.2's `defaults.focus_ring_*`: a ring, the Button's; 0.7.2 has no colour for its inner
        // ring (platform-facts.md (§2.1.5), `focus_ring_offset`), 0.8.0 states it.
        focus_ring: focus_ring(c, &colors, FOCUS_RING_OFFSET),
        caret: CaretSpec {
            width: None,
            blink_on_ms: system
                .and_then(|s| s.caret_blink_on_ms)
                .or(Some(CARET_BLINK_ON_MS)),
            blink_off_ms: system
                .and_then(|s| s.caret_blink_off_ms)
                .or(Some(CARET_BLINK_OFF_MS)),
            blink_timeout_ms: system.and_then(|s| s.caret_blink_timeout_ms),
            shape: None,
        },
        elevations: Arc::from([]),
        button: button(c, &hc, &colors, &typography, &layout),
        checkbox: checkbox(c, &hc, &colors, &typography, &layout),
        radio: radio(c, &hc, &colors, &typography, &layout),
        scrollbar: scrollbar(
            c,
            &colors,
            system
                .and_then(|s| s.scrollbar_width_px)
                .unwrap_or(SCROLLBAR_GROOVE_WIDTH),
            system
                .and_then(|s| s.scrollbar_min_thumb_length_px)
                .unwrap_or(SCROLLBAR_MIN_THUMB_LENGTH),
        ),
        list: list(c, &hc, &colors, &typography, &layout, direction),
        tab: tab(c, &colors, &typography),
        menu: menu(c, &hc, &colors, &typography, units, fonts.menu),
        menu_bar: menu_bar(&colors),
        sidebar: sidebar(c, &colors, &typography, &layout),
        toolbar: toolbar(c, &hc, &colors, &typography, direction),
        status_bar: status_bar(&colors, &typography, &layout, units, fonts.status_bar),
        tooltip: tooltip(&colors, &typography, units, &layout, fonts.tooltip),
        window: window(
            c,
            &hc,
            &colors,
            laid_over(label(&typography.body, None), fonts.title_bar, units),
            &layout,
            ctx.window_active,
        ),
        input: input(
            c,
            &hc,
            &colors,
            &typography,
            &layout,
            physical_edges(
                Edges::new(
                    INPUT_PADDING_TOP,
                    INPUT_PADDING_RIGHT,
                    INPUT_PADDING_BOTTOM,
                    INPUT_PADDING_LEFT,
                ),
                direction,
            ),
            Some(INPUT_MIN_HEIGHT),
        ),
        text_area: input(
            c,
            &hc,
            &colors,
            &typography,
            &layout,
            physical_edges(
                Edges::new(
                    TEXT_AREA_PADDING_TOP,
                    TEXT_AREA_PADDING_RIGHT,
                    TEXT_AREA_PADDING_BOTTOM,
                    TEXT_AREA_PADDING_LEFT,
                ),
                direction,
            ),
            None,
        ),
        link: link(c, &colors, &typography, link_states),
        slider: slider(c, &colors),
        progress_bar: progress_bar(c, &hc, &colors),
        splitter: splitter(&colors),
        separator: separator(c, &colors, &layout),
        spinner: spinner(c),
        popover: popover(c, &colors, &typography, &layout),
        switch: switch(c, &hc, &typography),
        dialog: dialog(c, &colors, &typography, units, &layout, motion),
        combo_box: combo_box(c, &colors, &typography, &layout, direction),
        segmented_control: segmented_control(c, &hc, &typography, &layout),
        card: card(c, &colors),
        expander: expander(c, &colors, &typography, &layout, direction),
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
        colors,
        typography,
        layout,
    }
}

/// The system colours WinUI 3's `HighContrast` theme dictionary draws with,
/// as the system's settings state them for the resolved scheme at
/// `Contrast::High` (spec §8, "The Windows part", *High contrast*): the
/// window's (`COLOR_WINDOW`, `COLOR_WINDOWTEXT`), the button's
/// (`COLOR_BTNFACE`, `COLOR_BTNTEXT`) and the selection's
/// (`COLOR_HIGHLIGHT`, `COLOR_HIGHLIGHTTEXT`) colours, the link
/// (`COLOR_HOTLIGHT`) and the disabled text (`COLOR_GRAYTEXT`), each `None`
/// where not stated: that key keeps what it has without high contrast.
///
/// Each field a widget draws takes the system colour the dictionary gives its
/// brush, as 0.7.2's `windows::apply_sys_colors` writes it (each key's
/// brush cited there, microsoft-ui-xaml 8463f45): `COLOR_WINDOW` the
/// window's, the view's, the menu's, the tooltip's and the title bar's
/// backgrounds and the progress bar's groove; `COLOR_WINDOWTEXT` the text,
/// the informational fill, the focus ring, the border (at an opacity of 1,
/// [`HIGH_CONTRAST_BORDER_OPACITY`]), the placeholder, the tooltip's text and
/// the progress bar's outline (1 px wide,
/// [`HIGH_CONTRAST_PROGRESS_BAR_BORDER_WIDTH`]); `COLOR_BTNFACE` the push
/// button's, the text field's and the segmented control's fills and the
/// hovered on switch; `COLOR_BTNTEXT` the push button's, the text field's,
/// the menu's, the title's and the segmented control's text;
/// `COLOR_HIGHLIGHT` the selection, the accent and every accent fill, the
/// list's selected row, the segmented control's outline and selected
/// segment and the focused text field's outline; `COLOR_HIGHLIGHTTEXT` the
/// text on them and the hovered unchecked box and circle; `COLOR_HOTLIGHT`
/// the link; `COLOR_GRAYTEXT` the disabled text, the inactive title and a
/// disabled checked or mixed box's fill and outline, a disabled checked
/// circle's outline and its dot; `COLOR_WINDOW` also a disabled checked or
/// mixed box's mark and a disabled checked circle's fill ([`checkbox`],
/// [`radio`]).
#[derive(Clone, Copy, Default)]
struct HighContrast {
    /// `COLOR_WINDOW`.
    window: Option<Color>,
    /// `COLOR_WINDOWTEXT`.
    window_text: Option<Color>,
    /// `COLOR_BTNFACE`.
    button_face: Option<Color>,
    /// `COLOR_BTNTEXT`.
    button_text: Option<Color>,
    /// `COLOR_HIGHLIGHT`.
    highlight: Option<Color>,
    /// `COLOR_HIGHLIGHTTEXT`.
    highlight_text: Option<Color>,
    /// `COLOR_HOTLIGHT`.
    hotlight: Option<Color>,
    /// `COLOR_GRAYTEXT`.
    gray_text: Option<Color>,
    /// The focused text field's outline: under high contrast the accent's
    /// fill as the mapping leaves it, which a palette's or an explicit
    /// accent does not replace (the accent's fill is that outline outside
    /// high contrast alone, spec §6, step 2).
    focused_outline: Option<Color>,
}

impl HighContrast {
    /// The high-contrast colours of `system` for `ctx`: `None` unless
    /// `Contrast::High` is resolved, which the preset defines only through
    /// the system's colours for it ([`CONTRAST_LEVELS`]; spec §6, resolution
    /// step 1), and the system states colours for the scheme at that level.
    fn of(system: Option<&SystemOverrides>, ctx: &ThemeContext) -> Option<Self> {
        if ctx.contrast != Contrast::High {
            return None;
        }
        let colors = system?.colors_for(ctx.scheme, Contrast::High)?;
        let surfaces = colors.surfaces.as_ref();
        let window = surfaces.and_then(|s| s.window);
        let button = surfaces.and_then(|s| s.button);
        Some(Self {
            window: window.and_then(|s| s.background),
            window_text: window.and_then(|s| s.foreground),
            button_face: button.and_then(|s| s.background),
            button_text: button.and_then(|s| s.foreground),
            highlight: colors.selection.and_then(|s| s.background),
            highlight_text: colors.selection.and_then(|s| s.foreground),
            hotlight: colors.link,
            gray_text: colors.disabled_text,
            focused_outline: None,
        })
    }

    /// Lay the colours over each of `c`'s fields whose every key takes the
    /// same system colour: the view's background, the menu's and the focus
    /// visual's inner ring (`COLOR_WINDOW`: `FocusStrokeColorInnerBrush` =
    /// `SystemColorWindowColor`, Common_themeresources_any.xaml:473 at
    /// microsoft-ui-xaml 8463f45); the accent, the accent's fill and the segmented
    /// control's outline (`COLOR_HIGHLIGHT`); the selection's text and the
    /// hovered unchecked box and circle (`COLOR_HIGHLIGHTTEXT`:
    /// `CheckBoxCheckBackgroundFillUncheckedPointerOver`,
    /// `RadioButtonOuterEllipseFillPointerOver` =
    /// `SystemColorHighlightTextColorBrush`, CheckBox_themeresources.xaml:142,
    /// RadioButton_themeresources.xaml:81 at microsoft-ui-xaml 8463f45); the
    /// focus ring and the informational fill
    /// (`COLOR_WINDOWTEXT`); the link (`COLOR_HOTLIGHT`); the hovered on
    /// switch (`COLOR_BTNFACE`); the inactive title (`COLOR_GRAYTEXT`). A
    /// field whose keys take different colours, or some none, the widgets
    /// and [`roles`] pick key by key.
    fn lay_over(&self, c: &mut SchemeColors) {
        let lay = |field: &mut Color, color: Option<Color>| {
            if let Some(color) = color {
                *field = color;
            }
        };
        lay(&mut c.view_background, self.window);
        lay(&mut c.menu_background, self.window);
        lay(&mut c.focus_ring_inner, self.window);
        lay(&mut c.accent, self.highlight);
        lay(&mut c.accent_fill, self.highlight);
        lay(&mut c.control_stroke, self.highlight);
        lay(&mut c.selection_foreground, self.highlight_text);
        lay(&mut c.checkbox_hover, self.highlight_text);
        lay(&mut c.focus_ring, self.window_text);
        lay(&mut c.info, self.window_text);
        lay(&mut c.link, self.hotlight);
        if let Some(face) = self.button_face {
            c.switch_hover_on = Some(face);
        }
        lay(&mut c.inactive_title, self.gray_text);
    }
}

/// Lay the system's accent settings over `c` for `scheme`, as 0.7.2's
/// `windows::build_theme` writes them (spec §8, "The Windows part",
/// *Derived by the `windows-11` builder*; the WinUI resources cited there):
/// the accent (`UIColorType::Accent`); the accent's fill
/// (`AccentFillColorDefault`), `AccentLight2` in dark and `AccentDark1` in
/// light, the accent where that shade is not stated; the link
/// (`AccentTextFillColorPrimary`), `AccentLight3` in dark and `AccentDark2`
/// in light; the informational fill (`SystemFillColorAttention`),
/// `AccentLight2` in dark and the accent in light; and, outside high
/// contrast, the link's hovered colour, `AccentLight3` in dark and
/// `AccentDark3` in light. A value whose setting is not stated keeps `c`'s.
/// Returns the link's pressed colour: outside high contrast `AccentLight2` in
/// dark and `AccentDark1` in light, where stated; else the preset's (`c`'s
/// accent fill before the accent is laid over it, the key's value in
/// `windows-11.toml`).
fn lay_accent_over(
    c: &mut SchemeColors,
    system: &SystemOverrides,
    scheme: Scheme,
    high_contrast: bool,
) -> Color {
    let preset_pressed = c.accent_fill;
    let accent = system.accent;
    let ramp = system.windows_accent_ramp;
    let [_, light2, light3] = ramp.map_or([None; 3], |r| r.light);
    let [dark1, dark2, dark3] = ramp.map_or([None; 3], |r| r.dark);
    let dark = scheme == Scheme::Dark;
    let pick = |in_dark: Option<Color>, in_light: Option<Color>| {
        if dark { in_dark } else { in_light }
    };
    if let Some(accent) = accent {
        c.accent = accent;
    }
    if let Some(fill) = pick(light2, dark1).or(accent) {
        c.accent_fill = fill;
    }
    if let Some(link) = pick(light3, dark2) {
        c.link = link;
    }
    if let Some(info) = pick(light2, accent) {
        c.info = info;
    }
    if high_contrast {
        return preset_pressed;
    }
    if let Some(hover) = pick(light3, dark3) {
        c.link_hover = hover;
    }
    pick(light2, dark1).unwrap_or(preset_pressed)
}

/// Lay the palette `p` over `c`: each slot whose WinUI colour has its name
/// replaces that colour, as the module documentation lists, and the accent's
/// slots are written as given ([`lay_explicit_accent_over`]'s roles), unless
/// an explicit accent (`explicit_accent`) replaces them; the slots that state
/// only their roles are read by [`roles`] and the widgets.
fn lay_palette_over(c: &mut SchemeColors, p: &Palette, explicit_accent: bool) {
    c.window_background = p.window_bg;
    c.view_background = p.view_bg;
    c.sidebar_background = p.sidebar_bg;
    c.window_foreground = p.foreground;
    c.window_foreground_muted = p.foreground_muted;
    c.window_border = p.border;
    c.link = p.link;
    c.error = p.error;
    c.warning = p.warning;
    c.success = p.success;
    if explicit_accent {
        return;
    }
    c.accent = p.accent;
    c.accent_fill = p.accent;
    c.accent_foreground = p.accent_fg;
    c.info = p.info;
    c.selection_foreground = p.selection_fg;
}

/// Lay the explicit accent `accent` over `c` for `scheme`, as given (spec
/// §6, step 2): the accent, the accent's fill and the informational fill,
/// whatever the system's shades; the text on the accent's fill
/// [`text_on_accent`].
fn lay_explicit_accent_over(c: &mut SchemeColors, accent: Color, scheme: Scheme) {
    c.accent = accent;
    c.accent_fill = accent;
    c.info = accent;
    c.accent_foreground = text_on_accent(accent, scheme);
}

/// The text on the fill `accent` in `scheme`: the scheme's
/// `TextOnAccentFillColorPrimary` (the preset's `accent_foreground`,
/// platform-facts.md (§2.21)) where its contrast ratio on `accent` is at
/// least [`MIN_TEXT_ON_ACCENT_CONTRAST`], else the other scheme's.
fn text_on_accent(accent: Color, scheme: Scheme) -> Color {
    let (own, other) = match scheme {
        Scheme::Light => (LIGHT.accent_foreground, DARK.accent_foreground),
        Scheme::Dark => (DARK.accent_foreground, LIGHT.accent_foreground),
    };
    if own.contrast_ratio(accent) >= MIN_TEXT_ON_ACCENT_CONTRAST {
        own
    } else {
        other
    }
}

/// The system's fonts the widgets take: the menu's (`menu_font`), the
/// title's (`title_bar_font`), the status bar's (`status_bar_font`) and the
/// tooltip's (`tooltip_font`, else `status_bar_font`: `lfStatusFont` is "the
/// font used in status bars and tooltips", `NONCLIENTMETRICSW`, as 0.7.2's
/// reader copies it).
#[derive(Clone, Copy, Default)]
struct SystemFonts<'a> {
    /// The menu's.
    menu: Option<&'a OsFont>,
    /// The window title's.
    title_bar: Option<&'a OsFont>,
    /// The status bar's.
    status_bar: Option<&'a OsFont>,
    /// The tooltip's.
    tooltip: Option<&'a OsFont>,
}

impl<'a> SystemFonts<'a> {
    /// The fonts `system` states.
    fn of(system: Option<&'a SystemOverrides>) -> Self {
        let Some(s) = system else {
            return Self::default();
        };
        Self {
            menu: s.menu_font.as_ref(),
            title_bar: s.title_bar_font.as_ref(),
            status_bar: s.status_bar_font.as_ref(),
            tooltip: s.tooltip_font.as_ref().or(s.status_bar_font.as_ref()),
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

/// The colour roles of one scheme from its colours `c`. The window's surface
/// is WinUI's window colours, the view's its card background
/// (`defaults.surface_color`), the tooltip's the tooltip's own colours; the
/// selection is the accent, lettered in the selection's text (0.7.2:
/// `selection_background` is `defaults.accent_color` where unstated,
/// `selection_inactive_background` and `selection_inactive_text_color` the
/// selection's, and the text selection's colours the selection's,
/// `src/resolve/inheritance.rs`, `resolve_defaults_internal`); the caret is
/// the window's text (`input.caret_color`, `resolve_safety_nets`). Under high
/// contrast `hc` gives the roles 0.7.2's `defaults.*` keys whose colours its
/// reader writes there ([`HighContrast`]): the window's background
/// (`background_color`), text (`text_color`) and border (`border.color`),
/// the view's background (`surface_color`), the tooltip's colours, the
/// accent's text (`accent_text_color`), the disabled text and the caret.
///
/// The active palette `palette`, whose colours `c` holds already, states the
/// roles its slots name (spec §7, "The roles a palette states"), over the
/// high-contrast colours too: the eight surfaces with its one text colour,
/// the window's muted text, border and divider, the caret, the four status
/// colours' origin, and, unless an explicit accent (`explicit_accent`)
/// replaces them, the selection's background. The accent's text is `c`'s
/// wherever a palette or an explicit accent states it.
fn roles(
    c: &SchemeColors,
    hc: &HighContrast,
    palette: Option<&Palette>,
    explicit_accent: bool,
) -> ColorRoles {
    let origin = if palette.is_some() {
        RoleOrigin::Custom
    } else {
        RoleOrigin::Native
    };
    let status = |fill| StatusColors {
        fill,
        foreground: c.status_foreground,
        standalone: None,
        origin,
    };
    let unstated = SurfaceColors::default();
    // A colour the palette states stands over the high-contrast one.
    let system = |color: Option<Color>| color.filter(|_| palette.is_none());
    let text = system(hc.window_text).unwrap_or(c.window_foreground);
    let surface = |background: fn(&Palette) -> Color| {
        palette.map_or(unstated, |p| {
            SurfaceColors::new(Some(background(p)), Some(p.foreground), None, None, None)
        })
    };
    let selection_background = palette
        .filter(|_| !explicit_accent)
        .map_or(c.accent, |p| p.selection_bg);
    let accent_foreground = if palette.is_some() || explicit_accent {
        c.accent_foreground
    } else {
        hc.highlight_text.unwrap_or(c.accent_foreground)
    };
    ColorRoles {
        window: SurfaceColors::new(
            Some(system(hc.window).unwrap_or(c.window_background)),
            Some(text),
            Some(c.window_foreground_muted),
            Some(system(hc.window_text).unwrap_or(c.window_border)),
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
            Some(palette.map_or(hc.window.unwrap_or(c.acrylic_fill), |p| p.tooltip_bg)),
            Some(text),
            None,
            None,
            None,
        ),
        osd: unstated,
        selection: SelectionColors::new(
            selection_background,
            c.selection_foreground,
            Some(selection_background),
            Some(c.selection_foreground),
        ),
        text_selection: SelectionColors::new(
            selection_background,
            c.selection_foreground,
            None,
            None,
        ),
        find_highlight: None,
        scrim: None,
        accent: AccentColors {
            fill: c.accent,
            foreground: accent_foreground,
            standalone: None,
        },
        focus_ring: c.focus_ring,
        caret: palette.map_or(text, |p| p.caret),
        link: c.link,
        // 0.7.2: `link.visited_text_color` is `defaults.link_color` where unstated.
        link_visited: None,
        disabled_text: hc.gray_text.unwrap_or(c.disabled_text),
        shadow: c.shadow,
        disabled_opacity: DISABLED_OPACITY,
        error: status(c.error),
        warning: status(c.warning),
        success: status(c.success),
        info: status(c.info),
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
/// scale, linear on Windows (spec §6, resolution step 2), not rounded (0.7.2
/// rounds only where `font_whole_pixels` is stated, `src/resolve/validate.rs`).
fn font_px(size: FontSize, units: FontUnits) -> f32 {
    linear_font_px(size, units.dpi, units.text_scale, None)
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
    line_height_px: f32,
    units: FontUnits,
) -> TextStyle {
    TextStyle {
        size_px: font_px(size, units),
        defined_size: Some(size),
        weight,
        line_height: font_px(FontSize::Px(line_height_px), units),
        stated: TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::LINE_HEIGHT,
        ..TextStyle::default()
    }
}

/// The text styles. The fonts' colour is the window's foreground (0.7.2:
/// `defaults.font.color` is `defaults.text_color`, and
/// `defaults.mono_font`'s is `defaults.font`'s, where unstated:
/// `resolve_defaults_internal`). The caption takes the UI font's weight,
/// which the TOML does not state for it (0.7.2's `resolve_text_scale_entry`);
/// the section heading is `headline`, the dialog title `title1` and the
/// display style `display` (spec §4). The body's line height is the
/// multiplier times its size, in its unit (spec §4; 0.7.2's
/// `resolve_text_scale_entry`, then its conversion in `validate`). Sizes
/// convert at the system's font DPI, else [`FONT_DPI`], and scale by
/// `text_scale` (spec §6, resolution step 2); each keeps its unscaled size as
/// its `defined_size`. The system's UI font (`ui_font`, 0.7.2's
/// `defaults.font` from `lfMessageFont`) is laid over the body, and so its
/// weight is the caption's and its size gives the body's line height. The
/// window's foreground is `text`, the window's role ([`roles`]; under high
/// contrast the window text, 0.7.2's `defaults.text_color`).
fn typography(text: Color, system: Option<&SystemOverrides>, text_scale: f32) -> Typography {
    let units = FontUnits {
        dpi: system.and_then(|s| s.font_dpi).unwrap_or(FONT_DPI),
        text_scale,
    };
    let ui = system.and_then(|s| s.ui_font.as_ref());
    let body_size = FontSize::Pt(FONT_SIZE_PT);
    let mut body = laid_over(
        font(FONT_FAMILY, body_size, FONT_WEIGHT, text, units),
        ui,
        units,
    );
    let line_height = match ui.and_then(|f| f.size).unwrap_or(body_size) {
        FontSize::Pt(size) => FontSize::Pt(LINE_HEIGHT * size),
        FontSize::Px(size) => FontSize::Px(LINE_HEIGHT * size),
    };
    body.line_height = font_px(line_height, units);
    body.stated |= TextStated::LINE_HEIGHT;
    // The monospace font's line height, which no source states: the multiplier times its size
    // (spec §4, `Typography`: `body` and `monospace` follow a slot's rule).
    let mut monospace = font(
        MONO_FAMILY,
        FontSize::Pt(MONO_SIZE_PT),
        MONO_WEIGHT,
        text,
        units,
    );
    monospace.line_height = font_px(FontSize::Pt(LINE_HEIGHT * MONO_SIZE_PT), units);
    monospace.stated |= TextStated::LINE_HEIGHT;
    let caption_weight = body.weight;
    Typography {
        display: Some(text_scale_entry(
            FontSize::Pt(DISPLAY_SIZE_PT),
            DISPLAY_WEIGHT,
            DISPLAY_LINE_HEIGHT_PX,
            units,
        )),
        title1: Some(text_scale_entry(
            FontSize::Pt(TITLE1_SIZE_PT),
            TITLE1_WEIGHT,
            TITLE1_LINE_HEIGHT_PX,
            units,
        )),
        title2: None,
        title3: None,
        title4: None,
        headline: Some(text_scale_entry(
            FontSize::Pt(HEADLINE_SIZE_PT),
            HEADLINE_WEIGHT,
            HEADLINE_LINE_HEIGHT_PX,
            units,
        )),
        body,
        callout: None,
        subheadline: None,
        caption: Some(text_scale_entry(
            FontSize::Pt(CAPTION_SIZE_PT),
            caption_weight,
            CAPTION_LINE_HEIGHT_PX,
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

/// A part's outline in `color`, inside its edge, `width` wide, the fill under
/// the whole of it.
fn stroke(width: f32, color: Color) -> PartProp {
    PartProp::Stroke(Stroke::uniform(width, color))
}

/// A part's outline in `color`, `width` wide, the fill ending at its inner
/// edge (WinUI's `InnerBorderEdge`; the module documentation).
fn outside_stroke(width: f32, color: Color) -> PartProp {
    PartProp::Stroke(Stroke::uniform_aligned(width, color, StrokeAlign::Outside))
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

/// `label` at the size `size`, which it keeps as its `defined_size`.
fn sized(mut label: TextStyle, size: FontSize, units: FontUnits) -> TextStyle {
    label.size_px = font_px(size, units);
    label.defined_size = Some(size);
    label
}

/// The push button: `ControlFillColorDefault` in the default border, its
/// colour, radius and width (0.7.2's `resolve_border_inheritance`; the
/// default shadow, which is on), outside the fill (`InnerBorderEdge`),
/// padded, of least sizes; its hovered, pressed and disabled fills, checked
/// the accent's fill; the primary variant filled with the accent's fill, its
/// outline inside its edge (`OuterBorderEdge`), lettered in the accent's text
/// (0.7.2: `primary_text_color` is `defaults.accent_text_color` where
/// unstated). Its label is the window's text, pressed the muted text, checked
/// the accent's text, disabled the disabled text. Under high contrast the
/// fill is the button face and the label and the outline the button text
/// ([`HighContrast`]), hovered and pressed the highlight text behind the
/// highlight, the outline hovered the highlight, pressed the highlight text
/// and disabled the grey text.
/// The flat variant is WinUI's subtle button, which has no checked form:
/// `SubtleButtonStyle` styles a `Button` (`TargetType="Button"`,
/// `Button_themeresources.xaml:297` at microsoft-ui-xaml 8463f45), which
/// cannot be checked, and `ToggleButton_themeresources.xaml` there defines
/// `DefaultToggleButtonStyle` alone (:180), no subtle toggle. So no rule
/// states a checked flat button, which draws the flat look of its other
/// states.
fn button(
    c: &SchemeColors,
    hc: &HighContrast,
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
    let flat = |state: WidgetState| {
        RuleSelector::on(state).matcher(ButtonMatcher {
            variant: Some(ButtonVariant::Flat),
            shape: None,
        })
    };
    // Under high contrast the outline is ButtonBorderBrush = SystemControlForegroundTransparentBrush
    // (Button_themeresources.xaml:86 at microsoft-ui-xaml 8463f45), there SystemColorButtonTextColor
    // (generic.xaml:2054 at 8463f45), the button text, not the window text the default border is.
    let body = RuleBuilder::with_exclusive(PartStyle::default(), PRESSED_HOVER)
        .base([fill(hc.button_face.unwrap_or(c.control_fill))])
        .base(
            hc.button_text
                .or(roles.window.border)
                .map(|color| outside_stroke(layout.border_width, color)),
        )
        .base([
            corners(layout.corner_radius),
            PartProp::Padding(Edges::new(
                BUTTON_PADDING_TOP,
                BUTTON_PADDING_HORIZONTAL,
                BUTTON_PADDING_BOTTOM,
                BUTTON_PADDING_HORIZONTAL,
            )),
            PartProp::MinSize([Some(BUTTON_MIN_WIDTH), Some(BUTTON_MIN_HEIGHT)]),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.accent_fill)],
        )
        // Hovered and pressed under high contrast, ButtonBackgroundPointerOver / …Pressed =
        // SystemColorHighlightTextColorBrush (Button_themeresources.xaml:79-80 at
        // microsoft-ui-xaml 8463f45), behind the highlight label (:83-84).
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(hc.highlight_text.unwrap_or(c.control_fill_hover))],
        )
        // Pressed, the outline flat ControlStrokeColorDefault: ButtonBorderBrushPressed =
        // ControlStrokeColorDefaultBrush (Button_themeresources.xaml:138, :40, Pressed state
        // :200-201; Common_themeresources_any.xaml:243, :39 at microsoft-ui-xaml 8463f45); under
        // high contrast SystemColorHighlightTextColorBrush (Button_themeresources.xaml:88).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [
                fill(hc.highlight_text.unwrap_or(c.control_fill_pressed)),
                outside_stroke(
                    layout.border_width,
                    hc.highlight_text.unwrap_or(c.control_stroke),
                ),
            ],
        )
        // Hovered, the outline as at rest: ButtonBorderBrushPointerOver is the rest brush
        // (Button_themeresources.xaml:137, :136 at microsoft-ui-xaml 8463f45); under high contrast
        // SystemColorHighlightColorBrush (:87).
        .rules(
            hc.highlight
                .or(roles.window.border)
                .map(|_| RuleSelector::on(WidgetState::HOVER)),
            hc.highlight
                .or(roles.window.border)
                .map(|color| outside_stroke(layout.border_width, color)),
        )
        .rule(
            primary(),
            [fill(c.accent_fill)]
                .into_iter()
                .chain(border(roles, layout.border_width)),
        )
        // Disabled, ButtonBackgroundDisabled = ControlFillColorDisabledBrush
        // (Button_themeresources.xaml:131, :33 at microsoft-ui-xaml 8463f45); under high contrast
        // ButtonBackgroundDisabled and AccentButtonBackgroundDisabled =
        // SystemControlBackgroundBaseLowBrush (:81, :57), the button face (generic.xaml:2010).
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(hc.button_face.unwrap_or(c.control_fill_disabled))],
        )
        // Disabled under high contrast, the outline the grey text: ButtonBorderBrushDisabled and
        // AccentButtonBorderBrushDisabled = SystemControlDisabledTransparentBrush
        // (Button_themeresources.xaml:89, :65 at microsoft-ui-xaml 8463f45), there
        // SystemColorGrayTextColor (generic.xaml:2032 at 8463f45).
        .rules(
            hc.gray_text
                .map(|_| RuleSelector::on(WidgetState::DISABLED)),
            hc.gray_text
                .map(|color| outside_stroke(layout.border_width, color)),
        )
        // The flat button, WinUI's subtle button (SubtleButtonStyle), after the push button's
        // rules so that its own win, each from its resources (Button_themeresources.xaml:17-28,
        // high contrast :66-69 and :74-77 at microsoft-ui-xaml 8463f45; `docs/facts/windows/
        // button.toml` on `v0.8.0-rc1` at `78ee0efd`, row W1-021). At rest, SubtleFillColor-
        // Transparent in a transparent outline; under high contrast SystemControlBackground-
        // BaseLowBrush, the button face (generic.xaml:2010), in SystemControlForeground-
        // TransparentBrush, which high contrast makes the button text (generic.xaml:2054).
        .rule(
            flat(WidgetState::NONE),
            [
                fill(hc.button_face.unwrap_or(SUBTLE_FILL_TRANSPARENT)),
                hc.button_text
                    .map_or(PartProp::Stroke(Stroke::None), |color| {
                        outside_stroke(layout.border_width, color)
                    }),
            ],
        )
        // Hovered, SubtleFillColorSecondary filling it and its outline; under high contrast
        // SystemColorHighlightTextColor in SystemColorHighlightColor.
        .rule(
            flat(WidgetState::HOVER),
            [
                fill(hc.highlight_text.unwrap_or(c.subtle_fill)),
                outside_stroke(layout.border_width, hc.highlight.unwrap_or(c.subtle_fill)),
            ],
        )
        // Pressed, SubtleFillColorTertiary filling it and its outline; under high contrast
        // SystemColorHighlightTextColor in both.
        .rule(
            flat(WidgetState::PRESSED),
            [
                fill(hc.highlight_text.unwrap_or(c.subtle_fill_pressed)),
                outside_stroke(
                    layout.border_width,
                    hc.highlight_text.unwrap_or(c.subtle_fill_pressed),
                ),
            ],
        )
        // Disabled, as at rest; under high contrast the button face in
        // SystemControlDisabledTransparentBrush, which high contrast makes the grey text
        // (generic.xaml:2032).
        .rule(
            flat(WidgetState::DISABLED),
            [
                fill(hc.button_face.unwrap_or(SUBTLE_FILL_TRANSPARENT)),
                hc.gray_text
                    .map_or(PartProp::Stroke(Stroke::None), |color| {
                        outside_stroke(layout.border_width, color)
                    }),
            ],
        )
        .build();
    // `button.hover_text_color`: ButtonForegroundPointerOver = TextFillColorPrimary
    // (Button_themeresources.xaml:133 at microsoft-ui-xaml 8463f45), the window's text, stated for
    // `HOVER` (spec §2, `from_state`); pressed, ButtonForegroundPressed = TextFillColorSecondary
    // (:134). Under high contrast both are SystemControlHighlightBaseHighBrush (:83-84),
    // SystemColorHighlightColor (generic.xaml:2069 at microsoft-ui-xaml 8463f45).
    let label = RuleBuilder::new(label(
        &typography.body,
        Some(hc.button_text.unwrap_or(c.window_foreground)),
    ))
    .rule(
        RuleSelector::on(WidgetState::HOVER),
        [TextProp::Color(hc.highlight.unwrap_or(c.window_foreground))],
    )
    .rule(
        RuleSelector::on(WidgetState::CHECKED),
        [TextProp::Color(roles.accent.foreground)],
    )
    .rule(
        RuleSelector::on(WidgetState::PRESSED),
        [TextProp::Color(
            hc.highlight.unwrap_or(c.window_foreground_muted),
        )],
    )
    .rule(primary(), [TextProp::Color(roles.accent.foreground)])
    // Disabled, ButtonForegroundDisabled = TextFillColorDisabled (Button_themeresources.xaml:135 at
    // microsoft-ui-xaml 8463f45); under high contrast SystemControlDisabledBaseMediumLowBrush
    // (:85), SystemColorGrayTextColor (generic.xaml:2026), as AccentButtonForegroundDisabled
    // (:61) gives the primary button.
    .rule(
        RuleSelector::on(WidgetState::DISABLED),
        [TextProp::Color(hc.gray_text.unwrap_or(c.disabled_text))],
    )
    // The flat (subtle) button's label: SubtleButtonForeground… = TextFillColorPrimary at rest and
    // hovered, TextFillColorSecondary pressed, TextFillColorDisabled disabled, the push button's;
    // under high contrast SystemColorHighlightColor hovered and pressed
    // (SystemControlHighlightBaseHighBrush, generic.xaml:2069) and SystemColorGrayTextColor
    // disabled (SystemControlDisabledBaseMediumLowBrush, generic.xaml:2026)
    // (Button_themeresources.xaml:21-24, :70-73 at microsoft-ui-xaml 8463f45;
    // `docs/facts/windows/button.toml` on `v0.8.0-rc1` at `78ee0efd`, row W1-021). At rest
    // SubtleButtonForeground, under high contrast SystemColorButtonTextColor, stated with no state
    // so that a checked flat button, which WinUI has not (the doc comment above), keeps it.
    .rule(
        flat(WidgetState::NONE),
        [TextProp::Color(
            hc.button_text.unwrap_or(c.window_foreground),
        )],
    )
    .rule(
        flat(WidgetState::HOVER),
        [TextProp::Color(hc.highlight.unwrap_or(c.window_foreground))],
    )
    .rule(
        flat(WidgetState::PRESSED),
        [TextProp::Color(
            hc.highlight.unwrap_or(c.window_foreground_muted),
        )],
    )
    .rule(
        flat(WidgetState::DISABLED),
        [TextProp::Color(hc.gray_text.unwrap_or(c.disabled_text))],
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
/// `checkbox.*` serves both: its unchecked fill in its unchecked outline at
/// the default border width (0.7.2: `checkbox.border.line_width` is
/// `defaults.border.line_width` where unstated), no shadow, under high
/// contrast `COLOR_BTNFACE` in `COLOR_BTNTEXT`; under the pointer the hover
/// fill; checked, filled and outlined in the accent's fill; disabled, the
/// disabled fill, under high contrast `COLOR_WINDOW`, which a checked check
/// box and radio ([`checkbox`], [`radio`]) replace with their own.
fn indicator(
    c: &SchemeColors,
    hc: &HighContrast,
    layout: &LayoutMetrics,
) -> RuleBuilder<PartStyle, PartProp> {
    RuleBuilder::new(PartStyle::default())
        // Under high contrast, CheckBoxCheckBackgroundFillUnchecked = SystemColorButtonFaceColor in
        // CheckBoxCheckBackgroundStrokeUnchecked = SystemColorButtonTextColor
        // (CheckBox_themeresources.xaml:141, :129 at microsoft-ui-xaml 8463f45);
        // RadioButtonOuterEllipseFill and …Stroke the same (RadioButton_themeresources.xaml:80, :76).
        .base([
            fill(hc.button_face.unwrap_or(c.control_alt_fill)),
            stroke(
                layout.border_width,
                hc.button_text.unwrap_or(c.strong_stroke),
            ),
            // `checkbox.border.shadow_enabled`: platform-facts.md (§2.5): no shadow
            PartProp::Shadows(None),
            PartProp::MinSize([Some(CHECKBOX_INDICATOR_SIZE); 2]),
            PartProp::MaxSize([Some(CHECKBOX_INDICATOR_SIZE); 2]),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.checkbox_hover)],
        )
        // Checked: CheckBoxCheckBackgroundFillChecked and …StrokeChecked = AccentFillColorDefaultBrush
        // (CheckBox_themeresources.xaml:233, :221; :57, :45 at microsoft-ui-xaml 8463f45), under
        // high contrast both SystemColorHighlightColor (:145, :133), the accent's fill there
        // ([`HighContrast::lay_over`]); RadioButtonOuterEllipseCheckedStroke the same, and outside
        // high contrast …CheckedFill (RadioButton_themeresources.xaml:142, :146; :26, :30), where
        // 0.7.2 outlined both in the default border colour.
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [
                fill(c.accent_fill),
                stroke(layout.border_width, c.accent_fill),
            ],
        )
        // Disabled, under high contrast CheckBoxCheckBackgroundFillUncheckedDisabled =
        // SystemColorWindowColorBrush (CheckBox_themeresources.xaml:144 at microsoft-ui-xaml
        // 8463f45); RadioButtonOuterEllipseFillDisabled the same (RadioButton_themeresources.xaml:83).
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(hc.window.unwrap_or(c.control_fill_disabled))],
        )
}

/// The label of a check box or a radio button: the UI font in its colour;
/// disabled, the disabled text's colour.
fn check_label(c: &SchemeColors, typography: &Typography) -> RuleSet<TextStyle, TextProp> {
    RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.disabled_text)],
        )
        .build()
}

/// The check box: [`indicator`] with the default corners (0.7.2's
/// `checkbox.border.corner_radius`, `resolve_border_inheritance`), a check
/// mark in the accent's text colour (0.7.2: `checkbox.indicator_color` is
/// `defaults.accent_text_color` where unstated), [`check_label`]. Mixed, the
/// box filled and outlined in the accent's fill. Checked or mixed, hovered
/// and pressed, the accent's secondary and tertiary fills. Disabled, the box
/// is outlined in `ControlStrongStrokeColorDisabled`. Under high contrast a
/// disabled box is outlined in `COLOR_GRAYTEXT`, a disabled checked or mixed
/// box is `COLOR_GRAYTEXT` with its mark
/// in `COLOR_WINDOW` ([`HighContrast`]), and the hovered and pressed boxes
/// and marks take WinUI's high-contrast brushes.
fn checkbox(
    c: &SchemeColors,
    hc: &HighContrast,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> CheckboxWidget {
    // Hovered, the mark as at rest: CheckBoxCheckGlyphForegroundCheckedPointerOver =
    // TextOnAccentFillColorPrimary, the rest brush (CheckBox_themeresources.xaml:246, :245 at
    // microsoft-ui-xaml 8463f45); pressed, TextOnAccentFillColorSecondary (:247). Under high
    // contrast the checked mark is SystemColorHighlightTextColor at rest (:157, the accent's text
    // there, [`roles`]), SystemColorButtonFaceColor hovered and SystemColorButtonTextColor pressed
    // (:158, :159); the mixed bar SystemColorHighlightColor hovered and
    // SystemColorHighlightTextColor pressed (:162, :163), outside it as the checked mark (:250,
    // :251).
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Color(roles.accent.foreground)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [IconProp::Color(roles.accent.foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [IconProp::Color(c.accent_text_secondary)],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::CHECKED),
            [IconProp::Color(
                hc.button_face.unwrap_or(roles.accent.foreground),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::CHECKED),
            [IconProp::Color(
                hc.button_text.unwrap_or(c.accent_text_secondary),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::INDETERMINATE),
            [IconProp::Color(
                hc.highlight.unwrap_or(roles.accent.foreground),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::INDETERMINATE),
            [IconProp::Color(
                hc.highlight_text.unwrap_or(c.accent_text_secondary),
            )],
        )
        // Disabled, checked or mixed: TextOnAccentFillColorDisabled (CheckBox_themeresources.xaml:248,
        // :252, set by the CheckedDisabled and IndeterminateDisabled states, :471, :587); under high
        // contrast SystemColorWindowColor (:160, :164).
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [IconProp::Color(hc.window.unwrap_or(c.accent_text_disabled))],
        )
        .build();
    let disabled_checked = hc.gray_text.unwrap_or(c.accent_fill_disabled);
    let disabled_edge = hc.gray_text.unwrap_or(c.strong_stroke_disabled);
    // An unchecked box pressed: CheckBoxCheckBackgroundFillUncheckedPressed =
    // ControlAltFillColorQuarternary (CheckBox_themeresources.xaml:231 at microsoft-ui-xaml
    // 8463f45) in CheckBoxCheckBackgroundStrokeUncheckedPressed = ControlStrongStrokeColorDisabled
    // (:219). A disabled box keeps its own look. An unchecked box hovered keeps its outline:
    // CheckBoxCheckBackgroundStrokeUncheckedPointerOver = ControlStrongStrokeColorDefault, the rest
    // brush (CheckBox_themeresources.xaml:218, :42; rest :217, :41). A mixed box at rest:
    // CheckBoxCheckBackgroundFillIndeterminate and …StrokeIndeterminate =
    // AccentFillColorDefaultBrush (CheckBox_themeresources.xaml:237, :225; :61, :49), the checked
    // box's brushes (:233, :221); its mark TextOnAccentFillColorPrimary (:249), the checked mark's
    // (:245). Checked or mixed, hovered: …FillCheckedPointerOver, …StrokeCheckedPointerOver,
    // …IndeterminatePointerOver = AccentFillColorSecondaryBrush (:234, :222, :238, :226; :58, :46,
    // :62, :50), the accent's fill at ACCENT_FILL_SECONDARY_OPACITY; pressed, the …Pressed ones =
    // AccentFillColorTertiaryBrush (:235, :223, :239, :227), at ACCENT_FILL_TERTIARY_OPACITY. Under
    // high contrast: checked and hovered SystemColorButtonTextColor (:146, :134), pressed
    // SystemColorButtonFaceColor (:147, :135); mixed and hovered filled SystemColorHighlightTextColor
    // in SystemColorHighlightColor (:150, :138), pressed SystemColorHighlightColor (:151, :139);
    // unchecked, hovered outlined and pressed filled and outlined SystemColorHighlightColor (:130,
    // :143, :131).
    let checked_or_mixed = WidgetState::CHECKED | WidgetState::INDETERMINATE;
    let secondary = c.accent_fill.mul_alpha(ACCENT_FILL_SECONDARY_OPACITY);
    let tertiary = c.accent_fill.mul_alpha(ACCENT_FILL_TERTIARY_OPACITY);
    let checked_hover = hc.button_text.unwrap_or(secondary);
    let checked_pressed = hc.button_face.unwrap_or(tertiary);
    let mixed_pressed = hc.highlight.unwrap_or(tertiary);
    let indicator = indicator(c, hc, layout)
        .base([corners(layout.corner_radius)])
        .rule(
            RuleSelector::on(WidgetState::INDETERMINATE),
            [
                fill(c.accent_fill),
                stroke(layout.border_width, c.accent_fill),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER).except(checked_or_mixed),
            [stroke(
                layout.border_width,
                hc.highlight.unwrap_or(c.strong_stroke),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(checked_or_mixed | WidgetState::DISABLED),
            [
                fill(hc.highlight.unwrap_or(c.alt_fill_quarternary)),
                stroke(
                    layout.border_width,
                    hc.highlight.unwrap_or(c.strong_stroke_disabled),
                ),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::CHECKED),
            [
                fill(checked_hover),
                stroke(layout.border_width, checked_hover),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::CHECKED),
            [
                fill(checked_pressed),
                stroke(layout.border_width, checked_pressed),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::INDETERMINATE),
            [
                fill(hc.highlight_text.unwrap_or(secondary)),
                stroke(layout.border_width, hc.highlight.unwrap_or(secondary)),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::INDETERMINATE),
            [
                fill(mixed_pressed),
                stroke(layout.border_width, mixed_pressed),
            ],
        )
        // Disabled, the box outlined in CheckBoxCheckBackgroundStrokeUncheckedDisabled =
        // ControlStrongStrokeColorDisabledBrush (CheckBox_themeresources.xaml:220, :44 at
        // microsoft-ui-xaml 8463f45), under high contrast SystemColorGrayTextColorBrush (:132).
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [stroke(layout.border_width, disabled_edge)],
        )
        // Disabled and checked or mixed: AccentFillColorDisabled in ControlStrongStrokeColorDisabled
        // (CheckBoxCheckBackgroundFillCheckedDisabled, …StrokeCheckedDisabled, set at
        // CheckBox_themeresources.xaml:465-468; …IndeterminateDisabled at :578-581); under high
        // contrast both SystemColorGrayTextColor (:148, :136; :152, :140).
        .rules(
            [
                RuleSelector::on(WidgetState::DISABLED | WidgetState::CHECKED),
                RuleSelector::on(WidgetState::DISABLED | WidgetState::INDETERMINATE),
            ],
            [
                fill(disabled_checked),
                stroke(layout.border_width, disabled_edge),
            ],
        )
        .build();
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator,
        checkmark,
        check_label(c, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The radio button: [`indicator`] (0.7.2 states no corners for it), a dot
/// in the accent's text colour (`checkbox.indicator_color`), [`check_label`].
/// Unselected, pressed and disabled, the circle as the check box's; selected,
/// hovered and pressed, the accent's secondary and tertiary fills. Under high
/// contrast a checked circle is `COLOR_HIGHLIGHTTEXT` in the highlight's
/// outline, its dot `COLOR_HIGHLIGHT`, and the hovered, pressed and disabled
/// circles and dots take WinUI's high-contrast brushes.
fn radio(
    c: &SchemeColors,
    hc: &HighContrast,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> RadioWidget {
    // Disabled, the dot as the source names it: RadioButtonCheckGlyphFillDisabled =
    // TextOnAccentFillColorPrimary, the rest's colour (RadioButton_themeresources.xaml:153, :37,
    // set at :333; the rest's RadioButtonCheckGlyphFill, :150, :34, at microsoft-ui-xaml 8463f45).
    // The disabled key names the `Color` resource where the property is a `Brush`; what the
    // runtime draws from it is not measured. Under high contrast SystemColorGrayTextColor (:95);
    // enabled, RadioButtonCheckGlyphFill = SystemColorHighlightColorBrush (:92).
    // Hovered and pressed, checked: RadioButtonCheckGlyphFillPointerOver and …Pressed =
    // TextOnAccentFillColorPrimaryBrush, the rest's (:151, :152; :35, :36), under high contrast
    // SystemColorButtonTextColorBrush (:93, :94).
    let dot = RuleBuilder::new(PartStyle::default())
        .base([
            fill(hc.highlight.unwrap_or(roles.accent.foreground)),
            PartProp::MinSize([Some(RADIO_DOT_SIZE); 2]),
            PartProp::MaxSize([Some(RADIO_DOT_SIZE); 2]),
        ])
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER | WidgetState::CHECKED),
                RuleSelector::on(WidgetState::PRESSED | WidgetState::CHECKED),
            ],
            [fill(hc.button_text.unwrap_or(roles.accent.foreground))],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(hc.gray_text.unwrap_or(roles.accent.foreground))],
        )
        .build();
    // An unselected circle hovered keeps its outline, RadioButtonOuterEllipseStrokePointerOver =
    // ControlStrongStrokeColorDefaultBrush, the rest's (RadioButton_themeresources.xaml:135, :19;
    // rest :134, :18), under high contrast SystemColorHighlightColorBrush (:77); pressed, filled in
    // RadioButtonOuterEllipseFillPressed = ControlAltFillColorQuarternaryBrush (:140, :24) inside
    // …StrokePressed = ControlStrongStrokeColorDisabledBrush (:136, :20), under high contrast both
    // SystemColorHighlightTextColorBrush (:82, :78); disabled, outlined in …StrokeDisabled =
    // ControlStrongStrokeColorDisabledBrush (:137, :21), under high contrast SystemColorGrayTextColor
    // (:79). A selected circle hovered and pressed, filled and outlined in
    // …CheckedFillPointerOver, …CheckedStrokePointerOver = AccentFillColorSecondaryBrush (:147,
    // :143; :31, :27) and the …Pressed ones = AccentFillColorTertiaryBrush (:148, :144; :32, :28);
    // under high contrast hovered SystemColorButtonFaceColorBrush in SystemColorButtonTextColorBrush
    // (:89, :85), pressed SystemColorButtonFaceColorBrush in the same (:90, :86).
    // Disabled and checked, the circle filled and stroked in AccentFillColorDisabled
    // (RadioButtonOuterEllipseCheckedFillDisabled, …CheckedStrokeDisabled, set at
    // RadioButton_themeresources.xaml:326-330); under high contrast filled in
    // SystemColorWindowColor and stroked in SystemColorGrayTextColor (:91, :87). Checked under high
    // contrast, the circle is filled in RadioButtonOuterEllipseCheckedFill =
    // SystemColorHighlightTextColorBrush (:88) inside the highlight's outline (:84).
    let secondary = c.accent_fill.mul_alpha(ACCENT_FILL_SECONDARY_OPACITY);
    let tertiary = c.accent_fill.mul_alpha(ACCENT_FILL_TERTIARY_OPACITY);
    let unchecked_pressed = hc.highlight_text.unwrap_or(c.alt_fill_quarternary);
    let checked_pressed = hc.button_face.unwrap_or(tertiary);
    let indicator = indicator(c, hc, layout)
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(hc.highlight_text.unwrap_or(c.accent_fill))],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER).except(WidgetState::CHECKED),
            [stroke(
                layout.border_width,
                hc.highlight.unwrap_or(c.strong_stroke),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED)
                .except(WidgetState::CHECKED | WidgetState::DISABLED),
            [
                fill(unchecked_pressed),
                stroke(
                    layout.border_width,
                    hc.highlight_text.unwrap_or(c.strong_stroke_disabled),
                ),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::CHECKED),
            [
                fill(hc.button_face.unwrap_or(secondary)),
                stroke(layout.border_width, hc.button_text.unwrap_or(secondary)),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::CHECKED),
            [
                fill(checked_pressed),
                stroke(layout.border_width, checked_pressed),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [stroke(
                layout.border_width,
                hc.gray_text.unwrap_or(c.strong_stroke_disabled),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED | WidgetState::CHECKED),
            [
                fill(hc.window.unwrap_or(c.accent_fill_disabled)),
                stroke(
                    layout.border_width,
                    hc.gray_text.unwrap_or(c.accent_fill_disabled),
                ),
            ],
        )
        .build();
    RadioWidget::new(
        WidgetOrigin::Native,
        indicator,
        dot,
        check_label(c, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The focus visual at `offset`: the 2px outer ring in the focus ring's
/// colour around the 1px inner one in `FocusStrokeColorInner`, under high
/// contrast `COLOR_WINDOWTEXT` around `COLOR_WINDOW` ([`HighContrast`]),
/// shown on keyboard focus only.
fn focus_ring(c: &SchemeColors, roles: &ColorRoles, offset: f32) -> FocusIndicator {
    FocusIndicator::new(
        FocusKind::Ring,
        Stroke::uniform(FOCUS_RING_WIDTH, roles.focus_ring),
        Stroke::uniform(FOCUS_RING_INNER_WIDTH, c.focus_ring_inner),
        offset,
        None,
        Some(if FOCUS_RING_KEYBOARD_ONLY {
            FocusTrigger::KeyboardOnly
        } else {
            FocusTrigger::Always
        }),
    )
}

/// The list or table view: the window's background (0.7.2's
/// `list.background_color` where unstated, `resolve_safety_nets`) in the
/// default border, its colour, radius and width (`resolve_border_inheritance`;
/// the default shadow, which is on); rows of a least height, padded (spec §5,
/// `L4`: WinUI's `ListViewItem` pads its content), with the alternate row's
/// fill; the highlight's hovered and selected fills, both
/// `SubtleFillColorSecondary`; the window's text, which a hovered and a
/// selected row keep (`list.hover_text_color`, `list.selection_text_color`),
/// the disabled text's colour on a disabled row; headers in the window's
/// background; the grid in the default border colour (0.7.2:
/// `list.grid_color` is `defaults.border.color` where unstated). 0.7.2
/// states no header height and no header separator. A row's focus ring is
/// `ListViewItem`'s, inside the row ([`focus_ring`]).
fn list(
    c: &SchemeColors,
    hc: &HighContrast,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
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
        .base([
            PartProp::Padding(physical_edges(
                Edges::new(
                    LIST_ROW_PADDING_VERTICAL,
                    LIST_ROW_PADDING_RIGHT,
                    LIST_ROW_PADDING_VERTICAL,
                    LIST_ROW_PADDING_LEFT,
                ),
                direction,
            )),
            PartProp::MinSize([None, Some(LIST_ROW_HEIGHT)]),
            // ListViewItem's focus visual: FocusVisualPrimaryBrush = FocusStrokeColorOuterBrush,
            // FocusVisualSecondaryBrush = FocusStrokeColorInnerBrush (ListViewItem_themeresources.xaml:181,
            // :182; :29, :30 at microsoft-ui-xaml 8463f45), under high contrast
            // SystemColorWindowTextColor and SystemColorWindowColor (:94, :95), the global ring's.
            PartProp::Focus(Some(Arc::new(focus_ring(
                c,
                roles,
                LIST_ROW_FOCUS_RING_OFFSET,
            )))),
        ])
        .rule(alternate, [fill(c.list_alternate_row)])
        .build();
    let highlight = RuleBuilder::new(PartStyle::default())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.subtle_fill)])
        // ListViewItemBackgroundPressed = SubtleFillColorTertiary (ListViewItem_themeresources.xaml:171,
        // PressedBackground :256 at microsoft-ui-xaml 8463f45).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.subtle_fill_pressed)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(hc.highlight.unwrap_or(c.subtle_fill))],
        )
        .build();
    // `list.hover_text_color`: ListViewItemForegroundPointerOver = TextFillColorPrimary
    // (ListViewItem_themeresources.xaml:176 at microsoft-ui-xaml 8463f45), the window's text,
    // which high contrast does not change; `selection_text_color`: platform-facts.md (§2.15),
    // ListViewItemForegroundSelected = TextFillColorPrimary, the window's text, under high
    // contrast the highlight's text; each stated for its state (spec §2, `from_state`).
    let hover_text = match hc.window_text {
        Some(_) => c.window_foreground,
        None => typography.body.color,
    };
    let item_label = RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(hover_text)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(
                hc.highlight_text.unwrap_or(typography.body.color),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.disabled_text)],
        )
        .build();
    let header = RuleBuilder::new(PartStyle::default())
        .base([fill(c.window_background)])
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
        roles.window.border.unwrap_or(c.window_border),
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

/// The scroll bar: a transparent groove and a handle in its measured colour,
/// else in the muted foreground (0.7.2: `scrollbar.thumb_color` is
/// `defaults.muted_color` where unstated), each of an exact thickness across
/// the bar; the hovered handle's own colour.
fn scrollbar(
    c: &SchemeColors,
    roles: &ColorRoles,
    groove_width: f32,
    min_thumb_length: f32,
) -> ScrollbarWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base([fill(SUBTLE_FILL_TRANSPARENT)]),
        groove_width,
    )
    .build();
    let thumb = across(
        RuleBuilder::new(PartStyle::default()).base(
            c.scrollbar_thumb
                .or(roles.window.foreground_muted)
                .map(fill),
        ),
        SCROLLBAR_THUMB_WIDTH,
    )
    .rule(
        RuleSelector::on(WidgetState::HOVER),
        [fill(c.scrollbar_thumb_hover)],
    )
    .build();
    ScrollbarWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        min_thumb_length,
        SCROLLBAR_OVERLAY,
    )
}

/// The tooltip: the tooltip's colours, the default border, its colour, radius
/// and width (`resolve_border_inheritance`; the default shadow, which is on),
/// padded and of a greatest width; its text in the UI font at its own size,
/// with the system's tooltip font `os` laid over it (0.7.2's `tooltip.font`),
/// in the tooltip's text (the window's text, under high contrast the window
/// text, [`roles`]).
fn tooltip(
    roles: &ColorRoles,
    typography: &Typography,
    units: FontUnits,
    layout: &LayoutMetrics,
    os: Option<&OsFont>,
) -> TooltipWidget {
    let mut panel = PartStyle::default();
    for prop in roles
        .tooltip
        .background
        .map(fill)
        .into_iter()
        .chain([
            corners(layout.corner_radius),
            PartProp::Padding(Edges::new(
                TOOLTIP_PADDING_TOP,
                TOOLTIP_PADDING_HORIZONTAL,
                TOOLTIP_PADDING_BOTTOM,
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
        laid_over(
            sized(
                label(&typography.body, roles.tooltip.foreground),
                FontSize::Pt(TOOLTIP_FONT_SIZE_PT),
                units,
            ),
            os,
            units,
        ),
    )
}

/// The window: its frame in the window's background (0.7.2:
/// `window.background_color` is `defaults.background_color` where unstated)
/// with the default border, its colour and width, the large corners
/// (`resolve_border_inheritance`; the default shadow, which is on); the title
/// bar, active or not, in the window's background, Mica's solid fallback; the
/// title in `title_font` (the UI font with the system's title font laid
/// over it, 0.7.2's `window.title_bar_font`), in the window's text, the
/// inactive one in its own colour. Under high contrast the title bar is the
/// window colour and the title the button text ([`HighContrast`]).
fn window(
    c: &SchemeColors,
    hc: &HighContrast,
    roles: &ColorRoles,
    title_font: TextStyle,
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
    let mut title_bar = PartStyle::default();
    if let Some(prop) = roles.window.background.map(fill) {
        title_bar.apply(&prop);
    }
    let title_color = if active {
        hc.button_text.unwrap_or(c.window_foreground)
    } else {
        c.inactive_title
    };
    WindowChrome::new(
        WidgetOrigin::Native,
        None,
        frame.clone(),
        frame,
        title_bar.clone(),
        title_bar,
        label(&title_font, Some(title_color)),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
    )
}

/// The text field, and with `padding` and no least height the text area
/// (0.7.2's `text_area.border` takes the field's colour, radius, width and
/// shadow where unstated, `resolve_widget_to_widget`):
/// `ControlFillColorDefault` in the default border, its colour, radius and
/// width (`resolve_border_inheritance`), outside the fill
/// (`InnerBorderEdge`), no shadow; hovered, the same outline
/// (`input.hover_border_color`); focused, the outline in the accent's fill;
/// disabled, the disabled fill; the window's text, its own disabled colour;
/// the placeholder in the muted text. Under high contrast the fill is the
/// button face, the text the button text, the placeholder the window text
/// and the focused outline the mapping's accent fill
/// ([`HighContrast::focused_outline`]).
fn input(
    c: &SchemeColors,
    hc: &HighContrast,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    padding: Edges,
    min_height: Option<f32>,
) -> InputWidget {
    let field = RuleBuilder::new(PartStyle::default())
        .base([fill(hc.button_face.unwrap_or(c.control_fill))])
        .base(
            roles
                .window
                .border
                .map(|color| outside_stroke(layout.border_width, color)),
        )
        .base([
            corners(layout.corner_radius),
            // `input.border.shadow_enabled`: platform-facts.md (§2.4): no shadow
            PartProp::Shadows(None),
            PartProp::Padding(padding),
        ])
        .base(min_height.map(|height| PartProp::MinSize([None, Some(height)])));
    // `input.hover_border_color`: TextControlBorderBrushPointerOver is the rest brush,
    // TextControlElevationBorderBrush (TextBox_themeresources.xaml:135, :134 at microsoft-ui-xaml
    // 8463f45), the preset's border colour, which high contrast does not change; stated for
    // `HOVER` (spec §2, `from_state`). Pressed, the same: the TextBox has no Pressed state
    // (:255, :256, :275, :291), so under the pressing pointer it is PointerOver; the click
    // focuses it (FOCUSED).
    let hover_border = match hc.window_text {
        Some(_) => Some(c.window_border),
        None => roles.window.border,
    };
    let field = field
        .rules(
            hover_border.into_iter().flat_map(|_| {
                [
                    RuleSelector::on(WidgetState::HOVER),
                    RuleSelector::on(WidgetState::PRESSED),
                ]
            }),
            hover_border.map(|color| outside_stroke(layout.border_width, color)),
        )
        .rule(
            RuleSelector::on(WidgetState::FOCUSED),
            [outside_stroke(
                layout.border_width,
                hc.focused_outline.unwrap_or(c.accent_fill),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.control_fill_disabled)],
        )
        .build();
    // Hovered and pressed, the text as at rest: TextControlForegroundPointerOver =
    // TextFillColorPrimary, the rest brush (TextBox_themeresources.xaml:139, :138 at
    // microsoft-ui-xaml 8463f45), and the TextBox has no Pressed state (:255-291).
    let text_color = hc.button_text.unwrap_or(c.window_foreground);
    let text = RuleBuilder::new(label(&typography.body, Some(text_color)))
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
            ],
            [TextProp::Color(text_color)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.input_disabled_text)],
        )
        .build();
    let placeholder = RuleBuilder::new(TextStyle::default())
        .base([TextProp::Color(
            hc.window_text.unwrap_or(c.window_foreground_muted),
        )])
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
/// `defaults.link_color` where unstated), underlined; its hovered and
/// pressed colours `states`, the pressed one of [`lay_accent_over`],
/// where given (a palette's link has none: spec §6, step 2); its disabled
/// colour.
fn link(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    states: Option<(Color, Color)>,
) -> LinkWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(LINK_BACKGROUND)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(LINK_BACKGROUND)],
        )
        .build();
    let mut label =
        RuleBuilder::with_exclusive(label(&typography.body, Some(roles.link)), PRESSED_HOVER).base(
            [TextProp::Decoration(TextDecoration {
                kind: LINK_DECORATION,
                color: None,
                thickness: None,
            })],
        );
    if let Some((hover, pressed)) = states {
        label = label
            .rule(
                RuleSelector::on(WidgetState::HOVER),
                [TextProp::Color(hover)],
            )
            .rule(
                RuleSelector::on(WidgetState::PRESSED),
                [TextProp::Color(pressed)],
            );
    }
    let label = label
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.link_disabled)],
        )
        .build();
    LinkWidget::new(WidgetOrigin::Native, container, label)
}

/// The slider: a groove in `ControlStrongFillColorDefault` of an exact
/// thickness, its filled part in the accent's fill, each with its disabled
/// colour; the handle in its own colour, of an exact diameter.
fn slider(c: &SchemeColors, roles: &ColorRoles) -> SliderWidget {
    let track = across(
        RuleBuilder::new(PartStyle::default()).base([fill(c.strong_fill)]),
        SLIDER_TRACK_HEIGHT,
    )
    .rule(
        RuleSelector::on(WidgetState::DISABLED),
        [fill(c.strong_fill_disabled)],
    )
    .build();
    let active_track = RuleBuilder::new(PartStyle::default())
        .base([fill(c.accent_fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.accent_fill_disabled)],
        )
        .build();
    // Hovered and pressed, the outer thumb as at rest: SliderOuterThumbBackground has no
    // PointerOver or Pressed variant (Slider_themeresources.xaml:123, Border :198 at
    // microsoft-ui-xaml 8463f45); what changes is the inner ellipse (:119-120).
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
            [fill(c.slider_thumb)],
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

/// The progress bar: a groove in `ControlStrongStrokeColorDefault` with no
/// outline, in the default border colour, its own corners, no shadow,
/// unpadded, of an exact height and a least length; the progress in the
/// accent's fill. Under high contrast the groove is the window colour in a
/// 1 px outline ([`HighContrast`]).
fn progress_bar(c: &SchemeColors, hc: &HighContrast, roles: &ColorRoles) -> ProgressBarWidget {
    let border_width = if hc.window_text.is_some() {
        HIGH_CONTRAST_PROGRESS_BAR_BORDER_WIDTH
    } else {
        PROGRESS_BAR_BORDER_WIDTH
    };
    let track = RuleBuilder::new(PartStyle::default())
        .base([fill(hc.window.unwrap_or(c.strong_stroke))])
        .base(border(roles, border_width))
        .base([
            corners(PROGRESS_BAR_CORNER_RADIUS),
            // `progress_bar.border.shadow_enabled`: platform-facts.md (§2.10): no shadow
            PartProp::Shadows(None),
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
        .base([fill(c.accent_fill)])
        .build();
    ProgressBarWidget::new(WidgetOrigin::Native, track, progress, None)
}

/// The separator: the default border colour at the default border width
/// (0.7.2: `separator.line_color` and `line_width` are
/// `defaults.border.color` and `line_width` where unstated).
fn separator(c: &SchemeColors, roles: &ColorRoles, layout: &LayoutMetrics) -> SeparatorWidget {
    SeparatorWidget::new(
        WidgetOrigin::Native,
        roles.window.border.unwrap_or(c.window_border),
        layout.border_width,
        None,
        None,
    )
}

/// The splitter: its handle in the default border colour, under the pointer
/// too (0.7.2: `splitter.divider_color` is `defaults.border.color` and
/// `hover_color` the divider's where unstated).
fn splitter(roles: &ColorRoles) -> SplitterWidget {
    let handle = RuleBuilder::new(PartStyle::default())
        .base(roles.window.border.map(fill))
        // `splitter.hover_color`: platform-facts.md (§2.17), Windows has no draggable divider
        // control and SplitView's pane edge no pointer-over resource: the divider as at rest,
        // stated for `HOVER` and `PRESSED` (spec §2, `from_state`): nothing is pressed either
        // (SplitView_themeresources.xaml at microsoft-ui-xaml 8463f45 holds no PointerOver or
        // Pressed key).
        .rules(
            roles.window.border.into_iter().flat_map(|_| {
                [
                    RuleSelector::on(WidgetState::HOVER),
                    RuleSelector::on(WidgetState::PRESSED),
                ]
            }),
            roles.window.border.map(fill),
        )
        .build();
    SplitterWidget::new(WidgetOrigin::Native, handle, SPLITTER_DIVIDER_WIDTH)
}

/// The spinner, in the accent's fill.
fn spinner(c: &SchemeColors) -> SpinnerWidget {
    SpinnerWidget::new(
        WidgetOrigin::Native,
        c.accent_fill,
        SPINNER_DIAMETER,
        SPINNER_MIN_DIAMETER,
        SPINNER_STROKE_WIDTH,
        None,
    )
}

/// The menu: its panel in its own background; items padded, their hover
/// fill; the window's text, which a hovered item keeps
/// (`menu.hover_text_color`), the disabled text's colour when disabled;
/// icons at their own size; separators in the default border colour (0.7.2:
/// `menu.separator_color` is `defaults.border.color` where unstated). 0.7.2
/// states no row height, no pressed item and no outline, corners or shadow.
/// The system's menu font `os` is laid over the items' (0.7.2's
/// `menu.font`); under high contrast their text is the button text
/// ([`HighContrast`]).
fn menu(
    c: &SchemeColors,
    hc: &HighContrast,
    roles: &ColorRoles,
    typography: &Typography,
    units: FontUnits,
    os: Option<&OsFont>,
) -> MenuWidget {
    let panel = RuleBuilder::new(PartStyle::default())
        .base([fill(c.menu_background)])
        .build();
    let item = RuleBuilder::new(PartStyle::default())
        .base([PartProp::Padding(Edges::new(
            MENU_ITEM_PADDING_TOP,
            MENU_ITEM_PADDING_HORIZONTAL,
            MENU_ITEM_PADDING_BOTTOM,
            MENU_ITEM_PADDING_HORIZONTAL,
        ))])
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.subtle_fill)])
        .build();
    let item_text = laid_over(
        label(
            &typography.body,
            Some(hc.button_text.unwrap_or(c.window_foreground)),
        ),
        os,
        units,
    );
    // `menu.hover_text_color`: MenuFlyoutItemForegroundPointerOver = TextFillColorPrimary
    // (MenuFlyout_themeresources.xaml:178 at microsoft-ui-xaml 8463f45), the window's text, which
    // high contrast does not change; stated for `HOVER` (spec §2, `from_state`).
    let hover_text = match hc.button_text {
        Some(_) => c.window_foreground,
        None => item_text.color,
    };
    let item_label = RuleBuilder::new(item_text)
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(hover_text)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.disabled_text)],
        )
        .build();
    let icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(MENU_ICON_SIZE)])
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
/// `defaults.background_color` where unstated), a tab of least sizes, padded,
/// no shadow, no outline, its hover layer; the selected tab's own fill, in the
/// default border colour at its own width and corners; the muted text, which a
/// hovered tab keeps (`tab.hover_text_color`), the window's text on the
/// selected tab. 0.7.2 states no indicator.
fn tab(c: &SchemeColors, roles: &ColorRoles, typography: &Typography) -> TabWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .build();
    let tab = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base([
            // `tab.border.shadow_enabled`: 0.7.2 windows-11.toml (uncited)
            PartProp::Shadows(None),
            // `tab.border.color`, `border.line_width`: platform-facts.md (§2.11): an unselected
            // tab's outline is transparent, the selected tab's alone is drawn
            PartProp::Stroke(Stroke::None),
            PartProp::MinSize([Some(TAB_MIN_WIDTH), Some(TAB_MIN_HEIGHT)]),
            PartProp::Padding(Edges::symmetric(
                TAB_PADDING_VERTICAL,
                TAB_PADDING_HORIZONTAL,
            )),
        ])
        // Hovered and pressed, no outline, as at rest: PointerOver and Pressed set TabContainer's
        // Background and the foregrounds only (TabView.xaml:315-334 at microsoft-ui-xaml 8463f45);
        // the selected states set TabViewSelectedItemBorderBrush (:344, :363).
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.tab_hover), PartProp::Stroke(Stroke::None)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.tab_pressed), PartProp::Stroke(Stroke::None)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [
                fill(c.tab_selected),
                stroke(TAB_BORDER_WIDTH, c.window_border),
                corners(TAB_CORNER_RADIUS),
            ],
        )
        .build();
    // `tab.hover_text_color`: TabViewItemHeaderForegroundPointerOver = TextFillColorSecondary
    // (TabView_themeresources.xaml:15 at microsoft-ui-xaml 8463f45), the rest colour, stated for
    // `HOVER` (spec §2, `from_state`), before the selected rule.
    let label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground_muted)))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.window_foreground_muted)],
        )
        // TabViewItemHeaderForegroundPressed = TextFillColorTertiary (TabView_themeresources.xaml:13;
        // TabView.xaml:328; Common_themeresources_any.xaml:211).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(c.inactive_title)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(c.window_foreground)],
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
        Some(TAB_ITEM_GAP),
        None,
        Some(TAB_OUTWARD_CORNER_RADIUS),
    )
}

/// The toolbar: the window's background (0.7.2: `toolbar.background_color`
/// is `defaults.background_color` where unstated) with no outline, in the
/// default border colour, its own corners, no shadow, padded, of a least
/// height; its buttons' hovered and pressed layers (under high contrast
/// `hc`'s highlight) and checked fill, their labels' hovered (under high
/// contrast alone), pressed and disabled colours (under high contrast `hc`'s
/// highlight text, and its grey text for the disabled one); a checked
/// button's label in the accent's text, pressed and disabled in its pressed
/// and disabled forms, its fill disabled the accent's disabled fill, and
/// under high contrast its fill and label hovered, pressed and disabled, as
/// WinUI's checked toggle pairs them; icons at
/// the toolbar's size (0.7.2: `toolbar.icon_size` is
/// `defaults.icon_sizes.toolbar` where unstated).
fn toolbar(
    c: &SchemeColors,
    hc: &HighContrast,
    roles: &ColorRoles,
    typography: &Typography,
    direction: Direction,
) -> ToolbarWidget {
    let bar = RuleBuilder::new(PartStyle::default())
        .base(roles.window.background.map(fill))
        .base(border(roles, TOOLBAR_BORDER_WIDTH))
        .base([
            corners(TOOLBAR_CORNER_RADIUS),
            // `toolbar.border.shadow_enabled`: platform-facts.md §2.13 border.shadow_enabled: none,
            // the template's only shadow wraps the overflow menu (:863-864)
            PartProp::Shadows(None),
            PartProp::Padding(physical_edges(
                Edges::new(
                    TOOLBAR_PADDING_VERTICAL,
                    TOOLBAR_PADDING_RIGHT,
                    TOOLBAR_PADDING_VERTICAL,
                    TOOLBAR_PADDING_LEFT,
                ),
                direction,
            )),
            PartProp::MinSize([None, Some(TOOLBAR_BAR_HEIGHT)]),
        ])
        .build();
    // Hovered and pressed, AppBarButtonBackgroundPointerOver / …Pressed = SubtleFillColorSecondary /
    // …Tertiary (AppBarButton_themeresources.xaml:6-7 at microsoft-ui-xaml 8463f45); under high
    // contrast SystemControlHighlightListLowBrush / …ListMediumBrush (:41-42), both
    // SystemColorHighlightColor (generic.xaml:2079-2080), behind the highlight-text label.
    let button = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(hc.highlight.unwrap_or(c.subtle_fill))],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(hc.highlight.unwrap_or(c.subtle_fill_pressed))],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.accent_fill)],
        )
        // Checked under high contrast, AppBarToggleButtonBackgroundChecked =
        // SystemColorHighlightColorBrush (AppBarToggleButton_themeresources.xaml:74 at
        // microsoft-ui-xaml 8463f45), the accent's fill above; hovered and pressed
        // …CheckedPointerOver / …CheckedPressed = SystemColorHighlightTextColorBrush (:75-76),
        // disabled …CheckedDisabled = SystemColorGrayTextColorBrush (:77), outside high contrast
        // AccentFillColorDisabledBrush (:12).
        .rules(
            hc.highlight_text
                .map(|_| RuleSelector::on(WidgetState::CHECKED | WidgetState::HOVER)),
            hc.highlight_text.map(fill),
        )
        .rules(
            hc.highlight_text
                .map(|_| RuleSelector::on(WidgetState::CHECKED | WidgetState::PRESSED)),
            hc.highlight_text.map(fill),
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED | WidgetState::DISABLED),
            [fill(hc.gray_text.unwrap_or(c.accent_fill_disabled))],
        )
        .build();
    let button_icon = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(ICON_TOOLBAR)])
        .build();
    // Hovered, the label as at rest: AppBarButtonForegroundPointerOver = TextFillColorPrimary
    // (AppBarButton_themeresources.xaml:80, :79 at microsoft-ui-xaml 8463f45), under high contrast
    // SystemControlHighlightAltBaseHighBrush (:45), SystemColorHighlightTextColor; pressed,
    // AppBarButtonForegroundPressed = TextFillColorSecondary (:81, set :257-259), under high
    // contrast SystemControlHighlightAltBaseHighBrush (:46), SystemColorHighlightTextColor
    // (generic.xaml:2058 at microsoft-ui-xaml 8463f45); disabled,
    // AppBarButtonForegroundDisabled = TextFillColorDisabled (:82, :12, set :268-270), under high
    // contrast SystemControlDisabledBaseMediumLowBrush (:47), SystemColorGrayTextColor
    // (generic.xaml:2026 at microsoft-ui-xaml 8463f45): the button's disabled opacity,
    // `DISABLED_OPACITY`, is 1, so no fade draws it.
    let button_label = label(&typography.body, None);
    let (selectors, color) = rest_color_rules(&button_label, [WidgetState::HOVER]);
    let button_label = RuleBuilder::new(button_label)
        .rules(selectors, color)
        .rules(
            hc.highlight_text
                .map(|_| RuleSelector::on(WidgetState::HOVER)),
            hc.highlight_text.map(TextProp::Color),
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(
                hc.highlight_text.unwrap_or(c.window_foreground_muted),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(hc.gray_text.unwrap_or(c.disabled_text))],
        )
        // Checked, AppBarToggleButtonForegroundChecked = TextOnAccentFillColorPrimaryBrush
        // (AppBarToggleButton_themeresources.xaml:22, :152 at microsoft-ui-xaml 8463f45), the
        // accent's text, hovered the same (:23, :153), pressed TextOnAccentFillColorSecondaryBrush
        // (:24, :154; light / dark #B3FFFFFF / #80000000, Common_themeresources_any.xaml:217, :13),
        // disabled TextOnAccentFillColorDisabled (:25, :155; #FFFFFF / #87FFFFFF, :218, :14). Under high
        // contrast SystemColorHighlightTextColorBrush (AppBarToggleButton_themeresources.xaml:87), on
        // the highlight; hovered and pressed …CheckedPointerOver / …CheckedPressed =
        // SystemColorHighlightColorBrush (:88-89), on the highlight text; disabled
        // …CheckedDisabled = SystemColorWindowColorBrush (:90), on the grey text.
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [TextProp::Color(
                hc.highlight_text.unwrap_or(roles.accent.foreground),
            )],
        )
        .rules(
            hc.highlight
                .map(|_| RuleSelector::on(WidgetState::CHECKED | WidgetState::HOVER)),
            hc.highlight.map(TextProp::Color),
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED | WidgetState::PRESSED),
            [TextProp::Color(
                hc.highlight.unwrap_or(c.accent_text_secondary),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::CHECKED | WidgetState::DISABLED),
            [TextProp::Color(hc.window.unwrap_or(c.accent_text_disabled))],
        )
        .build();
    ToolbarWidget::new(
        WidgetOrigin::Native,
        bar,
        button,
        button_label,
        button_icon,
        RuleBuilder::new(PartStyle::default()).build(),
        Some(TOOLBAR_ITEM_GAP),
    )
}

/// The status bar: the window's background (0.7.2:
/// `status_bar.background_color` is `defaults.background_color` where
/// unstated) in the default border colour and width (0.7.2's partial border
/// inheritance: colour and width only). 0.7.2 states no padding. Its text is
/// the UI font with the system's status bar font `os` laid over it (0.7.2's
/// `status_bar.font`).
fn status_bar(
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    units: FontUnits,
    os: Option<&OsFont>,
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
        laid_over(label(&typography.body, None), os, units),
        PartStyle::default(),
    )
}

/// The sidebar: its transparent pane in the default border colour and width
/// (0.7.2's partial border inheritance: colour and width only), padded on
/// its sides alone (the vertical has no one value); an item's hover and
/// selected layers, both `SubtleFillColorSecondary`; the window's text, which
/// a selected item keeps (`sidebar.selection_text_color`).
fn sidebar(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SidebarWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base([fill(c.sidebar_background)])
        .base(border(roles, layout.border_width))
        .base([PartProp::PaddingSides(
            Edges::symmetric(0.0, SIDEBAR_PADDING_HORIZONTAL),
            EdgeFlags::LEFT | EdgeFlags::RIGHT,
        )])
        .build();
    // Pressed: NavigationViewItemBackgroundPressed = SubtleFillColorTertiary
    // (NavigationView_themeresources.xaml:77 at microsoft-ui-xaml 8463f45).
    let item = RuleBuilder::new(PartStyle::default())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.subtle_fill)])
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.subtle_fill_pressed)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(c.subtle_fill)],
        )
        .build();
    SidebarWidget::new(
        WidgetOrigin::Native,
        container,
        item,
        // `sidebar.selection_text_color`: platform-facts.md (§2.12),
        // NavigationViewItemForegroundSelected = TextFillColorPrimary, the window's text, stated
        // for `SELECTED` (spec §2, `from_state`); hovered the same,
        // NavigationViewItemForegroundPointerOver (NavigationView_themeresources.xaml:88), pressed
        // NavigationViewItemForegroundPressed = TextFillColorSecondary (:89).
        RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
            .rule(
                RuleSelector::on(WidgetState::HOVER),
                [TextProp::Color(c.window_foreground)],
            )
            .rule(
                RuleSelector::on(WidgetState::PRESSED),
                [TextProp::Color(c.window_foreground_muted)],
            )
            .rule(
                RuleSelector::on(WidgetState::SELECTED),
                [TextProp::Color(c.window_foreground)],
            )
            .build(),
        RuleBuilder::new(IconStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        None,
    )
}

/// The popover: the in-app acrylic's fallback colour in the flyout's
/// outline at the default border width, the large corners (0.7.2:
/// `popover.border.corner_radius` is `defaults.border.corner_radius_lg`
/// where unstated, `resolve_border_inheritance`; the default shadow, which is
/// on), padded; its text in the window's. A palette's popover background
/// (`surface_bg`) stands in place of the acrylic's colour.
fn popover(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> PopoverWidget {
    let mut panel = PartStyle::default();
    for prop in [
        fill(roles.popover.background.unwrap_or(c.acrylic_fill)),
        stroke(layout.border_width, c.flyout_stroke),
        corners(layout.corner_radius_lg),
        PartProp::Padding(Edges::new(
            POPOVER_PADDING_TOP,
            POPOVER_PADDING_HORIZONTAL,
            POPOVER_PADDING_BOTTOM,
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

/// The hovered on switch's whole fill: the system's colour where one states
/// it (high contrast's `COLOR_BTNFACE`), else `AccentFillColorSecondary`,
/// the accent's fill at [`ACCENT_FILL_SECONDARY_OPACITY`], so it follows
/// whichever accent is laid over the preset's.
fn switch_hover_on(c: &SchemeColors) -> Color {
    c.switch_hover_on
        .unwrap_or_else(|| c.accent_fill.mul_alpha(ACCENT_FILL_SECONDARY_OPACITY))
}

/// The switch: a track of exact sizes and corners, off in its fill and
/// outline, on in the accent's fill with no outline (0.7.2's
/// `checked_track_border_color` unset: "`None` draws no outline"), with
/// their hover fills, and disabled, off and on; a thumb of an exact diameter,
/// off in the muted text, on in the accent's text, with no outline (0.7.2
/// states none for it); its On and Off content [`switch_label`], after the
/// track at [`SWITCH_LABEL_GAP`], keeping to it.
fn switch(c: &SchemeColors, hc: &HighContrast, typography: &Typography) -> SwitchWidget {
    let size = [Some(SWITCH_TRACK_WIDTH), Some(SWITCH_TRACK_HEIGHT)];
    let track = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.control_alt_fill),
            stroke(SWITCH_TRACK_BORDER_WIDTH, c.strong_stroke),
            corners(SWITCH_TRACK_RADIUS),
            PartProp::MinSize(size),
            PartProp::MaxSize(size),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.accent_fill), PartProp::Stroke(Stroke::None)],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.switch_hover_off)],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::CHECKED),
            [fill(switch_hover_on(c))],
        )
        // ToggleSwitchFillOffPressed = ControlAltFillColorQuarternary (ToggleSwitch_themeresources.xaml:137,
        // the Pressed state :294 at microsoft-ui-xaml 8463f45); a checked switch keeps its checked
        // look.
        .rule(
            RuleSelector::on(WidgetState::PRESSED).except(WidgetState::CHECKED),
            [fill(c.alt_fill_quarternary)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [
                fill(c.switch_off_disabled),
                stroke(SWITCH_TRACK_BORDER_WIDTH, c.strong_stroke_disabled),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED | WidgetState::CHECKED),
            [fill(c.accent_fill_disabled), PartProp::Stroke(Stroke::None)],
        )
        .build();
    let thumb = RuleBuilder::new(PartStyle::default())
        .base([
            fill(c.window_foreground_muted),
            PartProp::Stroke(Stroke::None),
            PartProp::MinSize([Some(SWITCH_THUMB_DIAMETER); 2]),
            PartProp::MaxSize([Some(SWITCH_THUMB_DIAMETER); 2]),
        ])
        // The on thumb's diameter, `switch.thumb_diameter` (the constant's citation), stated for
        // `CHECKED` (spec §2, `from_state`).
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [
                fill(c.accent_foreground),
                PartProp::MinSize([Some(SWITCH_THUMB_DIAMETER); 2]),
                PartProp::MaxSize([Some(SWITCH_THUMB_DIAMETER); 2]),
            ],
        )
        .build();
    SwitchWidget::new(
        WidgetOrigin::Native,
        track,
        thumb,
        switch_label(c, hc, typography),
        // `label_side`, `label_gap_stretches`: the template's inner grid is
        // `HorizontalAlignment="Left"`, its columns `Auto`, 12, `Auto`, the content after the
        // switch (ToggleSwitch_themeresources.xaml:493-503, :505-506 at microsoft-ui-xaml
        // 8463f45), so the label keeps to the track.
        Some(LabelSide::Trailing),
        Some(SWITCH_LABEL_GAP),
        Some(false),
        None,
        // 0.7.2: `switch.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(DISABLED_OPACITY),
    )
}

/// The switch's On and Off content: the content presenters'
/// `Foreground` is the style's, `ToggleSwitchContentForeground` =
/// `TextFillColorPrimaryBrush` (ToggleSwitch_themeresources.xaml:505-506,
/// :191, :127 and :7 at microsoft-ui-xaml 8463f45), the body's colour, in
/// `ContentControlThemeFontFamily` at `ControlContentThemeFontSize` 14
/// (:195-196; generic.xaml:3939, :3961), the body font, Normal weight (the
/// style sets none, :190-202); the PointerOver, Pressed and On states animate
/// no presenter's `Foreground` (:245-324, :461-487), and `Disabled` sets both
/// to `ToggleSwitchContentForegroundDisabled` = `TextFillColorDisabledBrush`
/// (:325-336, :128 and :8), on or off; under high contrast
/// `SystemColorGrayTextColorBrush` (:68), `hc`'s grey text.
fn switch_label(
    c: &SchemeColors,
    hc: &HighContrast,
    typography: &Typography,
) -> RuleSet<TextStyle, TextProp> {
    let same = [
        RuleSelector::on(WidgetState::HOVER),
        RuleSelector::on(WidgetState::PRESSED),
        RuleSelector::on(WidgetState::CHECKED),
    ];
    RuleBuilder::new(label(&typography.body, None))
        .rules(same, [TextProp::Color(typography.body.color)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(hc.gray_text.unwrap_or(c.disabled_text))],
        )
        .build()
}

/// The dialog: its panel in the dialog's background where a palette states
/// it (`surface_bg`), else the window's, with the default border,
/// its colour and width, the large corners (`resolve_border_inheritance`;
/// the default shadow, which is on), its padding and size limits; its title
/// in the UI font at its own size and weight, its body in the UI font (0.7.2:
/// `dialog.title_font` and `body_font` are `defaults.font` where unstated).
fn dialog(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    units: FontUnits,
    layout: &LayoutMetrics,
    motion: Motion,
) -> DialogWidget {
    let mut panel = PartStyle::default();
    for prop in [fill(roles.dialog.background.unwrap_or(c.window_background))]
        .into_iter()
        .chain(border(roles, layout.border_width))
        .chain([
            corners(layout.corner_radius_lg),
            PartProp::Padding(Edges::new(
                DIALOG_PADDING_TOP,
                DIALOG_PADDING_HORIZONTAL,
                DIALOG_PADDING_BOTTOM,
                DIALOG_PADDING_HORIZONTAL,
            )),
            PartProp::MinSize([Some(DIALOG_MIN_WIDTH), Some(DIALOG_MIN_HEIGHT)]),
            PartProp::MaxSize([Some(DIALOG_MAX_WIDTH), Some(DIALOG_MAX_HEIGHT)]),
        ])
    {
        panel.apply(&prop);
    }
    let mut title = sized(
        label(&typography.body, None),
        FontSize::Pt(DIALOG_TITLE_SIZE_PT),
        units,
    );
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

/// The combo box: `ControlFillColorDefault` in the default border, its
/// colour, radius and width (`resolve_border_inheritance`), no shadow,
/// padded, of least sizes, its hover and disabled fills; the window's text,
/// the disabled text's colour when disabled; its arrow's size and column.
fn combo_box(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
) -> ComboBoxWidget {
    let field = RuleBuilder::new(PartStyle::default())
        .base([fill(c.control_fill)])
        .base(border(roles, layout.border_width))
        .base([
            corners(layout.corner_radius),
            // `combo_box.border.shadow_enabled`: platform-facts.md (§2.24): no shadow
            PartProp::Shadows(None),
            PartProp::Padding(physical_edges(
                Edges::new(
                    COMBO_BOX_PADDING_TOP,
                    COMBO_BOX_PADDING_RIGHT,
                    COMBO_BOX_PADDING_BOTTOM,
                    COMBO_BOX_PADDING_LEFT,
                ),
                direction,
            )),
            PartProp::MinSize([Some(COMBO_BOX_MIN_WIDTH), Some(COMBO_BOX_MIN_HEIGHT)]),
        ])
        // Hovered, the edge as at rest: ComboBoxBorderBrushPointerOver = ControlElevationBorderBrush,
        // the rest brush (ComboBox_themeresources.xaml:265 / :55, rest :264 / :54, PointerOver
        // :416-417 at microsoft-ui-xaml 8463f45).
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            std::iter::once(fill(c.control_fill_hover)).chain(border(roles, layout.border_width)),
        )
        // ComboBoxBackgroundPressed = ControlFillColorTertiary (ComboBox_themeresources.xaml:244,
        // Pressed state :433 at microsoft-ui-xaml 8463f45), flattened as `button.active_background`;
        // ComboBoxBorderBrushPressed = ControlStrokeColorDefaultBrush, flat (:266 / :56, Pressed
        // :435-436; Common_themeresources_any.xaml:333 / :129 → :243 / :39).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [
                fill(c.control_fill_pressed),
                stroke(layout.border_width, c.control_stroke),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.control_fill_disabled)],
        )
        .build();
    // Hovered, the label as at rest: ComboBoxForegroundPointerOver = TextFillColorPrimary
    // (ComboBox_themeresources.xaml:251, :250); pressed, ComboBoxForegroundPressed =
    // TextFillColorSecondary (:252, Pressed state :439).
    let label = RuleBuilder::new(label(&typography.body, Some(c.window_foreground)))
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [TextProp::Color(c.window_foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(c.window_foreground_muted)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.disabled_text)],
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
        Some(COMBO_BOX_ARROW_AREA_WIDTH),
        // 0.7.2: `combo_box.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The segmented control: a track of `ControlAltFillColorSecondary` in its
/// own outline at the default width and corners
/// (`resolve_border_inheritance`), no shadow; segments of a least height,
/// padded, their hover layer and the selected one's own fill; the UI font's
/// colour, which the selected segment keeps
/// (`segmented_control.active_text_color`). Under high contrast the track is
/// the button face in the highlight, the selected segment the highlight, the
/// text the button text and the selected one the highlight's text
/// ([`HighContrast`]).
fn segmented_control(
    c: &SchemeColors,
    hc: &HighContrast,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> SegmentedControlWidget {
    let track = RuleBuilder::new(PartStyle::default())
        .base([
            fill(hc.button_face.unwrap_or(c.control_alt_fill)),
            stroke(layout.border_width, c.control_stroke),
            corners(layout.corner_radius),
            // `segmented_control.border.shadow_enabled`: platform-facts.md (§2.25): no shadow
            PartProp::Shadows(None),
        ])
        .build();
    let segment = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::MinSize([None, Some(SEGMENT_HEIGHT)]),
            PartProp::Padding(Edges::new(
                SEGMENT_PADDING_TOP,
                SEGMENT_PADDING_HORIZONTAL,
                SEGMENT_PADDING_BOTTOM,
                SEGMENT_PADDING_HORIZONTAL,
            )),
        ])
        // Hovered and pressed, no outline, as at rest: the item's BorderBrush is
        // SegmentedItemBorderBrush = ControlFillColorTransparent (SegmentedItem.xaml:384,
        // :138-139 / :21-22 at Windows Community Toolkit v8.2.251219; Common_themeresources_any.xaml
        // :310 / :106 at microsoft-ui-xaml 8463f45); PointerOver and Pressed set PART_Hover's
        // Background and the foregrounds only (SegmentedItem.xaml:427-476).
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER).except(WidgetState::SELECTED),
                RuleSelector::on(WidgetState::PRESSED).except(WidgetState::SELECTED),
            ],
            [PartProp::Stroke(Stroke::None)],
        )
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(c.subtle_fill)])
        // SegmentedItemBackgroundPressed = SubtleFillColorTertiary (SegmentedItem.xaml:132 at
        // Windows Community Toolkit v8.2.251219).
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.subtle_fill_pressed)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(hc.highlight.unwrap_or(c.control_fill))],
        )
        .build();
    // `segmented_control.active_text_color`: platform-facts.md (§2.25), the selected item lettered
    // TextFillColorPrimary, the rest colour, under high contrast the highlight's text; stated for
    // `SELECTED` (spec §2, `from_state`).
    // Hovered, the rest colour: SegmentedItemForegroundPointerOver = TextFillColorPrimary
    // (SegmentedItem.xaml:152, :150); pressed, SegmentedItemForegroundPressed =
    // TextFillColorSecondary (:156).
    let segment_text = label(&typography.body, hc.button_text);
    let selected_text = hc.highlight_text.unwrap_or(segment_text.color);
    let (selectors, color) = rest_color_rules(&segment_text, [WidgetState::HOVER]);
    let segment_label = RuleBuilder::new(segment_text)
        .rules(selectors, color)
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [TextProp::Color(c.window_foreground_muted)],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(selected_text)],
        )
        .build();
    SegmentedControlWidget::new(
        WidgetOrigin::Native,
        track,
        segment,
        segment_label,
        RuleBuilder::new(PartStyle::default()).build(),
        SEGMENT_SEPARATOR_WIDTH,
        Some(SEGMENT_DISABLED_OPACITY),
    )
}

/// The card: the card's background where a palette states it
/// (`surface_bg`), else the view's (0.7.2: `card.background_color` is
/// `defaults.surface_color` where unstated) in its outline, of its own width
/// and corners, no shadow, padded.
fn card(c: &SchemeColors, roles: &ColorRoles) -> CardWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(roles.card.background.or(roles.view.background).map(fill))
        .base([
            stroke(CARD_BORDER_WIDTH, c.window_border),
            corners(CARD_CORNER_RADIUS),
            // `card.border.shadow_enabled`: 0.7.2 windows-11.toml (uncited)
            PartProp::Shadows(None),
            PartProp::Padding(Edges::symmetric(
                CARD_PADDING_VERTICAL,
                CARD_PADDING_HORIZONTAL,
            )),
        ])
        .build();
    CardWidget::new(WidgetOrigin::Native, container)
}

/// The expander: the default border, its colour, radius and width
/// (`resolve_border_inheritance`), no shadow; a header of a least height,
/// padded; the arrow's size and colour (the window's text), side and gap; the
/// content indented; the frame drawn.
fn expander(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
    direction: Direction,
) -> ExpanderWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(border(roles, layout.border_width))
        .base([
            corners(layout.corner_radius),
            // `expander.border.shadow_enabled`: platform-facts.md (§2.27): no shadow
            PartProp::Shadows(None),
        ])
        .build();
    let header = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::MinSize([None, Some(EXPANDER_HEADER_HEIGHT)]),
            PartProp::Padding(physical_edges(
                Edges::new(
                    EXPANDER_PADDING_VERTICAL,
                    EXPANDER_PADDING_RIGHT,
                    EXPANDER_PADDING_VERTICAL,
                    EXPANDER_PADDING_LEFT,
                ),
                direction,
            )),
            // `expander.hover_background`: platform-facts.md (§2.27), the header's Background is
            // ExpanderHeaderBackground = CardBackgroundFillColorDefault (Expander.xaml:111;
            // Expander_themeresources.xaml:29; Common_themeresources_any.xaml:260 at
            // microsoft-ui-xaml 8463f45), which neither the PointerOver nor the Pressed state
            // sets (Expander_themeresources.xaml:122-165)
            fill(c.view_background),
        ])
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
            ],
            [fill(c.view_background)],
        )
        .build();
    let arrow = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Size(EXPANDER_ARROW_SIZE)])
        .base([IconProp::Color(c.window_foreground)])
        .build();
    // Hovered and pressed, the title as at rest: ExpanderHeaderForegroundPointerOver and
    // ExpanderHeaderForegroundPressed = TextFillColorPrimary (Expander_themeresources.xaml:31-32,
    // :30).
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
