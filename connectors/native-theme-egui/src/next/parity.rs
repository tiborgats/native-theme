//! Connector parity of the whole mapping (docs/v0.8.0-plan.md, section 1, rule 2; M4, step 1):
//! what the connector hands to egui, compiled from the 0.7.2 model by the crate root's path and
//! from the 0.8.0 model by [`super`], is equal on every preset of
//! [`super::root_parity::PRESETS`], light and dark.
//!
//! The 0.7.2 side is resolved as [`super::root_parity::resolve`] resolves it, and compiled with
//! the default preferences, whose text scale is 1, as the 0.8.0 theme is resolved for, and the
//! preset's `LayoutTheme`.
//!
//! The form of comparison per egui type the connector produces:
//!
//! - `egui::Style` derives `PartialEq` and `Debug`, and so do every type it holds that the
//!   mapping writes: `Visuals` (with `Widgets` and its five `WidgetVisuals`, `Selection`,
//!   `TextCursorStyle`, the two `Shadow`s), `Spacing` (with `ScrollStyle` and the `Margin`s),
//!   `Interaction`, `ScrollAnimation`, and the `text_styles` map of `FontId`s (egui 0.37.0
//!   `src/style.rs`). The base style and each role cell, in each of its four variants, are
//!   compared by `PartialEq`; a difference is reported by the lines in which their `Debug`
//!   forms (`{:#?}`) differ, so one listed difference names one leaf of the `Style`.
//! - `egui::Frame` derives `PartialEq` and `Debug` (egui 0.37.0 `src/containers/frame.rs`):
//!   each surface's frame is compared as the style is.
//! - The focus ring the install plugin paints is the crate's own `FocusRing` (its stroke, an
//!   `egui::Stroke`, its offset and whether it shows for the keyboard only), which derives
//!   `PartialEq` and `Debug`, compared as the style is.
//! - The fonts: `egui::FontDefinitions` are made from a `fonts::FontPlan`, which the
//!   application passes to the 0.7.2 builder; neither `to_theme` makes one, so none is
//!   compared. The text sizes the mapping writes are the `FontId`s of the styles above.
//! - The notes (`Note`) name the leaf whose value was sanitised, saturated or transparent, a
//!   0.7.2 leaf on one side and its 0.8.0 home on the other: not compared.
//!
//! Compared so far: the base style, each role's `Normal` cell and its `Selected`, `Disabled`
//! and `SelectedDisabled` cells where the role has them, each surface's frame, the
//! `WindowTitleBar`'s from the theme resolved for an inactive window as well, and the focus
//! ring. A difference is allowed only as a listed one, per leaf (a `*` segment standing for
//! any one, an entry naming the leaf itself taken first) and preset, with its cause, from which
//! the test computes the value the 0.8.0 side must hold there; an entry that differs nowhere
//! on a preset it names is stale and fails. A cell's leaf that holds its base style's value on
//! both sides differs as the base does, which the base's comparison lists and checks; a
//! variant cell's leaf that holds the value of the cell it is copied from (`Normal`, for
//! `SelectedDisabled` `Selected`) likewise, and a frame's leaf that holds egui's own frame's
//! over each side's base style.
//! A difference that is rule 0 on a state ([`Cause::Rule0`], [`Cause::StateKeepsBase`]) is a
//! model gap, listed in [`RULE_0_ON_A_PRESET`] with the 0.8.0 home the preset leaves unstated;
//! every other one in [`DIFFERENCES`].

use egui::style::{WidgetVisuals, Widgets};
use native_theme::next::{Preset, ThemeContext};

use super::build::{base_source, cell_source, focus_ring_source, frame_source, variant_source};
use super::root_parity::{PRESETS, resolve};
use crate::convert::to_color32;
use crate::plugin::{FocusRingSource, build_focus_ring_from};
use crate::style::base::{BaseSource, base_style_from};
use crate::style::cells::CellSource;
use crate::style::frames::{FrameSource, surface_frame_from};
use crate::style::line::{Line, LineAlign};
use crate::style::roles::{CellInput, MenuBarTitles, role_cell_from};
use crate::style::variants::{VariantSource, disabled_cell, selected_cell, selected_disabled_cell};
use crate::{AccessibilityPreferences, Role, Surface};

/// Why a leaf differs, which gives the value the 0.8.0 side holds there.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cause {
    /// The 0.8.0 theme states no value for a hovered or pressed state where egui's own entries
    /// change it from rest, and egui's own stays (spec §10, rule 0): egui's own style's value
    /// at the leaf. A model gap.
    Rule0,
    /// The 0.8.0 theme states no value for a state, and the role writes none there, so the
    /// cell keeps its base style's (the base cell is egui's treatment of the role): the 0.8.0
    /// base style's value at the leaf. A model gap.
    StateKeepsBase,
    /// The model has no home for the value, or the 0.8.0 theme states none at rest where the
    /// 0.7.2 resolver gives one, and egui's own stays (spec §10, rule 0): egui's own style's
    /// value at the leaf.
    Egui,
    /// The 0.8.0 theme states no value at rest for the role where the 0.7.2 resolver gives one,
    /// and the cell keeps its base style's: the 0.8.0 base style's value at the leaf.
    Base,
    /// The part's translucent border lies inside it (`Inside` or `Center`): the 0.7.2 stroke
    /// composited over the fill the entry paints (`W/M2/fill-stroke-decision.md`, the egui row;
    /// `style::line`).
    OverFill,
    /// The role keeps its resting stroke under the pointer and on a press
    /// (`roles::keeps_resting_stroke`), and its resting stroke is the base style's, the
    /// button's border over its fill: the 0.8.0 base style's resting stroke.
    RestingStroke,
    /// The pressed entry takes the hovered one's value by the mapping's own rule: the 0.8.0
    /// cell's hovered value, which its own entry lists.
    AsHovered,
}

