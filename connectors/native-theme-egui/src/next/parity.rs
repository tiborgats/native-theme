//! Connector parity of the whole mapping (docs/v0.8.0-plan.md, section 1, rule 2; M4, step 1):
//! what the connector hands to egui, the atlas the crate root's builder makes of the 0.7.2 model
//! and the one [`super::to_theme`] makes of the 0.8.0 model, is equal on every preset of
//! [`super::root_parity::PRESETS`], light and dark.
//!
//! The 0.7.2 side is resolved as [`super::root_parity::resolve`] resolves it, and built with
//! the default preferences, whose text scale is 1, as the 0.8.0 theme is resolved for, the
//! preset's `LayoutTheme` and, with feature `system-fonts`, `fonts::FontPlan::from_system`'s
//! plan for the light theme, as `from_preset` builds it, so both atlases measure the Body row
//! on the faces [`super::to_theme`] installs (without the feature, on egui's default faces).
//! The 0.8.0 side is built from each scheme's theme and the same theme resolved for a window
//! that is not active.
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
//! - The fonts: the `egui::FontDefinitions` each atlas installs (`None` where it installs
//!   none) are compared by `PartialEq` (epaint 0.37.0 `src/text/font_data.rs`, `FontData`'s by
//!   index, tweak and bytes); a difference fails, as no listed one names them. The text sizes
//!   the mapping writes are the `FontId`s of the styles above.
//! - The notes (`Note`) name the leaf whose value was sanitised, saturated or transparent, a
//!   0.7.2 leaf on one side and its 0.8.0 home on the other: not compared.
//!
//! Compared, per preset, what the atlas holds once (its accessibility preferences, layout gaps,
//! icon set and fonts) and each scheme's icon theme, the 0.7.2 atlas given the icon set and
//! themes `from_preset` gives it; and per scheme of the two atlases: the base style, each role's
//! `Normal` cell and its
//! `Selected`, `Disabled` and `SelectedDisabled` cells where the role has one of its own (the
//! atlas sharing the `Normal` cell, for `SelectedDisabled` the `Disabled` one, in place of a
//! cell a role has none of, which is then the same on both sides), each surface's frame, the
//! `WindowTitleBar`'s from the theme resolved for an inactive window as well, the focus ring and
//! the menu bar's titles. A difference is allowed only as a listed one, per leaf (a `*` segment
//! standing for any one, a `{a,b}` segment for each it names, an entry naming the leaf itself
//! taken first) and preset, with its cause, from which the test computes the value the 0.8.0
//! side must hold there; an entry is stale and fails where a leaf it names on a preset it
//! names differs nowhere. A cell's leaf that holds its base style's value on both sides, and
//! that neither side's role writes (each side's role built over the other side's base style
//! holds that base's value there), differs as the base does, which the base's comparison lists
//! and checks; a variant cell's leaf that holds the value of the cell it is copied from
//! (`Normal`, for `SelectedDisabled` `Selected`) likewise, and a frame's leaf that holds egui's
//! own frame's over each side's base style. A leaf either side writes is compared.
//! A difference that is rule 0 on a state ([`Cause::Rule0`], [`Cause::StateKeepsBase`]) is a
//! model gap, listed in [`RULE_0_ON_A_PRESET`] with the 0.8.0 home the preset leaves unstated;
//! every other one in [`DIFFERENCES`].
//!
//! What no atlas holds, the crate root's functions, is compared in [`super::root_parity`]:
//! among them `default_button`, `input_margin`, `input_frame`, `text_area_margin`,
//! `text_area_frame`, the expander's arrow, `icons::icon_size`, and the fonts a plan makes
//! (`fonts::font_definitions`, with `system-fonts` of `FontPlan::from_system`'s plan).

use std::collections::HashSet;
use std::sync::Arc;

use egui::style::{WidgetVisuals, Widgets};
use native_theme::next::{
    Color, IconStated, IconStyle, PartStated, PartStyle, Preset, ResolvedTheme, Stroke, TextStated,
    TextStyle, ThemeContext, ThemePatch, WidgetState,
};
use native_theme::theme::ColorMode;

use super::build::{self, base_source, cell_source, variant_source};
use super::root_parity::{PRESETS, resolve};
use crate::atlas::SchemeStyles;
use crate::convert::{Rgba, button_padding_outside, padding_with_line, to_color32};
use crate::style::cells::CellSource;
use crate::style::frames::{FrameSource, surface_frame_from};
use crate::style::line::{Line, LineAlign};
use crate::style::roles::{CellInput, role_cell_from};
use crate::style::variants::{disabled_cell, selected_cell, selected_disabled_cell};
use crate::{AccessibilityPreferences, Role, RoleVariant, Surface};

/// Why a leaf differs, which gives the value the 0.8.0 side holds there.
#[derive(Clone, Copy, Debug)]
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
    /// The 0.8.0 preset states the value at the home, where 0.7.2's resolver gave another: the
    /// home's value as the builder converts it, a stroke colour over the fill its entry paints
    /// (`style::line`); no value where the home is unstated.
    Stated(Home),
    /// `spacing.button_padding` as the builder sums it from a part the 0.8.0 preset states
    /// (`style::base::button_padding_of`): each stated padding side plus the part's stated
    /// stroke width (`convert::padding_with_line`), a pair's unstated side targeting the
    /// starting style's component (egui's own on the base style, the base style's in a role
    /// cell); no value where the part states no padding side or no stroke.
    Padding(PaddingHome),
    /// The role writes the leaf alike on both models over a base style that differs: the 0.7.2
    /// role built over the 0.8.0 base style holds the 0.8.0 value (the base's own difference is
    /// listed on `base`).
    OverBase,
}

