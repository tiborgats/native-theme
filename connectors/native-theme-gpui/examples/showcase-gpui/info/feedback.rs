//! What the Feedback page's widgets report about themselves (spec §3.4).

use gpui::transparent_white;
use gpui_component::{Colorize as _, theme::Theme};

use super::{WidgetInfo, claim};
use crate::demo::{CircleKind, MarkerKind, Severity, ShimmerKind, SpinnerKind, TagKind};

/// An `Alert` of `severity` at the default Size, a `banner` or not. The
/// caller adds what the Alert says.
pub fn alert(t: &Theme, severity: Severity, banner: bool) -> WidgetInfo {
    // (text and icon, fill, edge), each read in its variant's arm of
    // AlertVariant. The fill and the edge are the variant's colour mixed
    // toward transparent white in Oklab -- `mix_oklab`'s factor is the first
    // colour's share (theme/color.rs:44-49) -- so the swatches are the mixes.
    let (text, fill, edge) = match severity {
        Severity::Info => (
            claim(
                "text and icon",
                "info",
                t.info,
                "gpui-component/alert.rs:29",
            ),
            claim(
                "bg, info at 4%",
                "info",
                t.info.mix_oklab(transparent_white(), 0.04),
                "gpui-component/alert.rs:39",
            ),
            claim(
                "border, info at 30%",
                "info",
                t.info.mix_oklab(transparent_white(), 0.3),
                "gpui-component/alert.rs:49",
            ),
        ),
        Severity::Success => (
            claim(
                "text and icon",
                "success",
                t.success,
                "gpui-component/alert.rs:30",
            ),
            claim(
                "bg, success at 4%",
                "success",
                t.success.mix_oklab(transparent_white(), 0.04),
                "gpui-component/alert.rs:40",
            ),
            claim(
                "border, success at 30%",
                "success",
                t.success.mix_oklab(transparent_white(), 0.3),
                "gpui-component/alert.rs:50",
            ),
        ),
        Severity::Warning => (
            claim(
                "text and icon",
                "warning",
                t.warning,
                "gpui-component/alert.rs:31",
            ),
            claim(
                "bg, warning at 4%",
                "warning",
                t.warning.mix_oklab(transparent_white(), 0.04),
                "gpui-component/alert.rs:41",
            ),
            claim(
                "border, warning at 30%",
                "warning",
                t.warning.mix_oklab(transparent_white(), 0.3),
                "gpui-component/alert.rs:51",
            ),
        ),
        Severity::Error => (
            claim(
                "text and icon",
                "danger",
                t.danger,
                "gpui-component/alert.rs:32",
            ),
            claim(
                "bg, danger at 4%",
                "danger",
                t.danger.mix_oklab(transparent_white(), 0.04),
                "gpui-component/alert.rs:42",
            ),
            claim(
                "border, danger at 30%",
                "danger",
                t.danger.mix_oklab(transparent_white(), 0.3),
                "gpui-component/alert.rs:52",
            ),
        ),
    };
    let name = severity.name();
    let info = WidgetInfo::new("Alert").color(text).color(fill).color(edge);
    let info = if banner {
        info.variant(format!("{name}, banner")).not_themeable(
            "banner",
            "full width, with no corner radius and no title (alert.rs, banner)",
        )
    } else {
        info.variant(name)
            .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    };
    info.not_themeable("tints", "the variant colour mixed toward transparent white in Oklab: 4% of it for the fill and 30% for the edge -- one token at three strengths, the swatches showing what is painted (alert.rs, AlertVariant)")
        .not_themeable("padding", "px literals per Size, but reachable: Alert is Styled and applies the caller's refinement after its own paddings (alert.rs, Alert::render). Its corner radius already comes from the theme (alert.rs, Alert::render radius). What is missing is a model -- native-theme states no alert widget, so there is nothing to carry. Our gap, not upstream's")
        .not_themeable("icon", format!("the {name} variant's default, not a fixed glyph: Alert::icon replaces it with any Icon (alert.rs, Alert::icon), and an Icon takes a path or raw SVG bytes (icon.rs, Icon::path, Icon::data), so a platform icon from this connector's loader can be handed to it"))
        .not_themeable("icon size", "the Alert's text size: an Icon given no size takes the font size it inherits (icon.rs, Icon::into_svg), and the Alert sets text_sm (alert.rs, Alert::render) -- a rem, so it follows the platform font. defaults.icon_sizes is in absolute px and there is no alert in the model to hang it on")
}

