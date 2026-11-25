//! Connector parity of the theme-drawn controls (docs/v0.8.0-plan.md, section
//! 1, rule 2; M4, step 1): each look type's 0.8.0 constructor gives the look
//! its 0.7.2 constructor gives, and so does each value a control reads beside
//! its look ([`ThemeSource`]: the ground it is faded over, its focus ring, the
//! focus ring's trigger, its label's font, the switch's body text and label
//! font and the spinner's colours), on the presets the connector parity covers, kde-breeze,
//! adwaita, windows-11, macos-monterey, macos-tahoe, material-3 and ios-13,
//! light and dark, in every state each constructor takes.
//!
//! The 0.7.2 side is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`, on the macOS presets and ios-13 at
//! Apple's font DPI,
//! `next::parity::old_context`) and read unscaled; the 0.8.0 side for the
//! scheme's default context, whose text scale is 1. Every look is given on
//! these presets, so a 0.7.2 constructor's `None` or a 0.8.0 one's `Unmapped`
//! (the control drawn as gpui-component's own) is a difference too, and
//! fails where both sides give none, so no leaf passes by being absent from
//! both. A difference is allowed only as an entry of [`DIFFERENCES`], with
//! its reason, and the 0.8.0 answer of a listed leaf is the one its 0.8.0
//! home gives ([`Report::moved`]); an entry that no longer differs fails.

use std::fmt::Debug;

use gpui::px;
use gpui_component::Size;
use native_theme::next::{Preset, ThemeContext, WidgetState};

use super::checkbox::DISABLED_INDICATOR_ALPHA;
use super::{
    CheckboxLook, FocusRing, LabelFont, ProgressBarLook, SeparatorLook, SliderLook, SpinnerLook,
    SwitchLabelFont, SwitchLook, TabLook, ThemeSource, ThumbOutline, rgba,
};
use crate::next::{Mapped, Unmapped, fill, stroke_color};
use crate::{ColorMode, Native, ResolvedTheme};

/// The presets compared: the 0.7.2 key and the 0.8.0 preset.
const PRESETS: &[(&str, Preset)] = &[
    ("kde-breeze", Preset::KdeBreeze),
    ("adwaita", Preset::Adwaita),
    ("windows-11", Preset::Windows11),
    ("macos-monterey", Preset::MacosMonterey),
    ("macos-tahoe", Preset::MacosTahoe),
    ("material-3", Preset::Material3),
    ("ios-13", Preset::Ios13),
];

