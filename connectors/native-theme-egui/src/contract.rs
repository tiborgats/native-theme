//! The mapping contract (spec §13 T10, T10b, T12; §13.1): `mapping.toml` checked against what
//! the atlas publishes, over every bundled preset in both modes, and the contrast of every
//! text-on-fill pair egui paints against the native pair it was assembled from.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use egui::{Color32, CornerRadius, Margin, Shadow, Stroke, Style, Theme, Vec2};
use native_theme::theme::{ColorMode, ResolvedTheme, Theme as NativeTheme};

use crate::convert::{self, Rgba};
use crate::install_tests::resolved;
use crate::mapping_tests::{
    Manifest, Row, Sink, Verdict, role_by_key, surface_by_key, variant_by_key,
};
use crate::style_diff::{all_frame_paths, all_frames, all_style_paths, all_styles, self_equal};
use crate::{Role, RoleVariant, Surface, ThemeAtlas};

// ---- the 32 combinations (C6) ---------------------------------------------------------------

struct Combination {
    key: &'static str,
    mode: ColorMode,
    scheme: Theme,
    atlas: ThemeAtlas,
    resolved: ResolvedTheme,
    /// `serde_json::to_value(&resolved)` with the four `layout.` leaves (`with_layout`): the leaf
    /// paths of §13.1 are its dotted paths.
    json: serde_json::Value,
}

impl Combination {
    fn label(&self) -> String {
        format!(
            "{}/{}",
            self.key,
            if self.mode == ColorMode::Dark {
                "dark"
            } else {
                "light"
            }
        )
    }
    fn styles(&self) -> &crate::atlas::SchemeStyles {
        self.atlas.scheme(self.scheme)
    }
    fn cell(&self, role: Role, variant: RoleVariant) -> &Style {
        self.styles().cell(role, variant)
    }
    /// The Body size the atlas writes (§8.5): the theme's size scaled, egui's own where that
    /// is not a positive normal number — `None` only if egui's style had no Body slot, as
    /// Task 13's `compile_scheme` reads it.
    fn body_size(&self) -> Option<f32> {
        let scaled =
            crate::scaled_text_size(self.resolved.defaults.font.size, self.atlas.accessibility());
        if scaled.is_normal() && scaled > 0.0 {
            return Some(scaled);
        }
        self.scheme
            .default_style()
            .text_styles
            .get(&egui::TextStyle::Body)
            .map(|f| f.size)
    }
    /// The Body row height the build measured for this scheme (§6.15; Task 13's
    /// `compile_scheme`), recomputed the same way: `fonts::body_row_height` over the
    /// definitions the atlas installs — the plan's (`ThemeAtlas::fonts`), else egui's default —
    /// at the Body size the atlas writes (`body_size`).
    fn row_height(&self) -> Option<f32> {
        let size = self.body_size()?;
        match self.atlas.fonts() {
            Some(defs) => crate::fonts::body_row_height(defs, size),
            None => crate::fonts::body_row_height(&egui::FontDefinitions::default(), size),
        }
    }
}

/// `Theme::list_presets()` (`native-theme/src/model/mod.rs:653`) — sixteen; the four `-live`
/// merge bases are not selectable — in both modes: the 32 combinations. One atlas per preset,
/// carrying both variants, built with no plan and `AccessibilityPreferences::default()`.
fn combinations() -> Vec<Combination> {
    let mut out = Vec::new();
    for info in NativeTheme::list_presets() {
        let light = resolved(info.key, ColorMode::Light);
        let dark = resolved(info.key, ColorMode::Dark);
        let atlas = ThemeAtlas::builder(info.key, &light, &dark).build();
        for (mode, scheme, resolved) in [
            (ColorMode::Light, Theme::Light, light),
            (ColorMode::Dark, Theme::Dark, dark),
        ] {
            let json = serde_json::to_value(&resolved).expect("ResolvedTheme serialises"); // native-theme/src/model/resolved.rs:155
            let json = with_layout(json, atlas.layout());
            out.push(Combination {
                key: info.key,
                mode,
                scheme,
                atlas: atlas.clone(),
                resolved,
                json,
            });
        }
    }
    assert_eq!(out.len(), 32, "C6's 32 combinations");
    out
}

/// `json` with the four `layout.` leaves under the manifest's names, as T3's `leaf_paths` adds
/// them: they are the builder's `LayoutTheme`, not `ResolvedTheme` fields, and `LayoutTheme`
/// serialises them as `*_px` and skips a `None` (`native-theme/src/model/widgets/mod.rs:893-911`).
fn with_layout(mut json: serde_json::Value, layout: &crate::LayoutTheme) -> serde_json::Value {
    if let Some(object) = json.as_object_mut() {
        object.insert(
            "layout".to_owned(),
            serde_json::json!({
                "widget_gap": layout.widget_gap,
                "container_margin": layout.container_margin,
                "window_margin": layout.window_margin,
                "section_gap": layout.section_gap,
            }),
        );
    }
    json
}

// ---- the manifest (§13.1): Task 12's typed reader, `mapping_tests::Manifest::load()` ----------

// ---- reading a sink at its location ---------------------------------------------------------

/// The value at one sink, typed as the field is. `PartialEq` on `f32` is what T10 wants: a `NaN`
/// anywhere fails the self-comparison below.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Val {
    F32(f32),
    OptF32(Option<f32>),
    Bool(bool),
    Color(Color32),
    OptColor(Option<Color32>),
    Radius(CornerRadius),
    Vec2(Vec2),
    Stroke(Stroke),
    Shadow(Shadow),
    Offset([i8; 2]),
    Handle(egui::style::HandleShape),
}

fn widget_entry<'a>(
    w: &'a egui::style::Widgets,
    state: &str,
) -> Option<&'a egui::style::WidgetVisuals> {
    Some(match state {
        "noninteractive" => &w.noninteractive,
        "inactive" => &w.inactive,
        "hovered" => &w.hovered,
        "active" => &w.active,
        "open" => &w.open,
        _ => return None,
    })
}

fn margin_side(m: Margin, side: &str) -> Option<Val> {
    Some(Val::F32(f32::from(match side {
        "left" => m.left,
        "right" => m.right,
        "top" => m.top,
        "bottom" => m.bottom,
        _ => return None,
    })))
}

/// One field of a `Shadow` by §13.1's spelling (`epaint/src/shadow.rs:13-30`).
fn shadow_field(s: Shadow, field: &str) -> Option<Val> {
    Some(match field {
        "color" => Val::Color(s.color),
        "offset" => Val::Offset(s.offset),
        "blur" => Val::F32(f32::from(s.blur)),
        "spread" => Val::F32(f32::from(s.spread)),
        _ => return None,
    })
}