impl Cause {
    const ALL: [Self; 7] = [
        Self::Rule0,
        Self::StateKeepsBase,
        Self::Egui,
        Self::Base,
        Self::OverFill,
        Self::RestingStroke,
        Self::AsHovered,
    ];

    /// Whether the difference records a model gap.
    fn is_gap(self) -> bool {
        matches!(self, Self::Rule0 | Self::StateKeepsBase)
    }
}

const K: &str = "kde-breeze";
const A: &str = "adwaita";
const W: &str = "windows-11";
const M: &str = "macos-monterey";
const T: &str = "macos-tahoe";
const M3: &str = "material-3";
const I: &str = "ios-13";

/// Every preset of [`PRESETS`], by key; the test asks that it is.
const ALL: &[&str] = &[K, A, W, M, T, M3, I];

/// A listed difference: what is compared, the presets' keys, the leaf's path in the `Debug`
/// form (a `*` segment matches any one), its cause, and the source of the cause.
struct Difference {
    what: &'static str,
    presets: &'static [&'static str],
    path: &'static str,
    cause: Cause,
    reason: &'static str,
}

const fn d(
    what: &'static str,
    presets: &'static [&'static str],
    path: &'static str,
    cause: Cause,
    reason: &'static str,
) -> Difference {
    Difference {
        what,
        presets,
        path,
        cause,
        reason,
    }
}

/// Rule 0 on a state: the preset states no rule for it, egui's own entries differ there from
/// rest, so egui's own value stays.
const STATE_RULE_0: &str = "the 0.8.0 theme states no rule for the state, and egui's own \
     entries change the property between rest and the state: egui's own (spec §10, rule 0); \
     0.7.2's resolver gave the resting value or another state's";

/// A state the role writes nothing into where the theme states no rule for it.
const STATE_KEEPS_BASE: &str = "the 0.8.0 theme states no rule for the state, and the role \
     writes nothing there: the cell keeps its base style's value (M4.E2c); 0.7.2's resolver \
     gave the resting value";

/// Rule 0 on the text-stroke width: the theme states none for any state, and egui's own
/// entries widen it under the pointer and on a press.
const WIDTH_RULE_0: &str = "the 0.8.0 theme states no text-stroke width, and egui's own \
     entries change it between rest and the state (egui 0.37.0 src/style.rs, `Widgets::dark` \
     and `Widgets::light`): egui's own (spec §10, rule 0); 0.7.2 copied the resting width";

/// The Apple and Windows borders lie inside their parts.
const INSIDE: &str = "the part's translucent border lies inside it in the 0.8.0 theme \
     (macOS's and WinUI's controls), where 0.7.2 stated no alignment";

const RESTING: &str = "the role keeps its resting stroke under the pointer and on a press, the \
     base style's: macOS's push-button border over its fill";

const MENU_CORNERS: &str = "the 0.8.0 theme states no corners for a menu item \
     (menu.item(NONE).corners), nor for the panel (menu.panel(NONE).corners, the table of \
     homes' home on material-3), where 0.7.2's resolver gave menu.border.corner_radius 0: the \
     cell keeps the base's";

const TAB_OUTLINE: &str = "the 0.8.0 theme states no corners, stroke colour or stroke width \
     for the selected tab (tab.tab(SELECTED), the table of homes' `tab.border` rows) where \
     0.7.2's resolver made them up (transparent, 0): the cell keeps the base's";

const STATUS_BAR_LABEL: &str = "the model has no hovered or pressed state for a status bar's \
     label (status_bar.label is one text style): egui's own (spec §10, rule 0); 0.7.2 copied \
     the resting colour";

const SPLITTER_DRAG: &str = "the splitter's drag line takes its hover colour \
     (`derived::splitter_strokes`, D7), which the 0.8.0 theme leaves unstated (the hover's \
     model gap)";

const WIDGET_STROKES: &str = ".visuals.widgets.*.bg_stroke.color";

/// A surface's border inside it, drawn over the surface's fill.
const FRAME_INSIDE: &str = "the surface's translucent border lies inside it in the 0.8.0 theme, \
     where 0.7.2 stated no alignment: drawn over the surface's fill";

const CARD_OUTLINE: &str = "the 0.8.0 theme states no stroke or corners for the card \
     (card.container(NONE)) where 0.7.2 gives a transparent zero-width outline and square \
     corners: the frame keeps egui's `Frame::group` over the 0.8.0 base style";

