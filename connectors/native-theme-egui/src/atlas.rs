//! The atlas handle, its builder and its diagnostics (spec §4.2, §4.3). Task 10: `Note`;
//! Task 11: `ThemeAtlas`, `AtlasInner`, `SchemeStyles`, `Builder`.

use std::sync::Arc;

use crate::fonts::FontPlan;
use crate::style::{BuildInput, compile, egui_preset};
use crate::{
    AccessibilityPreferences, ColorMode, IconSet, LayoutTheme, ResolvedTheme, Role, RoleVariant,
    Surface,
};

/// A non-fatal observation made while compiling a [`ThemeAtlas`]. Six variants.
///
/// `#[non_exhaustive]` on the *enum* keeps adding a variant non-breaking; it does **not**
/// protect a variant's payload, which stays exhaustively patternable downstream (verified with
/// a two-crate probe on rustc 1.97.1). Since the diagnostics channel is the surface most
/// likely to be enriched, each struct variant carries its own `#[non_exhaustive]`, so a
/// downstream `match` must spell `{ path, .. }` and adding a field stays additive.
/// [`Surface::Panel`](crate::Surface::Panel) and both [`fonts::FontBytes`](crate::fonts::FontBytes) tuple variants are deliberately left
/// exhaustive: `#[non_exhaustive]` on a *tuple* variant makes it unconstructible outside this
/// crate (`E0603`, verified), and constructing them is the documented call shape.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Note {
    /// A theme value was non-finite, or a text size not a positive normal `f32`, and was
    /// replaced by the documented fallback.
    /// `path` is the native-theme field path, e.g. `"button.border.corner_radius"`.
    #[non_exhaustive]
    ValueSanitised {
        /// The native-theme leaf path whose value was replaced (§7.2: the leaf, never the egui sink).
        path: &'static str,
    },
    /// A length saturated at the `i8` bound of an epaint type — in practice
    /// `epaint::Margin`, the only `i8` sink this crate writes from theme data (§6.14 leaves
    /// all shadow geometry to egui). **Real data loss.** `u8` saturation — corner radii — is
    /// benign and is deliberately **not** reported; see §7.2.
    #[non_exhaustive]
    ValueSaturated {
        /// The native-theme leaf path whose length saturated.
        path: &'static str,
    },
    /// The theme asked for a font family for which the plan holds no bytes — or, from
    /// `fonts::FontPlan::from_system`, which the OS has no face of under that name (compared
    /// case-insensitively, §8.2).
    /// egui's own face stays in that family's place.
    #[non_exhaustive]
    FontFamilyUnavailable {
        /// The family the theme asked for.
        family: std::sync::Arc<str>,
    },
    /// The face chosen for a family has no `wght` axis — `FontData::variation_axes()`
    /// (`epaint/src/text/fonts.rs:153-173`) reports none — and either its own weight differs
    /// from the one the theme asked for, or it was registered with
    /// [`fonts::FontPlan::variable_face`](crate::fonts::FontPlan::variable_face). The face renders at its own weight (§8.2).
    #[non_exhaustive]
    FontWeightAxisUnsupported {
        /// The family the face was chosen for.
        family: std::sync::Arc<str>,
    },
    /// A native colour with alpha `0` was written, as given, to a `WidgetVisuals::bg_fill`,
    /// which egui documents as "Must never be `Color32::TRANSPARENT`"
    /// (`egui/src/style.rs:1292-1294`). Nothing is substituted — a substitute would be a
    /// colour no platform stated — so the widget paints no background in that state (§6.4).
    #[non_exhaustive]
    TransparentFill {
        /// The native-theme leaf path of the colour with alpha `0`.
        path: &'static str,
    },
    /// A registered face's bytes failed the parse epaint makes when it loads a face —
    /// `skrifa::FontRef::from_index(data, index)`, the only fallible step of epaint's
    /// `FontFace::new` (`epaint/src/text/font.rs:386-388`), whose failure epaint turns into a
    /// panic in release builds too (`epaint/src/text/fonts.rs:990`) — or they parsed with a zero
    /// `unitsPerEm`, which epaint divides by (§8.2). The face was dropped and
    /// the family falls back to the next face in its chain, egui's own if no other. `family`
    /// is the name the face was registered under.
    #[non_exhaustive]
    FontDataInvalid {
        /// The family name the face was registered under.
        family: std::sync::Arc<str>,
    },
}

/// The builder's stored style patch (§4.3): a named type, because the spelled-out
/// `Option<Box<dyn Fn(&mut egui::Style) + 'a>>` is `clippy::type_complexity`.
pub(crate) type StylePatch<'a> = Box<dyn Fn(&mut egui::Style) + 'a>;

/// The focus ring of one scheme (§6.18): stroke and offset from `defaults.focus_ring_*`;
/// filled by Task 21.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FocusRing {
    pub(crate) stroke: egui::Stroke,
    pub(crate) offset: f32,
}

