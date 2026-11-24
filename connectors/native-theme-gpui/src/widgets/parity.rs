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
//! its reason; an entry that no longer differs fails.

use std::fmt::Debug;

use gpui::px;
use gpui_component::Size;
use native_theme::next::{PartStated, Preset, ThemeContext, WidgetState};

use super::{
    CheckboxLook, FocusRing, LabelFont, ProgressBarLook, SeparatorLook, SliderLook, SpinnerLook,
    SwitchLabelFont, SwitchLook, TabLook, ThemeSource, ThumbOutline, rgba,
};
use crate::next::{Mapped, Unmapped};
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
                 difference); the disabled checked mark is surface where 0.7.2 draws it in \
                 its disabled label colour (selected-disabled-icon-color, \
                 _md-comp-checkbox.scss:50, v0_192 at material-web cbd34a8; the look reads \
                 checkbox.checkmark(DISABLED|CHECKED).color, CheckboxLook::next_of)",
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
        leaf: "CheckboxLook::of",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 outlines the box in its uncited default line, 0.5 px in the \
                 separator, checked or not; 0.8.0's ios-13 states the outline WebKit strokes \
                 round an empty iOS check box, checkboxRadioBorderColor (secondaryLabel), \
                 1.5 px as it shows (checkboxRadioBorderWidth × 2 clipped to the edge), and \
                 none round a checked one (RenderThemeIOS.mm:1354, :1357-1368, :1483-1491, \
                 :1502 at WebKit 31adf9d; native-theme/tests/next_homes.toml, rows \
                 checkbox.unchecked_border_color, checkbox.border.color and \
                 checkbox.border.line_width, listed differences); the outline's colour is \
                 compared apart (CheckboxLook::of border), its width here; and an empty box \
                 at rest in WebKit's -webkit-control-background, systemBackground #ffffff / \
                 #000000, in place of 0.7.2's uncited #ffffff / #1c1c1e \
                 (checkboxRadioBackgroundColor, RenderThemeIOS.mm:1391, :1395, :1027 at WebKit \
                 31adf9d; system-color-for-css-value-expected.txt:7, :9 at WebKit de320ba; \
                 native-theme/tests/next_homes.toml, row \
                 checkbox.unchecked_background, a listed difference)",
    },
    Difference {
        leaf: "CheckboxLook::radio",
        presets: &["ios-13"],
        reason: "as the check box's (CheckboxLook::of): the radio button's indicator is the \
                 same part on ios-13",
    },
    Difference {
        leaf: "CheckboxLook::of",
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
        leaf: "CheckboxLook::radio",
        presets: &["kde-breeze", "windows-11"],
        reason: "as the check box's (CheckboxLook::of): the disabled checked radio's circle \
                 is filled as the box (breezehelper.cpp:1012-1019; \
                 RadioButtonOuterEllipseCheckedFillDisabled, RadioButton_themeresources.xaml:149, \
                 :33)",
    },
    Difference {
        leaf: "CheckboxLook::radio",
        presets: &["material-3"],
        reason: "0.7.2 fills a checked or disabled radio button as its check box and draws \
                 the dot in on-primary; 0.8.0's material-3 states material-web's radio, a ring \
                 and a dot in primary with nothing between them in any state, disabled and \
                 checked both on-surface at 0.38 (radio/internal/radio.ts:108-120, \
                 _radio.scss:126-127, :180-183, _md-comp-radio-button.scss:28-29, :45, v0_192 \
                 at material-web cbd34a8; native-theme/tests/next_homes.toml, rows \
                 checkbox.checked_background, checkbox.indicator_color and \
                 checkbox.disabled_background, listed differences)",
    },
    Difference {
        leaf: "SliderLook::hover_thumb",
        presets: &["adwaita"],
        reason: "0.7.2's adwaita states an uncited slider.thumb_hover_color (#ebebeb / \
                 #3a3a3e); 0.8.0's adwaita states libadwaita's $slider_hover_color, white in \
                 both schemes (_scale.scss:62-76, _colors.scss:289 at libadwaita 1.10.0; \
                 native-theme/tests/next_homes.toml, row slider.thumb_hover_color, a listed \
                 difference), so the look's hovered thumb is white",
    },
    Difference {
        leaf: "SwitchLabelFont",
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
];

