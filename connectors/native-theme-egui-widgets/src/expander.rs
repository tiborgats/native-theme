//! [`Expander`]: a disclosure header and the body it reveals, laid out as `expander.*` states.

use native_theme_egui::convert::{to_color32, to_margin};
use native_theme_egui::egui;
use native_theme_egui::native_theme::theme::{
    ArrowSide, ResolvedExpanderTheme, ResolvedPadding, ResolvedTheme,
};
use native_theme_egui::{Role, ThemeAtlas, expander_icon};

use crate::parts::Parts;
use crate::scope;

/// egui's own arrow is a triangle three quarters of its icon box
/// (`egui/src/containers/collapsing_header.rs:341`): the share, egui's own.
pub(crate) const EGUI_ARROW_SHARE: f32 = 0.75;

/// An expander: a header that shows or hides the body under it, in the `Role::Expander` scope.
///
/// **Tier P.** egui's `CollapsingHeader` places its arrow at half of `Spacing::indent`, its
/// title at `indent` and its body `indent` in (`egui/src/containers/collapsing_header.rs:516`,
/// `:585-589`, `:164`), one number for three the platforms set apart, always puts the arrow
/// before the title, and frames the header alone (`:561-568`). Where the theme states any of
/// `expander.arrow_side`, `arrow_gap`, `content_indent` or `frame_enabled`
/// (`docs/platform-facts.md` §2.27), this crate lays the expander out itself:
///
/// * the header is as wide as the `Ui`, at least the scope's `interact_size.y` tall
///   (`expander.header_height`), padded by the stated `expander.border.padding` sides (the
///   scope's `button_padding` on a side left unstated), its fill the scope's state fill
///   (`expander.hover_background` under the pointer);
/// * the arrow, `expander.arrow_icon_size` across in `expander.arrow_color` (the state's
///   text colour where unstated), is egui's triangle: before the title it turns from the
///   title to down as the body opens, egui's own turn; after it (`arrow_side` trailing) from
///   down to up, as libadwaita's and WinUI's trailing chevrons turn;
/// * `arrow_gap` lies between the arrow's box and the title (`Spacing::icon_spacing` where
///   unstated);
/// * the body is `content_indent` in (`Spacing::indent` where unstated), with no guide line;
/// * where `frame_enabled` is true, one frame in `expander.border`'s colour, width and radius
///   (the scope's `noninteractive` stroke and radius) holds the header and the body; where it
///   is false or unstated there is none.
///
/// Where the theme states none of the four, or with no atlas installed, it is egui's own
/// `CollapsingHeader` (in the expander's scope, with the connector's `expander_icon`, where an
/// atlas is installed). The open state lives in egui's `CollapsingState` under the header's
/// id, so it animates and persists as egui's does.
#[must_use = "You should call .show()"]
pub struct Expander {
    title: egui::WidgetText,
    id_salt: egui::IdSalt,
    default_open: bool,
}

/// What [`Expander::show`] returns: the header's response, and the body's value where the
/// body was shown.
pub struct ExpanderResponse<R> {
    /// The header's response: `clicked()` on the frame it toggles the body.
    pub header_response: egui::Response,
    /// What the body closure returned, where the body is (partly) open.
    pub body_returned: Option<R>,
}

impl Expander {
    /// An expander titled `title`, closed at first, its id salted by the title's text.
    pub fn new(title: impl Into<egui::WidgetText>) -> Self {
        let title = title.into();
        Self {
            id_salt: egui::IdSalt::new(title.text()),
            title,
            default_open: false,
        }
    }

    /// Salt the expander's id with `salt` in place of the title, as
    /// `CollapsingHeader::id_salt`.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = egui::IdSalt::new(salt);
        self
    }

    /// Whether the body is open the first time the expander is shown.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Show the header and, where open, the body `add_body` fills.
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        add_body: impl FnOnce(&mut egui::Ui) -> R,
    ) -> ExpanderResponse<R> {
        let resolved: Option<ResolvedTheme> = ThemeAtlas::from_ctx(ui.ctx())
            .map(|atlas| atlas.resolved_for(ui.ctx().theme()).clone());
        let Some(t) = resolved else {
            return own(ui, self, None, add_body);
        };
        let e = &t.expander;
        if e.arrow_side.is_none()
            && e.arrow_gap.is_none()
            && e.content_indent.is_none()
            && e.frame_enabled.is_none()
        {
            return scope::open(ui, Role::Expander, true, |ui| {
                own(ui, self, Some(&t), add_body)
            });
        }
        scope::open(ui, Role::Expander, true, |ui| {
            painted(ui, self, e, add_body)
        })
    }
}

