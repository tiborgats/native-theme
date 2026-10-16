//! native-theme icon payloads → egui image sources.

/// Re-exported **here and not at the crate root**, because `egui::IconData`
/// (`egui/src/viewport.rs:183`, the window/taskbar icon) already occupies that name and this
/// crate re-exports `egui`.
pub use native_theme::icons::IconData;

use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap};
use std::hash::{DefaultHasher, Hasher};
use std::sync::Arc;
use std::time::Duration;

use native_theme::color::Rgba;
use native_theme::icons::{AnimatedIcon, IconProvider, IconRole, TransformAnimation};
use native_theme::theme::IconSet;

use crate::{ResolvedTheme, ThemeAtlas};

/// Which of native-theme's five per-context icon sizes to use
/// (`ResolvedIconSizes`, `native-theme/src/model/resolved.rs:15-26`). egui has no icon-size
/// vocabulary of its own (spec §9.4).
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
/// (spec §9.2), and the role or name gives [`to_image`] its alt text. Constructed through the builder below: private
/// fields, because a
/// `#[non_exhaustive]` struct with no constructor cannot be built outside its defining crate
/// at all (`E0639`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IconKey {
    id: KeyId,
    set: IconSet,
    icon_theme: Option<String>,
    size: Option<u32>,
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
    /// ([`crate::ThemeAtlas::icon_theme`], spec §9.2). [`custom_icon_to_image_source`] looks a
    /// freedesktop icon up in it, and without it in the system's icon theme.
    #[must_use]
    pub fn icon_theme(mut self, icon_theme: &str) -> Self {
        self.icon_theme = Some(icon_theme.to_owned());
        self
    }

    /// The requested edge length in logical pixels, which [`custom_icon_to_image_source`]
    /// hands every system set's loader as that loader takes it: the pixel size a freedesktop
    /// icon is looked up at
    /// ([`FreedesktopLoader::size`](native_theme::icons::FreedesktopLoader::size), which picks
    /// the theme's directory nearest it), an SF Symbol's point size
    /// ([`SfSymbolsLoader::size`](native_theme::icons::SfSymbolsLoader::size)) and a Segoe
    /// glyph's pixel size
    /// ([`SegoeFluentLoader::size`](native_theme::icons::SegoeFluentLoader::size)). A size of 0
    /// is asked for as 1, as those two load nothing at 0. A size those two do not accept
    /// above that, one whose `size² × 4` raster bytes exceed their 64 MiB decode bound, gives
    /// `None`, and so does a raster with a side above `max_texture_side`: egui refuses an
    /// oversized raster rather than resampling it (spec §9.3). A Material or Lucide icon is
    /// scalable and loads the same at every size. Stored as a private `u32` — the type the
    /// three loaders take — computed as `(if points > 0.0 { points.round() } else { 0.0 }) as
    /// u32`, which is total over every `f32`: `NaN`, `-0.0` and every negative fold to the
    /// single key `0`, and the float-to-int `as` cast saturates at `u32::MAX` (spec §7.1). An
    /// integer, not an `f32`, is what lets `IconKey` derive `Eq` and `Hash`. Without this call
    /// the key has no size, and each loader takes its own default: 24 pixels, 24 points and 32
    /// pixels.
    #[must_use]
    pub fn size(mut self, points: f32) -> Self {
        let rounded = if points > 0.0 { points.round() } else { 0.0 };
        // The float-to-int `as` cast saturates and folds NaN to 0 (§7.1).
        self.size = Some(rounded as u32);
        self
    }

    /// The colour a monochrome icon is drawn in. [`to_image_source`] bakes it into the
    /// `IconData::Svg` bytes an application loaded — through
    /// `native_theme::icons::colorize_monochrome_svg` for a bundled set, through
    /// `native_theme::icons::fill_current_color` for a freedesktop icon — and
    /// [`custom_icon_to_image_source`] loads the icon in it, so the pixels, and through their
    /// hash the URI, carry it (spec §9.2). Not `egui::Image::tint`, a draw-time multiply that
    /// leaves black black. Unset, a bundled set's bytes are used as they are, which is right
    /// for a full-colour icon, and a freedesktop icon, an SF Symbol and a Segoe glyph take the
    /// atlas's text colour. A freedesktop icon whose own stylesheet sets its `color`, as
    /// nearly every Breeze `currentColor` icon does, keeps it, tint or not, and a full-colour
    /// one its palette; the tint colours bundled monochrome sets, the freedesktop icons that
    /// set none and a system set's glyphs (spec §9.2).
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

/// The 64-bit hash of the final bytes, their length and, for a raster icon, its size, as
/// `bytes://native-theme/<16 hex digits>`, with `.svg` for an SVG (spec §9.2). `DefaultHasher::new`
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
/// `.svg` for `IconData::Svg` (spec §9.2). Pass the final bytes; [`to_image_source`] calls it with
/// the bytes after colouring.
// An `IconData` variant this crate does not know hashes its bytes with no extension
// (`to_image_source` returns `None` for it before any URI is used).
#[must_use]
pub fn uri(icon: &IconData) -> String {
    match icon {
        IconData::Svg(bytes) => hashed_uri(bytes, None, true),
        IconData::Rgba(rgba) => hashed_uri(rgba.data(), Some((rgba.width(), rgba.height())), false),
        _ => hashed_uri(icon.bytes(), None, false),
    }
}

