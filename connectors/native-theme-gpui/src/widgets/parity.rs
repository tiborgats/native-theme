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
use native_theme::next::{Preset, ThemeContext, WidgetState};

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
        reason: "the 0.8.0 preset states no disabled colour of switch.label, so a disabled \
                 switch's label is gpui-component's own, muted_foreground, \
                 colors.window.foreground_muted (spec §10, rule 0), where the 0.7.2 look took \
                 defaults.disabled_text_color",
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
        leaf: "CheckboxLook::of hover_border",
        presets: &["adwaita", "windows-11"],
        reason: "0.7.2 states no checkbox.hover_border_color, so the look has no hover \
                 outline; 0.8.0 states the unchecked box's outline for HOVER \
                 (native-theme/tests/next_homes.toml, row checkbox.hover_border_color, a \
                 listed difference): adwaita's $trough_hover_color (_checks.scss:20-35 at \
                 libadwaita 1.10.0), windows-11's rest ControlStrongStrokeColorDefault, which \
                 WinUI keeps (CheckBox_themeresources.xaml:218, :42 at microsoft-ui-xaml \
                 8463f45); so an enabled box hovers in the outline stated there, a checked one \
                 in its checked outline",
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
                 checkbox.border.line_width, listed differences)",
    },
    Difference {
        leaf: "CheckboxLook::radio",
        presets: &["ios-13"],
        reason: "as the check box's (CheckboxLook::of): the radio button's indicator is the \
                 same part on ios-13",
    },
    Difference {
        leaf: "SliderLook",
        presets: &["adwaita"],
        reason: "0.7.2's adwaita states an uncited slider.thumb_hover_color (#ebebeb / \
                 #3a3a3e); 0.8.0's adwaita states libadwaita's $slider_hover_color, white in \
                 both schemes (_scale.scss:62-76, _colors.scss:289 at libadwaita 1.10.0; \
                 native-theme/tests/next_homes.toml, row slider.thumb_hover_color, a listed \
                 difference), so the look's hovered thumb is white",
    },
    Difference {
        leaf: "SwitchLabelFont",
        presets: &["ios-13"],
        reason: "the 0.8.0 ios-13 states no font of switch.label (no source: iOS's switch \
                 draws no label), so a switch's label is lettered in gpui-component's own font \
                 (spec §10, rule 0): text_sm (input_text_size(Size::Medium)) in the window's \
                 family and weight (gpui-component 0.10.0 switch.rs, Switch::render), where \
                 the 0.7.2 switch took the body text's size and weight (SwitchTheme states no \
                 label font)",
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
        ],
        reason: "the 0.8.0 presets state switch.label as the platform's switch label, the \
                 body font (kde-breeze qqc2-desktop-style's control.font, adwaita \
                 AdwSwitchRow's title, windows-11 ToggleSwitch's content, macOS the body \
                 font, material-3 Android's MaterialSwitch text), its family and slant \
                 included, where the 0.7.2 switch took the body text's size and weight alone \
                 and left the family and slant to the window's (SwitchTheme states no label \
                 font); the size and weight are the same",
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
            self.listed.push(leaf);
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
    r.leaf(
        "TabLook::hover",
        "",
        old_tab.map(|look| look.hover),
        new_tab.map(|look| look.hover),
    );
    for disabled in [false, true] {
        let state = format!(" disabled={disabled}");
        r.leaf(
            "SliderLook",
            &state,
            SliderLook::of(&old, disabled),
            SliderLook::next_of(&new, disabled),
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
            r.leaf(
                "SwitchLook::label_gap",
                &state,
                old_switch.map(|look| look.label_gap),
                new_switch.map(|look| look.label_gap),
            );
            r.leaf(
                "SwitchLook",
                &state,
                old_switch.map(unlabelled),
                new_switch.map(unlabelled),
            );
            r.leaf(
                "SwitchLook hover_track",
                &state,
                old_switch.map(|look| look.hover_track),
                new_switch.map(|look| look.hover_track),
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
            // The hover outline and the hover fill apart, so a listed difference of either leaves
            // the rest of the look compared.
            // The label apart too, so a listed difference of the disabled label leaves the rest
            // of the look compared; disabled with it the mark, which the look draws in the
            // label's disabled colour where a disabled fill is stated (`CheckboxLook::next_of`).
            let unframed = |look: CheckboxLook| CheckboxLook {
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
            r.leaf(
                if disabled {
                    "CheckboxLook::of label disabled"
                } else {
                    "CheckboxLook::of label"
                },
                &state,
                old_box.map(text),
                new_box.map(text),
            );
            r.leaf(
                "CheckboxLook::of hover_border",
                &state,
                old_box.map(|look| look.hover_border),
                new_box.map(|look| look.hover_border),
            );
            r.leaf(
                "CheckboxLook::of hover_fill",
                &state,
                old_box.map(|look| look.hover_fill),
                new_box.map(|look| look.hover_fill),
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
            r.leaf(
                if disabled {
                    "CheckboxLook::radio label disabled"
                } else {
                    "CheckboxLook::radio label"
                },
                &state,
                old_radio.map(text),
                new_radio.map(text),
            );
            r.leaf(
                "CheckboxLook::radio hover_border",
                &state,
                old_radio.map(|look| look.hover_border),
                new_radio.map(|look| look.hover_border),
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
