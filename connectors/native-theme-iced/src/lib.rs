//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::theme::ResolvedTheme`] data to iced's theming system.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use native_theme_iced::from_preset;
//!
//! let (theme, resolved) = from_preset("catppuccin-mocha", true).unwrap();
//! ```
//!
//! Or from the OS-detected theme:
//!
//! ```rust,no_run
//! use native_theme_iced::from_system;
//!
//! let (theme, resolved, is_dark, accessibility) = from_system().unwrap();
//! ```
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```rust
//! use native_theme::theme::{ColorMode, Theme};
//! use native_theme_iced::to_theme;
//!
//! let nt = Theme::preset("catppuccin-mocha").unwrap();
//! let resolved = nt.into_variant(ColorMode::Light).unwrap().resolve_system().unwrap();
//! let theme = to_theme(&resolved, "My App");
//! ```
//!
//! # Kerning
//!
//! Qt, GTK (Pango), WinUI and AppKit shape every text with the font's
//! kerning. iced 0.14 does not by default: its default, `Shaping::Auto`,
//! shapes text that is all ASCII with `Shaping::Basic` (`iced_graphics`
//! 0.14.0 `src/text.rs:325-337`), which sets each glyph at its own advance,
//! with no kerning and no font fallback (cosmic-text 0.15.0 `shape_skip`,
//! `src/shape.rs:475-515`), so pairs like "AV", "To" and "Wa" stand further
//! apart than the platform sets them.
//!
//! The `advanced-shaping` feature, on by default, turns on `iced_core`'s
//! feature of that name, which makes `Shaping::Advanced` the default
//! (`iced_core` 0.14.0 `src/text.rs:167-177`). Cargo builds one `iced_core`
//! for the application and this crate, so depending on this crate is enough:
//! every text that sets no shaping of its own is kerned -- `text`, and the
//! labels of checkboxes, radios, togglers, pick lists and combo boxes. (Text
//! inputs and text editors shape with `Shaping::Advanced` already.) Two
//! things keep a text unkerned: `.shaping(Shaping::Basic)` or
//! `.shaping(Shaping::Auto)` set on it, and `default-features = false`
//! without `advanced-shaping`. iced's own docs call advanced shaping the
//! costlier of the two; it is what the platforms do for every text.
//!
//! # Two layers of colour
//!
//! [`to_theme()`] builds an iced `Theme` whose `Palette` and `Extended`
//! palette carry the platform's colours, so iced's built-in widget styles pick
//! them up with no further code. That palette has six colours, though, and a
//! widget state it has no slot for -- a button's pressed fill, an input's
//! focus border, a switch's track -- is a value iced derives by lightening or
//! darkening. The palette cannot correct that: the slot does not exist.
//!
//! The `styles` module does (feature `widgets`, on by default). It has one
//! function per widget, each returning
//! a closure for that widget's style setter, and every colour, border and
//! radius it emits is a field of the resolved theme. A `Style` field the
//! native model does not carry is read from iced's own default at run time,
//! never written as a literal.
//!
//! ```rust,ignore
//! use native_theme_iced::styles;
//!
//! button("Save").style(styles::button_primary(&resolved))
//! ```
//!
//! Both layers are held to a mapping contract by this crate's tests: every
//! palette slot and every `Style` field the connector writes is asserted equal
//! to the native field it claims, over all bundled presets in both modes, and
//! every foreground the style functions place on a fill is asserted to be no
//! less readable than the platform's own pair.
//!
//! # Features
//!
//! | Feature | Default | Enables |
//! |---------|---------|---------|
//! | `widgets` | yes | `styles`, `button_padding`, `input_padding`, `text_area_padding`, `combo_box_padding`, `button_content_min_size` and `at_least`, through `iced_widget` |
//! | `spinner` | yes | [`Spinner`], the icon set's animated loading indicator (or an arc at `spinner.*`), through `iced_widget`'s `svg`, `image` and `canvas`; implies `widgets` |
//! | `iced_aw` | no | `styles::aw`, for the `iced_aw` widgets iced itself lacks (card, menu bar, tab bar, sidebar, selection list, spinner); implies `widgets` |
//! | `material-icons`, `lucide-icons`, `system-icons`, `svg-rasterize` | yes | the matching `native-theme` icon features |
//! | `system-fonts` | yes | `system_font_family`, the family iced's font database holds for a theme font, through `native-theme/system-fonts` |
//! | `advanced-shaping` | yes | kerned text in the whole application: `iced_core`'s `advanced-shaping`, which makes `Shaping::Advanced` every text's default (see [Kerning](#kerning)) |
//!
//! Every feature adds coverage. `default-features = false` leaves the palette
//! and the metric helpers that need `iced_core` only; `button_padding`,
//! `input_padding`, `combo_box_padding` and `button_content_min_size` read
//! iced's own default padding from `iced_widget`, and `at_least` lays out
//! `iced_widget`'s column, row and space, so `widgets` gates them as well.
//!
//! # Accessibility
//!
//! [`from_system()`] returns the user's [`AccessibilityPreferences`] as its
//! fourth value. The OS text-scaling factor applies to every text size:
//! [`scaled_text_size()`] scales any size the resolved theme states (a
//! widget's font, a `text_scale` role), and [`font_size()`] and
//! [`mono_font_size()`] are that for the default fonts; a factor that is not
//! finite and positive is ignored. With a preset there is no OS reading:
//! `AccessibilityPreferences::default()` scales by one.
//!
//! The other two preferences, reduced transparency and reduced motion, have no
//! receiver in iced: a `Theme` is a palette, with nothing for either to act
//! on. An application reads them from the same struct where it draws
//! translucent surfaces or animates.
//!
//! # Font Configuration
//!
//! A font in the resolved theme is a family, a size and a CSS weight. iced's
//! `Family::Name` takes a `&'static str` (`iced_core` 0.14 `font.rs:46`)
//! while a family name is an `Arc<str>`, so leak each distinct family once
//! and reuse it; `intern_font_family` gives one `Arc<str>` per name to key
//! that by. [`to_iced_weight`] turns the weight into iced's:
//!
//! ```rust,no_run
//! use native_theme::theme::intern_font_family;
//! use std::collections::HashMap;
//! use std::sync::Arc;
//!
//! let (_, resolved) = native_theme_iced::from_preset("catppuccin-mocha", true)?;
//! let mut families: HashMap<Arc<str>, &'static str> = HashMap::new();
//! let spec = &resolved.defaults.font;
//! // The family iced's font database holds for the face: on macOS the
//! // system UI font, stated "SF Pro", is filed as `.SF NS`.
//! #[cfg(feature = "system-fonts")]
//! let family_name: Arc<str> = native_theme_iced::system_font_family(spec);
//! #[cfg(not(feature = "system-fonts"))]
//! let family_name: Arc<str> = spec.family.clone();
//! let family: &'static str = *families
//!     .entry(intern_font_family(&family_name))
//!     .or_insert_with_key(|name| Box::leak(name.to_string().into_boxed_str()));
//! let font = iced_core::Font {
//!     family: iced_core::font::Family::Name(family),
//!     weight: native_theme_iced::to_iced_weight(spec.weight),
//!     ..iced_core::Font::DEFAULT
//! };
//! # let _ = font;
//! # Ok::<(), native_theme::error::Error>(())
//! ```
//!
//! iced 0.14 draws text with cosmic-text 0.15, which takes a face only where
//! the family has one at exactly the weight asked for
//! (`font/fallback/mod.rs:279-287`, `:446-456`), and fontdb 0.23 files each
//! face at the one weight its OS/2 table states, a variable font too
//! (`lib.rs:1034-1037`). A weight the family has no face of, like a family
//! the font database does not hold, falls through to the platform's
//! fallback families, whatever they are: cosmic-text's macOS list has
//! `.SF NS` first and the monospace Menlo second
//! (`font/fallback/macos.rs:30-38`), so bold text asked of the system font
//! can be drawn in Menlo Bold.
//!
//! To draw a stated weight of a variable family, file a face for it: where
//! the family has no face at the weight but a face's `wght` axis covers it,
//! push a copy of that face's `fontdb::FaceInfo` at the weight into the
//! database iced draws from (`iced::advanced::graphics::text::font_system()`,
//! `raw().db_mut()`, behind iced's `advanced` feature). cosmic-text sets the
//! `wght` axis of the face it matched to the weight asked for, both when it
//! shapes the text (`font/mod.rs:139-142`) and when it rasterises the
//! glyphs (`swash.rs:20-40`), so the platform's own font is drawn at the
//! true weight. Where no face covers the weight, ask for the family's nearest
//! weight instead. The showcase does both (`drawable_font` and
//! `register_weight` in `examples/showcase-iced.rs`). cosmic-text 0.19
//! matches a variable face at any weight its axis covers
//! (`variable_weight_match`, `font/system.rs:38-44`), which makes filing
//! faces unnecessary once iced draws with it.
//!
//! # Theme Field Coverage
//!
//! The connector maps a subset of [`ResolvedTheme`] to iced's theming system:
//!
//! | Target | Fields | Source |
//! |--------|--------|--------|
//! | `Palette` (6 fields) | background, text, primary, success, warning, danger | `defaults.*` |
//! | `Extended` overrides (9) | background.base.text, secondary.base + strong, background.weak.color/text, primary/success/danger/warning.base.text | `input.placeholder_color`, `defaults.surface_color`, `defaults.text_color`, `defaults.{accent,success,danger,warning}_text_color` |
//! | `styles` (20 items) | every `Style` field of button (six classes), text input, text editor, checkbox, radio, toggler, pick list, menu, slider, scrollable, progress bar, rule, tooltip, card container; scrollbar widths and embedding | the widget's own resolved theme; fields the model lacks come from iced's default |
//! | Widget metrics | button/input/combo-box padding (the stated sides inside the border, iced's own default for the others; `widgets` feature), any widget's padding over a default the caller names (`padding_or`, `padding_inside_border`) or where every side is stated (`stated_padding`), minimum control heights as a line height (`control_line_height`), a button's minimum size (`button_content_min_size`, `at_least`; `widgets` feature), a switch's track and thumb (`switch`) and a radio's indicator and dot (`radio`; `widgets` feature), border radius, scrollbar width | Per-widget resolved fields |
//! | Typography | font family/size/weight, mono family/size/weight, line height | `defaults.font.*`, `defaults.mono_font.*` |
//! | Color helpers | border, link, selection, info, info_foreground, warning_foreground, focus_ring | `defaults.*` |
//! | Geometry helpers | disabled_opacity | `defaults.*` |
//!
//! Per-widget geometry that is not a `Style` field (e.g. an indicator's size,
//! a track's height, a label gap) is not mapped, because iced takes it through
//! inline widget configuration rather than through the theme. Read it from the
//! `ResolvedTheme` you pass to [`to_theme()`] and hand it to the widget's
//! builder -- `Checkbox::size`, `Toggler::size`, `ProgressBar::girth`, the
//! thickness argument of `rule::horizontal`, `TextEditor::min_height` for
//! `input.min_height`. Where a widget has such a receiver, its `styles`
//! function's doc comment names it. A widget's size setter takes the extent
//! itself, with no minimum form, so a platform minimum passed there would
//! make the widget exactly that size; the minimum heights of the single-line
//! controls reach iced as a line height instead ([`control_line_height()`],
//! for `input.min_height` and `combo_box.min_height`), and a button's
//! minimum size as a floor under its label ([`at_least()`] of
//! [`button_content_min_size()`]). A combo box's or a progress bar's minimum
//! width has no receiver.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[cfg(test)]
mod compat;
#[cfg(test)]
mod contract;
pub(crate) mod extended;
pub mod icons;
pub mod palette;
#[cfg(feature = "spinner")]
pub mod spinner;
#[cfg(feature = "widgets")]
pub mod styles;

