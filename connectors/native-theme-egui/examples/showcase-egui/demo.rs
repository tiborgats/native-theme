//! Demo helpers: each applies one seam and records the instance it drew (spec §10.4).

use native_theme::icons::{FreedesktopLoader, SegoeIconsLoader, SfSymbolsLoader};
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
    /// *This instance* notes from the helper's own arguments, never a claim about the theme: what
    /// each is about, and the note.
    pub notes: Vec<(&'static str, String)>,
    /// The element of `docs/showcase-elements.toml` the instance is, which Widget Info shows.
    pub element: Option<String>,
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
    /// Whether Widget Info may show it: every instance but a layout box (`Registry::untarget`).
    pub target: bool,
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
    /// The rectangle of each element of `docs/showcase-elements.toml` this pass draws, by id, in
    /// window-content coordinates (logical pixels): what `--dump-layout` writes.
    places: std::collections::BTreeMap<String, egui::Rect>,
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
        self.places.clear();
    }

    /// Where the element `id` of `docs/showcase-elements.toml` is this pass: `rect` in the
    /// coordinates of `ui`'s layer, taken to the window's.
    pub(crate) fn place(&mut self, ui: &egui::Ui, id: &str, rect: egui::Rect) {
        self.place_on(ui.ctx(), ui.layer_id(), id, rect);
    }

    /// `place` for a rectangle of `layer`.
    pub(crate) fn place_on(
        &mut self,
        ctx: &egui::Context,
        layer: egui::LayerId,
        id: &str,
        rect: egui::Rect,
    ) {
        let rect = ctx
            .layer_transform_to_global(layer)
            .map_or(rect, |t| t.mul_rect(rect));
        self.places.insert(id.to_string(), rect);
    }

    /// The elements placed this pass.
    pub(crate) fn places(&self) -> &std::collections::BTreeMap<String, egui::Rect> {
        &self.places
    }

    /// The instance `response` answers for is the element `id`: Widget Info shows the element's
    /// rows when it is hovered, and the element is placed where the instance is drawn.
    pub(crate) fn tag(&mut self, id: &str, response: &egui::Response) {
        self.name(id, response);
        self.place_on(&response.ctx, response.layer_id, id, response.rect);
    }

    /// The instance `response` answers for is the element `id`, placed elsewhere: Widget Info
    /// shows the element's rows when it is hovered.
    pub(crate) fn name(&mut self, id: &str, response: &egui::Response) {
        if let Some(record) = self.records.iter_mut().rev().find(|r| r.id == response.id) {
            record.info.element = Some(id.to_string());
        }
    }

    /// The instance `response` answers for is a layout box, not a Widget Info target: hovering
    /// the empty room it holds keeps what Widget Info shows, as the other two showcases do.
    pub(crate) fn untarget(&mut self, response: &egui::Response) {
        if let Some(record) = self.records.iter_mut().rev().find(|r| r.id == response.id) {
            record.target = false;
        }
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
            target: true,
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
            .filter(|r| r.contains_pointer && r.target)
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
    /// Note on the last record that it is a button drawn in a `ghost` scope.
    pub(crate) fn ghost_last(&mut self) {
        self.amend_last(|i| {
            i.notes.push((GHOST_NOTE.0, GHOST_NOTE.1.to_string()));
        });
    }
}

/// What a button drawn in a `ghost` scope notes under *This instance*.
pub(crate) const GHOST_NOTE: (&str, &str) = (
    "fill and border at rest",
    "transparent, the border's width kept: a Ghost button, as gpui-component's (GC/button/button.rs:942, :1032)",
);

/// Make the rest of `ui` Ghost: the resting entry's fill and border colour transparent, the
/// border's width kept, so a button in it is transparent at rest, as gpui-component's ghost
/// button is (`GC/button/button.rs:942`, `:1032`), and filled and bordered hovered and pressed
/// as its role states. It keeps its full frame, and so its size, in every state, where
/// `frame_when_inactive(false)` lays it out at rest without the frame's border, which the
/// hovered frame adds to its size (`egui/src/widgets/button.rs:364-368`). Each button drawn in
/// it is noted with `Registry::ghost_last` where it is recorded.
pub(crate) fn ghost(ui: &mut egui::Ui) {
    let rest = &mut ui.style_mut().visuals.widgets.inactive;
    rest.weak_bg_fill = egui::Color32::TRANSPARENT;
    rest.bg_stroke.color = egui::Color32::TRANSPARENT;
}