impl Cause {
    const ALL: [Self; 8] = [
        Self::Rule0,
        Self::StateKeepsBase,
        Self::Egui,
        Self::Base,
        Self::OverFill,
        Self::RestingStroke,
        Self::Knob,
        Self::OverBase,
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

const BUTTON_MIN_HEIGHT: &str = "button.min_height: 0.7.2's 32 is the height at the default \
     font, not a minimum (issue #4 Z1); button.body(NONE).min_size[1] is unstated on kde-breeze \
     (the table of homes' listed difference)";

/// A role's resting outline, beside its hovered and pressed one that rule 0 gives.
const OUTLINE_AT_REST: &str = ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.color";

/// The pressed outline a role keeps at rest; its hovered one is the base's on both models, the
/// 0.8.0 presets stating the button's hovered outline.
const STATE_STROKES: &str = ".visuals.widgets.active.bg_stroke.color";

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
        BUTTON_MIN_HEIGHT,
    ),
    d(
        "Button",
        &[K],
        ".spacing.interact_size",
        Cause::Egui,
        BUTTON_MIN_HEIGHT,
    ),
    d(
        "base",
        &[M],
        ".visuals.widgets.{inactive,open}.bg_stroke.color",
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "Button",
        &[M],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.color",
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
        &[M],
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
        &[W, M],
        OUTLINE_AT_REST,
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "SegmentedControl",
        &[W, M],
        OUTLINE_AT_REST,
        Cause::OverFill,
        INSIDE,
    ),
    d(
        "Expander",
        &[A, W, M, T],
        ".visuals.widgets.{noninteractive,hovered,active}.bg_stroke.color",
        Cause::OverFill,
        "the expander's frame (expander.container(NONE)) is a translucent stroke laid inside \
         it in the 0.8.0 theme, where 0.7.2 stated no alignment",
    ),
    d("Tab", &[W], OUTLINE_AT_REST, Cause::OverFill, INSIDE),
    d("Tab", &[M, I], WIDGET_STROKES, Cause::Base, TAB_OUTLINE),
    // hovered and pressed, the tab's outline on adwaita and material-3 is alike on both models
    d("Tab", &[A, M3], OUTLINE_AT_REST, Cause::Base, TAB_OUTLINE),
    d(
        "Tab",
        &[T],
        ".visuals.widgets.{noninteractive,open}.bg_stroke.color",
        Cause::Base,
        TAB_OUTLINE,
    ),
    d(
        "Tab",
        &[M, I],
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
        ".visuals.widgets.{inactive,hovered,active,open}.corner_radius.*",
        Cause::Base,
        MENU_CORNERS,
    ),
    d(
        "StatusBar",
        ALL,
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
        ".visuals.widgets.{hovered,active}.expansion",
        Cause::Knob,
        KNOB,
    ),
    d("List", &[M], STATE_STROKES, Cause::RestingStroke, RESTING),
    d(
        "Scrollbar",
        &[M],
        STATE_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d(
        "Sidebar",
        &[M],
        STATE_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d("Slider", &[M], STATE_STROKES, Cause::RestingStroke, RESTING),
    d(
        "StatusBar",
        &[M],
        STATE_STROKES,
        Cause::RestingStroke,
        RESTING,
    ),
    d("Switch", &[M], STATE_STROKES, Cause::RestingStroke, RESTING),
    d(
        "Toolbar",
        &[M],
        STATE_STROKES,
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
        &[M, T, I],
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
    d(
        "Scrollbar",
        &[M3],
        ".visuals.widgets.{inactive,hovered,active}.corner_radius.*",
        Cause::Base,
        THUMB_UNSTATED,
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

/// The part whose padding sides and stroke width a cell's `spacing.button_padding` sums, on a
/// resolved theme.
#[derive(Clone, Copy, Debug)]
struct PaddingHome {
    part: fn(&ResolvedTheme) -> PartStyle,
}

/// The leaf [`Cause::Padding`]'s part gives on `new` over the starting style's `over`, as
/// `Debug` writes it; `None` where the part states no padding side or no stroke.
fn padding_leaf(home: PaddingHome, new: &ResolvedTheme, over: egui::Vec2) -> Option<String> {
    let part = (home.part)(new);
    let width = super::stroke_width_of(&part)?;
    let [top, right, bottom, left] = super::padding_of(&part);
    if [top, right, bottom, left].iter().all(Option::is_none) {
        return None;
    }
    let padding = native_theme::theme::ResolvedPadding {
        top,
        right,
        bottom,
        left,
    };
    let sum = button_padding_outside(over, &padding_with_line(&padding, width));
    Some(format!("{sum:?}"))
}

/// A 0.8.0 home: its path, the query whose `from_state` bit says whether a rule states it for
/// its state (spec §2; at rest, whether the theme states it at all), and its value where so
/// stated, as the builder reads it.
#[derive(Clone, Copy, Debug)]
struct Home {
    path: &'static str,
    stated: fn(&ResolvedTheme) -> bool,
    value: fn(&ResolvedTheme) -> Option<HomeValue>,
}

/// A home's value as the builder reads it: a fill or a text colour, written as given; a stroke
/// colour, which the builder lays over the fill its entry paints; a stroke width; a corner
/// radius, which the builder rounds to egui's whole points (`convert::to_corner_radius`).
#[derive(Clone, Copy, Debug)]
enum HomeValue {
    Color(Rgba),
    Line(Rgba),
    Width(f32),
    Radius(f32),
}

/// Whether `bit` of a part is stated for `state`: by a rule for the state (`from_state`), or,
/// at rest, by the theme.
fn part_stated(part: &PartStyle, bit: PartStated, state: WidgetState) -> bool {
    if state == WidgetState::NONE {
        part.stated.contains(bit)
    } else {
        part.from_state.contains(bit)
    }
}

/// Whether an icon's stroke width is stated for `state`: by a rule for the state
/// (`from_state`), or, at rest, by the theme.
fn icon_width_stated(icon: &IconStyle, state: WidgetState) -> bool {
    if state == WidgetState::NONE {
        icon.stated.contains(IconStated::STROKE_WIDTH)
    } else {
        icon.from_state.contains(IconStated::STROKE_WIDTH)
    }
}

/// A part's stroke colour for `state` as the builder reads it: at rest the stated line
/// (`build::edge`), in a state also a stroke stated as none, transparent (`build::state_line`).
fn line_value(part: &PartStyle, state: WidgetState) -> Option<HomeValue> {
    let color = match part.stroke {
        Stroke::None if state != WidgetState::NONE => Some(build::rgba(Color::TRANSPARENT)),
        _ => build::line(part),
    };
    part_stated(part, PartStated::STROKE, state)
        .then_some(color)
        .flatten()
        .map(HomeValue::Line)
}

/// A text style's or an icon's colour: whether it is stated for `state`, and its value so.
trait ColorAt {
    fn color_stated(&self, state: WidgetState) -> bool;
    fn color(&self) -> Option<Rgba>;
}

impl ColorAt for TextStyle {
    fn color_stated(&self, state: WidgetState) -> bool {
        if state == WidgetState::NONE {
            self.stated.contains(TextStated::COLOR)
        } else {
            self.from_state.contains(TextStated::COLOR)
        }
    }

    fn color(&self) -> Option<Rgba> {
        build::text_color(self)
    }
}

impl ColorAt for IconStyle {
    fn color_stated(&self, state: WidgetState) -> bool {
        if state == WidgetState::NONE {
            self.stated.contains(IconStated::COLOR)
        } else {
            self.from_state.contains(IconStated::COLOR)
        }
    }

    fn color(&self) -> Option<Rgba> {
        super::reads::icon(self).map(build::rgba)
    }
}

/// The [`Home`] `<widget>.<part>(<STATE>).<property>`, `property` a part's `fill`,
/// `stroke.color`, `stroke.width` or `corners`, a text style's or an icon's `color`, or an icon's
/// `stroke_width`; `<STATE>` one
/// state or several joined by `|`.
macro_rules! home {
    (@at $w:ident . $p:ident ($($s:ident)|+) . $($prop:ident).+, $stated:expr, $value:expr) => {
        Home {
            path: concat!(
                stringify!($w), ".", stringify!($p), "(", stringify!($($s)|+), ").",
                stringify!($($prop).+)
            ),
            stated: $stated,
            value: $value,
        }
    };
    ($w:ident . $p:ident ($($s:ident)|+) . fill) => {
        home!(
            @at $w.$p($($s)|+).fill,
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                part_stated(&r.$w.$p(s), PartStated::FILL, s)
            },
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                let part = r.$w.$p(s);
                part_stated(&part, PartStated::FILL, s)
                    .then(|| build::fill(&part))
                    .flatten()
                    .map(HomeValue::Color)
            }
        )
    };
    ($w:ident . $p:ident ($($s:ident)|+) . stroke . color) => {
        home!(
            @at $w.$p($($s)|+).stroke.color,
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                part_stated(&r.$w.$p(s), PartStated::STROKE, s)
            },
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                line_value(&r.$w.$p(s), s)
            }
        )
    };
    ($w:ident . $p:ident ($($s:ident)|+) . stroke . width) => {
        home!(
            @at $w.$p($($s)|+).stroke.width,
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                part_stated(&r.$w.$p(s), PartStated::STROKE, s)
            },
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                let part = r.$w.$p(s);
                part_stated(&part, PartStated::STROKE, s)
                    .then(|| super::stroke_width_of(&part))
                    .flatten()
                    .map(HomeValue::Width)
            }
        )
    };
    ($w:ident . $p:ident ($($s:ident)|+) . corners) => {
        home!(
            @at $w.$p($($s)|+).corners,
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                part_stated(&r.$w.$p(s), PartStated::CORNERS, s)
            },
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                let part = r.$w.$p(s);
                part_stated(&part, PartStated::CORNERS, s)
                    .then(|| super::radius_of(&part))
                    .flatten()
                    .map(HomeValue::Radius)
            }
        )
    };
    ($w:ident . $p:ident ($($s:ident)|+) . stroke_width) => {
        home!(
            @at $w.$p($($s)|+).stroke_width,
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                icon_width_stated(&r.$w.$p(s), s)
            },
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                let icon = r.$w.$p(s);
                icon_width_stated(&icon, s)
                    .then_some(icon.stroke_width)
                    .flatten()
                    .map(HomeValue::Width)
            }
        )
    };
    ($w:ident . $p:ident ($($s:ident)|+) . color) => {
        home!(
            @at $w.$p($($s)|+).color,
            |r: &ResolvedTheme| r.$w.$p($(WidgetState::$s)|+).color_stated($(WidgetState::$s)|+),
            |r: &ResolvedTheme| {
                let s = $(WidgetState::$s)|+;
                let style = r.$w.$p(s);
                style
                    .color_stated(s)
                    .then(|| style.color())
                    .flatten()
                    .map(HomeValue::Color)
            }
        )
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
        } else if property == "stroke_width" {
            "stroke_width = 3.0".to_owned()
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

/// A value the 0.8.0 preset states where 0.7.2's resolver gave another.
const STATED: &str = "the 0.8.0 preset states the value at the home, from the source its \
     comment there cites (M2.FS4); 0.7.2's resolver gave the resting value, another state's, or \
     a layer laid over the resting fill";

const THUMB_CORNERS: &str = "the scroll bar's thumb and trough are rounded by the corners the \
     0.8.0 preset states for the thumb, a pill, egui's largest radius, which epaint clamps to \
     half the thumb's shorter side; 0.7.2 rounded them by defaults.border.corner_radius";

const THUMB_UNSTATED: &str = "the 0.8.0 theme states no corners for the scroll bar's thumb \
     (scrollbar.thumb(NONE).corners) where 0.7.2 rounded the thumb and trough by \
     defaults.border.corner_radius: the cell keeps the base's";

/// A button stroked as none at rest.
const NO_LINE_AT_REST: &str = "macos-tahoe states its push button's bezel stroked as none \
     (M2.FS4, measured): the width is 0 (spec §10, rule 2) and the colour under it egui's own, \
     which draws nothing; 0.7.2 stated a translucent hairline";

/// The preset states `what`'s `path` at `home` on `presets`.
const fn s(
    what: &'static str,
    presets: &'static [&'static str],
    path: &'static str,
    home: Home,
) -> Difference {
    d(what, presets, path, Cause::Stated(home), STATED)
}

