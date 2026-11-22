//! The bundled presets and what a platform builder resolves from (spec §6).

use std::collections::BTreeMap;
use std::sync::Arc;

use super::color::Color;
use super::geometry::Stroke;
use super::icons::{IconSizes, IconThemeSpec};
use super::layout::LayoutMetrics;
use super::motion::{CaretSpec, MotionTokens, UiTimings};
use super::palette::Palette;
use super::presets::kde;
use super::query::{Contrast, Scheme, ThemeContext, sanitized_text_scale};
use super::reader::{OsColors, SystemOverrides};
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
use super::widgets::{
    InputWidget, LinkWidget, ProgressBarWidget, SeparatorWidget, SliderWidget, SpinnerWidget,
    SplitterWidget,
};
use super::widgets::{
    MenuBarWidget, MenuWidget, SidebarWidget, StatusBarWidget, TabWidget, ToolbarWidget,
};
use super::window::{CaptionLayout, WindowChrome};
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

    /// The preset of the platform this runs on, without a reader feature
    /// (spec §6): Linux by the desktop (KDE → `KdeBreeze`, else `Adwaita`),
    /// macOS by its generation (≥ 26 → `MacosTahoe`, else `MacosMonterey`),
    /// Windows → `Windows11`, iOS → `Ios13`, Android → `Material3`, any other
    /// target → `Adwaita`.
    ///
    /// The Linux, macOS and Windows answers are 0.7.2's
    /// [`platform_preset_name`](crate::pipeline::platform_preset_name)'s: the
    /// desktop the last system read refined (before one, `XDG_CURRENT_DESKTOP`
    /// and, with the `kde` feature, a `kdeglobals` file), and the macOS
    /// version read through the `macos` feature, `MacosMonterey` without it.
    #[must_use]
    pub fn detect() -> Self {
        #[cfg(target_os = "linux")]
        {
            match crate::detect::system().refined_linux_desktop_sync() {
                crate::detect::LinuxDesktop::Kde => Self::KdeBreeze,
                _ => Self::Adwaita,
            }
        }
        #[cfg(all(target_os = "macos", feature = "macos"))]
        {
            let generation = crate::pipeline::macos_generation(crate::macos::os_version()).0;
            if generation.name == Self::MacosTahoe.as_str() {
                Self::MacosTahoe
            } else {
                Self::MacosMonterey
            }
        }
        // Without the reader feature the version is not read, as in 0.7.2;
        // plan M3 item 2 proves a `SystemVersion.plist` read on a macOS 26
        // runner before it replaces this.
        #[cfg(all(target_os = "macos", not(feature = "macos")))]
        {
            Self::MacosMonterey
        }
        #[cfg(target_os = "windows")]
        {
            Self::Windows11
        }
        #[cfg(target_os = "ios")]
        {
            Self::Ios13
        }
        #[cfg(target_os = "android")]
        {
            Self::Material3
        }
        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
            target_os = "ios",
            target_os = "android"
        )))]
        {
            Self::Adwaita
        }
    }

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

/// A function run on the resolved theme (spec §6, resolution step 4).
type Transform = Arc<dyn Fn(&mut ResolvedTheme) + Send + Sync>;

/// A theme's palettes (spec §6, the three palette methods): one that locks
/// the effective scheme to its own, or one slot per scheme.
#[derive(Clone, Debug, PartialEq)]
enum Palettes {
    /// [`Theme::with_palette`]'s palette.
    Lock(Arc<Palette>),
    /// [`Theme::with_palettes`]' and [`Theme::with_palette_for`]'s palettes.
    Slots {
        light: Option<Arc<Palette>>,
        dark: Option<Arc<Palette>>,
    },
}

impl Palettes {
    /// No palette.
    const NONE: Self = Self::Slots {
        light: None,
        dark: None,
    };

    /// The palette for `scheme`: the locking palette, else the slot of
    /// `scheme`.
    fn active(&self, scheme: Scheme) -> Option<&Palette> {
        match self {
            Self::Lock(palette) => Some(palette),
            Self::Slots { light, dark } => match scheme {
                Scheme::Light => light.as_deref(),
                Scheme::Dark => dark.as_deref(),
            },
        }
    }
}

