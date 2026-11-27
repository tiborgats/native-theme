//! The 0.8.0 path draws what the 0.7.2 path draws (docs/v0.8.0-plan.md, section 1, rule 2;
//! spec §10, rule 4): each widget on the 0.8.0 model, drawn under the atlas of each model of
//! every platform preset (`crate::next::parity::PRESETS`, asserted against `Preset::ALL`
//! there), light and dark, in each of its states, paints the same shapes, every difference
//! listed in [`DIFFERENCES`] with its cause and what it lets differ ([`Differs`]), and a listed
//! case still equal in every value that leaves; an entry that no longer differs fails.

use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// A case drawn: its name and what it adds.
type Case<'a> = (&'static str, &'a dyn Fn(&mut egui::Ui) -> egui::Response);

/// The cases that draw differently and are not listed, and the listed ones seen, by preset.
type Drawn = (Vec<String>, Vec<(&'static str, &'static str)>);

/// What a listed difference lets differ.
#[derive(Clone, Copy, PartialEq)]
enum Differs {
    /// The outlines' colours alone ([`outlines_masked`]).
    Outline,
    /// The label's colour alone ([`labels_masked`]).
    Label,
    /// The label's gap alone: each text lies the row's width change farther along.
    Gap,
    /// A circle's outline, its outer edge kept ([`rings_masked`]).
    Ring,
    /// The field's outline (the first shape's stroke), and every other shape lying the
    /// outline's width change farther in from the side it is nearer.
    Inset,
    /// The header's side padding: the texts and the paths lie one offset farther along.
    Padding,
    /// The control's frame (the first shape's stroke): the frame the frame's width change
    /// larger on each side, every other shape that change farther along and down, a corner of
    /// one either square or the frame's, and a rect filled in an outline's colour
    /// ([`outlines_masked`]).
    Frame,
    /// The label's side alone: the label before the track, where the track began, and every
    /// other shape the label's width and the gap farther along ([`sides_swapped`]).
    Side,
    /// The label's side ([`Differs::Side`]) and its colour ([`labels_masked`]).
    SideAndLabel,
    /// The label's side ([`Differs::Side`]) and the gap the row's width change wider.
    SideAndGap,
    /// The label's side ([`Differs::Side`]), the gap the row's width change wider, and the
    /// label's colour ([`labels_masked`]).
    SideGapAndLabel,
    /// A circle's fill, a dot inside it kept ([`fills_masked`]): a radio's circle, a slider's
    /// knob.
    Fill,
    /// A radio's dot's colour: the dot's fill ([`dots_masked`]) and the colour egui letters a
    /// text without a colour of its own in, the cell's `fg_stroke` the dot is painted in, where
    /// the label's sections each carry their own ([`fallbacks_masked`]).
    Dot,
    /// The dividers between the segments ([`dividers_closed`]): each painted in 0.7.2 and none in
    /// 0.8.0, the frame and every shape after one the dividers' width nearer the start.
    Dividers,
    /// The dividers' colour alone ([`divider_fills_masked`]): 0.7.2 paints them in the outline's
    /// colour, 0.8.0 in the one the theme states for them.
    DividerFill,
    /// A rect's fill where both models fill it from a home P7 changed, each in that home's
    /// value as the case paints it ([`rect_fills_masked`], [`rect_fills`]): a drop-down's field,
    /// a segmented control's track and its selected segment.
    RectFill,
    /// What each of these lets differ, at once, each of them one that changes no place
    /// ([`masked`]).
    Each(&'static [Differs]),
}

/// A listed difference: the widget's case, the presets on which it draws differently, what may
/// differ, and why.
struct Difference {
    case: &'static str,
    presets: &'static [&'static str],
    differs: Differs,
    reason: &'static str,
}

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    Difference {
        case: "radio off",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        differs: Differs::Outline,
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio on",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        differs: Differs::Outline,
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio off disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::Each(&[Differs::Fill, Differs::Outline]),
        reason: MACOS_RADIO_DISABLED,
    },
    Difference {
        case: "radio on disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::Each(&[Differs::Fill, Differs::Outline, Differs::Dot]),
        reason: MACOS_RADIO_DISABLED,
    },
    Difference {
        case: "radio off",
        presets: &["ios-13"],
        differs: Differs::Each(&[Differs::Ring, Differs::Fill]),
        reason: IOS_RADIO,
    },
    Difference {
        case: "radio on",
        presets: &["ios-13"],
        differs: Differs::Ring,
        reason: IOS_RADIO,
    },
    Difference {
        case: "radio on",
        presets: &["material-3"],
        differs: Differs::Each(&[Differs::Fill, Differs::Dot]),
        reason: M3_RADIO,
    },
    Difference {
        case: "radio off disabled",
        presets: &["material-3"],
        differs: Differs::Each(&[Differs::Label, Differs::Fill, Differs::Outline]),
        reason: M3_RADIO_DISABLED,
    },
    Difference {
        case: "radio on disabled",
        presets: &["material-3"],
        differs: Differs::Each(&[Differs::Label, Differs::Fill, Differs::Outline]),
        reason: M3_RADIO_DISABLED,
    },
    Difference {
        case: "radio off disabled",
        presets: &["ios-13"],
        differs: Differs::Each(&[Differs::Label, Differs::Ring, Differs::Fill]),
        reason: IOS_RADIO_DISABLED,
    },
    Difference {
        case: "radio on disabled",
        presets: &["ios-13"],
        differs: Differs::Each(&[Differs::Label, Differs::Ring, Differs::Fill, Differs::Dot]),
        reason: IOS_RADIO_DISABLED,
    },
    Difference {
        case: "radio on disabled",
        presets: &["kde-breeze"],
        differs: Differs::Each(&[Differs::Fill, Differs::Outline, Differs::Dot]),
        reason: KDE_RADIO_DISABLED,
    },
    Difference {
        case: "radio on",
        presets: &["adwaita"],
        differs: Differs::Ring,
        reason: ADWAITA_RADIO_CHECKED,
    },
    Difference {
        case: "radio on disabled",
        presets: &["adwaita"],
        differs: Differs::Ring,
        reason: ADWAITA_RADIO_CHECKED,
    },
    Difference {
        case: "radio off disabled",
        presets: &["windows-11"],
        differs: Differs::Outline,
        reason: WINDOWS_RADIO_DISABLED,
    },
    Difference {
        case: "radio on disabled",
        presets: &["windows-11"],
        differs: Differs::Each(&[Differs::Fill, Differs::Outline, Differs::Dot]),
        reason: WINDOWS_RADIO_DISABLED,
    },
    Difference {
        case: "combo box",
        presets: &["macos-monterey"],
        differs: Differs::Outline,
        reason: COMBO_CELL,
    },
    Difference {
        case: "combo box",
        presets: &["windows-11"],
        differs: Differs::Outline,
        reason: WINDOWS_COMBO,
    },
    Difference {
        case: "combo box disabled",
        presets: &["macos-monterey"],
        differs: Differs::Outline,
        reason: COMBO_CELL,
    },
    Difference {
        case: "combo box disabled",
        presets: &["windows-11"],
        differs: Differs::Outline,
        reason: WINDOWS_COMBO_DISABLED,
    },
    Difference {
        case: "combo box",
        presets: &["macos-tahoe"],
        differs: Differs::Inset,
        reason: TAHOE_COMBO,
    },
    Difference {
        case: "combo box disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Inset,
        reason: TAHOE_COMBO,
    },
    Difference {
        case: "combo box",
        presets: &["ios-13"],
        differs: Differs::Inset,
        reason: IOS_COMBO,
    },
    Difference {
        case: "combo box disabled",
        presets: &["ios-13"],
        differs: Differs::Inset,
        reason: IOS_COMBO,
    },
    Difference {
        case: "segmented control",
        presets: &["windows-11", "macos-monterey"],
        differs: Differs::Outline,
        reason: SEGMENTED_CELL,
    },
    Difference {
        case: "segmented control disabled",
        presets: &["windows-11", "macos-monterey"],
        differs: Differs::Outline,
        reason: SEGMENTED_CELL,
    },
    Difference {
        case: "segmented control",
        presets: &["macos-tahoe"],
        differs: Differs::Frame,
        reason: TAHOE_SEGMENTED,
    },
    Difference {
        case: "segmented control disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Frame,
        reason: TAHOE_SEGMENTED,
    },
    Difference {
        case: "segmented control",
        presets: &["kde-breeze"],
        differs: Differs::Dividers,
        reason: KDE_SEGMENTED,
    },
    Difference {
        case: "segmented control",
        presets: &["adwaita"],
        differs: Differs::Each(&[Differs::DividerFill, Differs::RectFill]),
        reason: ADWAITA_SEGMENTED,
    },
    Difference {
        case: "segmented control disabled",
        presets: &["adwaita"],
        differs: Differs::Each(&[Differs::DividerFill, Differs::RectFill]),
        reason: ADWAITA_SEGMENTED,
    },
    Difference {
        case: "combo box",
        presets: &["adwaita"],
        differs: Differs::RectFill,
        reason: ADWAITA_COMBO,
    },
    Difference {
        case: "combo box disabled",
        presets: &["adwaita"],
        differs: Differs::RectFill,
        reason: ADWAITA_COMBO,
    },
    Difference {
        case: "segmented control disabled",
        presets: &["kde-breeze"],
        differs: Differs::Dividers,
        reason: KDE_SEGMENTED,
    },
    Difference {
        case: "slider",
        presets: &["ios-13"],
        differs: Differs::Fill,
        reason: IOS_SLIDER_KNOB,
    },
    Difference {
        case: "expander closed",
        presets: &["macos-tahoe"],
        differs: Differs::Padding,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "expander open",
        presets: &["macos-tahoe"],
        differs: Differs::Padding,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "expander closed disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Padding,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "expander open disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Padding,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "switch off disabled",
        presets: &["kde-breeze"],
        differs: Differs::Label,
        reason: SWITCH_DISABLED_LABEL,
    },
    Difference {
        case: "switch on disabled",
        presets: &["kde-breeze"],
        differs: Differs::Label,
        reason: SWITCH_DISABLED_LABEL,
    },
    Difference {
        case: "switch off",
        presets: &["material-3", "ios-13"],
        differs: Differs::Side,
        reason: SWITCH_LEADING,
    },
    Difference {
        case: "switch on",
        presets: &["material-3", "ios-13"],
        differs: Differs::Side,
        reason: SWITCH_LEADING,
    },
    Difference {
        case: "switch off disabled",
        presets: &["ios-13"],
        differs: Differs::SideAndLabel,
        reason: IOS_SWITCH_DISABLED,
    },
    Difference {
        case: "switch on disabled",
        presets: &["ios-13"],
        differs: Differs::SideAndLabel,
        reason: IOS_SWITCH_DISABLED,
    },
    Difference {
        case: "switch off",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::SideAndGap,
        reason: MACOS_SWITCH,
    },
    Difference {
        case: "switch on",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::SideAndGap,
        reason: MACOS_SWITCH,
    },
    Difference {
        case: "switch off disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::SideGapAndLabel,
        reason: MACOS_SWITCH_DISABLED,
    },
    Difference {
        case: "switch on disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::SideGapAndLabel,
        reason: MACOS_SWITCH_DISABLED,
    },
    Difference {
        case: "switch off disabled",
        presets: &["material-3"],
        differs: Differs::SideAndLabel,
        reason: M3_SWITCH_DISABLED,
    },
    Difference {
        case: "switch on disabled",
        presets: &["material-3"],
        differs: Differs::SideAndLabel,
        reason: M3_SWITCH_DISABLED,
    },
    Difference {
        case: "switch off",
        presets: &["adwaita"],
        differs: Differs::SideGapAndLabel,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch on",
        presets: &["adwaita"],
        differs: Differs::SideGapAndLabel,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch off disabled",
        presets: &["adwaita"],
        differs: Differs::SideGapAndLabel,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch on disabled",
        presets: &["adwaita"],
        differs: Differs::SideGapAndLabel,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch off",
        presets: &["windows-11"],
        differs: Differs::Gap,
        reason: WINDOWS_SWITCH_GAP,
    },
    Difference {
        case: "switch on",
        presets: &["windows-11"],
        differs: Differs::Gap,
        reason: WINDOWS_SWITCH_GAP,
    },
    Difference {
        case: "switch off disabled",
        presets: &["windows-11"],
        differs: Differs::Gap,
        reason: WINDOWS_SWITCH_GAP,
    },
    Difference {
        case: "switch on disabled",
        presets: &["windows-11"],
        differs: Differs::Gap,
        reason: WINDOWS_SWITCH_GAP,
    },
];

/// kde-breeze's disabled switch label.
const SWITCH_DISABLED_LABEL: &str = "the label's colour alone differs: 0.7.2 has no switch \
     label and takes defaults.disabled_text_color; the 0.8.0 kde-breeze states the platform's \
     disabled switch label, qqc2-desktop-style's, the Disabled [Colors:Window] text at its \
     opacity 0.6 (#a0a1a399 / #686a6c99: Switch.qml:72, :75 at qqc2-desktop-style v6.31.0; \
     plasmadesktoptheme.cpp:273-274 at Kirigami 8319acc7); crate::next::parity, \
     switch::Look.disabled_text";

/// The switch label's side on the presets that lead with it.
const SWITCH_LEADING: &str = "the label's side alone differs: 0.7.2's SwitchTheme states no \
     label side, so the label lies after the track, as egui's own Checkbox lays its text; the \
     0.8.0 preset states switch.label_side Leading, the platform's label before its switch \
     (iOS SwiftUI's ToggleStyle.switch, \"a leading label and a trailing switch\", \
     developer.apple.com, read 2026-11-23; material-3 Android's MaterialSwitch text at the \
     start, materialswitch/res/values/styles.xml:26 at material-components-android 66c334b), \
     so the label lies where the track began and the track and its thumb the label's width and \
     the gap farther along; crate::next::parity, switch::Geometry.leading";

/// material-3's disabled switch label.
const M3_SWITCH_DISABLED: &str = "the label's side and colour: the 0.8.0 material-3 states \
     switch.label_side Leading, Android's MaterialSwitch text at the start \
     (materialswitch/res/values/styles.xml:26 at material-components-android 66c334b), where \
     0.7.2 states no side and the label lies after the track; and the label in Android's \
     disabled MaterialSwitch text, on-background at material_emphasis_medium 0.6 (#1d1b2099 / \
     #e6e0e999: android:Widget.CompoundButton's textColor ?attr/textColorPrimaryDisableOnly, \
     AOSP core/res/res/values/styles.xml:386 at android-15.0.0_r1; \
     m3_primary_text_disable_only.xml:19, dimens.xml:21, materialswitch/res/values/styles.xml:17-29 \
     at material-components-android 66c334b), where 0.7.2 takes defaults.disabled_text_color; \
     crate::next::parity, switch::Geometry.leading, switch::Look.disabled_text";

/// adwaita's switch label.
const ADWAITA_SWITCH: &str = "the label's side, gap and colour: the 0.8.0 adwaita states \
     switch.label_side Leading, AdwSwitchRow's title first and the switch a suffix at the row's \
     end (adw-action-row.ui:40-43, :91-94, adw-switch-row.c:161 at libadwaita 1.10.0), where \
     0.7.2 states no side and the label lies after the track; \
     switch.label_gap 6, the least gap of AdwSwitchRow's header box (_lists.scss:132 at \
     libadwaita 1.10.0), where 0.7.2 has none and the label lies at the scope's own \
     icon_spacing, 2 pt farther; and the label in AdwSwitchRow's title colour, the row's text, \
     #000006cc in light (_lists.scss:5-7, :521; _misc.scss:202-204; _colors.scss:37, :62 at \
     libadwaita 1.10.0), at --disabled-opacity 50 % when disabled (#00000666 / #ffffff80: \
     _labels.scss:8-10, _colors.scss:37, :100, :320 at libadwaita 1.10.0), where 0.7.2 took the \
     window's text and defaults.disabled_text_color; crate::next::parity, switch::Look.text and \
     .disabled_text, switch::Geometry.leading and .label_gap";

/// windows-11's switch label.
const WINDOWS_SWITCH_GAP: &str = "the label's gap: the 0.8.0 windows-11 states \
     switch.label_gap 12, ToggleSwitch's fixed column (ToggleSwitch_themeresources.xaml:499-503 \
     at microsoft-ui-xaml 8463f45), where 0.7.2 has none and the label lies at the scope's own \
     icon_spacing, 4 pt nearer; crate::next::parity, switch::Geometry.label_gap";

