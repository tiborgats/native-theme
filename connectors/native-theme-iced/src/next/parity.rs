//! Connector parity of the 0.8.0 work (docs/v0.8.0-plan.md, section 1, rule
//! 2; M4, step 1): what the connector hands to iced, computed from the 0.7.2
//! model by the connector's public mapping and from the 0.8.0 model by
//! [`super`], is equal: the iced theme and the style each ported style
//! function returns, in every status of its widget.
//!
//! Presets covered: kde-breeze, adwaita, windows-11, macos-monterey,
//! macos-tahoe, material-3 and ios-13, light and dark ([`PRESETS`]). The
//! 0.7.2 side is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`), with no accessibility scaling; the
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

use std::fmt::Debug;

use iced_core::theme::Theme;
use native_theme::next::{Preset, ThemeContext};

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
    reason: &'static [&'static str],
}

/// How a listed style function's 0.8.0 side differs from the 0.7.2 side:
/// where its strokes lie, which of its state options the 0.8.0 theme
/// leaves to iced, whether it leaves the corners or the stroke and its
/// width to iced, whether it rounds a menu's popup by the menu's own
/// corners, and what it states that 0.7.2 has no field for. The expected
/// value is the 0.7.2 side's output with its strokes laid there, those
/// options unset, the menu's popup rounded by 0.7.2's
/// `menu.border.corner_radius` and the added property's value laid in.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Listed {
    pub align: LineAlign,
    pub states: States,
    pub iced_corners: bool,
    // The two read by the `iced_aw` sources alone.
    #[cfg_attr(not(feature = "iced_aw"), allow(dead_code))]
    pub iced_stroke: bool,
    #[cfg_attr(not(feature = "iced_aw"), allow(dead_code))]
    pub menu_corners: bool,
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
}

/// The state options of a style function's source that the 0.8.0 theme
/// states nothing for (spec §2, `from_state`), each then as
/// [`crate::styles::in_state`] gives it: the hovered and the pressed fill,
/// stroke and label, an open drop-down's stroke, a switch's hovered thumb
/// and a slider's filled rail under the pointer and dragged.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct States {
    pub hover_fill: bool,
    pub hover_stroke: bool,
    pub hover_label: bool,
    pub pressed_fill: bool,
    pub pressed_stroke: bool,
    pub pressed_label: bool,
    pub open_stroke: bool,
    pub hover_thumb: bool,
    pub rail: bool,
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
        open_stroke: false,
        hover_thumb: false,
        rail: false,
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
            menu_corners: false,
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
            menu_corners: false,
            added: Added::Nothing,
        },
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
            menu_corners: false,
            added: Added::Nothing,
        },
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
            menu_corners: false,
            added: Added::Nothing,
        },
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

/// The presets on which a check box and a radio state a hover fill and no
/// hover stroke, the stroke outside the box.
const HOVER_FILL_ONLY: &[&str] = &["adwaita", "material-3", "ios-13"];