/// A listed difference (plan, section 1, rule 2): the leaf, the presets on
/// which it differs, and why.
struct Difference {
    leaf: &'static str,
    presets: &'static [&'static str],
    reason: &'static str,
}

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    Difference {
        leaf: "SwitchLook::label disabled",
        presets: &["macos-monterey", "macos-tahoe", "ios-13"],
        reason: "0.7.2 has no switch label and the look took defaults.disabled_text_color; the \
                 0.8.0 presets state the disabled switch's label at its rest colour, as the \
                 platform draws it: macOS labelColor (os-probe run 45210835145, macOS 15 and \
                 26, `switch-label-macos-{15,26}/measure.toml` `*.normal.toggle_*_disabled` \
                 within 3 levels of the 13 pt labelColor reference), ios-13 the list row's \
                 label colour #000000 / #ffffff (ios-host run 45230061151, iOS 18.6, \
                 `measure_lists.toml` `run1.\"<a>\".switch_row.disabled` `label_text_color`)",
    },
    Difference {
        leaf: "SwitchLook::label disabled",
        presets: &["kde-breeze", "adwaita"],
        reason: "0.7.2 has no switch label and the look took defaults.disabled_text_color; the \
                 0.8.0 presets state the platform's disabled switch label: kde-breeze \
                 qqc2-desktop-style's, the Disabled [Colors:Window] text at its opacity 0.6 \
                 (#a0a1a399 / #686a6c99: Switch.qml:72, :75 at qqc2-desktop-style v6.31.0; \
                 plasmadesktoptheme.cpp:273-274 at Kirigami 8319acc7), adwaita AdwSwitchRow's \
                 title, the row's text at --disabled-opacity 50 % (#00000666 / #ffffff80: \
                 _labels.scss:8-10, _colors.scss:37, :100, :320 at libadwaita 1.10.0)",
    },
    Difference {
        leaf: "SwitchLook::label",
        presets: &["adwaita"],
        reason: "0.7.2 has no switch label and the look took the window's text, #2e3436 in \
                 light (no source at libadwaita, next/presets/adwaita.rs); the 0.8.0 adwaita \
                 states AdwSwitchRow's title in the row's text, view_fg_color and \
                 card_fg_color RGB(0 0 6 / 80%) (#000006cc: _lists.scss:5-7, :521; \
                 _misc.scss:202-204; _colors.scss:37, :62 at libadwaita 1.10.0); in dark both \
                 are white",
    },
    Difference {
        leaf: "SwitchLook::label_gap",
        presets: &["kde-breeze", "windows-11", "adwaita"],
        reason: "0.7.2 has no switch label, so the look has no gap and gpui-component's own \
                 is drawn; the 0.8.0 presets state the platform's: kde-breeze 4, \
                 qqc2-desktop-style's Switch spacing, Kirigami.Units.smallSpacing \
                 (Switch.qml:26, :70 at qqc2-desktop-style v6.31.0; units.cpp:37 at Kirigami \
                 8319acc7); windows-11 12, ToggleSwitch's fixed column \
                 (ToggleSwitch_themeresources.xaml:499-503 at microsoft-ui-xaml 8463f45); \
                 adwaita 6, the least gap of AdwSwitchRow's header box (_lists.scss:132 at \
                 libadwaita 1.10.0)",
    },
    Difference {
        leaf: "SwitchLook::label disabled",
        presets: &["material-3"],
        reason: "0.7.2 has no switch label and the look took defaults.disabled_text_color, \
                 on-surface at 0.38; the 0.8.0 material-3 states Android's MaterialSwitch \
                 text, on-background at material_emphasis_medium 0.6 when disabled \
                 (#1d1b2099 / #e6e0e999: android:Widget.CompoundButton's textColor \
                 ?attr/textColorPrimaryDisableOnly, AOSP core/res/res/values/styles.xml:386 at \
                 android-15.0.0_r1; m3_primary_text_disable_only.xml:19, dimens.xml:21, \
                 materialswitch/res/values/styles.xml:17-29 at material-components-android \
                 66c334b)",
    },
    Difference {
        leaf: "CheckboxLook::of label disabled",
        presets: &["material-3"],
        reason: "0.7.2's disabled check box label is defaults.disabled_text_color, on-surface \
                 at 0.38, an inheritance no source states for it (no M3 token for the label); \
                 the 0.8.0 material-3 states Android's, the label's colour at \
                 material_emphasis_medium 0.6 (#1d1b2099 / #e6e0e999: \
                 android:Widget.CompoundButton's textColor ?attr/textColorPrimaryDisableOnly, \
                 AOSP core/res/res/values/styles.xml:386 at android-15.0.0_r1; \
                 checkbox/res/values/styles.xml:26-28, m3_primary_text_disable_only.xml:19, \
                 dimens.xml:21 at material-components-android 66c334b; \
                 native-theme/tests/next_homes.toml, row checkbox.disabled_text_color, a listed \
                 difference)",
    },
    Difference {
        leaf: "CheckboxLook::of mark disabled",
        presets: &["material-3"],
        reason: "the disabled checked mark is surface where 0.7.2 draws it in its disabled \
                 label colour (selected-disabled-icon-color, _md-comp-checkbox.scss:50, v0_192 \
                 at material-web cbd34a8; the look reads \
                 checkbox.checkmark(DISABLED|CHECKED).color, CheckboxLook::next_of)",
    },
    Difference {
        leaf: "CheckboxLook::of mark disabled",
        presets: &["macos-tahoe"],
        reason: "the 0.8.0 macos-tahoe states the measured disabled checked box's mark for \
                 DISABLED, #fefefe80 light and #fdffff80 dark (os-probe run 45345116763, \
                 macOS 26, `*.normal.checkbox_disabled_on` `label`), where the 0.7.2 look drew \
                 it in the label's disabled colour where a disabled fill is stated, else as \
                 enabled; the look reads checkbox.checkmark(DISABLED|CHECKED).color",
    },
    Difference {
        leaf: "CheckboxLook::of mark disabled",
        presets: &["macos-monterey"],
        reason: "the 0.8.0 macos-monterey states the measured disabled checked box's mark for \
                 DISABLED in light, #01010140 (os-probe run 45345116763, macOS 15, \
                 `*.normal.checkbox_disabled_on` `label`), and none in dark, where the run \
                 read no one colour (`label` non_uniform_fill): light, the look reads \
                 checkbox.checkmark(DISABLED|CHECKED).color; dark, the mark keeps \
                 gpui-component's own treatment (spec §10, rule 0), the enabled mark, \
                 checkbox.checkmark(CHECKED).color, at half alpha \
                 (primary_foreground.opacity(0.5), gpui-component 0.10.0 checkbox.rs:230-234); \
                 where the 0.7.2 look drew it in the label's disabled colour where a disabled \
                 fill is stated, else as enabled",
    },
    Difference {
        leaf: "CheckboxLook::radio mark disabled",
        presets: &["macos-tahoe"],
        reason: "as the check box's (CheckboxLook::of mark disabled): the 0.8.0 macos-tahoe \
                 states the measured disabled checked radio's dot for DISABLED, the check box's \
                 mark colour, #fefefe80 light and #fdffff80 dark (os-probe run 45345116763, \
                 macOS 26, `*.normal.radio_disabled_on` `label`), where the 0.7.2 look drew it \
                 in the label's disabled colour where a disabled fill is stated, else as \
                 enabled; the look reads radio.dot(DISABLED|CHECKED).fill",
    },
    Difference {
        leaf: "CheckboxLook::radio mark disabled",
        presets: &["macos-monterey"],
        reason: "as the check box's (CheckboxLook::of mark disabled): the 0.8.0 macos-monterey \
                 states the measured disabled checked radio's dot for DISABLED in light, \
                 #01010140 (os-probe run 45345116763, macOS 15, `*.normal.radio_disabled_on` \
                 `label`), read at radio.dot(DISABLED|CHECKED).fill, and none in dark (not \
                 read), where gpui-component's radio draws the same mark through \
                 checkbox_check_icon, dimmed to half alpha when disabled (gpui-component \
                 0.10.0 radio.rs:249-256, checkbox.rs:230-234), so the dark disabled dot is \
                 radio.dot(CHECKED).fill at half alpha; where the 0.7.2 look drew it in the \
                 label's disabled colour where a disabled fill is stated, else as enabled",
    },
    Difference {
        leaf: "CheckboxLook::radio label disabled",
        presets: &["material-3"],
        reason: "as the check box's (CheckboxLook::of label disabled): the radio button's \
                 label inherits the same textColor (radiobutton/res/values/styles.xml:19-33 at \
                 material-components-android 66c334b sets none), so 0.6 where 0.7.2 has 0.38, \
                 the disabled dot with it",
    },
    Difference {
        leaf: "CheckboxLook::of hover_border",
        presets: &["material-3"],
        reason: "0.7.2 states no checkbox.hover_border_color, so the look has no hover \
                 outline; the 0.8.0 material-3 states an unchecked check box's outline for \
                 HOVER, on-surface (unselected-hover-outline-color, _md-comp-checkbox.scss:111-112, \
                 v0_192 at material-web cbd34a8; native-theme/tests/next_homes.toml, row \
                 checkbox.hover_border_color, a listed difference); so an enabled box hovers in \
                 on-surface, a checked one in its checked outline",
    },
    Difference {
        leaf: "TabLook::hover",
        presets: &["adwaita"],
        reason: "0.7.2's tab.hover_background #e8e8e8 / #3a3a3e is its uncited menu hover; \
                 0.8.0's adwaita states libadwaita's notebook tab hover, $hover_color, the \
                 window text at 7 % (#0000060e / #ffffff12), a layer the look composites \
                 over the bar (_notebook.scss:127-129, _colors.scss:268, :33, :96 at \
                 libadwaita 1.10.0; native-theme/tests/next_homes.toml, row \
                 tab.hover_background, a listed difference)",
    },
    Difference {
        leaf: "CheckboxLook::of hover_fill",
        presets: &["windows-11"],
        reason: "0.7.2's checkbox.hover_background, an uncited #0000000a / #ffffff0f, is a \
                 layer the look composites over the unchecked fill; 0.8.0's windows-11 states \
                 WinUI's whole hovered fill, which the look takes as it is (spec §9, step 2): \
                 ControlAltFillColorTertiary #0000000f / #ffffff0b \
                 (CheckBoxCheckBackgroundFillUncheckedPointerOver, CheckBox_themeresources.xaml:230, \
                 :54; Common_themeresources_any.xaml:235, :31 at microsoft-ui-xaml 8463f45; \
                 native-theme/tests/next_homes.toml, row checkbox.hover_background, a listed \
                 difference)",
    },
    Difference {
        leaf: "CheckboxLook::radio hover_fill",
        presets: &["windows-11"],
        reason: "as the check box's (CheckboxLook::of hover_fill): the radio button's \
                 RadioButtonOuterEllipseFillPointerOver is the same ControlAltFillColorTertiary \
                 (RadioButton_themeresources.xaml:139, :23 at microsoft-ui-xaml 8463f45)",
    },
    Difference {
        leaf: "SwitchLook hover_track",
        presets: &["windows-11"],
        reason: "0.7.2's switch.hover_checked_background is the window colour at alpha 0.1, \
                 a layer the look composites over the on track; 0.8.0's windows-11 states \
                 WinUI's whole hovered fill, which the look takes as it is (spec §9, step 2): \
                 AccentFillColorSecondary, the accent's fill at 0.9 opacity \
                 (ToggleSwitchFillOnPointerOver, ToggleSwitch_themeresources.xaml:144, :24; \
                 Common_themeresources_any.xaml:330, :126 at microsoft-ui-xaml 8463f45; \
                 native-theme/tests/next_homes.toml, row switch.hover_checked_background, a \
                 listed difference)",
    },
    Difference {
        leaf: "CheckboxLook::radio hover_border",
        presets: &["adwaita"],
        reason: "0.7.2 states no checkbox.hover_border_color, so the look has no hover \
                 outline; 0.8.0's adwaita states an unchecked radio's outline for HOVER, \
                 $trough_hover_color, which libadwaita draws on a hovered unchecked check and \
                 radio alike (_checks.scss:20-35 at libadwaita 1.10.0; \
                 native-theme/tests/next_homes.toml, row checkbox.hover_border_color, a listed \
                 difference); so an enabled radio hovers in the outline stated there, a checked \
                 one in its checked outline",
    },
    Difference {
        leaf: "CheckboxLook::of hover_border",
        presets: &["macos-monterey", "macos-tahoe", "ios-13"],
        reason: "0.7.2 states no checkbox.hover_border_color, so the look has no hover \
                 outline; 0.8.0 states the box's rest outline for HOVER \
                 (native-theme/tests/next_homes.toml, row checkbox.hover_border_color, a \
                 listed difference): a hovered AppKit check box draws as at rest (os-probe run \
                 45198750893, macOS 15 and 26, `*.hovered.checkbox_off` and `…checkbox_on` \
                 differ from the normal capture in 0 pixels), and WebKit's iOS \
                 checkboxRadioBorderColor reads no hover (RenderThemeIOS.mm:1357-1368 at \
                 WebKit 31adf9d); so an enabled box hovers in its rest outline, a checked one \
                 in its checked outline (on ios-13 none)",
    },
    Difference {
        leaf: "CheckboxLook::radio hover_border",
        presets: &["macos-monterey", "macos-tahoe", "ios-13"],
        reason: "as the check box's (CheckboxLook::of hover_border): the radio button's \
                 indicator is the same part on these presets",
    },
    Difference {
        leaf: "CheckboxLook::of outline",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 outlines the box in its uncited default line, 0.5 px in the \
                 separator, checked or not; 0.8.0's ios-13 states the outline WebKit strokes \
                 round an empty iOS check box, checkboxRadioBorderColor (secondaryLabel), \
                 1.5 px as it shows (checkboxRadioBorderWidth × 2 clipped to the edge), and \
                 none round a checked one (RenderThemeIOS.mm:1354, :1357-1368, :1483-1491, \
                 :1502 at WebKit 31adf9d; native-theme/tests/next_homes.toml, rows \
                 checkbox.unchecked_border_color, checkbox.border.color and \
                 checkbox.border.line_width, listed differences); the mark, inset by the \
                 outline where no padding is stated, is 19 px where 0.7.2's is 21 px",
    },
    Difference {
        leaf: "CheckboxLook::of fill",
        presets: &["ios-13"],
        reason: "0.8.0's ios-13 fills an empty box at rest in WebKit's \
                 -webkit-control-background, systemBackground #ffffff / #000000, in place of \
                 0.7.2's uncited #ffffff / #1c1c1e (checkboxRadioBackgroundColor, \
                 RenderThemeIOS.mm:1391, :1395, :1027 at WebKit 31adf9d; \
                 system-color-for-css-value-expected.txt:7, :9 at WebKit de320ba; \
                 native-theme/tests/next_homes.toml, row checkbox.unchecked_background, a \
                 listed difference); the look reads it at checkbox.indicator(NONE).fill",
    },
    Difference {
        leaf: "CheckboxLook::radio fill",
        presets: &["ios-13"],
        reason: "as the check box's (CheckboxLook::of fill): the radio button's indicator is \
                 the same part on ios-13; the look reads it at radio.indicator(NONE).fill",
    },
    Difference {
        leaf: "CheckboxLook::of opacity",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 fades a disabled box by its uncited defaults.disabled_opacity \
                 0.3 (ios-13.toml); 0.8.0's draws it at full opacity, as WebKit's iOS style \
                 sheet resets a disabled check box's and radio button's opacity 0.4 with \
                 `opacity: initial` (html.css:962-963, :1031-1034, :1040-1047 at WebKit \
                 31adf9d), their disabled colours the theme's own; the look reads \
                 checkbox.disabled_opacity, 1",
    },
    Difference {
        leaf: "CheckboxLook::radio opacity",
        presets: &["ios-13"],
        reason: "as the check box's (CheckboxLook::of opacity): the same rule of the iOS style \
                 sheet covers the radio button (html.css:1040-1047 at WebKit 31adf9d); the \
                 look reads radio.disabled_opacity, 1",
    },
    Difference {
        leaf: "CheckboxLook::of fill disabled",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 fills a disabled box, checked or not, in its uncited \
                 checkbox.disabled_background #e5e5ea80 / #2c2c2e80 (ios-13.toml); 0.8.0's \
                 states WebKit's: empty, -webkit-control-background, systemBackground #ffffff \
                 / #000000 (checkboxRadioBackgroundColor, RenderThemeIOS.mm:1387-1388, :1027 \
                 at WebKit 31adf9d), checked, opaqueTertiaryFill #eeeeef / #1c1c1e \
                 (RenderThemeIOS.mm:1372, :1387-1388; ios-18 expected.txt:90-91, :93); the look \
                 reads them at checkbox.indicator(DISABLED).fill and \
                 checkbox.indicator(DISABLED|CHECKED).fill",
    },
    Difference {
        leaf: "CheckboxLook::radio fill disabled",
        presets: &["ios-13"],
        reason: "as the check box's (CheckboxLook::of fill disabled): WebKit fills a disabled \
                 radio button's circle as the box, checked in opaqueTertiaryFill \
                 (RenderThemeIOS.mm:1561-1563 at WebKit 31adf9d); the look reads \
                 radio.indicator(DISABLED).fill and radio.indicator(DISABLED|CHECKED).fill",
    },
    Difference {
        leaf: "CheckboxLook::of border disabled",
        presets: &["adwaita", "material-3"],
        reason: "these 0.8.0 presets state no checkbox.indicator(DISABLED).stroke, so a \
                 disabled box keeps gpui-component's own treatment (spec §10, rule 0): its \
                 enabled border, checkbox.indicator(CHECKED).stroke.color checked and \
                 checkbox.indicator(NONE).stroke.color unchecked, at half alpha \
                 (disabled_indicator_color, gpui-component 0.10.0 checkbox.rs:286-290, \
                 :359-361), where the 0.7.2 look kept the enabled border; the 0.8.0 look is \
                 compared to that home at half alpha (Report::moved)",
    },
    Difference {
        leaf: "CheckboxLook::of border disabled",
        presets: &["kde-breeze"],
        reason: "as adwaita's and material-3's: the 0.8.0 kde-breeze states no \
                 checkbox.indicator(DISABLED).stroke, so a disabled box keeps \
                 gpui-component's own treatment (spec §10, rule 0), its enabled border at half \
                 alpha, where the 0.7.2 look kept the enabled border. kde-breeze does state a \
                 disabled checked box's frame for DISABLED|CHECKED, Breeze's Disabled \
                 Highlight #e3e5e7 / #1f2124 (`indicator` in native-theme/src/next/presets/ \
                 kde.rs, switch_disabled_on_frame), which the look does not reach: by spec §2's \
                 from_state contract a connector keeping its own treatment of DISABLED asks \
                 the query with that flag alone, and a rule for DISABLED|CHECKED does not set \
                 its bit there; a spec question, not a connector fix",
    },
    Difference {
        leaf: "CheckboxLook::of border disabled",
        presets: &["windows-11"],
        reason: "the 0.8.0 windows-11 states WinUI's disabled box's outline, \
                 CheckBoxCheckBackgroundStrokeUncheckedDisabled and …StrokeCheckedDisabled = \
                 ControlStrongStrokeColorDisabled, #37000000 / #28ffffff \
                 (CheckBox_themeresources.xaml:220, :44, :224, set at :465-468 at \
                 microsoft-ui-xaml 8463f45; XAML's #AARRGGBB, alpha first: #RRGGBBAA \
                 #00000037 / #ffffff28), where the 0.7.2 look kept the enabled border; \
                 gpui-component's half alpha of the rest border would give #00000039 \
                 (#RRGGBBAA, alpha last) in light; the look reads \
                 checkbox.indicator(DISABLED).stroke.color and \
                 checkbox.indicator(DISABLED|CHECKED).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::of border disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "the 0.8.0 macOS presets state the measured disabled box's edge, checked or not: \
                 macos-monterey #d1d1d180 on the sides and the bottom in light and none in \
                 dark (every edge pixel within 6 alpha levels of the fill), macos-tahoe none \
                 (every edge pixel the fill) (os-probe run 45345116763, macOS 15 and 26, \
                 `*.normal.checkbox_disabled_{off,on}` `edge_*`), where the 0.7.2 look kept \
                 the enabled border; the look reads checkbox.indicator(DISABLED).stroke.color \
                 and checkbox.indicator(DISABLED|CHECKED).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::of border disabled",
        presets: &["ios-13"],
        reason: "the 0.8.0 ios-13 states WebKit's disabled box's outline: empty, \
                 checkboxRadioBorderColor secondaryLabel at checkboxRadioBorderDisabledOpacity \
                 0.3 (RenderThemeIOS.mm:1355, :1359-1362 at WebKit 31adf9d), checked, none \
                 (:1502), where the 0.7.2 look kept the enabled border; the look reads checkbox.indicator(DISABLED).stroke.color and \
                 checkbox.indicator(DISABLED|CHECKED).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::radio border disabled",
        presets: &["adwaita"],
        reason: "as the check box's (CheckboxLook::of border disabled): the 0.8.0 adwaita \
                 states no radio.indicator(DISABLED).stroke, and gpui-component dims a disabled \
                 radio's border_color, radio.indicator(CHECKED).stroke.color checked and \
                 radio.indicator(NONE).stroke.color unchecked, to half alpha \
                 (gpui-component 0.10.0 radio.rs:193-200)",
    },
    Difference {
        leaf: "CheckboxLook::radio border disabled",
        presets: &["kde-breeze"],
        reason: "as the check box's (CheckboxLook::of border disabled, kde-breeze): no \
                 radio.indicator(DISABLED).stroke is stated, so gpui-component dims a disabled \
                 radio's border_color to half alpha (gpui-component 0.10.0 radio.rs:193-200); \
                 the Disabled Highlight frame kde-breeze states for DISABLED|CHECKED \
                 (#e3e5e7 / #1f2124, switch_disabled_on_frame) is not reached by the query \
                 with DISABLED alone (spec §2, from_state), a spec question",
    },
    Difference {
        leaf: "CheckboxLook::radio border disabled",
        presets: &["windows-11"],
        reason: "the 0.8.0 windows-11 states WinUI's disabled circle's outline, \
                 RadioButtonOuterEllipseStrokeDisabled = ControlStrongStrokeColorDisabled \
                 (RadioButton_themeresources.xaml:137, :21 at microsoft-ui-xaml 8463f45), \
                 checked …CheckedStrokeDisabled = AccentFillColorDisabled (set at :326-330), \
                 #37000000 / #28ffffff both (XAML's #AARRGGBB, alpha first), where the 0.7.2 \
                 look kept the enabled border; the look reads \
                 radio.indicator(DISABLED).stroke.color and \
                 radio.indicator(DISABLED|CHECKED).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::radio border disabled",
        presets: &["macos-monterey", "macos-tahoe", "ios-13"],
        reason: "as the check box's (CheckboxLook::of border disabled): the radio button's \
                 indicator is the same part on these presets (macos-monterey's light circle \
                 measured #d0d0d07c, the same run, `*.normal.radio_disabled_off` `edge_*`, the \
                 preset stating the box's #d1d1d180 for both)",
    },
    Difference {
        leaf: "CheckboxLook::radio border disabled",
        presets: &["material-3"],
        reason: "the 0.8.0 material-3 states material-web's disabled radio's ring, \
                 disabled-unselected-icon-color and disabled-selected-icon-color on-surface at \
                 0.38 (_radio.scss:163-166, :180-183, _md-comp-radio-button.scss:28-33, v0_192 \
                 at material-web cbd34a8), where the 0.7.2 look kept the enabled border; the \
                 look reads radio.indicator(DISABLED).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::radio outline",
        presets: &["ios-13"],
        reason: "as the check box's (CheckboxLook::of outline): the radio button's indicator is the \
                 same part on ios-13",
    },
    Difference {
        leaf: "CheckboxLook::of fill disabled",
        presets: &["kde-breeze", "windows-11"],
        reason: "0.7.2 has one disabled fill for a checked and an unchecked box; 0.8.0 states \
                 the disabled checked box's own (native-theme/tests/next_homes.toml, row \
                 checkbox.disabled_background, a listed difference): kde-breeze the Disabled \
                 Button under the Disabled Highlight at 0.3, #eceded / #25272b \
                 (breezehelper.cpp:865-869 at breeze f0b1d75; docs/platform-facts.md §2.5 \
                 disabled_background), windows-11 AccentFillColorDisabled #37000000 / \
                 #28FFFFFF (CheckBox_themeresources.xaml:236, :60 at microsoft-ui-xaml \
                 8463f45); the look reads it at checkbox.indicator(DISABLED|CHECKED).fill",
    },
    Difference {
        leaf: "CheckboxLook::radio fill disabled",
        presets: &["kde-breeze", "windows-11"],
        reason: "as the check box's (CheckboxLook::of fill disabled): the disabled checked radio's circle \
                 is filled as the box (breezehelper.cpp:1012-1019; \
                 RadioButtonOuterEllipseCheckedFillDisabled, RadioButton_themeresources.xaml:149, \
                 :33)",
    },
    Difference {
        leaf: "CheckboxLook::of fill disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2 states no disabled fill, so a disabled box keeps its rest or checked \
                 fill; 0.8.0 states the measured disabled box (os-probe run 45345116763, macOS \
                 15 and 26, `*.normal.checkbox_disabled_{off,on}` `fill`; \
                 native-theme/tests/next_homes.toml, row checkbox.disabled_background, a listed \
                 difference): macos-monterey #ffffff80 / #ffffff15 checked or not, macos-tahoe \
                 #0000000d / #ffffff0d unchecked and #0076f470 / #0982ff70 checked on the \
                 default accent; the look reads them at checkbox.indicator(DISABLED).fill and \
                 checkbox.indicator(DISABLED|CHECKED).fill",
    },
    Difference {
        leaf: "CheckboxLook::radio fill disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "as the check box's (CheckboxLook::of fill disabled): the disabled radio button's circle is \
                 filled as the box (the same run, `*.normal.radio_disabled_{off,on}` `fill`)",
    },
    Difference {
        leaf: "CheckboxLook::radio fill",
        presets: &["material-3"],
        reason: "0.7.2 fills a checked radio button as its check box, in primary; 0.8.0's \
                 material-3 states material-web's radio, a ring and a dot with nothing between \
                 them in any state (radio/internal/radio.ts:108-120, _radio.scss:109-113 at \
                 material-web cbd34a8; native-theme/tests/next_homes.toml, row \
                 checkbox.checked_background, a listed difference); the look reads \
                 radio.indicator(CHECKED).fill, transparent",
    },
    Difference {
        leaf: "CheckboxLook::radio mark",
        presets: &["material-3"],
        reason: "0.7.2 draws the radio button's dot in on-primary, over the check box's fill; \
                 0.8.0's material-3 states material-web's dot, in selected-icon-color primary \
                 (radio/internal/radio.ts:119, _radio.scss:126-127, \
                 _md-comp-radio-button.scss:45, v0_192 at material-web cbd34a8; \
                 native-theme/tests/next_homes.toml, row checkbox.indicator_color, a listed \
                 difference); the look reads radio.dot(CHECKED).fill",
    },
    Difference {
        leaf: "CheckboxLook::radio fill disabled",
        presets: &["material-3"],
        reason: "0.7.2 fills a disabled radio button as its check box, on-surface at 0.38; \
                 0.8.0's material-3 states material-web's disabled radio, the ring and the dot \
                 in on-surface at 0.38 with nothing between them (_radio.scss:163-166, \
                 :180-183, _md-comp-radio-button.scss:28-33 at material-web cbd34a8; \
                 native-theme/tests/next_homes.toml, row checkbox.disabled_background, a \
                 listed difference); the look reads radio.indicator(DISABLED).fill, \
                 transparent",
    },
    Difference {
        leaf: "SliderLook hover_thumb",
        presets: &["adwaita"],
        reason: "0.7.2's adwaita states an uncited slider.thumb_hover_color (#ebebeb / \
                 #3a3a3e); 0.8.0's adwaita states libadwaita's $slider_hover_color, white in \
                 both schemes (_scale.scss:62-76, _colors.scss:289 at libadwaita 1.10.0; \
                 native-theme/tests/next_homes.toml, row slider.thumb_hover_color, a listed \
                 difference), so the look's hovered thumb is white",
    },
    Difference {
        leaf: "SwitchLabelFont family and style",
        presets: &[
            "kde-breeze",
            "adwaita",
            "windows-11",
            "macos-monterey",
            "macos-tahoe",
            "material-3",
            "ios-13",
        ],
        reason: "the 0.8.0 presets state switch.label as the platform's switch label, the \
                 body font (kde-breeze qqc2-desktop-style's control.font, adwaita \
                 AdwSwitchRow's title, windows-11 ToggleSwitch's content, macOS the body \
                 font, material-3 Android's MaterialSwitch text, ios-13 the list row's \
                 UILabel in the Body text style), its family and slant \
                 included, where the 0.7.2 switch took the body text's size and weight alone \
                 and left the family and slant to the window's (SwitchTheme states no label \
                 font); the size and weight are the same",
    },
    Difference {
        leaf: "CheckboxLook::of hover_border",
        presets: &["adwaita", "windows-11"],
        reason: "0.7.2 states no checkbox.hover_border_color, so the look has no hover \
                 outline; 0.8.0 states the unchecked box's outline for HOVER \
                 (native-theme/tests/next_homes.toml, row checkbox.hover_border_color, a \
                 listed difference): adwaita's $trough_hover_color (_checks.scss:20-35 at \
                 libadwaita 1.10.0), windows-11's rest ControlStrongStrokeColorDefault, which \
                 WinUI keeps (CheckBox_themeresources.xaml:218, :42 at microsoft-ui-xaml \
                 8463f45); so an enabled box hovers in the outline stated there, a checked one \
                 in its checked hovered outline: on adwaita none, on windows-11 \
                 AccentFillColorSecondary, the accent's fill at 0.9, #0067c0e6 / #4cc2ffe6 \
                 (CheckBoxCheckBackgroundStrokeCheckedPointerOver, CheckBox_themeresources.xaml:222, \
                 :46; Common_themeresources_any.xaml:330, :126); the look reads \
                 checkbox.indicator(HOVER|CHECKED).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::of outline",
        presets: &["adwaita", "windows-11"],
        reason: "0.7.2's adwaita and windows-11 outline a checked box in the default border \
                 colour; 0.8.0's adwaita states none round a checked box, as libadwaita draws a checked or mixed \
                 check with `box-shadow: none`, its only outline (_checks.scss:27, :52-56 at \
                 libadwaita 1.10.0); windows-11 outlines it in AccentFillColorDefault, the \
                 accent's fill, #0067c0 / #4cc2ff (CheckBoxCheckBackgroundStrokeChecked, \
                 CheckBox_themeresources.xaml:221, :45 at microsoft-ui-xaml 8463f45) \
                 (native-theme/tests/next_homes.toml, rows checkbox.border.color and \
                 checkbox.unchecked_border_color, listed differences); the look reads \
                 checkbox.indicator(CHECKED).stroke.color and checkbox.indicator(NONE).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::radio outline",
        presets: &["adwaita", "windows-11"],
        reason: "as the check box's (CheckboxLook::of outline): libadwaita's rule covers the \
                 radio (_checks.scss:52-56), WinUI's RadioButtonOuterEllipseCheckedStroke is \
                 AccentFillColorDefaultBrush (RadioButton_themeresources.xaml:142, :26), and \
                 WebKit strokes the iOS radio as the box; the look reads \
                 radio.indicator(CHECKED).stroke.color and radio.indicator(NONE).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::radio hover_border",
        presets: &["windows-11"],
        reason: "0.7.2 states no checkbox.hover_border_color, so the look has no hover \
                 outline; 0.8.0's windows-11 states an unselected circle's outline for HOVER, \
                 the rest ControlStrongStrokeColorDefault, which WinUI keeps \
                 (RadioButtonOuterEllipseStrokePointerOver, RadioButton_themeresources.xaml:135, \
                 :19 at microsoft-ui-xaml 8463f45), and a selected one's, \
                 AccentFillColorSecondary, the accent's fill at 0.9, #0067c0e6 / #4cc2ffe6 \
                 (…CheckedStrokePointerOver, :143, :27; Common_themeresources_any.xaml:330, \
                 :126) (native-theme/tests/next_homes.toml, row checkbox.hover_border_color, \
                 a listed difference); the look reads radio.indicator(HOVER).stroke.color and \
                 radio.indicator(HOVER|CHECKED).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::of mark disabled",
        presets: &["kde-breeze", "windows-11", "ios-13"],
        reason: "0.7.2 draws a disabled box's mark in its disabled label colour (where a \
                 disabled fill is stated); the 0.8.0 presets state the disabled checked or \
                 mixed mark as the platform draws it, which the look reads at \
                 checkbox.checkmark(DISABLED|CHECKED).color: kde-breeze palette.text() in the \
                 Disabled group, Disabled [Colors:View] ForegroundNormal #aaabac / #606263 \
                 (breezehelper.cpp:938, :948-952 at breeze f0b1d75; docs/platform-facts.md \
                 §2.1.6); windows-11 TextOnAccentFillColorDisabled #FFFFFF / #87FFFFFF \
                 (CheckBox_themeresources.xaml:248, :72, :471 at microsoft-ui-xaml 8463f45); \
                 ios-13 WebKit's tertiaryLabel (RenderThemeIOS.mm:1410-1413, :1541-1542 at \
                 WebKit 31adf9d)",
    },
    Difference {
        leaf: "CheckboxLook::radio mark disabled",
        presets: &["kde-breeze", "windows-11", "ios-13"],
        reason: "as the check box's (CheckboxLook::of mark disabled), the dot at \
                 radio.dot(DISABLED|CHECKED).fill: kde-breeze palette.text() in the Disabled \
                 group (breezehelper.cpp:1059); windows-11 RadioButtonCheckGlyphFillDisabled, \
                 the rest's TextOnAccentFillColorPrimary as the source names it \
                 (RadioButton_themeresources.xaml:153, :37, :333; the Color-for-a-Brush key's \
                 drawn result not measured); ios-13 tertiaryLabel (RenderThemeIOS.mm:1578-1579)",
    },
    Difference {
        leaf: "SwitchLook::label_gap",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2 has no switch label, so the look has no gap; the 0.8.0 macOS presets \
                 state switch.label_gap, SwiftUI's switch Toggle's, 8 (os-probe run \
                 45210835145, macOS 15 and 26, `switch-label.toml` `[[item]] toggle_off` \
                 fitting width 33 + 8 + the switch, \
                 docs/evidence/os-probe/2026-11-23-run-45210835145/ on m2-fs4d)",
    },
    Difference {
        leaf: "TabLook::hover",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2's macos-monterey states tab.hover_background #e8e8ed / #3a3a3c, \
                 uncited, macos-tahoe leaves it unset; the 0.8.0 presets state \
                 tab.tab(HOVER).fill, the tab's measured rest fill, as AppKit draws a hovered \
                 tab as at rest (os-probe run 45198750893, macOS 15 and 26, `*.hovered.tab` \
                 differs from the normal capture in 0 pixels, \
                 docs/evidence/os-probe/2026-11-23-run-45198750893/ on m2-fs4c; \
                 native-theme/tests/next_homes.toml, row tab.hover_background, a listed \
                 difference), drawn over the bar",
    },
    Difference {
        leaf: "TabLook::idle",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2's macOS presets give an unselected tab the window's background \
                 (uncited); the 0.8.0 presets state tab.tab(NONE).fill, NSTabView's measured \
                 unselected tab, #0000000d / #ffffff0d on macos-monterey, #00000013 / \
                 #ffffff13 on macos-tahoe (os-probe run 45252258079, Reduce Transparency off, \
                 `*.normal.tab` `fill`, docs/evidence/os-probe/2026-11-23-run-45252258079/ on \
                 m2-fs4c; native-theme/tests/next_homes.toml, row tab.background_color, a \
                 listed difference)",
    },
    Difference {
        leaf: "TabLook::active",
        presets: &["macos-tahoe"],
        reason: "0.7.2's macos-tahoe gives a selected tab the window's background and the body \
                 text (uncited); the 0.8.0 preset states tab.tab(SELECTED).fill and \
                 tab.label(SELECTED).color, NSTabView's measured selected tab on macOS 26, on \
                 the accent #0077f9 / #077dff, lettered white (os-probe run 45252258079, Reduce \
                 Transparency off, `*.normal.tab_One` `fill`, `label`; \
                 native-theme/tests/next_homes.toml, rows tab.active_background and \
                 tab.active_text_color, listed differences)",
    },
    Difference {
        leaf: "TabLook::active",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey gives a selected tab the window's background and the \
                 body text (uncited); the 0.8.0 preset states tab.tab(SELECTED).fill and \
                 tab.label(SELECTED).color, NSTabView's measured selected tab on macOS 15, \
                 #ffffff / #ffffff49 lettered #262626 / #ffffffd9 (os-probe run 45252258079, \
                 Reduce Transparency off, `*.normal.tab_One` `fill`, `label`; \
                 native-theme/tests/next_homes.toml, rows tab.active_background and \
                 tab.active_text_color, listed differences)",
    },
    Difference {
        leaf: "CheckboxLook::of hover_fill",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states checkbox.hover_background #e0e0e5 / #444446, \
                 uncited; the 0.8.0 preset states checkbox.indicator(HOVER).fill, the check \
                 box's rest fill, as AppKit draws a hovered check box as at rest (os-probe run \
                 45198750893, macOS 15, `*.hovered.checkbox_off` differs from the normal \
                 capture in 0 pixels, docs/evidence/os-probe/2026-11-23-run-45198750893/ on \
                 m2-fs4c; native-theme/tests/next_homes.toml, row checkbox.hover_background, a \
                 listed difference)",
    },
    Difference {
        leaf: "CheckboxLook::radio hover_fill",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states checkbox.hover_background #e0e0e5 / #444446, \
                 uncited, for the radio button too; the 0.8.0 preset states no rule for \
                 radio.indicator(HOVER).fill (not measured; the check box measured in the same \
                 run draws hovered as at rest, os-probe run 45198750893, macOS 15, \
                 `*.hovered.checkbox_off` differs from the normal capture in 0 pixels, \
                 docs/evidence/os-probe/2026-11-23-run-45198750893/ on m2-fs4c; \
                 native-theme/tests/next_homes.toml, row checkbox.hover_background, a listed \
                 difference, \"unstated\" there), so the look's hovered fill is the rest fill \
                 radio.indicator(NONE).fill",
    },
    Difference {
        leaf: "CheckboxLook::of hover_fill",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 states checkbox.hover_background #d1d1d6 / #3a3a3c, uncited; \
                 the 0.8.0 preset states no fill for checkbox.indicator(HOVER), as WebKit's iOS \
                 checkboxRadioBackgroundColor reads no hover state (RenderThemeIOS.mm:1370-1395 \
                 at WebKit 31adf9d; native-theme/tests/next_homes.toml, row \
                 checkbox.hover_background, a listed difference, \"unstated\" there), so the \
                 look's hovered fill is the rest fill checkbox.indicator(NONE).fill",
    },
    Difference {
        leaf: "CheckboxLook::radio hover_fill",
        presets: &["ios-13"],
        reason: "as the check box's (CheckboxLook::of hover_fill): WebKit draws the radio \
                 button's circle by the same checkboxRadioBackgroundColor, so the look's \
                 hovered fill is the rest fill radio.indicator(NONE).fill",
    },
    Difference {
        leaf: "CheckboxLook::of hover_fill",
        presets: &["adwaita"],
        reason: "0.7.2's adwaita states checkbox.hover_background #d5d5d5 / #464649, \
                 uncited; the 0.8.0 preset states checkbox.indicator(HOVER).fill, the rest \
                 fill #ffffff / #303034, as libadwaita changes only the outline of an unchecked \
                 check under the pointer (`&:not(:checked):not(:indeterminate) { &:hover { \
                 box-shadow: inset 0 0 0 2px $trough_hover_color; } }`, _checks.scss:33-40 at \
                 libadwaita 1.10.0; native-theme/tests/next_homes.toml, row \
                 checkbox.hover_background, a listed difference)",
    },
    Difference {
        leaf: "CheckboxLook::radio hover_fill",
        presets: &["adwaita"],
        reason: "as the check box's (CheckboxLook::of hover_fill): libadwaita's rule is the \
                 same for `check, radio`, so radio.indicator(HOVER).fill is the rest fill",
    },
    Difference {
        leaf: "SliderLook thumb_color",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 leaves slider.thumb_color unset, so it is \
                 defaults.surface_color, #f2f2f7 / #1c1c1e; the 0.8.0 preset states UISlider's \
                 thumb as measured, white in both schemes, slider.thumb(NONE).fill (ios-host \
                 run 45230061151, iOS 18.6, `measure_lists.toml` `press1.\"<a>\".slider.normal` \
                 `fill` #ffffffff, docs/evidence/ios-host/2026-11-23-run-45230061151/ on \
                 m2-fs4d; native-theme/tests/next_homes.toml, row slider.thumb_color, a listed \
                 difference)",
    },
    Difference {
        leaf: "SwitchLook hover_track",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states switch.hover_unchecked_background #c8c8cd / \
                 #4a4a4c, uncited; the 0.8.0 preset states switch.track(HOVER).fill, the off \
                 track's rest fill, as AppKit draws a hovered switch as at rest (os-probe run \
                 45208260830, macOS 15, `*.hovered.switch_off` differs from the normal capture \
                 in 0 pixels, docs/evidence/os-probe/2026-11-23-run-45208260830/ on m2-fs4c; \
                 native-theme/tests/next_homes.toml, row switch.hover_unchecked_background, a \
                 listed difference)",
    },
    Difference {
        leaf: "SwitchLook hover_track",
        presets: &["adwaita"],
        reason: "0.7.2's adwaita states switch.hover_checked_background #2e78cc, uncited; the \
                 0.8.0 preset states switch.track(HOVER|CHECKED).fill, libadwaita's hover layer \
                 over the on track, currentColor, the accent's text, at 10 % over the accent, \
                 the whole hovered fill (`switch:checked { &:hover { background-image: \
                 image(color-mix(in srgb, currentColor 10%, transparent)); } }`, \
                 _switch.scss:41-45 at libadwaita 1.10.0; native-theme/tests/next_homes.toml, \
                 row switch.hover_checked_background, a listed difference)",
    },
    Difference {
        leaf: "SliderLook hover_thumb",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states slider.thumb_hover_color #e0e0e0 (light), \
                 uncited; the 0.8.0 preset states slider.thumb(HOVER).fill, the knob's rest \
                 fill, as AppKit draws a hovered knob as at rest (os-probe run 45198750893, \
                 macOS 15, `*.hovered.slider_knob` differs from the normal capture in 0 \
                 pixels, docs/evidence/os-probe/2026-11-23-run-45198750893/ on m2-fs4c; \
                 native-theme/tests/next_homes.toml, row slider.thumb_hover_color, a listed \
                 difference)",
    },
    Difference {
        leaf: "ThemeSource::focus_ring inner",
        presets: &["windows-11"],
        reason: "0.7.2 states no second ring; the 0.8.0 windows-11 states WinUI's focus visual \
                 with its 1 px inner ring in FocusStrokeColorInner, #b3ffffff / #b3000000 \
                 (platform-facts.md §2.1.5, focus_ring_color and focus_ring_offset; \
                 FocusVisualSecondaryThickness 1, ListViewItem_themeresources.xaml:252 at \
                 microsoft-ui-xaml 8463f45), which the ring reads at \
                 focus_ring.inner_stroke",
    },
];