/// material-3's disabled radio.
const M3_RADIO_DISABLED: &str = "the label's colour, and the circle's fill and outline: 0.7.2 \
     draws the label in defaults.disabled_text_color, on-surface at 0.38, fills the circle in \
     that colour and outlines it as at rest (#49454f unchecked, primary checked, in light); the \
     0.8.0 material-3 states Android's disabled label, the label's colour at \
     material_emphasis_medium 0.6 (#1d1b2099 / #e6e0e999: android:Widget.CompoundButton's \
     textColor ?attr/textColorPrimaryDisableOnly, AOSP core/res/res/values/styles.xml:386 at \
     android-15.0.0_r1; radiobutton/res/values/styles.xml:19-33 sets none, \
     m3_primary_text_disable_only.xml:19, dimens.xml:21 at material-components-android 66c334b), \
     and material-web's disabled radio, its ring and its dot in on-surface at 0.38 \
     (#1d1b2061 / #e6e0e961: disabled-unselected-icon-color, disabled-selected-icon-color, \
     _radio.scss:163-166, :180-183; _md-comp-radio-button.scss:28-33 at material-web cbd34a8), \
     nothing filling the space between them (radio.ts:108-120, _radio.scss:109-113), so the \
     circle is unfilled and outlined in that colour, and the dot, checked, is as before";

/// material-3's radio.
const M3_RADIO: &str = "a checked circle's fill and the dot's colour: 0.7.2 fills a checked \
     circle in primary and draws the dot in its Checkbox cell's on-primary (#ffffff / \
     #381e72); the 0.8.0 material-3 states \
     material-web's radio, one SVG of a ring and a dot in the icon's colour, nothing filling the \
     space between them (radio.ts:108-120, the ring :113-118, the dot :119; _radio.scss:109-113 \
     at material-web cbd34a8), checked selected-icon-color, primary (_radio.scss:126-127; \
     _md-comp-radio-button.scss:45, v0_192), so the checked circle is unfilled and the dot is \
     primary (#6750a4 / #d0bcff); the label is lettered in its own colour, as before";

/// ios-13's radio circle.
const IOS_RADIO: &str = "the circle's outline: 0.7.2's ios-13 outlines it in its uncited \
     default line, 0.5 px in the separator, checked or not; the 0.8.0 ios-13 states the outline \
     WebKit strokes round an empty iOS radio, checkboxRadioBorderColor (secondaryLabel), 1.5 px \
     as it shows (checkboxRadioBorderWidth × 2 clipped to the edge), and none round a checked \
     one (RenderThemeIOS.mm:1354, :1357-1368, :1483-1491, :1502 at WebKit 31adf9d; \
     native-theme/tests/next_homes.toml, rows checkbox.unchecked_border_color and \
     checkbox.border.line_width, listed differences); the circle's outer edge stays; and, in \
     dark, an empty circle's fill: the 0.8.0 ios-13 states WebKit's checkboxRadioBackgroundColor, \
     -webkit-control-background, systemBackground (#000000 in dark: RenderThemeIOS.mm:1391, \
     :1395, :1027 at WebKit 31adf9d), where 0.7.2 took its uncited \
     checkbox.unchecked_background, #1c1c1e (in light both are #ffffff)";

/// ios-13's disabled radio.
const IOS_RADIO_DISABLED: &str = "the circle, its outline, the dot and the label's colour: \
     0.7.2's ios-13 fades a disabled radio as one by its defaults.disabled_opacity 0.3; the \
     0.8.0 ios-13 states radio.disabled_opacity 1, the iOS style sheet's `opacity: initial` for \
     check boxes and radios (html.css:1031-1034, :1040-1047 at WebKit 31adf9d), and WebKit's \
     disabled colours: an empty circle in systemBackground, outlined 1.5 px in secondaryLabel at \
     checkboxRadioBorderDisabledOpacity 0.3 (RenderThemeIOS.mm:1387-1388, :1355, :1359-1362, \
     :1027), a checked one filled in opaqueTertiaryFill with no outline (#eeeeef / #1c1c1e: \
     :1372, :1387-1388, :1502, :1561-1563), the dot in tertiaryLabel (#3c3c434c / #ebebf54c: \
     :1410-1413, :1578-1579), and the label in its disabled colour (#c7c7cc / #3a3a3c, 0.7.2 \
     ios-13.toml, uncited) unfaded; the circle's outer edge stays";

/// kde-breeze's disabled radio.
const KDE_RADIO_DISABLED: &str = "a checked circle's fill and the dot's colour: 0.7.2 fills a \
     disabled checked circle in #f0f0f0 / #272a2e and draws the dot in #a0a1a3 / #686a6c; the \
     0.8.0 kde-breeze states Breeze's \
     disabled checked circle, Disabled [Colors:Button] BackgroundNormal under the Disabled \
     Highlight at highlightBackgroundAlpha (#eceded / #25272b: breezehelper.cpp:862, :865-869, \
     :1012-1019, :40 at breeze f0b1d75; breezemetrics.h:176; platform-facts.md §2.5), and its \
     dot in palette.text() of the Disabled group, Disabled [Colors:View] ForegroundNormal \
     (#aaabac / #606263: breezehelper.cpp:938, :1059 at breeze f0b1d75; platform-facts.md \
     §2.1.6); a checked circle's outline: 0.7.2 keeps the checked one's #3daee9, the 0.8.0 \
     kde-breeze's rule for DISABLED | CHECKED states the frame of a checked radio, \
     palette.highlight() of the Disabled group, the Disabled Highlight (#e3e5e7 / #1f2124: \
     breezehelper.cpp:853-854, :1005-1006 at breeze f0b1d75; platform-facts.md §2.5, §2.21), \
     which the query at that state asking DISABLED states (spec §2, from_state); the label is \
     lettered in its own colour, as before";

/// adwaita's checked radio.
const ADWAITA_RADIO_CHECKED: &str = "a checked circle's outline: 0.7.2's adwaita outlines it \
     1 px wide; the 0.8.0 adwaita states none round a checked check or radio, \
     libadwaita's `&:checked` with `box-shadow: none` (_checks.scss:52-56 at libadwaita 1.10.0; \
     the outline is the inset box-shadow, :27), disabled too, `:disabled` changing no colour \
     (:62-64), so the accent fills the circle to its outer edge, which stays";

/// windows-11's disabled radio.
const WINDOWS_RADIO_DISABLED: &str = "the circle's outline, a checked one's fill, and the dot's \
     colour: 0.7.2 draws its Checkbox cell's disabled outline (#00000072 / #ffffff8b unchecked), \
     no fill, and the dot in #0000005c / #ffffff5d; the 0.8.0 windows-11 states WinUI's \
     disabled radio: unchecked, outlined in \
     RadioButtonOuterEllipseStrokeDisabled = ControlStrongStrokeColorDisabled (#00000037 / \
     #ffffff28: RadioButton_themeresources.xaml:137, :21 at microsoft-ui-xaml 8463f45), checked, \
     filled and outlined in AccentFillColorDisabled (#00000037 / #ffffff28: \
     RadioButtonOuterEllipseCheckedFillDisabled, …CheckedStrokeDisabled, set at :326-330; :149, \
     :145), the outline composited over the fill as the Checkbox cell lays its bg_stroke \
     (RADIO_CELL), and the dot RadioButtonCheckGlyphFillDisabled = TextOnAccentFillColorPrimary, \
     the rest's (#ffffff / #000000: :153, :37), which the radio lays into the cell's fg_stroke";

/// The macOS presets' disabled radio.
const MACOS_RADIO_DISABLED: &str = "the circle's fill and outline and the dot's colour: 0.7.2 \
     draws its Checkbox cell's Disabled and SelectedDisabled colours; the 0.8.0 macOS presets \
     state the measured disabled radio (os-probe run 45345116763, macOS 15 and 26, \
     `*.normal.radio_disabled_off` and `*.normal.radio_disabled_on` `fill`, `edge_*`, the dot \
     `label`; macos-monterey's checked one the unchecked one's fill, macos-tahoe's #0076f470 / \
     #0982ff70), the outline composited over the fill as the \
     Checkbox cell lays its bg_stroke (RADIO_CELL), and the checked dot's colour laid into the \
     cell's fg_stroke; macos-monterey states no disabled dot in dark (the run's \
     `*.normal.radio_disabled_on` `label` is no one colour), so there the dot is the cell's own \
     disabled fg_stroke, egui's treatment of the state (spec §10, rule 0), where 0.7.2 drew \
     #ffffff";

/// kde-breeze's segmented control.
const KDE_SEGMENTED: &str = "the divider between the segments: 0.7.2's kde-breeze states an \
     uncited segmented_control.separator_width 1, which the control paints as a 1 pt line in \
     its outline's colour; the 0.8.0 kde-breeze states 0, no line: Breeze's tab bar, KDE's \
     segmented control (platform-facts.md §2.25), fills an unselected tab with no pen and widens \
     it by TabBar_TabOverlap into the next, an overlap of the fills, not a line \
     (breezehelper.cpp:1488-1506, breezestyle.cpp:7238 at breeze f0b1d75; native-theme-egui, \
     next::parity, the segmented control's item_spacing entry), so the control is 1 pt \
     narrower and the second segment lies 1 pt nearer the start";

/// ios-13's slider knob.
const IOS_SLIDER_KNOB: &str = "the knob's fill: 0.7.2's ios-13 states no slider.thumb_color, \
     which its resolver fills in (#f2f2f7 / #1c1c1e); the 0.8.0 ios-13 states the measured \
     thumb, white in both schemes (ios-host run 45230061151, iOS 18.6, measure_lists.toml \
     `press1.\"<a>\".slider.normal` `fill`); crate::next::parity, slider::Look.thumb_color";

/// ios-13's disabled switch label.
const IOS_SWITCH_DISABLED: &str = "the label's side and colour: the 0.8.0 ios-13 states \
     switch.label_side Leading, as SWITCH_LEADING gives, and the disabled switch label \
     (switch.label(DISABLED)) in the label colour as at rest, #000000 / #ffffff, the list row's \
     UILabel beside a disabled UISwitch (ios-host run 45230061151, iOS 18.6, measure_lists.toml \
     `run1.\"<a>\".switch_row.disabled` `label_text_color`), faded with the switch, where 0.7.2 \
     has no switch label and takes defaults.disabled_text_color; crate::next::parity, \
     switch::Geometry.leading, switch::Look.disabled_text";

/// The macOS presets' switch label.
const MACOS_SWITCH: &str = "the label's side and gap: the 0.8.0 macOS presets state \
     switch.label_side Leading, SwiftUI's ToggleStyle.switch, \"a leading label and a trailing \
     switch\" (developer.apple.com, read 2026-11-23), and switch.label_gap 8, the measured \
     SwiftUI Toggle's label 8 px before its switch (os-probe run 45210835145, macOS 15 and 26, \
     switch-label.toml `[[item]] toggle_off` fitting width 33 + 8 + the switch), where 0.7.2 \
     states neither and the label lies after the track at the scope's own icon_spacing, 2 pt \
     nearer; crate::next::parity, switch::Geometry.leading and .label_gap";

/// adwaita's segmented control: its dividers, its track and its checked toggle (material-3's
/// separator fill is its outline's colour, drawn alike).
const ADWAITA_SEGMENTED: &str = "the dividers' colour and the track's and the checked \
     toggle's fills: 0.7.2 states the dividers no colour of their own, so the control paints \
     them in its outline's; the 0.8.0 adwaita states AdwToggleGroup's `> separator`, \
     GtkSeparator's `background: $border_color` (_toggle-group.scss:141-159, _misc.scss:21-22 \
     at libadwaita 1.10.0), segmented_control.separator(NONE).fill, which the control paints \
     them in (crate::next::parity, segmented_control::Look.separator); it states the track in \
     libadwaita's own form, the window text at `$button_color` 10 %, translucent \
     (_toggle-group.scss:12, _buttons.scss:1), and its checked toggle \
     `--active-toggle-bg-color`, #ffffff in light and white at 20 % in dark, a layer over the \
     track (_toggle-group.scss:79-80, _colors.scss:238, :245-247; docs/platform-facts.md \
     §2.25; native-theme/tests/next_homes.toml, rows segmented_control.background_color and \
     segmented_control.active_background, listed differences), where 0.7.2 states the track \
     laid over the window, opaque (#e6e6e7 / #38383c), and the dark checked toggle laid over \
     that (#606063); a disabled control fades them as one over its backdrop \
     (segmented_control.rs, fade_as_one); native-theme-egui, next::parity, the \
     SegmentedControl entries, ADWAITA_FIELD_FILLS and ADWAITA_CHECKED_TOGGLE";

/// adwaita's drop-down field.
const ADWAITA_COMBO: &str = "the field's fill alone: the 0.8.0 adwaita states the drop-down's \
     trigger, a raised button, in libadwaita's own form, the window text at `$button_color` \
     10 %, translucent (_buttons.scss:1, :28-29, _dropdowns.scss:1-12 at libadwaita 1.10.0; \
     docs/platform-facts.md §2.24; native-theme/tests/next_homes.toml, row \
     combo_box.background_color, a listed difference), where 0.7.2 states it laid over the \
     window, opaque (#e6e6e7 / #38383c); a disabled field keeps that rest fill in its Disabled \
     cell, libadwaita stating no disabled fill, and egui fades it by the cell's disabled_alpha; \
     native-theme-egui, next::parity, the ComboBox and ComboBox/Disabled entries, \
     ADWAITA_FIELD_FILLS";

/// The macOS presets' disabled switch label.
const MACOS_SWITCH_DISABLED: &str = "the label's side, gap and colour: as MACOS_SWITCH gives, \
     and the disabled switch label (switch.label(DISABLED)) in labelColor as at rest, \
     #000000d8 / #ffffffd8, the measured SwiftUI Toggle's (os-probe run 45210835145, \
     switch-label-macos-{15,26}/measure.toml `*.normal.toggle_*_disabled` within 3 levels of \
     the labelColor reference), where 0.7.2 has no switch label and takes \
     defaults.disabled_text_color; crate::next::parity, switch::Look.disabled_text";

/// macos-tahoe's drop-down field.
const TAHOE_COMBO: &str = "the field's outline and the text's inset: 0.7.2's macos-tahoe \
     states the default line (0.5); the 0.8.0 macos-tahoe states the pop-up button's outline as \
     none, as macOS 26 draws it (os-probe run 45198750893, macOS 26, `*.normal.popup` `edge_*`: \
     every edge pixel is the fill; native-theme/tests/next_homes.toml, rows \
     combo_box.border.line_width and combo_box.border.color, listed differences), so the text \
     and the arrow lie the line's 0.5 pt nearer the edge (native-theme-egui, next::parity, the \
     ComboBox button_padding entry)";

/// ios-13's drop-down field.
const IOS_COMBO: &str = "the field's outline and the text's inset: 0.7.2's ios-13 states the \
     default border (0.5 in the separator, uncited); the 0.8.0 ios-13 states the border WebKit \
     draws round an iOS select, 1px of -webkit-control-background, systemBackground \
     (html.css:1101 at WebKit 31adf9d; RenderThemeIOS.mm:1027; \
     native-theme/tests/next_homes.toml, rows combo_box.border.line_width and \
     combo_box.border.color, listed differences), so the text and the arrow lie 0.5 pt farther \
     in";

/// macos-tahoe's segmented control.
const TAHOE_SEGMENTED: &str = "the frame: 0.7.2's macos-tahoe states the default line, which \
     the control draws as a 1 pt frame; the 0.8.0 macos-tahoe states the segmented control's \
     outline as none, as macOS 26 draws it (os-probe run 45198750893, macOS 26, \
     `*.normal.segment` `edge_top`, `edge_bottom`; native-theme/tests/next_homes.toml, rows \
     segmented_control.border.line_width and segmented_control.border.color, listed \
     differences), so the control is the frame's width smaller on each side and its end \
     segments take its corners";

/// macos-tahoe's expander header.
const TAHOE_EXPANDER: &str = "the header's side padding: the header pads a side it states none \
     of by the base style's button_padding, which macos-tahoe's push button, its bezel stroked \
     as none (M2.FS4, measured; native-theme/tests/next_homes.toml, row button.border.line_width, \
     a listed difference), now leaves at its padding alone, where 0.7.2 added its 0.5 hairline \
     (native-theme-egui, next::parity, the base button_padding entry): the arrow and the title \
     lie 0.5 pt nearer the start";

/// The drop-down is egui's `ComboBox`, outlined in its cell's stroke.
const COMBO_CELL: &str = "the button's outline colour alone differs: it is the connector's \
     ComboBox cell's bg_stroke, whose translucent border lies inside the part in the 0.8.0 theme \
     and is composited over its fill (native-theme-egui, next::parity, the ComboBox and \
     ComboBox/Disabled entries of its differences)";

/// windows-11's combo box outlined in the elevation border's colour, outside its fill.
const WINDOWS_COMBO: &str = "the outline's colour alone differs, in dark: combo_box.field's \
     stroke, which 0.7.2's windows-11 takes from the default border, CardStrokeColorDefault \
     #19000000, and the 0.8.0 preset states as the three sides of WinUI's \
     ControlElevationBorderBrush, ControlStrokeColorDefault #12FFFFFF (its edge not drawn; \
     native-theme/tests/next_homes.toml, row combo_box.border.color, a listed difference), laid \
     outside the fill, as Border's default BackgroundSizing, InnerBorderEdge, lays it (the \
     combo box's Background border states none), so handed to egui as stated, not composited \
     over the fill; in light both are #0F000000";