/// egui's own `CollapsingHeader`, with the connector's arrow where a theme is given.
fn own<R>(
    ui: &mut egui::Ui,
    expander: Expander,
    t: Option<&ResolvedTheme>,
    add_body: impl FnOnce(&mut egui::Ui) -> R,
) -> ExpanderResponse<R> {
    let Expander {
        title,
        id_salt,
        default_open,
    } = expander;
    // Where egui lays the header out (`egui/src/containers/collapsing_header.rs:512-590`): the
    // title `indent` in, in the button font, centred on the row; the arrow's box
    // `icon_width_inner` square, centred half an indent in.
    let indent = ui.spacing().indent;
    let arrow = ui.spacing().icon_width_inner;
    let title_size = title
        .clone()
        .into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            egui::TextStyle::Button,
        )
        .size();
    let header = egui::CollapsingHeader::new(title)
        .id_salt(id_salt)
        .default_open(default_open);
    let header = match t {
        Some(t) => header.icon(expander_icon(t)),
        None => header,
    };
    let out = header.show(ui, add_body);
    let rect = out.header_response.rect;
    let mut parts = Parts::default();
    parts.push("header", rect);
    parts.push(
        "arrow",
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 0.5 * indent, rect.center().y),
            egui::Vec2::splat(arrow),
        ),
    );
    parts.push(
        "title",
        egui::Rect::from_min_size(
            egui::pos2(rect.left() + indent, rect.center().y - 0.5 * title_size.y),
            title_size,
        ),
    );
    if let Some(body) = &out.body_response {
        parts.push("body", body.rect);
    }
    parts.store(ui.ctx(), out.header_response.id);
    ExpanderResponse {
        header_response: out.header_response,
        body_returned: out.body_returned,
    }
}

/// A stated, finite length; `own` where the theme states none or one that is not finite.
fn stated_or(v: Option<f32>, own: f32) -> f32 {
    v.and_then(scope::length).unwrap_or(own)
}

/// The expander laid out as `e` states it.
fn painted<R>(
    ui: &mut egui::Ui,
    expander: Expander,
    e: &ResolvedExpanderTheme,
    add_body: impl FnOnce(&mut egui::Ui) -> R,
) -> ExpanderResponse<R> {
    let Expander {
        title,
        id_salt,
        default_open,
    } = expander;
    let id = ui.make_persistent_id(id_salt);
    let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        default_open,
    );
    let edge = ui.visuals().widgets.noninteractive;
    let frame = if e.frame_enabled == Some(true) {
        egui::Frame::NONE
            .stroke(edge.bg_stroke)
            .corner_radius(edge.corner_radius)
    } else {
        egui::Frame::NONE
    };
    let out = frame.show(ui, |ui| {
        let (header, mut parts) = header(ui, &mut state, id, title, e);
        let own_indent = ui.spacing().indent;
        let indent = stated_or(e.content_indent, own_indent);
        let body = state.show_body_unindented(ui, |ui| {
            let inset = ResolvedPadding {
                top: None,
                right: None,
                bottom: None,
                left: Some(indent),
            };
            egui::Frame::NONE
                .inner_margin(to_margin(egui::Margin::ZERO, &inset))
                .show(ui, add_body)
                .inner
        });
        if let Some(body) = &body {
            parts.push("body", body.response.rect);
        }
        parts.store(ui.ctx(), header.id);
        (header, body.map(|b| b.inner))
    });
    let (header_response, body_returned) = out.inner;
    ExpanderResponse {
        header_response,
        body_returned,
    }
}

