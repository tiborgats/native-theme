//! Connector parity of the 0.8.0 work (docs/v0.8.0-plan.md, section 1, rule
//! 2; M4, step 1): what the connector hands to iced, computed from the 0.7.2
//! model by the connector's public mapping and from the 0.8.0 model by
//! [`super`], is equal: the iced theme and the style each ported style
//! function returns, in every status of its widget.
//!
//! Presets covered: kde-breeze, adwaita, windows-11, macos-monterey,
//! macos-tahoe, material-3 and ios-13, light and dark ([`PRESETS`]). The
//! 0.7.2 side is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`), with no accessibility scaling, at the
//! font DPI native-theme's model parity resolves it at ([`resolve`]); the
//! 0.8.0 side for the scheme's default context, whose text scale is 1.
//!
//! The form of comparison per iced type the connector produces:
//!
//! - `iced_core::Theme` (with its `Palette` and `palette::Extended`) derives
//!   `PartialEq`: the palette, the extended palette and the name are each
//!   compared by it, the name being the theme's own on either side.
//! - Each widget's `Style` that a ported style function returns derives
//!   `PartialEq` and is compared by it, the function called with its side's
//!   own iced theme in every `Status` of the widget (the style functions
//!   read iced's own fields from the theme in effect); a widget built of
//!   iced's own (the segmented control, the expander) in every status of
//!   the widget it is built of, and in each of its arguments.
//! - `iced_widget::scrollable::Scrollbar` derives `PartialEq` and is
//!   compared by it.
//! - Each `iced_aw` 0.14.1 `Style` (`card`, `menu_bar`, `tab_bar`,
//!   `sidebar`, `selection_list`) derives no `PartialEq`: it is compared as
//!   the tuple of its fields, destructured whole (`aw_leaves`), in every
//!   value of `iced_aw`'s shared `Status`; the spinner's wrapper is an
//!   `iced_widget` container style, compared by its `PartialEq`.
//!
//! A difference is allowed only as an entry of [`DIFFERENCES`], with its
//! reason: a style function whose 0.8.0 stroke lies inside or across the
//! widget's edge, where 0.7.2 stated no alignment; one whose hovered or
//! pressed fill, stroke or label the 0.8.0 theme states nothing for, which
//! is as [`crate::styles::in_state`] gives it, iced's own where iced's own
//! style changes it for the status (spec §2, `from_state`); one whose
//! outline or corners the
//! 0.8.0 theme leaves unstated where 0.7.2's resolver made them up, which
//! are iced's own (spec §5, §10 rule 0); one whose part the 0.8.0 theme
//! states a property for that 0.7.2 has no field for, drawn as iced can
//! ([`Added`], its exact value laid into the 0.7.2 side's source). The
//! entry asserts, in every status
//! of the style function on the preset it names (a key with `/light` or
//! `/dark` names one scheme), the value the 0.8.0 side must give, whether
//! or not it differs there: the 0.7.2 side's own output with each of its
//! strokes laid where the entry says ([`crate::styles::Line::aligned`]), so
//! the stroke is composited over the fill each status paints, or over it at
//! half its alpha, and with the state options, the stroke or the corners
//! the entry names unset ([`Listed`]); any other value fails. Each (style function, preset)
//! pair an entry lists must differ in some status, or the entry is stale and
//! fails.
//!
//! An entry may also name `Style` fields the 0.8.0 preset states at a home
//! where 0.7.2's resolver gave another value ([`Stated`], M2.FS4): in the
//! statuses it names, the expected value is that output with each such
//! field set to the home's value through the conversion the 0.8.0 builder
//! gives it (a state's fill as its whole fill, a stroke laid over the fill
//! the style paints, [`crate::styles::Line::over`]), never through the
//! 0.8.0 style function itself; a home the 0.8.0 theme leaves unstated
//! fails.

use std::fmt::Debug;

use iced_core::theme::Theme;
use native_theme::next::{ButtonQuery, PartStyle, Preset, ThemeContext, WidgetState};

use super::LineAlign;
use crate::{ColorMode, ResolvedTheme};

/// The presets the test covers: the 0.7.2 key and the 0.8.0 preset.
pub(crate) const PRESETS: &[(&str, Preset)] = &[
    ("kde-breeze", Preset::KdeBreeze),
    ("adwaita", Preset::Adwaita),
    ("windows-11", Preset::Windows11),
    ("macos-monterey", Preset::MacosMonterey),
    ("macos-tahoe", Preset::MacosTahoe),
    ("material-3", Preset::Material3),
    ("ios-13", Preset::Ios13),
];

/// A listed difference (plan, section 1, rule 2): the style function, the
/// presets on which it differs (a preset's key for both schemes, or
/// `key/light` or `key/dark` for one), how the 0.8.0 side differs, and why.
struct Difference {
    leaf: &'static str,
    presets: &'static [&'static str],
    listed: Listed,
    /// The fields the preset states ([`Stated`]), in groups.
    stated: &'static [&'static [Stated]],
    reason: &'static [&'static str],
}

impl Difference {
    /// This entry with the `Style` fields `stated` names set to their
    /// homes' values, for `reason`.
    const fn stating(
        self,
        stated: &'static [&'static [Stated]],
        reason: &'static [&'static str],
    ) -> Self {
        Self {
            stated,
            reason,
            ..self
        }
    }
}

/// A `Style` field the 0.8.0 preset states at a home where 0.7.2's
/// resolver gave another value: in the statuses of state `at` (the
/// widget's status, with the checked or selected flag of a style function
/// drawn checked or selected), `field` is set to `home`'s value.
#[derive(Clone, Copy, Debug)]
struct Stated {
    at: WidgetState,
    field: Field,
    home: Home,
}

/// [`Stated`] in the statuses of `at`.
const fn set(at: WidgetState, field: Field, home: Home) -> Stated {
    Stated { at, field, home }
}

/// A `Style` field a [`Stated`] sets, in the order they are set: a stroke
/// colour is laid over the fill as set before it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Field {
    /// `background` (a tab bar's and a sidebar's `tab_label_background`).
    Background,
    /// A sidebar's `background`: the panel behind its items.
    Panel,
    /// A slider's `handle.background`.
    HandleBackground,
    /// `text_color`.
    TextColor,
    /// A check box's `icon_color`, a radio's `dot_color`.
    IconColor,
    /// `border.width`.
    BorderWidth,
    /// `border.color` (a radio's `border_color`): the stroke laid over the
    /// fill the style paints ([`crate::styles::Line::over`]).
    BorderColor,
    /// A segment's rounded corners, those its style rounds at the
    /// control's ends (`crate::styles::segment_from`): the track's corner
    /// radius less its stroke width.
    InnerRadius,
    /// A segment separator's `background`: the track's stroke colour, none
    /// where the track's stroke is stated as none (`segmented_source` of
    /// [`super::styles`]); unfaded, so listed where the control is not
    /// disabled or no line is drawn.
    Separator,
    /// A pane grid's `picked_split.color`.
    PickedSplitColor,
    /// A pane grid's `hovered_split.color`.
    HoveredSplitColor,
    /// `border.radius`, every corner: a uniform circular corner radius.
    Radius,
    /// A slider's filled rail, `rail.backgrounds.0`.
    RailFill,
}

impl Field {
    const ALL: [Self; 13] = [
        Self::Background,
        Self::Panel,
        Self::HandleBackground,
        Self::TextColor,
        Self::IconColor,
        Self::BorderWidth,
        Self::BorderColor,
        Self::InnerRadius,
        Self::Separator,
        Self::PickedSplitColor,
        Self::HoveredSplitColor,
        Self::Radius,
        Self::RailFill,
    ];
}

/// A home's value as the 0.8.0 builder reads it: a fill (a stated
/// `Paint::None` transparent, [`super::fill_of`]) or a text colour; a
/// stroke width ([`super::stroke_width_of`]); the part whose stroke the
/// builder lays over the fill ([`super::borders::line_or_none`]), on a
/// disabled button `faded` by its opacity (`crate::styles::faded`); a
/// colour of a disabled control the builder fades as one widget over the
/// `ground` by `opacity`, painted on `under` composited over the ground (or
/// on the ground itself), [`crate::styles::faded_as_one`].
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "widgets"), allow(dead_code))]
enum Value {
    Color(iced_core::Color),
    Width(f32),
    Stroke {
        part: Box<PartStyle>,
        faded: Option<f32>,
    },
    Faded {
        color: iced_core::Color,
        under: Option<iced_core::Color>,
        ground: iced_core::Color,
        opacity: Option<f32>,
    },
}

/// A 0.8.0 home: its path and its value where the theme states it for its
/// state (a rule for the state, its `from_state` bit; at rest, its `stated`
/// bit).
#[derive(Clone, Copy)]
struct Home {
    path: &'static str,
    value: fn(&native_theme::next::ResolvedTheme) -> Option<Value>,
}

impl Debug for Home {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.path)
    }
}

/// The readers of a [`Home`]'s value: each reads `part` where `gate`, the
/// part queried with the state whose rule decides, states the property for
/// `state` (a rule for the state, its `from_state` bit; at rest, its
/// `stated` bit).
mod read {
    use native_theme::next::{PartStated, PartStyle, TextStated, TextStyle, WidgetState};

    use super::Value;

    /// Whether `gate` states `bit` for `state`.
    fn part_stated(gate: &PartStyle, bit: PartStated, state: WidgetState) -> bool {
        if state == WidgetState::NONE {
            gate.stated.contains(bit)
        } else {
            gate.from_state.contains(bit)
        }
    }

    /// The fill.
    pub(super) fn fill(part: &PartStyle, gate: &PartStyle, state: WidgetState) -> Option<Value> {
        part_stated(gate, PartStated::FILL, state)
            .then(|| crate::next::fill_of(part))
            .flatten()
            .map(Value::Color)
    }

    /// The stroke, colour and alignment, or none.
    pub(super) fn stroke(part: &PartStyle, gate: &PartStyle, state: WidgetState) -> Option<Value> {
        part_stated(gate, PartStated::STROKE, state).then(|| Value::Stroke {
            part: Box::new(part.clone()),
            faded: None,
        })
    }

    /// The stroke's width, none 0.
    pub(super) fn width(part: &PartStyle, gate: &PartStyle, state: WidgetState) -> Option<Value> {
        part_stated(gate, PartStated::STROKE, state)
            .then(|| crate::next::stroke_width_of(part))
            .flatten()
            .map(Value::Width)
    }

    /// The corners as the builder hands them to iced (`borders::radius`,
    /// [`crate::next::radius_of`]), as a width.
    #[cfg(feature = "widgets")]
    pub(super) fn corners(part: &PartStyle, gate: &PartStyle, state: WidgetState) -> Option<Value> {
        part_stated(gate, PartStated::CORNERS, state)
            .then(|| crate::next::radius_of(part))
            .flatten()
            .map(Value::Width)
    }

    /// The style functions, which alone read corners, are compared with the
    /// `widgets` feature only: no value without it.
    #[cfg(not(feature = "widgets"))]
    pub(super) fn corners(_: &PartStyle, _: &PartStyle, _: WidgetState) -> Option<Value> {
        None
    }

    /// A text style's colour.
    pub(super) fn color(text: &TextStyle, gate: &TextStyle, state: WidgetState) -> Option<Value> {
        let stated = if state == WidgetState::NONE {
            gate.stated
        } else {
            gate.from_state
        };
        stated
            .contains(TextStated::COLOR)
            .then(|| crate::next::text_color_of(text))
            .flatten()
            .map(Value::Color)
    }
}

/// The [`Home`] `<widget>.<part>(<STATE>).<property>`, read by [`read`]'s
/// `<property>` (`fill`, `stroke`, `width`, a text style's `color`);
/// `<STATE>` one state or several joined by `|`; `; Primary` the primary
/// button's variant; `by <FLAG>` read at `<STATE>` where a rule for `<FLAG>`
/// alone states it, as the builder reads a checked box under the pointer
/// (`borders::flagged`).
macro_rules! home {
    ($r:ident, $w:ident . $p:ident ($($s:ident)|+)) => {
        Home {
            path: concat!(
                stringify!($w), ".", stringify!($p), "(", stringify!($($s)|+), ").",
                stringify!($r)
            ),
            value: |t| {
                let s = $(WidgetState::$s)|+;
                let part = t.$w.$p(s);
                read::$r(&part, &part, s)
            },
        }
    };
    ($r:ident, $w:ident . $p:ident ($($s:ident)|+; Primary)) => {
        Home {
            path: concat!(
                stringify!($w), ".", stringify!($p), "(", stringify!($($s)|+),
                "; variant=Primary).", stringify!($r)
            ),
            value: |t| {
                let s = $(WidgetState::$s)|+;
                let part = t.$w.$p(ButtonQuery::primary().state(s));
                read::$r(&part, &part, s)
            },
        }
    };
    ($r:ident, $w:ident . $p:ident ($($s:ident)|+) by $f:ident) => {
        Home {
            path: concat!(
                stringify!($w), ".", stringify!($p), "(", stringify!($($s)|+), ").",
                stringify!($r), " where a rule for ", stringify!($f), " states it"
            ),
            value: |t| {
                let f = WidgetState::$f;
                read::$r(&t.$w.$p($(WidgetState::$s)|+), &t.$w.$p(f), f)
            },
        }
    };
    (stroke, $w:ident . $p:ident ($($q:tt)+) faded by $ow:ident . $of:ident) => {
        Home {
            path: concat!(
                stringify!($w), ".", stringify!($p), "(", stringify!($($q)+), ").stroke, faded \
                 by ", stringify!($ow), ".", stringify!($of)
            ),
            value: |t| match (home!(stroke, $w.$p($($q)+)).value)(t)? {
                Value::Stroke { part, .. } => Some(Value::Stroke {
                    part,
                    faded: t.$ow.$of,
                }),
                _ => None,
            },
        }
    };
}

/// How a listed style function's 0.8.0 side differs from the 0.7.2 side:
/// where its strokes lie, which of its state options the 0.8.0 theme
/// leaves to iced, whether it leaves the corners or the stroke and its
/// width to iced, and what it states that 0.7.2 has no field for. The
/// expected value is the 0.7.2 side's output with its strokes laid there,
/// those options unset and the added property's value laid in.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Listed {
    pub align: LineAlign,
    pub states: States,
    pub iced_corners: bool,
    // Read by the `iced_aw` sources alone.
    #[cfg_attr(not(feature = "iced_aw"), allow(dead_code))]
    pub iced_stroke: bool,
    pub added: Added,
}

/// A property the 0.8.0 theme states that 0.7.2 has no field for, with the
/// exact value the 0.8.0 side must draw, laid into the 0.7.2 side's source
/// for the expected value.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Added {
    Nothing,
    /// A scroll bar handle's outline: the pen's width and alignment, its
    /// colour at rest, under the pointer and dragged, round a pill
    /// ([`crate::styles::pill`]).
    ScrollerOutline {
        width: f32,
        align: LineAlign,
        rest: iced_core::Color,
        hovered: iced_core::Color,
        dragged: iced_core::Color,
    },
    /// A scroll bar handle's fill under the pointer and dragged, which the
    /// 0.8.0 theme states (`scrollbar.thumb(HOVER)`, `(PRESSED)`) where
    /// 0.7.2 gives another.
    ScrollerFills {
        hovered: iced_core::Color,
        dragged: iced_core::Color,
    },
    /// A checked button's fill and label, which 0.7.2 leaves unset
    /// (`button.checked_background`, `button.checked_text_color`): the ones
    /// the 0.8.0 theme states (`button.body(CHECKED)`,
    /// `button.label(CHECKED)`), laid where the 0.7.2 source has none, the
    /// fill only where the entry leaves no checked fill to iced
    /// ([`States::checked`] clear).
    CheckedButton,
}

/// The state options of a style function's source that the 0.8.0 theme
/// states nothing for (spec §2, `from_state`), each then as
/// [`crate::styles::in_state`] gives it: the hovered and the pressed fill,
/// stroke and label, a focused field's and an open drop-down's stroke, a
/// switch's hovered thumb, a slider's filled rail under the pointer and
/// dragged, and a checked button's fill (`checked`: drawn as the primary
/// button's, spec §10, rule 0a).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct States {
    pub hover_fill: bool,
    pub hover_stroke: bool,
    pub hover_label: bool,
    pub pressed_fill: bool,
    pub pressed_stroke: bool,
    pub pressed_label: bool,
    pub focus_stroke: bool,
    pub open_stroke: bool,
    pub hover_thumb: bool,
    pub rail: bool,
    pub checked: bool,
}

impl States {
    /// No state option left to iced.
    const STATED: Self = Self {
        hover_fill: false,
        hover_stroke: false,
        hover_label: false,
        pressed_fill: false,
        pressed_stroke: false,
        pressed_label: false,
        focus_stroke: false,
        open_stroke: false,
        hover_thumb: false,
        rail: false,
        checked: false,
    };
}

impl Listed {
    /// Strokes laid at `align`, every state option stated, the corners the
    /// theme's.
    pub(crate) const fn aligned(align: LineAlign) -> Self {
        Self {
            align,
            states: States::STATED,
            iced_corners: false,
            iced_stroke: false,
            added: Added::Nothing,
        }
    }
}

/// The reason of a translucent stroke the theme lays inside the box.
const INSIDE: &str = "the part's border is a translucent stroke the theme lays inside the \
     box, over the fill (StrokeAlign::Inside, the alignment Stroke::uniform gives); iced on \
     wgpu ends the fill at the border's inner edge, so the connector hands iced the stroke \
     composited over the fill the status paints, where 0.7.2 handed it the stated colour, \
     which showed the window through the band (docs/v0.8.0-spec.md §2, StrokeAlign; the M2 \
     fill/stroke decision, connector table, iced row)";

/// The reason of a translucent stroke centred on the box's edge.
const CENTER: &str = "the part's border is a translucent pen centred on the edge \
     (StrokeAlign::Center); iced on wgpu ends the fill at the border's inner edge, so the \
     connector hands iced the band's mean, the stroke over the fill at half its alpha, where \
     0.7.2 handed it the stated colour (docs/v0.8.0-spec.md §2, StrokeAlign; the M2 \
     fill/stroke decision, connector table, iced row)";

/// The macOS presets, whose control strokes are translucent and inside the
/// box.
const MACOS: &[&str] = &["macos-monterey", "macos-tahoe"];

/// The macOS presets and windows-11, whose surface strokes are translucent
/// and inside the box (windows-11's push button and text field strokes lie
/// outside it, `StrokeAlign::Outside`, and do not differ).
const MACOS_WINDOWS: &[&str] = &["windows-11", "macos-monterey", "macos-tahoe"];

/// The reason of a hovered status the 0.8.0 theme states nothing for.
const HOVER: &str = "the theme states no value for the hovered widget: 0.7.2 leaves the \
     soft option unset (docs/inheritance-rules.toml, [no_inheritance]: no rule fills it, \
     None keeps the look at rest), which its connector drew as the widget at rest; the \
     0.8.0 theme has no rule for HOVER (the from_state bit clear when queried with HOVER \
     alone), and the connector keeps iced's own style for that status (docs/v0.8.0-spec.md \
     §2, from_state; §10, rule 0)";

/// The reason of ios-13's hovered box and circle, whose fill 0.7.2 states.
const IOS_BOX_HOVER: &str = "the hovered box's and circle's fill: 0.7.2's ios-13 states \
     checkbox.hover_background #d1d1d6 / #3a3a3c, uncited, which its connector drew on the \
     radio button too; the 0.8.0 preset states no fill for checkbox.indicator(HOVER) or \
     radio.indicator(HOVER), as WebKit's iOS checkboxRadioBackgroundColor reads no hover state \
     (RenderThemeIOS.mm:1370-1395 at WebKit 31adf9d; native-theme/tests/next_homes.toml, row \
     checkbox.hover_background, a listed difference, \"unstated\" there), so the fill is \
     iced's own for the status, the outline the stated rest one";

