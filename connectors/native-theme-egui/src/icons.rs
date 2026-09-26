//! native-theme icon payloads → egui image sources.

/// Re-exported **here and not at the crate root**, because `egui::IconData`
/// (`egui/src/viewport.rs:183`, the window/taskbar icon) already occupies that name and this
/// crate re-exports `egui`.
pub use native_theme::theme::IconData;

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hasher};
use std::time::Duration;

use native_theme::color::Rgba;
use native_theme::theme::{AnimatedIcon, IconProvider, IconRole, IconSet, TransformAnimation};

use crate::{ResolvedTheme, ThemeAtlas};

/// Which of native-theme's five per-context icon sizes to use
/// (`ResolvedIconSizes`, `native-theme/src/model/resolved.rs:15-26`). egui has no icon-size
/// vocabulary of its own (§9.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconContext {
    /// Small icon size for inline use: `defaults.icon_sizes.small`.
    Small,
    /// Icon size for toolbar buttons: `defaults.icon_sizes.toolbar`.
    Toolbar,
    /// Icon size for panel headers: `defaults.icon_sizes.panel`.
    Panel,
    /// Icon size for dialog buttons: `defaults.icon_sizes.dialog`.
    Dialog,
    /// Large icon size for menus and lists: `defaults.icon_sizes.large`.
    Large,
}

/// The icon size for a context, in logical pixels, for
/// `Image::fit_to_exact_size(Vec2::splat(..))` (`egui/src/widgets/image.rs:177`). Do **not**
/// pre-multiply by `Context::pixels_per_point`.
#[must_use]
pub fn icon_size(theme: &ResolvedTheme, context: IconContext) -> f32 {
    let sizes = &theme.defaults.icon_sizes;
    match context {
        IconContext::Small => sizes.small,
        IconContext::Toolbar => sizes.toolbar,
        IconContext::Panel => sizes.panel,
        IconContext::Dialog => sizes.dialog,
        IconContext::Large => sizes.large,
    }
}

const URI_PREFIX: &str = "bytes://native-theme/";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum KeyId {
    Role(IconRole),
    Name(String),
}

/// The inputs an icon is looked up and coloured by: the role or name, the set, the icon
/// theme, the size and the tint. They decide the bytes, whose hash alone makes the URI
/// (§9.2), and the role or name gives [`to_image`] its alt text. Constructed through the builder below: private
/// fields, because a
/// `#[non_exhaustive]` struct with no constructor cannot be built outside its defining crate
/// at all (`E0639`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IconKey {
    id: KeyId,
    set: IconSet,
    icon_theme: Option<String>,
    size: Option<u16>,
    tint: Option<egui::Color32>,
}

impl IconKey {
    /// Start a key for a role in a set.
    #[must_use]
    pub fn role(role: IconRole, set: IconSet) -> Self {
        Self {
            id: KeyId::Role(role),
            set,
            icon_theme: None,
            size: None,
            tint: None,
        }
    }

    /// Start a key for a named icon.
    #[must_use]
    pub fn name(name: &str, set: IconSet) -> Self {
        Self {
            id: KeyId::Name(name.to_owned()),
            set,
            icon_theme: None,
            size: None,
            tint: None,
        }
    }

    /// The freedesktop icon-theme name the icon was looked up in; given only when there is
    /// one — `if let Some(name) = atlas.icon_theme(ctx.theme()) { key = key.icon_theme(name) }`
    /// ([`crate::ThemeAtlas::icon_theme`], §9.2).
    #[must_use]
    pub fn icon_theme(mut self, icon_theme: &str) -> Self {
        self.icon_theme = Some(icon_theme.to_owned());
        self
    }

    /// The requested edge length in logical pixels; freedesktop lookup is size-dependent
    /// (`native-theme/src/icons.rs:137`). Stored as a private `u16` — the type
    /// `native_theme::icons::FreedesktopLoader::size` takes (`native-theme/src/icons.rs:137`)
    /// — computed as `(if points > 0.0 { points.round() } else { 0.0 }) as u16`, which is
    /// total over every `f32`: `NaN`, `-0.0` and every negative fold to the single key `0`,
    /// and the float-to-int `as` cast saturates at `65535` (§7.1). An integer, not an `f32`,
    /// is what lets `IconKey` derive `Eq` and `Hash`. Without this call the key has no size,
    /// and [`custom_icon_to_image_source`]'s freedesktop lookup takes `FreedesktopLoader`'s
    /// own default, 24 (`native-theme/src/icons.rs:126`).
    #[must_use]
    pub fn size(mut self, points: f32) -> Self {
        let rounded = if points > 0.0 { points.round() } else { 0.0 };
        // The float-to-int `as` cast saturates and folds NaN to 0 (§7.1).
        self.size = Some(rounded as u16);
        self
    }