/// The colour a freedesktop icon, or a system set's monochrome glyph, is loaded in: the key's
/// tint, else `defaults.text_color` of the installed atlas's theme for `ctx.theme()`; `None`
/// with neither (spec §9.2). Two `Context` accessors, one after the other, never nested (spec §10.3).
fn load_color(ctx: &egui::Context, key: &IconKey) -> Option<Rgba> {
    if let Some(tint) = key.tint_rgba() {
        return Some(tint);
    }
    let atlas = ThemeAtlas::from_ctx(ctx)?;
    let theme = ctx.theme();
    Some(atlas.resolved_for(theme).defaults.text_color)
}

/// The final bytes of an SVG the application loaded (spec §9.2): a freedesktop icon's
/// `currentColor` filled in with [`load_color`]'s colour where the document sets no `color` of
/// its own (`native_theme::icons::fill_current_color`); another set's coloured with the key's
/// tint through `native_theme::icons::colorize_monochrome_svg`; the bytes as they are without
/// a colour.
fn final_svg_bytes(ctx: &egui::Context, key: &IconKey, bytes: &[u8]) -> Vec<u8> {
    match key.set {
        IconSet::Freedesktop => match load_color(ctx, key) {
            Some(color) => native_theme::icons::fill_current_color(bytes, color).into_owned(),
            None => bytes.to_vec(),
        },
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

/// A named icon [`custom_icon_to_image_source`] loaded, with what its set's loader was given.
#[derive(Clone)]
struct LoadedIcon {
    set: IconSet,
    icon_theme: Option<String>,
    size: Option<u32>,
    color: Option<Rgba>,
    icon: Arc<IconData>,
}

/// The named icons [`custom_icon_to_image_source`] loaded, by the provider's name; stored
/// under `loaded_key()`, dropped by `forget_icons`. It grows by one entry per distinct name,
/// set, icon theme, size and colour until then, as the URI registry does: a colour that
/// changes every frame adds an entry every frame.
#[derive(Clone, Default)]
pub(crate) struct LoadedIcons {
    by_name: HashMap<String, Vec<LoadedIcon>>,
}

impl LoadedIcons {
    /// The icon `name` loaded from `set` in this icon theme, at this size, in this colour.
    fn get(
        &self,
        name: &str,
        set: IconSet,
        icon_theme: Option<&str>,
        size: Option<u32>,
        color: Option<Rgba>,
    ) -> Option<Arc<IconData>> {
        self.by_name
            .get(name)?
            .iter()
            .find(|kept| {
                kept.set == set
                    && kept.icon_theme.as_deref() == icon_theme
                    && kept.size == size
                    && kept.color == color
            })
            .map(|kept| Arc::clone(&kept.icon))
    }
}

/// A provider of one name and no SVG of its own: the name's load alone.
#[derive(Debug)]
struct NameOnly<'a>(&'a str);

impl IconProvider for NameOnly<'_> {
    fn icon_name(&self, _: IconSet) -> Option<&str> {
        Some(self.0)
    }

    fn icon_svg(&self, _: IconSet) -> Option<Cow<'static, [u8]>> {
        None
    }
}

/// A provider's own SVG alone, without its name.
#[derive(Debug)]
struct SvgOnly<'a>(&'a dyn IconProvider);

impl IconProvider for SvgOnly<'_> {
    fn icon_name(&self, _: IconSet) -> Option<&str> {
        None
    }

    fn icon_svg(&self, set: IconSet) -> Option<Cow<'static, [u8]>> {
        self.0.icon_svg(set)
    }
}

/// The `Context` data key of the loaded icons, `Id::new("native-theme-egui/loaded-icons")`.
pub(crate) fn loaded_key() -> egui::Id {
    egui::Id::new("native-theme-egui/loaded-icons")
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
/// side, an overflowing `width * height * 4` or a buffer of another length (spec §9.3), and for
/// an `IconData` variant this crate does not know (`IconData` is `#[non_exhaustive]`,
/// `native-theme/src/model/icons.rs:292`), and for an `IconData::Svg`, which this crate never
/// decodes (spec §9.1). The
/// oversize test needs a `Context` and lives in [`to_image_source`] and [`to_image`].
#[must_use]
pub fn to_color_image(icon: &IconData) -> Option<egui::ColorImage> {
    let IconData::Rgba(rgba) = icon else {
        return None;
    };
    let data = rgba.data();
    let w = usize::try_from(rgba.width()).ok()?;
    let h = usize::try_from(rgba.height()).ok()?;
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
/// `IconSet::Freedesktop` key, a `currentColor` the document sets no `color` for filled in
/// with the `#rrggbb` of the tint, else of `defaults.text_color` of the installed atlas's
/// `ResolvedTheme` for `ctx.theme()` (`native_theme::icons::fill_current_color`; the bytes
/// as they are where no atlas is installed); a full-colour icon, and one whose stylesheet
/// sets its `color`, untouched (spec §9.2). A freedesktop icon the application loads itself is
/// best loaded with `FreedesktopLoader::color` in the same colour, so a symbolic icon's
/// placeholders take it too (spec §9.2); [`custom_icon_to_image_source`] loads a provider's icon
/// that way and passes it through no colouring here. `IconData::Rgba` →
/// `ImageSource::Texture`, uploaded once via `Context::load_texture`
/// (`egui/src/context.rs:2390`) under the same URI and reused afterwards. `None` for an
/// image with a side above `max_texture_side` — rejected, never resampled (spec §9.3) — and for
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
/// second one, because `load_texture` itself takes the `Context` lock (spec §10.3).
///
/// **The texture is freed at the next install of another atlas**, and by
/// [`crate::ThemeAtlas::clear`]: both call [`forget_icons`], which removes the stored handle, so
/// a kept `ImageSource::Texture` names a freed texture afterwards, and a kept
/// `ImageSource::Bytes` keeps the previous colour. Call this, or [`to_image`], in each pass
/// rather than keeping its result: the stored handle is reused, so a pass uploads nothing new.
#[must_use]
pub fn to_image_source(
    ctx: &egui::Context,
    key: &IconKey,
    icon: &IconData,
) -> Option<egui::ImageSource<'static>> {
    match icon {
        IconData::Svg(bytes) => Some(svg_source(ctx, final_svg_bytes(ctx, key, bytes))),
        _ => raster_source(ctx, icon),
    }
}

/// `ImageSource::Bytes` of these final SVG bytes under their `.svg` URI, registered for
/// [`forget_icons`].
fn svg_source(ctx: &egui::Context, bytes: Vec<u8>) -> egui::ImageSource<'static> {
    let uri = hashed_uri(&bytes, None, true);
    remember_uri(ctx, &uri);
    egui::ImageSource::Bytes {
        uri: Cow::Owned(uri),
        bytes: egui::load::Bytes::from(bytes),
    }
}

