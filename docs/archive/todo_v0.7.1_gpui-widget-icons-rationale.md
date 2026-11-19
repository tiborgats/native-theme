# gpui: the icons gpui-component's widgets draw follow the theme's icon set — rationale

Status: written 2026-11-14, revised after six reviews; approved by the maintainer
as amended, with its specification and plan, on 2026-11-19.

Amended 2026-11-19: the work is built inside 0.7.2 by the maintainer's decision
of that day (W11), and its upstream citations were read again at
gpui-component 0.10.0 and gpui-fast 0.1.16, where one statement was false (W3,
the loading button) and three notes are added (§1.2, W4, W8). The lines below
stay those of the versions and the commit the next paragraph names.

Abbreviations: `GC`, `GP`, `GF` are the `src/` folders of gpui-component 0.9.1,
gpui-pre 0.3.12 and gpui-fast 0.1.15 in Cargo's registry; `GA` is
gpui-kit-assets 0.9.1; `CN` is `connectors/native-theme-gpui`; `SC` is
`CN/examples/showcase-gpui`. Lines are read at those versions and at commit
`0fa28027`.

## 0. The problem

gpui-component's widgets build icons of their own as they render: the caret of a
`Select`, the check mark of a chosen row, a dialog's close button, the window
controls, an alert's status icon, and about fifty more places (§1.2). They are
gpui-component's bundled Lucide glyphs. An application that takes its own icons
from the theme's icon set (Breeze on KDE, Adwaita on GNOME, Material, ...) through
`native_theme_gpui::icons` therefore shows two sets side by side. The repository
states the rule this breaks in many places, for example the connector's module
doc: "An icon the set does not have is `None`; nothing is substituted"
(`CN/src/icons.rs:30`), and the showcase: "Icon themes are never mixed"
(`SC/support.rs:773-774`).

Until 0.9.0 an application could not make the widgets' icons follow a set it
switches at run time: a widget names its icon with no setter, and GPUI keeps a
drawn icon by its path (`docs/todo.md:3812-3836`). The backlog asked upstream for
a hook (`docs/todo.md:3972-4016`), and the showcase lists the widgets' own icons
as not themeable (`SC/info/chrome.rs:993-999`, `SC/info/mod.rs:444-448`).
gpui-component 0.9.0 added the hook: `set_icon_resolver` (`GC/icon.rs:77-97`).
This document decides how the connector and the showcase use it.

## 1. Facts

### 1.1 The hook

- `set_icon_resolver(cx, resolver)` stores one function for the application,
  `Fn(&str, &App) -> Option<SharedString>`, replacing any earlier one
  (`GC/icon.rs:77-97`). It receives an icon's asset path and returns the path to
  draw in its place, or `None` to keep it (`resolve_icon_path`, `:101-106`). The
  stored function cannot be read back, and calling `resolve_icon_path` from
  inside a resolver calls that resolver again.
- It is asked for every `Icon` that has a path: one built from an `IconName`,
  from `Icon::path`, or rendered as a view (`GC/icon.rs:140-151`, `:206-211`,
  `:260-281`), and for the two glyphs gpui-component draws as a raw `svg`: the
  check box's check mark (`GC/checkbox.rs:220-222`) and the questionnaire's
  (`GC/questionnaire/components.rs:866`). No other raw `svg` path is in
  gpui-component. So it covers the application's own `Icon::new(IconName::…)` as
  well as the widgets'.
- It is not asked for an `Icon` built from bytes (`Icon::data`,
  `GC/icon.rs:167-170`), for native menus on macOS and Windows
  (`GC/native_menu/mod.rs:226-256`), or for a `gpui_kit_assets::IconName`
  rendered by itself (`GA/src/icon.rs:19-27`).
- An `IconName`'s path is `icons/<name>.svg` (`GA/build.rs:48`); gpui-component's
  `IconName` has 106 variants (the lines of `GA/default-icons.txt`).
- The returned path is loaded through the application's one `AssetSource`
  (`GP/assets.rs:13-19`, `'static + Send + Sync`; installed with
  `Application::with_assets` before the application runs, `GP/app.rs:265-273`;
  `GP/svg_renderer.rs:255-261`). GPUI Kit installs none by itself; an application
  passes `gpui_kit::assets::Assets`, which holds the 106 default icons and
  returns an error for any other path (`GA/src/native_assets.rs:23-29`). The error is
  logged at each draw and nothing is drawn (`GP/elements/svg.rs:182-185`).
- An SVG is drawn as a mask: only each pixel's alpha is kept
  (`GP/svg_renderer.rs:231-262`), and the shape is painted in one colour, the
  icon's text colour (`GP/window.rs:5230-5292`; `GC/icon.rs:213-239`). A
  multi-colour SVG becomes a silhouette. gpui-fast does the same
  (`GF/svg_renderer.rs:231-262`, `GF/window.rs:5283-5315`).
- Both cores build their SVG renderer with support for an image embedded in the
  SVG (resvg 0.46.0 with `raster-images`, gpui-pre's `Cargo.toml:456-462`,
  gpui-fast's `:276-282`; usvg's default resolver accepts a `data:` reference
  to a PNG, JPEG, GIF or WebP image, `usvg-0.46.0/src/parser/image.rs:59-74`,
  and resvg decodes it, `resvg-0.46.0/src/image.rs:62-77`). Read, not run.
- A drawn SVG is cached by its path and size and never loaded again
  (`GP/platform.rs:1751-1757`, `GP/window.rs:5246-5262`), and an application
  cannot drop it (`docs/todo.md:3812`). The hook's documentation says what follows
  from that: "returning a new path is how an application switches icon styles at
  runtime"; and "The resolver runs every time an icon is drawn and should be
  cheap. Call `App::refresh_windows` after changing what it returns"
  (`GC/icon.rs:84-91`).
- An `Icon` without a size is drawn at the text size it inherits
  (`GC/icon.rs:224`); a sized one at 12, 14, 16 or 24 px at a 16 px font, or the
  caller's pixels (`:225-231`).

### 1.2 What the widgets draw

Read at 0.9.1, outside test code: the backlog's list (the sidebar toggle; the
`Select`, `Combobox` and dropdown caret; the chosen row's check mark; the empty
list's inbox; the window controls; the dialog's and the sheet's close button; the
command palette's search icon and check mark; the menu's check mark, external
link and submenu arrow; the Settings search icon; the number input's minus and
plus; the speech button's and the speech indicator's glyphs; the `Diff`'s arrows)
and more that the backlog does not list (accordion, alert, attachment, avatar,
breadcrumb, carousel, the check box, clipboard, the dock's tab panel, the input's
clear, mask and fold buttons, the OTP mask, the search bar, the inspector, the
list's empty state and search, the message scroller, notification, pagination,
the questionnaire, rating, the setting page's reset, the sidebar menu, spinner,
the table's empty state and sort arrows, calendar, date picker). The whole list
with lines is in the research of 2026-11-14 and goes into the specification.
At 0.10.0 the list gains the check box's indeterminate dash (`Minus`, through the
resolver like its check mark) and, under a right-to-left layout, `ChevronLeft`
in the breadcrumb, the submenu arrow and the sidebar menu's caret; nothing
leaves it.

### 1.3 The connector and the showcase today

- The connector hands an application an `ImageSource` for `gpui::img`, which
  keeps an icon's colours (`CN/src/icons.rs:752-913`). It has public tables from
  each `IconName` to the icon's name in Lucide, Material and the freedesktop
  naming (`lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`,
  `freedesktop_name_candidates`, `:210`, `:354`, `:557`): freedesktop names for
  all 106, some of them marked as the nearest name the naming has (`:552-554`);
  Lucide for all but `StarFill`; Material for all but `StarOff`. It has no
  resolver and no asset source.
- The connector's own `widgets::Checkbox` draws its check mark two ways. Where
  the theme states no stroke, a raw `svg` at `IconName::Check`'s path, without
  asking the resolver (`CN/src/widgets/checkbox.rs:430-436`). Where the theme
  states `checkbox.check_mark_stroke_width` (kde-breeze, adwaita and material-3
  do), the outline of Lucide's check icon, stroked at that width
  (`:303-310`, `:317-360`).
