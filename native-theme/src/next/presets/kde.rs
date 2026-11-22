//! The `kde-breeze` preset's builder (spec §6): KDE Plasma's Breeze, light
//! and dark.
//!
//! The builder runs in three steps: the preset's constants, one table per
//! scheme for the colours and plain constants for the rest, with the system's
//! settings ([`ResolveInputs::system`]) laid over them where given; the
//! colour roles and the globals made from them; the widgets derived from the
//! roles, the globals and the colours. A widget's colour that is a role's
//! colour is taken from the roles, not from a second copy of the constant.
//! The system's colours apply where `SystemOverrides` carries colours for the
//! scheme and contrast resolved for; what 0.7.2's KDE reader derives from
//! them, the builder derives (spec §8, "The KDE part"). The active palette
//! ([`ResolveInputs::palette`]) and an explicit accent
//! ([`ResolveInputs::accent`]) are not read yet.
//!
//! Defined by the preset (spec §1, "Axis values without data"): the variant
//! `ButtonVariant::Primary` (0.7.2's `primary_*` fields), a list's alternate
//! rows (`ListKind::alternate`) and a table's row corners
//! (`ListVariant::Table`), the scroll bar's two orientations, no control size
//! and the contrast level `Normal` only; a query for anything else returns the
//! part's base style.
//!
//! A part's exclusive state group names only the flags that part has a rule
//! for: a state without data of its own returns what the query returns
//! without that state, so a flag the part does not style removes no other.
//! The groups: the button's body `DISABLED, PRESSED, HOVER`; the check box's
//! and the radio button's indicator `DISABLED, HOVER`; the list's header and
//! the scroll bar's handle `PRESSED, HOVER`; every other part (the labels,
//! the check mark, the dot, the list's container, row and header separator,
//! the scroll bar's groove) none.
//!
//! Where two rules can both match, the later one wins (spec §3, "State
//! combinations a source does not state"); the precedences, each with its
//! source:
//! - the button's body: the primary fill over the pressed, checked and
//!   default fills, as 0.7.2's connectors keep `primary_background` in every
//!   state, and the disabled fill over it;
//! - the list's row: the hovered row of a view without the focus
//!   (`HOVER | UNFOCUSED_VIEW`) over the hovered row (`HOVER`), as such a
//!   view draws its rows from the Inactive group
//!   (`docs/issues/0004_kde-leftovers-rationale.md`, L2); the selection's fill
//!   over both, and neither hover outline on a selected row, as Breeze fills
//!   and outlines "a hovered, unselected row" (the rationale, E5); a table's
//!   square corners over the highlight's radius, the table's rule last (E5:
//!   "Breeze rounds a list's highlight 4.5 and draws a table's square");
//! - the scroll bar's handle: the handle of a view without the focus with the
//!   pointer on the bar but off the handle (`HOVER_WITHIN | UNFOCUSED_VIEW`)
//!   over that view's handle (`UNFOCUSED_VIEW`), its rule after that one;
//!   the hovered and the pressed handle over both, their rules last, as the
//!   pointer on the handle is on the bar too and 0.7.2's hovered handle has
//!   one colour whatever the view's focus (`scrollbar.thumb_hover_color`:
//!   platform-facts.md (§2.8)). In the view that has the focus no source
//!   states the handle with the pointer on the bar but off the handle: that
//!   query (`HOVER_WITHIN`) returns the focused view's handle at rest.
//!
//! Issue #4's states (spec §5, "Issue #4's leftovers and their homes"):
//! - `L2`, a hovered row in a view without the focus, is derived: E5's hover
//!   alphas, 0.3 for the fill and 0.8 for the outline, applied to the
//!   Highlight of the Inactive colour group, the inactive selection (the
//!   rationale, L2: such a view's rows are "drawn … from the Inactive
//!   group"). Against the rationale's measurements the rule gives the
//!   outline's edge pixel exactly (#cce5f7 / #1a3a4b, under E5's
//!   half-covered pixel) and the row's fill within one 8-bit step (#edf6fc /
//!   #16232a over the view, measured #ecf5fb / #16232b); that residual is
//!   not explained.
//! - `L4`, a row's own box: its highlight margins in the row's base, and the
//!   highlight's 4.5 corners on the rules that draw a highlight (the hovered
//!   rows and the selection), so a row at rest, an alternate row's stripe
//!   too, states no corners.
//! - `S3`, the handle in the view that has the focus: Breeze's measured
//!   colour. Under the system's colours the handle at rest is the plain
//!   handle in every view, as 0.7.2 draws it: Breeze's rule, `focusColor()`
//!   (platform-facts.md (§2.8)), through the idle handle's overlay gives
//!   #d0e4ef / #253b48 on Breeze's colours, not the measured #b1d9ef /
//!   #2a546b, so no derivation from another scheme's colours is stated.
//! - `S4`, the handle of a view without the focus with the pointer on the
//!   bar but off the handle: the idle handle's overlay at the factor 1
//!   instead of 0.7 (platform-facts.md (§2.8)), from the window's colours
//!   where they are stated.
//!
//! The system's scheme-independent settings apply wherever given: the UI,
//! monospace and caption fonts (with the headings derived from the UI font),
//! the font DPI, the icon theme and its sizes (unless the system was read in
//! the other scheme), the cursor, the caret's blink and the double-click and
//! drag timings.
//!
//! Of the context it acts on the scheme, and on the contrast only to pick
//! the system's colours. The motion preference, the text scale, the
//! direction and the accessibility and window flags change nothing here: no
//! row of the parity table needs them.
//!
//! Sources: `kde-breeze.toml` of 0.7.2 (`src/presets/kde-breeze.toml`), each
//! constant with the citation the TOML gives it (`docs/platform-facts.md` by
//! section, the upstream file at its pinned version) or `(uncited)`; 0.7.2's
//! inheritance rules (`src/resolve/inheritance.rs`, `src/resolve/validate.rs`,
//! `src/resolve/validate_helpers.rs`) where 0.7.2 fills a value the TOML does
//! not state; `docs/issues/0004_kde-leftovers-rationale.md` for issue #4's
//! states 0.7.2 has no field for (spec §5, "Issue #4's leftovers and their
//! homes": `L2`, `L4`, `S3`, `S4`).

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{Corners, Edges, Stroke};
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::LayoutMetrics;
use crate::next::motion::{CaretSpec, MotionTokens, UiTimings};
use crate::next::paint::Paint;
use crate::next::platform::KdeRoleKind;
use crate::next::platform::kcu_mix::{kcu_mix_of_mix, kcu_mix_qreal};
use crate::next::platform::kcu_overlay::kcu_overlay;
use crate::next::platform::kde_selection_inactive::kde_selection_inactive;
use crate::next::platform::kde_shade::{KdeShadeRole, kde_shade};
use crate::next::preset::{Preset, ResolveInputs, nothing_stated};
use crate::next::query::{
    ButtonVariant, Contrast, ListVariant, Orientation, Scheme, ThemeContext, WidgetState,
};
use crate::next::reader::{OsColors, OsFont, SystemOverrides};
use crate::next::roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
use crate::next::rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, Patched, RuleBuilder, RuleSelector, RuleSet,
    TextProp,
};
use crate::next::style::{
    FocusIndicator, FocusKind, FocusTrigger, FontSize, FontStyle, IconStyle, PartStyle, TextStated,
    TextStyle,
};
use crate::next::typography::{TextRendering, Typography};
use crate::next::widgets::{
    ButtonWidget, CheckboxWidget, ListWidget, RadioWidget, ResolvedTheme, ScrollbarWidget,
    TooltipWidget, WidgetOrigin,
};

// ---------------------------------------------------------------------------
// The constants
// ---------------------------------------------------------------------------

/// The colours of one scheme the builder works from, and its icon theme: the
/// constants `kde-breeze.toml` states, each colour from its 8-bit sRGB
/// channels ([`srgb8`]); where the system's colours apply, each replaced by
/// the system's setting where the system states it, and each colour 0.7.2's
/// KDE reader derives replaced by that derivation where the system states
/// every colour it reads ([`lay_system_over`]).
#[derive(Clone, Copy)]
struct SchemeColors {
    window_background: Color,
    window_foreground: Color,
    window_foreground_muted: Color,
    window_border: Color,
    view_background: Color,
    selection_background: Color,
    selection_foreground: Color,
    selection_unfocused_background: Color,
    accent: Color,
    focus_ring: Color,
    link: Color,
    disabled_text: Color,
    shadow: Color,
    error: Color,
    warning: Color,
    success: Color,
    info: Color,
    icon_theme: &'static str,
    button_background: Color,
    button_foreground: Color,
    button_frame: Color,
    button_checked: Color,
    button_pressed: Color,
    button_default: Color,
    button_default_frame: Color,
    button_disabled_background: Color,
    button_disabled_foreground: Color,
    checkbox_checked: Color,
    checkbox_frame: Color,
    view_foreground: Color,
    checkbox_disabled_foreground: Color,
    list_alternate_row: Color,
    list_header_hover: Color,
    list_header_pressed: Color,
    list_header_separator: Color,
    list_row_hover: Color,
    list_row_hover_frame: Color,
    view_disabled_foreground: Color,
    list_grid: Color,
    scrollbar_track: Color,
    scrollbar_thumb: Color,
    scrollbar_thumb_hover: Color,
    tooltip_background: Color,
    tooltip_foreground: Color,
    selection_unfocused_foreground: Color,
    /// The handle in the view that has the focus: Breeze's measured colour,
    /// which no derivation from the scheme's colours reproduces, so none
    /// where the system's colours apply.
    scrollbar_thumb_focused: Option<Color>,
    /// The handle of a view without the focus with the pointer on the bar
    /// but off the handle: no constant; [`resolve`] computes it from the
    /// window's colours ([`bar_hover_handle`]), under the system's colours
    /// only where the system states both.
    scrollbar_thumb_bar_hover: Option<Color>,
    /// The caret's colour where stated: `[Colors:View]` DecorationFocus where
    /// the system states it; unstated, 0.7.2's `input.caret_color` rule,
    /// `defaults.text_color` (`src/resolve/inheritance.rs`,
    /// `resolve_safety_nets`), the window's foreground.
    caret: Option<Color>,
    /// `[Colors:View]` ForegroundInactive, where the system states it.
    view_foreground_muted: Option<Color>,
    link_visited: Option<Color>,
}