/// One colour scheme's compiled styles (§3.4).
pub(crate) struct SchemeStyles {
    pub(crate) base: Arc<egui::Style>,
    /// `cells[role.index()][variant.index()]`; a cell with no data of its own shares its
    /// `Normal` `Arc` (§3.4).
    pub(crate) cells: [[Arc<egui::Style>; 3]; 25],
    /// `frames[surface.index()]`.
    pub(crate) frames: [egui::Frame; 11],
    /// The ring the install plugin paints (§6.18); `None` until Task 21 builds it.
    pub(crate) focus_ring: Option<FocusRing>,
}

impl SchemeStyles {
    /// The cell of a role in a variant. The indices are below 25 and 3 by construction
    /// (`ROLES`, `VARIANTS`); the base style is the fallback the lint-free `get` needs, never
    /// reached.
    pub(crate) fn cell(&self, role: Role, variant: RoleVariant) -> &Arc<egui::Style> {
        self.cells
            .get(role.index())
            .and_then(|row| row.get(variant.index()))
            .unwrap_or(&self.base)
    }

    /// The frame of a surface; the index is below 11 by construction (`SURFACES`), and
    /// egui's own preset over the base is the fallback the lint-free `get` needs.
    pub(crate) fn frame(&self, surface: Surface) -> egui::Frame {
        self.frames
            .get(surface.index())
            .copied()
            .unwrap_or_else(|| egui_preset(surface, &self.base))
    }
}

/// What a `ThemeAtlas` holds (§4.2 names every field's type, which is what makes the handle
/// `Send + Sync`).
pub(crate) struct AtlasInner {
    pub(crate) name: String,
    pub(crate) light: ResolvedTheme,
    pub(crate) dark: ResolvedTheme,
    pub(crate) schemes: [SchemeStyles; 2],
    pub(crate) accessibility: AccessibilityPreferences,
    pub(crate) layout: LayoutTheme,
    pub(crate) os_mode: Option<ColorMode>,
    pub(crate) icon_set: IconSet,
    pub(crate) icon_theme: [Option<String>; 2],
    /// The definitions `install` sets (§10.3 step 1); `None` when no plan was given. Task 22
    /// fills it and adds the install step that reads it.
    pub(crate) fonts: Option<egui::FontDefinitions>,
    pub(crate) notes: Vec<Note>,
}

impl AtlasInner {
    /// One scheme's styles: `schemes[0]` is `Light`, `schemes[1]` is `Dark`, as `build` stores them.
    pub(crate) fn scheme(&self, theme: egui::Theme) -> &SchemeStyles {
        match theme {
            egui::Theme::Light => &self.schemes[0],
            egui::Theme::Dark => &self.schemes[1],
        }
    }
}

/// `into` with every `TextStyle::Name` key of `from` that it lacks (§7.5): `into`'s own `Arc`
/// when there is none — an application without named styles pays nothing — and a copy with
/// the keys inserted otherwise. The five stock keys always keep `into`'s values.
pub(crate) fn carry_name_keys(from: &egui::Style, into: &Arc<egui::Style>) -> Arc<egui::Style> {
    let missing: Vec<(&egui::TextStyle, &egui::FontId)> = from
        .text_styles
        .iter()
        .filter(|(key, _)| {
            matches!(key, egui::TextStyle::Name(_)) && !into.text_styles.contains_key(*key)
        })
        .collect();
    if missing.is_empty() {
        return Arc::clone(into);
    }
    let mut style = egui::Style::clone(into);
    for (key, font) in missing {
        style.text_styles.insert(key.clone(), font.clone());
    }
    Arc::new(style)
}

/// The `Context` data key under which [`ThemeAtlas::install`] publishes the atlas
/// (§10.3 step 3) and [`ThemeAtlas::from_ctx`] reads it back.
pub(crate) fn atlas_key() -> egui::Id {
    egui::Id::new("native-theme-egui/atlas")
}