    /// The colour a monochrome SVG icon is drawn in. [`to_image_source`] bakes it into the
    /// `IconData::Svg` bytes — with `native_theme::icons::colorize_monochrome_svg` for a
    /// bundled set, by replacing `currentColor` for a freedesktop icon — so the pixels, and
    /// through their hash the URI, carry it (§9.2). Not `egui::Image::tint`, a draw-time
    /// multiply that leaves black black. Unset, a bundled set's bytes are used as they are,
    /// which is right for a full-colour icon, and a freedesktop icon takes the text colour. A
    /// freedesktop icon whose own stylesheet sets its `color`, as nearly every Breeze
    /// `currentColor` icon does, keeps it, tint or not; the tint colours bundled monochrome sets and the
    /// freedesktop icons that set none (§9.2).
    #[must_use]
    pub fn tint(mut self, tint: egui::Color32) -> Self {
        self.tint = Some(tint);
        self
    }

    /// The alt text `to_image` gives: the role's `IconRole::name()`, or the key's name.
    fn alt_text(&self) -> &str {
        match &self.id {
            KeyId::Role(role) => role.name(),
            KeyId::Name(name) => name,
        }
    }

    /// The key's tint as native-theme's colour type, unmultiplied sRGBA
    /// (`Color32::to_srgba_unmultiplied`, `ecolor/src/color32.rs:248`).
    fn tint_rgba(&self) -> Option<Rgba> {
        self.tint.map(|tint| {
            let [r, g, b, a] = tint.to_srgba_unmultiplied();
            Rgba::new(r, g, b, a)
        })
    }
}

/// `#rrggbb` of a colour, as an SVG `fill` takes it (§9.2; the alpha is discarded).
pub(crate) fn hex_rgb(c: Rgba) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)
}

/// §9.2's predicate: does the document set a `color` property — the name `color` not
/// preceded by a letter, digit, `-` or `_`, then optional whitespace and `:` or `=` —
/// anywhere in it? usvg takes `color` from the nearest ancestor that sets it, so the test is
/// over the whole document.
pub(crate) fn sets_color(svg: &str) -> bool {
    let bytes = svg.as_bytes();
    svg.match_indices("color").any(|(at, _)| {
        let preceded = at
            .checked_sub(1)
            .and_then(|i| bytes.get(i))
            .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'-' || *b == b'_');
        if preceded {
            return false;
        }
        at.checked_add("color".len())
            .and_then(|i| bytes.get(i..))
            .and_then(|rest| rest.iter().find(|b| !b.is_ascii_whitespace()))
            .is_some_and(|b| *b == b':' || *b == b'=')
    })
}

/// The 64-bit hash of the final bytes, their length and, for a raster icon, its size, as
/// `bytes://native-theme/<16 hex digits>`, with `.svg` for an SVG (§9.2). `DefaultHasher::new`
/// is keyed the same each time (std's `hash/random.rs`, lines 97–107), unlike a `RandomState`.
fn hashed_uri(bytes: &[u8], raster_size: Option<(u32, u32)>, svg: bool) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(bytes);
    hasher.write_usize(bytes.len());
    if let Some((width, height)) = raster_size {
        hasher.write_u32(width);
        hasher.write_u32(height);
    }
    let hash = hasher.finish();
    if svg {
        format!("{URI_PREFIX}{hash:016x}.svg")
    } else {
        format!("{URI_PREFIX}{hash:016x}")
    }
}

/// The URI of these bytes: `bytes://native-theme/` and the 16 lower-case hex digits of a
/// 64-bit hash of `icon`'s bytes and, for `IconData::Rgba`, its width and height, ending in
/// `.svg` for `IconData::Svg` (§9.2). Pass the final bytes; [`to_image_source`] calls it with
/// the bytes after colouring.
// An `IconData` variant this crate does not know hashes its bytes with no extension
// (`to_image_source` returns `None` for it before any URI is used).
#[must_use]
pub fn uri(icon: &IconData) -> String {
    match icon {
        IconData::Svg(bytes) => hashed_uri(bytes, None, true),
        IconData::Rgba {
            width,
            height,
            data,
        } => hashed_uri(data, Some((*width, *height)), false),
        _ => hashed_uri(icon.bytes(), None, false),
    }
}

/// The colour a freedesktop icon is drawn in: the key's tint, else `defaults.text_color` of the
/// installed atlas's theme for `ctx.theme()`; `None` with neither (§9.2). Two `Context`
/// accessors, one after the other, never nested (§10.3).
fn freedesktop_color(ctx: &egui::Context, key: &IconKey) -> Option<Rgba> {
    if let Some(tint) = key.tint_rgba() {
        return Some(tint);
    }
    let atlas = ThemeAtlas::from_ctx(ctx)?;
    let theme = ctx.theme();
    Some(atlas.resolved_for(theme).defaults.text_color)
}

/// The final SVG bytes (§9.2): a bundled set's coloured with the key's tint through
/// `colorize_monochrome_svg`; a freedesktop icon's `currentColor` replaced by `c`'s `#rrggbb`
/// where the bytes still hold `currentColor` and the document sets no `color` of its own
/// (`str::replace`, iced's step, `connectors/native-theme-iced/src/icons.rs:292-293`); every
/// other icon's bytes as they are.
fn final_svg_bytes(key: &IconKey, bytes: &[u8], freedesktop: Option<Rgba>) -> Vec<u8> {
    match key.set {
        IconSet::Freedesktop => {
            let Some(c) = freedesktop else {
                return bytes.to_vec();
            };
            match std::str::from_utf8(bytes) {
                Ok(text) if text.contains("currentColor") && !sets_color(text) => {
                    text.replace("currentColor", &hex_rgb(c)).into_bytes()
                }
                _ => bytes.to_vec(),
            }
        }
        _ => match key.tint_rgba() {
            Some(tint) => native_theme::icons::colorize_monochrome_svg(bytes, tint),
            None => bytes.to_vec(),
        },
    }
}