/// The light scheme's constants.
const LIGHT: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md (§2.1.3): [Colors:Window] BackgroundNormal
    // 239,240,241 (BreezeLight.colors:129, breeze v6.7.5)
    window_background: srgb8(0xeff0_f1ff),
    // `defaults.text_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundNormal
    // 35,38,41 (BreezeLight.colors:137, breeze v6.7.5)
    // `defaults.danger_text_color`, `warning_text_color`, `success_text_color`,
    // `info_text_color`: platform-facts.md (§2.1.4): [Colors:Window] ForegroundNormal 35,38,41
    // (BreezeLight.colors:137, breeze v6.7.5)
    window_foreground: srgb8(0x2326_29ff),
    // `defaults.muted_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundInactive
    // 112,125,138 (BreezeLight.colors:133, breeze v6.7.5)
    window_foreground_muted: srgb8(0x707d_8aff),
    // `defaults.border.color`: 0.7.2 kde-breeze.toml (uncited)
    window_border: srgb8(0xbcc0_bfff),
    // `defaults.surface_color`: platform-facts.md (§2.1.3): [Colors:View] BackgroundNormal
    // 255,255,255 (BreezeLight.colors:115, breeze v6.7.5)
    view_background: srgb8(0xffff_ffff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): [Colors:Selection]
    // BackgroundNormal 61,174,233 (BreezeLight.colors:87, breeze v6.7.5)
    // `button.hover_border_color`: platform-facts.md (§2.3): hover_border_color and
    // focus_border_color: a non-flat button's pen turns Highlight, [Colors:Selection]
    // BackgroundNormal 61,174,233 (BreezeLight.colors:87, breeze v6.7.5), while hovered, pressed
    // or keyboard-focused (breezehelper.cpp:710-712 at breeze f0b1d75)
    // `button.focus_border_color`: 0.7.2 kde-breeze.toml (uncited)
    // `checkbox.border.color`: platform-facts.md (§2.5): checked, palette.highlight(),
    // [Colors:Selection] BackgroundNormal
    selection_background: srgb8(0x3dae_e9ff),
    // `defaults.selection_text_color`, also `defaults.accent_text_color`: platform-facts.md
    // (§2.1.3): [Colors:Selection] ForegroundNormal 255,255,255 (BreezeLight.colors:95, breeze
    // v6.7.5)
    selection_foreground: srgb8(0xffff_ffff),
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3):
    // selection_inactive_background, KColorUtils::tint([Colors:Window] BackgroundNormal
    // 239,240,241, [Colors:Selection] BackgroundNormal 61,174,233, 0.4), as
    // [ColorEffects:Inactive] ChangeSelectionColor=true (BreezeLight.colors:19 at breeze
    // f0b1d75) has KColorScheme build an inactive window's selection from the Window set
    // (kcolorscheme.cpp:304-321 at kcolorscheme 27066d47; measured)
    selection_unfocused_background: srgb8(0xc2e0_f5ff),
    // `defaults.accent_color`: platform-facts.md (§2.1.3): BreezeLight.colors has no [General]
    // AccentColor, so [Colors:View] DecorationFocus 61,174,233 (BreezeLight.colors:116, breeze
    // v6.7.5)
    // `checkbox.hover_border_color`: platform-facts.md (§2.5): focusColor(), [Colors:View]
    // DecorationFocus 61,174,233 (BreezeLight.colors:116, breeze v6.7.5): under the pointer the
    // box's or radio's outline is stroked in it, checked or not (breezehelper.cpp:895-907,
    // :1041-1054 at breeze f0b1d75)
    accent: srgb8(0x3dae_e9ff),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): focus_ring_color: Breeze's
    // keyboard focus band, [Colors:Selection] BackgroundNormal 61,174,233
    // (BreezeLight.colors:87, breeze v6.7.5) at alpha Blend_Value 0.3 (breezemetrics.h:176)
    focus_ring: srgb8(0x3dae_e94d),
    // `defaults.link_color`: platform-facts.md (§2.1.3): [Colors:View] ForegroundLink
    // 41,128,185 (BreezeLight.colors:120, breeze v6.7.5)
    link: srgb8(0x2980_b9ff),
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): Disabled [Colors:Window]
    // ForegroundNormal, KColorScheme 6.31.0 [ColorEffects:Disabled]
    disabled_text: srgb8(0xa0a1_a3ff),
    // `defaults.shadow_color`: platform-facts.md (§2.1.3): (none) in Breeze, the preset's own
    // #00000040
    shadow: srgb8(0x0000_0040),
    // `defaults.danger_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNegative
    // 218,68,83 (BreezeLight.colors:121, breeze v6.7.5)
    error: srgb8(0xda44_53ff),
    // `defaults.warning_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNeutral
    // 246,116,0 (BreezeLight.colors:122, breeze v6.7.5)
    warning: srgb8(0xf674_00ff),
    // `defaults.success_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundPositive
    // 39,174,96 (BreezeLight.colors:124, breeze v6.7.5)
    success: srgb8(0x27ae_60ff),
    // `defaults.info_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundActive
    // 61,174,233 (BreezeLight.colors:118, breeze v6.7.5)
    info: srgb8(0x3dae_e9ff),
    // `light.defaults.icon_theme`: 0.7.2 kde-breeze.toml (uncited)
    icon_theme: "breeze",
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): [Colors:Button] BackgroundNormal
    // 252,252,252
    // `button.background_color`: 0.7.2 kde-breeze.toml (uncited)
    // `button.hover_background`: platform-facts.md (§2.3): ← background_color, no hover fill: a
    // non-flat button's fill is palette.button() unless it is down, checked or the default
    // button, and hover only sets the pen to Highlight (breezehelper.cpp:693-707, :710-712)
    // `list.header_background`: platform-facts.md (§2.15): [Colors:Button] BackgroundNormal
    // 252,252,252 (BreezeLight.colors:31, breeze v6.7.5): Breeze fills a QHeaderView section with
    // QPalette::Button and never reads the Header set for it (breezestyle.cpp:6827, :6838-6839 at
    // breeze f0b1d75; measured)
    button_background: srgb8(0xfcfc_fcff),
    // `button.font.color`: platform-facts.md (§2.3): [Colors:Button] ForegroundNormal 35,38,41
    // (BreezeLight.colors:39, breeze v6.7.5)
    // `button.hover_text_color`, `button.checked_text_color`: 0.7.2 kde-breeze.toml (uncited)
    // `button.default_button_text_color`: platform-facts.md (§2.3): default_button_text_color,
    // [Colors:Button] ForegroundNormal 35,38,41 (BreezeLight.colors:39, breeze v6.7.5)
    // `list.header_font.color`: platform-facts.md (§2.15): [Colors:Button] ForegroundNormal
    // 35,38,41 (BreezeLight.colors:39, breeze v6.7.5): Qt letters a QHeaderView section in
    // QPalette::ButtonText (qcommonstyle.cpp:1659-1660; Breeze does not override CE_HeaderLabel;
    // measured)
    button_foreground: srgb8(0x2326_29ff),
    // `button.border.color`: platform-facts.md (§2.3): border.color, the plain button frame,
    // mix([Colors:Button] BackgroundNormal 252,252,252 (BreezeLight.colors:31), [Colors:Button]
    // ForegroundNormal 35,38,41 (BreezeLight.colors:39), frameContrast 0.2)
    // (breezehelper.cpp:697-699 at breeze f0b1d75; measured)
    button_frame: srgb8(0xd1d1_d2ff),
    // `button.checked_background`: platform-facts.md (§2.3): a checked button is
    // KColorUtils::mix(Button #fcfcfc, ButtonText #232629, 0.125), lettered ButtonText
    // (breezehelper.cpp:695-699, breezestyle.cpp:5350-5362)
    button_checked: srgb8(0xe1e1_e2ff),
    // `button.active_background`: platform-facts.md (§2.3): a pressed button is
    // KColorUtils::mix(Button, Highlight, Blend_Value 0.3) (breezehelper.cpp:694,
    // breezemetrics.h:176 at breeze f0b1d75); its text inherits font.color, ButtonText in every
    // state (platform-facts.md, breezestyle.cpp:5350-5362)
    button_pressed: srgb8(0xc3e5_f6ff),
    // `button.default_button_background`: platform-facts.md (§2.3): default_button_background,
    // KColorUtils::mix(Button #fcfcfc, Highlight #3daee9, 0.2), an active window's default
    // button (breezehelper.cpp:700-702 at breeze f0b1d75; measured)
    button_default: srgb8(0xd6ec_f8ff),
    // `button.default_button_border_color`: platform-facts.md (§2.3):
    // default_button_border_color, mix(Highlight #3daee9, mix(Button, ButtonText, frameContrast
    // 0.2) #d1d1d2, 0.5) (breezehelper.cpp:700-702 at breeze f0b1d75; measured)
    button_default_frame: srgb8(0x87c0_ddff),
    // `button.disabled_background`: platform-facts.md (§2.1.6): Disabled [Colors:Button]
    // BackgroundNormal / ForegroundNormal, KColorScheme 6.31.0
    // `checkbox.disabled_background`: platform-facts.md (§2.1.6): Disabled [Colors:Button]
    // BackgroundNormal (the box) / [Colors:Window] ForegroundNormal (the label), KColorScheme
    // 6.31.0
    button_disabled_background: srgb8(0xf0f0_f0ff),
    // `button.disabled_text_color`: 0.7.2 kde-breeze.toml (uncited)
    button_disabled_foreground: srgb8(0xa8a9_aaff),
    // `checkbox.checked_background`: platform-facts.md (§2.5): Button #fcfcfc under Selection
    // #3daee9 at alpha 0.3
    checkbox_checked: srgb8(0xc2e4_f6ff),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): separatorColor(),
    // mix(Window, WindowText, 0.2)
    checkbox_frame: srgb8(0xc6c8_c9ff),
    // `checkbox.indicator_color`: platform-facts.md (§2.5): palette.text(), [Colors:View]
    // ForegroundNormal
    // `list.item_font.color`: platform-facts.md (§2.15): [Colors:View] ForegroundNormal 35,38,41
    // (BreezeLight.colors:123, breeze v6.7.5)
    // `list.hover_text_color`: platform-facts.md (§2.15): hover_text_color ← item_font.color,
    // [Colors:View] ForegroundNormal 35,38,41 (BreezeLight.colors:123): a hovered row is lettered
    // QPalette::Text
    view_foreground: srgb8(0x2326_29ff),
    // `checkbox.disabled_text_color`: 0.7.2 kde-breeze.toml (uncited)
    checkbox_disabled_foreground: srgb8(0xa0a1_a3ff),
    // `list.alternate_row_background`: platform-facts.md (§2.15): [Colors:View]
    // BackgroundAlternate 247,247,247 (BreezeLight.colors:114, breeze v6.7.5)
    list_alternate_row: srgb8(0xf7f7_f7ff),
    // `list.header_hover_background`: platform-facts.md (§2.15): header_hover_background,
    // KColorUtils::mix(Button #fcfcfc, [Colors:View] DecorationHover 61,174,233
    // (BreezeLight.colors:117), 0.2), a hovered header section (breezestyle.cpp:6828-6837 at
    // breeze f0b1d75)
    list_header_hover: srgb8(0xd6ec_f8ff),
    // `list.header_active_background`: platform-facts.md (§2.15): header_active_background,
    // KColorUtils::mix(Button #fcfcfc, [Colors:View] DecorationFocus 61,174,233
    // (BreezeLight.colors:116), 0.2), a pressed header section (breezestyle.cpp:6828-6837 at
    // breeze f0b1d75)
    list_header_pressed: srgb8(0xd6ec_f8ff),
    // `list.header_separator_color`: platform-facts.md (§2.15): header_separator_color,
    // [Colors:Window] ForegroundNormal 35,38,41 (BreezeLight.colors:137) at alpha frameContrast
    // 0.2, the line on a section's trailing edge and along the header's bottom, none after the
    // last or a lone section (breezestyle.cpp:6803-6893 at breeze f0b1d75; measured)
    list_header_separator: srgb8(0x2326_2933),
    // `list.hover_background`: platform-facts.md (§2.15): [Colors:Selection] BackgroundNormal
    // 61,174,233 (BreezeLight.colors:87, breeze v6.7.5) at alpha Blend_Value 0.3, a focused
    // view's hovered row (breezestyle.cpp:4871-4875 at breeze f0b1d75; measured)
    list_row_hover: srgb8(0x3dae_e94d),
    // `list.hover_border_color`: platform-facts.md (§2.15): hover_border_color,
    // [Colors:Selection] BackgroundNormal 61,174,233 (BreezeLight.colors:87) at alpha 0.8, the
    // 1 px outline of a hovered, unselected row (breezestyle.cpp:4880-4884 at breeze f0b1d75;
    // measured)
    list_row_hover_frame: srgb8(0x3dae_e9cc),
    // `list.disabled_text_color`: platform-facts.md (§2.1.6): Disabled [Colors:View]
    // ForegroundNormal, KColorScheme 6.31.0
    view_disabled_foreground: srgb8(0xaaab_acff),
    // `list.grid_color`: platform-facts.md (§2.15): QPalette::Mid, KColorScheme's MidShade of
    // [Colors:Window] BackgroundNormal 239,240,241 (BreezeLight.colors:129, breeze v6.7.5) at
    // [KDE] contrast 4 (BreezeLight.colors:195 at breeze f0b1d75), a QTableView's grid
    // (qcommonstyle.cpp:5126-5128 at qtbase v6.11.0; kcolorscheme.cpp:685 at 27066d47;
    // measured)
    list_grid: srgb8(0xc4c8_ccff),
    // `scrollbar.track_color`: platform-facts.md (§2.8): no fill: Breeze's page controls draw
    // nothing and the scroll area paints the bar's area in its viewport's own colour, Base by a
    // list, Window by a plain area (breezestyle.cpp:1490-1495, :1806-1822 at breeze f0b1d75;
    // measured)
    scrollbar_track: srgb8(0x0000_0000),
    // `scrollbar.thumb_color`: platform-facts.md (§2.8): at rest, the pointer off the bar:
    // WindowText #232629 at 0.5 × 0.7, filled at half of that over Window #eff0f1, so
    // WindowText at 0.175 over Window (breezehelper.cpp:279, :349-355, :1284;
    // breezestyle.cpp:6401-6405 at breeze f0b1d75; measured)
    scrollbar_thumb: srgb8(0xcbcd_cdff),
    // `scrollbar.thumb_hover_color`: platform-facts.md (§2.8): [Colors:View] DecorationHover at
    // half alpha over the window (breezehelper.cpp:292, :1284; breezestyle.cpp:6400-6405 at
    // breeze f0b1d75)
    // `scrollbar.thumb_active_color`: platform-facts.md (§2.8): no pressed colour of its own, the
    // hovered one
    scrollbar_thumb_hover: srgb8(0x96cf_edff),
    // `tooltip.background_color`: 0.7.2 kde-breeze.toml (uncited)
    tooltip_background: srgb8(0xf7f7_f7ff),
    // `tooltip.font.color`: platform-facts.md (§2.7): [Colors:Tooltip] ForegroundNormal
    // 35,38,41 (BreezeLight.colors:109, breeze v6.7.5)
    tooltip_foreground: srgb8(0x2326_29ff),
    // `defaults.selection_inactive_text_color`: platform-facts.md (§2.1.3):
    // selection_inactive_text_color, [Colors:Window] ForegroundNormal 35,38,41
    // (BreezeLight.colors:137, breeze v6.7.5)
    selection_unfocused_foreground: srgb8(0x2326_29ff),
    // `scrollbar.thumb` in the view that has the focus: docs/issues/0004_kde-leftovers-rationale.md
    // (S3): measured #b1d9ef (docs/evidence/breeze-offscreen/2026-11-13/)
    scrollbar_thumb_focused: Some(srgb8(0xb1d9_efff)),
    scrollbar_thumb_bar_hover: None,
    caret: None,
    view_foreground_muted: None,
    // `link.visited_text_color`: platform-facts.md (§2.28): [Colors:View] ForegroundVisited
    // 155,89,182 (BreezeLight.colors:125, breeze v6.7.5)
    link_visited: Some(srgb8(0x9b59_b6ff)),
};