/// A complete egui theme compiled from native-theme data.
///
/// `Arc`-backed: cloning is one atomic increment, exactly like [`egui::Context`].
/// `Send + Sync + 'static`, so it can be built on a watcher thread and published into
/// [`egui::Context::data_mut`] (`egui/src/context.rs:1033`). This is the one load-bearing
/// auto-trait claim in the crate, so its ground is named: the atlas holds nothing but
/// `Arc<egui::Style>`, `egui::Frame`, [`Note`], [`AccessibilityPreferences`](crate::AccessibilityPreferences), two
/// [`ResolvedTheme`](crate::ResolvedTheme)s, its [`LayoutTheme`](crate::LayoutTheme) (four `Option<f32>`), its name (`String`), the
/// OS [`ColorMode`](crate::ColorMode) (`Option`), the
/// [`IconSet`](crate::IconSet) and the icon-theme name per `egui::Theme` (`Option<String>` each) it carries, the
/// focus ring per `egui::Theme` (§6.18: an `Option` of an `egui::Stroke` and an `f32` offset),
/// and the `egui::FontDefinitions` [`Builder::build`] made from its [`fonts::FontPlan`](crate::fonts::FontPlan) for
/// [`ThemeAtlas::install`] (none when no plan was given), which are `BTreeMap`s of `String`,
/// `FontFamily` (an `Arc<str>` in `Name`) and `Arc<FontData>` (`epaint/src/text/fonts.rs:431-444`,
/// `:74-95`), a `FontData` being a `Cow<'static, [u8]>`, a `u32` and a `FontTweak` of plain
/// values (`:112-122`, `:208-264`) — [`Builder::style_patch`]'s closure
/// is applied at build and not kept; and a `ResolvedTheme` is plain
/// owned data with no interior mutability and no `Rc`
/// (`native-theme/src/model/resolved.rs:156-213`).
///
/// It carries, per `egui::Theme` (Light and Dark):
/// * one base [`egui::Style`],
/// * one [`egui::Style`] per ([`Role`](crate::Role), [`RoleVariant`](crate::RoleVariant)),
/// * one [`egui::Frame`] per [`Surface`](crate::Surface),
/// * one focus ring, or none where the theme states no ring (§6.18),
///
/// plus the two [`ResolvedTheme`](crate::ResolvedTheme)s it was built from. The styles reach widgets through
/// [`ThemeAtlas::install`] (the base styles), [`NativeThemeUiExt`](crate::NativeThemeUiExt) and
/// [`ThemeAtlas::role_modifier`] (the role styles) and [`ThemeAtlas::surface_frame`] (the
/// frames). No raw role or base `Arc<Style>` is handed out: installed raw, it would drop the
/// application's `TextStyle::Name` keys, on which `TextStyle::resolve` panics in release
/// (`egui/src/style.rs:112-120`, §14 item 30), and every seam of §3.3 is covered by those
/// calls.
#[derive(Clone)]
pub struct ThemeAtlas(Arc<AtlasInner>);

impl std::fmt::Debug for ThemeAtlas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeAtlas")
            .field("name", &self.0.name)
            .field("notes", &self.0.notes.len())
            .finish_non_exhaustive()
    }
}

