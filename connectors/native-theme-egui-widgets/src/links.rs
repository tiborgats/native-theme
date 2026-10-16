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

use native_theme_egui::convert::{composite_over, to_color32};
use native_theme_egui::egui;
use native_theme_egui::native_theme::theme::ResolvedLinkTheme;
use native_theme_egui::{NativeThemeUiExt as _, Role, RoleVariant, ThemeAtlas};

/// The URLs a [`hyperlink`] of this `Context` was clicked for: its visited set, kept in
/// `Context` data for the life of the `Context`.
#[derive(Clone, Default)]
struct Visited(BTreeSet<String>);

fn visited_key() -> egui::Id {
    egui::Id::new("native-theme-egui-widgets::visited")
}

/// `ui`'s link theme, from the installed atlas in the scheme egui is drawing.
fn link_theme(ui: &egui::Ui) -> Option<ResolvedLinkTheme> {
    ThemeAtlas::from_ctx(ui.ctx()).map(|atlas| atlas.resolved_for(ui.ctx().theme()).link.clone())
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
                    d.get_temp_mut_or_default::<Visited>(visited_key())
                        .0
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
    l: &ResolvedLinkTheme,
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
        let colour = if !ui.is_enabled() {
            l.disabled_text_color
        } else if pressed {
            l.active_text_color
        } else if hovered {
            l.hover_text_color
        } else if visited {
            l.visited_text_color
        } else {
            l.font.color
        };
        let background = if hovered && ui.is_enabled() {
            composite_over(l.hover_background, l.background_color)
        } else {
            to_color32(l.background_color)
        };
        ui.visuals_mut().hyperlink_color = to_color32(colour);
        let mut text = text.color(to_color32(colour)).background_color(background);
        if l.underline_enabled {
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