/// The header row: allocated, sensed and painted, toggling `state` on a click; with the
/// `header`, `arrow` and `title` [`Parts`] it painted.
fn header(
    ui: &mut egui::Ui,
    state: &mut egui::collapsing_header::CollapsingState,
    id: egui::Id,
    title: egui::WidgetText,
    e: &ResolvedExpanderTheme,
) -> (egui::Response, Parts) {
    let own_pad = ui.spacing().button_padding;
    let pad = &e.border.padding;
    let (left, right) = (
        stated_or(pad.left, own_pad.x),
        stated_or(pad.right, own_pad.x),
    );
    let (top, bottom) = (
        stated_or(pad.top, own_pad.y),
        stated_or(pad.bottom, own_pad.y),
    );
    let arrow = stated_or(Some(e.arrow_icon_size), ui.spacing().icon_width_inner);
    let gap = stated_or(e.arrow_gap, ui.spacing().icon_spacing);
    let trailing = e.arrow_side == Some(ArrowSide::Trailing);

    let width = ui.available_width();
    let wrap = (width - left - right - arrow - gap).max(0.0);
    let galley = title.into_galley(
        ui,
        Some(egui::TextWrapMode::Extend),
        wrap,
        egui::TextStyle::Button,
    );
    let height = (galley.size().y + top + bottom).max(ui.spacing().interact_size.y);
    let (_, rect) = ui.allocate_space(egui::vec2(width, height));
    let mut response = ui.interact(rect, id, egui::Sense::click());
    if response.clicked() {
        state.toggle(ui);
        response.mark_changed();
    }
    let enabled = ui.is_enabled();
    let text = galley.text().to_string();
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::CollapsingHeader, enabled, &text)
    });
    let (arrow_x, text_x) = if trailing {
        (rect.right() - right - arrow, rect.left() + left)
    } else {
        (rect.left() + left, rect.left() + left + arrow + gap)
    };
    let centre_y = rect.center().y;
    let arrow_box = egui::Rect::from_min_size(
        egui::pos2(arrow_x, centre_y - 0.5 * arrow),
        egui::Vec2::splat(arrow),
    );
    let title_rect = egui::Rect::from_min_size(
        egui::pos2(text_x, centre_y - 0.5 * galley.size().y),
        galley.size(),
    );
    let mut parts = Parts::default();
    parts.push("header", rect);
    parts.push("arrow", arrow_box);
    parts.push("title", title_rect);
    if !ui.is_rect_visible(rect) {
        return (response, parts);
    }
    let visuals = ui.style().interact(&response);
    let painter = ui.painter();
    painter.rect_filled(rect, visuals.corner_radius, visuals.weak_bg_fill);
    let colour = e.arrow_color.map_or(visuals.fg_stroke.color, to_color32);
    painter.add(egui::Shape::convex_polygon(
        triangle(arrow_box, state.openness(ui.ctx()), trailing),
        colour,
        egui::Stroke::NONE,
    ));
    let text_colour = visuals.text_color();
    painter.galley(title_rect.min, galley, text_colour);
    (response, parts)
}

/// egui's arrow in `rect` (`egui/src/containers/collapsing_header.rs:336-357`): a triangle
/// pointing down, three quarters of the box, turned by the openness -- a quarter turn from
/// the title (right) to down before the title, egui's own, and half a turn from down to up
/// after it.
fn triangle(rect: egui::Rect, openness: f32, trailing: bool) -> Vec<egui::Pos2> {
    use std::f32::consts::{FRAC_PI_2, PI};
    let open = openness.clamp(0.0, 1.0);
    let angle = if trailing {
        open * PI
    } else {
        (open - 1.0) * FRAC_PI_2
    };
    let (sin, cos) = angle.sin_cos();
    let c = rect.center();
    let half_w = 0.5 * EGUI_ARROW_SHARE * rect.width();
    let half_h = 0.5 * EGUI_ARROW_SHARE * rect.height();
    [(-half_w, -half_h), (half_w, -half_h), (0.0, half_h)]
        .into_iter()
        .map(|(dx, dy)| egui::pos2(c.x + cos * dx - sin * dy, c.y + sin * dx + cos * dy))
        .collect()
}