impl ThemeAtlas {
    /// Start building from a light and a dark [`ResolvedTheme`](crate::ResolvedTheme).
    ///
    /// Both are required: egui keeps a separate `Style` per `egui::Theme`
    /// (`egui/src/memory/mod.rs:196`, `:200`). Pass the same value twice if only one exists.
    ///
    /// The `#[must_use]` carries a message because [`Builder`] is itself `#[must_use]`; a bare
    /// one is `clippy::double_must_use`, which §13's **Lints, MSRV, package** test runs as an
    /// error.
    #[must_use = "this starts the builder; call `build()` to produce the atlas"]
    pub fn builder<'a>(
        name: &'a str,
        light: &'a ResolvedTheme,
        dark: &'a ResolvedTheme,
    ) -> Builder<'a> {
        Builder {
            name,
            light,
            dark,
            layout: None,
            accessibility: None,
            fonts: None,
            os_mode: None,
            icon_set: None,
            icon_theme: [None, None],
            style_patch: None,
        }
    }

    /// Display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// The source [`ResolvedTheme`](crate::ResolvedTheme) for an `egui::Theme` — pass `ctx.theme()` for the scheme
    /// egui is drawing. Total: every atlas holds both.
    #[must_use]
    pub fn resolved_for(&self, theme: egui::Theme) -> &ResolvedTheme {
        match theme {
            egui::Theme::Light => &self.0.light,
            egui::Theme::Dark => &self.0.dark,
        }
    }

    /// The accessibility preferences the atlas was built with, applied as
    /// [`Builder::accessibility`] describes, or `AccessibilityPreferences::default()`
    /// (`native-theme/src/lib.rs:262-271`). Hand it to the text-size accessors of §4.7, so a
    /// size read at a call site matches the size the atlas installed.
    #[must_use]
    pub fn accessibility(&self) -> &AccessibilityPreferences {
        &self.0.accessibility
    }

    /// The layout spacing the atlas was built with, as [`Builder::layout`] took it, or
    /// `LayoutTheme::default()` (`native-theme/src/model/widgets/mod.rs:890-913`, all four
    /// fields `None`). Read `container_margin` and `section_gap` here, the two per-call values
    /// no `Style` field carries (§5.1), so an atlas from [`from_system`](crate::from_system) needs no second
    /// detection of the OS.
    #[must_use]
    pub fn layout(&self) -> &LayoutTheme {
        &self.0.layout
    }

    /// The OS colour mode the atlas carries: `SystemTheme::mode` on the [`from_system`](crate::from_system) /
    /// [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) path, the value given to [`Builder::os_mode`], or
    /// `None` — [`from_preset`](crate::from_preset) and [`to_theme`](crate::to_theme) carry none. The install plugin feeds it to
    /// egui where the integration reports no OS scheme (§3.2, §10.3).
    #[must_use]
    pub fn os_mode(&self) -> Option<ColorMode> {
        self.0.os_mode
    }

    /// The theme's icon set, so icon code — a spinner drawing the set's animated indicator
    /// (`native_theme::icons::load_icon_indicator(set)` for a bundled set; for
    /// `IconSet::Freedesktop`, `FreedesktopLoader::load_indicator(atlas.icon_theme(ctx.theme()))`,
    /// because `load_icon_indicator` asks for the system's theme there,
    /// `native-theme/src/icons.rs:508`), an [`icons::IconKey`](crate::icons::IconKey) — follows the theme
    /// with no second input. On the [`from_preset`](crate::from_preset) path it is `Resolved::icon_set`
    /// (`native-theme/src/model/resolved.rs:259`), which native-theme already falls back to
    /// `system_icon_set()` when the preset states none; on the [`from_system`](crate::from_system) /
    /// [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) path it is `SystemTheme::icon_set`
    /// (`native-theme/src/lib.rs:469`); a [`Builder`] atlas takes [`Builder::icon_set`], else
    /// `native_theme::theme::system_icon_set()` (`native-theme/src/model/icons.rs:512`) — the
    /// same fallback native-theme applies.
    #[must_use]
    pub fn icon_set(&self) -> IconSet {
        self.0.icon_set
    }

    /// The freedesktop icon-theme name the theme names for one colour scheme, for
    /// [`icons::IconKey::icon_theme`](crate::icons::IconKey::icon_theme); pass `ctx.theme()` for the scheme egui is drawing.
    /// Per scheme because the variants differ — `kde-breeze` names `breeze` for light and
    /// `breeze-dark` for dark (`native-theme/src/presets/kde-breeze.toml:9`, `:317`) — and
    /// egui draws either style whenever the scheme changes, so one name would be wrong for the
    /// other. On the [`from_preset`](crate::from_preset) path it is each variant's `Resolved::icon_theme`
    /// (`native-theme/src/model/resolved.rs:264`, from `Theme::resolve` of that mode); on the
    /// [`from_system`](crate::from_system) / [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) path it is
    /// `SystemTheme::icon_theme_for` of that mode (§9.2); a [`Builder`] atlas takes
    /// [`Builder::icon_theme`] per scheme. `None` where the theme states none for that scheme
    /// and detection failed; nothing is invented.
    #[must_use]
    pub fn icon_theme(&self, theme: egui::Theme) -> Option<&str> {
        match theme {
            egui::Theme::Light => self.0.icon_theme[0].as_deref(),
            egui::Theme::Dark => self.0.icon_theme[1].as_deref(),
        }
    }

    /// One role's style in one appearance variant, packaged as an `egui::style::StyleModifier`
    /// (declared at `egui/src/style.rs:194`; **not** re-exported at egui's root —
    /// `egui/src/lib.rs:488` re-exports only
    /// `style::{FontSelection, Spacing, Style, TextStyle, Visuals}`).
    ///
    /// Feed to `Popup::style` (`egui/src/containers/popup.rs:417`), `MenuConfig::style`
    /// (`egui/src/containers/menu.rs:107`), `MenuBar::style` (`:241`) or
    /// `ComboBox::popup_style` (`egui/src/containers/combo_box.rs:199`) — the only four
    /// acceptors in egui 0.36.2 (§3.2); `Tooltip` reaches `Popup::style` through its public
    /// `popup` field (`egui/src/containers/tooltip.rs:9`).
    ///
    /// For menus this is required, not convenient: without it a menu's style is the
    /// `Context`'s passed through egui's `menu_style` (`egui/src/containers/menu.rs:22`),
    /// which overwrites `spacing.button_padding` (`:23`), **four** `bg_stroke`s — `active`
    /// (`:24`), `open` (`:25`), `hovered` (`:26`), `inactive` (`:28`) — and
    /// `widgets.inactive.weak_bg_fill` (`:27`) and carries none of the menu's own values;
    /// the `Role::Menu` cell applies the same function and then those values (§3.4). It is
    /// also the menu's chrome: egui builds every menu's frame from the style
    /// the modifier produces ([`Surface`](crate::Surface) gives the detail), so the fill, stroke, radius,
    /// shadow and margins of a menu are the ones the `Role::Menu` cell inherits from the base
    /// style, and its items' border, radius and padding the cell's own (§5.2).
    ///
    /// The modifier replaces the whole `Style`, like `impl From<Style> for StyleModifier`
    /// (`egui/src/style.rs:211-215`), except that every `TextStyle::Name` key of the style it
    /// receives is carried into the replacement — the closure is handed that style
    /// (`egui/src/style.rs:194`, `:225-229`), so this costs nothing when there is none. Its
    /// closure captures one `Arc<Style>`.
    #[must_use]
    pub fn role_modifier(
        &self,
        theme: egui::Theme,
        role: Role,
        variant: RoleVariant,
    ) -> egui::style::StyleModifier {
        let cell = Arc::clone(self.scheme(theme).cell(role, variant));
        egui::style::StyleModifier::new(move |style: &mut egui::Style| {
            *style = egui::Style::clone(&carry_name_keys(style, &cell));
        })
    }

    /// The [`egui::Frame`] recipe for one container surface.
    ///
    /// Feed to `Panel::frame` (`egui/src/containers/panel.rs:413`), `CentralPanel::frame`
    /// (`:1206`), `Window::frame` (`egui/src/containers/window.rs:265`), `Window::title_frame`
    /// (`:272`), `Modal::frame` (`egui/src/containers/modal.rs:53`), `Popup::frame`
    /// (`egui/src/containers/popup.rs:369`) or `Frame::show`
    /// (`egui/src/containers/frame.rs:404`).
    ///
    /// This is the **only** way to theme container margins: five of egui's eight `Frame`
    /// presets hardcode their inner margin and read no `Style::spacing` (§3.2).
    #[must_use = "this returns the frame recipe; hand it to a container's `.frame(..)`"]
    pub fn surface_frame(&self, theme: egui::Theme, surface: Surface) -> egui::Frame {
        self.scheme(theme).frame(surface)
    }

    /// Non-fatal observations made while compiling this atlas: sanitised values, saturated
    /// margins, font requests that could not be met. Inspect or log; never match exhaustively.
    #[must_use]
    pub fn notes(&self) -> &[Note] {
        &self.0.notes
    }

    /// Install into an [`egui::Context`]: the atlas's fonts, both colour schemes' base styles
    /// (keeping the application's own `TextStyle::Name` keys), the atlas into `Context` data,
    /// the install plugin, the icon-cache flush and a repaint — §10.3 lists the steps, their
    /// order and what each must not touch. The plugin supplies the OS colour scheme where the
    /// integration reports none, paints the focus ring (§6.18) and keeps the OS title bar on the
    /// scheme the UI is drawn in (§10.3).
    ///
    /// Run it on every application start, even when egui memory is persisted:
    /// `Options::dark_style` and `light_style` are `#[serde(skip)]`
    /// (`egui/src/memory/mod.rs:195`, `:199`). Safe to call at any time, including inside a
    /// pass; the new theme reaches the whole UI on the next pass.
    ///
    /// **The colour-scheme choice stays egui's.** `install` never touches
    /// `Options::theme_preference`, which is serde-persisted (`egui/src/memory/mod.rs:206`
    /// has no `serde(skip)`), so the user's in-app Light/Dark choice survives a restart.
    /// Follow the OS with `ctx.set_theme(egui::ThemePreference::System)`; pin a scheme with
    /// `ctx.set_theme(egui::ThemePreference::Dark)` or `Light`. The trap: `Context::set_theme`
    /// takes `impl Into<ThemePreference>` (`egui/src/context.rs:2170`) and
    /// `From<Theme> for ThemePreference` exists (`egui/src/memory/theme.rs:79-86`), so
    /// `ctx.set_theme(egui::Theme::Dark)` compiles and *pins* dark — it never means "follow".
    /// "The OS" is what the integration reports in `RawInput::system_theme` and, where it
    /// reports `None` (Linux under winit 0.30.13), this atlas's [`ThemeAtlas::os_mode`].
    ///
    /// `Options::fallback_theme` (`egui/src/memory/mod.rs:212`, default `Theme::Dark` at
    /// `:331`) and `Options::sync_window_theme` (`:229`, default `true` at `:333`) are egui's
    /// too and `install` leaves them alone: set them with
    /// `ctx.options_mut(|o| o.fallback_theme = ..)` (`egui/src/context.rs:1135`).
    pub fn install(&self, ctx: &egui::Context) {
        // §10.3 step 1: the plan's fonts, whenever the atlas was built with a plan — even one in
        // which no face was found — so a family the plan has no face for is egui's own again, not
        // the previous install's; an atlas built with no plan leaves the `Context`'s fonts alone.
        // `set_fonts`, never `add_font`, which de-duplicates by name only (`egui/src/context.rs:2106`).
        if let Some(defs) = self.fonts() {
            ctx.set_fonts(defs.clone());
        }
        // 2. Both base styles, each first given the `Name` keys of the style it replaces:
        //    `style_of` (`egui/src/context.rs:2221`) then `set_style_of` (`:2250`); §7.5, §10.3.
        for theme in [egui::Theme::Dark, egui::Theme::Light] {
            let previous = ctx.style_of(theme);
            ctx.set_style_of(theme, carry_name_keys(&previous, &self.scheme(theme).base));
        }
        // 3. The atlas published into `Context` data (`egui/src/context.rs:1033`).
        let published = self.clone();
        ctx.data_mut(|data| {
            data.insert_temp(atlas_key(), published);
        });
        // 4. The install plugin (`egui/src/context.rs:2047`).
        ctx.add_plugin(crate::plugin::NativeThemePlugin::default());
        // 5. The icons this crate cached: a recoloured icon has new bytes and a URI of its own,
        //    and egui drops a URI's texture and bytes only when told to (§10.3 step 5).
        crate::icons::forget_icons(ctx);
        // 6. The repaint, last (`egui/src/context.rs:1821`).
        ctx.request_repaint();
    }

    /// The atlas most recently published into this `Context` by [`ThemeAtlas::install`], or
    /// `None` when none is (never installed, or removed by [`ThemeAtlas::clear`]).
    #[must_use]
    pub fn from_ctx(ctx: &egui::Context) -> Option<Self> {
        let key = atlas_key();
        ctx.data(|data| data.get_temp::<Self>(key))
    }

    /// Remove the atlas from `Context` data (`IdTypeMap::remove`,
    /// `egui/src/util/id_type_map.rs:572`). The install plugin then finds nothing and does
    /// nothing, and [`NativeThemeUiExt`](crate::NativeThemeUiExt) degrades as it does when nothing was installed. The
    /// `Style`s already written into `Options` are left alone; call
    /// `ctx.set_style_of(t, t.default_style())` (`egui/src/memory/theme.rs:24-29`) for each
    /// `egui::Theme` `t`, and `ctx.set_fonts(egui::FontDefinitions::default())` if the atlas
    /// installed a font plan, to get stock egui back.
    pub fn clear(ctx: &egui::Context) {
        let key = atlas_key();
        ctx.data_mut(|data| data.remove::<Self>(key));
    }

    /// One scheme's compiled styles: the store every seam and every test module reads cells
    /// through (`base`, `cell(role, variant)`, `frame(surface)`).
    pub(crate) fn scheme(&self, theme: egui::Theme) -> &SchemeStyles {
        self.0.scheme(theme)
    }

    /// The `FontDefinitions` `Builder::build` made from the atlas's plan (§10.3 step 1), or
    /// `None` when the atlas was built with no plan.
    pub(crate) fn fonts(&self) -> Option<&egui::FontDefinitions> {
        self.0.fonts.as_ref()
    }
}

