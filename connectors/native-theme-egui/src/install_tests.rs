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