/// A theme to resolve: a preset, the context it resolves for by default, the
/// system's settings, a name, palettes, an accent and transforms, each where
/// given (spec §6).
///
/// [`resolve`](Self::resolve) runs the preset's builder. Only `kde-breeze`'s
/// is written, and it does not apply the palette or the accent yet; every
/// other preset resolves to a theme that states nothing (plan, M1, M2):
/// scaffolding until each preset's builder lands, not a theme to draw with.
//
// Not built yet, each with its milestone (plan, M1): `with_patch`;
// `from_system`, `from_system_with_preset`, `from_system_or_default`;
// `from_toml`; `diagnostics`.
#[derive(Clone)]
pub struct Theme {
    preset: Preset,
    name: Option<Arc<str>>,
    context: ThemeContext,
    system: Option<Arc<SystemOverrides>>,
    palettes: Palettes,
    accent: Option<Color>,
    preserve_palette_in_high_contrast: bool,
    // `patches: Vec<ThemePatch>` comes with the `ThemePatch` type (plan, M1).
    transforms: Vec<Transform>,
}

impl std::fmt::Debug for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Theme")
            .field("preset", &self.preset)
            .field("name", &self.name)
            .field("context", &self.context)
            .field("system", &self.system)
            .field("palettes", &self.palettes)
            .field("accent", &self.accent)
            .field(
                "preserve_palette_in_high_contrast",
                &self.preserve_palette_in_high_contrast,
            )
            .field("transforms", &self.transforms.len())
            .finish()
    }
}

impl Theme {
    /// The theme of `preset`, for the default context, without system
    /// settings, name, palette, accent or transform.
    #[must_use]
    pub fn preset(preset: Preset) -> Self {
        Self {
            preset,
            name: None,
            context: ThemeContext::default(),
            system: None,
            palettes: Palettes::NONE,
            accent: None,
            preserve_palette_in_high_contrast: false,
            transforms: Vec::new(),
        }
    }

    /// The theme of the platform's preset ([`Preset::detect`]), as
    /// [`preset`](Self::preset) makes it.
    #[must_use]
    pub fn for_current_platform() -> Self {
        Self::preset(Preset::detect())
    }