/// The leaves compared and the differences found, for one preset and scheme.
struct Report {
    label: String,
    preset: &'static str,
    compared: usize,
    failures: Vec<String>,
    /// The listed differences found differing.
    listed: Vec<&'static str>,
}

impl Report {
    fn new(preset: &'static str, is_dark: bool) -> Self {
        let scheme = if is_dark { "dark" } else { "light" };
        Self {
            label: format!("{preset}/{scheme}"),
            preset,
            compared: 0,
            failures: Vec::new(),
            listed: Vec::new(),
        }
    }

    /// Compare one leaf: the 0.7.2 constructor's answer and the 0.8.0 one's.
    /// A leaf [`DIFFERENCES`] lists for this preset fails here: a listed
    /// difference is compared to the value its 0.8.0 home gives, through
    /// [`Report::moved`].
    fn leaf<T: PartialEq + Debug>(
        &mut self,
        leaf: &'static str,
        state: &str,
        old: Option<T>,
        new: Mapped<T>,
    ) {
        self.compared += 1;
        let differs = match (&old, &new) {
            (Some(old), Ok(new)) => old != new,
            _ => true,
        };
        if !differs {
            return;
        }
        if DIFFERENCES
            .iter()
            .any(|d| d.leaf == leaf && d.presets.contains(&self.preset))
        {
            self.failures.push(format!(
                "{}: {leaf}{state}: listed as a difference but compared to no home: compare it \
                 through Report::moved",
                self.label
            ));
            return;
        }
        let new = match new {
            Ok(new) => format!("{new:?}"),
            Err(Unmapped(home)) => format!("unmapped at {home}"),
        };
        self.failures.push(format!(
            "{}: {leaf}{state}: 0.7.2 {old:?}, 0.8.0 {new}",
            self.label
        ));
    }