/// Make the rest of `ui` a tool button's, the icon-only button of a toolbar, the status bar and
/// the tab row's page menu: Ghost ([`ghost`]), padded by `button.border.padding` and with no
/// border and no minimum height, so it is its icon and that padding — the tool buttons of
/// `docs/showcase-elements.toml` name the four padding sides and no border width or minimum
/// size. egui pads a button by one pair (`Spacing::button_padding`), so the left side pads
/// left and right and the top side top and bottom; a side the theme leaves unstated keeps the
/// scope's own.
pub(crate) fn tool_button(ui: &mut egui::Ui, padding: &native_theme::theme::ResolvedPadding) {
    ghost(ui);
    let own = ui.spacing().button_padding;
    let spacing = ui.spacing_mut();
    spacing.button_padding =
        egui::vec2(padding.left.unwrap_or(own.x), padding.top.unwrap_or(own.y));
    spacing.interact_size.y = 0.0;
    let widgets = &mut ui.style_mut().visuals.widgets;
    for state in [
        &mut widgets.noninteractive,
        &mut widgets.inactive,
        &mut widgets.hovered,
        &mut widgets.active,
        &mut widgets.open,
    ] {
        state.bg_stroke.width = 0.0;
        state.expansion = 0.0;
    }
}

/// A toolbar row's items `toolbar.item_gap` apart, which the toolbar scope's
/// `item_spacing.x` carries, and `layout.widget_gap` apart where the theme states no item gap,
/// as the other two showcases space theirs.
pub(crate) fn toolbar_gap(
    ui: &mut egui::Ui,
    t: &native_theme::theme::ResolvedTheme,
    layout: &native_theme::theme::LayoutTheme,
) {
    if t.toolbar.item_gap.is_none()
        && let Some(gap) = layout.widget_gap
    {
        ui.spacing_mut().item_spacing.x = gap;
    }
}

