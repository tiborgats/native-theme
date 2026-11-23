//! The Icons page.

use gpui::{AnyElement, App, Context, IntoElement, ParentElement, Styled, div, prelude::*};
use gpui_component::{IconName, IconNamed as _, h_flex, v_flex};

use native_theme::icons::IconSet;
use native_theme::icons::icon_name as native_icon_name;

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, AnimatedArt, IconArt, IconCell, IconSizeContext};
use crate::pages::{ICON_GRID_GAP, page_root};
#[cfg(feature = "svg-rasterize")]
use crate::support::GPUI_ICONS;
use crate::support::{IconSource, is_native_icon_set, with_gap};
#[cfg(feature = "svg-rasterize")]
use gpui::SharedString;
#[cfg(feature = "svg-rasterize")]
use native_theme_gpui::icons::{LoadedSet, ModeIcons, WidgetIconsReport};

/// The icon the Icon Sizes section shows at each size.
pub(crate) const ICON_SIZES_ICON: IconName = IconName::Folder;

impl Showcase {
    /// The Icon Sizes section: the chosen icon theme's `ICON_SIZES_ICON` at each
    /// size `defaults.icon_sizes` names, each through its own builder.
    fn render_icon_sizes_section(&self, cx: &App) -> impl IntoElement {
        let ui = &self.info_ui;
        let set = self.icon_set_label();
        let drawn = self.chrome_icon(&ICON_SIZES_ICON);
        let preset = self.platform_preset();
        let cells = IconSizeContext::ALL.map(|context| {
            demo::icon_size_cell(ui, cx, context, &drawn, &ICON_SIZES_ICON, &set, preset)
        });
        let widget_gap = geometry::widget_gap(&self.layout);
        with_gap(v_flex(), widget_gap)
            .child(demo::heading(ui, cx, "icons-heading-sizes", "Icon Sizes"))
            .child(
                with_gap(h_flex(), widget_gap)
                    .items_end()
                    .flex_wrap()
                    .children(cells),
            )
    }

    /// The Animated Icons section: the loading indicator the chosen icon theme ships,
    /// frame by frame or turning, or a caption saying it ships none.
    fn render_animated_icons_section(&self, cx: &App) -> impl IntoElement {
        let ui = &self.info_ui;
        let set = self.icon_set_label();
        let bundled = matches!(
            self.icon_set_enum,
            Some(IconSet::Material | IconSet::Lucide)
        );
        let fg = self.icon_cache_fg;
        let reduce_motion = cx.reduce_motion();
        let mut cards: Vec<AnyElement> = Vec::new();

        for (i, ((_set_name, frames), duration_ms)) in self
            .animated_frame_sources
            .iter()
            .zip(&self.animated_frame_durations)
            .enumerate()
        {
            // Under reduced motion each animation holds its first frame, as
            // gpui holds its own at their start.
            let frame = match self.animated_frame_indices.get(i) {
                Some(&shown) if !reduce_motion => frames.get(shown),
                _ => frames.first(),
            };
            if let Some(frame) = frame {
                cards.push(
                    demo::animated_icon(
                        ui,
                        cx,
                        format!("icons-animated-frames-{i}"),
                        &set,
                        AnimatedArt::Frames {
                            frame: frame.clone(),
                            count: frames.len(),
                            duration_ms: *duration_ms,
                        },
                        bundled,
                        fg,
                    )
                    .into_any_element(),
                );
            }
        }
        for (i, (_set_name, svg, duration_ms)) in self.animated_spin_sources.iter().enumerate() {
            cards.push(
                demo::animated_icon(
                    ui,
                    cx,
                    format!("icons-animated-spin-{i}"),
                    &set,
                    AnimatedArt::Spin {
                        svg,
                        duration_ms: *duration_ms,
                    },
                    bundled,
                    fg,
                )
                .into_any_element(),
            );
        }

        let body = if cards.is_empty() {
            demo::caption(
                ui,
                cx,
                "icons-no-animations",
                "No animated icons available for the current icon theme",
            )
            .into_any_element()
        } else {
            with_gap(h_flex(), geometry::widget_gap(&self.layout))
                .flex_wrap()
                .children(cards)
                .into_any_element()
        };
        with_gap(v_flex(), geometry::widget_gap(&self.layout))
            .child(demo::heading(
                ui,
                cx,
                "icons-heading-animated",
                "Animated Icons",
            ))
            .when(reduce_motion, |this| {
                this.child(demo::caption(
                    ui,
                    cx,
                    "icons-reduced-motion",
                    "(reduced motion: showing each animation's first frame)",
                ))
            })
            .child(body)
    }

