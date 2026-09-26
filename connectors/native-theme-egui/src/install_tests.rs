//! §13 T5, T6, T8, T13–T16 and T18 land here task by task. Task 11: the atlas's own
//! accessors (T18 (a) among them), the builder's defaults and `role_modifier`'s merge.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use crate::{
    AccessibilityPreferences, ColorMode, IconSet, LayoutTheme, ResolvedTheme, Role, RoleVariant,
    Surface, ThemeAtlas,
};
use native_theme::theme::Theme;

/// A bundled preset resolved for one mode, by the route the iced connector's `from_preset`
/// takes (`connectors/native-theme-iced/src/lib.rs:259-266`).
pub(crate) fn resolved(name: &str, mode: ColorMode) -> ResolvedTheme {
    Theme::preset(name)
        .unwrap()
        .into_variant(mode)
        .unwrap()
        .resolve_system()
        .unwrap()
}

/// One pass of `f` on `ctx`; the texture delta is dropped unapplied, as egui's own tests do
/// (`egui/src/data/output.rs:74`), so a dropped `FullOutput` never trips epaint's debug assert.
pub(crate) fn pass(
    ctx: &egui::Context,
    input: egui::RawInput,
    f: impl FnMut(&mut egui::Ui),
) -> egui::FullOutput {
    let mut out = ctx.run_ui(input, f);
    out.textures_delta.clear();
    out
}

/// T18 (a), and every other accessor: the atlas reports what the builder was given, per theme.
#[test]
fn the_atlas_reports_what_the_builder_was_given() {
    let light = resolved("kde-breeze", ColorMode::Light);
    let dark = resolved("adwaita", ColorMode::Dark);
    let layout = LayoutTheme {
        widget_gap: Some(6.0),
        ..LayoutTheme::default()
    };
    let prefs = AccessibilityPreferences {
        text_scaling_factor: 1.5,
        ..AccessibilityPreferences::default()
    };
    let atlas = ThemeAtlas::builder("Mixed", &light, &dark)
        .layout(&layout)
        .accessibility(&prefs)
        .os_mode(ColorMode::Dark)
        .icon_set(IconSet::Lucide)
        .icon_theme(egui::Theme::Light, "breeze")
        .icon_theme(egui::Theme::Dark, "breeze-dark")
        .build();
    assert_eq!(atlas.name(), "Mixed");
    assert_eq!(atlas.resolved_for(egui::Theme::Light), &light);
    assert_eq!(atlas.resolved_for(egui::Theme::Dark), &dark);
    assert_eq!(atlas.accessibility(), &prefs);
    assert_eq!(atlas.layout(), &layout);
    assert_eq!(atlas.os_mode(), Some(ColorMode::Dark));
    assert_eq!(atlas.icon_set(), IconSet::Lucide);
    assert_eq!(atlas.icon_theme(egui::Theme::Light), Some("breeze"));
    assert_eq!(atlas.icon_theme(egui::Theme::Dark), Some("breeze-dark"));
}

/// §4.2, §4.3: the documented defaults of every optional input.
#[test]
fn the_builder_defaults_are_the_documented_ones() {
    let light = resolved("kde-breeze", ColorMode::Light);
    let atlas = ThemeAtlas::builder("Breeze", &light, &light).build();
    assert_eq!(atlas.accessibility(), &AccessibilityPreferences::default());
    assert_eq!(atlas.layout(), &LayoutTheme::default());
    assert_eq!(atlas.os_mode(), None);
    assert_eq!(atlas.icon_set(), native_theme::theme::system_icon_set());
    assert_eq!(atlas.icon_theme(egui::Theme::Light), None);
    assert_eq!(atlas.icon_theme(egui::Theme::Dark), None);
    // No conversion runs on kde-breeze that emits a note; a later task that makes one emit
    // here changes this line with its reason.
    assert!(atlas.notes().is_empty(), "{:?}", atlas.notes());
    let again = ThemeAtlas::builder("Breeze", &light, &light).build();
    assert_eq!(atlas.notes(), again.notes());
    assert_eq!(atlas.clone().name(), atlas.name());
}

/// §4.2's auto-trait claim: the atlas can be built on a watcher thread and published into
/// `Context::data_mut`.
#[test]
fn the_atlas_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<ThemeAtlas>();
}