/// A `Progress` bar at `value` percent, labelled `label` beside it, drawn
/// while gpui's `reduce_motion` is as given.
pub fn progress(t: &Theme, label: &str, value: f32, reduce_motion: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Progress")
        .color(claim(
            "bar",
            "progress_bar",
            t.progress_bar,
            "gpui-component/progress/progress.rs:86",
        ))
        .color(claim(
            "track, progress_bar at 20% (progress/progress.rs:135)",
            "progress_bar",
            t.progress_bar.opacity(0.2),
            "gpui-component/progress/progress.rs:86",
        ))
        .not_themeable("animation", "not hardcoded upstream: the fill transitions over the theme's duration_normal and easing_move (progress/progress.rs, Progress). Theme::motion is a writable field (theme/mod.rs, MotionTokens) the connector leaves at its default, because native-theme models no motion -- our model's gap, not upstream's");
    let info = if reduce_motion {
        info.instance("reduced motion", "on, so a change of value jumps to it, unanimated (gpui-base/motion.rs, transition_with_status)")
    } else {
        info
    };
    info.instance("value", format!("{value}%"))
        .instance("label", label.to_string())
}

/// A `ProgressCircle` of `kind`, drawn while gpui's `reduce_motion` is as
/// given. `styled` is whether an indeterminate one took its size from
/// `geometry::spinner_size`, whose line is recorded where
/// `demo::progress_circle` applies it.
pub fn progress_circle(
    t: &Theme,
    kind: CircleKind,
    styled: bool,
    reduce_motion: bool,
) -> WidgetInfo {
    let info = WidgetInfo::new("ProgressCircle").variant(kind.name());
    // Under reduced motion an indeterminate circle holds its loop's start,
    // where both ends are at 0 (progress_circle.rs:201-205), and an arc of
    // no length is not painted (:117): only the track shows.
    let arc_drawn = kind != CircleKind::Indeterminate || !reduce_motion;
    let info = if arc_drawn {
        info.color(claim(
            "arc",
            "progress_bar",
            t.progress_bar,
            "gpui-component/progress/progress_circle.rs:176",
        ))
    } else {
        info
    };
    let info = info.color(claim(
        "track, progress_bar at 20% (progress/progress_circle.rs:110)",
        "progress_bar",
        t.progress_bar.opacity(0.2),
        "gpui-component/progress/progress_circle.rs:176",
    ));
    let determinate_size = "size_2 to size_5 per Size, rems, so it follows the platform font; the model carries no circular-progress diameter";
    let info = match kind {
        CircleKind::Value | CircleKind::ValueLarge => info
            .not_themeable("size", determinate_size)
            .instance("animation", if reduce_motion {
                "none: reduced motion is on, so a change of value jumps to it (gpui-base/motion.rs, transition_with_status); this one's value never changes either"
            } else {
                "a change of value sweeps the arc over the theme's duration_normal and easing_move (progress/progress_circle.rs, ProgressCircle::render); this one's value never changes, so it stands still"
            }),
        CircleKind::Indeterminate if styled => info
            .not_themeable("size", "drawn at 75% of spinner.diameter, because a ProgressCircle scales a Size::Size by 0.75 where a Spinner takes it whole (progress/progress_circle.rs, ProgressCircle)"),
        CircleKind::Indeterminate => info.not_themeable("size", determinate_size),
    };
    let info = match kind {
        CircleKind::Value => info.instance("value", "73%, at the default Size"),
        CircleKind::ValueLarge => info.instance("value", "100%, at Size::Large"),
        CircleKind::Indeterminate if reduce_motion => info.instance("animation", "none: reduced motion is on, so the 1s loop holds at its start, where both ends of the arc are at 0 and no arc is drawn -- only the track shows (progress/progress_circle.rs, ProgressCircle::render_circle; gpui-pre/elements/animation.rs, AnimationExt)"),
        CircleKind::Indeterminate => info.instance("animation", "the arc's two ends chase each other round a 1s loop, a literal (progress/progress_circle.rs, ProgressCircle::render): the colours hold still and only the arc's extent moves. Under reduced motion the loop holds at its start, where no arc is drawn"),
    };
    info.not_themeable("track opacity", "the same progress_bar colour at 20%: the model states one bar colour, and the track is derived from it (progress/progress_circle.rs, ProgressCircle::render_circle)")
        .not_themeable("stroke width", "15% of the diameter, capped at 5px (progress/progress_circle.rs, ProgressCircle::render_circle stroke_width)")
}