pub(crate) fn info(kind: &'static str, seams: Vec<Seam>) -> InstanceInfo {
    InstanceInfo {
        kind,
        seams,
        read: Vec::new(),
        notes: Vec::new(),
        element: None,
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

/// `role`'s modifier for an open menu (`MenuConfig::style`): the popup padded inside its border
/// by `popover.border.padding` on each side the theme states, where egui pads by its own
/// `menu_margin` (`Frame::popup`, `egui/src/containers/frame.rs:220-230`, a frame whose total
/// margin is that margin and its stroke), and its rows one under another, `menu.row_height`
/// apart where the theme states it: the theme states no gap between two menu rows, where egui
/// would put `item_spacing.y` between them.
fn menu_modifier(
    ui: &egui::Ui,
    role: Role,
    variant: RoleVariant,
) -> Option<egui::style::StyleModifier> {
    let theme = ui.ctx().theme();
    let atlas = ThemeAtlas::from_ctx(ui.ctx())?;
    let role_style = atlas.role_modifier(theme, role, variant);
    let padding = atlas.resolved_for(theme).popover.border.padding;
    Some(egui::style::StyleModifier::new(
        move |style: &mut egui::Style| {
            role_style.apply(style);
            style.spacing.menu_margin =
                native_theme_egui::convert::to_margin(style.spacing.menu_margin, &padding);
            style.spacing.item_spacing.y = 0.0;
        },
    ))
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

/// What a widget of the companion crate notes under *This instance*.
pub(crate) const WIDGETS_NOTE: (&str, &str) = (
    "drawn by",
    "native-theme-egui-widgets, in the role's scope, painted from the role's leaves themselves (docs/todo_egui-widgets-spec.md §4)",
);

/// A widget of the companion crate `native-theme-egui-widgets`, which opens `role`'s scope in
/// `variant` itself: the closure adds it, and its `Response` is recorded with that role seam
/// and `WIDGETS_NOTE`.
pub(crate) fn widget(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let response = add(ui);
    let mut info = info(kind, vec![Seam::Role(role, variant)]);
    info.notes
        .push((WIDGETS_NOTE.0, WIDGETS_NOTE.1.to_string()));
    reg.record(&response, info, false);
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
    framed_with(
        reg,
        ui,
        (surface, |_: &mut egui::Frame| {}),
        body,
        kind,
        add,
    )
}

/// [`framed`] with the surface's frame as `adjust` changes it: a card's padding the theme
/// leaves unstated, say.
pub(crate) fn framed_with<R>(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    (surface, adjust): (Surface, impl FnOnce(&mut egui::Frame)),
    body: Option<(Role, RoleVariant)>,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui, &mut Registry) -> R,
) -> egui::InnerResponse<R> {
    let mut frame = ui.native_frame(surface);
    adjust(&mut frame);
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
        // The bar's titles edge to edge, each as tall as its text and `menu.border.padding`: the
        // theme states no gap between two titles and no bar height, and a title is no button
        // `interact_size.y` would make taller.
        let titles = modifier.clone();
        let bar_style = egui::style::StyleModifier::new(move |style: &mut egui::Style| {
            titles.apply(style);
            style.spacing.item_spacing.x = 0.0;
            style.spacing.interact_size.y = 0.0;
        });
        bar = bar.style(bar_style);
    }
    if let Some(menus) = menu_modifier(ui, role, variant) {
        bar = bar.config(egui::containers::menu::MenuConfig::new().style(menus));
    }
    let seam = Seam::Role(role, variant);
    let response = add(ui, bar, Applied(seam), reg);
    reg.record(&response, info(kind, vec![seam]), true);
    response
}

/// A menu button added straight into a `Ui` that carries `seam` — a tab row's page menu —
/// recorded with that seam; its menu given `menu`'s modifier through `MenuConfig::style`
/// (§4.2), and the open menu's own `Ui` styled and recorded with `menu`, as the chrome's menus
/// are (`styled`); `add` fills the menu, adding its items through the `Applied` it gets.
pub(crate) fn menu_button(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    seam: Applied,
    kind: &'static str,
    button: egui::Button<'_>,
    menu: (Role, RoleVariant),
    add: impl FnOnce(&mut egui::Ui, Applied, &mut Registry),
) -> egui::Response {
    let (role, variant) = menu;
    let mut menu_button = egui::containers::menu::MenuButton::from_button(button);
    if let Some(modifier) = menu_modifier(ui, role, variant) {
        menu_button = menu_button.config(egui::containers::menu::MenuConfig::new().style(modifier));
    }
    let (response, _) = menu_button.ui(ui, |ui| {
        let open = styled_menu(reg, ui, role, variant);
        add(ui, open, reg);
    });
    seam.record(reg, &response, kind);
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

/// [`styled`] for an open menu's `Ui`, its rows one under another as [`menu_modifier`] lays
/// them: `native_set_style` puts back the cell's `item_spacing`.
pub(crate) fn styled_menu(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
) -> Applied {
    let applied = styled(reg, ui, role, variant, "Menu");
    ui.spacing_mut().item_spacing.y = 0.0;
    applied
}

/// A chrome panel's seams, each taken once (§10.4's chrome table): `live` set on the parent
/// `Ui` while the panel is shown, so its separator line takes that style (§4.4's recipe);
/// `surface`'s frame for `Panel::frame`; `body` set as the first statement inside
/// (`PanelSeams::enter`). `PanelSeams::record` records the panel with exactly these seams.
pub(crate) struct PanelSeams {
    surface: Surface,
    live: Option<(Role, RoleVariant)>,
    body: Option<(Role, RoleVariant)>,
    /// Set by `PanelSeams::unpadded`: the panel's own inner margin is dropped and its content
    /// pads itself by this `layout.container_margin`.
    content_margin: Option<Option<f32>>,
    /// Set by `PanelSeams::lift_margin`: the frame's inner margin, moved onto the content below
    /// a row that sits flush at the panel's top.
    lifted: Option<egui::Margin>,
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
            content_margin: None,
            lifted: None,
            frame: ui.native_frame(surface),
        }
    }

    /// The frame's inner margin, taken off the frame and handed to the caller, who pads the
    /// content below a row that sits flush at the panel's top with it: the page tabs, as the
    /// gpui showcase's sit above the page's padding (`showcase-gpui/app.rs:1784`). Recorded
    /// with the panel.
    pub(crate) fn lift_margin(&mut self) -> egui::Margin {
        let margin = std::mem::take(&mut self.frame.inner_margin);
        self.lifted = Some(margin);
        margin
    }

    /// The panel with no inner margin of its own, its content padded by `container_margin`
    /// (`layout.container_margin`) where the theme states one, as the gpui showcase pads its
    /// side panel's settings and inspector
    /// (`connectors/native-theme-gpui/examples/showcase-gpui/demo.rs:606-630`), so a separator
    /// and a tab bar's rule inside run from edge to edge. Recorded with the panel.
    pub(crate) fn unpadded(mut self, container_margin: Option<f32>) -> Self {
        self.frame.inner_margin = egui::Margin::ZERO;
        self.content_margin = Some(container_margin);
        self
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
        let mut info = info(kind, seams);
        if let Some(margin) = self.content_margin {
            info.notes.push((
                "inner margin",
                "none: the content pads itself by layout.container_margin, as the gpui showcase's side panel does".to_string(),
            ));
            info.read.push((
                "layout.container_margin",
                margin.map_or_else(|| "not stated: no padding".to_string(), |m| m.to_string()),
            ));
        }
        if let Some(margin) = self.lifted {
            info.notes.push((
                "inner margin",
                format!(
                    "{margin:?}, the surface's, below the page tabs: the tabs sit flush at the top, as the gpui showcase's do"
                ),
            ));
        }
        reg.record(response, info, true);
    }
}

