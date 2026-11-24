//! Demo helpers: each applies one seam and records the instance it drew (spec §10.4).

use std::borrow::Cow;

use native_theme::Rgba;
use native_theme::icons::{AnimatedIcon, FreedesktopLoader, IconProvider, IconRole, IconSet};
use native_theme_egui::{
    NativeThemeUiExt as _, Note, Role, RoleVariant, Surface, icons, menu_bar_modifier,
    role_modifier,
};
use native_theme_showcase_support::DIALOG_PADDING_PX;
use native_theme_showcase_support::next::values::StatedFont;
use native_theme_showcase_support::status::DrawnFamily;

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
    /// What egui draws here unthemed, where no leaf states it (an unstated leaf, a Ghost
    /// button's resting fill): the lines Widget Info shows under *Not themeable* (R11), each
    /// what it is about and what is drawn.
    pub not_themeable: Vec<(&'static str, String)>,
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
    /// The instance's whole rectangle (`Response::rect`), where `rect` is the part its clip
    /// leaves (`interact_rect`, egui 0.36.2 `src/ui.rs:926`): a cell cut short by its view's
    /// edge has a whole rectangle past it.
    #[cfg(test)]
    pub whole: egui::Rect,
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
    /// The `Ui` each Basic group was laid out in this pass, by the group's index in reading
    /// order: where a group ends, which its last control need not show.
    #[cfg(test)]
    pub basic_groups: Vec<(usize, egui::Rect)>,
}