/// `ImageSource::Texture` of an `IconData::Rgba`, uploaded once under its URI and registered
/// for [`forget_icons`] ([`to_image_source`]'s contract); `None` for a side above
/// `max_texture_side` and for every other variant.
fn raster_source(ctx: &egui::Context, icon: &IconData) -> Option<egui::ImageSource<'static>> {
    let IconData::Rgba(rgba) = icon else {
        return None;
    };
    let max = ctx.input(|i| i.max_texture_side);
    let too_big = |side: u32| usize::try_from(side).map_or(true, |s| s > max);
    if too_big(rgba.width()) || too_big(rgba.height()) {
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

/// Same, for an [`IconProvider`]: an application-supplied one, or an [`IconRole`], which is
/// one (`native-theme/src/model/icons.rs:547`), so a role's icon from the installed theme's
/// set and icon theme at its size is one call ([`custom_icon_to_image`] gives it as an
/// `egui::Image` with its alt text). Loads the key's set through
/// [`native_theme::icons::load_custom_icon_in`], which tries `provider.icon_name(set)` through
/// the set's loader, then the provider's own `icon_svg(set)`, and no other set: a freedesktop
/// icon from the key's icon theme ([`IconKey::icon_theme`]; the system's without one), and
/// every system set at the key's size ([`IconKey::size`]). A freedesktop icon, an SF Symbol
/// and a Segoe glyph are loaded in the colour [`to_image_source`] gives a freedesktop icon:
/// the tint, else `defaults.text_color` of the installed atlas's theme for `ctx.theme()`. A
/// freedesktop icon takes it where its foreground is left to the caller (a symbolic icon's
/// placeholders, a `currentColor` the document sets no `color` for); one whose own
/// stylesheet sets its colour (Breeze's `current-color-scheme`) and a full-colour one keep
/// theirs. The provider's own SVG for a system set, freedesktop included, is coloured in it
/// too, as a monochrome icon (`native_theme::icons::colorize_monochrome_svg`, an implicit or
/// black fill as well as a `currentColor`), on every platform (an SF Symbol's on Linux and
/// Windows as well), so give a system set a monochrome SVG. A size a system set's loader
/// does not accept, and a raster with a side above `max_texture_side`, give `None`, never a
/// resampled raster (spec §9.3; [`IconKey::size`]). A Material or Lucide icon, and the
/// provider's own SVG for one, take the tint alone and are untouched without one
/// ([`IconKey::tint`]). The loaded bytes are keyed as [`to_image_source`] keys an icon, with
/// no second colouring pass, so the provider's bytes are what the URI's hash covers. `None`
/// where the provider has none. For an `IconRole`, build the key with
/// `IconKey::role(role, set)`; for another provider, with `IconKey::name(n, set)`, `n` its
/// `icon_name(set)` where it has one; where it has none, a name that describes it, which
/// becomes its alt text — the URI is the hash of the bytes alone (spec §9.2).
///
/// **What a call costs.** The icon a provider's name loads is loaded once per name, set,
/// icon theme, size and colour — for a freedesktop icon a lookup in the icon theme and a read
/// of its file from disk (native-theme's `freedesktop.rs` performance note), for an SF Symbol
/// or a Segoe glyph a raster the OS draws — and kept in `ctx`'s data until [`forget_icons`],
/// which the next install of another atlas and [`crate::ThemeAtlas::clear`] call; an icon
/// file changed on disk is seen after that. The icon theme is the key's, else the system's as
/// native-theme's detection cache names it (`native_theme::detect::system_icon_theme`), so
/// after `native_theme::detect::invalidate_caches` a changed system icon theme is loaded
/// from. A later call copies, hashes and registers the kept bytes; another colour — another
/// tint, or the other scheme's text colour — loads the icon once more, and each distinct
/// colour stays kept until [`forget_icons`], as each URI does: a colour that changes every
/// frame keeps one more icon every frame. A name the set's loader does not find is looked up
/// again on every call, and the provider's own SVG is coloured again on every call.
#[must_use]
pub fn custom_icon_to_image_source(
    ctx: &egui::Context,
    provider: &dyn IconProvider,
    key: &IconKey,
) -> Option<egui::ImageSource<'static>> {
    let color = match key.set {
        IconSet::Freedesktop | IconSet::SfSymbols | IconSet::SegoeFluent => load_color(ctx, key),
        _ => key.tint_rgba(),
    };
    // At 0 an SF Symbol or a Segoe glyph loads nothing (`SfSymbolsLoader::size`,
    // `SegoeFluentLoader::size`).
    let size = key.size.map(|size| size.max(1));
    // The icon theme a freedesktop load searches: the key's, else the system's from
    // native-theme's detection cache (`detect::system_icon_theme`, what `FreedesktopLoader`
    // reads without one), so a kept icon is keyed by the theme it came from and a system
    // theme detected anew after `detect::invalidate_caches` is loaded from, not the kept one.
    let resolved_theme = match (&key.icon_theme, key.set) {
        (None, IconSet::Freedesktop) => native_theme::detect::system_icon_theme().ok(),
        _ => None,
    };
    let icon_theme = key.icon_theme.as_deref().or(resolved_theme.as_deref());
    let load = |provider: &dyn IconProvider| {
        native_theme::icons::load_custom_icon_in(provider, key.set, icon_theme, color, size)
    };
    let named = provider.icon_name(key.set).and_then(|name| {
        let kept = ctx.data_mut(|d| {
            d.get_temp_mut_or_default::<LoadedIcons>(loaded_key())
                .get(name, key.set, icon_theme, size, color)
        });
        kept.or_else(|| {
            let icon = Arc::new(load(&NameOnly(name))?);
            let loaded = LoadedIcon {
                set: key.set,
                icon_theme: icon_theme.map(str::to_owned),
                size,
                color,
                icon: Arc::clone(&icon),
            };
            ctx.data_mut(|d| {
                d.get_temp_mut_or_default::<LoadedIcons>(loaded_key())
                    .by_name
                    .entry(name.to_owned())
                    .or_default()
                    .push(loaded);
            });
            Some(icon)
        })
    });
    let own;
    let icon: &IconData = match &named {
        Some(icon) => icon,
        None => {
            own = load(&SvgOnly(provider))?;
            &own
        }
    };
    // Loaded in its colour: keyed as it is, with no second colouring pass.
    match icon {
        IconData::Svg(bytes) => Some(svg_source(ctx, bytes.to_vec())),
        icon => raster_source(ctx, icon),
    }
}