/// An icon of `role` from the chosen set and theme at `size` points (§10.4's icon rule, §9.2):
/// a freedesktop icon is read from `icon_theme` — the chosen theme; the OS's own only when the
/// choice is `system`, which passes `None` — in the text colour
/// (`FreedesktopLoader::{new, theme, size, color, load}`, `native-theme/src/icons.rs:127`, `:157`,
/// `:137`, `:143`, `:177`); an SF Symbol or a Segoe glyph is loaded in the text colour too
/// (`SfSymbolsLoader::color`, `native-theme/src/icons.rs:283`; `SegoeIconsLoader::color`,
/// `:353`); a bundled icon is tinted the text colour. `None` where the set or theme lacks it:
/// the caller shows it as absent, never from another set.
pub(crate) fn role_image(
    ui: &egui::Ui,
    role: IconRole,
    set: IconSet,
    icon_theme: Option<&str>,
    size: f32,
) -> Option<egui::Image<'static>> {
    let text = ui.visuals().text_color();
    let [r, g, b, _] = text.to_srgba_unmultiplied();
    let mut key = icons::IconKey::role(role, set).size(size);
    let data = match set {
        IconSet::Freedesktop => {
            // The saturating float-to-int cast `IconKey::size` makes (§4.10, §7.1).
            let mut loader = FreedesktopLoader::new(role)
                .size(size.round() as u16)
                .color([r, g, b]);
            if let Some(name) = icon_theme {
                loader = loader.theme(name);
                key = key.icon_theme(name);
            }
            loader.load()?
        }
        IconSet::SfSymbols => SfSymbolsLoader::new(role).color([r, g, b]).load()?,
        IconSet::SegoeIcons => SegoeIconsLoader::new(role).color([r, g, b]).load()?,
        _ => {
            key = key.tint(text);
            native_theme::icons::load_icon(role, set)?
        }
    };
    icons::to_image(ui.ctx(), &key, &data)
        .map(|image| image.fit_to_exact_size(egui::Vec2::splat(size)))
}

/// A chrome icon the gpui showcase draws by a gpui-component `IconName` that no `IconRole` stands
/// for (`showcase-gpui/support.rs:430-460`), loaded by the name native-theme-gpui gives it in
/// each set (parity rule R5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChromeIcon {
    SquareTerminal,
    RotateCw,
    PanelLeft,
}

impl ChromeIcon {
    /// The icon's name in `set` (`connectors/native-theme-gpui/src/icons.rs`: Lucide `:226`,
    /// `:239`, `:245`; Material `:358`, `:371`, `:377`; freedesktop `:474`, `:479`, `:669-675`,
    /// the last by desktop, as GTK desktops and KDE name it apart); `None` in a set that has
    /// none — SF Symbols and Segoe Fluent here — where the button shows its label instead.
    fn name(self, set: IconSet) -> Option<&'static str> {
        match (self, set) {
            (Self::SquareTerminal, IconSet::Lucide) => Some("square-terminal"),
            (Self::SquareTerminal, IconSet::Material) => Some("terminal"),
            (Self::SquareTerminal, IconSet::Freedesktop) => Some("utilities-terminal"),
            (Self::RotateCw, IconSet::Lucide) => Some("rotate-cw"),
            (Self::RotateCw, IconSet::Material) => Some("rotate_right"),
            (Self::RotateCw, IconSet::Freedesktop) => Some("object-rotate-right"),
            (Self::PanelLeft, IconSet::Lucide) => Some("panel-left"),
            (Self::PanelLeft, IconSet::Material) => Some("side_navigation"),
            (Self::PanelLeft, IconSet::Freedesktop) => Some(if gtk_desktop() {
                "sidebar-show"
            } else {
                "sidebar-expand-left"
            }),
            _ => None,
        }
    }
}