/// Read a `Style` field by §13.1's sink path — the spelling `style_diff` emits — for every
/// field the manifest writes. A path this reader lacks fails the test naming it; extend the
/// reader, never skip the row.
fn read_style(style: &Style, path: &str) -> Option<Val> {
    let seg: Vec<&str> = path.split('.').collect();
    let v = &style.visuals;
    let sp = &style.spacing;
    Some(match seg.as_slice() {
        ["visuals", "widgets", state, rest @ ..] => {
            let e = widget_entry(&v.widgets, state)?;
            match rest {
                ["bg_fill"] => Val::Color(e.bg_fill),
                ["weak_bg_fill"] => Val::Color(e.weak_bg_fill),
                ["bg_stroke", "color"] => Val::Color(e.bg_stroke.color),
                ["bg_stroke", "width"] => Val::F32(e.bg_stroke.width),
                ["bg_stroke"] => Val::Stroke(e.bg_stroke),
                ["fg_stroke", "color"] => Val::Color(e.fg_stroke.color),
                ["fg_stroke", "width"] => Val::F32(e.fg_stroke.width),
                ["fg_stroke"] => Val::Stroke(e.fg_stroke),
                ["corner_radius"] => Val::Radius(e.corner_radius),
                ["expansion"] => Val::F32(e.expansion),
                _ => return None,
            }
        }
        ["visuals", "selection", "bg_fill"] => Val::Color(v.selection.bg_fill),
        ["visuals", "selection", "stroke", "color"] => Val::Color(v.selection.stroke.color),
        ["visuals", "selection", "stroke", "width"] => Val::F32(v.selection.stroke.width),
        ["visuals", "panel_fill"] => Val::Color(v.panel_fill),
        ["visuals", "window_fill"] => Val::Color(v.window_fill),
        ["visuals", "extreme_bg_color"] => Val::Color(v.extreme_bg_color),
        ["visuals", "faint_bg_color"] => Val::Color(v.faint_bg_color),
        ["visuals", "hyperlink_color"] => Val::Color(v.hyperlink_color),
        ["visuals", "error_fg_color"] => Val::Color(v.error_fg_color),
        ["visuals", "warn_fg_color"] => Val::Color(v.warn_fg_color),
        ["visuals", "override_text_color"] => Val::OptColor(v.override_text_color),
        ["visuals", "text_edit_bg_color"] => Val::OptColor(v.text_edit_bg_color),
        ["visuals", "weak_text_color"] => Val::OptColor(v.weak_text_color),
        ["visuals", "disabled_alpha"] => Val::F32(v.disabled_alpha),
        ["visuals", "window_corner_radius"] => Val::Radius(v.window_corner_radius),
        ["visuals", "menu_corner_radius"] => Val::Radius(v.menu_corner_radius),
        ["visuals", "window_stroke", "color"] => Val::Color(v.window_stroke.color),
        ["visuals", "window_stroke", "width"] => Val::F32(v.window_stroke.width),
        ["visuals", "window_shadow"] => Val::Shadow(v.window_shadow),
        ["visuals", "window_shadow", field] => shadow_field(v.window_shadow, field)?,
        ["visuals", "popup_shadow"] => Val::Shadow(v.popup_shadow),
        ["visuals", "popup_shadow", field] => shadow_field(v.popup_shadow, field)?,
        ["visuals", "text_cursor", "stroke", "color"] => Val::Color(v.text_cursor.stroke.color),
        ["visuals", "slider_trailing_fill"] => Val::Bool(v.slider_trailing_fill),
        ["visuals", "collapsing_header_frame"] => Val::Bool(v.collapsing_header_frame),
        ["visuals", "handle_shape"] => Val::Handle(v.handle_shape),
        ["spacing", "interact_size", "x"] => Val::F32(sp.interact_size.x),
        ["spacing", "interact_size", "y"] => Val::F32(sp.interact_size.y),
        ["spacing", "button_padding"] => Val::Vec2(sp.button_padding),
        ["spacing", "button_padding", "x"] => Val::F32(sp.button_padding.x),
        ["spacing", "button_padding", "y"] => Val::F32(sp.button_padding.y),
        ["spacing", "item_spacing", "x"] => Val::F32(sp.item_spacing.x),
        ["spacing", "item_spacing", "y"] => Val::F32(sp.item_spacing.y),
        ["spacing", "window_margin", side] => margin_side(sp.window_margin, side)?,
        ["spacing", "menu_margin", side] => margin_side(sp.menu_margin, side)?,
        ["spacing", "icon_width"] => Val::F32(sp.icon_width),
        ["spacing", "icon_width_inner"] => Val::F32(sp.icon_width_inner),
        ["spacing", "icon_spacing"] => Val::F32(sp.icon_spacing),
        ["spacing", "slider_rail_height"] => Val::F32(sp.slider_rail_height),
        ["spacing", "combo_width"] => Val::F32(sp.combo_width),
        ["spacing", "tooltip_width"] => Val::F32(sp.tooltip_width),
        ["spacing", "extra_text_line_spacing"] => Val::F32(sp.extra_text_line_spacing),
        ["spacing", "scroll", "bar_width"] => Val::F32(sp.scroll.bar_width),
        ["spacing", "scroll", "handle_min_length"] => Val::F32(sp.scroll.handle_min_length),
        ["spacing", "scroll", "bar_inner_margin"] => Val::F32(sp.scroll.bar_inner_margin),
        ["spacing", "scroll", "bar_outer_margin"] => Val::F32(sp.scroll.bar_outer_margin),
        ["spacing", "scroll", "floating_width"] => Val::F32(sp.scroll.floating_width),
        ["spacing", "scroll", "floating_allocated_width"] => {
            Val::F32(sp.scroll.floating_allocated_width)
        }
        ["spacing", "scroll", "floating"] => Val::Bool(sp.scroll.floating),
        ["spacing", "scroll", "foreground_color"] => Val::Bool(sp.scroll.foreground_color),
        [slot, "size"] if slot.starts_with("text_styles[") => {
            let name = slot
                .trim_start_matches("text_styles[")
                .trim_end_matches(']');
            let key = match name {
                "Small" => egui::TextStyle::Small,
                "Body" => egui::TextStyle::Body,
                "Monospace" => egui::TextStyle::Monospace,
                "Button" => egui::TextStyle::Button,
                "Heading" => egui::TextStyle::Heading,
                _ => return None,
            };
            Val::F32(style.text_styles.get(&key)?.size)
        }
        // `None` in egui's own `Style` (`egui/src/style.rs:1432`): an optional size.
        ["override_font_id", "size"] => {
            Val::OptF32(style.override_font_id.as_ref().map(|f| f.size))
        }
        ["animation_time"] => Val::F32(style.animation_time),
        _ => return None,
    })
}

/// Read a `Frame` field by §13.1's `Frame` spelling.
fn read_frame(frame: &egui::Frame, path: &str) -> Option<Val> {
    let seg: Vec<&str> = path.split('.').collect();
    Some(match seg.as_slice() {
        ["fill"] => Val::Color(frame.fill),
        ["stroke"] => Val::Stroke(frame.stroke),
        ["stroke", "color"] => Val::Color(frame.stroke.color),
        ["stroke", "width"] => Val::F32(frame.stroke.width),
        ["corner_radius"] => Val::Radius(frame.corner_radius),
        ["shadow"] => Val::Shadow(frame.shadow),
        ["shadow", field] => shadow_field(frame.shadow, field)?,
        ["inner_margin", side] => margin_side(frame.inner_margin, side)?,
        ["outer_margin", side] => margin_side(frame.outer_margin, side)?,
        _ => return None,
    })
}

/// The `Style` a sink's location starts from before any row writes it — §3.4's inheritance,
/// §13.1's *Inheritance*: egui's own `Style` for the base style, the base style for a role's
/// `Normal` cell (with egui's `menu_style` applied for `Role::Menu`, §3.4), and the `Normal`
/// cell for its `Selected` and `Disabled` cells. With `AccessibilityPreferences::default()`
/// and no patch, the stored base style is the one the cells were derived from.
fn inherited_style(c: &Combination, sink: &Sink) -> Style {
    match (sink.scope.as_deref(), sink.variant.as_deref()) {
        (None, _) => c.scheme.default_style(), // egui/src/memory/theme.rs:24-29
        (Some(scope), None | Some("normal")) => {
            let mut s = Style::clone(&c.styles().base);
            if scope == "menu" {
                egui::containers::menu::menu_style(&mut s); // egui/src/containers/menu.rs:22
            }
            s
        }
        (Some(scope), Some(_)) => Style::clone(c.cell(role_by_key(scope), RoleVariant::Normal)),
    }
}

