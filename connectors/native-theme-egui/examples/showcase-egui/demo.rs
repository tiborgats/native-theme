//! Demo helpers: each applies one seam and records the instance it drew (spec §10.4).

use native_theme::icons::FreedesktopLoader;
use native_theme::theme::{IconRole, IconSet};
use native_theme_egui::{NativeThemeUiExt as _, Role, RoleVariant, Surface, ThemeAtlas, icons};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Seam {
    Base,
    Role(Role, RoleVariant),
    Surface(Surface),
}

/// What one drawn instance says about itself; the rows come from the manifest (`info.rs`).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct InstanceInfo {
    /// The widget's kind, e.g. "Button", "TextEdit (password)".
    pub kind: &'static str,
    pub seams: Vec<Seam>,
    /// The §4.7 accessors and `ResolvedTheme` leaves the helper read, with their values.
    pub read: Vec<(&'static str, String)>,
    /// *This instance* notes from the helper's own arguments, never a claim about the theme.
    pub notes: Vec<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Record {
    pub id: egui::Id,
    pub rect: egui::Rect,
    pub layer: egui::LayerId,
    pub info: InstanceInfo,
    /// One of §10.4's 21 container types, or a chrome panel.
    #[cfg(test)]
    pub container: bool,
    /// `Response::contains_pointer` (`egui/src/response.rs:333`) when recorded.
    pub contains_pointer: bool,
}

/// The instance Widget Info shows.
#[derive(Clone, Debug)]
pub(crate) struct Shown {
    pub id: egui::Id,
    pub info: InstanceInfo,
}

/// The pass's registrations; live for one pass.
#[derive(Default)]
pub(crate) struct Registry {
    records: Vec<Record>,
    /// What Widget Info shows.
    shown: Option<Shown>,
    /// The candidate and the `InputState::time` it was first chosen at.
    pending: Option<(egui::Id, f64)>,
    /// The page changed or a theme was installed since the last `end_pass`.
    screen_changed: bool,
    /// Ids recorded twice in one pass (§13.2 `no_id_is_recorded_twice`).
    #[cfg(test)]
    pub recorded_twice: std::collections::BTreeSet<String>,
    /// Kinds `base` recorded on a `Ui` whose style is not the base style: a widget inside a
    /// role's body, whose info would name rows it is not painted with.
    #[cfg(test)]
    pub base_off_base: std::collections::BTreeSet<&'static str>,
}

impl Registry {
    pub(crate) fn begin_pass(&mut self) {
        self.records.clear();
    }
    pub(crate) fn record(
        &mut self,
        response: &egui::Response,
        info: InstanceInfo,
        _container: bool,
    ) {
        #[cfg(test)]
        if self.records.iter().any(|r| r.id == response.id) {
            self.recorded_twice.insert(format!("{:?}", response.id));
        }
        self.records.push(Record {
            id: response.id,
            rect: response.interact_rect,
            layer: response.layer_id,
            info,
            #[cfg(test)]
            container: _container,
            contains_pointer: response.contains_pointer(),
        });
    }
    #[cfg(test)]
    pub(crate) fn records(&self) -> &[Record] {
        &self.records
    }

    /// The active page changed or a theme was installed: what is shown stays only if the next pass draws it again.
    pub(crate) fn screen_changed(&mut self) {
        self.screen_changed = true;
    }

    pub(crate) fn shown(&self) -> Option<&Shown> {
        self.shown.as_ref()
    }

    /// At the end of `ui`: choose, settle, show.
    pub(crate) fn end_pass(&mut self, ctx: &egui::Context, hold: Option<egui::Rect>) {
        let (now, pointer) = ctx.input(|i| (i.time, i.pointer.latest_pos()));
        if std::mem::take(&mut self.screen_changed)
            && self
                .shown
                .as_ref()
                .is_some_and(|s| !self.records.iter().any(|r| r.id == s.id))
        {
            self.shown = None;
            self.pending = None;
        }
        if pointer.zip(hold).is_some_and(|(p, hold)| hold.contains(p)) {
            self.pending = None;
            return;
        }
        // Smallest global area among the records containing the pointer; a tie goes to the first recorded.
        let choice = self
            .records
            .iter()
            .filter(|r| r.contains_pointer)
            .map(|r| {
                let rect = ctx
                    .layer_transform_to_global(r.layer)
                    .map_or(r.rect, |t| t.mul_rect(r.rect));
                (rect.area(), r)
            })
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, r)| r);
        let Some(choice) = choice else {
            self.pending = None;
            return;
        };
        if self.shown.as_ref().is_some_and(|s| s.id == choice.id) {
            self.pending = None;
            return;
        }
        let settle = crate::INFO_SETTLE.as_secs_f64();
        match self.pending {
            Some((id, since)) if id == choice.id => {
                let elapsed = now - since;
                if elapsed >= settle {
                    self.shown = Some(Shown {
                        id: choice.id,
                        info: choice.info.clone(),
                    });
                    self.pending = None;
                } else {
                    let left = std::time::Duration::try_from_secs_f64(settle - elapsed)
                        .unwrap_or(crate::INFO_SETTLE);
                    ctx.request_repaint_after(left);
                }
            }
            _ => {
                self.pending = Some((choice.id, now));
                ctx.request_repaint_after(crate::INFO_SETTLE);
            }
        }
    }
    /// Add the accessors or leaves a page read, or a *This instance* note, to the last record.
    pub(crate) fn amend_last(&mut self, f: impl FnOnce(&mut InstanceInfo)) {
        if let Some(last) = self.records.last_mut() {
            f(&mut last.info);
        }
    }
}

