//! What the Icons page's icons report about themselves (spec §3.4).
//!
//! Each icon is its own instance: the set it came from, the name that set
//! gives it, and whether the set has it at all. A set that has none says so
//! and shows the placeholder -- no other set's icon stands in for it.

use gpui::Hsla;
use gpui_component::theme::Theme;

use super::chrome::chrome_icon_note;
use super::{WidgetInfo, claim, px_text};
use crate::demo::{ANIMATED_ICON_SIZE, AnimatedKind, ICON_CELL_SIZE, IconDrawn, IconSizeContext};
use crate::support::ChromeIcon;

/// `c` as the colour an SVG recoloured with it is painted in: the connector
/// writes it into the SVG as `#rrggbb`, which has no alpha
/// (native-theme-gpui icons.rs:1245-1250).
fn opaque(c: Hsla) -> Hsla {
    Hsla { a: 1., ..c }
}

/// The name under an icon: a Label, text_xs.
fn name_label(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.color(claim(
        "name",
        "foreground",
        t.foreground,
        "gpui-component/label.rs:211",
    ))
}

/// What an icon of a gallery is drawn as, and what that paints.
fn drawn(info: WidgetInfo, t: &Theme, set: &str, drawn: IconDrawn, fg: Hsla) -> WidgetInfo {
    match drawn {
        IconDrawn::Builtin => info
            // No colour of its own: the Icon takes the text colour it
            // inherits (icon.rs:170, :219), which the showcase sets on its
            // window.
            .color(claim(
                "icon, inherited foreground",
                "foreground",
                t.foreground,
                "showcase",
            ))
            .not_themeable(
                "color",
                "the inherited text colour unless text_color() sets one (icon.rs, Icon::into_svg)",
            )
            .not_themeable("SVG shapes", "101 in gpui_component::IconName, a compatibility subset: gpui_kit_assets::IconName carries the whole Lucide catalogue (icon.rs, component_icon_names)")
            .instance(
                "size",
                "Medium: size_4, a rem (icon.rs, Icon::into_svg)",
            ),
        IconDrawn::Bundled => info
            .color(claim("icon", "foreground", opaque(fg), "showcase"))
            .not_themeable(
                "icon pixels",
                "the set's own SVG, bundled with native-theme, recoloured by the showcase with foreground, opaque: the connector writes the colour over the SVG's currentColor or black (native-theme-gpui/icons.rs, colorize_svg)",
            )
            .instance(
                "size",
                format!("{}px, the showcase's own", px_text(ICON_CELL_SIZE.as_f32())),
            ),
        IconDrawn::System => info
            .not_themeable("icon pixels", "the icon theme's own files. A symbolic SVG is rasterised with the platform's font colour, which the showcase passes to the loader; a raster or multi-colour icon keeps the colours it ships with (showcase-gpui/support.rs, load_all_icons)")
            .instance(
                "size",
                format!("{}px, the showcase's own", px_text(ICON_CELL_SIZE.as_f32())),
            ),
        IconDrawn::Missing | IconDrawn::Unreadable => info
            .color(claim(
                "missing-icon placeholder",
                "skeleton",
                t.skeleton,
                "showcase",
            ))
            .config("placeholder radius", format!("radius: {}px", t.radius.as_f32()))
            .instance(
                "size",
                format!("{}px, the showcase's own", px_text(ICON_CELL_SIZE.as_f32())),
            ),
    }
    .instance(
        "resolved",
        match drawn {
            IconDrawn::Builtin | IconDrawn::Bundled | IconDrawn::System => {
                format!("Some: {set} has it")
            }
            IconDrawn::Missing => format!(
                "None: {set} has no icon for it, and no other icon theme's icon stands in -- the cell shows the placeholder"
            ),
            IconDrawn::Unreadable => format!(
                "Some: {set} has it, but it did not convert to an image (native-theme-gpui/icons.rs, to_image_source), so the cell shows the placeholder"
            ),
        },
    )
}