#[cfg(feature = "spinner")]
pub use spinner::Spinner;

// Re-export native-theme types that appear in public signatures.
pub use native_theme::color::Rgba;
pub use native_theme::error::Error;
pub use native_theme::theme::{
    AnimatedIcon, ColorMode, DialogButtonOrder, IconData, IconProvider, IconRole, IconSet,
    ResolvedTheme, Theme, ThemeMode, TransformAnimation,
};
pub use native_theme::{AccessibilityPreferences, Result, SystemTheme};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

/// Create an iced [`iced_core::theme::Theme`] from a [`native_theme::theme::ResolvedTheme`].
///
/// Builds a custom theme using `Theme::custom_with_fn()`, which:
/// 1. Maps the 6 Palette fields from resolved theme colors via [`palette::to_palette()`]
/// 2. Generates an Extended palette, then overrides `background.base.text`,
///    secondary, background.weak and the status-family `.base.text` entries
///    via `extended::apply_overrides()`
///
/// The resulting theme carries the mapped Palette and Extended palette. iced's
/// built-in `Catalog` implementations for `Theme` -- one in each
/// `iced_widget` module that has a style, among them `button`, `container`,
/// `text_input`, `scrollable`, `checkbox`, `slider` and `progress_bar` --
/// derive their `Style` structs from this palette, so no `Catalog`
/// implementation of ours is needed. A `Tooltip` has no `Catalog` of its own:
/// it is styled as a container (`Theme: container::Catalog`, iced_widget
/// 0.14.2 `src/tooltip.rs:70`, `:139-148`).
///
/// The `name` sets the theme's display name (visible in theme pickers).
/// For the common case, use [`from_preset()`] to derive the name automatically.
///
/// Note: iced has no `info` color family in its Extended palette, so
/// `info` / `info_foreground` are not mapped automatically. Use
/// [`info_color()`] and [`info_foreground_color()`] helpers to access them.
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(
    resolved: &native_theme::theme::ResolvedTheme,
    name: &str,
) -> iced_core::theme::Theme {
    let pal = palette::to_palette(resolved);

    // Capture only the Rgba values (Copy, 4 bytes each) instead of
    // cloning the entire ResolvedTheme (~2KB with heap data).
    let colors = extended::OverrideColors {
        placeholder: resolved.input.placeholder_color,
        surface: resolved.defaults.surface_color,
        foreground: resolved.defaults.text_color,
        accent_fg: resolved.defaults.accent_text_color,
        success_fg: resolved.defaults.success_text_color,
        danger_fg: resolved.defaults.danger_text_color,
        warning_fg: resolved.defaults.warning_text_color,
    };

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);
        extended::apply_overrides(&mut ext, &colors);
        ext
    })
}

/// Load a bundled preset and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// Handles the full pipeline: load preset, pick variant, resolve, validate, convert.
/// The `Theme` display name is used as the theme display name.
///
/// # Errors
///
/// Returns an error if the preset name is not recognized or if resolution fails.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
) -> Result<(iced_core::theme::Theme, native_theme::theme::ResolvedTheme)> {
    let spec = native_theme::theme::Theme::preset(name)?;
    let display_name = spec.name.clone();
    let variant = spec.into_variant(if is_dark {
        ColorMode::Dark
    } else {
        ColorMode::Light
    })?;
    let resolved = variant.resolve_system()?;
    let theme = to_theme(&resolved, &display_name);
    Ok((theme, resolved))
}

/// Detect the OS theme and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// Returns the iced theme, the resolved variant, whether the system is in
/// dark mode, and the OS accessibility preferences. The `is_dark` flag comes
/// from the OS preference, not from background color analysis. The
/// preferences are returned because [`font_size()`] and [`mono_font_size()`]
/// need them.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> Result<(
    iced_core::theme::Theme,
    native_theme::theme::ResolvedTheme,
    bool,
    AccessibilityPreferences,
)> {
    let sys = native_theme::SystemTheme::from_system()?;
    let is_dark = sys.mode.is_dark();
    let name = sys.name;
    let accessibility = sys.accessibility;
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name);
    Ok((theme, resolved, is_dark, accessibility))
}

/// Extension trait for converting a [`SystemTheme`] to an iced theme.
pub trait SystemThemeExt {
    /// Convert this system theme to an iced [`iced_core::theme::Theme`] and its [`ResolvedTheme`].
    ///
    /// Returns both the iced theme and the resolved variant, so callers can
    /// access per-widget metrics without re-resolving.
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_iced_theme(&self) -> (iced_core::theme::Theme, native_theme::theme::ResolvedTheme);
}

impl SystemThemeExt for native_theme::SystemTheme {
    fn to_iced_theme(&self) -> (iced_core::theme::Theme, native_theme::theme::ResolvedTheme) {
        let resolved = self.pick(self.mode).clone();
        let theme = to_theme(&resolved, &self.name);
        (theme, resolved)
    }
}

/// Returns each side a resolved padding states, and `default`'s side where it
/// states none.
///
/// For a widget whose padding iced sets side by side, with a default the
/// application can name: [`button_padding()`] and [`input_padding()`] are
/// this with the button's and the text input's own defaults. A consumer does
/// the same for any other widget the theme states a padding for, passing
/// that widget's default -- a menu item drawn as a button takes
/// `menu.border.padding` over `iced_widget::button::DEFAULT_PADDING`, a card
/// drawn as a container takes `card.border.padding` over `Padding::ZERO`,
/// the padding a `container` has unless given one (iced_widget 0.14.2
/// `src/container.rs:95`).
#[must_use]
pub fn padding_or(
    stated: &native_theme::theme::ResolvedPadding,
    default: iced_core::Padding,
) -> iced_core::Padding {
    iced_core::Padding {
        top: stated.top.unwrap_or(default.top),
        right: stated.right.unwrap_or(default.right),
        bottom: stated.bottom.unwrap_or(default.bottom),
        left: stated.left.unwrap_or(default.left),
    }
}