/// windows-11's disabled combo box keeps its rest outline, the elevation border's colour.
const WINDOWS_COMBO_DISABLED: &str = "the outline's colour alone differs, in dark: a disabled \
     combo box keeps its rest outline, combo_box.field's stroke, which 0.7.2's windows-11 takes \
     from the default border, CardStrokeColorDefault #19000000, and the 0.8.0 preset states as the \
     three sides of WinUI's ControlElevationBorderBrush, ControlStrokeColorDefault #12FFFFFF, as \
     ComboBoxBorderBrushDisabled is (native-theme/tests/next_homes.toml, row \
     combo_box.border.color, a listed difference); in light both are #0F000000";

/// The segments are framed and divided in their cell's stroke.
const SEGMENTED_CELL: &str = "the frame's and the dividers' colour alone differs: it is the \
     connector's SegmentedControl cell's bg_stroke, whose translucent border lies inside the part \
     in the 0.8.0 theme and is composited over its fill (native-theme-egui, next::parity, the \
     SegmentedControl entry of its differences); a disabled control fades that colour as one";

/// The radio's circle is egui's, outlined in its Checkbox cell's stroke.
const RADIO_CELL: &str = "the circle's outline colour alone differs: radio.indicator's stroke, \
     laid over the connector's Checkbox cell as the cell lays its bg_stroke, a translucent border \
     inside the part in the 0.8.0 theme, composited over its fill (native-theme-egui, \
     next::parity, the Checkbox entry of its differences)";

/// A context with the atlas of each model of `key` installed, drawn in its scheme: the 0.7.2
/// theme resolved as `crate::next::parity::resolve` resolves it, and the 0.8.0 preset as
/// `next::to_theme` takes it (its themes for an active and an inactive window). Neither atlas
/// installs a font plan, so both letter in egui's own fonts: the connector's parity compares
/// the fonts each plan installs (`native-theme-egui`, `next::root_parity`), this one the widgets.
fn both_installed(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> Result<(egui::Context, egui::Context), Box<dyn std::error::Error>> {
    let scheme = if is_dark {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    let theme = NextTheme::preset(preset);
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx));
    let new_atlas =
        ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive).build()?;
    // The 0.7.2 atlas takes the 0.8.0 one's icon set and themes, so both spinners draw the
    // same set's indicator: the connector's parity compares the icon sets.
    let (light, _) = crate::next::parity::resolve(key, preset, false)?;
    let (dark, _) = crate::next::parity::resolve(key, preset, true)?;
    // The preset's layout, as `from_preset` hands it to the 0.7.2 builder (the connector's
    // `next::parity` builds its 0.7.2 atlas so).
    let layout = native_theme_egui::native_theme::theme::Theme::preset(key)?.layout;
    let mut old_builder = ThemeAtlas::builder(key, &light, &dark)
        .layout(&layout)
        .icon_set(new_atlas.icon_set());
    for scheme in [egui::Theme::Light, egui::Theme::Dark] {
        if let Some(name) = new_atlas.icon_theme(scheme) {
            old_builder = old_builder.icon_theme(scheme, name);
        }
    }
    let old = egui::Context::default();
    old_builder.build().install(&old);
    let new = egui::Context::default();
    new_atlas.install(&new);
    for ctx in [&old, &new] {
        ctx.set_theme(scheme);
        ctx.enable_accesskit();
    }
    Ok((old, new))
}

/// Draw each case under both atlases of every preset and scheme, the second of two passes, and
/// return the cases that draw differently and are not listed, and the listed ones seen.
fn drawn_alike(cases: &[Case<'_>]) -> Result<Drawn, Box<dyn std::error::Error>> {
    let mut failures = Vec::new();
    let mut listed = Vec::new();
    for &(key, preset) in crate::next::parity::PRESETS {
        for is_dark in [false, true] {
            let (old, new) = both_installed(key, preset, is_dark)?;
            let fills = rect_fills(key, preset, is_dark)?;
            for &(case, add) in cases {
                let _ = response(&old, at(1.0), add);
                let _ = response(&new, at(1.0), add);
                let (a, ra) = response(&old, at(1.0), add);
                let (b, rb) = response(&new, at(1.0), add);
                let (a, b) = (flat(&a.shapes), flat(&b.shapes));
                if a == b && ra.rect == rb.rect {
                    continue;
                }
                let entry = DIFFERENCES
                    .iter()
                    .find(|d| d.case == case && d.presets.contains(&key));
                let is_listed = entry.is_some();
                // A listed case differs in what its entry lets differ alone: every other value
                // of every shape is equal.
                if let Some(entry) = entry {
                    listed.push((key, case));
                    if alike_but(entry.differs, &fills, (&a, ra.rect), (&b, rb.rect)) {
                        continue;
                    }
                }
                let shown = |s: &egui::Shape| match s {
                    egui::Shape::Text(t) => format!(
                        "text at {:?}, {:?}, {:?}",
                        t.pos,
                        t.galley.size(),
                        t.galley.job.sections.first().map(|s| &s.format),
                    ),
                    other => format!("{other:?}"),
                };
                let different: Vec<String> = a
                    .iter()
                    .zip(&b)
                    .filter(|(a, b)| a != b)
                    .map(|(a, b)| format!("\n    0.7.2 {}\n    0.8.0 {}", shown(a), shown(b)))
                    .collect();
                failures.push(format!(
                    "{key}/{}: {case}{}: rect 0.7.2 {:?}, 0.8.0 {:?}; {} and {} shapes; \
                     different:{}",
                    if is_dark { "dark" } else { "light" },
                    if is_listed {
                        " (listed: it differs beyond what its entry lets differ)"
                    } else {
                        ""
                    },
                    ra.rect,
                    rb.rect,
                    a.len(),
                    b.len(),
                    different.concat(),
                ));
            }
        }
    }
    Ok((failures, listed))
}

/// Within this, a place is the one a listed difference moves it to (the layout's `f32` sums).
const NEAR: f32 = 1e-3;

fn near(p: egui::Pos2, q: egui::Pos2) -> bool {
    (p - q).length() < NEAR
}

/// Where a shape lies: a text's position, a rect's least corner, a circle's centre, a path's
/// first point.
fn anchor(shape: &egui::Shape) -> Option<egui::Pos2> {
    match shape {
        egui::Shape::Text(t) => Some(t.pos),
        egui::Shape::Rect(r) => Some(r.rect.min),
        egui::Shape::Circle(c) => Some(c.center),
        egui::Shape::Path(p) => p.points.first().copied(),
        _ => None,
    }
}

/// `b` with its place taken as `a`'s where it lies within [`NEAR`] of `a`'s moved by `by`:
/// equal to `a` where `b` is `a` moved by `by` and alike in every other value. A text's galley
/// is laid out relative to its position, so only the position moves, and a galley laid out
/// alike ([`laid_out_alike`]) from a job that wraps it at another width is taken as `a`'s.
fn snapped(a: &egui::Shape, b: &egui::Shape, by: egui::Vec2) -> egui::Shape {
    let at = |p: egui::Pos2, q: egui::Pos2| near(p + by, q);
    let mut b = b.clone();
    match (a, &mut b) {
        (egui::Shape::Text(a), egui::Shape::Text(b)) if at(a.pos, b.pos) => {
            b.pos = a.pos;
            if laid_out_alike(&a.galley, &b.galley, true) {
                b.galley = a.galley.clone();
            }
        }
        (egui::Shape::Rect(a), egui::Shape::Rect(b))
            if at(a.rect.min, b.rect.min) && at(a.rect.max, b.rect.max) =>
        {
            b.rect = a.rect;
        }
        (egui::Shape::Circle(a), egui::Shape::Circle(b)) if at(a.center, b.center) => {
            b.center = a.center;
        }
        (egui::Shape::Path(a), egui::Shape::Path(b))
            if a.points.len() == b.points.len()
                && a.points.iter().zip(&b.points).all(|(p, q)| at(*p, *q)) =>
        {
            b.points.clone_from(&a.points);
        }
        _ => {}
    }
    b
}

/// Whether `b` is `a` but for its labels' colours: the colour a text without one of its own is
/// lettered in, an override ([`labels_masked`]), and the colour its galley's sections are laid
/// out in ([`laid_out_alike`]).
fn labels_alike(a: &[egui::Shape], b: &[egui::Shape]) -> bool {
    labels_masked(a) == labels_masked(&galleys_taken(a, b))
}

/// `b` with each text's galley taken as `a`'s where the two are laid out alike but for their
/// colours ([`laid_out_alike`]), every other value kept.
fn galleys_taken(a: &[egui::Shape], b: &[egui::Shape]) -> Vec<egui::Shape> {
    a.iter()
        .zip(b)
        .map(|(x, y)| match (x, y) {
            (egui::Shape::Text(x), egui::Shape::Text(y))
                if laid_out_alike(&x.galley, &y.galley, false) =>
            {
                let mut y = y.clone();
                y.galley = x.galley.clone();
                egui::Shape::Text(y)
            }
            _ => y.clone(),
        })
        .collect()
}

/// `a` and `b` with what `differs` lets differ masked in both, where it changes no place: a
/// colour, an outline's width laid into its circle ([`Differs::Ring`]), or several of these
/// ([`Differs::Each`]), each in turn; `None` for a difference that moves a shape. `fills` are
/// the rect fills [`Differs::RectFill`] lets differ.
fn masked(
    differs: Differs,
    fills: &RectFills,
    a: &[egui::Shape],
    b: &[egui::Shape],
) -> Option<(Vec<egui::Shape>, Vec<egui::Shape>)> {
    let both = |mask: fn(&[egui::Shape]) -> Vec<egui::Shape>| Some((mask(a), mask(b)));
    match differs {
        Differs::Outline => both(outlines_masked),
        Differs::Label => Some((labels_masked(a), labels_masked(&galleys_taken(a, b)))),
        Differs::Ring => both(rings_masked),
        Differs::Fill => both(fills_masked),
        Differs::Dot => both(|s| fallbacks_masked(&dots_masked(s))),
        Differs::DividerFill => divider_fills_masked(a, b),
        Differs::RectFill => rect_fills_masked(a, b, fills),
        Differs::Each(each) => each.iter().try_fold((a.to_vec(), b.to_vec()), |(a, b), d| {
            masked(*d, fills, &a, &b)
        }),
        Differs::Gap
        | Differs::Inset
        | Differs::Padding
        | Differs::Frame
        | Differs::Side
        | Differs::SideAndLabel
        | Differs::SideAndGap
        | Differs::SideGapAndLabel
        | Differs::Dividers => None,
    }
}

/// Whether the galley `b` is laid out as `a`: the same text in the same sections and fonts, in
/// the same rect and mesh bounds with as many vertices, and in the same colours where `colour`
/// says so.
fn laid_out_alike(a: &egui::Galley, b: &egui::Galley, colour: bool) -> bool {
    let formats = |g: &egui::Galley| {
        g.job
            .sections
            .iter()
            .map(|s| {
                let mut format = s.format.clone();
                if !colour {
                    format.color = egui::Color32::TRANSPARENT;
                }
                (s.leading_space, s.byte_range.clone(), format)
            })
            .collect::<Vec<_>>()
    };
    a.job.text == b.job.text
        && a.rect == b.rect
        && a.mesh_bounds == b.mesh_bounds
        && a.num_vertices == b.num_vertices
        && formats(a) == formats(b)
}

/// Whether `b`, drawn in the rect `rb`, is `a`, drawn in `ra`, in every value but what
/// `differs` lets differ (`fills`, the rect fills [`Differs::RectFill`] lets differ).
fn alike_but(
    differs: Differs,
    fills: &RectFills,
    (a, ra): (&[egui::Shape], egui::Rect),
    (b, rb): (&[egui::Shape], egui::Rect),
) -> bool {
    use egui::Shape;
    // the dividers' shapes are 0.7.2's alone
    if differs != Differs::Dividers && a.len() != b.len() {
        return false;
    }
    let frames = match (a.first(), b.first()) {
        (Some(Shape::Rect(fa)), Some(Shape::Rect(fb))) => Some((fa.clone(), fb.clone())),
        _ => None,
    };
    match differs {
        Differs::Outline
        | Differs::Label
        | Differs::Ring
        | Differs::Fill
        | Differs::Dot
        | Differs::DividerFill
        | Differs::RectFill
        | Differs::Each(_) => ra == rb && masked(differs, fills, a, b).is_some_and(|(a, b)| a == b),
        Differs::Gap => {
            // the row's background spans the row, which the gap's change widens
            let along = egui::vec2(rb.width() - ra.width(), 0.0);
            let b: Vec<Shape> = a
                .iter()
                .zip(b)
                .map(|(x, y)| match (x, y) {
                    (Shape::Text(_), _) => snapped(x, y, along),
                    (Shape::Rect(x), Shape::Rect(y)) if x.rect == ra && y.rect == rb => {
                        let mut y = y.clone();
                        y.rect = x.rect;
                        Shape::Rect(y)
                    }
                    _ => y.clone(),
                })
                .collect();
            ra.min == rb.min && ra.height() == rb.height() && a == b.as_slice()
        }
        Differs::Side | Differs::SideAndLabel | Differs::SideAndGap | Differs::SideGapAndLabel => {
            let grown = rb.width() - ra.width();
            let gap = matches!(differs, Differs::SideAndGap | Differs::SideGapAndLabel);
            if !gap && grown != 0.0 {
                return false;
            }
            let Some(b) = sides_swapped(a, (b, rb), ra, grown) else {
                return false;
            };
            let alike = if matches!(differs, Differs::Side | Differs::SideAndGap) {
                a == b.as_slice()
            } else {
                labels_alike(a, &b)
            };
            ra.min == rb.min && ra.height() == rb.height() && alike
        }
        Differs::Dividers => dividers_closed(a, ra).is_some_and(|(a, ra)| {
            let moved: Vec<Shape> = a
                .iter()
                .zip(b)
                .map(|((x, by), y)| snapped(x, y, egui::vec2(-by, 0.0)))
                .collect();
            let a: Vec<Shape> = a.into_iter().map(|(x, _)| x).collect();
            a.len() == b.len() && near(ra.min, rb.min) && near(ra.max, rb.max) && a == moved
        }),
        Differs::Inset => {
            let Some((fa, mut fb)) = frames else {
                return false;
            };
            let inward = fb.stroke.width - fa.stroke.width;
            let centre = fa.rect.center().x;
            fb.stroke = fa.stroke;
            let b: Vec<Shape> = std::iter::once(Shape::Rect(fb))
                .chain(a.iter().zip(b).skip(1).map(|(x, y)| {
                    let by = match anchor(x) {
                        Some(p) if p.x > centre => -inward,
                        _ => inward,
                    };
                    snapped(x, y, egui::vec2(by, 0.0))
                }))
                .collect();
            ra == rb && a == b.as_slice()
        }
        Differs::Padding => {
            let along = a.iter().zip(b).find_map(|pair| match pair {
                (Shape::Text(x), Shape::Text(y)) => Some(egui::vec2(y.pos.x - x.pos.x, 0.0)),
                _ => None,
            });
            let Some(along) = along else {
                return false;
            };
            let b: Vec<Shape> = a
                .iter()
                .zip(b)
                .map(|(x, y)| match x {
                    Shape::Text(_) | Shape::Path(_) => snapped(x, y, along),
                    _ => y.clone(),
                })
                .collect();
            ra == rb && a == b.as_slice()
        }
        Differs::Frame => {
            let Some((fa, mut fb)) = frames else {
                return false;
            };
            let grown = egui::Vec2::splat(fb.stroke.width - fa.stroke.width);
            let fits = |x: egui::Rect, y: egui::Rect| {
                near(x.min, y.min) && near(x.max + 2.0 * grown, y.max)
            };
            if !(fits(ra, rb) && fits(fa.rect, fb.rect)) {
                return false;
            }
            let corner = fb.corner_radius;
            fb.rect = fa.rect;
            fb.stroke.width = fa.stroke.width;
            let (a, b) = (
                outlines_masked(a),
                outlines_masked(
                    &std::iter::once(Shape::Rect(fb))
                        .chain(b.iter().skip(1).cloned())
                        .collect::<Vec<_>>(),
                ),
            );
            let b: Vec<Shape> = a
                .iter()
                .zip(&b)
                .enumerate()
                .map(|(i, (x, y))| {
                    if i == 0 {
                        return y.clone();
                    }
                    let mut y = snapped(x, y, grown);
                    if let (Shape::Rect(x), Shape::Rect(y)) = (x, &mut y) {
                        let c = y.corner_radius;
                        let square_or_frame = [
                            (c.nw, corner.nw),
                            (c.ne, corner.ne),
                            (c.sw, corner.sw),
                            (c.se, corner.se),
                        ]
                        .iter()
                        .all(|(v, f)| *v == 0 || v == f);
                        if square_or_frame {
                            y.corner_radius = x.corner_radius;
                        }
                    }
                    y
                })
                .collect();
            a == b
        }
    }
}

/// `b`, drawn in `rb` with its label before the track, with its shapes put back where `a`,
/// drawn in `ra` with the label after it, has them, where each lies where the swap moves it:
/// the label to where `a`'s track began, and every other shape `a`'s label's width and gap,
/// the gap `grown` wider, farther along; the row's background, which spans the row, taken as
/// `a`'s. `None` where `a` draws no label or no track.
fn sides_swapped(
    a: &[egui::Shape],
    (b, rb): (&[egui::Shape], egui::Rect),
    ra: egui::Rect,
    grown: f32,
) -> Option<Vec<egui::Shape>> {
    use egui::Shape;
    let label = a.iter().find_map(|s| match s {
        Shape::Text(t) => Some(t),
        _ => None,
    })?;
    let track = a.iter().find_map(|s| match s {
        Shape::Rect(r) if r.rect != ra => Some(r.rect),
        _ => None,
    })?;
    // 0.7.2's gap, between the track's end and the label after it
    let gap = label.pos.x - track.max.x;
    let to_start = egui::vec2(track.min.x - label.pos.x, 0.0);
    let along = egui::vec2(label.galley.size().x + gap + grown, 0.0);
    Some(
        a.iter()
            .zip(b)
            .map(|(x, y)| match (x, y) {
                (Shape::Text(_), _) => snapped(x, y, to_start),
                (Shape::Rect(x), Shape::Rect(y)) if x.rect == ra && y.rect == rb => {
                    let mut y = y.clone();
                    y.rect = x.rect;
                    Shape::Rect(y)
                }
                _ => snapped(x, y, along),
            })
            .collect(),
    )
}

/// `shapes` with each circle that lies inside an earlier one round the same centre (a radio's
/// dot) filled transparent, every other value kept: what a listed difference, a dot's fill,
/// leaves to compare.
fn dots_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    let mut circles: Vec<(egui::Pos2, f32)> = Vec::new();
    let mut masked = Vec::with_capacity(shapes.len());
    for shape in shapes {
        let mut shape = shape.clone();
        if let egui::Shape::Circle(c) = &mut shape {
            if circles
                .iter()
                .any(|(centre, radius)| *centre == c.center && c.radius < *radius)
            {
                c.fill = egui::Color32::TRANSPARENT;
            }
            circles.push((c.center, c.radius));
        }
        masked.push(shape);
    }
    masked
}

