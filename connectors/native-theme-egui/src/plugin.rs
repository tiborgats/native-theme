//! The install plugin (§10.3) and the focus-ring bookkeeping (§6.18). This half records, per
//! pass and per viewport, the `Ui`s a role scope styled and the radius each took; the plugin
//! reads the record at the end of the pass.

use std::sync::Arc;

use egui::{Context, CornerRadius, Id, Rect, StrokeKind};

use crate::atlas::FocusRing;
use crate::convert::{clamp_length, to_color32, u8_from_f32_saturating};
use crate::style::push_note;
use crate::{ColorMode, Note, ResolvedTheme, ThemeAtlas};

/// §6.18: the ring from `defaults.focus_ring_color`, `focus_ring_width` and
/// `focus_ring_offset`. A non-finite width or offset is egui's own value — no ring — with a
/// `Note::ValueSanitised` per leaf; a stated zero width is no ring and no note, as gpui's
/// `focus_ring = width > 0.0` (`connectors/native-theme-gpui/src/lib.rs:227`). The offset is
/// kept signed and unclamped: negative means inside the edge (`docs/platform-facts.md:1106`).
pub(crate) fn build_focus_ring(theme: &ResolvedTheme, notes: &mut Vec<Note>) -> Option<FocusRing> {
    let (color, width, offset) = (
        theme.defaults.focus_ring_color,
        theme.defaults.focus_ring_width,
        theme.defaults.focus_ring_offset,
    );
    let mut finite = true;
    if !width.is_finite() {
        push_note(
            notes,
            Note::ValueSanitised {
                path: "defaults.focus_ring_width",
            },
        );
        finite = false;
    }
    if !offset.is_finite() {
        push_note(
            notes,
            Note::ValueSanitised {
                path: "defaults.focus_ring_offset",
            },
        );
        finite = false;
    }
    if !finite || width <= 0.0 {
        return None;
    }
    Some(FocusRing {
        stroke: egui::Stroke::new(clamp_length(width), to_color32(color)),
        offset,
    })
}

/// The one temporary `Context` data entry per viewport holding this pass's scope records and
/// registered focus shapes (§6.18; §4.3). One entry, not one per `Ui`: egui's garbage
/// collection gathers only the elements it can serialize and skips a temporary value
/// (`egui/src/util/id_type_map.rs:712-722`); its `gc` scope walks that gathered map (`:728-729`).
#[derive(Clone, Default)]
pub(crate) struct FocusEntry {
    /// `Context::cumulative_pass_nr` of the pass the lists belong to.
    pub(crate) pass_nr: u64,
    /// Each styled `Ui`'s `unique_id` with its role style's `widgets.active.corner_radius`.
    pub(crate) scopes: Vec<(Id, CornerRadius)>,
    /// `register_focus_shape`'s outlines: widget id, rect, radius.
    pub(crate) shapes: Vec<(Id, Rect, CornerRadius)>,
}

impl FocusEntry {
    /// Empties both lists when the entry belongs to an earlier pass (§6.18).
    pub(crate) fn begin(&mut self, pass_nr: u64) {
        if self.pass_nr != pass_nr {
            self.scopes.clear();
            self.shapes.clear();
            self.pass_nr = pass_nr;
        }
    }
}

/// The entry's key for the current viewport (§4.3): per viewport, because an immediate
/// viewport runs a pass nested inside its parent's and `cumulative_pass_nr` is counted per
/// viewport (`egui/src/context.rs:1781-1793`).
pub(crate) fn focus_key(ctx: &Context) -> Id {
    Id::new(("native-theme-egui/focus", ctx.viewport_id()))
}

/// Record that `ui_id` was styled with a role whose active corner radius is `radius`, for this
/// pass. A second record for the same `Ui` replaces the first (§6.18). The pass number and the
/// key are read before `data_mut` is entered: no `Context` accessor inside another (§10.3).
pub(crate) fn record_scope(ctx: &Context, ui_id: Id, radius: CornerRadius) {
    let pass_nr = ctx.cumulative_pass_nr();
    let key = focus_key(ctx);
    ctx.data_mut(|data| {
        let entry = data.get_temp_mut_or_insert_with(key, FocusEntry::default);
        entry.begin(pass_nr);
        match entry.scopes.iter_mut().find(|(id, _)| *id == ui_id) {
            Some(slot) => slot.1 = radius,
            None => entry.scopes.push((ui_id, radius)),
        }
    });
}

