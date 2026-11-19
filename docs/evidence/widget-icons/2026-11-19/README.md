# The widget icons' measurement, 2026-11-19

What the installed freedesktop icon themes and the bundled Lucide and Material sets give
gpui-component's 106 `IconName`s through the gpui connector's lookup, before the connector's
widget icons were built ([specification](../../../todo_v0.7.2_gpui-widget-icons-spec.md),
Part C). The table is [`listing.md`](listing.md).

- **Produced by**: the gpui showcase's ignored test `widget_icons_listing`, at commit
  `c89dfe855537c234f740ea8ad09821818957bc88` (the tracked files clean), run by hand:
  `NT_WIDGET_ICONS_LISTING=docs/evidence/widget-icons/2026-11-19 cargo test -p native-theme-gpui --features native,watch --example showcase-gpui -- --ignored widget_icons_listing --nocapture`.
  The test was removed once this run was recorded; that commit holds it.
- **Date**: 2026-11-19.
- **The machine**, as the run reports it through the library and `std`: OS `linux`; desktop
  `Kde` (`native_theme::detect::detect_linux_desktop()`); system icon theme `breeze-dark`.
- **Judged themes**: `breeze`, `breeze-dark`, `Adwaita`. Listed, not judged: `AdwaitaLegacy`,
  `char-white`.

Read on 2026-11-19 from the package manager (`pacman -Q`, by package name, at the
maintainer's word); the session does not read the icon base directories, so the owner of
each theme's directory was not queried.

| Theme | Package | Version |
|---|---|---|
| `breeze` | `breeze-icons` | 6.31.0-1.1 |
| `breeze-dark` | `breeze-icons` | 6.31.0-1.1 |
| `Adwaita` | `adwaita-icon-theme` | 51.0-1 |
| `AdwaitaLegacy` (listed, not judged) | | |
| `char-white` (listed, not judged) | | |

No drawing of an installed icon theme is kept here: each is that theme's artwork under its
own licence. `listing.md` names each icon, its size, whether it is one colour and a hash of
its bytes; the sheets the run drew were read in the repository's `target/` and are not kept.

## What the run reported

The specification's seven stop conditions (C.5), as `listing.md`'s section 9 gives them:

- 1, 2, 4 and 5: not met. Every judged theme and both bundled sets give the nine essentials at
  16 px; empty at 16 px: `breeze` 8, `breeze-dark` 8, `Adwaita` 0, Lucide 1, Material 1.
- 3 (read from the four-name sheets): not met. Breeze's `Check` candidates are one icon, a
  check mark; its `Loader` is `process-working-symbolic`, Breeze's busy glyph, before the
  three dots of `content-loading`; Adwaita's `CircleCheck` (a plain check) and `Loader` (the
  dots) are the nearest names, as the rationale expected; one order serves both themes.
- 6: met, by icons in several colours: `CaseSensitive` (`format-text-uppercase`) and
  `CircleX` (`dialog-error`) in `breeze` and `breeze-dark`. The showcase's slots that take an
  `Icon` alone draw them as nothing.
- 7: met at the first load of a theme: kde-breeze Light, the handle's median 25.3 ms against
  the showcase's 15.7 ms; adwaita Light, 18.4 ms against 17.7 ms. The dark steps: 5.5 and
  4.5 ms against 15.8 and 17.7 ms.

A first run the same day, with `ONE_COLOUR_TOLERANCE` at 492 (the bundled sets' largest
deviation alone), judged Breeze's one-colour `weather-clear-night-symbolic` (570) as several
colours; its 1,802 rows showed no deviation between 570 and 25500, and the tolerance was
set between them (the specification's B.3) before this run. That run's table was not kept.