/// A `Spinner` of `kind`. A Medium one takes `geometry::spinner_size` where
/// a native theme is installed, whose line is recorded where `demo::spinner`
/// applies it; `styled` is whether it did. It is drawn while gpui's
/// `reduce_motion` is as given.
pub fn spinner(t: &Theme, kind: SpinnerKind, styled: bool, reduce_motion: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Spinner")
        .variant(kind.name())
        // No colour of its own: the Loader icon takes the text colour it
        // inherits (spinner.rs:64-66, icon.rs:219), which the showcase sets
        // on its window.
        .color(claim(
            "icon, inherited foreground",
            "foreground",
            t.foreground,
            "showcase",
        ));
    let info = match kind {
        SpinnerKind::Small => info.instance("size", "Small via the Size enum: size_3p5, a rem (icon.rs, Icon::into_svg)"),
        SpinnerKind::Large => info.instance("size", "Large via the Size enum: size_6, a rem (icon.rs, Icon::into_svg)"),
        SpinnerKind::Medium if styled => info,
        SpinnerKind::Medium => info.instance("size", "Medium, upstream's default (spinner.rs, Spinner::new): size_4, a rem (icon.rs, Icon::into_svg)"),
    };
    info.not_themeable("animation speed", "a 0.8s private field with no setter, and not one of the theme's motion tokens (spinner.rs, Spinner) -- nothing to write, upstream or here")
        .instance("animation", if reduce_motion {
            "none: reduced motion is on, so the Loader icon stands still at its start angle and no frames are drawn for it (spinner.rs, Spinner::render; gpui-pre/elements/animation.rs, AnimationExt)"
        } else {
            "the Loader icon turns a full circle every 0.8s; only its angle moves, its colour holds (spinner.rs, Spinner::render)"
        })
}

/// A `Skeleton` placeholder, `secondary` or not, `height` tall and `width`
/// wide where the showcase gives it one, rounded with `radius_lg` or with
/// `radius`, drawn while gpui's `reduce_motion` is as given.
pub fn skeleton(
    t: &Theme,
    secondary: bool,
    height: f32,
    width: Option<f32>,
    radius_lg: bool,
    reduce_motion: bool,
) -> WidgetInfo {
    let info = WidgetInfo::new("Skeleton");
    // The fill is what is painted at the top of the pulse: the whole
    // element's opacity then falls to half and rises again
    // (skeleton.rs:48-56), a mid-pulse colour no swatch could hold. Under
    // reduced motion the pulse holds its start, full opacity, so the fill
    // is simply what is painted.
    let info = match (secondary, reduce_motion) {
        (true, false) => info.variant("secondary").color(claim(
            "bg, skeleton at 50% (skeleton.rs:43), at the pulse's peak",
            "skeleton",
            t.skeleton.opacity(0.5),
            "gpui-component/skeleton.rs:43",
        )),
        (true, true) => info.variant("secondary").color(claim(
            "bg, skeleton at 50% (skeleton.rs:43)",
            "skeleton",
            t.skeleton.opacity(0.5),
            "gpui-component/skeleton.rs:43",
        )),
        (false, false) => info.color(claim(
            "bg, at the pulse's peak",
            "skeleton",
            t.skeleton,
            "gpui-component/skeleton.rs:45",
        )),
        (false, true) => info.color(claim(
            "bg",
            "skeleton",
            t.skeleton,
            "gpui-component/skeleton.rs:45",
        )),
    };
    let info = if radius_lg {
        info.config(
            "border-radius",
            format!("radius_lg: {}px", t.radius_lg.as_f32()),
        )
    } else {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    };
    info.not_themeable("animation", "a 2s literal, and unlike the Accordion's or the Switch's it is not one of the theme's motion tokens (skeleton.rs, Skeleton) -- nothing to write, upstream or here")
        .instance("pulse", if reduce_motion {
            "none: reduced motion is on, so the placeholder holds the pulse's start, full opacity, and no frames are drawn for it (skeleton.rs, Skeleton::render; gpui-pre/elements/animation.rs, AnimationExt)"
        } else {
            "the whole placeholder fades from full opacity to half and back on a bounced 2s loop (skeleton.rs, Skeleton::render); its fill holds, so the swatch is the fill at the top of the pulse"
        })
        .instance("shape", match width {
            Some(width) => format!("{height}px tall and {width}px wide, the showcase's sizes; its rounding is the theme's, not a number of its own"),
            None => format!("{height}px tall and as wide as its column, the showcase's sizes; its rounding is the theme's, not a number of its own"),
        })
}