/// The URIs this crate registered in a `Context`, for `forget_icons`; stored under
/// `handles_key()`.
#[derive(Clone, Default)]
pub(crate) struct IconRegistry {
    uris: BTreeSet<String>,
}

/// The `Context` data key of the registry, `Id::new("native-theme-egui/icon-handles")`.
pub(crate) fn handles_key() -> egui::Id {
    egui::Id::new("native-theme-egui/icon-handles")
}

/// The `Context` data key of the stored `TextureHandle` of a raster icon's URI.
pub(crate) fn texture_id(uri: &str) -> egui::Id {
    egui::Id::new(("native-theme-egui/icon-texture", uri))
}

fn remember_uri(ctx: &egui::Context, uri: &str) {
    ctx.data_mut(|d| {
        // Called on every draw of an icon: the `String` is allocated only for a new URI.
        let uris = &mut d
            .get_temp_mut_or_default::<IconRegistry>(handles_key())
            .uris;
        if !uris.contains(uri) {
            uris.insert(uri.to_owned());
        }
    });
}

/// Convert a decoded RGBA icon into an `egui::ColorImage`; `None`, never a panic, for a zero
/// side, an overflowing `width * height * 4` or a buffer of another length (§9.3), and for
/// an `IconData` variant this crate does not know (`IconData` is `#[non_exhaustive]`,
/// `native-theme/src/model/icons.rs:292`), and for an `IconData::Svg`, which this crate never
/// decodes (§9.1). The
/// oversize test needs a `Context` and lives in [`to_image_source`] and [`to_image`].
#[must_use]
pub fn to_color_image(icon: &IconData) -> Option<egui::ColorImage> {
    let IconData::Rgba {
        width,
        height,
        data,
    } = icon
    else {
        return None;
    };
    let w = usize::try_from(*width).ok()?;
    let h = usize::try_from(*height).ok()?;
    if w == 0 || h == 0 {
        return None;
    }
    let expected = w.checked_mul(h)?.checked_mul(4)?;
    if expected != data.len() {
        return None;
    }
    Some(egui::ColorImage::from_rgba_unmultiplied([w, h], data))
}

/// Build an `egui::ImageSource` for an icon and register its URI for [`forget_icons`]:
/// `IconData::Svg` → `ImageSource::Bytes` under the `.svg` URI of its final bytes — for a
/// bundled set, coloured with the key's tint when it has one ([`IconKey::tint`]); for an
/// `IconSet::Freedesktop` key whose bytes still contain `currentColor` and set no `color`
/// of their own, `currentColor` replaced by the `#rrggbb` of the tint, else of
/// `defaults.text_color` of the installed atlas's `ResolvedTheme` for `ctx.theme()` (the
/// bytes as they are where no atlas is installed); a full-colour icon, and one whose
/// stylesheet sets its `color`, untouched (§9.2). Load a freedesktop icon for it with `FreedesktopLoader::color` in the
/// same colour (§9.2). `IconData::Rgba` →
/// `ImageSource::Texture`, uploaded once via `Context::load_texture`
/// (`egui/src/context.rs:2390`) under the same URI and reused afterwards. `None` for an
/// image with a side above `max_texture_side` — rejected, never resampled (§9.3) — and for
/// an `IconData` variant this crate does not know.
///
/// **Texture ownership is part of the contract.** `egui::TextureHandle` frees its texture on
/// drop (`epaint/src/texture_handle.rs:25-29`) while `ImageSource::Texture(SizedTexture)`
/// owns nothing (`egui/src/widgets/image.rs:586`), so this **stores** the handle in
/// `ctx.data_mut()` keyed by the URI — it is `Clone` (`epaint/src/texture_handle.rs:31-39`)
/// and `Send + Sync`, as `IdTypeMap::insert_temp` requires
/// (`egui/src/util/id_type_map.rs:425`) — and returns
/// `ImageSource::Texture(SizedTexture::from_handle(&handle))` (`egui/src/load.rs:473`). The
/// cache is read in one `ctx.data_mut`, the texture uploaded outside it, and inserted in a
/// second one, because `load_texture` itself takes the `Context` lock (§10.3).
#[must_use]
pub fn to_image_source(
    ctx: &egui::Context,
    key: &IconKey,
    icon: &IconData,
) -> Option<egui::ImageSource<'static>> {
    match icon {
        IconData::Svg(bytes) => {
            let freedesktop = if key.set == IconSet::Freedesktop {
                freedesktop_color(ctx, key)
            } else {
                None
            };
            let final_bytes = final_svg_bytes(key, bytes, freedesktop);
            let uri = hashed_uri(&final_bytes, None, true);
            remember_uri(ctx, &uri);
            Some(egui::ImageSource::Bytes {
                uri: Cow::Owned(uri),
                bytes: egui::load::Bytes::from(final_bytes),
            })
        }
        IconData::Rgba { width, height, .. } => {
            let max = ctx.input(|i| i.max_texture_side);
            let too_big = |side: u32| usize::try_from(side).map_or(true, |s| s > max);
            if too_big(*width) || too_big(*height) {
                return None;
            }
            let uri = uri(icon);
            let handle_id = texture_id(&uri);
            let stored = ctx.data(|d| d.get_temp::<egui::TextureHandle>(handle_id));
            let handle = match stored {
                Some(handle) => handle,
                None => {
                    let image = to_color_image(icon)?;
                    // Outside every accessor closure: `load_texture` takes the lock itself (§10.3).
                    let handle = ctx.load_texture(uri.clone(), image, egui::TextureOptions::LINEAR);
                    ctx.data_mut(|d| d.insert_temp(handle_id, handle.clone()));
                    handle
                }
            };
            remember_uri(ctx, &uri);
            Some(egui::ImageSource::Texture(
                egui::load::SizedTexture::from_handle(&handle),
            ))
        }
        _ => None,
    }
}