/// A style function whose stroke lies inside the box on `presets`.
const fn inside(leaf: &'static str, presets: &'static [&'static str]) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            align: LineAlign::Inside,
            states: States::STATED,
            iced_corners: false,
            iced_stroke: false,
            added: Added::Nothing,
        },
        stated: &[],
        reason: &[INSIDE],
    }
}

/// The reason of a hovered link whose label the 0.8.0 theme states nothing
/// for.
const HOVER_LINK_LABEL: &str = "the theme states no label for the hovered link: 0.7.2's \
     preset leaves link.hover_text_color unset and its resolver fills it from \
     link.font.color (docs/inheritance-rules.toml, link.hover_text_color), a value the 0.8.0 \
     model does not carry (docs/v0.8.0-spec.md §5); no rule for HOVER states link.label's \
     colour (the from_state bit clear when queried with HOVER alone), and iced's own text \
     button changes the label under the pointer (iced_widget 0.14.2 src/button.rs, fn text: \
     the label at 0.8 alpha), so the label is iced's own for the status (docs/v0.8.0-spec.md \
     §2, from_state; §10, rule 0)";

/// The reason of a switch's hovered thumb.
const HOVER_THUMB: &str = "the theme states no hovered thumb: 0.7.2 has no field for it, and \
     its connector drew the thumb at rest; the 0.8.0 theme has no rule for HOVER that states \
     switch.thumb's fill (the from_state bit clear when queried with HOVER alone), and iced's \
     own toggler changes the thumb under the pointer (iced_widget 0.14.2 src/toggler.rs, fn \
     default: the toggled thumb at half alpha, the other background.weak), so the thumb is \
     iced's own for the status (docs/v0.8.0-spec.md §2, from_state; §10, rule 0)";

/// The reason of a slider's filled rail under the pointer and dragged.
const SLIDER_RAIL: &str = "the theme states no filled rail under the pointer or dragged: 0.7.2 \
     has no field for it, and its connector drew the rail at rest; the 0.8.0 theme has no \
     rule for HOVER or PRESSED that states slider.active_track's fill (the from_state bit \
     clear when queried with that flag alone), and iced's own slider changes the filled rail \
     in both statuses (iced_widget 0.14.2 src/slider.rs, fn default: primary.strong under the \
     pointer, primary.weak dragged), so it is iced's own for the status (docs/v0.8.0-spec.md \
     §2, from_state; §10, rule 0)";

/// The reason of a dragged scroll bar thumb the 0.8.0 theme states nothing
/// for.
const PRESSED_THUMB: &str = "the theme states no pressed thumb: 0.7.2's preset leaves \
     scrollbar.thumb_active_color unset (docs/inheritance-rules.toml, [no_inheritance]: no \
     rule fills it), which its connector drew as the hovered thumb; the 0.8.0 theme has no \
     rule for PRESSED that states scrollbar.thumb's fill (the from_state bit clear when \
     queried with PRESSED alone; docs/v0.8.0-spec.md §5, scrollbar.thumb_active_color is \
     thumb(PRESSED).fill), and the dragged scroller is iced's own for the status \
     (iced_widget 0.14.2 src/scrollable.rs, fn default; docs/v0.8.0-spec.md §2, from_state; \
     §10, rule 0)";

/// An `iced_aw` style function's difference on `presets`: its strokes laid
/// at `align`, the state options `states` names left to `iced_aw`, and, per
/// `own` (stroke, corners), the outline's stroke and width or its corners
/// `iced_aw`'s own.
const fn aw(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
    states: States,
    own: (bool, bool),
    reason: &'static [&'static str],
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            align,
            states,
            iced_corners: own.1,
            iced_stroke: own.0,
            added: Added::Nothing,
        },
        stated: &[],
        reason,
    }
}

/// The reason of a selected tab whose outline the 0.8.0 theme leaves
/// unstated.
const TAB_STROKE: &str = "the theme states no outline for the selected tab: 0.7.2's preset \
     leaves tab.border unset and its resolver makes up a transparent line 0 wide \
     (validate_border), a value the 0.8.0 model does not carry (docs/v0.8.0-spec.md §5, notes \
     to the table: a value 0.7.2's resolver makes up is not carried; tab.border is \
     tab.tab(SELECTED)'s stroke and corners), and the selected tab's outline is iced_aw's own \
     tab bar's (iced_aw 0.14.1 src/style/tab_bar.rs, fn primary; §10, rule 0)";

/// The reason of a tab not selected whose outline the 0.8.0 theme leaves
/// unstated.
const TAB_IDLE_STROKE: &str = "nor an outline for a tab not selected: windows-11, kde-breeze, \
     adwaita and material-3 state it as none (docs/platform-facts.md §2.11), the macOS and \
     iOS presets state nothing for it, and 0.7.2's connector drew such a tab with no \
     outline, a width of 0 of its own; where tab.tab(NONE) states no stroke, the tab's \
     outline is iced_aw's own tab bar's \
     (iced_aw 0.14.1 src/style/tab_bar.rs, Style's default: 1 wide; docs/v0.8.0-spec.md \
     §10, rule 0)";

/// The reason of a hovered list row 0.7.2's resolver fills in.
const LIST_HOVER: &str = "the theme states no fill and no label for the hovered row: 0.7.2's \
     preset leaves list.hover_background and list.hover_text_color unset and its resolver \
     fills them from list.background_color and list.item_font.color \
     (docs/inheritance-rules.toml), values the 0.8.0 model does not carry \
     (docs/v0.8.0-spec.md §5, notes to the table: a hovered list row's fill); no rule for \
     HOVER states list.highlight's fill or list.item_label's colour, and both are iced_aw's \
     own for the status";

/// A style function whose hovered state options `states` names the theme
/// states nothing for on `presets`, its strokes laid at `align`.
const fn hover(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
    states: States,
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            align,
            states,
            iced_corners: false,
            iced_stroke: false,
            added: Added::Nothing,
        },
        stated: &[],
        reason: match align {
            LineAlign::Outside => &[HOVER],
            LineAlign::Inside => &[INSIDE, HOVER],
            LineAlign::Center => &[CENTER, HOVER],
        },
    }
}

/// The hovered state options left to iced: the fill (`fill`), with it the
/// label of a widget that has one (`label`: a label is iced's own where
/// the fill it lies on is), and the stroke (`stroke`).
const fn hovered(fill: bool, label: bool, stroke: bool) -> States {
    States {
        hover_fill: fill,
        hover_label: label,
        hover_stroke: stroke,
        ..States::STATED
    }
}

/// The reason of an open drop-down whose stroke the 0.8.0 theme states
/// nothing for.
const OPEN: &str = "the theme states no stroke for the open drop-down: 0.7.2 leaves \
     combo_box.open_border_color unset (docs/inheritance-rules.toml, [no_inheritance]), which \
     its connector drew as the field's border at rest; the 0.8.0 theme has no rule for OPEN \
     that states combo_box.field's stroke (the from_state bit clear when queried with OPEN \
     alone), and iced's own pick list changes the open field's border colour \
     (iced_widget 0.14.2 src/pick_list.rs, fn default: primary.strong), so it is iced's own \
     for the status (docs/v0.8.0-spec.md §2, from_state; §10, rule 0)";

/// A drop-down whose open stroke the theme states nothing for on
/// `presets`, with the hovered state options `states` names, its strokes
/// laid at `align`.
const fn open(presets: &'static [&'static str], align: LineAlign, states: States) -> Difference {
    let hovered = hover("pick_list", presets, align, states);
    Difference {
        listed: Listed {
            states: States {
                open_stroke: true,
                ..states
            },
            ..hovered.listed
        },
        reason: match (align, states.hover_fill || states.hover_stroke) {
            (LineAlign::Outside, true) => &[HOVER, OPEN],
            (LineAlign::Outside, false) => &[OPEN],
            (LineAlign::Inside, _) => &[INSIDE, HOVER, OPEN],
            (LineAlign::Center, _) => &[CENTER, HOVER, OPEN],
        },
        ..hovered
    }
}

/// The reason of a focused text field whose stroke the 0.8.0 theme states
/// nothing for.
const FIELD_FOCUS: &str = "the theme states no stroke for the focused field: 0.7.2 leaves \
     input.focus_border_color unset (docs/inheritance-rules.toml, [no_inheritance]), which its connector drew as the \
     field's border at rest where the focus ring is no inset border (the macOS ring, 3 wide \
     at offset -1, straddles the edge and crate::focus_ring draws it); the 0.8.0 theme has no \
     rule for FOCUSED that states the field's stroke, and iced's own text input and text \
     editor recolour the focused field's border (iced_widget 0.14.2 src/text_input.rs and \
     src/text_editor.rs, fn default: primary.strong), so its colour is iced's own for the \
     status";

/// A text field whose focused stroke the theme states nothing for on
/// `presets`, its strokes laid at `align`; its hovered stroke stated (the
/// rest one, as 0.7.2 drew it).
const fn field(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            states: States {
                focus_stroke: true,
                ..States::STATED
            },
            ..Listed::aligned(align)
        },
        stated: &[],
        reason: match align {
            LineAlign::Outside => &[FIELD_FOCUS],
            LineAlign::Inside => &[INSIDE, FIELD_FOCUS],
            LineAlign::Center => &[CENTER, FIELD_FOCUS],
        },
    }
}

/// The reason of a pressed widget the 0.8.0 theme states no fill for.
const PRESSED: &str = "the theme states no fill for the pressed widget: 0.7.2's connector \
     drew a pressed button, toolbar button, segment or expander header the theme states no \
     pressed fill for as the hovered one (button.active_background unset: the hover layer; \
     toolbar.button_active_background unset: the button's; a segment's and a header's \
     pressed look the hovered one); the 0.8.0 theme has no rule for PRESSED that states the \
     part's fill (the from_state bit clear when queried with PRESSED alone), so the fill is \
     as styles::in_state gives it: iced's own where iced's own style changes it for the \
     status, else the fill at rest; neither iced's secondary button, which the button and \
     the toolbar button are drawn against, nor the segment's and the header's \
     button::Style::default() changes the fill between Active and Pressed, so it is the \
     fill at rest (iced_widget 0.14.2 src/button.rs, fn secondary; docs/v0.8.0-spec.md §2, \
     from_state; §10, rule 0)";

/// A style function whose pressed fill the theme states nothing for on
/// `presets`, its strokes laid at `align`.
const fn pressed(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            states: States {
                pressed_fill: true,
                ..States::STATED
            },
            ..Listed::aligned(align)
        },
        stated: &[],
        reason: match align {
            LineAlign::Outside => &[PRESSED],
            LineAlign::Inside => &[INSIDE, PRESSED],
            LineAlign::Center => &[CENTER, PRESSED],
        },
    }
}

/// The reason of a checked button the 0.8.0 theme states no checked fill
/// for.
const CHECKED: &str = "the theme states no fill for the checked button: 0.7.2 leaves \
     button.checked_background (or toolbar.button_checked_background) unset \
     (docs/inheritance-rules.toml, [no_inheritance]: the pressed pair shows a checked toggle), \
     which its connector drew in the pressed button's fill and label; iced's button has no \
     checked status, and the 0.8.0 theme has no rule for CHECKED that states button.body's \
     (or toolbar.button's) fill, so the checked button is drawn in the connector's primary \
     button's fill and label in the same status, iced's own way of marking a chosen button, a \
     stated button.label(CHECKED) laid over, its outline, corners and shadow the unchecked \
     button's (docs/v0.8.0-spec.md §10, rule 0a)";

/// A button whose checked fill the theme states nothing for on `presets`,
/// its strokes laid at `align`, and with `pressed_fill` its pressed fill
/// too.
const fn checked(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
    pressed_fill: bool,
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            states: States {
                pressed_fill,
                checked: true,
                ..States::STATED
            },
            ..Listed::aligned(align)
        },
        stated: &[],
        reason: match (align, pressed_fill) {
            (LineAlign::Outside, false) => &[CHECKED],
            (LineAlign::Outside, true) => &[PRESSED, CHECKED],
            (LineAlign::Inside, false) => &[INSIDE, CHECKED],
            (LineAlign::Inside, true) => &[INSIDE, PRESSED, CHECKED],
            (LineAlign::Center, false) => &[CENTER, CHECKED],
            (LineAlign::Center, true) => &[CENTER, PRESSED, CHECKED],
        },
    }
}

/// The reason of a toolbar button whose hovered and pressed fill the 0.8.0
/// theme states nothing for.
const TOOLBAR_STATE_FILLS: &str = "the theme states no fill for the hovered or the pressed \
     toolbar button: 0.7.2 leaves toolbar.button_hover_background and \
     toolbar.button_active_background unset on these presets, which its connector drew in \
     the push button's button.hover_background and button.active_background; no rule for \
     HOVER or PRESSED states toolbar.button's fill, the toolbar button's own home \
     (native-theme/tests/next_homes.toml, rows toolbar.button_hover_background and \
     toolbar.button_active_background), and a control unfilled at rest does not borrow the \
     push button's state fills, so the fill is as styles::in_state gives it against iced's \
     own secondary button, the toolbar button's fill at rest being none \
     (docs/v0.8.0-spec.md §2, from_state; §10, rule 0)";

/// A toolbar button on `presets` whose hovered, pressed and checked fill
/// the theme states nothing for ([`TOOLBAR_STATE_FILLS`], [`CHECKED`]).
const fn unfilled_toolbar(presets: &'static [&'static str]) -> Difference {
    Difference {
        leaf: "toolbar_button",
        presets,
        listed: Listed {
            states: States {
                hover_fill: true,
                pressed_fill: true,
                checked: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[TOOLBAR_STATE_FILLS, CHECKED],
    }
}

/// A toolbar button on `presets` whose checked fill the theme states nothing
/// for ([`CHECKED`]), its hovered and pressed fills stated
/// ([`MEASURED_TOOLBAR_FILLS`]).
const fn unchecked_toolbar(presets: &'static [&'static str]) -> Difference {
    Difference {
        leaf: "toolbar_button",
        presets,
        listed: Listed {
            states: States {
                checked: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[MEASURED_TOOLBAR_FILLS, CHECKED],
    }
}

/// The reason of the macOS presets' toolbar button fills.
const MEASURED_TOOLBAR_FILLS: &str = "0.7.2's macOS toolbar states no \
     toolbar.button_hover_background or button_active_background, so its connector drew \
     iced's secondary button in those states; the 0.8.0 presets state the real toolbar's \
     measured hovered and pressed button, opaque over the opaque title bar, macos-monterey \
     #dfdfdf / #414141 and #d2d2d2 / #4e4e4e, macos-tahoe #f2f2f2 / #393939 and #e6e6e6 / \
     #484848 (os-probe run 45252258079, Reduce Transparency off, `*.hovered.toolbar_button`, \
     `*.pressed.toolbar_button` `fill`; native-theme/tests/next_homes.toml, rows \
     toolbar.button_hover_background and toolbar.button_active_background, listed \
     differences), toolbar.button(HOVER) and toolbar.button(PRESSED)";

/// The reason of a checked button whose fill and label the 0.8.0 theme
/// states and 0.7.2 does not.
const STATED_CHECKED: &str = "0.7.2 states no button.checked_background or \
     checked_text_color on these presets, so its connector drew a checked button in the \
     pressed pair that stands in for them; the 0.8.0 presets state the measured checked \
     toggle (docs/v0.8.0-spec.md §10, rule 0a; docs/platform-facts.md §2.3; \
     native-theme/tests/next_homes.toml, rows button.checked_background and \
     button.checked_text_color, listed differences): on macos-tahoe the accent mixed with \
     black (light) or white (dark) by 0.025 and a white label, on ios-13 the default tint at \
     alpha 0x2e (light) or 0x40 (dark) with its label in the tint, which the 0.8.0 side draws \
     in each status, the border composited over that fill; a toolbar button whose checked \
     fill the theme leaves unstated takes the stated checked label alone";

/// A button whose checked fill and label the 0.8.0 theme states and 0.7.2
/// leaves unset on `presets`, its strokes laid at `align`, and with
/// `pressed_fill` no pressed fill stated.
const fn stated_checked(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
    pressed_fill: bool,
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            states: States {
                pressed_fill,
                ..States::STATED
            },
            added: Added::CheckedButton,
            ..Listed::aligned(align)
        },
        stated: &[],
        reason: match (align, pressed_fill) {
            (LineAlign::Outside, false) => &[STATED_CHECKED],
            (LineAlign::Outside, true) => &[PRESSED, STATED_CHECKED],
            (LineAlign::Inside, false) => &[INSIDE, STATED_CHECKED],
            (LineAlign::Inside, true) => &[INSIDE, PRESSED, STATED_CHECKED],
            (LineAlign::Center, false) => &[CENTER, STATED_CHECKED],
            (LineAlign::Center, true) => &[CENTER, PRESSED, STATED_CHECKED],
        },
    }
}

/// The reason of a scroll bar handle whose outline and corners the 0.8.0
/// theme states.
const SCROLLER_OUTLINE: &str = "the theme states the scroll bar handle's outline and \
     corners, which 0.7.2 has no field for (its scrollbar states the handle's fills and \
     width alone), so its connector drew iced's own scroller border, none round corners of \
     2; the kde-breeze preset states Breeze's handle (native-theme's kde-breeze preset, S2: \
     Helper::renderScrollBarHandle and scrollBarHandleColor, breezehelper.cpp at breeze \
     f0b1d75): a pen 1.001 wide centred on the edge in the handle's colour at alpha \
     frameContrast, 0.2, WindowText in a view without the focus, DecorationHover #3daee9 \
     under the pointer and dragged, round a pill, which iced draws as the pen over the fill \
     at half its alpha (CENTER) and as a radius its renderers clamp to half the shorter side \
     (styles::pill; docs/v0.8.0-plan.md §2, M4 step 1: each visible property the new model \
     states and the toolkit can draw wired through)";

/// Breeze's WindowText at frameContrast, the handle's outline at rest in a
/// view without the focus: #232629 light, #fcfcfc dark, at alpha 0.2.
const BREEZE_TEXT_FRAME_LIGHT: iced_core::Color =
    iced_core::Color::from_rgba8(0x23, 0x26, 0x29, 0.2);
const BREEZE_TEXT_FRAME_DARK: iced_core::Color =
    iced_core::Color::from_rgba8(0xfc, 0xfc, 0xfc, 0.2);

/// Breeze's DecorationHover at frameContrast, the handle's outline under
/// the pointer and dragged: #3daee9 at alpha 0.2.
const BREEZE_HOVER_FRAME: iced_core::Color = iced_core::Color::from_rgba8(0x3d, 0xae, 0xe9, 0.2);

/// Breeze's scrollable on the scheme `presets` names, its handle outlined
/// at rest in `rest`.
const fn breeze_scroller(presets: &'static [&'static str], rest: iced_core::Color) -> Difference {
    Difference {
        leaf: "scrollable",
        presets,
        listed: Listed {
            added: Added::ScrollerOutline {
                width: 1.001,
                align: LineAlign::Center,
                rest,
                hovered: BREEZE_HOVER_FRAME,
                dragged: BREEZE_HOVER_FRAME,
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[SCROLLER_OUTLINE],
    }
}

/// The macOS scrollable on the scheme `presets` names: the measured overlay
/// knob hovered and held, `knob` in both (os-probe run 45252258079,
/// `*.hovered.scroller_knob_overlay`, `*.pressed.scroller_knob_overlay`
/// `fill`; native-theme/tests/next_homes.toml, rows
/// scrollbar.thumb_hover_color and scrollbar.thumb_active_color, listed
/// differences).
const fn macos_scroller(presets: &'static [&'static str], knob: iced_core::Color) -> Difference {
    Difference {
        leaf: "scrollable",
        presets,
        listed: Listed {
            added: Added::ScrollerFills {
                hovered: knob,
                dragged: knob,
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[STATED],
    }
}

/// The reason of a `Style` field the 0.8.0 preset states at a home where
/// 0.7.2's resolver gave another value.
const STATED: &str = "the 0.8.0 preset states the field's home, from the source its comment \
     there cites (M2.FS4), where 0.7.2's resolver gave the resting value, another state's, a \
     layer laid over the resting fill or a stroke the platform does not draw (macos-tahoe's \
     push button bezel, stroked as none: no line, 0 wide, spec §10, rule 2): the field is the \
     home's value as the 0.8.0 builder hands it to iced, a state's fill its whole fill \
     (docs/v0.8.0-spec.md §9, step 2), a stroke laid over the fill the style paints";

/// The `Style` fields `stated` names on `presets`, the strokes laid at
/// `align`, no state option left to iced.
const fn stated_fields(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
    stated: &'static [&'static [Stated]],
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed::aligned(align),
        stated,
        reason: match align {
            LineAlign::Outside => &[STATED],
            LineAlign::Inside => &[INSIDE, STATED],
            LineAlign::Center => &[CENTER, STATED],
        },
    }
}

/// The [`WidgetState`] of the flags joined by `|`, in a constant.
macro_rules! at {
    ($($s:ident)|+) => {
        WidgetState::NONE $(.union(WidgetState::$s))+
    };
}

/// A button's outline at rest and disabled (`button.body(NONE)`), and under
/// the pointer and pressed (`body(HOVER)`, `body(PRESSED)`), of the state
/// `$flag` joins (a checked toggle's): its width and its stroke.
macro_rules! button_outline {
    ($($flag:ident)?; $($variant:ident)?) => {
        [
            set(at!(NONE $(| $flag)?), Field::BorderWidth, home!(width, button.body(NONE $(; $variant)?))),
            set(at!(NONE $(| $flag)?), Field::BorderColor, home!(stroke, button.body(NONE $(; $variant)?))),
            set(at!(DISABLED $(| $flag)?), Field::BorderWidth, home!(width, button.body(NONE $(; $variant)?))),
            set(
                at!(DISABLED $(| $flag)?),
                Field::BorderColor,
                home!(stroke, button.body(NONE $(; $variant)?) faded by button.disabled_opacity),
            ),
            set(at!(HOVER $(| $flag)?), Field::BorderWidth, home!(width, button.body(HOVER $(; $variant)?))),
            set(at!(HOVER $(| $flag)?), Field::BorderColor, home!(stroke, button.body(HOVER $(; $variant)?))),
            set(at!(PRESSED $(| $flag)?), Field::BorderWidth, home!(width, button.body(PRESSED $(; $variant)?))),
            set(at!(PRESSED $(| $flag)?), Field::BorderColor, home!(stroke, button.body(PRESSED $(; $variant)?))),
        ]
    };
}

/// The hovered and the pressed fill of a button (`button.body(HOVER)`,
/// `body(PRESSED)`).
const BUTTON_STATE_FILLS: [Stated; 2] = [
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, button.body(HOVER)),
    ),
    set(
        at!(PRESSED),
        Field::Background,
        home!(fill, button.body(PRESSED)),
    ),
];