    /// Compare one leaf a listed difference moves to the value `expected`
    /// computes from its 0.8.0 home: where [`DIFFERENCES`] lists the leaf for
    /// this preset, the 0.8.0 constructor's answer is `expected` and differs
    /// from the 0.7.2 one's; elsewhere the leaf is compared as [`Report::leaf`]
    /// compares it.
    fn moved<T: PartialEq + Debug>(
        &mut self,
        leaf: &'static str,
        state: &str,
        old: Option<T>,
        new: Mapped<T>,
        expected: Mapped<T>,
    ) {
        if !DIFFERENCES
            .iter()
            .any(|d| d.leaf == leaf && d.presets.contains(&self.preset))
        {
            self.leaf(leaf, state, old, new);
            return;
        }
        self.compared += 1;
        match (&old, &new, &expected) {
            (_, Ok(new), Ok(expected)) if new == expected => {
                if old.as_ref() != Some(new) {
                    self.listed.push(leaf);
                }
            }
            _ => self.failures.push(format!(
                "{}: {leaf}{state}: listed as moved to {expected:?}, 0.7.2 {old:?}, 0.8.0 {new:?}",
                self.label
            )),
        }
    }
}

/// The two resolved themes of `preset` for `is_dark`.
fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(ResolvedTheme, native_theme::next::ResolvedTheme)> {
    let (mode, ctx) = if is_dark {
        (ColorMode::Dark, ThemeContext::dark())
    } else {
        (ColorMode::Light, ThemeContext::light())
    };
    let old = crate::Theme::preset(key)?
        .into_variant(mode)?
        .into_resolved(&crate::next::parity::old_context(preset))?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, new))
}