/// The value the atlas publishes at `sink` (§13.1's location rule), and the value the location
/// inherits there (`inherited_style`; for a `Surface` frame, egui's preset frame over the base
/// style, §3.4).
fn read_sink(c: &Combination, sink: &Sink) -> (Option<Val>, Option<Val>) {
    match (&sink.scope, &sink.surface) {
        (Some(scope), _) => {
            let cell = c.cell(
                role_by_key(scope),
                variant_by_key(sink.variant.as_deref().unwrap_or("normal")),
            );
            (
                read_style(cell, &sink.path),
                read_style(&inherited_style(c, sink), &sink.path),
            )
        }
        (None, Some(surface)) => {
            let surface = surface_by_key(surface);
            let frame = c.styles().frame(surface);
            let preset = crate::style::egui_preset(surface, &c.styles().base); // §3.4, the §4.5 table
            (
                read_frame(&frame, &sink.path),
                read_frame(&preset, &sink.path),
            )
        }
        (None, None) => (
            read_style(&c.styles().base, &sink.path),
            read_style(&inherited_style(c, sink), &sink.path),
        ),
    }
}

// ---- the native leaf --------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum Native {
    Color(Rgba),
    Number(f32),
    Bool(bool),
    Null,
    Other,
}

/// The leaf's value in the resolved theme, by its §13.1 path over `serde_json::to_value`.
/// A colour is its `Display` string (`native-theme/src/color.rs:138-150`), parsed back with
/// `Rgba::from_str` (`:188`); a `FontSize` object stops the walk.
fn native(c: &Combination, leaf: &str) -> Native {
    let mut cur = &c.json;
    for seg in leaf.split('.') {
        cur = match cur.get(seg) {
            Some(next) => next,
            None => return Native::Other,
        };
    }
    match cur {
        serde_json::Value::Null => Native::Null,
        serde_json::Value::Bool(b) => Native::Bool(*b),
        serde_json::Value::Number(n) => n
            .as_f64()
            .map_or(Native::Other, |f| Native::Number(f as f32)),
        serde_json::Value::String(s) => Rgba::from_str(s).map_or(Native::Other, Native::Color),
        _ => Native::Other,
    }
}

/// A leaf as `Option<f32>` — the optional sizes and padding sides of §2/§6 — read through the
/// public `ResolvedTheme` fields by the same path is what the formulas below take; the JSON
/// walk gives the same value, so it is the one reader here.
fn number(c: &Combination, leaf: &str) -> Option<f32> {
    match native(c, leaf) {
        Native::Number(n) => Some(n),
        _ => None,
    }
}
fn colour(c: &Combination, leaf: &str) -> Option<Rgba> {
    match native(c, leaf) {
        Native::Color(x) => Some(x),
        _ => None,
    }
}

// ---- the oracle -----------------------------------------------------------------------------

/// §6.13's fold reaches the strokes a border-colour leaf fills — `*.border.color`,
/// `checkbox.unchecked_border_color`, `input.hover_border_color` — and no other colour: a
/// separator, grid, divider or menu-separator line keeps its own stated colour (§5.1's
/// `defaults.border.opacity` row).
fn folds_opacity(leaf: &str) -> bool {
    leaf.ends_with(".border.color")
        || leaf == "checkbox.unchecked_border_color"
        || leaf == "input.hover_border_color"
}
fn is_text_size(path: &str) -> bool {
    (path.starts_with("text_styles[") || path.starts_with("override_font_id"))
        && path.ends_with(".size")
}

/// §7.2's conversion of a stated leaf into `sink`, by the sink's own type (T10's rule for
/// `direct` and `scoped` rows). `Ok(None)` for a `Null` leaf: the location keeps what it
/// inherits (§13.1, *Inheritance*).
fn converted(
    c: &Combination,
    leaf: &str,
    sink: &Sink,
    inherited: Val,
) -> Result<Option<Val>, String> {
    let t = &c.resolved;
    let size = |x: f32| {
        if is_text_size(&sink.path) {
            crate::scaled_text_size(x, c.atlas.accessibility()) // §8.5, §8.6
        } else {
            convert::clamp_length(x)
        }
    };
    Ok(Some(match (native(c, leaf), inherited) {
        (Native::Null, _) => return Ok(None),
        (Native::Color(x), Val::Color(_)) => Val::Color(if folds_opacity(leaf) {
            convert::to_color32_with_opacity(x, t.defaults.border.opacity) // §6.13: a border colour carries the fold
        } else {
            convert::to_color32(x)
        }),
        (Native::Color(x), Val::OptColor(_)) => Val::OptColor(Some(convert::to_color32(x))),
        (Native::Number(x), Val::F32(base)) => Val::F32(if sink.path == "visuals.disabled_alpha" {
            convert::unit_interval(x)
        } else if sink.path.ends_with("stroke.width") {
            convert::finite_or(x, base).max(0.0) // `to_stroke`'s width rule (§7.2)
        } else if sink.path.contains("_margin.") {
            f32::from(convert::i8_from_f32_saturating(x)) // an `i8` margin side (§7.2)
        } else {
            size(x)
        }),
        (Native::Number(x), Val::OptF32(_)) => Val::OptF32(Some(size(x))),
        (Native::Number(x), Val::Radius(base)) => Val::Radius(convert::to_corner_radius(base, x)),
        (Native::Bool(b), Val::Bool(_)) => Val::Bool(b),
        (native, inherited) => {
            return Err(format!(
                "no conversion from {native:?} into {inherited:?}; add the sink's formula to `expected`"
            ));
        }
    }))
}

fn padding_of(c: &Combination, widget: &str) -> native_theme::theme::ResolvedPadding {
    native_theme::theme::ResolvedPadding {
        top: number(c, &format!("{widget}.border.padding.top")),
        right: number(c, &format!("{widget}.border.padding.right")),
        bottom: number(c, &format!("{widget}.border.padding.bottom")),
        left: number(c, &format!("{widget}.border.padding.left")),
    }
}

/// A widget's `ResolvedWidgetBorder` (`native-theme/src/model/border.rs:233-246`), read back
/// from the JSON leaves, for the helpers that take the whole border.
fn border_of(c: &Combination, widget: &str) -> native_theme::theme::ResolvedWidgetBorder {
    native_theme::theme::ResolvedWidgetBorder {
        color: colour(c, &format!("{widget}.border.color")).expect("required"),
        corner_radius: number(c, &format!("{widget}.border.corner_radius")).expect("required"),
        line_width: number(c, &format!("{widget}.border.line_width")).expect("required"),
        shadow_enabled: matches!(
            native(c, &format!("{widget}.border.shadow_enabled")),
            Native::Bool(true)
        ),
        padding: padding_of(c, widget),
    }
}

/// The first stated colour of a §6.4 chain; the chain's last leaf is required.
fn chain(c: &Combination, leaves: &[&str]) -> Result<Rgba, String> {
    leaves
        .iter()
        .find_map(|leaf| colour(c, leaf))
        .ok_or_else(|| format!("no leaf of {leaves:?} is stated, yet the last is required"))
}

