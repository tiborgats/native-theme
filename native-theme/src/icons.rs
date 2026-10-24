//! Icon loading and dispatch.

use std::fmt;

use std::borrow::Cow;

use crate::color::Rgba;
#[allow(unused_imports)]
use crate::detect::{system_icon_set, system_icon_theme};
use crate::model::IconSet;
pub use crate::model::{
    AnimatedIcon, EmptyFrameListError, FrameList, FramesData, IconData, IconProvider, IconRole,
    RgbaData, TransformAnimation, TransformData, icon_name,
};
#[allow(unused_imports)]
use crate::model::{bundled_icon_by_name, bundled_icon_svg};

/// The largest buffer, in bytes, built while decoding an icon from outside
/// data: a PNG's output buffer, a sprite sheet's frames (`frame_count ×
/// svg_len`) and a raster the OS draws (an SF Symbol's or a stock shell
/// icon's bitmap). A raster drawn at a requested size (`size² × 4`: an SVG,
/// a Segoe glyph) is checked against it too, but only as a backstop: the
/// size is clamped to [`MAX_ICON_SIZE`] first, and 512² × 4 bytes is 1 MiB,
/// so that check cannot fail.
///
/// 64 MiB, the png crate's own default memory limit (`Limits::default()`,
/// png-0.18.1 `src/decoder/mod.rs:81-87`): a 4096 × 4096 RGBA image, beyond
/// any icon directory's size.
#[cfg_attr(
    not(any(
        all(target_os = "linux", feature = "system-icons"),
        feature = "svg-rasterize"
    )),
    allow(dead_code)
)]
pub(crate) const MAX_ICON_DECODE_BYTES: usize = 64 * 1024 * 1024;

/// The largest size, in pixels (points for an SF Symbol), that the system
/// loaders ([`FreedesktopLoader::size`], [`SfSymbolsLoader::size`],
/// [`SegoeFluentLoader::size`]), and through them [`load_custom_icon`] and
/// [`load_custom_icon_in`], and `rasterize::rasterize_svg` (feature
/// `svg-rasterize`) load or draw an icon at: a larger size is taken as this
/// one, and 0 as 1.
///
/// 512, so the largest raster one call builds is 512² × 4 bytes, 1 MiB, where
/// a size of 10 000 would allocate 400 MB for one icon. A connector that loads
/// and rasterizes its icons through these shares the bound.
pub const MAX_ICON_SIZE: u32 = 512;

/// Whether a `size × size` RGBA raster (`size² × 4` bytes) is inside
/// [`MAX_ICON_DECODE_BYTES`]; a size of 0 is not.
#[cfg_attr(
    not(all(
        any(target_os = "macos", target_os = "windows"),
        feature = "system-icons"
    )),
    allow(dead_code)
)]
pub(crate) fn raster_size_in_bound(size: u32) -> bool {
    raster_bytes_in_bound(size, size).is_some()
}

/// The bytes of a `width × height` RGBA raster (`width × height × 4`) where
/// they are inside [`MAX_ICON_DECODE_BYTES`]; `None` for a side of 0 or a
/// raster over the bound.
#[cfg_attr(
    not(any(
        all(
            any(target_os = "macos", target_os = "windows"),
            feature = "system-icons"
        ),
        feature = "svg-rasterize"
    )),
    allow(dead_code)
)]
pub(crate) fn raster_bytes_in_bound(width: u32, height: u32) -> Option<usize> {
    if width == 0 || height == 0 {
        return None;
    }
    u64::from(width)
        .checked_mul(u64::from(height))?
        .checked_mul(4)
        .filter(|&bytes| bytes <= MAX_ICON_DECODE_BYTES as u64)
        .and_then(|bytes| usize::try_from(bytes).ok())
}

/// The red, green and blue of a loader's foreground colour; the alpha is
/// dropped, as the loaders draw the glyph's own coverage.
fn rgb(color: Rgba) -> [u8; 3] {
    [color.r, color.g, color.b]
}

/// Identifies what icon to load: a semantic role, a platform-specific name,
/// or a custom provider.
///
/// Constructed via [`From`] impls -- users rarely reference this type directly.
/// Pass an [`IconRole`], `&str`, or `&dyn IconProvider` to a per-set loader's
/// `new()` constructor (e.g. [`FreedesktopLoader::new`] or [`MaterialLoader::new`]).
///
/// # Examples
///
/// ```
/// use native_theme::icons::IconId;
/// use native_theme::icons::IconRole;
///
/// // All of these work via Into<IconId>:
/// let _: IconId = IconRole::ActionSave.into();
/// let _: IconId = "edit-copy".into();
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum IconId<'a> {
    /// Load by semantic role (most common).
    Role(IconRole),
    /// Load by platform-specific name string.
    Name(&'a str),
    /// Load via a custom [`IconProvider`] implementation.
    Custom(&'a dyn IconProvider),
}

impl From<IconRole> for IconId<'_> {
    fn from(role: IconRole) -> Self {
        IconId::Role(role)
    }
}

impl<'a> From<&'a str> for IconId<'a> {
    fn from(name: &'a str) -> Self {
        IconId::Name(name)
    }
}

impl<'a> From<&'a dyn IconProvider> for IconId<'a> {
    fn from(provider: &'a dyn IconProvider) -> Self {
        IconId::Custom(provider)
    }
}

/// A provider by reference, such as a variant of the icon enum
/// `native-theme-build` generates: `MaterialLoader::new(&AppIcon::PlayPause)`.
impl<'a, T: IconProvider> From<&'a T> for IconId<'a> {
    fn from(provider: &'a T) -> Self {
        IconId::Custom(provider)
    }
}

// =============================================================================
// Typed per-set icon loaders
// =============================================================================
//
// Each set has its own loader struct exposing only the methods meaningful
// for that set. This eliminates the silent-ignore bug class where a method
// like `.theme()` could be called on a non-freedesktop loader and have no
// effect. Calling a set-specific method on the wrong loader is now a
// compile error.
//
// For runtime-set dispatch (common in connector code), use the free
// functions `load_icon(id, set)`, `load_custom_icon(provider, set, color, size)`,
// `load_custom_icon_in(provider, set, icon_theme, color, size)` and
// `load_icon_indicator(set, theme)` at the bottom of this section.

/// Loader for freedesktop.org icon themes (GTK / Linux).
///
/// Honors `.theme()` for BOTH role-based and name-based lookups — unlike
/// the previous single-struct design, no dispatch layer can silently
/// drop the theme override.
///
/// An icon comes only from the theme or from a theme in the `Inherits=`
/// chain its `index.theme` declares, searched depth-first in declared
/// order as the Icon Theme Specification's lookup does, never from
/// `hicolor` unless `hicolor` is the theme asked for, nor from a loose
/// file in an icon directory or `/usr/share/pixmaps`. An icon the theme
/// and its parents lack is `None`, and so is every icon of a theme that
/// is not installed.
///
/// The installed themes are read once, at the process's first freedesktop
/// lookup (freedesktop-icons caches them the same way), and a theme's
/// `Inherits=` chain at its own first lookup, so a theme installed after
/// that is not searched, and a changed `Inherits=` is not seen, until the
/// process restarts.
///
/// ```
/// # #[cfg(all(target_os = "linux", feature = "system-icons"))]
/// # {
/// use native_theme::icons::FreedesktopLoader;
/// use native_theme::icons::IconRole;
///
/// // By role, system theme:
/// let _icon = FreedesktopLoader::new(IconRole::ActionCopy).load();
///
/// // By name, explicit theme + size + symbolic fg color:
/// let _icon = FreedesktopLoader::new("edit-copy")
///     .theme("Adwaita")
///     .size(48)
///     .color(native_theme::Rgba::rgb(0, 0, 0))
///     .load();
/// # }
/// ```
#[derive(Debug)]
#[must_use]
pub struct FreedesktopLoader<'a> {
    // Fields read only in the Linux + system-icons cfg branch of `load`.
    #[allow(dead_code)]
    wanted: Wanted<'a>,
    #[allow(dead_code)]
    size: u32,
    #[allow(dead_code)]
    fg_color: Option<[u8; 3]>,
    #[allow(dead_code)]
    theme: Option<&'a str>,
}

/// What a [`FreedesktopLoader`] looks up.
#[derive(Debug)]
enum Wanted<'a> {
    /// One icon: a role, a name or a provider ([`FreedesktopLoader::new`]).
    // Read only in the Linux + system-icons cfg branch of `load`.
    #[allow(dead_code)]
    Id(IconId<'a>),
    /// The first of several names the theme has ([`FreedesktopLoader::any`]).
    #[allow(dead_code)]
    AnyOf(&'a [&'a str]),
}

impl<'a> FreedesktopLoader<'a> {
    /// Create a freedesktop loader. Defaults: size 24, no fg_color, system theme.
    pub fn new(id: impl Into<IconId<'a>>) -> Self {
        Self::wanting(Wanted::Id(id.into()))
    }

    /// Create a loader for the first of `names` the theme has, for a
    /// concept icon themes name differently (Breeze's `view-visible`,
    /// Adwaita's `view-reveal`). Each theme of the chain (see
    /// [`FreedesktopLoader`]) is searched for every name, in order, before
    /// the theme it inherits from, so the theme's own icon under a later
    /// name wins over its parent's under an earlier one: Adwaita's
    /// `starred-symbolic` over AdwaitaLegacy's raster `emblem-favorite`,
    /// which [`Self::new`] tried name by name would load. Within a theme a
    /// name is tried as `<name>-symbolic`, then as itself. Every name is
    /// looked up in the one theme and its chain, so no icon comes from
    /// another set. Defaults as [`Self::new`]; no names load `None`.
    ///
    /// With one name, `any(&[name])` differs from `new(name)` in one case: a
    /// theme that has the plain `name` while a theme it inherits from has
    /// `<name>-symbolic`. `any` takes the theme's own plain icon, as it
    /// searches a theme before its parent; `new` searches the whole chain for
    /// the symbolic form first and takes the parent's.
    ///
    /// ```
    /// # #[cfg(all(target_os = "linux", feature = "system-icons"))]
    /// # {
    /// use native_theme::icons::FreedesktopLoader;
    ///
    /// let _eye = FreedesktopLoader::any(&["view-visible", "view-reveal"])
    ///     .theme("Adwaita")
    ///     .load();
    /// # }
    /// ```
    pub fn any(names: &'a [&'a str]) -> Self {
        Self::wanting(Wanted::AnyOf(names))
    }

    /// A loader for `wanted`, with [`Self::new`]'s defaults.
    fn wanting(wanted: Wanted<'a>) -> Self {
        Self {
            wanted,
            size: 24,
            fg_color: None,
            theme: None,
        }
    }

    /// Set the requested icon size in pixels (default 24), clamped to
    /// `1..=`[`MAX_ICON_SIZE`]: a larger size is looked up as 512, in the
    /// theme's directory nearest it.
    pub fn size(mut self, size: u32) -> Self {
        self.size = size.clamp(1, MAX_ICON_SIZE);
        self
    }

    /// Set the foreground color for GTK symbolic icon colorization; its
    /// alpha is ignored: the icon keeps its own coverage.
    pub fn color(mut self, color: Rgba) -> Self {
        self.fg_color = Some(rgb(color));
        self
    }

    /// Set the foreground color if `Some`, leave default if `None`; its
    /// alpha is ignored, as in [`Self::color`].
    pub fn color_opt(mut self, color: Option<Rgba>) -> Self {
        self.fg_color = color.map(rgb);
        self
    }

    /// Override the freedesktop icon theme (e.g. "Adwaita", "breeze").
    /// When unset, uses the system-detected theme, and searches no theme
    /// where detection fails (see [`Self::load`]).
    pub fn theme(mut self, theme: &'a str) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Load the icon, returning its data.
    ///
    /// `None` when neither the theme nor a theme it inherits from has the
    /// icon (see [`FreedesktopLoader`]).
    ///
    /// With no [`theme`](Self::theme) set, the icon comes from the system's
    /// icon theme, and where that cannot be detected no theme is searched,
    /// as none stands in for the undetected one: a role or a name gives
    /// `None`, and a custom [`IconProvider`] gives only its own freedesktop
    /// SVG ([`IconProvider::icon_svg`]), which depends on no theme, where it
    /// has one. [`system_icon_theme()`](crate::detect::system_icon_theme)
    /// gives the reason.
    ///
    /// Requires the `system-icons` feature, and Linux; `None` otherwise.
    #[must_use]
    pub fn load(self) -> Option<IconData> {
        self.load_with(system_icon_theme)
    }

    /// [`Self::load`], with `detect` giving the system's icon theme where no
    /// theme is set.
    #[allow(unused_variables)]
    fn load_with(self, detect: impl FnOnce() -> crate::Result<String>) -> Option<IconData> {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        {
            let detected;
            let theme: Option<&str> = match self.theme {
                Some(t) => Some(t),
                None => {
                    detected = detect().ok();
                    detected.as_deref()
                }
            };
            match self.wanted {
                Wanted::AnyOf(names) => crate::freedesktop::load_freedesktop_icon_by_names(
                    names,
                    theme?,
                    self.size,
                    self.fg_color,
                ),
                Wanted::Id(IconId::Role(role)) => {
                    let name = icon_name(role, IconSet::Freedesktop)?;
                    crate::freedesktop::load_freedesktop_icon_by_name(
                        name,
                        theme?,
                        self.size,
                        self.fg_color,
                    )
                }
                Wanted::Id(IconId::Name(name)) => {
                    crate::freedesktop::load_freedesktop_icon_by_name(
                        name,
                        theme?,
                        self.size,
                        self.fg_color,
                    )
                }
                Wanted::Id(IconId::Custom(provider)) => {
                    if let Some(theme) = theme
                        && let Some(name) = provider.icon_name(IconSet::Freedesktop)
                        && let Some(data) = crate::freedesktop::load_freedesktop_icon_by_name(
                            name,
                            theme,
                            self.size,
                            self.fg_color,
                        )
                    {
                        return Some(data);
                    }
                    provider.icon_svg(IconSet::Freedesktop).map(IconData::Svg)
                }
            }
        }
        #[cfg(not(all(target_os = "linux", feature = "system-icons")))]
        {
            None
        }
    }

    /// Load the theme's animated process-working spinner.
    ///
    /// `theme` of `None` uses the system-detected theme, and gives `None`
    /// where detection fails — no theme stands in for the undetected one;
    /// `Some(t)` overrides.
    /// Associated function (no `self`) — the spinner is a property of the
    /// theme, not of any particular icon id. Found only where
    /// [`Self::load`] would find an icon: in the theme or a theme it
    /// inherits from, never in `hicolor` unless that is the theme.
    ///
    /// Requires the `system-icons` feature, and Linux; `None` otherwise.
    #[must_use]
    #[allow(unused_variables)]
    pub fn load_indicator(theme: Option<&str>) -> Option<AnimatedIcon> {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        {
            crate::freedesktop::load_freedesktop_spinner(theme)
        }
        #[cfg(not(all(target_os = "linux", feature = "system-icons")))]
        {
            None
        }
    }
}