pub(crate) fn info(kind: &'static str, seams: Vec<Seam>) -> InstanceInfo {
    InstanceInfo {
        kind,
        seams,
        read: Vec::new(),
        notes: Vec::new(),
    }
}

/// `role`'s modifier from the installed atlas, `None` when none is installed (egui's own look).
fn role_modifier(
    ui: &egui::Ui,
    role: Role,
    variant: RoleVariant,
) -> Option<egui::style::StyleModifier> {
    let theme = ui.ctx().theme();
    ThemeAtlas::from_ctx(ui.ctx()).map(|atlas| atlas.role_modifier(theme, role, variant))
}

/// A widget inside `ui.native_scope(role, variant, ..)`: the closure adds it and
/// returns its `Response`, which is recorded with the role seam.
pub(crate) fn scoped(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let response = ui.native_scope(role, variant, add).inner;
    reg.record(
        &response,
        info(kind, vec![Seam::Role(role, variant)]),
        false,
    );
    response
}

/// The seam a helper applied to a `Ui` whose widgets are added straight into it — a row of
/// tabs or links, a menu bar's buttons — handed to the closure, so each widget records that
/// seam rather than one written a second time. A widget in a scope of its own sits in a child
/// `Ui` made at the cursor, which a wrapping row cannot move to its next line (`Ui::scope_dyn`,
/// `egui/src/ui.rs:2203-2213`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Applied(Seam);

impl Applied {
    /// Record a widget egui returns a `Response` for (a menu button, a submenu button).
    pub(crate) fn record(self, reg: &mut Registry, response: &egui::Response, kind: &'static str) {
        reg.record(response, info(kind, vec![self.0]), false);
    }

    /// Add a widget and record it with this seam.
    pub(crate) fn add(
        self,
        reg: &mut Registry,
        ui: &mut egui::Ui,
        kind: &'static str,
        add: impl FnOnce(&mut egui::Ui) -> egui::Response,
    ) -> egui::Response {
        let response = add(ui);
        self.record(reg, &response, kind);
        response
    }
}

/// A container inside `ui.native_scope(role, variant, ..)` — a row, a table, a grid whose
/// widgets take the role together: the closure lays them out, adding each through the `Applied`
/// seam, and returns the container's `Response`, recorded as a container with the role seam.
pub(crate) fn scoped_container(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui, Applied, &mut Registry) -> egui::Response,
) -> egui::Response {
    let seam = Seam::Role(role, variant);
    let response = ui
        .native_scope(role, variant, |ui| add(ui, Applied(seam), reg))
        .inner;
    reg.record(&response, info(kind, vec![seam]), true);
    response
}

/// A widget in `role`'s scope whose popup is an `Area` the scope does not reach (§1.5) — a
/// `ComboBox`: the closure gets the same role's modifier for `ComboBox::popup_style`
/// (`egui/src/containers/combo_box.rs:199`), and the seam as `Applied` for the popup's rows; one
/// role seam, applied twice and recorded with the widget and each row.
pub(crate) fn scoped_popup(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(
        &mut egui::Ui,
        Option<egui::style::StyleModifier>,
        Applied,
        &mut Registry,
    ) -> egui::Response,
) -> egui::Response {
    let modifier = role_modifier(ui, role, variant);
    let applied = Applied(Seam::Role(role, variant));
    let response = ui
        .native_scope(role, variant, |ui| add(ui, modifier, applied, reg))
        .inner;
    reg.record(
        &response,
        info(kind, vec![Seam::Role(role, variant)]),
        false,
    );
    response
}