/// `shapes` with each circle that lies inside no earlier one round the same centre (a radio's
/// circle, a slider's knob) filled transparent, a dot inside it and every other value kept:
/// what a listed difference, a circle's fill, leaves to compare.
fn fills_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    let mut circles: Vec<(egui::Pos2, f32)> = Vec::new();
    let mut masked = Vec::with_capacity(shapes.len());
    for shape in shapes {
        let mut shape = shape.clone();
        if let egui::Shape::Circle(c) = &mut shape {
            if !circles
                .iter()
                .any(|(centre, radius)| *centre == c.center && c.radius < *radius)
            {
                c.fill = egui::Color32::TRANSPARENT;
            }
            circles.push((c.center, c.radius));
        }
        masked.push(shape);
    }
    masked
}

/// `shapes` with the fallback colour of each text whose galley's sections each carry a colour
/// of their own (none `Color32::PLACEHOLDER`, which egui letters in the fallback) made
/// transparent, every other value kept: a colour the text is not lettered in, what a listed
/// difference, the cell's `fg_stroke` the dot is painted in, leaves to compare.
fn fallbacks_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    shapes
        .iter()
        .cloned()
        .map(|mut shape| {
            if let egui::Shape::Text(t) = &mut shape
                && t.galley
                    .job
                    .sections
                    .iter()
                    .all(|s| s.format.color != egui::Color32::PLACEHOLDER)
            {
                t.fallback_color = egui::Color32::TRANSPARENT;
            }
            shape
        })
        .collect()
}

/// `a`, drawn in `ra`, with its dividers taken out and the row closed up, each shape with the
/// distance it lies farther along than closed up: a divider is a rect after the first, the
/// frame, with no outline, filled in the frame's outline colour, as the control paints the
/// lines; every shape lying past a divider is that divider's width nearer the start, and the
/// frame's right edge and `ra`'s the dividers' widths together, the frame given already
/// closed up (moved by none). `None` where `a` draws no frame or no divider.
fn dividers_closed(
    a: &[egui::Shape],
    ra: egui::Rect,
) -> Option<(Vec<(egui::Shape, f32)>, egui::Rect)> {
    let Some(egui::Shape::Rect(frame)) = a.first() else {
        return None;
    };
    let line = frame.stroke.color;
    let dividers: Vec<egui::Rect> = a
        .iter()
        .skip(1)
        .filter_map(|s| match s {
            egui::Shape::Rect(r) if r.stroke.width == 0.0 && r.fill == line => Some(r.rect),
            _ => None,
        })
        .collect();
    if dividers.is_empty() {
        return None;
    }
    let closed: f32 = dividers.iter().map(egui::Rect::width).sum();
    let mut frame = frame.clone();
    frame.rect.max.x -= closed;
    let mut kept = vec![(egui::Shape::Rect(frame), 0.0)];
    for shape in a.iter().skip(1) {
        if let egui::Shape::Rect(r) = shape
            && dividers.contains(&r.rect)
            && r.stroke.width == 0.0
            && r.fill == line
        {
            continue;
        }
        let by = anchor(shape).map_or(0.0, |p| {
            dividers
                .iter()
                .filter(|d| p.x >= d.max.x - NEAR)
                .map(egui::Rect::width)
                .sum()
        });
        kept.push((shape.clone(), by));
    }
    let mut ra = ra;
    ra.max.x -= closed;
    Some((kept, ra))
}

/// `shapes` with each circle's outline taken off and its width laid back into the radius, every
/// other value kept: the outer edge the radio keeps, its circle's radius inset by the outline's
/// width (`radio_button.rs`, each state's `expansion` less its stroke's width), what a listed
/// difference, a circle's outline, leaves to compare.
fn rings_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    shapes
        .iter()
        .cloned()
        .map(|mut shape| {
            if let egui::Shape::Circle(c) = &mut shape {
                c.radius += c.stroke.width;
                c.stroke = egui::Stroke::NONE;
            }
            shape
        })
        .collect()
}

/// `shapes` with each outline's colour made transparent, and a rect filled in an outline's
/// colour (the segmented control's dividers), its width and every other value kept: what a
/// listed difference, an outline colour, leaves to compare.
fn outlines_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    let none = egui::Color32::TRANSPARENT;
    let outlines = outline_colours(shapes);
    shapes
        .iter()
        .cloned()
        .map(|mut shape| {
            match &mut shape {
                egui::Shape::Rect(r) => {
                    r.stroke.color = none;
                    if outlines.contains(&r.fill) {
                        r.fill = none;
                    }
                }
                egui::Shape::Circle(c) => c.stroke.color = none,
                egui::Shape::Ellipse(e) => e.stroke.color = none,
                egui::Shape::LineSegment { stroke, .. } => stroke.color = none,
                egui::Shape::Path(p) => p.stroke.color = egui::epaint::ColorMode::Solid(none),
                _ => {}
            }
            shape
        })
        .collect()
}

/// The dividers' colour alone: each rect `a` (0.7.2) fills in one of its outlines' colours (the
/// segmented control's dividers, [`outlines_masked`]) and the rect `b` (0.8.0) draws in its
/// place, at the same rect with the same corners and no stroke, both filled transparent, every
/// other value kept. `None` where the two draw a different number of shapes.
fn divider_fills_masked(
    a: &[egui::Shape],
    b: &[egui::Shape],
) -> Option<(Vec<egui::Shape>, Vec<egui::Shape>)> {
    if a.len() != b.len() {
        return None;
    }
    let none = egui::Color32::TRANSPARENT;
    let outlines = outline_colours(a);
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    for (old, new) in a.iter_mut().zip(b.iter_mut()) {
        if let (egui::Shape::Rect(old), egui::Shape::Rect(new)) = (old, new)
            && outlines.contains(&old.fill)
            && old.rect == new.rect
            && old.corner_radius == new.corner_radius
            && old.stroke.width == 0.0
            && new.stroke.width == 0.0
        {
            old.fill = none;
            new.fill = none;
        }
    }
    Some((a, b))
}

/// `a` and `b` with the fill of each pair of rects at the same place in both, in the same
/// rect, made transparent where `a`'s is one of `fills.old` and `b`'s one of `fills.new`, every
/// other value kept: what a listed difference, a field's, a track's or a segment's fill, leaves
/// to compare; `None` where the two have not as many shapes.
fn rect_fills_masked(
    a: &[egui::Shape],
    b: &[egui::Shape],
    fills: &RectFills,
) -> Option<(Vec<egui::Shape>, Vec<egui::Shape>)> {
    if a.len() != b.len() {
        return None;
    }
    let none = egui::Color32::TRANSPARENT;
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    for (old, new) in a.iter_mut().zip(b.iter_mut()) {
        if let (egui::Shape::Rect(old), egui::Shape::Rect(new)) = (old, new)
            && old.rect == new.rect
            && fills.old.contains(&old.fill)
            && fills.new.contains(&new.fill)
        {
            old.fill = none;
            new.fill = none;
        }
    }
    Some((a, b))
}

/// The rect fills a listed [`Differs::RectFill`] lets differ, of the 0.7.2 model (`old`) and
/// of the 0.8.0 model (`new`).
#[derive(Debug, Default)]
struct RectFills {
    old: Vec<egui::Color32>,
    new: Vec<egui::Color32>,
}

/// The fills of the homes P7 changed on adwaita ([`ADWAITA_COMBO`], [`ADWAITA_SEGMENTED`]),
/// read from the two resolved themes of `preset` (`crate::next::parity::resolve`): 0.7.2's
/// `combo_box.background_color`, `segmented_control.background_color` and `hover_background`,
/// and in dark `active_background`; 0.8.0's `combo_box.field(NONE)`,
/// `segmented_control.track(NONE)` and `segment(HOVER)` fills, and in dark
/// `segment(SELECTED)`'s (`native-theme/tests/next_homes.toml`, the rows of those keys), each
/// as the cases paint it ([`painted_fills`]).
fn rect_fills(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> Result<RectFills, Box<dyn std::error::Error>> {
    use native_theme_egui::native_theme::next::WidgetState;
    let (old, new) = crate::next::parity::resolve(key, preset, is_dark)?;
    let (sc, cb) = (&old.segmented_control, &old.combo_box);
    let old = painted_fills(
        PaintedHomes {
            field: Some(to_color32(cb.background_color)),
            track: Some(to_color32(sc.background_color)),
            hover: sc.hover_background.map(to_color32),
            selected: is_dark.then(|| to_color32(sc.active_background)),
        },
        crate::segmented_control::Look::of(&old).background,
        [Some(cb.disabled_opacity), Some(sc.disabled_opacity)],
    );
    let fill = |part| {
        native_theme_egui::next::fill_of(&part, "a P7 home")
            .ok()
            .flatten()
    };
    let (sc, cb) = (&new.segmented_control, &new.combo_box);
    let new = painted_fills(
        PaintedHomes {
            field: fill(cb.field(WidgetState::NONE)),
            track: fill(sc.track(WidgetState::NONE)),
            hover: fill(sc.segment(WidgetState::HOVER)),
            selected: is_dark
                .then(|| fill(sc.segment(WidgetState::SELECTED)))
                .flatten(),
        },
        crate::segmented_control::Look::next_of(&new).background,
        [cb.disabled_opacity, sc.disabled_opacity],
    );
    Ok(RectFills { old, new })
}

/// One model's values at the homes [`rect_fills`] reads.
struct PaintedHomes {
    /// The drop-down's field.
    field: Option<egui::Color32>,
    /// The segmented control's track.
    track: Option<egui::Color32>,
    /// A hovered segment, over the track.
    hover: Option<egui::Color32>,
    /// The selected segment, over the track.
    selected: Option<egui::Color32>,
}

/// `homes` as the cases paint them: each as given at rest; a disabled drop-down's field faded
/// by egui, `gamma_multiply` by the cell's `disabled_alpha`, the first of `alphas`
/// (`combo_box.disabled_opacity`; `Painter::multiply_opacity`, `egui/src/painter.rs` at egui
/// 0.37.0); a disabled segmented control's track and selected segment faded as one over
/// `ground`, the window's background, by the second (`segmented_control.disabled_opacity`;
/// `segmented_control.rs`, `fade_as_one`).
fn painted_fills(
    homes: PaintedHomes,
    ground: Option<egui::Color32>,
    [combo_alpha, segmented_alpha]: [Option<f32>; 2],
) -> Vec<egui::Color32> {
    let PaintedHomes {
        field,
        track,
        hover,
        selected,
    } = homes;
    let mut fills: Vec<egui::Color32> = [field, track, hover, selected]
        .into_iter()
        .flatten()
        .collect();
    if let (Some(field), Some(alpha)) = (field, combo_alpha) {
        fills.push(field.gamma_multiply(alpha));
    }
    if let (Some(track), Some(ground), Some(alpha)) = (track, ground, segmented_alpha) {
        let group = ground.blend(track);
        fills.push(crate::fade::faded(ground, group, alpha));
        if let Some(selected) = selected {
            fills.push(crate::fade::faded(ground, group.blend(selected), alpha));
        }
    }
    fills
}

/// The colours of the outlines `shapes` stroke.
fn outline_colours(shapes: &[egui::Shape]) -> Vec<egui::Color32> {
    shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(r) if r.stroke.width > 0.0 => Some(r.stroke.color),
            egui::Shape::Circle(c) if c.stroke.width > 0.0 => Some(c.stroke.color),
            egui::Shape::Ellipse(e) if e.stroke.width > 0.0 => Some(e.stroke.color),
            egui::Shape::LineSegment { stroke, .. } if stroke.width > 0.0 => Some(stroke.color),
            egui::Shape::Path(p) if p.stroke.width > 0.0 => match p.stroke.color {
                egui::epaint::ColorMode::Solid(c) => Some(c),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// `shapes` with each text's colours, the fallback a label without one of its own is lettered
/// in and an override, made transparent, every other value kept: what a listed difference, a
/// label's colour, leaves to compare.
fn labels_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    let none = egui::Color32::TRANSPARENT;
    shapes
        .iter()
        .cloned()
        .map(|mut shape| {
            if let egui::Shape::Text(t) = &mut shape {
                t.fallback_color = none;
                t.override_text_color = t.override_text_color.map(|_| none);
            }
            shape
        })
        .collect()
}

/// Every listed difference of `cases` that drew alike everywhere it is listed.
fn stale(cases: &[&str], listed: &[(&str, &str)]) -> Vec<String> {
    DIFFERENCES
        .iter()
        .filter(|d| cases.contains(&d.case))
        .flat_map(|d| {
            d.presets
                .iter()
                .filter(|p| !listed.contains(&(**p, d.case)))
                .map(|p| {
                    format!(
                        "{p}: {}: listed as a difference ({}), but drawn alike: remove the entry",
                        d.case, d.reason
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Assert the cases draw alike from both models.
fn assert_drawn_alike(cases: &[Case<'_>]) -> TestResult {
    let (mut failures, listed) = drawn_alike(cases)?;
    let names: Vec<&str> = cases.iter().map(|&(case, _)| case).collect();
    failures.extend(stale(&names, &listed));
    assert!(
        failures.is_empty(),
        "{} cases draw differently:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn the_switch_draws_alike_from_both_models() -> TestResult {
    let switch = |on: bool, enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add(Switch::new(&mut on.clone()).label("Wi-Fi").enabled(enabled))
        }
    };
    let (off, on, off_disabled, on_disabled) = (
        switch(false, true),
        switch(true, true),
        switch(false, false),
        switch(true, false),
    );
    assert_drawn_alike(&[
        ("switch off", &off),
        ("switch on", &on),
        ("switch off disabled", &off_disabled),
        ("switch on disabled", &on_disabled),
    ])
}

#[test]
fn the_radio_button_draws_alike_from_both_models() -> TestResult {
    let radio = |checked: bool, enabled: bool| {
        move |ui: &mut egui::Ui| ui.add(RadioButton::new(checked, "Radio").enabled(enabled))
    };
    let (off, on, off_disabled, on_disabled) = (
        radio(false, true),
        radio(true, true),
        radio(false, false),
        radio(true, false),
    );
    assert_drawn_alike(&[
        ("radio off", &off),
        ("radio on", &on),
        ("radio off disabled", &off_disabled),
        ("radio on disabled", &on_disabled),
    ])
}

#[test]
fn the_progress_bar_and_the_spinner_draw_alike_from_both_models() -> TestResult {
    let bar = |ui: &mut egui::Ui| ui.add(ProgressBar::new(0.4));
    let spinner = |ui: &mut egui::Ui| ui.add(Spinner::new());
    assert_drawn_alike(&[("progress bar", &bar), ("spinner", &spinner)])
}

#[test]
fn the_slider_draws_alike_from_both_models() -> TestResult {
    let slider = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add(
                Slider::new(&mut 40.0, 0.0..=100.0)
                    .label("Volume")
                    .enabled(enabled),
            )
        }
    };
    let (on, off) = (slider(true), slider(false));
    assert_drawn_alike(&[("slider", &on), ("slider disabled", &off)])
}

#[test]
fn the_segmented_control_draws_alike_from_both_models() -> TestResult {
    let control = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add_enabled_ui(enabled, |ui| {
                ui.add(SegmentedControl::new(&mut 1, ["Day", "Week"]))
            })
            .inner
        }
    };
    let (on, off) = (control(true), control(false));
    assert_drawn_alike(&[
        ("segmented control", &on),
        ("segmented control disabled", &off),
    ])
}

#[test]
fn the_links_draw_alike_from_both_models() -> TestResult {
    let link = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add_enabled_ui(enabled, |ui| {
                let w = links::link(ui, "Link");
                ui.add(w)
            })
            .inner
        }
    };
    let hyperlink = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add_enabled_ui(enabled, |ui| {
                let w = links::hyperlink(ui, "Docs", "https://example.org");
                ui.add(w)
            })
            .inner
        }
    };
    let (a, b, c, d) = (link(true), link(false), hyperlink(true), hyperlink(false));
    assert_drawn_alike(&[
        ("link", &a),
        ("link disabled", &b),
        ("hyperlink", &c),
        ("hyperlink disabled", &d),
    ])
}