/// Whether the desktop names its icons as GNOME's Adwaita does, as native-theme-gpui decides it
/// (`connectors/native-theme-gpui/src/icons.rs:429-437`).
#[cfg(target_os = "linux")]
fn gtk_desktop() -> bool {
    use native_theme::detect::LinuxDesktop;
    matches!(
        native_theme::detect::detect_linux_desktop(),
        LinuxDesktop::Gnome
            | LinuxDesktop::Budgie
            | LinuxDesktop::Cinnamon
            | LinuxDesktop::Mate
            | LinuxDesktop::Xfce
    )
}

/// No freedesktop icon theme is read outside Linux.
#[cfg(not(target_os = "linux"))]
fn gtk_desktop() -> bool {
    false
}

/// `icon` from the chosen set and theme at `size` points, loaded and coloured as `role_image`
/// loads a role's; `None` where the set or theme lacks it, never another set's icon.
pub(crate) fn named_image(
    ui: &egui::Ui,
    icon: ChromeIcon,
    set: IconSet,
    icon_theme: Option<&str>,
    size: f32,
) -> Option<egui::Image<'static>> {
    let name = icon.name(set)?;
    let text = ui.visuals().text_color();
    let [r, g, b, _] = text.to_srgba_unmultiplied();
    let mut key = icons::IconKey::name(name, set).size(size);
    let data = if set == IconSet::Freedesktop {
        // The saturating float-to-int cast `IconKey::size` makes (§4.10, §7.1).
        let mut loader = FreedesktopLoader::new(name)
            .size(size.round() as u16)
            .color([r, g, b]);
        if let Some(theme) = icon_theme {
            loader = loader.theme(theme);
            key = key.icon_theme(theme);
        }
        loader.load()?
    } else {
        key = key.tint(text);
        native_theme::icons::load_icon(name, set)?
    };
    icons::to_image(ui.ctx(), &key, &data)
        .map(|image| image.fit_to_exact_size(egui::Vec2::splat(size)))
}

/// The `FontFamily::Name` the showcase registers the OS's semibold face under. Application
/// code only: the connector registers one face per family and never a `Name` (§8).
pub(crate) const SEMIBOLD_FAMILY: &str = "showcase-semibold";

/// The `FontFamily::Name` the showcase registers the OS's face of the theme's family at `weight`
/// under, for text the theme sets at a weight other than its body's: the section headings, at
/// `text_scale.section_heading.weight`. Application code only, as `SEMIBOLD_FAMILY`.
pub(crate) fn weight_family(weight: u16) -> egui::FontFamily {
    egui::FontFamily::Name(format!("showcase-weight-{weight}").into())
}

/// The OS's faces of the theme's family at the weights the showcase draws besides the body's:
/// its semibold face, for the inspector's headings gpui draws `font_semibold()` (parity decision
/// 3), and the face at each weight of each variant's text-scale roles (`text_scale.*.weight`)
/// and table header (`list.header_font.weight`), for the page's section headings, its
/// Typography group and its table. Each is `native_theme::fonts::system_face` at its weight, added with
/// `Context::add_font` after every `ThemeAtlas::install`, whose `set_fonts` replaces the
/// definitions: the semibold one under `SEMIBOLD_FAMILY`, the others under [`weight_family`].
/// `add_font` skips a name the loaded fonts already hold (`egui/src/context.rs:2133-2143`), and
/// they still hold the previous install's face when the next install is pending, so each
/// registration takes a name of its own.
#[derive(Default)]
pub(crate) struct Semibold {
    registrations: u64,
}