/// A `ShimmerText` of `kind` reading `text`, drawn while gpui's
/// `reduce_motion` is as given.
pub fn shimmer_text(t: &Theme, kind: ShimmerKind, text: &str, reduce_motion: bool) -> WidgetInfo {
    let info = WidgetInfo::new("ShimmerText").variant(kind.name());
    let info = match kind {
        ShimmerKind::Default | ShimmerKind::Reverse => info.color(claim(
            "text, inherited foreground",
            "foreground",
            t.foreground,
            "showcase",
        )),
        ShimmerKind::Slow => info.color(claim(
            "text",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        )),
    };
    let info = info
        .not_themeable("highlight", "the text colour mixed with background (light) or foreground (dark), at 75%/60% peak (shimmer.rs, shimmer_highlight_color)")
        .not_themeable("reduced motion", "gpui's App::reduce_motion: the text renders once, unanimated (shimmer.rs, ShimmerText::render)")
        .not_themeable("sweep", "2s by default, a literal rather than a motion token (shimmer.rs, ShimmerStyle)")
        .instance("animation", if reduce_motion {
            "none: reduced motion is on, so the text is drawn plain, with no highlight (shimmer.rs, ShimmerText::render)"
        } else {
            "only the highlight band moves, painted over glyphs whose own colour holds (shimmer.rs, ShimmerGlyphs::paint_highlight)"
        });
    let info = match kind {
        ShimmerKind::Default => info.instance("sweep", "the default: 2s, left to right, the highlight's half-width 30% of the text"),
        ShimmerKind::Slow => info.instance("sweep", "3s, left to right; the text is the showcase's muted_foreground, which the highlight is mixed from"),
        ShimmerKind::Reverse => info.instance("sweep", "2s, right to left, the highlight's half-width 50% of the text"),
    };
    info.instance("text", text.to_string())
}

/// The `Empty` state, its media icon at `geometry::icon_size_large` where a
/// native theme is installed, whose line is recorded where `demo::empty`
/// applies it.
pub fn empty(t: &Theme, title: &str, description: &str) -> WidgetInfo {
    WidgetInfo::new("Empty")
        .color(claim(
            "media bg",
            "muted",
            t.muted,
            "gpui-component/empty.rs:219",
        ))
        .color(claim(
            "media icon",
            "foreground",
            t.foreground,
            "gpui-component/empty.rs:220",
        ))
        .color(claim(
            "title",
            "foreground",
            t.foreground,
            "gpui-component/empty.rs:77",
        ))
        .color(claim(
            "description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/empty.rs:319",
        ))
        .config(
            "media frame radius",
            format!("radius_tokens().lg: {}px", t.radius_tokens().lg.as_f32()),
        )
        .not_themeable("corner radius", "radius_tokens().xl on the Empty itself, which rounds nothing visible: the Empty paints no fill and draws no edge (empty.rs, Empty::render). The rounding that shows is the media frame's (empty.rs, EmptyMedia)")
        .not_themeable("edge", "none drawn: Empty sets border_dashed and border_color(border) but no border width, so its edge is 0px wide (empty.rs, Empty::render). It is Styled and applies the caller's refinement last, so a caller's border width would draw it, dashed, in border")
        .not_themeable("media frame", "2rem square -- the platform's font, not a literal -- and settable: EmptyMedia applies the caller's refinement last (empty.rs, EmptyMedia). Nothing sizes it to the icon inside, which is defaults.icon_sizes.large: 32px, or 48px on KDE, so the icon outgrows the frame wherever 2rem is smaller")
        .instance("title", title.to_string())
        .instance("description", description.to_string())
        .instance("action", "the Refresh Button, which reports itself")
}