#[test]
fn the_combo_box_draws_alike_from_both_models() -> TestResult {
    let combo = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ComboBox::from_id_salt("fruit")
                .selected_text("Apple")
                .width(140.0)
                .enabled(enabled)
                .show_ui(ui, |ui| ui.label("Banana"))
                .response
        }
    };
    let (on, off) = (combo(true), combo(false));
    assert_drawn_alike(&[("combo box", &on), ("combo box disabled", &off)])
}

#[test]
fn the_expander_draws_alike_from_both_models() -> TestResult {
    let expander = |open: bool, enabled: bool| {
        move |ui: &mut egui::Ui| {
            Expander::new("Details")
                .default_open(open)
                .enabled(enabled)
                .show(ui, |ui| ui.label("Body"))
                .header_response
        }
    };
    let (closed, open, closed_disabled, open_disabled) = (
        expander(false, true),
        expander(true, true),
        expander(false, false),
        expander(true, false),
    );
    assert_drawn_alike(&[
        ("expander closed", &closed),
        ("expander open", &open),
        ("expander closed disabled", &closed_disabled),
        ("expander open disabled", &open_disabled),
    ])
}

// ---- patched themes: the branches no preset reaches ---------------------------------------------

/// `preset` with `patch` laid over it, resolved light.
fn patched(preset: Preset, patch: &str) -> Result<NextResolved, Box<dyn std::error::Error>> {
    let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
    Ok(NextTheme::preset(preset)
        .with_patch(patch)
        .resolve(&ThemeContext::light()))
}

/// A context drawn light with the atlas of `preset` and `patch` installed, with no font plan.
fn patched_installed(
    preset: Preset,
    patch: &str,
) -> Result<egui::Context, Box<dyn std::error::Error>> {
    let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
    let theme = NextTheme::preset(preset).with_patch(patch);
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx));
    let ctx = egui::Context::default();
    ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive)
        .build()?
        .install(&ctx);
    ctx.set_theme(egui::Theme::Light);
    Ok(ctx)
}

type NextResolved = native_theme_egui::native_theme::next::ResolvedTheme;

/// The switch's parts and shapes, drawn off, enabled, in the second of two passes.
fn drawn_switch(ctx: &egui::Context) -> (Parts, Vec<egui::Shape>) {
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut false).label("Wi-Fi"));
    let _ = response(ctx, at(1.0), add);
    let (out, r) = response(ctx, at(1.0), add);
    (Parts::of(&r).unwrap_or_default(), flat(&out.shapes))
}

/// The rectangle painted at `rect`.
fn painted_rect(shapes: &[egui::Shape], rect: egui::Rect) -> Option<egui::epaint::RectShape> {
    shapes.iter().find_map(|s| match s {
        egui::Shape::Rect(r) if r.rect == rect => Some(r.clone()),
        _ => None,
    })
}

/// A track stroke stated as none is no outline (spec §10, rule 2), where material-3's states
/// one; a fill stated as none is transparent.
#[test]
fn a_switch_track_stated_as_none_has_no_outline_and_no_fill() -> TestResult {
    use crate::switch::Look;
    let stock = Look::next_of(&patched(Preset::Material3, "")?).ok_or("stock material-3")?;
    assert!(stock.unchecked_track_border_color.is_some(), "{stock:?}");
    let none =
        "[[widgets.switch.rules]]\nstates = []\ntrack.stroke = \"none\"\ntrack.fill = \"none\"\n";
    let look = Look::next_of(&patched(Preset::Material3, none)?).ok_or("patched material-3")?;
    assert_eq!(look.unchecked_track_border_color, None);
    assert_eq!(look.unchecked_background, egui::Color32::TRANSPARENT);
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::Material3, none)?);
    let track = parts.get("track").ok_or("no track")?;
    let rect = painted_rect(&shapes, track).ok_or("no track painted")?;
    assert_eq!(rect.fill, egui::Color32::TRANSPARENT);
    assert_eq!(rect.stroke, egui::Stroke::NONE);
    Ok(())
}

/// `switch.label_gap`, here a gap kde-breeze does not state, puts the label that far after the
/// track; the label is lettered in `switch.label`'s colour where stated.
#[test]
fn a_switch_label_is_at_the_stated_gap_in_the_stated_colour() -> TestResult {
    let (parts, _) = drawn_switch(&patched_installed(Preset::KdeBreeze, "")?);
    let own = parts.get("label").ok_or("no label")?.left()
        - parts.get("track").ok_or("no track")?.right();
    let patch = "[widgets.switch]\nlabel_gap = 23.0\n\n[[widgets.switch.rules]]\nstates = []\nlabel.color = \"#ff00ff\"\n";
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::KdeBreeze, patch)?);
    let (label, track) = (
        parts.get("label").ok_or("no label")?,
        parts.get("track").ok_or("no track")?,
    );
    assert!(
        own != 23.0,
        "egui's own gap is the stated one: the test proves nothing"
    );
    assert_eq!(label.left() - track.right(), 23.0);
    let text = shapes
        .iter()
        .find_map(|s| match s {
            egui::Shape::Text(t) => Some(t.fallback_color),
            _ => None,
        })
        .ok_or("no label drawn")?;
    assert_eq!(text, egui::Color32::from_rgb(255, 0, 255));
    Ok(())
}

/// The first text drawn: its position and the colour it is lettered in.
fn drawn_text(shapes: &[egui::Shape]) -> Option<(egui::Pos2, egui::Color32)> {
    shapes.iter().find_map(|s| match s {
        egui::Shape::Text(t) => Some((t.pos, t.fallback_color)),
        _ => None,
    })
}

/// `switch.label_side` stated `Leading`, here on kde-breeze, which states `Trailing`, puts the
/// label before the track, `switch.label_gap` before it, and the label is drawn where its part
/// is recorded.
#[test]
fn a_leading_switch_label_lies_before_the_track() -> TestResult {
    use crate::switch::Geometry;
    let stock = Geometry::next_of(&patched(Preset::KdeBreeze, "")?).ok_or("no geometry")?;
    assert!(!stock.leading, "kde-breeze leads: the test proves nothing");
    let gap = stock.label_gap.ok_or("kde-breeze states no label gap")?;
    let patch = "[widgets.switch]\nlabel_side = \"leading\"\n";
    let g = Geometry::next_of(&patched(Preset::KdeBreeze, patch)?).ok_or("no geometry")?;
    assert!(g.leading, "{g:?}");
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::KdeBreeze, patch)?);
    let (label, track) = (
        parts.get("label").ok_or("no label")?,
        parts.get("track").ok_or("no track")?,
    );
    assert!(
        (track.left() - label.right() - gap).abs() < 1e-3,
        "{label:?} {track:?}"
    );
    let (pos, _) = drawn_text(&shapes).ok_or("no label drawn")?;
    assert!((pos - label.min).length() < 1e-3, "{pos:?} {label:?}");
    Ok(())
}

/// The switch's parts, shapes and rect, drawn off, enabled, filling `width` (a
/// `centered_and_justified` layout), in the second of two passes.
fn drawn_switch_in(ctx: &egui::Context, width: f32) -> (Parts, Vec<egui::Shape>, egui::Rect) {
    let add = |ui: &mut egui::Ui| {
        let size = egui::vec2(width, ui.spacing().interact_size.y);
        let layout = egui::Layout::centered_and_justified(egui::Direction::TopDown);
        ui.allocate_ui_with_layout(size, layout, |ui| {
            ui.add(Switch::new(&mut false).label("Wi-Fi"))
        })
        .inner
    };
    let _ = response(ctx, at(1.0), add);
    let (out, r) = response(ctx, at(1.0), add);
    (Parts::of(&r).unwrap_or_default(), flat(&out.shapes), r.rect)
}

/// `switch.label_gap_stretches` stated `true`, here on kde-breeze, which states `false`, puts
/// the width a switch laid out wider than its content has left over between the label and the
/// track, the two at the ends of the row (egui grows the gap by whole points, so the far one
/// within a point of the end); laid out no wider, the two keep the stated gap. With the label
/// leading the label lies at the row's start and the track at its end.
#[test]
fn a_stretching_switch_gap_puts_the_label_and_the_track_at_the_ends() -> TestResult {
    use crate::switch::Geometry;
    let stock = Geometry::next_of(&patched(Preset::KdeBreeze, "")?).ok_or("no geometry")?;
    assert!(
        !stock.stretches,
        "kde-breeze stretches: the test proves nothing"
    );
    let gap = stock.label_gap.ok_or("kde-breeze states no label gap")?;
    let width = 400.0;
    let (parts, _, rect) = drawn_switch_in(&patched_installed(Preset::KdeBreeze, "")?, width);
    assert_eq!(rect.width(), width, "the switch is not laid out wider");
    let label = parts.get("label").ok_or("no label")?;
    assert!(
        rect.right() - label.right() >= 1.0,
        "the label lies at the end unstretched: the test proves nothing"
    );
    let patch = "[widgets.switch]\nlabel_gap_stretches = true\n";
    let g = Geometry::next_of(&patched(Preset::KdeBreeze, patch)?).ok_or("no geometry")?;
    assert!(g.stretches, "{g:?}");
    let ctx = patched_installed(Preset::KdeBreeze, patch)?;
    let (parts, shapes, rect) = drawn_switch_in(&ctx, width);
    let (label, track) = (
        parts.get("label").ok_or("no label")?,
        parts.get("track").ok_or("no track")?,
    );
    assert!(
        (track.left() - rect.left()).abs() < 1e-3,
        "{track:?} {rect:?}"
    );
    let short = rect.right() - label.right();
    assert!((-1e-3..1.0).contains(&short), "{label:?} {rect:?}");
    let (pos, _) = drawn_text(&shapes).ok_or("no label drawn")?;
    assert!((pos - label.min).length() < 1e-3, "{pos:?} {label:?}");
    let (parts, _) = drawn_switch(&ctx);
    let (label, track) = (
        parts.get("label").ok_or("no label")?,
        parts.get("track").ok_or("no track")?,
    );
    assert!(
        (label.left() - track.right() - gap).abs() < 1e-3,
        "{label:?} {track:?}"
    );
    let both = "[widgets.switch]\nlabel_gap_stretches = true\nlabel_side = \"leading\"\n";
    let (parts, shapes, rect) =
        drawn_switch_in(&patched_installed(Preset::KdeBreeze, both)?, width);
    let (label, track) = (
        parts.get("label").ok_or("no label")?,
        parts.get("track").ok_or("no track")?,
    );
    assert!(
        (label.left() - rect.left()).abs() < 1e-3,
        "{label:?} {rect:?}"
    );
    let short = rect.right() - track.right();
    assert!((-1e-3..1.0).contains(&short), "{track:?} {rect:?}");
    let (pos, _) = drawn_text(&shapes).ok_or("no label drawn")?;
    assert!((pos - label.min).length() < 1e-3, "{pos:?} {label:?}");
    Ok(())
}

/// `switch.show_state_glyphs` stated `true` is reported at its home and draws nothing: no
/// bundled preset states it, and the model states no glyph to draw.
#[test]
fn a_switch_stating_state_glyphs_reports_them_unmapped() -> TestResult {
    assert_eq!(
        Switch::next_glyphs_unmapped(&patched(Preset::KdeBreeze, "")?),
        None
    );
    let patch = "[widgets.switch]\nshow_state_glyphs = true\n";
    assert_eq!(
        Switch::next_glyphs_unmapped(&patched(Preset::KdeBreeze, patch)?),
        Some(native_theme_egui::Note::unmapped(
            "switch.show_state_glyphs"
        ))
    );
    let off = "[widgets.switch]\nshow_state_glyphs = false\n";
    assert_eq!(
        Switch::next_glyphs_unmapped(&patched(Preset::KdeBreeze, off)?),
        None
    );
    let (_, stock) = drawn_switch(&patched_installed(Preset::KdeBreeze, "")?);
    let (_, glyphs) = drawn_switch(&patched_installed(Preset::KdeBreeze, patch)?);
    assert_eq!(stock, glyphs);
    Ok(())
}

/// A fill a widget reads that the theme states as layers that are all gradients has no single
/// colour for egui to paint: the widget draws egui's own there, as for a fill the theme leaves
/// unstated, and [`crate::next_fills_unmapped`] reports it at its home (spec §10, rule 4); a
/// switch whose checked track fill is one is egui's own `Checkbox`. The radio's homes are each
/// of its indicator's and dot's at rest, hovered and pressed, checked or not, and disabled, a
/// rule for each state laying the gradients there; the links' are the container's at rest and
/// hovered, the segmented control's its hovered segment's.
#[test]
fn a_widgets_all_gradient_fill_is_reported_at_its_home() -> TestResult {
    use native_theme_egui::Note;
    use native_theme_egui::native_theme::next::{
        Color, GradientStop, LinkRule, Paint, PaintLayer, PartPatch, RadioRule,
        SegmentedControlRule, SliderRule, SwitchRule, ThemePatch, WidgetState,
    };
    let gradient = Paint::layers(vec![PaintLayer::LinearGradient {
        angle_deg: 90.0,
        stops: std::sync::Arc::from(vec![
            GradientStop {
                offset: 0.0,
                color: Color::new(0.1, 0.2, 0.9, 1.0),
            },
            GradientStop {
                offset: 1.0,
                color: Color::TRANSPARENT,
            },
        ]),
        blend: Default::default(),
    }]);
    let fill = PartPatch::new().fill(gradient);
    let (hover, press, on, off) = (
        WidgetState::HOVER,
        WidgetState::PRESSED,
        WidgetState::CHECKED,
        WidgetState::DISABLED,
    );
    let mut patch = ThemePatch::new()
        .switch(SwitchRule::new().states(on).track(fill.clone()))
        .slider(SliderRule::new().thumb(fill.clone()))
        .link(LinkRule::new().container(fill.clone()))
        .link(LinkRule::new().states(hover).container(fill.clone()))
        .segmented_control(
            SegmentedControlRule::new()
                .states(hover)
                .segment(fill.clone()),
        )
        .radio(RadioRule::new().indicator(fill.clone()).dot(fill.clone()));
    for state in [hover, press, on, hover | on, press | on, off, off | on] {
        patch = patch.radio(
            RadioRule::new()
                .states(state)
                .indicator(fill.clone())
                .dot(fill.clone()),
        );
    }
    let stock = NextTheme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    let r = NextTheme::preset(Preset::KdeBreeze)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    assert!(crate::switch::Look::next_of(&stock).is_some());
    assert_eq!(crate::switch::Look::next_of(&r), None);
    assert!(crate::slider::Look::next_of(&stock).thumb_color.is_some());
    assert_eq!(crate::slider::Look::next_of(&r).thumb_color, None);
    assert_eq!(crate::next_fills_unmapped(&stock), Vec::new());
    let notes = crate::next_fills_unmapped(&r);
    for home in [
        "switch.track(CHECKED).fill",
        "slider.thumb(NONE).fill",
        "link.container(NONE).fill",
        "link.container(HOVER).fill",
        "segmented_control.segment(HOVER).fill",
        "radio.indicator(NONE).fill",
        "radio.indicator(HOVER).fill",
        "radio.indicator(PRESSED).fill",
        "radio.indicator(CHECKED).fill",
        "radio.indicator(HOVER|CHECKED).fill",
        "radio.indicator(PRESSED|CHECKED).fill",
        "radio.indicator(DISABLED).fill",
        "radio.indicator(DISABLED|CHECKED).fill",
        "radio.dot(CHECKED).fill",
        "radio.dot(HOVER|CHECKED).fill",
        "radio.dot(PRESSED|CHECKED).fill",
        "radio.dot(DISABLED|CHECKED).fill",
    ] {
        assert!(notes.contains(&Note::unmapped(home)), "{home}: {notes:?}");
    }
    Ok(())
}

