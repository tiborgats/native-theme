//! Connector parity of the whole mapping (docs/v0.8.0-plan.md, section 1, rule 2; M4, step 1):
//! what the connector hands to egui, the atlas the crate root's builder makes of the 0.7.2 model
//! and the one [`super::to_theme`] makes of the 0.8.0 model, is equal on every preset of
//! [`super::root_parity::PRESETS`], light and dark.
//!
//! The 0.7.2 side is resolved as [`super::root_parity::resolve`] resolves it, and built with
//! the default preferences, whose text scale is 1, as the 0.8.0 theme is resolved for, the
//! preset's `LayoutTheme` and no font plan, so both atlases measure the Body row on egui's
//! default faces. The 0.8.0 side is built from each scheme's theme and the same theme resolved
//! for a window that is not active.
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
//! Compared, per scheme of the two atlases: the base style, each role's `Normal` cell and its
//! `Selected`, `Disabled` and `SelectedDisabled` cells where the role has one of its own (the
//! atlas sharing the `Normal` cell, for `SelectedDisabled` the `Disabled` one, in place of a
//! cell a role has none of, which is then the same on both sides), each surface's frame, the
//! `WindowTitleBar`'s from the theme resolved for an inactive window as well, the focus ring and
//! the menu bar's titles. A difference is allowed only as a listed one, per leaf (a `*` segment standing for
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

use std::sync::Arc;

use egui::style::{WidgetVisuals, Widgets};
use native_theme::next::{
    IconStated, IconStyle, PartStated, Preset, ResolvedTheme, TextStated, TextStyle, ThemeContext,
    ThemePatch, WidgetState,
};

use super::build::{base_source, cell_source, frame_source, variant_source};
use super::root_parity::{PRESETS, resolve};
use crate::atlas::SchemeStyles;
use crate::convert::to_color32;
use crate::style::cells::CellSource;
use crate::style::frames::FrameSource;
use crate::style::line::{Line, LineAlign};
use crate::{AccessibilityPreferences, Role, RoleVariant, Surface};

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
    /// The slider's knob keeps its stated diameter in every state (`derived::slider_geometry`):
    /// the entry's expansion is the resting one less the width by which the entry's own
    /// outline is wider than the resting one's.
    Knob,
}