/// A cell of the Native Theme Icons grid: native-theme's icon for `role` in
/// `set`, which calls it `name` where the showcase knows the name. Where
/// the set is gpui-component's own (`builtin`), `name` is the asset of the
/// IconName the connector maps the role to.
pub fn role_icon(
    t: &Theme,
    role: &str,
    set: &str,
    builtin: bool,
    name: Option<&str>,
    icon: IconDrawn,
    fg: Hsla,
) -> WidgetInfo {
    let info = WidgetInfo::new("IconRole").variant(role);
    let info = drawn(name_label(info, t), t, set, icon, fg)
        .not_themeable(
            "geometry",
            "none: no geometry:: builder applies to an icon grid. The cell padding and the gap are the showcase's own layout",
        )
        .instance("icon theme", set.to_string());
    match (builtin, name) {
        (true, Some(name)) => info.instance(
            "icon",
            format!("gpui-component's own, {name}: the IconName the connector maps the role to (native-theme-gpui/icons.rs, icon_name)"),
        ),
        (true, None) => info.instance(
            "icon",
            "none: the connector maps the role to no IconName of gpui-component's (native-theme-gpui/icons.rs, icon_name)",
        ),
        (false, Some(name)) => info.instance("icon name", name.to_string()),
        (false, None) => info.instance(
            "icon name",
            format!("none: native-theme names no icon for the role in {set}"),
        ),
    }
}

/// A cell of the gpui-component Icons grid: the icon `set` gives gpui-component's
/// IconName `name`, found by `role` where one maps to it.
pub fn gpui_icon(
    t: &Theme,
    name: &str,
    role: Option<&str>,
    set: &str,
    icon: IconDrawn,
    fg: Hsla,
) -> WidgetInfo {
    let info = WidgetInfo::new("IconName").variant(name);
    let info = drawn(name_label(info, t), t, set, icon, fg)
        .not_themeable(
            "geometry",
            "none: no geometry:: builder applies to an icon grid. The cell padding and the gap are the showcase's own layout",
        )
        .instance("icon theme", set.to_string());
    match (icon, role) {
        (IconDrawn::Builtin, _) => info.instance(
            "lookup",
            "none: gpui-component draws its own icon for the name, from the application's assets (icon.rs, Icon::into_svg)",
        ),
        (_, Some(role)) => info.instance(
            "lookup",
            format!("by IconRole::{role}, the role the showcase maps the name to (showcase-gpui/support.rs, role_for_gpui_icon); a freedesktop theme without an icon for the role is asked for the name the connector gives the icon there instead, never another theme (showcase-gpui/support.rs, load_gpui_icons)"),
        ),
        (_, None) => info.instance(
            "lookup",
            "by name: no IconRole maps to it, so the showcase asks the set for the name the connector gives it there (showcase-gpui/support.rs, load_gpui_icons)",
        ),
    }
}

