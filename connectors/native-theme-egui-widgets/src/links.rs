//! **Tier W**: egui's `Link` and `Hyperlink` in `link.*`'s state and visited colours.
//!
//! The connector grades `link.hover_text_color`, `link.active_text_color`,
//! `link.hover_background` and `link.visited_text_color` DERIVED: a per-call colour egui's
//! own `Link` takes. These functions are the ready-made form of that call.
//!
//! `Link` paints in `visuals.hyperlink_color` (`egui/src/widgets/hyperlink.rs:47`) only as a
//! fallback, which `TextShape` puts in place of `Color32::PLACEHOLDER` alone
//! (`epaint/src/shapes/text_shape.rs:25`), the colour of text with none of its own
//! (`egui/src/widget_text.rs:423`). So the text is coloured with `WidgetText::color`: in a
//! disabled `Ui` `link.disabled_text_color`; else, by the link's interaction in the previous
//! pass (`Context::read_response`), `link.active_text_color` while pressed and
//! `link.hover_text_color` while hovered; else `link.visited_text_color` for a visited
//! [`hyperlink`] and `link.font.color` otherwise. The scope's `hyperlink_color` is set to the
//! same colour, so egui's own hover underline matches it. `link.underline_enabled` underlines
//! the text at rest (`WidgetText::underline`, drawn in the text's colour), and
//! `link.background_color`, with `link.hover_background` composited over it while hovered, is
//! the text's background (`WidgetText::background_color`).
//!
//! A link wrapped over several rows answers on its first row only: a wrapped `Label` gives
//! each further row an id of its own (`egui/src/widgets/label.rs:219-227`).
//!
//! What stays egui's: `Link` underlines itself on hover or focus
//! (`egui/src/widgets/hyperlink.rs:50-54`) whatever `link.underline_enabled` states, and a
//! bare [`link`] carries no URL, so it has no visited colour.
//!
//! With no atlas installed, both add egui's own widget unchanged.

use std::collections::BTreeSet;
use std::sync::Arc;

use native_theme_egui::convert::to_color32;
use native_theme_egui::egui;
use native_theme_egui::native_theme::next::{
    DecorationKind, ResolvedTheme as NextResolvedTheme, TextStated, WidgetState,
};
use native_theme_egui::{NativeThemeUiExt as _, ResolvedTheme, Role, RoleVariant, ThemeAtlas};

use crate::next;

/// The URLs a [`hyperlink`] of this `Context` was clicked for: its visited set, kept in
/// `Context` data for the life of the `Context`. Shared, so a read clones the `Arc` and not the
/// set (`IdTypeMap::get_temp` clones the value it finds, `egui/src/util/id_type_map.rs:448-450`);
/// a click copies it only while a read still holds it.
#[derive(Clone, Default)]
struct Visited(Arc<BTreeSet<String>>);

fn visited_key() -> egui::Id {
    egui::Id::unique("native-theme-egui-widgets::visited")
}

/// What a link reads of a theme, every colour read in one place, so the crate's colour-leaf
/// coverage test sees the widget's reads. Each is `None` where the theme states none: the
/// text keeps egui's own colour there, the background none, the hover layer nothing, and the
/// underline egui's own (spec §10, rule 0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    /// `link.font.color`: the text at rest.
    pub(crate) text_color: Option<egui::Color32>,
    pub(crate) background_color: Option<egui::Color32>,
    pub(crate) hover_background: Option<egui::Color32>,
    pub(crate) hover_text_color: Option<egui::Color32>,
    pub(crate) active_text_color: Option<egui::Color32>,
    pub(crate) visited_text_color: Option<egui::Color32>,
    pub(crate) disabled_text_color: Option<egui::Color32>,
    pub(crate) underline_enabled: Option<bool>,
}

impl Look {
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        let l = &t.link;
        let c = |colour| Some(to_color32(colour));
        Self {
            text_color: c(l.font.color),
            background_color: c(l.background_color),
            hover_background: c(l.hover_background),
            hover_text_color: c(l.hover_text_color),
            active_text_color: c(l.active_text_color),
            visited_text_color: c(l.visited_text_color),
            disabled_text_color: c(l.disabled_text_color),
            underline_enabled: Some(l.underline_enabled),
        }
    }

    /// The look of a link of a 0.8.0 theme, at the homes of the 0.7.2 fields `of` reads:
    /// `link.label`'s colour at `NONE`, `HOVER`, `PRESSED`, `VISITED` and `DISABLED`,
    /// `link.container(NONE)`'s fill, the hover layer `link.container(HOVER)`'s fill where a
    /// rule for `HOVER` states it (spec §2, `from_state`), and the underline at rest, whether
    /// `link.label(NONE)`'s stated decoration is an underline.
    pub(crate) fn next_of(r: &NextResolvedTheme) -> Self {
        let l = &r.link;
        let label = |s: WidgetState| next::text_color_of(&l.label(s));
        let rest = l.label(WidgetState::NONE);
        let hover = l.container(WidgetState::HOVER);
        Self {
            text_color: next::text_color_of(&rest),
            background_color: next::fill_of(&l.container(WidgetState::NONE)),
            hover_background: next::ruled_fill(&hover, &hover),
            hover_text_color: label(WidgetState::HOVER),
            active_text_color: label(WidgetState::PRESSED),
            visited_text_color: label(WidgetState::VISITED),
            disabled_text_color: label(WidgetState::DISABLED),
            underline_enabled: rest
                .stated
                .contains(TextStated::DECORATION)
                .then_some(rest.decoration.kind == DecorationKind::Underline),
        }
    }

    /// The look as a link paints it, so two looks that paint alike compare equal: a
    /// transparent background, which paints as none, given as none, and a hover layer that
    /// paints the background as it is at rest given as none.
    #[cfg(test)]
    pub(crate) fn painted(self) -> Self {
        let clear = |c: Option<egui::Color32>| c.filter(|c| *c != egui::Color32::TRANSPARENT);
        let rest = self.background_color.unwrap_or(egui::Color32::TRANSPARENT);
        Self {
            background_color: clear(self.background_color),
            hover_background: self
                .hover_background
                .filter(|&hover| rest.blend(hover) != rest),
            ..self
        }
    }

    /// The text's colour in a state: disabled, pressed, hovered, visited or at rest, each
    /// where the theme states it.
    fn text(
        &self,
        enabled: bool,
        pressed: bool,
        hovered: bool,
        visited: bool,
    ) -> Option<egui::Color32> {
        if !enabled {
            self.disabled_text_color
        } else if pressed {
            self.active_text_color
        } else if hovered {
            self.hover_text_color
        } else if visited {
            self.visited_text_color
        } else {
            self.text_color
        }
    }
}