/// A sidebar's panel (`sidebar.container(NONE)`) and its outline laid over
/// it, in each state [`sidebar_at`] gives a status: the style paints the
/// panel in every status.
macro_rules! sidebar_panel {
    ($($s:ident),+) => {
        [$(
            set(at!($s), Field::Panel, home!(fill, sidebar.container(NONE))),
            set(at!($s), Field::BorderColor, home!(stroke, sidebar.container(NONE))),
        )+]
    };
}

/// [`sidebar_panel`] in every status.
const SIDEBAR_PANEL: [Stated; 10] = sidebar_panel!(NONE, SELECTED, HOVER, PRESSED, FOCUSED);

/// A button's outline in every status ([`button_outline`]).
const BUTTON_OUTLINE: [Stated; 8] = button_outline!(;);

/// A checked toggle's outline in every status.
const CHECKED_OUTLINE: [Stated; 8] = button_outline!(CHECKED;);

/// The primary button's outline in every status.
const PRIMARY_OUTLINE: [Stated; 8] = button_outline!(; Primary);

/// macos-tahoe's push button: [`BUTTON_STATE_FILLS`] and
/// [`BUTTON_OUTLINE`].
const TAHOE_BUTTON: &[&[Stated]] = &[&BUTTON_STATE_FILLS, &BUTTON_OUTLINE];

/// A toggle on macos-tahoe: [`TAHOE_BUTTON`], checked [`CHECKED_OUTLINE`].
const TAHOE_TOGGLE: &[&[Stated]] = &[&BUTTON_STATE_FILLS, &BUTTON_OUTLINE, &CHECKED_OUTLINE];

/// A toggle's outline, checked or not.
const TOGGLE_OUTLINE: &[&[Stated]] = &[&BUTTON_OUTLINE, &CHECKED_OUTLINE];

/// A button's stroke colour in the state `$s` (with `| $flag`, a checked
/// toggle's), of the primary variant with `Primary`.
macro_rules! button_stroke {
    ($s:ident $(| $flag:ident)?; $($v:ident)?) => {
        set(
            at!($s $(| $flag)?),
            Field::BorderColor,
            home!(stroke, button.body($s $(; $v)?)),
        )
    };
}