/// The values the FS4 presets state at a home where 0.7.2's resolver gave another (M2.FS4b).
const STATED_ON_A_PRESET: &[Difference] = &[
    d(
        "Scrollbar",
        &[K],
        ".visuals.widgets.{inactive,hovered,active}.corner_radius.*",
        Cause::Stated(home!(scrollbar.thumb(NONE).corners)),
        THUMB_CORNERS,
    ),
    s(
        "base",
        &[M, T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(button.body(HOVER).fill),
    ),
    s(
        "base",
        &[M, T],
        ".visuals.widgets.active.weak_bg_fill",
        home!(button.body(PRESSED).fill),
    ),
    s(
        "base",
        &[T, I],
        ".visuals.widgets.{inactive,open}.bg_stroke.width",
        home!(button.body(NONE).stroke.width),
    ),
    d(
        "base",
        &[T],
        ".visuals.widgets.{inactive,open}.bg_stroke.color",
        Cause::Egui,
        NO_LINE_AT_REST,
    ),
    s(
        "base",
        &[I],
        ".visuals.widgets.{inactive,open}.bg_stroke.color",
        home!(button.body(NONE).stroke.color),
    ),
    s(
        "base",
        &[T, I],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(button.body(HOVER).stroke.width),
    ),
    s(
        "base",
        &[T, I],
        ".visuals.widgets.active.bg_stroke.width",
        home!(button.body(PRESSED).stroke.width),
    ),
    s(
        "base",
        &[M, T, M3, I],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(button.body(HOVER).stroke.color),
    ),
    s(
        "base",
        &[W, M, T, M3, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(button.body(PRESSED).stroke.color),
    ),
    s(
        "Button",
        &[M, T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(button.body(HOVER).fill),
    ),
    s(
        "Button",
        &[M, T],
        ".visuals.widgets.active.weak_bg_fill",
        home!(button.body(PRESSED).fill),
    ),
    s(
        "Button",
        &[M],
        ".visuals.widgets.active.fg_stroke.color",
        home!(button.label(PRESSED).color),
    ),
    s(
        "Button",
        &[T, I],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.width",
        home!(button.body(NONE).stroke.width),
    ),
    d(
        "Button",
        &[T],
        ".visuals.widgets.{inactive,open}.bg_stroke.color",
        Cause::Egui,
        NO_LINE_AT_REST,
    ),
    s(
        "Button",
        &[I],
        ".visuals.widgets.{inactive,open}.bg_stroke.color",
        home!(button.body(NONE).stroke.color),
    ),
    s(
        "Button",
        &[I],
        ".visuals.widgets.noninteractive.bg_stroke.color",
        home!(button.body(NONE).stroke.color),
    ),
    s(
        "Button",
        &[T, I],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(button.body(HOVER).stroke.width),
    ),
    s(
        "Button",
        &[T, I],
        ".visuals.widgets.active.bg_stroke.width",
        home!(button.body(PRESSED).stroke.width),
    ),
    s(
        "Button",
        &[M, T, M3, I],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(button.body(HOVER).stroke.color),
    ),
    s(
        "Button",
        &[W, M, T, M3, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(button.body(PRESSED).stroke.color),
    ),
    s(
        "Button/Disabled",
        &[I],
        ".visuals.widgets.inactive.bg_stroke.color",
        home!(button.body(NONE).stroke.color),
    ),
    s(
        "Checkbox",
        &[W],
        ".visuals.widgets.hovered.bg_fill",
        home!(checkbox.indicator(HOVER).fill),
    ),
    s(
        "Checkbox",
        &[K, A, W, M3, I],
        ".visuals.widgets.active.bg_fill",
        home!(checkbox.indicator(PRESSED).fill),
    ),
    s(
        "Checkbox",
        &[A, M, T, M3, I],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(checkbox.indicator(HOVER).stroke.color),
    ),
    s(
        "Checkbox",
        &[A, W, M, T, M3, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(checkbox.indicator(PRESSED).stroke.color),
    ),
    s(
        "Checkbox",
        &[A, I],
        ".visuals.widgets.active.bg_stroke.width",
        home!(checkbox.indicator(PRESSED).stroke.width),
    ),
    s(
        "Checkbox",
        &[W, I],
        ".visuals.widgets.active.fg_stroke.color",
        home!(checkbox.checkmark(PRESSED).color),
    ),
    s(
        "Checkbox/Selected",
        &[I],
        ".visuals.widgets.active.bg_fill",
        home!(checkbox.indicator(PRESSED | CHECKED).fill),
    ),
    s(
        "Checkbox/Disabled",
        &[M3],
        ".visuals.override_text_color.0",
        home!(checkbox.label(DISABLED).color),
    ),
    s(
        "Checkbox/Disabled",
        &[M3],
        ".visuals.widgets.inactive.fg_stroke.color",
        home!(checkbox.label(DISABLED).color),
    ),
    s(
        "Checkbox/SelectedDisabled",
        &[M3],
        ".visuals.override_text_color.0",
        home!(checkbox.label(CHECKED | DISABLED).color),
    ),
    s(
        "Checkbox/SelectedDisabled",
        &[M3],
        ".visuals.widgets.inactive.fg_stroke.color",
        home!(checkbox.checkmark(DISABLED | CHECKED).color),
    ),
    s(
        "ComboBox",
        &[K, A, W, I],
        ".visuals.widgets.active.weak_bg_fill",
        home!(combo_box.field(PRESSED).fill),
    ),
    s(
        "ComboBox",
        &[W, I],
        ".visuals.widgets.active.fg_stroke.color",
        home!(combo_box.label(PRESSED).color),
    ),
    s(
        "Expander",
        &[W, M3],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(expander.header(HOVER).fill),
    ),
    s(
        "Expander",
        &[A, W, M3],
        ".visuals.widgets.active.weak_bg_fill",
        home!(expander.header(PRESSED).fill),
    ),
    s(
        "Input",
        &[I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(input.field(PRESSED).stroke.color),
    ),
    s(
        "List",
        &[A, W, M3],
        ".visuals.widgets.active.bg_fill",
        home!(list.highlight(PRESSED).fill),
    ),
    s(
        "SegmentedControl",
        &[W, M3],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(segmented_control.segment(HOVER).fill),
    ),
    s(
        "SegmentedControl",
        &[A, W, M3],
        ".visuals.widgets.active.weak_bg_fill",
        home!(segmented_control.segment(PRESSED).fill),
    ),
    s(
        "SegmentedControl",
        &[W],
        ".visuals.widgets.active.fg_stroke.color",
        home!(segmented_control.label(PRESSED).color),
    ),
    s(
        "Sidebar",
        &[A, W, M3],
        ".visuals.widgets.active.weak_bg_fill",
        home!(sidebar.item(PRESSED).fill),
    ),
    s(
        "Sidebar",
        &[M3],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(sidebar.item_label(HOVER).color),
    ),
    s(
        "Sidebar",
        &[W, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(sidebar.item_label(PRESSED).color),
    ),
    s(
        "Switch",
        &[A, W, I],
        ".visuals.widgets.active.weak_bg_fill",
        home!(switch.track(PRESSED).fill),
    ),
    s(
        "Tab",
        &[A],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(tab.tab(HOVER).fill),
    ),
    s(
        "Tab",
        &[A, W, M3],
        ".visuals.widgets.active.weak_bg_fill",
        home!(tab.tab(PRESSED).fill),
    ),
    d(
        "Tab",
        &[T],
        ".visuals.widgets.noninteractive.bg_stroke.width",
        Cause::Base,
        "macos-tahoe's tab outline home (`tab.tab(SELECTED).stroke`) gives the cell no width \
         (`build::edge`): the cell keeps its base style's; 0.7.2's resolver gave the tab a \
         border width of 0",
    ),
    s(
        "Tab",
        &[W],
        ".visuals.widgets.active.fg_stroke.color",
        home!(tab.label(PRESSED).color),
    ),
    s(
        "Toolbar",
        &[W],
        ".visuals.widgets.active.fg_stroke.color",
        home!(toolbar.button_label(PRESSED).color),
    ),
    d(
        "base",
        &[T],
        ".spacing.button_padding",
        Cause::Padding(BUTTON_PADDING),
        PADDING_NO_LINE,
    ),
    d(
        "Button",
        &[T],
        ".spacing.button_padding",
        Cause::Padding(BUTTON_PADDING),
        PADDING_NO_LINE,
    ),
    d(
        "ComboBox",
        &[T],
        ".spacing.button_padding",
        Cause::Padding(COMBO_BOX_PADDING),
        "the pop-up button states its padding above, below and on the side away from its \
         arrow, each plus its 0.5 outline; the arrow's side is unstated, so the horizontal pair \
         targets the base style's `button_padding.x`, which macos-tahoe's push button stroked \
         as none (M2.FS4, measured) now leaves at its padding alone, 12 where 0.7.2's was 12.5",
    ),
    d(
        "List",
        &[T],
        ".spacing.button_padding",
        Cause::OverBase,
        "the list writes only the vertical padding, narrowed to fit its stated row height \
         (`roles::fit_row_padding`), alike on both models; the horizontal is the base style's, \
         which macos-tahoe's push button stroked as none (M2.FS4, measured) now leaves at its \
         padding alone",
    ),
    s(
        "Checkbox",
        &[I],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(checkbox.indicator(HOVER).stroke.width),
    ),
    s(
        "Checkbox",
        &[I],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.width",
        home!(checkbox.indicator(NONE).stroke.width),
    ),
    s(
        "Checkbox",
        &[I],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.color",
        home!(checkbox.indicator(NONE).stroke.color),
    ),
    // the checked box's hover and press outline, the combined state's where a rule states one
    // for the state alone (`build::variant_source`)
    s(
        "Checkbox/Selected",
        &[M, T, I],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(checkbox.indicator(HOVER | CHECKED).stroke.color),
    ),
    s(
        "Checkbox/Selected",
        &[M, T, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(checkbox.indicator(PRESSED | CHECKED).stroke.color),
    ),
    s(
        "Checkbox/Selected",
        &[I],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(checkbox.indicator(HOVER | CHECKED).stroke.width),
    ),
    s(
        "Checkbox/Selected",
        &[I],
        ".visuals.widgets.active.bg_stroke.width",
        home!(checkbox.indicator(PRESSED | CHECKED).stroke.width),
    ),
    s(
        "Checkbox/Selected",
        &[I],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.color",
        home!(checkbox.indicator(CHECKED).stroke.color),
    ),
    // the checked box's outline width, a stroke stated as none for `CHECKED` no width
    s(
        "Checkbox/Selected",
        &[I],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.width",
        home!(checkbox.indicator(CHECKED).stroke.width),
    ),
    // the checked box's outline, which the disabled checked cell keeps
    s(
        "Checkbox/SelectedDisabled",
        &[I],
        ".visuals.widgets.inactive.bg_stroke.color",
        home!(checkbox.indicator(CHECKED).stroke.color),
    ),
    // the disabled cell keeps the outline at rest over its disabled fill
    s(
        "Checkbox/Disabled",
        &[I],
        ".visuals.widgets.inactive.bg_stroke.color",
        home!(checkbox.indicator(NONE).stroke.color),
    ),
    s(
        "ComboBox",
        &[M],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(combo_box.field(HOVER).fill),
    ),
    s(
        "ComboBox",
        &[W, M, T, I],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(combo_box.field(HOVER).stroke.color),
    ),
    s(
        "ComboBox",
        &[W, M, T, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(combo_box.field(PRESSED).stroke.color),
    ),
    s(
        "ComboBox",
        &[T, I],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(combo_box.field(HOVER).stroke.width),
    ),
    s(
        "ComboBox",
        &[T, I],
        ".visuals.widgets.active.bg_stroke.width",
        home!(combo_box.field(PRESSED).stroke.width),
    ),
    s(
        "ComboBox",
        &[T, I],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.width",
        home!(combo_box.field(NONE).stroke.width),
    ),
    d(
        "ComboBox",
        &[T],
        ".visuals.widgets.{inactive,open}.bg_stroke.color",
        Cause::Egui,
        NO_OUTLINE_AT_REST,
    ),
    s(
        "ComboBox",
        &[I],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.color",
        home!(combo_box.field(NONE).stroke.color),
    ),
    // the disabled cell keeps the outline at rest, as the Button's does
    s(
        "ComboBox/Disabled",
        &[I],
        ".visuals.widgets.inactive.bg_stroke.color",
        home!(combo_box.field(NONE).stroke.color),
    ),
    s(
        "SegmentedControl",
        &[A, W, M, T],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(segmented_control.segment(HOVER).stroke.color),
    ),
    s(
        "SegmentedControl",
        &[A, W, T],
        ".visuals.widgets.active.bg_stroke.color",
        home!(segmented_control.segment(PRESSED).stroke.color),
    ),
    s(
        "SegmentedControl",
        &[A, W, T],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(segmented_control.segment(HOVER).stroke.width),
    ),
    s(
        "SegmentedControl",
        &[A, W, T],
        ".visuals.widgets.active.bg_stroke.width",
        home!(segmented_control.segment(PRESSED).stroke.width),
    ),
    s(
        "SegmentedControl",
        &[T],
        ".visuals.widgets.{noninteractive,inactive,open}.bg_stroke.width",
        home!(segmented_control.track(NONE).stroke.width),
    ),
    d(
        "SegmentedControl",
        &[T],
        ".visuals.widgets.{inactive,open}.bg_stroke.color",
        Cause::Egui,
        NO_OUTLINE_AT_REST,
    ),
    s(
        "Slider",
        &[A],
        ".visuals.widgets.hovered.bg_fill",
        home!(slider.thumb(HOVER).fill),
    ),
    s(
        "Slider",
        &[A],
        ".visuals.widgets.active.bg_fill",
        home!(slider.thumb(PRESSED).fill),
    ),
    s(
        "Splitter",
        &[K, A],
        ".visuals.widgets.noninteractive.bg_stroke.color",
        home!(splitter.handle(NONE).fill),
    ),
    s(
        "Splitter",
        &[K, A],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(splitter.handle(HOVER).fill),
    ),
    s(
        "Splitter",
        &[K, A, M3],
        ".visuals.widgets.active.fg_stroke.color",
        home!(splitter.handle(PRESSED).fill),
    ),
    s(
        "Tab",
        &[K, W],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(tab.tab(HOVER).stroke.color),
    ),
    s(
        "Tab",
        &[K, W],
        ".visuals.widgets.active.bg_stroke.color",
        home!(tab.tab(PRESSED).stroke.color),
    ),
    s(
        "Tab",
        &[K, W],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(tab.tab(HOVER).stroke.width),
    ),
    s(
        "Tab",
        &[K, W],
        ".visuals.widgets.active.bg_stroke.width",
        home!(tab.tab(PRESSED).stroke.width),
    ),
];

/// A pop-up button or a segmented control stroked as none at rest.
const NO_OUTLINE_AT_REST: &str = "macos-tahoe states the pop-up button's and the segmented \
     control's outline as none (M2.FS4, measured): the width is 0 (spec §10, rule 2) and the \
     colour under it egui's own, which draws nothing; 0.7.2 stated a translucent hairline";

/// `button.body(NONE)`, whose padding and stroke width the base style's and the Button cell's
/// `button_padding` sum.
const BUTTON_PADDING: PaddingHome = PaddingHome {
    part: |r| r.button.body(WidgetState::NONE),
};

/// `combo_box.field(NONE)`, whose padding and stroke width the ComboBox cell's
/// `button_padding` sums.
const COMBO_BOX_PADDING: PaddingHome = PaddingHome {
    part: |r| r.combo_box.field(WidgetState::NONE),
};

/// The push button's padding with no line round it.
const PADDING_NO_LINE: &str = "macos-tahoe states its push button's bezel stroked as none \
     (M2.FS4, measured): the padding is the stated sides plus a width of 0 (spec §10, rule 2), \
     where 0.7.2 added its 0.5 hairline";

const RULE_0_ON_A_PRESET: &[Gap] = &[
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
        "Checkbox",
        &[T],
        ".visuals.widgets.hovered.bg_fill",
        home!(checkbox.indicator(HOVER).fill),
    ),
    r0(
        "Checkbox",
        &[M, T],
        ".visuals.widgets.active.bg_fill",
        home!(checkbox.indicator(PRESSED).fill),
    ),
    // the checked box's hover and press fill, the state asked alone (spec §2, `from_state`)
    r0(
        "Checkbox/Selected",
        &[T],
        ".visuals.widgets.hovered.bg_fill",
        home!(checkbox.indicator(HOVER).fill),
    ),
    r0(
        "Checkbox/Selected",
        &[M, T],
        ".visuals.widgets.active.bg_fill",
        home!(checkbox.indicator(PRESSED).fill),
    ),
    r0(
        "Checkbox",
        &[M, T],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(checkbox.checkmark(HOVER).color),
    ),
    r0(
        "Checkbox",
        &[M, T],
        ".visuals.widgets.active.fg_stroke.color",
        home!(checkbox.checkmark(PRESSED).color),
    ),
    // the check mark is a line in the state's `fg_stroke`, which egui's own entries widen under
    // the pointer (egui 0.37.0 `src/style.rs`, `Widgets::light`, `Widgets::dark`)
    r0(
        "Checkbox",
        &[K, A, M3],
        ".visuals.widgets.hovered.fg_stroke.width",
        home!(checkbox.checkmark(HOVER).stroke_width),
    ),
    r0(
        "ComboBox",
        &[M, T],
        ".visuals.widgets.active.weak_bg_fill",
        home!(combo_box.field(PRESSED).fill),
    ),
    r0(
        "ComboBox",
        &[M, T],
        ".visuals.widgets.active.fg_stroke.color",
        home!(combo_box.label(PRESSED).color),
    ),
    r0(
        "Expander",
        &[T],
        ".visuals.widgets.hovered.weak_bg_fill",
        home!(expander.header(HOVER).fill),
    ),
    r0(
        "Expander",
        &[M, T, I],
        ".visuals.widgets.active.weak_bg_fill",
        home!(expander.header(PRESSED).fill),
    ),
    r0(
        "Expander",
        &[M, T, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(expander.header_label(HOVER).color),
    ),
    r0(
        "Expander",
        &[M, T],
        ".visuals.widgets.active.fg_stroke.color",
        home!(expander.header_label(PRESSED).color),
    ),
    r0(
        "Input",
        &[M, T],
        ".visuals.widgets.active.bg_stroke.color",
        home!(input.field(PRESSED).stroke.color),
    ),
    r0(
        "Input",
        &[M, T],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(input.text(HOVER).color),
    ),
    r0(
        "List",
        &[M, T],
        ".visuals.widgets.hovered.bg_fill",
        home!(list.highlight(HOVER).fill),
    ),
    r0(
        "List",
        &[M, T, I],
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
        &[M, T, I],
        ".visuals.widgets.active.weak_bg_fill",
        home!(segmented_control.segment(PRESSED).fill),
    ),
    r0(
        "SegmentedControl",
        &[I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(segmented_control.label(HOVER).color),
    ),
    r0(
        "SegmentedControl",
        &[M, T],
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
        &[M, T, I],
        ".visuals.widgets.active.weak_bg_fill",
        home!(sidebar.item(PRESSED).fill),
    ),
    r0(
        "Sidebar",
        &[I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(sidebar.item_label(HOVER).color),
    ),
    r0(
        "Sidebar",
        &[M, T],
        ".visuals.widgets.active.fg_stroke.color",
        home!(sidebar.item_label(PRESSED).color),
    ),
    r0(
        "Slider",
        &[M, T],
        ".visuals.widgets.hovered.bg_fill",
        home!(slider.thumb(HOVER).fill),
    ),
    r0(
        "Slider",
        &[M, T, I],
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
        &[M, T],
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
        &[M, T, I],
        ".visuals.widgets.active.weak_bg_fill",
        home!(tab.tab(PRESSED).fill),
    ),
    r0(
        "Tab",
        &[M, T],
        ".visuals.widgets.active.fg_stroke.color",
        home!(tab.label(PRESSED).color),
    ),
    r0(
        "Toolbar",
        &[M, T, I],
        ".visuals.widgets.hovered.fg_stroke.color",
        home!(toolbar.button_label(HOVER).color),
    ),
    r0(
        "Toolbar",
        &[M, T],
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
            &[T],
            ".visuals.widgets.hovered.fg_stroke.color",
            Cause::StateKeepsBase,
            STATE_KEEPS_BASE,
        ),
        home: home!(splitter.handle(HOVER).fill),
    },
    r0(
        "Splitter",
        &[M, T, I],
        ".visuals.widgets.active.fg_stroke.color",
        home!(splitter.handle(PRESSED).fill),
    ),
    r0(
        "Input",
        &[M, T],
        ".visuals.widgets.active.bg_stroke.width",
        home!(input.field(PRESSED).stroke.width),
    ),
    // the Scrollbar cell's thumb is the base style's `bg_fill` (`base_style_from`, §5.9)
    r0(
        "Scrollbar",
        &[T],
        ".visuals.widgets.hovered.bg_fill",
        home!(scrollbar.thumb(HOVER).fill),
    ),
    r0(
        "Scrollbar",
        &[W, T],
        ".visuals.widgets.active.bg_fill",
        home!(scrollbar.thumb(PRESSED).fill),
    ),
    r0(
        "SegmentedControl",
        &[I],
        ".visuals.widgets.hovered.bg_stroke.color",
        home!(segmented_control.segment(HOVER).stroke.color),
    ),
    r0(
        "SegmentedControl",
        &[M, I],
        ".visuals.widgets.active.bg_stroke.color",
        home!(segmented_control.segment(PRESSED).stroke.color),
    ),
    r0(
        "SegmentedControl",
        &[I],
        ".visuals.widgets.hovered.bg_stroke.width",
        home!(segmented_control.segment(HOVER).stroke.width),
    ),
    r0(
        "SegmentedControl",
        &[M, I],
        ".visuals.widgets.active.bg_stroke.width",
        home!(segmented_control.segment(PRESSED).stroke.width),
    ),
];

/// Every listed difference: [`DIFFERENCES`], the model gaps', then [`STATED_ON_A_PRESET`].
fn listed() -> impl Iterator<Item = &'static Difference> {
    DIFFERENCES
        .iter()
        .chain(RULE_0_ON_A_PRESET.iter().map(|g| &g.diff))
        .chain(STATED_ON_A_PRESET)
}

/// Whether `path` is the leaf `pattern` names, a `*` segment standing for any one and a
/// `{a,b}` segment for each it names.
fn matches(pattern: &str, path: &str) -> bool {
    let (mut p, mut l) = (pattern.split('.'), path.split('.'));
    loop {
        match (p.next(), l.next()) {
            (None, None) => return true,
            (Some(a), Some(b)) if a == "*" || a == b || one_of(a, b) => {}
            _ => return false,
        }
    }
}

/// Whether `segment` is one of those a `{a,b}` pattern segment names.
fn one_of(pattern: &str, segment: &str) -> bool {
    pattern
        .strip_prefix('{')
        .and_then(|p| p.strip_suffix('}'))
        .is_some_and(|names| names.split(',').any(|n| n == segment))
}

/// Whether a listed path names one leaf, not a pattern of several.
fn names_one(path: &str) -> bool {
    !path.contains('*') && !path.contains('{')
}

/// A `Debug` form's leaves: each one's path and value.
type Leaves = [(String, String)];

/// Each line of a `{:#?}` form as the path of the field it holds
/// (`.visuals.widgets.hovered.bg_fill`) and the line's value, without its trailing comma; an
/// element of an array or a tuple is named by its index (`.offset.0`, `.offset.1`).
fn leaves(debug: &str) -> Vec<(String, String)> {
    // each open container's indent, key, and the index of its next element where it is an
    // array or a tuple
    let mut stack: Vec<(usize, String, Option<usize>)> = Vec::new();
    let mut out = Vec::new();
    for line in debug.lines() {
        let indent = line.len().saturating_sub(line.trim_start().len());
        let text = line.trim();
        while stack.last().is_some_and(|(i, _, _)| *i >= indent) {
            stack.pop();
        }
        let (key, value) = match text.split_once(": ") {
            Some((k, v)) if !k.contains(' ') => (k.to_owned(), v.to_owned()),
            // a closing line is no element
            _ if text.starts_with(['}', ']', ')']) => (String::new(), text.to_owned()),
            _ => match stack.last_mut().and_then(|(_, _, next)| next.as_mut()) {
                Some(next) => {
                    let index = next.to_string();
                    *next = next.saturating_add(1);
                    (index, text.to_owned())
                }
                None => (String::new(), text.to_owned()),
            },
        };
        let mut path: Vec<&str> = stack.iter().map(|(_, k, _)| k.as_str()).collect();
        path.push(&key);
        let path = path.join(".");
        let value = value.trim_end_matches(',').to_owned();
        if value.ends_with('{') {
            stack.push((indent, key.clone(), None));
        } else if value.ends_with('[') || value.ends_with('(') {
            stack.push((indent, key.clone(), Some(0)));
        }
        out.push((path, value));
    }
    out
}

/// An array's elements are leaves of their own, each named by its index, and a struct inside
/// an array is named by its index too.
#[test]
fn an_array_element_is_named_by_its_index() {
    let shadow = egui::Shadow {
        offset: [2, 4],
        blur: 18,
        spread: 3,
        color: egui::Color32::GREEN,
    };
    let form = leaves(&format!("{:#?}", [shadow, egui::Shadow::NONE]));
    assert_eq!(leaf_at(&form, ".0.offset.0").as_deref(), Some("2"));
    assert_eq!(leaf_at(&form, ".0.offset.1").as_deref(), Some("4"));
    assert_eq!(leaf_at(&form, ".1.offset.0").as_deref(), Some("0"));
    assert_eq!(leaf_at(&form, ".1.blur").as_deref(), Some("0"));
}

/// The value at `path` among `leaves`.
fn leaf_at(leaves: &Leaves, path: &str) -> Option<String> {
    leaves
        .iter()
        .find(|(p, _)| p == path)
        .map(|(_, v)| v.clone())
}

/// What a compared value was copied from on each side, and the same build over the other
/// side's: a role cell's base styles, a variant cell's `Normal` (or `Selected`) cell, a
/// frame's egui preset over each side's base style.
struct Bases<'a> {
    old: &'a Leaves,
    new: &'a Leaves,
    /// The 0.7.2 side's build over the 0.8.0 side's base, and the 0.8.0 side's over the 0.7.2
    /// side's; `None` where it cannot be built, which compares every leaf.
    swapped: Option<(&'a Leaves, &'a Leaves)>,
}

impl Bases<'_> {
    /// Whether the leaf at `path`, `o` on the 0.7.2 side and `n` on the 0.8.0 side, holds its
    /// base's value on both sides and neither side's build writes it: each build over the other
    /// side's base holds that base's value there.
    fn neither_writes(&self, path: &str, o: &str, n: &str) -> bool {
        let at = |l: &Leaves| leaf_at(l, path);
        let Some((old_swapped, new_swapped)) = self.swapped else {
            return false;
        };
        at(self.old).as_deref() == Some(o)
            && at(self.new).as_deref() == Some(n)
            && at(old_swapped).as_deref() == Some(n)
            && at(new_swapped).as_deref() == Some(o)
    }
}

/// One comparison: what is compared, the preset and scheme.
struct Side<'a> {
    what: &'a str,
    preset: &'static str,
    label: &'a str,
    bases: Option<Bases<'a>>,
}

/// A model gap's difference where it occurs: the gap's place in [`RULE_0_ON_A_PRESET`], the
/// preset and scheme, the leaf, and its 0.7.2 and 0.8.0 values.
struct GapHit {
    gap: usize,
    label: String,
    path: String,
    old: String,
    new: String,
}

/// What the comparisons found: the failures; each listed entry's leaves that differ as listed
/// (`seen`) and every leaf it claims (`claimed`), by the entry's place in [`listed`], the
/// preset and the leaf's path; and the model gaps' differences.
#[derive(Default)]
struct Tally {
    failures: Vec<String>,
    seen: HashSet<(usize, &'static str, String)>,
    claimed: HashSet<(usize, &'static str, String)>,
    gaps: Vec<GapHit>,
}

/// The leaves in which `old` and `new` differ where no entry lists the difference, or where
/// the 0.8.0 side's value is not the one the entry's cause gives (`expected`, `None` where the
/// cause does not apply), into `tally`'s failures; the listed ones into its `seen`, and each
/// leaf an entry claims, differing or not, into its `claimed`. A leaf both sides hold at
/// their bases' values, which neither side's build writes ([`Bases::neither_writes`]), differs
/// as the bases do and is not compared. An unlisted difference names the causes that would
/// give its value.
fn compare(
    side: &Side<'_>,
    old: &impl std::fmt::Debug,
    new: &impl std::fmt::Debug,
    expected: &dyn Fn(&str, Cause) -> Option<String>,
    tally: &mut Tally,
) {
    let (old, new) = (leaves(&format!("{old:#?}")), leaves(&format!("{new:#?}")));
    let label = side.label;
    let what = side.what;
    if old.len() != new.len() {
        tally.failures.push(format!(
            "{label}: {what}: the Debug forms differ in shape: {} lines against {}",
            old.len(),
            new.len()
        ));
    }
    let entries: Vec<(usize, &Difference)> = listed()
        .enumerate()
        .filter(|(_, d)| d.what == what && d.presets.contains(&side.preset))
        .collect();
    for ((path, o), (new_path, n)) in old.iter().zip(&new) {
        if path != new_path {
            tally
                .failures
                .push(format!("{label}: {what}: {path} against {new_path}"));
            return;
        }
        // an entry naming the leaf itself before one whose pattern matches it
        let listing = |exact: bool| {
            entries
                .iter()
                .find(|(_, d)| names_one(d.path) == exact && matches(d.path, path))
        };
        let entry = listing(true).or_else(|| listing(false));
        if let Some((i, _)) = entry {
            tally.claimed.insert((*i, side.preset, path.clone()));
        }
        if o == n
            || side
                .bases
                .as_ref()
                .is_some_and(|b| b.neither_writes(path, o, n))
        {
            continue;
        }
        let Some(&(i, d)) = entry else {
            let causes: Vec<Cause> = Cause::ALL
                .into_iter()
                .filter(|c| expected(path, *c).as_ref() == Some(n))
                .collect();
            tally.failures.push(format!(
                "{label}: {what}: {path}: 0.7.2 {o}, 0.8.0 {n}; the causes that give it: {causes:?}"
            ));
            continue;
        };
        let want = expected(path, d.cause);
        if want.as_ref() == Some(n) && !d.reason.is_empty() {
            tally.seen.insert((i, side.preset, path.clone()));
            if let Some(gap) = i
                .checked_sub(DIFFERENCES.len())
                .filter(|gap| *gap < RULE_0_ON_A_PRESET.len())
            {
                tally.gaps.push(GapHit {
                    gap,
                    label: label.to_owned(),
                    path: path.clone(),
                    old: o.clone(),
                    new: n.clone(),
                });
            }
        } else {
            tally.failures.push(format!(
                "{label}: {what}: {path}: listed ({:?}), but 0.8.0 {n}, the cause gives {want:?}",
                d.cause
            ));
        }
    }
}

/// The leaf [`Cause::Stated`]'s home gives on `new`, as `Debug` writes it: a colour or a width
/// as it is, a stroke colour as `over` lays it over the fill its entry paints; `None` where the
/// home is unstated.
fn stated_leaf(
    home: Home,
    new: &ResolvedTheme,
    over: impl FnOnce(egui::Color32) -> Option<String>,
) -> Option<String> {
    match (home.value)(new)? {
        HomeValue::Color(c) => Some(format!("{:?}", to_color32(c))),
        HomeValue::Line(c) => over(to_color32(c)),
        HomeValue::Width(w) => Some(format!("{w:?}")),
        HomeValue::Radius(v) => Some(format!("{:?}", crate::convert::u8_from_f32_saturating(v))),
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
    let mut tally = Tally::default();
    if PRESETS.iter().map(|(key, _)| *key).ne(ALL.iter().copied()) {
        tally
            .failures
            .push("ALL is not the presets compared".to_owned());
    }
    for d in DIFFERENCES
        .iter()
        .chain(STATED_ON_A_PRESET)
        .filter(|d| d.cause.is_gap())
    {
        tally.failures.push(format!(
            "{} {}: a model gap's cause, listed outside RULE_0_ON_A_PRESET",
            d.what, d.path
        ));
    }
    for g in RULE_0_ON_A_PRESET.iter().filter(|g| !g.diff.cause.is_gap()) {
        tally.failures.push(format!(
            "{} {}: not a model gap's cause",
            g.diff.what, g.home.path
        ));
    }
    let prefs = AccessibilityPreferences::default();
    for &(key, preset) in PRESETS {
        let spec = native_theme::theme::Theme::preset(key)?;
        let layout = spec.layout.clone();
        // the icon set and themes as `from_preset` hands them to the 0.7.2 builder
        let context = native_theme::ResolutionContext::for_tests();
        let old_icons = [ColorMode::Light, ColorMode::Dark]
            .map(|mode| spec.resolve_with(mode, &context))
            .map(|r| r.map(|r| (r.icon_set, r.icon_theme)));
        let [old_light_icons, old_dark_icons] = old_icons;
        let ((icon_set, light_icon_theme), (_, dark_icon_theme)) =
            (old_light_icons?, old_dark_icons?);
        let (old_light, new_light) = resolve(key, preset, false)?;
        let (old_dark, new_dark) = resolve(key, preset, true)?;
        let resolve_inactive = |ctx: ThemeContext| {
            native_theme::next::Theme::preset(preset).resolve(&ctx.with_window_active(false))
        };
        let light_inactive = resolve_inactive(ThemeContext::light());
        let dark_inactive = resolve_inactive(ThemeContext::dark());
        let mut builder = crate::ThemeAtlas::builder(key, &old_light, &old_dark)
            .layout(&layout)
            .accessibility(&prefs)
            .icon_set(icon_set);
        if let Some(name) = light_icon_theme.as_deref() {
            builder = builder.icon_theme(egui::Theme::Light, name);
        }
        if let Some(name) = dark_icon_theme.as_deref() {
            builder = builder.icon_theme(egui::Theme::Dark, name);
        }
        // the OS's faces, as `from_preset` plans them, where `to_theme` installs them too
        #[cfg(feature = "system-fonts")]
        {
            builder = builder.fonts(crate::fonts::FontPlan::from_system(&old_light));
        }
        let old_atlas = builder.build();
        let new_atlas = super::to_theme(&new_light, &light_inactive, &new_dark, &dark_inactive)?;
        if old_atlas.fonts() != new_atlas.fonts() {
            tally
                .failures
                .push(format!("{key}: the atlases install different fonts"));
        }
        // the faces both atlases measured their rows on
        let defs = new_atlas.fonts().cloned().unwrap_or_default();
        // what the atlas holds once, and each scheme's icon theme
        let side = |what| Side {
            what,
            preset: key,
            label: key,
            bases: None,
        };
        let none = |_: &str, _: Cause| None;
        compare(
            &side("Accessibility"),
            old_atlas.accessibility(),
            new_atlas.accessibility(),
            &none,
            &mut tally,
        );
        compare(
            &side("Layout"),
            old_atlas.layout(),
            new_atlas.layout(),
            &none,
            &mut tally,
        );
        compare(
            &side("IconSet"),
            &old_atlas.icon_set(),
            &new_atlas.icon_set(),
            &none,
            &mut tally,
        );
        for (what, scheme) in [
            ("IconTheme/light", egui::Theme::Light),
            ("IconTheme/dark", egui::Theme::Dark),
        ] {
            compare(
                &side(what),
                &old_atlas.icon_theme(scheme),
                &new_atlas.icon_theme(scheme),
                &none,
                &mut tally,
            );
        }
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
            let gaps_before = tally.gaps.len();
            let (old_styles, new_styles) = (old_atlas.scheme(scheme), new_atlas.scheme(scheme));
            // each side's sources and cell input as its atlas compiled them
            let old_sources = crate::style::Sources::old(old, &prefs, &layout);
            let new_sources = super::build::sources(new, inactive, &mut Vec::new())?;
            let cell_input = |sources: &crate::style::Sources| CellInput {
                scheme,
                row_height: crate::atlas::row_height(scheme, sources, &defs),
                defs: Some(&defs),
                rule_0: sources.rule_0,
            };
            let (old_input, new_input) = (cell_input(&old_sources), cell_input(&new_sources));
            let (old_base, new_base) = (&*old_styles.base, &*new_styles.base);
            let new_src = base_source(new, &mut Vec::new());
            let own = leaves(&format!("{:#?}", scheme.default_style()));
            let old_leaves = leaves(&format!("{old_base:#?}"));
            let new_leaves = leaves(&format!("{new_base:#?}"));
            // `open` paints the resting fill (`OpenFrom::Inactive`); its `weak_bg_fill` is
            // then the title bar's
            let base_over = |color: egui::Color32, path: &str| {
                let w = &new_base.visuals.widgets;
                let fill = if path.ends_with("open.bg_stroke.color") {
                    w.inactive.weak_bg_fill
                } else {
                    entry(w, path)?.weak_bg_fill
                };
                let line = Line {
                    color,
                    align: new_src.button.align,
                };
                Some(format!("{:?}", line.over(Some(fill))))
            };
            let base_expected = |path: &str, cause: Cause| match cause {
                Cause::Rule0 | Cause::Egui => leaf_at(&own, path),
                Cause::OverFill => base_over(to_color32(old.button.border.color), path),
                Cause::Stated(home) => stated_leaf(home, new, |c| base_over(c, path)),
                Cause::Padding(home) => {
                    padding_leaf(home, new, scheme.default_style().spacing.button_padding)
                }
                Cause::StateKeepsBase
                | Cause::Base
                | Cause::RestingStroke
                | Cause::Knob
                | Cause::OverBase => None,
            };
            let side = Side {
                what: "base",
                preset: key,
                label: &label,
                bases: None,
            };
            compare(&side, old_base, new_base, &base_expected, &mut tally);
            let new_cells = cell_source(new, &mut Vec::new());
            let new_variant_src = variant_source(new, &mut Vec::new());
            for role in Role::all() {
                let what = format!("{role:?}");
                let old_cell = &**old_styles.cell(*role, RoleVariant::Normal);
                let new_cell = &**new_styles.cell(*role, RoleVariant::Normal);
                let new_cell_leaves = leaves(&format!("{new_cell:#?}"));
                let pressed = |path: &str| {
                    (path.contains(".hovered.") || path.contains(".active."))
                        .then(|| path.replacen(".active.", ".hovered.", 1))
                };
                let cell_over = |color: egui::Color32, path: &str| {
                    let (align, weak) = outline_of(*role, &new_cells)?;
                    let painted = entry(&new_cell.visuals.widgets, path)?;
                    let line = Line { color, align };
                    let fill = weak.map(|weak| {
                        if weak {
                            painted.weak_bg_fill
                        } else {
                            painted.bg_fill
                        }
                    });
                    Some(format!("{:?}", line.over(fill)))
                };
                // each side's role over the other side's base
                let swapped = |sources: &crate::style::Sources,
                               input: &CellInput<'_>,
                               base: &egui::Style| {
                    let cell = role_cell_from(*role, base, &sources.cells, input, &mut Vec::new());
                    leaves(&format!("{cell:#?}"))
                };
                let old_swapped = swapped(&old_sources, &old_input, new_base);
                let new_swapped = swapped(&new_sources, &new_input, old_base);
                let cell_expected = |path: &str, cause: Cause| match cause {
                    Cause::Rule0 | Cause::Egui => leaf_at(&own, path),
                    Cause::StateKeepsBase | Cause::Base => leaf_at(&new_leaves, path),
                    Cause::OverFill => cell_over(
                        entry(&old_cell.visuals.widgets, path)?.bg_stroke.color,
                        path,
                    ),
                    Cause::Stated(home) => stated_leaf(home, new, |c| cell_over(c, path)),
                    Cause::Padding(home) => {
                        padding_leaf(home, new, new_base.spacing.button_padding)
                    }
                    Cause::OverBase => leaf_at(&old_swapped, path),
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
                    bases: Some(Bases {
                        old: &old_leaves,
                        new: &new_leaves,
                        swapped: Some((&old_swapped, &new_swapped)),
                    }),
                };
                compare(&side, old_cell, new_cell, &cell_expected, &mut tally);
                let old_normal_leaves = leaves(&format!("{old_cell:#?}"));
                let old_selected_leaves = selected_leaves(old_styles, *role);
                let new_selected_leaves = selected_leaves(new_styles, *role);
                for (name, variant) in VARIANTS {
                    let what = format!("{role:?}/{name}");
                    let has = own_cell(old_styles, *role, variant);
                    if has != own_cell(new_styles, *role, variant) {
                        tally
                            .failures
                            .push(format!("{label}: {what}: a cell on one model only"));
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
                    // each side's variant over the other side's cell it is copied from
                    let from_variant = match variant {
                        RoleVariant::SelectedDisabled => RoleVariant::Selected,
                        _ => RoleVariant::Normal,
                    };
                    let swapped = |sources: &crate::style::Sources,
                                   input: &CellInput<'_>,
                                   from: &egui::Style| {
                        let v = &sources.variants;
                        let notes = &mut Vec::new();
                        let cell = match variant {
                            RoleVariant::Normal => None,
                            RoleVariant::Selected => selected_cell(*role, from, v, input, notes),
                            RoleVariant::Disabled => disabled_cell(*role, from, v, notes),
                            RoleVariant::SelectedDisabled => {
                                selected_disabled_cell(*role, from, v, notes)
                            }
                        };
                        cell.map(|c| leaves(&format!("{c:#?}")))
                    };
                    let old_swapped = swapped(
                        &old_sources,
                        &old_input,
                        new_styles.cell(*role, from_variant),
                    );
                    let new_swapped = swapped(
                        &new_sources,
                        &new_input,
                        old_styles.cell(*role, from_variant),
                    );
                    let checked = variant != RoleVariant::Disabled;
                    let variant_over = |color: egui::Color32, path: &str| {
                        let (align, weak) = if checked {
                            (new_variant_src.checked.look.align, Some(false))
                        } else {
                            outline_of(*role, &new_cells)?
                        };
                        let painted = entry(&n.visuals.widgets, path)?;
                        let line = Line { color, align };
                        let fill = weak.map(|weak| {
                            if weak {
                                painted.weak_bg_fill
                            } else {
                                painted.bg_fill
                            }
                        });
                        Some(format!("{:?}", line.over(fill)))
                    };
                    let variant_expected = |path: &str, cause: Cause| match cause {
                        Cause::Rule0 | Cause::Egui => leaf_at(&own, path),
                        Cause::StateKeepsBase | Cause::Base => leaf_at(&new_leaves, path),
                        Cause::OverFill => {
                            variant_over(entry(&o.visuals.widgets, path)?.bg_stroke.color, path)
                        }
                        Cause::Stated(home) => stated_leaf(home, new, |c| variant_over(c, path)),
                        Cause::RestingStroke
                        | Cause::Knob
                        | Cause::Padding(_)
                        | Cause::OverBase => None,
                    };
                    let side = Side {
                        what: &what,
                        preset: key,
                        label: &label,
                        bases: Some(Bases {
                            old: old_from,
                            new: new_from,
                            swapped: old_swapped.as_deref().zip(new_swapped.as_deref()),
                        }),
                    };
                    compare(&side, o, n, &variant_expected, &mut tally);
                }
            }
            // a model gap seen in this scheme: its home unstated for its state on the preset
            let gaps = tally
                .gaps
                .get(gaps_before..)
                .unwrap_or_default()
                .iter()
                .filter_map(|hit| RULE_0_ON_A_PRESET.get(hit.gap));
            let mut stated = Vec::new();
            for g in gaps {
                if (g.home.stated)(new) {
                    stated.push(format!(
                        "{label}: {} {}: a model gap, but a rule states {} for its state",
                        g.diff.what, g.diff.path, g.home.path
                    ));
                }
            }
            tally.failures.extend(stated);
            let new_frames = &new_sources.frames;
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
                let align = frame_align(*surface, new_frames);
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
                // each side's frame over the other side's base, against egui's preset there
                let swapped = |sources: &crate::style::Sources, base: &egui::Style| {
                    let frame = surface_frame_from(
                        *surface,
                        base,
                        &sources.frames,
                        scheme,
                        &mut Vec::new(),
                    );
                    leaves(&format!("{frame:#?}"))
                };
                let old_swapped = swapped(&old_sources, new_base);
                let new_swapped = swapped(&new_sources, old_base);
                let side = Side {
                    what: &what,
                    preset: key,
                    label: &label,
                    bases: Some(Bases {
                        old: &old_preset,
                        new: &new_preset,
                        swapped: Some((&old_swapped, &new_swapped)),
                    }),
                };
                compare(&side, &old_frame, &new_frame, &frame_expected, &mut tally);
            }
            let side = |what| Side {
                what,
                preset: key,
                label: &label,
                bases: None,
            };
            compare(
                &side("FocusRing"),
                &old_styles.focus_ring,
                &new_styles.focus_ring,
                &|_, _| None,
                &mut tally,
            );
            compare(
                &side("MenuBar"),
                &old_styles.menu_bar,
                &new_styles.menu_bar,
                &|_, _| None,
                &mut tally,
            );
        }
    }
    let Tally {
        mut failures,
        seen,
        claimed,
        gaps,
    } = tally;
    for preset in Preset::ALL {
        if !PRESETS.iter().any(|(_, p)| p == preset) {
            failures.push(format!("{preset:?}: not compared"));
        }
    }
    // each leaf a listed entry claims on a preset it names differs there as listed
    for (i, d) in listed().enumerate() {
        for key in d.presets {
            let mut stale: Vec<&str> = claimed
                .iter()
                .filter(|claim| claim.0 == i && claim.1 == *key && !seen.contains(*claim))
                .map(|(_, _, path)| path.as_str())
                .collect();
            stale.sort_unstable();
            if !claimed.iter().any(|(j, p, _)| *j == i && p == key) {
                failures.push(format!(
                    "{} {} on {key}: listed, but no leaf is at the path: remove the entry",
                    d.what, d.path
                ));
            } else if !stale.is_empty() {
                failures.push(format!(
                    "{} {} on {key}: listed, but differs nowhere at {stale:?}",
                    d.what, d.path
                ));
            }
        }
    }
    // the model gaps, for the facts task that fixes the presets (M2.FS4b)
    println!("RULE_0_ON_A_PRESET, as found:");
    for hit in &gaps {
        if let Some(g) = RULE_0_ON_A_PRESET.get(hit.gap) {
            println!(
                "{}\t{}\t{}\t{}\t{:?}\t0.7.2 {}\t0.8.0 {}",
                hit.label, g.diff.what, g.home.path, hit.path, g.diff.cause, hit.old, hit.new
            );
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