impl ThemeAtlas {
    /// The focus ring of one scheme, or `None` where the theme states none (§6.18).
    pub(crate) fn focus_ring(&self, theme: egui::Theme) -> Option<&FocusRing> {
        self.0.scheme(theme).focus_ring.as_ref()
    }
}

/// Additive builder for [`ThemeAtlas`]. Every optional input is a method here, so a future
/// optional input never changes an existing signature.
#[must_use = "call `build()` to produce the atlas"]
pub struct Builder<'a> {
    name: &'a str,
    light: &'a ResolvedTheme,
    dark: &'a ResolvedTheme,
    layout: Option<&'a LayoutTheme>,
    accessibility: Option<&'a AccessibilityPreferences>,
    fonts: Option<FontPlan>,
    os_mode: Option<ColorMode>,
    icon_set: Option<IconSet>,
    icon_theme: [Option<&'a str>; 2],
    style_patch: Option<StylePatch<'a>>,
}

impl<'a> Builder<'a> {
    /// Layout spacing. The atlas writes `widget_gap` into `spacing.item_spacing` (§6.16) and
    /// `window_margin` into [`Surface::CentralPanel`](crate::Surface::CentralPanel)'s inner margin; a `None` leaves that egui
    /// value at its stock default. `container_margin` and `section_gap` have no `Style` field;
    /// the application reads them per call from [`ThemeAtlas::layout`], which keeps the
    /// layout given here — a nested container's `Frame::inner_margin` and `Ui::add_space`
    /// (§5.1).
    ///
    /// A separate input because [`LayoutTheme`](crate::LayoutTheme) lives on `native_theme::theme::Theme`
    /// (`native-theme/src/model/mod.rs:269`) and on `SystemTheme`
    /// (`native-theme/src/lib.rs:488`), and **not** on [`ResolvedTheme`](crate::ResolvedTheme)
    /// (`native-theme/src/model/resolved.rs:156-213`). All four fields are `Option<f32>`
    /// (`native-theme/src/model/widgets/mod.rs:896-913`).
    pub fn layout(mut self, layout: &'a LayoutTheme) -> Self {
        self.layout = Some(layout);
        self
    }