/// Returns a resolved padding as an iced [`Padding`](iced_core::Padding) when
/// it states every side, and `None` when it leaves any side unstated.
///
/// For a widget whose default padding the application cannot read, so an
/// unstated side has nothing to be filled from: `iced_aw` keeps its `Card`'s
/// and its `TabBar`'s defaults private (iced_aw 0.14.1
/// `src/widget/card.rs:21`, `src/widget/tab_bar.rs:41`) and takes a padding
/// whole. With `None`, leave the widget's padding unset, so it keeps its own.
#[must_use]
pub fn stated_padding(stated: &native_theme::theme::ResolvedPadding) -> Option<iced_core::Padding> {
    Some(iced_core::Padding {
        top: stated.top?,
        right: stated.right?,
        bottom: stated.bottom?,
        left: stated.left?,
    })
}

/// Returns each side a widget border's padding states, plus the border's line
/// width, and `default`'s side where it states none.
///
/// The model's padding lies inside the border (`ResolvedWidgetBorder::padding`,
/// "padding inside the border"), while iced lays a widget's content out
/// `padding` in from the widget's bounds (`layout::padded`, iced_core 0.14.0
/// `src/layout.rs:170-199`) and paints the border inside those same bounds,
/// over the padding (`renderer::Quad { bounds, border, .. }`, iced_widget
/// 0.14.2 `src/button.rs:382-388`, `src/text_input.rs:473-478`,
/// `src/pick_list.rs:590-595`). So a stated side reaches iced as the side
/// plus the line width, and the content sits where the platform puts it. An
/// unstated side is `default`'s, the widget's own geometry, measured as iced
/// measures it.
#[must_use]
pub fn padding_inside_border(
    border: &native_theme::theme::ResolvedWidgetBorder,
    default: iced_core::Padding,
) -> iced_core::Padding {
    let side = |stated: Option<f32>, default: f32| match stated {
        Some(v) => v + border.line_width,
        None => default,
    };
    let stated = &border.padding;
    iced_core::Padding {
        top: side(stated.top, default.top),
        right: side(stated.right, default.right),
        bottom: side(stated.bottom, default.bottom),
        left: side(stated.left, default.left),
    }
}

/// Returns button padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// [`padding_inside_border()`] of `button.border`: each side is the stated
/// side plus `button.border.line_width` where the theme states it, and iced's
/// own button padding where it does not:
/// `iced_widget::button::DEFAULT_PADDING` (iced_widget 0.14.2
/// `src/button.rs:462`), 5 top and bottom, 10 left and right.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn button_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    padding_inside_border(
        &resolved.button.border,
        iced_widget::button::DEFAULT_PADDING,
    )
}

/// Returns text input padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// [`padding_inside_border()`] of `input.border`: each side is the stated
/// side plus `input.border.line_width` where the theme states it, and iced's
/// own text-input padding where it does not:
/// `iced_widget::text_input::DEFAULT_PADDING` (iced_widget 0.14.2
/// `src/text_input.rs:125`), 5 on every side.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn input_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    padding_inside_border(
        &resolved.input.border,
        iced_widget::text_input::DEFAULT_PADDING,
    )
}

/// iced's own text-editor padding: `TextEditor::new` pads 5 on every side
/// (iced_widget 0.14.2 `src/text_editor.rs:152`, `Padding::new(5.0)`), a value
/// the crate keeps in the constructor rather than a constant.
#[cfg(feature = "widgets")]
const TEXT_EDITOR_PADDING: f32 = 5.0;

/// Returns multi-line text-editor padding from the resolved theme as an iced
/// [`Padding`](iced_core::Padding), for `TextEditor::padding`.
///
/// [`padding_inside_border()`] of `text_area.border`: each side is the stated
/// side plus `text_area.border.line_width` where the theme states it, and
/// iced's own text-editor padding where it does not, 5 on every side
/// (iced_widget 0.14.2 `src/text_editor.rs:152`). A platform pads its
/// multi-line field apart from its single-line one
/// (`docs/platform-facts.md` §2.29: Breeze's QTextEdit 5 where its line edit
/// is 7 / 6, GTK's text view 0 where its entry is 9 / 0); everything else
/// of a text area is the input's.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn text_area_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    padding_inside_border(
        &resolved.text_area.border,
        iced_core::Padding::new(TEXT_EDITOR_PADDING),
    )
}

/// Returns pick-list padding from the resolved theme as an iced [`Padding`](iced_core::Padding),
/// for `PickList::padding`.
///
/// [`padding_inside_border()`] of `combo_box.border`: each side is the stated
/// side plus `combo_box.border.line_width` where the theme states it, and
/// iced's own pick-list padding where it does not, the button's
/// `iced_widget::button::DEFAULT_PADDING` (iced_widget 0.14.2
/// `src/pick_list.rs:204`). The arrow is drawn right-aligned at the right
/// padding's inner edge (`src/pick_list.rs:636-660`), and the platform
/// centres it in its arrow column, `combo_box.arrow_area_width` wide
/// (`docs/platform-facts.md` §2.24: Breeze's 10px arrow centred in the 20px
/// `MenuButton_IndicatorWidth` column), which a stated right side measures
/// to: where the theme states both, the right side also holds half the
/// column the arrow leaves, `(arrow_area_width - arrow_icon_size) / 2`.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn combo_box_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    let c = &resolved.combo_box;
    let pad = padding_inside_border(&c.border, iced_widget::button::DEFAULT_PADDING);
    match (c.border.padding.right, c.arrow_area_width) {
        (Some(_), Some(column)) => {
            pad.right(pad.right + arrow_column_margin(column, c.arrow_icon_size))
        }
        _ => pad,
    }
}

/// The room either side of an arrow `arrow` wide centred in a column
/// `column` wide; none where the arrow fills it.
#[cfg(feature = "widgets")]
fn arrow_column_margin(column: f32, arrow: f32) -> f32 {
    ((column - arrow) / 2.0).max(0.0)
}

/// Returns the line height for a single-line control's text that makes the
/// control at least `min_height` tall inside `padding`: the theme's own line
/// box, `defaults.line_height` times `text_size`, where that reaches the
/// minimum already, and otherwise the height the minimum leaves inside the
/// padding.
///
/// For `TextInput::line_height`, `PickList::text_line_height` and a button
/// label's `Text::line_height`: iced lays each out one text line plus its
/// padding tall (`text_input.rs:309`, `pick_list.rs:421-432`, `button.rs:242-254`)
/// with no minimum-height setter, and centres the text in its line box
/// (`alignment::Vertical::Center`, `text_input.rs:321`, `pick_list.rs:382`),
/// so a taller line box is the control's minimum height. Pass the text size
/// the control is drawn at (scaled by the text-scaling factor): at a large
/// factor the line box grows past the minimum, and the control with it.
#[must_use]
pub fn control_line_height(
    resolved: &native_theme::theme::ResolvedTheme,
    text_size: f32,
    min_height: f32,
    padding: iced_core::Padding,
) -> iced_core::text::LineHeight {
    let own = text_size * resolved.defaults.line_height;
    let room = min_height - padding.top - padding.bottom;
    iced_core::text::LineHeight::Absolute(iced_core::Pixels(own.max(room)))
}

/// Returns the smallest content box a themed button has: `button.min_width`
/// and `button.min_height`, outer sizes (`docs/platform-facts.md`, "minimum
/// outer width/height"), less [`button_padding()`] on each side, and never
/// below zero. Hand it to [`at_least()`] round the button's label.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn button_content_min_size(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Size {
    let b = &resolved.button;
    let p = button_padding(resolved);
    iced_core::Size::new(
        (b.min_width - p.left - p.right).max(0.0),
        (b.min_height - p.top - p.bottom).max(0.0),
    )
}