/// A widget drawn with the base style: recorded with `Seam::Base`.
pub(crate) fn base(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    #[cfg(test)]
    if ui.style().as_ref() != ui.ctx().global_style().as_ref() {
        reg.base_off_base.insert(kind);
    }
    let response = add(ui);
    reg.record(&response, info(kind, vec![Seam::Base]), false);
    response
}

/// A container drawn with the base style — `Resize`, `Scene`, `Area`, `ui.group` and the
/// like, which no seam reaches: the closure gets the registry, so the body records its own
/// widgets, and returns the container's `Response`, recorded as a container with `Seam::Base`.
pub(crate) fn contained(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui, &mut Registry) -> egui::Response,
) -> egui::Response {
    let response = add(ui, reg);
    reg.record(&response, info(kind, vec![Seam::Base]), true);
    response
}

/// A container framed with `ui.native_frame(surface)`, its body optionally in a
/// role set with `native_set_style` as the first statement inside (§4.4, §1.5);
/// recorded as a container with the surface seam and the body's role seam. The body gets
/// the registry, so it records its own widgets.
pub(crate) fn framed<R>(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    surface: Surface,
    body: Option<(Role, RoleVariant)>,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui, &mut Registry) -> R,
) -> egui::InnerResponse<R> {
    let frame = ui.native_frame(surface);
    let out = frame.show(ui, |ui| {
        if let Some((role, variant)) = body {
            ui.native_set_style(role, variant);
        }
        add(ui, reg)
    });
    let mut seams = vec![Seam::Surface(surface)];
    seams.extend(body.map(|(r, v)| Seam::Role(r, v)));
    reg.record(&out.response, info(kind, seams), true);
    out
}

/// What a container that takes its look as values is given (§1.5, §3.2).
pub(crate) struct Chrome {
    /// `surface`'s frame: `Window::frame`, `Modal::frame`, `Popup::frame`, the tooltip's
    /// `popup.frame`, `Panel::frame`.
    pub frame: egui::Frame,
    /// `Surface::WindowTitleBar`'s frame for `Window::title_frame`; `None` unless asked for.
    pub title_frame: Option<egui::Frame>,
    /// `role`'s modifier for `Popup::style` (a tooltip's `popup.style`); `None` unless asked
    /// for, or with no atlas installed.
    pub modifier: Option<egui::style::StyleModifier>,
}

/// A `Window`, `Modal`, `Popup`, `Tooltip` or `Panel`: the helper makes the frames and the
/// modifier the arguments name, hands them to `show`, which builds the container and returns
/// its `Response` when it is shown, and records the container with exactly those seams. `show`
/// also gets the registry, so the body records itself — `styled` as the body's first statement,
/// the other helpers for its widgets.
pub(crate) fn surfaced(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    surface: Surface,
    title: bool,
    role: Option<(Role, RoleVariant)>,
    kind: &'static str,
    show: impl FnOnce(&mut egui::Ui, Chrome, &mut Registry) -> Option<egui::Response>,
) -> Option<egui::Response> {
    let chrome = Chrome {
        frame: ui.native_frame(surface),
        title_frame: title.then(|| ui.native_frame(Surface::WindowTitleBar)),
        modifier: role.and_then(|(r, v)| role_modifier(ui, r, v)),
    };
    let response = show(ui, chrome, reg)?;
    let mut seams = vec![Seam::Surface(surface)];
    if title {
        seams.push(Seam::Surface(Surface::WindowTitleBar));
    }
    seams.extend(role.map(|(r, v)| Seam::Role(r, v)));
    reg.record(&response, info(kind, seams), true);
    Some(response)
}

/// A popup or menu whose style arrives as a `StyleModifier` alone — `Popup::context_menu`, a
/// page `MenuBar`'s `style` and `MenuConfig::style` (§4.2): the closure passes the modifier to
/// its acceptor and returns the `Response` to record; it also gets the registry, so a menu's
/// items record themselves (a disabled item through `scoped` in `RoleVariant::Disabled`).
pub(crate) fn modifier(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui, Option<egui::style::StyleModifier>, &mut Registry) -> egui::Response,
) -> egui::Response {
    let modifier = role_modifier(ui, role, variant);
    let response = add(ui, modifier, reg);
    reg.record(
        &response,
        info(kind, vec![Seam::Role(role, variant)]),
        false,
    );
    response
}

