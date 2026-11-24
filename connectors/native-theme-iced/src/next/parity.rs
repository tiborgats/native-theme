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

    /// This entry with a disabled check box's stroke left to iced
    /// ([`States::disabled_stroke`]), for `reason`.
    const fn disabled_stroke(self, reason: &'static [&'static str]) -> Self {
        Self {
            listed: Listed {
                states: States {
                    disabled_stroke: true,
                    ..self.listed.states
                },
                ..self.listed
            },
            reason,
            ..self
        }
    }
}

/// The reason of a disabled check box whose stroke the 0.8.0 theme states
/// nothing for.
const DISABLED_STROKE: &str = "the theme states no stroke for the disabled check box: 0.7.2's \
     connector drew a disabled box in its outline at rest, checked or not; the 0.8.0 theme has \
     no rule for DISABLED that states checkbox.indicator's stroke (the from_state bit clear \
     when queried with DISABLED alone), and iced's own check box recolours its border when \
     disabled (iced_widget 0.14.2 src/checkbox.rs, fn primary: background.weak for \
     background.strong unchecked, background.strong for primary.base checked), so its colour \
     is iced's own for the status, its width the theme's at rest, which iced does not change \
     (docs/v0.8.0-spec.md §2, from_state; §10, rule 0)";

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
    /// `background` (a tab bar's `tab_label_background`).
    Background,
    /// A slider's `handle.background`.
    HandleBackground,
    /// `text_color`.
    TextColor,
    /// A check box's `icon_color`.
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
}

impl Field {
    const ALL: [Self; 10] = [
        Self::Background,
        Self::HandleBackground,
        Self::TextColor,
        Self::IconColor,
        Self::BorderWidth,
        Self::BorderColor,
        Self::InnerRadius,
        Self::Separator,
        Self::PickedSplitColor,
        Self::HoveredSplitColor,
    ];
}