/// The dark scheme's constants.
const DARK: SchemeColors = SchemeColors {
    // `defaults.background_color`: platform-facts.md (§2.1.3): [Colors:Window] BackgroundNormal
    // 32,35,38 (BreezeDark.colors:130, breeze v6.7.5)
    window_background: srgb8(0x2023_26ff),
    // `defaults.text_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundNormal
    // 252,252,252 (BreezeDark.colors:138, breeze v6.7.5)
    // `defaults.danger_text_color`, `warning_text_color`, `success_text_color`,
    // `info_text_color`: platform-facts.md (§2.1.4): [Colors:Window] ForegroundNormal 252,252,252
    // (BreezeDark.colors:138, breeze v6.7.5)
    window_foreground: srgb8(0xfcfc_fcff),
    // `defaults.muted_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundInactive
    // 161,169,177 (BreezeDark.colors:134, breeze v6.7.5)
    window_foreground_muted: srgb8(0xa1a9_b1ff),
    // `defaults.border.color`: 0.7.2 kde-breeze.toml (uncited)
    window_border: srgb8(0x4d54_5bff),
    // `defaults.surface_color`: platform-facts.md (§2.1.3): [Colors:View] BackgroundNormal
    // 20,22,24 (BreezeDark.colors:116, breeze v6.7.5)
    view_background: srgb8(0x1416_18ff),
    // `defaults.selection_background`: platform-facts.md (§2.1.3): [Colors:Selection]
    // BackgroundNormal 61,174,233 (BreezeDark.colors:88, breeze v6.7.5)
    // `button.hover_border_color`: platform-facts.md (§2.3): hover_border_color and
    // focus_border_color: a non-flat button's pen turns Highlight, [Colors:Selection]
    // BackgroundNormal 61,174,233 (BreezeDark.colors:88, breeze v6.7.5), while hovered, pressed
    // or keyboard-focused (breezehelper.cpp:710-712 at breeze f0b1d75)
    // `button.focus_border_color`: 0.7.2 kde-breeze.toml (uncited)
    // `checkbox.border.color`: platform-facts.md (§2.5): checked, palette.highlight(),
    // [Colors:Selection] BackgroundNormal
    selection_background: srgb8(0x3dae_e9ff),
    // `defaults.selection_text_color`, also `defaults.accent_text_color`: platform-facts.md
    // (§2.1.3): [Colors:Selection] ForegroundNormal 252,252,252 (BreezeDark.colors:96, breeze
    // v6.7.5)
    selection_foreground: srgb8(0xfcfc_fcff),
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3):
    // selection_inactive_background, KColorUtils::tint([Colors:Window] BackgroundNormal
    // 32,35,38, [Colors:Selection] BackgroundNormal 61,174,233, 0.4), as
    // [ColorEffects:Inactive] ChangeSelectionColor=true (BreezeDark.colors:20 at breeze
    // f0b1d75) has KColorScheme build an inactive window's selection from the Window set
    // (kcolorscheme.cpp:304-321 at kcolorscheme 27066d47; measured)
    selection_unfocused_background: srgb8(0x1b41_55ff),
    // `defaults.accent_color`: platform-facts.md (§2.1.3): BreezeDark.colors has no [General]
    // AccentColor, so [Colors:View] DecorationFocus 61,174,233 (BreezeDark.colors:117, breeze
    // v6.7.5)
    // `checkbox.hover_border_color`: platform-facts.md (§2.5): focusColor(), [Colors:View]
    // DecorationFocus 61,174,233 (BreezeDark.colors:117, breeze v6.7.5): under the pointer the
    // box's or radio's outline is stroked in it, checked or not (breezehelper.cpp:895-907,
    // :1041-1054 at breeze f0b1d75)
    accent: srgb8(0x3dae_e9ff),
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): focus_ring_color: Breeze's
    // keyboard focus band, [Colors:Selection] BackgroundNormal 61,174,233
    // (BreezeDark.colors:88, breeze v6.7.5) at alpha Blend_Value 0.3 (breezemetrics.h:176)
    focus_ring: srgb8(0x3dae_e94d),
    // `defaults.link_color`: platform-facts.md (§2.1.3): [Colors:View] ForegroundLink
    // 29,153,243 (BreezeDark.colors:121, breeze v6.7.5)
    link: srgb8(0x1d99_f3ff),
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): Disabled [Colors:Window]
    // ForegroundNormal, KColorScheme 6.31.0 [ColorEffects:Disabled]
    disabled_text: srgb8(0x686a_6cff),
    // `defaults.shadow_color`: platform-facts.md (§2.1.3): (none) in Breeze, the preset's own
    // #00000060
    shadow: srgb8(0x0000_0060),
    // `defaults.danger_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNegative
    // 218,68,83 (BreezeDark.colors:122, breeze v6.7.5)
    error: srgb8(0xda44_53ff),
    // `defaults.warning_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNeutral
    // 246,116,0 (BreezeDark.colors:123, breeze v6.7.5)
    warning: srgb8(0xf674_00ff),
    // `defaults.success_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundPositive
    // 39,174,96 (BreezeDark.colors:125, breeze v6.7.5)
    success: srgb8(0x27ae_60ff),
    // `defaults.info_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundActive
    // 61,174,233 (BreezeDark.colors:119, breeze v6.7.5)
    info: srgb8(0x3dae_e9ff),
    // `dark.defaults.icon_theme`: 0.7.2 kde-breeze.toml (uncited)
    icon_theme: "breeze-dark",
    // `checkbox.unchecked_background`: platform-facts.md (§2.5): [Colors:Button] BackgroundNormal
    // 41,44,48 (BreezeDark.colors:32)
    // `button.background_color`: 0.7.2 kde-breeze.toml (uncited)
    // `button.hover_background`: platform-facts.md (§2.3): ← background_color, no hover fill: a
    // non-flat button's fill is palette.button() unless it is down, checked or the default
    // button, and hover only sets the pen to Highlight (breezehelper.cpp:693-707, :710-712)
    // `list.header_background`: platform-facts.md (§2.15): [Colors:Button] BackgroundNormal
    // 41,44,48 (BreezeDark.colors:32, breeze v6.7.5): Breeze fills a QHeaderView section with
    // QPalette::Button and never reads the Header set for it (breezestyle.cpp:6827, :6838-6839 at
    // breeze f0b1d75; measured)
    button_background: srgb8(0x292c_30ff),
    // `button.font.color`: platform-facts.md (§2.3): [Colors:Button] ForegroundNormal 252,252,252
    // (BreezeDark.colors:40, breeze v6.7.5)
    // `button.hover_text_color`, `button.checked_text_color`: 0.7.2 kde-breeze.toml (uncited)
    // `button.default_button_text_color`: platform-facts.md (§2.3): default_button_text_color,
    // [Colors:Button] ForegroundNormal 252,252,252 (BreezeDark.colors:40, breeze v6.7.5)
    // `list.header_font.color`: platform-facts.md (§2.15): [Colors:Button] ForegroundNormal
    // 252,252,252 (BreezeDark.colors:40, breeze v6.7.5): Qt letters a QHeaderView section in
    // QPalette::ButtonText (qcommonstyle.cpp:1659-1660; Breeze does not override CE_HeaderLabel;
    // measured)
    button_foreground: srgb8(0xfcfc_fcff),
    // `button.border.color`: platform-facts.md (§2.3): border.color, the plain button frame,
    // mix([Colors:Button] BackgroundNormal 41,44,48 (BreezeDark.colors:32), [Colors:Button]
    // ForegroundNormal 252,252,252 (BreezeDark.colors:40), frameContrast 0.2)
    // (breezehelper.cpp:697-699 at breeze f0b1d75; measured)
    button_frame: srgb8(0x5356_59ff),
    // `button.checked_background`: platform-facts.md (§2.3): a checked button is
    // KColorUtils::mix(Button #292c30, ButtonText #fcfcfc, 0.125), lettered ButtonText
    // (breezehelper.cpp:695-699, breezestyle.cpp:5350-5362)
    button_checked: srgb8(0x4346_4aff),
    // `button.active_background`: platform-facts.md (§2.3): a pressed button is
    // KColorUtils::mix(Button, Highlight, Blend_Value 0.3) (breezehelper.cpp:694,
    // breezemetrics.h:176 at breeze f0b1d75); its text inherits font.color, ButtonText in every
    // state (platform-facts.md, breezestyle.cpp:5350-5362)
    button_pressed: srgb8(0x2f53_68ff),
    // `button.default_button_background`: platform-facts.md (§2.3): default_button_background,
    // KColorUtils::mix(Button #292c30, Highlight #3daee9, 0.2), an active window's default
    // button (breezehelper.cpp:700-702 at breeze f0b1d75; measured)
    button_default: srgb8(0x2d46_55ff),
    // `button.default_button_border_color`: platform-facts.md (§2.3):
    // default_button_border_color, mix(Highlight #3daee9, mix(Button, ButtonText, frameContrast
    // 0.2) #535659, 0.5) (breezehelper.cpp:700-702 at breeze f0b1d75; measured)
    button_default_frame: srgb8(0x4882_a1ff),
    // `button.disabled_background`: platform-facts.md (§2.1.6): Disabled [Colors:Button]
    // BackgroundNormal / ForegroundNormal, KColorScheme 6.31.0
    // `checkbox.disabled_background`: platform-facts.md (§2.1.6): Disabled [Colors:Button]
    // BackgroundNormal (the box) / [Colors:Window] ForegroundNormal (the label), KColorScheme
    // 6.31.0
    button_disabled_background: srgb8(0x272a_2eff),
    // `button.disabled_text_color`: 0.7.2 kde-breeze.toml (uncited)
    button_disabled_foreground: srgb8(0x6d6f_72ff),
    // `checkbox.checked_background`: platform-facts.md (§2.5): Button #292c30 under Selection
    // #3daee9 at alpha 0.3
    checkbox_checked: srgb8(0x2f53_68ff),
    // `checkbox.unchecked_border_color`: platform-facts.md (§2.5): separatorColor(),
    // mix(Window, WindowText, 0.2)
    checkbox_frame: srgb8(0x4c4e_51ff),
    // `checkbox.indicator_color`: platform-facts.md (§2.5): palette.text(), [Colors:View]
    // ForegroundNormal
    // `list.item_font.color`: platform-facts.md (§2.15): [Colors:View] ForegroundNormal
    // 252,252,252 (BreezeDark.colors:124, breeze v6.7.5)
    // `list.hover_text_color`: platform-facts.md (§2.15): hover_text_color ← item_font.color,
    // [Colors:View] ForegroundNormal 252,252,252 (BreezeDark.colors:124): a hovered row is
    // lettered QPalette::Text
    view_foreground: srgb8(0xfcfc_fcff),
    // `checkbox.disabled_text_color`: 0.7.2 kde-breeze.toml (uncited)
    checkbox_disabled_foreground: srgb8(0x686a_6cff),
    // `list.alternate_row_background`: platform-facts.md (§2.15): [Colors:View]
    // BackgroundAlternate 29,31,34 (BreezeDark.colors:115, breeze v6.7.5)
    list_alternate_row: srgb8(0x1d1f_22ff),
    // `list.header_hover_background`: platform-facts.md (§2.15): header_hover_background,
    // KColorUtils::mix(Button #292c30, [Colors:View] DecorationHover 61,174,233
    // (BreezeDark.colors:118), 0.2), a hovered header section (breezestyle.cpp:6828-6837 at
    // breeze f0b1d75)
    list_header_hover: srgb8(0x2d46_55ff),
    // `list.header_active_background`: platform-facts.md (§2.15): header_active_background,
    // KColorUtils::mix(Button #292c30, [Colors:View] DecorationFocus 61,174,233
    // (BreezeDark.colors:117), 0.2), a pressed header section (breezestyle.cpp:6828-6837 at
    // breeze f0b1d75)
    list_header_pressed: srgb8(0x2d46_55ff),
    // `list.header_separator_color`: platform-facts.md (§2.15): header_separator_color,
    // [Colors:Window] ForegroundNormal 252,252,252 (BreezeDark.colors:138) at alpha
    // frameContrast 0.2, the line on a section's trailing edge and along the header's bottom,
    // none after the last or a lone section (breezestyle.cpp:6803-6893 at breeze f0b1d75;
    // measured)
    list_header_separator: srgb8(0xfcfc_fc33),
    // `list.hover_background`: platform-facts.md (§2.15): [Colors:Selection] BackgroundNormal
    // 61,174,233 (BreezeDark.colors:88, breeze v6.7.5) at alpha Blend_Value 0.3, a focused
    // view's hovered row (breezestyle.cpp:4871-4875 at breeze f0b1d75; measured)
    list_row_hover: srgb8(0x3dae_e94d),
    // `list.hover_border_color`: platform-facts.md (§2.15): hover_border_color,
    // [Colors:Selection] BackgroundNormal 61,174,233 (BreezeDark.colors:88) at alpha 0.8, the 1
    // px outline of a hovered, unselected row (breezestyle.cpp:4880-4884 at breeze f0b1d75;
    // measured)
    list_row_hover_frame: srgb8(0x3dae_e9cc),
    // `list.disabled_text_color`: platform-facts.md (§2.1.6): Disabled [Colors:View]
    // ForegroundNormal, KColorScheme 6.31.0
    view_disabled_foreground: srgb8(0x6062_63ff),
    // `list.grid_color`: platform-facts.md (§2.15): QPalette::Mid, KColorScheme's MidShade of
    // [Colors:Window] BackgroundNormal 32,35,38 (BreezeDark.colors:130, breeze v6.7.5) at [KDE]
    // contrast 4 (BreezeDark.colors:202 at breeze f0b1d75), a QTableView's grid
    // (qcommonstyle.cpp:5126-5128 at qtbase v6.11.0; kcolorscheme.cpp:685 at 27066d47;
    // measured)
    list_grid: srgb8(0x1c1f_21ff),
    // `scrollbar.track_color`: platform-facts.md (§2.8): no fill: Breeze's page controls draw
    // nothing and the scroll area paints the bar's area in its viewport's own colour, Base by a
    // list, Window by a plain area (breezestyle.cpp:1490-1495, :1806-1822 at breeze f0b1d75;
    // measured)
    scrollbar_track: srgb8(0x0000_0000),
    // `scrollbar.thumb_color`: platform-facts.md (§2.8): at rest, the pointer off the bar:
    // WindowText #fcfcfc at 0.5 × 0.7, filled at half of that over Window #202326, so
    // WindowText at 0.175 over Window (breezehelper.cpp:279, :349-355, :1284;
    // breezestyle.cpp:6401-6405 at breeze f0b1d75; measured)
    scrollbar_thumb: srgb8(0x4649_4bff),
    // `scrollbar.thumb_hover_color`: platform-facts.md (§2.8): [Colors:View] DecorationHover at
    // half alpha over the window (breezehelper.cpp:292, :1284; breezestyle.cpp:6400-6405 at
    // breeze f0b1d75)
    // `scrollbar.thumb_active_color`: platform-facts.md (§2.8): no pressed colour of its own, the
    // hovered one
    scrollbar_thumb_hover: srgb8(0x2f68_88ff),
    // `tooltip.background_color`: platform-facts.md (§2.7): [Colors:Tooltip] BackgroundNormal
    // 41,44,48 (BreezeDark.colors:102)
    tooltip_background: srgb8(0x292c_30ff),
    // `tooltip.font.color`: platform-facts.md (§2.7): [Colors:Tooltip] ForegroundNormal
    // 252,252,252 (BreezeDark.colors:110, breeze v6.7.5)
    tooltip_foreground: srgb8(0xfcfc_fcff),
    // `defaults.selection_inactive_text_color`: platform-facts.md (§2.1.3):
    // selection_inactive_text_color, [Colors:Window] ForegroundNormal 252,252,252
    // (BreezeDark.colors:138, breeze v6.7.5)
    selection_unfocused_foreground: srgb8(0xfcfc_fcff),
    // `scrollbar.thumb` in the view that has the focus: docs/issues/0004_kde-leftovers-rationale.md
    // (S3): measured #2a546b (docs/evidence/breeze-offscreen/2026-11-13/)
    scrollbar_thumb_focused: Some(srgb8(0x2a54_6bff)),
    scrollbar_thumb_bar_hover: None,
    caret: None,
    view_foreground_muted: None,
    // `link.visited_text_color`: platform-facts.md (§2.28): [Colors:View] ForegroundVisited
    // 155,89,182 (BreezeDark.colors:126, breeze v6.7.5)
    link_visited: Some(srgb8(0x9b59_b6ff)),
};