/// Tell the install plugin the outline of widget `id`'s focus ring for **this pass**, when
/// it is not a rounded rectangle around the widget's own rect — a switch's track inside a
/// widget that also holds its label, a composite whose focus belongs to one part. A widget
/// whose outline is its rect needs no call: the ring takes the corner radius of the
/// innermost role scope around it ([`NativeThemeUiExt::native_scope`](crate::NativeThemeUiExt::native_scope)), or outside any the
/// radius of the root `Ui`'s style at the end of the pass, normally the base style's (§6.18).
/// `rect` is the shape the ring surrounds (the ring is still offset from it by
/// `defaults.focus_ring_offset`) and `corner_radius` its radius. Call it after the
/// widget's `Response` exists, in the same pass, from outside any other `Context` accessor
/// closure (the lock discipline of §10.3).
///
/// Stored in the viewport's one temporary focus entry (`IdTypeMap::insert_temp`,
/// `egui/src/util/id_type_map.rs:425`), keyed `Id::new(("native-theme-egui/focus",
/// ctx.viewport_id()))` (`egui/src/context.rs:3949`), beside §6.18's scope records and that
/// viewport's `ctx.cumulative_pass_nr()` (`egui/src/context.rs:1781-1793`). The first registration of a new
/// pass empties the entry's lists, and the plugin reads them only in the pass they hold, so
/// a shape registered in an earlier pass — by a widget no longer drawn, or drawn elsewhere —
/// is ignored, and the entry never holds more than one pass's shapes. A `rect` that is not finite (`Rect::is_finite`,
/// `emath/src/rect.rs:524`) is not stored. Needs no atlas: without an install plugin the
/// entry is simply never read.
pub fn register_focus_shape(ctx: &Context, id: Id, rect: Rect, corner_radius: CornerRadius) {
    if !rect.is_finite() {
        return;
    }
    let pass_nr = ctx.cumulative_pass_nr();
    let key = focus_key(ctx);
    ctx.data_mut(|data| {
        let entry = data.get_temp_mut_or_insert_with(key, FocusEntry::default);
        entry.begin(pass_nr);
        entry.shapes.push((id, rect, corner_radius));
    });
}

/// epaint's font-atlas width bound: `max_texture_side.at_most(16 * 1024)`
/// (`epaint/src/text/fonts.rs:975`).
const FONT_ATLAS_MAX_WIDTH: usize = 16 * 1024;

/// The pixels `path.control_box().expand()` can add to a glyph outline's width
/// (`epaint/src/text/font.rs:266-267`): kurbo's `Rect::expand` floors one side and ceils the
/// other outward (`kurbo-0.13.1/src/rect.rs:430-442`, the kurbo epaint reaches through
/// `vello_cpu`), adding less than one pixel on each.
const GLYPH_BOX_ROUNDING: usize = 2;

/// [`ThemeAtlas::text_size_cap`]'s derivation (M7-04): the font atlas's width
/// (`epaint/src/text/fonts.rs:975`) less [`GLYPH_BOX_ROUNDING`], over the width of the widest
/// glyph at one point, `pixels_per_point × widest_glyph_em`, floored. `None` where that is not
/// a positive normal `f32`: a `max_texture_side` of two pixels or fewer, or a
/// `pixels_per_point` or width that is zero, negative or not finite.
pub(crate) fn derive_text_size_cap(
    max_texture_side: usize,
    pixels_per_point: f32,
    widest_glyph_em: f32,
) -> Option<f32> {
    let width = max_texture_side
        .min(FONT_ATLAS_MAX_WIDTH)
        .saturating_sub(GLYPH_BOX_ROUNDING);
    let width = f32::from(u16::try_from(width).ok()?); // at most 16382: exact
    let cap = (width / (pixels_per_point * widest_glyph_em)).floor();
    (cap.is_normal() && cap > 0.0).then_some(cap)
}

/// Whether a text size of `style` — a `text_styles` entry or `override_font_id` — is above `cap`.
fn exceeds_cap(style: &egui::Style, cap: f32) -> bool {
    style
        .text_styles
        .values()
        .chain(style.override_font_id.as_ref())
        .any(|font| font.size > cap)
}

/// Lower every text size of `style` above `cap` to it (M7-04).
pub(crate) fn clamp_text_sizes(style: &mut egui::Style, cap: f32) {
    for font in style
        .text_styles
        .values_mut()
        .chain(style.override_font_id.as_mut())
    {
        if font.size > cap {
            font.size = cap;
        }
    }
}