    /// The Native Theme Icons section: native-theme's icon for every
    /// IconRole in the current set, or the placeholder where the set has
    /// none.
    fn render_native_icons_section(&self, cx: &App) -> impl IntoElement {
        let ui = &self.info_ui;
        let set = self.icon_set_label();
        let builtin = self.icon_set_enum.is_none();
        let fg = self.icon_cache_fg;

        let cells: Vec<(String, IconArt, Option<String>)> = self
            .loaded_icons
            .iter()
            .enumerate()
            .map(|(i, (role, data, source))| {
                let role_name = format!("{role:?}");
                if builtin {
                    return match native_theme_gpui::icons::icon_name(*role) {
                        Some(icon) => {
                            let asset = icon.clone().path().to_string();
                            (role_name, IconArt::Builtin(icon), Some(asset))
                        }
                        None => (role_name, IconArt::Missing, None),
                    };
                }
                let image = self.loaded_icon_sources.get(i).cloned().flatten();
                let art = match (image, data, source) {
                    (Some(image), _, IconSource::System) => IconArt::System(image),
                    (Some(image), _, IconSource::Bundled) => IconArt::Bundled(image),
                    (_, Some(_), _) => IconArt::Unreadable,
                    (_, None, _) => IconArt::Missing,
                };
                let name = self
                    .icon_set_enum
                    .and_then(|s| native_icon_name(*role, s))
                    .map(str::to_string);
                (role_name, art, name)
            })
            .collect();

        let loaded = cells
            .iter()
            .filter(|(_, art, _)| !matches!(art, IconArt::Missing))
            .count();
        let elsewhere = if is_native_icon_set(&self.icon_theme_name) {
            ""
        } else {
            " (not this platform's icon theme)"
        };
        let title = format!(
            "Native Theme Icons: {set} [{loaded}/{} loaded]{elsewhere}",
            cells.len()
        );

        let icons: Vec<_> = cells
            .into_iter()
            .map(|(role_name, art, name)| {
                demo::role_icon(
                    ui,
                    cx,
                    format!("icons-native-{role_name}"),
                    IconCell {
                        label: &role_name,
                        set: &set,
                        art,
                    },
                    builtin,
                    name.as_deref(),
                    fg,
                )
            })
            .collect();

        // Why the system icon theme is unavailable, where its detection
        // failed: the set's label leaves the reason out, as a label is short;
        // the iced and egui Icons pages give it too.
        let unavailable = self.icon_theme_unavailable();
        with_gap(v_flex(), geometry::widget_gap(&self.layout))
            .child(demo::heading(ui, cx, "icons-heading-native", title))
            .when_some(unavailable, |this, reason| {
                this.child(demo::caption(ui, cx, "icons-theme-unavailable", reason))
            })
            .child(div().flex().flex_wrap().gap(ICON_GRID_GAP).children(icons))
    }