/// Loader for Apple SF Symbols (macOS only).
///
/// SF Symbols has no concept of themes. A symbol is a monochrome template,
/// drawn black unless [`Self::color`] gives it a foreground colour.
#[derive(Debug)]
#[must_use]
pub struct SfSymbolsLoader<'a> {
    #[allow(dead_code)] // read only in the macOS cfg branch of `load`
    id: IconId<'a>,
    #[allow(dead_code)] // read only in the macOS cfg branch of `load`
    size: Option<u32>,
    #[allow(dead_code)] // read only in the macOS cfg branch of `load`
    fg_color: Option<[u8; 3]>,
}

impl<'a> SfSymbolsLoader<'a> {
    /// Construct a new SF Symbols loader for the given icon id. Defaults:
    /// size 24 points, no fg_color.
    pub fn new(id: impl Into<IconId<'a>>) -> Self {
        Self {
            id: id.into(),
            size: None,
            fg_color: None,
        }
    }

    /// Set the symbol's point size (default 24), the size its symbol
    /// configuration is made with, clamped to `1..=`[`MAX_ICON_SIZE`]. A
    /// symbol whose raster the OS draws over the 64 MiB decode bound loads
    /// `None`.
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size.clamp(1, MAX_ICON_SIZE));
        self
    }

    /// Set the foreground color the monochrome symbol is drawn in; its
    /// alpha is ignored: the symbol keeps its own coverage.
    pub fn color(mut self, color: Rgba) -> Self {
        self.fg_color = Some(rgb(color));
        self
    }

    /// Set the foreground color if `Some`, leave default if `None`; its
    /// alpha is ignored, as in [`Self::color`].
    pub fn color_opt(mut self, color: Option<Rgba>) -> Self {
        self.fg_color = color.map(rgb);
        self
    }

    /// Load the icon, returning its data.
    ///
    /// Requires the `system-icons` feature, and macOS; `None` otherwise.
    #[must_use]
    #[allow(unused_variables)]
    pub fn load(self) -> Option<IconData> {
        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        {
            match self.id {
                IconId::Role(role) => {
                    let name = icon_name(role, IconSet::SfSymbols)?;
                    crate::sficons::load_sf_icon_by_name(name, self.size, self.fg_color)
                }
                IconId::Name(n) => {
                    crate::sficons::load_sf_icon_by_name(n, self.size, self.fg_color)
                }
                IconId::Custom(p) => {
                    if let Some(n) = p.icon_name(IconSet::SfSymbols)
                        && let Some(data) =
                            crate::sficons::load_sf_icon_by_name(n, self.size, self.fg_color)
                    {
                        return Some(data);
                    }
                    p.icon_svg(IconSet::SfSymbols).map(IconData::Svg)
                }
            }
        }
        #[cfg(not(all(target_os = "macos", feature = "system-icons")))]
        {
            None
        }
    }
    // No load_indicator: SF Symbols has no animated spinner primitive.
}

/// Loader for Windows Segoe Fluent Icons (Windows only).
///
/// Segoe icons have no themes. A Segoe Fluent glyph is monochrome, drawn
/// white unless [`Self::color`] gives it a foreground colour; a stock shell
/// icon (the `SIID_*` and `IDI_QUESTION` names) is full-colour and keeps its
/// own colours.
#[derive(Debug)]
#[must_use]
pub struct SegoeFluentLoader<'a> {
    #[allow(dead_code)] // read only in the Windows cfg branch of `load`
    id: IconId<'a>,
    #[allow(dead_code)] // read only in the Windows cfg branch of `load`
    size: Option<u32>,
    #[allow(dead_code)] // read only in the Windows cfg branch of `load`
    fg_color: Option<[u8; 3]>,
}

impl<'a> SegoeFluentLoader<'a> {
    /// Construct a new Segoe Fluent loader for the given icon id. Defaults:
    /// size 32 pixels, no fg_color.
    pub fn new(id: impl Into<IconId<'a>>) -> Self {
        Self {
            id: id.into(),
            size: None,
            fg_color: None,
        }
    }

    /// Set a glyph's size in pixels (default 32), the em height its font
    /// is created with, clamped to `1..=`[`MAX_ICON_SIZE`]. A stock shell
    /// icon keeps the system's large icon size.
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size.clamp(1, MAX_ICON_SIZE));
        self
    }

    /// Set the foreground color a monochrome glyph is drawn in; its alpha
    /// is ignored: the glyph keeps its own coverage. A full-colour stock
    /// icon is unchanged.
    pub fn color(mut self, color: Rgba) -> Self {
        self.fg_color = Some(rgb(color));
        self
    }

    /// Set the foreground color if `Some`, leave default if `None`; its
    /// alpha is ignored, as in [`Self::color`].
    pub fn color_opt(mut self, color: Option<Rgba>) -> Self {
        self.fg_color = color.map(rgb);
        self
    }

    /// Load the icon, returning its data.
    ///
    /// Requires the `system-icons` feature, and Windows; `None` otherwise.
    #[must_use]
    #[allow(unused_variables)]
    pub fn load(self) -> Option<IconData> {
        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        {
            match self.id {
                IconId::Role(role) => {
                    let name = icon_name(role, IconSet::SegoeFluent)?;
                    crate::winicons::load_windows_icon_by_name(name, self.size, self.fg_color)
                }
                IconId::Name(n) => {
                    crate::winicons::load_windows_icon_by_name(n, self.size, self.fg_color)
                }
                IconId::Custom(p) => {
                    if let Some(n) = p.icon_name(IconSet::SegoeFluent)
                        && let Some(data) =
                            crate::winicons::load_windows_icon_by_name(n, self.size, self.fg_color)
                    {
                        return Some(data);
                    }
                    p.icon_svg(IconSet::SegoeFluent).map(IconData::Svg)
                }
            }
        }
        #[cfg(not(all(target_os = "windows", feature = "system-icons")))]
        {
            None
        }
    }
    // No load_indicator: Segoe has no animated spinner primitive.
}

/// Loader for Google Material Symbols (bundled SVG).
///
/// Material SVGs are scalable at render time, so no `size` method.
#[derive(Debug)]
#[must_use]
pub struct MaterialLoader<'a> {
    #[allow(dead_code)] // read only in the material-icons feature cfg branch of `load`
    id: IconId<'a>,
}

impl<'a> MaterialLoader<'a> {
    /// Construct a new Material Symbols loader for the given icon id.
    pub fn new(id: impl Into<IconId<'a>>) -> Self {
        Self { id: id.into() }
    }

    /// Load the icon, returning its data.
    ///
    /// Requires the `material-icons` feature; `None` otherwise.
    #[must_use]
    #[allow(unused_variables)]
    pub fn load(self) -> Option<IconData> {
        #[cfg(feature = "material-icons")]
        {
            match self.id {
                IconId::Role(role) => bundled_icon_svg(role, IconSet::Material)
                    .map(|b| IconData::Svg(Cow::Borrowed(b))),
                IconId::Name(n) => bundled_icon_by_name(n, IconSet::Material)
                    .map(|b| IconData::Svg(Cow::Borrowed(b))),
                IconId::Custom(p) => {
                    if let Some(n) = p.icon_name(IconSet::Material)
                        && let Some(b) = bundled_icon_by_name(n, IconSet::Material)
                    {
                        return Some(IconData::Svg(Cow::Borrowed(b)));
                    }
                    p.icon_svg(IconSet::Material).map(IconData::Svg)
                }
            }
        }
        #[cfg(not(feature = "material-icons"))]
        {
            None
        }
    }

    /// Load the Material animated spinner. Associated function; no `self`.
    ///
    /// Requires the `material-icons` feature; `None` otherwise.
    #[must_use]
    pub fn load_indicator() -> Option<AnimatedIcon> {
        #[cfg(feature = "material-icons")]
        {
            Some(crate::spinners::material_spinner())
        }
        #[cfg(not(feature = "material-icons"))]
        {
            None
        }
    }
}

/// Loader for Lucide Icons (bundled SVG).
#[derive(Debug)]
#[must_use]
pub struct LucideLoader<'a> {
    #[allow(dead_code)] // read only in the lucide-icons feature cfg branch of `load`
    id: IconId<'a>,
}

impl<'a> LucideLoader<'a> {
    /// Construct a new Lucide Icons loader for the given icon id.
    pub fn new(id: impl Into<IconId<'a>>) -> Self {
        Self { id: id.into() }
    }

    /// Load the icon, returning its data.
    ///
    /// Requires the `lucide-icons` feature; `None` otherwise.
    #[must_use]
    #[allow(unused_variables)]
    pub fn load(self) -> Option<IconData> {
        #[cfg(feature = "lucide-icons")]
        {
            match self.id {
                IconId::Role(role) => {
                    bundled_icon_svg(role, IconSet::Lucide).map(|b| IconData::Svg(Cow::Borrowed(b)))
                }
                IconId::Name(n) => bundled_icon_by_name(n, IconSet::Lucide)
                    .map(|b| IconData::Svg(Cow::Borrowed(b))),
                IconId::Custom(p) => {
                    if let Some(n) = p.icon_name(IconSet::Lucide)
                        && let Some(b) = bundled_icon_by_name(n, IconSet::Lucide)
                    {
                        return Some(IconData::Svg(Cow::Borrowed(b)));
                    }
                    p.icon_svg(IconSet::Lucide).map(IconData::Svg)
                }
            }
        }
        #[cfg(not(feature = "lucide-icons"))]
        {
            None
        }
    }

    /// Load the Lucide animated spinner. Associated function; no `self`.
    ///
    /// Requires the `lucide-icons` feature; `None` otherwise.
    #[must_use]
    pub fn load_indicator() -> Option<AnimatedIcon> {
        #[cfg(feature = "lucide-icons")]
        {
            Some(crate::spinners::lucide_spinner())
        }
        #[cfg(not(feature = "lucide-icons"))]
        {
            None
        }
    }
}

// =============================================================================
// Runtime-set dispatch helpers
// =============================================================================

/// Load an icon using the given set with default per-set options.
///
/// For set-specific options (freedesktop theme, fg_color), construct the
/// specific loader directly and chain the relevant methods.
///
/// Each set needs its loader's feature (see the loaders); `None` without it.
#[must_use]
pub fn load_icon<'a>(id: impl Into<IconId<'a>>, set: IconSet) -> Option<IconData> {
    let id = id.into();
    match set {
        IconSet::Freedesktop => FreedesktopLoader::new(id).load(),
        IconSet::SfSymbols => SfSymbolsLoader::new(id).load(),
        IconSet::SegoeFluent => SegoeFluentLoader::new(id).load(),
        IconSet::Material => MaterialLoader::new(id).load(),
        IconSet::Lucide => LucideLoader::new(id).load(),
    }
}

/// Load the animated loading indicator for the given set.
///
/// `theme` is the freedesktop theme to take the spinner from, as
/// [`FreedesktopLoader::load_indicator`] takes it: `None` uses the
/// system-detected theme. It is used only for [`IconSet::Freedesktop`];
/// for the sets that have no themes (Material, Lucide, SfSymbols,
/// SegoeFluent), the theme name is not used by the loader.
///
/// Returns `None` for sets without an animated spinner (SfSymbols, SegoeFluent).
///
/// Each set needs its loader's feature (see the loaders); `None` without it.
#[must_use]
pub fn load_icon_indicator(set: IconSet, theme: Option<&str>) -> Option<AnimatedIcon> {
    match set {
        IconSet::Freedesktop => FreedesktopLoader::load_indicator(theme),
        IconSet::Material => MaterialLoader::load_indicator(),
        IconSet::Lucide => LucideLoader::load_indicator(),
        IconSet::SfSymbols | IconSet::SegoeFluent => None,
    }
}