impl Cause {
    const ALL: [Self; 7] = [
        Self::Rule0,
        Self::StateKeepsBase,
        Self::Egui,
        Self::Base,
        Self::OverFill,
        Self::RestingStroke,
        Self::Knob,
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

const KNOB: &str = "the knob keeps its stated diameter in every state: each entry's expansion \
     is reckoned from its own outline width, which egui's own entries widen under the pointer \
     and on a press (WIDTH_RULE_0); 0.7.2 drew one width in every state";

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
        "Slider",
        ALL,
        ".visuals.widgets.*.expansion",
        Cause::Knob,
        KNOB,
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
    home: Home,
}

/// A 0.8.0 home a model gap leaves unstated for its state: its path, and the query whose
/// `from_state` bit says whether a rule states it for that state (spec §2).
struct Home {
    path: &'static str,
    stated: fn(&ResolvedTheme) -> bool,
}

/// A text style's or an icon's `from_state` bit of its colour.
trait ColorFromState {
    fn color_from_state(&self) -> bool;
}

impl ColorFromState for TextStyle {
    fn color_from_state(&self) -> bool {
        self.from_state.contains(TextStated::COLOR)
    }
}

impl ColorFromState for IconStyle {
    fn color_from_state(&self) -> bool {
        self.from_state.contains(IconStated::COLOR)
    }
}

/// The [`Home`] `<widget>.<part>(<STATE>).<property>`, `property` a part's `fill`,
/// `stroke.color` or `stroke.width`, or a text style's or an icon's `color`.
macro_rules! home {
    (@at $w:ident . $p:ident ($s:ident) . $($prop:ident).+, $stated:expr) => {
        Home {
            path: concat!(
                stringify!($w), ".", stringify!($p), "(", stringify!($s), ").",
                stringify!($($prop).+)
            ),
            stated: $stated,
        }
    };
    ($w:ident . $p:ident ($s:ident) . fill) => {
        home!(@at $w.$p($s).fill, |r: &ResolvedTheme| {
            r.$w.$p(WidgetState::$s).from_state.contains(PartStated::FILL)
        })
    };
    ($w:ident . $p:ident ($s:ident) . stroke . $prop:ident) => {
        home!(@at $w.$p($s).stroke.$prop, |r: &ResolvedTheme| {
            r.$w.$p(WidgetState::$s).from_state.contains(PartStated::STROKE)
        })
    };
    ($w:ident . $p:ident ($s:ident) . color) => {
        home!(@at $w.$p($s).color, |r: &ResolvedTheme| {
            r.$w.$p(WidgetState::$s).color_from_state()
        })
    };
}

impl Home {
    /// A patch with a rule for the home's state that states the home: a fill or colour as a
    /// marker colour, a stroke as a marker colour at a width no preset states.
    fn patch(&self) -> Result<ThemePatch, Box<dyn std::error::Error>> {
        let (widget, rest) = self.path.split_once('.').ok_or(self.path)?;
        let (part, rest) = rest.split_once('(').ok_or(self.path)?;
        let (state, property) = rest.split_once(").").ok_or(self.path)?;
        let value = if property.starts_with("stroke.") {
            "stroke = { width = 3.0, color = \"#ff00ff\" }".to_owned()
        } else {
            format!("{property} = \"#ff00ff\"")
        };
        Ok(ThemePatch::from_toml(&format!(
            "[[widgets.{widget}.rules]]\nstates = [\"{}\"]\n{part}.{value}\n",
            state.to_lowercase()
        ))?)
    }
}

/// Rule 0 on `presets`: `what`'s `path` shows egui's own where `home` is unstated.
const fn r0(
    what: &'static str,
    presets: &'static [&'static str],
    path: &'static str,
    home: Home,
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
        home!(button.body(HOVER).stroke.color),
    ),
    r0(
        "base",
        &[A, W, M, T, M3, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(button.body(PRESSED).stroke.color),
    ),
    r0(
        "base",
        &[M, T, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(button.label(HOVER).color),
    ),
    // the base style's `bg_fill` is the scroll bar's thumb (`base_style_from`, §5.9), the
    // button's fill its `weak_bg_fill`
    r0(
        "base",
        &[T],
        ".visuals.widgets.hovered.bg_fill",
        home!(scrollbar.thumb(HOVER).fill),
    ),
    r0(
        "base",
        &[W, T],
        ".visuals.widgets.active.bg_fill",
        home!(scrollbar.thumb(PRESSED).fill),
    ),
    r0(
        "base",
        &[T],
        ".visuals.widgets.active.weak_bg_fill",
        home!(button.body(PRESSED).fill),
    ),
    r0(
        "Button",
        &[K, M, T],
        ".visuals.widgets.active.fg_stroke.color",
        home!(button.label(PRESSED).color),
    ),
    r0(
        "Checkbox",
        &[K, T],
        ".visuals.widgets.hovered.bg_fill",
        home!(checkbox.indicator(HOVER).fill),
    ),
    r0(
        "Checkbox",
        ALL,
        ".visuals.widgets.active.bg_fill",
        home!(checkbox.indicator(PRESSED).fill),
    ),
    r0(
        "Checkbox",
        &[A, W, M, T, M3],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(checkbox.indicator(HOVER).stroke.color),
    ),
    r0(
        "Checkbox",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.bg_stroke.color",
        home!(checkbox.indicator(PRESSED).stroke.color),
    ),
    // the checked box's hover and press outline, the state asked alone (spec §2, `from_state`)
    r0(
        "Checkbox/Selected",
        &[A, W, M, T, M3],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(checkbox.indicator(HOVER).stroke.color),
    ),
    r0(
        "Checkbox/Selected",
        &[A, W, M, T, M3],
        ".visuals.widgets.active.bg_stroke.color",
        home!(checkbox.indicator(PRESSED).stroke.color),
    ),
    // the checked box's hover and press fill, the state asked alone (spec §2, `from_state`)
    r0(
        "Checkbox/Selected",
        &[K, T],
        ".visuals.widgets.hovered.bg_fill",
        home!(checkbox.indicator(HOVER).fill),
    ),
    r0(
        "Checkbox/Selected",
        ALL,
        ".visuals.widgets.active.bg_fill",
        home!(checkbox.indicator(PRESSED).fill),
    ),
    r0(
        "Checkbox",
        ALL,
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(checkbox.checkmark(HOVER).color),
    ),
    r0(
        "Checkbox",
        ALL,
        ".visuals.widgets.active.fg_stroke.color",
        home!(checkbox.checkmark(PRESSED).color),
    ),
    r0(
        "ComboBox",
        &[K, T, M3],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(combo_box.field(HOVER).fill),
    ),
    r0(
        "ComboBox",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        home!(combo_box.field(PRESSED).fill),
    ),
    r0(
        "ComboBox",
        &[K, M3],
        ".visuals.widgets.active.bg_stroke.color",
        home!(combo_box.field(PRESSED).stroke.color),
    ),
    r0(
        "ComboBox",
        &[K, A, W, M3],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(combo_box.label(HOVER).color),
    ),
    r0(
        "ComboBox",
        ALL,
        ".visuals.widgets.active.fg_stroke.color",
        home!(combo_box.label(PRESSED).color),
    ),
    r0(
        "Expander",
        &[K, W, T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(expander.header(HOVER).fill),
    ),
    r0(
        "Expander",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        home!(expander.header(PRESSED).fill),
    ),
    r0(
        "Expander",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(expander.header_label(HOVER).color),
    ),
    r0(
        "Expander",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(expander.header_label(PRESSED).color),
    ),
    r0(
        "Input",
        &[K, M3, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(input.field(PRESSED).stroke.color),
    ),
    r0(
        "Input",
        ALL,
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(input.text(HOVER).color),
    ),
    r0(
        "Input",
        &[K, A, W, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(input.text(PRESSED).color),
    ),
    r0(
        "List",
        &[M, T],
        ".visuals.widgets.hovered.bg_fill",
        home!(list.highlight(HOVER).fill),
    ),
    r0(
        "List",
        ALL,
        ".visuals.widgets.active.bg_fill",
        home!(list.highlight(PRESSED).fill),
    ),
    r0(
        "SegmentedControl",
        &[T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(segmented_control.segment(HOVER).fill),
    ),
    r0(
        "SegmentedControl",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        home!(segmented_control.segment(PRESSED).fill),
    ),
    r0(
        "SegmentedControl",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(segmented_control.label(HOVER).color),
    ),
    r0(
        "SegmentedControl",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(segmented_control.label(PRESSED).color),
    ),
    r0(
        "Sidebar",
        &[M, T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(sidebar.item(HOVER).fill),
    ),
    r0(
        "Sidebar",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        home!(sidebar.item(PRESSED).fill),
    ),
    r0(
        "Sidebar",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(sidebar.item_label(HOVER).color),
    ),
    r0(
        "Sidebar",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(sidebar.item_label(PRESSED).color),
    ),
    r0(
        "Slider",
        &[K, W, M, T],
        ".visuals.widgets.hovered.bg_fill",
        home!(slider.thumb(HOVER).fill),
    ),
    r0(
        "Slider",
        ALL,
        ".visuals.widgets.active.bg_fill",
        home!(slider.thumb(PRESSED).fill),
    ),
    r0(
        "Switch",
        &[T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(switch.track(HOVER).fill),
    ),
    r0(
        "Switch",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        home!(switch.track(PRESSED).fill),
    ),
    r0(
        "Tab",
        &[T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(tab.tab(HOVER).fill),
    ),
    r0(
        "Tab",
        ALL,
        ".visuals.widgets.active.weak_bg_fill",
        home!(tab.tab(PRESSED).fill),
    ),
    r0(
        "Tab",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(tab.label(PRESSED).color),
    ),
    r0(
        "Toolbar",
        &[K, A, W, M3, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(toolbar.button_label(HOVER).color),
    ),
    r0(
        "Toolbar",
        &[K, A, W, M, T, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(toolbar.button_label(PRESSED).color),
    ),
    Gap {
        diff: d(
            "Tab",
            &[M, T],
            ".visuals.selection.stroke.color",
            Cause::StateKeepsBase,
            STATE_KEEPS_BASE,
        ),
        home: home!(tab.label(SELECTED).color),
    },
    Gap {
        diff: d(
            "Splitter",
            &[W, T],
            ".visuals.widgets.hovered.fg_stroke.color",
            Cause::StateKeepsBase,
            STATE_KEEPS_BASE,
        ),
        home: home!(splitter.handle(HOVER).fill),
    },
    r0(
        "Splitter",
        ALL,
        ".visuals.widgets.active.fg_stroke.color",
        home!(splitter.handle(PRESSED).fill),
    ),
    r0(
        "base",
        &[A, M, T, M3, I],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(button.body(HOVER).stroke.width),
    ),
    r0(
        "base",
        &[A, M, T, M3, I],
        ".visuals.widgets.active.bg_stroke.width",
        home!(button.body(PRESSED).stroke.width),
    ),
    r0(
        "Checkbox",
        &[M3],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(checkbox.indicator(HOVER).stroke.width),
    ),
    r0(
        "Checkbox",
        &[M3],
        ".visuals.widgets.active.bg_stroke.width",
        home!(checkbox.indicator(PRESSED).stroke.width),
    ),
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

/// The variant cells compared, each by its name.
const VARIANTS: [(&str, RoleVariant); 3] = [
    ("Selected", RoleVariant::Selected),
    ("Disabled", RoleVariant::Disabled),
    ("SelectedDisabled", RoleVariant::SelectedDisabled),
];

/// Whether a role's cell in `variant` is a cell of its own, not the one the atlas shares in
/// its place: the `Normal` cell for `Selected` and `Disabled`, the `Disabled` one for
/// `SelectedDisabled`.
fn own_cell(styles: &SchemeStyles, role: Role, variant: RoleVariant) -> bool {
    let shared = match variant {
        RoleVariant::Normal => return false,
        RoleVariant::Selected | RoleVariant::Disabled => RoleVariant::Normal,
        RoleVariant::SelectedDisabled => RoleVariant::Disabled,
    };
    !Arc::ptr_eq(styles.cell(role, variant), styles.cell(role, shared))
}

/// The leaves of a role's `Selected` cell, where it has one of its own.
fn selected_leaves(styles: &SchemeStyles, role: Role) -> Option<Vec<(String, String)>> {
    own_cell(styles, role, RoleVariant::Selected)
        .then(|| leaves(&format!("{:#?}", styles.cell(role, RoleVariant::Selected))))
}

#[test]
fn the_atlas_agrees_on_both_models() -> Result<(), Box<dyn std::error::Error>> {
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
            g.diff.what, g.home.path
        ));
    }
    let prefs = AccessibilityPreferences::default();
    for &(key, preset) in PRESETS {
        let layout = native_theme::theme::Theme::preset(key)?.layout;
        let (old_light, new_light) = resolve(key, preset, false)?;
        let (old_dark, new_dark) = resolve(key, preset, true)?;
        let resolve_inactive = |ctx: ThemeContext| {
            native_theme::next::Theme::preset(preset).resolve(&ctx.with_window_active(false))
        };
        let light_inactive = resolve_inactive(ThemeContext::light());
        let dark_inactive = resolve_inactive(ThemeContext::dark());
        let old_atlas = crate::ThemeAtlas::builder(key, &old_light, &old_dark)
            .layout(&layout)
            .accessibility(&prefs)
            .build();
        let new_atlas = super::to_theme(&new_light, &light_inactive, &new_dark, &dark_inactive)?;
        for (is_dark, scheme, old, new, inactive) in [
            (
                false,
                egui::Theme::Light,
                &old_light,
                &new_light,
                &light_inactive,
            ),
            (
                true,
                egui::Theme::Dark,
                &old_dark,
                &new_dark,
                &dark_inactive,
            ),
        ] {
            let label = format!("{key}/{}", if is_dark { "dark" } else { "light" });
            let seen_before = seen.len();
            let (old_styles, new_styles) = (old_atlas.scheme(scheme), new_atlas.scheme(scheme));
            let (old_base, new_base) = (&*old_styles.base, &*new_styles.base);
            let new_src = base_source(new);
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
                Cause::StateKeepsBase | Cause::Base | Cause::RestingStroke | Cause::Knob => None,
            };
            let side = Side {
                what: "base",
                preset: key,
                label: &label,
                bases: None,
            };
            compare(
                &side,
                old_base,
                new_base,
                &base_expected,
                &mut failures,
                &mut seen,
            );
            let new_cells = cell_source(new);
            let new_variant_src = variant_source(new);
            for role in Role::all() {
                let what = format!("{role:?}");
                let old_cell = &**old_styles.cell(*role, RoleVariant::Normal);
                let new_cell = &**new_styles.cell(*role, RoleVariant::Normal);
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
                    Cause::Knob => {
                        let w = &new_cell.visuals.widgets;
                        let outline = entry(w, path)?.fg_stroke.width;
                        let rest = w.inactive.expansion + w.inactive.fg_stroke.width;
                        Some(format!("{:?}", rest - outline))
                    }
                };
                let side = Side {
                    what: &what,
                    preset: key,
                    label: &label,
                    bases: Some((&old_leaves, &new_leaves)),
                };
                compare(
                    &side,
                    old_cell,
                    new_cell,
                    &cell_expected,
                    &mut failures,
                    &mut seen,
                );
                let old_normal_leaves = leaves(&format!("{old_cell:#?}"));
                let old_selected_leaves = selected_leaves(old_styles, *role);
                let new_selected_leaves = selected_leaves(new_styles, *role);
                for (name, variant) in VARIANTS {
                    let what = format!("{role:?}/{name}");
                    let has = own_cell(old_styles, *role, variant);
                    if has != own_cell(new_styles, *role, variant) {
                        failures.push(format!("{label}: {what}: a cell on one model only"));
                    }
                    if !has {
                        continue;
                    }
                    let (o, n) = (
                        &**old_styles.cell(*role, variant),
                        &**new_styles.cell(*role, variant),
                    );
                    // `SelectedDisabled` is copied from `Selected`, the others from `Normal`
                    let (old_from, new_from) =
                        match (variant, &old_selected_leaves, &new_selected_leaves) {
                            (RoleVariant::SelectedDisabled, Some(o), Some(n)) => (o, n),
                            _ => (&old_normal_leaves, &new_cell_leaves),
                        };
                    let checked = variant != RoleVariant::Disabled;
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
                        Cause::RestingStroke | Cause::Knob => None,
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
            // a model gap seen in this scheme: its home unstated for its state on the preset
            let gaps = seen
                .get(seen_before..)
                .unwrap_or_default()
                .iter()
                .filter_map(|(i, _)| RULE_0_ON_A_PRESET.get(i.checked_sub(DIFFERENCES.len())?));
            for g in gaps {
                if (g.home.stated)(new) {
                    failures.push(format!(
                        "{label}: {} {}: a model gap, but a rule states {} for its state",
                        g.diff.what, g.diff.path, g.home.path
                    ));
                }
            }
            let new_frames = frame_source(new, inactive);
            let own_style = scheme.default_style();
            for surface in Surface::all() {
                let what = format!("{surface:?}");
                let old_frame = old_styles.frame(*surface);
                let new_frame = new_styles.frame(*surface);
                let preset_leaves = |base: &egui::Style| {
                    leaves(&format!("{:#?}", crate::style::egui_preset(*surface, base)))
                };
                let (old_preset, new_preset) = (preset_leaves(old_base), preset_leaves(new_base));
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
            let side = Side {
                what: "FocusRing",
                preset: key,
                label: &label,
                bases: None,
            };
            compare(
                &side,
                &old_styles.focus_ring,
                &new_styles.focus_ring,
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
                &old_styles.menu_bar,
                &new_styles.menu_bar,
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

/// The style a listed difference's `what` names in one scheme's styles: the base style, a
/// role's `Normal` cell, or its `Selected` one (`<Role>/Selected`).
fn style_of<'a>(styles: &'a SchemeStyles, what: &str) -> Option<&'a egui::Style> {
    if what == "base" {
        return Some(&styles.base);
    }
    let (role, variant) = match what.split_once('/') {
        Some((role, "Selected")) => (role, RoleVariant::Selected),
        Some(_) => return None,
        None => (what, RoleVariant::Normal),
    };
    let role = Role::all().iter().find(|r| format!("{r:?}") == role)?;
    Some(styles.cell(*role, variant))
}

/// Each model gap's home is one the mapping reads: on the first preset the gap lists, a rule
/// stating the home for its state ([`Home::patch`]) changes the gap's leaf in the light
/// scheme's atlas.
#[test]
fn each_model_gap_is_read_where_the_theme_states_it() -> Result<(), Box<dyn std::error::Error>> {
    let mut failures = Vec::new();
    for g in RULE_0_ON_A_PRESET {
        let key = g.diff.presets.first().ok_or(g.home.path)?;
        let (_, preset) = PRESETS.iter().find(|(k, _)| k == key).ok_or(g.home.path)?;
        let theme = native_theme::next::Theme::preset(*preset);
        let ctx = ThemeContext::light();
        let inactive = theme.resolve(&ctx.clone().with_window_active(false));
        let dark = theme.resolve(&ThemeContext::dark());
        let dark_inactive = theme.resolve(&ThemeContext::dark().with_window_active(false));
        let patched = theme.clone().with_patch(g.home.patch()?).resolve(&ctx);
        if !(g.home.stated)(&patched) {
            failures.push(format!("{}: the patch does not state it", g.home.path));
        }
        let leaf = |r: &ResolvedTheme| -> Result<Option<String>, Box<dyn std::error::Error>> {
            let atlas = super::to_theme(r, &inactive, &dark, &dark_inactive)?;
            let style =
                style_of(atlas.scheme(egui::Theme::Light), g.diff.what).ok_or(g.diff.what)?;
            Ok(leaf_at(&leaves(&format!("{style:#?}")), g.diff.path))
        };
        let (plain, stated) = (leaf(&theme.resolve(&ctx))?, leaf(&patched)?);
        if plain.is_none() || plain == stated {
            failures.push(format!(
                "{key} {} {}: {} stated for its state leaves it at {plain:?}",
                g.diff.what, g.diff.path, g.home.path
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    Ok(())
}