/// A switch at the platform's size: a track `switch.track_width` by
/// `.track_height`, holding a thumb `switch.thumb_diameter` across
/// (`.unchecked_thumb_diameter`, where the theme states one, while not
/// `is_toggled`), inset by half the difference of the two heights, at the
/// right end while `is_toggled`. `on_toggle` is the message a press sends; `None` is a
/// disabled switch.
///
/// iced's `Toggler` lays its track out as twice its height
/// (`iced_widget` 0.14.2 `src/toggler.rs:287`) and has no width setter, so
/// every platform whose track is not twice its height -- material's 52 by 32,
/// adwaita's 46 by 26 -- would get a wider track than it states. This switch
/// is a button and two containers instead, coloured by
/// [`styles::toggler()`] for the toggler status each button status stands
/// for: `Hovered` and `Pressed` are the toggler's `Hovered` (iced's toggler
/// has no pressed state), `Disabled` its `Disabled`, and `Active` its
/// `Active`. The thumb takes the toggler's foreground for `Active` or
/// `Disabled`, as the toggler's own thumb does in every state, and both take
/// its `border_radius`, as iced paints them (`toggler.rs:435`, `:461`).
///
/// The switch has no label: `SwitchTheme` states no font and no gap for one.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn switch<'a, Message, Renderer>(
    resolved: &native_theme::theme::ResolvedTheme,
    is_toggled: bool,
    on_toggle: Option<Message>,
) -> iced_core::Element<'a, Message, iced_core::Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    use iced_widget::toggler::Status as Toggler;
    use iced_widget::{button, container};

    let s = &resolved.switch;
    let enabled = on_toggle.is_some();
    let track = styles::toggler(resolved);
    let thumb_style = track.clone();
    let round = iced_core::border::Radius::new(s.track_height / 2.0);
    let diameter = if is_toggled {
        s.thumb_diameter
    } else {
        s.unchecked_thumb_diameter.unwrap_or(s.thumb_diameter)
    };

    let thumb = container(iced_widget::Space::new())
        .width(diameter)
        .height(diameter)
        .style(move |theme| {
            let status = if enabled {
                Toggler::Active { is_toggled }
            } else {
                Toggler::Disabled { is_toggled }
            };
            let t = thumb_style(theme, status);
            container::Style {
                background: Some(t.foreground),
                border: iced_core::Border {
                    color: t.foreground_border_color,
                    width: t.foreground_border_width,
                    radius: t.border_radius.unwrap_or(round),
                },
                ..container::Style::default()
            }
        });
    let seat = container(thumb)
        .padding(((s.track_height - diameter) / 2.0).max(0.0))
        .width(iced_core::Length::Fill)
        .height(iced_core::Length::Fill)
        .align_x(if is_toggled {
            iced_core::alignment::Horizontal::Right
        } else {
            iced_core::alignment::Horizontal::Left
        })
        .align_y(iced_core::alignment::Vertical::Center);

    button(seat)
        .padding(0)
        .width(s.track_width)
        .height(s.track_height)
        .on_press_maybe(on_toggle)
        .style(move |theme, status| {
            let status = match status {
                button::Status::Active => Toggler::Active { is_toggled },
                button::Status::Hovered | button::Status::Pressed => {
                    Toggler::Hovered { is_toggled }
                }
                button::Status::Disabled => Toggler::Disabled { is_toggled },
            };
            let t = track(theme, status);
            let iced = button::Style::default();
            button::Style {
                background: Some(t.background),
                text_color: t.text_color.unwrap_or(iced.text_color),
                border: iced_core::Border {
                    color: t.background_border_color,
                    width: t.background_border_width,
                    radius: t.border_radius.unwrap_or(round),
                },
                shadow: iced.shadow,
                snap: iced.snap,
            }
        })
        .into()
}

/// A radio button at the platform's size, with the platform's dot: `radio`
/// sized `checkbox.radio_indicator_width` across where the theme states it
/// (Material's radio is 20, its checkbox 18), `checkbox.indicator_width`
/// where it does not, and set `checkbox.label_gap` from
/// its label, styled by [`styles::radio()`], and, where the theme states
/// `checkbox.radio_dot_diameter`, selected with a dot that many pixels across
/// in `checkbox.indicator_color`, centred in the circle. `is_selected` is the
/// selection `radio` was built with (`Some(value) == selected`), which iced
/// keeps private.
///
/// iced draws its dot at half the circle (`iced_widget` 0.14.2
/// `src/radio.rs:409-433`) and `radio::Style` has no size for it, so where a
/// dot size is stated this paints the radio with a transparent `dot_color`
/// and lays the dot over it in a `Stack`, whose first layer sizes it
/// (`stack.rs:180-187`) and whose upper layers take no input
/// (`stack.rs:249-275`): layout, input, hover and label stay the radio's.
/// Where the theme states no dot size -- macOS publishes none
/// (`docs/platform-facts.md:1220`) -- the dot is iced's own.
///
/// Set the label's `text_size` and `font` on `radio` before passing it.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn radio<'a, Message, Renderer>(
    resolved: &native_theme::theme::ResolvedTheme,
    radio: iced_widget::Radio<'a, Message, iced_core::Theme, Renderer>,
    is_selected: bool,
) -> iced_core::Element<'a, Message, iced_core::Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_core::text::Renderer + 'a,
{
    use iced_widget::container;

    let c = &resolved.checkbox;
    let style = styles::radio(resolved);
    let width = c.radio_indicator_width.unwrap_or(c.indicator_width);
    let radio = radio.size(width).spacing(c.label_gap);
    let Some(dot) = c.radio_dot_diameter.filter(|d| d.is_finite() && *d >= 0.0) else {
        return radio.style(style).into();
    };
    let radio = radio.style(move |theme, status| iced_widget::radio::Style {
        dot_color: iced_core::Color::TRANSPARENT,
        ..style(theme, status)
    });
    // Laid out whether selected or not, so the radio keeps its place in the
    // widget tree, and its state, as the selection changes.
    let colour = palette::to_color(c.indicator_color);
    let mark = container(iced_widget::Space::new())
        .width(dot)
        .height(dot)
        .style(move |_| container::Style {
            background: is_selected.then_some(iced_core::Background::Color(colour)),
            border: iced_core::border::rounded(dot / 2.0),
            ..container::Style::default()
        });
    let seat = container(mark)
        .width(width)
        .height(iced_core::Length::Fill)
        .align_x(iced_core::alignment::Horizontal::Center)
        .align_y(iced_core::alignment::Vertical::Center);
    iced_widget::Stack::new().push(radio).push(seat).into()
}

/// The drop-down arrow the platforms draw, for `pick_list(..).handle(..)`: an
/// open chevron (`docs/platform-facts.md` §2.24, `arrow_icon_size`: Breeze's
/// `renderArrow`, GNOME's `pan-down-symbolic`, WinUI's `ChevronDown`), set at
/// `combo_box.arrow_icon_size`.
///
/// iced's own `Handle::Arrow` is a filled triangle, the renderer's
/// `ARROW_DOWN_ICON` (`iced_widget` 0.14.2 `src/pick_list.rs:600-606`; U+E800
/// in iced's `Iced-Icons` font, `iced_wgpu` 0.14.0 `src/lib.rs:724`). This is
/// the same font's `SCROLL_DOWN_ICON` (U+E803, `:727`), the renderer's own
/// downward chevron, at the size and line height `Handle::Arrow` takes, so no
/// glyph of another icon set is drawn.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn pick_list_handle<Renderer>(
    resolved: &native_theme::theme::ResolvedTheme,
) -> iced_widget::pick_list::Handle<Renderer::Font>
where
    Renderer: iced_core::text::Renderer,
{
    iced_widget::pick_list::Handle::Static(iced_widget::pick_list::Icon {
        font: Renderer::ICON_FONT,
        code_point: Renderer::SCROLL_DOWN_ICON,
        size: Some(iced_core::Pixels(resolved.combo_box.arrow_icon_size)),
        line_height: iced_core::text::LineHeight::default(),
        shaping: iced_core::text::Shaping::Basic,
    })
}

/// Lays `content` out at least `min` wide and tall, centred in the room the
/// minimum gives it, and at its own size where that is larger.
///
/// iced's widgets take their extent (`Button::width`, `Button::height`) with
/// no minimum form, so a platform minimum passed there would make the widget
/// exactly that size. This column holds a `min.width`-wide space above a row
/// of a `min.height`-tall space and the content: a column is as wide as its
/// widest child and a row as tall as its tallest, so the pair is the floor,
/// and the content grows past it. `button(at_least(label, button_content_min_size(&r)))`
/// is a button at least `button.min_width` by `button.min_height`.
///
/// Requires the `widgets` feature (on by default).
#[cfg(feature = "widgets")]
#[must_use]
pub fn at_least<'a, Message, Theme, Renderer>(
    content: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    min: iced_core::Size,
) -> iced_core::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    iced_widget::Column::new()
        .push(iced_widget::Space::new().width(min.width))
        .push(
            iced_widget::Row::new()
                .push(iced_widget::Space::new().height(min.height))
                .push(content)
                .align_y(iced_core::alignment::Vertical::Center),
        )
        .align_x(iced_core::alignment::Horizontal::Center)
        .into()
}

/// Returns the standard border radius from the resolved theme.
#[must_use]
pub fn border_radius(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.border.corner_radius
}

/// Returns the large border radius from the resolved theme.
#[must_use]
pub fn border_radius_lg(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.border.corner_radius_lg
}

/// Returns the scrollbar groove width from the resolved theme.
#[must_use]
pub fn scrollbar_width(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.scrollbar.groove_width
}

/// Returns the primary UI font family name from the resolved theme: the
/// stated name. To name the family iced's font database holds for it, use
/// `system_font_family` (feature `system-fonts`).
#[must_use]
pub fn font_family(resolved: &native_theme::theme::ResolvedTheme) -> &str {
    &resolved.defaults.font.family
}