/// What `sink` must hold (§5, §6): `Ok(Some(v))`, or `Ok(None)` where the location keeps what it
/// inherits (§13.1, *Inheritance*: a sink moves only where its leaf is stated). The value is a
/// property of the location, not of one declaring row: where several rows declare one sink — a
/// §6.4 fallback chain, a §6.1 state layer over its idle fill, §6.13's opacity fold over a border
/// colour, a formula of several inputs — every one of them is held to the one formula that fills
/// it (T10: "for a `when` sink the formula of the row that writes it"). A location with one
/// declaring row and no formula here is that row's §7.2 conversion. `Err` names a sink no formula
/// covers: add its §6 formula here, never skip it.
fn expected(
    c: &Combination,
    rows: &BTreeMap<String, Row>,
    sink: &Sink,
    inherited: Val,
) -> Result<Option<Val>, String> {
    let t = &c.resolved;
    let p = sink.path.as_str();
    let scope = sink.scope.as_deref();
    let variant = sink.variant.as_deref().unwrap_or("normal");
    let surface = sink.surface.as_deref();
    let state = p
        .strip_prefix("visuals.widgets.")
        .and_then(|r| r.split('.').next())
        .unwrap_or("");
    let hover_or_press = matches!(state, "hovered" | "active");
    let c32 = |x: Rgba| Val::Color(convert::to_color32(x));
    let folded = |x: Rgba| {
        Val::Color(convert::to_color32_with_opacity(
            x,
            t.defaults.border.opacity,
        ))
    };
    // §6.1 (C17): a stated layer composited over the idle fill; an unstated one copies the idle
    // fill, written as given (§6.4).
    let layered = |layer: Option<Rgba>, idle: Rgba| {
        Val::Color(layer.map_or(convert::to_color32(idle), |l| {
            convert::composite_over(l, idle)
        }))
    };
    let req = |leaf: &str| {
        colour(c, leaf).ok_or_else(|| format!("`{leaf}` is required but not a colour"))
    };
    let inherited_style = || inherited_style(c, sink);
    let cell = || scope.map(|s| c.cell(role_by_key(s), variant_by_key(variant)));
    Ok(Some(match (scope, variant, surface, p) {
        // ---- §6.1: hover and pressed fills are state layers ----
        (None | Some("button"), "normal", None, "visuals.widgets.hovered.weak_bg_fill") => layered(
            colour(c, "button.hover_background"),
            req("button.background_color")?,
        ),
        (None | Some("button"), "normal", None, "visuals.widgets.active.weak_bg_fill") => layered(
            colour(c, "button.active_background").or_else(|| colour(c, "button.hover_background")),
            req("button.background_color")?,
        ),
        (Some(w @ ("combo_box" | "segmented_control" | "tab")), "normal", None, p)
            if hover_or_press && p.ends_with(".weak_bg_fill") =>
        {
            layered(
                colour(c, &format!("{w}.hover_background")),
                req(&format!("{w}.background_color"))?,
            )
        }
        (Some("switch"), "normal", None, p) if hover_or_press && p.ends_with(".weak_bg_fill") => {
            layered(
                colour(c, "switch.hover_unchecked_background"),
                req("switch.unchecked_background")?,
            )
        }
        (Some("checkbox"), "normal", None, p)
            if p.ends_with(".bg_fill") && state != "noninteractive" =>
        {
            let idle = chain(
                c,
                &["checkbox.unchecked_background", "checkbox.background_color"],
            )?;
            if hover_or_press {
                layered(colour(c, "checkbox.hover_background"), idle)
            } else {
                c32(idle)
            }
        }
        // §6.4: the expander has no idle fill; an unstated hover leaves no highlight.
        (Some("expander"), "normal", None, p) if hover_or_press && p.ends_with(".weak_bg_fill") => {
            colour(c, "expander.hover_background").map_or(Val::Color(Color32::TRANSPARENT), c32)
        }
        // ---- §6.3, §6.4, §6.9: fallbacks copied as given ----
        (
            Some(w @ ("button" | "combo_box")),
            "disabled",
            None,
            "visuals.widgets.inactive.weak_bg_fill",
        )
        | (Some(w @ "button"), "disabled", None, "visuals.selection.bg_fill") => c32(chain(
            c,
            &[
                &format!("{w}.disabled_background"),
                &format!("{w}.background_color"),
            ],
        )?),
        (Some("checkbox"), "disabled", None, "visuals.widgets.inactive.bg_fill") => c32(chain(
            c,
            &[
                "checkbox.disabled_background",
                "checkbox.unchecked_background",
                "checkbox.background_color",
            ],
        )?),
        (Some("input"), "disabled", None, "visuals.text_edit_bg_color") => {
            Val::OptColor(Some(convert::to_color32(chain(
                c,
                &["input.disabled_background", "input.background_color"],
            )?)))
        }
        (Some("slider"), "disabled", None, "visuals.widgets.inactive.bg_fill") => c32(chain(
            c,
            &["slider.disabled_track_color", "slider.track_color"],
        )?),
        (Some("slider"), "disabled", None, "visuals.selection.bg_fill") => c32(chain(
            c,
            &["slider.disabled_fill_color", "slider.fill_color"],
        )?),
        (Some("switch"), "disabled", None, "visuals.widgets.inactive.weak_bg_fill") => c32(chain(
            c,
            &[
                "switch.disabled_unchecked_background",
                "switch.unchecked_background",
            ],
        )?),
        (Some("switch"), "disabled", None, "visuals.selection.bg_fill") => c32(chain(
            c,
            &[
                "switch.disabled_checked_background",
                "switch.checked_background",
            ],
        )?),
        (None | Some("scrollbar"), "normal", None, "visuals.widgets.active.bg_fill") => c32(chain(
            c,
            &[
                "scrollbar.thumb_active_color",
                "scrollbar.thumb_hover_color",
            ],
        )?),
        (Some("slider"), "normal", None, p) if hover_or_press && p.ends_with(".bg_fill") => c32(
            chain(c, &["slider.thumb_hover_color", "slider.thumb_color"])?,
        ),
        (Some(_), "disabled", None, "visuals.disabled_alpha") => Val::F32(1.0), // §6.3, §6.17
        // ---- §6.13: border colours, folded, with their §6.4 fallbacks ----
        (Some("checkbox"), "normal", None, p) if p.ends_with(".bg_stroke.color") => folded(chain(
            c,
            &["checkbox.unchecked_border_color", "checkbox.border.color"],
        )?),
        (Some("input"), "normal", None, p) if hover_or_press && p.ends_with(".bg_stroke.color") => {
            folded(chain(
                c,
                &["input.hover_border_color", "input.border.color"],
            )?)
        }
        // ---- §5.9: `slider.fill_color` is invisible without the trailing fill ----
        (Some("slider"), _, None, "visuals.slider_trailing_fill") => Val::Bool(true),
        // §5.9, §6.6: a diameter's knob is round, in the base style and every slider cell.
        (_, _, None, "visuals.handle_shape") => Val::Handle(egui::style::HandleShape::Circle),
        // §5.5, §5.9: the scroll handle reads `bg_fill`, never `fg_stroke`.
        (None, _, None, "spacing.scroll.foreground_color") => Val::Bool(false),
        // §6.1: the expander's hover fill lies in the collapsing-header frame.
        (Some("expander"), _, None, "visuals.collapsing_header_frame") => Val::Bool(true),
        // ---- §6.5 D1: the scroll fields, from the two widths and overlay_mode ----
        (
            None,
            _,
            None,
            p @ ("spacing.scroll.floating"
            | "spacing.scroll.bar_width"
            | "spacing.scroll.bar_inner_margin"
            | "spacing.scroll.bar_outer_margin"
            | "spacing.scroll.floating_width"
            | "spacing.scroll.floating_allocated_width"),
        ) => {
            let (Some(g), Some(tw)) = (
                number(c, "scrollbar.groove_width"),
                number(c, "scrollbar.thumb_width"),
            ) else {
                return Err("scrollbar.groove_width and thumb_width are required".to_owned());
            };
            if !(g.is_finite() && tw.is_finite()) {
                return Ok(None);
            }
            let g = convert::clamp_length(g);
            let tw = convert::clamp_length(tw).min(g);
            let pad = (g - tw) * 0.5;
            let overlay = matches!(native(c, "scrollbar.overlay_mode"), Native::Bool(true));
            match (overlay, p) {
                (_, "spacing.scroll.floating") => Val::Bool(overlay),
                (false, "spacing.scroll.bar_width") => Val::F32(tw),
                (false, "spacing.scroll.bar_inner_margin" | "spacing.scroll.bar_outer_margin") => {
                    Val::F32(pad)
                }
                (true, "spacing.scroll.floating_width") => Val::F32(tw),
                (true, "spacing.scroll.bar_width") => Val::F32(g),
                (true, "spacing.scroll.floating_allocated_width") => Val::F32(0.0),
                _ => return Ok(None), // the other branch's fields: not written
            }
        }
        // ---- §6.6 D2: the slider's row and knob ----
        (Some("slider"), "normal", None, "spacing.interact_size.y") => {
            let d = t.slider.thumb_diameter; // a required size
            if !d.is_finite() {
                return Ok(None);
            }
            Val::F32(convert::clamp_length(1.25 * d))
        }
        (Some("slider"), _, None, p) if p.ends_with(".expansion") => {
            let cell = cell().ok_or("a scoped sink")?;
            let w = cell.visuals.widgets.inactive.fg_stroke.width;
            let d = t.slider.thumb_diameter;
            Val::F32(match (c.row_height(), d.is_finite()) {
                (Some(h), true) => {
                    let thickness = h.max(cell.spacing.interact_size.y); // egui/src/widgets/slider.rs:957-959
                    ((convert::clamp_length(d) - 0.8 * thickness) * 0.5).min(0.0) - w
                }
                _ => -w,
            })
        }
        // ---- §6.7 D3: the spinner's size ----
        (Some("spinner"), "normal", None, "spacing.interact_size.y") => {
            let Val::F32(base_y) = inherited else {
                return Err("an f32 sink".to_owned());
            };
            let d = convert::finite_or(t.spinner.diameter, base_y);
            let m = convert::finite_or(t.spinner.min_diameter, 0.0);
            Val::F32(convert::clamp_length(d.max(m)))
        }
        // ---- §6.8 D4: the slider's radius, capped at half the rail ----
        (Some("slider"), "normal", None, "visuals.widgets.inactive.corner_radius") => {
            let Val::Radius(base) = inherited else {
                return Err("a radius sink".to_owned());
            };
            let cell = cell().ok_or("a scoped sink")?;
            let r = t.defaults.border.corner_radius;
            let cap = cell.spacing.slider_rail_height * 0.5;
            Val::Radius(convert::to_corner_radius(
                base,
                if r.is_finite() {
                    r.max(0.0).min(cap)
                } else {
                    r
                },
            ))
        }
        // ---- §6.10 R-BAR: the toolbar's bar height less its frame ----
        (Some("toolbar"), "normal", None, "spacing.interact_size.y") => {
            let Some(h) = t.toolbar.bar_height else {
                return Ok(None);
            };
            if !h.is_finite() {
                return Ok(None);
            }
            let frame = c.styles().frame(Surface::Panel(crate::PanelSide::Top));
            let cell = cell().ok_or("a scoped sink")?;
            let total_v = f32::from(frame.inner_margin.top)
                + f32::from(frame.inner_margin.bottom)
                + 2.0 * frame.stroke.width
                + f32::from(frame.outer_margin.top)
                + f32::from(frame.outer_margin.bottom)
                + f32::from(convert::i8_from_f32_saturating(
                    cell.visuals.widgets.noninteractive.bg_stroke.width,
                ));
            Val::F32(convert::clamp_length(h - total_v))
        }
        // ---- §6.11 R-ARROW, and the checkbox's inset in the same field ----
        (Some("expander"), "normal", None, "spacing.icon_width_inner") => {
            let s = t.expander.arrow_icon_size; // a required size
            if !s.is_finite() {
                return Ok(None);
            }
            Val::F32(convert::clamp_length(s * (4.0 / 3.0)))
        }
        (Some("checkbox"), "normal", None, "spacing.icon_width_inner") => {
            let pad = padding_of(c, "checkbox");
            let cell = cell().ok_or("a scoped sink")?;
            match (pad.left, pad.right, pad.top, pad.bottom) {
                (Some(l), Some(r), Some(tp), Some(b))
                    if l.is_finite() && r.is_finite() && tp.is_finite() && b.is_finite() =>
                {
                    Val::F32(convert::clamp_length(
                        cell.spacing.icon_width - (f32::midpoint(l, r) + f32::midpoint(tp, b)),
                    ))
                }
                _ => return Ok(None),
            }
        }
        // ---- §6.12: the combo box's arrow area ----
        (Some("combo_box"), "normal", None, "spacing.icon_spacing") => {
            let Some(w) = t.combo_box.arrow_area_width else {
                return Ok(None);
            };
            if !w.is_finite() {
                return Ok(None);
            }
            let cell = cell().ok_or("a scoped sink")?;
            Val::F32(convert::clamp_length(w - cell.spacing.icon_width))
        }
        // ---- §5's pair rule (§7.2 `to_button_padding`): four sides and the line width. The
        // expander reads `.x` on its trailing side only, so its `.x` is that side alone (§7.4). ----
        (Some("expander"), "normal", None, "spacing.button_padding.x") => {
            match convert::padding_with_border(&border_of(c, "expander")).right {
                Some(right) => Val::F32(convert::clamp_length(right)),
                None => return Ok(None),
            }
        }
        (_, "normal", None, p @ ("spacing.button_padding.x" | "spacing.button_padding.y")) => {
            let widget = scope.unwrap_or("button");
            let padding = convert::to_button_padding(
                inherited_style().spacing.button_padding,
                &border_of(c, widget),
            );
            Val::F32(if p.ends_with(".x") {
                padding.x
            } else {
                padding.y
            })
        }
        // ---- §6.15: the line spacing, from the row height the build measured ----
        (None, _, None, "spacing.extra_text_line_spacing") => {
            Val::F32(match (c.row_height(), c.body_size()) {
                (Some(row), Some(size)) if t.defaults.line_height.is_finite() => {
                    convert::clamp_length(t.defaults.line_height * size - row)
                }
                _ => 0.0,
            })
        }
        // ---- §6.16: the layout gap, a Builder input; T10's atlases carry none ----
        (None, _, None, "spacing.item_spacing.x" | "spacing.item_spacing.y") => {
            match c.atlas.layout().widget_gap {
                Some(g) if g.is_finite() => Val::F32(convert::clamp_length(g)),
                _ => return Ok(None),
            }
        }
        // ---- §6.14: the shadow gate over egui's own shadow ----
        (None, _, None, p)
            if p.starts_with("visuals.window_shadow.")
                || p.starts_with("visuals.popup_shadow.") =>
        {
            let (owner, own) = if p.starts_with("visuals.window_shadow.") {
                ("window", c.scheme.default_style().visuals.window_shadow)
            } else {
                ("defaults", c.scheme.default_style().visuals.popup_shadow)
            };
            let enabled = matches!(
                native(c, &format!("{owner}.border.shadow_enabled")),
                Native::Bool(true)
            );
            let field = p.rsplit('.').next().unwrap_or("");
            shadow_field(
                convert::to_shadow(own, t.defaults.shadow_color, enabled),
                field,
            )
            .ok_or("a shadow field")?
        }
        (None, _, Some(owner @ ("dialog" | "popover" | "tooltip")), p)
            if p.starts_with("shadow.") =>
        {
            let own = c.scheme.default_style().visuals.popup_shadow;
            let enabled = matches!(
                native(c, &format!("{owner}.border.shadow_enabled")),
                Native::Bool(true)
            );
            let field = p.rsplit('.').next().unwrap_or("");
            shadow_field(
                convert::to_shadow(own, t.defaults.shadow_color, enabled),
                field,
            )
            .ok_or("a shadow field")?
        }
        // ---- one declaring row: its §7.2 conversion ----
        _ => {
            // `defaults.border.opacity` is §6.13's fold, part of the border colour's conversion.
            let declaring: Vec<(&str, &Row)> = rows
                .iter()
                .filter(|(leaf, _)| leaf.as_str() != "defaults.border.opacity")
                .filter(|(_, r)| {
                    r.sinks.iter().any(|s| {
                        s.path == sink.path
                            && s.scope == sink.scope
                            && s.variant == sink.variant
                            && s.surface == sink.surface
                    })
                })
                .map(|(leaf, r)| (leaf.as_str(), r))
                .collect();
            match declaring.as_slice() {
                [(leaf, row)] if row.verdict != Verdict::Derived => {
                    return converted(c, leaf, sink, inherited);
                }
                _ => {
                    let leaves: Vec<&str> = declaring.iter().map(|(leaf, _)| *leaf).collect();
                    return Err(format!(
                        "no formula for this location, declared by {leaves:?}; add its §6 formula to `expected`"
                    ));
                }
            }
        }
    }))
}