/// `Some` of a value both models always give, as the 0.8.0 side's `Ok`.
fn given<T>(v: Option<T>) -> Mapped<T> {
    v.ok_or(Unmapped("(a value the 0.8.0 side gives as `None`)"))
}

/// The state a control shows, `checked` or not, apart from its disabled bit.
fn shown(checked: bool) -> WidgetState {
    if checked {
        WidgetState::CHECKED
    } else {
        WidgetState::NONE
    }
}

/// A colour of the window's colours, `home` its path.
fn window_colour(
    colour: Option<native_theme::next::Color>,
    home: &'static str,
) -> Mapped<gpui::Hsla> {
    crate::next::stated(colour, home).map(crate::next::hsla)
}

/// A hovered unselected tab's fill from its 0.8.0 homes: the fill a rule
/// states for `HOVER`, over the bar; else the tab at rest.
fn tab_hover_home(n: &native_theme::next::ResolvedTheme) -> Mapped<gpui::Hsla> {
    let t = &n.tab;
    match crate::next::state_fill_of(&t.tab(WidgetState::HOVER), "tab.tab(HOVER).fill")? {
        Some(hover) => Ok(crate::colors::over(
            fill(&t.bar(WidgetState::NONE), "tab.bar(NONE).fill")?,
            hover,
        )),
        None => fill(&t.tab(WidgetState::NONE), "tab.tab(NONE).fill"),
    }
}