const DIFFERENCES: &[Difference] = &[
    d(
        "base",
        ALL,
        ".visuals.widgets.hovered.fg_stroke.width",
        Cause::Egui,
        WIDTH_RULE_0,
    ),
    d(
        "base",
        ALL,
        ".visuals.widgets.active.fg_stroke.width",
        Cause::Egui,
        WIDTH_RULE_0,
    ),
    d(
        "base",
        &[K],
        ".spacing.interact_size",
        Cause::Egui,
        "button.min_height: 0.7.2's 32 is the height at the default font, not a minimum \
         (issue #4 Z1); button.body(NONE).min_size[1] is unstated on kde-breeze (the table of \
         homes' listed difference)",
    ),
    d(
        "base",
        &[M, T],
        ".visuals.widgets.inactive.bg_stroke.color",
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "base",
        &[M, T],
        ".visuals.widgets.open.bg_stroke.color",
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "Button",
        &[M, T],
        ".visuals.widgets.noninteractive.bg_stroke.color",
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "Checkbox",
        &[W, M, T],
        WIDGET_STROKES,
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "Checkbox/Selected",
        &[W, M, T],
        WIDGET_STROKES,
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "Button/Disabled",
        &[M, T],
        ".visuals.widgets.inactive.bg_stroke.color",
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "ComboBox/Disabled",
        &[M],
        ".visuals.widgets.inactive.bg_stroke.color",
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "ComboBox",
        &[W, M, T],
        WIDGET_STROKES,
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "SegmentedControl",
        &[W, M, T],
        WIDGET_STROKES,
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "Expander",
        &[A, W, M, T],
        WIDGET_STROKES,
        Cause::OverFill,
        "the expander's frame (expander.container(NONE)) is a translucent stroke laid inside \
         it in the 0.8.0 theme, where 0.7.2 stated no alignment",
    ),
    d("Tab", &[W], WIDGET_STROKES, Cause::OverFill, INSIDE),
    d(
        "Tab",
        &[A, M, T, M3, I],
        WIDGET_STROKES,
        Cause::Base,
        TAB_OUTLINE,
    ),
    d(
        "Tab",
        &[M, T, I],
        ".visuals.widgets.*.bg_stroke.width",
        Cause::Base,
        TAB_OUTLINE,
    ),
    d(
        "Tab",
        &[A, M, T, M3, I],
        ".visuals.widgets.*.corner_radius.*",
        Cause::Base,
        TAB_OUTLINE,
    ),
    d(
        "Menu",
        &[A, W, M, T, I],
        ".visuals.widgets.*.corner_radius.*",
        Cause::Base,
        MENU_CORNERS,
    ),
    d(
        "StatusBar",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        Cause::Egui,
        STATUS_BAR_LABEL,
    ),
    d(
        "StatusBar",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        Cause::Egui,
        STATUS_BAR_LABEL,
    ),
    d(
        "Splitter",
        &[W, T],
        ".visuals.widgets.active.fg_stroke.color",
        Cause::AsHovered,
        SPLITTER_DRAG,
    ),
    d(
        "List",
        &[M, T],
        ".visuals.widgets.*.bg_stroke.color",
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "Scrollbar",
        &[M, T],
        WIDGET_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "Sidebar",
        &[M, T],
        WIDGET_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "Slider",
        &[M, T],
        WIDGET_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "StatusBar",
        &[M, T],
        WIDGET_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "Switch",
        &[M, T],
        WIDGET_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "Toolbar",
        &[M, T],
        WIDGET_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "Window",
        &[W, M, T],
        ".stroke.color",
        Cause::OverFill,
        FRAME_INSIDE,
    ),
    d(
        "Dialog",
        &[W, M, T],
        ".stroke.color",
        Cause::OverFill,
        FRAME_INSIDE,
    ),
    d(
        "Popover",
        &[A, W, M, T],
        ".stroke.color",
        Cause::OverFill,
        FRAME_INSIDE,
    ),
    d(
        "Tooltip",
        &[W, M, T],
        ".stroke.color",
        Cause::OverFill,
        FRAME_INSIDE,
    ),
    d("Card", &[W], ".stroke.color", Cause::OverFill, FRAME_INSIDE),
    d(
        "Card",
        &[A, M, T, M3, I],
        ".stroke.width",
        Cause::Base,
        CARD_OUTLINE,
    ),
    d(
        "Card",
        &[A, M, T, M3, I],
        ".stroke.color",
        Cause::Base,
        CARD_OUTLINE,
    ),
    d(
        "Card",
        &[M, T, I],
        ".corner_radius.*",
        Cause::Base,
        CARD_OUTLINE,
    ),
];

/// A model gap: a native preset that leaves a state's value unstated where the platform draws
/// one, so that the mapping shows egui's own or the base style's there (spec §10, rule 0) — a
/// listed difference whose cause is a gap's, and the 0.8.0 home (its state and property) the
/// presets leave unstated. Fixed in the presets by a facts task, not here.
struct Gap {
    diff: Difference,
    home: &'static str,
}

/// Rule 0 on `presets`: `what`'s `path` shows egui's own where `home` is unstated.
const fn r0(
    what: &'static str,
    presets: &'static [&'static str],
    path: &'static str,
    home: &'static str,
) -> Gap {
    Gap {
        diff: d(what, presets, path, Cause::Rule0, STATE_RULE_0),
        home,
    }
}

