# v0.5.9: the gpui showcase's layout, compacted

Status: approved by the maintainer's request, 2026-09-24. Executed with superpowers:subagent-driven-development, every subagent on Opus.

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

### Task 2: docs and archive (S7)

- Make the CHANGELOG and `docs/todo.md` updates.
- Move this document to `docs/archive/` with an "As built" note.
- Commit: `docs: the showcase layout, implemented and archived`.
