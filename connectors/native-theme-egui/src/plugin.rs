//! The install plugin (spec §10.3) and the focus-ring bookkeeping (spec §6.18). This half records, per
//! pass and per viewport, the `Ui`s a role scope styled and the radius each took; the plugin
//! reads the record at the end of the pass.

use std::sync::Arc;

use egui::{Context, CornerRadius, Id, Rect, StrokeKind};

use crate::atlas::FocusRing;
use crate::convert::{clamp_length, to_color32, u8_from_f32_saturating};
use crate::fonts::WidestGlyph;
use crate::style::push_note;
use crate::{ColorMode, Note, ResolvedTheme, ThemeAtlas};

/// Spec §6.18: the ring from `defaults.focus_ring_color`, `focus_ring_width` and
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
/// registered focus shapes (spec §6.18; §4.3). One entry, not one per `Ui`: egui's garbage
/// collection gathers only the elements it can serialize and skips a temporary value
/// (`egui/src/util/id_type_map.rs:712-722`); its `gc` scope walks that gathered map (`:728-729`).
/// So the install plugin drops a viewport's entry itself once egui has closed the viewport.
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
    /// Empties both lists when the entry belongs to an earlier pass (spec §6.18).
    pub(crate) fn begin(&mut self, pass_nr: u64) {
        if self.pass_nr != pass_nr {
            self.scopes.clear();
            self.shapes.clear();
            self.pass_nr = pass_nr;
        }
    }
}

/// The entry's key for the current viewport (spec §4.3): per viewport, because an immediate
/// viewport runs a pass nested inside its parent's and `cumulative_pass_nr` is counted per
/// viewport (`egui/src/context.rs:1781-1793`).
pub(crate) fn focus_key(ctx: &Context) -> Id {
    focus_key_of(ctx.viewport_id())
}

/// The entry's key for `viewport`.
fn focus_key_of(viewport: egui::ViewportId) -> Id {
    Id::new(("native-theme-egui/focus", viewport))
}