impl Registry {
    pub(crate) fn begin_pass(&mut self) {
        self.records.clear();
        self.places.clear();
        #[cfg(test)]
        self.basic_groups.clear();
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
            #[cfg(test)]
            whole: response.rect,
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
    /// Add a *Not themeable* line to the last record.
    pub(crate) fn amend_last(&mut self, f: impl FnOnce(&mut InstanceInfo)) {
        if let Some(last) = self.records.last_mut() {
            f(&mut last.info);
        }
    }
    /// Add the *Not themeable* line `what`: `note` to the last record.
    pub(crate) fn note_last(&mut self, what: &'static str, note: &str) {
        self.amend_last(|i| i.not_themeable.push((what, note.to_string())));
    }
    /// Add a *Not themeable* line to every record this pass names the element `id`: a chrome
    /// element is its panel and the row inside it, and Widget Info shows either.
    pub(crate) fn note_element(&mut self, id: &str, what: &'static str, note: &str) {
        for record in self
            .records
            .iter_mut()
            .filter(|r| r.info.element.as_deref() == Some(id))
        {
            record.info.not_themeable.push((what, note.to_string()));
        }
    }
    /// Note on the last record that it is a button drawn in a `ghost` scope.
    pub(crate) fn ghost_last(&mut self) {
        self.amend_last(|i| {
            i.not_themeable
                .push((GHOST_NOTE.0, GHOST_NOTE.1.to_string()));
        });
    }
}

/// What a button drawn in a `ghost` scope notes under *Not themeable*.
pub(crate) const GHOST_NOTE: (&str, &str) = (
    "fill and border at rest",
    "transparent, the border's width kept: a Ghost button, as gpui-component's (gpui-component 0.9.1 src/button/button.rs:1027, :1117)",
);

/// Make the rest of `ui` Ghost: the resting entry's fill and border colour transparent, the
/// border's width kept, so a button in it is transparent at rest, as gpui-component's ghost
/// button is (gpui-component 0.9.1 `src/button/button.rs:1027`, `bg_color`'s `Ghost` arm, and
/// `:1117`, `border_color`'s), and filled and bordered hovered and pressed
/// as its role states. It keeps its full frame, and so its size, in every state, as
/// `Button::frame_when_inactive(false)` does since egui 0.37, which hides a resting
/// button's fill and stroke and keeps its margin (egui 0.37.0 `src/widgets/button.rs:191-205`,
/// `src/theme/default_style.rs:77-84`). Each button drawn in
/// it is noted with `Registry::ghost_last` where it is recorded.
pub(crate) fn ghost(ui: &mut egui::Ui) {
    let rest = &mut ui.style_mut().visuals.widgets.inactive;
    rest.weak_bg_fill = egui::Color32::TRANSPARENT;
    rest.bg_stroke.color = egui::Color32::TRANSPARENT;
}

/// The sides of `padding`, top, right, bottom and left (`next::padding_of`'s order), the theme
/// leaves unstated, joined by ", " in that order as a *Not themeable* line names them; `None`
/// where it states all four.
pub(crate) fn unstated_sides(padding: [Option<f32>; 4]) -> Option<String> {
    let sides: Vec<&str> = ["top", "right", "bottom", "left"]
        .into_iter()
        .zip(padding)
        .filter(|(_, stated)| stated.is_none())
        .map(|(side, _)| side)
        .collect();
    (!sides.is_empty()).then(|| sides.join(", "))
}

/// `margin` with each side `padding` (top, right, bottom, left) leaves unstated set to that
/// side of `fill`, as `convert::to_margin` sets a frame's margin, and each stated side as it
/// is: a frame whose stated sides its surface already pads by takes a fallback on the others.
pub(crate) fn pad_unstated(
    margin: egui::Margin,
    padding: [Option<f32>; 4],
    fill: [f32; 4],
) -> egui::Margin {
    let [top, right, bottom, left] = padding;
    let [fill_top, fill_right, fill_bottom, fill_left] = fill;
    let side = |stated: Option<f32>, fill: f32| stated.is_none().then_some(fill);
    native_theme_egui::convert::to_margin(
        margin,
        &native_theme::theme::ResolvedPadding {
            top: side(top, fill_top),
            right: side(right, fill_right),
            bottom: side(bottom, fill_bottom),
            left: side(left, fill_left),
        },
    )
}

pub(crate) fn info(kind: &'static str, seams: Vec<Seam>) -> InstanceInfo {
    InstanceInfo {
        kind,
        seams,
        not_themeable: Vec::new(),
        element: None,
    }
}

/// `role`'s modifier for an open menu (`MenuConfig::style`), of the installed 0.8.0 theme: the
/// popup framed by `popover.panel`'s stroke colour, stroke width and corner radius, each where
/// the theme states it, where the base style frames every menu by the window border
/// (`Frame::popup` reads `window_stroke` and `menu_corner_radius`), padded inside that border by
/// the menu role's `menu.panel(NONE).padding` on each side the theme states it (E4: kde-breeze
/// 3, the items 3 + the 1 px line from the panel's edge), else by `popover.panel.padding` on
/// each side the theme states, where egui pads by its own `menu_margin` (`Frame::popup`,
/// `egui/src/containers/frame.rs:220-230`, a frame whose total margin is that margin and its
/// stroke), and its rows one under another, `menu.item(NONE).min_size[1]` apart where the theme
/// states it:
/// the theme states no gap between two menu rows, where egui would put `item_spacing.y`
/// between them.
fn menu_modifier(
    ui: &egui::Ui,
    role: Role,
    variant: RoleVariant,
) -> Option<egui::style::StyleModifier> {
    use native_theme::next::WidgetState;
    use native_theme_egui::next::{padding_of, radius_of, stroke_color_of, stroke_width_of};
    let theme = ui.ctx().theme();
    let atlas = crate::app::OldAtlas::from_ctx(ui.ctx())?;
    let role_style = atlas.role_modifier(theme, role, variant);
    let r = atlas.next_shown(ui.ctx())?;
    let panel = &r.popover.panel;
    let (color, width, radius) = (
        stroke_color_of(panel),
        stroke_width_of(panel),
        radius_of(panel),
    );
    // the sides the menu role pads (`menu_margin`) keep its padding
    let (menu, popover) = (
        padding_of(&r.menu.panel(WidgetState::NONE)),
        padding_of(panel),
    );
    let [top, right, bottom, left] =
        std::array::from_fn(|side| match (menu.get(side), popover.get(side)) {
            (Some(None), Some(&side)) => side,
            _ => None,
        });
    let padding = native_theme::theme::ResolvedPadding {
        top,
        right,
        bottom,
        left,
    };
    Some(egui::style::StyleModifier::new(
        move |style: &mut egui::Style| {
            use native_theme_egui::convert::{to_corner_radius, to_margin};
            role_style.apply(style);
            if let Some(color) = color {
                style.visuals.window_stroke.color = color;
            }
            if let Some(width) = width {
                style.visuals.window_stroke.width = width;
            }
            if let Some(radius) = radius {
                style.visuals.menu_corner_radius =
                    to_corner_radius(style.visuals.menu_corner_radius, radius);
            }
            style.spacing.menu_margin = to_margin(style.spacing.menu_margin, &padding);
            style.spacing.item_spacing.y = 0.0;
        },
    ))
}

/// A widget inside `ui.native_scope(role, variant, ..)`: the closure adds it and
/// returns its `Response`, which is recorded with the role seam; a menu row's or an expander
/// header's least height grown above a text scale of 1 ([`grow_rows`]).
pub(crate) fn scoped(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let response = ui
        .native_scope(role, variant, |ui| {
            grow_rows(ui, role);
            add(ui)
        })
        .inner;
    reg.record(
        &response,
        info(kind, vec![Seam::Role(role, variant)]),
        false,
    );
    response
}

/// In `role`'s scope, where it is `Role::Menu` or `Role::Expander`: a stated
/// `menu.item(NONE).min_size[1]` or `expander.header(NONE).min_size[1]` of the installed 0.8.0
/// theme, which the connector writes into `interact_size.y`, the least height egui's `Button`
/// gives a row (`egui/src/widgets/button.rs:306-309`) and its `CollapsingHeader` a header
/// (`egui/src/containers/collapsing_header.rs:531-532`), grown above a text scale of 1 to at
/// least the row's line box in `menu.item_label(NONE)` or `expander.header_label(NONE)`
/// ([`next_line_box`]), rounded up, and its padding (`button_padding` above and below)
/// ([`next_grown`]), as gpui's menu rows grow (`geometry::menu_item`, `with_height_rule`) and
/// the iced showcase's (`rows::scaled_height`); an unstated height is egui's own and is not
/// grown: the one rule every stated row height takes (M9-150; for the
/// expander the controller's ruling of 2026-10-23, M9c T6b). At a factor of 1 or less the stated
/// height stands, and the connector fits the cell's vertical padding to it
/// (`style::roles::fit_row_padding`), so the row is drawn that tall. The companion crate's
/// `Expander` opens the scope again and follows the rule itself.
fn grow_rows(ui: &mut egui::Ui, role: Role) {
    use native_theme::next::WidgetState;
    // the stated height as the connector wrote it, sanitised
    let stated = ui.spacing().interact_size.y;
    let padding = 2.0 * ui.spacing().button_padding.y;
    let grown = next_value(ui, |r| {
        let (height, label) = match role {
            Role::Menu => (
                r.menu.item(WidgetState::NONE).min_size[1],
                r.menu.item_label(WidgetState::NONE),
            ),
            Role::Expander => (
                r.expander.header(WidgetState::NONE).min_size[1],
                r.expander.header_label(WidgetState::NONE),
            ),
            _ => return None,
        };
        height.map(|_| {
            let line = next_line_box(r, &label, label.size_px);
            next_grown(r, stated, line, padding, 0.0)
        })
    })
    .flatten();
    if let Some(grown) = grown {
        ui.spacing_mut().interact_size.y = grown;
    }
}

/// A widget of the companion crate `native-theme-egui-widgets`, which opens `role`'s scope in
/// `variant` itself: the closure adds it, and its `Response` is recorded with that role seam.
pub(crate) fn widget(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let response = add(ui);
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
    let mut chrome = Chrome {
        frame: ui.native_frame(surface),
        title_frame: title.then(|| ui.native_frame(Surface::WindowTitleBar)),
        modifier: role.and_then(|(r, v)| role_modifier(ui, r, v)),
    };
    // the installed 0.8.0 theme's `dialog.panel.padding`, gpui-component's on each side it
    // leaves unstated
    let unstated = (surface == Surface::Dialog)
        .then(|| next_value(ui, |r| native_theme_egui::next::padding_of(&r.dialog.panel)))
        .flatten()
        .and_then(|padding| {
            chrome.frame.inner_margin =
                pad_unstated(chrome.frame.inner_margin, padding, [DIALOG_PADDING_PX; 4]);
            unstated_sides(padding)
        });
    let response = show(ui, chrome, reg)?;
    let mut seams = vec![Seam::Surface(surface)];
    if title {
        seams.push(Seam::Surface(Surface::WindowTitleBar));
    }
    seams.extend(role.map(|(r, v)| Seam::Role(r, v)));
    let mut info = info(kind, seams);
    if let Some(sides) = unstated {
        info.not_themeable.push((
            "inner margin",
            format!(
                "not stated by the theme on {sides}: gpui-component's {DIALOG_PADDING_PX}px \
                 (DIALOG_PADDING_PX), inside the border line, as the gpui showcase's Dialog \
                 keeps it"
            ),
        ));
    }
    reg.record(&response, info, true);
    Some(response)
}

/// The context menu `target` opens on a secondary click, `Popup::context_menu`, whose style
/// arrives as a `StyleModifier` alone (§4.2): given `menu`'s modifier, its open `Ui` styled in
/// `menu` and recorded as `kind` ([`styled`]); `add` fills it, adding its items through the
/// registry it gets (a disabled item through `scoped` in `RoleVariant::Disabled`). The target is
/// the caller's, drawn and recorded in the style it is drawn in.
pub(crate) fn context_menu(
    reg: &mut Registry,
    ui: &egui::Ui,
    target: &egui::Response,
    menu: (Role, RoleVariant),
    kind: &'static str,
    add: impl FnOnce(&mut egui::Ui, &mut Registry),
) {
    let (role, variant) = menu;
    let mut popup = egui::Popup::context_menu(target);
    if let Some(modifier) = role_modifier(ui, role, variant) {
        popup = popup.style(modifier);
    }
    popup.show(|ui| {
        styled(reg, ui, role, variant, kind);
        add(ui, reg);
    });
}

/// A `MenuBar` given `menu_bar_modifier` for `MenuBar::style`, the `Role::Menu` cell with the
/// theme's menu-bar title fields, and `role`'s modifier for `MenuConfig::style` (§4.2): the
/// closure builds the bar with the modifiers and records the menu buttons egui builds in it
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
    if let Some(modifier) = menu_bar_modifier(ui) {
        // The bar's titles edge to edge, each as tall as its text and its vertical padding:
        // `menu.bar_padding_vertical` where the theme states it (kde-breeze: 18 + 2 × 6 = 30,
        // as Breeze's bar measures), else the menu cell's, `menu.border.padding`, which the
        // connector narrows where a stated `menu.row_height` is shorter than a label and that
        // padding at a text scale of 1 or less (`roles::fit_row_padding`; none under
        // macos-monterey). The theme states no gap between two titles and no bar height, and a
        // title is no button `interact_size.y` would make taller.
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
/// adds to that body without a scope of their own. An open menu's rows take their least height
/// grown above a text scale of 1 ([`grow_rows`]).
pub(crate) fn styled(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    role: Role,
    variant: RoleVariant,
    kind: &'static str,
) -> Applied {
    let seam = Seam::Role(role, variant);
    ui.native_set_style(role, variant);
    grow_rows(ui, role);
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

    /// The frame's inner margin, taken off the frame and handed to the caller, who pads the
    /// content below a row that sits flush at the panel's top with it: the page tabs, as the
    /// gpui showcase's sit above the page's padding (`showcase-gpui/app.rs:2211`).
    pub(crate) fn lift_margin(&mut self) -> egui::Margin {
        std::mem::take(&mut self.frame.inner_margin)
    }

    /// The panel with no inner margin of its own, its content padded by the caller by
    /// `layout.container_margin` where the theme states one, as the gpui showcase pads its
    /// side panel's settings and inspector
    /// (`connectors/native-theme-gpui/examples/showcase-gpui/demo.rs:1735-1746`), so a separator
    /// and a tab bar's rule inside run from edge to edge.
    pub(crate) fn unpadded(mut self) -> Self {
        self.frame.inner_margin = egui::Margin::ZERO;
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
        reg.record(response, info(kind, seams), true);
    }
}

/// An icon of `role` from the chosen set and theme at `size` points (§10.4's icon rule, §9.2),
/// in the text colour: `icons::custom_icon_to_image` with the role for its provider, a
/// freedesktop icon read from `icon_theme` — the chosen theme; the OS's own only when the
/// choice is `system`, which passes `None` — and every system set's at `size`. `None` where
/// the set or theme lacks it: the caller shows it as absent, never from another set.
pub(crate) fn role_image(
    ui: &egui::Ui,
    role: IconRole,
    set: IconSet,
    icon_theme: Option<&str>,
    size: f32,
) -> Option<egui::Image<'static>> {
    let key = chrome_key(icons::IconKey::role(role, set), ui, icon_theme, size);
    icons::custom_icon_to_image(ui.ctx(), &role, &key)
        .map(|image| image.fit_to_exact_size(egui::Vec2::splat(size)))
}

/// `key` at `size`, from `icon_theme` where one is chosen, tinted the `Ui`'s text colour: a
/// freedesktop icon, an SF Symbol and a Segoe glyph are loaded in it, a bundled one coloured
/// (`IconKey::tint`).
fn chrome_key(
    key: icons::IconKey,
    ui: &egui::Ui,
    icon_theme: Option<&str>,
    size: f32,
) -> icons::IconKey {
    let key = key.size(size).tint(ui.visuals().text_color());
    match icon_theme {
        Some(name) => key.icon_theme(name),
        None => key,
    }
}

/// A chrome icon the gpui showcase draws by a gpui-component `IconName` that no `IconRole` stands
/// for (`showcase-gpui/support.rs:515-526`, `role_for_gpui_icon`), loaded by the names
/// native-theme-gpui gives it in each set (parity rule R5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChromeIcon {
    SquareTerminal,
    RotateCw,
    PanelLeft,
}