/// An enabled slider's thumb under the pointer from its 0.8.0 homes: the
/// fill a rule states for `HOVER`, else the thumb at rest; `None` disabled.
fn slider_hover_home(
    n: &native_theme::next::ResolvedTheme,
    disabled: bool,
) -> Mapped<Option<gpui::Hsla>> {
    if disabled {
        return Ok(None);
    }
    let s = &n.slider;
    match crate::next::state_fill_of(&s.thumb(WidgetState::HOVER), "slider.thumb(HOVER).fill")? {
        Some(hover) => Ok(Some(hover)),
        None => fill(&s.thumb(WidgetState::NONE), "slider.thumb(NONE).fill").map(Some),
    }
}

/// An enabled switch's track under the pointer from its 0.8.0 homes: the
/// fill a rule states for `HOVER` (at the checked state where checked), else
/// the track at rest; `None` disabled.
fn switch_hover_home(
    n: &native_theme::next::ResolvedTheme,
    checked: bool,
    disabled: bool,
) -> Mapped<Option<gpui::Hsla>> {
    if disabled {
        return Ok(None);
    }
    let s = &n.switch;
    let ruled = s
        .track(WidgetState::HOVER)
        .from_state
        .contains(native_theme::next::PartStated::FILL);
    if ruled {
        fill(
            &s.track(WidgetState::HOVER | shown(checked)),
            "switch.track(HOVER|shown).fill",
        )
        .map(Some)
    } else {
        fill(&s.track(shown(checked)), "switch.track(shown).fill").map(Some)
    }
}

/// A switch's label colour from its 0.8.0 homes: disabled, the one a rule
/// states for `DISABLED`, else gpui-component's muted_foreground,
/// `colors.window.foreground_muted`; enabled, the stated one at its state,
/// else gpui-component's foreground, `colors.window.foreground`.
fn switch_label_home(
    n: &native_theme::next::ResolvedTheme,
    checked: bool,
    disabled: bool,
) -> Mapped<gpui::Hsla> {
    use native_theme::next::TextStated;
    let s = &n.switch;
    if disabled {
        if s.label(WidgetState::DISABLED)
            .from_state
            .contains(TextStated::COLOR)
        {
            crate::next::text_color(
                &s.label(WidgetState::DISABLED | shown(checked)),
                "switch.label(DISABLED|shown).color",
            )
        } else {
            window_colour(
                n.colors.window.foreground_muted,
                "colors.window.foreground_muted",
            )
        }
    } else {
        match crate::next::text_color_of(&s.label(shown(checked))) {
            Some(colour) => Ok(colour),
            None => window_colour(n.colors.window.foreground, "colors.window.foreground"),
        }
    }
}

/// A check box's or radio's label colour from its 0.8.0 homes, `label` the
/// label at a state: disabled, the one a rule states for `DISABLED`, else
/// gpui-component's muted_foreground, `colors.window.foreground_muted`;
/// enabled, the label at rest.
fn indicator_label_home(
    n: &native_theme::next::ResolvedTheme,
    label: &dyn Fn(WidgetState) -> native_theme::next::TextStyle,
    disabled: bool,
) -> Mapped<gpui::Hsla> {
    use native_theme::next::TextStated;
    if !disabled {
        return crate::next::text_color(&label(WidgetState::NONE), "label(NONE).color");
    }
    let off = label(WidgetState::DISABLED);
    if off.from_state.contains(TextStated::COLOR) {
        crate::next::text_color(&off, "label(DISABLED).color")
    } else {
        window_colour(
            n.colors.window.foreground_muted,
            "colors.window.foreground_muted",
        )
    }
}

/// A check box's or radio's hover outline from its 0.8.0 homes, `indicator`
/// the indicator at a state: where a rule states the stroke for `HOVER`, its
/// colour at the shown state; `None` where none does, or disabled.
fn hover_border_home(
    indicator: &dyn Fn(WidgetState) -> native_theme::next::PartStyle,
    checked: bool,
    disabled: bool,
) -> Mapped<Option<gpui::Hsla>> {
    let ruled = indicator(WidgetState::HOVER)
        .from_state
        .contains(native_theme::next::PartStated::STROKE);
    if disabled || !ruled {
        return Ok(None);
    }
    stroke_color(
        &indicator(WidgetState::HOVER | shown(checked)),
        "indicator(HOVER|shown).stroke.color",
    )
    .map(Some)
}

/// An enabled check box's or radio's fill under the pointer from its 0.8.0
/// homes: the fill resolved at `HOVER` and the shown state where a rule
/// states one; else unchecked the fill at rest, checked none; checked, none
/// too where it is the checked fill; `None` disabled.
fn hover_fill_home(
    indicator: &dyn Fn(WidgetState) -> native_theme::next::PartStyle,
    checked: bool,
    disabled: bool,
) -> Mapped<Option<gpui::Hsla>> {
    if disabled {
        return Ok(None);
    }
    let hover = crate::next::state_fill_of(
        &indicator(WidgetState::HOVER | shown(checked)),
        "indicator(HOVER|shown).fill",
    )?;
    if checked {
        let on = fill(&indicator(WidgetState::CHECKED), "indicator(CHECKED).fill")?;
        return Ok(hover.filter(|hover| *hover != on));
    }
    match hover {
        Some(hover) => Ok(Some(hover)),
        None => fill(&indicator(WidgetState::NONE), "indicator(NONE).fill").map(Some),
    }
}

/// A check box's or radio's outline from its 0.8.0 homes, `indicator` the
/// indicator at a state: enabled, the stroke's colour at the shown state
/// (disabled, compared on its own); the width at rest; and the mark's size
/// it leaves, the indicator's width less, on each side, the padding the
/// indicator states there, else the outline's width.
fn outline_home(
    indicator: &dyn Fn(WidgetState) -> native_theme::next::PartStyle,
    checked: bool,
    disabled: bool,
) -> Mapped<(gpui::Hsla, gpui::Pixels, gpui::Pixels)> {
    let colour = if disabled {
        gpui::transparent_black()
    } else {
        stroke_color(&indicator(shown(checked)), "indicator(shown).stroke.color")?
    };
    let rest = indicator(WidgetState::NONE);
    let width = crate::next::stroke_width(&rest, "indicator(NONE).stroke.width")?;
    let side = crate::next::stated(rest.min_size[0], "indicator(NONE).min_size[0]")?;
    let [top, right, bottom, left] = crate::next::padding_of(&rest);
    let inset = |pad: Option<f32>| pad.filter(|v| v.is_finite() && *v >= 0.0).unwrap_or(width);
    let mark = (side - inset(left) - inset(right)).min(side - inset(top) - inset(bottom));
    let length = |v: f32, home| super::length(v).ok_or(Unmapped(home));
    Ok((
        colour,
        length(width, "indicator(NONE).stroke.width")?,
        length(mark.max(0.0), "indicator(NONE).padding")?,
    ))
}

/// A switch's label font from its 0.8.0 home, `switch.label(NONE)`: each
/// property where it states it, else `None`, gpui-component's own.
fn switch_font_home(n: &native_theme::next::ResolvedTheme) -> Mapped<SwitchLabelFont> {
    use native_theme::next::TextStated;
    let text = n.switch.label(WidgetState::NONE);
    let has = |bit: TextStated| text.stated.contains(bit);
    Ok(SwitchLabelFont {
        family: has(TextStated::FAMILY).then(|| crate::font_family(&text.family)),
        size: if has(TextStated::SIZE_PX) {
            Some(super::length(text.size_px).ok_or(Unmapped("switch.label(NONE).size_px"))?)
        } else {
            None
        },
        weight: has(TextStated::WEIGHT).then(|| gpui::FontWeight(f32::from(text.weight))),
        style: if has(TextStated::STYLE) {
            Some(crate::next::stated(
                crate::next::font_style(text.style),
                "switch.label(NONE).style",
            )?)
        } else {
            None
        },
        line_height: n.typography.line_height_multiplier,
    })
}

