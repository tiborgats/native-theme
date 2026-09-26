use crate::atlas::carry_name_keys;
use crate::plugin::record_scope;
use crate::style::egui_preset;
use crate::{Role, RoleVariant, Surface, ThemeAtlas};

mod sealed {
    pub trait Sealed {}
    impl Sealed for egui::Ui {}
    impl Sealed for native_theme::SystemTheme {}
}

/// Role scoping and frame recipes on an [`egui::Ui`], from the atlas installed in its
/// `Context` ([`ThemeAtlas::from_ctx`](crate::ThemeAtlas::from_ctx)), in the scheme `self.ctx().theme()` reports
/// (`egui/src/context.rs:2158`). Every method degrades to egui's own behaviour when no atlas
/// is installed; none panics and none returns an `Option`.
///
/// **Invariant, and it is not enforced by the seal:** a method name added here must exist
/// on neither `egui::Ui` nor `egui::Context`. Rust tries the receiver types in order — `Ui`,
/// `&Ui`, `&mut Ui`, then through `Deref` (`egui/src/ui.rs:92-99`) `Context`, `&Context` — and
/// at each step an inherent method beats a trait method, but the first step that has a match
/// wins. So a trait method taking `&mut self` silently rebinds every existing `ui.<name>()`
/// call of a `Context` method of that name, and a trait method taking `&self` every call of
/// an inherent `Ui` method of that name taking `&mut self`; where the inherent `Ui` method is
/// reached first, it shadows the new trait method instead (all three verified with a probe on
/// rustc 1.98.1). §12.2 clause 6 carries the same rule.
pub trait NativeThemeUiExt: sealed::Sealed {
    /// Run `add_contents` in a child `Ui` styled for `role` in `variant`.
    ///
    /// Sugar over `ui.scope_builder(egui::UiBuilder::new().style(..), ..)`
    /// (`egui/src/ui.rs:2194`); a plain `ui.scope(..)` (`egui/src/ui.rs:2186`) when no atlas
    /// is installed. Every `TextStyle::Name` key of `self.style()` that the role style lacks
    /// is carried into the child, so an application's named text styles resolve inside the
    /// scope as outside it (`TextStyle::resolve` would panic on a missing one,
    /// `egui/src/style.rs:112-120`); the role's `Arc` is handed over as is when there is no
    /// such key; a copy is made only when there is one.
    ///
    /// It also records, for this pass, the child `Ui`'s `unique_id` (`egui/src/ui.rs:357-359`)
    /// with the role style's `widgets.active.corner_radius`, so the focus ring around a widget
    /// inside it takes the role's corners: the plugin uses the innermost recorded `Ui` whose
    /// final rect contains the focused widget's `interact_rect` on its layer (§6.18).
    ///
    /// **Does not reach `Area`-based containers** (`Window`, `Popup`, `Tooltip`, `Modal`,
    /// menus, `ComboBox` popups) when wrapped around them: their content `Ui` is built from
    /// the `Context`'s style (§1.5). Popups and menus take [`ThemeAtlas::role_modifier`](crate::ThemeAtlas::role_modifier);
    /// `Window` and `Modal` take [`NativeThemeUiExt::native_frame`] for their chrome and this
    /// scope, or [`NativeThemeUiExt::native_set_style`], as the first statement *inside* their
    /// closure for their body (§1.5, §3.3).
    fn native_scope<R>(
        &mut self,
        role: Role,
        variant: RoleVariant,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::InnerResponse<R>;

    /// Replace this `Ui`'s own style with `role`'s in `variant` for the remainder of the `Ui`
    /// (`Ui::set_style`, `egui/src/ui.rs:387`), carrying its `TextStyle::Name` keys as
    /// [`NativeThemeUiExt::native_scope`] does; nothing changes when no atlas is installed.
    /// Records this `Ui` and the role's corner radius for the focus ring as `native_scope`
    /// does, a later call on the same `Ui` replacing the record (§6.18). On the root `Ui`
    /// that `Context::run_ui` hands over, the ring reads the root's own style at the end of
    /// the pass instead, so a `Ui::reset_style` there (§4.4) is seen. The record covers the
    /// whole `Ui` while the restyle applies from this call on, and on any other `Ui` a style
    /// the application later sets by other means (`Ui::set_style`, `Ui::reset_style`) is
    /// not seen by the ring — §6.18's stated residuals.
    ///
    /// The spelling for a role that must be live on a `Ui` the application does not create:
    /// the body of a `Window` or `Modal` (§1.5), and the parent of a `Panel`, whose separator
    /// line is painted from the parent's style (§4.4).
    fn native_set_style(&mut self, role: Role, variant: RoleVariant);

    /// `surface`'s [`egui::Frame`] ([`ThemeAtlas::surface_frame`](crate::ThemeAtlas::surface_frame)). When nothing is installed
    /// it is the frame egui itself builds for that container from this `Ui`'s style, the
    /// same as passing no frame: `Frame::window` for [`Surface::Window`](crate::Surface::Window) and
    /// [`Surface::WindowTitleBar`](crate::Surface::WindowTitleBar) (`egui/src/containers/window.rs:630-631`),
    /// `Frame::popup` for [`Surface::Dialog`](crate::Surface::Dialog), [`Surface::Popover`](crate::Surface::Popover) and [`Surface::Tooltip`](crate::Surface::Tooltip)
    /// (`egui/src/containers/modal.rs:100`, `egui/src/containers/popup.rs:603`),
    /// `Frame::group` for [`Surface::Card`](crate::Surface::Card) (`egui/src/ui.rs:2147-2148`),
    /// `Frame::side_top_panel` for [`Surface::Panel`](crate::Surface::Panel) (`egui/src/containers/panel.rs:950`)
    /// and `Frame::central_panel` for [`Surface::CentralPanel`](crate::Surface::CentralPanel) (`:1243`).
    #[must_use = "this returns the frame recipe; hand it to a container's `.frame(..)`"]
    fn native_frame(&self, surface: Surface) -> egui::Frame;
}

impl NativeThemeUiExt for egui::Ui {
    fn native_scope<R>(
        &mut self,
        role: Role,
        variant: RoleVariant,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::InnerResponse<R> {
        let Some(atlas) = ThemeAtlas::from_ctx(self.ctx()) else {
            return self.scope(add_contents);
        };
        let theme = self.ctx().theme();
        let style = carry_name_keys(self.style(), atlas.scheme(theme).cell(role, variant));
        let radius = style.visuals.widgets.active.corner_radius;
        self.scope_builder(egui::UiBuilder::new().style(style), |child| {
            record_scope(child.ctx(), child.unique_id(), radius);
            add_contents(child)
        })
    }

    fn native_set_style(&mut self, role: Role, variant: RoleVariant) {
        let Some(atlas) = ThemeAtlas::from_ctx(self.ctx()) else {
            return;
        };
        let theme = self.ctx().theme();
        let style = carry_name_keys(self.style(), atlas.scheme(theme).cell(role, variant));
        let radius = style.visuals.widgets.active.corner_radius;
        record_scope(self.ctx(), self.unique_id(), radius);
        self.set_style(style);
    }

    fn native_frame(&self, surface: Surface) -> egui::Frame {
        match ThemeAtlas::from_ctx(self.ctx()) {
            Some(atlas) => atlas.surface_frame(self.ctx().theme(), surface),
            None => egui_preset(surface, self.style()),
        }
    }
}

use native_theme::theme::{ColorMode, ResolvedTheme, Theme};
use native_theme::{AccessibilityPreferences, Result, SystemTheme};

/// Compile an atlas from one resolved variant. Pure mapping; borrows; never touches the OS.
///
/// egui keeps a separate `Style` per `egui::Theme`; with a single variant this installs the
/// same values into both, so an OS light/dark flip changes nothing.
///
/// Built with `AccessibilityPreferences::default()`. Two variants, the user's preferences
/// or any other input go through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) (`to_theme(r, name)` is
/// `ThemeAtlas::builder(name, r, r).build()`).
#[must_use = "this builds the styles; it does not install them"]
pub fn to_theme(resolved: &ResolvedTheme, name: &str) -> ThemeAtlas {
    ThemeAtlas::builder(name, resolved, resolved).build()
}

/// Compile an atlas from a bundled preset, including its [`LayoutTheme`](crate::LayoutTheme)
/// (`native_theme::theme::Theme::layout`, `native-theme/src/model/mod.rs:269`) and, with
/// feature `system-fonts`, the preset's typefaces through
/// `fonts::FontPlan::from_system(&light)` (`light` the resolved `ColorMode::Light`
/// variant), whose notes join [`ThemeAtlas::notes`](crate::ThemeAtlas::notes), as in [`from_system`]. Its
/// [`ThemeAtlas::name`](crate::ThemeAtlas::name) is the preset's `Theme::name` (`native-theme/src/model/mod.rs:257`), as
/// in both siblings (`connectors/native-theme-iced/src/lib.rs:260`,
/// `connectors/native-theme-gpui/src/lib.rs:263`).
/// An application with fonts of its own builds through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with
/// `.fonts(FontPlan::from_system(&light).with_base(its_defs))`; this constructor uses
/// egui's default base.
///
/// The atlas carries **both** of the preset's variants — `ColorMode::Light` into the light
/// `Style`, `ColorMode::Dark` into the dark one, each through `Theme::resolve`
/// (`native-theme/src/model/mod.rs:450`), whose `pick_variant` cross-fallback serves a
/// one-variant preset (`:362-368`) and whose resolution is `ThemeMode::resolve_system`'s
/// (`:472`; `native-theme/src/resolve/mod.rs:267-268`) — for
/// the reason [`SystemThemeExt::to_egui_atlas`] gives: under `ThemePreference::System` egui
/// selects between its two styles every pass, so a preset compiled into one scheme would
/// leave the other scheme wrong. This is the one deviation from
/// `native_theme_gpui::from_preset`, which compiles the `is_dark` variant only.
///
/// The icon set is those two `Resolved`s' `icon_set`, a theme-level value they share
/// (`native-theme/src/model/mod.rs:468-470`); the icon theme
/// is each variant's own `Resolved::icon_theme`, `Theme::resolve(ColorMode::Light)`'s for
/// the light scheme and `ColorMode::Dark`'s for the dark one: [`ThemeAtlas::icon_set`](crate::ThemeAtlas::icon_set),
/// [`ThemeAtlas::icon_theme`](crate::ThemeAtlas::icon_theme).
///
/// The atlas carries no OS colour mode ([`ThemeAtlas::os_mode`](crate::ThemeAtlas::os_mode) is `None`), so on Linux,
/// where the integration reports none either, egui falls back to `Options::fallback_theme`.
/// To follow the OS there, build through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with [`Builder::os_mode`](crate::Builder::os_mode).
///
/// `is_dark` selects the returned [`ResolvedTheme`](crate::ResolvedTheme); it does not pin egui's colour scheme —
/// that is egui's `ctx.set_theme(..)` ([`ThemeAtlas::install`](crate::ThemeAtlas::install)). It is explicit and never
/// inferred: some presets (`solarized`, `gruvbox`) have ambiguous lightness.
///
/// `prefs` is applied as [`Builder::accessibility`](crate::Builder::accessibility) applies it. Pass
/// `&AccessibilityPreferences::default()` for none, or
/// `&AccessibilityPreferences::from_system()` to honour the OS preferences under a preset —
/// accessibility is orthogonal to the theme choice, which is why
/// `native_theme_gpui::from_preset` takes the same argument
/// (`connectors/native-theme-gpui/src/lib.rs:257-261`).
///
/// # Errors
/// Propagates `Theme::preset` and `Theme::resolve` failures.
#[must_use = "this builds the styles; it does not install them"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> Result<(ThemeAtlas, ResolvedTheme)> {
    let spec = Theme::preset(name)?;
    // Both variants, each through `Theme::resolve`, whose `pick_variant` cross-fallback serves
    // a one-variant preset (`native-theme/src/model/mod.rs:362-368`).
    let light = spec.resolve(ColorMode::Light)?;
    let dark = spec.resolve(ColorMode::Dark)?;
    let mut builder = ThemeAtlas::builder(&spec.name, &light.variant, &dark.variant)
        .layout(&spec.layout)
        .accessibility(prefs)
        // a theme-level value both `Resolved`s share (`native-theme/src/model/mod.rs:468-470`)
        .icon_set(light.icon_set);
    if let Some(icon_theme) = light.icon_theme.as_deref() {
        builder = builder.icon_theme(egui::Theme::Light, icon_theme);
    }
    if let Some(icon_theme) = dark.icon_theme.as_deref() {
        builder = builder.icon_theme(egui::Theme::Dark, icon_theme);
    }
    #[cfg(feature = "system-fonts")]
    {
        builder = builder.fonts(crate::fonts::FontPlan::from_system(&light.variant));
    }
    let atlas = builder.build();
    let resolved = if is_dark { dark.variant } else { light.variant };
    Ok((atlas, resolved))
}

/// Detect and compile the OS theme, carrying **both** variants, the OS colour mode, the OS
/// accessibility preferences, the OS [`LayoutTheme`](crate::LayoutTheme) and — with feature `system-fonts` — the
/// OS typefaces through `fonts::FontPlan::from_system`, whose notes join
/// [`ThemeAtlas::notes`](crate::ThemeAtlas::notes). Exactly [`SystemThemeExt::to_egui_atlas`] of
/// `SystemTheme::from_system()`. The `bool` is `sys.mode.is_dark()` —
/// the OS preference, not a luminance guess — and the [`ResolvedTheme`](crate::ResolvedTheme) is the variant it
/// selects, as in both siblings' `from_system`.
/// An application with fonts of its own builds through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with
/// `.fonts(FontPlan::from_system(&light).with_base(its_defs))`; this constructor uses
/// egui's default base.
///
/// The layout is `SystemTheme::layout` (`native-theme/src/lib.rs:488`), the platform reader's
/// values merged field-wise over the preset's, fed to [`Builder::layout`](crate::Builder::layout); so this path
/// reaches the same spacing fields [`from_preset`] does. The accessibility preferences travel
/// inside the atlas ([`ThemeAtlas::accessibility`](crate::ThemeAtlas::accessibility)), which is why the tuple has no fourth
/// element as `native_theme_iced::from_system`'s does
/// (`connectors/native-theme-iced/src/lib.rs:283-288`).
///
/// # Errors
/// Propagates `SystemTheme::from_system`.
#[must_use = "this builds the styles; it does not install them"]
pub fn from_system() -> Result<(ThemeAtlas, ResolvedTheme, bool)> {
    let sys = SystemTheme::from_system()?;
    let atlas = sys.to_egui_atlas();
    let is_dark = sys.mode.is_dark();
    let resolved = if is_dark { sys.dark } else { sys.light };
    Ok((atlas, resolved, is_dark))
}

/// Compile a detected [`SystemTheme`](native_theme::SystemTheme). Sealed, like the extension trait of §4.5.
pub trait SystemThemeExt: sealed::Sealed {
    /// Compile an atlas carrying both OS variants, the OS colour mode, the OS accessibility
    /// preferences and the OS layout (`SystemTheme::layout`, `native-theme/src/lib.rs:488`).
    ///
    /// Deviation from `SystemThemeExt::to_iced_theme`
    /// (`connectors/native-theme-iced/src/lib.rs:305`) and `to_gpui_theme`
    /// (`connectors/native-theme-gpui/src/lib.rs:328`), which return one toolkit theme: egui
    /// stores one `Style` per `egui::Theme` and, under `ThemePreference::System`, picks one
    /// every pass from `RawInput::system_theme` (`egui/src/memory/mod.rs:358-359`), so
    /// returning one variant would guarantee a half-themed application. That input is the
    /// live OS scheme on macOS and Windows, where winit 0.30.13 reports one and sends
    /// `ThemeChanged` (egui-winit 0.36.2 `State::on_window_event`, lines 450–451 of its
    /// `src/lib.rs`); on Linux winit reports `None`, and the
    /// scheme egui sees is this atlas's OS mode, supplied by the install plugin (§3.2)
    /// and renewed by the next atlas installed (a `ThemeWatcher` rebuild).
    ///
    /// Equivalent to — and the chain to spell out when adding [`Builder::style_patch`](crate::Builder::style_patch) or a
    /// plan of the application's own —
    /// `ThemeAtlas::builder(&self.name, &self.light, &self.dark).layout(&self.layout)`
    /// `.accessibility(&self.accessibility).os_mode(self.mode).icon_set(self.icon_set)`
    /// `.icon_theme(egui::Theme::Light, light).icon_theme(egui::Theme::Dark, dark)`
    /// `.fonts(plan).build()` (each `icon_theme` only when
    /// `self.icon_theme_for(ColorMode::Light)` or `(ColorMode::Dark)` is `Some`, §9.2), `plan`
    /// being `fonts::FontPlan::from_system(&self.light)` with feature `system-fonts`, and no
    /// `.fonts(..)` call without. An application with fonts
    /// of its own builds through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with
    /// `.fonts(FontPlan::from_system(&light).with_base(its_defs))`; this constructor uses
    /// egui's default base.
    #[must_use = "this builds the styles; it does not install them"]
    fn to_egui_atlas(&self) -> ThemeAtlas;
}

impl SystemThemeExt for SystemTheme {
    fn to_egui_atlas(&self) -> ThemeAtlas {
        let mut builder = ThemeAtlas::builder(&self.name, &self.light, &self.dark)
            .layout(&self.layout)
            .accessibility(&self.accessibility)
            .os_mode(self.mode)
            .icon_set(self.icon_set);
        if let Some(icon_theme) = self.icon_theme_for(ColorMode::Light) {
            builder = builder.icon_theme(egui::Theme::Light, icon_theme);
        }
        if let Some(icon_theme) = self.icon_theme_for(ColorMode::Dark) {
            builder = builder.icon_theme(egui::Theme::Dark, icon_theme);
        }
        #[cfg(feature = "system-fonts")]
        {
            builder = builder.fonts(crate::fonts::FontPlan::from_system(&self.light));
        }
        builder.build()
    }
}
