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

use egui::epaint::RectShape;

/// `preset_atlas` with an OS colour mode (`Builder::os_mode`, §4.3); `pub(crate)` for Task 24's
/// constructor tests.
pub(crate) fn preset_atlas_with_os_mode(id: &str, mode: ColorMode) -> ThemeAtlas {
    let name = native_theme::theme::Theme::preset(id)
        .expect("a bundled preset")
        .name;
    ThemeAtlas::builder(
        &name,
        &resolved(id, ColorMode::Light),
        &resolved(id, ColorMode::Dark),
    )
    .os_mode(mode)
    .build()
}

/// Copies, at the end of each pass, the shapes of the layers the test names. Plugins run in
/// the order they were added (`egui/src/context.rs:2043`), so a probe added after `install`
/// runs after the install plugin and sees its ring; `end_pass` drains the layers only
/// afterwards (`egui/src/layers.rs:213`).
#[derive(Default)]
struct ProbeState {
    layers: Vec<egui::LayerId>,
    seen: Vec<(egui::LayerId, egui::Shape)>,
}

#[derive(Clone, Default)]
struct LayerProbe(Arc<std::sync::Mutex<ProbeState>>);

impl egui::plugin::Plugin for LayerProbe {
    fn debug_name(&self) -> &'static str {
        "native-theme-egui test layer probe"
    }

    fn on_end_pass(&mut self, ui: &mut egui::Ui) {
        let mut state = self.0.lock().expect("the probe's lock");
        let layers = state.layers.clone();
        state.seen = ui.ctx().graphics(|graphics| {
            let mut seen = Vec::new();
            for layer in layers {
                if let Some(list) = graphics.get(layer) {
                    seen.extend(
                        list.all_entries()
                            .map(|clipped| (layer, clipped.shape.clone())),
                    );
                }
            }
            seen
        });
    }
}

fn watch(probe: &LayerProbe, layers: &[egui::LayerId]) {
    probe.0.lock().expect("the probe's lock").layers = layers.to_vec();
}

fn seen(probe: &LayerProbe) -> Vec<(egui::LayerId, egui::Shape)> {
    probe.0.lock().expect("the probe's lock").seen.clone()
}

/// The ring shapes among `seen` on `layer`: rect strokes in the ring's stroke, unfilled and
/// painted outside their rect (`epaint/src/shapes/rect_shape.rs:114-122`).
fn rings_on(
    seen: &[(egui::LayerId, egui::Shape)],
    layer: egui::LayerId,
    stroke: egui::Stroke,
) -> Vec<RectShape> {
    seen.iter()
        .filter(|(on, _)| *on == layer)
        .filter_map(|(_, shape)| match shape {
            egui::Shape::Rect(rect)
                if rect.stroke == stroke
                    && rect.fill == egui::Color32::TRANSPARENT
                    && rect.stroke_kind == egui::StrokeKind::Outside =>
            {
                Some(rect.clone())
            }
            _ => None,
        })
        .collect()
}

/// A widget's radius grown by the ring's offset — the concentric inner edge §6.18 states,
/// recomputed here from the leaves so the test does not lean on the plugin's own helper.
fn grown(radius: egui::CornerRadius, by: f32) -> egui::CornerRadius {
    let g = |c: u8| crate::convert::u8_from_f32_saturating(f32::from(c) + by);
    egui::CornerRadius {
        nw: g(radius.nw),
        ne: g(radius.ne),
        sw: g(radius.sw),
        se: g(radius.se),
    }
}

