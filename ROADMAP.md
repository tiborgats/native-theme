# Roadmap

Pre-1.0 milestones. Priorities can shift between minor versions — this roadmap
is a snapshot of current direction, not a commitment.

See also:

- [CHANGELOG.md](CHANGELOG.md) — what has already shipped
- [`docs/archive/`](docs/archive/) — detailed design docs and phase notes from completed milestones

## v0.6.0 — Full theme geometry in the iced connector

Right now `native-theme-iced` maps **colors only**: radii, border widths,
padding, and shadows exist in `ResolvedTheme` but iced widgets ignore them
because iced applies geometry via per-widget inline configuration, not through
its `Catalog` color system. Apps that want platform-accurate geometry have to
read each metric helper (`border_radius`, `button_padding`, …) and wire it
manually per widget.

**Planned deliverable:** drop-in style-function replacements — e.g.
`native_theme_iced::button::primary(&resolved)` — that bake radius, border
width, padding, and disabled opacity into the returned `Style`. Apps keep
using `iced::Theme` as-is; geometry becomes a per-widget choice between
iced's built-in style and the native-theme style at runtime (no Cargo
feature flag, no wrapper type).

Detailed design: [`docs/todo_v0.6.0_iced-full-theme-geometry.md`](docs/todo_v0.6.0_iced-full-theme-geometry.md).

## v0.6.1 — Full per-widget geometry in the gpui connector

`native-theme-gpui` maps the 108-field `ThemeColor` palette and global
geometry (`radius`, `shadow`, `font_*`) well, but roughly **65 per-widget
geometry fields** in `ResolvedTheme` currently have no path to the rendered
UI because gpui-component's `Theme` / `ThemeConfig` has no fields to receive
them. A KDE Breeze button renders with the correct colors but the wrong
padding, height, and border stroke.

**Planned deliverable:** per-widget geometry mapping for the full widget
set — buttons, inputs, checkboxes, scrollbars, sliders, tabs, menus, tooltips,
dialogs, progress bars, switches, toolbars, lists, spinners, combo boxes,
splitters, separators, segmented controls, expanders, and layout metrics.
This requires coordinated work with gpui-component to expose receiving
fields; the design doc spells out the widget-by-widget gap analysis.

Detailed design: [`docs/todo_v0.6.1_gpui-full-theme.md`](docs/todo_v0.6.1_gpui-full-theme.md).

## Beyond v0.6

No milestone targets committed yet. Likely candidates:

- Live theme-switching stability work on Windows (COM STA lifecycle)
- Expanded preset coverage (platform themes for more macOS / Windows versions)
- Additional icon-set bundles if demand emerges
- v1.0 stabilization once the v0.6 connector parity is achieved and the API
  has stayed unchanged for a full minor version

Ideas and discussion land in the `docs/` folder and in GitHub issues before
being promoted to a roadmap line.