impl ChromeIcon {
    /// The icon's alt text, the same in every set and theme: the gpui-component `IconName` it
    /// stands for, as Lucide spells it (`connectors/native-theme-gpui/src/icons.rs:282`,
    /// `:296`, `:303`), a name that describes it rather than the file a set or theme holds it
    /// under (`icons::custom_icon_to_image_source`'s doc).
    fn alt_text(self) -> &'static str {
        match self {
            Self::SquareTerminal => "square-terminal",
            Self::RotateCw => "rotate-cw",
            Self::PanelLeft => "panel-left",
        }
    }

    /// The icon's names in `set` (`connectors/native-theme-gpui/src/icons.rs`: Lucide `:282`,
    /// `:296`, `:303`; Material `:426`, `:440`, `:447`; freedesktop
    /// `freedesktop_name_candidates`, `:597`, `:603`, `:644`, where Breeze's
    /// `sidebar-expand-left` and Adwaita's `sidebar-show` name one icon, so the icon theme has
    /// whichever its convention names); empty in a set that has none — SF Symbols and Segoe
    /// Fluent here — where the button shows its label instead.
    fn names(self, set: IconSet) -> &'static [&'static str] {
        match (self, set) {
            (Self::SquareTerminal, IconSet::Lucide) => &["square-terminal"],
            (Self::SquareTerminal, IconSet::Material) => &["terminal"],
            (Self::SquareTerminal, IconSet::Freedesktop) => &["utilities-terminal"],
            (Self::RotateCw, IconSet::Lucide) => &["rotate-cw"],
            (Self::RotateCw, IconSet::Material) => &["rotate_right"],
            (Self::RotateCw, IconSet::Freedesktop) => &["object-rotate-right"],
            (Self::PanelLeft, IconSet::Lucide) => &["panel-left"],
            (Self::PanelLeft, IconSet::Material) => &["side_navigation"],
            (Self::PanelLeft, IconSet::Freedesktop) => &["sidebar-expand-left", "sidebar-show"],
            _ => &[],
        }
    }
}

/// A chrome icon is a provider of its first name in each set, for `icons::custom_icon_to_image`;
/// it has no SVG of its own.
impl IconProvider for ChromeIcon {
    fn icon_name(&self, set: IconSet) -> Option<&str> {
        self.names(set).first().copied()
    }

    fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
        None
    }
}

/// `icon` from the chosen set and theme at `size` points, loaded and coloured as `role_image`
/// loads a role's; a freedesktop icon under the first of its names the icon theme has, each
/// looked up in that one theme (`FreedesktopLoader::any`, `native-theme/src/icons.rs:259`), as
/// the gpui showcase loads it, read from disk once per key and kept in `cache`. Its alt text is
/// `ChromeIcon::alt_text`, whichever name loaded. `None` where the set or theme lacks it, never
/// another set's icon.
pub(crate) fn named_image(
    cache: &mut IconCache,
    ui: &egui::Ui,
    icon: ChromeIcon,
    set: IconSet,
    icon_theme: Option<&str>,
    size: f32,
) -> Option<egui::Image<'static>> {
    let key = chrome_key(
        icons::IconKey::name(icon.alt_text(), set),
        ui,
        icon_theme,
        size,
    );
    if set != IconSet::Freedesktop {
        return icons::custom_icon_to_image(ui.ctx(), &icon, &key)
            .map(|image| image.fit_to_exact_size(egui::Vec2::splat(size)));
    }
    let IconCache {
        named,
        #[cfg(test)]
        loads,
        ..
    } = cache;
    named
        .entry(key)
        .or_insert_with_key(|key| {
            #[cfg(test)]
            loads.push(IconLoad::Named(key.clone()));
            let [r, g, b, _] = ui.visuals().text_color().to_srgba_unmultiplied();
            // The saturating float-to-int cast `IconKey::size` makes (§4.10, §7.1).
            let loader = FreedesktopLoader::any(icon.names(set))
                .size(size.round() as u32)
                .color(Rgba::rgb(r, g, b));
            let data = match icon_theme {
                Some(theme) => loader.theme(theme),
                None => loader,
            }
            .load()?;
            icons::to_image(ui.ctx(), key, &data)
        })
        .clone()
        // Fitted after the clone: two sizes the key rounds to one each take their own.
        .map(|image| image.fit_to_exact_size(egui::Vec2::splat(size)))
}

/// The icons the showcase loads itself, kept from the pass that first draws each until the
/// next install (`App::adopt_pair`: `ThemeAtlas::install` forgets the connector's icon URIs,
/// `forget_icons`, which a kept image names) or the next change of the icon choice: the
/// Icons page's loading indicator and the images of its frames, and `named_image`'s
/// freedesktop icons. Each map has its own key, so another choice or scheme misses and loads
/// its own:
/// - the indicator, by the set and the icon theme;
/// - a frame's image, by the indicator's `IconKey` (its name, the set and the icon theme, and
///   for a set other than freedesktop the tint; no size, as the page fits each draw), the
///   scheme (which colours a freedesktop frame) and the frame's index (`pages::icons`);
/// - a named icon, by the key `chrome_key` makes: its name, the set, the icon theme, the size,
///   rounded, and the tint; the image is kept as loaded, and each draw fits its clone to the
///   size it asks for.
///
/// An icon the theme lacks is kept as absent too, so it is looked up once.
/// The widgets crate's `Spinner` keeps its indicator for the same reason, "a spinner draws a
/// frame in every pass" (`native-theme-egui-widgets/src/spinner.rs`).
///
/// The connector asks for its images to be built in each pass, not kept
/// (`icons::to_image_source`), since `forget_icons` frees what they name, a raster's texture
/// among them. Keeping them holds because the two calls that run it are covered: every install
/// goes through `App::adopt_pair`, which clears this beside it, and the showcase calls no
/// `ThemeAtlas::clear`; a call would need `IconCache::clear` beside it.
#[derive(Default)]
pub(crate) struct IconCache {
    indicators: std::collections::HashMap<(IconSet, Option<String>), Option<AnimatedIcon>>,
    frames: std::collections::HashMap<
        (icons::IconKey, egui::Theme, usize),
        Option<egui::Image<'static>>,
    >,
    named: std::collections::HashMap<icons::IconKey, Option<egui::Image<'static>>>,
    /// Every load and frame build, in order, for the tests to count; never cleared.
    #[cfg(test)]
    pub(crate) loads: Vec<IconLoad>,
}

/// One load or build of [`IconCache`], as the tests count them.
#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum IconLoad {
    /// The loading indicator of a set and icon theme, loaded.
    Indicator(IconSet, Option<String>),
    /// The image of an indicator frame built: the indicator's key, the scheme, the frame.
    Frame(icons::IconKey, egui::Theme, usize),
    /// A `named_image` freedesktop icon loaded.
    Named(icons::IconKey),
}

impl IconCache {
    /// Forget every icon kept; the next pass loads what it draws again.
    pub(crate) fn clear(&mut self) {
        self.indicators.clear();
        self.frames.clear();
        self.named.clear();
    }