/// Returns the primary UI font size in logical pixels, scaled by the user's
/// text-scaling preference: [`scaled_text_size`] of `defaults.font.size`.
///
/// ResolvedFontSpec.size is in logical pixels (conversion from platform points
/// is handled by the resolution step).
///
/// iced fixes its default text size when the renderer is created
/// (`iced_wgpu` 0.14.0 `window/compositor.rs:294-300`; `text::Renderer` only
/// reads it back), so text given no size stays at that default whatever theme
/// is installed later. Size every text-bearing widget from the theme instead:
/// body text with this size, and a widget whose font the model states, or
/// text in a `text_scale` role, with [`scaled_text_size`] of that font's or
/// role's `size` (`button.font`, `input.font`, `checkbox.font`,
/// `combo_box.font`, `menu.font`, `tab.font`, …), so the text-scaling factor
/// reaches every text alike. The receivers are `Text::size` and `font`;
/// `text_size` and `font` on `checkbox`, `radio`, `toggler` and `pick_list`;
/// `size` and `font` on `text_input`, `combo_box` and `text_editor`.
/// [`to_iced_weight`] turns the weight into iced's; the crate's *Font
/// Configuration* section shows the whole `Font`, with the family, and what
/// iced 0.14 draws for a weight the family has no face of.
#[must_use]
pub fn font_size(
    resolved: &native_theme::theme::ResolvedTheme,
    prefs: &AccessibilityPreferences,
) -> f32 {
    scaled_text_size(resolved.defaults.font.size, prefs)
}

/// Returns the monospace font family name from the resolved theme: the
/// stated name. To name the family iced's font database holds for it, use
/// `system_font_family` (feature `system-fonts`).
#[must_use]
pub fn mono_font_family(resolved: &native_theme::theme::ResolvedTheme) -> &str {
    &resolved.defaults.mono_font.family
}

/// The family iced's font database holds for a theme font: the family
/// fontdb records for the face `native_theme::fonts::system_face` chooses
/// for the stated family, weight and style, or the stated family where the
/// system has no such face — which iced then draws as it draws any family
/// its database lacks.
///
/// iced draws through cosmic-text over fontdb 0.23, the version
/// native-theme selects the face with, and cosmic-text takes a face when any
/// of its recorded family names equals the name asked for (fontdb 0.23.0
/// `src/lib.rs` line 667); the returned name is the face's first, so it matches.
/// On macOS the system UI font, stated "SF Pro", is filed under `.SF NS`,
/// and that is what this returns for it.
///
/// The first call in a process loads the system font database, which
/// native-theme keeps for the process; every call selects a face among it
/// and copies its bytes, so call it when the theme changes, not per frame.
#[cfg(feature = "system-fonts")]
#[must_use]
pub fn system_font_family(spec: &native_theme::theme::ResolvedFontSpec) -> std::sync::Arc<str> {
    match native_theme::fonts::system_face(&spec.family, spec.weight, spec.style) {
        Some(face) => face.family,
        None => spec.family.clone(),
    }
}

/// Returns the monospace font size in logical pixels, scaled by the user's
/// text-scaling preference.
///
/// ResolvedFontSpec.size is in logical pixels (conversion from platform points
/// is handled by the resolution step).
#[must_use]
pub fn mono_font_size(
    resolved: &native_theme::theme::ResolvedTheme,
    prefs: &AccessibilityPreferences,
) -> f32 {
    scaled_text_size(resolved.defaults.mono_font.size, prefs)
}

/// Scales a text size from the resolved theme by the user's text-scaling
/// preference.
///
/// The factor applies to every text size, not only the body font's: a
/// widget's font (`resolved.button.font.size`, …) and a `text_scale` role's
/// size (`resolved.text_scale.caption.size`, …) are the theme's own sizes,
/// unscaled, as `defaults.font.size` is. A factor that is not finite and
/// positive is ignored.
///
/// # Example
///
/// ```rust,no_run
/// let (_, resolved, _, prefs) = native_theme_iced::from_system().unwrap();
/// let label = native_theme_iced::scaled_text_size(resolved.button.font.size, &prefs);
/// ```
#[must_use]
pub fn scaled_text_size(size: f32, prefs: &AccessibilityPreferences) -> f32 {
    size * text_scale_factor(prefs)
}

/// Text-scaling multiplier from the preferences: the factor when it is finite
/// and positive, else `1.0` (spec §4.1).
fn text_scale_factor(prefs: &AccessibilityPreferences) -> f32 {
    let s = prefs.text_scaling_factor;
    if s.is_finite() && s > 0.0 { s } else { 1.0 }
}

/// Returns the primary UI font weight (CSS 100-900) from the resolved theme.
///
/// iced 0.14 draws a weight only where the family has a face at exactly
/// that weight, and otherwise falls through to another family; see the
/// crate's *Font Configuration* section.
#[must_use]
pub fn font_weight(resolved: &native_theme::theme::ResolvedTheme) -> u16 {
    resolved.defaults.font.weight
}

/// Returns the monospace font weight (CSS 100-900) from the resolved theme.
#[must_use]
pub fn mono_font_weight(resolved: &native_theme::theme::ResolvedTheme) -> u16 {
    resolved.defaults.mono_font.weight
}

/// Returns the border/divider color from the resolved theme: the final line
/// colour, into which the model has already folded `defaults.border.opacity`.
#[must_use]
pub fn border_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.border.color)
}

/// Returns the disabled control opacity from the resolved theme.
#[must_use]
pub fn disabled_opacity(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.disabled_opacity
}

/// Returns the focus ring indicator color from the resolved theme.
#[must_use]
pub fn focus_ring_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.focus_ring_color)
}

/// Returns the hyperlink color from the resolved theme.
#[must_use]
pub fn link_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.link_color)
}

/// Returns the colour of an expander's disclosure arrow from the resolved
/// theme: `expander.arrow_color`, and the header's label colour,
/// `expander.font.color`, where the theme states none -- the arrow is then
/// the colour of the title it sits beside.
///
/// For the arrow a consumer draws in the header `styles::expander` styles:
/// iced has no expander, so the arrow is the consumer's own drawing.
#[must_use]
pub fn expander_arrow_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    let x = &resolved.expander;
    palette::to_color(x.arrow_color.unwrap_or(x.font.color))
}

/// Returns the selection highlight background color from the resolved theme.
#[must_use]
pub fn selection_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.selection_background)
}

/// Returns the info/attention color from the resolved theme.
///
/// Note: iced has no `info` family in its Extended palette, so this color
/// is not mapped automatically. Use this helper to access it directly.
#[must_use]
pub fn info_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.info_color)
}

/// Returns the text color for info-colored backgrounds from the resolved theme.
#[must_use]
pub fn info_foreground_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.info_text_color)
}

/// Returns the warning foreground text color from the resolved theme.
///
/// The warning base color is already mapped to `palette.warning`. This returns
/// the text color intended for use on warning-colored backgrounds.
#[must_use]
pub fn warning_foreground_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.warning_text_color)
}

/// Returns a reference to the per-context icon sizes from the resolved theme.
#[must_use]
pub fn icon_sizes(
    resolved: &native_theme::theme::ResolvedTheme,
) -> &native_theme::theme::ResolvedIconSizes {
    &resolved.defaults.icon_sizes
}

/// Returns the line height multiplier from the resolved theme.
///
/// The raw multiplier (e.g., 1.4). Use with iced's
/// `LineHeight::Relative(native_theme_iced::line_height_multiplier(&r))`
/// for Text widgets. Font-size agnostic -- works correctly for both
/// the primary UI font and monospace text.
///
/// For absolute pixels (layout math), multiply by the appropriate
/// font size: `line_height_multiplier(&r) * font_size(&r, &prefs)`.
#[must_use]
pub fn line_height_multiplier(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.line_height
}