/// The UI font's family.
// `defaults.font.family`: platform-facts.md (§1.3.1): [General] font, Breeze default Noto Sans,
// 10pt, 400 (kfontsettingsdata.cpp)
const FONT_FAMILY: &str = "Noto Sans";

/// The UI font's size, in points.
// `defaults.font.size`: 0.7.2 kde-breeze.toml (uncited)
const FONT_SIZE_PT: f32 = 10.0;

/// The UI font's weight.
// `defaults.font.weight`: 0.7.2 kde-breeze.toml (uncited)
const FONT_WEIGHT: u16 = 400;

/// The monospace font's family.
// `defaults.mono_font.family`: platform-facts.md (§1.3.1): [General] fixed, Breeze default Hack,
// 10pt, 400 (kfontsettingsdata.cpp)
const MONO_FAMILY: &str = "Hack";

/// The monospace font's size, in points.
// `defaults.mono_font.size`: 0.7.2 kde-breeze.toml (uncited)
const MONO_SIZE_PT: f32 = 10.0;

/// The monospace font's weight.
// `defaults.mono_font.weight`: 0.7.2 kde-breeze.toml (uncited)
const MONO_WEIGHT: u16 = 400;

/// The line height, a multiple of the font size.
// `defaults.line_height`: platform-facts.md (§2.1.1): Noto Sans sTypo metrics, (1069+293+0)/1000
const LINE_HEIGHT: f32 = 1.36;

/// Whether a font's pixel size is rounded to a whole number.
// `defaults.font_whole_pixels`: platform-facts.md (§2.1.1): font_whole_pixels, Qt rounds a point
// size's pixel size to hundredths, then to a whole number, halves up, before the scale
// (qfontdatabase.cpp:2865-2869 at qtbase v6.12.0; measured)
const FONT_WHOLE_PIXELS: bool = true;

/// The caption's size, in points.
// `text_scale.caption.size`: platform-facts.md (§2.19): smallestReadableFont, default 8pt, 400
const CAPTION_SIZE_PT: f32 = 8.0;

/// The caption's weight.
// `text_scale.caption.weight`: 0.7.2 kde-breeze.toml (uncited)
const CAPTION_WEIGHT: u16 = 400;

/// A section heading's size, in points.
// `text_scale.section_heading.size`: platform-facts.md (§2.19): Kirigami Heading level 2, the 10pt
// body × 1.20, Font.Normal (kirigami 8319acc, controls/Heading.qml:20-21,35)
const HEADLINE_SIZE_PT: f32 = 12.0;

/// A section heading's weight.
// `text_scale.section_heading.weight`: 0.7.2 kde-breeze.toml (uncited)
const HEADLINE_WEIGHT: u16 = 400;

/// A dialog title's size, in points.
// `text_scale.dialog_title.size`: platform-facts.md (§2.19): Kirigami Heading level 1, the 10pt
// body × 1.35, Font.Normal (kirigami 8319acc, controls/Heading.qml:17-18,35)
const TITLE1_SIZE_PT: f32 = 13.5;

/// A dialog title's weight.
// `text_scale.dialog_title.weight`: 0.7.2 kde-breeze.toml (uncited)
const TITLE1_WEIGHT: u16 = 400;

/// Kirigami `Heading` level 1 (the default level): the body font's point size
/// times this (kirigami `8319acc`, `src/controls/Heading.qml:17-18,32`; 0.7.2's
/// `kde/fonts.rs`).
const HEADING_LEVEL_1_FACTOR: f32 = 1.35;

/// Kirigami `Heading` level 2: the body font's point size times this
/// (kirigami `8319acc`, `src/controls/Heading.qml:20-21,32`; 0.7.2's
/// `kde/fonts.rs`).
const HEADING_LEVEL_2_FACTOR: f32 = 1.20;

/// Kirigami `Heading`'s weight for its default `type: Heading.Type.Normal`
/// (kirigami `8319acc`, `src/templates/Heading.qml:89`): `Font.Normal`, CSS
/// 400 (`src/controls/Heading.qml:35`; 0.7.2's `kde/fonts.rs`).
const HEADING_WEIGHT: u16 = 400;

/// The default corner radius.
// `defaults.border.corner_radius`: 0.7.2 kde-breeze.toml (uncited)
const CORNER_RADIUS: f32 = 5.0;

/// The large corner radius.
// `defaults.border.corner_radius_lg`: 0.7.2 kde-breeze.toml (uncited)
const CORNER_RADIUS_LG: f32 = 8.0;

/// The default border width.
// `defaults.border.line_width`: 0.7.2 kde-breeze.toml (uncited)
const BORDER_WIDTH: f32 = 1.0;

/// The default border opacity.
// `defaults.border.opacity`: 0.7.2 kde-breeze.toml (uncited)
const BORDER_OPACITY: f32 = 0.2;

/// Whether surfaces cast shadows.
// `defaults.border.shadow_enabled`: 0.7.2 kde-breeze.toml (uncited)
const SHADOW_ENABLED: bool = true;

/// The opacity of a disabled widget drawn as one group.
// `defaults.disabled_opacity`: platform-facts.md (§2.1.6): 1.0, Breeze dims a disabled control by
// its colours alone
const DISABLED_OPACITY: f32 = 1.0;

/// The focus ring's width.
// `defaults.focus_ring_width`: platform-facts.md (§2.1.5): focus_ring_width and focus_ring_offset:
// the band is PM_FocusFrameHMargin, 2, wide, and runs from 1 px inside the widget's rectangle to 1
// px outside it (breezestyle.cpp:737-739, :6642-6651 at breeze f0b1d75)
const FOCUS_RING_WIDTH: f32 = 2.0;

/// The focus ring's offset from the part's edge.
// `defaults.focus_ring_offset`: 0.7.2 kde-breeze.toml (uncited)
const FOCUS_RING_OFFSET: f32 = -1.0;

/// Whether the focus ring is shown only for keyboard focus.
// `defaults.focus_ring_keyboard_only`: platform-facts.md (§2.1.5): focus_ring_keyboard_only: drawn
// only after a focus change from the keyboard (breezestyle.cpp:1649-1652 at breeze f0b1d75)
const FOCUS_RING_KEYBOARD_ONLY: bool = true;

/// The icon set.
// `icon_set`: 0.7.2 kde-breeze.toml (uncited)
const ICON_SET: IconSet = IconSet::Freedesktop;

/// A toolbar icon's size.
// `defaults.icon_sizes.toolbar`: platform-facts.md (§2.1.8): KIconLoader's MainToolbar 22, Small
// 16, Desktop 48, Dialog 32 and Panel 48, Breeze's sizes
const ICON_TOOLBAR: f32 = 22.0;