    /// The loading indicator `set` ships — from `icon_theme` for a freedesktop set, the
    /// system's where it is `None` — and the image of the frame it shows in this pass
    /// (`icons::animated_frame_index`, the first frame for a spin and under reduced motion),
    /// built with `key`, in this pass's scheme; `None` where the set ships no indicator.
    pub(crate) fn indicator(
        &mut self,
        ctx: &egui::Context,
        key: &icons::IconKey,
        (set, icon_theme): (IconSet, Option<&str>),
        reduce_motion: bool,
    ) -> Option<(&AnimatedIcon, Option<egui::Image<'static>>)> {
        let Self {
            indicators,
            frames,
            #[cfg(test)]
            loads,
            ..
        } = self;
        let anim = indicators
            .entry((set, icon_theme.map(str::to_owned)))
            .or_insert_with(|| {
                #[cfg(test)]
                loads.push(IconLoad::Indicator(set, icon_theme.map(str::to_owned)));
                match set {
                    IconSet::Freedesktop => FreedesktopLoader::load_indicator(icon_theme),
                    set => native_theme::icons::load_icon_indicator(set, None),
                }
            })
            .as_ref()?;
        let (index, frame) = icons::animated_frame_index(ctx, anim, reduce_motion)
            .and_then(|i| Some((i, anim.frame_list()?.get(i)?)))
            .unwrap_or((0, anim.first_frame()));
        let scheme = ctx.theme();
        let image = frames
            .entry((key.clone(), scheme, index))
            .or_insert_with(|| {
                #[cfg(test)]
                loads.push(IconLoad::Frame(key.clone(), scheme, index));
                icons::to_image(ctx, key, frame)
            })
            .clone();
        Some((anim, image))
    }
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

/// The atlas's `Note::FontFamilyUnavailable` notes as `(family, substitute)` pairs: each family
/// the theme asked for that the connector's font plan found no face of
/// (`fonts::FontPlan::from_system`), and the family drawn in its place, `None` where egui's
/// own face stays (the note's doc, `connectors/native-theme-egui/src/atlas.rs`).
pub(crate) fn unavailable_families(notes: &[Note]) -> Vec<(&str, Option<&str>)> {
    notes
        .iter()
        .filter_map(|note| match note {
            Note::FontFamilyUnavailable {
                family, substitute, ..
            } => Some((&**family, substitute.as_deref())),
            _ => None,
        })
        .collect()
}

/// The family drawn in place of the theme's `stated` family: the substitute the pair of
/// `unavailable` ([`unavailable_families`]) for `stated` names; `None` where no pair names
/// `stated`, so its own face is drawn, or where its pair names no substitute.
pub(crate) fn drawn_family<'a>(
    unavailable: &[(&str, Option<&'a str>)],
    stated: &str,
) -> Option<&'a str> {
    unavailable
        .iter()
        .find(|(family, _)| *family == stated)
        .and_then(|(_, substitute)| *substitute)
}

/// The families the atlas's `Note::FontDataInvalid` notes name: each the name a face of the font
/// plan was registered under, which failed epaint's parse and was dropped, so egui's own face
/// stays in its place (the note's doc, `connectors/native-theme-egui/src/atlas.rs`). A face of
/// the platform's substitute is registered under the theme's family (connector `fonts.rs`,
/// `FontPlan::from_system`), so the note names the theme's family for it too.
pub(crate) fn invalid_families(notes: &[Note]) -> Vec<&str> {
    notes
        .iter()
        .filter_map(|note| match note {
            Note::FontDataInvalid { family, .. } => Some(&**family),
            _ => None,
        })
        .collect()
}

/// How egui draws text of `planned`, a family the font plan loads, by the atlas's notes on it
/// (`unavailable`, [`unavailable_families`]; `invalid`, [`invalid_families`]) and `generic`,
/// egui's generic family it is drawn in (`proportional`, `monospace`) with egui's own first face
/// of it: as stated where no note names it; in the platform's substitute its
/// `FontFamilyUnavailable` note names; and in egui's own face where that note names none (the
/// OS has no face of it, `native_theme::fonts::system_face`'s `None`), where the substitute's
/// face failed epaint's parse, or, as "in no face egui can draw", where the family's own face
/// did.
pub(crate) fn planned_drawing<'a>(
    unavailable: &[(&str, Option<&'a str>)],
    invalid: &[&str],
    planned: &str,
    (kind, own): (&'a str, Option<&'a str>),
) -> DrawnFamily<'a> {
    let generic = |held_weights| DrawnFamily::Generic {
        toolkit: "egui",
        kind,
        family: own,
        held_weights,
    };
    let noted = unavailable.iter().any(|(family, _)| *family == planned);
    let dropped = invalid.contains(&planned);
    match drawn_family(unavailable, planned) {
        Some(substitute) if !dropped => DrawnFamily::Named(substitute),
        _ if noted => generic(None),
        _ if dropped => generic(Some(&[])),
        _ => DrawnFamily::Stated,
    }
}

/// What egui draws the theme's families in, as the status bar, the Theme tab and Widget Info
/// name them (the shared crate's `status::DrawnFamily`): the two families the connector's font
/// plan loads, the installed 0.8.0 light theme's `typography.body.family` and
/// `typography.monospace.family` (`next::font_plan_from_system(light)`), the atlas's notes on them ([`unavailable_families`],
/// [`invalid_families`]), and
/// egui's own first face of its proportional and monospace families in the fonts the pass draws
/// with, which stays where the plan registers no face. egui draws every font in one of those
/// two faces: the monospace font in the monospace one, every other font in the proportional one
/// (the connector's never-`Name` invariant, `docs/archive/todo_v0.6.0_egui-connector-spec.md`
/// §8.1, §5.8 item 7), whatever family the font states.
pub(crate) struct DrawnFamilies<'a> {
    unavailable: Vec<(&'a str, Option<&'a str>)>,
    invalid: Vec<&'a str>,
    proportional: Option<String>,
    monospace: Option<String>,
    /// The light theme's `typography.body.family` and its label ([`Self::drawn`]).
    body: (String, String),
    /// The light theme's `typography.monospace.family` and its label.
    mono: (String, String),
}

impl<'a> DrawnFamilies<'a> {
    /// The families `atlas` draws in, the fonts read from `ctx`, which has run a pass
    /// (`Context::fonts` panics before the first, `egui/src/context.rs:1097-1106`); `None`
    /// where the atlas holds no 0.8.0 light theme, whose families the font plan loads.
    pub(crate) fn of(ctx: &egui::Context, atlas: &'a crate::app::OldAtlas) -> Option<Self> {
        let first = |family: egui::FontFamily| {
            ctx.fonts(|f| {
                f.definitions()
                    .families
                    .get(&family)
                    .and_then(|faces| faces.first().cloned())
            })
        };
        // The families the plan loads: the installed 0.8.0 light theme's
        // (`next::font_plan_from_system(light)`, `app::next_atlas`).
        let light = atlas.next_resolved(egui::Theme::Light)?;
        let (body, mono) = (
            light.typography.body.family.to_string(),
            light.typography.monospace.family.to_string(),
        );
        let mut families = Self {
            unavailable: unavailable_families(atlas.notes()),
            invalid: invalid_families(atlas.notes()),
            proportional: first(egui::FontFamily::Proportional),
            monospace: first(egui::FontFamily::Monospace),
            body: (body, String::new()),
            mono: (mono, String::new()),
        };
        families.body.1 = families
            .drawn(&families.body.0, false)
            .label(&families.body.0);
        families.mono.1 = families
            .drawn(&families.mono.0, true)
            .label(&families.mono.0);
        Some(families)
    }

    /// How egui draws text of `planned`, a family the font plan loads, in its `Monospace` family
    /// where `mono`, else its `Proportional` one: by the atlas's notes ([`planned_drawing`]);
    /// and in egui's own face wherever the build loads no system font (feature `system-fonts`
    /// off, so the atlas has no font plan and leaves egui's faces in place, `Builder::fonts`).
    fn drawn(&self, planned: &str, mono: bool) -> DrawnFamily<'_> {
        let (kind, own) = if mono {
            ("monospace", self.monospace.as_deref())
        } else {
            ("proportional", self.proportional.as_deref())
        };
        if !cfg!(feature = "system-fonts") {
            return DrawnFamily::Unloaded {
                toolkit: "egui",
                kind,
                family: own,
            };
        }
        planned_drawing(&self.unavailable, &self.invalid, planned, (kind, own))
    }

    /// How egui draws text of a theme font whose family is `stated`, the monospace font where
    /// `mono`: as [`Self::drawn`] of the family the plan loads for its kind where `stated` is
    /// that family (compared caselessly, as the plan's lookup compares, connector `fonts.rs`);
    /// in that family's face otherwise, which the label names with its own label.
    pub(crate) fn drawn_at(&self, stated: &str, mono: bool) -> DrawnFamily<'_> {
        let ((planned, label), font) = if mono {
            (&self.mono, "the light variant's typography.monospace")
        } else {
            (&self.body, "the light variant's typography.body")
        };
        if stated.eq_ignore_ascii_case(planned) {
            self.drawn(planned, mono)
        } else {
            DrawnFamily::OtherFont {
                toolkit: "egui",
                font,
                family: label,
            }
        }
    }

    /// A theme font's `stated` family as a label that says what egui draws
    /// ([`Self::drawn_at`]), the monospace font where `mono`.
    pub(crate) fn label(&self, stated: &str, mono: bool) -> String {
        self.drawn_at(stated, mono).label(stated)
    }

    /// The family of `font`, the text style at the home `path`, as Widget Info's rows show it
    /// (`next::values::home_value`'s lookup): [`Self::label`], the monospace font's for
    /// `typography.monospace`, the one text style the connector draws in egui's monospace family
    /// (`next::font_plan_from_system`, the connector's `fonts.rs` module doc).
    pub(crate) fn home_label(&self, path: &str, font: &StatedFont) -> String {
        self.label(&font.family, path == "typography.monospace")
    }
}

/// The OS's faces of the theme's family at the weights the showcase draws besides the body's:
/// its semibold face, for the inspector's headings gpui draws `font_semibold()` (parity decision
/// 3), and the face at each weight of each installed 0.8.0 theme's text roles
/// (`next::text_role_weight`), table header (`list.header_label(NONE).weight`) and tab label
/// (`tab.label(NONE).weight`), for the page's section headings, its
/// Typography group, its table and its tabs. Each is `native_theme::fonts::system_face` at its weight, added with
/// `Context::add_font` after every `ThemeAtlas::install`, whose `set_fonts` replaces the
/// definitions: the semibold one under `SEMIBOLD_FAMILY`, the others under [`weight_family`].
/// `add_font` skips a name the loaded fonts already hold (`egui/src/context.rs:2133-2143`), and
/// they still hold the previous install's face when the next install is pending, so each
/// registration takes a name of its own.
#[derive(Default)]
pub(crate) struct Semibold {
    registrations: u64,
    /// The faces of the last registration that failed epaint's parse and were not added: the
    /// family and the weight asked for ([`Semibold::add_face`]).
    invalid: Vec<(String, u16)>,
}