- The showcase already loads, for each of the 106 names, the chosen set's icon
  (`load_gpui_icons`, `SC/support.rs:653-769`), at each loader's default size:
  24 px for a freedesktop theme (`native-theme/src/icons.rs:226`, `:266-270`),
  32 px for Segoe Fluent (`:509-512`), 24 points for SF Symbols, whose pixels
  come at their natural size (`:423-424`):
  1. where the name has a role (the first role in `IconRole::ALL`'s order whose
     `icons::icon_name` is that name; 31 roles map to 29 names,
     `CN/src/icons.rs:141-196`; `SC/support.rs:521-526`), the set's icon for the
     role; in a freedesktop theme, failing that, the first of the name's
     freedesktop names the same theme has; in the other sets a name with a role
     stops here;
  2. where it has no role: in a freedesktop theme the first of its freedesktop
     names the theme has; in Lucide and Material the icon under the table's
     name;
  3. else none. No step looks in another theme or set. A freedesktop set whose
     icon theme cannot be detected loads nothing (`SC/support.rs:673-684`).
- The showcase has a second list beside that one: `load_all_icons`
  (`SC/support.rs:467-492`) loads the icon of each role, by the role, in every
  set. Some of the Basic page's icons are drawn from it (`role_chrome_icon`,
  `SC/app.rs:1022-1050`: its Copy, Paste and Delete buttons,
  `SC/pages/basic.rs:87-104`), and so is the Icons page's gallery of roles.
  The list of the 106 names serves the Icons page's gallery of `IconName`s
  and everything else the showcase draws by an `IconName`, through
  `chrome_icon` (`SC/app.rs:967-973`) and `sample_icon` (`:1055`): the
  chrome (the toolbar's Settings, `SC/chrome.rs:236`; the tab bar's caret,
  `:312`; the command palette's entries, `:451`; the theme-error alert's
  `CircleX`, `SC/app.rs:2225`; the dialogs' icons, `:2198-2199`) and the
  pages' samples (the Buttons page's icon buttons,
  `SC/pages/buttons.rs:60-79`; the Overlays page's menu rows,
  `SC/pages/overlays.rs:20-24`). Because the list of names tries the role
  first, the two lists give the same icon today for the 29 names that have a
  role wherever the role's lookup finds one; where it finds none in a
  freedesktop theme, the list of names goes on to the name's freedesktop
  names (step 1 above) and the list of roles has nothing (the palette's
  `Bell` under Adwaita is `alarm`, since the role's `notification-active` is
  a KDE name, `native-theme/src/model/icons.rs:780-781`). The iced and egui
  showcases draw their icons by role, and three by name with their own
  copies of the table's entries
  (`connectors/native-theme-iced/examples/showcase-iced.rs:4557-4575`;
  `connectors/native-theme-egui/examples/showcase-egui/chrome.rs:1348`,
  `:1384`, `:1495-1517`; `demo.rs:915-950`).
- The connector has a public function, deprecated since 0.7.0, that gives
  one freedesktop name for an `IconName` by the desktop: the last of the
  table's names on a GTK desktop, the first elsewhere
  (`freedesktop_name_for_gpui_icon`, `CN/src/icons.rs:498-520`). Every entry
  with more than one name is today Breeze's name, then GTK's, which the
  table's documentation says (`:526-527`) and two tests rely on, loading the
  function's answers from the installed Breeze and Adwaita (`:2741-2775`,
  `:2780-2802`).
- A freedesktop theme can give pixels for an icon as well as an SVG
  (`ChromeIcon::Raster`'s doc, `SC/support.rs`).
- The showcase's chrome draws an SVG from that list as
  `Icon::default().data(bytes)`, a mask in the text colour; pixels through
  `gpui::img`; nothing where the set has none (`ChromeIcon`,
  `SC/support.rs:831-852`, `SC/demo.rs:133-149`). The widgets' own icons stay
  Lucide, which the Widget Info says.
- The showcase installs `gpui_kit::assets::Assets` (`SC/main.rs:1092-1093`).
- A loaded theme names its icon set and, for the freedesktop set, the icon theme
  of its mode: `native_theme::Resolved::icon_set` and `icon_theme`
  (`native-theme/src/model/resolved.rs:263-283`). `ResolvedTheme`, which is what
  the connector's `apply` receives (`CN/src/lib.rs:1158-1163`), does not carry
  them; `SystemTheme`, which holds the icon theme of each mode
  (`native-theme/src/lib.rs:688-748`), and the connector's `Loaded` do
  (`CN/src/lib.rs:494-499`). `from_preset` returns a `ResolvedTheme` and so drops
  them (`CN/src/lib.rs:445-475`).
- After `apply_system_theme` an application can switch between light and dark
  through gpui-component alone, with no further call to the connector, whose
  observer restores the rest for the new mode (`CN/src/lib.rs:1112-1121`).
- Ten of the seventeen bundled presets name the Lucide set. gpui-kit-assets
  describes its catalogue as Lucide 1.43.0 plus a few icons of GPUI Kit's own
  (its README, line 51); native-theme bundles Lucide 1.41.0
  (`native-theme/icons/SOURCES.toml:13`). Of the 106 default glyphs, 96 have the
  same shapes as the native-theme Lucide icon the connector's table by name
  maps them to (`lucide_name_for_gpui_icon`). Nine differ: the four window
  controls, the resize corner, the two sort arrows and the GitHub mark are not
  Lucide drawings, and the table maps them to Lucide icons of the same meaning
  (a close button to `x`, minimize to `minus`); the inspector is a Lucide icon
  that the table maps to another one. `StarFill` has no entry
  (`CN/src/icons.rs:305-309`): upstream's is an earlier Lucide star with a
  fill, and gpui-component's `Rating` draws it (`GC/rating.rs:162`).
- The lookup by role, which the showcase tries first, gives another icon than
  the connector's table by name for a few names of the bundled sets (the 31
  roles compared one by one). Under Lucide: the window's minimize, the role's
  `minimize`, which in Lucide is the glyph for leaving full screen, against
  the table's `minus` (`native-theme/src/model/icons.rs:866`;
  `CN/src/icons.rs:320`); and `IconName::Delete`, the role's trash can against
  the table's `delete`, the backspace glyph upstream draws. Under Material:
  the four chevrons, for which the roles give arrows and the table chevrons
  (`native-theme/src/model/icons.rs:823-826`; `CN/src/icons.rs:377-381`), and
  the error circle (`error` against `cancel`). For a window's maximize and
  restore both lookups give the same icons in both sets, the full-screen
  glyphs (Lucide's `maximize` and `minimize-2`, Material's `open_in_full` and
  `close_fullscreen`).