/// No bundled preset, light or dark, in an active or an inactive window, states a fill the
/// crate's widgets read in a form egui cannot paint: where one did, the widget would draw
/// egui's own with only [`crate::next_fills_unmapped`]'s note naming the home.
#[test]
fn no_bundled_preset_leaves_a_widgets_fill_unmapped() {
    let mut unmapped = Vec::new();
    for preset in Preset::ALL {
        for (scheme, ctx) in [
            ("light", ThemeContext::light()),
            ("dark", ThemeContext::dark()),
        ] {
            for active in [true, false] {
                let r = NextTheme::preset(*preset).resolve(&ctx.clone().with_window_active(active));
                for note in crate::next_fills_unmapped(&r) {
                    unmapped.push(format!("{preset:?} {scheme}, active {active}: {note:?}"));
                }
            }
        }
    }
    assert!(unmapped.is_empty(), "{unmapped:#?}");
}

/// A hovered and a pressed switch letter the label in `switch.label(HOVER)`'s and
/// `switch.label(PRESSED)`'s colour where a rule for the state states one, here colours
/// kde-breeze does not state; at rest it keeps its own.
#[test]
fn a_hovered_and_a_pressed_switch_label_take_their_state_colours() -> TestResult {
    use crate::switch::Look;
    let (red, blue) = (
        egui::Color32::from_rgb(255, 0, 0),
        egui::Color32::from_rgb(0, 0, 255),
    );
    let stock = Look::next_of(&patched(Preset::KdeBreeze, "")?).ok_or("no look")?;
    for stated in [stock.hover_text, stock.pressed_text] {
        assert!(
            stated.is_none_or(|c| c != red && c != blue),
            "kde-breeze states the patched colours: the test proves nothing"
        );
    }
    let patch = "[[widgets.switch.rules]]\nstates = [\"hover\"]\nlabel.color = \"#ff0000\"\n\n\
                 [[widgets.switch.rules]]\nstates = [\"pressed\"]\nlabel.color = \"#0000ff\"\n";
    let look = Look::next_of(&patched(Preset::KdeBreeze, patch)?).ok_or("no look")?;
    assert_eq!(
        (look.hover_text, look.pressed_text),
        (Some(red), Some(blue))
    );
    let ctx = patched_installed(Preset::KdeBreeze, patch)?;
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut false).label("Wi-Fi"));
    let (parts, shapes) = drawn_switch(&ctx);
    let (_, rest) = drawn_text(&shapes).ok_or("no label drawn")?;
    assert!(rest != red && rest != blue, "{rest:?}");
    let centre = parts.get("track").ok_or("no track")?.center();
    let hover = egui::RawInput {
        events: vec![egui::Event::PointerMoved(centre)],
        ..at(1.0)
    };
    let _ = response(&ctx, hover, add);
    let (out, r) = response(&ctx, at(1.0), add);
    assert!(r.hovered(), "the switch is not hovered");
    assert_eq!(drawn_text(&flat(&out.shapes)).map(|(_, c)| c), Some(red));
    let press = egui::RawInput {
        time: Some(1.0),
        ..pointer(centre, true)
    };
    let _ = response(&ctx, press, add);
    let (out, r) = response(&ctx, at(1.0), add);
    assert!(r.is_pointer_button_down_on(), "the switch is not pressed");
    assert_eq!(drawn_text(&flat(&out.shapes)).map(|(_, c)| c), Some(blue));
    Ok(())
}

/// A hovered switch whose label no rule states for `HOVER` letters it in egui's own hovered
/// label colour, its scope's `widgets.hovered.fg_stroke` (spec §10, rule 0), as egui's
/// `Checkbox` letters an uncoloured label, not in its rest colour; here on ios-13, which states
/// no hovered switch label.
#[test]
fn an_unstated_hovered_switch_label_is_egui_s_own() -> TestResult {
    use crate::switch::Look;
    let look = Look::next_of(&patched(Preset::Ios13, "")?).ok_or("no look")?;
    assert_eq!(
        look.hover_text, None,
        "ios-13 states a hovered switch label"
    );
    let ctx = patched_installed(Preset::Ios13, "")?;
    let mut own = None;
    let _ = response(&ctx, at(1.0), |ui| {
        ui.native_scope(Role::Switch, RoleVariant::Normal, |ui| {
            own = Some(ui.visuals().widgets.hovered.text_color());
            ui.label("")
        })
        .inner
    });
    let own = own.ok_or("the scope was not shown")?;
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut false).label("Wi-Fi"));
    let (parts, shapes) = drawn_switch(&ctx);
    let (_, rest) = drawn_text(&shapes).ok_or("no label drawn")?;
    assert_ne!(
        rest, own,
        "the label at rest is egui's hovered colour: the test proves nothing"
    );
    let centre = parts.get("track").ok_or("no track")?.center();
    let hover = egui::RawInput {
        events: vec![egui::Event::PointerMoved(centre)],
        ..at(1.0)
    };
    let _ = response(&ctx, hover, add);
    let (out, r) = response(&ctx, at(1.0), add);
    assert!(r.hovered(), "the switch is not hovered");
    assert_eq!(drawn_text(&flat(&out.shapes)).map(|(_, c)| c), Some(own));
    Ok(())
}

/// A pressed switch whose label no rule states for `PRESSED` letters it in egui's own pressed
/// label colour, its scope's `widgets.active.fg_stroke` (spec §10, rule 0), as egui's
/// `Checkbox` letters an uncoloured label, not in its rest colour; here on ios-13, which states
/// no pressed switch label, its label at rest patched red, as its own is egui's pressed colour.
#[test]
fn an_unstated_pressed_switch_label_is_egui_s_own() -> TestResult {
    use crate::switch::Look;
    let red = egui::Color32::from_rgb(255, 0, 0);
    let patch = "[[widgets.switch.rules]]\nlabel.color = \"#ff0000\"\n";
    let look = Look::next_of(&patched(Preset::Ios13, patch)?).ok_or("no look")?;
    assert_eq!(
        (look.text, look.pressed_text),
        (Some(red), None),
        "the patched ios-13's rest and pressed switch labels"
    );
    let ctx = patched_installed(Preset::Ios13, patch)?;
    let mut own = None;
    let _ = response(&ctx, at(1.0), |ui| {
        ui.native_scope(Role::Switch, RoleVariant::Normal, |ui| {
            own = Some(ui.visuals().widgets.active.text_color());
            ui.label("")
        })
        .inner
    });
    let own = own.ok_or("the scope was not shown")?;
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut false).label("Wi-Fi"));
    let (parts, shapes) = drawn_switch(&ctx);
    let (_, rest) = drawn_text(&shapes).ok_or("no label drawn")?;
    assert_ne!(
        rest, own,
        "the label at rest is egui's pressed colour: the test proves nothing"
    );
    let centre = parts.get("track").ok_or("no track")?.center();
    let press = egui::RawInput {
        time: Some(1.0),
        ..pointer(centre, true)
    };
    let _ = response(&ctx, press, add);
    let (out, r) = response(&ctx, at(1.0), add);
    assert!(r.is_pointer_button_down_on(), "the switch is not pressed");
    assert_eq!(drawn_text(&flat(&out.shapes)).map(|(_, c)| c), Some(own));
    Ok(())
}

/// `switch.label(NONE).size_px` letters the label at that size in egui's `Proportional` family,
/// as the connector's role fonts are, here a size kde-breeze does not state; where it is
/// unstated the size is egui's own, read of an unstated size directly, as every platform preset
/// states one ([`Geometry::label_size`]); a size that is not positive sends the switch to egui's
/// own counterpart.
#[test]
fn a_switch_label_takes_its_stated_size() -> TestResult {
    use crate::switch::Geometry;
    let font = |shapes: &[egui::Shape]| {
        shapes.iter().find_map(|s| match s {
            egui::Shape::Text(t) => t
                .galley
                .job
                .sections
                .first()
                .map(|section| section.format.font_id.clone()),
            _ => None,
        })
    };
    assert_eq!(Geometry::label_size(None), Some(None));
    let stock = Geometry::next_of(&patched(Preset::KdeBreeze, "")?).ok_or("no geometry")?;
    assert!(
        stock.label_size.is_some_and(|size| size != 31.0),
        "{stock:?}"
    );
    let (_, shapes) = drawn_switch(&patched_installed(Preset::KdeBreeze, "")?);
    let own = font(&shapes).ok_or("no label drawn")?;
    assert!(
        own.size != 31.0,
        "egui's own size is the stated one: the test proves nothing"
    );
    let patch = "[[widgets.switch.rules]]\nstates = []\nlabel.size_px = 31.0\n";
    let g = Geometry::next_of(&patched(Preset::KdeBreeze, patch)?).ok_or("no geometry")?;
    assert_eq!(g.label_size, Some(31.0));
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::KdeBreeze, patch)?);
    assert_eq!(
        font(&shapes).ok_or("no label drawn")?,
        egui::FontId::proportional(31.0)
    );
    let label = parts.get("label").ok_or("no label")?;
    assert!(label.height() >= 31.0, "{label:?}");
    let zero = "[[widgets.switch.rules]]\nstates = []\nlabel.size_px = 0.0\n";
    assert_eq!(Geometry::next_of(&patched(Preset::KdeBreeze, zero)?), None);
    Ok(())
}

/// A switch label whose colours the theme leaves unstated is lettered in the body text's at rest
/// and in the theme's disabled text disabled, a stated one in its own; read of unstated colours
/// directly, as every platform preset states the label's colour at rest and disabled
/// ([`Look::label_colors`]); a stated colour is drawn on a preset at rest in
/// `a_switch_label_is_at_the_stated_gap_in_the_stated_colour`, disabled in
/// `the_switch_draws_alike_from_both_models` (kde-breeze's listed disabled label).
///
/// [`Look::label_colors`]: crate::switch::Look::label_colors
#[test]
fn an_unstated_switch_label_is_lettered_in_the_body_and_the_disabled_text() {
    use crate::switch::Look;
    let [stated, stated_disabled, body, disabled] = [
        egui::Color32::from_rgb(255, 0, 0),
        egui::Color32::from_rgb(0, 255, 0),
        egui::Color32::from_rgb(0, 0, 255),
        egui::Color32::from_rgb(255, 255, 0),
    ];
    assert_eq!(
        Look::label_colors(None, None, Some(body), disabled),
        (Some(body), disabled)
    );
    assert_eq!(
        Look::label_colors(None, None, None, disabled),
        (None, disabled)
    );
    assert_eq!(
        Look::label_colors(Some(stated), Some(stated_disabled), Some(body), disabled),
        (Some(stated), stated_disabled)
    );
}

/// A checked switch reads its own states: a disabled checked track and thumb at
/// `DISABLED|CHECKED`, from a rule for that state alone, and a checked thumb's outline at
/// `thumb(CHECKED)`, drawn so.
#[test]
fn a_checked_switch_reads_its_checked_disabled_and_outlined_thumb() -> TestResult {
    use crate::switch::{Geometry, Look};
    let (red, green, blue) = (
        egui::Color32::from_rgb(255, 0, 0),
        egui::Color32::from_rgb(0, 255, 0),
        egui::Color32::from_rgb(0, 0, 255),
    );
    let patch = "[[widgets.switch.rules]]\nstates = [\"disabled\", \"checked\"]\n\
                 track.fill = \"#0000ff\"\nthumb.fill = \"#ff0000\"\n\n\
                 [[widgets.switch.rules]]\nstates = [\"checked\"]\n\
                 thumb.stroke = { width = 2.0, color = \"#00ff00\" }\n";
    let r = patched(Preset::KdeBreeze, patch)?;
    let look = Look::next_of(&r).ok_or("no look")?;
    assert_eq!(look.disabled_checked_background, Some(blue));
    assert_eq!(look.disabled_checked_thumb_color, Some(red));
    assert!(look.disabled_thumb_color != Some(red), "{look:?}");
    assert_eq!(look.checked_thumb_border_color, Some(green));
    assert!(look.thumb_border_color != Some(green), "{look:?}");
    let g = Geometry::next_of(&r).ok_or("no geometry")?;
    assert_eq!(g.checked_thumb_border, Some(2.0));
    let ctx = patched_installed(Preset::KdeBreeze, patch)?;
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut true));
    let _ = response(&ctx, at(1.0), add);
    let (out, _) = response(&ctx, at(1.0), add);
    let thumb = flat(&out.shapes)
        .into_iter()
        .find_map(|s| match s {
            egui::Shape::Circle(c) => Some(c),
            _ => None,
        })
        .ok_or("no thumb drawn")?;
    assert_eq!(
        thumb.stroke,
        egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 0))
    );
    Ok(())
}

/// A hovered track's stated fill is the whole fill it shows, taken as it is (spec §9, step 2),
/// not laid over its rest fill: windows-11's hovered on track is WinUI's
/// AccentFillColorSecondary, the accent at 0.9 opacity, `#0067c0e6` in light
/// (native-theme/tests/next_homes.toml, row `switch.hover_checked_background`).
#[test]
fn a_hovered_switch_track_is_its_stated_whole_fill() -> TestResult {
    let ctx = patched_installed(Preset::Windows11, "")?;
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut true).label("Wi-Fi"));
    let _ = response(&ctx, at(1.0), add);
    let (out, r) = response(&ctx, at(1.0), add);
    let track = Parts::of(&r)
        .unwrap_or_default()
        .get("track")
        .ok_or("no track")?;
    let rest = painted_rect(&flat(&out.shapes), track).ok_or("no track painted")?;
    let hover = egui::RawInput {
        events: vec![egui::Event::PointerMoved(track.center())],
        ..at(1.0)
    };
    let _ = response(&ctx, hover, add);
    let (out, r) = response(&ctx, at(1.0), add);
    assert!(r.hovered(), "the switch is not hovered");
    let hovered = painted_rect(&flat(&out.shapes), track).ok_or("no hovered track painted")?;
    let stated = egui::Color32::from_rgba_unmultiplied(0x00, 0x67, 0xc0, 0xe6);
    assert_ne!(
        rest.fill, stated,
        "the rest fill is the hovered one: the test proves nothing"
    );
    assert_eq!(hovered.fill, stated);
    Ok(())
}

/// A track with pill corners is rounded by half its shorter side.
#[test]
fn a_pill_switch_track_is_rounded_by_half_its_height() -> TestResult {
    use crate::switch::Geometry;
    let patch = "[[widgets.switch.rules]]\nstates = []\ntrack.corners = \"pill\"\ntrack.min_size = [40.0, 30.0]\n";
    let g = Geometry::next_of(&patched(Preset::Windows11, patch)?).ok_or("no geometry")?;
    assert_eq!((g.track, g.radius), (egui::vec2(40.0, 30.0), 15.0));
    Ok(())
}

/// The radio's label lies `radio.label_gap` after its circle, where the stated gap is not its
/// cell's own.
#[test]
fn a_radio_label_is_at_the_stated_gap() -> TestResult {
    let label_x = |ctx: &egui::Context| -> Option<(f32, f32)> {
        let add = |ui: &mut egui::Ui| ui.add(RadioButton::new(true, "Radio"));
        let _ = response(ctx, at(1.0), add);
        let (out, r) = response(ctx, at(1.0), add);
        let circle = Parts::of(&r)?.get("indicator")?;
        let text = flat(&out.shapes).into_iter().find_map(|s| match s {
            egui::Shape::Text(t) => Some(t.pos.x),
            _ => None,
        })?;
        Some((circle.right(), text))
    };
    let (right, own) = label_x(&patched_installed(Preset::KdeBreeze, "")?).ok_or("stock")?;
    assert!(
        own - right != 23.0,
        "the stated gap is the cell's: the test proves nothing"
    );
    let patch = "[widgets.radio]\nlabel_gap = 23.0\n";
    let (right, text) = label_x(&patched_installed(Preset::KdeBreeze, patch)?).ok_or("patched")?;
    assert_eq!(text - right, 23.0);
    Ok(())
}