/// Load a custom icon from an [`IconProvider`] for a set known only at
/// runtime, in `color`, at `size`.
///
/// The provider's [`icon_name`](IconProvider::icon_name) for `set` is loaded
/// first; where it has none, or the set has no such icon, its own
/// [`icon_svg`](IconProvider::icon_svg) for `set` is used. No other set is
/// tried.
///
/// Each set takes `color` its own way: an SF Symbol or a Segoe glyph loads
/// in it ([`SfSymbolsLoader::color`], [`SegoeFluentLoader::color`]), and a
/// Material or Lucide icon, or the provider's own SVG, is coloured through
/// [`colorize_monochrome_svg`]. A freedesktop icon takes it where it has a
/// foreground left to the caller: a symbolic icon's placeholders
/// ([`FreedesktopLoader::color`]) and a `currentColor` the document sets no
/// `color` for ([`fill_current_color`]) are drawn in it. An icon whose own
/// stylesheet sets the `color` its `currentColor` takes (Breeze's
/// `current-color-scheme`) keeps that colour, as it is not yet rewritten from
/// the theme as KDE's own icon loader does, and a full-colour icon keeps its
/// palette. `None` leaves each icon in the colour it loads in. A freedesktop
/// icon comes from the system's icon theme; [`load_custom_icon_in`] takes
/// another, and [`SystemTheme::icon`](crate::SystemTheme::icon) the theme's
/// own. A provider's own SVG is coloured as a monochrome icon, so give a
/// colour only for monochrome icons; with `None` a full-colour SVG is
/// returned unchanged.
///
/// `size` is clamped to `1..=`[`MAX_ICON_SIZE`] and reaches each system
/// set's loader as that loader takes it: the pixel size a freedesktop icon
/// is looked up at ([`FreedesktopLoader::size`], which picks the theme's
/// directory nearest it, so a PNG icon comes back at that directory's
/// size), an SF Symbol's point size ([`SfSymbolsLoader::size`]) and a Segoe
/// glyph's pixel size ([`SegoeFluentLoader::size`]); a loader that cannot
/// draw the icon at that size returns `None`, as for an icon it does not
/// find. `None` keeps each loader's default (24 pixels, 24 points and 32
/// pixels). A Material or Lucide icon, or the provider's own SVG, is
/// scalable and loads the same at every size.
///
/// Each set needs its loader's feature (see the loaders); `None` without it.
#[must_use]
pub fn load_custom_icon(
    provider: &(impl IconProvider + ?Sized),
    set: IconSet,
    color: Option<Rgba>,
    size: Option<u32>,
) -> Option<IconData> {
    load_custom_icon_in(provider, set, None, color, size)
}

/// [`load_custom_icon`], with a freedesktop icon looked up in `icon_theme`
/// ([`FreedesktopLoader::theme`]) where it is `Some`, the system's icon
/// theme where it is `None`. `size` is clamped to `1..=`[`MAX_ICON_SIZE`]
/// as there.
///
/// For the icon theme the application's theme names
/// ([`SystemTheme::icon_theme_for`](crate::SystemTheme::icon_theme_for),
/// [`Resolved::icon_theme`](crate::Resolved::icon_theme)), so a provider's
/// icon comes from the same icon theme as the role icons beside it. An
/// icon the theme and the themes it inherits from lack is the provider's
/// own SVG, or `None`; no other theme is searched. The other sets have no
/// themes and do not use it.
#[must_use]
pub fn load_custom_icon_in(
    provider: &(impl IconProvider + ?Sized),
    set: IconSet,
    icon_theme: Option<&str>,
    color: Option<Rgba>,
    size: Option<u32>,
) -> Option<IconData> {
    load_provider_icon(provider, set, icon_theme, color, size)
}

/// The runtime-set load behind
/// [`SystemTheme::icon`](crate::SystemTheme::icon): `id` from `set` in
/// `color` at `size`, a freedesktop icon from `theme` (the system's where
/// `None`).
pub(crate) fn load_icon_in<'a>(
    id: IconId<'a>,
    set: IconSet,
    theme: Option<&'a str>,
    color: Option<Rgba>,
    size: Option<u32>,
) -> Option<IconData> {
    match id {
        IconId::Custom(provider) => load_provider_icon(provider, set, theme, color, size),
        id => load_set_icon(id, set, theme, color, size),
    }
}

/// A provider's icon: its name for `set` through the set's loader, else its
/// own SVG, coloured.
fn load_provider_icon(
    provider: &(impl IconProvider + ?Sized),
    set: IconSet,
    theme: Option<&str>,
    color: Option<Rgba>,
    size: Option<u32>,
) -> Option<IconData> {
    if let Some(name) = provider.icon_name(set)
        && let Some(data) = load_set_icon(IconId::Name(name), set, theme, color, size)
    {
        return Some(data);
    }
    provider
        .icon_svg(set)
        .map(|svg| colorize_svg_data(IconData::Svg(svg), color))
}

/// A role or a name from `set`'s loader, in `color`, at `size` where the
/// loader takes one; each system loader's `size` clamps it to
/// `1..=MAX_ICON_SIZE`.
fn load_set_icon<'a>(
    id: IconId<'a>,
    set: IconSet,
    theme: Option<&'a str>,
    color: Option<Rgba>,
    size: Option<u32>,
) -> Option<IconData> {
    match set {
        IconSet::Freedesktop => {
            let loader = FreedesktopLoader::new(id).color_opt(color);
            let loader = match size {
                Some(size) => loader.size(size),
                None => loader,
            };
            match theme {
                Some(theme) => loader.theme(theme),
                None => loader,
            }
            .load()
            .map(|data| fill_freedesktop_svg_data(data, color))
        }
        IconSet::SfSymbols => {
            let loader = SfSymbolsLoader::new(id).color_opt(color);
            match size {
                Some(size) => loader.size(size),
                None => loader,
            }
            .load()
        }
        IconSet::SegoeFluent => {
            let loader = SegoeFluentLoader::new(id).color_opt(color);
            match size {
                Some(size) => loader.size(size),
                None => loader,
            }
            .load()
        }
        IconSet::Material => MaterialLoader::new(id)
            .load()
            .map(|data| colorize_svg_data(data, color)),
        IconSet::Lucide => LucideLoader::new(id)
            .load()
            .map(|data| colorize_svg_data(data, color)),
    }
}

/// An SVG coloured through [`colorize_monochrome_svg`] where `color` is
/// given; anything else unchanged.
fn colorize_svg_data(data: IconData, color: Option<Rgba>) -> IconData {
    match (data, color) {
        (IconData::Svg(svg), Some(color)) => {
            IconData::Svg(Cow::Owned(colorize_monochrome_svg(&svg, color)))
        }
        (data, _) => data,
    }
}

/// A freedesktop SVG's `currentColor` filled in with `color` through
/// [`fill_current_color`] where the document sets no `color` of its own, as
/// a rasteriser would otherwise draw it black; an SVG that sets its own (a
/// Breeze `current-color-scheme` stylesheet), one without `currentColor`, a
/// raster, or no `color`, unchanged.
fn fill_freedesktop_svg_data(data: IconData, color: Option<Rgba>) -> IconData {
    match (data, color) {
        (IconData::Svg(svg), Some(color)) => {
            let filled = match fill_current_color(&svg, color) {
                Cow::Owned(filled) => Some(filled),
                Cow::Borrowed(_) => None,
            };
            IconData::Svg(filled.map_or(svg, Cow::Owned))
        }
        (data, _) => data,
    }
}

/// Colour a monochrome SVG icon in its bytes.
///
/// A rasteriser given no `color` draws an implicit fill and an unresolved
/// `currentColor` black, and a draw-time tint multiplies, which leaves black
/// black — so a Material or Lucide icon would draw black on a dark scheme.
/// This writes `color` into the bytes instead, with gpui's algorithm:
///
/// 1. `currentColor` becomes the colour's `#rrggbb` (Lucide-style SVGs);
/// 2. explicit black fills — `fill="black"`, `fill="#000000"`, `fill="#000"` —
///    and explicit black strokes — `stroke="black"`, `stroke="#000000"`,
///    `stroke="#000"` — become the hex (third-party SVGs with hardcoded black);
/// 3. where none of those occurs, `fill="#rrggbb"` is injected into a root
///    `<svg>` tag that has no `fill=` attribute (Material-style SVGs), before
///    the `/` of a self-closing tag.
///
/// Bytes that are not UTF-8 come back unchanged (a lossy replacement would
/// corrupt them). The colour's alpha is discarded: an SVG `fill` or `stroke`
/// attribute takes opaque hex. Not handled, as in gpui: CSS inline styles
/// (`style="fill:black"`), `fill="rgb(0,0,0)"`, and explicit black on a child
/// element when the root tag has another fill. For monochrome icon sets; a
/// full-colour SVG should not be passed through it.
#[must_use]
pub fn colorize_monochrome_svg(svg: &[u8], color: Rgba) -> Vec<u8> {
    let hex = format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b);

    let Ok(svg_str) = std::str::from_utf8(svg) else {
        return svg.to_vec();
    };

    // 1. currentColor
    let replaced = if svg_str.contains("currentColor") {
        svg_str.replace("currentColor", &hex)
    } else {
        svg_str.to_owned()
    };

    // 2. explicit black fills and strokes
    let fill_hex = format!("fill=\"{hex}\"");
    let replaced = replaced
        .replace("fill=\"black\"", &fill_hex)
        .replace("fill=\"#000000\"", &fill_hex)
        .replace("fill=\"#000\"", &fill_hex);
    let stroke_hex = format!("stroke=\"{hex}\"");
    let replaced = replaced
        .replace("stroke=\"black\"", &stroke_hex)
        .replace("stroke=\"#000000\"", &stroke_hex)
        .replace("stroke=\"#000\"", &stroke_hex);

    if replaced != svg_str {
        return replaced.into_bytes();
    }

    // 3. No currentColor and no explicit black: inject a fill into a root
    // <svg> tag that has none (implicit black fill).
    if let Some(pos) = svg_str.find("<svg")
        && let Some(tail) = svg_str.get(pos..)
        && let Some(close) = tail.find('>')
    {
        let tag_end = pos.saturating_add(close);
        if let Some(tag) = svg_str.get(pos..tag_end)
            && !tag.contains("fill=")
        {
            // A self-closing tag: inject before the '/' of '<svg .../>'.
            let is_self_closing = tag_end > 0
                && svg_str
                    .as_bytes()
                    .get(tag_end.saturating_sub(1))
                    .is_some_and(|&b| b == b'/');
            let inject_pos = if is_self_closing {
                tag_end.saturating_sub(1)
            } else {
                tag_end
            };
            if let Some(before) = svg_str.get(..inject_pos)
                && let Some(after) = svg_str.get(inject_pos..)
            {
                let mut result = String::with_capacity(svg_str.len().saturating_add(20));
                result.push_str(before);
                result.push_str(&format!(" fill=\"{hex}\""));
                result.push_str(after);
                return result.into_bytes();
            }
        }
    }

    // A non-black fill and no currentColor: unchanged.
    svg.to_vec()
}

/// Fill a symbolic icon's `currentColor` in with `color`, in its bytes.
///
/// A symbolic freedesktop icon, the `process-working-symbolic` spinner among
/// them, comes back from [`FreedesktopLoader`] without a colour of its own
/// with its foreground as `currentColor`, and a rasteriser resolves
/// `currentColor` from the nearest `color` property, black where the
/// document sets none (usvg 0.45.1 `src/parser/style.rs:170-173`). This
/// replaces every `currentColor` with the colour's `#rrggbb` where the
/// document sets no `color` property or attribute of its own; one that does,
/// as Breeze's stylesheet does per colour scheme, keeps the colour it states.
/// `color` is what `currentColor` stands for: a theme's
/// `defaults.text_color`.
///
/// The bytes come back borrowed and unchanged where they hold no
/// `currentColor`, set a `color` of their own, or are not UTF-8. The
/// colour's alpha is discarded, as in [`colorize_monochrome_svg`]. Unlike
/// that function it touches nothing else, so it suits a full-colour icon
/// too.
///
/// # Examples
///
/// ```
/// use native_theme::Rgba;
/// use native_theme::icons::fill_current_color;
///
/// let text = Rgba::rgb(0x22, 0x22, 0x26);
/// let symbolic = br#"<svg><path fill="currentColor"/></svg>"#;
/// assert_eq!(
///     &*fill_current_color(symbolic, text),
///     br##"<svg><path fill="#222226"/></svg>"##
/// );
///
/// let styled = br#"<svg><style>.a { color: #fcfcfc }</style><path class="a" fill="currentColor"/></svg>"#;
/// assert_eq!(&*fill_current_color(styled, text), &styled[..]);
/// ```
#[must_use]
pub fn fill_current_color(svg: &[u8], color: Rgba) -> Cow<'_, [u8]> {
    match std::str::from_utf8(svg) {
        Ok(text) if text.contains("currentColor") && !sets_color(text) => Cow::Owned(
            text.replace(
                "currentColor",
                &format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b),
            )
            .into_bytes(),
        ),
        _ => Cow::Borrowed(svg),
    }
}