impl Semibold {
    /// After `atlas.install(ctx)`: the faces of the family the connector's font plan registers
    /// for the installed 0.8.0 light theme's body text (§4.6) — the family it names, or the
    /// platform's substitute
    /// where the OS lacks it, as the atlas's notes record ([`drawn_family`]) — where the OS has
    /// them, each where epaint can load it ([`Semibold::add_face`]); none otherwise, and the
    /// text keeps the regular face. One `native_theme::fonts::system_face` per weight
    /// registered.
    pub(crate) fn register(&mut self, ctx: &egui::Context, atlas: &crate::app::OldAtlas) {
        #[cfg(feature = "system-fonts")]
        self.register_with(ctx, atlas, |family, weight, style| {
            native_theme::fonts::system_face(family, weight, style)
                .map(|face| (face.data.to_vec(), face.index))
        });
        #[cfg(not(feature = "system-fonts"))]
        let _ = (ctx, atlas, &mut self.registrations);
    }

    /// [`Semibold::register`] with `face` in place of `native_theme::fonts::system_face`: the
    /// bytes and the face index of the font file of a family at a weight and a slant, `None`
    /// where there is none.
    #[cfg(feature = "system-fonts")]
    pub(crate) fn register_with(
        &mut self,
        ctx: &egui::Context,
        atlas: &crate::app::OldAtlas,
        face: impl Fn(&str, u16, native_theme::theme::FontStyle) -> Option<(Vec<u8>, u32)>,
    ) {
        use native_theme::next::{FontStyle, TextStated, WidgetState};
        use native_theme::theme::FontStyle as Slant;
        use native_theme_egui::{TextRole, next};

        self.registrations = self.registrations.saturating_add(1);
        self.invalid.clear();
        // The faces the connector's font plan asks for (`next::font_plan_from_system(light)`):
        // the light theme's body text, where it states its family and slant, as the
        // connector's face request reads them, each slant the one `system_face` takes.
        let Some(light) = atlas.next_resolved(egui::Theme::Light) else {
            return;
        };
        let body = &light.typography.body;
        let style = match body.style {
            FontStyle::Normal => Slant::Normal,
            FontStyle::Italic => Slant::Italic,
            FontStyle::Oblique => Slant::Oblique,
            _ => return,
        };
        if !body.stated.contains(TextStated::FAMILY | TextStated::STYLE) {
            return;
        }
        let mut families = vec![(
            crate::SEMIBOLD_WEIGHT,
            egui::FontFamily::Name(SEMIBOLD_FAMILY.into()),
        )];
        // The weights the pages draw text at besides the body's (`weighted_family`): each
        // text role's (`next::text_role_weight`, the section headings and the Typography
        // group), the table header's where stated (`pages::data::TableHeader`) and the tab
        // label's where stated (`next_tab_label`), of each
        // scheme's installed theme whose body weight is stated (`next::font_weight`).
        for scheme in [egui::Theme::Light, egui::Theme::Dark] {
            let Some(t) = atlas.next_resolved(scheme) else {
                continue;
            };
            let Ok(body_weight) = next::font_weight(t) else {
                continue;
            };
            let header = t.list.header_label(WidgetState::NONE);
            let roles = [
                TextRole::Caption,
                TextRole::SectionHeading,
                TextRole::DialogTitle,
                TextRole::Display,
            ]
            .map(|role| next::text_role_weight(t, role));
            let header = header
                .stated
                .contains(TextStated::WEIGHT)
                .then_some(header.weight);
            let tab = next::role_font_weight(t, native_theme_egui::Role::Tab).ok();
            for weight in roles.into_iter().chain(header).chain(tab) {
                let family = weight_family(weight);
                if weight != body_weight && !families.iter().any(|(_, f)| *f == family) {
                    families.push((weight, family));
                }
            }
        }
        // The family the connector's font plan draws the body in, as `FontPlan::from_system`
        // recorded it: the platform's substitute its `FontFamilyUnavailable` note names where
        // the OS lacks the theme's family, else the theme's own.
        let unavailable = unavailable_families(atlas.notes());
        let drawn = drawn_family(&unavailable, &body.family).unwrap_or(&body.family);
        for (weight, family) in families {
            let Some((bytes, index)) = face(drawn, weight, style) else {
                continue;
            };
            self.add_face(ctx, (drawn, weight), family, bytes, index);
        }
    }