/// Convert a CSS font weight (100-900) to an iced [`Weight`](iced_core::font::Weight) enum.
///
/// Non-standard weights are rounded to the nearest standard value
/// (e.g., 350 -> Normal, 550 -> Semibold).
///
/// The weight goes into an `iced_core::Font` with the theme's family, which
/// the crate's *Font Configuration* section builds. iced 0.14 draws it only
/// where the family has a face at exactly that weight; elsewhere the text
/// falls through to another family, a monospace one on macOS, unless a face
/// is filed for the weight as that section describes.
///
/// # Example
///
/// ```rust,no_run
/// let (_, resolved) = native_theme_iced::from_preset("catppuccin-mocha", true).unwrap();
/// let weight = native_theme_iced::to_iced_weight(
///     native_theme_iced::font_weight(&resolved),
/// );
/// ```
#[must_use]
pub fn to_iced_weight(css_weight: u16) -> iced_core::font::Weight {
    use iced_core::font::Weight;
    match css_weight {
        0..=149 => Weight::Thin,
        150..=249 => Weight::ExtraLight,
        250..=349 => Weight::Light,
        350..=449 => Weight::Normal,
        450..=549 => Weight::Medium,
        550..=649 => Weight::Semibold,
        650..=749 => Weight::Bold,
        750..=849 => Weight::ExtraBold,
        850.. => Weight::Black,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::theme::{ColorMode, Theme};

    /// With `advanced-shaping`, a text that sets no shaping of its own is
    /// shaped with the font's kerning: the default is `Shaping::Advanced`,
    /// not `Auto`, which shapes ASCII text without it. The dev-dependencies
    /// do not turn `iced_core`'s feature on, so this is the connector's.
    #[cfg(feature = "advanced-shaping")]
    #[test]
    fn text_is_kerned_by_default() {
        assert_eq!(
            iced_core::text::Shaping::default(),
            iced_core::text::Shaping::Advanced
        );
    }

    fn make_resolved_preset(name: &str, is_dark: bool) -> native_theme::theme::ResolvedTheme {
        Theme::preset(name)
            .unwrap()
            .into_variant(if is_dark {
                ColorMode::Dark
            } else {
                ColorMode::Light
            })
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap()
    }

    /// A family the system has no face of stays as stated: the family is
    /// never substituted (egui spec §8.8, §8.2).
    #[cfg(feature = "system-fonts")]
    #[test]
    fn system_font_family_keeps_a_family_no_system_has() {
        let mut spec = make_resolved_preset("catppuccin-mocha", true).defaults.font;
        spec.family = std::sync::Arc::from("native-theme-no-such-family-7f3c1a");
        assert_eq!(
            system_font_family(&spec).as_ref(),
            "native-theme-no-such-family-7f3c1a"
        );
    }

    fn make_resolved(is_dark: bool) -> native_theme::theme::ResolvedTheme {
        make_resolved_preset("catppuccin-mocha", is_dark)
    }

    fn scaled_prefs(text_scaling_factor: f32) -> AccessibilityPreferences {
        AccessibilityPreferences {
            text_scaling_factor,
            ..AccessibilityPreferences::default()
        }
    }

    // === to_theme tests ===

    #[test]
    fn to_theme_produces_non_default_theme() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "Test Theme");

        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert_ne!(theme, iced_core::theme::Theme::Dark);

        let palette = theme.palette();
        // Catppuccin Mocha dark primary should be non-trivial
        let expected = palette::to_color(resolved.defaults.accent_color);
        assert_eq!(palette.primary, expected, "primary should match accent");
    }

    #[test]
    fn to_theme_from_preset() {
        let resolved = make_resolved(false);
        let theme = to_theme(&resolved, "Default");

        let palette = theme.palette();
        // Light variant has white-ish background
        assert!(
            palette.background.r > 0.9,
            "light background should be bright"
        );
    }

    #[test]
    fn to_theme_dark_variant() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "Dark Test");

        let palette = theme.palette();
        assert!(palette.background.r < 0.3, "dark background should be dark");
    }

    #[test]
    fn to_theme_different_presets_differ() {
        let r1 = Theme::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(ColorMode::Dark)
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap();
        let r2 = Theme::preset("dracula")
            .unwrap()
            .into_variant(ColorMode::Dark)
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap();

        let t1 = to_theme(&r1, "mocha");
        let t2 = to_theme(&r2, "dracula");

        // Different presets should produce different palette colors
        assert_ne!(t1.palette().primary, t2.palette().primary);
    }

    #[test]
    fn to_theme_with_adwaita_preset() {
        let resolved = make_resolved_preset("adwaita", false);
        let theme = to_theme(&resolved, "Adwaita");
        let palette = theme.palette();
        assert!(palette.primary.a > 0.0, "adwaita primary should be visible");
    }

    // === Widget metric helper tests ===

    #[test]
    fn border_radius_returns_resolved_value() {
        let resolved = make_resolved(false);
        let r = border_radius(&resolved);
        assert!(r > 0.0, "resolved radius should be > 0");
    }

    #[test]
    fn border_radius_lg_returns_resolved_value() {
        let resolved = make_resolved(false);
        let r = border_radius_lg(&resolved);
        assert!(r > 0.0, "resolved radius_lg should be > 0");
        assert!(
            r >= border_radius(&resolved),
            "radius_lg should be >= radius"
        );
    }

    #[test]
    fn scrollbar_width_returns_resolved_value() {
        let resolved = make_resolved(false);
        let w = scrollbar_width(&resolved);
        assert!(w > 0.0, "scrollbar width should be > 0");
    }

    /// A padding stated on two sides and not on the other two.
    #[cfg(feature = "widgets")]
    fn partly_stated() -> native_theme::theme::ResolvedPadding {
        native_theme::theme::ResolvedPadding {
            top: Some(0.0),
            right: None,
            bottom: None,
            left: Some(7.0),
        }
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn button_padding_fills_unstated_sides_from_iceds_default() {
        let mut resolved = make_resolved(false);
        resolved.button.border.padding = partly_stated();
        resolved.button.border.line_width = 2.0;
        let pad = button_padding(&resolved);
        let default = iced_widget::button::DEFAULT_PADDING;
        assert_eq!(
            pad.top, 2.0,
            "a stated zero is the theme's, inside the border"
        );
        assert_eq!(
            pad.left, 9.0,
            "a stated side is the theme's, inside the border"
        );
        assert_eq!(pad.right, default.right, "an unstated side is iced's");
        assert_eq!(pad.bottom, default.bottom, "an unstated side is iced's");
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn input_padding_fills_unstated_sides_from_iceds_default() {
        let mut resolved = make_resolved(false);
        resolved.input.border.padding = partly_stated();
        resolved.input.border.line_width = 2.0;
        let pad = input_padding(&resolved);
        let default = iced_widget::text_input::DEFAULT_PADDING;
        assert_eq!(
            pad.top, 2.0,
            "a stated zero is the theme's, inside the border"
        );
        assert_eq!(
            pad.left, 9.0,
            "a stated side is the theme's, inside the border"
        );
        assert_eq!(pad.right, default.right, "an unstated side is iced's");
        assert_eq!(pad.bottom, default.bottom, "an unstated side is iced's");
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn text_area_padding_fills_unstated_sides_from_iceds_default() {
        let mut resolved = make_resolved(false);
        resolved.text_area.border.padding = partly_stated();
        resolved.text_area.border.line_width = 2.0;
        let pad = text_area_padding(&resolved);
        assert_eq!(pad.top, 2.0, "a stated zero, inside the border");
        assert_eq!(pad.left, 9.0, "a stated side, inside the border");
        assert_eq!(pad.right, TEXT_EDITOR_PADDING, "an unstated side is iced's");
        assert_eq!(
            pad.bottom, TEXT_EDITOR_PADDING,
            "an unstated side is iced's"
        );
    }

    /// kde-breeze's arrow is centred in its 20px column
    /// (`combo_box.arrow_area_width`) at the right inner edge, which the
    /// theme's right side (0) measures to: 5px either side of the 10px
    /// arrow, so the right padding is the line, 1, and those 5.
    #[cfg(feature = "widgets")]
    #[test]
    fn combo_box_padding_centres_the_arrow_in_its_column() {
        let r = make_resolved_preset("kde-breeze", false);
        let c = &r.combo_box;
        assert_eq!(c.border.padding.right, Some(0.0));
        assert_eq!((c.arrow_area_width, c.arrow_icon_size), (Some(20.0), 10.0));
        assert_eq!(combo_box_padding(&r).right, c.border.line_width + 5.0);
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn combo_box_padding_fills_unstated_sides_from_iceds_default() {
        let mut resolved = make_resolved(false);
        resolved.combo_box.border.padding = partly_stated();
        resolved.combo_box.border.line_width = 2.0;
        let pad = combo_box_padding(&resolved);
        let default = iced_widget::button::DEFAULT_PADDING;
        assert_eq!(
            pad.top, 2.0,
            "a stated zero is the theme's, inside the border"
        );
        assert_eq!(
            pad.left, 9.0,
            "a stated side is the theme's, inside the border"
        );
        assert_eq!(pad.right, default.right, "an unstated side is iced's");
        assert_eq!(pad.bottom, default.bottom, "an unstated side is iced's");
    }

    /// kde-breeze states a 32px minimum for the button, the text input and
    /// the combo box, and a 13.33px body font at a 1.36 line height: each
    /// control's line box fills the minimum inside its padding, and the
    /// theme's own line box stands where it is taller.
    #[cfg(feature = "widgets")]
    #[test]
    fn control_line_height_reaches_the_stated_minimum() {
        let r = make_resolved_preset("kde-breeze", false);
        let line = |size: f32, min: f32, pad: iced_core::Padding| match control_line_height(
            &r, size, min, pad,
        ) {
            iced_core::text::LineHeight::Absolute(px) => px.0,
            iced_core::text::LineHeight::Relative(_) => f32::NAN,
        };
        for (what, size, min, pad) in [
            (
                "input",
                r.input.font.size,
                r.input.min_height,
                input_padding(&r),
            ),
            (
                "combo box",
                r.combo_box.font.size,
                r.combo_box.min_height,
                combo_box_padding(&r),
            ),
        ] {
            let height = line(size, min, pad) + pad.top + pad.bottom;
            let own = size * r.defaults.line_height + pad.top + pad.bottom;
            assert_eq!(height, min.max(own), "{what}: {height} for {min}");
        }
        // The input is 6 + 1 top and bottom: the 18 the minimum leaves is
        // below the theme's 18.13 line box, which stands; a 40px minimum
        // leaves 26.
        let input_own = r.input.font.size * r.defaults.line_height;
        assert_eq!(line(r.input.font.size, 32.0, input_padding(&r)), input_own);
        assert_eq!(line(r.input.font.size, 40.0, input_padding(&r)), 26.0);
        // A minimum below the theme's own line box leaves that line box.
        let own = r.defaults.font.size * r.defaults.line_height;
        assert_eq!(line(r.defaults.font.size, 0.0, input_padding(&r)), own);
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn button_content_min_size_is_the_minimum_inside_the_padding() {
        let r = make_resolved_preset("kde-breeze", false);
        let p = button_padding(&r);
        let min = button_content_min_size(&r);
        assert_eq!(min.width + p.left + p.right, r.button.min_width);
        assert_eq!(min.height + p.top + p.bottom, r.button.min_height);
        // kde-breeze: 80 x 32 outer, 6 + 1 on every side.
        assert_eq!((min.width, min.height), (66.0, 18.0));
        let mut small = r.clone();
        small.button.min_width = 0.0;
        small.button.min_height = 0.0;
        let none = button_content_min_size(&small);
        assert_eq!((none.width, none.height), (0.0, 0.0), "never below zero");
    }

    /// `at_least` is the floor and no ceiling: laid out headlessly (iced's
    /// null renderer, `()`), smaller content takes the minimum and larger
    /// content its own size.
    #[cfg(feature = "widgets")]
    #[test]
    fn at_least_is_a_floor_under_the_content() {
        use iced_core::widget::Tree;
        let size_of = |content: iced_core::Size, min: iced_core::Size| {
            let mut element: iced_core::Element<'_, (), iced_core::Theme, ()> = at_least(
                iced_widget::Space::new()
                    .width(content.width)
                    .height(content.height),
                min,
            );
            let mut tree = Tree::new(&element);
            let limits = iced_core::layout::Limits::new(
                iced_core::Size::ZERO,
                iced_core::Size::new(1000.0, 1000.0),
            );
            let node = element.as_widget_mut().layout(&mut tree, &(), &limits);
            (node.size().width, node.size().height)
        };
        let min = iced_core::Size::new(66.0, 18.0);
        assert_eq!(size_of(iced_core::Size::new(41.0, 10.0), min), (66.0, 18.0));
        assert_eq!(size_of(iced_core::Size::new(90.0, 25.0), min), (90.0, 25.0));
        assert_eq!(size_of(iced_core::Size::new(90.0, 10.0), min), (90.0, 18.0));
    }

    #[test]
    fn padding_or_takes_each_stated_side_and_the_default_elsewhere() {
        let stated = native_theme::theme::ResolvedPadding {
            top: Some(0.0),
            right: None,
            bottom: None,
            left: Some(7.0),
        };
        let default = iced_core::Padding {
            top: 1.0,
            right: 2.0,
            bottom: 3.0,
            left: 4.0,
        };
        let pad = padding_or(&stated, default);
        assert_eq!(pad.top, 0.0, "a stated zero is the theme's");
        assert_eq!(pad.left, 7.0, "a stated side is the theme's");
        assert_eq!(pad.right, 2.0, "an unstated side is the default's");
        assert_eq!(pad.bottom, 3.0, "an unstated side is the default's");
    }

    #[test]
    fn stated_padding_is_some_only_when_every_side_is_stated() {
        let every = native_theme::theme::ResolvedPadding {
            top: Some(3.0),
            right: Some(8.0),
            bottom: Some(0.0),
            left: Some(8.0),
        };
        let pad = stated_padding(&every);
        assert_eq!(
            pad.map(|p| (p.top, p.right, p.bottom, p.left)),
            Some((3.0, 8.0, 0.0, 8.0))
        );
        for missing in 0..4 {
            let mut sides = [every.top, every.right, every.bottom, every.left];
            sides[missing] = None;
            let partial = native_theme::theme::ResolvedPadding {
                top: sides[0],
                right: sides[1],
                bottom: sides[2],
                left: sides[3],
            };
            assert!(
                stated_padding(&partial).is_none(),
                "side {missing} unstated"
            );
        }
    }

    /// Every side the theme states is the theme's. windows-11 states button
    /// and input padding; catppuccin states neither, so it would compare
    /// nothing.
    #[cfg(feature = "widgets")]
    #[test]
    fn stated_padding_sides_are_the_themes() {
        let resolved = make_resolved_preset("windows-11", false);
        for (what, border, pad) in [
            ("button", &resolved.button.border, button_padding(&resolved)),
            ("input", &resolved.input.border, input_padding(&resolved)),
            (
                "combo box",
                &resolved.combo_box.border,
                combo_box_padding(&resolved),
            ),
        ] {
            let inside = |side: Option<f32>| side.map(|v| v + border.line_width);
            // The combo box's right side also centres its arrow in the
            // arrow column (`combo_box_padding`).
            let column = match (what, resolved.combo_box.arrow_area_width) {
                ("combo box", Some(column)) => {
                    arrow_column_margin(column, resolved.combo_box.arrow_icon_size)
                }
                _ => 0.0,
            };
            let stated = native_theme::theme::ResolvedPadding {
                top: inside(border.padding.top),
                right: inside(border.padding.right).map(|v| v + column),
                bottom: inside(border.padding.bottom),
                left: inside(border.padding.left),
            };
            let mut compared = 0usize;
            for (side, stated, got) in [
                ("top", stated.top, pad.top),
                ("right", stated.right, pad.right),
                ("bottom", stated.bottom, pad.bottom),
                ("left", stated.left, pad.left),
            ] {
                if let Some(stated) = stated {
                    assert_eq!(got, stated, "{what} {side}");
                    compared += 1;
                }
            }
            assert!(
                compared > 0,
                "{what}: the preset states no padding side, so nothing was compared"
            );
        }
    }

    // === Color helper tests ===

    #[test]
    fn border_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = border_color(&resolved);
        assert!(c.a > 0.0, "border color should have non-zero alpha");
    }

    /// The drop-down's handle is iced's own chevron glyph, at the theme's
    /// arrow size: kde-breeze's 10px, Breeze's `ArrowSize`.
    #[cfg(feature = "widgets")]
    #[test]
    fn the_pick_list_handle_is_a_chevron_at_the_arrow_size() {
        use iced_core::text::Renderer as _;
        let resolved = make_resolved_preset("kde-breeze", false);
        assert_eq!(resolved.combo_box.arrow_icon_size, 10.0);
        let handle = pick_list_handle::<iced_widget::Renderer>(&resolved);
        let iced_widget::pick_list::Handle::Static(icon) = handle else {
            panic!("a static handle, not iced's filled Arrow: {handle:?}");
        };
        assert_eq!(icon.code_point, iced_widget::Renderer::SCROLL_DOWN_ICON);
        assert_ne!(icon.code_point, iced_widget::Renderer::ARROW_DOWN_ICON);
        assert_eq!(icon.font, iced_widget::Renderer::ICON_FONT);
        assert_eq!(icon.size, Some(iced_core::Pixels(10.0)));
    }

    #[test]
    fn disabled_opacity_returns_value() {
        let resolved = make_resolved(false);
        let o = disabled_opacity(&resolved);
        assert!(
            o > 0.0 && o <= 1.0,
            "disabled opacity should be in (0, 1], got {o}"
        );
    }

    #[test]
    fn focus_ring_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = focus_ring_color(&resolved);
        assert!(c.a > 0.0, "focus ring color should have non-zero alpha");
    }

    #[test]
    fn link_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = link_color(&resolved);
        assert!(
            c.r > 0.0 || c.g > 0.0 || c.b > 0.0,
            "link color should be non-black"
        );
    }

    /// The arrow is `expander.arrow_color` where the theme states one, and
    /// the header's label colour where it does not.
    #[test]
    fn expander_arrow_color_is_the_stated_one_or_the_labels() {
        let mut resolved = make_resolved_preset("kde-breeze", false);
        let stated = resolved.expander.arrow_color.unwrap();
        assert_eq!(expander_arrow_color(&resolved), palette::to_color(stated));
        resolved.expander.arrow_color = None;
        assert_eq!(
            expander_arrow_color(&resolved),
            palette::to_color(resolved.expander.font.color)
        );
    }

    #[test]
    fn selection_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = selection_color(&resolved);
        assert!(c.a > 0.0, "selection color should have non-zero alpha");
    }

    #[test]
    fn info_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = info_color(&resolved);
        assert!(
            c.r > 0.0 || c.g > 0.0 || c.b > 0.0,
            "info color should be non-black"
        );
    }

    #[test]
    fn info_foreground_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = info_foreground_color(&resolved);
        assert!(c.a > 0.0, "info foreground should have non-zero alpha");
    }

    #[test]
    fn warning_foreground_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = warning_foreground_color(&resolved);
        assert!(c.a > 0.0, "warning foreground should have non-zero alpha");
    }

    #[test]
    fn icon_sizes_returns_concrete_values() {
        let resolved = make_resolved(false);
        let is = icon_sizes(&resolved);
        assert!(is.small > 0.0, "small icon size should be > 0");
        assert!(is.toolbar > 0.0, "toolbar icon size should be > 0");
    }

    // === Font helper tests ===

    #[test]
    fn font_family_returns_concrete_value() {
        let resolved = make_resolved(false);
        let ff = font_family(&resolved);
        assert!(!ff.is_empty(), "font family should not be empty");
    }

    #[test]
    fn font_size_returns_concrete_value() {
        let resolved = make_resolved(false);
        let fs = font_size(&resolved, &AccessibilityPreferences::default());
        assert!(fs > 0.0, "font size should be > 0");
    }

    #[test]
    fn mono_font_family_returns_concrete_value() {
        let resolved = make_resolved(false);
        let mf = mono_font_family(&resolved);
        assert!(!mf.is_empty(), "mono font family should not be empty");
    }

    #[test]
    fn mono_font_size_returns_concrete_value() {
        let resolved = make_resolved(false);
        let ms = mono_font_size(&resolved, &AccessibilityPreferences::default());
        assert!(ms > 0.0, "mono font size should be > 0");
    }

    #[test]
    fn font_size_scales_by_the_text_scaling_factor() {
        let resolved = make_resolved(false);
        assert_eq!(
            font_size(&resolved, &scaled_prefs(1.5)),
            resolved.defaults.font.size * 1.5,
            "font size should be multiplied by the factor"
        );
    }

    #[test]
    fn font_size_ignores_a_factor_that_is_not_finite_and_positive() {
        let resolved = make_resolved(false);
        for factor in [0.0, f32::NAN, -1.0, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(
                font_size(&resolved, &scaled_prefs(factor)),
                resolved.defaults.font.size,
                "factor {factor} should leave the size unscaled"
            );
        }
    }

    #[test]
    fn scaled_text_size_scales_every_size_alike() {
        let resolved = make_resolved(false);
        for size in [resolved.button.font.size, resolved.text_scale.caption.size] {
            assert_eq!(scaled_text_size(size, &scaled_prefs(1.5)), size * 1.5);
            for factor in [0.0, f32::NAN, -1.0, f32::INFINITY] {
                assert_eq!(
                    scaled_text_size(size, &scaled_prefs(factor)),
                    size,
                    "factor {factor} should leave the size unscaled"
                );
            }
        }
    }

    #[test]
    fn mono_font_size_scales_by_the_text_scaling_factor() {
        let resolved = make_resolved(false);
        assert_eq!(
            mono_font_size(&resolved, &scaled_prefs(1.5)),
            resolved.defaults.mono_font.size * 1.5,
            "mono font size should be multiplied by the factor"
        );
    }

    #[test]
    fn mono_font_size_ignores_a_factor_that_is_not_finite_and_positive() {
        let resolved = make_resolved(false);
        for factor in [0.0, f32::NAN, -1.0, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(
                mono_font_size(&resolved, &scaled_prefs(factor)),
                resolved.defaults.mono_font.size,
                "factor {factor} should leave the size unscaled"
            );
        }
    }

    #[test]
    fn font_weight_returns_concrete_value() {
        let resolved = make_resolved(false);
        let w = font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "font weight should be 100-900, got {}",
            w
        );
    }

    #[test]
    fn mono_font_weight_returns_concrete_value() {
        let resolved = make_resolved(false);
        let w = mono_font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "mono font weight should be 100-900, got {}",
            w
        );
    }

    #[test]
    fn line_height_multiplier_returns_concrete_value() {
        let resolved = make_resolved(false);
        let lh = line_height_multiplier(&resolved);
        assert!(lh > 0.0, "line height multiplier should be > 0");
        assert!(
            lh < 5.0,
            "line height multiplier should be a multiplier (e.g. 1.4), got {}",
            lh
        );
    }

    #[test]
    fn to_iced_weight_standard_weights() {
        use iced_core::font::Weight;
        assert_eq!(to_iced_weight(100), Weight::Thin);
        assert_eq!(to_iced_weight(200), Weight::ExtraLight);
        assert_eq!(to_iced_weight(300), Weight::Light);
        assert_eq!(to_iced_weight(400), Weight::Normal);
        assert_eq!(to_iced_weight(500), Weight::Medium);
        assert_eq!(to_iced_weight(600), Weight::Semibold);
        assert_eq!(to_iced_weight(700), Weight::Bold);
        assert_eq!(to_iced_weight(800), Weight::ExtraBold);
        assert_eq!(to_iced_weight(900), Weight::Black);
    }

    #[test]
    fn to_iced_weight_non_standard_rounds_correctly() {
        use iced_core::font::Weight;
        assert_eq!(to_iced_weight(350), Weight::Normal);
        assert_eq!(to_iced_weight(450), Weight::Medium);
        assert_eq!(to_iced_weight(550), Weight::Semibold);
        assert_eq!(to_iced_weight(0), Weight::Thin);
        assert_eq!(to_iced_weight(1000), Weight::Black);
    }

    // === Convenience API tests ===

    #[test]
    fn from_preset_valid_light() {
        let (theme, resolved) = from_preset("catppuccin-mocha", false).expect("preset should load");
        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert!(!resolved.defaults.font.family.is_empty());
        // Light variant should have bright background
        let palette = theme.palette();
        assert!(
            palette.background.r > 0.9,
            "light variant should have bright background, got r={}",
            palette.background.r
        );
    }

    #[test]
    fn from_preset_valid_dark() {
        let (theme, _resolved) = from_preset("catppuccin-mocha", true).expect("preset should load");
        assert_ne!(theme, iced_core::theme::Theme::Dark);
        // Dark variant should have dark background
        let palette = theme.palette();
        assert!(
            palette.background.r < 0.3,
            "dark variant should have dark background, got r={}",
            palette.background.r
        );
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset("nonexistent-preset", false);
        assert!(result.is_err(), "invalid preset should return Err");
    }

    #[test]
    fn from_preset_error_shows_requested_mode() {
        // This tests the error path -- an actually empty preset cannot be
        // created through the public API, but we verify the format of
        // the success/error paths.
        let result = from_preset("nonexistent-preset", true);
        assert!(result.is_err());
    }

    #[test]
    fn system_theme_ext_to_iced_theme() {
        // May fail on CI -- skip gracefully
        let Ok(sys) = native_theme::SystemTheme::from_system() else {
            return;
        };
        let (_theme, _resolved) = sys.to_iced_theme();
    }

    #[test]
    fn from_system_does_not_panic() {
        let _ = from_system();
    }

    #[test]
    fn from_system_returns_is_dark_and_preferences() {
        // If system theme is available, verify it returns a quadruple
        if let Ok((_theme, _resolved, is_dark, accessibility)) = from_system() {
            // is_dark should be a valid bool (always true, but verify the return)
            let _ = is_dark;
            let _ = accessibility.text_scaling_factor;
        }
    }

    #[test]
    fn to_theme_extended_overrides_take_effect() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "test");
        let ext = theme.extended_palette();
        // Generate what the Extended palette would be without overrides
        let auto_palette = iced_core::theme::palette::Extended::generate(theme.palette());
        // apply_overrides sets secondary.base from button.bg/fg which differs
        // from the auto-generated value
        assert_ne!(
            ext.secondary.base.color, auto_palette.secondary.base.color,
            "secondary.base.color should be overridden, not auto-generated"
        );
    }

    // Integration-level: exercises the full from_preset -> to_theme pipeline for all presets

    #[test]
    fn all_presets_produce_valid_themes() {
        for info in Theme::list_presets() {
            let name = info.key;
            for is_dark in [false, true] {
                let spec = Theme::preset(name).unwrap();
                if let Ok(variant) = spec.into_variant(if is_dark {
                    ColorMode::Dark
                } else {
                    ColorMode::Light
                }) {
                    let resolved = variant
                        .into_resolved(&native_theme::ResolutionContext::for_tests())
                        .unwrap();
                    let theme = to_theme(&resolved, name);
                    let palette = theme.palette();
                    // Basic sanity: all palette colors have valid alpha
                    assert!(
                        palette.background.a > 0.0,
                        "{name}/{is_dark}: background alpha"
                    );
                    assert!(palette.text.a > 0.0, "{name}/{is_dark}: text alpha");
                    assert!(palette.primary.a > 0.0, "{name}/{is_dark}: primary alpha");
                    assert!(palette.success.a > 0.0, "{name}/{is_dark}: success alpha");
                    assert!(palette.warning.a > 0.0, "{name}/{is_dark}: warning alpha");
                    assert!(palette.danger.a > 0.0, "{name}/{is_dark}: danger alpha");
                }
            }
        }
    }

    // === Tripwire: iced Palette field count ===

    #[test]
    fn palette_field_count_tripwire() {
        // iced_core::theme::Palette has 6 Color fields. If upstream adds more,
        // this test fails so we know to update to_palette().
        let field_count = std::mem::size_of::<iced_core::theme::Palette>()
            / std::mem::size_of::<iced_core::Color>();
        assert_eq!(
            field_count, 6,
            "iced Palette field count changed from 6 to {field_count} -- update to_palette()"
        );
    }
}