/// A toolbar button's outline width, the button's at rest
/// (`button.body(NONE)`), in every status, checked or not.
macro_rules! toolbar_widths {
    () => {
        [
            set(
                at!(NONE),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
            set(
                at!(DISABLED),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
            set(
                at!(HOVER),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
            set(
                at!(PRESSED),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
            set(
                at!(CHECKED),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
            set(
                at!(DISABLED | CHECKED),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
            set(
                at!(HOVER | CHECKED),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
            set(
                at!(PRESSED | CHECKED),
                Field::BorderWidth,
                home!(width, button.body(NONE)),
            ),
        ]
    };
}

/// A toolbar button's outline width ([`toolbar_widths`]).
const TOOLBAR_WIDTHS: [Stated; 8] = toolbar_widths!();

/// A toolbar button's hovered and pressed fill, the macOS presets' measured
/// ones ([`MEASURED_TOOLBAR_FILLS`]).
const TOOLBAR_FILLS: [Stated; 2] = [
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, toolbar.button(HOVER)),
    ),
    set(
        at!(PRESSED),
        Field::Background,
        home!(fill, toolbar.button(PRESSED)),
    ),
];

/// macos-tahoe's toolbar button: [`TOOLBAR_WIDTHS`], [`TOOLBAR_FILLS`].
const TAHOE_TOOLBAR: &[&[Stated]] = &[&TOOLBAR_WIDTHS, &TOOLBAR_FILLS];

/// A button's hovered and pressed stroke colour.
const STATE_STROKES: &[&[Stated]] = &[&[button_stroke!(HOVER;), button_stroke!(PRESSED;)]];

/// A toggle's hovered and pressed stroke colour, checked or not.
const TOGGLE_STATE_STROKES: &[&[Stated]] = &[&[
    button_stroke!(HOVER;),
    button_stroke!(PRESSED;),
    button_stroke!(HOVER | CHECKED;),
    button_stroke!(PRESSED | CHECKED;),
]];

/// The primary button's hovered and pressed stroke colour.
const PRIMARY_STATE_STROKES: &[&[Stated]] = &[&[
    button_stroke!(HOVER; Primary),
    button_stroke!(PRESSED; Primary),
]];

/// A button's pressed stroke colour.
const PRESSED_STROKE: &[&[Stated]] = &[&[button_stroke!(PRESSED;)]];

/// A toggle's pressed stroke colour, checked or not.
const TOGGLE_PRESSED_STROKE: &[&[Stated]] =
    &[&[button_stroke!(PRESSED;), button_stroke!(PRESSED | CHECKED;)]];

/// A button's pressed label (`button.label(PRESSED)`).
const PRESSED_LABEL: Stated = set(
    at!(PRESSED),
    Field::TextColor,
    home!(color, button.label(PRESSED)),
);

/// macos-monterey's push button: the hovered fill and stroke, the pressed
/// fill and label, and the stroke at rest laid over the pressed fill (no
/// rule for PRESSED states one, and iced's secondary button keeps its
/// border pressed, iced_widget 0.14.2 `src/button.rs`, fn secondary).
const MONTEREY_BUTTON: &[&[Stated]] = &[
    &BUTTON_STATE_FILLS,
    &[
        PRESSED_LABEL,
        button_stroke!(HOVER;),
        set(
            at!(PRESSED),
            Field::BorderColor,
            home!(stroke, button.body(NONE)),
        ),
    ],
];

/// The macOS presets' unselected tab's measured fill and a hovered tab's as
/// its rest one.
const TAB_FILLS: [Stated; 2] = [
    set(at!(NONE), Field::Background, home!(fill, tab.tab(NONE))),
    set(at!(HOVER), Field::Background, home!(fill, tab.tab(HOVER))),
];

/// macos-monterey's toolbar button: the button's pressed label,
/// [`TOOLBAR_FILLS`].
const MONTEREY_TOOLBAR: &[&[Stated]] = &[&[PRESSED_LABEL], &TOOLBAR_FILLS];

/// A check box's hovered stroke, checked or not (`checkbox.indicator(HOVER)`,
/// `indicator(HOVER|CHECKED)` where a rule for HOVER states it).
const CHECK_HOVER_STROKES: [Stated; 2] = [
    set(
        at!(HOVER),
        Field::BorderColor,
        home!(stroke, checkbox.indicator(HOVER)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::BorderColor,
        home!(stroke, checkbox.indicator(HOVER | CHECKED) by HOVER),
    ),
];

/// windows-11's check box under the pointer: the unchecked box's fill and
/// stroke, the checked box's fill and stroke (AccentFillColorSecondary,
/// CheckBox_themeresources.xaml:234, :222 at microsoft-ui-xaml 8463f45);
/// and the checked box's outline at rest, AccentFillColorDefault (:221).
const W11_CHECK_HOVER: &[Stated] = &[
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, checkbox.indicator(HOVER)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::Background,
        home!(fill, checkbox.indicator(HOVER | CHECKED) by HOVER),
    ),
    CHECK_HOVER_STROKES[0],
    CHECK_HOVER_STROKES[1],
    set(
        at!(CHECKED),
        Field::BorderColor,
        home!(stroke, checkbox.indicator(CHECKED)),
    ),
];

/// macos-tahoe's check box under the pointer: the unchecked box's fill, the
/// checked box's (`indicator(HOVER|CHECKED)` where the rule for HOVER states
/// it), the strokes over them.
const TAHOE_CHECK_HOVER: [Stated; 4] = [
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, checkbox.indicator(HOVER)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::Background,
        home!(fill, checkbox.indicator(HOVER | CHECKED) by HOVER),
    ),
    CHECK_HOVER_STROKES[0],
    CHECK_HOVER_STROKES[1],
];

/// The macOS radio button under the pointer, as the check box (os-probe run
/// 45345116763, `*.hovered.radio_off`, `…radio_on` differ from the normal
/// capture in 0 pixels): the unselected circle's fill and the selected
/// one's (`indicator(HOVER|CHECKED)` where the rule for HOVER states it),
/// the strokes over them.
const MACOS_RADIO_HOVER: [Stated; 4] = [
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, radio.indicator(HOVER)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::Background,
        home!(fill, radio.indicator(HOVER | CHECKED) by HOVER),
    ),
    set(
        at!(HOVER),
        Field::BorderColor,
        home!(stroke, radio.indicator(HOVER)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::BorderColor,
        home!(stroke, radio.indicator(HOVER | CHECKED) by HOVER),
    ),
];

/// The expander header's hovered and pressed fill (`expander.header(HOVER)`,
/// `header(PRESSED)`).
const EXPANDER_FILLS: [Stated; 2] = [
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, expander.header(HOVER)),
    ),
    set(
        at!(PRESSED),
        Field::Background,
        home!(fill, expander.header(PRESSED)),
    ),
];

/// windows-11's radio under the pointer: the unselected circle's fill, and
/// its stroke at rest laid over it (no rule for HOVER states one, and iced's
/// own radio keeps its border colour under the pointer, iced_widget 0.14.2
/// `src/radio.rs`, fn default); the selected circle's outline at rest,
/// AccentFillColorDefault (RadioButton_themeresources.xaml:142 at
/// microsoft-ui-xaml 8463f45), and its fill and outline under the pointer,
/// AccentFillColorSecondary (:147, :143).
const W11_RADIO_HOVER: &[Stated] = &[
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, radio.indicator(HOVER)),
    ),
    set(
        at!(HOVER),
        Field::BorderColor,
        home!(stroke, radio.indicator(NONE)),
    ),
    set(
        at!(CHECKED),
        Field::BorderColor,
        home!(stroke, radio.indicator(CHECKED)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::Background,
        home!(fill, radio.indicator(HOVER | CHECKED)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::BorderColor,
        home!(stroke, radio.indicator(HOVER | CHECKED)),
    ),
];

/// A disabled check box's label (`checkbox.label(DISABLED)`), faded as one
/// widget over the window by `checkbox.disabled_opacity`.
const CHECK_DISABLED_LABEL: Home = Home {
    path: "checkbox.label(DISABLED).color, faded as one over colors.window.background by \
           checkbox.disabled_opacity",
    value: |t| {
        let c = &t.checkbox;
        Some(Value::Faded {
            color: super::text_color_of(&c.label(WidgetState::DISABLED))?,
            under: None,
            ground: super::to_color(t.colors.window.background?),
            opacity: c.disabled_opacity,
        })
    },
};

/// The mark of a disabled check box whose disabled fill the theme states
/// (`checkbox.indicator(DISABLED)`): the disabled label's colour on that
/// fill, faded as one widget over the window.
const CHECK_DISABLED_MARK: Home = Home {
    path: "checkbox.label(DISABLED).color on checkbox.indicator(DISABLED).fill, faded as one \
           over colors.window.background by checkbox.disabled_opacity",
    value: |t| {
        let color = super::text_color_of(&t.checkbox.label(WidgetState::DISABLED))?;
        disabled_mark(t, WidgetState::DISABLED, color)
    },
};

/// The mark of a disabled checked box whose disabled mark and fill the theme
/// states (`checkbox.checkmark(DISABLED)`, `checkbox.indicator(DISABLED)`):
/// the mark's own colour on the checked box's disabled fill, faded as one
/// widget over the window.
const CHECKED_DISABLED_MARK: Home = Home {
    path: "checkbox.checkmark(DISABLED|CHECKED).color on \
           checkbox.indicator(DISABLED|CHECKED).fill, faded as one over \
           colors.window.background by checkbox.disabled_opacity",
    value: |t| {
        use native_theme::next::IconStated;
        let c = &t.checkbox;
        let state = WidgetState::DISABLED.union(WidgetState::CHECKED);
        let mark = c.checkmark(state);
        if !c
            .checkmark(WidgetState::DISABLED)
            .from_state
            .contains(IconStated::COLOR)
            || !mark.stated.contains(IconStated::COLOR)
        {
            return None;
        }
        disabled_mark(t, state, super::to_color(mark.color))
    },
};

/// A disabled box's mark in `color` on `checkbox.indicator(state)`, where a
/// rule for DISABLED states the box's fill.
fn disabled_mark(
    t: &native_theme::next::ResolvedTheme,
    state: WidgetState,
    color: iced_core::Color,
) -> Option<Value> {
    let c = &t.checkbox;
    if !c
        .indicator(WidgetState::DISABLED)
        .from_state
        .contains(native_theme::next::PartStated::FILL)
    {
        return None;
    }
    Some(Value::Faded {
        color,
        under: Some(super::fill_of(&c.indicator(state))?),
        ground: super::to_color(t.colors.window.background?),
        opacity: c.disabled_opacity,
    })
}

/// material-3's check box: the hovered stroke, checked or not, and the
/// disabled label and mark.
const M3_CHECK: &[Stated] = &[
    CHECK_HOVER_STROKES[0],
    CHECK_HOVER_STROKES[1],
    set(at!(DISABLED), Field::TextColor, CHECK_DISABLED_LABEL),
    set(
        at!(DISABLED | CHECKED),
        Field::TextColor,
        CHECK_DISABLED_LABEL,
    ),
    set(at!(DISABLED), Field::IconColor, CHECK_DISABLED_MARK),
    set(
        at!(DISABLED | CHECKED),
        Field::IconColor,
        CHECKED_DISABLED_MARK,
    ),
];

/// An indicator's outline of a check box or a radio button
/// (`$w.indicator`): its width and its stroke at rest, checked, and under
/// the pointer (checked: where a rule for HOVER states it, as the builder
/// reads it, `borders::flagged`).
macro_rules! indicator_outline {
    ($w:ident) => {
        [
            set(at!(NONE), Field::BorderWidth, home!(width, $w.indicator(NONE))),
            set(at!(NONE), Field::BorderColor, home!(stroke, $w.indicator(NONE))),
            set(at!(CHECKED), Field::BorderWidth, home!(width, $w.indicator(CHECKED))),
            set(at!(CHECKED), Field::BorderColor, home!(stroke, $w.indicator(CHECKED))),
            set(at!(HOVER), Field::BorderWidth, home!(width, $w.indicator(HOVER))),
            set(at!(HOVER), Field::BorderColor, home!(stroke, $w.indicator(HOVER))),
            set(
                at!(HOVER | CHECKED),
                Field::BorderWidth,
                home!(width, $w.indicator(HOVER | CHECKED) by HOVER),
            ),
            set(
                at!(HOVER | CHECKED),
                Field::BorderColor,
                home!(stroke, $w.indicator(HOVER | CHECKED) by HOVER),
            ),
        ]
    };
}

/// ios-13's check box: WebKit's outline round an empty box and none round a
/// checked one, the hovered box as at rest ([`indicator_outline`]).
const IOS_CHECK: [Stated; 8] = indicator_outline!(checkbox);

/// ios-13's empty box at rest, WebKit's -webkit-control-background
/// (`checkbox.indicator(NONE)` fill; RenderThemeIOS.mm:1391, :1395, :1027 at
/// WebKit 31adf9d; native-theme/tests/next_homes.toml, row
/// checkbox.unchecked_background, a listed difference).
const IOS_CHECK_FILL: [Stated; 1] = [set(
    at!(NONE),
    Field::Background,
    home!(fill, checkbox.indicator(NONE)),
)];

/// ios-13's empty circle at rest ([`IOS_CHECK_FILL`]).
const IOS_RADIO_FILL: [Stated; 1] = [set(
    at!(NONE),
    Field::Background,
    home!(fill, radio.indicator(NONE)),
)];

/// A disabled check box's outline, `checkbox.indicator(state)`'s stroke
/// (`state` `NONE` or `CHECKED`, the box at rest), laid over the box's
/// disabled fill (`indicator(DISABLED|state)` where a rule for DISABLED
/// states one, else the fill at rest), the two faded as one widget over the
/// window by `checkbox.disabled_opacity` (`crate::styles::checkbox_from`).
#[cfg(feature = "widgets")]
fn disabled_outline(t: &native_theme::next::ResolvedTheme, state: WidgetState) -> Option<Value> {
    let c = &t.checkbox;
    let line = super::borders::line_or_none(&c.indicator(state), "the stated stroke").ok()?;
    let disabled = c
        .indicator(WidgetState::DISABLED)
        .from_state
        .contains(native_theme::next::PartStated::FILL);
    let fill = if disabled {
        super::fill_of(&c.indicator(WidgetState::DISABLED.union(state)))?
    } else {
        super::fill_of(&c.indicator(state))?
    };
    Some(Value::Faded {
        color: line.over(fill),
        under: Some(fill),
        ground: super::to_color(t.colors.window.background?),
        opacity: c.disabled_opacity,
    })
}

/// The style functions, which alone read [`disabled_outline`], are
/// compared with the `widgets` feature only: no value without it.
#[cfg(not(feature = "widgets"))]
fn disabled_outline(_: &native_theme::next::ResolvedTheme, _: WidgetState) -> Option<Value> {
    None
}

/// [`disabled_outline`] of the box not checked.
const CHECK_DISABLED_OUTLINE: Home = Home {
    path: "checkbox.indicator(NONE).stroke over checkbox.indicator(DISABLED).fill, faded as \
           one over colors.window.background by checkbox.disabled_opacity",
    value: |t| disabled_outline(t, WidgetState::NONE),
};

/// [`disabled_outline`] of the checked box.
const CHECKED_DISABLED_OUTLINE: Home = Home {
    path: "checkbox.indicator(CHECKED).stroke over checkbox.indicator(DISABLED|CHECKED).fill, \
           faded as one over colors.window.background by checkbox.disabled_opacity",
    value: |t| disabled_outline(t, WidgetState::CHECKED),
};

/// A disabled box's fill, `checkbox.indicator(state)`, faded as one widget
/// over the window by `checkbox.disabled_opacity`.
fn disabled_fill(t: &native_theme::next::ResolvedTheme, state: WidgetState) -> Option<Value> {
    Some(Value::Faded {
        color: super::fill_of(&t.checkbox.indicator(state))?,
        under: None,
        ground: super::to_color(t.colors.window.background?),
        opacity: t.checkbox.disabled_opacity,
    })
}

/// [`disabled_fill`] of the box not checked.
const CHECK_DISABLED_FILL: Home = Home {
    path: "checkbox.indicator(DISABLED).fill, faded as one over colors.window.background by \
           checkbox.disabled_opacity",
    value: |t| disabled_fill(t, WidgetState::DISABLED),
};

/// [`disabled_fill`] of the checked box.
const CHECKED_DISABLED_FILL: Home = Home {
    path: "checkbox.indicator(DISABLED|CHECKED).fill, faded as one over \
           colors.window.background by checkbox.disabled_opacity",
    value: |t| disabled_fill(t, WidgetState::DISABLED.union(WidgetState::CHECKED)),
};

/// A disabled checked box whose fill and mark the theme states for the
/// state (kde-breeze, windows-11: M2.FS4a3): the fill, the checked outline
/// over it, and the mark on it.
const CHECKED_DISABLED: [Stated; 3] = [
    set(
        at!(DISABLED | CHECKED),
        Field::Background,
        CHECKED_DISABLED_FILL,
    ),
    set(
        at!(DISABLED | CHECKED),
        Field::BorderColor,
        CHECKED_DISABLED_OUTLINE,
    ),
    set(
        at!(DISABLED | CHECKED),
        Field::IconColor,
        CHECKED_DISABLED_MARK,
    ),
];

/// The mark of a disabled checked box whose disabled fill the theme states:
/// [`CHECKED_DISABLED_MARK`] where it states the disabled mark, else the
/// disabled label's colour on the checked box's disabled fill, faded as one
/// widget over the window.
const CHECKED_DISABLED_MARK_OR_LABEL: Home = Home {
    path: "checkbox.checkmark(DISABLED|CHECKED).color, else checkbox.label(DISABLED).color, \
           on checkbox.indicator(DISABLED|CHECKED).fill, faded as one over \
           colors.window.background by checkbox.disabled_opacity",
    value: |t| {
        (CHECKED_DISABLED_MARK.value)(t).or_else(|| {
            let color = super::text_color_of(&t.checkbox.label(WidgetState::DISABLED))?;
            disabled_mark(t, WidgetState::DISABLED.union(WidgetState::CHECKED), color)
        })
    },
};

/// adwaita's disabled check box, its rest fill stated for DISABLED
/// (_checks.scss:62-64 at libadwaita 1.10.0): the mark of the unchecked box
/// in the disabled label's colour on it.
const ADWAITA_CHECK_DISABLED: [Stated; 1] =
    [set(at!(DISABLED), Field::IconColor, CHECK_DISABLED_MARK)];

/// The macOS disabled check box (os-probe run 45345116763,
/// `*.normal.checkbox_disabled_{off,on}`): the measured fills, the rest and
/// checked outlines over them, the marks on them.
const MACOS_CHECK_DISABLED: [Stated; 6] = [
    set(at!(DISABLED), Field::Background, CHECK_DISABLED_FILL),
    set(
        at!(DISABLED | CHECKED),
        Field::Background,
        CHECKED_DISABLED_FILL,
    ),
    set(at!(DISABLED), Field::BorderColor, CHECK_DISABLED_OUTLINE),
    set(
        at!(DISABLED | CHECKED),
        Field::BorderColor,
        CHECKED_DISABLED_OUTLINE,
    ),
    set(at!(DISABLED), Field::IconColor, CHECK_DISABLED_MARK),
    set(
        at!(DISABLED | CHECKED),
        Field::IconColor,
        CHECKED_DISABLED_MARK_OR_LABEL,
    ),
];

/// ios-13's disabled check box at its full opacity (WebKit's `opacity:
/// initial`, M2.FS4a3): the fills, the label and the marks, checked or not.
const IOS_CHECK_DISABLED_COLORS: [Stated; 6] = [
    set(at!(DISABLED), Field::Background, CHECK_DISABLED_FILL),
    set(
        at!(DISABLED | CHECKED),
        Field::Background,
        CHECKED_DISABLED_FILL,
    ),
    set(at!(DISABLED), Field::TextColor, CHECK_DISABLED_LABEL),
    set(
        at!(DISABLED | CHECKED),
        Field::TextColor,
        CHECK_DISABLED_LABEL,
    ),
    set(at!(DISABLED), Field::IconColor, CHECK_DISABLED_MARK),
    set(
        at!(DISABLED | CHECKED),
        Field::IconColor,
        CHECKED_DISABLED_MARK,
    ),
];

/// adwaita's check box: no outline round a checked box (`box-shadow:
/// none`, _checks.scss:52-56 at libadwaita 1.10.0), the hovered unchecked
/// stroke ([`indicator_outline`]), and the checked outline, none, over the
/// disabled box.
const ADWAITA_CHECK: [Stated; 10] = {
    let [a, b, c, d, e, f, g, h] = indicator_outline!(checkbox);
    [
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        set(
            at!(DISABLED | CHECKED),
            Field::BorderWidth,
            home!(width, checkbox.indicator(CHECKED)),
        ),
        set(
            at!(DISABLED | CHECKED),
            Field::BorderColor,
            CHECKED_DISABLED_OUTLINE,
        ),
    ]
};

/// adwaita's radio button, as its check box ([`ADWAITA_CHECK`]; iced has no
/// disabled radio status).
const ADWAITA_RADIO: [Stated; 8] = indicator_outline!(radio);

/// adwaita's unchecked box under the pointer: the rest fill, stated for
/// HOVER, as libadwaita changes only the outline of an unchecked check or
/// radio under the pointer (`&:not(:checked):not(:indeterminate) { &:hover {
/// box-shadow: … } }`, _checks.scss:33-40 at libadwaita 1.10.0;
/// native-theme/tests/next_homes.toml, row checkbox.hover_background).
const ADWAITA_UNCHECKED_HOVER: [Stated; 1] = [set(
    at!(HOVER),
    Field::Background,
    home!(fill, checkbox.indicator(HOVER)),
)];

/// adwaita's unselected radio button under the pointer, as its check box's
/// ([`ADWAITA_UNCHECKED_HOVER`]; `check, radio`, _checks.scss:20-40).
const ADWAITA_UNCHECKED_RADIO_HOVER: [Stated; 1] = [set(
    at!(HOVER),
    Field::Background,
    home!(fill, radio.indicator(HOVER)),
)];

/// adwaita's checked box under the pointer: libadwaita's hover layer,
/// currentColor at 10 %, over the accent, the whole hovered fill
/// (`&:checked, &:indeterminate { &:hover { background-image: … } }`,
/// _checks.scss:52-58 at libadwaita 1.10.0; spec §9, step 2).
const ADWAITA_CHECKED_HOVER: [Stated; 1] = [set(
    at!(HOVER | CHECKED),
    Field::Background,
    home!(fill, checkbox.indicator(HOVER | CHECKED) by HOVER),
)];

/// adwaita's checked radio button under the pointer, as its check box's
/// ([`ADWAITA_CHECKED_HOVER`]; `check, radio`, _checks.scss:20-59).
const ADWAITA_CHECKED_RADIO_HOVER: [Stated; 1] = [set(
    at!(HOVER | CHECKED),
    Field::Background,
    home!(fill, radio.indicator(HOVER | CHECKED) by HOVER),
)];

/// material-3's radio button: the dot in primary in every status (iced
/// paints the dot of a checked radio alone; the builder reads
/// `radio.dot(CHECKED)`, which no rule sets apart from the dot at rest), and
/// nothing filling the checked circle (radio/internal/radio.ts:108-120 at
/// material-web cbd34a8).
const M3_RADIO: [Stated; 6] = [
    set(at!(NONE), Field::IconColor, home!(fill, radio.dot(NONE))),
    set(at!(CHECKED), Field::IconColor, home!(fill, radio.dot(NONE))),
    set(at!(HOVER), Field::IconColor, home!(fill, radio.dot(NONE))),
    set(
        at!(HOVER | CHECKED),
        Field::IconColor,
        home!(fill, radio.dot(NONE)),
    ),
    set(
        at!(CHECKED),
        Field::Background,
        home!(fill, radio.indicator(CHECKED)),
    ),
    set(
        at!(HOVER | CHECKED),
        Field::Background,
        home!(fill, radio.indicator(HOVER | CHECKED)),
    ),
];

/// ios-13's disabled check box: the outline at rest, checked or not, faded
/// with the box ([`disabled_outline`]).
const IOS_CHECK_DISABLED: [Stated; 4] = [
    set(
        at!(DISABLED),
        Field::BorderWidth,
        home!(width, checkbox.indicator(NONE)),
    ),
    set(at!(DISABLED), Field::BorderColor, CHECK_DISABLED_OUTLINE),
    set(
        at!(DISABLED | CHECKED),
        Field::BorderWidth,
        home!(width, checkbox.indicator(CHECKED)),
    ),
    set(
        at!(DISABLED | CHECKED),
        Field::BorderColor,
        CHECKED_DISABLED_OUTLINE,
    ),
];

/// ios-13's radio button, as its check box ([`IOS_CHECK`]; iced has no
/// disabled radio status).
const IOS_RADIO: [Stated; 8] = indicator_outline!(radio);

/// A disabled text field's outline, `field(NONE)`'s stroke laid over the
/// field's disabled fill (`field(DISABLED)` where a rule for DISABLED states
/// one, else the fill at rest), then faded by the field's disabled opacity,
/// as `crate::styles::text_input_from` draws it.
#[cfg(feature = "widgets")]
fn field_disabled_line(w: &native_theme::next::InputWidget) -> Option<Value> {
    use native_theme::next::PartStated;
    let line =
        super::borders::line_or_none(&w.field(WidgetState::NONE), "the stated stroke").ok()?;
    let disabled = w.field(WidgetState::DISABLED);
    let fill = if disabled.from_state.contains(PartStated::FILL) {
        super::fill_of(&disabled)?
    } else {
        super::fill_of(&w.field(WidgetState::NONE))?
    };
    let color = line.over(fill);
    Some(Value::Color(w.disabled_opacity.map_or(color, |opacity| {
        crate::styles::faded(color, opacity)
    })))
}

/// The field outlines, which alone read [`field_disabled_line`], are
/// compared with the `widgets` feature only: no value without it.
#[cfg(not(feature = "widgets"))]
fn field_disabled_line(_: &native_theme::next::InputWidget) -> Option<Value> {
    None
}

/// A text field's outline (`$w.field(NONE)`, ios-13's WebKit border): its
/// width in every status, the builder keeping the rest width (`InputSource`'s
/// `border`), and its colour at rest and disabled ([`field_disabled_line`]);
/// hovered and focused the colour the theme states for the state, which
/// 0.7.2 states alike.
macro_rules! field_outline {
    ($w:ident) => {
        [
            set(at!(NONE), Field::BorderWidth, home!(width, $w.field(NONE))),
            set(at!(NONE), Field::BorderColor, home!(stroke, $w.field(NONE))),
            set(at!(HOVER), Field::BorderWidth, home!(width, $w.field(NONE))),
            set(
                at!(FOCUSED),
                Field::BorderWidth,
                home!(width, $w.field(NONE)),
            ),
            set(
                at!(FOCUSED | HOVER),
                Field::BorderWidth,
                home!(width, $w.field(NONE)),
            ),
            set(
                at!(DISABLED),
                Field::BorderWidth,
                home!(width, $w.field(NONE)),
            ),
            set(
                at!(DISABLED),
                Field::BorderColor,
                Home {
                    path: concat!(
                        stringify!($w),
                        ".field(NONE).stroke over ",
                        stringify!($w),
                        ".field(DISABLED).fill, faded by ",
                        stringify!($w),
                        ".disabled_opacity"
                    ),
                    value: |t| field_disabled_line(&t.$w),
                },
            ),
        ]
    };
}

/// ios-13's text field's outline ([`field_outline`]).
const IOS_INPUT_OUTLINE: [Stated; 7] = field_outline!(input);

/// ios-13's text area's outline ([`field_outline`]).
const IOS_TEXT_AREA_OUTLINE: [Stated; 7] = field_outline!(text_area);

/// A drop-down field's outline at rest, under the pointer and open
/// (`combo_box.field`): its stroke at rest and under the pointer, open the
/// width at rest (iced's own colour, no rule for OPEN), and open under the
/// pointer the hovered stroke (`field(OPEN|HOVER)`).
const COMBO_OUTLINE: [Stated; 7] = [
    set(
        at!(NONE),
        Field::BorderWidth,
        home!(width, combo_box.field(NONE)),
    ),
    set(
        at!(NONE),
        Field::BorderColor,
        home!(stroke, combo_box.field(NONE)),
    ),
    set(
        at!(HOVER),
        Field::BorderWidth,
        home!(width, combo_box.field(HOVER)),
    ),
    set(
        at!(HOVER),
        Field::BorderColor,
        home!(stroke, combo_box.field(HOVER)),
    ),
    set(
        at!(OPEN),
        Field::BorderWidth,
        home!(width, combo_box.field(NONE)),
    ),
    set(
        at!(OPEN | HOVER),
        Field::BorderWidth,
        home!(width, combo_box.field(OPEN | HOVER)),
    ),
    set(
        at!(OPEN | HOVER),
        Field::BorderColor,
        home!(stroke, combo_box.field(OPEN | HOVER)),
    ),
];

/// macos-monterey's drop-down under the pointer, closed and open: the
/// hovered fill (`combo_box.field(HOVER)`, the rest fill, as a hovered
/// pop-up button draws as at rest) and the hovered stroke laid over it.
const COMBO_HOVER: [Stated; 4] = [
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, combo_box.field(HOVER)),
    ),
    set(
        at!(HOVER),
        Field::BorderColor,
        home!(stroke, combo_box.field(HOVER)),
    ),
    set(
        at!(OPEN | HOVER),
        Field::Background,
        home!(fill, combo_box.field(OPEN | HOVER)),
    ),
    set(
        at!(OPEN | HOVER),
        Field::BorderColor,
        home!(stroke, combo_box.field(OPEN | HOVER)),
    ),
];

/// macos-tahoe's segmented control stroked as none: the track's width and
/// stroke, at rest and disabled (no line, which no fade shows).
const TAHOE_TRACK: [Stated; 4] = [
    set(
        at!(NONE),
        Field::BorderWidth,
        home!(width, segmented_control.track(NONE)),
    ),
    set(
        at!(NONE),
        Field::BorderColor,
        home!(stroke, segmented_control.track(NONE)),
    ),
    set(
        at!(DISABLED),
        Field::BorderWidth,
        home!(width, segmented_control.track(NONE)),
    ),
    set(
        at!(DISABLED),
        Field::BorderColor,
        home!(stroke, segmented_control.track(NONE)),
    ),
];

/// macos-tahoe's segment separator: none, the track stroked as none.
const TAHOE_SEPARATOR: [Stated; 2] = [
    set(
        at!(NONE),
        Field::Separator,
        home!(stroke, segmented_control.track(NONE)),
    ),
    set(
        at!(DISABLED),
        Field::Separator,
        home!(stroke, segmented_control.track(NONE)),
    ),
];

/// The line between two segments where the preset states it: the
/// separator part's fill, which 0.7.2 drew in the outline's colour; when
/// disabled, laid on the group and faded with the control
/// ([`DISABLED_SEPARATOR`]).
const SEPARATOR_FILL: [Stated; 2] = [
    set(
        at!(NONE),
        Field::Separator,
        home!(fill, segmented_control.separator(NONE)),
    ),
    set(at!(DISABLED), Field::Separator, DISABLED_SEPARATOR),
];

/// The popover panel's corners, `popover.panel.corners` (a part of no
/// rules, read at rest), as the builder hands them to iced.
const POPOVER_CORNERS: Home = Home {
    path: "popover.panel.corners",
    value: |t| read::corners(&t.popover.panel, &t.popover.panel, WidgetState::NONE),
};

/// A disabled control's separator, `segmented_control.separator(NONE)`'s
/// fill laid on the group fill (`track(NONE)`), the two faded as one widget
/// over the window by `segmented_control.disabled_opacity`
/// (`crate::styles::segment_separator_from`).
#[cfg(feature = "widgets")]
fn disabled_separator(t: &native_theme::next::ResolvedTheme) -> Option<Value> {
    let s = &t.segmented_control;
    Some(Value::Faded {
        color: super::fill_of(&s.separator(WidgetState::NONE))?,
        under: Some(super::fill_of(&s.track(WidgetState::NONE))?),
        ground: super::to_color(t.colors.window.background?),
        opacity: s.disabled_opacity,
    })
}

/// The style functions, which alone read [`disabled_separator`], are
/// compared with the `widgets` feature only: no value without it.
#[cfg(not(feature = "widgets"))]
fn disabled_separator(_: &native_theme::next::ResolvedTheme) -> Option<Value> {
    None
}

/// [`disabled_separator`]'s home.
const DISABLED_SEPARATOR: Home = Home {
    path: "segmented_control.separator(NONE).fill over segmented_control.track(NONE).fill, \
           faded as one over colors.window.background by segmented_control.disabled_opacity",
    value: disabled_separator,
};

/// The track's corner radius less its stroke width, which a segment's
/// rounded corners take (`crate::styles::segment_from`), where the theme
/// states both.
const TRACK_INNER_RADIUS: Home = Home {
    path: "segmented_control.track(NONE).corners less its stroke.width",
    value: track_inner_radius,
};

/// [`TRACK_INNER_RADIUS`]'s value.
#[cfg(feature = "widgets")]
fn track_inner_radius(t: &native_theme::next::ResolvedTheme) -> Option<Value> {
    let track = t.segmented_control.track(WidgetState::NONE);
    let inner = super::radius_of(&track)? - super::stroke_width_of(&track)?;
    Some(Value::Width(inner.max(0.0)))
}

/// The style functions, which alone read [`TRACK_INNER_RADIUS`], are
/// compared with the `widgets` feature only: no value without it.
#[cfg(not(feature = "widgets"))]
fn track_inner_radius(_: &native_theme::next::ResolvedTheme) -> Option<Value> {
    None
}

/// A segment's rounded corners in every status, selected or not
/// ([`TRACK_INNER_RADIUS`]).
const SEGMENT_CORNERS: [Stated; 8] = [
    set(at!(NONE), Field::InnerRadius, TRACK_INNER_RADIUS),
    set(at!(HOVER), Field::InnerRadius, TRACK_INNER_RADIUS),
    set(at!(PRESSED), Field::InnerRadius, TRACK_INNER_RADIUS),
    set(at!(DISABLED), Field::InnerRadius, TRACK_INNER_RADIUS),
    set(at!(SELECTED), Field::InnerRadius, TRACK_INNER_RADIUS),
    set(
        at!(HOVER | SELECTED),
        Field::InnerRadius,
        TRACK_INNER_RADIUS,
    ),
    set(
        at!(PRESSED | SELECTED),
        Field::InnerRadius,
        TRACK_INNER_RADIUS,
    ),
    set(
        at!(DISABLED | SELECTED),
        Field::InnerRadius,
        TRACK_INNER_RADIUS,
    ),
];