    /// Accessibility preferences, from `SystemTheme::accessibility`
    /// (`native-theme/src/lib.rs:490`) or, on the preset path,
    /// `AccessibilityPreferences::from_system()` (`native-theme/src/lib.rs:287`, `:300`).
    /// Applied wherever egui has a sink, as the sibling connectors apply them: gpui scales its
    /// theme's font sizes in `to_theme` (`connectors/native-theme-gpui/src/lib.rs:188`, `:190`)
    /// and forwards reduced motion to gpui (`:760`); iced scales through `font_size` and
    /// `mono_font_size` (`connectors/native-theme-iced/src/lib.rs:438-443`, `:457-462`).
    ///
    /// * **Text scaling.** Every text size the atlas writes — each `Style::text_styles` entry
    ///   and each `Style::override_font_id` size, in the base styles and in every role style —
    ///   is [`scaled_text_size`](crate::scaled_text_size) of the theme's size. No other length is scaled.
    /// * **Reduced motion.** With `reduce_motion` set, every style gets
    ///   `Style::animation_time = 0.0` (`egui/src/style.rs:318`; egui's default `0.2` at
    ///   `:1440`) and `Style::scroll_animation = ScrollAnimation::none()` (`:338`, `:858`).
    ///   `animation_time` is read through `Context::global_style()` — by
    ///   `Context::animate_bool` and `animate_bool_with_easing` (`egui/src/context.rs:3192`,
    ///   `:3208`) and by the `Area` fade-in (`egui/src/containers/area.rs:638`). On egui 0.36.2
    ///   a zero time makes a value animation return its target at once
    ///   (`egui/src/animation_manager.rs:88-93`), and a bool animation's `elapsed / 0.0` is
    ///   non-finite and snaps to its end value (`:56-60`). `scroll_animation` is read from the
    ///   `Ui`'s own style (`egui/src/ui.rs:1401`, `:1443`, `:1492`), which is why the role
    ///   styles carry it too.
    ///
    /// `high_contrast` and `reduce_transparency` are not applied; they are exposed through
    /// [`is_high_contrast`](crate::is_high_contrast) and [`is_reduced_transparency`](crate::is_reduced_transparency). gpui drops its modal scrim under
    /// `reduce_transparency` (`connectors/native-theme-gpui/src/colors.rs:514-523`); egui's
    /// counterpart, the `Modal` backdrop, is no `Style` field but a per-call
    /// `Modal::backdrop_color` (`egui/src/containers/modal.rs:62`, default
    /// `Color32::from_black_alpha(100)` at `:29`), so that choice is the application's at the
    /// call site. Without this call the atlas is built with
    /// `AccessibilityPreferences::default()`: factor `1.0`, motion on.
    pub fn accessibility(mut self, prefs: &'a AccessibilityPreferences) -> Self {
        self.accessibility = Some(prefs);
        self
    }