const RULE_0_ON_A_PRESET: &[Gap] = &[
    r0(
        "base",
        &[A, W, M, T, M3, I],
        ".visuals.widgets.hovered.bg_stroke.color",
        "button.body(HOVER).stroke.color",
    ),
    r0(
        "base",
        &[A, W, M, T, M3, I],
        ".visuals.widgets.active.bg_stroke.color",
        "button.body(PRESSED).stroke.color",
    ),
    r0(
        "base",
        &[M, T, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        "button.label(HOVER).color",
    ),
    // the base style's `bg_fill` is the scroll bar's thumb (`base_style_from`, §5.9), the
    // button's fill its `weak_bg_fill`
    r0(
        "base",
        &[T],
        ".visuals.widgets.hovered.bg_fill",
        "scrollbar.thumb(HOVER).fill",
    ),
    r0(
        "base",
        &[W, T],
        ".visuals.widgets.active.bg_fill",
        "scrollbar.thumb(PRESSED).fill",
    ),
    r0(
        "base",
        &[T],
        ".visuals.widgets.active.weak_bg_fill",
        "button.body(PRESSED).fill",
    ),
    r0(
        "Button",
        &[K, M, T],
        ".visuals.widgets.active.fg_stroke.color",
        "button.label(PRESSED).color",
    ),
    r0(
        "Checkbox",
        &[K, T],
        ".visuals.widgets.hovered.bg_fill",
        "checkbox.indicator(HOVER).fill",
    ),
    r0(
        "Checkbox",
        ALL,
        ".visuals.widgets.active.bg_fill",
        "checkbox.indicator(PRESSED).fill",
    ),
    r0(
        "Checkbox",
        &[A, W, M, T, M3],
        ".visuals.widgets.hovered.bg_stroke.color",
        "checkbox.indicator(HOVER).stroke.color",
    ),
    r0(
        "Checkbox",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.bg_stroke.color",
        "checkbox.indicator(PRESSED).stroke.color",
    ),
    // the checked box's hover and press outline, the state asked alone (spec §2, `from_state`)
    r0(
        "Checkbox/Selected",
        &[A, W, M, T, M3],
        ".visuals.widgets.hovered.bg_stroke.color",
        "checkbox.indicator(HOVER).stroke.color",
    ),
    r0(
        "Checkbox/Selected",
        &[A, W, M, T, M3],
        ".visuals.widgets.active.bg_stroke.color",
        "checkbox.indicator(PRESSED).stroke.color",
    ),
    // the checked box's hover and press fill, the state asked alone (spec §2, `from_state`)
    r0(
        "Checkbox/Selected",
        &[K, T],
        ".visuals.widgets.hovered.bg_fill",
        "checkbox.indicator(HOVER).fill",
    ),
    r0(
        "Checkbox/Selected",
        ALL,
        ".visuals.widgets.active.bg_fill",
        "checkbox.indicator(PRESSED).fill",
    ),
    r0(
        "Checkbox",
        ALL,
        ".visuals.widgets.hovered.fg_stroke.color",
        "checkbox.checkmark(HOVER).color",
    ),
    r0(
        "Checkbox",
        ALL,
        ".visuals.widgets.active.fg_stroke.color",
        "checkbox.checkmark(PRESSED).color",
    ),
    r0(
        "ComboBox",
        &[K, T, M3],
        ".visuals.widgets.hovered.weak_bg_fill",
        "combo_box.field(HOVER).fill",
    ),
    r0(
        "ComboBox",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        "combo_box.field(PRESSED).fill",
    ),
    r0(
        "ComboBox",
        &[K, M3],
        ".visuals.widgets.active.bg_stroke.color",
        "combo_box.field(PRESSED).stroke.color",
    ),
    r0(
        "ComboBox",
        &[K, A, W, M3],
        ".visuals.widgets.hovered.fg_stroke.color",
        "combo_box.label(HOVER).color",
    ),
    r0(
        "ComboBox",
        ALL,
        ".visuals.widgets.active.fg_stroke.color",
        "combo_box.label(PRESSED).color",
    ),
    r0(
        "Expander",
        &[K, W, T],
        ".visuals.widgets.hovered.weak_bg_fill",
        "expander.header(HOVER).fill",
    ),
    r0(
        "Expander",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        "expander.header(PRESSED).fill",
    ),
    r0(
        "Expander",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        "expander.header_label(HOVER).color",
    ),
    r0(
        "Expander",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        "expander.header_label(PRESSED).color",
    ),
    r0(
        "Input",
        &[K, M3, I],
        ".visuals.widgets.active.bg_stroke.color",
        "input.field(PRESSED).stroke.color",
    ),
    r0(
        "Input",
        ALL,
        ".visuals.widgets.hovered.fg_stroke.color",
        "input.text(HOVER).color",
    ),
    r0(
        "Input",
        &[K, A, W, M3],
        ".visuals.widgets.active.fg_stroke.color",
        "input.text(PRESSED).color",
    ),
    r0(
        "List",
        &[M, T],
        ".visuals.widgets.hovered.bg_fill",
        "list.highlight(HOVER).fill",
    ),
    r0(
        "List",
        ALL,
        ".visuals.widgets.active.bg_fill",
        "list.highlight(PRESSED).fill",
    ),
    r0(
        "SegmentedControl",
        &[T],
        ".visuals.widgets.hovered.weak_bg_fill",
        "segmented_control.segment(HOVER).fill",
    ),
    r0(
        "SegmentedControl",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        "segmented_control.segment(PRESSED).fill",
    ),
    r0(
        "SegmentedControl",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        "segmented_control.label(HOVER).color",
    ),
    r0(
        "SegmentedControl",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        "segmented_control.label(PRESSED).color",
    ),
    r0(
        "Sidebar",
        &[M, T],
        ".visuals.widgets.hovered.weak_bg_fill",
        "sidebar.item(HOVER).fill",
    ),
    r0(
        "Sidebar",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        "sidebar.item(PRESSED).fill",
    ),
    r0(
        "Sidebar",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        "sidebar.item_label(HOVER).color",
    ),
    r0(
        "Sidebar",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        "sidebar.item_label(PRESSED).color",
    ),
    r0(
        "Slider",
        &[K, W, M, T],
        ".visuals.widgets.hovered.bg_fill",
        "slider.thumb(HOVER).fill",
    ),
    r0(
        "Slider",
        &[K, W, M, T],
        ".visuals.widgets.active.bg_fill",
        "slider.thumb(PRESSED).fill",
    ),
    r0(
        "Switch",
        &[T],
        ".visuals.widgets.hovered.weak_bg_fill",
        "switch.track(HOVER).fill",
    ),
    r0(
        "Switch",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        "switch.track(PRESSED).fill",
    ),
    r0(
        "Tab",
        &[T],
        ".visuals.widgets.hovered.weak_bg_fill",
        "tab.tab(HOVER).fill",
    ),
    r0(
        "Tab",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        "tab.tab(PRESSED).fill",
    ),
    r0(
        "Tab",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        "tab.label(PRESSED).color",
    ),
    r0(
        "Toolbar",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        "toolbar.button_label(HOVER).color",
    ),
    r0(
        "Toolbar",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        "toolbar.button_label(PRESSED).color",
    ),
    Gap {
        diff: d(
            "Tab",
            &[M, T],
            ".visuals.selection.stroke.color",
            Cause::StateKeepsBase,
            STATE_KEEPS_BASE,
        ),
        home: "tab.label(SELECTED).color",
    },
    Gap {
        diff: d(
            "Splitter",
            &[W, T],
            ".visuals.widgets.hovered.fg_stroke.color",
            Cause::StateKeepsBase,
            STATE_KEEPS_BASE,
        ),
        home: "splitter.handle(HOVER).fill",
    },
];

/// Every listed difference: [`DIFFERENCES`], then the model gaps'.
fn listed() -> impl Iterator<Item = &'static Difference> {
    DIFFERENCES
        .iter()
        .chain(RULE_0_ON_A_PRESET.iter().map(|g| &g.diff))
}