/// [`custom_icon_to_image_source`], wrapped in an `egui::Image` with `alt_text` from the key,
/// as [`to_image`] wraps [`to_image_source`]: the role's `IconRole::name()` or the key's name.
/// `None` where [`custom_icon_to_image_source`] gives `None`.
///
/// A role's icon as the installed theme states it, its set, its icon theme for the scheme
/// egui draws and a size from the theme, drawn in a tint:
///
/// ```no_run
/// use native_theme_egui::icons::{IconContext, IconKey, custom_icon_to_image, icon_size};
/// use native_theme_egui::{IconRole, ThemeAtlas};
///
/// fn warning_icon(ui: &mut egui::Ui) {
///     let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) else {
///         return;
///     };
///     let scheme = ui.ctx().theme();
///     let role = IconRole::DialogWarning;
///     let side = icon_size(atlas.resolved_for(scheme), IconContext::Toolbar);
///     let mut key = IconKey::role(role, atlas.icon_set())
///         .size(side)
///         .tint(ui.visuals().warn_fg_color);
///     if let Some(name) = atlas.icon_theme(scheme) {
///         key = key.icon_theme(name);
///     }
///     if let Some(image) = custom_icon_to_image(ui.ctx(), &role, &key) {
///         ui.add(image.fit_to_exact_size(egui::Vec2::splat(side)));
///     }
/// }
/// ```
#[must_use]
pub fn custom_icon_to_image(
    ctx: &egui::Context,
    provider: &dyn IconProvider,
    key: &IconKey,
) -> Option<egui::Image<'static>> {
    let source = custom_icon_to_image_source(ctx, provider, key)?;
    Some(egui::Image::new(source).alt_text(key.alt_text()))
}