/// `style` with every text size above `cap` lowered to it: the same `Arc` when none is, a
/// copy only in that pathological case (M7-04).
pub(crate) fn capped(style: Arc<egui::Style>, cap: Option<f32>) -> Arc<egui::Style> {
    match cap {
        Some(cap) if exceeds_cap(&style, cap) => {
            let mut next = egui::Style::clone(&style);
            clamp_text_sizes(&mut next, cap);
            Arc::new(next)
        }
        _ => style,
    }
}

/// Derive the atlas's text-size cap for a pass, store it, and lower every size above it in both
/// base styles the `Context` holds (M7-04). `style_of` (`egui/src/context.rs:2221`) is an `Arc`
/// clone; `set_style_of` (`:2250`) runs only for a style with a size above the cap, so a pass
/// writes nothing unless a size needs lowering. Checked every pass, not only when an input
/// changed: a re-install of the same atlas writes its uncapped base styles again.
fn cap_text_sizes(
    ctx: &Context,
    atlas: &ThemeAtlas,
    max_texture_side: usize,
    pixels_per_point: f32,
) -> Option<f32> {
    let cap = atlas
        .widest_glyph_em()
        .and_then(|em| derive_text_size_cap(max_texture_side, pixels_per_point, em));
    atlas.set_text_size_cap(cap);
    if let Some(cap) = cap {
        for theme in [egui::Theme::Dark, egui::Theme::Light] {
            let style = ctx.style_of(theme);
            if exceeds_cap(&style, cap) {
                ctx.set_style_of(theme, capped(style, Some(cap)));
            }
        }
    }
    cap
}

/// The install plugin of §10.3: the OS colour scheme where the integration reports none, the
/// title bar that follows it, the focus ring of §6.18 and the text-size cap of M7-04
/// ([`ThemeAtlas::text_size_cap`]). It holds no theme data — every hook
/// reads the atlas `install` published, so a re-install takes effect with no second
/// registration (egui keeps the first plugin of a type, `egui/src/context.rs:2045`,
/// `egui/src/plugin.rs:206-210`) — only its own bookkeeping: which viewports' input it filled
/// this pass, and the last window theme it sent per viewport.
#[derive(Default)]
pub(crate) struct NativeThemePlugin {
    filled: egui::ViewportIdMap<bool>,
    last_sent: egui::ViewportIdMap<egui::SystemTheme>,
}

/// The atlas's OS mode, read from `Context` data (§10.3 step 3).
fn os_mode(ctx: &Context) -> Option<ColorMode> {
    ThemeAtlas::from_ctx(ctx).and_then(|atlas| atlas.os_mode())
}