/// Every look and every value beside one, of `preset` for `is_dark`.
fn compare_all(key: &'static str, preset: Preset, is_dark: bool) -> crate::Result<Report> {
    let (old, new) = resolve(key, preset, is_dark)?;
    let mut report = Report::new(key, is_dark);
    let r = &mut report;
    r.leaf(
        "SeparatorLook",
        "",
        SeparatorLook::of(&old),
        SeparatorLook::next_of(&new),
    );
    r.leaf(
        "ProgressBarLook",
        "",
        ProgressBarLook::of(&old),
        ProgressBarLook::next_of(&new),
    );
    for size in [
        Size::XSmall,
        Size::Small,
        Size::Medium,
        Size::Large,
        Size::Size(px(40.)),
    ] {
        r.leaf(
            "SpinnerLook",
            &format!(" {size:?}"),
            SpinnerLook::of_size(&old, size),
            SpinnerLook::next_of_size(&new, size),
        );
    }
    r.leaf(
        "SpinnerLook",
        " of",
        SpinnerLook::of(&old),
        SpinnerLook::next_of(&new),
    );
    let (old_tab, new_tab) = (TabLook::of(&old), TabLook::next_of(&new));
    // The hover and rest fills apart, so a listed difference of either
    // leaves the rest of the look compared.
    let unhovered = |look: TabLook| TabLook {
        hover: gpui::transparent_black(),
        idle: gpui::transparent_black(),
        active: gpui::transparent_black(),
        active_text: gpui::transparent_black(),
        ..look
    };
    r.moved(
        "TabLook::active",
        "",
        old_tab.map(|look| (look.active, look.active_text)),
        new_tab.map(|look| (look.active, look.active_text)),
        fill(
            &new.tab.tab(WidgetState::SELECTED),
            "tab.tab(SELECTED).fill",
        )
        .and_then(|fill| {
            crate::next::text_color(
                &new.tab.label(WidgetState::SELECTED),
                "tab.label(SELECTED).color",
            )
            .map(|text| (fill, text))
        }),
    );
    r.leaf(
        "TabLook",
        "",
        old_tab.map(unhovered),
        new_tab.map(unhovered),
    );
    r.moved(
        "TabLook::idle",
        "",
        old_tab.map(|look| look.idle),
        new_tab.map(|look| look.idle),
        fill(&new.tab.tab(WidgetState::NONE), "tab.tab(NONE).fill"),
    );
    r.moved(
        "TabLook::hover",
        "",
        old_tab.map(|look| look.hover),
        new_tab.map(|look| look.hover),
        tab_hover_home(&new),
    );
    for disabled in [false, true] {
        let state = format!(" disabled={disabled}");
        let (old_slider, new_slider) = (
            SliderLook::of(&old, disabled),
            SliderLook::next_of(&new, disabled),
        );
        // The thumb's fill and the hovered thumb apart, so a listed
        // difference of either leaves the rest of the look compared.
        let unhovered = |look: SliderLook| SliderLook {
            thumb_color: gpui::transparent_black(),
            hover_thumb: None,
            ..look
        };
        r.leaf(
            "SliderLook",
            &state,
            old_slider.map(unhovered),
            new_slider.map(unhovered),
        );
        // `SliderLook::next_of`: the thumb's fill, at `DISABLED` where the
        // slider is disabled.
        let thumb_state = if disabled {
            WidgetState::DISABLED
        } else {
            WidgetState::NONE
        };
        r.moved(
            "SliderLook thumb_color",
            &state,
            old_slider.map(|look| look.thumb_color),
            new_slider.map(|look| look.thumb_color),
            fill(&new.slider.thumb(thumb_state), "slider.thumb(shown).fill"),
        );
        r.moved(
            "SliderLook hover_thumb",
            &state,
            old_slider.map(|look| look.hover_thumb),
            new_slider.map(|look| look.hover_thumb),
            slider_hover_home(&new, disabled),
        );
        r.leaf(
            "ThumbOutline",
            &state,
            Some(ThumbOutline::of(&old, disabled)),
            ThumbOutline::next_of(&new, disabled),
        );
        for checked in [false, true] {
            let state = format!(" checked={checked} disabled={disabled}");
            let (old_switch, new_switch) = (
                SwitchLook::of(&old, checked, disabled),
                SwitchLook::next_of(&new, checked, disabled),
            );
            // The label apart, so a listed difference of the disabled label
            // leaves the rest of the look compared.
            // Disabled, the track apart too, which the 0.8.0 look dims as gpui-component does
            // where no rule states the disabled track's fill.
            let unlabelled = |look: SwitchLook| SwitchLook {
                label: gpui::transparent_black(),
                label_gap: None,
                hover_track: None,
                track: if disabled {
                    gpui::transparent_black()
                } else {
                    look.track
                },
                ..look
            };
            if disabled {
                let shown = if checked {
                    WidgetState::CHECKED
                } else {
                    WidgetState::NONE
                };
                let track = |s: WidgetState, home| fill(&new.switch.track(s), home);
                r.moved(
                    "SwitchLook track disabled",
                    &state,
                    old_switch.map(|look| look.track),
                    new_switch.map(|look| look.track),
                    if new
                        .switch
                        .track(WidgetState::DISABLED)
                        .from_state
                        .contains(native_theme::next::PartStated::FILL)
                    {
                        track(
                            WidgetState::DISABLED | shown,
                            "switch.track(DISABLED|shown).fill",
                        )
                    } else {
                        track(shown, "switch.track(shown).fill")
                            .map(|c| c.opacity(super::switch::DISABLED_TRACK_ALPHA))
                    },
                );
            }
            r.moved(
                "SwitchLook::label_gap",
                &state,
                old_switch.map(|look| look.label_gap),
                new_switch.map(|look| look.label_gap),
                new.switch
                    .label_gap
                    .map(|gap| super::length(gap).ok_or(Unmapped("switch.label_gap")))
                    .transpose(),
            );
            r.leaf(
                "SwitchLook",
                &state,
                old_switch.map(unlabelled),
                new_switch.map(unlabelled),
            );
            r.moved(
                "SwitchLook hover_track",
                &state,
                old_switch.map(|look| look.hover_track),
                new_switch.map(|look| look.hover_track),
                switch_hover_home(&new, checked, disabled),
            );
            r.moved(
                if disabled {
                    "SwitchLook::label disabled"
                } else {
                    "SwitchLook::label"
                },
                &state,
                old_switch.map(|look| look.label),
                new_switch.map(|look| look.label),
                switch_label_home(&new, checked, disabled),
            );
            // The hover outline and the hover fill apart, so a listed difference of either leaves
            // the rest of the look compared.
            // The label apart too, so a listed difference of the disabled label leaves the rest
            // of the look compared.
            // Disabled, the fill and the mark apart too, which the 0.8.0 look dims as
            // gpui-component does where no rule states the disabled fill or mark.
            // The outline, its colour and width and the mark's size it insets, apart too, so a
            // listed difference of it leaves the rest of the look compared; disabled its colour
            // on its own, which the 0.8.0
            // look dims where no rule states the disabled stroke.
            // Enabled, the fill and the mark apart too, and the whole control's opacity, so a
            // listed difference of one leaves the rest of the look compared.
            let unframed = |look: CheckboxLook| CheckboxLook {
                hover_border: None,
                hover_fill: None,
                label: gpui::transparent_black(),
                mark: gpui::transparent_black(),
                border: gpui::transparent_black(),
                border_width: px(0.),
                mark_size: px(0.),
                fill: gpui::transparent_black(),
                opacity: 1.,
                ..look
            };
            let outline = |look: CheckboxLook| {
                (
                    if disabled {
                        gpui::transparent_black()
                    } else {
                        look.border
                    },
                    look.border_width,
                    look.mark_size,
                )
            };
            let text = |look: CheckboxLook| look.label;
            let boxed = |s| new.checkbox.indicator(s);
            let round = |s| new.radio.indicator(s);
            for (look, radio) in [("CheckboxLook::of", false), ("CheckboxLook::radio", true)] {
                let (old_look, new_look) = if radio {
                    (
                        CheckboxLook::radio(&old, checked, disabled),
                        CheckboxLook::next_radio(&new, checked, disabled),
                    )
                } else {
                    (
                        CheckboxLook::of(&old, checked, disabled),
                        CheckboxLook::next_of(&new, checked, disabled),
                    )
                };
                let indicator: &dyn Fn(WidgetState) -> native_theme::next::PartStyle =
                    if radio { &round } else { &boxed };
                let label = |s| {
                    if radio {
                        new.radio.label(s)
                    } else {
                        new.checkbox.label(s)
                    }
                };
                let leaf = |name: &'static str, radio_name: &'static str| {
                    if radio { radio_name } else { name }
                };
                r.leaf(look, &state, old_look.map(unframed), new_look.map(unframed));
                if !disabled {
                    r.moved(
                        leaf("CheckboxLook::of fill", "CheckboxLook::radio fill"),
                        &state,
                        old_look.map(|look| look.fill),
                        new_look.map(|look| look.fill),
                        fill(&indicator(shown(checked)), "indicator(shown).fill"),
                    );
                    r.moved(
                        leaf("CheckboxLook::of mark", "CheckboxLook::radio mark"),
                        &state,
                        old_look.map(|look| look.mark),
                        new_look.map(|look| look.mark),
                        if radio {
                            fill(
                                &new.radio.dot(WidgetState::CHECKED),
                                "radio.dot(CHECKED).fill",
                            )
                        } else {
                            Ok(crate::next::hsla(
                                new.checkbox.checkmark(WidgetState::CHECKED).color,
                            ))
                        },
                    );
                }
                r.moved(
                    leaf("CheckboxLook::of opacity", "CheckboxLook::radio opacity"),
                    &state,
                    old_look.map(|look| look.opacity),
                    new_look.map(|look| look.opacity),
                    Ok(super::next_disabled_opacity(
                        disabled,
                        if radio {
                            new.radio.disabled_opacity
                        } else {
                            new.checkbox.disabled_opacity
                        },
                    )),
                );
                r.moved(
                    leaf("CheckboxLook::of outline", "CheckboxLook::radio outline"),
                    &state,
                    old_look.map(outline),
                    new_look.map(outline),
                    outline_home(indicator, checked, disabled),
                );
                r.moved(
                    match (radio, disabled) {
                        (false, true) => "CheckboxLook::of label disabled",
                        (false, false) => "CheckboxLook::of label",
                        (true, true) => "CheckboxLook::radio label disabled",
                        (true, false) => "CheckboxLook::radio label",
                    },
                    &state,
                    old_look.map(text),
                    new_look.map(text),
                    indicator_label_home(&new, &label, disabled),
                );
                r.moved(
                    leaf(
                        "CheckboxLook::of hover_border",
                        "CheckboxLook::radio hover_border",
                    ),
                    &state,
                    old_look.map(|look| look.hover_border),
                    new_look.map(|look| look.hover_border),
                    hover_border_home(indicator, checked, disabled),
                );
                r.moved(
                    leaf(
                        "CheckboxLook::of hover_fill",
                        "CheckboxLook::radio hover_fill",
                    ),
                    &state,
                    old_look.map(|look| look.hover_fill),
                    new_look.map(|look| look.hover_fill),
                    hover_fill_home(indicator, checked, disabled),
                );
            }
            let (old_box, new_box) = (
                CheckboxLook::of(&old, checked, disabled),
                CheckboxLook::next_of(&new, checked, disabled),
            );
            let (old_radio, new_radio) = (
                CheckboxLook::radio(&old, checked, disabled),
                CheckboxLook::next_radio(&new, checked, disabled),
            );
            if disabled {
                let shown = if checked {
                    WidgetState::CHECKED
                } else {
                    WidgetState::NONE
                };
                let dimmed = |stroke: Mapped<gpui::Hsla>| {
                    stroke.map(|colour| colour.opacity(DISABLED_INDICATOR_ALPHA))
                };
                // The fill disabled: the one a rule states for `DISABLED` (at the checked
                // state), else the enabled one, dimmed where gpui-component dims it (a checked
                // box, a radio checked or not).
                let disabled_fill =
                    |indicator: &dyn Fn(WidgetState) -> native_theme::next::PartStyle,
                     radio: bool,
                     home: &'static str,
                     stated_home: &'static str| {
                        if indicator(WidgetState::DISABLED)
                            .from_state
                            .contains(native_theme::next::PartStated::FILL)
                        {
                            fill(&indicator(WidgetState::DISABLED | shown), stated_home)
                        } else if checked || radio {
                            dimmed(fill(&indicator(shown), home))
                        } else {
                            fill(&indicator(shown), home)
                        }
                    };
                r.moved(
                    "CheckboxLook::of fill disabled",
                    &state,
                    old_box.map(|look| look.fill),
                    new_box.map(|look| look.fill),
                    disabled_fill(
                        &|s| new.checkbox.indicator(s),
                        false,
                        "checkbox.indicator(shown).fill",
                        "checkbox.indicator(DISABLED|shown).fill",
                    ),
                );
                r.moved(
                    "CheckboxLook::radio fill disabled",
                    &state,
                    old_radio.map(|look| look.fill),
                    new_radio.map(|look| look.fill),
                    disabled_fill(
                        &|s| new.radio.indicator(s),
                        true,
                        "radio.indicator(shown).fill",
                        "radio.indicator(DISABLED|shown).fill",
                    ),
                );
                // The border disabled: the stroke a rule states for `DISABLED` (at the checked
                // state), else the enabled one at half alpha, as gpui-component dims it.
                let disabled_border =
                    |indicator: &dyn Fn(WidgetState) -> native_theme::next::PartStyle,
                     home: &'static str,
                     stated_home: &'static str| {
                        if indicator(WidgetState::DISABLED)
                            .from_state
                            .contains(native_theme::next::PartStated::STROKE)
                        {
                            stroke_color(&indicator(WidgetState::DISABLED | shown), stated_home)
                        } else {
                            dimmed(stroke_color(&indicator(shown), home))
                        }
                    };
                r.moved(
                    "CheckboxLook::of border disabled",
                    &state,
                    old_box.map(|look| look.border),
                    new_box.map(|look| look.border),
                    disabled_border(
                        &|s| new.checkbox.indicator(s),
                        "checkbox.indicator(shown).stroke.color",
                        "checkbox.indicator(DISABLED|shown).stroke.color",
                    ),
                );
                r.moved(
                    "CheckboxLook::radio border disabled",
                    &state,
                    old_radio.map(|look| look.border),
                    new_radio.map(|look| look.border),
                    disabled_border(
                        &|s| new.radio.indicator(s),
                        "radio.indicator(shown).stroke.color",
                        "radio.indicator(DISABLED|shown).stroke.color",
                    ),
                );
                // The mark disabled: the one a rule states for `DISABLED`, else the
                // enabled one at half alpha, as gpui-component dims a disabled mark.
                let mark_ruled = new
                    .checkbox
                    .checkmark(WidgetState::DISABLED)
                    .from_state
                    .contains(native_theme::next::IconStated::COLOR);
                let box_mark = new
                    .checkbox
                    .checkmark(if mark_ruled {
                        WidgetState::DISABLED | WidgetState::CHECKED
                    } else {
                        WidgetState::CHECKED
                    })
                    .color;
                let box_mark = Ok(crate::next::hsla(box_mark));
                r.moved(
                    "CheckboxLook::of mark disabled",
                    &state,
                    old_box.map(|look| look.mark),
                    new_box.map(|look| look.mark),
                    if mark_ruled {
                        box_mark
                    } else {
                        dimmed(box_mark)
                    },
                );
                let dot_ruled = new
                    .radio
                    .dot(WidgetState::DISABLED)
                    .from_state
                    .contains(native_theme::next::PartStated::FILL);
                r.moved(
                    "CheckboxLook::radio mark disabled",
                    &state,
                    old_radio.map(|look| look.mark),
                    new_radio.map(|look| look.mark),
                    if dot_ruled {
                        fill(
                            &new.radio.dot(WidgetState::DISABLED | WidgetState::CHECKED),
                            "radio.dot(DISABLED|CHECKED).fill",
                        )
                    } else {
                        dimmed(fill(
                            &new.radio.dot(WidgetState::CHECKED),
                            "radio.dot(CHECKED).fill",
                        ))
                    },
                );
            }
        }
    }
    let (o, n) = (
        ThemeSource::Old(Native::unscaled(&old)),
        ThemeSource::Next(&new),
    );
    r.leaf(
        "ThemeSource::background",
        "",
        o.background(),
        given(n.background()),
    );
    // The second ring apart, so a listed difference of it leaves the rest of the ring compared.
    let (old_ring, new_ring) = (
        FocusRing::of(&old),
        FocusRing::next_of(&new).and_then(given),
    );
    let outer = |ring: FocusRing| FocusRing {
        inner: None,
        ..ring
    };
    r.leaf(
        "ThemeSource::focus_ring",
        "",
        old_ring.map(outer),
        new_ring.map(outer),
    );
    r.moved(
        "ThemeSource::focus_ring inner",
        "",
        old_ring.map(|ring| ring.inner),
        new_ring.map(|ring| ring.inner),
        match &new.focus_ring.inner_stroke {
            native_theme::next::Stroke::Uniform { width, color, .. } => {
                Ok(Some((crate::next::hsla(*color), *width)))
            }
            _ => Ok(None),
        },
    );
    r.leaf(
        "ThemeSource::keyboard_only",
        "",
        Some(o.keyboard_only()),
        Ok(n.keyboard_only()),
    );
    r.leaf(
        "ThemeSource::body_text",
        "",
        o.body_text(),
        given(n.body_text()),
    );
    // The family and the slant apart from the size, weight and line height,
    // so a listed difference of either leaves the other compared.
    let old_font = o
        .body_text()
        .map(|(size, line_height, weight)| SwitchLabelFont {
            family: None,
            size: Some(size),
            weight: Some(weight),
            style: None,
            line_height,
        });
    let (new_font, font_home) = (SwitchLabelFont::next_of(&new), switch_font_home(&new));
    let sized = |font: &SwitchLabelFont| (font.size, font.weight, font.line_height);
    let faced = |font: &SwitchLabelFont| (font.family.clone(), font.style);
    r.moved(
        "SwitchLabelFont",
        "",
        old_font.as_ref().map(sized),
        new_font.as_ref().map(sized).map_err(|e| *e),
        font_home.as_ref().map(sized).map_err(|e| *e),
    );
    r.moved(
        "SwitchLabelFont family and style",
        "",
        old_font.as_ref().map(faced),
        new_font.as_ref().map(faced).map_err(|e| *e),
        font_home.as_ref().map(faced).map_err(|e| *e),
    );
    r.leaf(
        "LabelFont checkbox",
        "",
        Some(LabelFont::old(&old.checkbox.font, Native::unscaled(&old))),
        LabelFont::next(
            &new.checkbox.label(WidgetState::NONE),
            new.typography.line_height_multiplier,
            "checkbox.label(NONE)",
        ),
    );
    r.leaf(
        "LabelFont radio",
        "",
        Some(LabelFont::old(&old.checkbox.font, Native::unscaled(&old))),
        LabelFont::next(
            &new.radio.label(WidgetState::NONE),
            new.typography.line_height_multiplier,
            "radio.label(NONE)",
        ),
    );
    r.leaf(
        "LabelFont tab",
        "",
        Some(LabelFont::old(&old.tab.font, Native::unscaled(&old))),
        LabelFont::next(
            &new.tab.label(WidgetState::NONE),
            new.typography.line_height_multiplier,
            "tab.label(NONE)",
        ),
    );
    r.leaf(
        "spinner colours",
        "",
        Some((old.spinner.fill_color, old.defaults.text_color)),
        given(
            new.colors
                .window
                .foreground
                .map(|text| (rgba(new.spinner.color), rgba(text))),
        ),
    );
    Ok(report)
}