/// Whether `path` is the leaf `pattern` names, a `*` segment standing for any one.
fn matches(pattern: &str, path: &str) -> bool {
    let (mut p, mut l) = (pattern.split('.'), path.split('.'));
    loop {
        match (p.next(), l.next()) {
            (None, None) => return true,
            (Some(a), Some(b)) if a == "*" || a == b => {}
            _ => return false,
        }
    }
}

/// A `Debug` form's leaves: each one's path and value.
type Leaves = [(String, String)];

/// Each line of a `{:#?}` form as the path of the field it holds
/// (`.visuals.widgets.hovered.bg_fill`) and the line's value, without its trailing comma.
fn leaves(debug: &str) -> Vec<(String, String)> {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut out = Vec::new();
    for line in debug.lines() {
        let indent = line.len().saturating_sub(line.trim_start().len());
        let text = line.trim();
        while stack.last().is_some_and(|(i, _)| *i >= indent) {
            stack.pop();
        }
        let (key, value) = match text.split_once(": ") {
            Some((k, v)) if !k.contains(' ') => (k.to_owned(), v.to_owned()),
            _ => (String::new(), text.to_owned()),
        };
        let mut path: Vec<&str> = stack.iter().map(|(_, k)| k.as_str()).collect();
        path.push(&key);
        let path = path.join(".");
        let value = value.trim_end_matches(',').to_owned();
        if value.ends_with('{') || value.ends_with('[') || value.ends_with('(') {
            stack.push((indent, key.clone()));
        }
        out.push((path, value));
    }
    out
}

/// The value at `path` among `leaves`.
fn leaf_at(leaves: &Leaves, path: &str) -> Option<String> {
    leaves
        .iter()
        .find(|(p, _)| p == path)
        .map(|(_, v)| v.clone())
}

/// One comparison: what is compared, the preset and scheme.
struct Side<'a> {
    what: &'a str,
    preset: &'static str,
    label: &'a str,
    /// For a role cell, the leaves of the base styles it was copied from, 0.7.2's and 0.8.0's.
    bases: Option<(&'a Leaves, &'a Leaves)>,
}