/// The radio is drawn from its own homes, not from the check box's cell: a theme that styles
/// the radio unlike the check box draws the radio's circle, outline, dot and label as stated.
#[test]
fn a_radio_styled_unlike_the_check_box_is_drawn_as_stated() -> TestResult {
    let drawn = |ctx: &egui::Context| {
        let add = |ui: &mut egui::Ui| ui.add(RadioButton::new(true, "Radio"));
        let _ = response(ctx, at(1.0), add);
        let (out, _) = response(ctx, at(1.0), add);
        let shapes = flat(&out.shapes);
        let circles: Vec<egui::epaint::CircleShape> = shapes
            .iter()
            .filter_map(|s| match s {
                egui::Shape::Circle(c) => Some(*c),
                _ => None,
            })
            .collect();
        let text = shapes.iter().find_map(|s| match s {
            egui::Shape::Text(t) => Some(
                t.galley
                    .job
                    .sections
                    .first()
                    .map(|s| s.format.color)
                    .filter(|&c| c != egui::Color32::PLACEHOLDER)
                    .unwrap_or(t.fallback_color),
            ),
            _ => None,
        });
        (circles, text)
    };
    let (green, red, blue, magenta) = (
        egui::Color32::from_rgb(0, 255, 0),
        egui::Color32::from_rgb(255, 0, 0),
        egui::Color32::from_rgb(0, 0, 255),
        egui::Color32::from_rgb(255, 0, 255),
    );
    let (stock, stock_text) = drawn(&patched_installed(Preset::KdeBreeze, "")?);
    let (circle, dot) = (
        stock.first().ok_or("no circle")?,
        stock.get(1).ok_or("no dot")?,
    );
    assert!(
        circle.fill != green
            && circle.stroke != egui::Stroke::new(2.0, red)
            && dot.fill != blue
            && stock_text != Some(magenta),
        "the check box's cell is the stated radio: the test proves nothing"
    );
    let patch = "[[widgets.radio.rules]]\nstates = []\nindicator.fill = \"#00ff00\"\n\
                 indicator.stroke = { width = 2.0, color = \"#ff0000\" }\n\
                 dot.fill = \"#0000ff\"\nlabel.color = \"#ff00ff\"\n";
    let (patched, text) = drawn(&patched_installed(Preset::KdeBreeze, patch)?);
    let (circle, dot) = (
        patched.first().ok_or("no circle")?,
        patched.get(1).ok_or("no dot")?,
    );
    assert_eq!(circle.fill, green);
    assert_eq!(circle.stroke, egui::Stroke::new(2.0, red));
    assert_eq!(dot.fill, blue);
    assert_eq!(text, Some(magenta));
    Ok(())
}

/// `radio.disabled_opacity` is the fade of a disabled radio as one widget over its backdrop:
/// its circle, in the stated disabled fill, is faded over the backdrop by the stated opacity.
#[test]
fn a_disabled_radio_is_faded_by_its_stated_opacity() -> TestResult {
    let ground = egui::Color32::from_rgb(0, 0, 0);
    let green = egui::Color32::from_rgb(0, 255, 0);
    let circle = |opacity: &str| -> Result<egui::Color32, Box<dyn std::error::Error>> {
        let patch = format!(
            "[widgets.radio]\ndisabled_opacity = {opacity}\n\n\
             [[widgets.radio.rules]]\nstates = [\"disabled\"]\nindicator.fill = \"#00ff00\"\n"
        );
        let ctx = patched_installed(Preset::KdeBreeze, &patch)?;
        let add = |ui: &mut egui::Ui| {
            ui.add(
                RadioButton::new(false, "Radio")
                    .enabled(false)
                    .backdrop(ground),
            )
        };
        let _ = response(&ctx, at(1.0), add);
        let (out, _) = response(&ctx, at(1.0), add);
        Ok(flat(&out.shapes)
            .into_iter()
            .find_map(|s| match s {
                egui::Shape::Circle(c) => Some(c.fill),
                _ => None,
            })
            .ok_or("no circle drawn")?)
    };
    let (quarter, half) = (circle("0.25")?, circle("0.5")?);
    assert_eq!(quarter, crate::fade::faded(ground, green, 0.25));
    assert_eq!(half, crate::fade::faded(ground, green, 0.5));
    assert_ne!(quarter, half);
    Ok(())
}

/// A progress bar's track stroke stated as none is no outline, on every preset whose bar has
/// one; its pill corners are half its stated height, its continuous ones the circular corner of
/// their radius.
#[test]
fn a_progress_bar_track_stated_as_none_has_no_outline() -> TestResult {
    use crate::progress_bar::Bar;
    let none = "[[widgets.progress_bar.rules]]\nstates = []\ntrack.stroke = \"none\"\n";
    let mut outlined = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Bar::next_of(&patched(preset, "")?);
        if stock.outline.is_none() {
            continue;
        }
        outlined += 1;
        let bar = Bar::next_of(&patched(preset, none)?);
        assert_eq!(bar.outline, None, "{preset:?}");
        assert_eq!(bar.radius, stock.radius, "{preset:?}");
    }
    assert!(
        outlined > 0,
        "no preset outlines its bar: the test proves nothing"
    );
    let pill = "[[widgets.progress_bar.rules]]\nstates = []\ntrack.corners = \"pill\"\n\
                track.min_size = [100.0, 6.0]\n";
    let bar = Bar::next_of(&patched(Preset::KdeBreeze, pill)?);
    assert_eq!(bar.radius, Some(3.0));
    assert_eq!(bar.min_width, Some(100.0));
    let continuous = "[[widgets.progress_bar.rules]]\nstates = []\n\
                      track.corners = { radius = 8.0, curve = \"continuous\" }\n";
    let bar = Bar::next_of(&patched(Preset::KdeBreeze, continuous)?);
    assert_eq!(
        bar.radius,
        Some(8.0),
        "a continuous curve is drawn circular"
    );
    Ok(())
}

/// A progress bar whose corners egui cannot draw as one radius (radii that differ corner by
/// corner) keeps egui's own, half its height, and is still outlined as stated (spec §10,
/// rule 0, per property).
#[test]
fn a_progress_bar_outline_stays_where_its_corners_are_egui_s() -> TestResult {
    use crate::progress_bar::Bar;
    let patch = "[[widgets.progress_bar.rules]]\nstates = []\n\
                 track.corners = { radius = { top_left = 1, top_right = 2, bottom_right = 3, \
                 bottom_left = 4 } }\n\
                 track.stroke = { width = 2.0, color = \"#ff00ff\" }\n";
    let magenta = egui::Color32::from_rgb(255, 0, 255);
    let bar = Bar::next_of(&patched(Preset::KdeBreeze, patch)?);
    assert_eq!(bar.radius, None);
    assert_eq!(bar.outline, Some((2.0, magenta)));
    let ctx = patched_installed(Preset::KdeBreeze, patch)?;
    let add = |ui: &mut egui::Ui| ui.add(ProgressBar::new(0.4).desired_width(200.0));
    let _ = response(&ctx, at(1.0), add);
    let (out, r) = response(&ctx, at(1.0), add);
    let outline = flat(&out.shapes)
        .into_iter()
        .find_map(|s| match s {
            egui::Shape::Rect(rect) if rect.stroke.color == magenta => Some(rect),
            _ => None,
        })
        .ok_or("no outline drawn")?;
    assert_eq!(outline.rect, r.rect);
    assert_eq!(outline.stroke.width, 2.0);
    assert_eq!(
        outline.corner_radius,
        egui::CornerRadius::from(0.5 * r.rect.height())
    );
    Ok(())
}

/// A link's stated underline underlines it at rest, and a hover fill a rule for `HOVER`
/// states is the layer laid over its background under the pointer.
#[test]
fn a_link_takes_its_stated_underline_and_hover_layer() -> TestResult {
    use crate::links::Look;
    let underlined = |ctx: &egui::Context| -> Option<f32> {
        let add = |ui: &mut egui::Ui| {
            let w = links::link(ui, "Link");
            ui.add(w)
        };
        let _ = response(ctx, at(1.0), add);
        let (out, _) = response(ctx, at(1.0), add);
        flat(&out.shapes).into_iter().find_map(|s| match s {
            egui::Shape::Text(t) => t
                .galley
                .job
                .sections
                .first()
                .map(|s| s.format.underline.width),
            _ => None,
        })
    };
    // Breeze underlines its links at rest; the patch states the opposite.
    let magenta = egui::Color32::from_rgb(255, 0, 255);
    let stock = Look::next_of(&patched(Preset::KdeBreeze, "")?);
    assert_eq!(stock.underline_enabled, Some(true), "{stock:?}");
    assert_ne!(stock.hover_background, Some(magenta), "{stock:?}");
    let width = underlined(&patched_installed(Preset::KdeBreeze, "")?).ok_or("no text")?;
    assert!(width > 0.0, "the stock link is not underlined: {width}");
    let patch = "[[widgets.link.rules]]\nstates = []\nlabel.decoration = { kind = \"none\" }\n\n\
                 [[widgets.link.rules]]\nstates = [\"hover\"]\ncontainer.fill = \"#ff00ff\"\n";
    let look = Look::next_of(&patched(Preset::KdeBreeze, patch)?);
    assert_eq!(look.underline_enabled, Some(false));
    assert_eq!(look.hover_background, Some(magenta));
    assert_eq!(
        underlined(&patched_installed(Preset::KdeBreeze, patch)?),
        Some(0.0)
    );
    Ok(())
}

/// A slider thumb's stroke stated as none is no outline (spec §10, rule 2): a width of 0, so the
/// knob is not given egui's own `fg_stroke`, on every preset whose thumb has one.
#[test]
fn a_slider_thumb_stated_as_none_has_no_outline() -> TestResult {
    use crate::slider::{Geometry, Look};
    let none = "[[widgets.slider.rules]]\nstates = []\nthumb.stroke = \"none\"\n";
    let mut outlined = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = patched(preset, "")?;
        if Geometry::next_of(&stock, 1.0)
            .and_then(|g| g.outline)
            .is_none_or(|w| w == 0.0)
        {
            continue;
        }
        outlined += 1;
        let r = patched(preset, none)?;
        assert_eq!(
            Geometry::next_of(&r, 1.0).ok_or("no geometry")?.outline,
            Some(0.0),
            "{preset:?}"
        );
        let look = Look::next_of(&r);
        assert_eq!(look.thumb_border_color, None, "{preset:?}");
        let ctx = patched_installed(preset, none)?;
        let add = |ui: &mut egui::Ui| ui.add(Slider::new(&mut 0.5, 0.0..=1.0));
        let _ = response(&ctx, at(1.0), add);
        let (out, _) = response(&ctx, at(1.0), add);
        let knob = flat(&out.shapes)
            .into_iter()
            .find_map(|s| match s {
                egui::Shape::Circle(c) => Some(c),
                _ => None,
            })
            .ok_or("no knob drawn")?;
        assert_eq!(knob.stroke, egui::Stroke::NONE, "{preset:?}");
    }
    assert!(
        outlined > 0,
        "no preset outlines its thumb: the test proves nothing"
    );
    Ok(())
}

/// A slider's rail, trailing and knob fills stated as a gradient, which egui cannot paint as one
/// colour, are each egui's own: the rail and the resting knob in the scope's `inactive.bg_fill`,
/// the trailing fill only where `slider_trailing_fill` is on; the stated sizes stay (spec §10,
/// rule 0, per property).
#[test]
fn a_slider_fill_egui_cannot_paint_is_egui_s_own() -> TestResult {
    use crate::slider::{Geometry, Look};
    let gradient = "{ kind = \"linear_gradient\", angle_deg = 0.0, stops = [{ offset = 0.0, color = \"#ff0000\" }, { offset = 1.0, color = \"#0000ff\" }] }";
    let patch = format!(
        "[[widgets.slider.rules]]\nstates = []\ntrack.fill = {gradient}\n\
         active_track.fill = {gradient}\nthumb.fill = {gradient}\n"
    );
    let r = patched(Preset::KdeBreeze, &patch)?;
    let look = Look::next_of(&r);
    assert_eq!(
        (look.track_color, look.fill_color, look.thumb_color),
        (None, None, None)
    );
    let g = Geometry::next_of(&r, 1.0).ok_or("no geometry")?;
    assert!(g.thumb.is_some() && g.rail.is_some(), "{g:?}");
    let ctx = patched_installed(Preset::KdeBreeze, &patch)?;
    let own = std::cell::Cell::new(None);
    let add = |ui: &mut egui::Ui| {
        ui.native_scope(Role::Slider, RoleVariant::Normal, |ui| {
            let v = ui.visuals();
            own.set(Some((v.widgets.inactive.bg_fill, v.slider_trailing_fill)));
        });
        ui.add(Slider::new(&mut 0.5, 0.0..=1.0))
    };
    let _ = response(&ctx, at(1.0), add);
    let (out, r) = response(&ctx, at(1.0), add);
    let (rest, trailing) = own.get().ok_or("no scope")?;
    let parts = Parts::of(&r).ok_or("no parts")?;
    let shapes = flat(&out.shapes);
    let rail = painted_rect(&shapes, parts.get("track").ok_or("no rail")?).ok_or("no rail")?;
    assert_eq!(rail.fill, rest);
    let fill = painted_rect(&shapes, parts.get("fill").ok_or("no fill")?);
    assert_eq!(fill.is_some(), trailing, "{fill:?}");
    let knob = shapes
        .iter()
        .find_map(|s| match s {
            egui::Shape::Circle(c) => Some(c.fill),
            _ => None,
        })
        .ok_or("no knob drawn")?;
    assert_eq!(knob, rest);
    Ok(())
}

/// A segmented control's stated vertical padding keeps its stated height, and a hover fill a
/// rule for `HOVER` states is its hover layer.
#[test]
fn a_segmented_control_takes_its_stated_padding_and_hover_layer() -> TestResult {
    use crate::segmented_control::Look;
    let mut unpadded = None;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Look::next_of(&patched(preset, "")?);
        if !stock.padded && stock.hover.is_none() {
            unpadded = Some(preset);
        }
    }
    let preset = unpadded.ok_or("every preset states the padding or a hover layer")?;
    let patch = "[[widgets.segmented_control.rules]]\nstates = []\nsegment.padding = { top = 3.0, right = 0.0, bottom = 3.0, left = 0.0 }\n\n\
                 [[widgets.segmented_control.rules]]\nstates = [\"hover\"]\nsegment.fill = \"#ff00ff\"\n";
    let look = Look::next_of(&patched(preset, patch)?);
    assert!(look.padded, "{preset:?}");
    assert_eq!(
        look.hover,
        Some(egui::Color32::from_rgb(255, 0, 255)),
        "{preset:?}"
    );
    Ok(())
}

/// A segmented control paints its dividers in the colour `segmented_control.separator(NONE)`'s
/// fill states, not in its outline's.
#[test]
fn a_segmented_control_paints_its_dividers_in_the_separator_fill() -> TestResult {
    use crate::segmented_control::Look;
    let magenta = egui::Color32::from_rgb(255, 0, 255);
    let patch = "[[widgets.segmented_control.rules]]\nstates = []\nseparator.fill = \"#ff00ff\"\n";
    // adwaita: its dividers are a point wide (kde-breeze's are none).
    assert_eq!(
        Look::next_of(&patched(Preset::Adwaita, patch)?).separator,
        Some(magenta)
    );
    let ctx = patched_installed(Preset::Adwaita, patch)?;
    let add = |ui: &mut egui::Ui| ui.add(SegmentedControl::new(&mut 1, ["Day", "Week"]));
    let _ = response(&ctx, at(1.0), add);
    let (out, r) = response(&ctx, at(1.0), add);
    let parts = Parts::of(&r).ok_or("no parts")?;
    let divider = parts
        .get("divider_0")
        .ok_or("no divider between the two segments")?;
    let painted = out.shapes.iter().any(|clipped| match &clipped.shape {
        egui::Shape::Rect(s) => s.fill == magenta && s.rect.intersects(divider),
        _ => false,
    });
    assert!(
        painted,
        "no divider painted in the separator fill at {divider:?}"
    );
    Ok(())
}

/// A drop-down field stroked per side and not reduced, which egui cannot draw as one outline,
/// keeps egui's border width and the theme's stated padding and arrow column (spec §10, rule
/// 0, per property).
#[test]
fn a_combo_box_field_stroked_per_side_keeps_its_stated_sides() -> TestResult {
    use crate::combo_box::Sides;
    let per_side = "[[widgets.combo_box.rules]]\nstates = []\nfield.stroke = { top = { width = 1.0, paint = \"#ff0000\" }, right = { width = 0.0, paint = \"none\" }, bottom = { width = 2.0, paint = \"#00ff00\" }, left = { width = 0.0, paint = \"none\" } }\n";
    let mut padded = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Sides::next_of(&patched(preset, "")?).ok_or("no stock sides")?;
        if stock.left.is_none() && stock.right.is_none() && stock.column.is_none() {
            continue;
        }
        padded += 1;
        let sides = Sides::next_of(&patched(preset, per_side)?).ok_or("no sides")?;
        assert_eq!(sides.border, None, "{preset:?}");
        assert_eq!(
            (sides.left, sides.right, sides.column),
            (stock.left, stock.right, stock.column),
            "{preset:?}"
        );
    }
    assert!(
        padded > 0,
        "no preset states the field's sides: the test proves nothing"
    );
    Ok(())
}