    /// The gpui-component Icons section: the icon the current set gives
    /// each of gpui-component's IconNames, or gpui-component's own.
    fn render_gpui_icons_section(&self, cx: &App) -> impl IntoElement {
        let ui = &self.info_ui;
        let set = self.icon_set_label();
        let builtin = self.icon_set_enum.is_none();
        let by_role = matches!(
            self.icon_set_enum,
            Some(IconSet::SfSymbols | IconSet::SegoeFluent)
        );
        let fg = self.icon_cache_fg;

        let arts: Vec<_> = self
            .gpui_icons
            .iter()
            .enumerate()
            .map(|(i, (_, icon, _, data, source))| {
                if builtin {
                    IconArt::Builtin(icon.clone())
                } else {
                    let image = self.gpui_icon_sources.get(i).cloned().flatten();
                    match (image, data, source) {
                        (Some(image), _, IconSource::System) => IconArt::System(image),
                        (Some(image), _, IconSource::Bundled) => IconArt::Bundled(image),
                        (_, Some(_), _) => IconArt::Unreadable,
                        (_, None, _) => IconArt::Missing,
                    }
                }
            })
            .collect();
        // The names whose cell draws an icon the lookup found, by name or by
        // role: gpui-component's own icons are looked up by nothing.
        let mapped = arts
            .iter()
            .filter(|art| matches!(art, IconArt::System(_) | IconArt::Bundled(_)))
            .count();

        let icons: Vec<_> = self
            .gpui_icons
            .iter()
            .zip(arts)
            .map(|((name, _, role, _, _), art)| {
                let role_name = role.map(|r| format!("{r:?}"));
                demo::gpui_icon(
                    ui,
                    cx,
                    format!("icons-gpui-{name}"),
                    IconCell {
                        label: name,
                        set: &set,
                        art,
                    },
                    role_name.as_deref(),
                    by_role,
                    fg,
                )
            })
            .collect();

        let title = format!(
            "gpui-component Icons ({} variants, {mapped} mapped to {set})",
            self.gpui_icons.len(),
        );

        #[cfg(feature = "svg-rasterize")]
        let widget_icons = widget_icons_caption(crate::app::widget_icons_report(cx).as_ref());
        #[cfg(not(feature = "svg-rasterize"))]
        let widget_icons = widget_icons_caption();

        with_gap(v_flex(), geometry::widget_gap(&self.layout))
            .child(demo::heading(ui, cx, "icons-heading-gpui", title))
            .child(demo::caption(
                ui,
                cx,
                "icons-gpui-lookup",
                gallery_lookup_caption(self.icon_set_enum),
            ))
            .child(demo::caption(ui, cx, "icons-widget-icons", widget_icons))
            .child(div().flex().flex_wrap().gap(ICON_GRID_GAP).children(icons))
    }

    // -----------------------------------------------------------------------
    // Page: Icons
    // -----------------------------------------------------------------------
    pub(crate) fn render_icons_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        page_root(&self.layout)
            .child(self.render_icon_sizes_section(cx))
            .child(self.render_animated_icons_section(cx))
            .child(self.render_native_icons_section(cx))
            .child(self.render_gpui_icons_section(cx))
    }
}

/// The gpui-component Icons section's caption on how its gallery finds a
/// name's icon under the chosen `set` (`None`: gpui-component's own).
pub(crate) fn gallery_lookup_caption(set: Option<IconSet>) -> &'static str {
    match set {
        None => "This gallery: gpui-component's own icons, which nothing looks up",
        Some(IconSet::SfSymbols | IconSet::SegoeFluent) => {
            "This gallery: SF Symbols and Segoe Fluent are known by role only, so a name shows its role's icon, and a name with no role none"
        }
        Some(_) => {
            "This gallery: each name looked up as the page draws it, at the icon theme's 24 px or the bundled set's own, any icon the set has; a name it does not find is marked"
        }
    }
}

/// The gpui-component Icons section's caption on what the widget icons
/// serve gpui-component's widgets, from their `report`; `None` where none
/// are installed.
#[cfg(feature = "svg-rasterize")]
pub(crate) fn widget_icons_caption(report: Option<&WidgetIconsReport>) -> String {
    const OWN: &str = "gpui-component's widgets: their own icons";
    match report {
        None | Some(WidgetIconsReport::NotInstalled) => {
            format!("{OWN} (the widget icons are not installed)")
        }
        Some(WidgetIconsReport::Off) => {
            format!("{OWN} (the built-in set is chosen, or no system icon theme was detected)")
        }
        Some(WidgetIconsReport::NotTold) => format!("{OWN} (no icon set given yet)"),
        Some(WidgetIconsReport::NotServed { set, .. }) => format!(
            "{OWN}: {} is not served to them in this release",
            set.name()
        ),
        Some(WidgetIconsReport::NoAssetSource) => {
            format!("{OWN} (the widget icons have no asset source)")
        }
        Some(WidgetIconsReport::Loaded(loaded)) => {
            loaded_caption(loaded).unwrap_or_else(|| UNDESCRIBED.to_string())
        }
        Some(_) => UNDESCRIBED.to_string(),
    }
}

