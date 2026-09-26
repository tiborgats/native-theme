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