- In the freedesktop naming the two lookups try the same name first for 26 of
  the 29 names that have a role (the two tables compared name by name; what an
  installed theme draws under a name was not read). Three differ:

  | `IconName` | The role's name | The table's candidates |
  |---|---|---|
  | `Check` | `emblem-default` (`native-theme/src/model/icons.rs:775`) | `dialog-ok`, `object-select` (`CN/src/icons.rs:629`) |
  | `CircleCheck` | `object-select-symbolic` (`:728`) | `emblem-ok-symbolic`, `object-select` (`CN/src/icons.rs:630`) |
  | `Loader` | `content-loading-symbolic` (`:774`) | `process-working`, `content-loading` (`CN/src/icons.rs:640`) |

  What the repository records about them:
  - `Check`. `object-select-symbolic` is recorded as the check mark of a
    chosen row on both desktops: GTK's drop-down marks the chosen item with it
    (`docs/todo_v0.8.0_model-completeness-gap-map-gnome.md:381`) and Kirigami
    Addons the current row
    (`docs/todo_v0.8.0_model-completeness-gap-map-kde.md:1387`); Adwaita and
    Breeze both ship it (`native-theme/src/model/icons.rs:725-726`).
    `dialog-ok` is recorded as the icon of KDE's OK button (the gap map,
    `:433`), not as a check mark drawn in a row or a box; the table has it
    first as Breeze's name, an order a test pins (`CN/src/icons.rs:3016-3026`).
    `emblem-default` has no record of being drawn anywhere.
  - `CircleCheck`. The table has `emblem-ok-symbolic` first as Breeze's name
    (the group's heading, `CN/src/icons.rs:617-618`). `emblem-ok` was tried
    for the role and is absent from Adwaita
    (`native-theme/src/model/icons.rs:725-727`). The by-name loader tries a
    name's symbolic form first (`native-theme/src/freedesktop.rs:341-345`), so
    in a theme without `emblem-ok-symbolic` the table comes to
    `object-select-symbolic` too, the role's name, which is the plain check
    mark of the entry above, not a check in a circle.
  - `Loader`. gpui-component's `Spinner` rotates `IconName::Loader` unless it
    is given another icon (`GC/spinner.rs:25`, `:35`, `:60-70`), and a loading
    button draws a `Spinner` (`GC/button/button_icon.rs:118-126`). On KDE
    the icon that is rotated as the spinner is recorded:
    `process-working-symbolic` in Qt Quick's busy indicator (the KDE gap map,
    `:442`; `KBusyIndicatorWidget` rotates the plain `process-working`,
    `:280`), the table's first name in its symbolic form, which the core's
    own spinner loader calls "always single frame"
    (`native-theme/src/freedesktop.rs:665-666`). Under its plain name the
    icon is in Breeze a strip of 15 animation frames (`:273-276`), which is
    why the loader tries the symbolic form first. On GNOME no icon of the
    theme is the spinner: GTK 4.24's spinner plays an animated drawing of its
    own (the GNOME gap map, `:372`, which lists the facts' note at
    `docs/platform-facts.md:1604-1605`, "KDE and GNOME use a rotating
    `process-working-symbolic` icon", as to be corrected, `:669`), and
    Adwaita has no `process-working` icon in either form
    (`native-theme/src/model/icons.rs:1393-1395`;
    `docs/archive/v0.4.0-animated-icons.md:37`; the 0.7.0 review's check of
    the installed theme, `docs/archive/todo_v0.7.0_code-review-findings.md:9416-9418`).
    That is why the role's name was moved to `content-loading-symbolic`,
    described as a three-dot loading indicator that Adwaita and Breeze both
    ship (`native-theme/src/model/icons.rs:769-774`; `CHANGELOG.md:1068`).
    `LoaderCircle` has no role; its table entry is the same as `Loader`'s
    (`CN/src/icons.rs:641`), and no widget of gpui-component 0.9.1 draws it.
    The connector's own `widgets::Spinner` does not draw `IconName::Loader`:
    it draws the icon set's animated indicator where it has one (Breeze's
    frames) and the theme's arc otherwise
    (`CN/src/widgets/spinner.rs:364-372`).

  Not every role's name is the naming specification's: `notification-active`
  is a KDE convention (`native-theme/src/model/icons.rs:780-781`).
- For SF Symbols and Segoe Fluent the showcase's lookup goes by role only
  (`SC/support.rs:698-767`), so it covers at most the 29 names that have a
  role; `Close`, which a dialog, a sheet, a notification, an alert and the
  input's clear button draw, is not among them.
- The showcase's Select has, besides its sets, a row that follows the theme
  and a row for the system's own set, whose icon theme is detected at each
  load (`native-theme/src/icons.rs:1329-1339`, `:1389`). While the row follows
  the theme, the showcase takes `default_icon_choice`, which falls back to the
  system's set where the theme's set or icon theme is not available on the
  machine (`native-theme/src/icons.rs:1439-1457`; `SC/app.rs:1172-1180`).
- gpui-component's widgets size some icons above the ladder: an empty list's or
  table's inbox at 48 px (`GC/list/delegate.rs:83`, `GC/table/delegate.rs:195`),
  a select's at 28 px (`GC/select.rs:284`). GPUI rasterises an SVG at twice the
  device pixels it is drawn at (`GP/window.rs:5247-5251`,
  `GP/svg_renderer.rs:81`).
- `docs/platform-facts.md:1242` records how each platform draws a check box's
  mark: Breeze paints a line, Adwaita and Material have a mark in their own
  style sheet, Windows draws a glyph of its icon font.
- The connector's `svg-rasterize` feature, on by default (`CN/Cargo.toml:30`,
  `:38`), gives it `native_theme::rasterize::rasterize_svg`
  (`native-theme/src/lib.rs:331-332`).

## 2. Decisions

### W1 — Who installs the hook: the connector, as public API

**Options.**

- (a) Record that the hook exists; an application writes its own resolver.
- (b) The showcase alone installs a resolver of its own.
- (c) The connector offers it; the showcase uses the connector's.

**(c).** The icon set is theme data (`Resolved::icon_set`), and a connector that
can carry theme data to the toolkit and does not is incomplete. (a) leaves every
application to rebuild what the showcase already has: the lookup per icon name,
an asset source and the path scheme. (b) proves the mechanism and keeps it out of
applications' reach. The lookup the showcase has (`load_gpui_icons`) moves into
the connector, and the showcase calls it there, so there is one
implementation. It changes for both callers in what W3 decides: under Lucide,
Material and a freedesktop theme it goes by the connector's table alone, and
two entries of the freedesktop table change; what the showcase itself draws
under a name that has a role takes the role's icon first, as today, and so
keeps its icons (W3, "The showcase's two lists"). And it takes three things the callers give
differently: the size to look up at (W9); whether only an icon the hook can carry counts as
found (W4's passing over); and the colour a loader may paint a symbolic icon
in (`color_opt`, `native-theme/src/icons.rs:288-293`), which the showcase
gives for the icons it draws as images and the handle does not give, since a
mask takes the shape alone. The handle asks at its own size for icons the
hook can carry; the showcase's chrome and Icons page ask as they do today and
take the first icon found. So for one name the widgets and the showcase's own
icons can show two drawings of the same theme: another size's; where the
first candidate is not one the hook can carry, another candidate's; and,
where the showcase draws by the role and role and table differ (W3), the
role's (under Material the showcase's own expander and number-input carets
are the role's arrows, `SC/pages/basic.rs:615-616`, `:780-783`, beside the
widgets' chevrons).

The showcase's chrome can draw an icon in several colours or in pixels as an
image only where its widget takes any element. Where a widget takes an `Icon`
and nothing else (a toggle's, a command's, an alert's, a stepper's and a
sidebar item's icon, a button's icon beside its label;
`ChromeIcon::svg_only`, `SC/support.rs:874-885`) a pixel icon is drawn as
nothing today, and with W6's test an SVG in several colours is too, where it
is a silhouette today. Letting those slots ask as the handle does was weighed
and not taken: the showcase would load a second list for a case that is not
known to occur, and drawing those slots through the handle's own paths would
leave them with upstream's glyph wherever the handle declines the set. The
measurement lists the slots that would lose an icon in a judged theme, and
one such slot stops the plan for the maintainer (W3, "The measurement").

### W2 — The mechanism: one handle, an asset source around the application's, and the resolver

The resolver must answer at draw time without I/O, the bytes must come from the
application's one `AssetSource`, and that source is given to GPUI before the
application context exists. That fixes the shape; the options are in how much
the application writes.

- (a) Free functions: the application writes its own `AssetSource` and resolver
  and calls the connector for the bytes of each icon.
- (b) One handle the application creates first. It gives an `AssetSource` that
  wraps the application's own (answering the connector's paths, passing every
  other path through), installs the resolver, and is told which icon set to
  serve.

**(b).** With (a) each application writes the asset source, the resolver and the
path rule of W5 again. The application's side of (b): create the handle, wrap
the asset source with it, and install it in the application context before the
first theme is applied. Installing stores the handle where the connector's
functions find it, so no public function changes its signature.

- **Installed once.** The handle sets the resolver when the application installs
  it and never again. A later change of set changes what the same resolver
  answers.
  Installing it replaces a resolver the application set before, which the
  README says.
- **A public lookup.** The hook holds one function and cannot be chained (§1.1),
  so the handle has a public method that answers, for a path, one of three: the
  path to draw; "the set has none for this name", with the empty icon's path;
  or "not mine" (not one of the 106; no set is being served; or the one name
  W4 leaves to upstream under Lucide). An application
  that wants its own rule sets its own resolver after the handle's and calls
  this method in it.
- **Two things it holds, and three states.** The handle always holds *the
  theme's icons*: a set and the icon theme of each mode, as it was last told,
  whatever state it is in. Its state only chooses what it serves:
  - *following the theme* (the state after installing): the theme's icons;
  - *a fixed set*: a set and icon themes of the application's own choice, which
    telling it the theme's icons does not change;
  - *off*: nothing; the widgets draw upstream's glyphs.
  A method for each state puts the handle into it; going back to following
  serves the theme's icons as last told, and the windows are refreshed.
  Without the states, the last call would win, and an application that chose a
  set by hand would lose it at the next system theme update, which a watcher
  delivers for any change of the desktop's theme. The states would not be
  needed if no connector function told the handle and the application always
  did; that was weighed against the next point and not taken, because it makes
  every application write the line that follows the theme's icons, and get the
  mode switch wrong without it.
