//! Icons (spec §10.4's palette table), laid out as the gpui showcase's Icons page
//! (`connectors/native-theme-gpui/examples/showcase-gpui/pages/icons.rs:19-287`): the chosen icon
//! theme's folder icon at each size `icons.sizes` states, the loading indicator the theme
//! ships, every icon of the theme above its role's name, and `ui.image` — an image carries no
//! theme, so every item is in the base style.

use native_theme::icons::IconSet;
use native_theme::icons::{IconRole, TransformAnimation};
use native_theme::next::{ResolvedTheme, WidgetState};
use native_theme_egui::icons::{self, IconContext};
use native_theme_egui::next::{border_color, border_corner_radius, icon_size};
use native_theme_showcase_support::rows;

use super::{DemoState, caption};
use crate::app::OldAtlas;
use crate::demo::{self, IconCache, Registry};

/// The size `icons.sizes` of the installed 0.8.0 theme `r` states for `context`; `None` where it
/// states none, and the icons of that size are then not drawn, their place saying so.
fn size_of(r: Option<&ResolvedTheme>, context: IconContext) -> Option<f32> {
    r.and_then(|r| icon_size(r, context).ok())
}

/// The contexts `icons.sizes` names, in the gpui showcase's order and by its names
/// (`showcase-gpui/demo.rs:8633-8695`).
const CONTEXTS: [(IconContext, &str); 5] = [
    (IconContext::Toolbar, "toolbar"),
    (IconContext::Small, "small"),
    (IconContext::Large, "large"),
    (IconContext::Dialog, "dialog"),
    (IconContext::Panel, "panel"),
];

pub(crate) fn show(
    reg: &mut Registry,
    _state: &mut DemoState,
    cache: &mut IconCache,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
    detect_icon_theme: fn() -> native_theme::Result<String>,
) {
    let r = atlas.next_shown(ui.ctx());
    let (set, icon_theme) = chosen;
    let set_label = set_label(*set, icon_theme.as_deref(), detect_icon_theme);
    let pad = atlas.layout().container_margin;

    // The folder icon at each context's size, above the context's name
    // (`showcase-gpui/demo.rs:8703-8745`); where the set has no folder icon, the name alone, and
    // where the theme states no size, the name saying so.
    caption(reg, ui, "Icon Sizes");
    // One height for every cell, its content from the bottom, so the names share a line, as
    // gpui's `items_end` row has them (`showcase-gpui/pages/icons.rs:35`).
    let tallest = CONTEXTS
        .iter()
        .filter_map(|(context, _)| size_of(r, *context))
        .fold(0.0, f32::max);
    ui.horizontal_wrapped(|ui| {
        for (context, name) in CONTEXTS {
            let size = size_of(r, context);
            let image = size.and_then(|size| {
                demo::role_image(
                    ui,
                    IconRole::FolderClosed,
                    *set,
                    icon_theme.as_deref(),
                    size,
                )
            });
            let name = match size {
                Some(_) => name.to_owned(),
                None => format!("{name}: no icons.sizes.{name} stated"),
            };
            let width = size.unwrap_or_default().max(small_width(ui, &name));
            let layout = egui::Layout::bottom_up(egui::Align::Center);
            cell(ui, (width, tallest), layout, pad, |ui| {
                demo::base(reg, ui, "Label · icon size", |ui| {
                    ui.label(egui::RichText::new(name).small())
                });
                if let Some(image) = image {
                    demo::base(reg, ui, "Image · icon size", |ui| ui.add(image));
                }
            });
        }
    });

    caption(reg, ui, "Animated Icons");
    icons_indicator(reg, cache, atlas, ui, chosen, &set_label);

    // Every role's icon at `icons.sizes.small` above the role's name
    // (`showcase-gpui/pages/icons.rs:138-217`); where the set has none, gpui's placeholder: a
    // square in `skeleton` — `button.body(NONE).fill` (`connectors/native-theme-gpui/src/colors.rs:205`,
    // `:582`), egui's own button fill (`widgets.inactive.weak_bg_fill`) where the theme states
    // none — at the theme's radius (`layout.corner_radius`, egui's own button radius without an
    // installed 0.8.0 theme), never another set's icon; where the theme states no small size,
    // none, and a line says so.
    let small = size_of(r, IconContext::Small);
    let images: Vec<(IconRole, Option<egui::Image<'static>>)> = small
        .map(|size| {
            IconRole::ALL
                .iter()
                .map(|role| {
                    let image = demo::role_image(ui, *role, *set, icon_theme.as_deref(), size);
                    (*role, image)
                })
                .collect()
        })
        .unwrap_or_default();
    let loaded = images.iter().filter(|(_, image)| image.is_some()).count();
    let elsewhere = if native_here(*set) {
        ""
    } else {
        " (not this platform's icon theme)"
    };
    caption(
        reg,
        ui,
        &format!(
            "Native Theme Icons: {set_label} [{loaded}/{} loaded]{elsewhere}",
            images.len()
        ),
    );
    // Why the system icon theme is unavailable, where its detection failed: the set's label
    // leaves the reason out, as a label is short; the iced and gpui Icons pages give it too.
    if let Err(error) = detect_icon_theme() {
        let reason = rows::icon_theme_unavailable(&error);
        demo::base(reg, ui, "Label · icon theme unavailable", |ui| {
            ui.label(egui::RichText::new(reason).small().weak())
        });
    }
    let Some(size) = small else {
        demo::base(reg, ui, "Label · no small icon size", |ui| {
            ui.label(
                egui::RichText::new("The theme states no icons.sizes.small: no icon is drawn")
                    .small()
                    .weak(),
            )
        });
        return;
    };
    let own = &ui.visuals().widgets.inactive;
    let skeleton = r
        .and_then(|r| demo::drawn_fill(&r.button.body(WidgetState::NONE)))
        .unwrap_or(own.weak_bg_fill);
    let radius = r.map_or(own.corner_radius, |r| {
        egui::CornerRadius::from(border_corner_radius(r))
    });
    ui.horizontal_wrapped(|ui| {
        for (role, image) in images {
            let name = format!("{role:?}");
            let width = size.max(small_width(ui, &name));
            let layout = egui::Layout::top_down(egui::Align::Center);
            cell(ui, (width, size), layout, pad, |ui| {
                match image {
                    Some(image) => {
                        demo::base(reg, ui, "Image", |ui| ui.add(image));
                    }
                    None => {
                        demo::base(reg, ui, "Image (absent)", |ui| {
                            let (rect, response) = ui
                                .allocate_exact_size(egui::Vec2::splat(size), egui::Sense::hover());
                            ui.painter().rect_filled(rect, radius, skeleton);
                            response
                        });
                    }
                }
                demo::base(reg, ui, "Label · icon name", |ui| {
                    ui.label(egui::RichText::new(name).small())
                });
            });
        }
    });

    let Some(toolbar) = size_of(r, IconContext::Toolbar) else {
        caption(reg, ui, "ui.image");
        demo::base(reg, ui, "Label · no toolbar icon size", |ui| {
            ui.label(
                egui::RichText::new("The theme states no icons.sizes.toolbar: no image is drawn")
                    .small()
                    .weak(),
            )
        });
        return;
    };
    if let Some(image) = demo::role_image(
        ui,
        IconRole::ActionSave,
        *set,
        icon_theme.as_deref(),
        toolbar,
    ) {
        caption(reg, ui, "ui.image");
        let source = image.source(ui.ctx());
        // `ui.image` fills the room its `Ui` offers (`ImageSize::default`,
        // `egui/src/widgets/image.rs:558-563`), so that room is the icon's size.
        demo::base(reg, ui, "ui.image", |ui| {
            ui.scope(|ui| {
                ui.set_max_size(egui::Vec2::splat(toolbar));
                ui.image(source)
            })
            .inner
        });
    }
}

/// The width `text` takes in `Small`, on one line.
fn small_width(ui: &egui::Ui, text: &str) -> f32 {
    egui::WidgetText::from(egui::RichText::new(text).small())
        .into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            egui::TextStyle::Small,
        )
        .size()
        .x
}