/// The leaves in which `old` and `new` differ where no entry lists the difference, or where
/// the 0.8.0 side's value is not the one the entry's cause gives (`expected`, `None` where the
/// cause does not apply); the listed ones `seen` records, by the entry's place in [`listed`].
/// An unlisted difference names the causes that would give its value.
fn compare(
    side: &Side<'_>,
    old: &impl std::fmt::Debug,
    new: &impl std::fmt::Debug,
    expected: &dyn Fn(&str, Cause) -> Option<String>,
    failures: &mut Vec<String>,
    seen: &mut Vec<(usize, &'static str)>,
) {
    let (old, new) = (leaves(&format!("{old:#?}")), leaves(&format!("{new:#?}")));
    let label = side.label;
    let what = side.what;
    if old.len() != new.len() {
        failures.push(format!(
            "{label}: {what}: the Debug forms differ in shape: {} lines against {}",
            old.len(),
            new.len()
        ));
    }
    for ((path, o), (new_path, n)) in old.iter().zip(&new) {
        if path != new_path {
            failures.push(format!("{label}: {what}: {path} against {new_path}"));
            return;
        }
        if o == n {
            continue;
        }
        if let Some((old_base, new_base)) = side.bases
            && leaf_at(old_base, path).as_ref() == Some(o)
            && leaf_at(new_base, path).as_ref() == Some(n)
        {
            continue;
        }
        // an entry naming the leaf itself before one whose pattern matches it
        let listing = |exact: bool| {
            listed().enumerate().find(|(_, d)| {
                d.what == what
                    && d.presets.contains(&side.preset)
                    && d.path.contains('*') != exact
                    && matches(d.path, path)
            })
        };
        let Some((i, d)) = listing(true).or_else(|| listing(false)) else {
            let causes: Vec<Cause> = Cause::ALL
                .into_iter()
                .filter(|c| expected(path, *c).as_ref() == Some(n))
                .collect();
            failures.push(format!(
                "{label}: {what}: {path}: 0.7.2 {o}, 0.8.0 {n}; the causes that give it: {causes:?}"
            ));
            continue;
        };
        let want = expected(path, d.cause);
        if want.as_ref() == Some(n) && !d.reason.is_empty() {
            seen.push((i, side.preset));
        } else {
            failures.push(format!(
                "{label}: {what}: {path}: listed ({:?}), but 0.8.0 {n}, the cause gives {want:?}",
                d.cause
            ));
        }
    }
}

/// The Body row height `Builder::build` measures for a source's Body size, on egui's
/// default faces.
fn row_height(src: &BaseSource) -> Option<f32> {
    let [_, body, ..] = src.sizes;
    body.and_then(|(size, _)| {
        crate::fonts::body_row_height(&egui::FontDefinitions::default(), size)
    })
}

/// The entry of `widgets` a `.visuals.widgets.<entry>.…` path names.
fn entry<'a>(widgets: &'a Widgets, path: &str) -> Option<&'a WidgetVisuals> {
    match path.split('.').nth(3)? {
        "noninteractive" => Some(&widgets.noninteractive),
        "inactive" => Some(&widgets.inactive),
        "hovered" => Some(&widgets.hovered),
        "active" => Some(&widgets.active),
        "open" => Some(&widgets.open),
        _ => None,
    }
}

/// Where a role's outline lies on the 0.8.0 side, and whether it is composited over the weak
/// fill (`Some(true)`), the strong one (`Some(false)`) or none: as the role writes it.
fn outline_of(role: Role, src: &CellSource) -> Option<(LineAlign, Option<bool>)> {
    Some(match role {
        Role::Button => (src.button.states.align, Some(true)),
        Role::Menu => (src.menu.outline.align, Some(true)),
        Role::Input => (src.input.outline.align, None),
        Role::Checkbox => (src.checkbox.outline.align, Some(false)),
        Role::Tab => (src.tab.outline.align, Some(true)),
        Role::ComboBox => (src.combo_box.outline.align, Some(true)),
        Role::SegmentedControl => (src.segmented_control.outline.align, Some(true)),
        Role::Expander => (src.expander.outline.align, Some(true)),
        _ => return None,
    })
}

/// Where a surface's stroke lies on the 0.8.0 side, for a frame that writes one.
fn frame_align(surface: Surface, src: &FrameSource) -> Option<LineAlign> {
    Some(match surface {
        Surface::Window => src.window.align,
        Surface::Dialog => src.dialog.align,
        Surface::Popover => src.popover.align,
        Surface::Tooltip => src.tooltip.align,
        Surface::Card => src.card.align,
        Surface::WindowTitleBar | Surface::Panel(_) | Surface::CentralPanel => return None,
    })
}

/// The variant cells compared, in [`variant_cells`]' order.
const VARIANTS: [&str; 3] = ["Selected", "Disabled", "SelectedDisabled"];

/// A role's `Selected`, `Disabled` and `SelectedDisabled` cells from its `Normal` cell, as
/// `style::compile` builds them; `None` where the role has none of its own.
fn variant_cells(
    role: Role,
    normal: &egui::Style,
    src: &VariantSource,
    scheme: egui::Theme,
    rule_0: bool,
) -> [Option<egui::Style>; 3] {
    let input = CellInput {
        scheme,
        row_height: None,
        defs: None,
        rule_0,
    };
    let mut notes = Vec::new();
    let selected = selected_cell(role, normal, src, &input, &mut notes);
    let both = selected
        .as_ref()
        .and_then(|s| selected_disabled_cell(role, s, src, &mut notes));
    let disabled = disabled_cell(role, normal, src, &mut notes);
    [selected, disabled, both]
}

