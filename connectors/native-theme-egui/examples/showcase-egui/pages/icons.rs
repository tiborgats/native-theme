//! Icons (spec §10.4's palette table): every icon of the chosen icon theme at each size
//! `defaults.icon_sizes` states, `ui.image`, and the loading indicator the theme ships — an
//! image carries no theme, so every item is in the base style.

use native_theme::icons::FreedesktopLoader;
use native_theme::theme::{IconRole, IconSet};
use native_theme_egui::ThemeAtlas;
use native_theme_egui::icons::{self, IconContext};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

const CONTEXTS: [IconContext; 5] = [
    IconContext::Small,
    IconContext::Toolbar,
    IconContext::Panel,
    IconContext::Dialog,
    IconContext::Large,
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
    let mut sizes: Vec<f32> = CONTEXTS.iter().map(|c| icons::icon_size(t, *c)).collect();
    sizes.sort_by(f32::total_cmp);
    sizes.dedup();

    caption(reg, ui, "The loading indicator");
    icons_indicator(reg, atlas, ui, chosen);

    if let Some(image) = demo::role_image(
        ui,
        IconRole::ActionSave,
        *set,
        icon_theme.as_deref(),
        icons::icon_size(t, IconContext::Toolbar),
    ) {
        caption(reg, ui, "ui.image");
        let source = image.source(ui.ctx());
        demo::base(reg, ui, "ui.image", |ui| ui.image(source));
    }

    for size in sizes {
        caption(reg, ui, &format!("Every icon at {size} points"));
        ui.horizontal_wrapped(|ui| {
            for role in IconRole::ALL {
                match demo::role_image(ui, role, *set, icon_theme.as_deref(), size) {
                    Some(image) => {
                        demo::base(reg, ui, "Image", |ui| ui.add(image));
                    }
                    None => {
                        demo::base(reg, ui, "Image (absent)", |ui| {
                            ui.weak(format!("{}: not in this set", role.name()))
                        });
                    }
                }
            }
        });
    }
}

/// The loading indicator the chosen icon theme ships (§10.4, the Icons row).
fn icons_indicator(
    reg: &mut Registry,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let (set, icon_theme) = chosen;
    let indicator = match set {
        IconSet::Freedesktop => FreedesktopLoader::load_indicator(icon_theme.as_deref()),
        set => native_theme::icons::load_icon_indicator(*set),
    };
    let Some(anim) = indicator else {
        demo::base(reg, ui, "loading indicator (absent)", |ui| {
            ui.label("loading indicator: not in this set")
        });
        return;
    };
    let reduce_motion = atlas.accessibility().reduce_motion;
    let mut key = icons::IconKey::name("indicator", *set);
    if *set != IconSet::Freedesktop {
        key = key.tint(ui.visuals().text_color());
    }
    let size = egui::Vec2::splat(icons::icon_size(t, icons::IconContext::Toolbar));
    let frame = icons::animated_frame_index(ui.ctx(), &anim, reduce_motion)
        .and_then(|i| anim.frame_list()?.get(i))
        .unwrap_or_else(|| anim.first_frame());
    // `spin_angle` is `None` for a frame animation and under reduced motion: no rotation.
    let angle = icons::spin_angle(ui.ctx(), &anim, reduce_motion).unwrap_or_default();
    if let Some(image) = icons::to_image(ui.ctx(), &key, frame) {
        let image = image
            .fit_to_exact_size(size)
            .rotate(angle, egui::Vec2::splat(0.5));
        demo::base(reg, ui, "loading indicator", |ui| ui.add(image));
    }
}