#[test]
fn the_compared_presets_are_every_platform_preset() {
    let compared: Vec<Preset> = PRESETS.iter().map(|&(_, preset)| preset).collect();
    let misnamed: Vec<&str> = PRESETS
        .iter()
        .filter(|&&(key, preset)| key != preset.as_str())
        .map(|&(key, _)| key)
        .collect();
    let missing: Vec<&Preset> = Preset::ALL
        .iter()
        .filter(|p| !compared.contains(p))
        .collect();
    assert!(
        compared.len() == Preset::ALL.len() && missing.is_empty() && misnamed.is_empty(),
        "{} presets compared, {} platform presets; not compared: {missing:?}; keys that are not \
         their preset's name: {misnamed:?}",
        compared.len(),
        Preset::ALL.len()
    );
}

#[test]
fn every_look_is_the_same_from_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut listed = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let report = compare_all(key, preset, is_dark)?;
            println!("{}: {} leaves compared", report.label, report.compared);
            failures.extend(report.failures);
            listed.extend(report.listed.into_iter().map(|leaf| (key, leaf)));
        }
    }
    for d in DIFFERENCES {
        for preset in d.presets {
            if !listed.contains(&(*preset, d.leaf)) {
                failures.push(format!(
                    "{preset}: {}: listed as a difference ({}), but equal: remove the entry",
                    d.leaf, d.reason
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

/// Every look constructor that takes a 0.7.2 theme has a 0.8.0 counterpart in
/// its file, named `next_` and its name.
#[test]
fn every_look_constructor_has_a_next_counterpart() {
    let files = [
        include_str!("checkbox.rs"),
        include_str!("progress.rs"),
        include_str!("separator.rs"),
        include_str!("slider.rs"),
        include_str!("spinner.rs"),
        include_str!("switch.rs"),
        include_str!("tab_bar.rs"),
    ];
    let mut constructors = 0;
    let mut missing = Vec::new();
    for src in files {
        for line in src.lines() {
            let Some(rest) = line.strip_prefix("    pub fn ") else {
                continue;
            };
            if !rest.contains("(resolved: &ResolvedTheme") {
                continue;
            }
            let Some(name) = rest.split(['(', '<']).next() else {
                continue;
            };
            constructors += 1;
            if !src.contains(&format!("    pub fn next_{name}(")) {
                missing.push(name.to_owned());
            }
        }
    }
    assert!(
        constructors >= 10 && missing.is_empty(),
        "{constructors} constructors, without a counterpart: {missing:?}"
    );
}