/// Whether an SVG sets a `color` property of its own: the name `color` not
/// preceded by a letter, digit, `-` or `_`, then optional whitespace and `:`
/// (a stylesheet or `style` declaration) or `=` (an attribute), anywhere in
/// it. usvg takes `color` from the nearest ancestor that sets it, so the
/// test is over the whole document. `stop-color`, `flood-color` and the like
/// are other properties, and an `id` or `class` named `color` is no
/// property.
fn sets_color(svg: &str) -> bool {
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

/// The icon base directories, in the Icon Theme Specification's order
/// (version 0.13, *Directory Layout*: "$HOME/.icons (for backwards
/// compatibility), in $XDG_DATA_DIRS/icons and in /usr/share/pixmaps (in
/// that order)"), with `$XDG_DATA_HOME`, which the XDG Base Directory
/// Specification ranks above every `$XDG_DATA_DIRS` entry, first among the
/// data dirs: `~/.icons`, `$XDG_DATA_HOME/icons`, then each `$XDG_DATA_DIRS`
/// entry's `icons`, keeping those that exist. These are the directories
/// freedesktop-icons 0.4.0 searches, without the `pixmaps` directory beside
/// each `icons` one, which no UI icon may come from.
///
/// This order decides what native-theme reads itself: the `index.theme`
/// that says whether a theme is installed and what it inherits is a
/// theme's first one here ([`theme_index`]), so a copy the user installed
/// wins over the system's. freedesktop-icons searches the same directories
/// data dirs first and `~/.icons` last
/// (`freedesktop-icons-0.4.0/src/theme/paths.rs:14-33`), reads every copy
/// of a theme through the first `index.theme` in that order
/// (`src/theme/mod.rs:169-177`, `:197-215`) and takes an icon from the
/// first copy that has it (`src/lib.rs:297-305`): for the icons of a theme
/// installed both ways, the system copy comes first, an order this crate
/// does not control.
///
/// Read once per process, as freedesktop-icons reads its own list once.
#[cfg(target_os = "linux")]
pub(crate) fn icon_base_dirs() -> &'static [std::path::PathBuf] {
    static DIRS: std::sync::OnceLock<Vec<std::path::PathBuf>> = std::sync::OnceLock::new();
    DIRS.get_or_init(|| {
        icon_base_dirs_in(
            std::env::home_dir().as_deref(),
            std::env::var_os("XDG_DATA_HOME").as_deref(),
            std::env::var_os("XDG_DATA_DIRS").as_deref(),
        )
    })
}

/// [`icon_base_dirs`] for the home directory `home` and the
/// `$XDG_DATA_HOME` (`data_home`) and `$XDG_DATA_DIRS` (`data_dirs`)
/// values, read the way freedesktop-icons' `xdg` 2.5.2 reads them
/// (`xdg-2.5.2/src/base_directories.rs:268-309`): a relative or unset
/// `$XDG_DATA_HOME` is `~/.local/share`, relative `$XDG_DATA_DIRS` entries
/// are dropped, and none left is `/usr/local/share:/usr/share`. Without a
/// home directory `xdg` gives no data dirs at all, and there is no
/// `~/.icons` either.
#[cfg(target_os = "linux")]
pub(crate) fn icon_base_dirs_in(
    home: Option<&std::path::Path>,
    data_home: Option<&std::ffi::OsStr>,
    data_dirs: Option<&std::ffi::OsStr>,
) -> Vec<std::path::PathBuf> {
    use std::path::PathBuf;

    let Some(home) = home else {
        return Vec::new();
    };
    let mut dirs: Vec<PathBuf> = std::iter::once(home.join(".icons"))
        .chain(
            xdg_data_home(data_home, Some(home))
                .into_iter()
                .chain(xdg_data_dirs(data_dirs))
                .map(|dir| dir.join("icons")),
        )
        .collect();
    dirs.retain(|dir| dir.exists());
    dirs
}

/// The data dirs `$XDG_DATA_DIRS` (`var`) names: its absolute entries, or
/// `/usr/local/share` and `/usr/share` when none is left, as `xdg` 2.5.2
/// and Qt (`qstandardpaths_unix.cpp:370-399` at v6.11.2) read it.
#[cfg(target_os = "linux")]
pub(crate) fn xdg_data_dirs(var: Option<&std::ffi::OsStr>) -> Vec<std::path::PathBuf> {
    use std::path::PathBuf;

    var.map(|dirs| {
        std::env::split_paths(dirs)
            .filter(|dir| dir.is_absolute())
            .collect::<Vec<_>>()
    })
    .filter(|dirs| !dirs.is_empty())
    .unwrap_or_else(|| {
        vec![
            PathBuf::from("/usr/local/share"),
            PathBuf::from("/usr/share"),
        ]
    })
}

/// The data home `$XDG_DATA_HOME` (`var`) names: itself when absolute, else
/// `<home>/.local/share`, as `xdg` 2.5.2 and Qt
/// (`qstandardpaths_unix.cpp:227-232` at v6.11.2) read it; `None` without
/// either.
#[cfg(target_os = "linux")]
pub(crate) fn xdg_data_home(
    var: Option<&std::ffi::OsStr>,
    home: Option<&std::path::Path>,
) -> Option<std::path::PathBuf> {
    var.map(std::path::PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| home.map(|home| home.join(".local/share")))
}

/// Whether `name` can name a theme directory: one plain path component,
/// so joining it to a base dir stays inside that base dir.
#[cfg(target_os = "linux")]
pub(crate) fn is_theme_name(name: &str) -> bool {
    use std::path::{Component, Path};

    let mut components = Path::new(name).components();
    matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(_)), None)
    )
}

/// The `index.theme` of `theme` in `bases`: its first readable one in
/// their order, the one the Icon Theme Specification reads ("The first
/// index.theme found while searching the base directories in order is
/// used", *Directory Layout*). `None` for a name that is no theme directory.
#[cfg(target_os = "linux")]
pub(crate) fn theme_index(theme: &str, bases: &[std::path::PathBuf]) -> Option<String> {
    if !is_theme_name(theme) {
        return None;
    }
    bases
        .iter()
        .find_map(|base| std::fs::read_to_string(base.join(theme).join("index.theme")).ok())
}

/// Whether `theme` is an icon theme installed in `bases`: its
/// [`theme_index`] lists `Directories`, a key the specification requires,
/// in its `[Icon Theme]` group. A cursor theme (`breeze_cursors`) or one
/// that only inherits (`default`) lists none.
#[cfg(target_os = "linux")]
pub(crate) fn has_theme_index(theme: &str, bases: &[std::path::PathBuf]) -> bool {
    theme_index(theme, bases)
        .is_some_and(|index| ini_value(&index, "Icon Theme", "Directories").is_some())
}

/// The installed icon themes in `bases`: each directory name there that
/// [`has_theme_index`] accepts.
#[cfg(target_os = "linux")]
fn installed_themes_in(bases: &[std::path::PathBuf]) -> std::collections::BTreeSet<String> {
    bases
        .iter()
        .filter_map(|base| std::fs::read_dir(base).ok())
        .flatten()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| has_theme_index(name, bases))
        .collect()
}

/// The installed icon themes in [`icon_base_dirs`] ([`installed_themes_in`]),
/// read once, at the process's first call of
/// [`is_freedesktop_theme_available`], [`list_freedesktop_themes`] or a
/// freedesktop icon lookup, which all read it: a theme listed or available
/// is one the lookup searches, and a theme installed after this snapshot is
/// none of them until the process restarts.
/// [`invalidate_caches`](crate::detect::invalidate_caches) leaves it, as
/// freedesktop-icons' own list of themes, taken at its first lookup (after
/// this one, `freedesktop-icons-0.4.0/src/theme/mod.rs:15`), cannot be read
/// again.
#[cfg(target_os = "linux")]
pub(crate) fn installed_themes() -> &'static std::collections::BTreeSet<String> {
    static THEMES: std::sync::OnceLock<std::collections::BTreeSet<String>> =
        std::sync::OnceLock::new();
    THEMES.get_or_init(|| installed_themes_in(icon_base_dirs()))
}

/// The value of `key` in the `[section]` group of the INI `content`,
/// trimmed: `None` if the group or key is missing or the value empty.
#[cfg(target_os = "linux")]
pub(crate) fn ini_value<'a>(content: &'a str, section: &str, key: &str) -> Option<&'a str> {
    let mut in_section = false;
    for line in content.lines().map(str::trim) {
        if let Some(group) = line.strip_prefix('[') {
            in_section = group.strip_suffix(']') == Some(section);
            continue;
        }
        if in_section
            && let Some((name, value)) = line.split_once('=')
            && name.trim() == key
            && !value.trim().is_empty()
        {
            return Some(value.trim());
        }
    }
    None
}

/// Check whether a freedesktop icon theme is installed on this system.
///
/// A theme is installed when its first `index.theme` in the icon base
/// directories the freedesktop icon lookup searches (`~/.icons/<theme>/`,
/// `$XDG_DATA_HOME/icons/<theme>/`, then `$XDG_DATA_DIRS/icons/<theme>/`)
/// lists `Directories` in its `[Icon Theme]` group, so a cursor-only theme
/// is not one. The installed themes are read once per process, at the
/// first call of this function, [`list_freedesktop_themes`] or a
/// freedesktop icon lookup: a theme installed later is not available until
/// the process restarts, as the lookup does not search it.
///
/// Always returns `false` on non-Linux platforms.
#[must_use]
pub fn is_freedesktop_theme_available(theme: &str) -> bool {
    #[cfg(target_os = "linux")]
    {
        installed_themes().contains(theme)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = theme;
        false
    }
}

// =============================================================================
// IconSetChoice: user's icon set selection intent
// =============================================================================

/// The user's icon set selection mode.
///
/// Represents the user's intent for which icons to display.  The key
/// invariant: only [`Default`](Self::Default) is re-derived on theme changes.
/// All other variants represent an explicit user choice that is preserved
/// across theme re-applications.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IconSetChoice {
    /// Follow the theme preset's recommendation.
    ///
    /// The `String` is the preset's `icon_theme` name (e.g. "Adwaita"),
    /// used for display ("default (Adwaita)") and for loading when the
    /// icon set is `Freedesktop`.
    ///
    /// This is the ONLY variant that gets overwritten on theme change.
    /// [`default_icon_choice()`] is the constructor that checks the theme
    /// is available (bundled sets are always available, SF Symbols and
    /// Segoe Fluent where their loader is compiled, freedesktop themes are
    /// checked via [`is_freedesktop_theme_available`]) before it returns
    /// this variant. A value built directly carries no such check.
    Default(String),

    /// Use the OS-configured icon theme.
    ///
    /// Resolved at load time via [`system_icon_set()`](crate::detect::system_icon_set).
    /// The display label ("system (breeze-dark)") is computed dynamically
    /// from [`system_icon_theme()`](crate::detect::system_icon_theme),
    /// so it tracks runtime OS theme changes. Where detection fails the
    /// label is short, `system (unavailable)`, as a control's label is:
    /// [`system_icon_theme()`](crate::detect::system_icon_theme) gives the
    /// reason, for a tooltip or a note where there is room for it. The
    /// freedesktop loaders then search no theme for this choice.
    System,

    /// User explicitly picked a specific installed freedesktop icon theme.
    ///
    /// The `String` is the theme directory name (e.g. "char-white",
    /// "breeze", "Papirus").  Loaded via `IconSet::Freedesktop` with
    /// `.theme(name)`.
    Freedesktop(String),

    /// Google Material Symbols (bundled).
    Material,

    /// Lucide Icons (bundled).
    Lucide,
}

impl fmt::Display for IconSetChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default(name) => write!(f, "default ({name})"),
            Self::System => f.write_str(&system_choice_label(&system_icon_theme())),
            Self::Freedesktop(name) => write!(f, "{name}"),
            Self::Material => write!(f, "Material (bundled)"),
            Self::Lucide => write!(f, "Lucide (bundled)"),
        }
    }
}

/// [`IconSetChoice::System`]'s label for the `detected` system icon theme:
/// its name, or `unavailable` where there is none, without the reason, which
/// can run to a reader's whole error.
fn system_choice_label(detected: &crate::Result<String>) -> String {
    match detected {
        Ok(name) => format!("system ({name})"),
        Err(_) => "system (unavailable)".to_string(),
    }
}

impl IconSetChoice {
    /// The effective [`IconSet`] loading mechanism for this choice.
    ///
    /// For [`Default`](Self::Default), returns the theme's icon set
    /// (caller passes it in).
    /// For [`Freedesktop`](Self::Freedesktop), always returns
    /// [`IconSet::Freedesktop`].
    /// For others, returns the corresponding bundled or system set.
    #[must_use]
    pub fn effective_icon_set(&self, theme_icon_set: IconSet) -> IconSet {
        match self {
            Self::Default(_) => theme_icon_set,
            Self::System => system_icon_set(),
            Self::Freedesktop(_) => IconSet::Freedesktop,
            Self::Material => IconSet::Material,
            Self::Lucide => IconSet::Lucide,
        }
    }

    /// The freedesktop theme name to pass to [`FreedesktopLoader::theme`], if any.
    ///
    /// Returns `Some(name)` for [`Default`](Self::Default) and
    /// [`Freedesktop`](Self::Freedesktop) variants.
    /// Returns `None` for [`System`](Self::System), [`Material`](Self::Material),
    /// [`Lucide`](Self::Lucide).
    ///
    /// The caller is responsible for only passing the result to
    /// [`FreedesktopLoader::theme`] when the effective icon set is
    /// [`IconSet::Freedesktop`].  For bundled sets (Material, Lucide),
    /// the theme name is not used by the loader.
    #[must_use]
    pub fn freedesktop_theme(&self) -> Option<&str> {
        match self {
            Self::Default(name) | Self::Freedesktop(name) => Some(name),
            Self::System | Self::Material | Self::Lucide => None,
        }
    }