/// `surface_frame` is total over the eleven surfaces and both themes, and every frame equals
/// itself (no `NaN`); Tasks 18 and 33 pin the values.
#[test]
fn surface_frame_is_total_over_both_themes() {
    let light = resolved("kde-breeze", ColorMode::Light);
    let dark = resolved("kde-breeze", ColorMode::Dark);
    let atlas = ThemeAtlas::builder("Breeze", &light, &dark).build();
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        for surface in Surface::all() {
            let frame = atlas.surface_frame(theme, *surface);
            assert_eq!(frame, frame, "{theme:?} {surface:?}");
        }
    }
}

/// §4.2 (`role_modifier`), §7.5: the modifier replaces the whole style and carries the
/// application's `TextStyle::Name` keys into the replacement.
#[test]
fn role_modifier_replaces_the_style_and_carries_the_applications_name_keys() {
    let light = resolved("kde-breeze", ColorMode::Light);
    let atlas = ThemeAtlas::builder("Breeze", &light, &light).build();
    let key = egui::TextStyle::Name("app-caption".into());
    let font = egui::FontId::proportional(11.0);
    let mut style = egui::Style::default();
    style.text_styles.insert(key.clone(), font.clone());
    style.explanation_tooltips = true; // a field the atlas never writes (§5.10)
    atlas
        .role_modifier(egui::Theme::Light, Role::Button, RoleVariant::Normal)
        .apply(&mut style);
    assert_eq!(
        style.text_styles.get(&key),
        Some(&font),
        "the Name key survived"
    );
    assert!(!style.explanation_tooltips, "the whole style was replaced");
    // With no Name key the replacement is the cell as it is: the five stock keys and no other.
    let mut plain = egui::Style::default();
    atlas
        .role_modifier(egui::Theme::Light, Role::Button, RoleVariant::Normal)
        .apply(&mut plain);
    assert_eq!(plain.text_styles.len(), 5);
}

/// §4.5, §7.5: `carry_name_keys` hands the cell's `Arc` over unchanged when the style it
/// merges from holds no `Name` key the cell lacks, and copies only otherwise.
#[test]
fn carry_name_keys_copies_only_when_a_name_key_is_missing() {
    let light = resolved("kde-breeze", ColorMode::Light);
    let atlas = ThemeAtlas::builder("Breeze", &light, &light).build();
    let cell = atlas
        .scheme(egui::Theme::Light)
        .cell(Role::Button, RoleVariant::Normal);
    let plain = egui::Style::default();
    assert!(std::sync::Arc::ptr_eq(
        &crate::atlas::carry_name_keys(&plain, cell),
        cell
    ));
    let key = egui::TextStyle::Name("app-caption".into());
    let mut named = egui::Style::default();
    named
        .text_styles
        .insert(key.clone(), egui::FontId::proportional(11.0));
    let merged = crate::atlas::carry_name_keys(&named, cell);
    assert!(!std::sync::Arc::ptr_eq(&merged, cell));
    assert_eq!(
        merged.text_styles.get(&key),
        Some(&egui::FontId::proportional(11.0))
    );
    assert_eq!(merged.text_styles.len(), 6);
}

use std::sync::Arc;

use crate::NativeThemeUiExt;

/// A preset's atlas through the public builder (`from_preset` lands in Task 24), both variants
/// through `resolved` above, named as the preset names itself (`native-theme/src/model/mod.rs:257`).
pub(crate) fn preset_atlas(id: &str) -> ThemeAtlas {
    let name = native_theme::theme::Theme::preset(id)
        .expect("a bundled preset")
        .name;
    ThemeAtlas::builder(
        &name,
        &resolved(id, ColorMode::Light),
        &resolved(id, ColorMode::Dark),
    )
    .build()
}

/// T14's `Context`: empty font definitions, so no face is parsed and no glyph laid out
/// (`epaint/src/text/fonts.rs:561-570`, `:640-648`).
pub(crate) fn bare_context() -> egui::Context {
    let ctx = egui::Context::default();
    ctx.set_fonts(egui::FontDefinitions::empty());
    ctx
}

