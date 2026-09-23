# v0.5.9: the gpui showcase's layout, compacted

Status: approved by the maintainer's request, 2026-09-23. Executed with superpowers:subagent-driven-development, every subagent on Opus.

## Rationale

The maintainer, after using the rearranged showcase: "lots of space is wasted. On the left panel, 'Icon set' is incorrect naming … it was called 'Icon theme' as icon set is a different thing. And below that, remove the menu, put there the content of the right panel (after a separator). The menu should be horizontal tabs on the top of the middle panel. The right panel should be eliminated."

**Naming.** The control chooses among:

- the TOML's default icon theme;
- the system theme;
- the installed freedesktop themes (Breeze, hicolor, …);
- gpui-component's built-in icons;
- Lucide;
- Material.

Each of these is an icon theme. The code's own comment before the module split called it the "Icon theme selector". In native-theme, `IconSet` names the loading mechanism (freedesktop, Material, Lucide, SF Symbols, Segoe). That word stays where it means the model's `IconSet`. What the user picks, and every text that names the pick, is an icon theme.

**Layout.** The window becomes two panels:

- **The left panel is everything about the theme.** It holds what the theme is (Theme, Mode, Icon theme) and what the widget under the pointer takes from it (the inspector).
- **The middle panel is the content.** Page navigation moves into tabs above it, which uses the width once spent on a separate navigation column and a separate inspector column.

**Consequences.**

- **The icon rail goes.** The left panel no longer holds page icons, so collapsing it to a rail shows nothing.
- **One toggle.** A single status-bar toggle shows or hides the whole left panel.
- **The inspector toggle goes.** It had nothing of its own left to hide, so its action, its Ctrl+I binding and its menu item go too.
- **The Sidebar widget needs a new demonstration.** The chrome no longer uses gpui-component's `Sidebar`, so the Layout page shows it as a sample again. A widget the showcase does not show fails the coverage gate. Showing it is truer than an exception.

## Spec

### S1. Window

From top to bottom:

- the title bar;
- the toolbar;
- the body: `h_resizable` of the left panel and the content panel, divided by a draggable handle;
- the status bar.

The inspector panel is gone. `NAV_WIDTH` and `INSPECTOR_WIDTH` are replaced by one named constant, `LEFT_PANEL_WIDTH`:

- It is 300, the width the inspector's content was laid out for.
- Its comment says that the model states no such width.
- A test asserts that the theme settings and the inspector fit at that width, under every offered preset at text scales 1 and 2. This extends the existing header fit test.