/// An animated icon of `set`, of `kind`, drawn while gpui's `reduce_motion`
/// is as given. A frame animation's frames are recoloured with `fg` where
/// the set is `bundled`.
pub fn animated_icon(
    t: &Theme,
    set: &str,
    kind: AnimatedKind,
    bundled: bool,
    fg: Hsla,
    reduce_motion: bool,
) -> WidgetInfo {
    let info = WidgetInfo::new("Animated icon");
    let info = match kind {
        AnimatedKind::Frames { count, duration_ms } => {
            let info = info.variant(format!("{set}, frames"));
            let info = if bundled {
                info.color(claim("frames", "foreground", opaque(fg), "showcase"))
                    .not_themeable(
                        "frame pixels",
                        "the set's own spinner icon, bundled with native-theme and turned a step further in each frame, recoloured by the showcase with foreground, opaque: the connector writes the colour over the SVG's currentColor or black (native-theme-gpui/icons.rs, colorize_svg)",
                    )
            } else {
                info.not_themeable(
                    "frame pixels",
                    "the icon theme's own indicator artwork, recoloured with foreground only where it is drawn in currentColor or black (native-theme-gpui/icons.rs, colorize_svg); a multi-colour sprite keeps its colours",
                )
            };
            info.instance("frames", format!("{count}, {duration_ms}ms each"))
                .instance(
                    "animation",
                    if reduce_motion {
                        "none: reduced motion is on, so the showcase shows the first frame and its timer steps no further (showcase-gpui/app.rs, start_animation_timer)".to_string()
                    } else {
                        format!("the showcase steps through the frames on a timer of its own, one every {duration_ms}ms; under reduced motion it holds the first (showcase-gpui/app.rs, start_animation_timer)")
                    },
                )
        }
        AnimatedKind::Spin { duration_ms } => info
            .variant(format!("{set}, spin"))
            // An SVG element paints every shape in its text colour
            // (gpui-pre elements/svg.rs, Svg::paint), which the showcase
            // sets to foreground.
            .color(claim("icon", "foreground", t.foreground, "showcase"))
            .not_themeable(
                "icon pixels",
                "the icon theme's one indicator icon, drawn as a mask in the showcase's foreground: gpui turns an SVG element but not an image, so every shape takes that one colour (native-theme-gpui/icons.rs, with_spin_animation)",
            )
            .instance(
                "animation",
                if reduce_motion {
                    "none: reduced motion is on, so the icon stands still at its start angle and no frames are drawn for it (native-theme-gpui/icons.rs, with_spin_animation; gpui-pre/elements/animation.rs, AnimationExt)".to_string()
                } else {
                    format!("a full turn every {duration_ms}ms (native-theme-gpui/icons.rs, with_spin_animation); under reduced motion it stands still at its start angle")
                },
            ),
    };
    let info = name_label(info, t)
        .color(claim("card border", "border", t.border, "showcase"))
        .not_themeable(
            "frame duration",
            "set by native-theme's loader, not by the icon set and not by the theme: a bundled set's one spinner icon is pre-rotated into frames of a fixed length, and a freedesktop theme's sprite sheet or single icon is given one. The model carries no animation timing",
        )
        .not_themeable(
            "geometry",
            "none: each card takes the showcase's own frame, which reads Theme::border, Theme::radius and the platform's defaults.border.line_width (showcase-gpui/support.rs, demo_frame)",
        );
    info.instance(
        "size",
        format!(
            "{}px, the showcase's own",
            px_text(ANIMATED_ICON_SIZE.as_f32())
        ),
    )
}

/// What `context` is, from platform-facts §2.1.8: what each platform calls
/// it and the size it documents, and what it documents none for.
fn icon_size_context(context: IconSizeContext) -> &'static str {
    match context {
        IconSizeContext::Toolbar => {
            "a toolbar's icons: 32pt regular and 24pt small on macOS, Fluent's AppBarButton 20 on Windows, KDE's MainToolbar group 22, GNOME's GTK_ICON_SIZE_NORMAL 16 (platform-facts §2.1.8)"
        }
        IconSizeContext::Small => {
            "small icons: macOS's sidebar icons, 16-20pt; SM_CXSMICON 16 on Windows; KDE's Small group 16; GNOME's GTK_ICON_SIZE_NORMAL 16 (platform-facts §2.1.8). The status bar's side-panel toggle and the items of the Layout page's Sidebar samples take this size"
        }
        IconSizeContext::Large => {
            "large icons: SM_CXICON 32 on Windows; KDE's Desktop group, 48 by Breeze's default; GNOME's GTK_ICON_SIZE_LARGE 32. macOS documents none (platform-facts §2.1.8)"
        }
        IconSizeContext::Dialog => {
            "a dialog's icon: KDE's Dialog group 32. macOS, Windows and GNOME document none; GNOME's 48 is GTK3's legacy (platform-facts §2.1.8)"
        }
        IconSizeContext::Panel => {
            "KDE's Panel group, 48 by Breeze's default, the C++ fallback. macOS, Windows and GNOME document none (platform-facts §2.1.8)"
        }
    }
}

/// A platform platform-facts §2.1.8 has a column for, as the native preset
/// installed names it.
#[derive(Clone, Copy)]
enum FactsPlatform {
    MacOs,
    Windows,
    Kde,
    Gnome,
}