// ---- T10 --------------------------------------------------------------------------------------

/// T10 (C6): every declared sink holds the value its location's formula gives, in all 32
/// combinations; a `None` leaf leaves the value the location inherits; `exceptions` are the only
/// relaxation.
#[test]
fn t10_every_sink_holds_the_leafs_value() {
    let manifest = Manifest::load();
    let combinations = combinations();
    let mut failures = Vec::new();
    let mut stale = Vec::new();
    for c in &combinations {
        let label = c.label();
        for (leaf, row) in &manifest.rows {
            if row.verdict == Verdict::Unmappable || row.tested_by.is_some() {
                continue;
            }
            for sink in &row.sinks {
                let at = (&sink.scope, &sink.variant, &sink.surface);
                let (got, inherited) = read_sink(c, sink);
                let (Some(got), Some(inherited)) = (got, inherited) else {
                    failures.push(format!(
                        "{label}: `{leaf}` → `{}` at {at:?}: the reader lacks this path",
                        sink.path
                    ));
                    continue;
                };
                let want = match expected(c, &manifest.rows, sink, inherited) {
                    Ok(want) => want.unwrap_or(inherited), // a None leaf: the location keeps what it inherits
                    Err(why) => {
                        failures.push(format!(
                            "{label}: `{leaf}` → `{}` at {at:?}: {why}",
                            sink.path
                        ));
                        continue;
                    }
                };
                match row
                    .exceptions
                    .iter()
                    .find(|(preset, _)| preset.as_str() == c.key)
                {
                    Some((_, why)) => {
                        if got == want {
                            stale.push(format!("{label}: `{leaf}` → `{}` is excepted ({why}) yet holds the expected value", sink.path));
                        }
                    }
                    None => {
                        if got != want {
                            failures.push(format!(
                                "{label}: `{leaf}` → `{}` at {at:?}: got {got:?}, want {want:?}",
                                sink.path
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} sink(s) differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(
        stale.is_empty(),
        "{} stale exception(s):\n{}",
        stale.len(),
        stale.join("\n")
    );
}

/// The §6.11 cases T10 names, on `adwaita`, the one bundled preset that states the checkbox
/// padding (`native-theme/src/presets/adwaita.toml:124`, `:135-136`): 14 where egui draws 8;
/// pairs set apart, the mean of both; every side above half the box, `clamp_length`'s floor.
#[test]
fn t10_a_none_leaf_keeps_eguis_value() {
    let light = resolved("adwaita", ColorMode::Light);
    let dark = resolved("adwaita", ColorMode::Dark);
    let inner = |light: &ResolvedTheme, dark: &ResolvedTheme, scheme: Theme| {
        let atlas = ThemeAtlas::builder("adwaita", light, dark).build();
        atlas
            .scheme(scheme)
            .cell(Role::Checkbox, RoleVariant::Normal)
            .spacing
            .icon_width_inner
    };
    for scheme in [Theme::Light, Theme::Dark] {
        let t = if scheme == Theme::Dark { &dark } else { &light };
        let box_width = t.checkbox.indicator_width; // a required size: `f32` on the resolved theme
        let side = t.checkbox.border.padding.left.expect("adwaita states it");
        assert_eq!(
            inner(&light, &dark, scheme),
            box_width - 2.0 * side,
            "{scheme:?}: 20 - (3 + 3) = 14"
        );

        let (mut l, mut d) = (light.clone(), dark.clone());
        for r in [&mut l, &mut d] {
            r.checkbox.border.padding.top = Some(side + 2.0);
            r.checkbox.border.padding.bottom = Some(side + 2.0);
        }
        assert_eq!(
            inner(&l, &d, scheme),
            box_width - (side + (side + 2.0)),
            "{scheme:?}: 20 - (3 + 5) = 12"
        );

        for r in [&mut l, &mut d] {
            let above_half = box_width * 0.5 + 1.0;
            r.checkbox.border.padding = native_theme::theme::ResolvedPadding {
                top: Some(above_half),
                right: Some(above_half),
                bottom: Some(above_half),
                left: Some(above_half),
            };
        }
        assert_eq!(
            inner(&l, &d, scheme),
            0.0,
            "{scheme:?}: clamp_length's floor"
        );
    }
    // Every other preset states no checkbox padding: egui's own value stands there.
    let egui_own = Theme::Light.default_style().spacing.icon_width_inner; // 8.0, egui/src/style.rs:1467
    for c in combinations() {
        if c.key != "adwaita" {
            assert_eq!(
                c.cell(Role::Checkbox, RoleVariant::Normal)
                    .spacing
                    .icon_width_inner,
                egui_own,
                "{}",
                c.label()
            );
        }
    }
}

/// T10b: every path `style_diff` can emit, and every `Frame` field of every `Surface`, is a
/// sink of at least one row or a key of `[unwritten]`, and never both.
#[test]
fn t10b_every_style_path_is_a_sink_or_unwritten() {
    let manifest = Manifest::load();
    let declared: BTreeSet<String> = manifest
        .rows
        .values()
        .flat_map(|r| {
            r.sinks.iter().map(|s| match &s.surface {
                Some(surface) => format!("{surface}.{}", s.path),
                None => s.path.clone(),
            })
        })
        .collect();
    let mut missing = Vec::new();
    let mut both = Vec::new();
    let mut check = |path: String| match (
        declared.contains(&path),
        manifest.unwritten.contains_key(&path),
    ) {
        (true, true) => both.push(path),
        (false, false) => missing.push(path),
        _ => {}
    };
    for path in all_style_paths() {
        check(path);
    }
    // Every `Frame` field of every `Surface`, in `frame_diff`'s spelling — the same list the
    // sinks' `path`s are written in (§13.1) — prefixed with the surface's key.
    let frame_paths = all_frame_paths();
    for surface in Surface::all() {
        for field in &frame_paths {
            check(format!("{}.{field}", surface.key()));
        }
    }
    assert!(
        missing.is_empty(),
        "{} field(s) neither a sink nor `[unwritten]`:\n{}",
        missing.len(),
        missing.join("\n")
    );
    assert!(
        both.is_empty(),
        "{} field(s) both a sink and `[unwritten]`:\n{}",
        both.len(),
        both.join("\n")
    );
}

/// T10's `NaN` sweep over the 32 combinations: `Style` derives `PartialEq` (`egui/src/style.rs:241`)
/// and one `NaN` anywhere makes a value unequal to itself; `number_formatter` compares by
/// `Arc::ptr_eq` (`:57-62`) and always equals itself. `self_equal` (Task 19) spells `v == v` so
/// that `clippy::eq_op`, deny-by-default, does not fire on it. One atlas per preset holds both
/// schemes, so the even combinations cover every style.
#[test]
fn t10_every_style_equals_itself() {
    for c in combinations().iter().filter(|c| c.mode == ColorMode::Light) {
        for (loc, s) in all_styles(&c.atlas) {
            assert!(self_equal(&*s), "{}: {loc:?} is not equal to itself", c.key);
        }
        for (loc, frame) in all_frames(&c.atlas) {
            assert!(
                self_equal(&frame),
                "{}: {loc:?} is not equal to itself",
                c.key
            );
        }
    }
}

// ---- T12 (C8) ---------------------------------------------------------------------------------

/// Straight-alpha sRGB in `0.0..=1.0`. A `Color32` is gamma-space sRGBA with *premultiplied*
/// alpha (`ecolor/src/color32.rs:8`), unmultiplied at this boundary (`:248`), before any blend.
#[derive(Clone, Copy, Debug)]
struct Straight {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

fn straight(c: Color32) -> Straight {
    let [r, g, b, a] = c.to_srgba_unmultiplied();
    Straight {
        r: f32::from(r) / 255.0,
        g: f32::from(g) / 255.0,
        b: f32::from(b) / 255.0,
        a: f32::from(a) / 255.0,
    }
}
fn straight_native(c: Rgba) -> Straight {
    Straight {
        r: f32::from(c.r) / 255.0,
        g: f32::from(c.g) / 255.0,
        b: f32::from(c.b) / 255.0,
        a: f32::from(c.a) / 255.0,
    }
}

/// `top` over `bottom`, straight-alpha source-over — the siblings' `over`
/// (`connectors/native-theme-iced/src/contract.rs:852-877`).
fn over(top: Straight, bottom: Straight) -> Straight {
    let under = bottom.a * (1.0 - top.a);
    let alpha = top.a + under;
    if alpha <= 0.0 {
        return Straight {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        };
    }
    if under <= 0.0 {
        return top;
    }
    let blend = |t: f32, b: f32| (t * top.a + b * under) / alpha;
    Straight {
        r: blend(top.r, bottom.r),
        g: blend(top.g, bottom.g),
        b: blend(top.b, bottom.b),
        a: alpha,
    }
}

/// WCAG 2.1 relative luminance and contrast ratio, as the iced connector's test-only
/// `contrast_ratio` computes them (`connectors/native-theme-iced/src/extended.rs:52-58`).
fn luminance(c: Straight) -> f32 {
    let lin = |v: f32| {
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b)
}
fn contrast_ratio(a: Straight, b: Straight) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}
/// Contrast of `fg` on `bg`, every layer composited first: `bg` over the `surface` it is
/// painted on, `fg` over the result.
fn pair_ratio(fg: Straight, bg: Straight, surface: Straight) -> f32 {
    let bg = over(bg, surface);
    contrast_ratio(over(fg, bg), bg)
}

struct Pair {
    what: &'static str,
    /// The egui painting site that combines the two.
    site: &'static str,
    /// The connector's colours: text, fill, and the surface under the fill.
    emitted: fn(&Combination) -> (Color32, Color32, Color32),
    /// The native pair `mapping.toml` names as their sources, on the same surface.
    native: fn(&ResolvedTheme) -> (Rgba, Rgba, Rgba),
    /// Both colours from one native widget's pair (§13 T12): asserted; else printed.
    asserted: bool,
    /// `[("preset/mode", "why")]`: a combination on which the pair legitimately degrades.
    exceptions: &'static [(&'static str, &'static str)],
}

fn base(c: &Combination) -> &Style {
    &c.styles().base
}

const PAIRS: &[Pair] = &[
    Pair {
        what: "TextEdit text on its background",
        site: "egui/src/widgets/text_edit/builder.rs:481-483, :739; egui/src/style.rs:1152-1153",
        emitted: |c| {
            let s = c.cell(Role::Input, RoleVariant::Normal);
            (
                s.visuals.widgets.inactive.fg_stroke.color,
                s.visuals.text_edit_bg_color(),
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.input.font.color,
                r.input.background_color,
                r.defaults.background_color,
            )
        },
        asserted: true,
        exceptions: &[],
    },
    Pair {
        what: "Strong text on the panel",
        site: "egui/src/style.rs:1147-1149; egui/src/widget_text.rs:485",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.strong_text_color(),
                s.visuals.panel_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.text_color,
                r.defaults.background_color,
                r.defaults.background_color,
            )
        },
        asserted: true,
        exceptions: &[],
    },
    Pair {
        what: "Button text on its resting fill",
        site: "egui/src/widget_style.rs:133-136, :159",
        emitted: |c| {
            let s = c.cell(Role::Button, RoleVariant::Normal);
            (
                s.visuals.widgets.inactive.fg_stroke.color,
                s.visuals.widgets.inactive.weak_bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.button.font.color,
                r.button.background_color,
                r.defaults.background_color,
            )
        },
        asserted: true,
        exceptions: &[],
    },
    Pair {
        what: "Button text on its hovered fill (C17 composite)",
        site: "egui/src/widget_style.rs:133-136, :159; §6.1",
        emitted: |c| {
            let s = c.cell(Role::Button, RoleVariant::Normal);
            (
                s.visuals.widgets.hovered.fg_stroke.color,
                s.visuals.widgets.hovered.weak_bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.button.hover_text_color,
                r.button.hover_background,
                r.button.background_color,
            )
        },
        asserted: true,
        // windows-11's button hover layer is translucent (§6.1): `composite_over` blends in premultiplied gamma space and rounds to `u8`
        // (`ecolor/src/color32.rs:343-345`, §7.2), the native pair here in straight-alpha
        // `f32`, so the two differ by rounding alone — 15.685 against 15.696, both far above AA.
        exceptions: &[(
            "windows-11/light",
            "composite_over's premultiplied u8 rounding of a translucent layer (§7.2)",
        )],
    },
    Pair {
        what: "Label on a panel",
        site: "egui/src/widgets/label.rs:296-299; egui/src/style.rs:1136-1139; egui/src/containers/frame.rs:188",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.text_color(),
                s.visuals.panel_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.text_color,
                r.defaults.background_color,
                r.defaults.background_color,
            )
        },
        asserted: true,
        exceptions: &[],
    },
    Pair {
        what: "Selected text on the selection fill",
        site: "egui/src/text_selection/visuals.rs:39-40",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.selection.stroke.color,
                s.visuals.selection.bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.selection_text_color,
                r.defaults.selection_background,
                r.defaults.background_color,
            )
        },
        asserted: true,
        exceptions: &[],
    },
    Pair {
        what: "The checkbox mark on the checked box",
        site: "egui/src/widget_style.rs:182, :188; egui/src/widgets/checkbox.rs:157",
        emitted: |c| {
            let s = c.cell(Role::Checkbox, RoleVariant::Selected);
            (
                s.visuals.widgets.inactive.fg_stroke.color,
                s.visuals.widgets.inactive.bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.checkbox.indicator_color,
                r.checkbox.checked_background,
                r.defaults.background_color,
            )
        },
        asserted: true,
        exceptions: &[],
    },
    Pair {
        what: "Hyperlink on a panel",
        site: "egui/src/widgets/hyperlink.rs:47; egui/src/containers/frame.rs:188",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.hyperlink_color,
                s.visuals.panel_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.link_color,
                r.defaults.background_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "Warning text on a panel (C19: printed, never enforced)",
        site: "egui/src/style.rs:1056; egui/src/containers/frame.rs:188",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.warn_fg_color,
                s.visuals.panel_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.warning_color,
                r.defaults.background_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "Error text on a panel (C19: printed, never enforced)",
        site: "egui/src/style.rs:1059; egui/src/containers/frame.rs:188",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.error_fg_color,
                s.visuals.panel_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.danger_color,
                r.defaults.background_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "An unscoped popup's or tooltip's label on the popup fill",
        site: "egui/src/containers/frame.rs:210, :219; egui/src/style.rs:1136-1139",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.text_color(),
                s.visuals.window_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.text_color,
                r.menu.background_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "An unscoped menu item's text on the menu fill (the item rests transparent)",
        site: "egui/src/containers/menu.rs:27; egui/src/widget_style.rs:133-136; egui/src/containers/frame.rs:210",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.widgets.inactive.fg_stroke.color,
                s.visuals.window_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.button.font.color,
                r.menu.background_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "A scoped menu item's text on the menu fill",
        site: "egui/src/containers/menu.rs:27; egui/src/widget_style.rs:133-136",
        emitted: |c| {
            let s = c.cell(Role::Menu, RoleVariant::Normal);
            (
                s.visuals.widgets.inactive.fg_stroke.color,
                s.visuals.window_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.menu.font.color,
                r.menu.background_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "The progress bar's label on its fill",
        site: "egui/src/widgets/progress_bar.rs:159, :197-199",
        emitted: |c| {
            let s = c.cell(Role::ProgressBar, RoleVariant::Normal);
            (
                s.visuals.selection.stroke.color,
                s.visuals.selection.bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.selection_text_color,
                r.progress_bar.fill_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "The progress bar's label on its track",
        site: "egui/src/widgets/progress_bar.rs:140, :197-199",
        emitted: |c| {
            let s = c.cell(Role::ProgressBar, RoleVariant::Normal);
            (
                s.visuals.selection.stroke.color,
                s.visuals.extreme_bg_color,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.selection_text_color,
                r.progress_bar.track_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "The window title, uncoloured, on the title-bar frame",
        site: "egui/src/style.rs:1136-1139; the Surface::WindowTitleBar frame (§5.2)",
        emitted: |c| {
            let s = base(c);
            (
                s.visuals.text_color(),
                c.styles().frame(Surface::WindowTitleBar).fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.text_color,
                r.window.title_bar_background,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "The checkbox label on the panel (B4)",
        site: "egui/src/widget_style.rs:133-136; egui/src/containers/frame.rs:188",
        emitted: |c| {
            let s = c.cell(Role::Checkbox, RoleVariant::Normal);
            (
                s.visuals.text_color(),
                s.visuals.panel_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.checkbox.font.color,
                r.defaults.background_color,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "A pressed list cell's text on its pressed fill",
        site: "egui/src/style.rs:1147-1149; egui/src/widget_style.rs:159",
        emitted: |c| {
            let s = c.cell(Role::List, RoleVariant::Normal);
            (
                s.visuals.strong_text_color(),
                s.visuals.widgets.active.weak_bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.list.header_font.color,
                r.list.hover_background,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "The switch substitute's label, checked, on the checked track",
        site: "egui/src/widget_style.rs:150-154",
        emitted: |c| {
            let s = c.cell(Role::Switch, RoleVariant::Normal);
            (
                s.visuals.selection.stroke.color,
                s.visuals.selection.bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.selection_text_color,
                r.switch.checked_background,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
    Pair {
        what: "The switch substitute's label, unchecked, on the unchecked track",
        site: "egui/src/widget_style.rs:133-136, :159",
        emitted: |c| {
            let s = c.cell(Role::Switch, RoleVariant::Normal);
            (
                s.visuals.widgets.inactive.fg_stroke.color,
                s.visuals.widgets.inactive.weak_bg_fill,
                s.visuals.panel_fill,
            )
        },
        native: |r| {
            (
                r.defaults.text_color,
                r.switch.unchecked_background,
                r.defaults.background_color,
            )
        },
        asserted: false,
        exceptions: &[],
    },
];

const AA: f32 = 4.5;

/// T12 (C8): no assembled pair reads worse than the platform's own — asserted where the
/// connector chose both colours, printed where egui pairs a text colour with a fill from
/// another source; every pair below AA is printed, never asserted (rationale: the platform's own
/// data, `docs/archive/todo_v0.5.9_theme-contracts-rationale.md:526`).
#[test]
fn t12_no_pair_contrasts_worse_than_the_platforms_own() {
    let combinations = combinations();
    let mut failures = Vec::new();
    let mut stale = Vec::new();
    let mut report = Vec::new();
    let mut below_aa = Vec::new();
    for c in &combinations {
        let label = c.label();
        for pair in PAIRS {
            let (fg, bg, surface) = (pair.emitted)(c);
            let emitted = pair_ratio(straight(fg), straight(bg), straight(surface));
            let (nfg, nbg, nsurface) = (pair.native)(&c.resolved);
            let native = pair_ratio(
                straight_native(nfg),
                straight_native(nbg),
                straight_native(nsurface),
            );
            let line = format!(
                "{label}: {} — emitted {emitted:.3}, native {native:.3} ({})",
                pair.what, pair.site
            );
            if emitted < AA {
                below_aa.push(line.clone());
            }
            if !pair.asserted {
                report.push(line);
                continue;
            }
            match pair.exceptions.iter().find(|(key, _)| *key == label) {
                Some((_, why)) => {
                    if emitted >= native {
                        stale.push(format!("{line}: excepted ({why}) yet no longer degrades"));
                    }
                }
                None => {
                    if emitted < native {
                        failures.push(line);
                    }
                }
            }
        }
    }
    println!("printed pairs ({}):\n{}", report.len(), report.join("\n"));
    println!(
        "pairs below AA {AA} ({}):\n{}",
        below_aa.len(),
        below_aa.join("\n")
    );
    assert!(
        failures.is_empty(),
        "{} pair(s) read worse than the platform's own:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(
        stale.is_empty(),
        "{} stale exception(s):\n{}",
        stale.len(),
        stale.join("\n")
    );
}
