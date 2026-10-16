//! Wall-clock timings of the connector's theme switch, `from_preset(key, is_dark, prefs)`, for
//! every bundled preset in both modes, of a pass of role scopes with a focused widget, and of a
//! pass of custom icons. Ignored by default; run on the release profile, before and after a perf
//! fix:
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
        assert!(ctx.memory(|m| m.focused()).is_some(), "the button is focused");
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
    println!("timing custom icons: {} provider SVGs, {provider:?}", own.len());
}