fn tab_press() -> egui::RawInput {
    egui::RawInput {
        events: vec![egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        ..Default::default()
    }
}

fn primary(pos: egui::Pos2, pressed: bool) -> egui::RawInput {
    egui::RawInput {
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
    }
}

fn window_focus(focused: bool) -> egui::RawInput {
    egui::RawInput {
        focused,
        ..Default::default()
    }
}

fn with_system_theme(theme: Option<egui::Theme>) -> egui::RawInput {
    egui::RawInput {
        system_theme: theme,
        ..Default::default()
    }
}

/// One pass through Task 11's `pass`; `show` runs on the root `Ui` and returns the widget
/// under test.
fn pass_ui(
    ctx: &egui::Context,
    input: egui::RawInput,
    show: &mut dyn FnMut(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let mut response = None;
    let _ = pass(ctx, input, |ui| response = Some(show(ui)));
    response.expect("the pass ran the closure")
}

/// `body` inside a window with no title bar — the title bar's collapse button would take the
/// `Tab` first (`egui/src/containers/window.rs:1377-1388`) — and no resize handles.
fn in_window(
    ui: &mut egui::Ui,
    body: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    egui::Window::new("ring")
        .title_bar(false)
        .resizable(false)
        .show(ui.ctx(), body)
        .and_then(|shown| shown.inner)
        .expect("the window shows its widget")
}

/// An installed atlas and a probe registered after it.
fn ring_harness(id: &str) -> (egui::Context, ThemeAtlas, LayerProbe) {
    let ctx = bare_context();
    let atlas = preset_atlas(id);
    atlas.install(&ctx);
    let probe = LayerProbe::default();
    ctx.add_plugin(probe.clone());
    (ctx, atlas, probe)
}

/// A fresh harness, one pass at rest and one with `Tab`: the focused widget's response and the
/// ring shapes on its layer after the second pass.
fn ring_after_tab(
    id: &str,
    show: &mut dyn FnMut(&mut egui::Ui) -> egui::Response,
) -> (ThemeAtlas, egui::Theme, egui::Response, Vec<RectShape>) {
    let (ctx, atlas, probe) = ring_harness(id);
    let theme = ctx.theme();
    let stroke = atlas
        .focus_ring(theme)
        .expect("the preset states a focus ring")
        .stroke;
    let first = pass_ui(&ctx, egui::RawInput::default(), show);
    watch(&probe, &[first.layer_id]);
    let focused = pass_ui(&ctx, tab_press(), show);
    let rings = rings_on(&seen(&probe), first.layer_id, stroke);
    (atlas, theme, focused, rings)
}

/// The first role whose `Normal` cell's active corner radius is not `not`, asserted to exist.
fn role_with_radius_other_than(
    atlas: &ThemeAtlas,
    theme: egui::Theme,
    not: egui::CornerRadius,
) -> (Role, egui::CornerRadius) {
    Role::all()
        .iter()
        .copied()
        .find_map(|role| {
            let radius = atlas
                .scheme(theme)
                .cell(role, RoleVariant::Normal)
                .visuals
                .widgets
                .active
                .corner_radius;
            (radius != not).then_some((role, radius))
        })
        .expect("a role whose active corner radius differs")
}

/// T14 (b), first half: rest, a click, `Tab`, the OS focus lost and regained.
#[test]
fn the_ring_marks_keyboard_focus_only_while_the_window_has_it() {
    let (ctx, atlas, probe) = ring_harness("kde-breeze");
    let theme = ctx.theme();
    let ring = atlas
        .focus_ring(theme)
        .expect("kde-breeze states a focus ring");
    let t = atlas.resolved_for(theme);
    assert_eq!(
        ring.stroke.color,
        crate::convert::to_color32(t.defaults.focus_ring_color)
    );
    assert_eq!(ring.stroke.width, t.defaults.focus_ring_width);
    assert_eq!(ring.offset, t.defaults.focus_ring_offset);
    let base_radius = atlas
        .scheme(theme)
        .base
        .visuals
        .widgets
        .active
        .corner_radius;
    let mut show = |ui: &mut egui::Ui| in_window(ui, |ui| ui.button("focus me"));

    // At rest.
    let button = pass_ui(&ctx, egui::RawInput::default(), &mut show);
    let layer = button.layer_id;
    assert_ne!(
        layer,
        egui::LayerId::background(),
        "a Window paints on a layer of its own"
    );
    watch(&probe, &[layer, egui::LayerId::background()]);
    let button = pass_ui(&ctx, egui::RawInput::default(), &mut show);
    assert!(
        rings_on(&seen(&probe), layer, ring.stroke).is_empty(),
        "no ring at rest"
    );

    // A primary click: pressed in one pass, released in the next.
    let centre = button.rect.center();
    let _ = pass_ui(&ctx, primary(centre, true), &mut show);
    assert!(
        rings_on(&seen(&probe), layer, ring.stroke).is_empty(),
        "no ring while pressed"
    );
    let button = pass_ui(&ctx, primary(centre, false), &mut show);
    assert!(button.clicked(), "the release lands on the button");
    assert!(
        rings_on(&seen(&probe), layer, ring.stroke).is_empty(),
        "no ring after a click"
    );
    let _ = pass_ui(&ctx, egui::RawInput::default(), &mut show);
    assert!(
        rings_on(&seen(&probe), layer, ring.stroke).is_empty(),
        "and none a pass later"
    );

    // Tab: nothing has focus, so the first widget that wants it takes it in this pass
    // (`egui/src/memory/mod.rs:672-678`), and the plugin paints the ring at the pass's end.
    let button = pass_ui(&ctx, tab_press(), &mut show);
    let rings = rings_on(&seen(&probe), layer, ring.stroke);
    assert_eq!(
        rings.len(),
        1,
        "exactly one ring after Tab, on the button's layer"
    );
    assert!(
        rings_on(&seen(&probe), egui::LayerId::background(), ring.stroke).is_empty(),
        "none on the background"
    );
    let painted = rings.first().expect("the one ring");
    assert_eq!(painted.rect, button.rect.expand(ring.offset));
    assert_eq!(painted.corner_radius, grown(base_radius, ring.offset));

    // The window loses the OS keyboard focus: egui keeps the focused widget, the ring goes.
    let button = pass_ui(&ctx, window_focus(false), &mut show);
    assert_eq!(
        ctx.memory(|m| m.focused()),
        Some(button.id),
        "egui still reports the button focused"
    );
    assert!(
        rings_on(&seen(&probe), layer, ring.stroke).is_empty(),
        "no ring without the OS focus"
    );
    let _ = pass_ui(&ctx, window_focus(true), &mut show);
    assert_eq!(
        rings_on(&seen(&probe), layer, ring.stroke).len(),
        1,
        "the ring is back with the focus"
    );
}

/// T14 (b), second half, and Review Focus 2's ring clause: the corners follow the innermost
/// scope, `native_set_style` on a window body, `native_set_style` on the root and a following
/// `reset_style`, and a registered shape. Each case is a fresh harness on `adwaita`.
#[test]
fn the_ring_takes_the_innermost_scope_s_radius() {
    let picker = preset_atlas("adwaita");
    let theme = bare_context().theme();
    let base = picker
        .scheme(theme)
        .base
        .visuals
        .widgets
        .active
        .corner_radius;
    let (role, radius) = role_with_radius_other_than(&picker, theme, base);
    let (outer, outer_radius) = role_with_radius_other_than(&picker, theme, radius);
    assert_ne!(outer_radius, radius);
    let offset = picker
        .focus_ring(theme)
        .expect("adwaita states a focus ring")
        .offset;
    let corner = |rings: &[RectShape]| rings.first().map(|r| r.corner_radius);

    // Inside a scope.
    let (_, _, _, rings) = ring_after_tab("adwaita", &mut |ui| {
        in_window(ui, |ui| {
            ui.native_scope(role, RoleVariant::Normal, |ui| ui.button("b"))
                .inner
        })
    });
    assert_eq!(rings.len(), 1);
    assert_eq!(
        corner(&rings),
        Some(grown(radius, offset)),
        "the scope's corners, not the base's"
    );

    // Nested: the inner scope wins.
    let (_, _, _, rings) = ring_after_tab("adwaita", &mut |ui| {
        in_window(ui, |ui| {
            ui.native_scope(outer, RoleVariant::Normal, |ui| {
                ui.native_scope(role, RoleVariant::Normal, |ui| ui.button("b"))
                    .inner
            })
            .inner
        })
    });
    assert_eq!(
        corner(&rings),
        Some(grown(radius, offset)),
        "the innermost scope's corners"
    );

    // A window body that took the role with `native_set_style`.
    let (_, _, _, rings) = ring_after_tab("adwaita", &mut |ui| {
        in_window(ui, |ui| {
            ui.native_set_style(role, RoleVariant::Normal);
            ui.button("b")
        })
    });
    assert_eq!(corner(&rings), Some(grown(radius, offset)));

    // The root `Ui`: its record never matches (`egui/src/ui.rs:171`), so the ring reads the
    // root's own style at the end of the pass.
    let (_, _, _, rings) = ring_after_tab("adwaita", &mut |ui| {
        ui.native_set_style(role, RoleVariant::Normal);
        ui.button("b")
    });
    assert_eq!(
        corner(&rings),
        Some(grown(radius, offset)),
        "a role left on the root"
    );
    let (_, _, _, rings) = ring_after_tab("adwaita", &mut |ui| {
        ui.native_set_style(role, RoleVariant::Normal);
        ui.reset_style();
        ui.button("b")
    });
    assert_eq!(
        corner(&rings),
        Some(grown(base, offset)),
        "the base style after reset_style"
    );

    // A registered shape takes precedence over the widget's rect and the scope's radius.
    let custom_rect = egui::Rect::from_min_size(egui::pos2(10.0, 10.0), egui::vec2(40.0, 20.0));
    let custom_radius = egui::CornerRadius {
        nw: 3,
        ne: 3,
        sw: 3,
        se: 3,
    };
    let (_, _, _, rings) = ring_after_tab("adwaita", &mut |ui| {
        ui.native_scope(role, RoleVariant::Normal, |ui| {
            let response = ui.button("b");
            crate::register_focus_shape(ui.ctx(), response.id, custom_rect, custom_radius);
            response
        })
        .inner
    });
    assert_eq!(
        rings.first().map(|r| r.rect),
        Some(custom_rect.expand(offset))
    );
    assert_eq!(corner(&rings), Some(grown(custom_radius, offset)));
}

fn set_theme_commands(out: &egui::FullOutput) -> Vec<egui::SystemTheme> {
    out.viewport_output
        .get(&egui::ViewportId::ROOT)
        .map(|vo| {
            vo.commands
                .iter()
                .filter_map(|c| match c {
                    egui::ViewportCommand::SetTheme(theme) => Some(*theme),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

/// One pass: the theme egui drew in, and the `SetTheme` commands the root viewport carries.
fn pass_theme(ctx: &egui::Context, input: egui::RawInput) -> (egui::Theme, Vec<egui::SystemTheme>) {
    let mut drawn = None;
    let out = pass(ctx, input, |ui| drawn = Some(ui.ctx().theme()));
    (drawn.expect("the pass ran"), set_theme_commands(&out))
}

/// T14 (c).
#[test]
fn the_os_mode_fills_a_missing_system_theme_and_the_title_bar_follows() {
    use egui::{SystemTheme, Theme, ThemePreference};

    // A Linux-shaped integration: `system_theme: None` every pass.
    let ctx = bare_context();
    preset_atlas_with_os_mode("kde-breeze", ColorMode::Light).install(&ctx);
    assert_eq!(
        pass_theme(&ctx, with_system_theme(None)),
        (Theme::Light, vec![SystemTheme::Light]),
        "first pass: the OS mode, and the title bar told once"
    );
    assert_eq!(
        pass_theme(&ctx, with_system_theme(None)),
        (Theme::Light, vec![]),
        "second pass: nothing to send"
    );
    assert_eq!(
        pass_theme(&ctx, with_system_theme(Some(Theme::Dark))),
        (Theme::Dark, vec![]),
        "a scheme the integration reports is left alone"
    );
    preset_atlas_with_os_mode("kde-breeze", ColorMode::Dark).install(&ctx);
    assert_eq!(
        pass_theme(&ctx, with_system_theme(None)),
        (Theme::Dark, vec![SystemTheme::Dark]),
        "an atlas with the other mode: sent once"
    );
    ctx.set_theme(Theme::Dark);
    assert_eq!(
        pass_theme(&ctx, with_system_theme(None)),
        (Theme::Dark, vec![SystemTheme::Dark]),
        "a pinned preference: egui's own command passes through"
    );
    ctx.set_theme(ThemePreference::System);
    assert_eq!(
        pass_theme(&ctx, with_system_theme(None)),
        (Theme::Dark, vec![SystemTheme::Dark]),
        "System again: the OS mode, never SystemDefault"
    );

    // An integration that reports the scheme: egui's own `SystemDefault` passes through.
    let ctx = bare_context();
    preset_atlas_with_os_mode("kde-breeze", ColorMode::Light).install(&ctx);
    assert_eq!(
        pass_theme(&ctx, with_system_theme(Some(Theme::Dark))),
        (Theme::Dark, vec![SystemTheme::SystemDefault])
    );

    // No OS mode: egui's fallback (`egui/src/memory/mod.rs:331`) and egui's own command.
    let ctx = bare_context();
    preset_atlas("kde-breeze").install(&ctx);
    assert_eq!(
        pass_theme(&ctx, with_system_theme(None)),
        (Theme::Dark, vec![SystemTheme::SystemDefault])
    );

    // `Options::sync_window_theme` off: no pass carries a `SetTheme`.
    let ctx = bare_context();
    ctx.options_mut(|o| o.sync_window_theme = false);
    preset_atlas_with_os_mode("kde-breeze", ColorMode::Light).install(&ctx);
    for _ in 0..2 {
        assert_eq!(
            pass_theme(&ctx, with_system_theme(None)),
            (Theme::Light, vec![])
        );
    }
}

/// §13 T6 (a)–(e), T5's plan leg and T14 (a)'s font-plan clause (plan Task 22).
mod t6_fonts {
    use std::borrow::Cow;
    use std::sync::Arc;

    use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};
    use native_theme::theme::{ColorMode, FontStyle, ResolvedTheme, Theme};

    use super::{pass, resolved};
    use crate::convert::clamp_length;
    use crate::fonts::{FontBytes, FontPlan, font_definitions, weight_coords};
    use crate::{Note, Role, RoleVariant, ThemeAtlas};

    /// One of egui's bundled faces, `&'static` as `FontDefinitions::default()` registers them
    /// with `FontData::from_static` (`epaint/src/text/fonts.rs:506-532`).
    fn bundled(name: &str) -> &'static [u8] {
        let defs = FontDefinitions::default();
        let data = defs.font_data.get(name).unwrap();
        match &data.font {
            Cow::Borrowed(bytes) => Some(*bytes),
            Cow::Owned(_) => None,
        }
        .unwrap()
    }

    fn hack() -> &'static [u8] {
        bundled("Hack")
    }

    /// `bytes` with the `head` table's `unitsPerEm` overwritten with `0`. OpenType's table
    /// directory holds `numTables` at offset 4 and 16-byte records from offset 12 (`tag`,
    /// `checksum`, `offset`, `length`); in `head`, `unitsPerEm` sits at offset 18, after
    /// `majorVersion`, `minorVersion`, `fontRevision`, `checksumAdjustment`, `magicNumber`
    /// and `flags`. read-fonts checks no checksum, so the patched file still parses.
    fn with_zero_units_per_em(bytes: &[u8]) -> Vec<u8> {
        let mut out = bytes.to_vec();
        let u16_at = |b: &[u8], at: usize| {
            u16::from_be_bytes([*b.get(at).unwrap(), *b.get(at + 1).unwrap()])
        };
        let u32_at = |b: &[u8], at: usize| {
            u32::from_be_bytes([
                *b.get(at).unwrap(),
                *b.get(at + 1).unwrap(),
                *b.get(at + 2).unwrap(),
                *b.get(at + 3).unwrap(),
            ])
        };
        let num_tables = usize::from(u16_at(&out, 4));
        let head = (0..num_tables)
            .map(|i| 12 + 16 * i)
            .find(|&record| out.get(record..record + 4) == Some(b"head"))
            .map(|record| u32_at(&out, record + 8) as usize)
            .unwrap();
        out.get_mut(head + 18..head + 20)
            .unwrap()
            .copy_from_slice(&[0, 0]);
        out
    }

    fn plan_for(t: &ResolvedTheme, bytes: &'static [u8]) -> FontPlan {
        FontPlan::new().face(
            &t.defaults.font.family,
            t.defaults.font.weight,
            t.defaults.font.style,
            FontBytes::Static(bytes),
        )
    }

    /// T6 (a): never a `FontFamily::Name`; the emoji tail survives; an empty plan is a no-op.
    #[test]
    fn t6_a_never_a_name_family_and_an_empty_plan_is_a_no_op() {
        let t = resolved("kde-breeze", ColorMode::Dark);
        let (defs, notes) = font_definitions(&t, &FontPlan::new());
        assert_eq!(defs, FontDefinitions::default());
        assert!(notes.is_empty());

        let plan = plan_for(&t, hack()).face(
            &t.defaults.mono_font.family,
            t.defaults.mono_font.weight,
            t.defaults.mono_font.style,
            FontBytes::Static(hack()),
        );
        let (defs, _) = font_definitions(&t, &plan);
        assert!(
            defs.families
                .keys()
                .all(|f| matches!(f, FontFamily::Proportional | FontFamily::Monospace))
        );
        let stock = FontDefinitions::default();
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            let chain = defs.families.get(&family).unwrap();
            let stock_chain = stock.families.get(&family).unwrap();
            assert_eq!(
                chain.get(1..),
                Some(stock_chain.as_slice()),
                "{family}: the head is the plan's face and the rest egui's own chain, emoji tail included"
            );
        }

        let atlas = ThemeAtlas::builder("t6", &t, &t).fonts(plan).build();
        for theme in [egui::Theme::Light, egui::Theme::Dark] {
            let scheme = atlas.scheme(theme);
            for style in std::iter::once(&scheme.base).chain(scheme.cells.iter().flatten()) {
                for id in style
                    .text_styles
                    .values()
                    .chain(style.override_font_id.iter())
                {
                    assert!(
                        matches!(id.family, FontFamily::Proportional | FontFamily::Monospace),
                        "{theme:?}: {id:?}"
                    );
                }
            }
        }
    }

    /// T6 (b): a face epaint would panic on, or divide by zero on, is dropped before epaint
    /// sees it. The cut-short face ends inside its table directory: read-fonts reads only the
    /// directory when it opens a face (`FontRef::new`, read-fonts 0.41.0 `src/lib.rs` lines 349–351)
    /// and never checks a table's range, and Hack's 17 records end at byte 284 with `head` at
    /// 284..338, so a cut after the directory would still parse.
    #[test]
    fn t6_b_an_invalid_face_is_dropped_with_a_note() {
        let t = resolved("adwaita", ColorMode::Light);
        let family: Arc<str> = Arc::clone(&t.defaults.font.family);
        let short: Arc<[u8]> = Arc::from(hack().get(..200).unwrap());
        let cases: Vec<(&str, FontPlan)> = vec![
            (
                "garbage bytes",
                FontPlan::new().face(
                    &family,
                    400,
                    FontStyle::Normal,
                    FontBytes::Static(b"this is not a font file"),
                ),
            ),
            (
                "a face cut short",
                FontPlan::new().face(&family, 400, FontStyle::Normal, FontBytes::Shared(short)),
            ),
            (
                "collection index 1 of a single face",
                FontPlan::new().face_at(
                    &family,
                    Some(400),
                    FontStyle::Normal,
                    FontBytes::Static(hack()),
                    1,
                ),
            ),
            (
                "unitsPerEm 0",
                FontPlan::new().face(
                    &family,
                    400,
                    FontStyle::Normal,
                    FontBytes::Shared(Arc::from(with_zero_units_per_em(hack()))),
                ),
            ),
        ];
        let stock = FontDefinitions::default();
        for (label, plan) in cases {
            let (defs, notes) = font_definitions(&t, &plan);
            assert_eq!(
                defs.families.get(&FontFamily::Proportional),
                stock.families.get(&FontFamily::Proportional),
                "{label}: the family keeps egui's own faces"
            );
            assert!(
                !defs
                    .font_data
                    .contains_key("native-theme-egui/proportional"),
                "{label}"
            );
            let invalid: Vec<&Note> = notes
                .iter()
                .filter(|n| matches!(n, Note::FontDataInvalid { .. }))
                .collect();
            assert_eq!(
                invalid,
                vec![&Note::FontDataInvalid {
                    family: Arc::clone(&family)
                }],
                "{label}"
            );
            let ctx = egui::Context::default();
            ctx.set_fonts(defs);
            let _ = pass(&ctx, egui::RawInput::default(), |ui| {
                ui.label("laid out with the plan's definitions");
            });
        }
    }

    /// T6 (c): a family the OS has no face of is reported, never replaced.
    #[cfg(feature = "system-fonts")]
    #[test]
    fn t6_c_a_family_no_system_has_is_reported_never_replaced() {
        let mut t = resolved("adwaita", ColorMode::Light);
        let pid = std::process::id();
        let sans: Arc<str> = Arc::from(format!("native-theme-egui-no-such-sans-{pid}"));
        let mono: Arc<str> = Arc::from(format!("native-theme-egui-no-such-mono-{pid}"));
        t.defaults.font.family = Arc::clone(&sans);
        t.defaults.mono_font.family = Arc::clone(&mono);
        let plan = FontPlan::from_system(&t);
        assert_eq!(plan.face_count(), 0);
        let (defs, notes) = font_definitions(&t, &plan);
        assert_eq!(
            notes,
            vec![
                Note::FontFamilyUnavailable { family: sans },
                Note::FontFamilyUnavailable { family: mono },
            ]
        );
        assert_eq!(defs, FontDefinitions::default());
    }

    /// T6 (d): the face `select_face` picks heads its chain, at the theme's `wght`.
    #[test]
    fn t6_d_the_chosen_faces_head_the_chains_at_the_asked_weight() {
        let mut t = resolved("adwaita", ColorMode::Light);
        t.defaults.font.family = Arc::from("Plan Sans");
        t.defaults.font.weight = 700;
        t.defaults.font.style = FontStyle::Normal;
        t.defaults.mono_font.family = Arc::from("Plan Mono");
        t.defaults.mono_font.weight = 400;
        t.defaults.mono_font.style = FontStyle::Normal;
        let plan = FontPlan::new()
            .face(
                "Plan Sans",
                400,
                FontStyle::Normal,
                FontBytes::Static(hack()),
            )
            .face(
                "Plan Sans",
                700,
                FontStyle::Normal,
                FontBytes::Static(bundled("Ubuntu-Light")),
            )
            .face(
                "Plan Sans",
                400,
                FontStyle::Italic,
                FontBytes::Static(bundled("NotoEmoji-Regular")),
            )
            // lower case on purpose: CSS Fonts 4 §5.1 caseless family matching (§8.2)
            .face(
                "plan mono",
                400,
                FontStyle::Normal,
                FontBytes::Static(bundled("emoji-icon-font")),
            );
        let (defs, notes) = font_definitions(&t, &plan);
        assert!(notes.is_empty(), "{notes:?}");
        let head = |family: FontFamily| {
            let name = defs.families.get(&family).unwrap().first().unwrap();
            Arc::clone(defs.font_data.get(name).unwrap())
        };
        let sans = head(FontFamily::Proportional);
        assert_eq!(sans.font.as_ref(), bundled("Ubuntu-Light"));
        assert_eq!(sans.tweak.coords, weight_coords(700));
        let mono = head(FontFamily::Monospace);
        assert_eq!(mono.font.as_ref(), bundled("emoji-icon-font"));
        assert_eq!(mono.tweak.coords, weight_coords(400));
    }

    /// T6 (e): a `FontFamily::Name` family of the plan's base survives, chain and data intact.
    #[test]
    fn t6_e_a_name_family_of_the_base_survives() {
        let t = resolved("adwaita", ColorMode::Light);
        let app = FontFamily::Name("app".into());
        let mut base = FontDefinitions::default();
        base.font_data.insert(
            "app-face".to_owned(),
            Arc::new(FontData::from_static(hack())),
        );
        base.families
            .insert(app.clone(), vec!["app-face".to_owned()]);
        let plan = plan_for(&t, hack()).with_base(base.clone());
        let (defs, _) = font_definitions(&t, &plan);
        assert_eq!(defs.families.get(&app), base.families.get(&app));
        assert_eq!(
            defs.font_data.get("app-face"),
            base.font_data.get("app-face")
        );
    }

    /// T5's plan leg at a text-scaling factor of `1.0`: §6.15 with egui's bundled `Hack` as
    /// the Body face, egui's own `row_height` as the oracle, and §6.6's slider `expansion`.
    #[test]
    fn t5_line_spacing_with_a_plan_matches_egui_row_height() {
        for preset in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let t = resolved(preset.key, mode);
                let plan = plan_for(&t, hack());
                let (defs, _) = font_definitions(&t, &plan);
                let atlas = ThemeAtlas::builder(preset.key, &t, &t).fonts(plan).build();
                let ctx = egui::Context::default();
                ctx.set_fonts(defs);
                let _ = pass(&ctx, egui::RawInput::default(), |_| {});
                for theme in [egui::Theme::Light, egui::Theme::Dark] {
                    let scheme = atlas.scheme(theme);
                    let size = scheme.base.text_styles.get(&TextStyle::Body).unwrap().size;
                    let row = ctx.fonts_mut(|f| f.row_height(&FontId::proportional(size)));
                    let want = t.defaults.line_height * size;
                    let expected = clamp_length(want - row);
                    for style in std::iter::once(&scheme.base).chain(scheme.cells.iter().flatten())
                    {
                        assert_eq!(
                            style.spacing.extra_text_line_spacing, expected,
                            "{} {mode:?} {theme:?}",
                            preset.key
                        );
                    }
                    // Every `Role::Slider` cell, in every variant (spec §13 T5).
                    for variant in RoleVariant::all() {
                        let slider = scheme.cell(Role::Slider, *variant);
                        let d = t.slider.thumb_diameter;
                        let w = slider.visuals.widgets.inactive.fg_stroke.width;
                        let expected_e = if d.is_finite() {
                            let thickness = row.max(slider.spacing.interact_size.y);
                            ((clamp_length(d) - 0.8 * thickness) * 0.5).min(0.0) - w
                        } else {
                            -w
                        };
                        let widgets = &slider.visuals.widgets;
                        for state in [
                            &widgets.noninteractive,
                            &widgets.inactive,
                            &widgets.hovered,
                            &widgets.active,
                            &widgets.open,
                        ] {
                            assert_eq!(
                                state.expansion, expected_e,
                                "{} {mode:?} {theme:?} {variant:?}",
                                preset.key
                            );
                        }
                    }
                }
            }
        }
    }

    /// T14 (a)'s font-plan clause: an install whose plan found no face restores egui's chain.
    #[test]
    fn t14_a_an_install_whose_plan_found_no_face_restores_egui_s_chain() {
        let t = resolved("adwaita", ColorMode::Light);
        let ctx = egui::Context::default();
        ctx.set_fonts(FontDefinitions::empty());
        let with_face = ThemeAtlas::builder("a", &t, &t)
            .fonts(plan_for(&t, hack()))
            .build();
        with_face.install(&ctx);
        let _ = pass(&ctx, egui::RawInput::default(), |_| {});
        let head = ctx.fonts(|f| {
            f.definitions()
                .families
                .get(&FontFamily::Proportional)
                .and_then(|chain| chain.first().cloned())
        });
        assert_eq!(head.as_deref(), Some("native-theme-egui/proportional"));

        let unrelated = FontPlan::new().face(
            "Unrelated",
            400,
            FontStyle::Normal,
            FontBytes::Static(hack()),
        );
        let none_found = ThemeAtlas::builder("b", &t, &t).fonts(unrelated).build();
        none_found.install(&ctx);
        let _ = pass(&ctx, egui::RawInput::default(), |_| {});
        let chain = ctx.fonts(|f| {
            f.definitions()
                .families
                .get(&FontFamily::Proportional)
                .cloned()
        });
        assert_eq!(
            chain,
            FontDefinitions::default()
                .families
                .get(&FontFamily::Proportional)
                .cloned()
        );
    }
}

/// §13 T18 (b)–(f), (h) and T4 (c) (plan Task 23).
mod t18_accessors {
    use std::sync::Arc;

    use egui::FontId;
    use native_theme::AccessibilityPreferences;
    use native_theme::theme::{
        ColorMode, FontStyle, ResolvedFontSpec, ResolvedPadding, ResolvedTheme,
    };

    // `resolved` (Task 11) and `pass` (Task 11, Global Constraints) are the file's own helpers.
    use super::{pass, resolved};
    use crate::convert::to_color32;
    use crate::icons::IconContext;
    use crate::{
        NativeThemeUiExt, Note, Role, RoleVariant, TextRole, ThemeAtlas, scaled_text_size,
    };

    /// §4.7's table, spelled out here so the test does not read it through the accessor.
    fn role_font_mut(t: &mut ResolvedTheme, role: Role) -> &mut ResolvedFontSpec {
        match role {
            Role::Button => &mut t.button.font,
            Role::Input => &mut t.input.font,
            Role::Checkbox => &mut t.checkbox.font,
            Role::Menu => &mut t.menu.font,
            Role::Tooltip => &mut t.tooltip.font,
            Role::Tab => &mut t.tab.font,
            Role::Sidebar => &mut t.sidebar.font,
            Role::Toolbar => &mut t.toolbar.font,
            Role::StatusBar => &mut t.status_bar.font,
            Role::Popover => &mut t.popover.font,
            Role::ComboBox => &mut t.combo_box.font,
            Role::SegmentedControl => &mut t.segmented_control.font,
            Role::Expander => &mut t.expander.font,
            Role::Link => &mut t.link.font,
            Role::List => &mut t.list.item_font,
            Role::Dialog => &mut t.dialog.body_font,
            Role::Window => &mut t.window.title_bar_font,
            Role::Scrollbar
            | Role::Slider
            | Role::ProgressBar
            | Role::Splitter
            | Role::Separator
            | Role::Switch
            | Role::Spinner
            | Role::Card => &mut t.defaults.font,
        }
    }

    fn reads_the_defaults_font(role: Role) -> bool {
        matches!(
            role,
            Role::Scrollbar
                | Role::Slider
                | Role::ProgressBar
                | Role::Splitter
                | Role::Separator
                | Role::Switch
                | Role::Spinner
                | Role::Card
        )
    }

    /// T18 (b): a stated side is the side plus the border's line width; `None` and `NaN` keep
    /// egui's `Margin::symmetric(4, 2)` (`egui/src/widgets/text_edit/builder.rs:136`).
    #[test]
    fn t18_b_input_margin_adds_the_line_width_to_stated_sides_only() {
        let mut t = resolved("adwaita", ColorMode::Light);
        t.input.border.line_width = 1.5;
        t.input.border.padding = ResolvedPadding {
            top: Some(3.0),
            right: None,
            bottom: Some(f32::NAN),
            left: Some(7.0),
        };
        let m = crate::input_margin(&t);
        // 7 + 1.5 = 8.5 and 3 + 1.5 = 4.5, each rounded to a whole point, half away from zero
        assert_eq!((m.left, m.top), (9, 5));
        assert_eq!((m.right, m.bottom), (4, 2));
    }

    /// T18 (c): each role's weight is its own font's — `font` for the fourteen, `list.item_font`,
    /// `dialog.body_font`, `window.title_bar_font`, and `defaults.font` for the other eight.
    #[test]
    fn t18_c_role_font_weight_reads_the_roles_own_font() {
        let mut t = resolved("adwaita", ColorMode::Light);
        let mut expected = Vec::new();
        for (i, role) in Role::all().iter().enumerate() {
            let weight = if reads_the_defaults_font(*role) {
                900
            } else {
                100 + u16::try_from(i).unwrap()
            };
            role_font_mut(&mut t, *role).weight = weight;
            expected.push((*role, weight));
        }
        // fonts no role reads stay out of the way
        t.list.header_font.weight = 150;
        t.dialog.title_font.weight = 160;
        for (role, weight) in expected {
            assert_eq!(crate::role_font_weight(&t, role), weight, "{role:?}");
        }
    }

    /// T18 (d): `true` exactly when the role's font is slanted and `defaults.font` is upright.
    #[test]
    fn t18_d_role_font_is_italic_only_where_the_installed_face_is_upright() {
        for role in Role::all() {
            let mut t = resolved("adwaita", ColorMode::Light);
            for r in Role::all() {
                role_font_mut(&mut t, *r).style = FontStyle::Normal;
            }
            t.defaults.font.style = FontStyle::Normal;
            assert!(
                !crate::role_font_is_italic(&t, *role),
                "{role:?}: every font upright"
            );
            for slant in [FontStyle::Italic, FontStyle::Oblique] {
                role_font_mut(&mut t, *role).style = slant;
                // the eight roles whose font is `defaults.font` slant the installed face itself
                assert_eq!(
                    crate::role_font_is_italic(&t, *role),
                    !reads_the_defaults_font(*role),
                    "{role:?} {slant:?}"
                );
                t.defaults.font.style = FontStyle::Oblique;
                assert!(
                    !crate::role_font_is_italic(&t, *role),
                    "{role:?}: defaults.font slanted"
                );
                t.defaults.font.style = FontStyle::Normal;
                role_font_mut(&mut t, *role).style = FontStyle::Normal;
            }
        }
    }

    /// T18 (e): the text accessors return their leaves, `scaled_text_size`d, as `Proportional`
    /// `FontId`s; `icon_size` takes no preferences and returns its leaf as stated.
    #[test]
    fn t18_e_the_text_accessors_return_their_leaves_scaled() {
        let t = resolved("windows-11", ColorMode::Dark);
        for factor in [1.0_f32, 2.0] {
            let prefs = AccessibilityPreferences {
                text_scaling_factor: factor,
                ..AccessibilityPreferences::default()
            };
            let s = |size: f32| scaled_text_size(size, &prefs);
            let roles = [
                (TextRole::Caption, &t.text_scale.caption),
                (TextRole::SectionHeading, &t.text_scale.section_heading),
                (TextRole::DialogTitle, &t.text_scale.dialog_title),
                (TextRole::Display, &t.text_scale.display),
            ];
            for (role, entry) in roles {
                assert_eq!(
                    crate::text_role_font(&t, role, &prefs),
                    FontId::proportional(s(entry.size)),
                    "{role:?} at {factor}"
                );
                assert_eq!(
                    crate::text_role_line_height(&t, role, &prefs),
                    s(entry.line_height),
                    "{role:?} at {factor}"
                );
                assert_eq!(crate::text_role_weight(&t, role), entry.weight, "{role:?}");
            }
            assert_eq!(
                crate::window_title_bar_font(&t, &prefs),
                FontId::proportional(s(t.window.title_bar_font.size))
            );
            assert_eq!(
                crate::window_title_bar_text_color(&t, true),
                to_color32(t.window.title_bar_font.color)
            );
            assert_eq!(
                crate::window_title_bar_text_color(&t, false),
                to_color32(t.window.inactive_title_bar_text_color)
            );
            assert_eq!(
                crate::list_header_font(&t, &prefs),
                FontId::proportional(s(t.list.header_font.size))
            );
            assert_eq!(crate::dialog_button_order(&t), t.dialog.button_order);
            assert_eq!(crate::font_size(&t, &prefs), s(t.defaults.font.size));
            assert_eq!(
                crate::mono_font_size(&t, &prefs),
                s(t.defaults.mono_font.size)
            );
        }
        let sizes = &t.defaults.icon_sizes;
        let contexts = [
            (IconContext::Small, sizes.small),
            (IconContext::Toolbar, sizes.toolbar),
            (IconContext::Panel, sizes.panel),
            (IconContext::Dialog, sizes.dialog),
            (IconContext::Large, sizes.large),
        ];
        for (context, leaf) in contexts {
            assert_eq!(crate::icons::icon_size(&t, context), leaf, "{context:?}");
        }
    }

    /// T18 (f): the closure paints in `expander.arrow_color` and hands the `Ui` back the
    /// `Arc<Style>` it held; egui's `convex_polygon` (`epaint/src/shapes/shape.rs:251-257`) is a
    /// `Shape::Path` whose `fill` is that colour.
    #[test]
    fn t18_f_expander_icon_paints_the_arrow_colour_and_restores_the_style() {
        let mut t = resolved("adwaita", ColorMode::Light);
        t.expander.arrow_color = Some(t.defaults.accent_color);
        let expected = to_color32(t.defaults.accent_color);
        let icon = crate::expander_icon(&t);
        let ctx = egui::Context::default();
        let mut restored = None;
        let out = pass(&ctx, egui::RawInput::default(), |ui| {
            let before = Arc::clone(ui.style());
            let response = ui.allocate_response(egui::vec2(16.0, 16.0), egui::Sense::hover());
            icon(ui, 1.0, &response);
            restored = Some(Arc::ptr_eq(ui.style(), &before));
        });
        assert_eq!(restored, Some(true));
        let painted = out
            .shapes
            .iter()
            .any(|s| matches!(&s.shape, egui::Shape::Path(p) if p.fill == expected));
        assert!(
            painted,
            "no polygon in the arrow colour among {} shapes",
            out.shapes.len()
        );

        // With no stated arrow colour the closure is egui's own arrow and touches no style.
        t.expander.arrow_color = None;
        let icon = crate::expander_icon(&t);
        let mut restored = None;
        let _ = pass(&ctx, egui::RawInput::default(), |ui| {
            let before = Arc::clone(ui.style());
            let response = ui.allocate_response(egui::vec2(16.0, 16.0), egui::Sense::hover());
            icon(ui, 0.0, &response);
            restored = Some(Arc::ptr_eq(ui.style(), &before));
        });
        assert_eq!(restored, Some(true));
    }

    /// One pass with a `TextEdit` of `id` inside the `Role::Input` scope, returning the frame
    /// `input_frame` computed before the field was added.
    fn input_pass(
        ctx: &egui::Context,
        t: &ResolvedTheme,
        id: egui::Id,
        text: &mut String,
    ) -> egui::Frame {
        let mut frame = None;
        let _ = pass(ctx, egui::RawInput::default(), |ui| {
            ui.native_scope(Role::Input, RoleVariant::Normal, |ui| {
                let f = crate::input_frame(ui, id, t);
                frame = Some(f);
                ui.add(egui::TextEdit::singleline(text).id(id).frame(f));
            });
        });
        frame.unwrap()
    }

    /// T18 (h): unfocused, the state's `bg_stroke`; focused, `input.focus_border_color` at the
    /// state's own width; a field focused before its first pass takes `widgets.active`; with
    /// the leaf `None`, the resting stroke.
    #[test]
    fn t18_h_input_frame_follows_focus_and_the_state() {
        let mut t = resolved("adwaita", ColorMode::Light);
        t.input.focus_border_color = Some(t.defaults.accent_color);
        let focus = to_color32(t.defaults.accent_color);
        let ctx = egui::Context::default();
        let atlas = ThemeAtlas::builder("t18h", &t, &t).build();
        atlas.install(&ctx);
        let cell = Arc::clone(
            atlas
                .scheme(ctx.theme())
                .cell(Role::Input, RoleVariant::Normal),
        );
        let id = egui::Id::new("t18h-field");
        let mut text = String::new();

        let unfocused = input_pass(&ctx, &t, id, &mut text);
        assert_eq!(unfocused.stroke, cell.visuals.widgets.inactive.bg_stroke);
        assert_eq!(unfocused.fill, cell.visuals.text_edit_bg_color());
        assert_eq!(
            unfocused.corner_radius,
            cell.visuals.widgets.inactive.corner_radius
        );

        ctx.memory_mut(|m| m.request_focus(id));
        let focused = input_pass(&ctx, &t, id, &mut text);
        let active = &cell.visuals.widgets.active;
        assert_eq!(
            focused.stroke,
            egui::Stroke::new(active.bg_stroke.width, focus)
        );
        assert_eq!(focused.corner_radius, active.corner_radius);

        // focused before its first pass: `widgets.active` (`egui/src/style.rs:1276-1278`)
        let first_id = egui::Id::new("t18h-focused-first");
        ctx.memory_mut(|m| m.request_focus(first_id));
        let mut first_text = String::new();
        let first = input_pass(&ctx, &t, first_id, &mut first_text);
        assert_eq!(
            first.stroke,
            egui::Stroke::new(active.bg_stroke.width, focus)
        );
        assert_eq!(first.corner_radius, active.corner_radius);

        // the soft option `None`: the resting stroke, focused or not
        let mut t_none = t.clone();
        t_none.input.focus_border_color = None;
        let atlas_none = ThemeAtlas::builder("t18h-none", &t_none, &t_none).build();
        atlas_none.install(&ctx);
        let resting = atlas_none
            .scheme(ctx.theme())
            .cell(Role::Input, RoleVariant::Normal)
            .visuals
            .widgets
            .inactive
            .bg_stroke;
        let none_id = egui::Id::new("t18h-none");
        let mut none_text = String::new();
        let _ = input_pass(&ctx, &t_none, none_id, &mut none_text);
        ctx.memory_mut(|m| m.request_focus(none_id));
        let focused_none = input_pass(&ctx, &t_none, none_id, &mut none_text);
        assert_eq!(focused_none.stroke, resting);
    }

    /// T18 (h), last clause: with `input.focus_border_color` equal to `input.selection_text_color`
    /// and a `1.0` line width — egui's selection-stroke width — the field paints exactly the
    /// shapes egui's own frame paints for the same `TextEdit` given `.margin(input_margin(t))`.
    #[test]
    fn t18_h_with_egui_s_own_colours_the_frame_is_egui_s_frame() {
        let mut t = resolved("adwaita", ColorMode::Light);
        t.input.focus_border_color = Some(t.input.selection_text_color);
        t.input.border.line_width = 1.0;
        let atlas = ThemeAtlas::builder("t18h-eq", &t, &t).build();
        let id = egui::Id::new("t18h-equal");
        let with_frame = egui::Context::default();
        let with_margin = egui::Context::default();
        atlas.install(&with_frame);
        atlas.install(&with_margin);
        let mut a = String::new();
        let mut b = String::new();
        let shapes_of = |ctx: &egui::Context, custom: bool, text: &mut String| {
            pass(ctx, egui::RawInput::default(), |ui| {
                ui.native_scope(Role::Input, RoleVariant::Normal, |ui| {
                    let edit = egui::TextEdit::singleline(text).id(id);
                    let edit = if custom {
                        edit.frame(crate::input_frame(ui, id, &t))
                    } else {
                        edit.margin(crate::input_margin(&t))
                    };
                    ui.add(edit);
                });
            })
            .shapes
        };
        for focused in [false, true] {
            if focused {
                with_frame.memory_mut(|m| m.request_focus(id));
                with_margin.memory_mut(|m| m.request_focus(id));
            }
            assert_eq!(
                shapes_of(&with_frame, true, &mut a),
                shapes_of(&with_margin, false, &mut b),
                "focused = {focused}"
            );
        }
    }

    /// T4 (c), with Review Focus 5's `0.0` and `-12.0`: a text size that is not a positive
    /// normal `f32` keeps egui's own size for its slot, is a `Note::ValueSanitised`, lays out
    /// without a panic, and every `FontId` accessor returns a positive normal size.
    #[test]
    fn t4_c_a_degenerate_text_size_is_sanitised_everywhere() {
        let prefs = AccessibilityPreferences::default();
        for hostile in [f32::NAN, f32::INFINITY, 1e-45_f32, 0.0, -12.0] {
            let mut t = resolved("kde-breeze", ColorMode::Light);
            t.defaults.font.size = hostile;
            t.button.font.size = hostile;
            let atlas = ThemeAtlas::builder("t4c", &t, &t).build();
            for path in ["defaults.font.size", "button.font.size"] {
                assert!(
                    atlas.notes().contains(&Note::ValueSanitised { path }),
                    "{hostile}: no note for {path}: {:?}",
                    atlas.notes()
                );
            }
            let ctx = egui::Context::default(); // egui's default fonts: text is laid out
            atlas.install(&ctx);
            let _ = pass(&ctx, egui::RawInput::default(), |ui| {
                ui.label("a label in Body");
                ui.native_scope(Role::Button, RoleVariant::Normal, |ui| {
                    let _ = ui.button("a button in its scope");
                });
            });
            // Every leaf a `FontId` accessor reads is made hostile too, on a copy the atlas above
            // never saw, so each accessor's own §8.5 fallback runs (the build's scope stays T4 (c)'s).
            let mut u = t.clone();
            for entry in [
                &mut u.text_scale.caption,
                &mut u.text_scale.section_heading,
                &mut u.text_scale.dialog_title,
                &mut u.text_scale.display,
            ] {
                entry.size = hostile;
            }
            u.window.title_bar_font.size = hostile;
            u.list.header_font.size = hostile;
            u.defaults.mono_font.size = hostile;
            let positive_normal = |size: f32| size.is_normal() && size > 0.0;
            for role in [
                TextRole::Caption,
                TextRole::SectionHeading,
                TextRole::DialogTitle,
                TextRole::Display,
            ] {
                assert!(
                    positive_normal(crate::text_role_font(&u, role, &prefs).size),
                    "{hostile} {role:?}"
                );
            }
            assert!(
                positive_normal(crate::window_title_bar_font(&u, &prefs).size),
                "{hostile}"
            );
            assert!(
                positive_normal(crate::list_header_font(&u, &prefs).size),
                "{hostile}"
            );
            assert!(positive_normal(crate::font_size(&u, &prefs)), "{hostile}");
            assert!(
                positive_normal(crate::mono_font_size(&u, &prefs)),
                "{hostile}"
            );
        }
    }
}

/// §13 T18 (g), T18 (a)'s `from_preset` clause and Review Focus 5's unknown preset (plan Task 24).
mod t18_constructors {
    use native_theme::theme::{ColorMode, Theme};
    use native_theme::{AccessibilityPreferences, SystemTheme};

    // `resolved` is the file's own helper (Task 11); `atlas_diff` is Task 19's.
    use super::resolved;
    use crate::style_diff::atlas_diff;
    use crate::{Error, SystemThemeExt, ThemeAtlas, from_preset, from_system, to_theme};

    /// Every style of two atlases, compared field by field: both base styles, every cell in
    /// every variant, and every `Surface` frame. Not `==`: `Style`'s `PartialEq` compares
    /// `number_formatter` by `Arc::ptr_eq` (`egui/src/style.rs:57-62`), and each build starts
    /// from a fresh `Style::default()` (`:1435`), so two builds are never `==` (Global Constraints).
    fn same_styles(a: &ThemeAtlas, b: &ThemeAtlas) -> bool {
        let changes = atlas_diff(a, b);
        assert!(changes.is_empty(), "the atlases differ at {changes:?}");
        true
    }

    /// T18 (a), `from_preset` clause: its `name()` is the preset's
    /// `Theme::name` (`native-theme/src/model/mod.rs:257`), and the atlas carries both variants, each
    /// variant's icon theme, and the requested variant as the returned `ResolvedTheme`.
    #[test]
    fn t18_a_from_preset_carries_the_presets_name_and_both_variants() {
        let prefs = AccessibilityPreferences::default();
        for preset in Theme::list_presets() {
            let spec = Theme::preset(preset.key).unwrap();
            let light = spec.resolve(ColorMode::Light).unwrap();
            let dark = spec.resolve(ColorMode::Dark).unwrap();
            for is_dark in [false, true] {
                let (atlas, chosen) = from_preset(preset.key, is_dark, &prefs).unwrap();
                assert_eq!(atlas.name(), spec.name, "{}", preset.key);
                assert_eq!(
                    atlas.resolved_for(egui::Theme::Light),
                    &light.variant,
                    "{}",
                    preset.key
                );
                assert_eq!(
                    atlas.resolved_for(egui::Theme::Dark),
                    &dark.variant,
                    "{}",
                    preset.key
                );
                assert_eq!(
                    &chosen,
                    if is_dark {
                        &dark.variant
                    } else {
                        &light.variant
                    },
                    "{}",
                    preset.key
                );
                assert_eq!(
                    atlas.icon_theme(egui::Theme::Light),
                    light.icon_theme.as_deref(),
                    "{}",
                    preset.key
                );
                assert_eq!(
                    atlas.icon_theme(egui::Theme::Dark),
                    dark.icon_theme.as_deref(),
                    "{}",
                    preset.key
                );
                assert_eq!(atlas.icon_set(), light.icon_set, "{}", preset.key);
                assert_eq!(atlas.os_mode(), None, "{}", preset.key);
                assert_eq!(atlas.accessibility(), &prefs, "{}", preset.key);
                assert_eq!(atlas.layout(), &spec.layout, "{}", preset.key);
            }
        }
    }

    /// Review Focus 5: an unknown preset is an `Err`, never a panic.
    #[test]
    fn from_preset_of_an_unknown_name_is_an_error() {
        let prefs = AccessibilityPreferences::default();
        assert!(matches!(
            from_preset("no-such-preset", false, &prefs),
            Err(Error::UnknownPreset { .. })
        ));
    }

    /// T18 (g), last clause: `to_theme(r, n)` is `ThemeAtlas::builder(n, r, r).build()`, style
    /// for style.
    #[test]
    fn t18_g_to_theme_is_the_builder_over_one_variant() {
        let r = resolved("gruvbox", ColorMode::Dark);
        let atlas = to_theme(&r, "gruvbox twice");
        let direct = ThemeAtlas::builder("gruvbox twice", &r, &r).build();
        assert_eq!(atlas.name(), "gruvbox twice");
        assert_eq!(atlas.resolved_for(egui::Theme::Light), &r);
        assert_eq!(atlas.resolved_for(egui::Theme::Dark), &r);
        assert_eq!(atlas.os_mode(), None);
        assert!(same_styles(&atlas, &direct));
    }

    /// T18 (g): `to_egui_atlas` reports every input it was given, and `from_system` is exactly
    /// that over `SystemTheme::from_system()`, or an error exactly when that is one.
    #[test]
    fn t18_g_to_egui_atlas_reports_what_it_was_given() {
        // On a runner with no desktop there is nothing to compare; native-theme's own test
        // accepts the same (`native-theme/src/watch/mod.rs:313`).
        let Ok(mut sys) = SystemTheme::from_system() else {
            assert!(
                from_system().is_err(),
                "from_system must fail exactly when SystemTheme::from_system does"
            );
            return;
        };
        // Values the detection did not return, so a field read from the wrong place shows.
        sys.mode = if sys.mode.is_dark() {
            ColorMode::Light
        } else {
            ColorMode::Dark
        };
        sys.accessibility = AccessibilityPreferences {
            text_scaling_factor: 1.5,
            reduce_motion: !sys.accessibility.reduce_motion,
            ..sys.accessibility.clone()
        };
        sys.layout.widget_gap = Some(11.0);
        sys.icon_set = match sys.icon_set {
            native_theme::theme::IconSet::Lucide => native_theme::theme::IconSet::Material,
            _ => native_theme::theme::IconSet::Lucide,
        };

        let atlas = sys.to_egui_atlas();
        assert_eq!(atlas.name(), sys.name);
        assert_eq!(atlas.os_mode(), Some(sys.mode));
        assert_eq!(atlas.accessibility(), &sys.accessibility);
        assert_eq!(atlas.layout(), &sys.layout);
        assert_eq!(atlas.icon_set(), sys.icon_set);
        assert_eq!(atlas.resolved_for(egui::Theme::Light), &sys.light);
        assert_eq!(atlas.resolved_for(egui::Theme::Dark), &sys.dark);
        assert_eq!(
            atlas.icon_theme(egui::Theme::Light),
            sys.icon_theme_for(ColorMode::Light)
        );
        assert_eq!(
            atlas.icon_theme(egui::Theme::Dark),
            sys.icon_theme_for(ColorMode::Dark)
        );
        // §6.16: `layout.widget_gap` is `spacing.item_spacing`, both axes, in the base style.
        for theme in [egui::Theme::Light, egui::Theme::Dark] {
            assert_eq!(
                atlas.scheme(theme).base.spacing.item_spacing,
                egui::vec2(11.0, 11.0)
            );
        }
        #[cfg(feature = "system-fonts")]
        {
            let plan = crate::fonts::FontPlan::from_system(&sys.light);
            let (_, notes) = crate::fonts::font_definitions(&sys.light, &plan);
            for note in &notes {
                assert!(
                    atlas.notes().contains(note),
                    "missing {note:?} in {:?}",
                    atlas.notes()
                );
            }
        }

        let (again, chosen, is_dark) = from_system().unwrap();
        let fresh = SystemTheme::from_system().unwrap();
        assert_eq!(again.name(), fresh.name);
        assert_eq!(again.os_mode(), Some(fresh.mode));
        assert_eq!(is_dark, fresh.mode.is_dark());
        assert_eq!(again.resolved_for(egui::Theme::Light), &fresh.light);
        assert_eq!(again.resolved_for(egui::Theme::Dark), &fresh.dark);
        assert_eq!(&chosen, if is_dark { &fresh.dark } else { &fresh.light });
        assert!(same_styles(&again, &fresh.to_egui_atlas()));
    }
}

/// T14 (a), the raster-icon clause: an `install` after `to_image_source` stored a raster
/// icon's `TextureHandle` leaves none in `ctx.data` (§10.3 step 5).
#[test]
fn install_leaves_no_stored_icon_texture_handle() {
    use crate::icons::{IconKey, IconRegistry, handles_key, texture_id, to_image_source, uri};
    let ctx = egui::Context::default();
    let (atlas, _) =
        crate::from_preset("adwaita", false, &AccessibilityPreferences::default()).unwrap();
    atlas.install(&ctx);
    let icon = crate::icons::IconData::Rgba {
        width: 4,
        height: 4,
        data: vec![0; 64],
    };
    let key = IconKey::name("raster", IconSet::Freedesktop);
    let before = ctx.tex_manager().read().num_allocated();
    let _ = to_image_source(&ctx, &key, &icon).unwrap();
    let handle_id = texture_id(&uri(&icon));
    assert!(ctx.data(|d| d.get_temp::<egui::TextureHandle>(handle_id).is_some()));
    atlas.install(&ctx);
    assert!(ctx.data(|d| d.get_temp::<egui::TextureHandle>(handle_id).is_none()));
    assert!(ctx.data(|d| d.get_temp::<IconRegistry>(handles_key()).is_none()));
    assert_eq!(ctx.tex_manager().read().num_allocated(), before);
}

// ---- T8: the documented limits (spec §13 T8; T8 (c) is Task 26's) ------------------------

fn kde_breeze() -> (ResolvedTheme, ResolvedTheme) {
    let theme = Theme::preset("kde-breeze").expect("bundled preset");
    let light = theme.resolve(ColorMode::Light).expect("resolves").variant;
    let dark = theme.resolve(ColorMode::Dark).expect("resolves").variant;
    (light, dark)
}

/// T8 (a) — §7.5, §14 item 30: `TextStyle::resolve` panics on a missing key in every build
/// (`egui/src/style.rs:112-120`), and each seam below replaces a whole style, so a seam that
/// forgot the merge would crash an application that names its own text styles.
#[test]
fn t8a_named_text_styles_survive_every_seam() {
    use egui::{FontId, TextStyle, Theme};
    let (light, dark) = kde_breeze();
    let atlas = ThemeAtlas::builder("t8a", &light, &dark).build();
    let key = TextStyle::Name("app-caption".into());
    let font = FontId::proportional(11.0);

    let ctx = egui::Context::default();
    ctx.set_fonts(egui::FontDefinitions::empty()); // no glyph is laid out (§13 T14's shape)
    for theme in [Theme::Dark, Theme::Light] {
        let (key, font) = (key.clone(), font.clone());
        ctx.style_mut_of(theme, move |s| {
            s.text_styles.insert(key, font); // egui/src/context.rs:2237
        });
    }
    atlas.install(&ctx);
    for theme in [Theme::Dark, Theme::Light] {
        let style = ctx.style_of(theme); // egui/src/context.rs:2221
        assert_eq!(
            style.text_styles.get(&key),
            Some(&font),
            "{theme:?}: install dropped the application's key (§10.3 step 2)"
        );
    }

    let _ = pass(&ctx, egui::RawInput::default(), |ui| {
        let inside_scope = ui
            .native_scope(Role::Button, RoleVariant::Normal, |ui| {
                ui.style().text_styles.get(&key).cloned()
            })
            .inner;
        assert_eq!(
            inside_scope,
            Some(font.clone()),
            "native_scope lost the key (§4.5)"
        );

        ui.native_set_style(Role::Sidebar, RoleVariant::Normal);
        assert_eq!(
            ui.style().text_styles.get(&key),
            Some(&font),
            "native_set_style lost the key (§4.5)"
        );
        // The documented failure path, once: resolve would panic on a missing key.
        assert_eq!(key.resolve(ui.style()), font);

        let modifier = ThemeAtlas::from_ctx(ui.ctx())
            .expect("installed")
            .role_modifier(ui.ctx().theme(), Role::Popover, RoleVariant::Normal);
        let in_popup = egui::Popup::new(
            egui::Id::new("t8a-popup"),
            ui.ctx().clone(),
            egui::PopupAnchor::Position(egui::pos2(20.0, 20.0)), // egui/src/containers/popup.rs:35
            ui.layer_id(),
        )
        .open(true) // :296
        .style(modifier) // :417
        .show(|ui| ui.style().text_styles.get(&key).cloned()) // :508
        .expect("an open popup shows its contents")
        .inner;
        assert_eq!(
            in_popup,
            Some(font.clone()),
            "role_modifier lost the key (§4.2)"
        );
    });
}

/// T8 (b) — §14 item 2b: the handle a `Window` paints is the outer `Ui`'s, so a
/// `Role::Scrollbar` scope as the first statement inside the closure does not reach it. On
/// `kde-breeze` the scrollbar is no overlay (`native-theme/src/presets/kde-breeze.toml:149`),
/// so egui paints the bar at full opacity (`egui/src/containers/scroll_area.rs:1483-1485`,
/// `:1495-1497`), and only the corner radius can tell the base style from the cell (§6.8).
#[test]
fn t8b_a_window_scrollbar_keeps_the_base_radius() {
    use egui::Shape;
    let (mut light, mut dark) = kde_breeze();
    assert!(!light.scrollbar.overlay_mode && !dark.scrollbar.overlay_mode);
    for r in [&mut light, &mut dark] {
        // The fields are public (T4): set the two radii apart first.
        r.button.border.corner_radius = r.defaults.border.corner_radius + 4.0;
    }
    let atlas = ThemeAtlas::builder("t8b", &light, &dark).build();
    let ctx = egui::Context::default();
    ctx.set_fonts(egui::FontDefinitions::empty());
    atlas.install(&ctx);

    let scheme = ctx.theme();
    let styles = atlas.scheme(scheme);
    let base = &styles.base;
    let cell = styles.cell(Role::Scrollbar, RoleVariant::Normal);
    let base_radius = base.visuals.widgets.inactive.corner_radius;
    assert_ne!(
        base_radius, cell.visuals.widgets.inactive.corner_radius,
        "the base and the Role::Scrollbar radii must differ for this test to discriminate"
    );
    let thumb = base.visuals.widgets.inactive.bg_fill; // scrollbar.thumb_color (§5.9)
    assert_eq!(
        thumb,
        crate::convert::to_color32(atlas.resolved_for(scheme).scrollbar.thumb_color)
    );
    assert_ne!(
        thumb, base.visuals.extreme_bg_color,
        "the track must not share the thumb's colour"
    );

    // The bar appears once the content size is known from a previous pass: run three.
    let mut handles = Vec::new();
    for _ in 0..3 {
        handles.clear();
        let _ = pass(&ctx, egui::RawInput::default(), |ui| {
            let ctx = ui.ctx().clone();
            let shown = egui::Window::new("t8b")
                .fixed_size([200.0, 100.0]) // egui/src/containers/window.rs:437
                // A new window fades in, multiplying every fill's alpha (`:227-234`).
                .fade_in(false)
                .vscroll(true) // :512; the ScrollArea branch, :738
                .show(&ctx, |ui| {
                    ui.native_set_style(Role::Scrollbar, RoleVariant::Normal);
                    for i in 0..50 {
                        ui.label(format!("row {i}"));
                    }
                })
                .expect("the window is open");
            let layer = shown.response.layer_id;
            ctx.graphics(|g| {
                // egui/src/context.rs:1045; egui/src/layers.rs:204, :186
                if let Some(list) = g.get(layer) {
                    for entry in list.all_entries() {
                        if let Shape::Rect(rect) = &entry.shape
                            && rect.fill == thumb
                        {
                            handles.push(rect.clone());
                        }
                    }
                }
            });
        });
    }
    assert_eq!(
        handles.len(),
        1,
        "exactly one handle in the window's layer (egui/src/containers/scroll_area.rs:1515-1519)"
    );
    assert_eq!(
        handles[0].corner_radius, base_radius,
        "the handle is painted from the outer Ui's style, the base style's (§14 item 2b)"
    );
}