/// A pane grid's split, picked and hovered, in the hovered handle's fill
/// (`splitter.handle(HOVER)`, which the builder reads for both).
const PANE_SPLITS: [Stated; 2] = [
    set(
        at!(NONE),
        Field::PickedSplitColor,
        home!(fill, splitter.handle(HOVER)),
    ),
    set(
        at!(NONE),
        Field::HoveredSplitColor,
        home!(fill, splitter.handle(HOVER)),
    ),
];

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    // button.body(NONE), its states and its variants. macos-monterey states
    // no checked fill (`body(CHECKED)`), and macos-tahoe and ios-13 state
    // one 0.7.2 has no field for; monterey, tahoe, material-3 and ios-13
    // state no checked toolbar fill (`toolbar.button(CHECKED)`), ios-13 a
    // checked label. The FS4 presets state monterey's and tahoe's hovered
    // and pressed fills, tahoe's bezel stroked as none, ios-13's outline,
    // and the hovered and pressed strokes of material-3 and windows-11.
    inside("button", &["macos-monterey"]).stating(MONTEREY_BUTTON, &[INSIDE, STATED]),
    checked(
        "button_toggle",
        &["macos-monterey"],
        LineAlign::Inside,
        false,
    )
    .stating(MONTEREY_BUTTON, &[INSIDE, CHECKED, STATED]),
    stated_fields("button", &["macos-tahoe"], LineAlign::Inside, TAHOE_BUTTON),
    stated_checked("button_toggle", &["macos-tahoe"], LineAlign::Inside, false)
        .stating(TAHOE_TOGGLE, &[INSIDE, STATED_CHECKED, STATED]),
    stated_fields(
        "button",
        &["ios-13"],
        LineAlign::Outside,
        &[&BUTTON_OUTLINE],
    ),
    stated_checked("button_toggle", &["ios-13"], LineAlign::Outside, false)
        .stating(TOGGLE_OUTLINE, &[STATED_CHECKED, STATED]),
    stated_fields("button", &["material-3"], LineAlign::Outside, STATE_STROKES),
    stated_fields(
        "button_toggle",
        &["material-3"],
        LineAlign::Outside,
        TOGGLE_STATE_STROKES,
    ),
    stated_fields(
        "button",
        &["windows-11"],
        LineAlign::Outside,
        PRESSED_STROKE,
    ),
    stated_fields(
        "button_toggle",
        &["windows-11"],
        LineAlign::Outside,
        TOGGLE_PRESSED_STROKE,
    ),
    unchecked_toolbar(&["macos-tahoe"])
        .stating(TAHOE_TOOLBAR, &[MEASURED_TOOLBAR_FILLS, CHECKED, STATED]),
    Difference {
        leaf: "toolbar_button",
        presets: &["ios-13"],
        listed: Listed {
            states: States {
                hover_fill: true,
                pressed_fill: true,
                checked: true,
                ..States::STATED
            },
            added: Added::CheckedButton,
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[&TOOLBAR_WIDTHS],
        reason: &[TOOLBAR_STATE_FILLS, CHECKED, STATED_CHECKED, STATED],
    },
    unchecked_toolbar(&["macos-monterey"])
        .stating(MONTEREY_TOOLBAR, &[MEASURED_TOOLBAR_FILLS, CHECKED, STATED]),
    unfilled_toolbar(&["material-3"]),
    // windows-11's primary variant lays its stroke inside the box, the plain
    // button's outside (`button.body(NONE; variant=Primary).stroke`); its
    // default button is the primary one.
    inside("button_primary", &["windows-11", "macos-monterey"]),
    inside("button_default", &["windows-11", "macos-monterey"]),
    stated_fields(
        "button_primary",
        &["macos-tahoe"],
        LineAlign::Inside,
        &[&PRIMARY_OUTLINE],
    ),
    stated_fields(
        "button_default",
        &["macos-tahoe"],
        LineAlign::Inside,
        &[&PRIMARY_OUTLINE],
    ),
    stated_fields(
        "button_primary",
        &["ios-13"],
        LineAlign::Outside,
        &[&PRIMARY_OUTLINE],
    ),
    stated_fields(
        "button_default",
        &["ios-13"],
        LineAlign::Outside,
        &[&PRIMARY_OUTLINE],
    ),
    stated_fields(
        "button_primary",
        &["material-3"],
        LineAlign::Outside,
        PRIMARY_STATE_STROKES,
    ),
    stated_fields(
        "button_default",
        &["material-3"],
        LineAlign::Outside,
        PRIMARY_STATE_STROKES,
    ),
    inside("button_danger", &["macos-monterey"]),
    inside("button_success", &["macos-monterey"]),
    inside("button_warning", &["macos-monterey"]),
    stated_fields(
        "button_danger",
        &["macos-tahoe"],
        LineAlign::Inside,
        &[&BUTTON_OUTLINE],
    ),
    stated_fields(
        "button_success",
        &["macos-tahoe"],
        LineAlign::Inside,
        &[&BUTTON_OUTLINE],
    ),
    stated_fields(
        "button_warning",
        &["macos-tahoe"],
        LineAlign::Inside,
        &[&BUTTON_OUTLINE],
    ),
    stated_fields(
        "button_danger",
        &["ios-13"],
        LineAlign::Outside,
        &[&BUTTON_OUTLINE],
    ),
    stated_fields(
        "button_success",
        &["ios-13"],
        LineAlign::Outside,
        &[&BUTTON_OUTLINE],
    ),
    stated_fields(
        "button_warning",
        &["ios-13"],
        LineAlign::Outside,
        &[&BUTTON_OUTLINE],
    ),
    stated_fields(
        "button_danger",
        &["material-3"],
        LineAlign::Outside,
        STATE_STROKES,
    ),
    stated_fields(
        "button_success",
        &["material-3"],
        LineAlign::Outside,
        STATE_STROKES,
    ),
    stated_fields(
        "button_warning",
        &["material-3"],
        LineAlign::Outside,
        STATE_STROKES,
    ),
    stated_fields(
        "button_danger",
        &["windows-11"],
        LineAlign::Outside,
        PRESSED_STROKE,
    ),
    stated_fields(
        "button_success",
        &["windows-11"],
        LineAlign::Outside,
        PRESSED_STROKE,
    ),
    stated_fields(
        "button_warning",
        &["windows-11"],
        LineAlign::Outside,
        PRESSED_STROKE,
    ),
    // input.field(NONE) and text_area.field(NONE), the macOS presets'
    // inside the box; input.field(FOCUSED): the macOS presets state no
    // focused stroke. The FS4 presets state adwaita's and the macOS
    // presets' hovered stroke as the rest one (`field(HOVER)`), as 0.7.2
    // drew it.
    field("text_input", MACOS, LineAlign::Inside),
    field("text_editor", MACOS, LineAlign::Inside),
    // input.field(NONE) and text_area.field(NONE): ios-13 states WebKit's
    // `border: 1px solid -apple-system-secondary-label` (html.css:457-466
    // at WebKit 31adf9d; native-theme/tests/next_homes.toml, rows
    // input.border.* and text_area.border.*, listed differences).
    stated_fields(
        "text_input",
        &["ios-13"],
        LineAlign::Outside,
        &[&IOS_INPUT_OUTLINE],
    ),
    stated_fields(
        "text_editor",
        &["ios-13"],
        LineAlign::Outside,
        &[&IOS_TEXT_AREA_OUTLINE],
    ),
    // tooltip.panel, dialog.panel, progress_bar.track(NONE).
    inside("tooltip", MACOS_WINDOWS),
    inside("dialog", MACOS_WINDOWS),
    inside("progress_bar", MACOS_WINDOWS),
    // card.container(NONE).
    inside("container_card", &["windows-11"]),
    // card.container(NONE): NSBox's and the .insetGrouped cell's measured edge,
    // and NSBox's measured fill.
    stated_fields(
        "container_card",
        &["macos-monterey"],
        LineAlign::Outside,
        &[&[
            set(
                at!(NONE),
                Field::Background,
                home!(fill, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::BorderWidth,
                home!(width, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::BorderColor,
                home!(stroke, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::Radius,
                home!(corners, card.container(NONE)),
            ),
        ]],
    ),
    stated_fields(
        "container_card",
        &["macos-tahoe"],
        LineAlign::Outside,
        &[&[
            set(
                at!(NONE),
                Field::Background,
                home!(fill, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::Radius,
                home!(corners, card.container(NONE)),
            ),
        ]],
    ),
    stated_fields(
        "container_card",
        &["ios-13"],
        LineAlign::Outside,
        &[&[set(
            at!(NONE),
            Field::Radius,
            home!(corners, card.container(NONE)),
        )]],
    ),
    // Breeze's progress groove, a pen centred on its edge
    // (progress_bar.track(NONE).stroke).
    Difference {
        leaf: "progress_bar",
        presets: &["kde-breeze"],
        listed: Listed {
            align: LineAlign::Center,
            states: States::STATED,
            iced_corners: false,
            iced_stroke: false,
            added: Added::Nothing,
        },
        stated: &[],
        reason: &[CENTER],
    },
    // checkbox.indicator and radio.indicator: the hovered fill and stroke
    // (`indicator(HOVER)`, `indicator(HOVER|CHECKED)`), the strokes of
    // windows-11 and the macOS presets inside the box. On the macOS presets
    // that is the transparent unchecked stroke too, which iced is handed as
    // the fill under it (0.7.2's `checkbox.unchecked_border_color` is
    // `#00000000`, the home's stroke the same colour, Inside).
    // The FS4 presets state kde-breeze's hovered box as 0.7.2 drew it,
    // adwaita's and material-3's hovered stroke, windows-11's hovered fill
    // and stroke, and material-3's disabled label (`label(DISABLED)`).
    hover(
        "checkbox",
        &["ios-13"],
        LineAlign::Outside,
        hovered(true, true, false),
    )
    .stating(
        &[
            &IOS_CHECK_FILL,
            &IOS_CHECK,
            &IOS_CHECK_DISABLED,
            &IOS_CHECK_DISABLED_COLORS,
        ],
        &[HOVER, STATED, IOS_BOX_HOVER],
    ),
    stated_fields(
        "checkbox",
        &["adwaita"],
        LineAlign::Outside,
        &[
            &ADWAITA_UNCHECKED_HOVER,
            &ADWAITA_CHECKED_HOVER,
            &ADWAITA_CHECK,
            &ADWAITA_CHECK_DISABLED,
        ],
    ),
    stated_fields(
        "checkbox",
        &["kde-breeze"],
        LineAlign::Outside,
        &[&[CHECKED_DISABLED[0], CHECKED_DISABLED[2]]],
    ),
    stated_fields("checkbox", &["material-3"], LineAlign::Outside, &[M3_CHECK]),
    stated_fields(
        "checkbox",
        &["macos-monterey"],
        LineAlign::Inside,
        &[
            &[
                set(
                    at!(HOVER),
                    Field::Background,
                    home!(fill, checkbox.indicator(HOVER)),
                ),
                CHECK_HOVER_STROKES[0],
            ],
            &MACOS_CHECK_DISABLED,
        ],
    ),
    stated_fields(
        "checkbox",
        &["windows-11"],
        LineAlign::Inside,
        &[W11_CHECK_HOVER, &CHECKED_DISABLED],
    ),
    hover(
        "checkbox",
        &["macos-tahoe"],
        LineAlign::Inside,
        hovered(false, true, false),
    )
    .stating(
        &[&TAHOE_CHECK_HOVER, &MACOS_CHECK_DISABLED],
        &[INSIDE, HOVER, STATED],
    ),
    hover(
        "radio",
        &["kde-breeze"],
        LineAlign::Outside,
        hovered(true, true, false),
    ),
    stated_fields(
        "radio",
        &["macos-monterey", "macos-tahoe"],
        LineAlign::Inside,
        &[&MACOS_RADIO_HOVER],
    ),
    stated_fields(
        "radio",
        &["windows-11"],
        LineAlign::Inside,
        &[W11_RADIO_HOVER],
    ),
    stated_fields(
        "radio",
        &["adwaita"],
        LineAlign::Outside,
        &[
            &ADWAITA_UNCHECKED_RADIO_HOVER,
            &ADWAITA_CHECKED_RADIO_HOVER,
            &ADWAITA_RADIO,
        ],
    ),
    stated_fields("radio", &["material-3"], LineAlign::Outside, &[&M3_RADIO]),
    hover(
        "radio",
        &["ios-13"],
        LineAlign::Outside,
        hovered(true, true, false),
    )
    .stating(
        &[&IOS_RADIO_FILL, &IOS_RADIO],
        &[HOVER, STATED, IOS_BOX_HOVER],
    ),
    // combo_box.field: the open stroke (`field(OPEN)`), which kde-breeze
    // alone states. The FS4 presets state the hovered stroke as the rest
    // one on adwaita, windows-11, the macOS presets and ios-13 (`field(HOVER)`),
    // macos-monterey's hovered fill as the rest one, and macos-tahoe's and
    // ios-13's outline at rest (`field(NONE)`: none, and WebKit's 1px
    // systemBackground border).
    open(
        &["adwaita", "material-3"],
        LineAlign::Outside,
        States::STATED,
    ),
    open(
        &["macos-tahoe", "ios-13"],
        LineAlign::Outside,
        States::STATED,
    )
    .stating(&[&COMBO_OUTLINE], &[OPEN, STATED]),
    open(&["windows-11"], LineAlign::Inside, States::STATED),
    open(&["macos-monterey"], LineAlign::Inside, States::STATED)
        .stating(&[&COMBO_HOVER], &[INSIDE, OPEN, STATED]),
    // switch.thumb(HOVER): no preset states a hovered thumb; windows-11 and
    // adwaita state the checked track's hovered fill (`track(HOVER|CHECKED)`:
    // AccentFillColorSecondary, and libadwaita's hover layer over the accent).
    Difference {
        leaf: "toggler",
        presets: &["kde-breeze", "macos-tahoe", "material-3", "ios-13"],
        listed: Listed {
            states: States {
                hover_thumb: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[HOVER_THUMB],
    },
    // macos-monterey's off track under the pointer, its rest fill (`track(HOVER)`).
    Difference {
        leaf: "toggler",
        presets: &["macos-monterey"],
        listed: Listed {
            states: States {
                hover_thumb: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[&[set(
            at!(HOVER),
            Field::Background,
            home!(fill, switch.track(HOVER)),
        )]],
        reason: &[HOVER_THUMB, STATED],
    },
    Difference {
        leaf: "toggler",
        presets: &["windows-11", "adwaita"],
        listed: Listed {
            states: States {
                hover_thumb: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[&[set(
            at!(HOVER | CHECKED),
            Field::Background,
            home!(fill, switch.track(HOVER | CHECKED) by HOVER),
        )]],
        reason: &[HOVER_THUMB, STATED],
    },
    // The menu's frame, popover.panel's stroke over menu.panel's fill.
    inside(
        "menu",
        &["adwaita", "windows-11", "macos-monterey", "macos-tahoe"],
    ),
    // slider.active_track(HOVER) and (PRESSED): no preset states them;
    // slider.thumb(HOVER): no hovered thumb fill (macos-monterey: in its dark
    // scheme); kde-breeze, windows-11 and adwaita state the hovered and the
    // pressed thumb's (`thumb(HOVER)`, `thumb(PRESSED)`), material-3 the
    // pressed one.
    // macos-tahoe: the knob as at rest hovered and pressed, the filled rail's measured pressed
    // fill (`active_track(PRESSED)`).
    Difference {
        leaf: "slider",
        presets: &["macos-tahoe"],
        listed: Listed {
            states: States {
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[&[
            set(
                at!(HOVER),
                Field::HandleBackground,
                home!(fill, slider.thumb(HOVER)),
            ),
            set(
                at!(PRESSED),
                Field::HandleBackground,
                home!(fill, slider.thumb(PRESSED)),
            ),
            set(
                at!(PRESSED),
                Field::RailFill,
                home!(fill, slider.active_track(PRESSED)),
            ),
        ]],
        reason: &[SLIDER_RAIL, STATED],
    },
    Difference {
        leaf: "slider",
        presets: &["kde-breeze", "windows-11", "adwaita", "macos-monterey"],
        listed: Listed {
            states: States {
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[&[
            set(
                at!(HOVER),
                Field::HandleBackground,
                home!(fill, slider.thumb(HOVER)),
            ),
            set(
                at!(PRESSED),
                Field::HandleBackground,
                home!(fill, slider.thumb(PRESSED)),
            ),
        ]],
        reason: &[SLIDER_RAIL, STATED],
    },
    Difference {
        leaf: "slider",
        presets: &["material-3"],
        listed: Listed {
            states: States {
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[&[set(
            at!(PRESSED),
            Field::HandleBackground,
            home!(fill, slider.thumb(PRESSED)),
        )]],
        reason: &[SLIDER_RAIL, STATED],
    },
    // ios-13: also the thumb at rest, UISlider's measured white, where 0.7.2
    // left slider.thumb_color unset (defaults.surface_color).
    Difference {
        leaf: "slider",
        presets: &["ios-13"],
        listed: Listed {
            states: States {
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[&[
            set(
                at!(NONE),
                Field::HandleBackground,
                home!(fill, slider.thumb(NONE)),
            ),
            set(
                at!(PRESSED),
                Field::HandleBackground,
                home!(fill, slider.thumb(PRESSED)),
            ),
        ]],
        reason: &[SLIDER_RAIL, STATED],
    },
    // Breeze's handle outline (scrollbar.thumb(UNFOCUSED_VIEW), (HOVER) and
    // (PRESSED): stroke and corners).
    breeze_scroller(&["kde-breeze/light"], BREEZE_TEXT_FRAME_LIGHT),
    breeze_scroller(&["kde-breeze/dark"], BREEZE_TEXT_FRAME_DARK),
    // scrollbar.thumb(PRESSED) and scrollbar.thumb(HOVER).
    Difference {
        leaf: "scrollable",
        presets: &["windows-11"],
        listed: Listed {
            align: LineAlign::Outside,
            states: States {
                pressed_fill: true,
                ..States::STATED
            },
            iced_corners: false,
            iced_stroke: false,
            added: Added::Nothing,
        },
        stated: &[],
        reason: &[PRESSED_THUMB],
    },
    // The measured overlay knob, #6a6a6adf / #f8f8f893 on macOS 15, #00000085 / #ffffff91 on
    // macOS 26.
    macos_scroller(
        &["macos-monterey/light"],
        iced_core::Color::from_rgba8(0x6a, 0x6a, 0x6a, 223.0 / 255.0),
    ),
    macos_scroller(
        &["macos-monterey/dark"],
        iced_core::Color::from_rgba8(0xf8, 0xf8, 0xf8, 147.0 / 255.0),
    ),
    macos_scroller(
        &["macos-tahoe/light"],
        iced_core::Color::from_rgba8(0x00, 0x00, 0x00, 133.0 / 255.0),
    ),
    macos_scroller(
        &["macos-tahoe/dark"],
        iced_core::Color::from_rgba8(0xff, 0xff, 0xff, 145.0 / 255.0),
    ),
    // segmented_control.track(NONE).stroke over the group fill.
    inside("segmented_control", &["windows-11", "macos-monterey"]),
    // macos-tahoe states its track stroked as none (`track(NONE)`): no
    // outline, no separator, and a segment's corners the track's whole
    // radius.
    stated_fields(
        "segmented_control",
        &["macos-tahoe"],
        LineAlign::Outside,
        &[&TAHOE_TRACK],
    ),
    stated_fields(
        "segment_separator",
        &["macos-tahoe"],
        LineAlign::Outside,
        &[&TAHOE_SEPARATOR],
    ),
    // adwaita states the line between toggles, GtkSeparator's $border_color
    // (segmented_control.separator(NONE).fill), where 0.7.2 drew the
    // outline's colour.
    stated_fields(
        "segment_separator",
        &["adwaita"],
        LineAlign::Outside,
        &[&SEPARATOR_FILL],
    ),
    stated_fields(
        "segment",
        &["macos-tahoe"],
        LineAlign::Outside,
        &[
            &SEGMENT_CORNERS,
            &[set(
                at!(PRESSED),
                Field::Background,
                home!(fill, segmented_control.segment(PRESSED)),
            )],
        ],
    ),
    // ios-13 states the toolbar's button label, toolbar.button_label(NONE),
    // in the tint, a bar button item's colour, and the macOS presets the real
    // toolbar's measured label, where 0.7.2 drew the body text.
    stated_fields(
        "toolbar",
        &["ios-13", "macos-monterey", "macos-tahoe"],
        LineAlign::Outside,
        &[&[set(
            at!(NONE),
            Field::TextColor,
            home!(color, toolbar.button_label(NONE)),
        )]],
    ),
    // splitter.handle(HOVER), both splits' colour.
    stated_fields(
        "pane_grid",
        &["kde-breeze", "adwaita"],
        LineAlign::Outside,
        &[&PANE_SPLITS],
    ),
    // segmented_control.segment(PRESSED) and expander.header(PRESSED): the
    // macOS and iOS presets state none, the others' FS4 presets do,
    // kde-breeze's pressed segment as 0.7.2 drew it (and kde-breeze's,
    // windows-11's and material-3's hovered header, `header(HOVER)`);
    // macos-tahoe states no hovered fill either.
    pressed("segment", &["ios-13"], LineAlign::Outside),
    // macos-monterey: an unselected segment's measured pressed fill; hovered, no fill of its own
    // (`segment(HOVER)` unstated).
    hover(
        "segment",
        &["macos-monterey"],
        LineAlign::Outside,
        hovered(true, false, false),
    )
    .stating(
        &[&[set(
            at!(PRESSED),
            Field::Background,
            home!(fill, segmented_control.segment(PRESSED)),
        )]],
        &[HOVER, STATED],
    ),
    stated_fields(
        "segment",
        &["adwaita", "windows-11", "material-3"],
        LineAlign::Outside,
        &[&[set(
            at!(PRESSED),
            Field::Background,
            home!(fill, segmented_control.segment(PRESSED)),
        )]],
    ),
    stated_fields(
        "expander",
        &["macos-monterey", "macos-tahoe"],
        LineAlign::Outside,
        &[&EXPANDER_FILLS],
    ),
    stated_fields(
        "expander",
        &["ios-13"],
        LineAlign::Outside,
        &[&[EXPANDER_FILLS[1]]],
    ),
    stated_fields(
        "expander",
        &["adwaita"],
        LineAlign::Outside,
        &[&[EXPANDER_FILLS[1]]],
    ),
    stated_fields(
        "expander",
        &["kde-breeze", "windows-11", "material-3"],
        LineAlign::Outside,
        &[&EXPANDER_FILLS],
    ),
    // iced_aw: card.container(NONE).
    inside("aw::card", &["windows-11"]),
    // NSBox's and the .insetGrouped cell's measured edge: macos-monterey's line and corners,
    // macos-tahoe's and ios-13's corners over no line; NSBox's measured fill.
    stated_fields(
        "aw::card",
        &["macos-monterey"],
        LineAlign::Outside,
        &[&[
            set(
                at!(NONE),
                Field::Background,
                home!(fill, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::BorderWidth,
                home!(width, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::BorderColor,
                home!(stroke, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::Radius,
                home!(corners, card.container(NONE)),
            ),
        ]],
    ),
    stated_fields(
        "aw::card",
        &["macos-tahoe"],
        LineAlign::Outside,
        &[&[
            set(
                at!(NONE),
                Field::Background,
                home!(fill, card.container(NONE)),
            ),
            set(
                at!(NONE),
                Field::Radius,
                home!(corners, card.container(NONE)),
            ),
        ]],
    ),
    stated_fields(
        "aw::card",
        &["ios-13"],
        LineAlign::Outside,
        &[&[set(
            at!(NONE),
            Field::Radius,
            home!(corners, card.container(NONE)),
        )]],
    ),
    // iced_aw: the menu's frame, popover.panel's stroke over menu.panel's
    // fill.
    inside(
        "aw::menu",
        &["adwaita", "windows-11", "macos-monterey", "macos-tahoe"],
    ),
    // iced_aw: kde-breeze's menu frame rounded by the popover's corners
    // (menu.panel states none), qqc2-desktop-style's Popup radius,
    // Kirigami.Units.cornerRadius 5, where 0.7.2's corner_radius_lg 8 is
    // uncited.
    stated_fields(
        "aw::menu",
        &["kde-breeze"],
        LineAlign::Outside,
        &[&[set(at!(NONE), Field::Radius, POPOVER_CORNERS)]],
    ),
    // iced_aw: tab.tab(SELECTED)'s and tab.tab(NONE)'s strokes and
    // tab.tab(HOVER)'s fill.
    inside("aw::tab_bar", &["windows-11"]),
    stated_fields(
        "aw::tab_bar",
        &["adwaita"],
        LineAlign::Outside,
        &[&[set(
            at!(HOVER),
            Field::Background,
            home!(fill, tab.tab(HOVER)),
        )]],
    ),
    aw(
        "aw::tab_bar",
        &["ios-13"],
        LineAlign::Outside,
        States::STATED,
        (true, false),
        &[TAB_STROKE, TAB_IDLE_STROKE],
    ),
    // the macOS presets state an unselected tab's measured fill (`tab.tab(NONE)`) and a hovered
    // tab's as its rest one (`tab.tab(HOVER)`), and the selected tab's measured fill and label
    // (`tab.tab(SELECTED)`, `tab.label(SELECTED)`; macos-tahoe's on the accent, lettered white),
    // its icon in the label's colour.
    aw(
        "aw::tab_bar",
        MACOS,
        LineAlign::Outside,
        States::STATED,
        (true, false),
        &[TAB_STROKE, TAB_IDLE_STROKE],
    )
    .stating(
        &[
            &TAB_FILLS,
            &[
                set(
                    at!(SELECTED),
                    Field::Background,
                    home!(fill, tab.tab(SELECTED)),
                ),
                set(
                    at!(SELECTED),
                    Field::TextColor,
                    home!(color, tab.label(SELECTED)),
                ),
                set(
                    at!(SELECTED),
                    Field::IconColor,
                    home!(color, tab.label(SELECTED)),
                ),
            ],
        ],
        &[TAB_STROKE, TAB_IDLE_STROKE, STATED],
    ),
    // iced_aw: sidebar.container(NONE)'s stroke and sidebar.item(HOVER).
    // the macOS presets state a hovered source-list row's fill transparent (`sidebar.item(HOVER)`),
    // and its items' label as the source-list row's measured rest label over the material's
    // Reduce Transparency rendering (os-probe run 45231684209, `*.normal.source_list_row`), where
    // 0.7.2 read labelColor; the panel's fill is 0.7.2's, that rendering.
    aw(
        "aw::sidebar",
        MACOS,
        LineAlign::Inside,
        States::STATED,
        (false, false),
        &[INSIDE],
    )
    .stating(
        &[
            &SIDEBAR_PANEL,
            &[
                set(
                    at!(NONE),
                    Field::TextColor,
                    home!(color, sidebar.item_label(NONE)),
                ),
                set(
                    at!(NONE),
                    Field::IconColor,
                    home!(color, sidebar.item_label(NONE)),
                ),
                set(
                    at!(HOVER),
                    Field::Background,
                    home!(fill, sidebar.item(HOVER)),
                ),
                set(
                    at!(HOVER),
                    Field::TextColor,
                    home!(color, sidebar.item_label(NONE)),
                ),
                set(
                    at!(HOVER),
                    Field::IconColor,
                    home!(color, sidebar.item_label(NONE)),
                ),
            ],
        ],
        &[INSIDE, STATED],
    ),
    // iced_aw: list.container(NONE)'s stroke, list.highlight(HOVER) and
    // list.item_label(HOVER).
    inside("aw::selection_list", &["windows-11"]),
    // the macOS presets state a hovered row's highlight transparent (`list.highlight(HOVER)`); its
    // label they leave to iced_aw.
    aw(
        "aw::selection_list",
        MACOS,
        LineAlign::Inside,
        hovered(false, true, false),
        (false, false),
        &[INSIDE, LIST_HOVER],
    )
    .stating(
        &[&[set(
            at!(HOVER),
            Field::Background,
            home!(fill, list.highlight(HOVER)),
        )]],
        &[INSIDE, LIST_HOVER, STATED],
    ),
    // link.label(HOVER): the macOS presets state none.
    Difference {
        leaf: "button_link",
        presets: MACOS,
        listed: Listed {
            states: States {
                hover_label: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[HOVER_LINK_LABEL],
    },
];

/// The leaves compared and the differences found, for one preset and scheme.
struct Report {
    label: String,
    preset: &'static str,
    compared: usize,
    /// The listed differences found differing: the entry's index, the style
    /// function and the entry's key of the preset.
    listed_seen: Vec<(usize, String, &'static str)>,
    failures: Vec<String>,
}

impl Report {
    fn new(preset: &'static str, is_dark: bool) -> Self {
        let scheme = if is_dark { "dark" } else { "light" };
        Self {
            label: format!("{preset}/{scheme}"),
            preset,
            compared: 0,
            listed_seen: Vec::new(),
            failures: Vec::new(),
        }
    }

    /// Compare one leaf. Where an entry lists its style function on this
    /// preset, the 0.8.0 value must be the one `expected` gives for the
    /// entry in every status, equal to the 0.7.2 side's or not; elsewhere a
    /// leaf that differs fails.
    fn leaf<T: PartialEq + Debug>(
        &mut self,
        leaf: &str,
        old: T,
        new: T,
        expected: impl Fn(Listed) -> T,
    ) {
        self.compare(leaf, old, new, |d| {
            if d.stated.is_empty() {
                Ok(expected(d.listed))
            } else {
                Err("it names stated fields, which this value has none of".to_owned())
            }
        });
    }

    /// [`Self::leaf`] of a style a [`Stated`] can set fields of, in a
    /// status of state `at` (`None` for a status the widget never asks
    /// for, in which no field is set), compared in the form `form` gives:
    /// the expected value is `expected`'s with each field the entry states
    /// at `at` set to its home's value on `theme`.
    #[cfg(feature = "widgets")]
    fn stated_leaf<S: Fields, T: PartialEq + Debug>(
        &mut self,
        leaf: &str,
        at: Option<WidgetState>,
        (old, new): (T, T),
        expected: impl Fn(Listed) -> S,
        form: impl Fn(S) -> T,
        theme: &native_theme::next::ResolvedTheme,
    ) {
        self.compare(leaf, old, new, |d| {
            let mut style = expected(d.listed);
            for field in Field::ALL {
                let here = |s: &&Stated| Some(s.at) == at && s.field == field;
                for s in d.stated.iter().copied().flatten().filter(here) {
                    let value = (s.home.value)(theme)
                        .ok_or_else(|| format!("{:?} names {:?}, unstated", s.field, s.home))?;
                    style.set(field, value)?;
                }
            }
            Ok(form(style))
        });
    }

    /// Compare one leaf against the value `expected` gives for the entry
    /// that lists its style function on this preset, if any.
    fn compare<T: PartialEq + Debug>(
        &mut self,
        leaf: &str,
        old: T,
        new: T,
        expected: impl Fn(&Difference) -> Result<T, String>,
    ) {
        self.compared += 1;
        let function = leaf.split('(').next().unwrap_or(leaf);
        let listed = DIFFERENCES.iter().enumerate().find_map(|(i, d)| {
            let key = d
                .presets
                .iter()
                .find(|key| **key == self.preset || **key == self.label)?;
            (d.leaf == function).then_some((i, d, *key))
        });
        let Some((i, entry, key)) = listed else {
            if old != new {
                let diff = debug_diff(&format!("{old:?}"), &format!("{new:?}"));
                self.failures
                    .push(format!("{}: {leaf}: {diff}", self.label));
            }
            return;
        };
        let listed = entry.listed;
        let expected = match expected(entry) {
            Ok(expected) => expected,
            Err(why) => {
                self.failures
                    .push(format!("{}: {leaf}: listed, but {why}", self.label));
                return;
            }
        };
        if expected == new {
            if old != new {
                self.listed_seen.push((i, function.to_owned(), key));
            }
        } else {
            let diff = debug_diff(&format!("{expected:?}"), &format!("{new:?}"));
            self.failures.push(format!(
                "{}: {leaf}: listed as {listed:?}, stating {:?}, but not the expected value \
                 (expected -> 0.8.0): {diff}",
                self.label, entry.stated
            ));
        }
    }

    /// The 0.8.0 side's value, or a failure where it is unmapped.
    fn mapped<T>(&mut self, leaf: &str, new: super::Mapped<T>) -> Option<T> {
        match new {
            Ok(v) => Some(v),
            Err(super::Unmapped(home)) => {
                self.failures
                    .push(format!("{}: {leaf}: 0.8.0 lacks {home}", self.label));
                None
            }
        }
    }
}

/// Where two `Debug` forms differ: each differing field, as the path of
/// the struct names and field names that lead to it, the first value and
/// the second.
fn debug_diff(old: &str, new: &str) -> String {
    let split = |s: &str| -> Vec<String> {
        let mut path: Vec<String> = Vec::new();
        let mut out = Vec::new();
        let mut word = String::new();
        for ch in s.chars() {
            match ch {
                '{' | '(' => {
                    path.push(word.trim().to_owned());
                    word.clear();
                }
                '}' | ')' | ',' => {
                    if !word.trim().is_empty() {
                        out.push(format!("{}.{}", path.join("."), word.trim()));
                    }
                    word.clear();
                    if ch != ',' {
                        path.pop();
                    }
                }
                _ => word.push(ch),
            }
        }
        if !word.trim().is_empty() {
            out.push(word.trim().to_owned());
        }
        out
    };
    let (o, n) = (split(old), split(new));
    if o.len() != n.len() {
        return format!("{old}, {new}");
    }
    o.iter()
        .zip(&n)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| format!("{a} -> {}", b.rsplit(':').next().unwrap_or(b).trim()))
        .collect::<Vec<_>>()
        .join("; ")
}

/// The two resolved themes of `preset` for `is_dark`, and 0.7.2's name and
/// mode, which its theme holds beside the variants. The 0.7.2 side is
/// resolved at the font DPI 0.7.2 resolves the platform's theme at, 72
/// "where a point is a pixel" for the Apple presets and 96 for the others
/// (`ResolutionContext::for_tests`), as native-theme's model parity resolves
/// it (native-theme/src/next/parity.rs, `MACOS_RESOLUTION_CONTEXT`,
/// `IOS_RESOLUTION_CONTEXT`): a text size is compared in the pixels both
/// models state it in.
pub(crate) fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(
    ResolvedTheme,
    String,
    ColorMode,
    native_theme::next::ResolvedTheme,
)> {
    let (mode, ctx) = if is_dark {
        (ColorMode::Dark, ThemeContext::dark())
    } else {
        (ColorMode::Light, ThemeContext::light())
    };
    let apple = matches!(
        preset,
        Preset::MacosMonterey | Preset::MacosTahoe | Preset::Ios13
    );
    let context = native_theme::ResolutionContext {
        font_dpi: if apple { 72.0 } else { 96.0 },
        ..native_theme::ResolutionContext::for_tests()
    };
    let theme = crate::Theme::preset(key)?;
    let name = theme.name.to_string();
    let old = theme.into_variant(mode)?.into_resolved(&context)?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, name, mode, new))
}

/// The iced theme's leaves.
fn compare_theme(report: &mut Report, old: &Theme, new: &super::Mapped<Theme>) {
    let Some(new) = report.mapped("to_theme", new.clone()) else {
        return;
    };
    let unlisted = |_| old.palette();
    report.leaf("to_theme.palette", old.palette(), new.palette(), unlisted);
    report.leaf(
        "to_theme.extended_palette",
        *old.extended_palette(),
        *new.extended_palette(),
        |_| *old.extended_palette(),
    );
    report.leaf("to_theme.name", old.to_string(), new.to_string(), |_| {
        old.to_string()
    });
}

/// Compare a style function's two outputs in each of `$statuses`, each side
/// called with its own iced theme; `$aligned` gives the 0.7.2 side's
/// function with its strokes laid at an alignment, the expected value of a
/// listed difference. A function of no status is called through `surface`.
#[cfg(feature = "widgets")]
macro_rules! style {
    ($report:expr, $themes:expr, $name:literal, $old:expr, $new:expr, $aligned:expr, $statuses:expr) => {{
        let aligned = $aligned;
        listed_style!(
            $report,
            $themes,
            $name,
            $old,
            $new,
            move |listed: Listed| aligned(listed.align),
            $statuses
        )
    }};
}

/// [`style`] of a style function that has hover options the 0.8.0 theme
/// can leave to iced: `$listed` gives the 0.7.2 side's function as the
/// whole [`Listed`] says.
#[cfg(feature = "widgets")]
macro_rules! listed_style {
    ($report:expr, $themes:expr, $name:literal, $old:expr, $new:expr, $listed:expr, $statuses:expr) => {{
        let (old_theme, new_theme): (&Theme, &Theme) = $themes;
        let old = $old;
        let listed = $listed;
        if let Some(new) = $report.mapped($name, $new) {
            for status in $statuses {
                $report.leaf(
                    &format!("{}({status:?})", $name),
                    old(old_theme, status),
                    new(new_theme, status),
                    |l: Listed| listed(l)(old_theme, status),
                );
            }
        }
    }};
}

/// [`listed_style`] of a style whose fields a [`Stated`] can set, each
/// status at its state with `$flags` (a checked or selected style
/// function's flag), the homes read on `$resolved`.
#[cfg(feature = "widgets")]
macro_rules! stated_style {
    ($report:expr, $themes:expr, $resolved:expr, $name:literal, $old:expr, $new:expr, $listed:expr, $statuses:expr, $flags:expr) => {{
        let (old_theme, new_theme): (&Theme, &Theme) = $themes;
        let old = $old;
        let listed = $listed;
        if let Some(new) = $report.mapped($name, $new) {
            for status in $statuses {
                $report.stated_leaf(
                    &format!("{}({status:?})", $name),
                    Some(status.at() | $flags),
                    (old(old_theme, status), new(new_theme, status)),
                    |l: Listed| listed(l)(old_theme, status),
                    |style| style,
                    $resolved,
                );
            }
        }
    }};
}

/// [`stated_style`] of a style function of no status, its fields set at
/// the state `$at`.
#[cfg(feature = "widgets")]
macro_rules! stated_surface {
    ($report:expr, $themes:expr, $resolved:expr, $name:literal, $old:expr, $new:expr, $listed:expr, $at:expr) => {{
        let (old_theme, new_theme): (&Theme, &Theme) = $themes;
        let old = $old;
        let listed = $listed;
        if let Some(new) = $report.mapped($name, $new) {
            $report.stated_leaf(
                concat!($name, "(())"),
                Some($at),
                (old(old_theme), new(new_theme)),
                |l: Listed| listed(l)(old_theme),
                |style| style,
                $resolved,
            );
        }
    }};
}

/// A style function of no status as one of a single status.
#[cfg(feature = "widgets")]
fn surface<T>(f: impl Fn(&Theme) -> T + Clone) -> impl Fn(&Theme, ()) -> T + Clone {
    move |theme, ()| f(theme)
}

/// A widget's status as the state the 0.8.0 theme is queried with.
#[cfg(feature = "widgets")]
trait AtState {
    fn at(&self) -> WidgetState;
}

/// `flag` where `on`, else no state.
#[cfg(feature = "widgets")]
fn flag_if(on: bool, flag: WidgetState) -> WidgetState {
    if on { flag } else { WidgetState::NONE }
}

#[cfg(feature = "widgets")]
impl AtState for iced_widget::button::Status {
    fn at(&self) -> WidgetState {
        match self {
            Self::Active => WidgetState::NONE,
            Self::Hovered => WidgetState::HOVER,
            Self::Pressed => WidgetState::PRESSED,
            Self::Disabled => WidgetState::DISABLED,
        }
    }
}

#[cfg(feature = "widgets")]
impl AtState for iced_widget::checkbox::Status {
    fn at(&self) -> WidgetState {
        let checked = |on| flag_if(on, WidgetState::CHECKED);
        match *self {
            Self::Active { is_checked } => checked(is_checked),
            Self::Hovered { is_checked } => WidgetState::HOVER | checked(is_checked),
            Self::Disabled { is_checked } => WidgetState::DISABLED | checked(is_checked),
        }
    }
}

#[cfg(feature = "widgets")]
impl AtState for iced_widget::radio::Status {
    fn at(&self) -> WidgetState {
        let checked = |on| flag_if(on, WidgetState::CHECKED);
        match *self {
            Self::Active { is_selected } => checked(is_selected),
            Self::Hovered { is_selected } => WidgetState::HOVER | checked(is_selected),
        }
    }
}

#[cfg(feature = "widgets")]
impl AtState for iced_widget::toggler::Status {
    fn at(&self) -> WidgetState {
        let checked = |on| flag_if(on, WidgetState::CHECKED);
        match *self {
            Self::Active { is_toggled } => checked(is_toggled),
            Self::Hovered { is_toggled } => WidgetState::HOVER | checked(is_toggled),
            Self::Disabled { is_toggled } => WidgetState::DISABLED | checked(is_toggled),
        }
    }
}

#[cfg(feature = "widgets")]
impl AtState for iced_widget::slider::Status {
    fn at(&self) -> WidgetState {
        match self {
            Self::Active => WidgetState::NONE,
            Self::Hovered => WidgetState::HOVER,
            Self::Dragged => WidgetState::PRESSED,
        }
    }
}

/// A style whose fields a [`Stated`] sets: each field it has, set as the
/// 0.8.0 builder hands iced its home's value; a field it has not fails.
#[cfg(feature = "widgets")]
trait Fields {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String>;
}

/// The colour `value` is, for `field`.
#[cfg(feature = "widgets")]
fn color_of(field: Field, value: &Value) -> Result<iced_core::Color, String> {
    use crate::styles::{composite_over, faded_as_one};
    match *value {
        Value::Color(c) => Ok(c),
        Value::Faded {
            color,
            under,
            ground,
            opacity,
        } => {
            let under = under.map_or(ground, |fill| composite_over(fill, ground));
            Ok(opacity.map_or(color, |opacity| faded_as_one(ground, under, color, opacity)))
        }
        Value::Width(_) | Value::Stroke { .. } => {
            Err(format!("{field:?} takes a colour, not {value:?}"))
        }
    }
}

/// The width `value` is, for `field`.
#[cfg(feature = "widgets")]
fn width_of(field: Field, value: &Value) -> Result<f32, String> {
    match value {
        Value::Width(w) => Ok(*w),
        _ => Err(format!("{field:?} takes a width, not {value:?}")),
    }
}

/// The colour iced is handed for the stroke of the part `value` is, laid
/// over `fill`, as the builder reads and lays it
/// ([`super::borders::line_or_none`], [`crate::styles::Line::over`]), and
/// faded where the value says.
#[cfg(feature = "widgets")]
fn line_over(
    field: Field,
    value: &Value,
    fill: iced_core::Color,
) -> Result<iced_core::Color, String> {
    match value {
        Value::Stroke { part, faded } => super::borders::line_or_none(part, "the stated stroke")
            .map(|line| {
                let color = line.over(fill);
                faded.map_or(color, |opacity| crate::styles::faded(color, opacity))
            })
            .map_err(|u| format!("{field:?}: {u:?}")),
        _ => Err(format!("{field:?} takes a stroke, not {value:?}")),
    }
}

/// A field `style` has not.
#[cfg(feature = "widgets")]
fn no_field(field: Field, style: &str) -> Result<(), String> {
    Err(format!("{style} has no field {field:?}"))
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::button::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => {
                self.background = Some(Background::Color(color_of(field, &value)?))
            }
            Field::TextColor => self.text_color = color_of(field, &value)?,
            Field::BorderWidth => self.border.width = width_of(field, &value)?,
            Field::BorderColor => {
                let fill = crate::styles::painted(self.background);
                self.border.color = line_over(field, &value, fill)?;
            }
            Field::InnerRadius => {
                let inner = width_of(field, &value)?;
                let r = &mut self.border.radius;
                for corner in [
                    &mut r.top_left,
                    &mut r.top_right,
                    &mut r.bottom_right,
                    &mut r.bottom_left,
                ] {
                    if *corner > 0.0 {
                        *corner = inner;
                    }
                }
            }
            Field::HandleBackground
            | Field::IconColor
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel
            | Field::RailFill => return no_field(field, "button::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::checkbox::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => self.background = Background::Color(color_of(field, &value)?),
            Field::TextColor => self.text_color = Some(color_of(field, &value)?),
            Field::IconColor => self.icon_color = color_of(field, &value)?,
            Field::BorderWidth => self.border.width = width_of(field, &value)?,
            // a disabled box's outline, faded with the box, as it is
            Field::BorderColor if matches!(value, Value::Faded { .. }) => {
                self.border.color = color_of(field, &value)?;
            }
            Field::BorderColor => {
                let fill = crate::styles::painted(Some(self.background));
                self.border.color = line_over(field, &value, fill)?;
            }
            Field::HandleBackground
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel
            | Field::RailFill => {
                return no_field(field, "checkbox::Style");
            }
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::radio::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => self.background = Background::Color(color_of(field, &value)?),
            Field::BorderWidth => self.border_width = width_of(field, &value)?,
            Field::BorderColor => {
                let fill = crate::styles::painted(Some(self.background));
                self.border_color = line_over(field, &value, fill)?;
            }
            Field::IconColor => self.dot_color = color_of(field, &value)?,
            Field::HandleBackground
            | Field::TextColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel
            | Field::RailFill => {
                return no_field(field, "radio::Style");
            }
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::toggler::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => self.background = Background::Color(color_of(field, &value)?),
            Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::BorderWidth
            | Field::BorderColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel
            | Field::RailFill => return no_field(field, "toggler::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::slider::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::HandleBackground => {
                self.handle.background = Background::Color(color_of(field, &value)?);
            }
            Field::RailFill => {
                self.rail.backgrounds.0 = Background::Color(color_of(field, &value)?);
            }
            Field::Background
            | Field::TextColor
            | Field::IconColor
            | Field::BorderWidth
            | Field::BorderColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel => return no_field(field, "slider::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::pick_list::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => self.background = Background::Color(color_of(field, &value)?),
            Field::BorderWidth => self.border.width = width_of(field, &value)?,
            Field::BorderColor => {
                let fill = crate::styles::painted(Some(self.background));
                self.border.color = line_over(field, &value, fill)?;
            }
            Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel
            | Field::RailFill => return no_field(field, "pick_list::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::container::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::TextColor => self.text_color = Some(color_of(field, &value)?),
            Field::BorderWidth => self.border.width = width_of(field, &value)?,
            Field::Radius => self.border.radius = width_of(field, &value)?.into(),
            Field::BorderColor => {
                let fill = crate::styles::painted(self.background);
                self.border.color = line_over(field, &value, fill)?;
            }
            Field::Separator => match value {
                Value::Stroke { part, .. } => {
                    let line = super::borders::line_or_none(&part, "the stated stroke")
                        .map_err(|u| format!("{field:?}: {u:?}"))?;
                    self.background = line.drawn.then_some(Background::Color(line.color));
                }
                // the separator part's own fill
                Value::Color(_) | Value::Faded { .. } => {
                    self.background = Some(Background::Color(color_of(field, &value)?));
                }
                Value::Width(_) => {
                    return Err(format!(
                        "{field:?} takes a stroke or a colour, not {value:?}"
                    ));
                }
            },
            Field::Background => {
                self.background = Some(Background::Color(color_of(field, &value)?));
            }
            Field::HandleBackground
            | Field::IconColor
            | Field::InnerRadius
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Panel
            | Field::RailFill => return no_field(field, "container::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::pane_grid::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        match field {
            Field::PickedSplitColor => self.picked_split.color = color_of(field, &value)?,
            Field::HoveredSplitColor => self.hovered_split.color = color_of(field, &value)?,
            Field::Background
            | Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::BorderWidth
            | Field::BorderColor
            | Field::InnerRadius
            | Field::Separator
            | Field::Radius
            | Field::Panel
            | Field::RailFill => return no_field(field, "pane_grid::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl AtState for iced_widget::text_input::Status {
    fn at(&self) -> WidgetState {
        let hovered = |on| flag_if(on, WidgetState::HOVER);
        match *self {
            Self::Active => WidgetState::NONE,
            Self::Hovered => WidgetState::HOVER,
            Self::Focused { is_hovered } => WidgetState::FOCUSED | hovered(is_hovered),
            Self::Disabled => WidgetState::DISABLED,
        }
    }
}

#[cfg(feature = "widgets")]
impl AtState for iced_widget::text_editor::Status {
    fn at(&self) -> WidgetState {
        let hovered = |on| flag_if(on, WidgetState::HOVER);
        match *self {
            Self::Active => WidgetState::NONE,
            Self::Hovered => WidgetState::HOVER,
            Self::Focused { is_hovered } => WidgetState::FOCUSED | hovered(is_hovered),
            Self::Disabled => WidgetState::DISABLED,
        }
    }
}

/// A text field's style: its border's width, and its colour laid over the
/// field's fill as the 0.8.0 builder lays it.
#[cfg(feature = "widgets")]
macro_rules! field_fields {
    ($style:ty, $name:literal) => {
        impl Fields for $style {
            fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
                match field {
                    Field::BorderWidth => self.border.width = width_of(field, &value)?,
                    Field::BorderColor => {
                        self.border.color = match value {
                            Value::Color(color) => color,
                            _ => line_over(
                                field,
                                &value,
                                crate::styles::painted(Some(self.background)),
                            )?,
                        };
                    }
                    Field::Background
                    | Field::HandleBackground
                    | Field::TextColor
                    | Field::IconColor
                    | Field::InnerRadius
                    | Field::Separator
                    | Field::PickedSplitColor
                    | Field::HoveredSplitColor
                    | Field::Radius
                    | Field::Panel
                    | Field::RailFill => return no_field(field, $name),
                }
                Ok(())
            }
        }
    };
}

#[cfg(feature = "widgets")]
field_fields!(iced_widget::text_input::Style, "text_input::Style");
#[cfg(feature = "widgets")]
field_fields!(iced_widget::text_editor::Style, "text_editor::Style");

#[cfg(feature = "widgets")]
impl AtState for iced_widget::pick_list::Status {
    fn at(&self) -> WidgetState {
        let hovered = |on| flag_if(on, WidgetState::HOVER);
        match *self {
            Self::Active => WidgetState::NONE,
            Self::Hovered => WidgetState::HOVER,
            Self::Opened { is_hovered } => WidgetState::OPEN | hovered(is_hovered),
        }
    }
}

#[cfg(feature = "iced_aw")]
impl Fields for iced_aw::style::tab_bar::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => {
                self.tab_label_background = Background::Color(color_of(field, &value)?);
            }
            Field::TextColor => self.text_color = color_of(field, &value)?,
            Field::IconColor => self.icon_color = color_of(field, &value)?,
            Field::HandleBackground
            | Field::BorderWidth
            | Field::BorderColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel
            | Field::RailFill => return no_field(field, "tab_bar::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "iced_aw")]
impl Fields for iced_aw::style::card::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        match field {
            Field::BorderWidth => self.border_width = width_of(field, &value)?,
            Field::BorderColor => {
                let fill = crate::styles::painted(Some(self.background));
                self.border_color = line_over(field, &value, fill)?;
            }
            Field::Radius => self.border_radius = width_of(field, &value)?,
            Field::Background => {
                self.background = iced_core::Background::Color(color_of(field, &value)?);
            }
            Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Panel
            | Field::RailFill => return no_field(field, "card::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "iced_aw")]
impl Fields for iced_aw::style::menu_bar::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        match field {
            // the menu's popup frame
            Field::Radius => self.menu_border.radius = width_of(field, &value)?.into(),
            Field::BorderWidth
            | Field::BorderColor
            | Field::Background
            | Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Panel
            | Field::RailFill => return no_field(field, "menu_bar::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "iced_aw")]
impl Fields for iced_aw::style::sidebar::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => {
                self.tab_label_background = Background::Color(color_of(field, &value)?);
            }
            Field::Panel => self.background = Some(Background::Color(color_of(field, &value)?)),
            Field::TextColor => self.text_color = color_of(field, &value)?,
            Field::IconColor => self.icon_color = color_of(field, &value)?,
            Field::BorderColor => {
                let fill = crate::styles::painted(self.background);
                self.border_color = Some(line_over(field, &value, fill)?);
            }
            Field::HandleBackground
            | Field::BorderWidth
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::RailFill => return no_field(field, "sidebar::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "iced_aw")]
impl Fields for iced_aw::style::selection_list::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::Background => self.background = Background::Color(color_of(field, &value)?),
            Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::BorderWidth
            | Field::BorderColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor
            | Field::Radius
            | Field::Panel
            | Field::RailFill => return no_field(field, "selection_list::Style"),
        }
        Ok(())
    }
}

/// An `iced_aw` status as the state the 0.8.0 theme is queried with for a
/// selection list: the row under the pointer `HOVER`,
/// every other status the rest (no field is stated for one).
#[cfg(feature = "iced_aw")]
fn hover_at(status: iced_aw::style::Status) -> WidgetState {
    match status {
        iced_aw::style::Status::Hovered => WidgetState::HOVER,
        _ => WidgetState::NONE,
    }
}

/// A sidebar's status as the state of its item, as `iced_aw` spells an
/// item's states as a tab's (`crate::styles::aw::sidebar`): the item under
/// the pointer, the selected item (`Active`) and an item not selected
/// (`Disabled`), the three iced_aw 0.14.1 asks for
/// (`widget/sidebar/sidebar.rs`); each status it never asks for, whose item
/// the style leaves to `iced_aw` and whose panel it paints, as the flag it
/// names, `Selected` as the selected item's.
#[cfg(feature = "iced_aw")]
fn sidebar_at(status: iced_aw::style::Status) -> WidgetState {
    use iced_aw::style::Status;
    match status {
        Status::Hovered => WidgetState::HOVER,
        Status::Active | Status::Selected => WidgetState::SELECTED,
        Status::Disabled => WidgetState::NONE,
        Status::Pressed => WidgetState::PRESSED,
        Status::Focused => WidgetState::FOCUSED,
    }
}

/// A tab bar's status as the state the 0.8.0 theme is queried with, as
/// `iced_aw` spells a tab's states (`crate::styles::aw::tab_bar`): the tab
/// under the pointer, the selected tab (`Active`) and a tab not selected
/// (`Disabled`); `None` for a status a tab bar never asks for.
#[cfg(feature = "iced_aw")]
fn tab_at(status: iced_aw::style::Status) -> Option<WidgetState> {
    use iced_aw::style::Status;
    match status {
        Status::Hovered => Some(WidgetState::HOVER),
        Status::Active => Some(WidgetState::SELECTED),
        Status::Disabled => Some(WidgetState::NONE),
        Status::Pressed | Status::Focused | Status::Selected => None,
    }
}

/// Compare a style function that has no stroke the theme aligns: an
/// unlisted difference fails.
#[cfg(feature = "widgets")]
macro_rules! plain {
    ($report:expr, $themes:expr, $name:literal, $old:expr, $new:expr, $statuses:expr) => {{
        let old = $old;
        let unaligned = old.clone();
        style!(
            $report,
            $themes,
            $name,
            old,
            $new,
            move |_| unaligned.clone(),
            $statuses
        )
    }};
}

/// The style functions ported so far.
#[cfg(feature = "widgets")]
fn compare_styles(
    report: &mut Report,
    themes: (&Theme, &Theme),
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    use crate::palette::to_color;
    use crate::styles::{
        self as o, ButtonSource, CheckFills, CheckboxSource, ExpanderSource, InputSource,
        LinkSource, MenuSource, Outline, PickListSource, ScrollbarSource, SegmentPosition,
        SegmentedSource, SliderSource, SwitchSource, ToolbarButtonSource,
    };
    use iced_widget::button::Status::{Active, Disabled, Hovered, Pressed};

    use super::styles as n;

    let button = [Active, Hovered, Pressed, Disabled];
    let b = ButtonSource::of(old);
    let checked_fill = super::state_fill_of(&new.button.body(WidgetState::CHECKED));
    let checked_label = super::state_text_color_of(&new.button.label(WidgetState::CHECKED));
    let at = move |l: Listed| {
        let s = b.listed(l);
        match l.added {
            Added::CheckedButton => ButtonSource {
                checked_background: s
                    .checked_background
                    .or(checked_fill.filter(|_| !l.states.checked)),
                checked_label: s.checked_label.or(checked_label),
                checked_primary: true,
                ..s
            },
            Added::Nothing | Added::ScrollerOutline { .. } | Added::ScrollerFills { .. } => s,
        }
    };
    let none = WidgetState::NONE;
    stated_style!(
        report,
        themes,
        new,
        "button",
        o::button(old),
        n::button(new),
        |l| o::button_from(&at(l)),
        button,
        none
    );
    for checked in [false, true] {
        let flags = flag_if(checked, WidgetState::CHECKED);
        stated_style!(
            report,
            themes,
            new,
            "button_toggle",
            o::button_toggle(old, checked),
            n::button_toggle(new, checked),
            |l| o::button_toggle_from(&at(l), checked),
            button,
            flags
        );
        let t = ToolbarButtonSource::of(old);
        stated_style!(
            report,
            themes,
            new,
            "toolbar_button",
            o::toolbar_button(old, checked),
            n::toolbar_button(new, checked),
            |l: Listed| o::toolbar_button_from(&at(l), &t.listed(l), checked),
            button,
            flags
        );
    }
    stated_style!(
        report,
        themes,
        new,
        "button_primary",
        o::button_primary(old),
        n::button_primary(new),
        |l| o::button_primary_from(&at(l)),
        button,
        none
    );
    stated_style!(
        report,
        themes,
        new,
        "button_default",
        o::button_default(old),
        n::button_default(new),
        |l| o::button_default_from(&at(l)),
        button,
        none
    );
    let d = &old.defaults;
    for (name, fill, label, class, new_fn) in [
        (
            "button_danger",
            d.danger_color,
            d.danger_text_color,
            iced_widget::button::danger as fn(&Theme, _) -> _,
            n::button_danger(new).map(|f| Box::new(f) as Box<dyn Fn(&Theme, _) -> _>),
        ),
        (
            "button_success",
            d.success_color,
            d.success_text_color,
            iced_widget::button::success,
            n::button_success(new).map(|f| Box::new(f) as Box<dyn Fn(&Theme, _) -> _>),
        ),
        (
            "button_warning",
            d.warning_color,
            d.warning_text_color,
            iced_widget::button::warning,
            n::button_warning(new).map(|f| Box::new(f) as Box<dyn Fn(&Theme, _) -> _>),
        ),
    ] {
        let (old_theme, new_theme) = themes;
        let class_old = |l: Option<Listed>| {
            let source = l.map_or(b, |l| b.listed(l));
            o::class_button(&source, to_color(fill), to_color(label), class)
        };
        let old_fn = class_old(None);
        if let Some(new_fn) = report.mapped(name, new_fn) {
            for status in button {
                report.stated_leaf(
                    &format!("{name}({status:?})"),
                    Some(status.at()),
                    (old_fn(old_theme, status), new_fn(new_theme, status)),
                    |l: Listed| class_old(Some(l))(old_theme, status),
                    |style| style,
                    new,
                );
            }
        }
    }
    let link = LinkSource::of(old);
    listed_style!(
        report,
        themes,
        "button_link",
        o::button_link(old),
        n::button_link(new),
        |l: Listed| o::button_link_from(&link.listed(l)),
        button
    );

    {
        use iced_widget::text_input::Status;
        let field = [
            Status::Active,
            Status::Hovered,
            Status::Focused { is_hovered: false },
            Status::Focused { is_hovered: true },
            Status::Disabled,
        ];
        let i = InputSource::of(&old.input, &old.input.border, &old.defaults);
        stated_style!(
            report,
            themes,
            new,
            "text_input",
            o::text_input(old),
            n::text_input(new),
            |l: Listed| o::text_input_from(&i.listed(l)),
            field,
            WidgetState::NONE
        );
    }
    {
        use iced_widget::text_editor::Status;
        let field = [
            Status::Active,
            Status::Hovered,
            Status::Focused { is_hovered: false },
            Status::Focused { is_hovered: true },
            Status::Disabled,
        ];
        let i = InputSource::of(&old.input, &old.text_area.border, &old.defaults);
        stated_style!(
            report,
            themes,
            new,
            "text_editor",
            o::text_editor(old),
            n::text_editor(new),
            |l: Listed| o::text_editor_from(&i.listed(l)),
            field,
            WidgetState::NONE
        );
    }

    {
        use iced_widget::checkbox::Status;
        let boxes = [false, true].into_iter().flat_map(|is_checked| {
            [
                Status::Active { is_checked },
                Status::Hovered { is_checked },
                Status::Disabled { is_checked },
            ]
        });
        let c = CheckboxSource::of(old);
        let window = to_color(old.defaults.background_color);
        stated_style!(
            report,
            themes,
            new,
            "checkbox",
            o::checkbox(old),
            n::checkbox(new),
            |l: Listed| o::checkbox_from(&c.listed(l), window),
            boxes,
            none
        );
    }
    {
        use iced_widget::radio::Status;
        let circles = [false, true].into_iter().flat_map(|is_selected| {
            [
                Status::Active { is_selected },
                Status::Hovered { is_selected },
            ]
        });
        let c = &old.checkbox;
        let fills = CheckFills::of(c);
        stated_style!(
            report,
            themes,
            new,
            "radio",
            o::radio(old),
            n::radio(new),
            |l: Listed| o::radio_from(
                fills.listed(l),
                to_color(c.indicator_color),
                to_color(c.font.color),
                c.border.line_width,
            ),
            circles,
            none
        );
    }
    {
        use iced_widget::toggler::Status;
        let switches = [false, true].into_iter().flat_map(|is_toggled| {
            [
                Status::Active { is_toggled },
                Status::Hovered { is_toggled },
                Status::Disabled { is_toggled },
            ]
        });
        let s = SwitchSource::of(old);
        let window = to_color(old.defaults.background_color);
        stated_style!(
            report,
            themes,
            new,
            "toggler",
            o::toggler(old),
            n::toggler(new),
            |l: Listed| o::toggler_from(&s.listed(l), window),
            switches,
            none
        );
    }
    {
        use iced_widget::pick_list::Status;
        let fields = [
            Status::Active,
            Status::Hovered,
            Status::Opened { is_hovered: false },
            Status::Opened { is_hovered: true },
        ];
        let c = PickListSource::of(old);
        stated_style!(
            report,
            themes,
            new,
            "pick_list",
            o::pick_list(old),
            n::pick_list(new),
            |l: Listed| o::pick_list_from(&c.listed(l)),
            fields,
            WidgetState::NONE
        );
    }

    let m = MenuSource::of(old);
    listed_style!(
        report,
        themes,
        "menu",
        surface(o::menu(old)),
        n::menu(new).map(surface),
        |l: Listed| surface(o::menu_from(&m.listed(l))),
        [()]
    );
    {
        use iced_widget::slider::Status;
        let s = SliderSource::of(old);
        stated_style!(
            report,
            themes,
            new,
            "slider",
            o::slider(old),
            n::slider(new),
            |l: Listed| o::slider_from(&s.listed(l)),
            [Status::Active, Status::Hovered, Status::Dragged],
            none
        );
    }
    {
        use iced_widget::scrollable::Status;
        let active = Status::Active {
            is_horizontal_scrollbar_disabled: false,
            is_vertical_scrollbar_disabled: false,
        };
        let hovered = |vertical, horizontal| Status::Hovered {
            is_horizontal_scrollbar_hovered: horizontal,
            is_vertical_scrollbar_hovered: vertical,
            is_horizontal_scrollbar_disabled: false,
            is_vertical_scrollbar_disabled: false,
        };
        let dragged = |vertical, horizontal| Status::Dragged {
            is_horizontal_scrollbar_dragged: horizontal,
            is_vertical_scrollbar_dragged: vertical,
            is_horizontal_scrollbar_disabled: false,
            is_vertical_scrollbar_disabled: false,
        };
        let s = ScrollbarSource::of(old);
        listed_style!(
            report,
            themes,
            "scrollable",
            o::scrollable(old),
            n::scrollable(new),
            |l: Listed| o::scrollable_from(&s.listed(l)),
            [
                active,
                hovered(false, false),
                hovered(true, false),
                hovered(false, true),
                dragged(true, false),
                dragged(false, true),
            ]
        );
        plain!(
            report,
            themes,
            "scrollbar",
            surface(move |_| o::scrollbar(old)),
            Ok(surface(move |_| n::scrollbar(new))),
            [()]
        );
    }
    {
        let sc = SegmentedSource::of(old);
        for disabled in [false, true] {
            let at = flag_if(disabled, WidgetState::DISABLED);
            stated_surface!(
                report,
                themes,
                new,
                "segmented_control",
                o::segmented_control(old, disabled),
                n::segmented_control(new, disabled),
                |l: Listed| o::segmented_control_from(
                    &sc.listed(Listed::aligned(l.align)),
                    disabled
                ),
                at
            );
            stated_surface!(
                report,
                themes,
                new,
                "segment_separator",
                o::segment_separator(old, disabled),
                n::segment_separator(new, disabled),
                |_: Listed| o::segment_separator(old, disabled),
                at
            );
        }
        for selected in [false, true] {
            for position in [
                SegmentPosition::First,
                SegmentPosition::Middle,
                SegmentPosition::Last,
                SegmentPosition::Only,
            ] {
                stated_style!(
                    report,
                    themes,
                    new,
                    "segment",
                    o::segment(old, selected, position),
                    n::segment(new, selected, position),
                    |l: Listed| o::segment_from(&sc.listed(l), selected, position),
                    button,
                    flag_if(selected, WidgetState::SELECTED)
                );
            }
        }
    }
    {
        let x = ExpanderSource::of(old);
        stated_style!(
            report,
            themes,
            new,
            "expander",
            o::expander(old),
            n::expander(new),
            |l: Listed| o::expander_from(&x.listed(l)),
            button,
            none
        );
        style!(
            report,
            themes,
            "expander_frame",
            surface(o::expander_frame(old)),
            n::expander_frame(new).map(surface),
            |a| surface(o::expander_frame_from(&x.listed(Listed::aligned(a)))),
            [()]
        );
    }

    let p = &old.progress_bar;
    style!(
        report,
        themes,
        "progress_bar",
        surface(o::progress_bar(old)),
        n::progress_bar(new).map(surface),
        |a| surface(o::progress_bar_from(
            to_color(p.track_color),
            to_color(p.fill_color),
            Outline::of(&p.border).aligned(a),
        )),
        [()]
    );
    plain!(
        report,
        themes,
        "rule",
        surface(o::rule(old)),
        Ok(surface(n::rule(new))),
        [()]
    );
    plain!(
        report,
        themes,
        "toolbar_rule",
        surface(o::toolbar_rule(old)),
        n::toolbar_rule(new).map(surface),
        [()]
    );
    stated_surface!(
        report,
        themes,
        new,
        "pane_grid",
        o::pane_grid(old),
        n::pane_grid(new),
        |_: Listed| o::pane_grid(old),
        WidgetState::NONE
    );
    let panel = |fill, label: Option<_>, border| {
        move |a: LineAlign| {
            surface(o::panel_from(
                to_color(fill),
                label.map(to_color),
                Some(Outline::of(border).aligned(a)),
            ))
        }
    };
    let t = &old.tooltip;
    style!(
        report,
        themes,
        "tooltip",
        surface(o::tooltip(old)),
        n::tooltip(new).map(surface),
        panel(t.background_color, Some(t.font.color), &t.border),
        [()]
    );
    let c = &old.card;
    stated_surface!(
        report,
        themes,
        new,
        "container_card",
        o::container_card(old),
        n::container_card(new),
        |l: Listed| o::panel_from(
            to_color(c.background_color),
            None,
            Some(Outline::of(&c.border).aligned(l.align)),
        ),
        WidgetState::NONE
    );
    let dl = &old.dialog;
    style!(
        report,
        themes,
        "dialog",
        surface(o::dialog(old)),
        n::dialog(new).map(surface),
        panel(dl.background_color, Some(dl.body_font.color), &dl.border),
        [()]
    );
    stated_surface!(
        report,
        themes,
        new,
        "toolbar",
        o::toolbar(old),
        n::toolbar(new),
        |_: Listed| o::toolbar(old),
        WidgetState::NONE
    );
    plain!(
        report,
        themes,
        "application",
        surface(o::application(old)),
        n::application(new).map(surface),
        [()]
    );
}

/// `iced_aw`'s styles in the form the parity test compares them: each
/// `Style` of `iced_aw` 0.14.1 derives `Debug` but no `PartialEq`, so it is
/// compared as the tuple of its fields, destructured whole, so that a field
/// `iced_aw` adds fails to compile here rather than go uncompared.
#[cfg(feature = "iced_aw")]
mod aw_leaves {
    use iced_aw::style::{card, menu_bar, selection_list, sidebar, tab_bar};
    use iced_core::border::Radius;
    use iced_core::{Background, Border, Color, Shadow};

    type Card = (
        Background,
        f32,
        f32,
        Color,
        Background,
        Color,
        Background,
        Color,
        Background,
        Color,
        Color,
    );

    pub(super) fn card(s: card::Style) -> Card {
        let card::Style {
            background,
            border_radius,
            border_width,
            border_color,
            head_background,
            head_text_color,
            body_background,
            body_text_color,
            foot_background,
            foot_text_color,
            close_color,
        } = s;
        (
            background,
            border_radius,
            border_width,
            border_color,
            head_background,
            head_text_color,
            body_background,
            body_text_color,
            foot_background,
            foot_text_color,
            close_color,
        )
    }

    type MenuBar = (
        Background,
        Border,
        Shadow,
        Background,
        Border,
        Shadow,
        Background,
        Border,
    );

    pub(super) fn menu(s: menu_bar::Style) -> MenuBar {
        let menu_bar::Style {
            bar_background,
            bar_border,
            bar_shadow,
            menu_background,
            menu_border,
            menu_shadow,
            path,
            path_border,
        } = s;
        (
            bar_background,
            bar_border,
            bar_shadow,
            menu_background,
            menu_border,
            menu_shadow,
            path,
            path_border,
        )
    }

    type TabBar = (
        Option<Background>,
        Option<Color>,
        f32,
        Radius,
        Background,
        Color,
        f32,
        Color,
        Option<Background>,
        Radius,
        Color,
    );

    pub(super) fn tab_bar(s: tab_bar::Style) -> TabBar {
        let tab_bar::Style {
            background,
            border_color,
            border_width,
            tab_border_radius,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color,
        } = s;
        (
            background,
            border_color,
            border_width,
            tab_border_radius,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color,
        )
    }

    type Sidebar = (
        Option<Background>,
        Option<Color>,
        f32,
        Background,
        Color,
        f32,
        Color,
        Option<Background>,
        Radius,
        Color,
    );

    pub(super) fn sidebar(s: sidebar::Style) -> Sidebar {
        let sidebar::Style {
            background,
            border_color,
            border_width,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color,
        } = s;
        (
            background,
            border_color,
            border_width,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color,
        )
    }

    pub(super) fn selection_list(s: selection_list::Style) -> (Color, Background, f32, Color) {
        let selection_list::Style {
            text_color,
            background,
            border_width,
            border_color,
        } = s;
        (text_color, background, border_width, border_color)
    }
}

/// The `iced_aw` style functions, each in every `Status` of `iced_aw`'s
/// shared enum.
#[cfg(feature = "iced_aw")]
fn compare_aw(
    report: &mut Report,
    themes: (&Theme, &Theme),
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    use iced_aw::style::Status;

    use crate::styles::aw::{self as o, CardSource, ListSource, MenuSource, TabBarSource};

    use super::styles::aw as n;

    let statuses = [
        Status::Active,
        Status::Hovered,
        Status::Pressed,
        Status::Disabled,
        Status::Focused,
        Status::Selected,
    ];

    let card = CardSource::of(old);
    if let Some(new_card) = report.mapped("aw::card", n::card(new)) {
        let (old_theme, new_theme) = themes;
        let old_card = o::card(old);
        for status in statuses {
            report.stated_leaf(
                &format!("aw::card({status:?})"),
                Some(WidgetState::NONE),
                (
                    aw_leaves::card(old_card(old_theme, status)),
                    aw_leaves::card(new_card(new_theme, status)),
                ),
                |l: Listed| o::card_from(&card.listed(l))(old_theme, status),
                aw_leaves::card,
                new,
            );
        }
    }
    let menu = MenuSource::of(old);
    if let Some(new_menu) = report.mapped("aw::menu", n::menu(new)) {
        let (old_theme, new_theme) = themes;
        let old_menu = o::menu(old);
        for status in statuses {
            report.stated_leaf(
                &format!("aw::menu({status:?})"),
                Some(WidgetState::NONE),
                (
                    aw_leaves::menu(old_menu(old_theme, status)),
                    aw_leaves::menu(new_menu(new_theme, status)),
                ),
                |l: Listed| o::menu_from(&menu.listed(l))(old_theme, status),
                aw_leaves::menu,
                new,
            );
        }
    }
    let tab = TabBarSource::of(old);
    if let Some(tab_bar) = report.mapped("aw::tab_bar", n::tab_bar(new)) {
        let (old_theme, new_theme) = themes;
        let old_tab_bar = o::tab_bar(old);
        for status in statuses {
            report.stated_leaf(
                &format!("aw::tab_bar({status:?})"),
                tab_at(status),
                (
                    aw_leaves::tab_bar(old_tab_bar(old_theme, status)),
                    aw_leaves::tab_bar(tab_bar(new_theme, status)),
                ),
                |l: Listed| o::tab_bar_from(&tab.listed(l))(old_theme, status),
                aw_leaves::tab_bar,
                new,
            );
        }
    }
    let sidebar = ListSource::of_sidebar(old);
    if let Some(new_sidebar) = report.mapped("aw::sidebar", n::sidebar(new)) {
        let (old_theme, new_theme) = themes;
        let old_sidebar = o::sidebar(old);
        for status in statuses {
            report.stated_leaf(
                &format!("aw::sidebar({status:?})"),
                Some(sidebar_at(status)),
                (
                    aw_leaves::sidebar(old_sidebar(old_theme, status)),
                    aw_leaves::sidebar(new_sidebar(new_theme, status)),
                ),
                |l: Listed| o::sidebar_from(&sidebar.listed(l))(old_theme, status),
                aw_leaves::sidebar,
                new,
            );
        }
    }
    let list = ListSource::of_list(old);
    if let Some(new_list) = report.mapped("aw::selection_list", n::selection_list(new)) {
        let (old_theme, new_theme) = themes;
        let old_list = o::selection_list(old);
        for status in statuses {
            report.stated_leaf(
                &format!("aw::selection_list({status:?})"),
                Some(hover_at(status)),
                (
                    aw_leaves::selection_list(old_list(old_theme, status)),
                    aw_leaves::selection_list(new_list(new_theme, status)),
                ),
                |l: Listed| o::selection_list_from(&list.listed(l))(old_theme, status),
                aw_leaves::selection_list,
                new,
            );
        }
    }
    plain!(
        report,
        themes,
        "aw::spinner",
        surface(o::spinner(old)),
        Ok(surface(n::spinner(new))),
        [()]
    );
}

/// Every leaf of `preset` for `is_dark`, compared.
fn compare_all(key: &'static str, preset: Preset, is_dark: bool) -> crate::Result<Report> {
    let (old, name, mode, new) = resolve(key, preset, is_dark)?;
    let mut report = Report::new(key, is_dark);
    let old_theme = crate::to_theme_in(&old, &name, mode);
    let new_theme = super::to_theme(&new);
    compare_theme(&mut report, &old_theme, &new_theme);
    #[cfg(feature = "widgets")]
    if let Ok(new_theme) = &new_theme {
        compare_styles(&mut report, (&old_theme, new_theme), &old, &new);
        #[cfg(feature = "iced_aw")]
        compare_aw(&mut report, (&old_theme, new_theme), &old, &new);
    }
    Ok(report)
}

#[test]
fn the_connector_hands_iced_the_same_from_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut listed_seen = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let report = compare_all(key, preset, is_dark)?;
            println!("{}: {} leaves compared", report.label, report.compared);
            failures.extend(report.failures);
            listed_seen.extend(report.listed_seen);
        }
    }
    for (i, d) in DIFFERENCES.iter().enumerate() {
        if !compared_here(d.leaf) {
            continue;
        }
        for &p in d.presets {
            if !listed_seen
                .iter()
                .any(|(j, leaf, q)| *j == i && leaf == d.leaf && *q == p)
            {
                failures.push(format!(
                    "{} on {p}: listed, but differs nowhere: remove the preset from the entry",
                    d.leaf
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} leaves differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

/// Whether this build compares the style function `leaf`: the iced theme in
/// every build, the `iced_aw` style functions with the `iced_aw` feature,
/// every other style function with the `widgets` feature, which builds them.
fn compared_here(leaf: &str) -> bool {
    let aw = leaf.starts_with("aw::");
    leaf.starts_with("to_theme")
        || (aw && cfg!(feature = "iced_aw"))
        || (!aw && cfg!(feature = "widgets"))
}

/// A key of [`Difference::presets`] as the preset's key and the scheme it
/// names, `None` for both.
fn split_key(key: &str) -> (&str, Option<&str>) {
    match key.split_once('/') {
        Some((preset, scheme)) => (preset, Some(scheme)),
        None => (key, None),
    }
}

#[test]
fn every_listed_difference_names_a_covered_preset_once() {
    for (i, d) in DIFFERENCES.iter().enumerate() {
        assert!(!d.reason.is_empty(), "{}: no reason", d.leaf);
        let stated = || d.stated.iter().copied().flatten();
        for (j, s) in stated().enumerate() {
            assert!(
                !stated().take(j).any(|t| t.at == s.at && t.field == s.field),
                "{}: {:?} stated twice at {:?}",
                d.leaf,
                s.field,
                s.at
            );
        }
        for p in d.presets {
            let (preset, scheme) = split_key(p);
            assert!(
                PRESETS.iter().any(|(key, _)| *key == preset)
                    && scheme.is_none_or(|s| s == "light" || s == "dark"),
                "{}: {p} is not covered",
                d.leaf
            );
            let overlaps = |q: &&str| {
                let (other, other_scheme) = split_key(q);
                other == preset
                    && (scheme.is_none() || other_scheme.is_none() || scheme == other_scheme)
            };
            assert!(
                !DIFFERENCES
                    .iter()
                    .enumerate()
                    .any(|(j, e)| j != i && e.leaf == d.leaf && e.presets.iter().any(overlaps)),
                "{} on {p}: listed twice",
                d.leaf
            );
        }
    }
}