- **How it is told the theme's icons.**
  - `apply_system_theme` has the `SystemTheme`, with the icon set and the icon
    theme of each mode (§1.3), and tells the handle itself. An application on
    the quick start's first path writes no line for it.
  - `apply` receives a `ResolvedTheme`, which has no icon set, and cannot tell
    the handle without a breaking change of its signature. An application on
    that path tells the handle itself, with one method that takes the set and
    the icon theme of each mode; a `Resolved`, a `SystemTheme` and the
    connector's `Loaded` each convert to that. A `Resolved` and a `Loaded`
    carry one mode's icon theme, which is taken for that mode: the other
    mode's is kept while the set stays the same, and is that one's too when
    the set changes, until it is told. An application that applies both
    variants of a theme tells the handle once for each, as it calls `apply`
    once for each. A preset's `Resolved` comes from resolving
    it with native-theme, as the README's example for this path shows;
    `from_preset`, which hides it, stays as it is.
  - What "the theme's icons" should be when the theme names a set the machine
    does not have is the application's choice, not the handle's: the handle
    serves what it is told (and W4 says what happens when that gives nothing).
    native-theme has a function for the choice, `default_icon_choice`, which
    falls back to the system's set (§1.3). The README's rule is one sentence:
    tell the handle the set the application draws its own icons from.
- **Following the mode.** Where the two modes' icon themes differ, the handle
  loads both, and the resolver answers from the table of the mode
  gpui-component's theme is in, which it reads from the application context it
  is given, the same source the connector's observer uses
  (`CN/src/lib.rs:998-1002`). A switch between light and dark that goes through
  gpui-component alone (§1.3) is then followed without a call, as the observer
  follows it for everything else.
- **Loading.** A set's tables are filled when the handle starts to serve it,
  not at draw time and not while it is only held: for each of the 106 names,
  the lookup of W3. Serving the set and icon themes that are already loaded
  does nothing.
- **Sharing.** The asset source and the resolver read one table from any
  thread, and a change of set replaces it whole. The specification picks the
  means; it has no path that can panic (no lock whose poisoning is unwrapped).
- **Compiled only with `svg-rasterize`.** W6 needs the rasteriser. The handle
  exists only with the connector's `svg-rasterize` feature, which is on by
  default, so there is no second behaviour at run time; what
  `apply_system_theme` does for the handle is compiled with it too. The README
  says the handle needs the feature, and the gate of the README's compiled
  examples (`CN/src/lib.rs:277-282`) gains it. A set whose own feature is off
  (`lucide-icons`, `material-icons`, `system-icons`, `CN/Cargo.toml:30-37`)
  gives nothing, and W4's rule for such a set applies.

### W3 — Which paths the resolver answers

The 106 paths of gpui-component's `IconName`, whoever draws them through
gpui-component's `Icon`: a widget, the application's own
`Icon::new(IconName::…)`, or an `Icon` given the same path by any other name.
Any other path (an application's own asset, an icon of the
full Lucide catalogue outside the 106) is not the connector's: it has no table
for it.

For each of the 106, which icon of the set is served:

- **In a freedesktop theme**: the connector's table by name alone: the first
  of the name's freedesktop names the theme has, in the by-name loader's order
  (the theme, then the themes it inherits from; in each, the names in the
  table's order, each in its symbolic form first,
  `native-theme/src/freedesktop.rs:336-348`). For the handle, an icon the hook
  cannot carry is passed over for the next in that order (W4). The reasons
  are below.
- **In Lucide and Material**: the connector's table by name alone. The options
  were the showcase's order, the role first, or the table. Where the two
  differ (§1.3), the role first changes what a glyph means: under Material a
  select's caret and an accordion's, a pagination's and a calendar's chevrons
  would become arrows; under Lucide the window's minimize would become the glyph for
  leaving full screen, and a backspace a trash can. The table is written name
  by name for gpui-component's glyphs. It is not perfect either: for a
  window's maximize and restore it gives the sets' full-screen glyphs, as the
  roles do (W4 states the cost), and a few names no widget draws have a loose
  match. What this changes in the showcase is under "The showcase's two
  lists" below.
- **In SF Symbols and Segoe Fluent**: none; W7.

**The freedesktop order.** The options were the showcase's present order (the
role's name, then the table's), the table's names then the role's, and the
table alone. The role's name and the table's first name are the same for 26
of the 29 names that have a role, so the choice is about the other three
(§1.3), and on what the repository records each of them goes to the table:

- `Check` is one of W4's essential glyphs: the mark of gpui-component's own
  check box (`GC/checkbox.rs:220-222`), of a checked menu row and of a chosen
  row. The table's `object-select` is recorded as the check mark both
  desktops draw in a chosen row; the role's `emblem-default` is recorded
  nowhere, and an emblem's name may well be a badge.
- `Loader` is what gpui-component's spinner rotates, and the icon KDE
  rotates is the table's first name in its symbolic form. The role's name
  would make that spinner a three-dot indicator, turning, under Breeze too.
- `CircleCheck`: the table's first name is the one it records as Breeze's
  for a check in a circle; the role's name is the plain check mark.

Under Adwaita the table and the role come to the same icons for these two
names, by the records of §1.3, and they are the nearest the theme has, not
the glyph: `CircleCheck` is the plain check mark, and `Loader` is the
three-dot indicator, which gpui-component's spinner and a loading button then
turn. That picture is accepted here and said in §3. The other ways were
weighed: an entry without `content-loading` would draw no spinner at all
under Adwaita, a control that vanishes while it should show progress; and
upstream's own glyph for this one name would be mixing by the connector's
choice, a third exception for a case the theme can answer with its own
loading icon. An application that wants the desktop's spinner uses the
connector's `widgets::Spinner`, which draws the theme's animated indicator
or arc (§1.3); the two spinners differ as the two check boxes of W8 do, and
the README's limits say so.