/// A `Tag` of `kind`, `outline` or not, reading `label`.
pub fn tag(t: &Theme, kind: TagKind, outline: bool, label: &str) -> WidgetInfo {
    // (name, fill, text, edge, text when outlined), each read in its
    // variant's arm of TagVariant's bg, fg and border. No arm branches on
    // the mode.
    let (name, fill, text, edge, outlined_text) = match kind {
        TagKind::Primary => (
            "Primary",
            claim("bg", "primary", t.primary, "gpui-component/tag.rs:29"),
            claim(
                "text",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/tag.rs:71",
            ),
            claim("border", "primary", t.primary, "gpui-component/tag.rs:48"),
            claim("text", "primary", t.primary, "gpui-component/tag.rs:69"),
        ),
        TagKind::Secondary => (
            "Secondary",
            claim("bg", "secondary", t.secondary, "gpui-component/tag.rs:30"),
            claim(
                "text",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/tag.rs:78",
            ),
            claim("border", "border", t.border, "gpui-component/tag.rs:49"),
            claim(
                "text",
                "muted_foreground",
                t.muted_foreground,
                "gpui-component/tag.rs:76",
            ),
        ),
        TagKind::Danger => (
            "Danger",
            claim("bg", "danger", t.danger, "gpui-component/tag.rs:31"),
            claim(
                "text",
                "danger_foreground",
                t.danger_foreground,
                "gpui-component/tag.rs:85",
            ),
            claim("border", "danger", t.danger, "gpui-component/tag.rs:50"),
            claim("text", "danger", t.danger, "gpui-component/tag.rs:83"),
        ),
        TagKind::Success => (
            "Success",
            claim("bg", "success", t.success, "gpui-component/tag.rs:32"),
            claim(
                "text",
                "success_foreground",
                t.success_foreground,
                "gpui-component/tag.rs:92",
            ),
            claim("border", "success", t.success, "gpui-component/tag.rs:51"),
            claim("text", "success", t.success, "gpui-component/tag.rs:90"),
        ),
        TagKind::Warning => (
            "Warning",
            claim("bg", "warning", t.warning, "gpui-component/tag.rs:33"),
            claim(
                "text",
                "warning_foreground",
                t.warning_foreground,
                "gpui-component/tag.rs:99",
            ),
            claim("border", "warning", t.warning, "gpui-component/tag.rs:52"),
            claim("text", "warning", t.warning, "gpui-component/tag.rs:97"),
        ),
        TagKind::Info => (
            "Info",
            claim("bg", "info", t.info, "gpui-component/tag.rs:34"),
            claim(
                "text",
                "info_foreground",
                t.info_foreground,
                "gpui-component/tag.rs:106",
            ),
            claim("border", "info", t.info, "gpui-component/tag.rs:53"),
            claim("text", "info", t.info, "gpui-component/tag.rs:104"),
        ),
    };
    let info = WidgetInfo::new("Tag").variant(if outline {
        format!("{name}, outline")
    } else {
        name.to_string()
    });
    let info = if outline {
        info.color(outlined_text)
            .color(edge)
            .not_themeable("outlined fill", "transparent_white(), a literal: an outlined Tag drops its variant background entirely, keeping only the border and the text (tag.rs, Tag::render)")
    } else {
        info.color(fill).color(text).color(edge)
    };
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("padding", "px_2p5 / py_1 at the default Size -- rems, so the platform's font -- and settable: Tag applies the caller's refinement last (tag.rs, Tag::render). native-theme states no tag widget. Our gap")
        .not_themeable("hover", "the whole Tag fades to 90% opacity, a literal, fill, text and edge alike (tag.rs, Tag::render)")
        .instance("label", label.to_string())
}