/// [`widget_icons_caption`] without the widget icons, which this build
/// lacks.
#[cfg(not(feature = "svg-rasterize"))]
pub(crate) fn widget_icons_caption() -> String {
    "gpui-component's widgets: their own icons (this build has no widget icons: feature svg-rasterize)".to_string()
}

/// The caption of a state of the widget icons the showcase does not
/// describe.
#[cfg(feature = "svg-rasterize")]
const UNDESCRIBED: &str =
    "gpui-component's widgets: a state of the widget icons this showcase does not describe";

/// [`widget_icons_caption`] of a set the widget icons loaded: what they
/// serve, or why they serve nothing. `None` for a set not served for no
/// reason a mode gives.
#[cfg(feature = "svg-rasterize")]
fn loaded_caption(loaded: &LoadedSet) -> Option<String> {
    let set = loaded.set.name();
    let freedesktop = loaded.set == IconSet::Freedesktop;
    let themes = if freedesktop {
        let theme = |mode: &ModeIcons| {
            mode.icon_theme
                .clone()
                .unwrap_or_else(|| "no icon theme detected".to_string())
        };
        let (light, dark) = (theme(&loaded.light), theme(&loaded.dark));
        if light == dark {
            format!(" ({light})")
        } else {
            format!(" ({light}, {dark})")
        }
    } else {
        String::new()
    };
    if !loaded.served() {
        let reason = [&loaded.light, &loaded.dark].into_iter().find_map(|mode| {
            if mode.gave_nothing {
                Some("gave nothing".to_string())
            } else if mode.missing_essentials.is_empty() {
                None
            } else {
                Some(format!(
                    "lacks {}, which nearly every window needs",
                    gallery_names(&mode.missing_essentials)
                ))
            }
        })?;
        return Some(format!(
            "gpui-component's widgets: their own icons: {set}{themes} {reason}"
        ));
    }
    let size = if freedesktop { "at 16 px and " } else { "" };
    let missing = if loaded.light.missing == loaded.dark.missing {
        gallery_names(&loaded.light.missing)
    } else {
        format!(
            "light: {}; dark: {}",
            gallery_names(&loaded.light.missing),
            gallery_names(&loaded.dark.missing)
        )
    };
    let mut kept = loaded.light.kept.clone();
    kept.extend(
        loaded
            .dark
            .kept
            .iter()
            .filter(|path| !loaded.light.kept.contains(path))
            .cloned(),
    );
    let kept = if kept.is_empty() {
        String::new()
    } else {
        format!("; kept as gpui-component's own: {}", gallery_names(&kept))
    };
    Some(format!(
        "gpui-component's widgets: {set}{themes}, {size}in one colour; not in the set, so drawn empty: {missing}{kept}"
    ))
}

/// The gallery's names (`GPUI_ICONS`) of the icons at `paths`,
/// comma-separated, a path the gallery does not list as itself; "none"
/// for none.
#[cfg(feature = "svg-rasterize")]
fn gallery_names(paths: &[SharedString]) -> String {
    if paths.is_empty() {
        return "none".to_string();
    }
    paths
        .iter()
        .map(|path| {
            GPUI_ICONS
                .iter()
                .find(|(_, icon)| icon.clone().path() == *path)
                .map_or_else(|| path.to_string(), |(name, _)| (*name).to_string())
        })
        .collect::<Vec<_>>()
        .join(", ")
}