impl Semibold {
    /// After `atlas.install(ctx)`: the faces of the family the connector's font plan registers
    /// for the theme's light variant (§4.6) — the family it names, or the platform's substitute
    /// where the OS lacks it — where the OS has them; none otherwise, and the text keeps the
    /// regular face.
    pub(crate) fn register(&mut self, ctx: &egui::Context, atlas: &ThemeAtlas) {
        #[cfg(feature = "system-fonts")]
        {
            let light = atlas.resolved_for(egui::Theme::Light);
            let body = &light.defaults.font;
            self.registrations += 1;
            let mut families = vec![(
                crate::SEMIBOLD_WEIGHT,
                egui::FontFamily::Name(SEMIBOLD_FAMILY.into()),
            )];
            for scheme in [egui::Theme::Light, egui::Theme::Dark] {
                let t = atlas.resolved_for(scheme);
                let s = &t.text_scale;
                for weight in [
                    s.caption.weight,
                    s.section_heading.weight,
                    s.dialog_title.weight,
                    s.display.weight,
                    t.list.header_font.weight,
                ] {
                    let family = weight_family(weight);
                    if weight != t.defaults.font.weight
                        && !families.iter().any(|(_, f)| *f == family)
                    {
                        families.push((weight, family));
                    }
                }
            }
            // The family the connector's font plan draws the body in: the theme's where the OS
            // has it, else the platform's substitute (`native_theme::fonts::substitute_family`,
            // fontconfig's on Linux), as `FontPlan::from_system` takes it.
            let drawn = if native_theme::fonts::system_face(&body.family, body.weight, body.style)
                .is_some()
            {
                body.family.to_string()
            } else {
                native_theme::fonts::substitute_family(&body.family)
                    .unwrap_or_else(|| body.family.to_string())
            };
            for (weight, family) in families {
                let Some(face) = native_theme::fonts::system_face(&drawn, weight, body.style)
                else {
                    continue;
                };
                let mut data = egui::FontData::from_owned(face.data.to_vec());
                data.index = face.index;
                // A face with a `wght` axis is set to the weight; a static face ignores it (§8.3).
                data.tweak.coords = native_theme_egui::fonts::weight_coords(weight);
                ctx.add_font(egui::epaint::text::FontInsert::new(
                    &format!("{family:?}-{}", self.registrations),
                    data,
                    vec![egui::epaint::text::InsertFontFamily {
                        family,
                        priority: egui::epaint::text::FontPriority::Highest,
                    }],
                ));
            }
        }
        #[cfg(not(feature = "system-fonts"))]
        let _ = (ctx, atlas, &mut self.registrations);
    }
}

/// `size` in the semibold family where the fonts this pass draws with hold it, else in the
/// proportional one: a `Name` family the fonts do not hold panics at the first text that
/// names it (`epaint/src/text/fonts.rs:1025`), and none is registered where the OS has no
/// such face or feature `system-fonts` is off.
pub(crate) fn semibold_font(ui: &egui::Ui, size: f32) -> egui::FontId {
    let family = egui::FontFamily::Name(SEMIBOLD_FAMILY.into());
    let held = ui
        .ctx()
        .fonts(|f| f.definitions().families.contains_key(&family));
    egui::FontId::new(
        size,
        if held {
            family
        } else {
            egui::FontFamily::Proportional
        },
    )
}

/// The family text at `weight` is drawn in: the proportional one at the body's own weight, the
/// face registered under [`weight_family`] at another where the fonts this pass draws with hold
/// it, and the proportional one where they do not (none is registered where the OS has no such
/// face or feature `system-fonts` is off): a `Name` family the fonts do not hold panics at the
/// first text that names it (`epaint/src/text/fonts.rs:1025`).
pub(crate) fn weighted_family(ui: &egui::Ui, weight: u16, body_weight: u16) -> egui::FontFamily {
    if weight == body_weight {
        return egui::FontFamily::Proportional;
    }
    let family = weight_family(weight);
    let held = ui
        .ctx()
        .fonts(|f| f.definitions().families.contains_key(&family));
    if held {
        family
    } else {
        egui::FontFamily::Proportional
    }
}

/// A page's or the inspector's section heading, in the theme's section-heading role
/// (`text_scale.section_heading`, the section divider of `docs/platform-facts.md` §2.19): its
/// size and line height through `text_role_font` and `text_role_line_height`, its weight
/// through [`weighted_family`], in the text colour — as the iced showcase's section titles
/// are. Without an installed atlas, egui's `Body` size, semibold.
pub(crate) fn heading_text(ui: &egui::Ui, text: impl Into<String>) -> egui::RichText {
    if ThemeAtlas::from_ctx(ui.ctx()).is_none() {
        let size = egui::TextStyle::Body.resolve(ui.style()).size;
        return egui::RichText::new(text)
            .font(semibold_font(ui, size))
            .color(ui.visuals().text_color());
    }
    role_text(ui, native_theme_egui::TextRole::SectionHeading, text)
}