/// Record that `ui_id` was styled with a role whose active corner radius is `radius`, for this
/// pass. With `replace`, a second record for the same `Ui` replaces the first (spec §6.18), at the
/// cost of a scan of the pass's records; without it the record is pushed, for a `Ui` created in
/// this pass, whose `unique_id` no earlier record holds (`egui/src/ui.rs:252-260`). The pass
/// number and the key are read before `data_mut` is entered: no `Context` accessor inside
/// another (spec §10.3).
pub(crate) fn record_scope(ctx: &Context, ui_id: Id, radius: CornerRadius, replace: bool) {
    let pass_nr = ctx.cumulative_pass_nr();
    let key = focus_key(ctx);
    ctx.data_mut(|data| {
        let entry = data.get_temp_mut_or_insert_with(key, FocusEntry::default);
        entry.begin(pass_nr);
        let slot = if replace {
            entry.scopes.iter_mut().find(|(id, _)| *id == ui_id)
        } else {
            None
        };
        match slot {
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
/// radius of the root `Ui`'s style at the end of the pass, normally the base style's (spec §6.18).
/// `rect` is the shape the ring surrounds (the ring is still offset from it by
/// `defaults.focus_ring_offset`) and `corner_radius` its radius. Call it after the
/// widget's `Response` exists, in the same pass, from outside any other `Context` accessor
/// closure (the lock discipline of spec §10.3).
///
/// Stored in the viewport's one temporary focus entry (`IdTypeMap::insert_temp`,
/// `egui/src/util/id_type_map.rs:425`), keyed `Id::new(("native-theme-egui/focus",
/// ctx.viewport_id()))` (`egui/src/context.rs:3949`), beside spec §6.18's scope records and that
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

/// How many pixels per em TrueType hinting can add to the size it draws at: skrifa's
/// interpreter rounds the pixels per em to an integer for a face whose `head` does not allow
/// fractional sizes (`skrifa-0.44.0/src/outline/glyf/mod.rs:175-186`), through
/// `F26Dot6::from_f64`, which rounds to the nearest 1/64
/// (`font-types-0.12.4/src/fixed.rs:316-321`), then `F26Dot6::round`, to the nearest integer,
/// half up (`:51-53`): at most 1/2 + 1/128.
const HINTED_PPEM_ROUNDING: f32 = 0.5 + 1.0 / 128.0;

/// [`ThemeAtlas::text_size_cap`]'s derivation (M7-04): the font atlas's width
/// (`epaint/src/text/fonts.rs:975`), less [`GLYPH_BOX_ROUNDING`] and the widest glyph's growth
/// under [`HINTED_PPEM_ROUNDING`], over the width of the widest glyph at one point,
/// `pixels_per_point × widest.em`, floored. `None` where that is not a positive normal `f32`:
/// an atlas too narrow for the slack, a `pixels_per_point` or width that is zero, negative or
/// not finite, or a cap below one point.
///
/// Hinting moves outline points under the target epaint hints for by default
/// (`HintingTarget::default`, `epaint/src/text/fonts.rs:310-313`: smooth, not light,
/// `preserve_linear_metrics`, `:376-384`), each engine chosen by skrifa's `AutoFallback`
/// (`skrifa-0.44.0/src/outline/hint.rs:27-34`), within these limits: the CFF hinter hints only
/// `y` and scales `x` (`cff/hint.rs:1055-1078`); the autohinter fits nothing horizontally for a
/// script of the default group (`autohint/metrics/mod.rs:364`, `:380-386`); the TrueType
/// interpreter runs every glyph program in backward-compatibility mode under
/// `preserve_linear_metrics` (`glyf/hint/engine/dispatch.rs:41-43`), in which its two point
/// movers move nothing horizontally (`glyf/hint/zone.rs:417-468`, `:476-505`). Not covered,
/// because the source states no bound for them: the TrueType `ISECT` instruction, which sets a
/// point to the intersection of two lines with no backward-compatibility check
/// (`glyf/hint/engine/outline.rs:611-663`); the autohinter's other script groups (CJK, Indic),
/// fitted horizontally; and a face whose `FontTweak::hinting_target` the application changed.
pub(crate) fn derive_text_size_cap(
    max_texture_side: usize,
    pixels_per_point: f32,
    widest: WidestGlyph,
) -> Option<f32> {
    let side = max_texture_side
        .min(FONT_ATLAS_MAX_WIDTH)
        .saturating_sub(GLYPH_BOX_ROUNDING);
    let side = f32::from(u16::try_from(side).ok()?); // at most 16382: exact
    let width = side - HINTED_PPEM_ROUNDING * widest.unscaled_em;
    let cap = (width / (pixels_per_point * widest.em)).floor();
    (cap.is_normal() && cap >= 1.0).then_some(cap)
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
pub(crate) fn cap_text_sizes(
    ctx: &Context,
    atlas: &ThemeAtlas,
    max_texture_side: usize,
    pixels_per_point: f32,
) -> Option<f32> {
    let cap = atlas
        .widest_glyph()
        .and_then(|widest| derive_text_size_cap(max_texture_side, pixels_per_point, widest));
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

/// [`ThemeAtlas::install`]'s lowering (M7-04), from the inputs of the pass the sizes will be
/// drawn in: inside a pass this plugin saw begin, that pass's own, as `input_hook` recorded
/// them; otherwise, once the viewport has completed a pass (`Context::cumulative_pass_nr`,
/// `egui/src/context.rs:1781`), the last pass's, which the `Context`'s input keeps. Before the
/// first pass nothing is lowered: the input then holds only egui's defaults, a 2048 px texture
/// and one pixel per point (`egui/src/input_state/mod.rs:348-349`), and lowering is one-way,
/// so a cap from them would hold the sizes down on a wider GPU; nothing is drawn before that
/// pass, whose `input_hook` lowers them. Not covered: a `Context`'s first install made inside
/// its very first pass, which this plugin did not see begin and which has completed no pass,
/// lowers nothing until the next pass.
pub(crate) fn cap_at_install(ctx: &Context, atlas: &ThemeAtlas) {
    let in_pass = ctx
        .with_plugin(|plugin: &mut NativeThemePlugin| {
            plugin.passes.last().map(|&(_, side, ppp)| (side, ppp))
        })
        .flatten();
    let inputs = in_pass.or_else(|| {
        (ctx.cumulative_pass_nr() > 0)
            .then(|| ctx.input(|i| (i.max_texture_side, i.pixels_per_point())))
    });
    if let Some((max_texture_side, pixels_per_point)) = inputs {
        cap_text_sizes(ctx, atlas, max_texture_side, pixels_per_point);
    }
}

/// The install plugin of spec §10.3: the OS colour scheme where the integration reports none, the
/// title bar that follows it, the focus ring of spec §6.18 and the text-size cap of M7-04
/// ([`ThemeAtlas::text_size_cap`]). It holds no theme data — every hook
/// reads the atlas `install` published, so a re-install takes effect with no second
/// registration (egui keeps the first plugin of a type, `egui/src/context.rs:2045`,
/// `egui/src/plugin.rs:206-210`) — only its own bookkeeping: which viewports' input it filled
/// this pass, the last window theme it sent per viewport, and the cap inputs of each pass in
/// progress.
#[derive(Default)]
pub(crate) struct NativeThemePlugin {
    filled: egui::ViewportIdMap<bool>,
    last_sent: egui::ViewportIdMap<egui::SystemTheme>,
    /// The `max_texture_side` and `pixels_per_point` of each pass in progress, innermost last:
    /// an immediate viewport runs its whole pass, hooks included, inside its parent's
    /// (`egui/src/context.rs:2799-2811`), so the cap one pass derives is put back to its
    /// parent's when it ends (M7-04).
    passes: Vec<(egui::ViewportId, usize, f32)>,
}

impl NativeThemePlugin {
    /// Drop the focus entry of every viewport whose input this plugin saw and that egui no
    /// longer has, as egui drops its own state of a closed viewport
    /// (`egui/src/context.rs:2756-2790`, `:2829-2835`), and the viewport from `filled`, which
    /// is the list scanned. egui has no accessor that lists its viewports; it counts one it no
    /// longer has as having completed no pass (`Context::cumulative_pass_nr_for`,
    /// `egui/src/context.rs:1788-1793`). A viewport whose input this plugin saw has begun a
    /// pass, so one still open has completed it unless that pass is in progress now, which
    /// only the viewports in `passes` can be. Call after this pass is pushed.
    fn forget_closed_viewports(&mut self, ctx: &Context) {
        let passes = &self.passes;
        let mut closed = Vec::new();
        self.filled.retain(|&viewport, _| {
            let open = viewport == egui::ViewportId::ROOT
                || passes.iter().any(|p| p.0 == viewport)
                || ctx.cumulative_pass_nr_for(viewport) > 0;
            if !open {
                closed.push(viewport);
            }
            open
        });
        if closed.is_empty() {
            return;
        }
        ctx.data_mut(|data| {
            for viewport in closed {
                data.remove::<FocusEntry>(focus_key_of(viewport));
            }
        });
    }
}

/// The atlas's OS mode, read from `Context` data (spec §10.3 step 3).
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
        let pixels_per_point = native * ctx.zoom_factor();
        // A pass of this viewport still listed never reached `output_hook`: it and any pass
        // nested in it are over.
        if let Some(stale) = self.passes.iter().position(|p| p.0 == input.viewport_id) {
            self.passes.truncate(stale);
        }
        self.passes
            .push((input.viewport_id, max_texture_side, pixels_per_point));
        self.forget_closed_viewports(ctx);
        if let Some(atlas) = atlas {
            cap_text_sizes(ctx, &atlas, max_texture_side, pixels_per_point);
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
        if let Some(this) = self.passes.last_mut() {
            (this.1, this.2) = (max_texture_side, pixels_per_point);
        }
        let derived = atlas
            .widest_glyph()
            .and_then(|widest| derive_text_size_cap(max_texture_side, pixels_per_point, widest));
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
    /// egui sends again only when the *preference* changes (spec §10.3).
    ///
    /// The inverse, under the same two conditions, once the atlas has no OS mode (a preset, or
    /// after [`ThemeAtlas::clear`]): a viewport this hook last told an explicit scheme gets
    /// `SetTheme(SystemTheme::SystemDefault)` once — the command egui believes it sent and will
    /// not send again — unless egui's own already rides in the output, and its `last_sent`
    /// entry is dropped, so the window and egui agree and the plugin is inert (M7-02). Not
    /// `SetTheme(ctx.theme())`: a preset pins its scheme through `Context::set_theme`, which
    /// egui's `sync_window_theme` sends itself. On X11 winit writes `_GTK_THEME_VARIANT =
    /// "dark"` for `SystemDefault` whatever the desktop's scheme (spec §10.3's table, from
    /// winit 0.30.13's `src/platform_impl/linux/x11/window.rs`, the `None` arm at line 638), so
    /// with `Options::fallback_theme = Light` the title bar goes dark under a light UI after
    /// this restore: egui's own behaviour from startup, which the restore brings back.
    ///
    /// The `filled` map is read, never emptied here: an immediate viewport runs its whole pass,
    /// hooks included, inside its parent's, and the parent's commands reach the outermost pass's
    /// output (`egui/src/context.rs:2799-2811`); `input_hook` overwrites each viewport's entry
    /// every pass, and drops the entry of a viewport egui has closed.
    ///
    /// First, the pass ending leaves the list of passes in progress, and where it was an
    /// immediate viewport's, nested in its parent's, the parent's cap is derived again from the
    /// parent's own inputs (M7-04): a child on a monitor of fewer pixels per point derives a
    /// larger cap, which the parent's role scopes after it in the same pass would otherwise read.
    fn output_hook(&mut self, ctx: &Context, output: &mut egui::FullOutput) {
        self.passes.pop();
        if let (Some(&(_, max_texture_side, pixels_per_point)), Some(atlas)) =
            (self.passes.last(), ThemeAtlas::from_ctx(ctx))
        {
            cap_text_sizes(ctx, &atlas, max_texture_side, pixels_per_point);
        }
        let filled = &self.filled;
        let (sync, follows_os) = ctx.options(|o| {
            (
                o.sync_window_theme,
                o.theme_preference == egui::ThemePreference::System,
            )
        });
        if !sync || !follows_os {
            return;
        }
        let Some(mode) = os_mode(ctx) else {
            let system_default = egui::ViewportCommand::SetTheme(egui::SystemTheme::SystemDefault);
            for (viewport, out) in output.viewport_output.iter_mut() {
                if self.last_sent.remove(viewport).is_some()
                    && !out.commands.contains(&system_default)
                {
                    out.commands.push(system_default.clone());
                }
            }
            return;
        };
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
/// rect contains the widget's `interact_rect` (spec §6.18): of the matches, the `Ui` last in the
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

/// Spec §6.18. The focused widget is read from this pass's widget table
/// (`egui/src/context.rs:3963-3965`, `egui/src/widget_rect.rs:127-129`), never from
/// `read_response`, which falls back to the previous pass's rect (spec §6.18). `ui` is the root `Ui`
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
    // `get_temp` clones both lists, once per pass while a widget has the focus: a read by
    // reference measured no gain above the run-to-run spread with 1000 scopes in a pass (M7-03).
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

    /// A widest glyph of `em` with no `FontTweak::scale`.
    fn widest(em: f32) -> WidestGlyph {
        WidestGlyph {
            em,
            unscaled_em: em,
        }
    }

    /// M7-04: the cap is the atlas width less two pixels and the hinted size's rounding over
    /// the widest glyph's width at one point, floored; the atlas is never wider than 16 × 1024
    /// (`epaint/src/text/fonts.rs:975`).
    #[test]
    fn the_text_size_cap_is_the_atlas_width_over_the_widest_glyph() {
        // 2048 − 2 − 65/128 = 2045.49…, over 1.0
        assert_eq!(derive_text_size_cap(2048, 1.0, widest(1.0)), Some(2045.0));
        assert_eq!(derive_text_size_cap(2048, 2.0, widest(1.0)), Some(1022.0));
        // (2046 − 4 × 65/128) / 4 = 511.0, less than 511 by the rounding: 510
        assert_eq!(derive_text_size_cap(2048, 1.0, widest(4.0)), Some(510.0));
        // A `FontTweak::scale` of 0.5: the rounding is on the unscaled width.
        let half = WidestGlyph {
            em: 0.5,
            unscaled_em: 1.0,
        };
        assert_eq!(derive_text_size_cap(2048, 1.0, half), Some(4090.0));
        assert_eq!(
            derive_text_size_cap(1 << 20, 1.0, widest(1.0)),
            Some(16381.0)
        );
        assert_eq!(
            derive_text_size_cap(16 * 1024, 1.0, widest(1.0)),
            Some(16381.0)
        );
        // Nothing to divide by, or no width: no cap.
        assert_eq!(derive_text_size_cap(2, 1.0, widest(1.0)), None);
        assert_eq!(derive_text_size_cap(0, 1.0, widest(1.0)), None);
        assert_eq!(derive_text_size_cap(2048, 0.0, widest(1.0)), None);
        assert_eq!(derive_text_size_cap(2048, f32::NAN, widest(1.0)), None);
        assert_eq!(derive_text_size_cap(2048, 1.0, widest(f32::INFINITY)), None);
        assert_eq!(derive_text_size_cap(2048, -1.0, widest(1.0)), None);
        // A cap below one point: none.
        assert_eq!(derive_text_size_cap(2048, 1.0, widest(4096.0)), None);
        assert_eq!(derive_text_size_cap(2048, 4096.0, widest(1.0)), None);
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