/// A home's value as the 0.8.0 builder reads it: a fill (a stated
/// `Paint::None` transparent, [`super::fill_color`]) or a text colour; a
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
            .then(|| crate::next::fill_color(part))
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
    /// A scroll bar handle's fill at rest and its outline: the pen's width
    /// and alignment, its colour at rest, under the pointer and dragged,
    /// round a pill ([`crate::styles::pill`]).
    ScrollerOutline {
        thumb: iced_core::Color,
        width: f32,
        align: LineAlign,
        rest: iced_core::Color,
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
/// dragged, a checked button's fill (`checked`: drawn as the primary
/// button's, spec §10, rule 0a), a disabled check box's stroke, and a
/// focused field's inset ring (`ring_over`: drawn over the field by
/// `next::focus_ring`, not as its border).
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
    pub disabled_stroke: bool,
    pub ring_over: bool,
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
        disabled_stroke: false,
        ring_over: false,
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

/// The reason of a hovered scroll bar thumb 0.7.2's resolver fills in.
const INHERITED_THUMB_HOVER: &str = "nor a hovered thumb: 0.7.2's preset leaves \
     scrollbar.thumb_hover_color unset and its resolver fills it from defaults.muted_color \
     (docs/inheritance-rules.toml, scrollbar.thumb_hover_color), a value the 0.8.0 model does \
     not carry (docs/v0.8.0-spec.md §5, a value 0.7.2's resolver makes up is not carried); no \
     rule for HOVER states scrollbar.thumb's fill, and the scroller under the pointer is \
     iced's own for the status";

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

/// The reason of a card whose outline the 0.8.0 theme leaves unstated.
const CARD_STROKE: &str = "the theme states no stroke for the card: 0.7.2's preset leaves \
     card.border's colour and width unset and its resolver makes up a transparent line 0 wide \
     (validate_border), a value the 0.8.0 model does not carry (docs/v0.8.0-spec.md §5, notes \
     to the table: a value 0.7.2's resolver makes up is not carried; the table of homes, \
     card.border.*: unstated_if_unset), and the card's outline is iced_aw's own card's \
     (iced_aw 0.14.1 src/style/card.rs, fn primary; docs/v0.8.0-spec.md §10, rule 0)";

/// The reason of a card whose corners the 0.8.0 theme leaves unstated.
const CARD_CORNERS: &str = "nor corners: 0.7.2's resolver makes up a square card \
     (validate_border), and the radius is iced_aw's own card's";

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

/// The reason of a hovered tab's fill the 0.8.0 theme states nothing for.
const TAB_HOVER_FILL: &str = "the theme states no fill for the hovered tab: 0.7.2 leaves \
     tab.hover_background unset (docs/inheritance-rules.toml, [no_inheritance]: no rule \
     fills it), which its connector drew as the idle tab; the 0.8.0 theme has no rule for \
     HOVER that states tab.tab's fill, and the fill is iced_aw's own for the status \
     (docs/v0.8.0-spec.md §2, from_state; §10, rule 0)";

/// The reason of a hovered sidebar item 0.7.2's resolver fills in.
const SIDEBAR_HOVER: &str = "the theme states no fill for the hovered sidebar item: 0.7.2's \
     preset leaves sidebar.hover_background unset and its resolver fills it from the \
     sidebar's background (docs/inheritance-rules.toml, sidebar.hover_background), a value the \
     0.8.0 model does not carry (docs/v0.8.0-spec.md §5, notes to the table: a hovered \
     sidebar item's fill); no rule for HOVER states sidebar.item's fill, and the fill is \
     iced_aw's own for the status";

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

/// The reason of a focused text field whose inset focus ring the 0.8.0 side
/// paints over it.
const RING_OVER: &str = "the focus ring is inset at the field's edge (adwaita: 2 wide at \
     offset -2, libadwaita 1.10.0 src/stylesheet/_drawing.scss, focus-ring): 0.7.2's connector \
     drew it as the focused field's border (src/styles.rs, InputColors::new); the 0.8.0 side \
     paints the ring over the field as the band over its outermost 2 px (next::focus_ring, \
     docs/v0.8.0-spec.md §2, FocusIndicator: the geometry of a Ring), as GTK draws the \
     outline last over the border box, which no focus rule of libadwaita's changes, so the \
     focused field keeps its own border, the stroke adwaita states for FOCUSED \
     (input.field(FOCUSED), text_area.field(FOCUSED): 0 wide, in colors.focus_ring, inside)";

/// A text field on adwaita whose inset focus ring the 0.8.0 side paints
/// over it, not as its focused border ([`States::ring_over`]).
const fn ring_over(leaf: &'static str) -> Difference {
    Difference {
        leaf,
        presets: &["adwaita"],
        listed: Listed {
            states: States {
                ring_over: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Inside)
        },
        stated: &[],
        reason: &[INSIDE, RING_OVER],
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

/// The reason of a toolbar button lettered in the toolbar's own labels.
const TOOLBAR_LABELS: &str = "0.7.2's connector lettered the toolbar button in the push \
     button's labels (button.text_color and its hovered, pressed, checked and disabled \
     colours); the 0.8.0 theme's toolbar button has a label of its own, \
     toolbar.button_label(<state>) (docs/v0.8.0-spec.md §5, the 27-widget contract table; \
     native-theme/tests/next_homes.toml, row toolbar.font.color), so the button is lettered \
     in its colour at rest, in a state's where a rule for the state states it, and where none \
     does as styles::in_state gives it against iced's own secondary button, a disabled label \
     the one at rest faded by button.disabled_opacity, a stated checked fill in the checked \
     label where a rule states it, else in the label at rest (0.7.2's connector: the pressed \
     one), never the push button's labels (docs/v0.8.0-spec.md §2, from_state; §10, rules 0 \
     and 0a)";

/// A toolbar button lettered in the toolbar's own labels on `presets`
/// ([`TOOLBAR_LABELS`]).
const fn toolbar_labels(presets: &'static [&'static str]) -> Difference {
    Difference {
        leaf: "toolbar_button",
        presets,
        listed: Listed::aligned(LineAlign::Outside),
        stated: &[],
        reason: &[TOOLBAR_LABELS],
    }
}

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
     frameContrast, 0.2, DecorationFocus #3daee9 at rest in the view that has the focus, \
     DecorationHover #3daee9 under the pointer and dragged, round a pill, which iced draws as \
     the pen over the fill at half its alpha (CENTER) and as a radius its renderers clamp to \
     half the shorter side (styles::pill; docs/v0.8.0-plan.md §2, M4 step 1: each visible \
     property the new model states and the toolkit can draw wired through)";

/// The reason of a scroll bar handle at rest drawn as the focused view's.
const FOCUSED_HANDLE: &str = "the scroller at rest is the handle of the view that has the \
     focus, scrollbar.thumb(NONE) (docs/v0.8.0-spec.md §5, S3; §3, WidgetState: \
     UNFOCUSED_VIEW only where the view lacks the focus, which iced does not report), where \
     0.7.2 states one handle, the plain one of a view without the focus \
     (scrollbar.thumb_color); the kde-breeze preset states Breeze's measured focused handle \
     (native-theme's kde-breeze preset, `scrollbar_thumb_focused`: \
     docs/issues/0004_kde-leftovers-rationale.md, S3), #b1d9ef light, #2a546b dark";

/// Breeze's handle in the view that has the focus (native-theme's
/// kde-breeze preset, `scrollbar_thumb_focused`): #b1d9ef light, #2a546b
/// dark.
const BREEZE_FOCUSED_HANDLE_LIGHT: iced_core::Color = iced_core::Color::from_rgb8(0xb1, 0xd9, 0xef);
const BREEZE_FOCUSED_HANDLE_DARK: iced_core::Color = iced_core::Color::from_rgb8(0x2a, 0x54, 0x6b);

/// Breeze's DecorationFocus at frameContrast, the handle's outline at rest
/// in the view that has the focus: #3daee9 at alpha 0.2.
const BREEZE_FOCUS_FRAME: iced_core::Color = iced_core::Color::from_rgba8(0x3d, 0xae, 0xe9, 0.2);

/// Breeze's DecorationHover at frameContrast, the handle's outline under
/// the pointer and dragged: #3daee9 at alpha 0.2.
const BREEZE_HOVER_FRAME: iced_core::Color = iced_core::Color::from_rgba8(0x3d, 0xae, 0xe9, 0.2);

/// Breeze's scrollable on the scheme `presets` names, its handle at rest
/// filled `thumb`.
const fn breeze_scroller(presets: &'static [&'static str], thumb: iced_core::Color) -> Difference {
    Difference {
        leaf: "scrollable",
        presets,
        listed: Listed {
            added: Added::ScrollerOutline {
                thumb,
                width: 1.001,
                align: LineAlign::Center,
                rest: BREEZE_FOCUS_FRAME,
                hovered: BREEZE_HOVER_FRAME,
                dragged: BREEZE_HOVER_FRAME,
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[SCROLLER_OUTLINE, FOCUSED_HANDLE],
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

/// macos-tahoe's toolbar button: [`TOOLBAR_WIDTHS`].
const TAHOE_TOOLBAR: &[&[Stated]] = &[&TOOLBAR_WIDTHS];

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
/// stroke, the checked box's stroke.
const W11_CHECK_HOVER: &[Stated] = &[
    set(
        at!(HOVER),
        Field::Background,
        home!(fill, checkbox.indicator(HOVER)),
    ),
    CHECK_HOVER_STROKES[0],
    CHECK_HOVER_STROKES[1],
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

/// A radio's hovered stroke (`radio.indicator(HOVER)`).
const RADIO_HOVER_STROKE: &[Stated] = &[set(
    at!(HOVER),
    Field::BorderColor,
    home!(stroke, radio.indicator(HOVER)),
)];

/// windows-11's radio under the pointer: the unselected circle's fill, and
/// its stroke at rest laid over it (no rule for HOVER states one, and iced's
/// own radio keeps its border colour under the pointer, iced_widget 0.14.2
/// `src/radio.rs`, fn default).
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
        under: Some(super::fill_color(&c.indicator(state))?),
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

/// ios-13's disabled check box: the outline's width at rest, checked or
/// not; its colour, which no rule for DISABLED states, is iced's own
/// ([`DISABLED_STROKE`]).
const IOS_CHECK_DISABLED: [Stated; 2] = [
    set(
        at!(DISABLED),
        Field::BorderWidth,
        home!(width, checkbox.indicator(NONE)),
    ),
    set(
        at!(DISABLED | CHECKED),
        Field::BorderWidth,
        home!(width, checkbox.indicator(CHECKED)),
    ),
];

/// ios-13's radio button, as its check box ([`IOS_CHECK`]; iced has no
/// disabled radio status).
const IOS_RADIO: [Stated; 8] = indicator_outline!(radio);

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
    unfilled_toolbar(&["macos-tahoe", "ios-13"]).stating(
        TAHOE_TOOLBAR,
        &[TOOLBAR_STATE_FILLS, CHECKED, STATED, TOOLBAR_LABELS],
    ),
    unfilled_toolbar(&["macos-monterey"])
        .stating(&[], &[TOOLBAR_STATE_FILLS, CHECKED, TOOLBAR_LABELS]),
    unfilled_toolbar(&["material-3"]).stating(&[], &[TOOLBAR_STATE_FILLS, CHECKED, TOOLBAR_LABELS]),
    toolbar_labels(&["kde-breeze", "adwaita", "windows-11"]),
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
    // adwaita's inset focus ring, which 0.7.2 drew as the focused field's
    // border, is painted over the field by `next::focus_ring`.
    ring_over("text_input"),
    ring_over("text_editor"),
    // tooltip.panel, dialog.panel, progress_bar.track(NONE).
    inside("tooltip", MACOS_WINDOWS),
    inside("dialog", MACOS_WINDOWS),
    inside("progress_bar", MACOS_WINDOWS),
    // card.container(NONE).
    inside("container_card", &["windows-11"]),
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
    // No preset states the disabled box's stroke (`indicator(DISABLED)`),
    // which is then iced's own.
    stated_fields(
        "checkbox",
        &["ios-13"],
        LineAlign::Outside,
        &[&IOS_CHECK, &IOS_CHECK_DISABLED],
    )
    .disabled_stroke(&[STATED, DISABLED_STROKE]),
    stated_fields(
        "checkbox",
        &["adwaita"],
        LineAlign::Outside,
        &[&CHECK_HOVER_STROKES],
    )
    .disabled_stroke(&[STATED, DISABLED_STROKE]),
    stated_fields("checkbox", &["material-3"], LineAlign::Outside, &[M3_CHECK])
        .disabled_stroke(&[STATED, DISABLED_STROKE]),
    inside("checkbox", &["macos-monterey"]).disabled_stroke(&[INSIDE, DISABLED_STROKE]),
    stated_fields(
        "checkbox",
        &["windows-11"],
        LineAlign::Inside,
        &[W11_CHECK_HOVER],
    )
    .disabled_stroke(&[INSIDE, STATED, DISABLED_STROKE]),
    hover(
        "checkbox",
        &["macos-tahoe"],
        LineAlign::Inside,
        hovered(true, true, false),
    )
    .disabled_stroke(&[INSIDE, HOVER, DISABLED_STROKE]),
    Difference {
        leaf: "checkbox",
        presets: &["kde-breeze"],
        listed: Listed::aligned(LineAlign::Outside),
        stated: &[],
        reason: &[],
    }
    .disabled_stroke(&[DISABLED_STROKE]),
    hover(
        "radio",
        &["kde-breeze"],
        LineAlign::Outside,
        hovered(true, true, false),
    ),
    hover(
        "radio",
        &["macos-monterey"],
        LineAlign::Inside,
        hovered(false, false, true),
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
        &[RADIO_HOVER_STROKE],
    ),
    hover(
        "radio",
        &["macos-tahoe"],
        LineAlign::Inside,
        hovered(true, true, true),
    ),
    stated_fields("radio", &["ios-13"], LineAlign::Outside, &[&IOS_RADIO]),
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
    // switch.thumb(HOVER): no preset states a hovered thumb; windows-11
    // states the checked track's hovered fill (`track(HOVER|CHECKED)`).
    Difference {
        leaf: "toggler",
        presets: &[
            "kde-breeze",
            "adwaita",
            "macos-monterey",
            "macos-tahoe",
            "material-3",
            "ios-13",
        ],
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
    Difference {
        leaf: "toggler",
        presets: &["windows-11"],
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
    Difference {
        leaf: "slider",
        presets: &["macos-monterey/dark", "macos-tahoe"],
        listed: Listed {
            states: States {
                hover_fill: true,
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[HOVER, SLIDER_RAIL],
    },
    Difference {
        leaf: "slider",
        presets: &["macos-monterey/light", "ios-13"],
        listed: Listed {
            states: States {
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        stated: &[],
        reason: &[SLIDER_RAIL],
    },
    Difference {
        leaf: "slider",
        presets: &["kde-breeze", "windows-11", "adwaita"],
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
    // Breeze's handle at rest (scrollbar.thumb(NONE): fill) and its outline
    // (scrollbar.thumb(NONE), (HOVER) and (PRESSED): stroke and corners).
    breeze_scroller(&["kde-breeze/light"], BREEZE_FOCUSED_HANDLE_LIGHT),
    breeze_scroller(&["kde-breeze/dark"], BREEZE_FOCUSED_HANDLE_DARK),
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
    Difference {
        leaf: "scrollable",
        presets: &["macos-tahoe"],
        listed: Listed {
            align: LineAlign::Outside,
            states: States {
                hover_fill: true,
                pressed_fill: true,
                ..States::STATED
            },
            iced_corners: false,
            iced_stroke: false,
            added: Added::Nothing,
        },
        stated: &[],
        reason: &[PRESSED_THUMB, INHERITED_THUMB_HOVER],
    },
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
    stated_fields(
        "segment",
        &["macos-tahoe"],
        LineAlign::Outside,
        &[&SEGMENT_CORNERS],
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
    pressed("segment", &["macos-monterey", "ios-13"], LineAlign::Outside),
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
    pressed(
        "expander",
        &["macos-monterey", "ios-13"],
        LineAlign::Outside,
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
    aw(
        "aw::card",
        &["macos-monterey", "macos-tahoe", "ios-13"],
        LineAlign::Outside,
        States::STATED,
        (true, true),
        &[CARD_STROKE, CARD_CORNERS],
    ),
    // iced_aw: the menu's frame, popover.panel's stroke over menu.panel's
    // fill.
    inside(
        "aw::menu",
        &["adwaita", "windows-11", "macos-monterey", "macos-tahoe"],
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
        &["macos-monterey", "ios-13"],
        LineAlign::Outside,
        States::STATED,
        (true, false),
        &[TAB_STROKE, TAB_IDLE_STROKE],
    ),
    aw(
        "aw::tab_bar",
        &["macos-tahoe"],
        LineAlign::Outside,
        hovered(true, false, false),
        (true, false),
        &[TAB_STROKE, TAB_IDLE_STROKE, TAB_HOVER_FILL],
    ),
    // iced_aw: sidebar.container(NONE)'s stroke and sidebar.item(HOVER).
    aw(
        "aw::sidebar",
        MACOS,
        LineAlign::Inside,
        hovered(true, false, false),
        (false, false),
        &[INSIDE, SIDEBAR_HOVER],
    ),
    // iced_aw: list.container(NONE)'s stroke, list.highlight(HOVER) and
    // list.item_label(HOVER).
    inside("aw::selection_list", &["windows-11"]),
    aw(
        "aw::selection_list",
        MACOS,
        LineAlign::Inside,
        hovered(true, true, false),
        (false, false),
        &[INSIDE, LIST_HOVER],
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
            | Field::HoveredSplitColor => return no_field(field, "button::Style"),
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
            | Field::HoveredSplitColor => {
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
            Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor => {
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
            | Field::HoveredSplitColor => return no_field(field, "toggler::Style"),
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
            Field::Background
            | Field::TextColor
            | Field::IconColor
            | Field::BorderWidth
            | Field::BorderColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor => return no_field(field, "slider::Style"),
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
            | Field::HoveredSplitColor => return no_field(field, "pick_list::Style"),
        }
        Ok(())
    }
}

#[cfg(feature = "widgets")]
impl Fields for iced_widget::container::Style {
    fn set(&mut self, field: Field, value: Value) -> Result<(), String> {
        use iced_core::Background;
        match field {
            Field::BorderWidth => self.border.width = width_of(field, &value)?,
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
                _ => return Err(format!("{field:?} takes a stroke, not {value:?}")),
            },
            Field::Background
            | Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::InnerRadius
            | Field::PickedSplitColor
            | Field::HoveredSplitColor => return no_field(field, "container::Style"),
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
            | Field::Separator => return no_field(field, "pane_grid::Style"),
        }
        Ok(())
    }
}

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
            Field::HandleBackground
            | Field::TextColor
            | Field::IconColor
            | Field::BorderWidth
            | Field::BorderColor
            | Field::InnerRadius
            | Field::Separator
            | Field::PickedSplitColor
            | Field::HoveredSplitColor => return no_field(field, "tab_bar::Style"),
        }
        Ok(())
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
        SegmentedSource, SliderSource, SwitchSource, ToolbarButtonSource, ToolbarLabels,
    };
    use iced_widget::button::Status::{Active, Disabled, Hovered, Pressed};

    use super::styles as n;

    let button = [Active, Hovered, Pressed, Disabled];
    let b = ButtonSource::of(old);
    let checked_fill = super::state_fill_of(
        &new.button.body(WidgetState::CHECKED),
        "button.body(CHECKED).fill",
    )
    .ok()
    .flatten();
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
            Added::Nothing | Added::ScrollerOutline { .. } => s,
        }
    };
    let none = WidgetState::NONE;
    let label = |state| new.toolbar.button_label(state);
    let toolbar_labels = ToolbarLabels {
        rest: super::text_color_of(&label(none)),
        hover: super::state_text_color_of(&label(WidgetState::HOVER)),
        pressed: super::state_text_color_of(&label(WidgetState::PRESSED)),
        checked: super::state_text_color_of(&label(WidgetState::CHECKED)),
        disabled: super::state_text_color_of(&label(WidgetState::DISABLED)),
    };
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
            |l: Listed| {
                let b = ButtonSource {
                    checked_primary: true,
                    ..at(l)
                };
                let t = ToolbarButtonSource {
                    labels: Some(toolbar_labels),
                    ..t.listed(l)
                };
                o::toolbar_button_from(&b, &t, checked)
            },
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
            o::class_button(&source, Some(to_color(fill)), Some(to_color(label)), class)
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
        listed_style!(
            report,
            themes,
            "text_input",
            o::text_input(old),
            n::text_input(new),
            |l: Listed| o::text_input_from(&i.listed(l)),
            field
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
        listed_style!(
            report,
            themes,
            "text_editor",
            o::text_editor(old),
            n::text_editor(new),
            |l: Listed| o::text_editor_from(&i.listed(l)),
            field
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
                Some(to_color(c.indicator_color)),
                Some(to_color(c.font.color)),
                Some(c.border.line_width),
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
            n::scrollable(new, WidgetState::NONE),
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
            Some(to_color(p.track_color)),
            Some(to_color(p.fill_color)),
            Outline::of(&p.border).aligned(a).into(),
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
                Some(to_color(fill)),
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
    style!(
        report,
        themes,
        "container_card",
        surface(o::container_card(old)),
        n::container_card(new).map(surface),
        panel(c.background_color, None, &c.border),
        [()]
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
    plain!(
        report,
        themes,
        "toolbar",
        surface(o::toolbar(old)),
        n::toolbar(new).map(surface),
        [()]
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
    /// A style function's output in the compared form.
    fn leaves<S, T>(
        f: impl Fn(&Theme, Status) -> S + Clone,
        form: fn(S) -> T,
    ) -> impl Fn(&Theme, Status) -> T + Clone {
        move |theme, status| form(f(theme, status))
    }

    let card = CardSource::of(old);
    listed_style!(
        report,
        themes,
        "aw::card",
        leaves(o::card(old), aw_leaves::card),
        n::card(new).map(|f| leaves(f, aw_leaves::card)),
        |l: Listed| leaves(o::card_from(&card.listed(l)), aw_leaves::card),
        statuses
    );
    let menu = MenuSource::of(old);
    listed_style!(
        report,
        themes,
        "aw::menu",
        leaves(o::menu(old), aw_leaves::menu),
        n::menu(new).map(|f| leaves(f, aw_leaves::menu)),
        |l: Listed| leaves(o::menu_from(&menu.listed(l)), aw_leaves::menu),
        statuses
    );
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
    listed_style!(
        report,
        themes,
        "aw::sidebar",
        leaves(o::sidebar(old), aw_leaves::sidebar),
        n::sidebar(new).map(|f| leaves(f, aw_leaves::sidebar)),
        |l: Listed| leaves(o::sidebar_from(&sidebar.listed(l)), aw_leaves::sidebar),
        statuses
    );
    let list = ListSource::of_list(old);
    listed_style!(
        report,
        themes,
        "aw::selection_list",
        leaves(o::selection_list(old), aw_leaves::selection_list),
        n::selection_list(new).map(|f| leaves(f, aw_leaves::selection_list)),
        |l: Listed| leaves(
            o::selection_list_from(&list.listed(l)),
            aw_leaves::selection_list
        ),
        statuses
    );
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