    /// This theme with the explicit name `name`, which names every resolved
    /// theme (spec §6, resolution step 2).
    #[must_use]
    pub fn with_name(mut self, name: impl Into<Arc<str>>) -> Self {
        self.name = Some(name.into());
        self
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

    /// This theme with the palette `palette`, which locks the effective
    /// scheme to the palette's own. It replaces the palettes of
    /// [`with_palettes`](Self::with_palettes) and
    /// [`with_palette_for`](Self::with_palette_for).
    #[must_use]
    pub fn with_palette(mut self, palette: Palette) -> Self {
        self.palettes = Palettes::Lock(Arc::new(palette));
        self
    }

    /// This theme with the palette `light` for the light scheme and `dark`
    /// for the dark one, which follow the context's scheme. It replaces the
    /// palette of [`with_palette`](Self::with_palette).
    #[must_use]
    pub fn with_palettes(mut self, light: Palette, dark: Palette) -> Self {
        self.palettes = Palettes::Slots {
            light: Some(Arc::new(light)),
            dark: Some(Arc::new(dark)),
        };
        self
    }

    /// This theme with the palette `palette` for `scheme`, the other
    /// scheme's palette kept where [`with_palettes`](Self::with_palettes) or
    /// this method gave one. It replaces the palette of
    /// [`with_palette`](Self::with_palette).
    #[must_use]
    pub fn with_palette_for(mut self, scheme: Scheme, palette: Palette) -> Self {
        let (mut light, mut dark) = match self.palettes {
            Palettes::Lock(_) => (None, None),
            Palettes::Slots { light, dark } => (light, dark),
        };
        let palette = Some(Arc::new(palette));
        match scheme {
            Scheme::Light => light = palette,
            Scheme::Dark => dark = palette,
        }
        self.palettes = Palettes::Slots { light, dark };
        self
    }

    /// This theme keeping its palette and its explicit accent in a resolve
    /// for `Contrast::High` where `preserve` is true. Where it is false (the
    /// default), a resolve for `High` passes neither on where `High` is
    /// available for the context's scheme, and does not take `High` while
    /// either is active: the nearest available level below it is resolved
    /// with them (spec §6, resolution step 1).
    #[must_use]
    pub fn preserve_palette_in_high_contrast(mut self, preserve: bool) -> Self {
        self.preserve_palette_in_high_contrast = preserve;
        self
    }

    /// This theme with the explicit accent `accent`, laid over the
    /// palette's (spec §6, resolution step 2).
    #[must_use]
    pub fn with_accent(mut self, accent: Color) -> Self {
        self.accent = Some(accent);
        self
    }

    /// This theme with `f` run on every resolved theme, after the transforms
    /// given before it (spec §6, resolution step 4).
    #[must_use]
    pub fn with_transform(
        mut self,
        f: impl Fn(&mut ResolvedTheme) + Send + Sync + 'static,
    ) -> Self {
        self.transforms.push(Arc::new(f));
        self
    }

    /// The preset.
    #[must_use]
    pub fn preset_id(&self) -> Preset {
        self.preset
    }

    /// The theme resolved for `ctx` (spec §6, "Resolution Algorithm"): the
    /// effective context and the active palette (step 1), the preset's
    /// builder run with them (step 2), then the transforms in the order
    /// given (step 4).
    ///
    /// The name is the explicit name, else the active palette's, else the
    /// system's theme name unless the system was read in the other scheme
    /// than the effective one, else the preset's display name.
    #[must_use]
    pub fn resolve(&self, ctx: &ThemeContext) -> ResolvedTheme {
        let (effective, inputs) = self.effective_inputs(ctx);
        let scheme = effective.scheme;
        let mut resolved = build(self.preset, effective.clone(), &inputs);
        resolved.context = effective;
        resolved.name = self
            .name
            .clone()
            .or_else(|| inputs.palette.map(|palette| palette.name.clone()))
            .or_else(|| {
                inputs
                    .system
                    .filter(|system| system.scheme.is_none_or(|s| s == scheme))
                    .and_then(|system| system.theme_name.clone())
            })
            .unwrap_or_else(|| Arc::from(self.preset.display_name()));
        for transform in &self.transforms {
            transform(&mut resolved);
        }
        resolved
    }

    /// Resolution step 1 (spec §6): the effective context for `ctx` and the
    /// inputs the preset's builder gets.
    ///
    /// The text scale is sanitized (clamped to `0.5..=4.0`, 1.0 where it is
    /// not finite). A contrast level is available for a scheme where the
    /// preset defines it ([`contrast_levels`]) or the system's settings carry
    /// colours for that scheme and level (an entry of `color_schemes` that
    /// states at least one colour, [`OsColors::states_a_colour`]). Where
    /// `High` is requested and available for `ctx.scheme`, and the theme does
    /// not [preserve](Self::preserve_palette_in_high_contrast) its palette, no
    /// palette and no explicit accent are passed on. Otherwise the active
    /// palette (the locking one, else the slot of the context's scheme) sets
    /// the effective scheme to its own. The effective contrast is the
    /// requested level where it is available for the effective scheme, else
    /// the nearest level below it that is available for the effective
    /// scheme; `High` meets an active palette or a passed accent only where
    /// the theme preserves them, and otherwise is not taken either.
    fn effective_inputs(&self, ctx: &ThemeContext) -> (ThemeContext, ResolveInputs<'_>) {
        let system = self.system.as_deref();
        let levels = contrast_levels(self.preset);
        let available = |scheme: Scheme, level: Contrast| {
            levels.contains(&level)
                || system
                    .and_then(|s| s.colors_for(scheme, level))
                    .is_some_and(OsColors::states_a_colour)
        };
        let mut effective = ctx.clone();
        effective.text_scale = sanitized_text_scale(ctx.text_scale);
        let requested = ctx.contrast;
        let suspended = requested == Contrast::High
            && available(ctx.scheme, Contrast::High)
            && !self.preserve_palette_in_high_contrast;
        let palette = if suspended {
            None
        } else {
            self.palettes.active(ctx.scheme)
        };
        if let Some(palette) = palette {
            effective.scheme = palette.scheme;
        }
        let accent = if suspended { None } else { self.accent };
        let high_withheld = requested == Contrast::High
            && !self.preserve_palette_in_high_contrast
            && (palette.is_some() || accent.is_some());
        effective.contrast = if !high_withheld && available(effective.scheme, requested) {
            requested
        } else {
            nearest_below(requested, |level| available(effective.scheme, level))
        };
        let inputs = ResolveInputs {
            system,
            palette,
            accent,
        };
        (effective, inputs)
    }

    /// The theme resolved for its own [`context`](Self::context).
    #[must_use]
    pub fn resolve_current(&self) -> ResolvedTheme {
        self.resolve(self.context())
    }
}

/// The preset's builder run for `context` and `inputs`: for a preset whose
/// builder is not written yet (plan, M1, M2), the theme with nothing stated.
fn build(preset: Preset, context: ThemeContext, inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    match preset {
        Preset::Adwaita => nothing_stated(preset, context),
        Preset::KdeBreeze => kde::resolve(&context, inputs),
        Preset::MacosMonterey => nothing_stated(preset, context),
        Preset::MacosTahoe => nothing_stated(preset, context),
        Preset::Windows11 => nothing_stated(preset, context),
        Preset::Material3 => nothing_stated(preset, context),
        Preset::Ios13 => nothing_stated(preset, context),
    }
}

/// The contrast levels `preset`'s builder defines (spec §1, "Axis values
/// without data"), each list holding `Normal`.
fn contrast_levels(preset: Preset) -> &'static [Contrast] {
    match preset {
        Preset::KdeBreeze => kde::CONTRAST_LEVELS,
        // The theme with nothing stated (`nothing_stated`) defines `Normal`
        // only.
        Preset::Adwaita
        | Preset::MacosMonterey
        | Preset::MacosTahoe
        | Preset::Windows11
        | Preset::Material3
        | Preset::Ios13 => &[Contrast::Normal],
    }
}