/// A small icon's size.
// `defaults.icon_sizes.small`: 0.7.2 kde-breeze.toml (uncited)
const ICON_SMALL: f32 = 16.0;

/// A large icon's size.
// `defaults.icon_sizes.large`: 0.7.2 kde-breeze.toml (uncited)
const ICON_LARGE: f32 = 48.0;

/// A dialog icon's size.
// `defaults.icon_sizes.dialog`: 0.7.2 kde-breeze.toml (uncited)
const ICON_DIALOG: f32 = 32.0;

/// A panel icon's size.
// `defaults.icon_sizes.panel`: 0.7.2 kde-breeze.toml (uncited)
const ICON_PANEL: f32 = 48.0;

/// The space between widgets.
// `layout.widget_gap`: 0.7.2 kde-breeze.toml (uncited)
const WIDGET_GAP: f32 = 6.0;

/// A container's margin.
// `layout.container_margin`: 0.7.2 kde-breeze.toml (uncited)
const CONTAINER_MARGIN: f32 = 6.0;

/// A window's margin.
// `layout.window_margin`: 0.7.2 kde-breeze.toml (uncited)
const WINDOW_MARGIN: f32 = 10.0;

/// The space between sections.
// `layout.section_gap`: 0.7.2 kde-breeze.toml (uncited)
const SECTION_GAP: f32 = 18.0;

/// The delay before a tooltip shows, in milliseconds.
// `tooltip.show_delay`: platform-facts.md (§2.7): Qt's m_toolTipWakeUpDelay{700} for
// SH_ToolTip_WakeUpDelay (qstylehints_p.h:45; qcommonstyle.cpp:5351-5356)
const TOOLTIP_SHOW_MS: u32 = 700;

/// The caret's visible half of a blink, in milliseconds.
// `input.caret_blink_on`: platform-facts.md (§2.4): half Qt's cursor flash time, 1000 where
// CursorBlinkRate is unset (khintssettings.cpp:79-80 at plasma-integration 6.8.2;
// qwidgetlinecontrol.cpp:1497-1509 at qtbase 6.12.0)
const CARET_BLINK_ON_MS: u32 = 500;

/// The caret's hidden half of a blink, in milliseconds.
// `input.caret_blink_off`: platform-facts.md (§2.4): the other half, hidden
const CARET_BLINK_OFF_MS: u32 = 500;

/// A button's least width.
// `button.min_width`: platform-facts.md (§2.3): min_width, Button_MinWidth 80 plus the frame's 2 a
// side less the 1 px shadow inset a side, the visible box (breezestyle.cpp:3555-3564,
// breezehelper.cpp:670-671 at breeze f0b1d75)
const BUTTON_MIN_WIDTH: f32 = 82.0;

/// A button's padding left and right.
// `button.border.padding.left`: platform-facts.md (§2.3): Button_MarginWidth = 6, both axes
const BUTTON_PADDING_HORIZONTAL: f32 = 6.0;

/// A button's padding above and below.
// `button.border.padding.top`: 0.7.2 kde-breeze.toml (uncited)
const BUTTON_PADDING_VERTICAL: f32 = 6.0;

/// The space between a button's icon and its label.
// `button.icon_text_gap`: 0.7.2 kde-breeze.toml (uncited)
const BUTTON_ICON_TEXT_GAP: f32 = 4.0;

/// The side of a check box's box and of a radio button's circle (0.7.2:
/// `radio_indicator_width` unset means `indicator_width`).
// `checkbox.indicator_width`: 0.7.2 kde-breeze.toml (uncited)
const CHECKBOX_INDICATOR_SIZE: f32 = 20.0;

/// The diameter of a radio button's dot.
// `checkbox.radio_dot_diameter`: platform-facts.md (§2.5): Breeze's dot, radius (18 - 12) / 2 = 3
const RADIO_DOT_SIZE: f32 = 6.0;

/// The width of a check mark's pen.
// `checkbox.check_mark_stroke_width`: platform-facts.md (§2.5): Breeze's check pen, PenWidth::Frame
// * 2 = 2.002
const CHECK_MARK_STROKE_WIDTH: f32 = 2.0;

/// The space between a check box's or radio button's indicator and its label.
// `checkbox.label_gap`: 0.7.2 kde-breeze.toml (uncited)
const CHECKBOX_LABEL_GAP: f32 = 4.0;

/// A row's highlight margin left and right (spec §5, `L4`: kde-breeze's
/// `list.border.padding` holds Breeze's highlight margins).
// `list.border.padding.left`: platform-facts.md (§2.15): 2; 1
const LIST_ROW_MARGIN_HORIZONTAL: f32 = 2.0;

/// A row's highlight margin above and below.
// `list.border.padding.top`: 0.7.2 kde-breeze.toml (uncited)
const LIST_ROW_MARGIN_VERTICAL: f32 = 1.0;

/// The radius of a list row's highlight (spec §5, `L4`).
// `list.highlight` corners: docs/issues/0004_kde-leftovers-rationale.md (E5, L4): Breeze rounds a list's
// highlight 4.5 and draws a table's square
const LIST_ROW_RADIUS: f32 = 4.5;

/// The groove's thickness across the bar.
// `scrollbar.groove_width`: 0.7.2 kde-breeze.toml (uncited)
const SCROLLBAR_GROOVE_WIDTH: f32 = 21.0;

/// The handle's thickness across the bar.
// `scrollbar.thumb_width`: 0.7.2 kde-breeze.toml (uncited)
const SCROLLBAR_THUMB_WIDTH: f32 = 8.0;

/// The handle's least length along the bar.
// `scrollbar.min_thumb_length`: 0.7.2 kde-breeze.toml (uncited)
const SCROLLBAR_MIN_THUMB_LENGTH: f32 = 20.0;

/// Whether the bar overlays the content.
// `scrollbar.overlay_mode`: 0.7.2 kde-breeze.toml (uncited)
const SCROLLBAR_OVERLAY: bool = false;

/// A tooltip's padding left and right.
// `tooltip.border.padding.left`: platform-facts.md (§2.7): ToolTip_FrameWidth = 3
const TOOLTIP_PADDING_HORIZONTAL: f32 = 3.0;

/// A tooltip's padding above and below.
// `tooltip.border.padding.top`: 0.7.2 kde-breeze.toml (uncited)
const TOOLTIP_PADDING_VERTICAL: f32 = 3.0;

/// A tooltip's greatest width.
// `tooltip.max_width`: 0.7.2 kde-breeze.toml (uncited)
const TOOLTIP_MAX_WIDTH: f32 = 300.0;

/// The state precedence of a part with rules for disabled, pressed and
/// hover: disabled over pressed over hover (spec §3's example; 0.7.2's
/// pressed colours win over its hover colours).
const DISABLED_PRESSED_HOVER: &[&[WidgetState]] = &[&[
    WidgetState::DISABLED,
    WidgetState::PRESSED,
    WidgetState::HOVER,
]];

/// The state precedence of a part with rules for disabled and hover but none
/// for pressed: disabled over hover.
const DISABLED_HOVER: &[&[WidgetState]] = &[&[WidgetState::DISABLED, WidgetState::HOVER]];

/// The state precedence of a part with rules for pressed and hover but none
/// for disabled: pressed over hover.
const PRESSED_HOVER: &[&[WidgetState]] = &[&[WidgetState::PRESSED, WidgetState::HOVER]];

/// The font DPI a point size converts at: 0.7.2's on every desktop but
/// macOS (`ResolutionContext::from_system`, `src/resolve/context.rs`; spec
/// §4, `Typography::font_dpi`).
const FONT_DPI: f32 = 96.0;

// ---------------------------------------------------------------------------
// The constants of the derivations from the system's colours, each as 0.7.2's
// KDE reader states and cites it (`src/kde/colors.rs`)
// ---------------------------------------------------------------------------

/// Breeze's `Metrics::Blend_Value` (breezemetrics.h:176 at breeze v6.7.5), the
/// alpha of the selection colour over a checked indicator's fill
/// (breezehelper.cpp:40), of the selection colour over a hovered item-view row
/// (breezestyle.cpp:4871-4875 at breeze f0b1d75) and of the selection colour in
/// the keyboard focus band (breezestyle.cpp:6776-6779); docs/platform-facts.md
/// §2.5, §2.15.
const BLEND_VALUE: f32 = 0.3;

/// `kdeglobals` `[KDE] frameContrast` where it is unset, KColorScheme's own
/// default (kcolorscheme.cpp:529-538 at 27066d47; docs/platform-facts.md §2.11).
const FRAME_CONTRAST_DEFAULT: f64 = 0.2;

/// `kdeglobals` `[KDE] contrast` where it is unset or not an integer,
/// KColorScheme's own default: `contrastF` is `0.1 * readEntry("contrast", 7)`
/// (kcolorscheme.cpp:519-527 at kcolorscheme v6.31.0; docs/platform-facts.md
/// §2.15).
const CONTRAST_DEFAULT: i32 = 7;

/// Whether KColorScheme builds an inactive window's selection from the Window
/// set where kdeglobals states neither `[ColorEffects:Inactive]
/// ChangeSelectionColor` nor the group's `Enable`: true
/// (kcolorscheme.cpp:304-316 at kcolorscheme v6.31.0).
const CHANGE_SELECTION_COLOR_DEFAULT: bool = true;

/// How far a checked, non-flat Breeze button's fill moves from `Button`
/// towards `ButtonText`: `KColorUtils::mix(palette.button(),
/// palette.buttonText(), 0.125)` (breezehelper.cpp:695-699 at breeze f0b1d75;
/// docs/platform-facts.md §2.3).
const CHECKED_BUTTON_MIX: f64 = 0.125;

/// How far a pressed, enabled, non-flat Breeze button's fill moves from
/// `Button` towards `Highlight`: `KColorUtils::mix(palette.button().color(),
/// highlightColor, Metrics::Blend_Value)`, the `qreal` 0.3
/// (breezehelper.cpp:694, breezemetrics.h:176 at breeze f0b1d75;
/// docs/platform-facts.md §2.3).
const PRESSED_BUTTON_MIX: f64 = 0.3;

/// How far an active window's default button moves from `Button` towards
/// `Highlight`: `KColorUtils::mix(palette.button().color(), highlightColor,
/// 0.2)` (breezehelper.cpp:700-702 at breeze f0b1d75; docs/platform-facts.md
/// §2.3).
const DEFAULT_BUTTON_MIX: f64 = 0.2;

/// How far the default button's frame moves from `Highlight` towards the
/// plain frame, `mix(Button, ButtonText, frameContrast)`: 0.5
/// (breezehelper.cpp:700-702 at breeze f0b1d75; docs/platform-facts.md §2.3).
const DEFAULT_BUTTON_FRAME_MIX: f64 = 0.5;

/// How far a hovered or pressed header section moves from `Button` towards
/// the hover or the focus colour: `KColorUtils::mix(palette.button(),
/// hoverColor, 0.2)` and `mix(…, focusColor, 0.2)` (breezestyle.cpp:6828-6837
/// at breeze f0b1d75; docs/platform-facts.md §2.15).
const HEADER_SECTION_MIX: f64 = 0.2;

/// The alpha of `Highlight` in a hovered item-view row's 1 px outline
/// (breezestyle.cpp:4880-4884 at breeze f0b1d75; docs/platform-facts.md
/// §2.15).
const LIST_HOVER_OUTLINE_ALPHA: f32 = 0.8;

/// The alpha a Breeze scroll-bar handle's colour is filled with over the
/// window, `overlayColors(bg, alphaColor(fg, 0.5))` (breezehelper.cpp:1284
/// at breeze f0b1d75; docs/platform-facts.md §2.8).
const SCROLLBAR_HANDLE_ALPHA: f32 = 0.5;

/// The alpha of the window text over the window in a Breeze scroll-bar handle at
/// rest, the pointer off the bar: the handle colour `WindowText` at 0.5
/// (breezehelper.cpp:279), times 0.7 off the bar (breezestyle.cpp:6401-6403),
/// filled at half of that over the window, `overlayColors(Window, alphaColor(fg,
/// 0.5))` (breezehelper.cpp:1284, :349-355; breezestyle.cpp:6405) at breeze
/// f0b1d75, with animations on, their default (breeze.kcfg:45-47);
/// docs/platform-facts.md §2.8.
const SCROLLBAR_IDLE_HANDLE_ALPHA: f32 = 0.175;