    /// Add `bytes`, the face at `index` of a font file of `drawn.0` at the weight `drawn.1`,
    /// under `family`, at the head of its chain, where epaint can load it ([`epaint_loads`]);
    /// otherwise add nothing and record it in [`Semibold::invalid`] once, however many families
    /// ask for it, as the connector drops a face of its plan that fails the same check and notes
    /// it once (`Note::FontDataInvalid`), so `family` stays unbound and text at that weight is
    /// drawn in the body's face ([`weighted_family`], [`semibold_font`]).
    #[cfg(feature = "system-fonts")]
    pub(crate) fn add_face(
        &mut self,
        ctx: &egui::Context,
        (drawn, weight): (&str, u16),
        family: egui::FontFamily,
        bytes: Vec<u8>,
        index: u32,
    ) {
        if !epaint_loads(&bytes, index) {
            if !self.invalid.iter().any(|(f, w)| f == drawn && *w == weight) {
                self.invalid.push((drawn.to_string(), weight));
            }
            return;
        }
        let mut data = egui::FontData::from_owned(bytes);
        data.index = index;
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

    /// The faces of the last registration that failed epaint's parse and were not added, each
    /// the family and the weight asked for.
    pub(crate) fn invalid(&self) -> &[(String, u16)] {
        &self.invalid
    }
}

/// Whether epaint can load the face at `index` of `bytes`: the parse epaint's `FontFace::new`
/// makes, `skrifa::FontRef::from_index` (`epaint/src/text/font.rs:386-388`), whose failure
/// epaint turns into a panic at the next pass, in release builds too
/// (`epaint/src/text/fonts.rs:990`), and a non-zero `unitsPerEm`, which epaint divides by
/// (`epaint/src/text/font.rs:215-218`). The connector makes the same check before it hands
/// epaint a face of its font plan (`connectors/native-theme-egui/src/fonts.rs`, `parses`), and
/// exports none.
#[cfg(feature = "system-fonts")]
fn epaint_loads(bytes: &[u8], index: u32) -> bool {
    use skrifa::MetadataProvider as _;
    skrifa::FontRef::from_index(bytes, index).is_ok_and(|font| {
        font.metrics(
            skrifa::instance::Size::unscaled(),
            skrifa::instance::LocationRef::default(),
        )
        .units_per_em
            != 0
    })
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

/// A page's or the inspector's section heading, in the installed 0.8.0 theme's section-heading
/// role (`typography.headline`, the section divider of `docs/platform-facts.md` §2.19): its
/// size and line height through `next::text_role_font` and `next::text_role_line_height`, its
/// weight through [`weighted_family`], in the text colour ([`next_role_text`]) — as the iced
/// showcase's section titles are. Without an installed 0.8.0 theme, egui's `Body` size,
/// semibold.
pub(crate) fn heading_text(ui: &egui::Ui, text: impl Into<String>) -> egui::RichText {
    if next_value(ui, |_| ()).is_none() {
        let size = egui::TextStyle::Body.resolve(ui.style()).size;
        return egui::RichText::new(text)
            .font(semibold_font(ui, size))
            .color(ui.visuals().text_color());
    }
    next_role_text(ui, native_theme_egui::TextRole::SectionHeading, text)
}

/// `text` one line box tall: the size of the font `ui` sets a widget's text in (its
/// `override_font_id`, else `style`'s) times the installed 0.8.0 theme's line height
/// (`next::line_height`, the body text's), the platform's line box, per call. The connector
/// writes the line box only where it is taller than the font's own row
/// (`Spacing::extra_text_line_spacing`, which egui adds between rows, never above the first); a
/// `TextStyle` is a font and nothing more, so a line box shorter than the row — Adwaita's 1.21 of
/// a Cantarell-sized font — is reachable only per text, `RichText::line_height`
/// (`egui/src/widget_text.rs:174`). Without an installed 0.8.0 theme, the text as it is.
pub(crate) fn next_lined(
    ui: &egui::Ui,
    text: impl Into<String>,
    style: egui::TextStyle,
) -> egui::RichText {
    let text = egui::RichText::new(text);
    let size = ui
        .style()
        .override_font_id
        .as_ref()
        .map_or_else(|| style.resolve(ui.style()).size, |font| font.size);
    match next_value(ui, native_theme_egui::next::line_height) {
        Some(line_height) => text.line_height(Some(size * line_height)),
        None => text,
    }
}

/// A tab's label, one line box tall ([`next_lined`]), at the weight the installed 0.8.0 theme
/// states for it (`next::role_font_weight(Role::Tab)`, `tab.label(NONE).weight`) where it also
/// states the body's (`next::font_weight`): in the face [`Semibold`] registers at that weight
/// where the fonts hold one, else in the body's ([`weighted_family`]). Where the theme states
/// no tab weight, egui's own: the scope's proportional face, the body's.
pub(crate) fn next_tab_label(ui: &egui::Ui, text: &'static str) -> egui::RichText {
    use native_theme_egui::next;
    let lined = next_lined(ui, text, egui::TextStyle::Button);
    match next_value(ui, |r| {
        (next::role_font_weight(r, Role::Tab), next::font_weight(r))
    }) {
        Some((Ok(weight), Ok(body))) => lined.family(weighted_family(ui, weight, body)),
        _ => lined,
    }
}

/// The line box of text in the 0.8.0 text style `style` of the theme `r`, drawn at `drawn`
/// points: the style's own `line_height` where the theme states one, else `drawn` times the
/// theme's line height (`next::line_height`), as the gpui showcase's `demo::line_box`.
pub(crate) fn next_line_box(
    r: &native_theme::next::ResolvedTheme,
    style: &native_theme::next::TextStyle,
    drawn: f32,
) -> f32 {
    if style
        .stated
        .contains(native_theme::next::TextStated::LINE_HEIGHT)
    {
        style.line_height
    } else {
        drawn * native_theme_egui::next::line_height(r)
    }
}

/// A stated row height of the 0.8.0 theme `r` by the control-height rule, as the gpui
/// connector's rows take one (`geometry::row_height_fit`): verbatim at a text scale of 1 or
/// less; above 1 at least `line`, the row's line box at the theme's text size, which its resolve
/// has scaled already, rounded up, with `padding` (the row's top and bottom together) and
/// `rule`, the width of a line drawn inside the row.
pub(crate) fn next_grown(
    r: &native_theme::next::ResolvedTheme,
    stated: f32,
    line: f32,
    padding: f32,
    rule: f32,
) -> f32 {
    if r.context.text_scale <= 1.0 {
        return stated;
    }
    stated.max(line.ceil() + padding + rule)
}

/// The value `build` gives for the installed 0.8.0 theme the window shows
/// (`OldAtlas::next_shown`: the scheme egui draws, for an inactive window while it is not
/// focused), as the gpui showcase's `support::next_value`; `None` without an installed 0.8.0
/// theme.
pub(crate) fn next_value<T>(
    ui: &egui::Ui,
    build: impl FnOnce(&native_theme::next::ResolvedTheme) -> T,
) -> Option<T> {
    let atlas = crate::app::OldAtlas::from_ctx(ui.ctx())?;
    atlas.next_shown(ui.ctx()).map(build)
}

/// One of the four layout metrics of the installed 0.8.0 theme (`layout.widget_gap`,
/// `container_margin`, `window_margin` or `section_gap`), as `build` reads it from the theme's
/// layout: `None` where the theme states none, and without an installed 0.8.0 theme, as the
/// gpui showcase's `support::next_metric`.
pub(crate) fn next_metric(
    ui: &egui::Ui,
    build: fn(&native_theme::next::LayoutMetrics) -> Option<f32>,
) -> Option<f32> {
    next_value(ui, |r| build(&r.layout)).flatten()
}

/// Make the rest of `ui` a tool button's, the icon-only button of a toolbar, the status bar and
/// the tab row's page menu: Ghost ([`ghost`]), padded by `padding`, the sides of
/// `button.body(NONE).padding` ([`tool_padding`]: top, right, bottom, left) of the installed
/// 0.8.0 theme, and with no minimum height, so it is its icon and that padding. egui pads a
/// button by one pair (`Spacing::button_padding`), so the left side pads left and right and the
/// top side top and bottom; a side the theme leaves unstated keeps the scope's own. Its outline
/// under the pointer and while pressed is the scope's: in a «toolbar» scope the connector writes
/// `toolbar.button(HOVER)` and `toolbar.button(PRESSED)`'s stroke where a rule states it, and
/// leaves the scope's own hovered and pressed button's where none does (spec §10, rule 0).
pub(crate) fn next_tool_button(ui: &mut egui::Ui, [top, _, _, left]: [Option<f32>; 4]) {
    ghost(ui);
    let own = ui.spacing().button_padding;
    let spacing = ui.spacing_mut();
    spacing.button_padding = egui::vec2(left.unwrap_or(own.x), top.unwrap_or(own.y));
    spacing.interact_size.y = 0.0;
}

/// The height egui gives an icon-only tool button with a `size` tall icon in a `Role::Button`
/// scope over `ui` ([`next_tool_button`]), `top` its stated top padding, else the scope's own
/// `button_padding.y`: the icon and, above and below it, the padding as egui frames a button,
/// its inner margin whole points, the padding rounded less the resting border's width rounded
/// (`Margin::from`, `epaint-0.37.0/src/margin.rs:108-120`;
/// `Frame::apply_stroke_and_expansion_without_layout_shift`,
/// `egui-0.37.0/src/containers/frame.rs:313-323`), and the border's own width outside it
/// (`Frame::total_margin`). Without an installed atlas, the scope is `ui`'s own style.
pub(crate) fn tool_button_height(ui: &egui::Ui, size: f32, top: Option<f32>) -> f32 {
    let mut style = (**ui.style()).clone();
    if let Some(atlas) = crate::app::OldAtlas::from_ctx(ui.ctx()) {
        atlas
            .role_modifier(ui.ctx().theme(), Role::Button, RoleVariant::Normal)
            .apply(&mut style);
    }
    let padding = top.unwrap_or(style.spacing.button_padding.y);
    let border = style.visuals.widgets.inactive.bg_stroke.width;
    size + 2.0 * (padding.round() + (-border).round() + border)
}

/// A tool button's padding, `button.body(NONE).padding` of the installed 0.8.0 theme `r`, top,
/// right, bottom, left, each where stated (`next::padding_of`); none without a 0.8.0 theme.
pub(crate) fn tool_padding(r: Option<&native_theme::next::ResolvedTheme>) -> [Option<f32>; 4] {
    r.map_or([None; 4], |r| {
        native_theme_egui::next::padding_of(&r.button.body(native_theme::next::WidgetState::NONE))
    })
}

/// A toolbar button's icon size, `toolbar.button_icon(NONE).size`, where the installed 0.8.0
/// theme `r` states it; `None` where it does not, or none is installed: the button then shows
/// its label in place of its icon.
pub(crate) fn tool_icon_size(r: Option<&native_theme::next::ResolvedTheme>) -> Option<f32> {
    let icon = r?
        .toolbar
        .button_icon(native_theme::next::WidgetState::NONE);
    icon.stated
        .contains(native_theme::next::IconStated::SIZE)
        .then_some(icon.size)
}

/// `icons.sizes.small` of the installed 0.8.0 theme the window shows, the size of the chrome's
/// small icons (the status bar's toggle, the page menu's caret, a dialog's close button, the
/// error banner's icon); `None` where it states none, or none is installed: each then shows its
/// label in place of its icon, or the banner its text alone.
pub(crate) fn small_icon_size(ui: &egui::Ui) -> Option<f32> {
    next_value(ui, |r| r.icons.sizes.small).flatten()
}

/// Text in the installed 0.8.0 theme's text role `role`, its
/// size and line height through `next::text_role_font` and `next::text_role_line_height`, which
/// the theme's resolve has scaled already, its weight through [`weighted_family`], in the text
/// colour; without an installed 0.8.0 theme, egui's `Body` text.
pub(crate) fn next_role_text(
    ui: &egui::Ui,
    role: native_theme_egui::TextRole,
    text: impl Into<String>,
) -> egui::RichText {
    use native_theme_egui::next;
    let colour = ui.visuals().text_color();
    let text = egui::RichText::new(text).color(colour);
    let Some((size, line_height, weight, body_weight)) = next_value(ui, |r| {
        (
            next::text_role_font(r, role).size,
            next::text_role_line_height(r, role),
            next::text_role_weight(r, role),
            next::font_weight(r),
        )
    }) else {
        return text;
    };
    let family = match body_weight {
        Ok(body_weight) => weighted_family(ui, weight, body_weight),
        Err(_) => egui::FontFamily::Proportional,
    };
    text.font(egui::FontId::new(size, family))
        .line_height(Some(line_height))
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
    /// as wide as its tabs (the Basic page's), which then wrap onto a further line where the
    /// row is too narrow for them, as the page's other rows do.
    pub full_width: bool,
    /// The row's element in `docs/showcase-elements.toml`, and its tabs', in order: a tab past
    /// the end of `tab_elements` is not one of the list's.
    pub element: &'static str,
    pub tab_elements: &'a [&'static str],
}

/// The selected tab's line: its colour, its thickness, and the edge it lies along, the tab's
/// top one or else its bottom one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TabLine {
    pub colour: egui::Color32,
    pub width: f32,
    pub top: bool,
}

/// What a row of tabs ([`tab_bar`]) draws beyond what its `Role::Tab` scope carries, read from
/// the installed 0.8.0 theme ([`TabLook::next`]): the page and inspector tab rows' and the Basic
/// page's.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TabLook {
    /// The least tab width, which egui's `Button` never reads from the style.
    pub min_width: f32,
    /// What the row's Widget Info names for the gap where the theme states none.
    pub gap_unstated: Option<&'static str>,
    /// The selected tab's line, where the theme states one ([`tab_indicator`]).
    pub line: Option<TabLine>,
    /// The radius the selected tab's bottom corners curve out by into the bar, where the theme
    /// states a positive, finite one ([`tab_flares`]).
    pub flare: Option<f32>,
    /// A tab's label, one line box tall, at the tab's weight ([`next_tab_label`]).
    pub label: fn(&egui::Ui, &'static str) -> egui::RichText,
}