/// `text` one line box tall: the size of the font `ui` sets a widget's text in (its
/// `override_font_id`, else `style`'s) times `defaults.line_height`, the platform's line box, per
/// call. The connector writes the line box only where it is taller than the font's own row
/// (`Spacing::extra_text_line_spacing`, which egui adds between rows, never above the first); a
/// `TextStyle` is a font and nothing more, so a line box shorter than the row — Adwaita's 1.21 of
/// a Cantarell-sized font — is reachable only per text, `RichText::line_height`
/// (`egui/src/widget_text.rs:174`). Without an installed atlas, the text as it is.
pub(crate) fn lined(
    ui: &egui::Ui,
    text: impl Into<String>,
    style: egui::TextStyle,
) -> egui::RichText {
    let text = egui::RichText::new(text);
    let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) else {
        return text;
    };
    let size = ui
        .style()
        .override_font_id
        .as_ref()
        .map_or_else(|| style.resolve(ui.style()).size, |font| font.size);
    let multiplier = atlas.resolved_for(ui.ctx().theme()).defaults.line_height;
    text.line_height(Some(size * multiplier))
}

/// Text in one of the theme's text-scale roles (`text_scale.*`): its size and line height
/// through `text_role_font` and `text_role_line_height`, its weight through
/// [`weighted_family`], in the text colour. Without an installed atlas, egui's `Body` text.
pub(crate) fn role_text(
    ui: &egui::Ui,
    role: native_theme_egui::TextRole,
    text: impl Into<String>,
) -> egui::RichText {
    let colour = ui.visuals().text_color();
    let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) else {
        return egui::RichText::new(text).color(colour);
    };
    let t = atlas.resolved_for(ui.ctx().theme());
    let prefs = atlas.accessibility();
    let size = native_theme_egui::text_role_font(t, role, prefs).size;
    let weight = native_theme_egui::text_role_weight(t, role);
    egui::RichText::new(text)
        .font(egui::FontId::new(
            size,
            weighted_family(ui, weight, t.defaults.font.weight),
        ))
        .line_height(Some(native_theme_egui::text_role_line_height(
            t, role, prefs,
        )))
        .color(colour)
}

/// A row of tabs, as `tab_bar` draws it.
pub(crate) struct TabBar<'a, T> {
    /// The row's kind, recorded as a container.
    pub kind: &'static str,
    /// Each tab's kind.
    pub tab_kind: &'static str,
    pub tabs: &'a [(T, &'static str)],
    pub current: T,
    /// `layout.container_margin`: the row's padding on its left and right, where the theme
    /// states one.
    pub margin: Option<f32>,
    /// Whether the tabs scroll sideways where the row is too narrow for them, as the page
    /// tabs do; the trailing widgets stay at the row's right end.
    pub scroll: bool,
    /// Whether the strip spans the width it is given (the page and inspector tab rows), or is
    /// as wide as its tabs (the Basic page's).
    pub full_width: bool,
    /// The row's element in `docs/showcase-elements.toml`, and its tabs', in order: a tab past
    /// the end of `tab_elements` is not one of the list's.
    pub element: &'static str,
    pub tab_elements: &'a [&'static str],
}