/// The alpha of the window text over the window in a Breeze scroll-bar handle
/// with the pointer on the bar but off the handle: the handle colour
/// `WindowText` at 0.5 (breezehelper.cpp:279), times the factor 1 on the bar
/// in place of 0.7 (breezestyle.cpp:6401-6403), filled at half of that over
/// the window (breezehelper.cpp:1284, :349-355; breezestyle.cpp:6405) at
/// breeze f0b1d75; docs/platform-facts.md §2.8.
const SCROLLBAR_BAR_HOVER_HANDLE_ALPHA: f32 = 0.25;

// ---------------------------------------------------------------------------
// The colours from the constants and the system's settings
// ---------------------------------------------------------------------------

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

/// The system's colours `os` laid over `c` field by field, as 0.7.2's KDE
/// reader lays its result over the preset (`src/kde/colors.rs`,
/// `populate_colors`; spec §8, "The KDE part"): a colour the system states
/// replaces the constant it is the setting of; a colour 0.7.2 derives is
/// derived, with the same inputs, kernel and arguments, only where the system
/// states every colour it reads, else the constant stays. A missing
/// `kde_frame_contrast` or `kde_contrast` is KColorScheme's default, and a
/// missing `kde_effects` changes the selection as KColorScheme's default does
/// ([`CHANGE_SELECTION_COLOR_DEFAULT`]).
fn lay_system_over(c: &mut SchemeColors, os: &OsColors) {
    let surfaces = os.surfaces.clone().unwrap_or_default();
    let window = surfaces.window.unwrap_or_default();
    let view = surfaces.view.unwrap_or_default();
    let button = surfaces.button.unwrap_or_default();
    let tooltip = surfaces.tooltip.unwrap_or_default();
    let selection = os.selection.unwrap_or_default();
    let [error, warning, success, info] = os.status.unwrap_or_default();
    let frame_contrast = os.kde_frame_contrast.unwrap_or(FRAME_CONTRAST_DEFAULT);
    let contrast = os.kde_contrast.unwrap_or(CONTRAST_DEFAULT);
    let change_selection = os
        .kde_effects
        .map_or(CHANGE_SELECTION_COLOR_DEFAULT, |e| e.change_selection_color);
    let (window_bg, window_fg) = (window.background, window.foreground);
    let (button_bg, button_fg) = (button.background, button.foreground);
    let (highlight, highlight_fg) = (selection.background, selection.foreground);
    let (focus, hover) = (os.kde_decoration_focus, os.kde_decoration_hover);

    // The settings, each where stated.
    replace(&mut c.window_background, window_bg);
    replace(&mut c.window_foreground, window_fg);
    replace(&mut c.window_foreground_muted, window.foreground_muted);
    replace(&mut c.view_background, view.background);
    replace(&mut c.view_foreground, view.foreground);
    c.view_foreground_muted = view.foreground_muted;
    replace(&mut c.selection_background, highlight);
    replace(&mut c.selection_foreground, highlight_fg);
    replace(&mut c.accent, focus);
    replace(&mut c.link, os.link);
    c.link_visited = os.link_visited.or(c.link_visited);
    replace(&mut c.error, error);
    replace(&mut c.warning, warning);
    replace(&mut c.success, success);
    replace(&mut c.info, info);
    replace(&mut c.button_background, button_bg);
    replace(&mut c.button_foreground, button_fg);
    replace(&mut c.list_alternate_row, os.alternate_row_background);
    replace(&mut c.tooltip_background, tooltip.background);
    replace(&mut c.tooltip_foreground, tooltip.foreground);
    c.caret = focus.or(c.caret);
    // Breeze's measured pixel is not another scheme's colour.
    c.scrollbar_thumb_focused = None;

    // The derived colours, each where every colour it reads is stated.
    replace(&mut c.focus_ring, highlight.map(focus_ring_of));
    replace(
        &mut c.selection_unfocused_background,
        inactive_selection(
            highlight,
            window_bg,
            highlight,
            KdeRoleKind::Background,
            change_selection,
        ),
    );
    replace(
        &mut c.selection_unfocused_foreground,
        inactive_selection(
            highlight_fg,
            window_fg,
            highlight,
            KdeRoleKind::Foreground,
            change_selection,
        ),
    );
    replace(
        &mut c.button_checked,
        button_bg
            .zip(button_fg)
            .map(|(bg, fg)| mix(bg, fg, CHECKED_BUTTON_MIX)),
    );
    replace(
        &mut c.button_pressed,
        button_bg
            .zip(highlight)
            .map(|(bg, hl)| mix(bg, hl, PRESSED_BUTTON_MIX)),
    );
    replace(
        &mut c.button_default,
        button_bg
            .zip(highlight)
            .map(|(bg, hl)| mix(bg, hl, DEFAULT_BUTTON_MIX)),
    );
    replace(
        &mut c.button_default_frame,
        button_bg
            .zip(button_fg)
            .zip(highlight)
            .map(|((bg, fg), hl)| {
                eight_bit(kcu_mix_of_mix(
                    rgba(hl),
                    rgba(bg),
                    rgba(fg),
                    frame_contrast,
                    DEFAULT_BUTTON_FRAME_MIX,
                ))
            }),
    );
    replace(
        &mut c.button_frame,
        button_bg
            .zip(button_fg)
            .map(|(bg, fg)| mix(bg, fg, frame_contrast)),
    );
    replace(
        &mut c.checkbox_checked,
        button_bg
            .zip(highlight)
            .map(|(bg, hl)| over(translucent(hl, BLEND_VALUE), bg)),
    );
    replace(
        &mut c.checkbox_frame,
        window_bg
            .zip(window_fg)
            .map(|(bg, fg)| mix(bg, fg, frame_contrast)),
    );
    replace(
        &mut c.list_grid,
        window_bg.map(|bg| eight_bit(kde_shade(rgba(bg), KdeShadeRole::Mid, contrast as f32))),
    );
    replace(
        &mut c.list_row_hover,
        highlight.map(|hl| translucent(hl, BLEND_VALUE)),
    );
    replace(
        &mut c.list_row_hover_frame,
        highlight.map(|hl| translucent(hl, LIST_HOVER_OUTLINE_ALPHA)),
    );
    replace(
        &mut c.list_header_hover,
        button_bg
            .zip(hover)
            .map(|(bg, hv)| mix(bg, hv, HEADER_SECTION_MIX)),
    );
    replace(
        &mut c.list_header_pressed,
        button_bg
            .zip(focus)
            .map(|(bg, fc)| mix(bg, fc, HEADER_SECTION_MIX)),
    );
    replace(
        &mut c.list_header_separator,
        window_fg.map(|fg| translucent(fg, frame_contrast as f32)),
    );
    replace(
        &mut c.scrollbar_thumb_hover,
        window_bg
            .zip(hover)
            .map(|(bg, hv)| overlay(bg, hv, SCROLLBAR_HANDLE_ALPHA)),
    );
    replace(
        &mut c.scrollbar_thumb,
        window_bg
            .zip(window_fg)
            .map(|(bg, fg)| overlay(bg, fg, SCROLLBAR_IDLE_HANDLE_ALPHA)),
    );
    c.scrollbar_thumb_bar_hover = window_bg
        .zip(window_fg)
        .map(|(bg, fg)| bar_hover_handle(bg, fg));
}

/// Breeze's handle of a view without the focus with the pointer on the bar
/// but off the handle, over the window `window` with the window text `text`:
/// the idle handle's overlay at the factor 1 ([`SCROLLBAR_BAR_HOVER_HANDLE_ALPHA`];
/// platform-facts.md (§2.8)).
fn bar_hover_handle(window: Color, text: Color) -> Color {
    overlay(window, text, SCROLLBAR_BAR_HOVER_HANDLE_ALPHA)
}

/// `value` in `slot` where given.
fn replace(slot: &mut Color, value: Option<Color>) {
    if let Some(value) = value {
        *slot = value;
    }
}

/// `c` as the kernels take a colour, `[r, g, b, a]`.
fn rgba(c: Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

/// A kernel's colour held in 8 bits, as 0.7.2's reader holds every colour it
/// derives (`Rgba::from_f32`: each channel clamped, times 255, rounded).
fn eight_bit([r, g, b, a]: [f32; 4]) -> Color {
    let [r, g, b, a] = Color::new(r, g, b, a).to_srgb8();
    Color::from_srgb8(r, g, b, a)
}

/// `KColorUtils::mix(a, b, bias)` in 8 bits.
fn mix(a: Color, b: Color, bias: f64) -> Color {
    eight_bit(kcu_mix_qreal(rgba(a), rgba(b), bias))
}

/// `c` at `alpha`, held in 8 bits.
fn translucent(c: Color, alpha: f32) -> Color {
    eight_bit(rgba(c.with_alpha(alpha)))
}

/// Breeze's keyboard focus band in the selection colour `highlight`: that
/// colour at [`BLEND_VALUE`] (breezestyle.cpp:6776-6779, breezemetrics.h:176
/// at breeze f0b1d75; docs/platform-facts.md §2.1.5).
fn focus_ring_of(highlight: Color) -> Color {
    translucent(highlight, BLEND_VALUE)
}

/// `top` painted source-over on the opaque `bottom`, in 8 bits.
fn over(top: Color, bottom: Color) -> Color {
    eight_bit(rgba(top.composite_over(bottom)))
}

/// `KColorUtils::overlayColors(under, paint)` of `paint` given the alpha
/// `alpha`, in 8 bits.
fn overlay(under: Color, paint: Color, alpha: f32) -> Color {
    eight_bit(kcu_overlay(rgba(under), rgba(paint.with_alpha(alpha))))
}

/// KColorScheme's inactive Selection colour of a role of `kind`, before any
/// inactive effect ([`kde_selection_inactive`]), where every colour the
/// kernel reads on its branch is given: the role's own `selection` colour
/// where `change` is false; else `window`, and for a background role
/// `selection_background` too. A colour the branch does not read may be
/// missing (0.7.2's `inactive_selection`); the kernel is handed a read one in
/// its place.
fn inactive_selection(
    selection: Option<Color>,
    window: Option<Color>,
    selection_background: Option<Color>,
    kind: KdeRoleKind,
    change: bool,
) -> Option<Color> {
    let (own, window, selection_background) = if !change {
        let own = selection?;
        (own, own, own)
    } else if kind == KdeRoleKind::Background {
        let (window, selection_background) = (window?, selection_background?);
        (window, window, selection_background)
    } else {
        let window = window?;
        (window, window, window)
    };
    Some(eight_bit(kde_selection_inactive(
        rgba(own),
        rgba(window),
        rgba(selection_background),
        kind,
        change,
    )))
}

// ---------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------

/// The contrast levels the preset defines (spec §1, "Axis values without
/// data"): `Normal` only, as the module documentation says.
pub(crate) const CONTRAST_LEVELS: &[Contrast] = &[Contrast::Normal];

/// The theme of `kde-breeze` for `ctx.scheme`, from the preset's constants
/// and the system's settings in `inputs`, where given.
// `inputs.palette` and `inputs.accent` are not applied yet: the palette's
// and the accent's derivations (`kde_accent`, spec §6, resolution step 2)
// are the KDE unit of the plan's M2.
pub(crate) fn resolve(ctx: &ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let mut scheme_colors = match ctx.scheme {
        Scheme::Light => LIGHT,
        Scheme::Dark => DARK,
    };
    scheme_colors.scrollbar_thumb_bar_hover = Some(bar_hover_handle(
        scheme_colors.window_background,
        scheme_colors.window_foreground,
    ));
    let system = inputs.system;
    let os = system.and_then(|s| s.colors_for(ctx.scheme, ctx.contrast));
    if let Some(os) = os {
        lay_system_over(&mut scheme_colors, os);
    }
    let c = &scheme_colors;
    let colors = roles(c, os, system.and_then(|s| s.accent));
    let typography = typography(&colors, system);
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
        cursor_size_px: system.and_then(|s| s.cursor_size_px),
        cursor_theme: system.and_then(|s| s.cursor_theme.clone()),
    };
    // The system's icon theme with its sizes, unless the system was read in
    // the other scheme (spec §6, resolution step 2).
    let (icon_theme, icon_sizes) = system
        .filter(|s| s.scheme.is_none_or(|scheme| scheme == ctx.scheme))
        .and_then(|s| Some((s.icon_theme.clone()?, s.icon_sizes)))
        .unwrap_or_else(|| (Arc::from(c.icon_theme), None));
    let icon_size = |pick: fn(&IconSizes) -> Option<f32>, preset: f32| {
        icon_sizes.as_ref().and_then(pick).unwrap_or(preset)
    };
    let unstated = nothing_stated(Preset::KdeBreeze, ctx.clone());
    ResolvedTheme {
        name: Arc::from(Preset::KdeBreeze.display_name()),
        preset: Preset::KdeBreeze,
        context: ctx.clone(),
        icons: IconThemeSpec {
            set: ICON_SET,
            theme: Some(icon_theme),
            // 0.7.2's `Resolved::icon_theme_explicit`: the TOML states the theme
            // (`src/model/resolved.rs`).
            theme_explicit: true,
            sizes: IconSizes::new(
                Some(icon_size(|s| s.toolbar, ICON_TOOLBAR)),
                Some(icon_size(|s| s.small, ICON_SMALL)),
                Some(icon_size(|s| s.large, ICON_LARGE)),
                Some(icon_size(|s| s.dialog, ICON_DIALOG)),
                Some(icon_size(|s| s.panel, ICON_PANEL)),
            ),
        },
        timings: UiTimings {
            tooltip_show_ms: Some(TOOLTIP_SHOW_MS),
            tooltip_hide_ms: None,
            tooltip_reshow_ms: None,
            double_click_ms: system.and_then(|s| s.double_click_ms),
            drag_threshold_px: system.and_then(|s| s.drag_threshold_px),
            scrollbar_auto_hide_ms: None,
            menu_hover_open_ms: None,
            key_repeat_delay_ms: None,
            key_repeat_interval_ms: None,
        },
        motion_tokens: MotionTokens {
            fast: None,
            standard: None,
            emphasized: None,
            spring_spatial: None,
            spring_effects: None,
            theme_change: None,
        },
        // 0.7.2's `defaults.focus_ring_*`: a ring.
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
        button: button(c, &colors, &typography, &layout),
        checkbox: checkbox(c, &colors, &typography, &layout),
        radio: radio(c, &colors, &typography, &layout),
        scrollbar: scrollbar(c),
        list: list(c, &colors, &typography, &layout),
        tooltip: tooltip(&colors, &typography, &layout),
        // Nothing stated yet: plan M2's KDE unit ("the rest of `kde-breeze`").
        window: unstated.window,
        input: unstated.input,
        text_area: unstated.text_area,
        link: unstated.link,
        slider: unstated.slider,
        progress_bar: unstated.progress_bar,
        splitter: unstated.splitter,
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
        colors,
        typography,
        layout,
    }
}