/// A `Badge` showing `count`, or a dot where it is `None`, on a Default
/// Button reading `label`, which reports
/// through the Badge. The Button's `geometry::button` line is recorded where
/// `demo::badge` applies it.
pub fn badge(t: &Theme, count: Option<usize>, label: &str) -> WidgetInfo {
    let info = WidgetInfo::new("Badge");
    let info = match count {
        Some(count) => info
            .variant("count")
            .color(claim("fill", "red", t.red, "gpui-component/badge.rs:125"))
            .not_themeable("text", "white() whatever the fill, not a theme field: the fill is red, which the connector writes from its danger colour, so the count reads only where that colour is dark enough for white (badge.rs, Badge::render)")
            .not_themeable("size", "none: a count badge is as wide as its digits, at least 0.875rem, with its text at a 10px literal at every Size. px(10) / px(16) / px(24) per Size is an icon badge's, which this demo does not show (badge.rs, Badge::render)")
            .not_themeable("padding", "0.125rem on a count badge at every Size, on a pill no caller can reach: Badge applies its refinement to the wrapper around the badged element, and the pill is an absolute child built after it (badge.rs, Badge::render). native-theme states no badge widget either")
            .instance("count", count.to_string()),
        None => info
            .variant("dot")
            .color(claim("dot fill", "red", t.red, "gpui-component/badge.rs:125"))
            .not_themeable("size", "a 6px literal at every Size, on a pill no caller can reach: Badge applies its refinement to the wrapper around the badged element, and the pill is an absolute child built after it (badge.rs, Badge::render). native-theme states no badge widget either"),
    };
    info.not_themeable("fill, overridden", "Badge::color replaces it; red is only the default")
        .instance("badged element", format!("a Default Button reading {label}. The Badge is exactly as large as the Button -- its pill is an absolute child -- so the Button reports through the Badge (badge.rs, Badge::render). Its own colours are a Default Button's, which the Buttons page's Default Button states"))
}

/// A `Marker` of `kind` reading `text`. A Plain one's icon takes
/// `geometry::icon_size_small` where a native theme is installed, whose line
/// is recorded where `demo::marker` applies it.
pub fn marker(t: &Theme, kind: MarkerKind, text: &str, reduce_motion: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Marker").variant(kind.name()).color(claim(
        "text",
        "muted_foreground",
        t.muted_foreground,
        "gpui-component/marker.rs:185",
    ));
    let info = match kind {
        MarkerKind::Plain => info.color(claim(
            "icon, the row's text colour",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/marker.rs:185",
        )),
        MarkerKind::Separator => info.color(claim(
            "separator line",
            "border",
            t.border,
            "gpui-component/marker.rs:200",
        )),
        MarkerKind::Border => info.color(claim(
            "bottom border",
            "border",
            t.border,
            "gpui-component/marker.rs:191",
        )),
        MarkerKind::Spinner => info
            .color(claim(
                "spinner, the row's text colour",
                "muted_foreground",
                t.muted_foreground,
                "gpui-component/marker.rs:185",
            ))
            .instance("spinner", if reduce_motion {
                "added by the Marker itself, loading with no icon slot: an XSmall Spinner, standing still at its start angle while reduced motion is on (marker.rs, Marker::render; gpui-pre/elements/animation.rs, AnimationExt)"
            } else {
                "added by the Marker itself, loading with no icon slot: an XSmall Spinner turning every 0.8s, a literal; only its angle moves (marker.rs, Marker::render; spinner.rs, Spinner)"
            }),
        MarkerKind::Shimmer if reduce_motion => info
            .not_themeable("shimmer", "none while reduced motion is on: the content slot falls back to plain text (marker.rs, MarkerContent::render)")
            .instance("animation", "none: reduced motion is on, so the text is drawn plain, with no highlight (marker.rs, MarkerContent::render)"),
        MarkerKind::Shimmer => info
            .not_themeable("shimmer", "the loading highlight ShimmerText paints, on the content slot only (marker.rs, Marker::render MarkerChild::Content)")
            .instance("animation", "only the highlight band moves, painted over text whose own colour holds (shimmer.rs, ShimmerGlyphs::paint_highlight)"),
    };
    info.not_themeable("row gap", "gap_2, 0.5rem -- the platform's font -- and settable: Marker applies the caller's refinement last (marker.rs, Marker::render)")
        .instance("text", text.to_string())
}