/// A cell `width` wide holding an icon of at most `icon` points over a `Small` name, its content
/// centred in a column as the gpui showcase's icon cells are (`items_center`,
/// `showcase-gpui/demo.rs:8569-8588`), laid out by `layout`, and padded by `pad` —
/// `layout.container_margin` in a gallery, standing in for the gpui cell's own `px_2`/`py_2`.
fn cell(
    ui: &mut egui::Ui,
    (width, icon): (f32, f32),
    layout: egui::Layout,
    pad: Option<f32>,
    add: impl FnOnce(&mut egui::Ui),
) {
    let pad = pad.unwrap_or_default();
    let height = icon + ui.spacing().item_spacing.y + ui.text_style_height(&egui::TextStyle::Small);
    // The padding is part of the cell's own size, not a `Frame` around it: a frame places its
    // rect where the row's cursor is, past a wrapping row's end (`Frame::show`).
    let both = pad + pad;
    let size = egui::vec2(width + both, height + both);
    // The whole cell is used, so the row advances past it whatever the cell holds.
    ui.allocate_ui_with_layout(size, layout, |ui| {
        ui.set_min_size(size);
        // The padding on the side the layout starts from; the far side's is the cell's size.
        ui.add_space(pad);
        add(ui);
    });
}

/// The icon theme as the gpui showcase's Icons page names it (`showcase-gpui/app.rs`,
/// `icon_set_label`): a freedesktop set with the theme its icons load from — where that is the
/// system's and it cannot be detected, `unavailable`, the reason being the Icon theme
/// drop-down's hover text and the page's line under its Native Theme Icons heading, as a label
/// is short — else the set's name. `detect_icon_theme` is
/// the system icon theme's detection (`App::detect_icon_theme`).
pub(crate) fn set_label(
    set: IconSet,
    icon_theme: Option<&str>,
    detect_icon_theme: fn() -> native_theme::Result<String>,
) -> String {
    if set != IconSet::Freedesktop {
        return set.name().to_string();
    }
    match icon_theme {
        Some(theme) => format!("freedesktop ({theme})"),
        None => match detect_icon_theme() {
            Ok(theme) => format!("freedesktop ({theme})"),
            Err(_) => "freedesktop (unavailable)".to_string(),
        },
    }
}