impl egui::plugin::Plugin for NativeThemePlugin {
    fn debug_name(&self) -> &'static str {
        "native-theme-egui"
    }

    /// `RawInput::system_theme` is filled from the atlas's OS mode **only when it is `None`**;
    /// the hook runs before `Options::begin_pass` copies the field
    /// (`egui/src/context.rs:966-968`, `egui/src/memory/mod.rs:358-359`). The viewport is the
    /// input's own: `Context::viewport_id` is not for use outside a pass.
    ///
    /// Then, before egui builds the pass's root `Ui` from the `Context`'s style
    /// (`egui/src/context.rs:802-808`, after `begin_pass` ran this hook at `:966`), the
    /// text-size cap of M7-04 for this pass: from the input's `max_texture_side`, or where it
    /// gives none the viewport's last (`Context::input_for`, `egui/src/context.rs:997`; egui
    /// keeps it, `egui/src/input_state/mod.rs:482`), and `pixels_per_point` as egui forms it,
    /// `native_pixels_per_point` (`1.0` where none is given) × `zoom_factor`
    /// (`egui/src/context.rs:451-455`).
    fn input_hook(&mut self, ctx: &Context, input: &mut egui::RawInput) {
        let atlas = ThemeAtlas::from_ctx(ctx);
        let filled = match (
            input.system_theme,
            atlas.as_ref().and_then(ThemeAtlas::os_mode),
        ) {
            (None, Some(mode)) => {
                input.system_theme = Some(if mode.is_dark() {
                    egui::Theme::Dark
                } else {
                    egui::Theme::Light
                });
                true
            }
            _ => false,
        };
        self.filled.insert(input.viewport_id, filled);
        if let Some(atlas) = atlas {
            let max_texture_side = input
                .max_texture_side
                .unwrap_or_else(|| ctx.input_for(input.viewport_id, |i| i.max_texture_side));
            // `RawInput::viewport` panics on a missing entry
            // (`egui/src/data/input/raw_input.rs:116-118`), so `get`.
            let native = input
                .viewports
                .get(&input.viewport_id)
                .and_then(|viewport| viewport.native_pixels_per_point)
                .unwrap_or(1.0);
            cap_text_sizes(ctx, &atlas, max_texture_side, native * ctx.zoom_factor());
        }
    }

    /// A zoom set during the last pass takes effect inside `begin_pass`
    /// (`egui/src/context.rs:438-441`), after `input_hook` ran with the previous
    /// `zoom_factor`, so the cap is derived again from the pass's own `pixels_per_point`; where
    /// it differs, the base styles and this root `Ui`'s style, already built from them, are
    /// lowered to it (M7-04). An application that runs passes with `Context::begin_pass` rather
    /// than `Context::run_ui` gets no `on_begin_pass` (`egui/src/context.rs:811`), and its cap
    /// follows a zoom change one pass late.
    fn on_begin_pass(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let Some(atlas) = ThemeAtlas::from_ctx(&ctx) else {
            return;
        };
        let (max_texture_side, pixels_per_point) =
            ctx.input(|i| (i.max_texture_side, i.pixels_per_point()));
        let derived = atlas
            .widest_glyph_em()
            .and_then(|em| derive_text_size_cap(max_texture_side, pixels_per_point, em));
        if derived == atlas.text_size_cap() {
            return;
        }
        if let Some(cap) = cap_text_sizes(&ctx, &atlas, max_texture_side, pixels_per_point)
            && exceeds_cap(ui.style(), cap)
        {
            clamp_text_sizes(ui.style_mut(), cap);
        }
    }

    /// In a pass whose `input_hook` filled the scheme, while `Options::sync_window_theme` holds
    /// and the preference is `ThemePreference::System`: every
    /// `ViewportCommand::SetTheme(SystemTheme::SystemDefault)` egui's `sync_window_theme` sent
    /// (`egui/src/context.rs:2474-2497`, at `:2456`, before this hook at `:2464`) becomes
    /// `SetTheme(<the OS mode>)`, and when the mode differs from the one last sent for that
    /// viewport — a rebuilt atlas after the desktop switched — the command is appended, because
    /// egui sends again only when the *preference* changes (§10.3).
    ///
    /// The `filled` map is read, never emptied: an immediate viewport runs its whole pass, hooks
    /// included, inside its parent's, and the parent's commands reach the outermost pass's
    /// output (`egui/src/context.rs:2799-2811`); `input_hook` overwrites each viewport's entry
    /// every pass.
    fn output_hook(&mut self, ctx: &Context, output: &mut egui::FullOutput) {
        let filled = &self.filled;
        let Some(mode) = os_mode(ctx) else {
            return;
        };
        let (sync, follows_os) = ctx.options(|o| {
            (
                o.sync_window_theme,
                o.theme_preference == egui::ThemePreference::System,
            )
        });
        if !sync || !follows_os {
            return;
        }
        let want = if mode.is_dark() {
            egui::SystemTheme::Dark
        } else {
            egui::SystemTheme::Light
        };
        for (viewport, out) in output.viewport_output.iter_mut() {
            if filled.get(viewport) != Some(&true) {
                continue;
            }
            let mut carried = false;
            for command in &mut out.commands {
                if let egui::ViewportCommand::SetTheme(theme) = command {
                    if *theme == egui::SystemTheme::SystemDefault {
                        *theme = want;
                    }
                    if *theme == want {
                        carried = true;
                    }
                }
            }
            if !carried && self.last_sent.get(viewport) != Some(&want) {
                out.commands.push(egui::ViewportCommand::SetTheme(want));
                carried = true;
            }
            if carried {
                self.last_sent.insert(*viewport, want);
            }
        }
    }

    fn on_end_pass(&mut self, ui: &mut egui::Ui) {
        paint_focus_ring(ui);
    }
}

