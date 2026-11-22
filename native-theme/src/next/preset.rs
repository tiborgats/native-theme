//! The bundled presets and what a platform builder resolves from (spec §6).

use std::collections::BTreeMap;
use std::sync::Arc;

use super::color::Color;
use super::geometry::Stroke;
use super::icons::{IconSizes, IconThemeSpec};
use super::layout::LayoutMetrics;
use super::motion::{CaretSpec, MotionTokens, UiTimings};
use super::query::ThemeContext;
use super::reader::SystemOverrides;
use super::roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
use super::rules::{RuleBuilder, RuleSet};
use super::style::{FocusIndicator, FocusKind, IconStyle, PartStyle, TextStyle};
use super::typography::{TextRendering, Typography};
use super::widgets::{
    ButtonWidget, CheckboxWidget, ListWidget, RadioWidget, ResolvedTheme, ScrollbarWidget,
    TooltipWidget, WidgetOrigin,
};
use crate::IconSet;

/// A bundled platform preset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Preset {
    /// GNOME's Adwaita.
    Adwaita,
    /// KDE Plasma's Breeze.
    KdeBreeze,
    /// macOS Monterey.
    MacosMonterey,
    /// macOS Tahoe.
    MacosTahoe,
    /// Windows 11.
    Windows11,
    /// Material 3.
    Material3,
    /// iOS 13.
    Ios13,
}

impl Preset {
    /// Every preset, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Adwaita,
        Self::KdeBreeze,
        Self::MacosMonterey,
        Self::MacosTahoe,
        Self::Windows11,
        Self::Material3,
        Self::Ios13,
    ];

    /// The preset of the machine-readable name `name` (`"kde-breeze"`), if
    /// there is one.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|p| p.as_str() == name)
    }

    /// The machine-readable name, as 0.7.2's preset keys spell it
    /// (`src/presets.rs`, the bundled presets' table).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Adwaita => "adwaita",
            Self::KdeBreeze => "kde-breeze",
            Self::MacosMonterey => "macos-monterey",
            Self::MacosTahoe => "macos-tahoe",
            Self::Windows11 => "windows-11",
            Self::Material3 => "material-3",
            Self::Ios13 => "ios-13",
        }
    }

    /// The human-readable name, as 0.7.2's display names spell it
    /// (`src/presets.rs`, the bundled presets' table).
    #[must_use]
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Adwaita => "Adwaita",
            Self::KdeBreeze => "KDE Breeze",
            Self::MacosMonterey => "macOS Monterey",
            Self::MacosTahoe => "macOS Tahoe",
            Self::Windows11 => "Windows 11",
            Self::Material3 => "Material",
            Self::Ios13 => "iOS",
        }
    }
}

/// What a platform builder resolves from besides the context: the system's
/// settings and an explicit accent.
///
/// The spec's `palette: Option<&'a Palette>` field comes with the `Palette`
/// type (plan, M1).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by the kde-breeze builder, the next task of M0"
    )
)]
pub(crate) struct ResolveInputs<'a> {
    /// The system's settings, where read.
    pub(crate) system: Option<&'a SystemOverrides>,
    /// An explicit accent.
    pub(crate) accent: Option<Color>,
}