impl TabLook {
    /// The look the installed 0.8.0 theme `r` states, each unstated part egui's own (spec §10
    /// rule 0): at least `tab.tab(NONE).min_size[0]` wide, else no least width, as egui's
    /// `Button` has none (egui-0.37.0 `src/widgets/button.rs:67`); the tabs the scope's
    /// `item_spacing.x` apart, `tab.item_gap` where stated, else the layout's own; the line of
    /// [`next_indicator_line`]; the flares of [`next_outward_radius`], which [`tab_bar`] paints in
    /// the scope's selected tab's fill and outline, the ones the connector writes from
    /// `tab.tab(SELECTED)`; the labels [`next_tab_label`]. Without a 0.8.0 theme, egui's own tabs.
    pub(crate) fn next(r: Option<&native_theme::next::ResolvedTheme>) -> Self {
        Self {
            min_width: r
                .and_then(|r| r.tab.tab(native_theme::next::WidgetState::NONE).min_size[0])
                .unwrap_or_default(),
            gap_unstated: match r {
                Some(r) if r.tab.item_gap.is_some() => None,
                // the connector's base style spaces a row's widgets by the layout's gap
                Some(r) if r.layout.widget_gap.is_some() => Some(
                    "not stated by the theme (tab.item_gap): the gap between the widgets of a \
                     row (item_spacing.x), layout.widget_gap",
                ),
                _ => Some(
                    "not stated by the theme (tab.item_gap): egui's own, the gap between the \
                     widgets of a row (item_spacing.x)",
                ),
            },
            line: r.and_then(next_indicator_line),
            flare: r.and_then(next_outward_radius),
            label: next_tab_label,
        }
    }
}

/// The selected tab's line on the 0.8.0 theme `r`: the selected tab's indicator,
/// `tab.indicator(SELECTED)`, its `fill` the line's colour and its exact height (`min_size[1]`)
/// its thickness, a positive one, along `tab.indicator_side` (spec §5, notes to the table), where
/// the theme states all three; else no line, as egui's `Button` draws none.
pub(crate) fn next_indicator_line(r: &native_theme::next::ResolvedTheme) -> Option<TabLine> {
    let indicator = r.tab.indicator(native_theme::next::WidgetState::SELECTED);
    let colour = native_theme_egui::next::fill_of(&indicator)?;
    let width = indicator.min_size[1].filter(|w| w.is_finite() && *w > 0.0)?;
    let side = r.tab.indicator_side?;
    Some(TabLine {
        colour,
        width,
        top: side == native_theme::next::TabIndicatorSide::Top,
    })
}

/// The selected tab's line, where the theme states one ([`TabLook::line`]; docs/platform-facts.md
/// §2.11): a strip that thick across the tab's outer edge on that side, over its outline, as
/// Breeze fills its strip from the tab's frame; on top, rounded as the tab's top corners are.
/// egui's `Button` draws no such line, so the showcase paints it over the tab.
fn tab_indicator(
    ui: &egui::Ui,
    tab: egui::Rect,
    top_corners: egui::CornerRadius,
    line: Option<TabLine>,
) {
    let Some(TabLine { colour, width, top }) = line else {
        return;
    };
    let (rect, radius) = if top {
        (
            egui::Rect::from_min_max(tab.min, egui::pos2(tab.max.x, tab.min.y + width)),
            top_corners,
        )
    } else {
        (
            egui::Rect::from_min_max(egui::pos2(tab.min.x, tab.max.y - width), tab.max),
            egui::CornerRadius::ZERO,
        )
    };
    ui.painter().rect_filled(rect, radius, colour);
}

/// `tab.active_outward_corner_radius` of the installed 0.8.0 theme `r`, the radius the selected
/// tab's bottom corners curve out by into the bar (docs/platform-facts.md §2.11, WinUI 3's
/// `TabView`), where it states a positive, finite one: [`tab_bar`] draws the flares for it and
/// the Basic page's column rule makes room for them. `None` where it draws none: unstated, 0,
/// or not a finite length.
pub(crate) fn next_outward_radius(r: &native_theme::next::ResolvedTheme) -> Option<f32> {
    r.tab
        .active_outward_corner_radius
        .filter(|r| r.is_finite() && *r > 0.0)
}

/// The selected tab's outward corners, about its outer bounds `tab`, painted with `painter`
/// after every tab of the row, so over a neighbour's fill, and within the row's clip; `fill` and
/// `outline` the selected tab's, the tab scope's `Visuals::selection.bg_fill` and resting
/// `bg_stroke`, as egui draws a selected button (egui-0.37.0
/// `src/theme/default_style.rs:41-52`), `bottom_line` whether its line ([`TabLook::line`])
/// lies along its bottom edge. With `r` the radius and `w` the outline's width: first a strip
/// in `fill` over the
/// tab's bottom edge between its side edges, `[x0 + w, x1 - w] x [y1 - w, y1]`, since egui's
/// `Button` strokes all four sides alike, unless a bottom-side indicator covers that edge
/// already; then a flare at each foot, in an `r` by `r` square at the tab's bottom set `w` in
/// from its outer edge, as WinUI's outline lies just outside its fill: the left one's right edge
/// at `x0 + w`, the right one's left edge at `x1 - w`. Clipped to the square, about its upper
/// outer corner: an annulus from `max(r - w, 0)` out to `r * sqrt(2) + 1`, past the square's
/// far corner, in `fill`, and, where `w` is positive, over it one from
/// `max(r - w, 0)` to `r` in the outline's colour, the outline continued down the curve on the
/// disc's side to the bar's bottom edge. So the square less the quarter disc of radius `r`
/// shows in the fill and the band in the outline; at the square's top the band covers the side
/// edge's own column, so the two are one line, and below it the fill covers the side edge's
/// foot. epaint strokes a circle outside its radius (`epaint-0.36.2`
/// `src/tessellator.rs:1531`), so each annulus is a circle of its inner radius stroked as wide
/// as the annulus, and one from 0 is a disc. The flares are placed from `tab` rounded to
/// physical pixels, the rectangle epaint strokes the `Button`'s frame on (its stroke
/// `StrokeKind::Inside`, `egui-0.36.2` `src/containers/frame.rs:435-441`; `epaint-0.36.2`
/// `src/tessellator.rs:1830-1846`), as epaint draws a circle where it is given, so the curve
/// meets the side edge it continues. egui clips to whole physical pixels, rounding each edge of
/// a clip to the nearest (`egui-wgpu-0.36.2` `src/renderer.rs:1143-1151`, `egui_glow-0.36.2`
/// `src/painter.rs:771-780`), so at a fractional scale at most half a physical pixel of the
/// fill can show beside a square.
fn tab_flares(
    painter: &egui::Painter,
    tab: egui::Rect,
    r: f32,
    (fill, outline): (egui::Color32, egui::Stroke),
    bottom_line: bool,
) {
    use egui::emath::GuiRounding as _;
    let tab = tab.round_to_pixels(painter.pixels_per_point());
    let w = if outline.width.is_finite() {
        outline.width.max(0.0)
    } else {
        0.0
    };
    let outline = outline.color;
    if w > 0.0 && !bottom_line {
        let strip = egui::Rect::from_min_max(
            egui::pos2(tab.min.x + w, tab.max.y - w),
            egui::pos2(tab.max.x - w, tab.max.y),
        );
        painter.rect_filled(strip, egui::CornerRadius::ZERO, fill);
    }
    let top = tab.max.y - r;
    let far = r * std::f32::consts::SQRT_2 + 1.0;
    // The outline's inner radius, where the fill begins too.
    let inner = (r - w).max(0.0);
    let ring = |painter: &egui::Painter, centre: egui::Pos2, outer: f32, color| {
        if inner > 0.0 {
            painter.circle_stroke(centre, inner, egui::Stroke::new(outer - inner, color));
        } else {
            painter.circle_filled(centre, outer, color);
        }
    };
    // Each foot's square's left edge and its curve's centre.
    for (edge, centre) in [
        (tab.min.x + w - r, tab.min.x + w - r),
        (tab.max.x - w, tab.max.x - w + r),
    ] {
        let square = egui::Rect::from_min_size(egui::pos2(edge, top), egui::Vec2::splat(r));
        let clipped = painter.with_clip_rect(square);
        let centre = egui::pos2(centre, top);
        ring(&clipped, centre, far, fill);
        if w > 0.0 {
            ring(&clipped, centre, r, outline);
        }
    }
}