`WINDOW_SIZE` is `LEFT_PANEL_WIDTH` + 880 (the page's width).

### S2. Left panel

From top to bottom:

1. The three labelled rows: **Theme**, **Mode**, **Icon theme**. They keep their current controls, spacing rules and infos, and the third row's label changes.
2. A horizontal `Separator`, built by a `demo::` helper, which reports itself.
3. The inspector: its TabBar (Widget / Theme) and its content, unchanged apart from where it sits. It fills the remaining height, and its content scrolls with `geometry::scrollbar_gutter`, as the content panel's does.

The panel is a plain element, not a `Sidebar`, because a `Sidebar`'s children must be `SidebarItem`s (sidebar/mod.rs:211). The panel still reports itself with a true info.

### S3. Content panel

- A page `TabBar` sits above the page's scroll area.
  - It has one `Tab` per `Page`, with the page's label.
  - `selected_index` is the active page.
  - A click dispatches `ShowPage`.
  - `menu(true)` gives an overflow menu when the tabs overflow (tab_bar.rs:109-116).
- It is built by `demo::tab_bar` and reports itself, as the inspector's TabBar does.
- The View menu's page items and the command palette still show pages.

### S4. Status bar and actions

- The status bar holds, in order: the left-panel toggle, the environment text, and the shown info's title. The inspector toggle goes.
- The toggle:
  - uses `PanelLeft` from the chosen icon theme;
  - is `selected` while the panel is shown;
  - dispatches the renamed action `ToggleSidePanel`, with the View menu item "Toggle Side Panel" and Ctrl+B;
  - fully hides and shows the left panel. There is no rail.
- `ToggleInspector`, its Ctrl+I binding and its menu item are removed.
- The dragged width of the left panel survives hiding and showing it, as the dragged widths did before.

### S5. Sidebar sample

- The Layout page gains a "Sidebar" section: one expanded `Sidebar` and one collapsed `Sidebar`.
  - Each has a header text and three `SidebarMenuItem`s whose icons come from the chosen icon theme at `geometry::icon_size_small`.
  - Each item and each sidebar reports itself.
- Task 5's icon-fit test moves to this sample. That covers the icon inside its item, and the collapsed sidebar's items fitting its width.
- `SidebarToggleButton` stays excepted (it would mix icon sets).

### S6. Naming

- The third row's label is "Icon theme".
- Every user-facing text that names the user's pick says "icon theme": infos, notes, the Icons page heading and notes, the status bar, tooltips, the command palette, and the CHANGELOG entries describing the showcase.
- Texts that name the model's `IconSet` keep "icon set".
- Code identifiers may keep their names.

### S8. The window manager draws the frame where it will

The maintainer's question was why the window buttons and corners don't look native. The showcase requests client-side decorations (`main.rs` `window_options`), so gpui-component draws them:
- the controls come from `TitleBar` with gpui-component's own icons (title_bar.rs:148-151);
- the frame is square by design, `BORDER_RADIUS = 0` (window_border.rs:18-22);
- the shadow is its own, `SHADOW_SIZE` 20 (:14).

A native KDE app gets KWin's decoration instead. gpui can request server-side decorations over xdg-decoration and reports what the compositor grants (gpui-pre-linux 0.3.6 `wayland/window.rs:1150-1161`).

**Request.** The showcase requests `WindowDecorations::Server`.

**Render.** It renders by what it was granted, `window.window_decorations()`:
- **`Decorations::Server`:** no `TitleBar`. The OS draws the title bar, controls, corners and shadow; `Root`'s border draws nothing in that arm (window_border.rs). The application's menus sit in a menu-bar row at the top of the window, as KDE applications place them. On macOS they stay in the system menu bar, and there is no row. The toolbar follows.
- **`Decorations::Client`**, where a compositor refuses (GNOME's Mutter draws no decorations for Wayland clients): today's `TitleBar` with the menus, unchanged.

**Title.** The OS title is `WINDOW_TITLE`, already set (main.rs `set_window_title`).

**Keep what the frame change touches truthful:**
- The `TitleBar` is still demonstrated: the Layout page shows it as a sample when the window's own frame is server-drawn. The coverage gate stays honest.
- The inspector's Theme tab "Window" section states what is drawn in each mode: the OS frame, or `Root`'s client frame.
- Tests cover both arms, opening the test window once with each request.

Check upstream for the other platforms before writing code:
- Windows (gpui-pre-windows): what `Server` gives there. If it is the native DWM frame, use it.
- macOS (gpui-pre-macos): a standard titled window with the traffic lights.

Record what each platform grants in the report. Never guess.

### S7. Tests and docs

- Tests that relied on the chrome Sidebar, the rail, the inspector panel or its toggle are updated or removed with the feature. The TabBar's navigation gets a test: clicking a tab shows its page.
- The coverage script passes.
- CHANGELOG `[Unreleased]`: the entries describing the Sidebar navigation, the inspector panel, the two status-bar toggles, `NAV_WIDTH` 200 and the window width describe the new layout.
- `docs/todo.md`'s screenshot item names the new layout.
- The archived showcase-app and unstated-sizes documents get no edits: they record what was built then. This document is archived when done.

## Plan

### Task 1: the layout and the naming (S1–S6)

1. **Failing tests.**
   - The body has exactly two panels.
   - The left panel holds, top to bottom: the three rows, a Separator, then the inspector TabBar. They fit at `LEFT_PANEL_WIDTH` under every offered preset at text scales 1 and 2.
   - A page TabBar sits above the page, and clicking a tab shows that page.
   - The status bar has one toggle, which hides and shows the left panel and keeps the panel's dragged width.
   - No `ToggleInspector` remains.
   - The third row's label reads "Icon theme".
   - The Layout page's Sidebar sample is drawn, and its icons fit.
2. **Implement** S1–S6.
3. **Check.** Run the coverage script and `env CARGO_BUILD_JOBS=4 ./pre-release-check.sh`.
4. **Commit.** One commit: `feat(showcase): two panels — the theme and its inspector on the left, the page with its tabs in the middle`.

### Task 2: the window manager draws the frame (S8)

1. **Failing tests.**
   - With `Decorations::Server`, no `TitleBar` is drawn, and the menu-bar row (not on macOS) sits above the toolbar.
   - With `Decorations::Client`, today's `TitleBar` holds the menus.
   - The Layout page's `TitleBar` sample is drawn.
   - The Theme tab's Window section names the mode.
2. **Implement S8.** Read each platform's backend first.
3. **Check.** Run the coverage script and `env CARGO_BUILD_JOBS=4 ./pre-release-check.sh`.
4. **Commit.** One commit: `feat(showcase): the window manager draws the window's frame where it will`.

### Task 3: docs and archive (S7, S8)

- Make the CHANGELOG and `docs/todo.md` updates.
- Move this document to `docs/archive/` with an "As built" note.
- Commit: `docs: the showcase layout, implemented and archived`.

## As built

Implemented 2026-09-23 on branch `v0.5.9-gpui-kit-0.6.6`, every subagent on
Opus, and archived in the commit after them. Tasks 1 and 2 each passed review
after one fix round. The controller's rulings are in the git-ignored SDD
ledger.

| Task | Commits |
|---|---|
| 1 | `8d48141`, fix `f4e3256` |
| 2 | plan `db09df8`, `5158e23`, fix `ae55019` |
| 3 | the archiving commit |

What differs from the text above:

- **S3, the Alert's place.** The content panel is the page TabBar, then a
  theme error's Alert, then the page. Task 1 first put the Alert above the
  tabs, and two of its tests contradicted each other. The ruling: the page
  navigation must not move when an error appears. `a_theme_error_is_an_alert`
  asserts that the TabBar's bounds do not change when the Alert appears, and
  that the Alert sits between the tabs and the page.
- **S1, the fit test.** It runs over all 16 bundled presets
  (`Theme::list_presets()`), not only the ones the preset switch offers on
  the host. Each native preset is measured at its platform's DPI (macOS and
  iOS at 72, the others at 96) and each colour-scheme preset at the host's,
  at text scales 1 and 2. `default` is built on one of them. The test takes
  about 12s alone.
- **S8, what each platform grants.** This was read from gpui-pre 0.3.6's
  source. Nothing was run on Windows or macOS.
  - Wayland: Server where the compositor offers xdg-decoration and grants
    server-side, as KWin does (gpui-pre-linux `wayland/window.rs:2094-2113`,
    `:1150-1168`). Client where there is no manager, as under Mutter, and the
    showcase then draws its `TitleBar`.
  - X11: Server, requested through `_MOTIF_WM_HINTS`
    (`x11/window.rs:1900-1950`).
  - Windows and macOS: neither backend overrides the request, and both always
    report Server (gpui-pre `platform.rs:1006`, `:1020-1022`). Their native
    title bar is hidden only by `titlebar.appears_transparent`, which
    `TitleBar::window_options()` set. The window now takes
    `WindowOptions::default()` with `window_decorations: Some(Server)`, so
    both keep their native title bar. `TitleBar::window_options` is no longer
    used.
  - gpui's test platform: always Server.
- **S8, the Client arm's tests.** Opening the test window with each request
  cannot reach the Client arm, because the test platform grants Server to
  both. Both requests are tested, and both assert Server with the menu-bar
  row. The Client arm is reached through a `#[cfg(test)]` seam,
  `Showcase::frame_for_test`, which overrides what `Showcase::frame` reports.
  `Root`, `TitleBar` and `WindowBorder` still read the platform's real answer,
  so under the seam no window controls and no client inset are drawn.
- **S8, the menu-bar row's inset.** The model states no menu-bar inset. The
  row borrows `layout.container_margin` for its left and right padding, and
  its info says the inset is borrowed. Where that is unstated too, it takes
  `MENU_BAR_PADDING`, the showcase's own 8px. The row has no fill or font of
  its own.
- **S8, the TitleBar sample.** An occluding box lies over it and carries its
  info, so a drag or a double click on the sample neither moves nor zooms the
  window.
- **S3, the page TabBar's menu.** Upstream adds the menu Button whether or
  not the tabs overflow (gpui-component `tab/tab_bar.rs:554`), and the
  TabBar's info says so.