/// The colour roles of one scheme from its colours `c`, the system's colours
/// `os` for the scheme and contrast resolved for and the system's accent
/// `system_accent`, where given. The accent is `os`'s, else `system_accent`
/// (spec §8, "Accent"); a given accent is the accent's fill and the
/// selection's background (and through it the text selection's) and, at
/// [`BLEND_VALUE`], the focus ring's colour (0.7.2's `apply_kde_accent`); the
/// widgets' other colours keep `c`'s KDE colours. A focus ring colour `os`
/// states is the ring's colour as given, and each text selection colour it
/// states replaces the text selection's. The view's and the tooltip's
/// surfaces hold their KDE colour sets' colours (`[Colors:View]`,
/// `[Colors:Tooltip]`), the view's muted foreground and the disabled text's
/// colour `[Colors:View]` ForegroundInactive where the system states it (spec
/// §8, "Disabled text"), else the preset's disabled colour.
fn roles(c: &SchemeColors, os: Option<&OsColors>, system_accent: Option<Color>) -> ColorRoles {
    // `defaults.{danger,warning,success,info}_text_color`: platform-facts.md (§2.1.4), each
    // [Colors:Window] ForegroundNormal, the window's foreground.
    let status = |fill| StatusColors {
        fill,
        foreground: c.window_foreground,
        standalone: None,
        origin: RoleOrigin::Native,
    };
    let unstated = SurfaceColors::default();
    let accent = os.and_then(|os| os.accent).or(system_accent);
    let selection_background = accent.unwrap_or(c.selection_background);
    let text_selection = os.and_then(|os| os.text_selection).unwrap_or_default();
    ColorRoles {
        window: SurfaceColors::new(
            Some(c.window_background),
            Some(c.window_foreground),
            Some(c.window_foreground_muted),
            Some(c.window_border),
            None,
        ),
        view: SurfaceColors::new(
            Some(c.view_background),
            Some(c.view_foreground),
            c.view_foreground_muted,
            None,
            None,
        ),
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
            selection_background,
            c.selection_foreground,
            Some(c.selection_unfocused_background),
            Some(c.selection_unfocused_foreground),
        ),
        // 0.7.2: `text_selection_background` and `text_selection_color` are the selection's
        // where unstated (`src/resolve/inheritance.rs`, `resolve_defaults_internal`).
        text_selection: SelectionColors::new(
            text_selection.background.unwrap_or(selection_background),
            text_selection.foreground.unwrap_or(c.selection_foreground),
            text_selection.unfocused_background,
            text_selection.unfocused_foreground,
        ),
        find_highlight: None,
        scrim: None,
        // `defaults.accent_text_color`: platform-facts.md (§2.1.3), [Colors:Selection]
        // ForegroundNormal, the selection's foreground.
        accent: AccentColors {
            fill: accent.unwrap_or(c.accent),
            foreground: c.selection_foreground,
            standalone: None,
        },
        focus_ring: os
            .and_then(|os| os.focus_ring)
            .unwrap_or_else(|| accent.map_or(c.focus_ring, focus_ring_of)),
        caret: c.caret.unwrap_or(c.window_foreground),
        link: c.link,
        link_visited: c.link_visited,
        disabled_text: c.view_foreground_muted.unwrap_or(c.disabled_text),
        shadow: c.shadow,
        disabled_opacity: DISABLED_OPACITY,
        error: status(c.error),
        warning: status(c.warning),
        success: status(c.success),
        info: status(c.info),
    }
}

/// A font size in logical pixels at the font DPI `dpi`, rounded as Qt rounds
/// it where [`FONT_WHOLE_PIXELS`] (0.7.2: `src/resolve/validate.rs`, the
/// whole-pixel sizes).
fn font_px(size: FontSize, dpi: f32) -> f32 {
    let px = size.to_logical_px(dpi);
    if FONT_WHOLE_PIXELS {
        qt_whole_px(px)
    } else {
        px
    }
}

/// `size` times `factor`, in its unit.
fn scaled(size: FontSize, factor: f32) -> FontSize {
    match size {
        FontSize::Pt(v) => FontSize::Pt(v * factor),
        FontSize::Px(v) => FontSize::Px(v * factor),
    }
}

/// Qt's rounding of a font's pixel size: to hundredths, then to a whole
/// number, halves up (0.7.2's `qt_whole_px`, `src/resolve/validate_helpers.rs`;
/// `QFontDatabasePrivate::load`, qtbase v6.12.0 qfontdatabase.cpp:2865-2869).
fn qt_whole_px(px: f32) -> f32 {
    // Every f32 at or above 2^23 is whole already; scaling it by 100 could
    // overflow to infinity.
    if px.abs() >= 8_388_608.0 {
        return px;
    }
    ((px * 100.0).round() / 100.0).round()
}

/// The line height of a style of `size`, in logical pixels: the multiplier
/// times the size before whole-pixel rounding, in its unit, converted at the
/// font DPI `dpi` (0.7.2's `resolve_text_scale_entry`,
/// `src/resolve/inheritance.rs`, then its conversion in `validate`; spec §4,
/// `Typography`).
fn line_height_px(size: FontSize, dpi: f32) -> f32 {
    scaled(size, LINE_HEIGHT).to_logical_px(dpi)
}

/// A font as 0.7.2 resolves it: the system's font `os` laid over the
/// preset's `family`, `size`, `weight` and normal style field by field (a
/// field `os` does not give keeps the preset's, 0.7.2's merge); its family,
/// size, weight and style stated, the style normal where neither states one
/// (`src/resolve/validate_helpers.rs`, `require_font`); and `color`, where
/// given.
fn font(
    os: Option<&OsFont>,
    (family, size, weight): (&str, FontSize, u16),
    color: Option<Color>,
    dpi: f32,
) -> TextStyle {
    let size = os.and_then(|f| f.size).unwrap_or(size);
    let mut font = TextStyle {
        family: os
            .and_then(|f| f.family.clone())
            .unwrap_or_else(|| Arc::from(family)),
        size_px: font_px(size, dpi),
        defined_size: Some(size),
        weight: os.and_then(|f| f.weight).unwrap_or(weight),
        style: os.and_then(|f| f.style).unwrap_or(FontStyle::Normal),
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
fn text_scale_entry(size: FontSize, weight: u16, dpi: f32) -> TextStyle {
    TextStyle {
        size_px: font_px(size, dpi),
        defined_size: Some(size),
        weight,
        line_height: line_height_px(size, dpi),
        stated: TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::LINE_HEIGHT,
        ..TextStyle::default()
    }
}

/// The text styles, with the system's fonts in `system` laid over the
/// preset's (spec §8, "The KDE part"; 0.7.2's `kde/fonts.rs`,
/// `populate_fonts`, merged over the preset). The fonts' colour is the
/// window's foreground (0.7.2: `defaults.font.color` is
/// `defaults.text_color`, and `defaults.mono_font`'s is `defaults.font`'s,
/// where unstated: `resolve_defaults_internal`). The caption takes the
/// caption font's size and weight; where the UI font's size is given, the
/// headline and the first title are Kirigami's headings of it
/// ([`HEADING_LEVEL_2_FACTOR`], [`HEADING_LEVEL_1_FACTOR`],
/// [`HEADING_WEIGHT`]). The body's line height follows the text-scale rule
/// (spec §4); `display`, which 0.7.2 resolves from the body font, is `None`
/// and falls back to the body. Sizes convert at the system's font DPI where
/// given, else at [`FONT_DPI`].
fn typography(colors: &ColorRoles, system: Option<&SystemOverrides>) -> Typography {
    let dpi = system.and_then(|s| s.font_dpi).unwrap_or(FONT_DPI);
    let text = colors.window.foreground;
    let ui = system.and_then(|s| s.ui_font.as_ref());
    let ui_size = ui.and_then(|f| f.size);
    let body_size = ui_size.unwrap_or(FontSize::Pt(FONT_SIZE_PT));
    let mut body = font(ui, (FONT_FAMILY, body_size, FONT_WEIGHT), text, dpi);
    body.line_height = line_height_px(body_size, dpi);
    body.stated |= TextStated::LINE_HEIGHT;
    let heading = |factor: f32, (size, weight): (f32, u16)| match ui_size {
        Some(ui_size) => text_scale_entry(scaled(ui_size, factor), HEADING_WEIGHT, dpi),
        None => text_scale_entry(FontSize::Pt(size), weight, dpi),
    };
    let caption = system.and_then(|s| s.caption_font.as_ref());
    Typography {
        display: None,
        title1: Some(heading(
            HEADING_LEVEL_1_FACTOR,
            (TITLE1_SIZE_PT, TITLE1_WEIGHT),
        )),
        title2: None,
        title3: None,
        title4: None,
        headline: Some(heading(
            HEADING_LEVEL_2_FACTOR,
            (HEADLINE_SIZE_PT, HEADLINE_WEIGHT),
        )),
        body,
        callout: None,
        subheadline: None,
        caption: Some(text_scale_entry(
            caption
                .and_then(|f| f.size)
                .unwrap_or(FontSize::Pt(CAPTION_SIZE_PT)),
            caption.and_then(|f| f.weight).unwrap_or(CAPTION_WEIGHT),
            dpi,
        )),
        monospace: font(
            system.and_then(|s| s.monospace_font.as_ref()),
            (MONO_FAMILY, FontSize::Pt(MONO_SIZE_PT), MONO_WEIGHT),
            text,
            dpi,
        ),
        line_height_multiplier: LINE_HEIGHT,
        fallbacks: Arc::from([]),
        emoji_family: None,
        rendering: TextRendering::default(),
        font_dpi: dpi,
        font_whole_pixels: Some(FONT_WHOLE_PIXELS),
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

/// The push button. Breeze's button has one look in every variant but the
/// primary one, which 0.7.2 fills with the accent: platform-facts.md (§2.3),
/// "Breeze has no primary variant; the pair inherits the accent and its text,
/// the accent-filled button" (0.7.2: `primary_background` and
/// `primary_text_color` are `defaults.accent_color` and
/// `defaults.accent_text_color` where unstated). Its text is ButtonText in
/// every state but the disabled one (`button.active_background`'s citation),
/// and hover changes only its pen (`button.hover_background`: "no hover fill").
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
    let body = RuleBuilder::with_exclusive(PartStyle::default(), DISABLED_PRESSED_HOVER)
        .base([
            fill(c.button_background),
            outline(layout, c.button_frame),
            corners(layout),
            PartProp::Padding(Edges::symmetric(
                BUTTON_PADDING_VERTICAL,
                BUTTON_PADDING_HORIZONTAL,
            )),
            PartProp::MinSize([Some(BUTTON_MIN_WIDTH), None]),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [fill(c.button_checked)],
        )
        .rule(
            RuleSelector::on(WidgetState::DEFAULT),
            [
                fill(c.button_default),
                outline(layout, c.button_default_frame),
            ],
        )
        // `button.hover_border_color`: platform-facts.md (§2.3), "a non-flat button's pen turns
        // Highlight, [Colors:Selection] BackgroundNormal, while hovered, pressed or
        // keyboard-focused": KDE's selection colour, which a portal accent does not replace
        // (0.7.2's `apply_kde_accent` leaves this field on kdeglobals' colour).
        .rules(
            [
                RuleSelector::on(WidgetState::HOVER),
                RuleSelector::on(WidgetState::PRESSED),
                RuleSelector::on(WidgetState::FOCUS_VISIBLE),
            ],
            [outline(layout, c.selection_background)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.button_pressed)],
        )
        // The primary fill stays in every state but the disabled one, as 0.7.2's connectors
        // keep `primary_background` (egui: "regardless of state", gpui: its active colour
        // derived from it).
        .rule(primary(), [fill(roles.accent.fill)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.button_disabled_background)],
        )
        .build();
    let label = RuleBuilder::new(label(&typography.body, Some(c.button_foreground)))
        .rule(primary(), [TextProp::Color(roles.accent.foreground)])
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.button_disabled_foreground)],
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
/// `checkbox.*` serves both: Button's background in a separator-coloured
/// frame, no shadow; checked, the checked fill in KDE's selection colour
/// (`checkbox.border.color`: platform-facts.md (§2.5), "checked,
/// palette.highlight(), `[Colors:Selection]` BackgroundNormal"); under the
/// pointer, checked or not, the frame in `[Colors:View]` DecorationFocus
/// (`checkbox.hover_border_color`: platform-facts.md (§2.5), "focusColor(),
/// `[Colors:View]` DecorationFocus"), and no fill of its own ("Breeze adds no
/// fill under the pointer"); disabled, the disabled button's background. A
/// portal accent replaces neither frame colour (0.7.2's `apply_kde_accent`
/// leaves both fields on kdeglobals' colours).
fn indicator(c: &SchemeColors, layout: &LayoutMetrics) -> RuleBuilder<PartStyle, PartProp> {
    RuleBuilder::with_exclusive(PartStyle::default(), DISABLED_HOVER)
        .base([
            fill(c.button_background),
            outline(layout, c.checkbox_frame),
            // `checkbox.border.shadow_enabled`: platform-facts.md (§2.5): no shadow
            PartProp::Shadows(None),
            PartProp::MinSize([Some(CHECKBOX_INDICATOR_SIZE); 2]),
            PartProp::MaxSize([Some(CHECKBOX_INDICATOR_SIZE); 2]),
        ])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [
                fill(c.checkbox_checked),
                outline(layout, c.selection_background),
            ],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [outline(layout, c.accent)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [fill(c.button_disabled_background)],
        )
}

/// The label of a check box or a radio button: the UI font in its colour;
/// disabled, the check box's own disabled text colour, which the system's
/// disabled text does not replace (0.7.2's reader leaves
/// `checkbox.disabled_text_color` on the preset's value).
fn check_label(c: &SchemeColors, typography: &Typography) -> RuleSet<TextStyle, TextProp> {
    RuleBuilder::new(label(&typography.body, None))
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.checkbox_disabled_foreground)],
        )
        .build()
}