/// The theme of `preset` for `context` that states nothing: every `Option`
/// `None`, every `stated` bit clear, every rule set empty over a default
/// base, `name` the preset's display name. A field the spec declares without
/// `Option` holds its type's default; the three whose types have none hold
/// what their sources state: every widget's origin is
/// [`WidgetOrigin::Native`] (spec §5: `Synthesized` is only for a control
/// the platform does not have, and an unsourced property of a native control
/// is left unstated), a status colour's origin is the preset's provenance
/// (spec §4, `RoleOrigin`) and the icon set is the preset's (0.7.2's preset
/// files, `icon_set`).
///
/// Scaffolding until each preset's builder lands (plan, M1), not a theme to
/// draw with.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "called by `Theme::resolve`, the next commit")
)]
pub(crate) fn nothing_stated(preset: Preset, context: ThemeContext) -> ResolvedTheme {
    fn part<M>() -> RuleSet<PartStyle, super::rules::PartProp, M> {
        RuleBuilder::new(PartStyle::default()).build()
    }
    fn text<M>() -> RuleSet<TextStyle, super::rules::TextProp, M> {
        RuleBuilder::new(TextStyle::default()).build()
    }
    fn icon<M>() -> RuleSet<IconStyle, super::rules::IconProp, M> {
        RuleBuilder::new(IconStyle::default()).build()
    }
    let surface = SurfaceColors::default();
    let selection = SelectionColors::new(Color::default(), Color::default(), None, None);
    let status = |origin| StatusColors {
        fill: Color::default(),
        foreground: Color::default(),
        standalone: None,
        origin,
    };
    // Spec §4, the comment on `RoleOrigin`: `[error, warning, success, info]`.
    let [error, warning, success, info] = match preset {
        Preset::Adwaita | Preset::KdeBreeze | Preset::Windows11 => [RoleOrigin::Native; 4],
        Preset::MacosMonterey | Preset::MacosTahoe | Preset::Ios13 => [RoleOrigin::SystemColor; 4],
        Preset::Material3 => [
            RoleOrigin::Native,
            RoleOrigin::Harmonized,
            RoleOrigin::Harmonized,
            RoleOrigin::Harmonized,
        ],
    };
    // 0.7.2's preset files, `icon_set` in `src/presets/<preset>.toml`.
    let icon_set = match preset {
        Preset::Adwaita | Preset::KdeBreeze => IconSet::Freedesktop,
        Preset::MacosMonterey | Preset::MacosTahoe | Preset::Ios13 => IconSet::SfSymbols,
        Preset::Windows11 => IconSet::SegoeFluent,
        Preset::Material3 => IconSet::Material,
    };
    let native = WidgetOrigin::Native;
    ResolvedTheme {
        name: Arc::from(preset.display_name()),
        preset,
        context,
        colors: ColorRoles {
            window: surface,
            view: surface,
            header: surface,
            sidebar: surface,
            card: surface,
            dialog: surface,
            popover: surface,
            tooltip: surface,
            osd: surface,
            selection,
            text_selection: selection,
            find_highlight: None,
            scrim: None,
            accent: AccentColors {
                fill: Color::default(),
                foreground: Color::default(),
                standalone: None,
            },
            focus_ring: Color::default(),
            caret: Color::default(),
            link: Color::default(),
            link_visited: None,
            disabled_text: Color::default(),
            shadow: Color::default(),
            disabled_opacity: f32::default(),
            error: status(error),
            warning: status(warning),
            success: status(success),
            info: status(info),
        },
        typography: Typography {
            display: None,
            title1: None,
            title2: None,
            title3: None,
            title4: None,
            headline: None,
            body: TextStyle::default(),
            callout: None,
            subheadline: None,
            caption: None,
            monospace: TextStyle::default(),
            line_height_multiplier: f32::default(),
            fallbacks: Arc::from([]),
            emoji_family: None,
            rendering: TextRendering::default(),
            font_dpi: f32::default(),
            font_whole_pixels: None,
        },
        icons: IconThemeSpec {
            set: icon_set,
            theme: None,
            theme_explicit: bool::default(),
            sizes: IconSizes::new(None, None, None, None, None),
        },
        layout: LayoutMetrics {
            widget_gap: None,
            container_margin: None,
            window_margin: None,
            section_gap: None,
            corner_radius: f32::default(),
            corner_radius_lg: f32::default(),
            border_width: f32::default(),
            border_opacity: f32::default(),
            shadow_enabled: bool::default(),
            cursor_size_px: None,
            cursor_theme: None,
        },
        timings: UiTimings {
            tooltip_show_ms: None,
            tooltip_hide_ms: None,
            tooltip_reshow_ms: None,
            double_click_ms: None,
            drag_threshold_px: None,
            scrollbar_auto_hide_ms: None,
            menu_hover_open_ms: None,
            key_repeat_delay_ms: None,
            key_repeat_interval_ms: None,
        },
        motion_tokens: MotionTokens {
            fast: None,
            standard: None,
            emphasized: None,
            spring_spatial: None,
            spring_effects: None,
            theme_change: None,
        },
        focus_ring: FocusIndicator::new(
            FocusKind::default(),
            Stroke::default(),
            Stroke::default(),
            f32::default(),
            None,
            None,
        ),
        caret: CaretSpec::default(),
        elevations: Arc::from([]),
        button: ButtonWidget::new(native, part(), text(), icon(), f32::default(), None),
        checkbox: CheckboxWidget::new(native, part(), icon(), text(), f32::default(), None),
        radio: RadioWidget::new(native, part(), part(), text(), f32::default(), None),
        scrollbar: ScrollbarWidget::new(native, part(), part(), f32::default(), bool::default()),
        list: ListWidget::new(
            native,
            part(),
            part(),
            text(),
            part(),
            text(),
            part(),
            Color::default(),
        ),
        tooltip: TooltipWidget::new(native, PartStyle::default(), TextStyle::default()),
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip_and_are_distinct() {
        for preset in Preset::ALL {
            assert_eq!(Preset::from_name(preset.as_str()), Some(*preset));
            let same_name = Preset::ALL
                .iter()
                .filter(|p| {
                    p.as_str() == preset.as_str() || p.display_name() == preset.display_name()
                })
                .count();
            assert_eq!(same_name, 1, "{preset:?}");
        }
        assert_eq!(Preset::from_name("kde-breeze"), Some(Preset::KdeBreeze));
        assert_eq!(Preset::KdeBreeze.display_name(), "KDE Breeze");
        assert_eq!(Preset::from_name("KDE Breeze"), None);
        assert_eq!(Preset::from_name("catppuccin-mocha"), None);
    }

    #[test]
    fn resolve_inputs_hold_what_they_are_given() {
        let system = SystemOverrides::default();
        let inputs = ResolveInputs {
            system: Some(&system),
            accent: Some(Color::WHITE),
        };
        assert_eq!(inputs.system, Some(&system));
        assert_eq!(inputs.accent, Some(Color::WHITE));
    }
}