#[test]
fn the_mapping_agrees_on_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut seen = Vec::new();
    if PRESETS.iter().map(|(key, _)| *key).ne(ALL.iter().copied()) {
        failures.push("ALL is not the presets compared".to_owned());
    }
    for d in DIFFERENCES.iter().filter(|d| d.cause.is_gap()) {
        failures.push(format!(
            "{} {}: a model gap's cause, listed outside RULE_0_ON_A_PRESET",
            d.what, d.path
        ));
    }
    for g in RULE_0_ON_A_PRESET.iter().filter(|g| !g.diff.cause.is_gap()) {
        failures.push(format!(
            "{} {}: not a model gap's cause",
            g.diff.what, g.home
        ));
    }
    for &(key, preset) in PRESETS {
        let layout = native_theme::theme::Theme::preset(key)?.layout;
        for (is_dark, scheme) in [(false, egui::Theme::Light), (true, egui::Theme::Dark)] {
            let (old, new) = resolve(key, preset, is_dark)?;
            let label = format!("{key}/{}", if is_dark { "dark" } else { "light" });
            let old_src = BaseSource::old(&old, &AccessibilityPreferences::default(), &layout);
            let new_src = base_source(&new);
            let old_base = base_style_from(
                scheme,
                row_height(&old_src),
                &old_src,
                false,
                &mut Vec::new(),
            );
            let new_base = base_style_from(
                scheme,
                row_height(&new_src),
                &new_src,
                true,
                &mut Vec::new(),
            );
            let own = leaves(&format!("{:#?}", scheme.default_style()));
            let old_leaves = leaves(&format!("{old_base:#?}"));
            let new_leaves = leaves(&format!("{new_base:#?}"));
            // `open` paints the resting fill (`OpenFrom::Inactive`); its `weak_bg_fill` is
            // then the title bar's
            let base_expected = |path: &str, cause: Cause| match cause {
                Cause::Rule0 | Cause::Egui => leaf_at(&own, path),
                Cause::OverFill => {
                    let w = &new_base.visuals.widgets;
                    let fill = if path.ends_with("open.bg_stroke.color") {
                        w.inactive.weak_bg_fill
                    } else {
                        entry(w, path)?.weak_bg_fill
                    };
                    let line = Line {
                        color: to_color32(old.button.border.color),
                        align: new_src.button.align,
                    };
                    Some(format!("{:?}", line.over(Some(fill))))
                }
                Cause::StateKeepsBase | Cause::Base | Cause::RestingStroke | Cause::AsHovered => {
                    None
                }
            };
            let side = Side {
                what: "base",
                preset: key,
                label: &label,
                bases: None,
            };
            compare(
                &side,
                &old_base,
                &new_base,
                &base_expected,
                &mut failures,
                &mut seen,
            );
            let defs = egui::FontDefinitions::default();
            let old_cells = CellSource::old(&old, &AccessibilityPreferences::default());
            let new_cells = cell_source(&new);
            let old_variant_src = VariantSource::old(&old);
            let new_variant_src = variant_source(&new);
            for role in Role::all() {
                let what = format!("{role:?}");
                let cell = |base: &egui::Style, src: &CellSource, rule_0, row| {
                    let input = CellInput {
                        scheme,
                        row_height: row,
                        defs: Some(&defs),
                        rule_0,
                    };
                    role_cell_from(*role, base, src, &input, &mut Vec::new())
                };
                let old_cell = cell(&old_base, &old_cells, false, row_height(&old_src));
                let new_cell = cell(&new_base, &new_cells, true, row_height(&new_src));
                let new_cell_leaves = leaves(&format!("{new_cell:#?}"));
                let pressed = |path: &str| {
                    (path.contains(".hovered.") || path.contains(".active."))
                        .then(|| path.replacen(".active.", ".hovered.", 1))
                };
                let cell_expected = |path: &str, cause: Cause| match cause {
                    Cause::Rule0 | Cause::Egui => leaf_at(&own, path),
                    Cause::StateKeepsBase | Cause::Base => leaf_at(&new_leaves, path),
                    Cause::OverFill => {
                        let (align, weak) = outline_of(*role, &new_cells)?;
                        let painted = entry(&new_cell.visuals.widgets, path)?;
                        let line = Line {
                            color: entry(&old_cell.visuals.widgets, path)?.bg_stroke.color,
                            align,
                        };
                        let fill = weak.map(|weak| {
                            if weak {
                                painted.weak_bg_fill
                            } else {
                                painted.bg_fill
                            }
                        });
                        Some(format!("{:?}", line.over(fill)))
                    }
                    Cause::RestingStroke => {
                        let rest = pressed(path)?.replacen(".hovered.", ".inactive.", 1);
                        leaf_at(&new_leaves, &rest)
                    }
                    Cause::AsHovered => path
                        .contains(".active.")
                        .then(|| pressed(path))
                        .flatten()
                        .and_then(|hovered| leaf_at(&new_cell_leaves, &hovered)),
                };
                let side = Side {
                    what: &what,
                    preset: key,
                    label: &label,
                    bases: Some((&old_leaves, &new_leaves)),
                };
                compare(
                    &side,
                    &old_cell,
                    &new_cell,
                    &cell_expected,
                    &mut failures,
                    &mut seen,
                );
                let old_normal_leaves = leaves(&format!("{old_cell:#?}"));
                let old_variants = variant_cells(*role, &old_cell, &old_variant_src, scheme, false);
                let new_variants = variant_cells(*role, &new_cell, &new_variant_src, scheme, true);
                let old_selected_leaves =
                    old_variants[0].as_ref().map(|s| leaves(&format!("{s:#?}")));
                let new_selected_leaves =
                    new_variants[0].as_ref().map(|s| leaves(&format!("{s:#?}")));
                for (i, (name, (o, n))) in VARIANTS
                    .iter()
                    .zip(old_variants.iter().zip(&new_variants))
                    .enumerate()
                {
                    let what = format!("{role:?}/{name}");
                    let (Some(o), Some(n)) = (o, n) else {
                        if o.is_some() != n.is_some() {
                            failures.push(format!("{label}: {what}: a cell on one model only"));
                        }
                        continue;
                    };
                    // `SelectedDisabled` is copied from `Selected`, the others from `Normal`
                    let (old_from, new_from) = match (i, &old_selected_leaves, &new_selected_leaves)
                    {
                        (2, Some(o), Some(n)) => (o, n),
                        _ => (&old_normal_leaves, &new_cell_leaves),
                    };
                    let n_leaves = leaves(&format!("{n:#?}"));
                    let checked = i != 1;
                    let variant_expected = |path: &str, cause: Cause| match cause {
                        Cause::Rule0 | Cause::Egui => leaf_at(&own, path),
                        Cause::StateKeepsBase | Cause::Base => leaf_at(&new_leaves, path),
                        Cause::OverFill => {
                            let (align, weak) = if checked {
                                (new_variant_src.checked.align, Some(false))
                            } else {
                                outline_of(*role, &new_cells)?
                            };
                            let painted = entry(&n.visuals.widgets, path)?;
                            let line = Line {
                                color: entry(&o.visuals.widgets, path)?.bg_stroke.color,
                                align,
                            };
                            let fill = weak.map(|weak| {
                                if weak {
                                    painted.weak_bg_fill
                                } else {
                                    painted.bg_fill
                                }
                            });
                            Some(format!("{:?}", line.over(fill)))
                        }
                        Cause::RestingStroke => None,
                        Cause::AsHovered => path
                            .contains(".active.")
                            .then(|| pressed(path))
                            .flatten()
                            .and_then(|hovered| leaf_at(&n_leaves, &hovered)),
                    };
                    let side = Side {
                        what: &what,
                        preset: key,
                        label: &label,
                        bases: Some((old_from, new_from)),
                    };
                    compare(&side, o, n, &variant_expected, &mut failures, &mut seen);
                }
            }
            let inactive = native_theme::next::Theme::preset(preset).resolve(
                &if is_dark {
                    ThemeContext::dark()
                } else {
                    ThemeContext::light()
                }
                .with_window_active(false),
            );
            let old_frames = FrameSource::old(&old, &layout);
            let new_frames = frame_source(&new, &inactive);
            let own_style = scheme.default_style();
            for surface in Surface::all() {
                let what = format!("{surface:?}");
                let frame = |base: &egui::Style, src: &FrameSource| {
                    surface_frame_from(*surface, base, src, scheme, &mut Vec::new())
                };
                let old_frame = frame(&old_base, &old_frames);
                let new_frame = frame(&new_base, &new_frames);
                let preset_leaves = |base: &egui::Style| {
                    leaves(&format!("{:#?}", crate::style::egui_preset(*surface, base)))
                };
                let (old_preset, new_preset) = (preset_leaves(&old_base), preset_leaves(&new_base));
                let own_preset = preset_leaves(&own_style);
                let align = frame_align(*surface, &new_frames);
                let frame_expected = |path: &str, cause: Cause| match (cause, align) {
                    (Cause::Egui, _) => leaf_at(&own_preset, path),
                    (Cause::Base, _) => leaf_at(&new_preset, path),
                    (Cause::OverFill, Some(align)) if path == ".stroke.color" => {
                        let line = Line {
                            color: old_frame.stroke.color,
                            align,
                        };
                        Some(format!("{:?}", line.over(Some(new_frame.fill))))
                    }
                    _ => None,
                };
                let side = Side {
                    what: &what,
                    preset: key,
                    label: &label,
                    bases: Some((&old_preset, &new_preset)),
                };
                compare(
                    &side,
                    &old_frame,
                    &new_frame,
                    &frame_expected,
                    &mut failures,
                    &mut seen,
                );
            }
            let old_ring = build_focus_ring_from(&FocusRingSource::old(&old), &mut Vec::new());
            let new_ring = build_focus_ring_from(&focus_ring_source(&new), &mut Vec::new());
            let side = Side {
                what: "FocusRing",
                preset: key,
                label: &label,
                bases: None,
            };
            compare(
                &side,
                &old_ring,
                &new_ring,
                &|_, _| None,
                &mut failures,
                &mut seen,
            );
            let side = Side {
                what: "MenuBar",
                preset: key,
                label: &label,
                bases: None,
            };
            compare(
                &side,
                &MenuBarTitles::of(&old_cells.menu),
                &MenuBarTitles::of(&new_cells.menu),
                &|_, _| None,
                &mut failures,
                &mut seen,
            );
        }
    }
    for preset in Preset::ALL {
        if !PRESETS.iter().any(|(_, p)| p == preset) {
            failures.push(format!("{preset:?}: not compared"));
        }
    }
    for (i, d) in listed().enumerate() {
        for key in d.presets {
            if !seen.iter().any(|(j, p)| *j == i && p == key) {
                failures.push(format!(
                    "{} {} on {key}: listed, but differs nowhere: remove the entry",
                    d.what, d.path
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