    /// Whether this choice should be re-derived when the theme changes.
    ///
    /// Only [`Default`](Self::Default) follows the preset.  All others
    /// are explicit user choices that must be preserved.
    #[must_use]
    pub fn follows_preset(&self) -> bool {
        matches!(self, Self::Default(_))
    }
}

/// Determine the default icon set choice for a theme.
///
/// When the TOML specifies `icon_theme` (`Some`) and the theme is
/// available, returns
/// [`IconSetChoice::Default(icon_theme)`](IconSetChoice::Default). Material
/// and Lucide are always available; SF Symbols only on macOS and Segoe
/// Fluent only on Windows, each with the `system-icons` feature, where its
/// loader is compiled; a freedesktop theme when
/// [`is_freedesktop_theme_available`] says so.
///
/// When the TOML does not specify `icon_theme` (`None`), or the
/// specified set or theme is not available, returns
/// [`IconSetChoice::System`].
#[must_use]
pub fn default_icon_choice(icon_set: IconSet, icon_theme: Option<&str>) -> IconSetChoice {
    let Some(theme) = icon_theme else {
        return IconSetChoice::System;
    };
    // All five IconSet variants listed explicitly -- no wildcard.
    // Adding a new variant produces a compiler error, forcing the author
    // to decide whether the new set's theme is "always available" or
    // needs a runtime check.
    let available = match icon_set {
        IconSet::Material | IconSet::Lucide => true,
        IconSet::Freedesktop => is_freedesktop_theme_available(theme),
        IconSet::SfSymbols => cfg!(all(target_os = "macos", feature = "system-icons")),
        IconSet::SegoeFluent => cfg!(all(target_os = "windows", feature = "system-icons")),
    };
    if available {
        IconSetChoice::Default(theme.to_string())
    } else {
        IconSetChoice::System
    }
}