/// A `MenuBar` given `role`'s modifier for both `MenuBar::style` and `MenuConfig::style` (§4.2):
/// the closure builds the bar with the modifier and records the menu buttons egui builds in it
/// through `Applied`; the bar's `Response` is recorded as a container with the role seam.
pub(crate) fn menu_bar(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui, egui::MenuBar, Applied, &mut Registry) -> egui::Response,
) -> egui::Response {
    let mut bar = egui::MenuBar::new();
    if let Some(modifier) = role_modifier(ui, role, variant) {
        bar = bar
            .style(modifier.clone())
            .config(egui::containers::menu::MenuConfig::new().style(modifier));
    }
    let seam = Seam::Role(role, variant);
    let response = add(ui, bar, Applied(seam), reg);
    reg.record(&response, info(kind, vec![seam]), true);
    response
}

/// `native_set_style` on a `Ui` the application did not create — a `Window`'s
/// or `Modal`'s body, an open menu (§1.5) — recorded on that `Ui`'s own response
/// (`egui/src/ui.rs:944`); the seam is returned as `Applied` for the widgets egui or the caller
/// adds to that body without a scope of their own.
pub(crate) fn styled(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
) -> Applied {
    let seam = Seam::Role(role, variant);
    ui.native_set_style(role, variant);
    let response = ui.response();
    reg.record(&response, info(kind, vec![seam]), true);
    Applied(seam)
}

/// A chrome panel's seams, each taken once (§10.4's chrome table): `live` set on the parent
/// `Ui` while the panel is shown, so its separator line takes that style (§4.4's recipe);
/// `surface`'s frame for `Panel::frame`; `body` set as the first statement inside
/// (`PanelSeams::enter`). `PanelSeams::record` records the panel with exactly these seams.
pub(crate) struct PanelSeams {
    surface: Surface,
    live: Option<(Role, RoleVariant)>,
    body: Option<(Role, RoleVariant)>,
    pub frame: egui::Frame,
}

impl PanelSeams {
    pub(crate) fn apply(
        ui: &mut egui::Ui,
        surface: Surface,
        live: Option<(Role, RoleVariant)>,
        body: Option<(Role, RoleVariant)>,
    ) -> Self {
        if let Some((role, variant)) = live {
            ui.native_set_style(role, variant);
        }
        Self {
            surface,
            live,
            body,
            frame: ui.native_frame(surface),
        }
    }

    /// The body's role, set as the first statement inside the panel's closure; `Applied` for
    /// the widgets added straight into the body.
    pub(crate) fn enter(&self, ui: &mut egui::Ui) -> Option<Applied> {
        let (role, variant) = self.body?;
        ui.native_set_style(role, variant);
        Some(Applied(Seam::Role(role, variant)))
    }

    pub(crate) fn record(&self, reg: &mut Registry, response: &egui::Response, kind: &'static str) {
        let mut seams = vec![Seam::Surface(self.surface)];
        for (role, variant) in [self.live, self.body].into_iter().flatten() {
            let seam = Seam::Role(role, variant);
            if !seams.contains(&seam) {
                seams.push(seam);
            }
        }
        reg.record(response, info(kind, seams), true);
    }
}

/// An icon of `role` from the chosen set and theme at `size` points (§10.4's icon rule, §9.2):
/// a freedesktop icon is read from `icon_theme` — the chosen theme; the OS's own only when the
/// choice is `system`, which passes `None` — in the text colour
/// (`FreedesktopLoader::{new, theme, size, color, load}`, `native-theme/src/icons.rs:127`, `:157`,
/// `:137`, `:143`, `:177`); a bundled icon is tinted the text colour. `None` where the set or
/// theme lacks it: the caller shows it as absent, never from another set.
pub(crate) fn role_image(
    ui: &egui::Ui,
    role: IconRole,
    set: IconSet,
    icon_theme: Option<&str>,
    size: f32,
) -> Option<egui::Image<'static>> {
    let text = ui.visuals().text_color();
    let mut key = icons::IconKey::role(role, set).size(size);
    let data = if set == IconSet::Freedesktop {
        let [r, g, b, _] = text.to_srgba_unmultiplied();
        // The saturating float-to-int cast `IconKey::size` makes (§4.10, §7.1).
        let mut loader = FreedesktopLoader::new(role)
            .size(size.round() as u16)
            .color([r, g, b]);
        if let Some(name) = icon_theme {
            loader = loader.theme(name);
            key = key.icon_theme(name);
        }
        loader.load()?
    } else {
        key = key.tint(text);
        native_theme::icons::load_icon(role, set)?
    };
    icons::to_image(ui.ctx(), &key, &data)
        .map(|image| image.fit_to_exact_size(egui::Vec2::splat(size)))
}