/// `ui`'s link look, from the installed atlas in the scheme egui is drawing, of either model;
/// `None` without one.
fn link_theme(ui: &egui::Ui) -> Option<Look> {
    let atlas = ThemeAtlas::from_ctx(ui.ctx())?;
    let theme = ui.ctx().theme();
    match atlas.resolved_for(theme) {
        Some(t) => Some(Look::of(t)),
        None => atlas.next_resolved_for(theme).map(Look::next_of),
    }
}

/// egui's `Link` in `link.*`'s colours. `ui` is read for the atlas and the scheme only.
pub fn link<'a>(ui: &egui::Ui, text: impl Into<egui::WidgetText>) -> impl egui::Widget + 'a {
    let theme = link_theme(ui);
    let text = text.into();
    move |ui: &mut egui::Ui| match theme {
        Some(l) => themed(ui, &l, text, None).0,
        None => ui.add(egui::Link::new(text)),
    }
}

/// egui's `Hyperlink` in `link.*`'s colours, with `link.visited_text_color` at rest once this
/// `Context` has seen it clicked, and its AccessKit node marked visited then. `ui` is read for
/// the atlas and the scheme only.
pub fn hyperlink<'a>(
    ui: &egui::Ui,
    text: impl Into<egui::WidgetText>,
    url: impl ToString,
) -> impl egui::Widget + 'a {
    let theme = link_theme(ui);
    let text = text.into();
    let url = url.to_string();
    move |ui: &mut egui::Ui| match theme {
        Some(l) => {
            let (response, visited) = themed(ui, &l, text, Some(&url));
            if response.clicked() && !visited {
                ui.ctx().data_mut(|d| {
                    Arc::make_mut(&mut d.get_temp_mut_or_default::<Visited>(visited_key()).0)
                        .insert(url);
                });
                ui.ctx().request_repaint();
            }
            response
        }
        None => ui.add(egui::Hyperlink::from_label_and_url(text, url)),
    }
}

/// Add the link, or the hyperlink to `url`, in the link scope with its colours; whether `url`
/// was visited.
fn themed(
    ui: &mut egui::Ui,
    l: &Look,
    text: egui::WidgetText,
    url: Option<&str>,
) -> (egui::Response, bool) {
    ui.native_scope(Role::Link, RoleVariant::Normal, |ui| {
        let visited = url.is_some_and(|url| {
            ui.ctx().data(|d| {
                d.get_temp::<Visited>(visited_key())
                    .is_some_and(|v| v.0.contains(url))
            })
        });
        // The interaction of the previous pass, as `Checkbox` reads it
        // (`egui/src/widgets/checkbox.rs:72-73`): `Hyperlink` adds its `Link` first, so the
        // link's id is the next one either way.
        let previous = ui.ctx().read_response(ui.next_auto_id());
        let hovered = previous.as_ref().is_some_and(egui::Response::hovered);
        let pressed = previous
            .as_ref()
            .is_some_and(egui::Response::is_pointer_button_down_on);
        let enabled = ui.is_enabled();
        let mut text = text;
        if let Some(colour) = l.text(enabled, pressed, hovered, visited) {
            ui.visuals_mut().hyperlink_color = colour;
            text = text.color(colour);
        }
        let hover = l.hover_background.filter(|_| hovered && enabled);
        let background = match (hover, l.background_color) {
            (Some(hover), rest) => Some(rest.unwrap_or(egui::Color32::TRANSPARENT).blend(hover)),
            (None, rest) => rest,
        };
        if let Some(background) = background {
            text = text.background_color(background);
        }
        if l.underline_enabled == Some(true) {
            text = text.underline();
        }
        let response = match url {
            Some(url) => ui.add(egui::Hyperlink::from_label_and_url(text, url)),
            None => ui.add(egui::Link::new(text)),
        };
        if visited {
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_visited();
            });
        }
        (response, visited)
    })
    .inner
}