    /// Font bytes. Without a plan the atlas maps font **sizes** only and leaves the
    /// `Context`'s fonts as they are. For the OS's own typefaces pass
    /// `fonts::FontPlan::from_system`'s plan (feature `system-fonts`), whose lookup notes
    /// then join [`ThemeAtlas::notes`].
    pub fn fonts(mut self, plan: FontPlan) -> Self {
        self.fonts = Some(plan);
        self
    }

    /// The OS colour mode, which the install plugin feeds to egui wherever the integration
    /// reports none — on Linux always, since winit 0.30.13 reports none there (§3.2).
    /// [`from_system`](crate::from_system) and [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) set it from `SystemTheme::mode`.
    /// An atlas built from a preset has none unless given one here; the OS's current mode is
    /// `if native_theme::detect::system_is_dark() { ColorMode::Dark } else { ColorMode::Light }`
    /// (`native-theme/src/detect.rs:141`), which reads `Light` wherever detection fails
    /// (`:139`). Without this call a preset-built atlas leaves the choice to egui's
    /// `Options::fallback_theme` on Linux.
    pub fn os_mode(mut self, mode: ColorMode) -> Self {
        self.os_mode = Some(mode);
        self
    }

    /// The icon set [`ThemeAtlas::icon_set`] reports. Without this call:
    /// `native_theme::theme::system_icon_set()`. [`from_preset`](crate::from_preset) and [`from_system`](crate::from_system) pass the
    /// theme's own.
    pub fn icon_set(mut self, set: IconSet) -> Self {
        self.icon_set = Some(set);
        self
    }

    /// The icon-theme name [`ThemeAtlas::icon_theme`] reports for `theme`; call it once per
    /// scheme. Without this call for a scheme: `None` for that scheme. [`from_preset`](crate::from_preset) and
    /// [`from_system`](crate::from_system) pass each variant's own when it has one.
    pub fn icon_theme(mut self, theme: egui::Theme, name: &'a str) -> Self {
        match theme {
            egui::Theme::Light => self.icon_theme[0] = Some(name),
            egui::Theme::Dark => self.icon_theme[1] = Some(name),
        }
        self
    }

    /// An application-owned adjustment applied **last** to every style the atlas holds —
    /// both base styles and every role style, after all theme data — for settings that are
    /// the application's, not the theme's (`Style::interaction`, `Style::debug` in a debug build, a
    /// `TextStyle::Name` key of its own, a `Spacing` value it wants fixed). It runs once per
    /// style at [`Builder::build`] and is not kept, so a `ThemeWatcher` rebuild keeps it only
    /// because the application's `rebuild` closure sets it again, as every builder input is
    /// set again. It can override a native value, deliberately: that is the application's
    /// decision, and nothing here second-guesses it. An application that installs fonts of
    /// its own instead of a plan sets `Spacing::extra_text_line_spacing` for them here. A
    /// second call replaces the first.
    pub fn style_patch(mut self, f: impl Fn(&mut egui::Style) + 'a) -> Self {
        self.style_patch = Some(Box::new(f));
        self
    }