/// The presets on which a check box, a radio and a drop-down field state a
/// hover fill and no hover stroke, the stroke inside the box.
const HOVER_FILL_ONLY_INSIDE: &[&str] = &["windows-11", "macos-monterey"];

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
        reason: match align {
            LineAlign::Outside => &[PRESSED],
            LineAlign::Inside => &[INSIDE, PRESSED],
            LineAlign::Center => &[CENTER, PRESSED],
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
        reason: &[SCROLLER_OUTLINE],
    }
}

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    // button.body(NONE), its states and its variants; macos-tahoe states a
    // hovered fill and no pressed one (`body(PRESSED)`), nor does the
    // toolbar (`toolbar.button(PRESSED)`).
    inside("button", &["macos-monterey"]),
    inside("button_toggle", &["macos-monterey"]),
    pressed("button", &["macos-tahoe"], LineAlign::Inside),
    pressed("button_toggle", &["macos-tahoe"], LineAlign::Inside),
    pressed("toolbar_button", &["macos-tahoe"], LineAlign::Outside),
    // windows-11's primary variant lays its stroke inside the box, the plain
    // button's outside (`button.body(NONE; variant=Primary).stroke`); its
    // default button is the primary one.
    inside(
        "button_primary",
        &["windows-11", "macos-monterey", "macos-tahoe"],
    ),
    inside(
        "button_default",
        &["windows-11", "macos-monterey", "macos-tahoe"],
    ),
    inside("button_danger", MACOS),
    inside("button_success", MACOS),
    inside("button_warning", MACOS),
    // input.field(NONE) and text_area.field(NONE).
    inside("text_input", MACOS),
    inside("text_editor", MACOS),
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
            menu_corners: false,
            added: Added::Nothing,
        },
        reason: &[CENTER],
    },
    // checkbox.indicator and radio.indicator: the hovered fill and stroke
    // (`indicator(HOVER)`, `indicator(HOVER|CHECKED)`), the strokes of
    // windows-11 and the macOS presets inside the box. On the macOS presets
    // that is the transparent unchecked stroke too, which iced is handed as
    // the fill under it (0.7.2's `checkbox.unchecked_border_color` is
    // `#00000000`, the home's stroke the same colour, Inside).
    // The label of a box whose hovered fill is iced's is iced's too
    // (`label(HOVER)`, which no preset states).
    hover(
        "checkbox",
        &["kde-breeze"],
        LineAlign::Outside,
        hovered(true, true, false),
    ),
    hover(
        "checkbox",
        HOVER_FILL_ONLY,
        LineAlign::Outside,
        hovered(false, false, true),
    ),
    hover(
        "checkbox",
        HOVER_FILL_ONLY_INSIDE,
        LineAlign::Inside,
        hovered(false, false, true),
    ),
    hover(
        "checkbox",
        &["macos-tahoe"],
        LineAlign::Inside,
        hovered(true, true, true),
    ),
    hover(
        "radio",
        &["kde-breeze"],
        LineAlign::Outside,
        hovered(true, true, false),
    ),
    hover(
        "radio",
        HOVER_FILL_ONLY_INSIDE,
        LineAlign::Inside,
        hovered(false, false, true),
    ),
    hover(
        "radio",
        &["macos-tahoe"],
        LineAlign::Inside,
        hovered(true, true, true),
    ),
    // combo_box.field: the hovered fill and stroke (`field(HOVER)`), and the
    // open stroke (`field(OPEN)`), which kde-breeze alone states.
    open(
        &["adwaita", "ios-13"],
        LineAlign::Outside,
        hovered(false, false, true),
    ),
    open(
        HOVER_FILL_ONLY_INSIDE,
        LineAlign::Inside,
        hovered(false, false, true),
    ),
    open(
        &["macos-tahoe"],
        LineAlign::Inside,
        hovered(true, false, true),
    ),
    open(&["material-3"], LineAlign::Outside, States::STATED),
    // switch.thumb(HOVER): no preset states a hovered thumb.
    Difference {
        leaf: "toggler",
        presets: &[
            "kde-breeze",
            "adwaita",
            "windows-11",
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
        reason: &[HOVER_THUMB],
    },
    // The menu's frame, popover.panel's stroke over menu.panel's fill.
    inside(
        "menu",
        &["adwaita", "windows-11", "macos-monterey", "macos-tahoe"],
    ),
    // slider.active_track(HOVER) and (PRESSED): no preset states them;
    // slider.thumb(HOVER): no hovered thumb fill (macos-monterey: in its dark
    // scheme).
    Difference {
        leaf: "slider",
        presets: &[
            "kde-breeze",
            "windows-11",
            "macos-monterey/dark",
            "macos-tahoe",
        ],
        listed: Listed {
            states: States {
                hover_fill: true,
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        reason: &[HOVER, SLIDER_RAIL],
    },
    Difference {
        leaf: "slider",
        presets: &["adwaita", "macos-monterey/light", "material-3", "ios-13"],
        listed: Listed {
            states: States {
                rail: true,
                ..States::STATED
            },
            ..Listed::aligned(LineAlign::Outside)
        },
        reason: &[SLIDER_RAIL],
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
            menu_corners: false,
            added: Added::Nothing,
        },
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
            menu_corners: false,
            added: Added::Nothing,
        },
        reason: &[PRESSED_THUMB, INHERITED_THUMB_HOVER],
    },
    // segmented_control.track(NONE).stroke over the group fill.
    inside("segmented_control", MACOS_WINDOWS),
    // segmented_control.segment(PRESSED) and expander.header(PRESSED): no
    // preset states them; macos-tahoe states no hovered fill either.
    pressed(
        "segment",
        &[
            "kde-breeze",
            "adwaita",
            "windows-11",
            "macos-monterey",
            "material-3",
            "ios-13",
        ],
        LineAlign::Outside,
    ),
    pressed(
        "expander",
        &["adwaita", "macos-monterey", "material-3", "ios-13"],
        LineAlign::Outside,
    ),
    // iced_aw: card.container(NONE).
    inside("aw::card", &["windows-11"]),
    aw(
        "aw::card",
        &["adwaita", "material-3"],
        LineAlign::Outside,
        States::STATED,
        (true, false),
        &[CARD_STROKE],
    ),
    aw(
        "aw::card",
        &["macos-monterey", "macos-tahoe", "ios-13"],
        LineAlign::Outside,
        States::STATED,
        (true, true),
        &[CARD_STROKE, CARD_CORNERS],
    ),
    // iced_aw: the menu's frame, popover.panel's stroke over menu.panel's
    // fill; ios-13's menu.panel(NONE).corners.
    inside(
        "aw::menu",
        &["adwaita", "windows-11", "macos-monterey", "macos-tahoe"],
    ),
    // iced_aw: tab.tab(SELECTED)'s and tab.tab(NONE)'s strokes and
    // tab.tab(HOVER)'s fill.
    inside("aw::tab_bar", &["windows-11"]),
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
        let expected = expected(listed);
        if expected == new {
            if old != new {
                self.listed_seen.push((i, function.to_owned(), key));
            }
        } else {
            let diff = debug_diff(&format!("{expected:?}"), &format!("{new:?}"));
            self.failures.push(format!(
                "{}: {leaf}: listed as {listed:?}, but not the expected value (expected -> \
                 0.8.0): {diff}",
                self.label
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
/// mode, which its theme holds beside the variants.
fn resolve(
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
    let theme = crate::Theme::preset(key)?;
    let name = theme.name.to_string();
    let old = theme
        .into_variant(mode)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
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

/// A style function of no status as one of a single status.
#[cfg(feature = "widgets")]
fn surface<T>(f: impl Fn(&Theme) -> T + Clone) -> impl Fn(&Theme, ()) -> T + Clone {
    move |theme, ()| f(theme)
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
    let at = move |l: Listed| b.listed(l);
    listed_style!(
        report,
        themes,
        "button",
        o::button(old),
        n::button(new),
        |l| o::button_from(&at(l)),
        button
    );
    for checked in [false, true] {
        listed_style!(
            report,
            themes,
            "button_toggle",
            o::button_toggle(old, checked),
            n::button_toggle(new, checked),
            |l| o::button_toggle_from(&at(l), checked),
            button
        );
        let t = ToolbarButtonSource::of(old);
        listed_style!(
            report,
            themes,
            "toolbar_button",
            o::toolbar_button(old, checked),
            n::toolbar_button(new, checked),
            |l: Listed| o::toolbar_button_from(&at(l), &t.aligned(l.align), checked),
            button
        );
    }
    listed_style!(
        report,
        themes,
        "button_primary",
        o::button_primary(old),
        n::button_primary(new),
        |l| o::button_primary_from(&at(l)),
        button
    );
    listed_style!(
        report,
        themes,
        "button_default",
        o::button_default(old),
        n::button_default(new),
        |l| o::button_default_from(&at(l)),
        button
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
                report.leaf(
                    &format!("{name}({status:?})"),
                    old_fn(old_theme, status),
                    new_fn(new_theme, status),
                    |l: Listed| class_old(Some(l))(old_theme, status),
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
        style!(
            report,
            themes,
            "text_input",
            o::text_input(old),
            n::text_input(new),
            |a| o::text_input_from(&i.aligned(a)),
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
        style!(
            report,
            themes,
            "text_editor",
            o::text_editor(old),
            n::text_editor(new),
            |a| o::text_editor_from(&i.aligned(a)),
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
        listed_style!(
            report,
            themes,
            "checkbox",
            o::checkbox(old),
            n::checkbox(new),
            |l: Listed| o::checkbox_from(&c.listed(l), window),
            boxes
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
        listed_style!(
            report,
            themes,
            "radio",
            o::radio(old),
            n::radio(new),
            |l: Listed| o::radio_from(
                fills.listed(l),
                to_color(c.indicator_color),
                to_color(c.font.color),
                c.border.line_width,
            ),
            circles
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
        listed_style!(
            report,
            themes,
            "toggler",
            o::toggler(old),
            n::toggler(new),
            |l: Listed| o::toggler_from(&s.listed(l), window),
            switches
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
        listed_style!(
            report,
            themes,
            "pick_list",
            o::pick_list(old),
            n::pick_list(new),
            |l: Listed| o::pick_list_from(&c.listed(l)),
            fields
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
        listed_style!(
            report,
            themes,
            "slider",
            o::slider(old),
            n::slider(new),
            |l: Listed| o::slider_from(&s.listed(l)),
            [Status::Active, Status::Hovered, Status::Dragged]
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
            style!(
                report,
                themes,
                "segmented_control",
                surface(o::segmented_control(old, disabled)),
                n::segmented_control(new, disabled).map(surface),
                |a| surface(o::segmented_control_from(
                    &sc.listed(Listed::aligned(a)),
                    disabled
                )),
                [()]
            );
            plain!(
                report,
                themes,
                "segment_separator",
                surface(o::segment_separator(old, disabled)),
                n::segment_separator(new, disabled).map(surface),
                [()]
            );
        }
        for selected in [false, true] {
            for position in [
                SegmentPosition::First,
                SegmentPosition::Middle,
                SegmentPosition::Last,
                SegmentPosition::Only,
            ] {
                listed_style!(
                    report,
                    themes,
                    "segment",
                    o::segment(old, selected, position),
                    n::segment(new, selected, position),
                    |l: Listed| o::segment_from(&sc.listed(l), selected, position),
                    button
                );
            }
        }
    }
    {
        let x = ExpanderSource::of(old);
        listed_style!(
            report,
            themes,
            "expander",
            o::expander(old),
            n::expander(new),
            |l: Listed| o::expander_from(&x.listed(l)),
            button
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
    plain!(
        report,
        themes,
        "pane_grid",
        surface(o::pane_grid(old)),
        n::pane_grid(new).map(surface),
        [()]
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
        |l: Listed| leaves(
            o::menu_from(&menu.listed(l, old.defaults.border.corner_radius)),
            aw_leaves::menu
        ),
        statuses
    );
    let tab = TabBarSource::of(old);
    listed_style!(
        report,
        themes,
        "aw::tab_bar",
        leaves(o::tab_bar(old), aw_leaves::tab_bar),
        n::tab_bar(new).map(|f| leaves(f, aw_leaves::tab_bar)),
        |l: Listed| leaves(o::tab_bar_from(&tab.listed(l)), aw_leaves::tab_bar),
        statuses
    );
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
        // The `iced_aw` style functions are compared with that feature only.
        if d.leaf.starts_with("aw::") && !cfg!(feature = "iced_aw") {
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