/// The nearest level below `requested` for which `has` holds, `Normal` where
/// it holds for none above it: every preset defines `Normal`
/// ([`contrast_levels`]).
fn nearest_below(requested: Contrast, has: impl Fn(Contrast) -> bool) -> Contrast {
    match requested {
        Contrast::High if has(Contrast::Increased) => Contrast::Increased,
        Contrast::Normal | Contrast::Increased | Contrast::High => Contrast::Normal,
    }
}

/// What a platform builder resolves from besides the context: the system's
/// settings, the active palette and an explicit accent (spec §6, resolution
/// step 2).
#[derive(Clone, Copy, Debug)]
pub(crate) struct ResolveInputs<'a> {
    /// The system's settings, where read.
    pub(crate) system: Option<&'a SystemOverrides>,
    /// The active palette, where there is one.
    pub(crate) palette: Option<&'a Palette>,
    /// An explicit accent.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "read by the builders' accent derivations (plan, M2)"
        )
    )]
    pub(crate) accent: Option<Color>,
}

/// The theme of `preset` for `context` that states nothing: every `Option`
/// `None`, every `stated` bit clear, every rule set empty over a default
/// base, `name` the preset's display name. A field the spec declares without
/// `Option` holds its type's default, and the window's caption layout names
/// no button; the three whose types have none hold what their sources state:
/// every widget's origin is
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
            part(),
            text(),
            part(),
            text(),
            part(),
            Color::default(),
        ),
        tab: TabWidget::unstated(native),
        menu: MenuWidget::unstated(native),
        menu_bar: MenuBarWidget::unstated(native),
        sidebar: SidebarWidget::unstated(native),
        toolbar: ToolbarWidget::unstated(native),
        status_bar: StatusBarWidget::unstated(native),
        tooltip: TooltipWidget::new(native, PartStyle::default(), TextStyle::default()),
        window: WindowChrome::new(
            native,
            CaptionLayout::new(Arc::from([]), Arc::from([])),
            PartStyle::default(),
            PartStyle::default(),
            PartStyle::default(),
            PartStyle::default(),
            TextStyle::default(),
            part(),
            icon(),
        ),
        input: InputWidget::new(native, part(), text(), text(), text(), None),
        text_area: InputWidget::new(native, part(), text(), text(), text(), None),
        link: LinkWidget::new(native, part(), text()),
        slider: SliderWidget::new(native, part(), part(), part(), part(), f32::default(), None),
        progress_bar: ProgressBarWidget::new(native, part(), part(), u32::default()),
        splitter: SplitterWidget::new(native, part(), f32::default()),
        separator: SeparatorWidget::new(
            native,
            Color::default(),
            f32::default(),
            f32::default(),
            f32::default(),
        ),
        spinner: SpinnerWidget::new(
            native,
            Color::default(),
            f32::default(),
            f32::default(),
            f32::default(),
            u32::default(),
        ),
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::reader::{KdeColorEffects, OsSelectionColors, OsSurfaceOverrides};
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

    /// On the platforms 0.7.2 answers for, `detect` gives its preset.
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    #[test]
    fn detect_agrees_with_0_7_2_on_this_host() {
        assert_eq!(
            Preset::detect().as_str(),
            crate::pipeline::platform_preset_name().name
        );
    }

    #[test]
    fn for_current_platform_is_the_detected_preset() {
        let theme = Theme::for_current_platform();
        assert_eq!(theme.preset_id(), Preset::detect());
    }

    /// A palette named `name` for `scheme`, its colours the defaults.
    fn palette(name: &str, scheme: Scheme) -> Palette {
        let c = Color::default();
        Palette::new(
            name, scheme, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c,
        )
    }

    /// The name of the palette `theme` passes to its builder for `ctx`, and
    /// the effective scheme.
    fn active(theme: &Theme, ctx: &ThemeContext) -> (Option<String>, Scheme) {
        let (effective, inputs) = theme.effective_inputs(ctx);
        (inputs.palette.map(|p| p.name.to_string()), effective.scheme)
    }

    fn named(name: &str) -> Option<String> {
        Some(name.to_owned())
    }

    #[test]
    fn a_locking_palette_sets_the_scheme_a_pair_follows_the_context() {
        let light = ThemeContext::light();
        let dark = ThemeContext::dark();
        let locked = Theme::preset(Preset::KdeBreeze).with_palette(palette("D", Scheme::Dark));
        assert_eq!(active(&locked, &light), (named("D"), Scheme::Dark));
        assert_eq!(active(&locked, &dark), (named("D"), Scheme::Dark));
        assert_eq!(locked.resolve(&light).context.scheme, Scheme::Dark);
        assert!(locked.resolve(&light).is_dark());

        let pair = Theme::preset(Preset::KdeBreeze)
            .with_palettes(palette("L", Scheme::Light), palette("D", Scheme::Dark));
        assert_eq!(active(&pair, &light), (named("L"), Scheme::Light));
        assert_eq!(active(&pair, &dark), (named("D"), Scheme::Dark));
        assert_eq!(pair.resolve(&light).context.scheme, Scheme::Light);
        assert_eq!(pair.resolve(&dark).context.scheme, Scheme::Dark);

        let one = Theme::preset(Preset::KdeBreeze)
            .with_palette_for(Scheme::Dark, palette("D", Scheme::Dark));
        assert_eq!(active(&one, &dark), (named("D"), Scheme::Dark));
        assert_eq!(active(&one, &light), (None, Scheme::Light));

        // A slot's palette sets the effective scheme to its own.
        let crossed = Theme::preset(Preset::KdeBreeze)
            .with_palette_for(Scheme::Light, palette("D", Scheme::Dark));
        assert_eq!(active(&crossed, &light), (named("D"), Scheme::Dark));
    }

    #[test]
    fn the_three_palette_methods_replace_one_another() {
        let light = ThemeContext::light();
        let dark = ThemeContext::dark();
        let base = Theme::preset(Preset::KdeBreeze);
        let lock = || palette("X", Scheme::Dark);
        let pair = || (palette("L", Scheme::Light), palette("D", Scheme::Dark));

        // `with_palettes` clears the lock.
        let (l, d) = pair();
        let t = base.clone().with_palette(lock()).with_palettes(l, d);
        assert_eq!(active(&t, &light), (named("L"), Scheme::Light));

        // `with_palette` clears both slots.
        let (l, d) = pair();
        let t = base.clone().with_palettes(l, d).with_palette(lock());
        assert_eq!(active(&t, &light), (named("X"), Scheme::Dark));
        assert_eq!(active(&t, &dark), (named("X"), Scheme::Dark));

        // `with_palette_for` clears the lock and sets one slot only.
        let t = base
            .clone()
            .with_palette(lock())
            .with_palette_for(Scheme::Dark, palette("D2", Scheme::Dark));
        assert_eq!(active(&t, &light), (None, Scheme::Light));
        assert_eq!(active(&t, &dark), (named("D2"), Scheme::Dark));

        let (l, d) = pair();
        let t = base
            .clone()
            .with_palettes(l, d)
            .with_palette_for(Scheme::Dark, palette("D2", Scheme::Dark));
        assert_eq!(active(&t, &light), (named("L"), Scheme::Light));
        assert_eq!(active(&t, &dark), (named("D2"), Scheme::Dark));

        let t = base
            .with_palette_for(Scheme::Light, palette("L2", Scheme::Light))
            .with_palette_for(Scheme::Dark, palette("D2", Scheme::Dark));
        assert_eq!(active(&t, &light), (named("L2"), Scheme::Light));
        assert_eq!(active(&t, &dark), (named("D2"), Scheme::Dark));
    }

    fn with_contrast(ctx: ThemeContext, contrast: Contrast) -> ThemeContext {
        ThemeContext { contrast, ..ctx }
    }

    /// System settings whose entry for `(scheme, contrast)` is `colors`.
    fn system_entry(scheme: Scheme, contrast: Contrast, colors: OsColors) -> SystemOverrides {
        SystemOverrides {
            color_schemes: vec![((scheme, contrast), colors)],
            ..SystemOverrides::default()
        }
    }

    /// System settings carrying colours for `(scheme, contrast)`.
    fn system_with(scheme: Scheme, contrast: Contrast) -> SystemOverrides {
        let colors = OsColors {
            accent: Some(Color::WHITE),
            ..OsColors::default()
        };
        system_entry(scheme, contrast, colors)
    }

    #[test]
    fn only_an_entry_stating_a_colour_makes_a_level_available() {
        let ctx = with_contrast(ThemeContext::light(), Contrast::High);
        let theme = Theme::preset(Preset::KdeBreeze);
        let empty = theme.clone().with_system_overrides(system_entry(
            Scheme::Light,
            Contrast::High,
            OsColors::default(),
        ));
        assert_eq!(empty.effective_inputs(&ctx).0.contrast, Contrast::Normal);
        let stated = theme
            .clone()
            .with_system_overrides(system_with(Scheme::Light, Contrast::High));
        assert_eq!(stated.effective_inputs(&ctx).0.contrast, Contrast::High);

        // The contrast scalars, the effects and an empty status array state
        // no colour.
        let effects = KdeColorEffects::new(None, None, true);
        for colors in [
            OsColors {
                kde_contrast: Some(7),
                ..OsColors::default()
            },
            OsColors {
                kde_frame_contrast: Some(0.2),
                kde_effects: Some(effects),
                ..OsColors::default()
            },
            OsColors {
                status: Some([None; 4]),
                surfaces: Some(OsSurfaceOverrides {
                    window: Some(SurfaceColors::default()),
                    ..OsSurfaceOverrides::default()
                }),
                selection: Some(OsSelectionColors::default()),
                ..OsColors::default()
            },
        ] {
            assert!(!colors.states_a_colour(), "{colors:?}");
            let t = theme.clone().with_system_overrides(system_entry(
                Scheme::Light,
                Contrast::High,
                colors,
            ));
            assert_eq!(t.effective_inputs(&ctx).0.contrast, Contrast::Normal);
        }

        // A nested colour states one.
        let c = Some(Color::WHITE);
        let surface = SurfaceColors {
            divider: c,
            ..SurfaceColors::default()
        };
        for colors in [
            OsColors {
                status: Some([None, None, None, c]),
                ..OsColors::default()
            },
            OsColors {
                surfaces: Some(OsSurfaceOverrides {
                    button: Some(surface),
                    ..OsSurfaceOverrides::default()
                }),
                ..OsColors::default()
            },
            OsColors {
                text_selection: Some(OsSelectionColors {
                    unfocused_foreground: c,
                    ..OsSelectionColors::default()
                }),
                ..OsColors::default()
            },
            OsColors {
                kde_decoration_hover: c,
                ..OsColors::default()
            },
            OsColors {
                grid: c,
                ..OsColors::default()
            },
        ] {
            assert!(colors.states_a_colour(), "{colors:?}");
            let t = theme.clone().with_system_overrides(system_entry(
                Scheme::Light,
                Contrast::High,
                colors,
            ));
            assert_eq!(t.effective_inputs(&ctx).0.contrast, Contrast::High);
        }
    }

    /// Without the level requested, the nearest level below it that is
    /// available for the effective scheme: the system's `Increased` colours
    /// make it available, with or without an accent.
    #[test]
    fn a_missing_level_falls_back_to_the_nearest_available_below() {
        let ctx = with_contrast(ThemeContext::light(), Contrast::High);
        let theme = Theme::preset(Preset::KdeBreeze);
        let increased = theme
            .clone()
            .with_system_overrides(system_with(Scheme::Light, Contrast::Increased));
        assert_eq!(
            increased.effective_inputs(&ctx).0.contrast,
            Contrast::Increased
        );
        assert_eq!(
            increased.resolve(&ctx).context.contrast,
            Contrast::Increased
        );
        let accented = increased.with_accent(Color::WHITE);
        let (effective, inputs) = accented.effective_inputs(&ctx);
        assert_eq!(effective.contrast, Contrast::Increased);
        assert_eq!(inputs.accent, Some(Color::WHITE));
        assert_eq!(theme.effective_inputs(&ctx).0.contrast, Contrast::Normal);
        assert_eq!(
            theme
                .with_accent(Color::WHITE)
                .effective_inputs(&ctx)
                .0
                .contrast,
            Contrast::Normal
        );
    }

    /// The builder resolves for the effective context, not the one asked
    /// for: a dark palette resolved for a light context gives the dark
    /// scheme's icon theme.
    #[test]
    fn the_builder_receives_the_effective_context() {
        let theme = Theme::preset(Preset::KdeBreeze);
        let dark = theme.resolve(&ThemeContext::dark()).icons.theme;
        let light = theme.resolve(&ThemeContext::light()).icons.theme;
        assert_ne!(dark, light);
        let locked = theme.with_palette(palette("D", Scheme::Dark));
        assert_eq!(locked.resolve(&ThemeContext::light()).icons.theme, dark);
    }

    #[test]
    fn every_preset_defines_the_normal_contrast() {
        for preset in Preset::ALL {
            assert!(
                contrast_levels(*preset).contains(&Contrast::Normal),
                "{preset:?}"
            );
        }
        assert_eq!(contrast_levels(Preset::KdeBreeze), &[Contrast::Normal]);
    }

    #[test]
    fn a_contrast_without_data_resolves_to_the_nearest_defined_below() {
        let holds = |levels: &'static [Contrast]| move |level| levels.contains(&level);
        assert_eq!(
            nearest_below(Contrast::High, holds(&[Contrast::Normal])),
            Contrast::Normal
        );
        assert_eq!(
            nearest_below(
                Contrast::High,
                holds(&[Contrast::Normal, Contrast::Increased])
            ),
            Contrast::Increased
        );
        assert_eq!(
            nearest_below(
                Contrast::Increased,
                holds(&[Contrast::Normal, Contrast::High])
            ),
            Contrast::Normal
        );
        let theme = Theme::preset(Preset::KdeBreeze);
        for level in [Contrast::Increased, Contrast::High] {
            let ctx = with_contrast(ThemeContext::light(), level);
            assert_eq!(theme.resolve(&ctx).context.contrast, Contrast::Normal);
        }
        // The system's colours make a level available.
        let ctx = with_contrast(ThemeContext::light(), Contrast::Increased);
        let t = theme.with_system_overrides(system_with(Scheme::Light, Contrast::Increased));
        assert_eq!(t.resolve(&ctx).context.contrast, Contrast::Increased);
    }

    #[test]
    fn high_without_high_data_keeps_the_palette_and_the_accent() {
        let ctx = with_contrast(ThemeContext::light(), Contrast::High);
        let theme = Theme::preset(Preset::KdeBreeze)
            .with_palettes(palette("L", Scheme::Light), palette("D", Scheme::Dark))
            .with_accent(Color::WHITE);
        for t in [
            theme.clone(),
            // Colours for the other scheme do not make `High` available here.
            theme
                .clone()
                .with_system_overrides(system_with(Scheme::Dark, Contrast::High)),
        ] {
            let (effective, inputs) = t.effective_inputs(&ctx);
            assert_eq!(effective.contrast, Contrast::Normal);
            assert_eq!(inputs.palette.map(|p| &*p.name), Some("L"));
            assert_eq!(inputs.accent, Some(Color::WHITE));
            assert_eq!(t.resolve(&ctx).context.contrast, Contrast::Normal);
        }
    }

    #[test]
    fn high_with_system_high_colours_suspends_the_palette_and_the_accent() {
        let ctx = with_contrast(ThemeContext::light(), Contrast::High);
        let theme = Theme::preset(Preset::KdeBreeze)
            .with_system_overrides(system_with(Scheme::Light, Contrast::High))
            .with_accent(Color::WHITE);
        for t in [
            theme.clone().with_palette(palette("X", Scheme::Dark)),
            theme
                .clone()
                .with_palettes(palette("L", Scheme::Light), palette("D", Scheme::Dark)),
        ] {
            let (effective, inputs) = t.effective_inputs(&ctx);
            assert_eq!(effective.contrast, Contrast::High);
            assert_eq!(effective.scheme, Scheme::Light);
            assert_eq!(inputs.palette, None);
            assert_eq!(inputs.accent, None);
            let resolved = t.resolve(&ctx);
            assert_eq!(resolved.context.contrast, Contrast::High);
            assert_eq!(&*resolved.name, "KDE Breeze");

            let kept = t.preserve_palette_in_high_contrast(true);
            let (effective, inputs) = kept.effective_inputs(&ctx);
            assert!(inputs.palette.is_some());
            assert_eq!(inputs.accent, Some(Color::WHITE));
            // The locking palette's scheme has no `High` colours: the nearest
            // level below that is available for it.
            let want = if effective.scheme == Scheme::Light {
                Contrast::High
            } else {
                Contrast::Normal
            };
            assert_eq!(effective.contrast, want);
            assert_eq!(kept.resolve(&ctx).context.contrast, want);
        }

        // `High` not available for the context's scheme keeps the palette;
        // its scheme has the system's `High` colours, but `High` meets an
        // active palette only where the theme preserves it.
        let t = Theme::preset(Preset::KdeBreeze)
            .with_system_overrides(system_with(Scheme::Dark, Contrast::High))
            .with_palette(palette("X", Scheme::Dark))
            .with_accent(Color::WHITE);
        let (effective, inputs) = t.effective_inputs(&ctx);
        assert_eq!(inputs.palette.map(|p| &*p.name), Some("X"));
        assert_eq!(inputs.accent, Some(Color::WHITE));
        assert_eq!(effective.scheme, Scheme::Dark);
        assert_eq!(effective.contrast, Contrast::Normal);
        assert_eq!(t.resolve(&ctx).context.contrast, Contrast::Normal);
    }

    #[test]
    fn high_withheld_from_a_palette_takes_the_nearest_available_level() {
        let ctx = with_contrast(ThemeContext::light(), Contrast::High);
        let mut system = system_with(Scheme::Dark, Contrast::High);
        system
            .color_schemes
            .extend(system_with(Scheme::Dark, Contrast::Increased).color_schemes);
        let t = Theme::preset(Preset::KdeBreeze)
            .with_system_overrides(system)
            .with_palette(palette("X", Scheme::Dark));
        assert_eq!(t.effective_inputs(&ctx).0.contrast, Contrast::Increased);
    }

    #[test]
    fn a_preserved_palette_meets_high_where_its_scheme_has_it() {
        let ctx = with_contrast(ThemeContext::light(), Contrast::High);
        let t = Theme::preset(Preset::KdeBreeze)
            .with_system_overrides(system_with(Scheme::Dark, Contrast::High))
            .with_palette(palette("X", Scheme::Dark))
            .preserve_palette_in_high_contrast(true);
        let (effective, inputs) = t.effective_inputs(&ctx);
        assert_eq!(inputs.palette.map(|p| &*p.name), Some("X"));
        assert_eq!(effective.scheme, Scheme::Dark);
        assert_eq!(effective.contrast, Contrast::High);
        assert_eq!(t.resolve(&ctx).context.contrast, Contrast::High);
    }

    #[test]
    fn the_name_follows_its_precedence() {
        let system = SystemOverrides {
            theme_name: Some(Arc::from("System")),
            ..SystemOverrides::default()
        };
        let theme = Theme::preset(Preset::KdeBreeze);
        let with_system = theme.clone().with_system_overrides(system.clone());
        let with_palette = with_system
            .clone()
            .with_palettes(palette("Pal", Scheme::Light), palette("Pal", Scheme::Dark));
        let ctx = ThemeContext::light();
        assert_eq!(
            &*with_palette.clone().with_name("Mine").resolve(&ctx).name,
            "Mine"
        );
        assert_eq!(&*with_palette.resolve(&ctx).name, "Pal");
        assert_eq!(&*with_system.resolve(&ctx).name, "System");
        assert_eq!(&*theme.resolve(&ctx).name, "KDE Breeze");
        // A system read in one scheme names no resolve in the other.
        let read_light = theme.with_system_overrides(SystemOverrides {
            scheme: Some(Scheme::Light),
            ..system
        });
        assert_eq!(&*read_light.resolve(&ctx).name, "System");
        assert_eq!(
            &*read_light.resolve(&ThemeContext::dark()).name,
            "KDE Breeze"
        );
    }

    #[test]
    fn transforms_run_in_order_after_the_name() {
        let theme = Theme::preset(Preset::KdeBreeze)
            .with_name("Mine")
            .with_transform(|t| t.name = Arc::from("First"))
            .with_transform(|t| t.name = Arc::from("Second"));
        assert_eq!(&*theme.resolve_current().name, "Second");
        let debug = format!("{theme:?}");
        assert!(debug.contains("transforms: 2"), "{debug}");
        assert!(debug.contains("Mine"), "{debug}");
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
        let named = theme.clone().with_system_overrides(system.clone());
        assert_eq!(named.system_overrides(), Some(&system));
        assert_eq!(&*named.resolve_current().name, "Breeze Dark");
        // A system read in one scheme names only a resolve for that scheme.
        let read_in = |scheme| {
            theme.clone().with_system_overrides(SystemOverrides {
                scheme: Some(scheme),
                theme_name: Some(Arc::from("BreezeLight")),
                ..SystemOverrides::default()
            })
        };
        let light = read_in(Scheme::Light);
        assert_eq!(&*light.resolve(&ThemeContext::light()).name, "BreezeLight");
        assert_eq!(&*light.resolve(&ThemeContext::dark()).name, "KDE Breeze");
        let dark = read_in(Scheme::Dark);
        assert_eq!(&*dark.resolve(&ThemeContext::light()).name, "KDE Breeze");
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
    fn every_preset_resolves_through_the_builder_match() {
        let positions: Vec<usize> = Preset::ALL.iter().map(|p| position(*p)).collect();
        assert_eq!(positions, (0..7).collect::<Vec<_>>());
        let ctx = ThemeContext::dark();
        for preset in Preset::ALL {
            let resolved = Theme::preset(*preset).resolve(&ctx);
            // kde-breeze's builder is written; every other preset states nothing yet.
            let built = *preset == Preset::KdeBreeze;
            assert_eq!(
                resolved != nothing_stated(*preset, ctx.clone()),
                built,
                "{preset:?}"
            );
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
