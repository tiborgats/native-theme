//! The bundled presets and what a platform builder resolves from (spec §6).

use std::collections::BTreeMap;
use std::sync::Arc;

use super::color::Color;
use super::geometry::Stroke;
use super::icons::{IconSizes, IconThemeSpec};
use super::layout::LayoutMetrics;
use super::motion::{CaretSpec, MotionTokens, UiTimings};
use super::query::{ThemeContext, sanitized_text_scale};
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

/// A theme to resolve: a preset, the context it resolves for by default and
/// the system's settings, where given (spec §6).
///
/// [`resolve`](Self::resolve) runs the preset's builder. No preset's builder
/// is written yet, so every preset resolves to a theme that states nothing
/// (plan, M1): scaffolding until each preset's builder lands, not a theme to
/// draw with.
//
// Not built yet, each with its milestone (plan, M1): `with_name`; the
// palettes (`with_palette`, `with_palettes`, `with_palette_for`,
// `preserve_palette_in_high_contrast`); `with_accent`; `with_patch`;
// `with_transform`; `from_system`, `from_system_with_preset`,
// `from_system_or_default`; `for_current_platform`; `from_toml`;
// `diagnostics`.
#[derive(Clone, Debug)]
pub struct Theme {
    preset: Preset,
    context: ThemeContext,
    system: Option<Arc<SystemOverrides>>,
}

impl Theme {
    /// The theme of `preset`, for the default context, without system
    /// settings.
    #[must_use]
    pub fn preset(preset: Preset) -> Self {
        Self {
            preset,
            context: ThemeContext::default(),
            system: None,
        }
    }

    /// The context [`resolve_current`](Self::resolve_current) resolves for.
    #[must_use]
    pub fn context(&self) -> &ThemeContext {
        &self.context
    }

    /// This theme with `ctx` as its context.
    #[must_use]
    pub fn with_context(mut self, ctx: ThemeContext) -> Self {
        self.context = ctx;
        self
    }

    /// The system's settings, where given.
    #[must_use]
    pub fn system_overrides(&self) -> Option<&SystemOverrides> {
        self.system.as_deref()
    }

    /// This theme with the system's settings `overrides`.
    #[must_use]
    pub fn with_system_overrides(mut self, overrides: SystemOverrides) -> Self {
        self.system = Some(Arc::new(overrides));
        self
    }

    /// The preset.
    #[must_use]
    pub fn preset_id(&self) -> Preset {
        self.preset
    }

    /// The theme resolved for `ctx`, its text scale sanitized (clamped to
    /// `0.5..=4.0`, 1.0 where it is not finite); its name is the system's theme name where the
    /// system settings state one, else the preset's display name.
    #[must_use]
    pub fn resolve(&self, ctx: &ThemeContext) -> ResolvedTheme {
        let mut effective = ctx.clone();
        effective.text_scale = sanitized_text_scale(ctx.text_scale);
        let mut resolved = build(self.preset, effective);
        resolved.name = self
            .system
            .as_ref()
            .and_then(|system| system.theme_name.clone())
            .unwrap_or_else(|| Arc::from(self.preset.display_name()));
        resolved
    }

    /// The theme resolved for its own [`context`](Self::context).
    #[must_use]
    pub fn resolve_current(&self) -> ResolvedTheme {
        self.resolve(self.context())
    }
}