/// The widget's radius grown by the ring's offset: the concentric curve at the ring's inner
/// edge (epaint grows the outer corner by the stroke width itself,
/// `epaint/src/tessellator.rs:1894-1897`), floored at `0` and saturated at `255`.
fn grow(radius: CornerRadius, by: f32) -> CornerRadius {
    let g = |c: u8| u8_from_f32_saturating(f32::from(c) + by);
    CornerRadius {
        nw: g(radius.nw),
        ne: g(radius.ne),
        sw: g(radius.sw),
        se: g(radius.se),
    }
}

/// The active corner radius of the innermost recorded scope on the widget's layer whose final
/// rect contains the widget's `interact_rect` (§6.18): of the matches, the `Ui` last in the
/// layer's order (`egui/src/widget_rect.rs:132-134`), since a `Ui` is registered before every
/// `Ui` it contains (`egui/src/ui.rs:170-183`). The root `Ui`'s entry still holds
/// `Rect::NOTHING` during `on_end_pass` (`egui/src/ui.rs:171`), so its record never matches.
fn scope_radius(
    ctx: &Context,
    entry: &FocusEntry,
    widget: &egui::WidgetRect,
) -> Option<CornerRadius> {
    ctx.viewport(|viewport| {
        let widgets = &viewport.this_pass.widgets;
        entry
            .scopes
            .iter()
            .filter_map(|(ui_id, radius)| {
                let (layer, order) = widgets.order(*ui_id)?;
                let rect = widgets.get(*ui_id)?.rect;
                (layer == widget.layer_id && rect.contains_rect(widget.interact_rect))
                    .then_some((order, *radius))
            })
            .max_by_key(|(order, _)| *order)
            .map(|(_, radius)| radius)
    })
}

