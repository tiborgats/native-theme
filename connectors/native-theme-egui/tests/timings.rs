//! Wall-clock timings of the connector's theme switch, `from_preset(key, is_dark, prefs)`, for
//! every bundled preset in both modes, of a pass of role scopes with a focused widget, of a
//! pass of custom icons, and of passes of the companion crate's switches, hyperlinks, expanders
//! and spinner. Ignored by default; run on the release profile, before and after a perf fix:
//!
//! `cargo test -p native-theme-egui --release --all-features --test timings -- --ignored --nocapture --test-threads=1`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

use std::borrow::Cow;
use std::time::{Duration, Instant};

use native_theme::theme::{IconSet, Theme};
use native_theme_egui::icons::{IconKey, custom_icon_to_image_source};
use native_theme_egui::{
    AccessibilityPreferences, IconProvider, IconRole, NativeThemeUiExt, Role, RoleVariant,
    ThemeAtlas,
};
use native_theme_egui_widgets::expander::Expander;
use native_theme_egui_widgets::links;
use native_theme_egui_widgets::spinner::Spinner;
use native_theme_egui_widgets::switch::Switch;

fn median<T>(runs: usize, mut f: impl FnMut() -> T) -> Duration {
    let mut times: Vec<Duration> = (0..runs)
        .map(|_| {
            let start = Instant::now();
            std::hint::black_box(f());
            start.elapsed()
        })
        .collect();
    times.sort();
    times.get(runs / 2).copied().unwrap_or_default()
}

#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn from_preset_every_bundled_preset() {
    let prefs = AccessibilityPreferences::default();
    for info in Theme::list_presets() {
        let key = info.key;
        let start = Instant::now();
        let first = native_theme_egui::from_preset(key, false, &prefs);
        let cold = start.elapsed();
        let light = median(21, || native_theme_egui::from_preset(key, false, &prefs));
        let dark = median(21, || native_theme_egui::from_preset(key, true, &prefs));
        println!(
            "timing from_preset {key}: first {cold:?} (ok: {}), light {light:?}, dark {dark:?}",
            first.is_ok()
        );
    }
}

/// A `Context` with kde-breeze's light atlas installed and one pass run.
fn installed_context() -> (egui::Context, ThemeAtlas) {
    let prefs = AccessibilityPreferences::default();
    let (atlas, _) = native_theme_egui::from_preset("kde-breeze", false, &prefs).unwrap();
    let ctx = egui::Context::default();
    atlas.install(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
    (ctx, atlas)
}

/// M7-03: a pass of `scopes` sibling `native_scope`s and one focused button, the ring's
/// bookkeeping (`record_scope` per scope, `paint_focus_ring` at the end of the pass) included.
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn native_scopes_per_pass_with_a_focused_widget() {
    for scopes in [100, 1000] {
        let (ctx, _atlas) = installed_context();
        let mut pass = || {
            ctx.run_ui(egui::RawInput::default(), |ui| {
                for _ in 0..scopes {
                    ui.native_scope(Role::Button, RoleVariant::Normal, |_| ());
                }
                ui.button("focused").request_focus();
            })
        };
        let _ = pass();
        assert!(
            ctx.memory(|m| m.focused()).is_some(),
            "the button is focused"
        );
        let time = median(21, &mut pass);
        println!("timing native_scope pass: {scopes} scopes, focused, {time:?}");
    }
}

/// A provider with no name in any set and a Material role's bundled SVG as its own.
#[derive(Debug)]
struct OwnSvg(IconRole);

impl IconProvider for OwnSvg {
    fn icon_name(&self, _: IconSet) -> Option<&str> {
        None
    }
    fn icon_svg(&self, _: IconSet) -> Option<Cow<'static, [u8]>> {
        self.0.icon_svg(IconSet::Material)
    }
}

/// M7-13: one pass's `custom_icon_to_image_source` calls for every `IconRole::ALL` provider
/// under a freedesktop key in the atlas's icon theme: a freedesktop hit, a lookup and a file
/// read; and for providers with only their own SVG, coloured and hashed in memory.
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn custom_icons_per_pass() {
    let (ctx, atlas) = installed_context();
    let icon_theme = atlas.icon_theme(ctx.theme()).unwrap().to_owned();
    let named: Vec<(IconRole, IconKey)> = IconRole::ALL
        .iter()
        .filter_map(|&role| {
            let name = role.icon_name(IconSet::Freedesktop)?;
            Some((
                role,
                IconKey::name(name, IconSet::Freedesktop).icon_theme(&icon_theme),
            ))
        })
        .collect();
    let hits = named
        .iter()
        .filter(|(role, key)| custom_icon_to_image_source(&ctx, role, key).is_some())
        .count();
    let freedesktop = median(21, || {
        named
            .iter()
            .filter_map(|(role, key)| custom_icon_to_image_source(&ctx, role, key))
            .count()
    });
    println!("timing custom icons: {hits} freedesktop hits in {icon_theme}, {freedesktop:?}");
    let own: Vec<(OwnSvg, IconKey)> = IconRole::ALL
        .iter()
        .filter(|role| role.icon_svg(IconSet::Material).is_some())
        .map(|&role| {
            (
                OwnSvg(role),
                IconKey::name(role.name(), IconSet::Freedesktop).icon_theme(&icon_theme),
            )
        })
        .collect();
    let provider = median(21, || {
        own.iter()
            .filter_map(|(svg, key)| custom_icon_to_image_source(&ctx, svg, key))
            .count()
    });
    println!(
        "timing custom icons: {} provider SVGs, {provider:?}",
        own.len()
    );
}