/// A drop-down field's stroke stated as none is no border: its text is inset by the padding
/// alone.
#[test]
fn a_combo_box_field_stated_as_none_has_no_border() -> TestResult {
    use crate::combo_box::Sides;
    let none = "[[widgets.combo_box.rules]]\nstates = []\nfield.stroke = \"none\"\n";
    let mut bordered = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Sides::next_of(&patched(preset, "")?);
        if !stock.is_some_and(|s| s.border.is_some_and(|w| w > 0.0)) {
            continue;
        }
        bordered += 1;
        let sides = Sides::next_of(&patched(preset, none)?).ok_or("no sides")?;
        assert_eq!(sides.border, Some(0.0), "{preset:?}");
        assert_eq!(
            (sides.left, sides.right),
            stock.map(|s| (s.left, s.right)).ok_or("no stock sides")?,
            "{preset:?}"
        );
    }
    assert!(
        bordered > 0,
        "no preset borders its field: the test proves nothing"
    );
    Ok(())
}

/// A context drawn light with the atlas of `preset` installed, its themes resolved for the
/// text scale `scale`, with no font plan.
fn scaled_installed(
    preset: Preset,
    scale: f32,
) -> Result<egui::Context, Box<dyn std::error::Error>> {
    patched_scaled_installed(preset, "", scale)
}

/// [`scaled_installed`] with `patch` applied to the preset.
fn patched_scaled_installed(
    preset: Preset,
    patch: &str,
    scale: f32,
) -> Result<egui::Context, Box<dyn std::error::Error>> {
    let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
    let theme = NextTheme::preset(preset).with_patch(patch);
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx.with_text_scale(scale)));
    let ctx = egui::Context::default();
    ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive)
        .build()?
        .install(&ctx);
    ctx.set_theme(egui::Theme::Light);
    Ok(ctx)
}

/// The expander's header response and parts, drawn open, in the second of two passes.
fn drawn_expander(ctx: &egui::Context) -> (egui::FullOutput, egui::Response, Parts) {
    let add = |ui: &mut egui::Ui| {
        Expander::new("Details")
            .default_open(true)
            .show(ui, |ui| ui.label("Body"))
            .header_response
    };
    let _ = response(ctx, at(1.0), add);
    let (out, r) = response(ctx, at(1.0), add);
    let parts = Parts::of(&r).unwrap_or_default();
    (out, r, parts)
}

/// Above a text scale of 1 the header is the row rule's height of the 0.8.0 theme, on the
/// painted header (kde-breeze, adwaita) and on egui's own (material-3): the stated
/// `expander.header(NONE)` height grown to the line box of `expander.header_label(NONE)`'s size
/// times `typography.line_height_multiplier`, rounded up, and the padding above and below it.
/// That size holds the text scale already, so a header that scaled it a second time would be
/// taller; it is not 0.7.2's scaled size: kde-breeze's is rounded after scaling (spec §6,
/// step 2), and material-3's follows Android's non-linear font scaling (`FontScaleConverter`,
/// 14 sp to 26 dp at a scale of 2.0) where 0.7.2 multiplied.
#[test]
fn the_expander_header_grows_by_its_line_box_once() -> TestResult {
    use crate::expander::Layout;
    use native_theme_egui::native_theme::next::WidgetState;
    let mut faults = Vec::new();
    let mut grown = 0;
    for preset in [Preset::KdeBreeze, Preset::Adwaita, Preset::Material3] {
        let at_one = drawn_expander(&scaled_installed(preset, 1.0)?)
            .1
            .rect
            .height();
        for scale in [1.5, 2.0] {
            let r =
                NextTheme::preset(preset).resolve(&ThemeContext::light().with_text_scale(scale));
            let layout = Layout::next_of(&r);
            let title = r.expander.header_label(WidgetState::NONE);
            let line = (title.size_px * r.typography.line_height_multiplier).ceil();
            let [_, stated] = r.expander.header(WidgetState::NONE).min_size;
            let stated = stated.ok_or("no stated header height")?;
            let ctx = scaled_installed(preset, scale)?;
            let mut own = 0.0;
            let _ = pass(&ctx, at(1.0), |ui| {
                own = ui
                    .native_scope(Role::Expander, RoleVariant::Normal, |ui| {
                        ui.spacing().button_padding.y
                    })
                    .inner;
            });
            let padding = if layout.lays_out() {
                layout.padding.top.unwrap_or(own) + layout.padding.bottom.unwrap_or(own)
            } else {
                own + own
            };
            let want = stated.max(line + padding);
            let got = drawn_expander(&ctx).1.rect.height();
            if layout.line != Some(line) || got != want {
                faults.push(format!(
                    "{preset:?} at {scale}: the header is {got}, its line box {:?}; want {want}, \
                     {line}",
                    layout.line
                ));
            }
            if got > at_one {
                grown += 1;
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
    assert!(
        grown > 0,
        "no header grows above a text scale of 1: the test proves nothing"
    );
    Ok(())
}

/// A line height stated on `expander.header_label(NONE)` is the title's line box above a text
/// scale of 1, on the painted header (kde-breeze, adwaita) and on egui's own (material-3), in
/// place of its size times `typography.line_height_multiplier`: the header is the stated height
/// grown to that line box, rounded up, and the padding above and below it.
#[test]
fn the_expander_header_grows_by_its_stated_line_height() -> TestResult {
    use crate::expander::Layout;
    use native_theme_egui::native_theme::next::WidgetState;
    let patch = "[[widgets.expander.rules]]\nstates = []\nheader_label.line_height = 61.0\n";
    let scale = 2.0;
    let mut faults = Vec::new();
    for preset in [Preset::KdeBreeze, Preset::Adwaita, Preset::Material3] {
        let resolved = |patch: &str| -> Result<NextResolved, Box<dyn std::error::Error>> {
            let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
            Ok(NextTheme::preset(preset)
                .with_patch(patch)
                .resolve(&ThemeContext::light().with_text_scale(scale)))
        };
        let (stock, r) = (resolved("")?, resolved(patch)?);
        let title = r.expander.header_label(WidgetState::NONE);
        let line = title.line_height.ceil();
        let stock_title = stock.expander.header_label(WidgetState::NONE);
        let by_size = (stock_title.size_px * stock.typography.line_height_multiplier).ceil();
        assert_ne!(
            line, by_size,
            "{preset:?}: the stated line box is the size's: the test proves nothing"
        );
        let layout = Layout::next_of(&r);
        let [_, stated] = r.expander.header(WidgetState::NONE).min_size;
        let stated = stated.ok_or("no stated header height")?;
        let ctx = patched_scaled_installed(preset, patch, scale)?;
        let mut own = 0.0;
        let _ = pass(&ctx, at(1.0), |ui| {
            own = ui
                .native_scope(Role::Expander, RoleVariant::Normal, |ui| {
                    ui.spacing().button_padding.y
                })
                .inner;
        });
        let padding = if layout.lays_out() {
            layout.padding.top.unwrap_or(own) + layout.padding.bottom.unwrap_or(own)
        } else {
            own + own
        };
        let want = stated.max(line + padding);
        let got = drawn_expander(&ctx).1.rect.height();
        if layout.line != Some(line) || got != want {
            faults.push(format!(
                "{preset:?}: the header is {got}, its line box {:?}; want {want}, {line}",
                layout.line
            ));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
    Ok(())
}

/// A stated arrow colour fills the arrow, on the painted header and on egui's own, and a
/// stated arrow size is the painted arrow's box.
#[test]
fn an_expander_arrow_takes_its_stated_colour_and_size() -> TestResult {
    use crate::expander::{Layout, Look};
    let magenta = egui::Color32::from_rgb(255, 0, 255);
    let patch = "[[widgets.expander.rules]]\nstates = []\narrow.color = \"#ff00ff\"\n\
                 arrow.size = 23.0\n";
    let mut painted = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = patched(preset, "")?;
        assert_ne!(
            Look::next_of(&stock).arrow_color,
            Some(magenta),
            "{preset:?}: the stated colour is the preset's: the test proves nothing"
        );
        assert_ne!(
            Layout::next_of(&stock).arrow_size,
            Some(23.0),
            "{preset:?}: the stated size is the preset's: the test proves nothing"
        );
        let (out, _, parts) = drawn_expander(&patched_installed(preset, patch)?);
        let filled = flat(&out.shapes).iter().any(|s| match s {
            egui::Shape::Path(p) => p.fill == magenta,
            _ => false,
        });
        assert!(filled, "{preset:?}: no arrow filled in the stated colour");
        if Layout::next_of(&patched(preset, patch)?).lays_out() {
            painted += 1;
            let arrow = parts.get("arrow").ok_or("no arrow")?;
            assert_eq!(arrow.size(), egui::vec2(23.0, 23.0), "{preset:?}");
        }
    }
    assert!(
        painted > 0,
        "no preset paints its header: the test proves nothing"
    );
    Ok(())
}

/// The stated frame, body indent and arrow gap lay the expander out, on every preset, those
/// whose own header is egui's included; a trailing arrow lies after the title.
#[test]
fn an_expander_takes_its_stated_frame_indent_gap_and_side() -> TestResult {
    use crate::expander::Layout;
    let leading = "[widgets.expander]\narrow_side = \"leading\"\narrow_gap = 23.0\n\
                   content_indent = 31.0\nframe_enabled = true\n";
    let trailing = "[widgets.expander]\narrow_side = \"trailing\"\n";
    let mut unframed = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Layout::next_of(&patched(preset, "")?);
        assert!(
            stock.arrow_gap != Some(23.0) && stock.content_indent != Some(31.0),
            "{preset:?}: the stated gap or indent is the preset's: the test proves nothing"
        );
        if stock.frame_enabled != Some(true) {
            unframed += 1;
        }
        let ctx = patched_installed(preset, leading)?;
        let mut body = None;
        let mut add = |ui: &mut egui::Ui| {
            let r = Expander::new("Details")
                .default_open(true)
                .show(ui, |ui| ui.label("Body"));
            body = r.body_returned.map(|l| l.rect);
            r.header_response
        };
        let _ = response(&ctx, at(1.0), &mut add);
        let (_, header) = response(&ctx, at(1.0), &mut add);
        let body = body.ok_or("no body")?;
        let parts = Parts::of(&header).ok_or("no parts")?;
        let (arrow, title) = (
            parts.get("arrow").ok_or("no arrow")?,
            parts.get("title").ok_or("no title")?,
        );
        assert!(parts.get("frame").is_some(), "{preset:?}: no frame");
        assert_eq!(body.left() - header.rect.left(), 31.0, "{preset:?}");
        assert_eq!(title.left() - arrow.right(), 23.0, "{preset:?}");
        let (_, _, parts) = drawn_expander(&patched_installed(preset, trailing)?);
        let (arrow, title) = (
            parts.get("arrow").ok_or("no arrow")?,
            parts.get("title").ok_or("no title")?,
        );
        assert!(
            arrow.left() > title.right(),
            "{preset:?}: {arrow:?} {title:?}"
        );
    }
    assert!(
        unframed > 0,
        "every preset frames its expander: the test proves nothing"
    );
    Ok(())
}

/// A site that keeps egui's own treatment of the flag `s` on a part it draws at `s | t`, `read`
/// from the theme, takes the theme's value where the theme states it for `s` by either of spec
/// §2's two queries (`from_state`), on every preset with `widget`'s `property` first stated by a
/// rule of no state (`base`), which clears the bit of the query at `s` alone, and then a rule as
/// `home_patch_toml` (`tools/showcase-support`) states a home: a rule for `s | t` alone, stating
/// `value`, is drawn at `s | t` (`drawn`), and a rule for `t` alone leaves egui's own treatment
/// (`None`). Each preset and rule that reads otherwise is pushed to `misses`.
fn two_flag_site<T: PartialEq + std::fmt::Debug>(
    misses: &mut Vec<String>,
    site: &str,
    (widget, property): (&str, &str),
    [s, t]: [&str; 2],
    [base, value]: [&str; 2],
    drawn: T,
    read: impl Fn(&NextResolved) -> Option<T>,
) -> TestResult {
    let base = format!("[[widgets.{widget}.rules]]\nstates = []\n{property} = {base}\n");
    for (states, expected) in [
        (format!("\"{s}\", \"{t}\""), Some(drawn)),
        (format!("\"{t}\""), None),
    ] {
        let patch = format!(
            "{base}[[widgets.{widget}.rules]]\nstates = [{states}]\n{property} = {value}\n"
        );
        for &(key, preset) in crate::next::parity::PRESETS {
            let got = read(&patched(preset, &patch)?);
            if got != expected {
                misses.push(format!(
                    "{site}, {key}: a rule for [{states}] reads {got:?}, not {expected:?}"
                ));
            }
        }
    }
    Ok(())
}

/// The switch's and the radio's looks keep egui's own treatment of a flag over a value stated
/// for a state with another flag only where the theme states nothing for the flag by either of
/// the two queries ([`two_flag_site`]).
#[test]
fn a_rule_for_two_flags_states_the_value_for_the_flag_asked() -> TestResult {
    use crate::radio_button::Over;
    use crate::switch::Look;
    let magenta = egui::Color32::from_rgb(0xff, 0, 0xff);
    let colours = ["\"#101010\"", "\"#ff00ff\""];
    let lines = [
        "{ width = 1.0, color = \"#101010\" }",
        "{ width = 2.0, color = \"#ff00ff\" }",
    ];
    let (hover_checked, press_checked) = (["hover", "checked"], ["pressed", "checked"]);
    let disabled_checked = ["disabled", "checked"];
    let mut misses = Vec::new();
    two_flag_site(
        &mut misses,
        "switch hovered checked track",
        ("switch", "track.fill"),
        hover_checked,
        colours,
        magenta,
        |r| Look::next_of(r)?.hover_checked_background,
    )?;
    two_flag_site(
        &mut misses,
        "radio hovered checked fill",
        ("radio", "indicator.fill"),
        hover_checked,
        colours,
        magenta,
        |r| Over::next_of(r, true, false).hovered.fill,
    )?;
    two_flag_site(
        &mut misses,
        "radio pressed checked fill",
        ("radio", "indicator.fill"),
        press_checked,
        colours,
        magenta,
        |r| Over::next_of(r, true, false).pressed.fill,
    )?;
    two_flag_site(
        &mut misses,
        "radio hovered checked dot",
        ("radio", "dot.fill"),
        hover_checked,
        colours,
        magenta,
        |r| Over::next_of(r, true, false).hovered.dot,
    )?;
    two_flag_site(
        &mut misses,
        "radio disabled checked outline",
        ("radio", "indicator.stroke"),
        disabled_checked,
        lines,
        magenta,
        |r| Over::next_of(r, true, true).rest.stroke.map(|(c, _)| c),
    )?;
    two_flag_site(
        &mut misses,
        "radio disabled checked outline width",
        ("radio", "indicator.stroke"),
        disabled_checked,
        lines,
        2.0,
        |r| Over::next_of(r, true, true).rest.stroke_width,
    )?;
    two_flag_site(
        &mut misses,
        "radio disabled checked label",
        ("radio", "label.color"),
        disabled_checked,
        colours,
        magenta,
        |r| Over::next_of(r, true, true).label,
    )?;
    assert!(misses.is_empty(), "{misses:#?}");
    Ok(())
}

/// An unchecked radio's entries take no dot colour, as egui letters a label with no colour of
/// its own in the entry's `fg_stroke` the dot is painted in (`Over::next_noted`): on
/// windows-11, light and dark, whose rules for `HOVER | CHECKED` and `PRESSED | CHECKED` state
/// the dot (the query at `HOVER | CHECKED` asking `HOVER` has the bit), the checked radio's
/// hovered and pressed dots are those rules', and the unchecked radio's, enabled or disabled,
/// none in any state.
#[test]
fn an_unchecked_radio_takes_no_dot_colour() -> TestResult {
    use crate::radio_button::Over;
    use native_theme_egui::native_theme::next::WidgetState;
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let r = NextTheme::preset(Preset::Windows11).resolve(&ctx);
        let on = Over::next_of(&r, true, false);
        for (name, entry, state) in [
            ("hovered", on.hovered, WidgetState::HOVER),
            ("pressed", on.pressed, WidgetState::PRESSED),
        ] {
            let dot = r.radio.dot(state | WidgetState::CHECKED);
            let stated = native_theme_egui::next::fill_of(&dot, "radio.dot")?;
            assert!(stated.is_some(), "{ctx:?}: no {name} checked dot");
            assert_eq!(entry.dot, stated, "{ctx:?}: the {name} checked dot");
        }
        for disabled in [false, true] {
            let off = Over::next_of(&r, false, disabled);
            for (name, entry) in [
                ("rest", off.rest),
                ("hovered", off.hovered),
                ("pressed", off.pressed),
            ] {
                assert_eq!(
                    entry.dot, None,
                    "{ctx:?}, disabled {disabled}: the unchecked radio's {name} dot"
                );
            }
        }
    }
    Ok(())
}