/// The preset's builder run for `context`: until each preset's builder is
/// written (plan, M1), the theme with nothing stated.
fn build(preset: Preset, context: ThemeContext) -> ResolvedTheme {
    match preset {
        Preset::Adwaita => nothing_stated(preset, context),
        Preset::KdeBreeze => nothing_stated(preset, context),
        Preset::MacosMonterey => nothing_stated(preset, context),
        Preset::MacosTahoe => nothing_stated(preset, context),
        Preset::Windows11 => nothing_stated(preset, context),
        Preset::Material3 => nothing_stated(preset, context),
        Preset::Ios13 => nothing_stated(preset, context),
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

    fn assert_clone_debug_send_sync<T: Clone + std::fmt::Debug + Send + Sync>() {}

    #[test]
    fn theme_is_clone_debug_send_sync() {
        assert_clone_debug_send_sync::<Theme>();
    }

    #[test]
    fn resolve_sanitizes_the_text_scale() {
        let theme = Theme::preset(Preset::KdeBreeze);
        for (given, used) in [
            (f32::NAN, 1.0),
            (f32::INFINITY, 1.0),
            (f32::NEG_INFINITY, 1.0),
            (0.1, 0.5),
            (9.0, 4.0),
            (0.5, 0.5),
            (4.0, 4.0),
            (1.25, 1.25),
        ] {
            let ctx = ThemeContext {
                text_scale: given,
                ..ThemeContext::default()
            };
            let resolved = theme.resolve(&ctx);
            assert_eq!(resolved.context.text_scale, used, "{given}");
            assert_eq!(
                ThemeContext::default().with_text_scale(given).text_scale,
                used,
                "{given}"
            );
            assert_eq!(
                resolved.context,
                ThemeContext {
                    text_scale: used,
                    ..ctx
                }
            );
        }
    }

    #[test]
    fn the_preset_and_the_context_read_back() {
        let ctx = ThemeContext::dark().with_text_scale(1.5);
        let theme = Theme::preset(Preset::Windows11).with_context(ctx.clone());
        assert_eq!(theme.preset_id(), Preset::Windows11);
        assert_eq!(theme.context(), &ctx);
        assert_eq!(theme.system_overrides(), None);
        let resolved = theme.resolve_current();
        assert_eq!(resolved.preset, Preset::Windows11);
        assert_eq!(resolved.context, ctx);
        assert!(resolved.is_dark());
        // `resolve` uses the context it is given, not the theme's.
        assert_eq!(
            theme.resolve(&ThemeContext::light()).context,
            ThemeContext::light()
        );
    }

    #[test]
    fn the_name_is_the_system_theme_name_else_the_display_name() {
        let theme = Theme::preset(Preset::KdeBreeze);
        assert_eq!(&*theme.resolve_current().name, "KDE Breeze");
        let unnamed = theme
            .clone()
            .with_system_overrides(SystemOverrides::default());
        assert_eq!(&*unnamed.resolve_current().name, "KDE Breeze");
        let system = SystemOverrides {
            theme_name: Some(Arc::from("Breeze Dark")),
            ..SystemOverrides::default()
        };
        let named = theme.with_system_overrides(system.clone());
        assert_eq!(named.system_overrides(), Some(&system));
        assert_eq!(&*named.resolve_current().name, "Breeze Dark");
    }

    /// The position of `preset` in declaration order, by a `match` without a
    /// wildcard: a new variant fails to compile here until it is placed.
    fn position(preset: Preset) -> usize {
        match preset {
            Preset::Adwaita => 0,
            Preset::KdeBreeze => 1,
            Preset::MacosMonterey => 2,
            Preset::MacosTahoe => 3,
            Preset::Windows11 => 4,
            Preset::Material3 => 5,
            Preset::Ios13 => 6,
        }
    }

    #[test]
    fn every_preset_resolves_through_the_builder_match_to_nothing_stated() {
        let positions: Vec<usize> = Preset::ALL.iter().map(|p| position(*p)).collect();
        assert_eq!(positions, (0..7).collect::<Vec<_>>());
        let ctx = ThemeContext::dark();
        for preset in Preset::ALL {
            let resolved = Theme::preset(*preset).resolve(&ctx);
            assert_eq!(resolved, nothing_stated(*preset, ctx.clone()), "{preset:?}");
            assert_eq!(resolved.preset, *preset);
            assert_eq!(&*resolved.name, preset.display_name());
        }
    }

    #[test]
    fn the_theme_with_nothing_stated_states_nothing() {
        use crate::next::query::ButtonQuery;
        use crate::next::query::WidgetState;
        let t = nothing_stated(Preset::Material3, ThemeContext::light());
        let q = ButtonQuery::new(WidgetState::HOVER);
        assert!(t.button.body(q).stated.is_empty());
        assert!(t.button.label(q).stated.is_empty());
        assert!(t.button.icon(q).stated.is_empty());
        assert!(t.tooltip.panel.stated.is_empty());
        assert!(t.tooltip.text.stated.is_empty());
        assert!(t.typography.body.stated.is_empty());
        assert_eq!(t.typography.caption, None);
        assert_eq!(t.button.disabled_opacity, None);
        assert_eq!(t.colors.window, SurfaceColors::default());
        assert_eq!(t.icons.theme, None);
        assert!(t.elevations.is_empty());
        assert!(t.custom_colors.is_empty());
        // The fields without `Option` and without a type default: from their sources.
        assert!(t.button.is_native() && t.list.is_native() && t.tooltip.is_native());
        assert_eq!(t.colors.error.origin, RoleOrigin::Native);
        assert_eq!(t.colors.warning.origin, RoleOrigin::Harmonized);
        assert_eq!(t.icons.set, IconSet::Material);
        let apple = nothing_stated(Preset::MacosTahoe, ThemeContext::light());
        assert_eq!(apple.colors.info.origin, RoleOrigin::SystemColor);
        assert_eq!(apple.icons.set, IconSet::SfSymbols);
    }
}
