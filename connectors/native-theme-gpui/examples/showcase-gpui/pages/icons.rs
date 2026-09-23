//! The Icons page.

use gpui::{AnyElement, App, Context, IntoElement, ParentElement, Styled, div, prelude::*};
use gpui_component::{IconName, IconNamed as _, h_flex, v_flex};

use native_theme::theme::{IconSet, icon_name as native_icon_name};

use crate::app::Showcase;
use crate::demo::{self, AnimatedArt, IconArt, IconCell, IconSizeContext};
use crate::support::{IconSource, is_native_icon_set};

/// The icon the Icon Sizes section shows at each size.
pub(crate) const ICON_SIZES_ICON: IconName = IconName::Folder;

impl Showcase {
    /// The Icon Sizes section: the chosen set's `ICON_SIZES_ICON` at each
    /// size `defaults.icon_sizes` names, each through its own builder.
    fn render_icon_sizes_section(&self, cx: &App) -> impl IntoElement {
        let ui = &self.info_ui;
        let set = self.icon_set_label();
        let drawn = self.chrome_icon(&ICON_SIZES_ICON);
        let preset = self.platform_preset();
        let cells = IconSizeContext::ALL.map(|context| {
            demo::icon_size_cell(ui, cx, context, &drawn, &ICON_SIZES_ICON, &set, preset)
        });
        v_flex()
            .gap_2()
            .child(demo::heading(ui, cx, "icons-heading-sizes", "Icon Sizes"))
            .child(h_flex().items_end().flex_wrap().gap_2().children(cells))
    }

    /// The Animated Icons section: the loading indicator the icon set ships,
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
            h_flex()
                .gap_6()
                .flex_wrap()
                .children(cards)
                .into_any_element()
        };
        v_flex()
            .gap_2()
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
        let elsewhere = if is_native_icon_set(&self.icon_set_name) {
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

        v_flex()
            .gap_2()
            .child(demo::heading(ui, cx, "icons-heading-native", title))
            .child(div().flex().flex_wrap().gap_2().children(icons))
    }

    /// The gpui-component Icons section: the icon the current set gives
    /// each of gpui-component's IconNames, or gpui-component's own.
    fn render_gpui_icons_section(&self, cx: &App) -> impl IntoElement {
        let ui = &self.info_ui;
        let set = self.icon_set_label();
        let builtin = self.icon_set_enum.is_none();
        let fg = self.icon_cache_fg;

        let icons: Vec<_> = self
            .gpui_icons
            .iter()
            .enumerate()
            .map(|(i, (name, icon, role, data, source))| {
                let art = if builtin {
                    IconArt::Builtin(icon.clone())
                } else {
                    let image = self.gpui_icon_sources.get(i).cloned().flatten();
                    match (image, data, source) {
                        (Some(image), _, IconSource::System) => IconArt::System(image),
                        (Some(image), _, IconSource::Bundled) => IconArt::Bundled(image),
                        (_, Some(_), _) => IconArt::Unreadable,
                        (_, None, _) => IconArt::Missing,
                    }
                };
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
                    fg,
                )
            })
            .collect();

        let mapped = self
            .gpui_icons
            .iter()
            .filter(|(_, _, r, _, _)| r.is_some())
            .count();
        let title = format!(
            "gpui-component Icons ({} variants, {mapped} mapped to {set})",
            self.gpui_icons.len(),
        );

        v_flex()
            .gap_2()
            .child(demo::heading(ui, cx, "icons-heading-gpui", title))
            .child(div().flex().flex_wrap().gap_2().children(icons))
    }

    // -----------------------------------------------------------------------
    // Page: Icons
    // -----------------------------------------------------------------------
    pub(crate) fn render_icons_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        v_flex()
            .gap_5()
            .p_4()
            .child(self.render_icon_sizes_section(cx))
            .child(self.render_animated_icons_section(cx))
            .child(self.render_native_icons_section(cx))
            .child(self.render_gpui_icons_section(cx))
    }
}