/// Drop every icon this crate cached in `ctx`, leaving unrelated application images alone;
/// [`crate::ThemeAtlas::install`] of an atlas other than the installed one calls it, and
/// [`crate::ThemeAtlas::clear`]. Two stores, both required: the `bytes://` URIs
/// are released with `Context::forget_image` (`egui/src/context.rs:3764`), which reaches only
/// the loader caches (`:3771-3780`), and the stored `TextureHandle`s are removed from
/// `ctx.data_mut()`, which is what frees their textures — `forget_image` never reaches a
/// handle this crate stored. The icons [`custom_icon_to_image_source`] loaded go too, so its
/// next call for each loads it again.
pub fn forget_icons(ctx: &egui::Context) {
    let registry = ctx.data_mut(|d| {
        d.remove::<LoadedIcons>(loaded_key());
        d.remove_temp::<IconRegistry>(handles_key())
    });
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

/// egui's clock in whole milliseconds; `0` where it is not a duration (spec §9.4's arithmetic is on
/// `u128`).
fn now_ms(ctx: &egui::Context) -> u128 {
    let seconds = ctx.input(|i| i.time);
    Duration::try_from_secs_f64(seconds)
        .map(|d| d.as_millis())
        .unwrap_or_default()
}

/// The frame index to draw for a frame-animated icon, scheduling one wake-up at the next frame
/// boundary (spec §9.4). The caller hands that frame's `IconData`, `frames[i]`, to
/// [`to_image_source`] under the icon's one key, and the URI follows the frame's bytes (spec §9.2).
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

/// The rotation angle in radians for a spin-animated icon, repainting every frame (spec §9.4);
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
    /// Breeze's convention: a stylesheet sets `color` per class. Only the `material-icons`
    /// tests use it.
    #[cfg(feature = "material-icons")]
    const BREEZE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><style id="current-color-scheme">.ColorScheme-Text { color:#232629; }</style><path class="ColorScheme-Text" fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
    /// A full-colour icon: no `currentColor`, no placeholder. Only the `material-icons` tests
    /// use it.
    #[cfg(feature = "material-icons")]
    const FULL_COLOUR: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#e01b24" d="M2 2h12v12H2z"/></svg>"##;

    /// `#rrggbb` of a colour, as `native_theme::icons::fill_current_color` writes it (the alpha
    /// is discarded). Only the `material-icons` tests and the freedesktop fixture tests use it.
    #[cfg(any(
        feature = "material-icons",
        all(target_os = "linux", feature = "system-icons")
    ))]
    fn hex_rgb(c: Rgba) -> String {
        format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)
    }

    fn svg(bytes: &'static [u8]) -> IconData {
        IconData::Svg(Cow::Borrowed(bytes))
    }

    fn rgba(side: u32) -> IconData {
        let n = usize::try_from(side).unwrap() * usize::try_from(side).unwrap() * 4;
        IconData::rgba(side, side, vec![0; n]).unwrap()
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
        use native_theme::ResolutionContext;
        use native_theme::theme::{ColorMode, Theme};
        let adwaita = Theme::preset("adwaita").unwrap();
        let ctx = ResolutionContext::for_tests();
        let light = adwaita
            .resolve_with(ColorMode::Light, &ctx)
            .unwrap()
            .variant;
        let dark = adwaita.resolve_with(ColorMode::Dark, &ctx).unwrap().variant;
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

    /// T9, per-scheme icon theme (`native-theme/src/presets/kde-breeze.toml:16`, `:410`).
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
        let indicator = native_theme::icons::load_icon_indicator(IconSet::Material, None).unwrap();
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

    /// T9, converters: `None` wherever epaint would assert (§9.3).
    #[test]
    fn to_color_image_refuses_what_epaint_would_assert_on() {
        assert!(to_color_image(&IconData::rgba(0, 4, vec![]).unwrap()).is_none());
        assert!(to_color_image(&IconData::rgba(4, 0, vec![]).unwrap()).is_none());
        // An overflowing size and a buffer of another length are refused by the core.
        assert!(IconData::rgba(u32::MAX, u32::MAX, vec![]).is_none());
        assert!(IconData::rgba(2, 2, vec![0; 15]).is_none());
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
        let icon = IconData::rgba(side, 1, vec![0; usize::try_from(side).unwrap() * 4]).unwrap();
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
        let frames = native_theme::icons::load_icon_indicator(IconSet::Material, None).unwrap();
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

    /// A provider's own SVG for a system set it names no icon in, or that the platform cannot
    /// load, is the icon, coloured as a monochrome icon (`colorize_monochrome_svg`, which fills
    /// this SVG's implicit black) in the colour [`custom_icon_to_image_source`] loads a system
    /// set in: the text colour of the installed atlas, else the tint. That holds for a
    /// freedesktop key too, where only a `currentColor` the SVG set no colour for used to be
    /// filled. A bundled set's is untouched without a tint, as [`IconKey::tint`] says, and
    /// coloured with one.
    #[test]
    fn a_providers_own_svg_takes_the_colour_its_set_loads_in() {
        const MONOCHROME: &[u8] =
            br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path d="M2 2h12v12H2z"/></svg>"#;
        let ctx = egui::Context::default();
        let (atlas, _) =
            crate::from_preset("adwaita", false, &AccessibilityPreferences::default()).unwrap();
        atlas.install(&ctx);
        ctx.set_theme(egui::Theme::Light);
        let text = atlas.resolved_for(egui::Theme::Light).defaults.text_color;
        let tint = egui::Color32::from_rgb(0x12, 0x34, 0x56);
        let provider = Provider {
            name: None,
            svg: Some(MONOCHROME),
        };
        let loaded = |key: &IconKey| {
            let source = custom_icon_to_image_source(&ctx, &provider, key)
                .unwrap_or_else(|| panic!("{key:?} loaded nothing"));
            bytes_of(&source).1
        };
        // The provider names no icon in any set (`name: None`), so no set's loader is asked and
        // its own SVG is the icon on every platform, macOS and Windows included.
        for set in [
            IconSet::Freedesktop,
            IconSet::SfSymbols,
            IconSet::SegoeFluent,
        ] {
            let key = IconKey::name("brand-mark", set);
            assert_eq!(
                loaded(&key),
                native_theme::icons::colorize_monochrome_svg(MONOCHROME, text),
                "{set:?}"
            );
            assert_eq!(
                loaded(&key.tint(tint)),
                native_theme::icons::colorize_monochrome_svg(
                    MONOCHROME,
                    Rgba::new(0x12, 0x34, 0x56, 0xff)
                ),
                "{set:?}"
            );
        }
        for set in [IconSet::Material, IconSet::Lucide] {
            let key = IconKey::name("brand-mark", set);
            assert_eq!(loaded(&key), MONOCHROME, "{set:?}");
            assert_eq!(
                loaded(&key.tint(tint)),
                native_theme::icons::colorize_monochrome_svg(
                    MONOCHROME,
                    Rgba::new(0x12, 0x34, 0x56, 0xff)
                ),
                "{set:?}"
            );
        }
    }

    /// Names one freedesktop icon for every set, as [`Provider`] does, and has no SVG.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn named(name: &'static str) -> Provider {
        Provider {
            name: Some(name),
            svg: None,
        }
    }

    /// Writes the theme `name` into `icons`, its one scalable directory holding each
    /// `(icon, svg)` as `<icon>.svg`.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn write_scalable_theme(
        icons: &std::path::Path,
        name: &str,
        files: &[(&str, &str)],
    ) -> std::io::Result<()> {
        let theme = icons.join(name);
        std::fs::create_dir_all(theme.join("places"))?;
        std::fs::write(
            theme.join("index.theme"),
            format!(
                "[Icon Theme]\nName={name}\nDirectories=places\n\n\
                 [places]\nSize=16\nType=Scalable\nMinSize=8\nMaxSize=512\n"
            ),
        )?;
        for (icon, svg) in files {
            std::fs::write(theme.join(format!("places/{icon}.svg")), svg)?;
        }
        Ok(())
    }

    /// A freedesktop icon whose own stylesheet sets the `color` its `currentColor` takes
    /// (Breeze's `current-color-scheme`) keeps that colour, and a full-colour one its palette,
    /// tint or not; one whose `currentColor` nothing in it resolves takes the tint, else the
    /// atlas's text colour. The theme is a fixture in a child process
    /// ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_freedesktop_custom_icon_is_coloured_where_it_sets_no_colour()
    -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str =
            "icons::tests::a_freedesktop_custom_icon_is_coloured_where_it_sets_no_colour";
        const STYLED: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><style id="current-color-scheme">.ColorScheme-Text { color:#232629; }</style><path class="ColorScheme-Text" style="fill:currentColor" d="M2 2h12v12H2z"/></svg>"#;
        // Breeze's `places/32/folder.svg` in shape: its body `fill:currentColor`, the accent
        // its stylesheet states, under a fill of its own.
        const FULL_COLOUR: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><style>.ColorScheme-Accent { color: #3daee9 }</style><path style="fill:currentColor" class="ColorScheme-Accent" d="M2 3h12l2 2h14v24H1V10h1z"/><path style="fill:#ffffff;fill-opacity:0.2" d="M14 3l1 3h15V5H16z"/></svg>"##;
        const BARE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2h12v12H2z"/></svg>"#;
        if crate::icon_fixture::in_child() {
            let ctx = egui::Context::default();
            let (atlas, _) =
                crate::from_preset("adwaita", false, &AccessibilityPreferences::default())?;
            atlas.install(&ctx);
            ctx.set_theme(egui::Theme::Light);
            let text = hex_rgb(atlas.resolved_for(egui::Theme::Light).defaults.text_color);
            let tint = egui::Color32::from_rgb(0x12, 0x34, 0x56);
            let loaded = |icon: &'static str, key: &IconKey| {
                custom_icon_to_image_source(&ctx, &named(icon), key)
                    .map(|source| String::from_utf8_lossy(&bytes_of(&source).1).into_owned())
                    .ok_or_else(|| format!("the fixture theme has no {icon}"))
            };
            for (icon, svg) in [("styled", STYLED), ("full-colour", FULL_COLOUR)] {
                let key = IconKey::name(icon, IconSet::Freedesktop);
                assert_eq!(loaded(icon, &key)?, svg);
                assert_eq!(loaded(icon, &key.tint(tint))?, svg);
            }
            let key = IconKey::name("bare", IconSet::Freedesktop);
            assert_eq!(loaded("bare", &key)?, BARE.replace("currentColor", &text));
            assert_eq!(
                loaded("bare", &key.tint(tint))?,
                BARE.replace("currentColor", "#123456")
            );
            return Ok(());
        }
        crate::icon_fixture::run(NAME, "colours", |icons| {
            write_scalable_theme(
                icons,
                "colours",
                &[
                    ("styled", STYLED),
                    ("full-colour", FULL_COLOUR),
                    ("bare", BARE),
                ],
            )
        })?;
        Ok(())
    }

    /// A custom freedesktop icon comes from the icon theme its key names
    /// ([`IconKey::icon_theme`]), not from the system's. The themes are fixtures in a child
    /// process ([`crate::icon_fixture`]), the system's `system`.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_custom_icon_comes_from_the_icon_theme_its_key_names()
    -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "icons::tests::a_custom_icon_comes_from_the_icon_theme_its_key_names";
        const SYSTEM: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#3daee9" d="M2 2h12v12H2z"/></svg>"##;
        const CHOSEN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#e01b24" d="M2 2h12v12H2z"/></svg>"##;
        if crate::icon_fixture::in_child() {
            let ctx = egui::Context::default();
            let loaded = |key: &IconKey| {
                custom_icon_to_image_source(&ctx, &named("folder"), key)
                    .map(|source| String::from_utf8_lossy(&bytes_of(&source).1).into_owned())
                    .ok_or("no folder")
            };
            let key = IconKey::name("folder", IconSet::Freedesktop);
            assert_eq!(loaded(&key.clone().icon_theme("chosen"))?, CHOSEN);
            assert_eq!(loaded(&key)?, SYSTEM);
            let only_chosen = IconKey::name("only-chosen", IconSet::Freedesktop);
            assert!(
                custom_icon_to_image_source(&ctx, &named("only-chosen"), &only_chosen).is_none(),
                "an icon the system's theme lacks is not taken from another"
            );
            assert_eq!(
                custom_icon_to_image_source(
                    &ctx,
                    &named("only-chosen"),
                    &only_chosen.icon_theme("chosen")
                )
                .map(|source| bytes_of(&source).1),
                Some(CHOSEN.as_bytes().to_vec())
            );
            return Ok(());
        }
        crate::icon_fixture::run(NAME, "system", |icons| {
            write_scalable_theme(icons, "system", &[("folder", SYSTEM)])?;
            write_scalable_theme(
                icons,
                "chosen",
                &[("folder", CHOSEN), ("only-chosen", CHOSEN)],
            )
        })?;
        Ok(())
    }

    /// M7-13: a provider's named icon is loaded once per icon theme, size and colour and kept
    /// until [`forget_icons`]: the fixture's file, rewritten after the first load, is read
    /// again only in another colour and after `forget_icons`. The theme is a fixture in a
    /// child process ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_named_custom_icon_is_loaded_once_until_forget_icons()
    -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "icons::tests::a_named_custom_icon_is_loaded_once_until_forget_icons";
        const FIRST: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#3daee9" d="M2 2h12v12H2z"/></svg>"##;
        const SECOND: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#e01b24" d="M2 2h12v12H2z"/></svg>"##;
        if let Some(icons) = crate::icon_fixture::icons_dir() {
            let ctx = egui::Context::default();
            let loaded = |key: &IconKey| {
                custom_icon_to_image_source(&ctx, &named("folder"), key)
                    .map(|source| String::from_utf8_lossy(&bytes_of(&source).1).into_owned())
                    .ok_or("no folder")
            };
            let key = IconKey::name("folder", IconSet::Freedesktop);
            assert_eq!(loaded(&key)?, FIRST);
            std::fs::write(icons.join("kept/places/folder.svg"), SECOND)?;
            assert_eq!(loaded(&key)?, FIRST, "the loaded icon is kept");
            let tinted = key.clone().tint(egui::Color32::from_rgb(0x12, 0x34, 0x56));
            assert_eq!(loaded(&tinted)?, SECOND, "another colour is loaded");
            forget_icons(&ctx);
            assert_eq!(loaded(&key)?, SECOND, "forget_icons drops the loaded icons");
            return Ok(());
        }
        crate::icon_fixture::run(NAME, "kept", |icons| {
            write_scalable_theme(icons, "kept", &[("folder", FIRST)])
        })?;
        Ok(())
    }

    /// M7-13: a key without an icon theme keeps the icon of the system's icon theme it was
    /// loaded from: once the system names another and native-theme's detection caches are
    /// invalidated, the next call loads that theme's icon. The themes are fixtures in a child
    /// process ([`crate::icon_fixture`]), the system's `before` and then `after`.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_kept_icon_follows_a_change_of_the_system_icon_theme()
    -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "icons::tests::a_kept_icon_follows_a_change_of_the_system_icon_theme";
        const BEFORE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#3daee9" d="M2 2h12v12H2z"/></svg>"##;
        const AFTER: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#e01b24" d="M2 2h12v12H2z"/></svg>"##;
        if let Some(kdeglobals) = crate::icon_fixture::kdeglobals() {
            let ctx = egui::Context::default();
            let loaded = || {
                let key = IconKey::name("folder", IconSet::Freedesktop);
                custom_icon_to_image_source(&ctx, &named("folder"), &key)
                    .map(|source| String::from_utf8_lossy(&bytes_of(&source).1).into_owned())
                    .ok_or("no folder")
            };
            assert_eq!(loaded()?, BEFORE);
            std::fs::write(kdeglobals, "[Icons]\nTheme=after\n")?;
            native_theme::detect::invalidate_caches();
            assert_eq!(loaded()?, AFTER, "the system's new icon theme");
            return Ok(());
        }
        crate::icon_fixture::run(NAME, "before", |icons| {
            write_scalable_theme(icons, "before", &[("folder", BEFORE)])?;
            write_scalable_theme(icons, "after", &[("folder", AFTER)])
        })?;
        Ok(())
    }

    /// M7-13: two providers whose names differ, under one key, each get their own icon: the
    /// kept icons are the names', not the key's. The theme is a fixture in a child process
    /// ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn two_provider_names_under_one_key_each_get_their_own_icon()
    -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "icons::tests::two_provider_names_under_one_key_each_get_their_own_icon";
        const FIRST: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#3daee9" d="M2 2h12v12H2z"/></svg>"##;
        const SECOND: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#e01b24" d="M2 2h12v12H2z"/></svg>"##;
        if crate::icon_fixture::in_child() {
            let ctx = egui::Context::default();
            let key = IconKey::name("brand-mark", IconSet::Freedesktop);
            let loaded = |name: &'static str| {
                custom_icon_to_image_source(&ctx, &named(name), &key)
                    .map(|source| String::from_utf8_lossy(&bytes_of(&source).1).into_owned())
                    .ok_or("no icon")
            };
            for _ in 0..2 {
                assert_eq!(loaded("first")?, FIRST);
                assert_eq!(loaded("second")?, SECOND);
            }
            return Ok(());
        }
        crate::icon_fixture::run(NAME, "names", |icons| {
            write_scalable_theme(icons, "names", &[("first", FIRST), ("second", SECOND)])
        })?;
        Ok(())
    }

    /// A custom freedesktop icon is looked up at the key's size ([`IconKey::size`]): the
    /// theme's directory of that size is the one it comes from, and a key's 0 is looked up
    /// at 1. The theme is a fixture in a child process ([`crate::icon_fixture`]) with a
    /// `folder` in a 16 and in a 64 directory, in a 0 directory, which a lookup at 0 matches,
    /// and in a scalable directory from 1 to 8, which a lookup at 1 matches, each its own.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_custom_icon_is_looked_up_at_the_keys_size() -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "icons::tests::a_custom_icon_is_looked_up_at_the_keys_size";
        let svg = |size: u32| {
            format!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}"><path fill="#3daee9" d="M0 0h{size}v{size}H0z"/></svg>"##
            )
        };
        if crate::icon_fixture::in_child() {
            let ctx = egui::Context::default();
            for size in [16, 64] {
                let points = f32::from(u8::try_from(size)?);
                let key = IconKey::name("folder", IconSet::Freedesktop).size(points);
                let source = custom_icon_to_image_source(&ctx, &named("folder"), &key)
                    .ok_or("the fixture theme has no folder")?;
                assert_eq!(
                    String::from_utf8_lossy(&bytes_of(&source).1),
                    svg(size),
                    "asked for {size}"
                );
            }
            for points in [0.0, -3.0, f32::NAN] {
                let key = IconKey::name("folder", IconSet::Freedesktop).size(points);
                let source = custom_icon_to_image_source(&ctx, &named("folder"), &key)
                    .ok_or("the fixture theme has no folder")?;
                assert_eq!(
                    String::from_utf8_lossy(&bytes_of(&source).1),
                    svg(1),
                    "asked for {points}"
                );
            }
            return Ok(());
        }
        crate::icon_fixture::run(NAME, "sized", |icons| {
            let theme = icons.join("sized");
            std::fs::create_dir_all(&theme)?;
            std::fs::write(
                theme.join("index.theme"),
                "[Icon Theme]\nName=sized\nDirectories=0/places,scalable/places,16/places,64/places\n\n\
                 [0/places]\nSize=0\nType=Fixed\n\n\
                 [scalable/places]\nSize=1\nType=Scalable\nMinSize=1\nMaxSize=8\n\n\
                 [16/places]\nSize=16\nType=Fixed\n\n\
                 [64/places]\nSize=64\nType=Fixed\n",
            )?;
            for (dir, size) in [("0", 0), ("scalable", 1), ("16", 16), ("64", 64)] {
                let dir = theme.join(format!("{dir}/places"));
                std::fs::create_dir_all(&dir)?;
                std::fs::write(dir.join("folder.svg"), svg(size))?;
            }
            Ok(())
        })?;
        Ok(())
    }

    /// An SF Symbol is configured at the key's size in points ([`IconKey::size`]): the raster
    /// is the one `SfSymbolsLoader` draws at that size, taller than at its default 24. Runs on
    /// the macOS CI leg.
    #[cfg(all(target_os = "macos", feature = "system-icons"))]
    #[test]
    fn an_sf_symbol_is_drawn_at_the_keys_size() {
        let name =
            native_theme::icons::icon_name(IconRole::ActionCopy, IconSet::SfSymbols).unwrap();
        let provider = Provider {
            name: Some(name),
            svg: None,
        };
        let ctx = egui::Context::default();
        let key = IconKey::name(name, IconSet::SfSymbols).size(64.0);
        let Some(egui::ImageSource::Texture(texture)) =
            custom_icon_to_image_source(&ctx, &provider, &key)
        else {
            panic!("{name} should load as a texture");
        };
        let raster = |loader: native_theme::icons::SfSymbolsLoader<'_>| match loader.load() {
            Some(IconData::Rgba(rgba)) => (rgba.width(), rgba.height()),
            _ => panic!("{name} should load as RGBA"),
        };
        let (width, height) = raster(native_theme::icons::SfSymbolsLoader::new(name).size(64));
        let (_, default_height) = raster(native_theme::icons::SfSymbolsLoader::new(name));
        assert_eq!(texture.size, egui::vec2(width as f32, height as f32));
        assert!(
            height > default_height,
            "{height} at 64 pt, {default_height} at 24"
        );
    }

    /// A Segoe glyph is drawn at the key's size in pixels ([`IconKey::size`]): 64 × 64, where
    /// `SegoeFluentLoader`'s default is 32. Runs on the Windows CI leg.
    #[cfg(all(target_os = "windows", feature = "system-icons"))]
    #[test]
    fn a_segoe_glyph_is_drawn_at_the_keys_size() {
        let name =
            native_theme::icons::icon_name(IconRole::ActionCopy, IconSet::SegoeFluent).unwrap();
        let provider = Provider {
            name: Some(name),
            svg: None,
        };
        let ctx = egui::Context::default();
        let key = IconKey::name(name, IconSet::SegoeFluent).size(64.0);
        let Some(egui::ImageSource::Texture(texture)) =
            custom_icon_to_image_source(&ctx, &provider, &key)
        else {
            panic!("{name} should load as a texture");
        };
        assert_eq!(texture.size, egui::vec2(64.0, 64.0));
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

    /// An `IconRole` is a provider: `custom_icon_to_image` loads a role's icon from the key's
    /// set in one call, and a role the set lacks gives `None`, from no other set.
    #[cfg(feature = "lucide-icons")]
    #[test]
    fn custom_icon_to_image_loads_a_role() {
        let ctx = egui::Context::default();
        let role = IconRole::DialogWarning;
        let key = IconKey::role(role, IconSet::Lucide).size(16.0);
        assert!(custom_icon_to_image(&ctx, &role, &key).is_some());
        // The alt text is the role's name: the image's `WidgetInfo.label`
        // (`egui/src/widgets/image.rs:408`), which AccessKit reports.
        use egui_kittest::kittest::Queryable as _;
        let harness = egui_kittest::Harness::new_ui(|ui| {
            if let Some(image) = custom_icon_to_image(ui.ctx(), &role, &key) {
                ui.add(image);
            }
        });
        let _ = harness.get_by_label(role.name());
        // SF Symbols has no open folder (`native-theme/src/model/icons.rs:571`) and the role
        // has no bundled SVG for it, so this is `None` on every platform, macOS included: no
        // name reaches the SF Symbols loader and no SVG of the provider's own is left.
        let lacking = IconRole::FolderOpen;
        let key = IconKey::role(lacking, IconSet::SfSymbols);
        assert!(custom_icon_to_image(&ctx, &lacking, &key).is_none());
    }

    /// `IconKey::size` is total over every `f32` (§4.10).
    #[test]
    fn a_size_folds_every_float_to_one_u32() {
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
            Some(u32::MAX)
        );
        assert_eq!(IconKey::name("x", IconSet::Material).size, None);
    }
}