**The table alone**, as under Lucide and Material: one rule for every set the
handle serves, by the same argument (the table is written name by name for
gpui-component's glyphs), with no entry per name and no second lookup. The
role's name adds nothing to it but for `Check`: for the other 28 names with a
role the table's candidates already hold the role's name. For `Check` a
theme that has `emblem-default` and neither of the table's names no longer
gives the widgets a check mark, and, the name being essential, is not served
(W4); no such theme is recorded, and the measurement's listing would show
one.

The table's entries, each a Breeze name and then a GTK name, are read by
more than this lookup: the deprecated public function that picks one name by
the desktop, and the tests that load its answers from the installed themes
(§1.3). So an entry is changed only where a record requires it, and keeps
that form:

- `Check` stays `dialog-ok`, `object-select`. The records favour the second
  for a chosen row, and say of the first only that it is a button's icon;
  whether Breeze's `dialog-ok` is a plain check mark was not read. So today's
  entry gives the widgets `dialog-ok` under Breeze, which has both names
  (`CN/src/icons.rs:3024`), and `object-select-symbolic`, the recorded check
  mark, under Adwaita, which lacks `dialog-ok`
  (`docs/archive/todo_v0.7.0_code-review-findings.md:9417`): only Breeze's
  drawing is open. Changing the entry now has costs: `object-select` first
  would make the deprecated function answer `dialog-ok` on a GTK desktop,
  which Adwaita lacks, so the test that loads its GTK answers from the
  installed Adwaita (`:2780-2802`) would fail, and the test that pins the
  order (`:3016-3052`) and the documented form would change; the role's name
  as a third would make that function answer `emblem-default` on a GTK
  desktop, a name recorded nowhere. It is decided on the measurement
  (condition 3): if Breeze's `dialog-ok` is not a plain check mark, the
  entry is changed then, with those consequences worked through.
- `Loader` and `LoaderCircle` become `process-working-symbolic`,
  `content-loading`: the symbolic form named outright, so that the plain
  name's strip of frames cannot be reached. It is still Breeze's name, then
  GTK's. The deprecated function's answer for the two names on a
  desktop that is not GTK becomes the symbolic name: where a theme has the
  symbolic icon a by-name loader finds the same file as before, and where it
  has only the strip under the plain name, the old answer loaded the strip
  and the new one loads nothing; the change is said in the CHANGELOG. The
  fixture's rows for the two names change with it (`:2941-2942`), and the
  test that loads the function's answers from the installed Breeze
  (`:2741-2775`) then checks, on the maintainer's machine and wherever
  Breeze is installed, that Breeze has `process-working-symbolic`.

For the 26 names the change of loader, from the role's to the by-names one,
moves an icon in two cases only, both about a theme and one it inherits
from: a theme with a plain name whose parent has the symbolic one
(`native-theme/src/icons.rs:243-247`), and a theme with a later name of the
entry whose parent has an earlier one (`Bell`: the theme's own `alarm` then
comes before a parent's `notification-active`). In both the theme's own icon
is taken where the parent's is today, which is the by-names loader's rule.

**The showcase's two lists.** The handle answers for `IconName`s; an icon an
application loads by a role is the application's own and stays by the role,
in every set. The showcase has both kinds (§1.3), and today they agree for
the 29 names with a role, wherever the role finds an icon, because the list
of names tries the role first. With the table alone they would part where
role and table differ: under Lucide a Delete button would be a trash can on
the Basic page and gpui-component's backspace glyph on the Buttons page and
in a menu row, and the iced and egui showcases, which draw these by role, would
show the trash can in both. So everything the showcase itself draws by an
`IconName` that has a role, through `chrome_icon` and `sample_icon`, the
chrome and the pages' samples alike, keeps today's order: the role's icon,
and where the role gives none, the name's icon from the list of names. The
role alone was the other way; it would lose the icons the fallback finds
today (the palette's `Bell` under Adwaita). So the showcase's own icons stay
what they are today in every set, with one exception: a theme in which the
role finds no `content-loading-symbolic` and today's fallback lands on the
strip under the plain `process-working`; there `Loader` becomes the theme's
plain `content-loading` if it has one, and nothing otherwise. And the three
showcases stay alike. The list of names, by the table alone, serves what
shows the widgets' own glyphs: the Icons page's gallery of `IconName`s,
which follows the handle's lookup and so changes where role and table differ
(under Lucide `Delete` and the window's minimize; under Material the
chevrons and the error circle; in a freedesktop theme `Check`,
`CircleCheck`, `Loader`, `LoaderCircle` and the two inheritance cases
above), and whatever the showcase draws under a name that has no role, for
which the table is today's lookup already. Under SF Symbols and Segoe Fluent
the connector has no table (W7), so its lookup gives nothing there, and the
gallery takes the role's icon from the list of roles for the names that have
one, which is what it shows today; no rule of the showcase goes into the
connector's function.

One sample changes on purpose. The Buttons page's loading buttons, two since
the GPUI Kit 0.10.0 port, one with an icon and one without, give the spinner an
icon of the showcase's own, the role's, because the widget's default was
upstream's glyph. With the handle the default follows the set, so neither
overrides the spinner's icon where the handle serves the set and its `Loader`;
otherwise they would show, beside the Feedback page's spinner
(`SC/demo.rs:5752`), the very picture this decision rejects for Breeze. The
first button keeps its own icon, so the page still shows a spinner standing in
for one. (As written on 2026-11-14 this paragraph said that gpui-component
draws no spinner on a button without an icon; from 0.10.0 it does,
`button/button.rs`, `RenderOnce for Button`.)

One difference between the showcases stays and is said: a widget's own glyph
in gpui (a select's caret, by the table: a chevron under Material) against
the caret the iced and egui showcases draw by the nearest role (`NavDown`: an
arrow under Material). The core has no role for a caret; the backlog gets an
entry for it.

The role's own freedesktop names in the core (`StatusCheck`'s
`emblem-default`, `StatusBusy`'s `content-loading-symbolic`) are not changed
here: they are what every user of a role loads, in iced and egui too, and are
reviewed on the measurement, with a backlog entry.

What would reverse an entry: the measurement (below) showing, in a judged
theme, that the first name that is found does not draw the glyph (a plain
check mark; a check in a circle; a single spinner frame) and a later name of
the entry does.

**The measurement.** It is the plan's first stop, placed after the parts it
needs (the lookup with the core's new method, W4, and W6's test) and before
the handle is built. Through the library itself it lists, as a table that is
committed with the plan's record (for each name the icon's name and theme,
its size, whether it is one colour, and a hash of its bytes) and as images
that are not committed (drawings of an installed icon theme are that theme's
artwork under its own licence; they are made in a scratch folder, read there,
and shown to the maintainer at a stop):

- for each judged theme, the 106 names with the icon the handle would serve
  (16 px, one colour, after passing over), and the same at 24 px (W9);
- for `Check`, `CircleCheck`, `Loader` and `LoaderCircle`, the icon under
  each name of the entry and under the role's own name (`emblem-default`
  for `Check`);
- a theme's light and dark variants side by side (W5);
- the showcase's `Icon`-only slots whose icon is in several colours (W1).

Judged are the freedesktop icon themes a bundled preset names and the
session's own system theme; one of them that is not installed is named in
the listing as not judged. Any other installed theme is listed without being
judged, and a theme searched only as a parent (hicolor, a legacy raster
theme) is not judged on its own. The bundled Lucide and Material sets are
listed too, at the one size they have, for W4's four window controls and the
gaps known without measuring (Material has no `StarOff`); conditions 1 and 2
apply to them as well. The session reads the
listing against the conditions below and records it with the plan. The plan
stops for the maintainer, with the images, where one is met:

1. a judged theme lacks an essential glyph, so that it would not be served
   (W4);
2. a judged theme leaves more than half of the 106 empty (W4);
3. for `Check`, `CircleCheck`, `Loader` or `LoaderCircle`, the first name
   found in a judged theme does not draw the glyph while a later name of the
   entry does, or two judged themes would need different orders. Where no
   name of the entry draws the glyph, the nearest name stands and the
   listing says so; the two cases known by record are Adwaita's (above).
   For `Check` under Breeze this is where the entry's order is decided;
4. `Loader` or `LoaderCircle` is a strip of frames in a judged theme, or
   Breeze has no `process-working-symbolic`, which §3 takes as given;
5. W9's condition on the size fails;
6. one of the showcase's `Icon`-only slots would lose its icon in a judged
   theme (W1);
7. the handle's load of a judged theme takes longer than the showcase's own
   reload of its icons does today for that theme (§4).

Otherwise the work goes on, and the listing is part of what the maintainer
sees before the release.

One consequence is said in the README: an application that serves artwork of its
own under one of those 106 paths gets the set's icon there once the handle is
installed and a set is given.

### W4 — An icon the set does not have: nothing is drawn

**Options**, for a name whose lookup gives none while the set gives others:

- (a) Keep upstream's Lucide glyph.
- (b) Draw nothing: the resolver returns the path of an empty SVG.
- (c) Use the hook only where the set has every icon; otherwise leave all 106 as
  upstream's.
- (d) Let the application choose between (a) and (b).

**(b).**

- (a) is substitution, which the project's rule forbids; it is the state the
  hook exists to end, kept for part of the icons.
- (c) is all Lucide whenever one icon is missing, which for a freedesktop theme
  is likely always: the feature would rarely switch on.
- (d) adds an option whose one value breaks the rule. An application that wants
  upstream's glyph for some names writes its own resolver over the handle's
  lookup (W2); the connector's own behaviour leaves the rule only in the two
  stated cases (the floor below, and W7).
- The cost is real and is stated: a button whose only content is the missing
  glyph (a password field's show-and-hide button, a carousel's arrows, a
  sidebar's toggle) is drawn empty, and an alert or a notification loses its
  status icon where the theme's is in several colours (W6). The glyphs no
  application can do without are kept from this by the floor below. So the
  handle reports what it serves: after a load, which names the set did not
  give; that a set gave nothing at all; that it lacks an essential glyph, and
  which; or that a set is not served in this step (W7). The README says to check that report for the
  sets an application supports. The showcase shows it on its Icons page, where
  it already marks an icon the set lacks.

**A set that gives nothing at all.** Where the lookup gives no drawable icon for
any of the 106 names (a freedesktop set whose icon theme is not detected, or a
set not available on the platform), the handle serves nothing: the lookup
answers "not mine" for every path, the widgets keep upstream's glyphs, and the
report says the set gave nothing. Emptying every glyph of every widget would
remove far more than the rule protects, and "nothing was loaded" is a failure to
report, not an icon set to follow.

**A set that cannot draw the controls.** The same reason reaches further than
the empty set. An application cannot test its users' icon themes, and a theme
whose icons are mostly in several colours, or mostly pixels, would by W6 and W7
give a handful of the 106 and leave the rest empty: on a user's machine the
close buttons, the carets and the menu's check mark would be gone, and the
report reaches only the developer. The options: draw whatever the set gives
and leave the rest empty, however little it is; or serve a set only where it
gives the glyphs no application can do without; a floor by the number or the
share of icons given was not taken, since any such number would be made up and
would protect no control in particular; and keeping upstream's glyph for a
missing essential alone, with the rest of the set served, was not taken
either, since it is (a) for those names: one Lucide caret among a theme's
icons, drawn for good. **The second.** The essential glyphs
are a closed list of nine names, the ones nearly every application's windows,
dialogs and forms draw: the four window controls (`WindowClose`,
`WindowMinimize`, `WindowMaximize`, `WindowRestore`), the close button of a
dialog, a sheet and a notification (`Close`), the caret of a select and a
combo box (`ChevronDown`), the check mark of gpui-component's check box, of a
chosen row and of a menu item (`Check`), and a number input's `Minus` and
`Plus`. From gpui-component 0.10.0 `Minus` is also the dash of a check box in
the mixed state. Lucide, Material and the
freedesktop naming have an icon for each (`CN/src/icons.rs:232-321`,
`:376-463`, `:564-632`). A set, in the table of either mode, that lacks one of
them is not served, as a set that gives nothing is not: the widgets keep
upstream's glyphs, and the report says which essential glyph was missing.

An application whose windows need another list (one without a title bar of its
own, for one) gives the handle its own, which takes the place of the nine: any
of the 106 names, in any number. An empty list is allowed and means no floor:
the set is then served with whatever it gives, and only the rule for a set
that gives nothing at all remains, which no list removes. That rule, too, is
judged in the table of each mode: a set whose table for either mode is empty
is not served.

The floor is mixing by the connector's own choice, the second stated exception
to the rule beside W7's, made in the one case where the alternative is a
control the user cannot see; the README's limits say so. A set that has the
essentials and lacks others is served with those others empty, as the rule
asks.

An icon the theme has but the hook cannot carry (several colours, W6; pixels,
W7) is passed over for the next in the lookup's order in the same theme (W3)
before the name counts as not given, so that one unusable drawing of a check
mark does not decline a theme that has a usable one under another name. Among
the nine only `Check` (`dialog-ok`, `object-select`) and
`Close` (`tab-close`, `window-close`) have a second name to try; the other
seven have one, and for them the next is the name's plain form in the same
theme, then the same name in a theme the theme inherits from
(`native-theme/src/freedesktop.rs:341-347`). Lucide and Material have one
entry for a name and so nothing to pass over to.

Passing over needs a lookup that goes on after an icon it found, and
native-theme's public loaders return the first
(`native-theme/src/freedesktop.rs:327-347`; `FreedesktopLoader::any`,
`native-theme/src/icons.rs:259`). The options: the connector asks for one
name at a time, which needs nothing new but changes the order among a name's
candidates (a parent theme's icon for an earlier name would come before the
theme's own for a later one, the order the loader keeps on purpose and a test
pins, `native-theme/src/freedesktop.rs:1373`), and cannot go on to a parent
theme's icon of the same name; or native-theme's by-name loader gains a form
that takes the caller's test and returns the first icon that passes it, in
its present order. **The second**: one new public method of
`native_theme::icons::FreedesktopLoader`, which holds for every form of the
loader (by a role, by a name, by names: each goes on in its own order to the
next icon it would find), nothing changed for its present callers, and the
order of the theme's icons stays one. Since W3 makes the by-names loader the handle's only lookup in a
freedesktop theme, "the next" is exact: the next icon that loader would come
to, through the theme and the themes it inherits from.

**The Lucide set.** Ten bundled presets name it, and most of gpui-component's
own glyphs are Lucide (§1.3). The options: serve nothing under it, since the
widgets mostly draw it already; or serve native-theme's Lucide like any other
set. **It is served like any other set**, by the table (W3). Serving nothing
would leave the eight glyphs that are not Lucide drawings (the window controls
and the sort arrows among them) beside the application's Lucide icons of the
same meaning, which is mixing. The cost: under those presets the window
controls are Lucide's `x`, `minus`, `maximize` and `minimize-2` where upstream
has glyphs drawn as window controls (under Material likewise `close`,
`minimize`, `open_in_full` and `close_fullscreen`), and maximize and restore
are the sets' full-screen glyphs, since neither set has window glyphs. Better
entries for those two (Lucide has a plain square and a pair of squares) are a
matter for the tables and are recorded; the measurement shows the four
controls before the handle is built. One name is the exception: for `StarFill`, which
native-theme's table lacks, the handle answers "not mine" under the Lucide
set, because upstream's glyph there is a Lucide star with a fill: an icon of
the set, not a substitute, and a `Rating` keeps its filled stars. The report
lists it as kept.

How many of the 106 names Breeze, Adwaita and the other installed themes give
was not measured for this document (§4). The plan measures it before the
handle is built (W3, "The measurement"), lists with it the gaps that are known
without measuring (Material has no `StarOff`), and stops for a decision if a
judged theme lacks an essential glyph, so that it would not be served at all,
or leaves more than half of the 106 empty.

### W5 — The paths, and switching sets

GPUI caches a drawn SVG by its path, so one path must always mean the same
bytes. The served path names the set and, for a freedesktop set, the icon theme:
one path per (set, icon theme, name). Giving another set changes every path; the
handle then calls `App::refresh_windows`, as the hook's documentation asks.

The icons of a set no longer used stay in GPUI's cache, because nothing can drop
them (`docs/todo.md:3812`): one byte per drawn pixel of each icon and size,
bounded by the sets a user switches through in one run. The alternative, one
path per name whatever the set, would show the first set's icon for ever.

If a theme's files change while the application runs, the handle keeps serving
the drawings it loaded until the set changes: it keeps the tables of the icon
themes it has loaded for the current set, so that a switch between light and
dark loads each icon theme once (the specification's cache), and GPUI keeps
what it drew under those paths. After another set has been chosen and the
theme again, the handle reloads the files, but the new drawing reaches only
the sizes a window has not drawn yet and windows opened afterwards: a drawing a
window already holds stays for the run, since GPUI's atlas keeps one tile per
path and size (`GP/window.rs:5247-5258`, `GP/platform.rs:1859-1861`) and the
theme's paths are the same as before. That is the staleness every user of GPUI's SVG
cache has, widened to the handle's own tables, and is not worked around: an
icon theme that changes under a running application is rare, and reloading on
every switch would cost the load the cache saves.

A freedesktop theme's light and dark variants are two icon themes with two
names, and so two sets of paths; W2 says how the handle follows the mode. A
mask keeps only an icon's shape, so the two variants may well draw alike; the
plan's first measurement compares them, and where they are the same the second
table costs a load and nothing else.

### W6 — Colour: only an icon of one colour is served

An SVG is drawn as a mask (§1.1), so the hook can only carry an icon that is a
shape in one colour. Symbolic and monochrome icons are; a full-colour icon would
come out as a silhouette of its outline, which is not the theme's icon.

**Options**, for an icon the set gives:

- (a) Serve every icon; accept silhouettes.
- (b) Serve an icon only where it is drawn in one colour; treat any other as
  "the set has no icon that can be drawn here", so W4 applies.
- (c) Rewrite a multi-colour SVG to one colour.

**(b).** (a) draws something the theme's author did not design, which the rule
against invented artwork excludes; (c) is the same with more work. Whether an
icon is one colour is decided by what it paints, not by its text: an SVG is
rasterised once, small, with native-theme's own rasteriser, and counts as one
colour when every pixel it paints has the same red, green and blue, whatever its
alpha (a symbolic icon with half-transparent parts is one colour, and a mask
draws it faithfully). A textual test would misjudge icons that set colours
through a stylesheet, as Breeze's do. An icon drawn in one colour that is not
grey passes too and is painted in the widget's colour, which is what a mask of
any one-colour icon gives and what gpui-component does with its own. An SVG
that paints nothing under the test, or that embeds an image, which
native-theme's rasteriser is built without and GPUI's draws
(`native-theme/Cargo.toml:74`), counts as not drawable. The
tolerance for antialiased edges is the specification's, fixed by a test over the
bundled Material and Lucide sets, which are one colour throughout, and over a
fixture that is not.

The showcase's chrome draws a set's SVG as a mask without this test today. It
takes the same test: a one-colour SVG is drawn as now, and one in several
colours through `gpui::img`, the chrome's existing path for pixel icons, which
keeps its colours; where a widget takes an `Icon` alone it is drawn as
nothing, as a pixel icon is there (W1). The Icons page already draws through
`gpui::img` and does not change. The showcase applies the test to icons it
loads by a role, which do not come through the connector's lookup, so the
test is a public function of the connector too; an application that draws a
theme's icon as a mask needs the same answer, and a copy in the showcase
would be a second implementation.

### W7 — SF Symbols and Segoe Fluent: not served in this step

Under the two platform sets whose icons are pixels, two things stand between
the set and the widgets.

- **The lookup.** For these sets it knows an icon only by its role, 29 of the
  106 names at most (§1.3), and `Close`, one of W4's essential glyphs, is not
  among them: by W4 the set would not be served at all, and without the floor
  77 or so glyphs would be drawn empty. Either way it is not following the
  set.
- **The carrier.** The hook names an SVG path and these icons are pixels. They
  could be wrapped in an SVG that embeds the image: a mask needs only the
  alpha, and both cores' renderers are built to draw an embedded image (§1.1).
  That is read, not run.

**Options.**

- (a) Serve the 29 and draw the rest empty.
- (b) Serve nothing under these two sets: the widgets keep upstream's glyphs.
- (c) Write the missing tables, from each of the 106 names to an SF Symbol and
  to a Segoe glyph, and embed the pixels.

**(b) in this step; (c) is the step that follows it.** (a) breaks more than it
mends. (c) is right and is not small: two tables of 106 names whose every
entry has to be checked on the platform that has the set, which this
repository can do only on its macOS and Windows test runners, and the
embedding tried on both cores at the sizes widgets draw. Until then the
widgets on macOS and Windows under their native sets draw Lucide beside the
application's native icons, as they do today. That is the first of the two
places where the sets still mix by the connector's own choice (W4's floor is
the other; what the hook cannot reach is W9's), a stated and temporary
exception to the rule, said in the README and
in the showcase's Widget Info, with a backlog entry that carries what was read
for (c): the carrier above, the image formats it takes (§1.1; PNG is the one
that keeps the alpha without loss), that GPUI rasterises an SVG at twice the
device pixels it is drawn at (§1.3), and the sizes the widgets use.

A freedesktop theme can also give pixels for an icon (§1.3). Such an icon
counts as one the set cannot give here, and W4 applies to it: the next
candidate is tried, and failing that the name is drawn empty and reported, or,
for an essential glyph, the set is not served. The two cases are treated
differently on purpose. Under SF
Symbols and Segoe Fluent the gap is known beforehand, is the same for every
user and comes from tables the connector lacks, so the handle declines the
whole set. In a freedesktop theme it depends on the theme, and the report
shows it; the measurement's second stop condition (W3) is for a theme where
it is more than half of the icons. The step that embeds pixels serves those icons too.

### W8 — The connector's own check box

- Where it draws the check mark's path as a raw `svg`
  (`CN/src/widgets/checkbox.rs:430-436`), it passes the path through
  `gpui_component::resolve_icon_path` first, as upstream's check box does
  (`GC/checkbox.rs:220-222`). Without that it would be the one check box whose
  mark ignores a resolver, the connector's or an application's.
- The same holds for the dash of the mixed state, which gpui-component 0.10.0
  draws as `Minus` through the same path and the connector's check box draws
  the same two ways as its check.
- Where the theme states a stroke width, it draws the outline of Lucide's check
  icon at that width (§1.3). That mark stays. On the platforms that state the
  stroke, a check box's mark is drawn by the style and not taken from the
  user's icon theme (`docs/platform-facts.md:1242`), and the stroke width is the
  theme datum the model has for it; its shape is gpui-component's because the
  model states none.
  So under Breeze, Adwaita or Material the connector's check box shows that
  mark while a menu's or a list's check mark shows the icon set's, and so does
  gpui-component's own check box, whose mark is an icon path the resolver
  answers like any other; the resolver cannot tell who asks. The README's
  limits say so, and the backlog gets an entry for the mark's shape.

### W9 — What stays outside the hook, and is said

- Native menus on macOS and Windows and a `gpui_kit_assets::IconName` rendered by
  itself are not resolved by upstream (§1.1).
- A Markdown task box's check mark is drawn from bytes gpui-base embeds
  (gpui-base 0.9.1 `src/text/node.rs:42-43`, `:3913-3917`), so no resolver
  reaches it.
- An `Icon` an application builds from bytes is not resolved; that is how the
  connector's and the showcase's own colour-preserving icons are drawn and is
  wanted.
- An icon's colours: the hook carries a shape (W6).
- The sizes: an SVG is looked up once and GPUI scales it to the size a widget
  asks for; a lookup per drawn size is not possible, since the resolver is
  given the path, not the size. In a freedesktop theme the handle looks up at
  16 px, the size nearest to what the widgets draw most (12 to 16 px, §1.1):
  the loader takes a theme's drawing for exactly that size where it has one,
  else the nearest (the size is passed at
  `native-theme/src/freedesktop.rs:260-268`; exact, then nearest, in
  freedesktop-icons 0.4.0 `src/theme/mod.rs:28-38`). Whether the
  installed themes have a 16 px drawing of their own, and how it compares with
  the 24 px one scaled down, was not read; the measurement shows both sizes
  side by side. 16 px stays where, in every judged theme, the lookup at 16 px
  gives an icon the hook can carry for no fewer of the 106 names than the
  lookup at 24 px, and for every essential glyph the lookup at 24 px gives;
  otherwise the plan stops with both listings (W3, "The measurement"). The
  showcase's
  chrome and its Icons page keep the sizes they look up at today (24 px in a
  freedesktop theme, §1.3), the size nearer to what they draw, so their sizes
  do not change (W3 and W6 say what else in them does); a theme with separate
  drawings then shows a widget's glyph and a toolbar's icon in the drawing for
  each one's size, as the desktop itself does. A theme's 22 px or 32 px
  drawing of the same icon is not used.
- The Icons page therefore shows two things that must not be read as one: its
  own mark for an icon the set lacks, which is for the page's lookup (its
  size, any icon it can draw), and the handle's report, which is for the
  widgets (16 px, an icon the hook can carry). The page labels each.

### W10 — The showcase

- Its asset source becomes the handle's, around `gpui_kit::assets::Assets`.
- Wherever it reloads its own icons, which every path does (a theme installed
  by either route, the Select, the command line, start-up, the watcher;
  `SC/app.rs:1108`), and whatever its Select's row, the showcase tells the
  handle the theme's icons: the set and icon themes its own lookup would draw
  under the row that follows the theme, which is `default_icon_choice`'s
  answer (§1.3), so the widgets never follow a set the chrome does not.
  Where that answer leaves the icon theme to the system's and the system's
  cannot be detected, there is nothing to tell: under the row that follows the
  theme, and under the row for the system's set, whose icons then load nothing
  either, the handle is switched off; under any other row it is told nothing
  and keeps the row's set, so the widgets never draw another set beside the
  chrome's.
- What it draws itself by an `IconName` that has a role (`chrome_icon`,
  `sample_icon`) takes the role's icon and, where the role gives none, the
  name's from the list of names; the list of names alone serves the Icons
  page's gallery of `IconName`s and the names without a role; the Buttons
  page's two loading buttons leave the spinner's default icon in place where
  the handle serves the set and its `Loader` (W3, "The showcase's two lists").
- Its Select's rows map to the handle's states (W2): the row that follows the
  theme puts it into following; a named set fixes that set; the row for the
  system's set fixes the system's set with the icon theme detected then, and
  fixes it again whenever the showcase reloads the system's icons; the row
  "gpui-component built-in (Lucide)" turns it off. A fixed set and "off" then
  survive every theme the showcase installs afterwards, as the Select's own
  choice does, and choosing the following row again serves the theme's icons
  as last told.
- The Widget Info's list of "upstream's icons" (`SC/info/chrome.rs:993-999`),
  its reason (`SC/info/mod.rs:444-448`), the entry beside it on the pixel sets
  and the Icons page's line on how an icon is looked up
  (`SC/info/icons.rs:181-184`) are rewritten to what is then true: under an SVG
  set that has the essential glyphs the widgets' icons are the set's and an
  icon the set lacks is not drawn; under a set that lacks one of them (W4),
  and under SF Symbols and Segoe Fluent (W7), they are upstream's; in the
  gallery of `IconName`s an icon is looked up by its name in the connector's
  table under Lucide, Material and a freedesktop theme, and by its role
  under the two pixel sets; in the gallery of roles, by the role in every
  set.
- `docs/showcase-exceptions.toml` lists `SidebarToggleButton` and `SpeechButton`
  as not shown because they "would mix icon themes" (`:52-53`). They stay
  exceptions: under the two pixel sets they still would (W7). Their reason is
  reworded to say so, and they are shown with the step that serves those sets.

### W11 — Where it lands

**Options.**

- (a) In v0.7.1, with the port's completion.
- (b) As a 0.7.x release of its own after v0.7.1.

**(b), proposed.** It is a new public API with a measurement still to make
(W4), and v0.7.1 is otherwise fixes and the dependency upgrade. 0.7.x
grows one small step at a time (`ROADMAP.md`). Two parts go into v0.7.1 whatever
is chosen: W8's first point, because without it the connector's check box
ignores an application's own resolver, which is a gap in the 0.9.1 port; and the
corrected statements about the hook (the port's document, N3 and N5). If the
maintainer prefers (a), nothing in the design changes; v0.7.1's asset run then
follows this work too.

**Decided otherwise, 2026-11-19.** The maintainer folded the work into 0.7.2,
after the dependency upgrades and the KDE steps 2 to 5 on the same branch; as
said of (a), nothing in the design changes, and 0.7.2's asset run follows this
work too.

## 3. What the maintainer approves

1. W1, W2: a public handle in `native_theme_gpui::icons` that wraps the
   application's asset source, installs the resolver once, has a public lookup,
   three states (following the theme, a fixed set, off) and a list of
   essential glyphs an application can replace (W4); beside it two public
   functions, the lookup by the table that the handle and the showcase share
   (it takes the size, whether only an icon the hook can carry counts, and
   the colour, W1) and the test whether an icon can be drawn as a mask (W6);
   `apply_system_theme` tells it the theme's icons and it follows the mode;
   on the `apply` path the application tells it.
2. W3, W5: the handle answers for the 106 paths of gpui-component's `IconName`
   whoever draws them, an application's own artwork under those paths
   included; under Lucide, Material and a freedesktop theme the icon comes
   from the connector's table by name, no longer by the role first; each
   set has paths of its own, and GPUI's cache keeps the icons of every set
   used in a run. With it:
   - two entries of the freedesktop table change (`Loader` and
     `LoaderCircle` to `process-working-symbolic`, `content-loading`), and
     with them the deprecated `freedesktop_name_for_gpui_icon`'s answer for
     those two names outside GTK desktops; `Check`'s entry stays, and its
     order is decided on the measurement;
   - under Breeze gpui-component's spinner turns the icon KDE turns;
   - under Adwaita, which has no spinner icon, it turns the theme's
     three-dot loading indicator, and `CircleCheck` there is the plain check
     mark: the nearest the theme has. The two other ways for the spinner are
     declined: no spinner at all, or upstream's glyph as a third exception
     to the no-mixing rule. This is the maintainer's to accept or refuse;
   - what the showcase draws itself under a name that has a role keeps
     today's order (the role's icon first) and so its icons; its gallery of
     `IconName`s is looked up by the connector's table, as the handle's
     icons are (at the page's size, and any icon the page can draw); the
     Buttons page's two loading buttons no longer override the spinner's icon
     where the handle serves the set's `Loader`;
   - until the measurement decides `Check`'s entry, the widgets' check mark
     is `dialog-ok` under Breeze and `object-select-symbolic` under Adwaita;
   - native-theme's `FreedesktopLoader` gains one public method, for passing
     over an icon the hook cannot carry (W4);
   - the plan's first stop is the measurement, with the seven conditions W3
     lists.
3. W4: an icon the set lacks is drawn empty, with a report of the missing names;
   upstream's glyph is never kept in its place, but for the filled star under
   Lucide; a set that gives nothing at all is not served; a set that lacks one
   of nine essential glyphs (the window controls, `Close`, `ChevronDown`,
   `Check`, `Minus`, `Plus`) is not served either, and the widgets keep
   upstream's glyphs: the second stated exception to the no-mixing rule; an
   application can give its own list in place of the nine, an empty one
   included; other buttons whose only content is a glyph the set lacks are
   drawn empty; the Lucide set is served, window controls included, and under
   Lucide and Material a window's maximize and restore become those sets'
   full-screen glyphs.
4. W6: only an icon drawn in one colour is served.
5. W7: under SF Symbols and Segoe Fluent the widgets keep upstream's glyphs in
   this step, a stated exception to the no-mixing rule, until the tables for
   those sets are written.
6. W8: the stroked check mark stays the check box's own.
7. W9: the handle looks a freedesktop icon up at 16 px, confirmed on the
   measurement; the showcase's own icons keep the sizes they are looked up at
   today (24 px in a freedesktop theme), so a widget's glyph and the
   showcase's icon of the same name can be two drawings of the theme: another
   size's, another candidate's where the first is not one the hook can
   carry, or the role's where the showcase draws by the role and role and
   table differ (W1); in the showcase's slots that take an `Icon` alone, an icon in
   several colours is drawn as nothing.
8. W10: the showcase's changes.
9. W11: the release it lands in: 0.7.2, by the maintainer's decision of
   2026-11-19.

## 4. What was checked, and what was not

Checked on 2026-11-14 by reading the sources named in §1; the first review
re-read them.

Not checked:

- How many of the 106 names each freedesktop theme gives, how many of those are
  one colour, and how many are pixels. Reading the installed themes needs the
  maintainer's permission (they are outside the project folder); the plan
  measures it through the library itself (W3, "The measurement"). The same measurement
  answers the further questions the decisions leave to it: what the installed
  themes draw for `Check`, `CircleCheck`, `Loader` and `LoaderCircle` under
  each of their names (W3); which of the showcase's `Icon`-only slots would
  lose an icon in several colours (W1); whether a theme's light and dark
  variants draw alike (W5); and how a theme's 16 px drawing compares with its
  24 px one (W9).
- Recorded, and confirmed by the measurement's listing of each name of the
  four entries: that Adwaita has neither `emblem-ok-symbolic` nor
  `process-working-symbolic` (§1.3; the 0.7.0 review checked the installed
  theme with and without `-symbolic`), and that Breeze ships
  `process-working-symbolic` (inferred from KDE's busy indicator loading it;
  no record names the file in Breeze; the connector's test that loads the
  table's Breeze names from the installed theme checks it, W3).
- Whether `dialog-ok` and `emblem-default` draw a plain check mark in the
  installed themes.
- That an embedded image is drawn through the mask (W7): read, not run; it is
  not needed in this step.
- Whether the SF Symbols and Segoe loaders take an icon by any name of the
  set, which the tables of W7's option (c) would rely on.
- Whether gpui-base draws any other icon in a way that bypasses the resolver,
  beyond the Markdown task box's check mark (W9).
- Whether each widget of §1.2 has a setter for its icon; the hook does not need
  one.
- How an empty icon is best served: an empty SVG, or an asset source that
  answers that it has none, which also draws nothing
  (`GP/svg_renderer.rs:255-261`). Neither was run; the specification chooses.
- The cost of loading 106 icons from a freedesktop theme and testing each for
  one colour. The measurement records it for each judged theme beside the
  time the showcase's own reload of its icons takes today for the same
  theme, which every theme switch in the showcase already waits for. Where
  the handle's load takes longer than that, the listing goes to the
  maintainer with both figures, and the load moves to a background task if
  the maintainer finds it noticeable, which the handle's shape allows; where it does
  not, it stays where it is.

## 5. Out of scope

- Icons with their colours inside widgets (an alert's coloured status icon as
  the theme draws it): the hook cannot carry them.
- Serving SF Symbols and Segoe Fluent (W7), and better table entries for a
  window's maximize and restore under Lucide and Material (W4): recorded in
  the backlog by this work.
- A hook for native menus, and dropping drawn SVGs from GPUI's cache: upstream's.
- The iced and egui connectors: their toolkits draw no icons of their own choice
  in the same way. Their showcases draw their icons by role, three by name
  from entries this work does not change, and do not change; the one difference this leaves beside the gpui showcase, a caret
  drawn by the nearest role, is recorded (W3).