/// M7-22: a pass of 200 labelled `Switch`es, each recording its `Parts`.
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn switches_per_pass() {
    let (ctx, _atlas) = installed_context();
    let mut on = [false; 200];
    let mut pass = || {
        ctx.run_ui(egui::RawInput::default(), |ui| {
            for on in &mut on {
                ui.add(Switch::new(on).label("Wi-Fi"));
            }
        })
    };
    let _ = pass();
    let time = median(21, &mut pass);
    println!("timing switches pass: {} switches, {time:?}", on.len());
}

/// M7-23: a pass of 100 `hyperlink`s, each clicked once before, so the visited set holds 100
/// URLs.
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn hyperlinks_per_pass_with_visited_urls() {
    let (ctx, _atlas) = installed_context();
    let urls: Vec<String> = (0..100)
        .map(|i| format!("https://example.org/{i}"))
        .collect();
    let mut rects = Vec::new();
    let mut clicked = 0;
    let pass = |input: egui::RawInput, rects: &mut Vec<egui::Rect>, clicked: &mut usize| {
        ctx.run_ui(input, |ui| {
            rects.clear();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for url in &urls {
                    let r = ui.add(links::hyperlink(ui, "Docs", url));
                    *clicked += usize::from(r.clicked());
                    rects.push(r.rect);
                }
            });
        })
    };
    let _ = pass(egui::RawInput::default(), &mut rects, &mut clicked);
    for i in 0..urls.len() {
        let pos = rects[i].center();
        for pressed in [true, false] {
            let input = egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                ..Default::default()
            };
            let _ = pass(input, &mut rects, &mut clicked);
        }
    }
    let gone = || egui::RawInput {
        events: vec![egui::Event::PointerGone],
        ..Default::default()
    };
    let _ = pass(gone(), &mut rects, &mut clicked);
    let time = median(21, || pass(gone(), &mut rects, &mut clicked));
    println!(
        "timing hyperlinks pass: {} hyperlinks, {clicked} visited, {time:?}",
        urls.len()
    );
}

/// M7-24: a pass of 100 closed `Expander`s.
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn expanders_per_pass() {
    let (ctx, _atlas) = installed_context();
    let mut pass = || {
        ctx.run_ui(egui::RawInput::default(), |ui| {
            for i in 0..100 {
                let _ = Expander::new("Details").id_salt(i).show(ui, |_| ());
            }
        })
    };
    let _ = pass();
    let time = median(21, &mut pass);
    println!("timing expanders pass: 100 expanders, {time:?}");
}

/// M7-25: a pass of one `Spinner` drawing Material's animated indicator, a frame's
/// `to_image_source` included, against the same pass with no spinner.
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn material_spinner_per_pass() {
    let prefs = AccessibilityPreferences::default();
    let (_, resolved) = native_theme_egui::from_preset("kde-breeze", false, &prefs).unwrap();
    let ctx = egui::Context::default();
    egui_extras::install_image_loaders(&ctx);
    ThemeAtlas::builder("timings", &resolved, &resolved)
        .icon_set(IconSet::Material)
        .build()
        .install(&ctx);
    let mut time = 0.0;
    let mut pass = |spinner: bool| {
        time += 0.05;
        let input = egui::RawInput {
            time: Some(time),
            ..Default::default()
        };
        ctx.run_ui(input, |ui| {
            if spinner {
                ui.add(Spinner::new());
            }
        })
    };
    let _ = pass(true);
    let _ = pass(true);
    let images = pass(true)
        .shapes
        .iter()
        .filter(|s| s.shape.texture_id() != egui::TextureId::default())
        .count();
    let with = median(201, || pass(true));
    let without = median(201, || pass(false));
    println!(
        "timing material spinner pass: {images} textured shape, {with:?}; empty pass {without:?}"
    );
}