/// The listed differences whose 0.8.0 value the test computes from the
/// home the reason names: the 0.7.2 look's leaf set to the home's value
/// through the conversion the 0.8.0 constructor applies to it, without
/// calling that constructor ([`Report::home_leaf`]). The leaf must differ
/// from 0.7.2's and equal that value; a home left unstated fails.
const FROM_HOME: &[Difference] = &[
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
        leaf: "CheckboxLook::of border",
        presets: &["adwaita", "windows-11", "ios-13"],
        reason: "0.7.2's adwaita and windows-11 outline a checked box in the default border \
                 colour, ios-13 every box in its uncited default line (the separator); 0.8.0's \
                 adwaita states none round a checked box, as libadwaita draws a checked or mixed \
                 check with `box-shadow: none`, its only outline (_checks.scss:27, :52-56 at \
                 libadwaita 1.10.0); windows-11 outlines it in AccentFillColorDefault, the \
                 accent's fill, #0067c0 / #4cc2ff (CheckBoxCheckBackgroundStrokeChecked, \
                 CheckBox_themeresources.xaml:221, :45 at microsoft-ui-xaml 8463f45); ios-13 \
                 strokes an empty box in checkboxRadioBorderColor (secondaryLabel) and a checked \
                 one in none (RenderThemeIOS.mm:1354, :1357-1368, :1483-1491, :1502 at WebKit \
                 31adf9d) (native-theme/tests/next_homes.toml, rows checkbox.border.color and \
                 checkbox.unchecked_border_color, listed differences); the look reads \
                 checkbox.indicator(CHECKED).stroke.color and checkbox.indicator(NONE).stroke.color",
    },
    Difference {
        leaf: "CheckboxLook::radio border",
        presets: &["adwaita", "windows-11", "ios-13"],
        reason: "as the check box's (CheckboxLook::of border): libadwaita's rule covers the \
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
        leaf: "CheckboxLook::of label disabled",
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
                 WebKit 31adf9d); the label as before",
    },
    Difference {
        leaf: "CheckboxLook::radio label disabled",
        presets: &["kde-breeze", "windows-11", "ios-13"],
        reason: "as the check box's (CheckboxLook::of label disabled), the dot at \
                 radio.dot(DISABLED|CHECKED).fill: kde-breeze palette.text() in the Disabled \
                 group (breezehelper.cpp:1059); windows-11 RadioButtonCheckGlyphFillDisabled, \
                 the rest's TextOnAccentFillColorPrimary as the source names it \
                 (RadioButton_themeresources.xaml:153, :37, :333; the Color-for-a-Brush key's \
                 drawn result not measured); ios-13 tertiaryLabel (RenderThemeIOS.mm:1578-1579)",
    },
    Difference {
        leaf: "CheckboxLook::of label disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "the macOS presets state no disabled mark (no measurement of AppKit's disabled \
                 checked box yet), so the look draws gpui-component's own disabled mark, the \
                 checked mark at half its opacity (gpui-component 0.10.0 src/checkbox.rs:230-234; \
                 docs/v0.8.0-spec.md §10, rule 0), where 0.7.2 draws the checked mark",
    },
    Difference {
        leaf: "CheckboxLook::radio label disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "as the check box's (CheckboxLook::of label disabled): gpui-component's radio \
                 draws the same mark (src/radio.rs:249)",
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
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states tab.hover_background #e8e8ed / #3a3a3c, \
                 uncited; the 0.8.0 preset states tab.tab(HOVER).fill, the tab's rest fill, \
                 the window's background, as AppKit draws a hovered tab as at rest (os-probe \
                 run 45198750893, macOS 15, `*.hovered.tab` differs from the normal capture in \
                 0 pixels, docs/evidence/os-probe/2026-11-23-run-45198750893/ on m2-fs4c; \
                 native-theme/tests/next_homes.toml, row tab.hover_background, a listed \
                 difference), drawn over the bar",
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
        leaf: "SliderLook::hover_thumb",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states slider.thumb_hover_color #e0e0e0 (light), \
                 uncited; the 0.8.0 preset states slider.thumb(HOVER).fill, the knob's rest \
                 fill, as AppKit draws a hovered knob as at rest (os-probe run 45198750893, \
                 macOS 15, `*.hovered.slider_knob` differs from the normal capture in 0 \
                 pixels, docs/evidence/os-probe/2026-11-23-run-45198750893/ on m2-fs4c; \
                 native-theme/tests/next_homes.toml, row slider.thumb_hover_color, a listed \
                 difference)",
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
    fn leaf<T: PartialEq + Debug>(
        &mut self,
        leaf: &'static str,
        state: &str,
        old: Option<T>,
        new: Mapped<T>,
    ) {
        self.compare(leaf, state, old, new, None);
    }

    /// Compare one leaf as [`Report::leaf`] does, and where a [`FROM_HOME`]
    /// entry lists it on this preset, the 0.8.0 answer with `expected`, the
    /// value computed from the entry's home (`None` where it is unstated).
    fn home_leaf<T: PartialEq + Debug>(
        &mut self,
        leaf: &'static str,
        state: &str,
        old: Option<T>,
        new: Mapped<T>,
        expected: Option<T>,
    ) {
        self.compare(leaf, state, old, new, Some(expected));
    }

    fn compare<T: PartialEq + Debug>(
        &mut self,
        leaf: &'static str,
        state: &str,
        old: Option<T>,
        new: Mapped<T>,
        expected: Option<Option<T>>,
    ) {
        self.compared += 1;
        let differs = match (&old, &new) {
            (Some(old), Ok(new)) => old != new,
            _ => true,
        };
        if !differs {
            return;
        }
        let computed = FROM_HOME
            .iter()
            .find(|d| d.leaf == leaf && d.presets.contains(&self.preset));
        let described = match (computed, &expected, &new) {
            (None, _, _) => DIFFERENCES
                .iter()
                .any(|d| d.leaf == leaf && d.presets.contains(&self.preset)),
            (Some(_), Some(Some(expected)), Ok(new)) => expected == new,
            (Some(_), _, _) => false,
        };
        if described {
            self.listed.push(leaf);
            return;
        }
        let new = match new {
            Ok(new) => format!("{new:?}"),
            Err(Unmapped(home)) => format!("unmapped at {home}"),
        };
        let listed = match (computed, expected) {
            (Some(d), Some(expected)) => format!(
                ", from its home {expected:?}: the listed difference ({}) no longer \
                 describes it",
                d.reason
            ),
            (Some(d), None) => format!(
                ": listed ({}) as computed from its home, but compared without it",
                d.reason
            ),
            (None, _) => String::new(),
        };
        self.failures.push(format!(
            "{}: {leaf}{state}: 0.7.2 {old:?}, 0.8.0 {new}{listed}",
            self.label
        ));
    }
}

/// The mark and the label of a disabled check box (`radio` false) or radio
/// button, from their homes as `CheckboxLook::next_of` converts them: the
/// mark at `checkbox.checkmark(DISABLED|CHECKED).color`
/// (`radio.dot(DISABLED|CHECKED).fill`) where a rule states it for
/// `DISABLED`, else the checked mark at gpui-component's own disabled
/// opacity (spec §10, rule 0); the label at `checkbox.label(DISABLED).color`
/// (`radio.label`). `None` where `disabled` is false or a home is unstated.
fn disabled_text_of(
    new: &native_theme::next::ResolvedTheme,
    radio: bool,
    disabled: bool,
) -> Option<(gpui::Hsla, gpui::Hsla)> {
    use native_theme::next::IconStated;
    if !disabled {
        return None;
    }
    let checked_disabled = WidgetState::DISABLED | WidgetState::CHECKED;
    let (stated, mark) = if radio {
        let stated = new
            .radio
            .dot(WidgetState::DISABLED)
            .from_state
            .contains(PartStated::FILL);
        let at = if stated {
            checked_disabled
        } else {
            WidgetState::CHECKED
        };
        (stated, crate::next::fill_of(&new.radio.dot(at))?)
    } else {
        let stated = new
            .checkbox
            .checkmark(WidgetState::DISABLED)
            .from_state
            .contains(IconStated::COLOR);
        let at = if stated {
            checked_disabled
        } else {
            WidgetState::CHECKED
        };
        let mark = new.checkbox.checkmark(at);
        if !mark.stated.contains(IconStated::COLOR) {
            return None;
        }
        (stated, crate::next::hsla(mark.color))
    };
    let mark = if stated {
        mark
    } else {
        mark.opacity(super::checkbox::UPSTREAM_DISABLED_MARK_OPACITY)
    };
    let label = if radio {
        new.radio.label(WidgetState::DISABLED)
    } else {
        new.checkbox.label(WidgetState::DISABLED)
    };
    let label = crate::next::text_color(&label, "checkbox.label(DISABLED).color").ok()?;
    Some((mark, label))
}

/// The outline of a check box (`radio` false) or radio button, from its homes as
/// `CheckboxLook::next_of` converts them: at rest the stroke colour of `indicator(CHECKED)`
/// where `checked`, else of `indicator(NONE)`; under the pointer, where a rule states a stroke
/// for `HOVER` and the control is enabled, the stroke colour of `indicator(HOVER|CHECKED)` or
/// `indicator(HOVER)`, else none. `None` where a home is unstated.
fn outline_of(
    new: &native_theme::next::ResolvedTheme,
    radio: bool,
    checked: bool,
    disabled: bool,
) -> Option<(gpui::Hsla, Option<gpui::Hsla>)> {
    let indicator = |state: WidgetState| {
        if radio {
            new.radio.indicator(state)
        } else {
            new.checkbox.indicator(state)
        }
    };
    let shown = if checked {
        WidgetState::CHECKED
    } else {
        WidgetState::NONE
    };
    let color = |state: WidgetState| crate::next::stroke_color(&indicator(state), "").ok();
    let hovers = !disabled
        && indicator(WidgetState::HOVER)
            .from_state
            .contains(PartStated::STROKE);
    let hover = if hovers {
        Some(color(WidgetState::HOVER | shown)?)
    } else {
        None
    };
    Some((color(shown)?, hover))
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
    // The hover fill apart, so a listed difference of it leaves the rest of
    // the look compared.
    let unhovered = |look: TabLook| TabLook {
        hover: gpui::transparent_black(),
        ..look
    };
    r.leaf(
        "TabLook",
        "",
        old_tab.map(unhovered),
        new_tab.map(unhovered),
    );
    // `TabLook::next_of`: the hovered tab's stated fill over the bar.
    let tab_hover = crate::next::state_fill_of(&new.tab.tab(WidgetState::HOVER))
        .zip(old_tab)
        .map(|(hover, look)| super::over(look.bar, hover));
    r.home_leaf(
        "TabLook::hover",
        "",
        old_tab.map(|look| look.hover),
        new_tab.map(|look| look.hover),
        tab_hover,
    );
    for disabled in [false, true] {
        let state = format!(" disabled={disabled}");
        let (old_slider, new_slider) = (
            SliderLook::of(&old, disabled),
            SliderLook::next_of(&new, disabled),
        );
        // The hovered thumb apart, so a listed difference of it leaves the
        // rest of the look compared.
        let unhovered = |look: SliderLook| SliderLook {
            hover_thumb: None,
            ..look
        };
        r.leaf(
            "SliderLook",
            &state,
            old_slider.map(unhovered),
            new_slider.map(unhovered),
        );
        // `SliderLook::next_of`: the hovered thumb's stated fill, whole,
        // where the slider is enabled.
        let thumb_hover = crate::next::state_fill_of(&new.slider.thumb(WidgetState::HOVER))
            .map(|hover| (!disabled).then_some(hover));
        r.home_leaf(
            "SliderLook::hover_thumb",
            &state,
            old_slider.map(|look| look.hover_thumb),
            new_slider.map(|look| look.hover_thumb),
            thumb_hover,
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
            let unlabelled = |look: SwitchLook| SwitchLook {
                label: gpui::transparent_black(),
                label_gap: None,
                hover_track: None,
                ..look
            };
            r.home_leaf(
                "SwitchLook::label_gap",
                &state,
                old_switch.map(|look| look.label_gap),
                new_switch.map(|look| look.label_gap),
                new.switch.label_gap.and_then(super::length).map(Some),
            );
            r.leaf(
                "SwitchLook",
                &state,
                old_switch.map(unlabelled),
                new_switch.map(unlabelled),
            );
            // `SwitchLook::next_of`: the hovered track's stated fill, whole
            // (on, at `HOVER|CHECKED` where a rule states it for `HOVER`),
            // where the switch is enabled.
            let hovered = new.switch.track(WidgetState::HOVER);
            let track_hover = if checked {
                hovered
                    .from_state
                    .contains(PartStated::FILL)
                    .then(|| {
                        crate::next::fill_of(
                            &new.switch.track(WidgetState::HOVER | WidgetState::CHECKED),
                        )
                    })
                    .flatten()
            } else {
                crate::next::state_fill_of(&hovered)
            };
            r.home_leaf(
                "SwitchLook hover_track",
                &state,
                old_switch.map(|look| look.hover_track),
                new_switch.map(|look| look.hover_track),
                track_hover.map(|hover| (!disabled).then_some(hover)),
            );
            r.leaf(
                if disabled {
                    "SwitchLook::label disabled"
                } else {
                    "SwitchLook::label"
                },
                &state,
                old_switch.map(|look| look.label),
                new_switch.map(|look| look.label),
            );
            // The outline, the hover outline and the hover fill apart, so a listed difference of
            // any leaves the rest of the look compared.
            // The label apart too, so a listed difference of the disabled label leaves the rest
            // of the look compared; disabled with it the mark, which the look reads at its own
            // disabled home, else draws as gpui-component does (`CheckboxLook::next_of`).
            let unframed = |look: CheckboxLook| CheckboxLook {
                border: gpui::transparent_black(),
                hover_border: None,
                hover_fill: None,
                label: gpui::transparent_black(),
                mark: if disabled {
                    gpui::transparent_black()
                } else {
                    look.mark
                },
                ..look
            };
            let text = |look: CheckboxLook| {
                (
                    if disabled {
                        look.mark
                    } else {
                        gpui::transparent_black()
                    },
                    look.label,
                )
            };
            let (old_box, new_box) = (
                CheckboxLook::of(&old, checked, disabled),
                CheckboxLook::next_of(&new, checked, disabled),
            );
            r.leaf(
                "CheckboxLook::of",
                &state,
                old_box.map(unframed),
                new_box.map(unframed),
            );
            r.home_leaf(
                if disabled {
                    "CheckboxLook::of label disabled"
                } else {
                    "CheckboxLook::of label"
                },
                &state,
                old_box.map(text),
                new_box.map(text),
                disabled_text_of(&new, false, disabled),
            );
            let outline = outline_of(&new, false, checked, disabled);
            r.home_leaf(
                "CheckboxLook::of border",
                &state,
                old_box.map(|look| look.border),
                new_box.map(|look| look.border),
                outline.map(|(border, _)| border),
            );
            r.home_leaf(
                "CheckboxLook::of hover_border",
                &state,
                old_box.map(|look| look.hover_border),
                new_box.map(|look| look.hover_border),
                outline.map(|(_, hover)| hover),
            );
            // `CheckboxLook::next_of`: the hovered box's stated fill, whole,
            // where it is enabled and unchecked.
            r.home_leaf(
                "CheckboxLook::of hover_fill",
                &state,
                old_box.map(|look| look.hover_fill),
                new_box.map(|look| look.hover_fill),
                crate::next::state_fill_of(&new.checkbox.indicator(WidgetState::HOVER))
                    .map(|hover| (!checked && !disabled).then_some(hover)),
            );
            let (old_radio, new_radio) = (
                CheckboxLook::radio(&old, checked, disabled),
                CheckboxLook::next_radio(&new, checked, disabled),
            );
            r.leaf(
                "CheckboxLook::radio",
                &state,
                old_radio.map(unframed),
                new_radio.map(unframed),
            );
            r.home_leaf(
                if disabled {
                    "CheckboxLook::radio label disabled"
                } else {
                    "CheckboxLook::radio label"
                },
                &state,
                old_radio.map(text),
                new_radio.map(text),
                disabled_text_of(&new, true, disabled),
            );
            let outline = outline_of(&new, true, checked, disabled);
            r.home_leaf(
                "CheckboxLook::radio border",
                &state,
                old_radio.map(|look| look.border),
                new_radio.map(|look| look.border),
                outline.map(|(border, _)| border),
            );
            r.home_leaf(
                "CheckboxLook::radio hover_border",
                &state,
                old_radio.map(|look| look.hover_border),
                new_radio.map(|look| look.hover_border),
                outline.map(|(_, hover)| hover),
            );
            r.leaf(
                "CheckboxLook::radio hover_fill",
                &state,
                old_radio.map(|look| look.hover_fill),
                new_radio.map(|look| look.hover_fill),
            );
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
    r.leaf(
        "ThemeSource::focus_ring",
        "",
        FocusRing::of(&old),
        given(FocusRing::next_of(&new)),
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
    r.leaf(
        "SwitchLabelFont",
        "",
        o.body_text()
            .map(|(size, line_height, weight)| SwitchLabelFont {
                family: None,
                size: Some(size),
                weight: Some(weight),
                style: None,
                line_height,
            }),
        SwitchLabelFont::next_of(&new),
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
    for d in DIFFERENCES.iter().chain(FROM_HOME) {
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