/// [`to_image_source`], with its colouring and its URI, wrapped in an `egui::Image` with
/// `alt_text` from the key's role,
/// `IconRole::name()` (`native-theme/src/model/icons.rs:164`), or the key's name, which feeds
/// `WidgetInfo.label` (`egui/src/widgets/image.rs:406-410`) and is shown on load failure
/// (`:678-703`). `None` where [`to_image_source`] gives
/// `None`, an `IconData` variant this crate does not know included.
#[must_use]
pub fn to_image(
    ctx: &egui::Context,
    key: &IconKey,
    icon: &IconData,
) -> Option<egui::Image<'static>> {
    let source = to_image_source(ctx, key, icon)?;
    Some(egui::Image::new(source).alt_text(key.alt_text()))
}

/// Same, for an application-supplied [`IconProvider`]. Loads through native-theme's
/// custom-provider path for the key's set — `FreedesktopLoader::new(provider)` with the key's
/// icon theme and size, and `FreedesktopLoader::color` in the colour [`to_image_source`]
/// uses, for `IconSet::Freedesktop`, else `native_theme::icons::load_icon(provider, set)`
/// (`native-theme/src/icons.rs:487`), each trying `provider.icon_name(set)` then
/// `icon_svg(set)` — then colours and keys the bytes as [`to_image_source`] does, so the
/// provider's bytes are what the URI's hash covers. `None` where the provider has none. Build
/// the key with `IconKey::name(n, set)`, `n` its `icon_name(set)` where it has one; where it
/// has none, a name that describes it, which becomes its alt text — the URI is the hash of
/// the bytes alone (§9.2).
#[must_use]
pub fn custom_icon_to_image_source(
    ctx: &egui::Context,
    provider: &dyn IconProvider,
    key: &IconKey,
) -> Option<egui::ImageSource<'static>> {
    let icon = match key.set {
        IconSet::Freedesktop => {
            let mut loader = native_theme::icons::FreedesktopLoader::new(provider);
            if let Some(theme) = key.icon_theme.as_deref() {
                loader = loader.theme(theme);
            }
            if let Some(size) = key.size {
                loader = loader.size(size);
            }
            if let Some(c) = freedesktop_color(ctx, key) {
                loader = loader.color([c.r, c.g, c.b]);
            }
            loader.load()?
        }
        set => native_theme::icons::load_icon(provider, set)?,
    };
    to_image_source(ctx, key, &icon)
}

/// Drop every icon this crate cached in `ctx`, leaving unrelated application images alone;
/// [`crate::ThemeAtlas::install`] calls it. Two stores, both required: the `bytes://` URIs
/// are released with `Context::forget_image` (`egui/src/context.rs:3764`), which reaches only
/// the loader caches (`:3771-3780`), and the stored `TextureHandle`s are removed from
/// `ctx.data_mut()`, which is what frees their textures — `forget_image` never reaches a
/// handle this crate stored.
pub fn forget_icons(ctx: &egui::Context) {
    let registry = ctx.data_mut(|d| d.remove_temp::<IconRegistry>(handles_key()));
    let Some(registry) = registry else { return };
    for uri in &registry.uris {
        ctx.forget_image(uri);
    }
    ctx.data_mut(|d| {
        for uri in &registry.uris {
            d.remove::<egui::TextureHandle>(texture_id(uri));
        }
    });
}

/// egui's clock in whole milliseconds; `0` where it is not a duration (§9.4's arithmetic is on
/// `u128`).
fn now_ms(ctx: &egui::Context) -> u128 {
    let seconds = ctx.input(|i| i.time);
    Duration::try_from_secs_f64(seconds)
        .map(|d| d.as_millis())
        .unwrap_or_default()
}