/// §6.18. The focused widget is read from this pass's widget table
/// (`egui/src/context.rs:3963-3965`, `egui/src/widget_rect.rs:127-129`), never from
/// `read_response`, which falls back to the previous pass's rect (§6.18). `ui` is the root `Ui`
/// `on_end_pass` receives: its own style is the radius where no record matches.
fn paint_focus_ring(ui: &egui::Ui) {
    let ctx = ui.ctx();
    let Some(atlas) = ThemeAtlas::from_ctx(ctx) else {
        return;
    };
    let theme = ctx.theme();
    let Some(ring) = atlas.focus_ring(theme) else {
        return;
    };
    if !ctx.input(|i| i.focused) {
        return; // the window has no OS keyboard focus (`egui/src/input_state/mod.rs:315`)
    }
    let Some(id) = ctx.memory(|m| m.focused()) else {
        return;
    };
    let Some(widget) = ctx.viewport(|v| v.this_pass.widgets.get(id).copied()) else {
        return;
    };
    let pass_nr = ctx.cumulative_pass_nr();
    let key = focus_key(ctx);
    let entry = ctx
        .data(|data| data.get_temp::<FocusEntry>(key))
        .filter(|entry| entry.pass_nr == pass_nr)
        .unwrap_or_default();
    let (edge, radius) = match entry.shapes.iter().find(|(shape_id, _, _)| *shape_id == id) {
        Some((_, rect, radius)) => (*rect, *radius),
        None => (
            widget.rect,
            scope_radius(ctx, &entry, &widget)
                .unwrap_or(ui.style().visuals.widgets.active.corner_radius),
        ),
    };
    let rect = edge.expand(ring.offset);
    if !rect.is_positive() {
        return; // an inset larger than the widget
    }
    let corners = grow(radius, ring.offset);
    let clip = widget
        .interact_rect
        .expand((ring.offset + ring.stroke.width).max(0.0));
    ctx.layer_painter(widget.layer_id)
        .with_clip_rect(clip)
        .rect_stroke(rect, corners, ring.stroke, StrokeKind::Outside);
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;

    fn theme_with(width: f32, offset: f32) -> crate::ResolvedTheme {
        let mut theme = native_theme::theme::Theme::preset("kde-breeze")
            .expect("a bundled preset")
            .resolve_with(
                crate::ColorMode::Light,
                &native_theme::ResolutionContext::for_tests(),
            )
            .expect("the light variant resolves")
            .variant;
        theme.defaults.focus_ring_width = width;
        theme.defaults.focus_ring_offset = offset;
        theme
    }

    #[test]
    fn a_zero_negative_or_non_finite_ring_is_none() {
        let mut notes = Vec::new();
        assert!(build_focus_ring(&theme_with(0.0, 2.0), &mut notes).is_none());
        assert!(build_focus_ring(&theme_with(-1.0, 2.0), &mut notes).is_none());
        assert!(
            notes.is_empty(),
            "a stated zero or negative width is no sanitised value"
        );
        assert!(build_focus_ring(&theme_with(f32::NAN, 2.0), &mut notes).is_none());
        assert_eq!(
            notes,
            vec![Note::ValueSanitised {
                path: "defaults.focus_ring_width"
            }]
        );
        // The other scheme's build reports the same leaf into the same `Vec`: once per atlas (§7.2).
        assert!(build_focus_ring(&theme_with(f32::NAN, 2.0), &mut notes).is_none());
        assert_eq!(
            notes,
            vec![Note::ValueSanitised {
                path: "defaults.focus_ring_width"
            }]
        );
        notes.clear();
        assert!(build_focus_ring(&theme_with(1.0, f32::INFINITY), &mut notes).is_none());
        assert_eq!(
            notes,
            vec![Note::ValueSanitised {
                path: "defaults.focus_ring_offset"
            }]
        );
    }

    /// M7-04: the cap is the atlas width less two pixels over the widest glyph's width at one
    /// point, floored; the atlas is never wider than 16 × 1024 (`epaint/src/text/fonts.rs:975`).
    #[test]
    fn the_text_size_cap_is_the_atlas_width_over_the_widest_glyph() {
        assert_eq!(derive_text_size_cap(2048, 1.0, 1.0), Some(2046.0));
        assert_eq!(derive_text_size_cap(2048, 2.0, 1.0), Some(1023.0));
        assert_eq!(derive_text_size_cap(2048, 1.0, 4.0), Some(511.0)); // 511.5, floored
        assert_eq!(derive_text_size_cap(2048, 1.5, 0.5), Some(2728.0));
        assert_eq!(derive_text_size_cap(1 << 20, 1.0, 1.0), Some(16382.0));
        assert_eq!(derive_text_size_cap(16 * 1024, 1.0, 1.0), Some(16382.0));
        // Nothing to divide by, or no width: no cap.
        assert_eq!(derive_text_size_cap(2, 1.0, 1.0), None);
        assert_eq!(derive_text_size_cap(0, 1.0, 1.0), None);
        assert_eq!(derive_text_size_cap(2048, 0.0, 1.0), None);
        assert_eq!(derive_text_size_cap(2048, f32::NAN, 1.0), None);
        assert_eq!(derive_text_size_cap(2048, 1.0, f32::INFINITY), None);
        assert_eq!(derive_text_size_cap(2048, -1.0, 1.0), None);
        // A width below one point at the cap's scale floors to zero: no cap.
        assert_eq!(derive_text_size_cap(2048, 1.0, 4096.0), None);
    }

    /// M7-04: `capped` hands the same `Arc` back when no size is above the cap, and lowers
    /// exactly the sizes above it — `text_styles` and `override_font_id` alike.
    #[test]
    fn capped_lowers_only_the_sizes_above_the_cap() {
        let style = Arc::new(egui::Style::default());
        assert!(Arc::ptr_eq(&capped(Arc::clone(&style), None), &style));
        assert!(Arc::ptr_eq(
            &capped(Arc::clone(&style), Some(1000.0)),
            &style
        ));
        let mut big = egui::Style {
            override_font_id: Some(egui::FontId::proportional(500.0)),
            ..egui::Style::default()
        };
        if let Some(body) = big.text_styles.get_mut(&egui::TextStyle::Body) {
            body.size = 400.0;
        }
        let out = capped(Arc::new(big), Some(300.0));
        assert_eq!(out.override_font_id.as_ref().map(|f| f.size), Some(300.0));
        let own = egui::Style::default();
        for (slot, font) in &out.text_styles {
            let want = if *slot == egui::TextStyle::Body {
                300.0
            } else {
                own.text_styles[slot].size
            };
            assert_eq!(font.size, want, "{slot:?}");
        }
    }

    #[test]
    fn a_stated_ring_keeps_its_signed_offset() {
        let mut notes = Vec::new();
        let theme = theme_with(2.0, -2.0);
        let ring = build_focus_ring(&theme, &mut notes).expect("a ring");
        assert_eq!(
            ring.stroke,
            egui::Stroke::new(2.0, to_color32(theme.defaults.focus_ring_color))
        );
        assert_eq!(ring.offset, -2.0);
        assert!(notes.is_empty());
    }
}