/// A row of tabs as the theme states them, in one `Role::Tab` scope (§10.4): each tab a
/// `Button::new(label).selected(..)`, whose own flag picks the selected tab's colours (§6.2) —
/// the cell's `tab.background_color`, `tab.active_background` and `tab.active_text_color`,
/// `tab.hover_background` (in place of the idle fill, over the strip) and `tab.hover_text_color`,
/// `tab.min_height`, the tab's padding and font — at least `tab.min_width` wide, which egui's
/// `Button` never reads from the style (connector spec §5.3, T18(a)), on a strip in
/// `tab.bar_background`. `tab.border` is the selected tab's (docs/platform-facts.md §2.11: KDE
/// strokes the selected tab only, an unselected one with no pen; WinUI's selected tab alone has
/// a border), so an unselected tab's stroke is set per call to the scope's width in no colour,
/// which keeps every tab the size its padding gives; and a tab is rounded on its two top corners
/// only, as both platforms that state a tab radius round it (Breeze `CornersTop`, WinUI
/// `TopCornerRadiusFilterConverter`, §2.11), the model's one `tab.border.corner_radius` on
/// each. The theme states no line under the selected tab or under the row, so none is drawn.
/// `trailing` adds what follows the tabs. Returns the tab clicked.
pub(crate) fn tab_bar<T: Copy + PartialEq>(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    t: &native_theme::theme::ResolvedTheme,
    bar: TabBar<'_, T>,
    trailing: impl FnOnce(&mut egui::Ui, Applied, &mut Registry),
) -> Option<T> {
    let min_width = t.tab.min_width;
    let mut picked = None;
    let strip = scoped_container(
        reg,
        ui,
        Role::Tab,
        RoleVariant::Normal,
        bar.kind,
        |ui, tab, reg| {
            let padding = egui::Vec2::X * bar.margin.unwrap_or_default();
            // The strip in `tab.bar_background`, the tab scope's `panel_fill`.
            let strip = egui::Frame::NONE
                .fill(ui.visuals().panel_fill)
                .inner_margin(padding);
            let out = strip.show(ui, |ui| {
                if bar.full_width {
                    ui.set_min_width(ui.available_width());
                }
                let mut tabs = |ui: &mut egui::Ui, reg: &mut Registry| {
                    // The scope's `tab.border` radius and width.
                    let idle = ui.visuals().widgets.inactive;
                    let top = egui::CornerRadius {
                        sw: 0,
                        se: 0,
                        ..idle.corner_radius
                    };
                    let no_pen =
                        egui::Stroke::new(idle.bg_stroke.width, egui::Color32::TRANSPARENT);
                    // Where the theme states neither side of `tab.border.padding`, a tab is its
                    // text and border, at least `tab.min_width` wide, as the other two
                    // showcases' tabs are: no padding stands in for the unstated one.
                    let padding = &t.tab.border.padding;
                    if padding.left.is_none() && padding.right.is_none() {
                        ui.spacing_mut().button_padding.x = idle.bg_stroke.width;
                    }
                    // The tabs `tab.item_gap` apart, and side by side where the theme states no
                    // gap, as the other two showcases' tabs are.
                    ui.spacing_mut().item_spacing.x = t.tab.item_gap.unwrap_or_default();
                    for (index, (value, label)) in bar.tabs.iter().enumerate() {
                        let selected = *value == bar.current;
                        let mut button =
                            egui::Button::new(lined(ui, *label, egui::TextStyle::Button))
                                .selected(selected)
                                .min_size(egui::Vec2::X * min_width)
                                .corner_radius(top);
                        if !selected {
                            button = button.stroke(no_pen);
                        }
                        let r = tab.add(reg, ui, bar.tab_kind, |ui| ui.add(button));
                        if let Some(id) = bar.tab_elements.get(index) {
                            reg.tag(id, &r);
                        }
                        if r.clicked() {
                            picked = Some(*value);
                        }
                    }
                };
                // The trailing widgets are the showcase's own tool buttons, which no tab leaf
                // states: Ghost, transparent at rest.
                let trailing = |ui: &mut egui::Ui, reg: &mut Registry| {
                    ui.scope(|ui| {
                        ghost(ui);
                        trailing(ui, tab, reg);
                    });
                };
                if bar.scroll {
                    // In a row, so the right-to-left layout takes the row's height, not the rest
                    // of the panel's.
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            trailing(ui, reg);
                            // No bar: the trailing page menu reaches every tab, and a tab row
                            // scrolls without a scroll bar under it, as a native tab bar does.
                            egui::ScrollArea::horizontal()
                                .id_salt(bar.kind)
                                .auto_shrink([false, true])
                                .scroll_bar_visibility(
                                    egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                                )
                                .show(ui, |ui| {
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::Center),
                                        |ui| tabs(ui, reg),
                                    );
                                });
                        });
                    });
                } else {
                    ui.horizontal(|ui| {
                        tabs(ui, reg);
                        trailing(ui, reg);
                    });
                }
            });
            out.response
        },
    );
    reg.amend_last(|i| {
        i.read.extend([
            ("tab.min_width", min_width.to_string()),
            (
                "layout.container_margin",
                bar.margin
                    .map_or_else(|| "not stated: no padding".to_string(), |m| m.to_string()),
            ),
        ]);
        if t.tab.item_gap.is_none() {
            i.notes.push((
                "the gap between tabs",
                "not stated by the theme: none, the tabs side by side".to_string(),
            ));
        }
    });
    reg.tag(bar.element, &strip);
    picked
}