/// List installed freedesktop icon themes.
///
/// The themes [`is_freedesktop_theme_available`] reports installed, from
/// the same snapshot: each theme whose first `index.theme` in the icon base
/// directories the freedesktop icon lookup searches (`~/.icons/`,
/// `$XDG_DATA_HOME/icons/`, then `$XDG_DATA_DIRS/icons/`) lists
/// `Directories` in its `[Icon Theme]` group (the Icon Theme
/// Specification requires the key), which leaves out cursor-only themes and
/// themes that only inherit, such as `default`. The snapshot is taken once
/// per process, so a theme installed after it is not listed until the
/// process restarts.
///
/// Excludes `hicolor` (the specification's shared theme for application
/// icons, which native-theme's icon lookup never falls back to). Returns a
/// sorted list of theme directory names; a directory that cannot be read is
/// skipped. Returns an empty `Vec` on non-Linux platforms.
///
/// Note: themes installed via Flatpak or Snap may reside outside
/// standard XDG paths and will not be discovered.
#[must_use]
pub fn list_freedesktop_themes() -> Vec<String> {
    #[cfg(target_os = "linux")]
    {
        installed_themes()
            .iter()
            .filter(|theme| *theme != "hicolor")
            .cloned()
            .collect()
    }
    #[cfg(not(target_os = "linux"))]
    {
        Vec::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_icon_tests {
    use super::*;

    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn undetected() -> crate::Result<String> {
        Err(crate::Error::PlatformUnsupported {
            platform: "the test's failing detection",
        })
    }

    /// With no theme set and no icon theme detected, nothing loads: no
    /// theme stands in for the one that could not be detected.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn freedesktop_loader_without_a_detected_theme_loads_nothing() {
        let custom: &dyn IconProvider = &IconRole::ActionCopy;
        for id in [
            IconId::Role(IconRole::ActionCopy),
            IconId::Name("edit-copy"),
            IconId::Custom(custom),
        ] {
            assert!(
                FreedesktopLoader::new(id).load_with(undetected).is_none(),
                "an icon loaded though no icon theme was detected"
            );
        }
    }

    /// A custom provider's own freedesktop SVG depends on no theme, so it
    /// still loads where no icon theme was detected; only the lookup of its
    /// freedesktop name in a theme is skipped.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn freedesktop_loader_without_a_detected_theme_gives_a_providers_own_svg() {
        #[derive(Debug)]
        struct AppIcon;
        impl IconProvider for AppIcon {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                Some("edit-copy")
            }
            fn icon_svg(&self, set: IconSet) -> Option<Cow<'static, [u8]>> {
                (set == IconSet::Freedesktop).then_some(Cow::Borrowed(b"<svg>app</svg>"))
            }
        }

        let provider: &dyn IconProvider = &AppIcon;
        assert_eq!(
            FreedesktopLoader::new(provider).load_with(undetected),
            Some(IconData::Svg(Cow::Borrowed(b"<svg>app</svg>"))),
            "the provider's own SVG did not load without a detected icon theme"
        );
    }

    /// A generated icon enum (native-theme-build) is handed to a loader by
    /// reference, as its README shows: `MaterialLoader::new(&AppIcon::X)`.
    #[test]
    #[cfg(feature = "material-icons")]
    fn a_provider_is_passed_to_a_loader_by_reference() {
        #[derive(Debug)]
        struct AppIcon;
        impl IconProvider for AppIcon {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, set: IconSet) -> Option<Cow<'static, [u8]>> {
                (set == IconSet::Material).then_some(Cow::Borrowed(b"<svg>app</svg>"))
            }
        }

        assert_eq!(
            MaterialLoader::new(&AppIcon).load(),
            Some(IconData::Svg(Cow::Borrowed(b"<svg>app</svg>"))),
            "a provider passed by reference did not load its own SVG"
        );
    }

    /// An explicit theme needs no detection: the detector never runs.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn freedesktop_loader_with_a_theme_does_not_detect() {
        let runs = std::cell::Cell::new(0);
        let _ = FreedesktopLoader::new("edit-copy")
            .theme("Adwaita")
            .load_with(|| {
                runs.set(runs.get() + 1);
                undetected()
            });
        assert_eq!(runs.get(), 0, "an explicit theme ran the detection");
    }

    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn freedesktop_loader_theme_override_honored_for_name_lookup() {
        // Regression pin for Phase 93-03 silent-ignore bug (Plan 93-09 fix).
        // The old IconLoader dispatched IconId::Name to a path that silently
        // dropped `.theme()`. FreedesktopLoader has a single `load()` method
        // that honors `.theme()` uniformly across Role/Name/Custom.
        //
        // Requires the Adwaita icon theme installed on the test host.
        let result = FreedesktopLoader::new("format-text-rich")
            .theme("Adwaita")
            .size(24)
            .load();
        assert!(
            result.is_some(),
            "theme('Adwaita') must resolve 'format-text-rich' in Adwaita, regardless of system theme"
        );
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_material_returns_svg() {
        let result = MaterialLoader::new(IconRole::ActionCopy).load();
        assert!(result.is_some(), "material ActionCopy should return Some");
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                let s = String::from_utf8_lossy(cow);
                assert!(s.contains("<svg"), "should contain SVG data");
            }
            _ => panic!("expected IconData::Svg for bundled material icon"),
        }
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn load_icon_lucide_returns_svg() {
        let result = LucideLoader::new(IconRole::ActionCopy).load();
        assert!(result.is_some(), "lucide ActionCopy should return Some");
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                let s = String::from_utf8_lossy(cow);
                assert!(s.contains("<svg"), "should contain SVG data");
            }
            _ => panic!("expected IconData::Svg for bundled lucide icon"),
        }
    }

    /// A freedesktop icon the system's theme lacks, or asked of a theme that
    /// is not installed, is `None`: Material's icon does not stand in. The
    /// system's theme `lacking` is a fixture in a child process
    /// ([`native_theme_test_fixtures`]) whose one icon, `dialog-error`, loads,
    /// so the `None` is a lookup's: it has no `edit-copy`.
    #[test]
    #[cfg(all(
        target_os = "linux",
        feature = "system-icons",
        feature = "material-icons"
    ))]
    fn load_icon_unknown_theme_no_cross_set_fallback() -> std::io::Result<()> {
        const NAME: &str = "icons::load_icon_tests::load_icon_unknown_theme_no_cross_set_fallback";
        const DIALOG_ERROR: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><circle fill="#da4453" cx="8" cy="8" r="7"/></svg>"##;
        if native_theme_test_fixtures::in_child() {
            assert_eq!(
                FreedesktopLoader::new(IconRole::DialogError).load(),
                Some(IconData::Svg(Cow::Borrowed(DIALOG_ERROR.as_bytes()))),
                "the system's theme is looked up"
            );
            assert!(MaterialLoader::new(IconRole::ActionCopy).load().is_some());
            assert_eq!(
                FreedesktopLoader::new(IconRole::ActionCopy).load(),
                None,
                "the system's theme lacks edit-copy"
            );
            assert_eq!(
                FreedesktopLoader::new(IconRole::ActionCopy)
                    .theme("native-theme-no-such-theme")
                    .load(),
                None,
                "the theme given is not installed"
            );
            return Ok(());
        }
        let child = native_theme_test_fixtures::Child::kde("lacking");
        native_theme_test_fixtures::run(NAME, child, |icons| {
            native_theme_test_fixtures::write_scalable_theme(
                icons,
                "lacking",
                &[("dialog-error", DIALOG_ERROR)],
            )
        })
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_all_roles_material() {
        // Material has 42 of 42 roles mapped, all return Some
        let mut some_count = 0;
        for role in IconRole::ALL.iter().copied() {
            if MaterialLoader::new(role).load().is_some() {
                some_count += 1;
            }
        }
        // bundled_icon_svg covers all 42 roles for Material
        assert_eq!(
            some_count, 42,
            "Material should cover all 42 roles via bundled SVGs"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn load_icon_all_roles_lucide() {
        let mut some_count = 0;
        for role in IconRole::ALL.iter().copied() {
            if LucideLoader::new(role).load().is_some() {
                some_count += 1;
            }
        }
        // bundled_icon_svg covers all 42 roles for Lucide
        assert_eq!(
            some_count, 42,
            "Lucide should cover all 42 roles via bundled SVGs"
        );
    }

    #[test]
    fn freedesktop_loader_stores_the_size_and_colour() {
        let loader = FreedesktopLoader::new("edit-copy");
        assert_eq!((loader.size, loader.fg_color), (24, None));
        let loader = loader.size(48).color(Rgba::new(10, 20, 30, 40));
        assert_eq!(loader.size, 48);
        assert_eq!(loader.fg_color, Some([10, 20, 30]), "the alpha is ignored");
        assert_eq!(loader.color_opt(None).fg_color, None);
    }

    /// Each system loader's size is clamped to `1..=MAX_ICON_SIZE`.
    #[test]
    fn the_system_loaders_clamp_the_size() {
        for (size, clamped) in [
            (0, 1),
            (48, 48),
            (MAX_ICON_SIZE, MAX_ICON_SIZE),
            (MAX_ICON_SIZE + 1, MAX_ICON_SIZE),
            (70_000, MAX_ICON_SIZE),
            (u32::MAX, MAX_ICON_SIZE),
        ] {
            assert_eq!(
                FreedesktopLoader::new("edit-copy").size(size).size,
                clamped,
                "freedesktop {size}"
            );
            assert_eq!(
                SfSymbolsLoader::new(IconRole::ActionCopy).size(size).size,
                Some(clamped),
                "SF Symbols {size}"
            );
            assert_eq!(
                SegoeFluentLoader::new(IconRole::ActionCopy).size(size).size,
                Some(clamped),
                "Segoe {size}"
            );
        }
    }

    #[test]
    fn raster_size_in_bound_is_the_decode_bound() {
        assert!(!raster_size_in_bound(0));
        assert!(raster_size_in_bound(1));
        assert!(raster_size_in_bound(4096));
        assert!(!raster_size_in_bound(4097));
        assert!(!raster_size_in_bound(u32::MAX));
    }

    #[test]
    fn raster_bytes_in_bound_is_the_decode_bound() {
        assert_eq!(raster_bytes_in_bound(0, 16), None);
        assert_eq!(raster_bytes_in_bound(16, 0), None);
        assert_eq!(raster_bytes_in_bound(3, 2), Some(24));
        assert_eq!(
            raster_bytes_in_bound(8192, 2048),
            Some(MAX_ICON_DECODE_BYTES)
        );
        assert_eq!(raster_bytes_in_bound(8192, 2049), None);
        assert_eq!(raster_bytes_in_bound(u32::MAX, u32::MAX), None);
    }

    #[test]
    fn sf_symbols_loader_stores_the_colour() {
        let loader = SfSymbolsLoader::new(IconRole::ActionCopy).color(Rgba::new(10, 20, 30, 40));
        assert_eq!(loader.fg_color, Some([10, 20, 30]), "the alpha is ignored");
        let loader = loader.color_opt(None);
        assert_eq!(loader.fg_color, None);
        let loader = loader.color_opt(Some(Rgba::rgb(40, 50, 60)));
        assert_eq!(loader.fg_color, Some([40, 50, 60]));
        assert_eq!(SfSymbolsLoader::new(IconRole::ActionCopy).fg_color, None);
    }

    #[test]
    fn sf_symbols_loader_stores_the_size() {
        assert_eq!(SfSymbolsLoader::new(IconRole::ActionCopy).size, None);
        assert_eq!(
            SfSymbolsLoader::new(IconRole::ActionCopy).size(48).size,
            Some(48)
        );
    }

    #[test]
    fn segoe_fluent_loader_stores_the_colour() {
        let loader = SegoeFluentLoader::new(IconRole::ActionCopy).color(Rgba::new(10, 20, 30, 40));
        assert_eq!(loader.fg_color, Some([10, 20, 30]), "the alpha is ignored");
        let loader = loader.color_opt(None);
        assert_eq!(loader.fg_color, None);
        let loader = loader.color_opt(Some(Rgba::rgb(40, 50, 60)));
        assert_eq!(loader.fg_color, Some([40, 50, 60]));
        assert_eq!(SegoeFluentLoader::new(IconRole::ActionCopy).fg_color, None);
    }

    #[test]
    fn segoe_fluent_loader_stores_the_size() {
        assert_eq!(SegoeFluentLoader::new(IconRole::ActionCopy).size, None);
        assert_eq!(
            SegoeFluentLoader::new(IconRole::ActionCopy).size(48).size,
            Some(48)
        );
    }

    /// An SF Symbol loads only on macOS with `system-icons`; elsewhere it is
    /// `None`, and no other set's icon stands in.
    #[test]
    fn an_sf_symbol_loads_only_on_macos_with_system_icons() {
        let result = SfSymbolsLoader::new(IconRole::ActionCopy).load();
        assert_eq!(
            result.is_some(),
            cfg!(all(target_os = "macos", feature = "system-icons"))
        );
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn bundled_icon_load_produces_cow_borrowed() {
        let result = MaterialLoader::new(IconRole::ActionCopy).load();
        assert!(
            matches!(result, Some(IconData::Svg(Cow::Borrowed(_)))),
            "bundled icon should produce Some(IconData::Svg(Cow::Borrowed(_)))"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_system_icon_by_name_tests {
    #[test]
    #[cfg(feature = "material-icons")]
    fn system_icon_by_name_material() {
        let result = super::MaterialLoader::new("content_copy").load();
        assert!(
            result.is_some(),
            "content_copy should be found in Material set"
        );
        assert!(matches!(result.unwrap(), super::IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn system_icon_by_name_lucide() {
        let result = super::LucideLoader::new("copy").load();
        assert!(result.is_some(), "copy should be found in Lucide set");
        assert!(matches!(result.unwrap(), super::IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn system_icon_by_name_unknown_returns_none() {
        let result = super::MaterialLoader::new("nonexistent_xyz").load();
        assert!(result.is_none(), "nonexistent name should return None");
    }

    #[test]
    fn system_icon_by_name_sf_on_linux_returns_none() {
        // On Linux, SfSymbols set is not available (cfg-gated to macOS)
        #[cfg(not(target_os = "macos"))]
        {
            let result = super::SfSymbolsLoader::new("doc.on.doc").load();
            assert!(
                result.is_none(),
                "SF Symbols should return None on non-macOS"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_custom_icon_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_with_icon_role_material() {
        let provider: &dyn IconProvider = &IconRole::ActionCopy;
        let result = MaterialLoader::new(provider).load();
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via material"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn custom_icon_with_icon_role_lucide() {
        let provider: &dyn IconProvider = &IconRole::ActionCopy;
        let result = LucideLoader::new(provider).load();
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via lucide"
        );
    }

    #[test]
    fn custom_icon_no_cross_set_fallback() {
        // Provider that returns None for all sets -- should NOT fall back
        #[derive(Debug)]
        struct NullProvider;
        impl IconProvider for NullProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
                None
            }
        }

        let provider: &dyn IconProvider = &NullProvider;
        let result = MaterialLoader::new(provider).load();
        assert!(
            result.is_none(),
            "NullProvider should return None (no cross-set fallback)"
        );
    }

    /// A provider that names and draws its icon for Material only has no
    /// freedesktop icon: its Material name and SVG do not stand in.
    #[test]
    fn a_custom_freedesktop_icon_does_not_take_the_providers_material_icon() {
        #[derive(Debug)]
        struct MaterialOnly;
        impl IconProvider for MaterialOnly {
            fn icon_name(&self, set: IconSet) -> Option<&str> {
                (set == IconSet::Material).then_some("content_copy")
            }
            fn icon_svg(&self, set: IconSet) -> Option<Cow<'static, [u8]>> {
                (set == IconSet::Material).then_some(Cow::Borrowed(
                    br#"<svg xmlns="http://www.w3.org/2000/svg"/>"#,
                ))
            }
        }

        let provider: &dyn IconProvider = &MaterialOnly;
        #[cfg(feature = "material-icons")]
        assert!(MaterialLoader::new(provider).load().is_some());
        assert_eq!(FreedesktopLoader::new(provider).load(), None);
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_via_dyn_dispatch() {
        let boxed: Box<dyn IconProvider> = Box::new(IconRole::ActionCopy);
        let provider: &dyn IconProvider = &*boxed;
        let result = MaterialLoader::new(provider).load();
        assert!(
            result.is_some(),
            "dyn dispatch through Box<dyn IconProvider> should work"
        );
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_bundled_svg_fallback() {
        // Provider that returns None from icon_name but Some from icon_svg
        #[derive(Debug)]
        struct SvgOnlyProvider;
        impl IconProvider for SvgOnlyProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
                Some(Cow::Borrowed(b"<svg>test</svg>"))
            }
        }

        let provider: &dyn IconProvider = &SvgOnlyProvider;
        let result = MaterialLoader::new(provider).load();
        assert!(
            result.is_some(),
            "provider with icon_svg should return Some"
        );
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                assert_eq!(cow.as_ref(), b"<svg>test</svg>");
            }
            _ => panic!("expected IconData::Svg"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod runtime_set_icon_tests {
    use super::*;

    // Test fixture colour, not a theme value.
    const COLOR: Rgba = Rgba::rgb(0x12, 0x34, 0x56);

    fn svg_text(data: IconData) -> String {
        match data {
            IconData::Svg(svg) => String::from_utf8(svg.into_owned()).unwrap(),
            other => panic!("expected IconData::Svg, got {other:?}"),
        }
    }

    #[derive(Debug)]
    struct SvgOnlyProvider;
    impl IconProvider for SvgOnlyProvider {
        fn icon_name(&self, _set: IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
            Some(Cow::Borrowed(
                b"<svg viewBox=\"0 0 24 24\"><path d=\"M0 0\"/></svg>",
            ))
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_role_comes_back_in_the_colour() {
        let svg = svg_text(
            load_custom_icon(&IconRole::ActionCopy, IconSet::Material, Some(COLOR), None).unwrap(),
        );
        assert!(
            svg.contains("#123456"),
            "colour written into the bytes: {svg}"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_role_comes_back_in_the_colour() {
        let svg = svg_text(
            load_custom_icon(&IconRole::ActionCopy, IconSet::Lucide, Some(COLOR), None).unwrap(),
        );
        assert!(
            svg.contains("#123456"),
            "colour written into the bytes: {svg}"
        );
        assert!(!svg.contains("currentColor"), "{svg}");
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn no_colour_leaves_the_bundled_bytes() {
        let loaded =
            load_custom_icon(&IconRole::ActionCopy, IconSet::Material, None, None).unwrap();
        assert_eq!(
            loaded,
            MaterialLoader::new(IconRole::ActionCopy).load().unwrap()
        );
    }

    #[test]
    fn provider_svg_comes_back_in_the_colour() {
        for set in [
            IconSet::Freedesktop,
            IconSet::SfSymbols,
            IconSet::SegoeFluent,
            IconSet::Material,
            IconSet::Lucide,
        ] {
            let svg = svg_text(load_custom_icon(&SvgOnlyProvider, set, Some(COLOR), None).unwrap());
            assert!(svg.contains("fill=\"#123456\""), "{set:?}: {svg}");
        }
    }

    /// A provider whose freedesktop name is `edit-copy`, with `self.0` as its
    /// own SVG.
    #[derive(Debug)]
    struct EditCopy(Option<&'static [u8]>);
    impl IconProvider for EditCopy {
        fn icon_name(&self, _set: IconSet) -> Option<&str> {
            Some("edit-copy")
        }
        fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
            self.0.map(Cow::Borrowed)
        }
    }

    /// In an icon theme that is not installed the provider's name finds
    /// nothing, and its own SVG, where it has one, is the icon. On every
    /// host: the theme given is the only one searched.
    #[test]
    fn a_custom_icon_missing_from_the_icon_theme_given_is_the_providers_own() {
        let missing = Some("native-theme-no-such-theme");
        assert!(
            load_custom_icon_in(&EditCopy(None), IconSet::Freedesktop, missing, None, None)
                .is_none()
        );
        let own = b"<svg viewBox=\"0 0 24 24\"><path d=\"M0 0\"/></svg>";
        assert_eq!(
            load_custom_icon_in(
                &EditCopy(Some(own)),
                IconSet::Freedesktop,
                missing,
                None,
                None
            ),
            Some(IconData::Svg(Cow::Borrowed(own)))
        );
    }

    /// A custom icon is loaded at a size clamped to `1..=MAX_ICON_SIZE`: a
    /// freedesktop lookup at 99 999 is made at 512, so it takes the theme's
    /// 512 directory, not its 1024 one, and one at 0 is made at 1, taking the
    /// 1 directory, not the 0 one. The theme is a fixture in a child process
    /// ([`native_theme_test_fixtures`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_custom_icon_is_loaded_at_a_clamped_size() -> std::io::Result<()> {
        const NAME: &str =
            "icons::runtime_set_icon_tests::a_custom_icon_is_loaded_at_a_clamped_size";
        const ZERO: &str = r##"<svg xmlns="http://www.w3.org/2000/svg"/>"##;
        const SMALL: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"/>"##;
        const AT_512: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"/>"##;
        const AT_1024: &str =
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"/>"##;
        if native_theme_test_fixtures::in_child() {
            let svg = |svg: &'static str| Some(IconData::Svg(Cow::Borrowed(svg.as_bytes())));
            let load = |size| {
                load_custom_icon_in(
                    &EditCopy(None),
                    IconSet::Freedesktop,
                    None,
                    None,
                    Some(size),
                )
            };
            assert_eq!(load(99_999), svg(AT_512), "asked for 99999");
            assert_eq!(load(MAX_ICON_SIZE + 1), svg(AT_512), "asked for 513");
            assert_eq!(load(1024), svg(AT_512), "asked for 1024");
            assert_eq!(load(0), svg(SMALL), "asked for 0");
            return Ok(());
        }
        let child = native_theme_test_fixtures::Child::kde("sized");
        native_theme_test_fixtures::run(NAME, child, |icons| {
            native_theme_test_fixtures::write_fixed_theme(
                icons,
                "sized",
                &[
                    (0, "edit-copy.svg", ZERO.as_bytes()),
                    (1, "edit-copy.svg", SMALL.as_bytes()),
                    (512, "edit-copy.svg", AT_512.as_bytes()),
                    (1024, "edit-copy.svg", AT_1024.as_bytes()),
                ],
            )
        })
    }

    /// A freedesktop icon is looked up in the icon theme given and no
    /// other: the given theme's own icon is the one loaded, and in one that
    /// is not installed the provider's name finds nothing though the
    /// system's theme has it. The themes are fixtures in a child process
    /// ([`native_theme_test_fixtures`]), the system's `system`.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_custom_icon_is_looked_up_in_the_icon_theme_given() -> std::io::Result<()> {
        const NAME: &str =
            "icons::runtime_set_icon_tests::a_custom_icon_is_looked_up_in_the_icon_theme_given";
        const SYSTEM: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#3daee9" d="M2 2h12v12H2z"/></svg>"##;
        const CHOSEN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#e01b24" d="M2 2h12v12H2z"/></svg>"##;
        if native_theme_test_fixtures::in_child() {
            let svg = |svg: &'static str| Some(IconData::Svg(Cow::Borrowed(svg.as_bytes())));
            let load = |theme| {
                load_custom_icon_in(&EditCopy(None), IconSet::Freedesktop, theme, None, None)
            };
            assert_eq!(load(None), svg(SYSTEM), "the system's theme");
            assert_eq!(load(Some("chosen")), svg(CHOSEN));
            assert_eq!(
                load(Some("native-theme-no-such-theme")),
                None,
                "an icon the theme given lacks is not taken from the system's"
            );
            return Ok(());
        }
        let child = native_theme_test_fixtures::Child::kde("system");
        native_theme_test_fixtures::run(NAME, child, |icons| {
            native_theme_test_fixtures::write_scalable_theme(
                icons,
                "system",
                &[("edit-copy", SYSTEM)],
            )?;
            native_theme_test_fixtures::write_scalable_theme(
                icons,
                "chosen",
                &[("edit-copy", CHOSEN)],
            )
        })
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn a_themeless_set_ignores_the_icon_theme() {
        assert_eq!(
            load_custom_icon_in(
                &IconRole::ActionCopy,
                IconSet::Material,
                Some("breeze"),
                Some(COLOR),
                None
            ),
            load_custom_icon(&IconRole::ActionCopy, IconSet::Material, Some(COLOR), None)
        );
    }

    #[test]
    fn unsized_provider_is_accepted() {
        let provider: &dyn IconProvider = &SvgOnlyProvider;
        assert!(load_custom_icon(provider, IconSet::Material, None, None).is_some());
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_id_in_the_dispatcher_is_coloured() {
        let provider: &dyn IconProvider = &IconRole::ActionCopy;
        let svg = svg_text(
            load_icon_in(
                IconId::Custom(provider),
                IconSet::Material,
                None,
                Some(COLOR),
                None,
            )
            .unwrap(),
        );
        assert!(svg.contains("#123456"), "{svg}");
    }

    #[test]
    fn provider_with_nothing_is_none() {
        #[derive(Debug)]
        struct NullProvider;
        impl IconProvider for NullProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
                None
            }
        }
        assert!(load_custom_icon(&NullProvider, IconSet::Material, Some(COLOR), None).is_none());
    }

    /// The SVG's text; empty for an icon that is not an SVG, which then
    /// fails the comparison.
    fn svg_of(data: IconData) -> String {
        match data {
            IconData::Svg(svg) => String::from_utf8_lossy(&svg).into_owned(),
            _ => String::new(),
        }
    }

    /// A freedesktop SVG whose `currentColor` nothing in the document
    /// resolves is filled in with the colour asked for (M5-04): a rasteriser
    /// draws an unset `currentColor` black.
    #[test]
    fn a_freedesktop_svg_s_bare_current_color_is_filled_in_the_colour() {
        let bare = br#"<svg xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M0 0h16v16H0z"/></svg>"#;
        let filled = fill_freedesktop_svg_data(IconData::Svg(Cow::Borrowed(bare)), Some(COLOR));
        assert_eq!(
            svg_of(filled),
            r##"<svg xmlns="http://www.w3.org/2000/svg"><path fill="#123456" d="M0 0h16v16H0z"/></svg>"##
        );
    }

    /// A freedesktop SVG whose own stylesheet sets the `color` its
    /// `currentColor` takes (Breeze's `current-color-scheme`), and a
    /// full-colour one, come back as they loaded, colour or not; so does
    /// any icon loaded with no colour.
    #[test]
    fn a_freedesktop_svg_with_its_own_colour_is_left_as_it_is() {
        let styled = br#"<svg xmlns="http://www.w3.org/2000/svg"><style id="current-color-scheme">.ColorScheme-Text { color:#232629; }</style><path class="ColorScheme-Text" style="fill:currentColor" d="M0 0h16v16H0z"/></svg>"#;
        let full_colour = br##"<svg xmlns="http://www.w3.org/2000/svg"><path fill="#3daee9" d="M0 0h16v16H0z"/><path fill="#ffffff" d="M2 2h4v4H2z"/></svg>"##;
        let bare = br#"<svg xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M0 0h16v16H0z"/></svg>"#;
        for (svg, color) in [
            (&styled[..], Some(COLOR)),
            (&full_colour[..], Some(COLOR)),
            (&bare[..], None),
        ] {
            let data = fill_freedesktop_svg_data(IconData::Svg(Cow::Borrowed(svg)), color);
            assert_eq!(svg_of(data), String::from_utf8_lossy(svg));
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn indicator_of_a_themeless_set_ignores_the_theme() {
        assert_eq!(
            load_icon_indicator(IconSet::Material, Some("breeze")),
            load_icon_indicator(IconSet::Material, None)
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod loading_indicator_tests {
    use super::*;

    // === Dispatch tests (through per-set loader API) ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn loading_indicator_lucide_returns_frames() {
        let anim = LucideLoader::load_indicator();
        assert!(anim.is_some(), "lucide should return Some");
        let anim = anim.unwrap();
        assert!(
            matches!(anim, AnimatedIcon::Frames(_)),
            "lucide should be pre-rotated Frames"
        );
        if let AnimatedIcon::Frames(data) = &anim {
            assert_eq!(data.frames().len(), 24);
            assert_eq!(data.frame_duration_ms().get(), 42);
        }
    }

    /// Freedesktop loading_indicator returns Some if the active icon theme
    /// has a `process-working` sprite sheet (e.g. Breeze), None otherwise.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn loading_indicator_freedesktop_depends_on_theme() {
        let anim = FreedesktopLoader::load_indicator(None);
        // Result depends on installed icon theme -- Some if process-working exists
        if let Some(anim) = anim {
            match anim {
                AnimatedIcon::Frames(data) => {
                    assert!(
                        !data.frames().is_empty(),
                        "Frames variant should have at least one frame"
                    );
                }
                AnimatedIcon::Transform(_) => {
                    // Single-frame theme icon with Spin -- valid result
                }
            }
        }
    }

    /// Freedesktop spinner depends on platform and icon theme.
    #[test]
    fn loading_indicator_freedesktop_does_not_panic() {
        let _result = FreedesktopLoader::load_indicator(None);
    }

    // === Direct spinner construction tests (any platform) ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_spinner_is_frames() {
        let anim = crate::spinners::lucide_spinner();
        assert!(
            matches!(anim, AnimatedIcon::Frames(_)),
            "lucide should be pre-rotated Frames"
        );
    }
}

// === New builder API tests ===

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod icon_loader_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_basic_role() {
        let icon = MaterialLoader::new(IconRole::ActionCopy).load();
        assert!(icon.is_some());
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_by_name() {
        let icon = MaterialLoader::new("content_copy").load();
        assert!(icon.is_some());
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_custom_provider() {
        let provider: &dyn IconProvider = &IconRole::ActionCopy;
        let icon = MaterialLoader::new(provider).load();
        assert!(icon.is_some());
    }

    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn freedesktop_loader_builder_chain_compiles() {
        // Smoke test: the builder chain constructs without runtime errors.
        // Size propagates through; the actual load result depends on the
        // system theme having 'document-save' at size 48.
        let _ = FreedesktopLoader::new(IconRole::ActionSave)
            .size(48)
            .color(Rgba::rgb(0, 0, 0))
            .theme("Adwaita")
            .load();
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_load_indicator() {
        let anim = MaterialLoader::load_indicator();
        assert!(anim.is_some());
    }

    #[test]
    fn load_icon_free_fn_dispatches() {
        let role = IconRole::ActionCopy;
        for (set, loaded) in [
            (IconSet::Freedesktop, FreedesktopLoader::new(role).load()),
            (IconSet::SfSymbols, SfSymbolsLoader::new(role).load()),
            (IconSet::SegoeFluent, SegoeFluentLoader::new(role).load()),
            (IconSet::Material, MaterialLoader::new(role).load()),
            (IconSet::Lucide, LucideLoader::new(role).load()),
        ] {
            assert_eq!(load_icon(role, set), loaded, "{set}");
        }
        #[cfg(all(feature = "material-icons", feature = "lucide-icons"))]
        assert_ne!(
            load_icon(role, IconSet::Material),
            load_icon(role, IconSet::Lucide)
        );
    }

    #[test]
    fn load_icon_indicator_free_fn_dispatches() {
        // SfSymbols and SegoeFluent have no spinner primitive.
        assert!(load_icon_indicator(IconSet::SfSymbols, None).is_none());
        assert!(load_icon_indicator(IconSet::SegoeFluent, None).is_none());
    }
}

#[cfg(all(test, feature = "svg-rasterize", feature = "lucide-icons"))]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod spinner_rasterize_tests {
    use super::*;

    #[test]
    fn lucide_spinner_icon_rasterizes() {
        let anim = crate::spinners::lucide_spinner();
        if let AnimatedIcon::Frames(data) = &anim {
            let first = data.frames().first();
            if let IconData::Svg(cow) = first {
                let result = crate::rasterize::rasterize_svg(cow, 24);
                assert!(result.is_ok(), "lucide loader should rasterize");
                if let Ok(IconData::Rgba(rgba)) = &result {
                    assert!(
                        rgba.data().iter().any(|&b| b != 0),
                        "lucide loader rasterized to empty image"
                    );
                }
            } else {
                panic!("lucide spinner frame should be Svg");
            }
        } else {
            panic!("lucide spinner should be Frames");
        }
    }
}

// =============================================================================
// IconSetChoice tests
// =============================================================================

#[cfg(test)]
mod icon_set_choice_tests {
    use super::*;

    #[test]
    fn system_choice_names_the_detected_theme() {
        assert_eq!(
            system_choice_label(&Ok("breeze-dark".to_string())),
            "system (breeze-dark)"
        );
    }

    #[test]
    fn system_choice_shows_a_failed_detection_as_unavailable() {
        let label = system_choice_label(&Err(crate::Error::PlatformUnsupported {
            platform: "the test's failing detection",
        }));
        assert_eq!(label, "system (unavailable)");
    }

    /// The label is short whatever the failure: no reader's error, which
    /// can run to several paths and a command line, reaches it.
    /// `system_icon_theme()` gives the reason.
    #[test]
    fn a_failed_detections_label_leaves_the_reason_out() {
        let failures = [
            crate::Error::PlatformUnsupported {
                platform: "the test's failing detection",
            },
            crate::Error::WatchUnavailable {
                reason: "a reason long enough to fill a drop-down twice over",
            },
        ];
        for failure in failures {
            let reason = failure.to_string();
            let label = system_choice_label(&Err(failure));
            assert_eq!(label, "system (unavailable)", "for {reason:?}");
            assert!(!label.contains(&reason), "{label:?} quotes {reason:?}");
        }
    }

    #[test]
    fn test_icon_set_choice_display_default() {
        let choice = IconSetChoice::Default("Adwaita".to_string());
        assert_eq!(choice.to_string(), "default (Adwaita)");
    }

    #[test]
    fn test_icon_set_choice_display_material() {
        assert_eq!(IconSetChoice::Material.to_string(), "Material (bundled)");
    }

    #[test]
    fn test_icon_set_choice_display_lucide() {
        assert_eq!(IconSetChoice::Lucide.to_string(), "Lucide (bundled)");
    }

    #[test]
    fn test_icon_set_choice_display_freedesktop() {
        let choice = IconSetChoice::Freedesktop("breeze".to_string());
        assert_eq!(choice.to_string(), "breeze");
    }

    #[test]
    fn test_icon_set_choice_follows_preset() {
        assert!(IconSetChoice::Default("Adwaita".to_string()).follows_preset());
        assert!(!IconSetChoice::System.follows_preset());
        assert!(!IconSetChoice::Freedesktop("breeze".to_string()).follows_preset());
        assert!(!IconSetChoice::Material.follows_preset());
        assert!(!IconSetChoice::Lucide.follows_preset());
    }

    #[test]
    fn test_icon_set_choice_effective_icon_set() {
        // Default returns whatever the theme specifies
        let choice = IconSetChoice::Default("Adwaita".to_string());
        assert_eq!(
            choice.effective_icon_set(IconSet::Freedesktop),
            IconSet::Freedesktop
        );
        assert_eq!(
            choice.effective_icon_set(IconSet::Material),
            IconSet::Material
        );

        // System returns the platform's icon set
        let sys = IconSetChoice::System.effective_icon_set(IconSet::Material);
        assert_eq!(sys, system_icon_set());

        // Freedesktop always returns Freedesktop
        let choice = IconSetChoice::Freedesktop("breeze".to_string());
        assert_eq!(
            choice.effective_icon_set(IconSet::Material),
            IconSet::Freedesktop
        );

        // Bundled sets return themselves
        assert_eq!(
            IconSetChoice::Material.effective_icon_set(IconSet::Freedesktop),
            IconSet::Material
        );
        assert_eq!(
            IconSetChoice::Lucide.effective_icon_set(IconSet::Freedesktop),
            IconSet::Lucide
        );
    }

    #[test]
    fn test_icon_set_choice_freedesktop_theme() {
        assert_eq!(
            IconSetChoice::Default("Adwaita".to_string()).freedesktop_theme(),
            Some("Adwaita")
        );
        assert_eq!(
            IconSetChoice::Freedesktop("breeze".to_string()).freedesktop_theme(),
            Some("breeze")
        );
        assert_eq!(IconSetChoice::System.freedesktop_theme(), None);
        assert_eq!(IconSetChoice::Material.freedesktop_theme(), None);
        assert_eq!(IconSetChoice::Lucide.freedesktop_theme(), None);
    }

    #[test]
    fn test_default_icon_choice_none() {
        // When icon_theme is None, always returns System regardless of icon_set
        assert_eq!(
            default_icon_choice(IconSet::Freedesktop, None),
            IconSetChoice::System
        );
        assert_eq!(
            default_icon_choice(IconSet::Material, None),
            IconSetChoice::System
        );
    }

    #[test]
    fn test_default_icon_choice_bundled() {
        // Material and Lucide are always available
        assert_eq!(
            default_icon_choice(IconSet::Material, Some("any-theme")),
            IconSetChoice::Default("any-theme".to_string())
        );
        assert_eq!(
            default_icon_choice(IconSet::Lucide, Some("any-theme")),
            IconSetChoice::Default("any-theme".to_string())
        );
    }

    /// SF Symbols and Segoe Fluent are available only where their loader is
    /// compiled; anywhere else their theme is not taken as the default.
    #[test]
    fn system_sets_are_available_only_where_their_loader_is_compiled() {
        let default =
            |set| default_icon_choice(set, Some("x")) == IconSetChoice::Default("x".into());
        assert_eq!(
            default(IconSet::SfSymbols),
            cfg!(all(target_os = "macos", feature = "system-icons"))
        );
        assert_eq!(
            default(IconSet::SegoeFluent),
            cfg!(all(target_os = "windows", feature = "system-icons"))
        );
    }

    #[test]
    fn test_list_freedesktop_themes_no_panic() {
        // Just verify it doesn't panic -- result varies by platform
        let themes = list_freedesktop_themes();
        // On Linux, should return a non-empty list on most desktop systems.
        // On other platforms, returns empty vec.
        #[cfg(not(target_os = "linux"))]
        assert!(themes.is_empty());
        // Suppress unused variable warning
        let _ = themes;
    }
}

#[cfg(all(test, target_os = "linux"))]
#[allow(clippy::unwrap_used, reason = "a test fails by panicking")]
mod installed_theme_tests {
    use super::*;
    use std::path::{Path, PathBuf};

    /// A throwaway icon base dir under `share/icons`, removed on drop, and
    /// the home directory `home` beside it.
    struct Bases {
        root: PathBuf,
    }

    impl Bases {
        fn new(tag: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "native-theme-installed-{}-{tag}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(root.join("share/icons")).unwrap();
            Self { root }
        }

        fn icons(&self) -> PathBuf {
            self.root.join("share/icons")
        }

        fn add(&self, base: &Path, theme: &str, index: &str) {
            std::fs::create_dir_all(base.join(theme)).unwrap();
            std::fs::write(base.join(theme).join("index.theme"), index).unwrap();
        }
    }

    impl Drop for Bases {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    /// An icon theme is an `index.theme` whose `[Icon Theme]` group lists
    /// `Directories`: a cursor theme and a theme that only inherits are none,
    /// and the key may have spaces around `=`.
    #[test]
    fn an_installed_theme_lists_directories_in_its_icon_theme_group() {
        let bases = Bases::new("predicate");
        let icons = bases.icons();
        bases.add(
            &icons,
            "icons",
            "[Icon Theme]\nName=icons\nDirectories=actions\n",
        );
        bases.add(
            &icons,
            "spaced",
            "[Icon Theme]\nName=spaced\nDirectories = actions\n",
        );
        bases.add(
            &icons,
            "breeze_cursors",
            "[Icon Theme]\nName=Breeze Cursors\nInherits=breeze\n",
        );
        bases.add(&icons, "default", "[Icon Theme]\nInherits=Adwaita\n");
        bases.add(
            &icons,
            "elsewhere",
            "[Icon Theme]\nName=elsewhere\n\n[Other]\nDirectories=actions\n",
        );
        let dirs = [icons];
        assert!(has_theme_index("icons", &dirs));
        assert!(has_theme_index("spaced", &dirs));
        assert!(!has_theme_index("breeze_cursors", &dirs));
        assert!(!has_theme_index("default", &dirs));
        assert!(!has_theme_index("elsewhere", &dirs));
        assert!(!has_theme_index("missing", &dirs));
        assert_eq!(
            installed_themes_in(&dirs).into_iter().collect::<Vec<_>>(),
            ["icons", "spaced"]
        );
    }

    /// The installed `breeze_cursors` is a cursor theme, not available.
    #[test]
    fn breeze_cursors_is_not_available() {
        if !Path::new("/usr/share/icons/breeze_cursors/index.theme").exists() {
            eprintln!("skipped: breeze_cursors is not installed");
            return;
        }
        assert!(!is_freedesktop_theme_available("breeze_cursors"));
        assert!(!list_freedesktop_themes().contains(&"breeze_cursors".to_string()));
    }

    /// Every listed theme is available, read from the one snapshot.
    #[test]
    fn listed_themes_are_available() {
        for theme in list_freedesktop_themes() {
            assert!(is_freedesktop_theme_available(&theme), "{theme}");
        }
    }

    /// The base dirs in the Icon Theme Specification's order, `~/.icons`
    /// first, then the data home, then each data dir.
    #[test]
    fn icon_base_dirs_put_the_user_s_first() {
        let bases = Bases::new("order");
        let home = bases.root.join("home");
        for dir in [".icons", ".local/share/icons"] {
            std::fs::create_dir_all(home.join(dir)).unwrap();
        }
        let system = bases.root.join("share");
        assert_eq!(
            icon_base_dirs_in(Some(&home), None, Some(system.as_os_str())),
            [
                home.join(".icons"),
                home.join(".local/share/icons"),
                system.join("icons")
            ]
        );
    }

    /// The first `index.theme` in base-dir order is the theme's, as the
    /// specification reads it: a user copy without `Directories` hides a
    /// system copy with them.
    #[test]
    fn the_first_index_theme_decides_whether_a_theme_is_installed() {
        let bases = Bases::new("first-index");
        let user = bases.root.join("home/.icons");
        bases.add(&user, "shadowed", "[Icon Theme]\nName=shadowed\n");
        bases.add(
            &bases.icons(),
            "shadowed",
            "[Icon Theme]\nName=shadowed\nDirectories=actions\n",
        );
        assert!(!has_theme_index("shadowed", &[user.clone(), bases.icons()]));
        assert!(has_theme_index("shadowed", &[bases.icons(), user]));
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod colorize_tests {
    use super::{colorize_monochrome_svg, fill_current_color, sets_color};
    use crate::color::Rgba;
    use std::borrow::Cow;

    const BLUE: Rgba = Rgba::new(51, 102, 204, 255);
    const RED: Rgba = Rgba::new(255, 0, 0, 255);
    const GREEN: Rgba = Rgba::new(20, 184, 82, 255);

    #[test]
    fn colorize_svg_replaces_fill_black() {
        let svg = b"<svg><path fill=\"black\" d=\"M0 0h24v24H0z\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, BLUE)).unwrap();
        assert!(
            !result_str.contains("fill=\"black\""),
            "fill=\"black\" should be replaced, got: {result_str}"
        );
        assert!(
            result_str.contains("fill=\"#3366cc\""),
            "should contain the hex fill, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_replaces_fill_hex_black() {
        let svg = b"<svg><rect fill=\"#000000\" width=\"24\" height=\"24\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            !result_str.contains("#000000"),
            "fill=\"#000000\" should be replaced, got: {result_str}"
        );
        assert!(
            result_str.contains("fill=\"#ff0000\""),
            "should contain the hex fill, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_replaces_fill_short_hex_black() {
        let svg = b"<svg><rect fill=\"#000\" width=\"24\" height=\"24\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, GREEN)).unwrap();
        assert!(
            !result_str.contains("fill=\"#000\""),
            "fill=\"#000\" should be replaced, got: {result_str}"
        );
        assert!(
            result_str.contains("fill=\"#14b852\""),
            "should contain the hex fill, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_current_color_still_works() {
        let svg = b"<svg><path stroke=\"currentColor\" d=\"M0 0\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            !result_str.contains("currentColor"),
            "currentColor should be replaced"
        );
        assert!(
            result_str.contains("#ff0000"),
            "should contain the hex colour"
        );
    }

    #[test]
    fn colorize_svg_implicit_black_still_works() {
        // SVG with no fill attribute at all (Material-style)
        let svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            result_str.contains("fill=\"#ff0000\""),
            "should inject fill into root svg tag, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_non_utf8_returns_original() {
        // Non-UTF-8 bytes: valid SVG prefix followed by an invalid byte sequence
        let mut svg = b"<svg><path fill=\"black\" d=\"M0 0\"/>".to_vec();
        svg.push(0xFF); // invalid UTF-8 byte
        svg.extend_from_slice(b"</svg>");
        let result = colorize_monochrome_svg(&svg, RED);
        assert_eq!(result, svg, "non-UTF-8 input should be returned unchanged");
    }

    #[test]
    fn colorize_svg_replaces_stroke_black() {
        let svg = b"<svg><path stroke=\"black\" d=\"M0 0h24\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, BLUE)).unwrap();
        assert!(
            !result_str.contains("stroke=\"black\""),
            "stroke=\"black\" should be replaced, got: {result_str}"
        );
        assert!(
            result_str.contains("stroke=\"#3366cc\""),
            "should contain the hex stroke, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_replaces_stroke_hex_black() {
        let svg = b"<svg><line stroke=\"#000000\" x1=\"0\" y1=\"0\" x2=\"24\" y2=\"24\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            !result_str.contains("#000000"),
            "stroke=\"#000000\" should be replaced"
        );
        assert!(
            result_str.contains("stroke=\"#ff0000\""),
            "should contain the hex stroke, got: {result_str}"
        );
    }

    #[test]
    fn colorize_self_closing_svg_produces_valid_xml() {
        // Self-closing <svg .../> tag: the fill must be injected before '/'
        let svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\" />";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            result_str.contains("fill=\"#ff0000\""),
            "should inject fill, got: {result_str}"
        );
        assert!(
            !result_str.contains("/ fill="),
            "fill must be before '/', got: {result_str}"
        );
        assert!(
            result_str.trim().ends_with("/>"),
            "should remain self-closing, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_with_fill_white_root() {
        // A root fill that is not black is kept, not replaced
        let svg = b"<svg fill=\"white\"><path/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            result_str.contains("fill=\"white\""),
            "fill=\"white\" should be preserved, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_with_fill_none_root() {
        // stroke="black" is replaced even when the root has fill="none"
        let svg = b"<svg fill=\"none\"><path stroke=\"black\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            !result_str.contains("stroke=\"black\""),
            "stroke=\"black\" should be replaced, got: {result_str}"
        );
        assert!(
            result_str.contains("stroke=\"#ff0000\""),
            "should contain the hex stroke, got: {result_str}"
        );
    }

    /// The colour's alpha is discarded: an SVG `fill` takes opaque hex (spec §9.2).
    #[test]
    fn colorize_svg_discards_alpha() {
        let svg = b"<svg><path fill=\"black\"/></svg>";
        let translucent = Rgba::new(255, 0, 0, 40);
        assert_eq!(
            colorize_monochrome_svg(svg, translucent),
            colorize_monochrome_svg(svg, RED)
        );
    }

    #[test]
    fn colorize_svg_replaces_stroke_short_hex_black() {
        let svg = b"<svg><path stroke=\"#000\" d=\"M0 0\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, BLUE)).unwrap();
        assert!(
            result_str.contains("stroke=\"#3366cc\""),
            "stroke=\"#000\" should be replaced, got: {result_str}"
        );
    }

    #[test]
    fn colorize_svg_keeps_a_root_fill_and_injects_none() {
        let svg = b"<svg fill=\"red\"><path d=\"M0 0\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, GREEN)).unwrap();
        // No second fill is injected, since the root has one
        assert!(result_str.contains("fill=\"red\""), "got: {result_str}");
        assert!(!result_str.contains("#14b852"), "got: {result_str}");
    }

    #[test]
    fn colorize_svg_mixed_fill_attributes() {
        // fill="black" is replaced, fill="red" is kept
        let svg = b"<svg><rect fill=\"black\" width=\"10\" height=\"10\"/><rect fill=\"red\" width=\"10\" height=\"10\"/></svg>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, GREEN)).unwrap();
        assert!(
            !result_str.contains("fill=\"black\""),
            "fill=\"black\" should be replaced, got: {result_str}"
        );
        assert!(
            result_str.contains("fill=\"red\""),
            "fill=\"red\" should be preserved, got: {result_str}"
        );
    }

    #[test]
    fn colorize_self_closing_svg_without_a_space_produces_valid_xml() {
        // `<svg .../>` with no space before '/': the fill still goes before it
        let svg = b"<svg xmlns='http://www.w3.org/2000/svg'/>";
        let result_str = String::from_utf8(colorize_monochrome_svg(svg, RED)).unwrap();
        assert!(
            result_str.contains("fill=\"#ff0000\"") && result_str.ends_with("/>"),
            "self-closing SVG should remain valid XML, got: {result_str}"
        );
        assert!(
            !result_str.contains("/ fill="),
            "fill should not be between / and >, got: {result_str}"
        );
    }

    #[test]
    fn fill_current_color_fills_an_unset_current_color() {
        let svg = br#"<svg><path fill="currentColor" stroke="currentColor"/></svg>"#;
        let filled = fill_current_color(svg, Rgba::new(51, 102, 204, 7));
        assert_eq!(
            &*filled,
            br##"<svg><path fill="#3366cc" stroke="#3366cc"/></svg>"##
        );
    }

    #[test]
    fn fill_current_color_borrows_what_it_leaves_alone() {
        let plain = br##"<svg><path fill="#2e3436"/></svg>"##;
        let styled = br#"<svg><style>.ColorScheme-Text { color:#232629; }</style><path class="ColorScheme-Text" fill="currentColor"/></svg>"#;
        let attribute = br##"<svg color="#fff"><path fill="currentColor"/></svg>"##;
        let not_utf8 = b"<svg>\xff currentColor</svg>";
        for svg in [&plain[..], &styled[..], &attribute[..], &not_utf8[..]] {
            assert!(
                matches!(fill_current_color(svg, BLUE), Cow::Borrowed(b) if b == svg),
                "{}",
                String::from_utf8_lossy(svg)
            );
        }
    }

    #[test]
    fn sets_color_reads_only_the_color_property() {
        assert!(!sets_color(r#"<svg><path fill="currentColor"/></svg>"#));
        assert!(sets_color(
            r##"<svg color="#fff"><path fill="currentColor"/></svg>"##
        ));
        assert!(sets_color("<svg><style>.a { color : #fff }</style></svg>"));
        assert!(sets_color(r#"<svg><path style="color:#fff"/></svg>"#));
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
}