/// T14 (a), less its font-plan clause (Task 22) and its raster-icon clause (Task 26).
#[test]
fn install_reaches_both_schemes_and_a_second_install_replaces_the_first() {
    let ctx = bare_context();
    let breeze = preset_atlas("kde-breeze");
    breeze.install(&ctx);
    for n in 0..3 {
        let _ = pass(&ctx, egui::RawInput::default(), |ui| {
            ui.label("pass");
        });
        for theme in [egui::Theme::Dark, egui::Theme::Light] {
            // A fresh `Context` holds no `TextStyle::Name` key, so the atlas's own `Arc` is
            // published unchanged (§10.3 step 2).
            assert!(
                Arc::ptr_eq(&ctx.style_of(theme), &breeze.scheme(theme).base),
                "{theme:?} base style is not the atlas's after pass {n}"
            );
        }
    }
    let adwaita = preset_atlas("adwaita");
    adwaita.install(&ctx);
    for theme in [egui::Theme::Dark, egui::Theme::Light] {
        assert!(Arc::ptr_eq(
            &ctx.style_of(theme),
            &adwaita.scheme(theme).base
        ));
        assert!(!Arc::ptr_eq(
            &ctx.style_of(theme),
            &breeze.scheme(theme).base
        ));
    }
    let found = ThemeAtlas::from_ctx(&ctx).expect("an atlas is installed");
    assert_eq!(found.name(), adwaita.name());
    ThemeAtlas::clear(&ctx);
    assert!(ThemeAtlas::from_ctx(&ctx).is_none());
}

/// T14 (d). A zero-delay request is served over two passes (`egui/src/context.rs:110-115`,
/// `:138-141`), so the loop runs until egui reports none outstanding.
#[test]
fn install_requests_a_repaint() {
    let ctx = bare_context();
    let atlas = preset_atlas("kde-breeze");
    atlas.install(&ctx);
    let mut settled = false;
    for _ in 0..10 {
        let _ = pass(&ctx, egui::RawInput::default(), |_ui| {});
        if !ctx.has_requested_repaint() {
            settled = true;
            break;
        }
    }
    assert!(
        settled,
        "a pass that shows nothing must stop asking for repaints"
    );
    atlas.install(&ctx);
    assert!(
        ctx.has_requested_repaint(),
        "install ends with a repaint request (§10.3 step 6)"
    );
}

/// Review Focus 1: with no atlas, and again after `clear`, every seam is egui's own and nothing
/// panics (§4.5). `role_modifier` needs an atlas to be called on, so it has no no-atlas path.
#[test]
fn without_an_atlas_every_seam_is_egui_s_own() {
    let ctx = bare_context();
    let check = |ctx: &egui::Context| {
        assert!(ThemeAtlas::from_ctx(ctx).is_none());
        let _ = pass(ctx, egui::RawInput::default(), |ui| {
            let parent = Arc::clone(ui.style());
            let same = ui
                .native_scope(Role::Button, RoleVariant::Normal, |child| {
                    Arc::ptr_eq(child.style(), &parent)
                })
                .inner;
            assert!(same, "native_scope without an atlas is a plain scope");
            ui.native_set_style(Role::Sidebar, RoleVariant::Normal);
            assert!(
                Arc::ptr_eq(ui.style(), &parent),
                "native_set_style without an atlas is a no-op"
            );
            assert_eq!(
                ui.native_frame(Surface::Window),
                egui::Frame::window(ui.style())
            );
            assert_eq!(
                ui.native_frame(Surface::Card),
                egui::Frame::group(ui.style())
            );
            assert_eq!(
                ui.native_frame(Surface::CentralPanel),
                egui::Frame::central_panel(ui.style())
            );
        });
    };
    check(&ctx);
    preset_atlas("kde-breeze").install(&ctx);
    ThemeAtlas::clear(&ctx);
    check(&ctx);
}

/// Review Focus 2: nested scopes — the inner role inside, the outer role again after.
#[test]
fn a_nested_scope_takes_the_inner_role_and_gives_the_outer_back() {
    let ctx = bare_context();
    let atlas = preset_atlas("kde-breeze");
    atlas.install(&ctx);
    let _ = pass(&ctx, egui::RawInput::default(), |ui| {
        let theme = ui.ctx().theme();
        let sidebar = atlas.scheme(theme).cell(Role::Sidebar, RoleVariant::Normal);
        let button = atlas.scheme(theme).cell(Role::Button, RoleVariant::Normal);
        assert!(
            !Arc::ptr_eq(sidebar, button),
            "the two cells differ, or the test proves nothing"
        );
        ui.native_scope(Role::Sidebar, RoleVariant::Normal, |outer| {
            assert!(
                Arc::ptr_eq(outer.style(), sidebar),
                "the outer scope holds the Sidebar cell"
            );
            outer.native_scope(Role::Button, RoleVariant::Normal, |inner| {
                assert!(
                    Arc::ptr_eq(inner.style(), button),
                    "the inner scope holds the Button cell"
                );
            });
            assert!(
                Arc::ptr_eq(outer.style(), sidebar),
                "the outer Ui keeps its cell after the inner scope"
            );
        });
    });
}