/// A tooltip reading `text` on a Default Button reading `label`, which
/// reports through the tooltip: the popup is drawn on a layer above the
/// page, where nothing of the showcase can wrap it. `built` is whether the
/// application built the tooltip itself; `styled` is whether it is refined
/// by `geometry::tooltip`, whose lines are recorded where
/// `demo::built_tooltip_button` applies it.
pub fn tooltip(t: &Theme, built: bool, styled: bool, label: &str, text: &str) -> WidgetInfo {
    let info = WidgetInfo::new("Tooltip")
        .variant(if built {
            "built by the application"
        } else {
            "Button::tooltip"
        })
        .color(claim(
            "bg",
            "popover",
            t.popover,
            "gpui-component/tooltip.rs:114",
        ))
        .color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/tooltip.rs:118",
        ));
    // geometry::tooltip paints the text with tooltip.font's colour, which no
    // ThemeColor field holds, over upstream's, and rounds it with the
    // platform's corner radius.
    let info = if built && styled {
        info
    } else {
        info.color(claim(
            "text",
            "popover_foreground",
            t.popover_foreground,
            "gpui-component/tooltip.rs:115",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    };
    let info = if built {
        info.not_themeable("delay", "gpui's own tooltip, set on an element rather than through Button::tooltip, so it takes tooltip_show_delay (gpui-pre/elements/div.rs, tooltip_show_delay), which the showcase does not set. native-theme states no hover delay, though the platforms do -- our model's gap")
            .not_themeable("position", "gpui's own sits a pixel off the pointer (gpui-pre/window.rs, prepaint_tooltip)")
            .instance("why built", "Button::tooltip takes a string and builds the tooltip itself (button/button.rs, Button::tooltip), so the only way to a refined one is to build it with Tooltip::element: the one place the platform's tooltip padding, radius and text colour reach the popup. The width is the content element's: on the popup it would clamp the popup and not the text, which then runs out of it")
    } else {
        info.not_themeable("delay", "500ms, and fixed for a Button's tooltip: Button::tooltip goes through Root's tooltip overlay, whose SHOW_DELAY is a module const (gpui-base/tooltip.rs, SHOW_DELAY). native-theme states no hover delay, though the platforms do -- our model's gap")
            .not_themeable("position", "beside its trigger, flipping to stay in the window, unless Button::tooltip_placement pins a side (gpui-base/tooltip.rs, TooltipPositioner)")
    };
    info.instance(
        "trigger",
        format!("a Default Button reading {label}, which reports through its tooltip. Its own colours are a Default Button's, which the Buttons page's Default Button states"),
    )
    .instance("text", text.to_string())
}

/// The `Notification` of `severity` a Default Button reading `label`
/// pushes, which reports through the Button: upstream draws the
/// notification on the Root's layer, where nothing of the showcase can wrap
/// it. The Button's `geometry::button` line is recorded where
/// `demo::notification_button` applies it.
pub fn notification(t: &Theme, severity: Severity, label: &str, message: &str) -> WidgetInfo {
    let icon = match severity {
        Severity::Info => claim("icon", "info", t.info, "gpui-component/notification.rs:42"),
        Severity::Success => claim(
            "icon",
            "success",
            t.success,
            "gpui-component/notification.rs:43",
        ),
        Severity::Warning => claim(
            "icon",
            "warning",
            t.warning,
            "gpui-component/notification.rs:44",
        ),
        Severity::Error => claim(
            "icon",
            "danger",
            t.danger,
            "gpui-component/notification.rs:45",
        ),
    };
    WidgetInfo::new("Notification")
        .variant(severity.name())
        .color(claim(
            "bg",
            "popover",
            t.popover,
            "gpui-component/notification.rs:424",
        ))
        .color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/notification.rs:423",
        ))
        .color(icon)
        .config("border-radius", format!("radius_lg: {}px", t.radius_lg.as_f32()))
        .not_themeable("animation", "module consts, 400ms in and 200ms out, not the theme's motion tokens (notification.rs, Notification)")
        .not_themeable("autohide", "after 5s, a literal, unless Notification::autohide(false) or an action keeps it open (notification.rs, ToastOptions)")
        .instance("trigger", format!("a Default Button reading {label}: a click pushes the notification through WindowExt::push_notification. Its own colours are a Default Button's, which the Buttons page's Default Button states"))
        .instance("message", message.to_string())
}