/// Scrolls a scrolling tab row so its selected tab, the `index`th, drawn at `tab`, shows whole
/// with its flares (`flare`, [`next_outward_radius`]), as the iced showcase's page tab strip scrolls
/// to the open tab (`reveal_open_tab`): once each time the selected tab, its place in the row or
/// the row's view changes, which a page opened from anywhere (`--tab`, the palette, a menu, a
/// click), a theme install and a window resize do, so a sideways scroll of the user's own stands
/// until then. Called while the row's scroll area lays its content out: egui scrolls that area,
/// the innermost round the target, the least that shows the target (egui 0.36.2
/// `src/containers/scroll_area.rs:1105-1139`), and the area takes the target on both axes, so
/// no area round the row scrolls for it (`:1086-1091`). The place and the view's width are kept
/// on egui's layout grid (`emath::GUI_ROUNDING`), on which egui lays them out, so a scroll,
/// which moves the tab and the row's content together, never reads as a change.
fn reveal_selected_tab(
    ui: &egui::Ui,
    kind: &str,
    index: usize,
    tab: egui::Rect,
    flare: Option<f32>,
) {
    let grid = |v: f32| (v / egui::emath::GUI_ROUNDING).round() as i64;
    let shown = (
        index,
        grid(tab.left() - ui.min_rect().left()),
        grid(tab.width()),
        grid(ui.clip_rect().width()),
    );
    let id = egui::Id::unique(kind).with("revealed tab");
    if ui.data(|d| d.get_temp::<(usize, i64, i64, i64)>(id)) != Some(shown) {
        ui.data_mut(|d| d.insert_temp(id, shown));
        let reach = egui::Vec2 {
            x: flare.unwrap_or_default(),
            ..egui::Vec2::ZERO
        };
        ui.scroll_to_rect(tab.expand2(reach), None);
    }
}

/// `show`, a scroll area nested in the page's, and the area brought into the page's view where a
/// widget inside it took an AccessKit `ScrollIntoView` request this pass: egui hands such a
/// request's scroll target to the innermost scroll area alone, which takes it and gives the
/// areas around it only a target set before it began (`ScrollArea::show`'s
/// `scroll_target[d].take()` and its restore of `saved_scroll_target`,
/// `egui-0.37.0/src/containers/scroll_area.rs`; the request's target, `Context::create_widget`,
/// `egui-0.37.0/src/context.rs`), so a widget the inner area scrolls to would stay where the page
/// shows nothing, under the status bar. The page's area takes the inner area's rect as its own
/// target here, after the inner one has ended.
pub(crate) fn nested_scroll<R>(
    ui: &mut egui::Ui,
    show: impl FnOnce(&mut egui::Ui) -> egui::scroll_area::ScrollAreaOutput<R>,
) -> egui::scroll_area::ScrollAreaOutput<R> {
    nested_scroll_in(ui, show, |out| out.inner_rect)
}

/// [`nested_scroll`] for a scroll area laid out inside what `show` returns, `rect` its rect: an
/// egui_extras table's body, which egui_extras always lays out in a `ScrollArea` of its own, one
/// that takes both scroll targets even where it scrolls neither way (egui_extras-0.37.0
/// `src/table.rs`, `Table::body`; `ScrollArea::show`, "We always take both scroll targets
/// regardless of which scroll axes are enabled").
pub(crate) fn nested_scroll_in<T>(
    ui: &mut egui::Ui,
    show: impl FnOnce(&mut egui::Ui) -> T,
    rect: impl FnOnce(&T) -> egui::Rect,
) -> T {
    let requests = |ui: &egui::Ui| {
        ui.input(|i| {
            i.events
                .iter()
                .filter(|event| {
                    matches!(event, egui::Event::AccessKitActionRequest(request)
                        if request.action == egui::accesskit::Action::ScrollIntoView)
                })
                .count()
        })
    };
    let before = requests(ui);
    let out = show(ui);
    if requests(ui) < before {
        ui.scroll_to_rect(rect(&out), None);
    }
    out
}

/// A row of tabs as the theme states them, in one `Role::Tab` scope (§10.4): each tab a
/// `Button::new(label).selected(..)`, whose own flag picks the selected tab's colours (§6.2) —
/// the cell's `tab.background_color`, `tab.active_background` and `tab.active_text_color`,
/// `tab.hover_background` (in place of the idle fill, over the strip) and `tab.hover_text_color`,
/// `tab.min_height`, the tab's padding and font (on the 0.8.0 model `tab.tab(<state>)`,
/// `tab.label(<state>)`, `tab.bar(NONE)`) — at least `look`'s least width wide
/// ([`TabLook::min_width`]), which egui's `Button` never reads from the style (connector spec
/// §5.3, T18(a)), on a strip in `tab.bar_background`, the tabs the scope's `item_spacing.x`
/// apart (`tab.item_gap` where stated). `tab.border` is the selected tab's (docs/platform-facts.md §2.11: KDE
/// strokes the selected tab only, an unselected one with no pen; WinUI's selected tab alone has
/// a border), so an unselected tab's stroke is set per call to the scope's width in no colour,
/// which keeps every tab the size its padding gives; and a tab is rounded on its two top corners
/// only, as both platforms that state a tab radius round it (Breeze `CornersTop`, WinUI
/// `TopCornerRadiusFilterConverter`, §2.11), the model's one `tab.border.corner_radius` on
/// each. The selected tab carries the line the theme states for it ([`tab_indicator`]); the
/// theme states no line under the row, so none is drawn there. Where the theme states
/// `tab.active_outward_corner_radius` ([`TabLook::flare`]), the selected tab's bottom corners
/// curve out into the bar, WinUI 3's folder tabs ([`tab_flares`]): its bottom edge is covered and
/// a flare spreads its fill from each foot into the bar's bottom edge, its outline running on
/// down the curve, painted after every tab with the tabs' own painter, so within the scroll
/// area's clip where the row scrolls; the row keeps that radius free before its first tab and
/// after its last, inside that clip, so the end tabs' flares show.
/// `trailing` adds what follows the tabs. Returns the tab clicked.
pub(crate) fn tab_bar<T: Copy + PartialEq>(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    look: &TabLook,
    bar: TabBar<'_, T>,
    trailing: impl FnOnce(&mut egui::Ui, Applied, &mut Registry),
) -> Option<T> {
    let min_width = look.min_width;
    let flare = look.flare;
    let mut picked = None;
    let strip = scoped_container(
        reg,
        ui,
        Role::Tab,
        RoleVariant::Normal,
        bar.kind,
        |ui, tab, reg| {
            let padding = egui::Vec2 {
                x: bar.margin.unwrap_or_default(),
                ..egui::Vec2::ZERO
            };
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
                    // The flares in the selected tab's own fill and outline.
                    let flare_paint = (ui.visuals().selection.bg_fill, idle.bg_stroke);
                    let gap = ui.spacing().item_spacing.x;
                    // Room for the end tabs' flares, inside the row's clip.
                    if let Some(r) = flare {
                        ui.add_space(r);
                    }
                    let mut selected_tab = None;
                    for (index, (value, label)) in bar.tabs.iter().enumerate() {
                        let selected = *value == bar.current;
                        let button = egui::Button::new((look.label)(ui, label))
                            .selected(selected)
                            .min_size(egui::Vec2 {
                                x: min_width,
                                ..egui::Vec2::ZERO
                            })
                            .corner_radius(top);
                        let r = tab.add(reg, ui, bar.tab_kind, |ui| {
                            if selected {
                                return ui.add(button);
                            }
                            // An unselected tab's outline in no pen, each state's at its own
                            // width: egui's frame takes each state's width out of its inner
                            // margin (`Frame::apply_stroke_and_expansion_without_layout_shift`,
                            // egui-0.37.0 `src/containers/frame.rs:313-323`, called by
                            // `src/theme/default_style.rs:69-72`), which `Button::stroke` does not
                            // put back (`src/widgets/button.rs:354-356`), so one pen for every
                            // state would size the tab by the state's width.
                            let kept = ui.visuals().widgets.clone();
                            let w = &mut ui.visuals_mut().widgets;
                            for state in [&mut w.inactive, &mut w.hovered, &mut w.active] {
                                state.bg_stroke.color = egui::Color32::TRANSPARENT;
                            }
                            let response = ui.add(button);
                            ui.visuals_mut().widgets = kept;
                            response
                        });
                        if selected {
                            tab_indicator(ui, r.rect, top, look.line);
                            selected_tab = Some(r.rect);
                            if bar.scroll {
                                reveal_selected_tab(ui, bar.kind, index, r.rect, flare);
                            }
                        }
                        if let Some(id) = bar.tab_elements.get(index) {
                            reg.tag(id, &r);
                        }
                        if r.clicked() {
                            picked = Some(*value);
                        }
                    }
                    if let Some(r) = flare {
                        // `r` after the last tab, whose spacing has moved the cursor `gap` on.
                        ui.add_space(r - gap);
                        if let Some(rect) = selected_tab {
                            let bottom_line = look.line.is_some_and(|line| !line.top);
                            tab_flares(ui.painter(), rect, r, flare_paint, bottom_line);
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
                } else if bar.full_width {
                    ui.horizontal(|ui| {
                        tabs(ui, reg);
                        trailing(ui, reg);
                    });
                } else {
                    ui.horizontal_wrapped(|ui| {
                        tabs(ui, reg);
                        trailing(ui, reg);
                    });
                }
            });
            out.response
        },
    );
    if let Some(note) = look.gap_unstated {
        reg.amend_last(|i| {
            i.not_themeable
                .push(("the gap between tabs", note.to_string()));
        });
    }
    reg.tag(bar.element, &strip);
    picked
}