/// Whether `set` is this platform's own, or bundled (`showcase-gpui/support.rs:505-513`).
fn native_here(set: IconSet) -> bool {
    match set {
        IconSet::Freedesktop => cfg!(target_os = "linux"),
        IconSet::SfSymbols => cfg!(any(target_os = "macos", target_os = "ios")),
        IconSet::SegoeFluent => cfg!(target_os = "windows"),
        _ => true,
    }
}

/// The loading indicator the chosen icon theme ships (§10.4, the Icons row), in a card framed
/// as the gpui showcase's `demo_frame` in the installed 0.8.0 theme's `layout.border_width`,
/// `colors.window.border` and `layout.corner_radius` (egui's own group stroke colour,
/// `widgets.noninteractive.bg_stroke`, where it states no border colour, and egui's own group
/// frame without an installed 0.8.0 theme) and padded by `layout.container_margin`, at
/// `icons.sizes.dialog`, above what it is — the gpui showcase's animated-icon card
/// (`showcase-gpui/demo.rs:8775-8832`); where the theme ships none, gpui's caption, and where it
/// states no dialog size, a line that says so.
fn icons_indicator(
    reg: &mut Registry,
    cache: &mut IconCache,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
    set_label: &str,
) {
    let r = atlas.next_shown(ui.ctx());
    let Some(size) = size_of(r, IconContext::Dialog) else {
        demo::base(reg, ui, "loading indicator (no size)", |ui| {
            ui.label(
                egui::RichText::new(
                    "The theme states no icons.sizes.dialog: no indicator is drawn",
                )
                .small()
                .weak(),
            )
        });
        return;
    };
    let (set, icon_theme) = chosen;
    let reduce_motion = atlas.accessibility().reduce_motion;
    // The icon theme is in the key for the cache alone: `icons::to_image` colours by the set
    // and the tint.
    let mut key = icons::IconKey::name("indicator", *set);
    if let Some(theme) = icon_theme {
        key = key.icon_theme(theme);
    }
    if *set != IconSet::Freedesktop {
        key = key.tint(ui.visuals().text_color());
    }
    let indicator = cache.indicator(ui.ctx(), &key, (*set, icon_theme.as_deref()), reduce_motion);
    let Some((anim, image)) = indicator else {
        demo::base(reg, ui, "loading indicator (absent)", |ui| {
            ui.label(
                egui::RichText::new("No animated icons available for the current icon theme")
                    .small()
                    .weak(),
            )
        });
        return;
    };
    if reduce_motion {
        demo::base(reg, ui, "Label · reduced motion", |ui| {
            ui.label(
                egui::RichText::new("(reduced motion: showing each animation's first frame)")
                    .small()
                    .weak(),
            )
        });
    }
    let what = match (
        anim.frame_list(),
        anim.frame_duration_ms(),
        anim.animation(),
    ) {
        (Some(frames), Some(ms), _) => format!("Frames: {} ({ms}ms)", frames.len()),
        (_, _, Some(TransformAnimation::Spin { duration_ms })) => {
            format!("Spin ({duration_ms}ms)")
        }
        _ => "an animation of a kind the page does not name".to_string(),
    };
    let label = if reduce_motion {
        format!("{set_label} - {what} (reduced motion)")
    } else {
        format!("{set_label} - {what}")
    };
    // `spin_angle` is `None` for a frame animation and under reduced motion: no rotation.
    let angle = icons::spin_angle(ui.ctx(), anim, reduce_motion).unwrap_or_default();
    let margin = atlas.layout().container_margin.unwrap_or_default();
    let own = &ui.visuals().widgets.noninteractive;
    let (stroke, radius) = match r {
        Some(r) => (
            egui::Stroke::new(
                r.layout.border_width,
                border_color(r).unwrap_or(own.bg_stroke.color),
            ),
            egui::CornerRadius::from(border_corner_radius(r)),
        ),
        None => (own.bg_stroke, own.corner_radius),
    };
    let card = egui::Frame::NONE
        .stroke(stroke)
        .corner_radius(radius)
        .inner_margin(margin);
    demo::contained(reg, ui, "animated icon card", |ui, reg| {
        card.show(ui, |ui| {
            let width = size.max(small_width(ui, &label));
            let layout = egui::Layout::top_down(egui::Align::Center);
            cell(ui, (width, size), layout, None, |ui| {
                if let Some(image) = image {
                    let image = image
                        .fit_to_exact_size(egui::Vec2::splat(size))
                        .rotate(angle, egui::Vec2::splat(0.5));
                    demo::base(reg, ui, "loading indicator", |ui| ui.add(image));
                }
                demo::base(reg, ui, "Label · animated icon", |ui| {
                    ui.label(egui::RichText::new(label).small())
                });
            });
        })
        .response
    });
}