/// The frame index to draw for a frame-animated icon, scheduling one wake-up at the next frame
/// boundary (§9.4). The caller hands that frame's `IconData`, `frames[i]`, to
/// [`to_image_source`] under the icon's one key, and the URI follows the frame's bytes (§9.2).
/// `None`, scheduling nothing, when the icon is not `AnimatedIcon::Frames` or
/// `reduced_motion` is `true` (pass `atlas.accessibility().reduce_motion` or
/// [`crate::is_reduced_motion`]) — draw `AnimatedIcon::first_frame()` then.
#[must_use]
pub fn animated_frame_index(
    ctx: &egui::Context,
    icon: &AnimatedIcon,
    reduced_motion: bool,
) -> Option<usize> {
    if reduced_motion {
        return None;
    }
    let frames = icon.frame_list()?;
    let per_frame = u128::from(icon.frame_duration_ms()?.get());
    let count = u128::try_from(frames.len()).ok()?;
    let cycle = count.checked_mul(per_frame)?;
    let position = now_ms(ctx).checked_rem(cycle)?;
    let index = position.checked_div(per_frame)?;
    let into_frame = position.checked_rem(per_frame)?;
    let until_next = per_frame.checked_sub(into_frame)?;
    ctx.request_repaint_after(Duration::from_millis(u64::try_from(until_next).ok()?));
    usize::try_from(index).ok()
}

