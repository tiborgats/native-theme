//! Icons (spec §10.4's palette table), laid out as the gpui showcase's Icons page
//! (`connectors/native-theme-gpui/examples/showcase-gpui/pages/icons.rs:14-276`): the chosen icon
//! theme's folder icon at each size `defaults.icon_sizes` states, the loading indicator the theme
//! ships, every icon of the theme above its role's name, and `ui.image` — an image carries no
//! theme, so every item is in the base style.

use native_theme::icons::FreedesktopLoader;
use native_theme::theme::{IconRole, IconSet, TransformAnimation};
use native_theme_egui::icons::{self, IconContext};
use native_theme_egui::{ThemeAtlas, border_color, border_radius};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

/// The contexts `defaults.icon_sizes` names, in the gpui showcase's order and by its names
/// (`showcase-gpui/demo.rs:4981-5000`).
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
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let (set, icon_theme) = chosen;
    let set_label = set_label(*set, icon_theme.as_deref());

    // The folder icon at each context's size, above the context's name
    // (`showcase-gpui/demo.rs:5051-5093`); where the set has no folder icon, the name alone.
    caption(reg, ui, "Icon Sizes");
    // One height for every cell, its content from the bottom, so the names share a line, as
    // gpui's `items_end` row has them (`showcase-gpui/pages/icons.rs:28`).
    let tallest = CONTEXTS
        .iter()
        .map(|(context, _)| icons::icon_size(t, *context))
        .fold(f32::NEG_INFINITY, f32::max);
    ui.horizontal_wrapped(|ui| {
        for (context, name) in CONTEXTS {
            let size = icons::icon_size(t, context);
            let image = demo::role_image(
                ui,
                IconRole::FolderClosed,
                *set,
                icon_theme.as_deref(),
                size,
            );
            let width = size.max(small_width(ui, name));
            let layout = egui::Layout::bottom_up(egui::Align::Center);
            cell(ui, width, tallest, layout, |ui| {
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
    icons_indicator(reg, atlas, ui, chosen, &set_label);

    // Every role's icon at `defaults.icon_sizes.small` above the role's name
    // (`showcase-gpui/pages/icons.rs:118-196`); where the set has none, gpui's placeholder: a
    // square in `skeleton` — `button.background_color` (`connectors/native-theme-gpui/src/colors.rs:179`,
    // `:550`) — at the theme's radius, never another set's icon.
    let size = icons::icon_size(t, IconContext::Small);
    let images: Vec<(IconRole, Option<egui::Image<'static>>)> = IconRole::ALL
        .iter()
        .map(|role| {
            let image = demo::role_image(ui, *role, *set, icon_theme.as_deref(), size);
            (*role, image)
        })
        .collect();
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
    let skeleton = native_theme_egui::convert::to_color32(t.button.background_color);
    let radius = border_radius(t);
    ui.horizontal_wrapped(|ui| {
        for (role, image) in images {
            let name = format!("{role:?}");
            let width = size.max(small_width(ui, &name));
            let layout = egui::Layout::top_down(egui::Align::Center);
            cell(ui, width, size, layout, |ui| {
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
                        reg.amend_last(|i| {
                            i.read.push((
                                "button.background_color",
                                t.button.background_color.to_string(),
                            ));
                        });
                    }
                }
                demo::base(reg, ui, "Label · icon name", |ui| {
                    ui.label(egui::RichText::new(name).small())
                });
            });
        }
    });

    let toolbar = icons::icon_size(t, IconContext::Toolbar);
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
/// `showcase-gpui/demo.rs:4916-4936`), laid out by `layout`.
fn cell(
    ui: &mut egui::Ui,
    width: f32,
    icon: f32,
    layout: egui::Layout,
    add: impl FnOnce(&mut egui::Ui),
) {
    let height = icon + ui.spacing().item_spacing.y + ui.text_style_height(&egui::TextStyle::Small);
    let size = egui::vec2(width, height);
    // The whole cell is used, so the row advances past it whatever the cell holds.
    ui.allocate_ui_with_layout(size, layout, |ui| {
        ui.set_min_size(size);
        add(ui);
    });
}

/// The icon theme as the gpui showcase's Icons page names it (`showcase-gpui/app.rs:612-625`): a
/// freedesktop set with the theme its icons load from — where that is the system's and it
/// cannot be detected, why — else the set's name.
fn set_label(set: IconSet, icon_theme: Option<&str>) -> String {
    if set != IconSet::Freedesktop {
        return set.name().to_string();
    }
    match icon_theme {
        Some(theme) => format!("freedesktop ({theme})"),
        None => match native_theme::theme::system_icon_theme() {
            Ok(theme) => format!("freedesktop ({theme})"),
            Err(error) => format!("freedesktop (unavailable: {error})"),
        },
    }
}

/// Whether `set` is this platform's own, or bundled (`showcase-gpui/support.rs:413-421`).
fn native_here(set: IconSet) -> bool {
    match set {
        IconSet::Freedesktop => cfg!(target_os = "linux"),
        IconSet::SfSymbols => cfg!(any(target_os = "macos", target_os = "ios")),
        IconSet::SegoeIcons => cfg!(target_os = "windows"),
        _ => true,
    }
}

/// The loading indicator the chosen icon theme ships (§10.4, the Icons row), in a card framed
/// as the gpui showcase's `demo_frame` in `defaults.border` and padded by
/// `layout.container_margin`, at `defaults.icon_sizes.dialog`, above what it is — the gpui
/// showcase's animated-icon card (`showcase-gpui/demo.rs:5120-5178`); where the theme ships
/// none, gpui's caption.
fn icons_indicator(
    reg: &mut Registry,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
    set_label: &str,
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let (set, icon_theme) = chosen;
    let indicator = match set {
        IconSet::Freedesktop => FreedesktopLoader::load_indicator(icon_theme.as_deref()),
        set => native_theme::icons::load_icon_indicator(*set),
    };
    let reduce_motion = atlas.accessibility().reduce_motion;
    let Some(anim) = indicator else {
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
    let mut key = icons::IconKey::name("indicator", *set);
    if *set != IconSet::Freedesktop {
        key = key.tint(ui.visuals().text_color());
    }
    let size = icons::icon_size(t, IconContext::Dialog);
    let frame = icons::animated_frame_index(ui.ctx(), &anim, reduce_motion)
        .and_then(|i| anim.frame_list()?.get(i))
        .unwrap_or_else(|| anim.first_frame());
    // `spin_angle` is `None` for a frame animation and under reduced motion: no rotation.
    let angle = icons::spin_angle(ui.ctx(), &anim, reduce_motion).unwrap_or_default();
    let margin = atlas.layout().container_margin.unwrap_or_default();
    let card = egui::Frame::NONE
        .stroke(egui::Stroke::new(
            t.defaults.border.line_width,
            border_color(t),
        ))
        .corner_radius(border_radius(t))
        .inner_margin(margin);
    demo::contained(reg, ui, "animated icon card", |ui, reg| {
        card.show(ui, |ui| {
            let width = size.max(small_width(ui, &label));
            let layout = egui::Layout::top_down(egui::Align::Center);
            cell(ui, width, size, layout, |ui| {
                if let Some(image) = icons::to_image(ui.ctx(), &key, frame) {
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
    reg.amend_last(|i| {
        i.read.extend([
            ("defaults.border.color", t.defaults.border.color.to_string()),
            (
                "defaults.border.line_width",
                t.defaults.border.line_width.to_string(),
            ),
            (
                "layout.container_margin",
                atlas
                    .layout()
                    .container_margin
                    .map_or_else(|| "not stated: no padding".to_string(), |m| m.to_string()),
            ),
        ]);
    });
}