/// The check box: [`indicator`] with the default corners, a check mark in
/// the view's text colour, [`check_label`].
fn checkbox(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> CheckboxWidget {
    let checkmark = RuleBuilder::new(IconStyle::default())
        .base(roles.view.foreground.map(IconProp::Color))
        .base([IconProp::StrokeWidth(Some(CHECK_MARK_STROKE_WIDTH))])
        .build();
    CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator(c, layout).base([corners(layout)]).build(),
        checkmark,
        check_label(c, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The radio button: [`indicator`] (0.7.2 states no corners for it), a dot
/// in the view's text colour (`checkbox.indicator_color`), [`check_label`].
fn radio(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> RadioWidget {
    let dot = RuleBuilder::new(PartStyle::default())
        .base(roles.view.foreground.map(fill))
        .base([
            PartProp::MinSize([Some(RADIO_DOT_SIZE); 2]),
            PartProp::MaxSize([Some(RADIO_DOT_SIZE); 2]),
        ])
        .build();
    RadioWidget::new(
        WidgetOrigin::Native,
        indicator(c, layout).build(),
        dot,
        check_label(c, typography),
        CHECKBOX_LABEL_GAP,
        // 0.7.2: `checkbox.disabled_opacity` is `defaults.disabled_opacity` where unstated.
        Some(roles.disabled_opacity),
    )
}

/// The list or table view: the view's background in the default frame
/// (`list.background_color`: platform-facts.md (§2.15), `[Colors:View]`
/// BackgroundNormal, the source of the view role; 0.7.2: `list.border.color`
/// is `defaults.border.color` where unstated); rows with the alternate row's
/// fill; over them the highlight with Breeze's margins and radius (a table's
/// square), the hovered row's tint and outline, in a view without the focus
/// from the inactive selection colour, and the selection (0.7.2: `list.selection_background` and
/// `selection_text_color` are the selection's where unstated); the view's
/// text, which a hovered row keeps (`list.hover_text_color`: "←
/// item_font.color"); headers in Button's colours. 0.7.2 states no row or
/// header height ("sizes to content").
fn list(
    c: &SchemeColors,
    roles: &ColorRoles,
    typography: &Typography,
    layout: &LayoutMetrics,
) -> ListWidget {
    let container = RuleBuilder::new(PartStyle::default())
        .base(roles.view.background.map(fill))
        .base(roles.window.border.map(|border| outline(layout, border)))
        .base([corners(layout)])
        .build();
    let alternate = RuleSelector::on(WidgetState::NONE).matcher(ListMatcher {
        variant: None,
        alternate: Some(true),
    });
    let table = RuleSelector::on(WidgetState::NONE).matcher(ListMatcher {
        variant: Some(ListVariant::Table),
        alternate: None,
    });
    // `L2`: a view without the focus draws its rows from the Inactive group
    // (docs/issues/0004_kde-leftovers-rationale.md, L2), so its hovered row takes the inactive
    // selection colour at the alphas of a focused view's (E5): derived, the outline's measured
    // pixel exact and the fill's within one 8-bit step, a residual not explained.
    let unfocused_hover = WidgetState::HOVER.union(WidgetState::UNFOCUSED_VIEW);
    let inactive_highlight = c.selection_unfocused_background;
    let row = RuleBuilder::new(PartStyle::default())
        .rule(alternate, [fill(c.list_alternate_row)])
        .build();
    // `L4`: the highlight is inset by Breeze's margins; its corners are stated only by the rules
    // that draw one.
    let highlight_corners = PartProp::Corners(Corners::uniform(LIST_ROW_RADIUS));
    let highlight = RuleBuilder::new(PartStyle::default())
        .base([PartProp::Margin(Edges::symmetric(
            LIST_ROW_MARGIN_VERTICAL,
            LIST_ROW_MARGIN_HORIZONTAL,
        ))])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.list_row_hover), highlight_corners.clone()],
        )
        // `list.hover_border_color`: "the 1 px outline of a hovered, unselected row".
        .rule(
            RuleSelector::on(WidgetState::HOVER).except(WidgetState::SELECTED),
            [outline(layout, c.list_row_hover_frame)],
        )
        .rule(
            RuleSelector::on(unfocused_hover),
            [
                fill(translucent(inactive_highlight, BLEND_VALUE)),
                highlight_corners.clone(),
            ],
        )
        .rule(
            RuleSelector::on(unfocused_hover).except(WidgetState::SELECTED),
            [outline(
                layout,
                translucent(inactive_highlight, LIST_HOVER_OUTLINE_ALPHA),
            )],
        )
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [fill(roles.selection.background), highlight_corners],
        )
        .rule(table, [PartProp::Corners(Corners::NONE)])
        .build();
    let item_label = RuleBuilder::new(label(&typography.body, roles.view.foreground))
        .rule(
            RuleSelector::on(WidgetState::SELECTED),
            [TextProp::Color(roles.selection.foreground)],
        )
        .rule(
            RuleSelector::on(WidgetState::DISABLED),
            [TextProp::Color(c.view_disabled_foreground)],
        )
        .build();
    let header = RuleBuilder::with_exclusive(PartStyle::default(), PRESSED_HOVER)
        .base([fill(c.button_background)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(c.list_header_hover)],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(c.list_header_pressed)],
        )
        .build();
    let header_label = RuleBuilder::new(label(&typography.body, Some(c.button_foreground))).build();
    let header_separator = RuleBuilder::new(PartStyle::default())
        .base([fill(c.list_header_separator)])
        .build();
    ListWidget::new(
        WidgetOrigin::Native,
        container,
        row,
        highlight,
        item_label,
        header,
        header_label,
        header_separator,
        c.list_grid,
    )
}

/// The scroll bar: a groove without fill and a handle of exact thicknesses
/// across the bar; the handle at rest is the focused view's handle (spec §5,
/// `S3`) where stated, with 0.7.2's plain handle in a view without the focus,
/// else the plain handle in every view; in a view without the focus with the
/// pointer on the bar but off the handle, the handle of `S4` where stated;
/// the hovered one serves the pressed one too (`scrollbar.thumb_active_color`:
/// platform-facts.md (§2.8), "no pressed colour of its own, the hovered
/// one").
fn scrollbar(c: &SchemeColors) -> ScrollbarWidget {
    let exact = |builder: RuleBuilder<PartStyle, PartProp>, thickness: f32| {
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
    };
    let track = exact(
        RuleBuilder::new(PartStyle::default()).base([fill(c.scrollbar_track)]),
        SCROLLBAR_GROOVE_WIDTH,
    )
    .build();
    let unfocused_view = c
        .scrollbar_thumb_focused
        .map(|_| RuleSelector::on(WidgetState::UNFOCUSED_VIEW));
    let thumb = exact(
        RuleBuilder::with_exclusive(PartStyle::default(), PRESSED_HOVER)
            .base([fill(c.scrollbar_thumb_focused.unwrap_or(c.scrollbar_thumb))]),
        SCROLLBAR_THUMB_WIDTH,
    )
    .rules(unfocused_view, [fill(c.scrollbar_thumb)])
    .rules(
        c.scrollbar_thumb_bar_hover.map(|_| {
            RuleSelector::on(WidgetState::HOVER_WITHIN.union(WidgetState::UNFOCUSED_VIEW))
        }),
        c.scrollbar_thumb_bar_hover.map(fill),
    )
    .rules(
        [
            RuleSelector::on(WidgetState::HOVER),
            RuleSelector::on(WidgetState::PRESSED),
        ],
        [fill(c.scrollbar_thumb_hover)],
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

/// The tooltip: the tooltip surface's colours in the default frame (0.7.2:
/// `tooltip.border` is `defaults.border` where unstated) with
/// ToolTip_FrameWidth's padding.
fn tooltip(roles: &ColorRoles, typography: &Typography, layout: &LayoutMetrics) -> TooltipWidget {
    let mut panel = PartStyle::default();
    let border = roles.window.border.map(|border| outline(layout, border));
    for prop in roles
        .tooltip
        .background
        .map(fill)
        .into_iter()
        .chain([
            corners(layout),
            PartProp::Padding(Edges::symmetric(
                TOOLTIP_PADDING_VERTICAL,
                TOOLTIP_PADDING_HORIZONTAL,
            )),
            PartProp::MaxSize([Some(TOOLTIP_MAX_WIDTH), None]),
        ])
        .chain(border)
    {
        panel.apply(&prop);
    }
    TooltipWidget::new(
        WidgetOrigin::Native,
        panel,
        label(&typography.body, roles.tooltip.foreground),
    )
}

#[cfg(test)]
mod tests;