impl FactsPlatform {
    /// The platform of the native preset `preset`, full or `-live`; `None`
    /// for any other theme.
    fn of_preset(preset: &str) -> Option<Self> {
        match preset.strip_suffix("-live").unwrap_or(preset) {
            "macos-sonoma" => Some(Self::MacOs),
            "windows-11" => Some(Self::Windows),
            "kde-breeze" => Some(Self::Kde),
            "adwaita" => Some(Self::Gnome),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::MacOs => "macOS",
            Self::Windows => "Windows",
            Self::Kde => "KDE",
            Self::Gnome => "GNOME",
        }
    }

    /// Whether platform-facts §2.1.8 documents a size for `context` on this
    /// platform: its (none) cells are macOS's large, dialog and panel,
    /// Windows's dialog and panel, and GNOME's dialog and panel
    /// (platform-facts.md:1132-1136).
    fn documents(self, context: IconSizeContext) -> bool {
        !matches!(
            (self, context),
            (Self::MacOs, IconSizeContext::Large)
                | (
                    Self::MacOs | Self::Windows | Self::Gnome,
                    IconSizeContext::Dialog | IconSizeContext::Panel
                )
        )
    }
}

/// A cell of the Icon Sizes section: the chrome's icon `drawn`, of `set`,
/// at the size `context` names, `size` px where a native theme gives one,
/// under the theme installed as `preset`. Its geometry line is recorded
/// where `demo::icon_size_cell` applies the builder.
pub fn icon_size(
    t: &Theme,
    context: IconSizeContext,
    drawn: &ChromeIcon,
    set: &str,
    size: Option<f32>,
    preset: &str,
) -> WidgetInfo {
    let field = match context {
        IconSizeContext::Toolbar => "toolbar.icon_size, which inherits defaults.icon_sizes.toolbar",
        IconSizeContext::Small => "defaults.icon_sizes.small",
        IconSizeContext::Large => "defaults.icon_sizes.large",
        IconSizeContext::Dialog => "defaults.icon_sizes.dialog",
        IconSizeContext::Panel => "defaults.icon_sizes.panel",
    };
    let name = context.name();
    // A number for a context the installed native preset's platform
    // documents none for is the preset's, and its size line says so.
    let unsourced = match FactsPlatform::of_preset(preset) {
        Some(platform) if !platform.documents(context) => format!(
            ". platform-facts §2.1.8 documents no {name} size for {}, so this number has no platform source: native-theme's {preset} preset states it",
            platform.name()
        ),
        _ => String::new(),
    };
    let info = name_label(WidgetInfo::new("Icon").variant(format!("{name} size")), t);
    let info = match drawn {
        ChromeIcon::Builtin(_) | ChromeIcon::Loaded(..) => info
            // An Icon takes the text colour it inherits (icon.rs:170,
            // :219), which the showcase sets on its window.
            .color(claim(
                "icon, inherited foreground",
                "foreground",
                t.foreground,
                "showcase",
            ))
            .instance(
                "size",
                match size {
                    Some(size) => format!("{}px, {field}{unsourced}", px_text(size)),
                    None => "upstream's own, the inherited text size: no native theme is installed, so no builder applies (icon.rs, Icon::into_svg)".to_string(),
                },
            ),
        ChromeIcon::Missing(_) | ChromeIcon::Unlisted(_) => info.instance(
            "size",
            match size {
                Some(size) => format!(
                    "{}px, {field}, which no icon shows{unsourced}",
                    px_text(size)
                ),
                None => "none: no native theme is installed, and no icon shows".to_string(),
            },
        ),
    };
    info.instance("field", field)
        .instance("context", icon_size_context(context))
        .instance(
            "unstated",
            "where the context line says a platform documents none, the size above has no platform source: native-theme's preset states that number, not the platform (platform-facts §2.1.8)",
        )
        .instance(
            "icon",
            chrome_icon_note(drawn, set, "the cell shows the context's name alone"),
        )
}