/// The rotation angle in radians for a spin-animated icon, repainting every frame (§9.4);
/// `None`, scheduling nothing, when the icon is not `AnimatedIcon::Transform` with
/// `TransformAnimation::Spin`, or `reduced_motion` is `true`. Feed to
/// `Image::rotate(angle, Vec2::splat(0.5))` (`egui/src/widgets/image.rs:239-243`), which
/// forces `corner_radius = ZERO` (`:241`).
#[must_use]
pub fn spin_angle(ctx: &egui::Context, icon: &AnimatedIcon, reduced_motion: bool) -> Option<f32> {
    if reduced_motion {
        return None;
    }
    let duration_ms = match icon.animation()? {
        TransformAnimation::Spin { duration_ms, .. } => u128::from(duration_ms.get()),
        _ => return None,
    };
    let position = now_ms(ctx).checked_rem(duration_ms)?;
    ctx.request_repaint();
    // Both values are below 2^32, so the casts lose nothing an angle can show.
    Some(position as f32 / duration_ms as f32 * std::f32::consts::TAU)
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
    use std::borrow::Cow;

    use egui::load::{LoadError, SizeHint};
    use native_theme::AccessibilityPreferences;

    // `IconRole`, `IconSet`, `AnimatedIcon`, `TransformAnimation`, `ThemeAtlas` and `IconData`
    // come with the glob: `icons.rs` imports or re-exports each.
    use super::*;

    /// A `currentColor` icon that sets no `color` of its own (§9.2).
    const PLAIN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    /// Breeze's convention: a stylesheet sets `color` per class.
    const BREEZE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><style id="current-color-scheme">.ColorScheme-Text { color:#232629; }</style><path class="ColorScheme-Text" fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    /// A full-colour icon: no `currentColor`, no placeholder. Only the `material-icons` tests
    /// use it.
    #[cfg(feature = "material-icons")]
    const FULL_COLOUR: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#e01b24" d="M2 2h12v12H2z"/></svg>"##;

    fn svg(bytes: &'static [u8]) -> IconData {
        IconData::Svg(Cow::Borrowed(bytes))
    }

    fn rgba(side: u32) -> IconData {
        let n = usize::try_from(side).unwrap() * usize::try_from(side).unwrap() * 4;
        IconData::Rgba {
            width: side,
            height: side,
            data: vec![0; n],
        }
    }

    fn bytes_of(source: &egui::ImageSource<'_>) -> (String, Vec<u8>) {
        match source {
            egui::ImageSource::Bytes { uri, bytes } => (uri.to_string(), bytes.to_vec()),
            _ => panic!("expected ImageSource::Bytes"),
        }
    }

    /// adwaita's two variants, whose text colours differ (asserted). Only the
    /// `material-icons` test uses it, so it is compiled with that feature alone.
    #[cfg(feature = "material-icons")]
    fn two_variant_atlas() -> ThemeAtlas {
        use native_theme::theme::{ColorMode, Theme};
        let light = Theme::preset("adwaita")
            .unwrap()
            .resolve(ColorMode::Light)
            .unwrap()
            .variant;
        let dark = Theme::preset("adwaita")
            .unwrap()
            .resolve(ColorMode::Dark)
            .unwrap()
            .variant;
        assert_ne!(light.defaults.text_color, dark.defaults.text_color);
        ThemeAtlas::builder("adwaita", &light, &dark).build()
    }

    #[derive(Debug)]
    struct Provider {
        name: Option<&'static str>,
        svg: Option<&'static [u8]>,
    }

    impl IconProvider for Provider {
        fn icon_name(&self, _: IconSet) -> Option<&str> {
            self.name
        }
        fn icon_svg(&self, _: IconSet) -> Option<Cow<'static, [u8]>> {
            self.svg.map(Cow::Borrowed)
        }
    }

    /// T9: the URI this crate produces is one the installed SVG loader accepts. The check goes
    /// through `Context::try_load_image`, which swallows each loader's `NotSupported` and falls
    /// through to `NoMatchingImageLoader` (`egui/src/context.rs:3873-3885`); `include_bytes` is
    /// what `ImageSource::Bytes` does when an `Image` loads it (`egui/src/widgets/image.rs:642`).
    #[test]
    fn a_produced_svg_uri_is_one_the_svg_loader_accepts() {
        let ctx = egui::Context::default();
        egui_extras::install_image_loaders(&ctx);
        let key = IconKey::role(IconRole::DialogWarning, IconSet::Lucide);
        let (uri, bytes) = bytes_of(&to_image_source(&ctx, &key, &svg(PLAIN)).unwrap());
        assert!(
            uri.starts_with("bytes://native-theme/") && uri.ends_with(".svg"),
            "{uri}"
        );
        assert!(egui::load::has_extension(&uri, "svg"));
        assert!(!uri.contains('#'));
        ctx.include_bytes(uri.clone(), bytes);
        let result = ctx.try_load_image(&uri, SizeHint::Width(16));
        assert!(
            !matches!(result, Err(LoadError::NoMatchingImageLoader { .. })),
            "no loader accepted {uri}"
        );
        assert!(!matches!(result, Err(LoadError::NoImageLoaders)));
        // A raster icon's URI has no extension: it names a texture, and no loader reads it.
        // `super::uri`: the local `uri` above shadows the function.
        assert!(!super::uri(&rgba(2)).contains('.'));
    }

    /// T9, per-scheme icon theme (`native-theme/src/presets/kde-breeze.toml:9`, `:317`).
    #[test]
    fn the_icon_theme_is_per_scheme() {
        for is_dark in [false, true] {
            let (atlas, _) =
                crate::from_preset("kde-breeze", is_dark, &AccessibilityPreferences::default())
                    .unwrap();
            assert_eq!(atlas.icon_theme(egui::Theme::Light), Some("breeze"));
            assert_eq!(atlas.icon_theme(egui::Theme::Dark), Some("breeze-dark"));
        }
    }

    /// T9, tint: baked into a bundled `currentColor` icon's bytes; absent, the bytes are the payload's.
    #[cfg(feature = "lucide-icons")]
    #[test]
    fn a_tint_is_baked_into_a_bundled_icons_bytes() {
        let ctx = egui::Context::default();
        let icon = native_theme::icons::LucideLoader::new(IconRole::DialogWarning)
            .load()
            .unwrap();
        assert!(
            std::str::from_utf8(icon.bytes())
                .unwrap()
                .contains("currentColor")
        );
        let tinted = IconKey::role(IconRole::DialogWarning, IconSet::Lucide)
            .tint(egui::Color32::from_rgb(0x12, 0x34, 0x56));
        let (tinted_uri, bytes) = bytes_of(&to_image_source(&ctx, &tinted, &icon).unwrap());
        let text = String::from_utf8(bytes).unwrap();
        assert!(
            text.contains("#123456") && !text.contains("currentColor"),
            "{text}"
        );
        let plain = IconKey::role(IconRole::DialogWarning, IconSet::Lucide);
        let (plain_uri, bytes) = bytes_of(&to_image_source(&ctx, &plain, &icon).unwrap());
        assert_eq!(bytes, icon.bytes());
        assert_ne!(tinted_uri, plain_uri);
    }

    /// T9, the URI follows the bytes: frames, the load colour per scheme, a full-colour icon,
    /// a Breeze-style icon, a provider's differing bytes.
    #[cfg(feature = "material-icons")]
    #[test]
    fn the_uri_follows_the_bytes() {
        let ctx = egui::Context::default();

        // Two frames of the Material indicator, `material_spinner` (`native-theme/src/spinners.rs:159-162`),
        // an `AnimatedIcon::Frames` (`material_spinner_is_frames`, `:212-221`), under one key.
        let indicator = native_theme::icons::load_icon_indicator(IconSet::Material).unwrap();
        let frames = indicator
            .frame_list()
            .expect("the Material indicator is frame-animated");
        let key = IconKey::role(IconRole::DialogInfo, IconSet::Material);
        let (frame_0, _) = bytes_of(&to_image_source(&ctx, &key, frames.first()).unwrap());
        let (frame_1, _) = bytes_of(&to_image_source(&ctx, &key, frames.get(1).unwrap()).unwrap());
        assert_ne!(frame_0, frame_1);

        // The same freedesktop key under both schemes of a two-variant atlas.
        let atlas = two_variant_atlas();
        atlas.install(&ctx);
        let key = IconKey::name("edit-copy", IconSet::Freedesktop);
        ctx.set_theme(egui::Theme::Light);
        let (light_uri, light_bytes) = bytes_of(&to_image_source(&ctx, &key, &svg(PLAIN)).unwrap());
        ctx.set_theme(egui::Theme::Dark);
        let (dark_uri, dark_bytes) = bytes_of(&to_image_source(&ctx, &key, &svg(PLAIN)).unwrap());
        assert_ne!(light_uri, dark_uri);
        for (theme, bytes) in [
            (egui::Theme::Light, light_bytes),
            (egui::Theme::Dark, dark_bytes),
        ] {
            let hex = hex_rgb(atlas.resolved_for(theme).defaults.text_color);
            let text = String::from_utf8(bytes).unwrap();
            assert!(
                text.contains(&hex) && !text.contains("currentColor"),
                "{theme:?}: {text}"
            );
        }

        // A full-colour icon: one URI, bytes untouched.
        ctx.set_theme(egui::Theme::Light);
        let (full_light, bytes) =
            bytes_of(&to_image_source(&ctx, &key, &svg(FULL_COLOUR)).unwrap());
        assert_eq!(bytes, FULL_COLOUR);
        ctx.set_theme(egui::Theme::Dark);
        let (full_dark, _) = bytes_of(&to_image_source(&ctx, &key, &svg(FULL_COLOUR)).unwrap());
        assert_eq!(full_light, full_dark);

        // A Breeze-style icon whose stylesheet sets `color`: handed on unchanged, one URI.
        let (breeze_dark, bytes) = bytes_of(&to_image_source(&ctx, &key, &svg(BREEZE)).unwrap());
        assert_eq!(bytes, BREEZE);
        ctx.set_theme(egui::Theme::Light);
        let (breeze_light, _) = bytes_of(&to_image_source(&ctx, &key, &svg(BREEZE)).unwrap());
        assert_eq!(breeze_light, breeze_dark);
        assert_eq!(breeze_light, uri(&svg(BREEZE)));

        // A provider returning different bytes for the same name yields different URIs.
        let key = IconKey::name("brand-mark", IconSet::Material);
        let first = Provider {
            name: None,
            svg: Some(FULL_COLOUR),
        };
        let second = Provider {
            name: None,
            svg: Some(PLAIN),
        };
        let (u1, _) = bytes_of(&custom_icon_to_image_source(&ctx, &first, &key).unwrap());
        let (u2, _) = bytes_of(&custom_icon_to_image_source(&ctx, &second, &key).unwrap());
        assert_ne!(u1, u2);
    }

    /// T9, provider: the provider's bytes are what the URI hashes.
    #[cfg(feature = "material-icons")]
    #[test]
    fn a_providers_bytes_are_what_the_uri_hashes() {
        let ctx = egui::Context::default();
        let provider = Provider {
            name: None,
            svg: Some(FULL_COLOUR),
        };
        let key = IconKey::name("brand-mark", IconSet::Material);
        let (u, bytes) = bytes_of(&custom_icon_to_image_source(&ctx, &provider, &key).unwrap());
        assert_eq!(u, uri(&svg(FULL_COLOUR)));
        assert_eq!(bytes, FULL_COLOUR);
    }

    /// §9.2's predicate, on the shapes it names.
    #[test]
    fn sets_color_reads_only_the_color_property() {
        assert!(!sets_color(std::str::from_utf8(PLAIN).unwrap()));
        assert!(sets_color(std::str::from_utf8(BREEZE).unwrap()));
        assert!(sets_color(
            r##"<svg color="#fff"><path fill="currentColor"/></svg>"##
        ));
        assert!(sets_color("<svg><style>.a { color : #fff }</style></svg>"));
        assert!(!sets_color(
            r##"<svg><stop stop-color="#fff"/><path fill="currentColor"/></svg>"##
        ));
        assert!(!sets_color(
            r##"<svg flood-color="#fff" lighting-color="#fff" solid-color="#fff"/>"##
        ));
        assert!(!sets_color(
            r##"<svg text-decoration-color="#fff" pagecolor="#fff" bordercolor="#fff"/>"##
        ));
        assert!(!sets_color(
            r#"<svg id="color" class="color"><path fill="currentColor"/></svg>"#
        ));
        assert!(!sets_color(r#"<svg color-interpolation="sRGB"/>"#));
    }

    /// T9, converters: `None` wherever epaint would assert (§9.3).
    #[test]
    fn to_color_image_refuses_what_epaint_would_assert_on() {
        assert!(
            to_color_image(&IconData::Rgba {
                width: 0,
                height: 4,
                data: vec![]
            })
            .is_none()
        );
        assert!(
            to_color_image(&IconData::Rgba {
                width: 4,
                height: 0,
                data: vec![]
            })
            .is_none()
        );
        assert!(
            to_color_image(&IconData::Rgba {
                width: u32::MAX,
                height: u32::MAX,
                data: vec![]
            })
            .is_none()
        );
        assert!(
            to_color_image(&IconData::Rgba {
                width: 2,
                height: 2,
                data: vec![0; 15]
            })
            .is_none()
        );
        assert!(to_color_image(&svg(PLAIN)).is_none());
        assert!(to_color_image(&rgba(2)).is_some());
    }

    /// §13 T8 (c) — §9.3: a side above `max_texture_side` (2048 on a bare `Context`,
    /// `egui/src/input_state/mod.rs:268`, `:349`) is a deliberate `None`, from `to_image_source`
    /// and `to_image` alike, not `load_texture`'s `debug_assert!` (`egui/src/context.rs:2399-2405`).
    #[test]
    fn t8c_an_oversized_raster_icon_is_none() {
        let ctx = egui::Context::default();
        let max = ctx.input(|i| i.max_texture_side);
        let side = u32::try_from(max).unwrap() + 1;
        let icon = IconData::Rgba {
            width: side,
            height: 1,
            data: vec![0; usize::try_from(side).unwrap() * 4],
        };
        let key = IconKey::name("raster", IconSet::Freedesktop);
        assert!(to_image_source(&ctx, &key, &icon).is_none());
        assert!(to_image(&ctx, &key, &icon).is_none());
    }

    /// T9, animation: `None` under reduced motion and for the other variant, else in range.
    #[cfg(feature = "material-icons")]
    #[test]
    fn animation_helpers_answer_none_under_reduced_motion_and_for_the_other_variant() {
        // No pass is needed: `ctx.input(|i| i.time)` reads the bare `Context`'s input state.
        let ctx = egui::Context::default();
        let frames = native_theme::icons::load_icon_indicator(IconSet::Material).unwrap();
        let spin = AnimatedIcon::transform(
            svg(PLAIN),
            TransformAnimation::Spin {
                duration_ms: std::num::NonZeroU32::new(800).unwrap(),
            },
        );
        assert!(animated_frame_index(&ctx, &frames, true).is_none());
        assert!(animated_frame_index(&ctx, &spin, false).is_none());
        let index = animated_frame_index(&ctx, &frames, false).unwrap();
        assert!(index < frames.frame_list().unwrap().len());
        assert!(spin_angle(&ctx, &spin, true).is_none());
        assert!(spin_angle(&ctx, &frames, false).is_none());
        assert!(spin_angle(&ctx, &spin, false).unwrap().is_finite());
    }

    /// T9's `forget_icons` clause: the icons' textures and SVG bytes go, the application's stay.
    #[test]
    fn forget_icons_frees_only_this_crates_textures() {
        let ctx = egui::Context::default();
        egui_extras::install_image_loaders(&ctx);
        let before = ctx.tex_manager().read().num_allocated();
        let app_texture = ctx.load_texture(
            "app-own",
            egui::ColorImage::example(),
            egui::TextureOptions::LINEAR,
        );
        ctx.include_bytes("bytes://app/own.svg", PLAIN);
        let key = IconKey::name("raster", IconSet::Freedesktop);
        let source = to_image_source(&ctx, &key, &rgba(4)).unwrap();
        assert!(matches!(source, egui::ImageSource::Texture(_)));
        assert_eq!(ctx.tex_manager().read().num_allocated(), before + 2);
        // A second call reuses the stored handle: no second upload.
        let _again = to_image_source(&ctx, &key, &rgba(4)).unwrap();
        assert_eq!(ctx.tex_manager().read().num_allocated(), before + 2);
        // An SVG icon's bytes reach the loaders as an `Image` hands them over
        // (`egui/src/widgets/image.rs:642`).
        let svg_key = IconKey::role(IconRole::DialogWarning, IconSet::Lucide);
        let (svg_uri, svg_bytes) = bytes_of(&to_image_source(&ctx, &svg_key, &svg(PLAIN)).unwrap());
        ctx.include_bytes(svg_uri.clone(), svg_bytes);
        assert!(ctx.try_load_bytes(&svg_uri).is_ok());
        forget_icons(&ctx);
        assert!(
            ctx.try_load_bytes(&svg_uri).is_err(),
            "the icon's SVG bytes went"
        );
        assert_eq!(
            ctx.tex_manager().read().num_allocated(),
            before + 1,
            "the icon went, the app's stayed"
        );
        assert!(
            ctx.try_load_bytes("bytes://app/own.svg").is_ok(),
            "the app's bytes stayed"
        );
        assert!(ctx.data(|d| d.get_temp::<IconRegistry>(handles_key()).is_none()));
        drop(app_texture);
        assert_eq!(ctx.tex_manager().read().num_allocated(), before);
    }

    /// An icon no theme holds is `None`; nothing is substituted.
    #[test]
    fn an_icon_no_theme_holds_is_none_and_nothing_is_substituted() {
        let ctx = egui::Context::default();
        let provider = Provider {
            name: Some("native-theme-egui-no-such-icon"),
            svg: None,
        };
        let unknown_theme = IconKey::name("native-theme-egui-no-such-icon", IconSet::Freedesktop)
            .icon_theme("native-theme-egui-no-such-theme");
        assert!(custom_icon_to_image_source(&ctx, &provider, &unknown_theme).is_none());
        let system_theme = IconKey::name("native-theme-egui-no-such-icon", IconSet::Freedesktop);
        assert!(custom_icon_to_image_source(&ctx, &provider, &system_theme).is_none());
        let bundled = IconKey::name("native-theme-egui-no-such-icon", IconSet::Material);
        assert!(custom_icon_to_image_source(&ctx, &provider, &bundled).is_none());
    }

    /// `to_image` carries the alt text: the role's name, or the key's name.
    #[test]
    fn to_image_names_the_icon() {
        let ctx = egui::Context::default();
        let by_role = IconKey::role(IconRole::DialogWarning, IconSet::Lucide);
        assert_eq!(by_role.alt_text(), IconRole::DialogWarning.name());
        let by_name = IconKey::name("brand-mark", IconSet::Material);
        assert_eq!(by_name.alt_text(), "brand-mark");
        assert!(to_image(&ctx, &by_name, &svg(PLAIN)).is_some());
    }

    /// `IconKey::size` is total over every `f32` (§4.10).
    #[test]
    fn a_size_folds_every_float_to_one_u16() {
        for bad in [f32::NAN, -0.0, -3.0, f32::NEG_INFINITY] {
            assert_eq!(
                IconKey::name("x", IconSet::Material).size(bad).size,
                Some(0)
            );
        }
        assert_eq!(
            IconKey::name("x", IconSet::Material).size(23.6).size,
            Some(24)
        );
        assert_eq!(
            IconKey::name("x", IconSet::Material)
                .size(f32::INFINITY)
                .size,
            Some(u16::MAX)
        );
        assert_eq!(IconKey::name("x", IconSet::Material).size, None);
    }
}