    /// Build. Infallible: a [`ResolvedTheme`](crate::ResolvedTheme) is complete by construction and every numeric
    /// conversion in this crate is total (see [`convert`](crate::convert)). Every face of the font plan is
    /// validated here, once, with the parse epaint will make ([`fonts::font_definitions`](crate::fonts::font_definitions));
    /// each rejected face is a [`Note::FontDataInvalid`] in [`ThemeAtlas::notes`], and the
    /// `egui::FontDefinitions` it returns are kept for [`ThemeAtlas::install`] (§10.3).
    ///
    /// Also computed here, per variant, is the body text's `Spacing::extra_text_line_spacing`
    /// (`egui/src/style.rs:424`), written into both base styles, which every role style
    /// starts from (§3.4): the
    /// theme's line box, `defaults.line_height` × the scaled `defaults.font.size`, less the row
    /// height epaint gives the `Proportional` face the atlas installs — the plan's face, or, for
    /// a plan with no face, the head of its base's chain ([`fonts::FontPlan::with_base`](crate::fonts::FontPlan::with_base); egui's
    /// `Ubuntu-Light` for an empty plan) and, with no plan, the head of
    /// `egui::FontDefinitions::default()`'s chain, egui's `Ubuntu-Light`, the face a `Context`
    /// keeps unless the application replaces it ([`Builder::style_patch`]) — at that size,
    /// computed from the face's own metrics exactly as epaint does
    /// (`epaint/src/text/font.rs:397-400`, `:561-565`, `:587`), and floored at egui's `0.0`
    /// (§6.15). The same row height sets the `Role::Slider` cells' `expansion`, which keeps
    /// the slider knob at `slider.thumb_diameter` when the Body row outgrows it (§6.6). The
    /// row height does not depend on `pixels_per_point`
    /// (`epaint/src/text/font.rs:561-565` scales by the font size alone), so no zoom or DPI
    /// change invalidates either value.
    #[must_use = "this builds the styles; it does not install them"]
    pub fn build(self) -> ThemeAtlas {
        let mut notes = Vec::new();
        let prefs = self.accessibility.cloned().unwrap_or_default();
        let layout = self.layout.cloned().unwrap_or_default();
        // §4.9, §10.3 step 1: the plan's definitions, validated once here, kept for `install`,
        // and the definitions §6.15 measures the Body row height on (Task 13's `body_row_height`).
        let fonts: Option<egui::FontDefinitions> = self.fonts.as_ref().map(|plan| {
            let (defs, plan_notes) = crate::fonts::font_definitions(self.light, plan);
            notes.extend(plan_notes);
            defs
        });
        let defs = fonts.clone().unwrap_or_default(); // egui's default faces for an atlas with no plan
        let light = compile_scheme(
            egui::Theme::Light,
            self.light,
            &prefs,
            &layout,
            &defs,
            &mut notes,
        );
        let dark = compile_scheme(
            egui::Theme::Dark,
            self.dark,
            &prefs,
            &layout,
            &defs,
            &mut notes,
        );
        // Task 32: the style patch over every style of both schemes, last.
        let _patch = self.style_patch;
        ThemeAtlas(Arc::new(AtlasInner {
            name: self.name.to_owned(),
            light: self.light.clone(),
            dark: self.dark.clone(),
            schemes: [light, dark],
            accessibility: prefs,
            layout,
            os_mode: self.os_mode,
            icon_set: self
                .icon_set
                .unwrap_or_else(native_theme::theme::system_icon_set),
            icon_theme: [
                self.icon_theme[0].map(str::to_owned),
                self.icon_theme[1].map(str::to_owned),
            ],
            fonts,
            notes,
        }))
    }
}

/// One scheme's styles, with §6.15's Body row measured on `defs` at the Body size `base_style`
/// writes. The throw-away `Vec` is deliberate: `base_style` reports the size's leaf; this call
/// only needs the same sanitised size.
fn compile_scheme(
    scheme: egui::Theme,
    theme: &ResolvedTheme,
    prefs: &AccessibilityPreferences,
    layout: &LayoutTheme,
    defs: &egui::FontDefinitions,
    notes: &mut Vec<Note>,
) -> SchemeStyles {
    let row_height = scheme
        .default_style()
        .text_styles
        .get(&egui::TextStyle::Body)
        .and_then(|own| {
            let body = crate::style::base::text_size(
                "defaults.font.size",
                theme.defaults.font.size,
                prefs,
                own.size,
                &mut Vec::new(),
            );
            crate::fonts::body_row_height(defs, body)
        });
    compile(
        &BuildInput {
            scheme,
            theme,
            prefs,
            layout,
            row_height,
        },
        notes,
    )
}
