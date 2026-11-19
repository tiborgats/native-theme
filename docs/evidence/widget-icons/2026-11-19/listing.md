# The widget icons' measurement

Written by the gpui showcase's ignored test `widget_icons_listing` (the 0.7.2 widget icons specification, Part C).

- **Commit**: `c89dfe855537c234f740ea8ad09821818957bc88`, the working tree's tracked files clean.
- **Build**: the profile `cargo test` builds the showcase's tests in (debug assertions on).
- **Machine**, as the library and `std` report it: OS `linux`; desktop `Kde` (`detect_linux_desktop`); system icon theme `breeze-dark` (`system_icon_theme`).
- **Judged**: `breeze` (kde-breeze Light; colour #232629), `breeze-dark` (kde-breeze Dark; colour #fcfcfc), `Adwaita` (adwaita Light; colour #2e3436). Not judged, not installed: none.
- **Listed, not judged** (16 px `Mask` rows only): `AdwaitaLegacy`, `char-white`.
- **The bundled sets**: Lucide and Material, the repository's own files.
- **The lookups** (`load_gpui_icon`): `Mask, 16` and `Mask, 24` with no colour, what the handle would serve; `Any, default` at the loader's default size, with the showcase's colour for the theme (the font colour of the first bundled preset's variant naming it, the colour after its name above), what the gallery draws.
- **The hash**: FNV-1a, 64 bits (offset basis `cbf29ce484222325`, prime `100000001b3`), over an SVG's bytes as loaded or an RGBA icon's pixel buffer, in hex.
- **The name found**: the first of the entry's candidates `n` whose own lookup, `FreedesktopLoader::any(&[n])` in the theme at the same size, colour and test, gives the same bytes; a candidate that ends in `-symbolic` is found in that form; for another, the same lookup of `<n>-symbolic` giving the same bytes says the symbolic form, else the plain one. Where two candidates' files have the same bytes the earlier one is named. The theme given is the theme searched, not the theme of its chain that held the file: *same hash in* names the other installed themes whose row for the name and lookup has the same hash.
- **One colour**: `mask_drawable`'s answer; for an SVG, its deviation (a copy of the connector's `one_colour_deviation`, from `rasterize_svg` at 16 px) beside `ONE_COLOUR_TOLERANCE` (3812). The run fails where the copies disagree with `mask_drawable`.
- **Sheets**: `target/widget-icons-sheets/2026-11-19/`, cells of 64 px, 10 to a row in the order of the gallery's `GPUI_ICONS` (the order of item 1); not committed.

## 1. The 106 names

### `breeze`

| IconName | Lookup | Name found | Theme searched | Size | One colour | Hash | Same hash in |
|---|---|---|---|---|---|---|---|
| ALargeSmall | Mask, 16 | `format-font-size-more` as `format-font-size-more-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `923128e4bc8ba8dd` |  |
| ALargeSmall | Mask, 24 | `format-font-size-more` as `format-font-size-more-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `e19e6c42696770aa` |  |
| ALargeSmall | Any, default | `format-font-size-more` as `format-font-size-more-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `e19e6c42696770aa` |  |
| ArrowDown | Mask, 16 | `go-down-skip` as `go-down-skip-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `75c53f657763633a` |  |
| ArrowDown | Mask, 24 | `go-down-skip` as `go-down-skip-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `e03f9fa251e15b4d` |  |
| ArrowDown | Any, default | `go-down-skip` as `go-down-skip-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `e03f9fa251e15b4d` |  |
| ArrowLeft | Mask, 16 | `go-previous-skip` as `go-previous-skip-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `a1518e5a43dcca80` |  |
| ArrowLeft | Mask, 24 | `go-previous-skip` as `go-previous-skip-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `607501d1042a89df` |  |
| ArrowLeft | Any, default | `go-previous-skip` as `go-previous-skip-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `607501d1042a89df` |  |
| ArrowRight | Mask, 16 | `go-next-skip` as `go-next-skip-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `ab62b1109accdc24` |  |
| ArrowRight | Mask, 24 | `go-next-skip` as `go-next-skip-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `1369311d6c2c224d` |  |
| ArrowRight | Any, default | `go-next-skip` as `go-next-skip-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `1369311d6c2c224d` |  |
| ArrowUp | Mask, 16 | `go-up-skip` as `go-up-skip-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `deb929073b7ee1fc` |  |
| ArrowUp | Mask, 24 | `go-up-skip` as `go-up-skip-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `3e6e943ace80a04f` |  |
| ArrowUp | Any, default | `go-up-skip` as `go-up-skip-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `3e6e943ace80a04f` |  |
| Asterisk | Mask, 16 | `rating` (plain) | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `5a3b736f175ac491` |  |
| Asterisk | Mask, 24 | `rating` (plain) | breeze | 24; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `9eaf8868337b9875` |  |
| Asterisk | Any, default | `rating` (plain) | breeze | default; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `9eaf8868337b9875` |  |
| Ban | Mask, 16 | `action-unavailable` as `action-unavailable-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `8cdc76f415046d88` |  |
| Ban | Mask, 24 | `action-unavailable` as `action-unavailable-symbolic` | breeze | 24; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `8cdc76f415046d88` |  |
| Ban | Any, default | `action-unavailable` as `action-unavailable-symbolic` | breeze | default; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `8cdc76f415046d88` |  |
| Battery | Mask, 16 | none | breeze | 16 | | | |
| Battery | Mask, 24 | none | breeze | 24 | | | |
| Battery | Any, default | `battery` (plain) | breeze | default; no viewBox | no: several colours (deviation 49215 / 3812) | `1ac1eca9c82e7da9` | `breeze-dark` |
| BatteryCharging | Mask, 16 | none | breeze | 16 | | | |
| BatteryCharging | Mask, 24 | none | breeze | 24 | | | |
| BatteryCharging | Any, default | `battery-100-charging` as `battery-100-charging-symbolic` | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 34680 / 3812) | `76126edaef852994` |  |
| BatteryFull | Mask, 16 | `battery-100` as `battery-100-symbolic` | breeze | 16; no viewBox | yes (deviation 0 / 3812) | `3e34fce44b653cbc` |  |
| BatteryFull | Mask, 24 | `battery-100` as `battery-100-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `6dbe46b2c5defc28` |  |
| BatteryFull | Any, default | `battery-100` as `battery-100-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `6dbe46b2c5defc28` |  |
| BatteryLow | Mask, 16 | `battery-020` as `battery-020-symbolic` | breeze | 16; no viewBox | yes (deviation 128 / 3812) | `0baae81a71835745` |  |
| BatteryLow | Mask, 24 | `battery-020` as `battery-020-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `c84bb845dfe061a2` |  |
| BatteryLow | Any, default | `battery-020` as `battery-020-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `c84bb845dfe061a2` |  |
| BatteryMedium | Mask, 16 | `battery-050` as `battery-050-symbolic` | breeze | 16; no viewBox | yes (deviation 128 / 3812) | `36db1e16af058774` |  |
| BatteryMedium | Mask, 24 | `battery-050` as `battery-050-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `063705277dc463ed` |  |
| BatteryMedium | Any, default | `battery-050` as `battery-050-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `063705277dc463ed` |  |
| BatteryWarning | Mask, 16 | `battery-010` as `battery-010-symbolic` | breeze | 16; no viewBox | yes (deviation 240 / 3812) | `52177a1e5e7f1629` |  |
| BatteryWarning | Mask, 24 | `battery-010` as `battery-010-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `0e9b709f959151e3` |  |
| BatteryWarning | Any, default | `battery-010` as `battery-010-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `0e9b709f959151e3` |  |
| Bell | Mask, 16 | `notification-active` as `notification-active-symbolic` | breeze | 16; no viewBox | yes (deviation 244 / 3812) | `ee5d7c84a6529a4f` |  |
| Bell | Mask, 24 | `notification-active` as `notification-active-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 222 / 3812) | `884ff976d7ae02c0` |  |
| Bell | Any, default | `notification-active` as `notification-active-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 222 / 3812) | `884ff976d7ae02c0` |  |
| BookOpen | Mask, 16 | `help-contents` as `help-contents-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `f6246c957b35ad80` |  |
| BookOpen | Mask, 24 | `help-contents` as `help-contents-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `07cb1a0aec86eb83` |  |
| BookOpen | Any, default | `help-contents` as `help-contents-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `07cb1a0aec86eb83` |  |
| Bot | Mask, 16 | none | breeze | 16 | | | |
| Bot | Mask, 24 | none | breeze | 24 | | | |
| Bot | Any, default | `face-smile` (plain) | breeze | default; viewBox `0 0 5.8208332 5.8208335` | no: several colours (deviation 33150 / 3812) | `131684302091942f` | `breeze-dark` |
| Building2 | Mask, 16 | `applications-office` as `applications-office-symbolic` | breeze | 16; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `d3f7482bd819ed38` |  |
| Building2 | Mask, 24 | `applications-office` as `applications-office-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `d3f7482bd819ed38` |  |
| Building2 | Any, default | `applications-office` as `applications-office-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `d3f7482bd819ed38` |  |
| Calendar | Mask, 16 | `view-calendar` as `view-calendar-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `28dc851305cf41c0` |  |
| Calendar | Mask, 24 | `view-calendar` as `view-calendar-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `0f88114e95443886` |  |
| Calendar | Any, default | `view-calendar` as `view-calendar-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `0f88114e95443886` |  |
| CaseSensitive | Mask, 16 | none | breeze | 16 | | | |
| CaseSensitive | Mask, 24 | none | breeze | 24 | | | |
| CaseSensitive | Any, default | `format-text-uppercase` as `format-text-uppercase-symbolic` | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 43232 / 3812) | `c40671b6ef2fbda1` |  |
| ChartPie | Mask, 16 | `office-chart-pie` as `office-chart-pie-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `904b8d2fd8bb9fb2` |  |
| ChartPie | Mask, 24 | `office-chart-pie` as `office-chart-pie-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `dec0a360557f800d` |  |
| ChartPie | Any, default | `office-chart-pie` as `office-chart-pie-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `dec0a360557f800d` |  |
| Check | Mask, 16 | `dialog-ok` as `dialog-ok-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` |  |
| Check | Mask, 24 | `dialog-ok` as `dialog-ok-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `a8d13ce782fb131f` |  |
| Check | Any, default | `dialog-ok` as `dialog-ok-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `a8d13ce782fb131f` |  |
| ChevronDown | Mask, 16 | `go-down` as `go-down-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `ed5ca0625947021f` |  |
| ChevronDown | Mask, 24 | `go-down` as `go-down-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `3280a69a3d9440bd` |  |
| ChevronDown | Any, default | `go-down` as `go-down-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `3280a69a3d9440bd` |  |
| ChevronLeft | Mask, 16 | `go-previous` as `go-previous-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `59fc678f1a03c743` |  |
| ChevronLeft | Mask, 24 | `go-previous` as `go-previous-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `869b36719674e2c0` |  |
| ChevronLeft | Any, default | `go-previous` as `go-previous-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `869b36719674e2c0` |  |
| ChevronRight | Mask, 16 | `go-next` as `go-next-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `1eeb8df06e3c9fae` |  |
| ChevronRight | Mask, 24 | `go-next` as `go-next-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `7926862058791def` |  |
| ChevronRight | Any, default | `go-next` as `go-next-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `7926862058791def` |  |
| ChevronsUpDown | Mask, 16 | `handle-sort` as `handle-sort-symbolic` | breeze | 16; viewBox `0 0 22 22` | yes (deviation 160 / 3812) | `ef154ac18ca46115` |  |
| ChevronsUpDown | Mask, 24 | `handle-sort` as `handle-sort-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `3bc66a9e3a47d360` |  |
| ChevronsUpDown | Any, default | `handle-sort` as `handle-sort-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `3bc66a9e3a47d360` |  |
| ChevronUp | Mask, 16 | `go-up` as `go-up-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `dd9afec33cf1712e` |  |
| ChevronUp | Mask, 24 | `go-up` as `go-up-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `6f279add3ccc9b89` |  |
| ChevronUp | Any, default | `go-up` as `go-up-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `6f279add3ccc9b89` |  |
| CircleAlert | Mask, 16 | `emblem-important` as `emblem-important-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `61bd666bc22647c6` |  |
| CircleAlert | Mask, 24 | `emblem-important` as `emblem-important-symbolic` | breeze | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `61bd666bc22647c6` |  |
| CircleAlert | Any, default | `emblem-important` as `emblem-important-symbolic` | breeze | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `61bd666bc22647c6` |  |
| CircleCheck | Mask, 16 | `emblem-ok-symbolic` (symbolic, as named) | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` |  |
| CircleCheck | Mask, 24 | `emblem-ok-symbolic` (symbolic, as named) | breeze | 24; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` |  |
| CircleCheck | Any, default | `emblem-ok-symbolic` (symbolic, as named) | breeze | default; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` |  |
| CircleUser | Mask, 16 | `user-identity` as `user-identity-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `914b9c2ca206b71b` |  |
| CircleUser | Mask, 24 | `user-identity` as `user-identity-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `f644433a0b31d303` |  |
| CircleUser | Any, default | `user-identity` as `user-identity-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `f644433a0b31d303` |  |
| CircleX | Mask, 16 | none | breeze | 16 | | | |
| CircleX | Mask, 24 | none | breeze | 24 | | | |
| CircleX | Any, default | `dialog-error` (plain) | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 47685 / 3812) | `24927e71c687bd88` |  |
| Close | Mask, 16 | `window-close` as `window-close-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `99cf5eaec3ba2564` |  |
| Close | Mask, 24 | `window-close` as `window-close-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `94c304d84e249c22` |  |
| Close | Any, default | `tab-close` as `tab-close-symbolic` | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 46665 / 3812) | `b9a243f9393e8834` |  |
| Copy | Mask, 16 | `edit-copy` as `edit-copy-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5aa7d4b71ec43245` |  |
| Copy | Mask, 24 | `edit-copy` as `edit-copy-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `104e9cc4f434dedf` |  |
| Copy | Any, default | `edit-copy` as `edit-copy-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `104e9cc4f434dedf` |  |
| Cpu | Mask, 16 | `computer` as `computer-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `a9ede46ca4499e33` |  |
| Cpu | Mask, 24 | `computer` as `computer-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 0 / 3812) | `b1da942e2e5bb8ae` |  |
| Cpu | Any, default | `cpu` (plain) | breeze | default; no viewBox | no: several colours (deviation 25500 / 3812) | `abb72e5196fe4253` | `breeze-dark` |
| Dash | Mask, 16 | `list-remove` as `list-remove-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0a9be8349d2c0b4f` |  |
| Dash | Mask, 24 | `list-remove` as `list-remove-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `207220628bbac616` |  |
| Dash | Any, default | `list-remove` as `list-remove-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `207220628bbac616` |  |
| Delete | Mask, 16 | `edit-delete` as `edit-delete-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5119f2098b513357` |  |
| Delete | Mask, 24 | `edit-delete` as `edit-delete-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `ed98fe8d2f0c27ad` |  |
| Delete | Any, default | `edit-delete` as `edit-delete-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `ed98fe8d2f0c27ad` |  |
| Ellipsis | Mask, 16 | `overflow-menu` as `overflow-menu-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f2113864328a80fa` |  |
| Ellipsis | Mask, 24 | `overflow-menu` as `overflow-menu-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `f3e0b1191e8fd7e4` |  |
| Ellipsis | Any, default | `overflow-menu` as `overflow-menu-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `f3e0b1191e8fd7e4` |  |
| EllipsisVertical | Mask, 16 | `overflow-menu` as `overflow-menu-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f2113864328a80fa` |  |
| EllipsisVertical | Mask, 24 | `overflow-menu` as `overflow-menu-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `f3e0b1191e8fd7e4` |  |
| EllipsisVertical | Any, default | `overflow-menu` as `overflow-menu-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `f3e0b1191e8fd7e4` |  |
| ExternalLink | Mask, 16 | `external-link` as `external-link-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `c8f71f17ffb78bc0` |  |
| ExternalLink | Mask, 24 | `external-link` as `external-link-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b2a34bb9d07e9698` |  |
| ExternalLink | Any, default | `external-link` as `external-link-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b2a34bb9d07e9698` |  |
| Eye | Mask, 16 | `view-visible` as `view-visible-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `4accbe13da9b26d9` |  |
| Eye | Mask, 24 | `view-visible` as `view-visible-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `1dc8484e189a2ae1` |  |
| Eye | Any, default | `view-visible` as `view-visible-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `1dc8484e189a2ae1` |  |
| EyeOff | Mask, 16 | `view-hidden` as `view-hidden-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `95d80f5dd7d324a8` |  |
| EyeOff | Mask, 24 | `view-hidden` as `view-hidden-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `0dfc04271e1d28d6` |  |
| EyeOff | Any, default | `view-hidden` as `view-hidden-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `0dfc04271e1d28d6` |  |
| File | Mask, 16 | `text-x-generic` (plain) | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f6bf16d1d553b0a` | `breeze-dark` |
| File | Mask, 24 | `text-x-generic` (plain) | breeze | 24; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze-dark` |
| File | Any, default | `text-x-generic` (plain) | breeze | default; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze-dark` |
| FileText | Mask, 16 | `text-x-generic` (plain) | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f6bf16d1d553b0a` | `breeze-dark` |
| FileText | Mask, 24 | `text-x-generic` (plain) | breeze | 24; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze-dark` |
| FileText | Any, default | `text-x-generic` (plain) | breeze | default; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze-dark` |
| Folder | Mask, 16 | `folder` as `folder-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `f6b1ad730ce91a0c` |  |
| Folder | Mask, 24 | `folder` as `folder-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `0cebecdb3adef293` |  |
| Folder | Any, default | `folder` as `folder-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `0cebecdb3adef293` |  |
| FolderClosed | Mask, 16 | `folder` as `folder-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `f6b1ad730ce91a0c` |  |
| FolderClosed | Mask, 24 | `folder` as `folder-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `0cebecdb3adef293` |  |
| FolderClosed | Any, default | `folder` as `folder-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `0cebecdb3adef293` |  |
| FolderOpen | Mask, 16 | `folder-open` as `folder-open-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2b4b8abd20b075d7` |  |
| FolderOpen | Mask, 24 | `folder-open` as `folder-open-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `a8c731d385a23845` |  |
| FolderOpen | Any, default | `folder-open` as `folder-open-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `a8c731d385a23845` |  |
| Frame | Mask, 16 | `select-rectangular` as `select-rectangular-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 325 / 3812) | `a00bcfc2c955810c` |  |
| Frame | Mask, 24 | `select-rectangular` as `select-rectangular-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `bad11c11616107a7` |  |
| Frame | Any, default | `select-rectangular` as `select-rectangular-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `bad11c11616107a7` |  |
| GalleryVerticalEnd | Mask, 16 | `view-list-icons` as `view-list-icons-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `d662bdf56edc92fd` |  |
| GalleryVerticalEnd | Mask, 24 | `view-list-icons` as `view-list-icons-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `af165021e10806db` |  |
| GalleryVerticalEnd | Any, default | `view-list-icons` as `view-list-icons-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `af165021e10806db` |  |
| Github | Mask, 16 | `applications-engineering` as `applications-engineering-symbolic` | breeze | 16; viewBox `0 0 22 22` | yes (deviation 160 / 3812) | `f4f31c07d19a5bcd` |  |
| Github | Mask, 24 | `applications-engineering` as `applications-engineering-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 160 / 3812) | `f4f31c07d19a5bcd` |  |
| Github | Any, default | `vcs-branch` as `vcs-branch-symbolic` | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 36312 / 3812) | `5cb430219c191c2e` |  |
| Globe | Mask, 16 | `globe` as `globe-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `4806ea6517aed73c` |  |
| Globe | Mask, 24 | `globe` as `globe-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `54bb04083682efcc` |  |
| Globe | Any, default | `globe` as `globe-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `54bb04083682efcc` |  |
| HardDrive | Mask, 16 | `drive-harddisk` as `drive-harddisk-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `d37ebe49449c838d` |  |
| HardDrive | Mask, 24 | `drive-harddisk` as `drive-harddisk-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `1c271c405770df45` |  |
| HardDrive | Any, default | `drive-harddisk` as `drive-harddisk-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `1c271c405770df45` |  |
| Heart | Mask, 16 | `emblem-favorite` as `emblem-favorite-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 160 / 3812) | `19cd0488306aaded` |  |
| Heart | Mask, 24 | `emblem-favorite` as `emblem-favorite-symbolic` | breeze | 24; viewBox `0 0 16 16` | yes (deviation 160 / 3812) | `19cd0488306aaded` |  |
| Heart | Any, default | `emblem-favorite` as `emblem-favorite-symbolic` | breeze | default; viewBox `0 0 16 16` | yes (deviation 160 / 3812) | `19cd0488306aaded` |  |
| HeartOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `ee5d5a02d49d98b3` |  |
| HeartOff | Mask, 24 | `non-starred` as `non-starred-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b89b9ad88781d92e` |  |
| HeartOff | Any, default | `non-starred` as `non-starred-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b89b9ad88781d92e` |  |
| Inbox | Mask, 16 | `mail-folder-inbox` as `mail-folder-inbox-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `60a509048b0ab957` |  |
| Inbox | Mask, 24 | `mail-folder-inbox` as `mail-folder-inbox-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `374afc69ce7b513b` |  |
| Inbox | Any, default | `mail-folder-inbox` as `mail-folder-inbox-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `374afc69ce7b513b` |  |
| Info | Mask, 16 | none | breeze | 16 | | | |
| Info | Mask, 24 | none | breeze | 24 | | | |
| Info | Any, default | `dialog-information` (plain) | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 37230 / 3812) | `08978aca80c87b18` |  |
| Inspector | Mask, 16 | `code-context` as `code-context-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `a4f0e2e5c32b8ed6` |  |
| Inspector | Mask, 24 | `code-context` as `code-context-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `34c0136b17f7c99f` |  |
| Inspector | Any, default | `code-context` as `code-context-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `34c0136b17f7c99f` |  |
| LayoutDashboard | Mask, 16 | `view-grid` as `view-grid-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `61435d0ac1b031d0` |  |
| LayoutDashboard | Mask, 24 | `view-grid` as `view-grid-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `a7c1011c765d5399` |  |
| LayoutDashboard | Any, default | `view-grid` as `view-grid-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `a7c1011c765d5399` |  |
| Loader | Mask, 16 | `process-working-symbolic` (symbolic, as named) | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `182da088be4153db` |  |
| Loader | Mask, 24 | `process-working-symbolic` (symbolic, as named) | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `81a2f95a6f1978a8` |  |
| Loader | Any, default | `process-working-symbolic` (symbolic, as named) | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `81a2f95a6f1978a8` |  |
| LoaderCircle | Mask, 16 | `process-working-symbolic` (symbolic, as named) | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `182da088be4153db` |  |
| LoaderCircle | Mask, 24 | `process-working-symbolic` (symbolic, as named) | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `81a2f95a6f1978a8` |  |
| LoaderCircle | Any, default | `process-working-symbolic` (symbolic, as named) | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `81a2f95a6f1978a8` |  |
| Map | Mask, 16 | `find-location` as `find-location-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `7d4a6e33cfcf2afd` |  |
| Map | Mask, 24 | `find-location` as `find-location-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 279 / 3812) | `f00285dd1c9347fe` |  |
| Map | Any, default | `find-location` as `find-location-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 279 / 3812) | `f00285dd1c9347fe` |  |
| Maximize | Mask, 16 | `view-fullscreen` as `view-fullscreen-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `a280239f514179ff` |  |
| Maximize | Mask, 24 | `view-fullscreen` as `view-fullscreen-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `41ba7794ae508793` |  |
| Maximize | Any, default | `view-fullscreen` as `view-fullscreen-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `41ba7794ae508793` |  |
| MemoryStick | Mask, 16 | `media-flash` as `media-flash-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `77cf71460b0b4526` |  |
| MemoryStick | Mask, 24 | `media-flash` as `media-flash-symbolic` | breeze | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `77cf71460b0b4526` |  |
| MemoryStick | Any, default | `memory` (plain) | breeze | default; no viewBox | no: several colours (deviation 37230 / 3812) | `970c6fd03a2cbf86` | `breeze-dark` |
| Menu | Mask, 16 | `open-menu` as `open-menu-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5bd0ec38997e0814` |  |
| Menu | Mask, 24 | `open-menu` as `open-menu-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `630ea84bb3657582` |  |
| Menu | Any, default | `open-menu` as `open-menu-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `630ea84bb3657582` |  |
| Mic | Mask, 16 | `audio-input-microphone` as `audio-input-microphone-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `1cfba4bce84ce52d` |  |
| Mic | Mask, 24 | `audio-input-microphone` as `audio-input-microphone-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `77459f22da9b0ad3` |  |
| Mic | Any, default | `audio-input-microphone` as `audio-input-microphone-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `77459f22da9b0ad3` |  |
| Minimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `ed5ca0625947021f` |  |
| Minimize | Mask, 24 | `window-minimize` as `window-minimize-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `3280a69a3d9440bd` |  |
| Minimize | Any, default | `window-minimize` as `window-minimize-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `3280a69a3d9440bd` |  |
| Minus | Mask, 16 | `list-remove` as `list-remove-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0a9be8349d2c0b4f` |  |
| Minus | Mask, 24 | `list-remove` as `list-remove-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `207220628bbac616` |  |
| Minus | Any, default | `list-remove` as `list-remove-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `207220628bbac616` |  |
| Moon | Mask, 16 | `weather-clear-night` as `weather-clear-night-symbolic` | breeze | 16; no viewBox | yes (deviation 570 / 3812) | `341efe5e9f225ade` |  |
| Moon | Mask, 24 | `weather-clear-night` as `weather-clear-night-symbolic` | breeze | 24; no viewBox | yes (deviation 570 / 3812) | `341efe5e9f225ade` |  |
| Moon | Any, default | `weather-clear-night` as `weather-clear-night-symbolic` | breeze | default; no viewBox | yes (deviation 570 / 3812) | `341efe5e9f225ade` |  |
| Network | Mask, 16 | `network-workgroup` as `network-workgroup-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c061e469bea64a65` |  |
| Network | Mask, 24 | `network-workgroup` as `network-workgroup-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 191 / 3812) | `8e3308e3379e888f` |  |
| Network | Any, default | `network-workgroup` as `network-workgroup-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 191 / 3812) | `8e3308e3379e888f` |  |
| Palette | Mask, 16 | `palette` as `palette-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `78637aebe2c9156d` |  |
| Palette | Mask, 24 | `palette` as `palette-symbolic` | breeze | 24; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `78637aebe2c9156d` |  |
| Palette | Any, default | `palette` as `palette-symbolic` | breeze | default; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `78637aebe2c9156d` |  |
| PanelBottom | Mask, 16 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `cba7cf686842597e` |  |
| PanelBottom | Mask, 24 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `28fe82d67883834c` |  |
| PanelBottom | Any, default | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `28fe82d67883834c` |  |
| PanelBottomOpen | Mask, 16 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `cba7cf686842597e` |  |
| PanelBottomOpen | Mask, 24 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `28fe82d67883834c` |  |
| PanelBottomOpen | Any, default | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `28fe82d67883834c` |  |
| PanelLeft | Mask, 16 | `sidebar-expand-left` as `sidebar-expand-left-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `03b4d50e736b1032` |  |
| PanelLeft | Mask, 24 | `sidebar-expand-left` as `sidebar-expand-left-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `8520f13cba9442cc` |  |
| PanelLeft | Any, default | `sidebar-expand-left` as `sidebar-expand-left-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `8520f13cba9442cc` |  |
| PanelLeftClose | Mask, 16 | `sidebar-show` as `sidebar-show-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `03b4d50e736b1032` |  |
| PanelLeftClose | Mask, 24 | `sidebar-show` as `sidebar-show-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `8520f13cba9442cc` |  |
| PanelLeftClose | Any, default | `view-left-close` as `view-left-close-symbolic` | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 34944 / 3812) | `45e57b4a2e43e582` |  |
| PanelLeftOpen | Mask, 16 | `view-left-new` as `view-left-new-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `e9bb5fc14f0fea07` |  |
| PanelLeftOpen | Mask, 24 | `view-left-new` as `view-left-new-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `37ae2c029524af0b` |  |
| PanelLeftOpen | Any, default | `view-left-new` as `view-left-new-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `37ae2c029524af0b` |  |
| PanelRight | Mask, 16 | `sidebar-expand-right` as `sidebar-expand-right-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `fd1a4c017b6e7d41` |  |
| PanelRight | Mask, 24 | `sidebar-expand-right` as `sidebar-expand-right-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `cd950a8d5c5eb82a` |  |
| PanelRight | Any, default | `sidebar-expand-right` as `sidebar-expand-right-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `cd950a8d5c5eb82a` |  |
| PanelRightClose | Mask, 16 | none | breeze | 16 | | | |
| PanelRightClose | Mask, 24 | none | breeze | 24 | | | |
| PanelRightClose | Any, default | `view-right-close` as `view-right-close-symbolic` | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 34944 / 3812) | `134443eec212329b` |  |
| PanelRightOpen | Mask, 16 | `view-right-new` as `view-right-new-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `631efb9b927d80a5` |  |
| PanelRightOpen | Mask, 24 | `view-right-new` as `view-right-new-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `9a127d31652f504f` |  |
| PanelRightOpen | Any, default | `view-right-new` as `view-right-new-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `9a127d31652f504f` |  |
| Pause | Mask, 16 | `media-playback-pause` as `media-playback-pause-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `b079ec02f5ad64be` |  |
| Pause | Mask, 24 | `media-playback-pause` as `media-playback-pause-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `2b0b4d88faacb6f9` |  |
| Pause | Any, default | `media-playback-pause` as `media-playback-pause-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `2b0b4d88faacb6f9` |  |
| Play | Mask, 16 | `media-playback-start` as `media-playback-start-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 191 / 3812) | `74f3d2e357272d64` |  |
| Play | Mask, 24 | `media-playback-start` as `media-playback-start-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `daed194571b8252e` |  |
| Play | Any, default | `media-playback-start` as `media-playback-start-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `daed194571b8252e` |  |
| Plus | Mask, 16 | `list-add` as `list-add-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6b2e1d04402d3f13` |  |
| Plus | Mask, 24 | `list-add` as `list-add-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `1d4a0900d18db0f1` |  |
| Plus | Any, default | `list-add` as `list-add-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `1d4a0900d18db0f1` |  |
| Redo | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `6ad935df86eb9db0` |  |
| Redo | Mask, 24 | `edit-redo` as `edit-redo-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `6ab7a8a96c193e2a` |  |
| Redo | Any, default | `edit-redo` as `edit-redo-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `6ab7a8a96c193e2a` |  |
| Redo2 | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `6ad935df86eb9db0` |  |
| Redo2 | Mask, 24 | `edit-redo` as `edit-redo-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `6ab7a8a96c193e2a` |  |
| Redo2 | Any, default | `edit-redo` as `edit-redo-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `6ab7a8a96c193e2a` |  |
| RefreshCw | Mask, 16 | `view-refresh` as `view-refresh-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `cbb75da0febc2e5a` |  |
| RefreshCw | Mask, 24 | `view-refresh` as `view-refresh-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `97d01e445c82852d` |  |
| RefreshCw | Any, default | `view-refresh` as `view-refresh-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `97d01e445c82852d` |  |
| Replace | Mask, 16 | `edit-find-replace` as `edit-find-replace-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `e00377dc24e99350` |  |
| Replace | Mask, 24 | `edit-find-replace` as `edit-find-replace-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `956dad0db5c92b0b` |  |
| Replace | Any, default | `edit-find-replace` as `edit-find-replace-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `956dad0db5c92b0b` |  |
| ResizeCorner | Mask, 16 | `drag-handle` as `drag-handle-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `022c022365c08b4c` |  |
| ResizeCorner | Mask, 24 | `drag-handle` as `drag-handle-symbolic` | breeze | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `022c022365c08b4c` |  |
| ResizeCorner | Any, default | `drag-handle` as `drag-handle-symbolic` | breeze | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `022c022365c08b4c` |  |
| RotateCw | Mask, 16 | `object-rotate-right` as `object-rotate-right-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `fc68f8c17b8db1bf` |  |
| RotateCw | Mask, 24 | `object-rotate-right` as `object-rotate-right-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `aea9054583224ecf` |  |
| RotateCw | Any, default | `object-rotate-right` as `object-rotate-right-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `aea9054583224ecf` |  |
| Search | Mask, 16 | `edit-find` as `edit-find-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `5a2c1613135c28bb` |  |
| Search | Mask, 24 | `edit-find` as `edit-find-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `ff9666c43d679605` |  |
| Search | Any, default | `edit-find` as `edit-find-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `ff9666c43d679605` |  |
| Settings | Mask, 16 | `preferences-system` as `preferences-system-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `8f7bfe2900a0f78d` |  |
| Settings | Mask, 24 | `preferences-system` as `preferences-system-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `e1d5125229ac97a7` |  |
| Settings | Any, default | `preferences-system` as `preferences-system-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `e1d5125229ac97a7` |  |
| Settings2 | Mask, 16 | `configure` as `configure-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `8f7bfe2900a0f78d` |  |
| Settings2 | Mask, 24 | `configure` as `configure-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `e1d5125229ac97a7` |  |
| Settings2 | Any, default | `configure` as `configure-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `e1d5125229ac97a7` |  |
| SortAscending | Mask, 16 | `view-sort-ascending` as `view-sort-ascending-symbolic` | breeze | 16; viewBox `0 0 4.2333332 4.2333335` | yes (deviation 192 / 3812) | `fefb013de39cd030` |  |
| SortAscending | Mask, 24 | `view-sort-ascending` as `view-sort-ascending-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `8def07aefa738ebd` |  |
| SortAscending | Any, default | `view-sort-ascending` as `view-sort-ascending-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `8def07aefa738ebd` |  |
| SortDescending | Mask, 16 | `view-sort-descending` as `view-sort-descending-symbolic` | breeze | 16; viewBox `0 0 4.2333332 4.2333335` | yes (deviation 192 / 3812) | `3e0c1a7566b5af1c` |  |
| SortDescending | Mask, 24 | `view-sort-descending` as `view-sort-descending-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `1ca6b863d936d859` |  |
| SortDescending | Any, default | `view-sort-descending` as `view-sort-descending-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `1ca6b863d936d859` |  |
| Square | Mask, 16 | `media-playback-stop` as `media-playback-stop-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4ca62b989db0cc51` |  |
| Square | Mask, 24 | `media-playback-stop` as `media-playback-stop-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `392d8c23e824d883` |  |
| Square | Any, default | `media-playback-stop` as `media-playback-stop-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `392d8c23e824d883` |  |
| SquareTerminal | Mask, 16 | `utilities-terminal` as `utilities-terminal-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `55279cac63820c07` |  |
| SquareTerminal | Mask, 24 | `utilities-terminal` as `utilities-terminal-symbolic` | breeze | 24; viewBox `0 0 22 22` | yes (deviation 160 / 3812) | `b4492ae2dd8f5c83` |  |
| SquareTerminal | Any, default | `utilities-terminal` as `utilities-terminal-symbolic` | breeze | default; viewBox `0 0 22 22` | yes (deviation 160 / 3812) | `b4492ae2dd8f5c83` |  |
| Star | Mask, 16 | `non-starred` as `non-starred-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `ee5d5a02d49d98b3` |  |
| Star | Mask, 24 | `non-starred` as `non-starred-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b89b9ad88781d92e` |  |
| Star | Any, default | `non-starred` as `non-starred-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b89b9ad88781d92e` |  |
| StarFill | Mask, 16 | `starred` as `starred-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `1a04acab760691f9` |  |
| StarFill | Mask, 24 | `starred` as `starred-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `7ac6055e8a1137ef` |  |
| StarFill | Any, default | `starred` as `starred-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `7ac6055e8a1137ef` |  |
| StarOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `ee5d5a02d49d98b3` |  |
| StarOff | Mask, 24 | `non-starred` as `non-starred-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b89b9ad88781d92e` |  |
| StarOff | Any, default | `non-starred` as `non-starred-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b89b9ad88781d92e` |  |
| Sun | Mask, 16 | `weather-clear` as `weather-clear-symbolic` | breeze | 16; no viewBox | yes (deviation 424 / 3812) | `4f620d3f7bb4bb0e` |  |
| Sun | Mask, 24 | `weather-clear` as `weather-clear-symbolic` | breeze | 24; no viewBox | yes (deviation 424 / 3812) | `4f620d3f7bb4bb0e` |  |
| Sun | Any, default | `weather-clear` as `weather-clear-symbolic` | breeze | default; no viewBox | yes (deviation 424 / 3812) | `4f620d3f7bb4bb0e` |  |
| ThumbsDown | Mask, 16 | `rating-unrated` as `rating-unrated-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `1f03abbaea43a7dd` |  |
| ThumbsDown | Mask, 24 | `rating-unrated` as `rating-unrated-symbolic` | breeze | 24; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `1f03abbaea43a7dd` |  |
| ThumbsDown | Any, default | `rating-unrated` as `rating-unrated-symbolic` | breeze | default; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `1f03abbaea43a7dd` |  |
| ThumbsUp | Mask, 16 | `approved` as `approved-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `58ac5a540c8134c8` |  |
| ThumbsUp | Mask, 24 | `approved` as `approved-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `efd0f050dc5141da` |  |
| ThumbsUp | Any, default | `approved` as `approved-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `efd0f050dc5141da` |  |
| TriangleAlert | Mask, 16 | none | breeze | 16 | | | |
| TriangleAlert | Mask, 24 | none | breeze | 24 | | | |
| TriangleAlert | Any, default | `dialog-warning` (plain) | breeze | default; viewBox `0 0 24 24` | no: several colours (deviation 48960 / 3812) | `bf59ed827fcc5b23` |  |
| Undo | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `7b784837cb2428b4` |  |
| Undo | Mask, 24 | `edit-undo` as `edit-undo-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `26663ce5e9327b3c` |  |
| Undo | Any, default | `edit-undo` as `edit-undo-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `26663ce5e9327b3c` |  |
| Undo2 | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `7b784837cb2428b4` |  |
| Undo2 | Mask, 24 | `edit-undo` as `edit-undo-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `26663ce5e9327b3c` |  |
| Undo2 | Any, default | `edit-undo` as `edit-undo-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `26663ce5e9327b3c` |  |
| User | Mask, 16 | `system-users` as `system-users-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `e354a0cac3a5a5e9` |  |
| User | Mask, 24 | `system-users` as `system-users-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `cc3236f28caae2cf` |  |
| User | Any, default | `system-users` as `system-users-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `cc3236f28caae2cf` |  |
| WindowClose | Mask, 16 | `window-close` as `window-close-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `99cf5eaec3ba2564` |  |
| WindowClose | Mask, 24 | `window-close` as `window-close-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `94c304d84e249c22` |  |
| WindowClose | Any, default | `window-close` as `window-close-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `94c304d84e249c22` |  |
| WindowMaximize | Mask, 16 | `window-maximize` as `window-maximize-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `dd9afec33cf1712e` |  |
| WindowMaximize | Mask, 24 | `window-maximize` as `window-maximize-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `6f279add3ccc9b89` |  |
| WindowMaximize | Any, default | `window-maximize` as `window-maximize-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `6f279add3ccc9b89` |  |
| WindowMinimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `ed5ca0625947021f` |  |
| WindowMinimize | Mask, 24 | `window-minimize` as `window-minimize-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `3280a69a3d9440bd` |  |
| WindowMinimize | Any, default | `window-minimize` as `window-minimize-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 128 / 3812) | `3280a69a3d9440bd` |  |
| WindowRestore | Mask, 16 | `window-restore` as `window-restore-symbolic` | breeze | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `f808ee3371596663` |  |
| WindowRestore | Mask, 24 | `window-restore` as `window-restore-symbolic` | breeze | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4a0e8a6cb4702f2e` |  |
| WindowRestore | Any, default | `window-restore` as `window-restore-symbolic` | breeze | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4a0e8a6cb4702f2e` |  |

### `breeze-dark`

| IconName | Lookup | Name found | Theme searched | Size | One colour | Hash | Same hash in |
|---|---|---|---|---|---|---|---|
| ALargeSmall | Mask, 16 | `format-font-size-more` as `format-font-size-more-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `55547edbd62de30b` |  |
| ALargeSmall | Mask, 24 | `format-font-size-more` as `format-font-size-more-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `e6bd6c080071176c` |  |
| ALargeSmall | Any, default | `format-font-size-more` as `format-font-size-more-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `e6bd6c080071176c` |  |
| ArrowDown | Mask, 16 | `go-down-skip` as `go-down-skip-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `1c695d87e607adb4` |  |
| ArrowDown | Mask, 24 | `go-down-skip` as `go-down-skip-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `1bd637d0fde07ce5` |  |
| ArrowDown | Any, default | `go-down-skip` as `go-down-skip-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `1bd637d0fde07ce5` |  |
| ArrowLeft | Mask, 16 | `go-previous-skip` as `go-previous-skip-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `79b710d6b9593806` |  |
| ArrowLeft | Mask, 24 | `go-previous-skip` as `go-previous-skip-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `03ba2003cc314be7` |  |
| ArrowLeft | Any, default | `go-previous-skip` as `go-previous-skip-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `03ba2003cc314be7` |  |
| ArrowRight | Mask, 16 | `go-next-skip` as `go-next-skip-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `fee9919441086b56` |  |
| ArrowRight | Mask, 24 | `go-next-skip` as `go-next-skip-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c70723118badfea5` |  |
| ArrowRight | Any, default | `go-next-skip` as `go-next-skip-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c70723118badfea5` |  |
| ArrowUp | Mask, 16 | `go-up-skip` as `go-up-skip-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `46d14502e49717aa` |  |
| ArrowUp | Mask, 24 | `go-up-skip` as `go-up-skip-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `71ae01dd8cd07227` |  |
| ArrowUp | Any, default | `go-up-skip` as `go-up-skip-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `71ae01dd8cd07227` |  |
| Asterisk | Mask, 16 | `rating` (plain) | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `9714fd08848b74bb` |  |
| Asterisk | Mask, 24 | `rating` (plain) | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 240 / 3812) | `23b9e570436f28df` |  |
| Asterisk | Any, default | `rating` (plain) | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 240 / 3812) | `23b9e570436f28df` |  |
| Ban | Mask, 16 | `action-unavailable` as `action-unavailable-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `9a7c0efd4465ea64` |  |
| Ban | Mask, 24 | `action-unavailable` as `action-unavailable-symbolic` | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `9a7c0efd4465ea64` |  |
| Ban | Any, default | `action-unavailable` as `action-unavailable-symbolic` | breeze-dark | default; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `9a7c0efd4465ea64` |  |
| Battery | Mask, 16 | none | breeze-dark | 16 | | | |
| Battery | Mask, 24 | none | breeze-dark | 24 | | | |
| Battery | Any, default | `battery` (plain) | breeze-dark | default; no viewBox | no: several colours (deviation 49215 / 3812) | `1ac1eca9c82e7da9` | `breeze` |
| BatteryCharging | Mask, 16 | none | breeze-dark | 16 | | | |
| BatteryCharging | Mask, 24 | none | breeze-dark | 24 | | | |
| BatteryCharging | Any, default | `battery-100-charging` as `battery-100-charging-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 54315 / 3812) | `5112806499db93e6` |  |
| BatteryFull | Mask, 16 | `battery-100` as `battery-100-symbolic` | breeze-dark | 16; no viewBox | yes (deviation 240 / 3812) | `8d8e9465a1534e6c` |  |
| BatteryFull | Mask, 24 | `battery-100` as `battery-100-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `4562cc143670a380` |  |
| BatteryFull | Any, default | `battery-100` as `battery-100-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `4562cc143670a380` |  |
| BatteryLow | Mask, 16 | `battery-020` as `battery-020-symbolic` | breeze-dark | 16; no viewBox | yes (deviation 240 / 3812) | `4909564e158b9b37` |  |
| BatteryLow | Mask, 24 | `battery-020` as `battery-020-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `89079f5c9d2d9c1a` |  |
| BatteryLow | Any, default | `battery-020` as `battery-020-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `89079f5c9d2d9c1a` |  |
| BatteryMedium | Mask, 16 | `battery-050` as `battery-050-symbolic` | breeze-dark | 16; no viewBox | yes (deviation 240 / 3812) | `cfa263f9b36fc324` |  |
| BatteryMedium | Mask, 24 | `battery-050` as `battery-050-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `97aa2924029b61b5` |  |
| BatteryMedium | Any, default | `battery-050` as `battery-050-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `97aa2924029b61b5` |  |
| BatteryWarning | Mask, 16 | `battery-010` as `battery-010-symbolic` | breeze-dark | 16; no viewBox | yes (deviation 240 / 3812) | `910727770d86e262` |  |
| BatteryWarning | Mask, 24 | `battery-010` as `battery-010-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `cb10ed3448844f6e` |  |
| BatteryWarning | Any, default | `battery-010` as `battery-010-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `cb10ed3448844f6e` |  |
| Bell | Mask, 16 | `notification-active` as `notification-active-symbolic` | breeze-dark | 16; no viewBox | yes (deviation 192 / 3812) | `cf4f27b8dad609f2` |  |
| Bell | Mask, 24 | `notification-active` as `notification-active-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 222 / 3812) | `4aabf14a414b7316` |  |
| Bell | Any, default | `notification-active` as `notification-active-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 222 / 3812) | `4aabf14a414b7316` |  |
| BookOpen | Mask, 16 | `help-contents` as `help-contents-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `193017aadc2040a6` |  |
| BookOpen | Mask, 24 | `help-contents` as `help-contents-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `42d9147184e3e63d` |  |
| BookOpen | Any, default | `help-contents` as `help-contents-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `42d9147184e3e63d` |  |
| Bot | Mask, 16 | none | breeze-dark | 16 | | | |
| Bot | Mask, 24 | none | breeze-dark | 24 | | | |
| Bot | Any, default | `face-smile` (plain) | breeze-dark | default; viewBox `0 0 5.8208332 5.8208335` | no: several colours (deviation 33150 / 3812) | `131684302091942f` | `breeze` |
| Building2 | Mask, 16 | `applications-office` as `applications-office-symbolic` | breeze-dark | 16; viewBox `0 0 22 22` | yes (deviation 240 / 3812) | `c20bc20887411bfa` |  |
| Building2 | Mask, 24 | `applications-office` as `applications-office-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 240 / 3812) | `c20bc20887411bfa` |  |
| Building2 | Any, default | `applications-office` as `applications-office-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 240 / 3812) | `c20bc20887411bfa` |  |
| Calendar | Mask, 16 | `view-calendar` as `view-calendar-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8ebf5fcf4fbf567c` |  |
| Calendar | Mask, 24 | `view-calendar` as `view-calendar-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `84c4b886c2944d34` |  |
| Calendar | Any, default | `view-calendar` as `view-calendar-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `84c4b886c2944d34` |  |
| CaseSensitive | Mask, 16 | none | breeze-dark | 16 | | | |
| CaseSensitive | Mask, 24 | none | breeze-dark | 24 | | | |
| CaseSensitive | Any, default | `format-text-uppercase` as `format-text-uppercase-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 42560 / 3812) | `9c6ed2d9c3a22f75` |  |
| ChartPie | Mask, 16 | `office-chart-pie` as `office-chart-pie-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `1f4707f34bc6c66a` |  |
| ChartPie | Mask, 24 | `office-chart-pie` as `office-chart-pie-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `9bfac420ee835bb7` |  |
| ChartPie | Any, default | `office-chart-pie` as `office-chart-pie-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `9bfac420ee835bb7` |  |
| Check | Mask, 16 | `dialog-ok` as `dialog-ok-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` |  |
| Check | Mask, 24 | `dialog-ok` as `dialog-ok-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `b00ffdbb5d4f80a5` |  |
| Check | Any, default | `dialog-ok` as `dialog-ok-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `b00ffdbb5d4f80a5` |  |
| ChevronDown | Mask, 16 | `go-down` as `go-down-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `b0ac80e68c954385` |  |
| ChevronDown | Mask, 24 | `go-down` as `go-down-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c6d8c47f59a6d1e7` |  |
| ChevronDown | Any, default | `go-down` as `go-down-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c6d8c47f59a6d1e7` |  |
| ChevronLeft | Mask, 16 | `go-previous` as `go-previous-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `632a9cca63757d29` |  |
| ChevronLeft | Mask, 24 | `go-previous` as `go-previous-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `4906047486ca19fa` |  |
| ChevronLeft | Any, default | `go-previous` as `go-previous-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `4906047486ca19fa` |  |
| ChevronRight | Mask, 16 | `go-next` as `go-next-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `584f2b9786ce5f50` |  |
| ChevronRight | Mask, 24 | `go-next` as `go-next-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `11622b3c81512089` |  |
| ChevronRight | Any, default | `go-next` as `go-next-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `11622b3c81512089` |  |
| ChevronsUpDown | Mask, 16 | `handle-sort` as `handle-sort-symbolic` | breeze-dark | 16; viewBox `0 0 22 22` | yes (deviation 126 / 3812) | `67c2e36088422b3e` |  |
| ChevronsUpDown | Mask, 24 | `handle-sort` as `handle-sort-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `f612d14d83e73772` |  |
| ChevronsUpDown | Any, default | `handle-sort` as `handle-sort-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `f612d14d83e73772` |  |
| ChevronUp | Mask, 16 | `go-up` as `go-up-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `5611e8e1a550b590` |  |
| ChevronUp | Mask, 24 | `go-up` as `go-up-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `380953cb2089e9c1` |  |
| ChevronUp | Any, default | `go-up` as `go-up-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `380953cb2089e9c1` |  |
| CircleAlert | Mask, 16 | `emblem-important` as `emblem-important-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `33e9c517d3920eb2` |  |
| CircleAlert | Mask, 24 | `emblem-important` as `emblem-important-symbolic` | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `33e9c517d3920eb2` |  |
| CircleAlert | Any, default | `emblem-important` as `emblem-important-symbolic` | breeze-dark | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `33e9c517d3920eb2` |  |
| CircleCheck | Mask, 16 | `emblem-ok-symbolic` (symbolic, as named) | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` |  |
| CircleCheck | Mask, 24 | `emblem-ok-symbolic` (symbolic, as named) | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` |  |
| CircleCheck | Any, default | `emblem-ok-symbolic` (symbolic, as named) | breeze-dark | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` |  |
| CircleUser | Mask, 16 | `user-identity` as `user-identity-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `ea61bff2647f04d3` |  |
| CircleUser | Mask, 24 | `user-identity` as `user-identity-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `7741af11680c83fd` |  |
| CircleUser | Any, default | `user-identity` as `user-identity-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `7741af11680c83fd` |  |
| CircleX | Mask, 16 | none | breeze-dark | 16 | | | |
| CircleX | Mask, 24 | none | breeze-dark | 24 | | | |
| CircleX | Any, default | `dialog-error` (plain) | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 47685 / 3812) | `1f9e83b1c2409605` |  |
| Close | Mask, 16 | `window-close` as `window-close-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `fec475d9f64dcf2c` |  |
| Close | Mask, 24 | `window-close` as `window-close-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `5cc27d0f1f4f05bc` |  |
| Close | Any, default | `tab-close` as `tab-close-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 46920 / 3812) | `e3a7cf2c8b647e2c` |  |
| Copy | Mask, 16 | `edit-copy` as `edit-copy-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 159 / 3812) | `a74bcc63c8e01533` |  |
| Copy | Mask, 24 | `edit-copy` as `edit-copy-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `3c59a9b43a1f2973` |  |
| Copy | Any, default | `edit-copy` as `edit-copy-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `3c59a9b43a1f2973` |  |
| Cpu | Mask, 16 | `computer` as `computer-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `50de050d5526f8eb` |  |
| Cpu | Mask, 24 | `computer` as `computer-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 0 / 3812) | `5fd4e1a26bd42882` |  |
| Cpu | Any, default | `cpu` (plain) | breeze-dark | default; no viewBox | no: several colours (deviation 25500 / 3812) | `abb72e5196fe4253` | `breeze` |
| Dash | Mask, 16 | `list-remove` as `list-remove-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `e573a187602106c2` |  |
| Dash | Mask, 24 | `list-remove` as `list-remove-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `51bde1d39abc9949` |  |
| Dash | Any, default | `list-remove` as `list-remove-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `51bde1d39abc9949` |  |
| Delete | Mask, 16 | `edit-delete` as `edit-delete-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `bfc4d808ef85f23a` |  |
| Delete | Mask, 24 | `edit-delete` as `edit-delete-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b2405516db73293c` |  |
| Delete | Any, default | `edit-delete` as `edit-delete-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 160 / 3812) | `b2405516db73293c` |  |
| Ellipsis | Mask, 16 | `overflow-menu` as `overflow-menu-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `a4da1d13fad43619` |  |
| Ellipsis | Mask, 24 | `overflow-menu` as `overflow-menu-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `51a3687323e4423f` |  |
| Ellipsis | Any, default | `overflow-menu` as `overflow-menu-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `51a3687323e4423f` |  |
| EllipsisVertical | Mask, 16 | `overflow-menu` as `overflow-menu-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `a4da1d13fad43619` |  |
| EllipsisVertical | Mask, 24 | `overflow-menu` as `overflow-menu-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `51a3687323e4423f` |  |
| EllipsisVertical | Any, default | `overflow-menu` as `overflow-menu-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `51a3687323e4423f` |  |
| ExternalLink | Mask, 16 | `external-link` as `external-link-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `3b338c5326c6923e` |  |
| ExternalLink | Mask, 24 | `external-link` as `external-link-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `3f258d6b315afb6a` |  |
| ExternalLink | Any, default | `external-link` as `external-link-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `3f258d6b315afb6a` |  |
| Eye | Mask, 16 | `view-visible` as `view-visible-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `4a844410717b90a9` |  |
| Eye | Mask, 24 | `view-visible` as `view-visible-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `5cdc5b6fc90a949f` |  |
| Eye | Any, default | `view-visible` as `view-visible-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `5cdc5b6fc90a949f` |  |
| EyeOff | Mask, 16 | `view-hidden` as `view-hidden-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `591a476dc8467a00` |  |
| EyeOff | Mask, 24 | `view-hidden` as `view-hidden-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `75300172430f8b2c` |  |
| EyeOff | Any, default | `view-hidden` as `view-hidden-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `75300172430f8b2c` |  |
| File | Mask, 16 | `text-x-generic` (plain) | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f6bf16d1d553b0a` | `breeze` |
| File | Mask, 24 | `text-x-generic` (plain) | breeze-dark | 24; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze` |
| File | Any, default | `text-x-generic` (plain) | breeze-dark | default; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze` |
| FileText | Mask, 16 | `text-x-generic` (plain) | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f6bf16d1d553b0a` | `breeze` |
| FileText | Mask, 24 | `text-x-generic` (plain) | breeze-dark | 24; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze` |
| FileText | Any, default | `text-x-generic` (plain) | breeze-dark | default; no viewBox | yes (deviation 0 / 3812) | `9e126a01fc15d0db` | `breeze` |
| Folder | Mask, 16 | `folder` as `folder-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `5dec625f544f9b52` |  |
| Folder | Mask, 24 | `folder` as `folder-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 239 / 3812) | `54af5ad3df06a0e9` |  |
| Folder | Any, default | `folder` as `folder-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 239 / 3812) | `54af5ad3df06a0e9` |  |
| FolderClosed | Mask, 16 | `folder` as `folder-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `5dec625f544f9b52` |  |
| FolderClosed | Mask, 24 | `folder` as `folder-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 239 / 3812) | `54af5ad3df06a0e9` |  |
| FolderClosed | Any, default | `folder` as `folder-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 239 / 3812) | `54af5ad3df06a0e9` |  |
| FolderOpen | Mask, 16 | `folder-open` as `folder-open-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 160 / 3812) | `36b1c1895c2cc557` |  |
| FolderOpen | Mask, 24 | `folder-open` as `folder-open-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `e4a7ec6f9e5d901f` |  |
| FolderOpen | Any, default | `folder-open` as `folder-open-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 224 / 3812) | `e4a7ec6f9e5d901f` |  |
| Frame | Mask, 16 | `select-rectangular` as `select-rectangular-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 14 / 3812) | `ab3a59c4bede8d8a` |  |
| Frame | Mask, 24 | `select-rectangular` as `select-rectangular-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 204 / 3812) | `005b8e4b0d992c47` |  |
| Frame | Any, default | `select-rectangular` as `select-rectangular-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 204 / 3812) | `005b8e4b0d992c47` |  |
| GalleryVerticalEnd | Mask, 16 | `view-list-icons` as `view-list-icons-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `de75685057dc6877` |  |
| GalleryVerticalEnd | Mask, 24 | `view-list-icons` as `view-list-icons-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `9c5f22f4c6a786e1` |  |
| GalleryVerticalEnd | Any, default | `view-list-icons` as `view-list-icons-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `9c5f22f4c6a786e1` |  |
| Github | Mask, 16 | `applications-engineering` as `applications-engineering-symbolic` | breeze-dark | 16; viewBox `0 0 22 22` | yes (deviation 96 / 3812) | `4e0db9289694aa8b` |  |
| Github | Mask, 24 | `applications-engineering` as `applications-engineering-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 96 / 3812) | `4e0db9289694aa8b` |  |
| Github | Any, default | `vcs-branch` as `vcs-branch-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 36108 / 3812) | `56f906ffaea2da72` |  |
| Globe | Mask, 16 | `globe` as `globe-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `f6124cda917cb08e` |  |
| Globe | Mask, 24 | `globe` as `globe-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `621f287f49f004da` |  |
| Globe | Any, default | `globe` as `globe-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `621f287f49f004da` |  |
| HardDrive | Mask, 16 | `drive-harddisk` as `drive-harddisk-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `efb8fd30e8ce8673` |  |
| HardDrive | Mask, 24 | `drive-harddisk` as `drive-harddisk-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 240 / 3812) | `97e3e4fe85eb0f33` |  |
| HardDrive | Any, default | `drive-harddisk` as `drive-harddisk-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 240 / 3812) | `97e3e4fe85eb0f33` |  |
| Heart | Mask, 16 | `emblem-favorite` as `emblem-favorite-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 64 / 3812) | `342a7d3826c31b57` |  |
| Heart | Mask, 24 | `emblem-favorite` as `emblem-favorite-symbolic` | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 64 / 3812) | `342a7d3826c31b57` |  |
| Heart | Any, default | `emblem-favorite` as `emblem-favorite-symbolic` | breeze-dark | default; viewBox `0 0 16 16` | yes (deviation 64 / 3812) | `342a7d3826c31b57` |  |
| HeartOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `9dbc44682de104bb` |  |
| HeartOff | Mask, 24 | `non-starred` as `non-starred-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `68ce347ab0846ad0` |  |
| HeartOff | Any, default | `non-starred` as `non-starred-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `68ce347ab0846ad0` |  |
| Inbox | Mask, 16 | `mail-folder-inbox` as `mail-folder-inbox-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `4f23b52995e6ee43` |  |
| Inbox | Mask, 24 | `mail-folder-inbox` as `mail-folder-inbox-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `b74352ca4d75bfcd` |  |
| Inbox | Any, default | `mail-folder-inbox` as `mail-folder-inbox-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 192 / 3812) | `b74352ca4d75bfcd` |  |
| Info | Mask, 16 | none | breeze-dark | 16 | | | |
| Info | Mask, 24 | none | breeze-dark | 24 | | | |
| Info | Any, default | `dialog-information` (plain) | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 37230 / 3812) | `376bd61251af1329` |  |
| Inspector | Mask, 16 | `code-context` as `code-context-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `6f4f3ad3b32c4ba2` |  |
| Inspector | Mask, 24 | `code-context` as `code-context-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `6f0f841cd9c3852d` |  |
| Inspector | Any, default | `code-context` as `code-context-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `6f0f841cd9c3852d` |  |
| LayoutDashboard | Mask, 16 | `view-grid` as `view-grid-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f0fc293a7191f5e0` |  |
| LayoutDashboard | Mask, 24 | `view-grid` as `view-grid-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ed912250c2305bc7` |  |
| LayoutDashboard | Any, default | `view-grid` as `view-grid-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ed912250c2305bc7` |  |
| Loader | Mask, 16 | `process-working-symbolic` (symbolic, as named) | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `2f8f10c84a9bd7b5` |  |
| Loader | Mask, 24 | `process-working-symbolic` (symbolic, as named) | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `80e99014277552ac` |  |
| Loader | Any, default | `process-working-symbolic` (symbolic, as named) | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `80e99014277552ac` |  |
| LoaderCircle | Mask, 16 | `process-working-symbolic` (symbolic, as named) | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `2f8f10c84a9bd7b5` |  |
| LoaderCircle | Mask, 24 | `process-working-symbolic` (symbolic, as named) | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `80e99014277552ac` |  |
| LoaderCircle | Any, default | `process-working-symbolic` (symbolic, as named) | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 240 / 3812) | `80e99014277552ac` |  |
| Map | Mask, 16 | `find-location` as `find-location-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `315f326e0a6f49f7` |  |
| Map | Mask, 24 | `find-location` as `find-location-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `9e1c8c1c38efbbb2` |  |
| Map | Any, default | `find-location` as `find-location-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `9e1c8c1c38efbbb2` |  |
| Maximize | Mask, 16 | `view-fullscreen` as `view-fullscreen-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 160 / 3812) | `f7bd04a2764fa755` |  |
| Maximize | Mask, 24 | `view-fullscreen` as `view-fullscreen-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `71263000f287fba1` |  |
| Maximize | Any, default | `view-fullscreen` as `view-fullscreen-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `71263000f287fba1` |  |
| MemoryStick | Mask, 16 | `media-flash` as `media-flash-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f5364ac5ff7bc27a` |  |
| MemoryStick | Mask, 24 | `media-flash` as `media-flash-symbolic` | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f5364ac5ff7bc27a` |  |
| MemoryStick | Any, default | `memory` (plain) | breeze-dark | default; no viewBox | no: several colours (deviation 37230 / 3812) | `970c6fd03a2cbf86` | `breeze` |
| Menu | Mask, 16 | `open-menu` as `open-menu-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `56980dc5605a12fc` |  |
| Menu | Mask, 24 | `open-menu` as `open-menu-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `951bd549e68bbb8c` |  |
| Menu | Any, default | `open-menu` as `open-menu-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `951bd549e68bbb8c` |  |
| Mic | Mask, 16 | `audio-input-microphone` as `audio-input-microphone-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `c337894c4f6896a9` |  |
| Mic | Mask, 24 | `audio-input-microphone` as `audio-input-microphone-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 239 / 3812) | `39e2d4a7c15dc865` |  |
| Mic | Any, default | `audio-input-microphone` as `audio-input-microphone-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 239 / 3812) | `39e2d4a7c15dc865` |  |
| Minimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `b0ac80e68c954385` |  |
| Minimize | Mask, 24 | `window-minimize` as `window-minimize-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c6d8c47f59a6d1e7` |  |
| Minimize | Any, default | `window-minimize` as `window-minimize-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c6d8c47f59a6d1e7` |  |
| Minus | Mask, 16 | `list-remove` as `list-remove-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `e573a187602106c2` |  |
| Minus | Mask, 24 | `list-remove` as `list-remove-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `51bde1d39abc9949` |  |
| Minus | Any, default | `list-remove` as `list-remove-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 176 / 3812) | `51bde1d39abc9949` |  |
| Moon | Mask, 16 | `weather-clear-night` as `weather-clear-night-symbolic` | breeze-dark | 16; no viewBox | yes (deviation 240 / 3812) | `9c352462d523de4e` |  |
| Moon | Mask, 24 | `weather-clear-night` as `weather-clear-night-symbolic` | breeze-dark | 24; no viewBox | yes (deviation 240 / 3812) | `9c352462d523de4e` |  |
| Moon | Any, default | `weather-clear-night` as `weather-clear-night-symbolic` | breeze-dark | default; no viewBox | yes (deviation 240 / 3812) | `9c352462d523de4e` |  |
| Network | Mask, 16 | `network-workgroup` as `network-workgroup-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `cf43d5533545a057` |  |
| Network | Mask, 24 | `network-workgroup` as `network-workgroup-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 208 / 3812) | `41b8dd88327a1525` |  |
| Network | Any, default | `network-workgroup` as `network-workgroup-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 208 / 3812) | `41b8dd88327a1525` |  |
| Palette | Mask, 16 | `palette` as `palette-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `2d804cb91f80da4b` |  |
| Palette | Mask, 24 | `palette` as `palette-symbolic` | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `2d804cb91f80da4b` |  |
| Palette | Any, default | `palette` as `palette-symbolic` | breeze-dark | default; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `2d804cb91f80da4b` |  |
| PanelBottom | Mask, 16 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `b8cb87cb1cdeed7e` |  |
| PanelBottom | Mask, 24 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `e9b7ee1bd68aa37e` |  |
| PanelBottom | Any, default | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `e9b7ee1bd68aa37e` |  |
| PanelBottomOpen | Mask, 16 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `b8cb87cb1cdeed7e` |  |
| PanelBottomOpen | Mask, 24 | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `e9b7ee1bd68aa37e` |  |
| PanelBottomOpen | Any, default | `view-split-top-bottom` as `view-split-top-bottom-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `e9b7ee1bd68aa37e` |  |
| PanelLeft | Mask, 16 | `sidebar-expand-left` as `sidebar-expand-left-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `e6229aca219088fc` |  |
| PanelLeft | Mask, 24 | `sidebar-expand-left` as `sidebar-expand-left-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `6bdada1c462be074` |  |
| PanelLeft | Any, default | `sidebar-expand-left` as `sidebar-expand-left-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `6bdada1c462be074` |  |
| PanelLeftClose | Mask, 16 | `sidebar-show` as `sidebar-show-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `e6229aca219088fc` |  |
| PanelLeftClose | Mask, 24 | `sidebar-show` as `sidebar-show-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `6bdada1c462be074` |  |
| PanelLeftClose | Any, default | `view-left-close` as `view-left-close-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 35520 / 3812) | `935efecc06b4f496` |  |
| PanelLeftOpen | Mask, 16 | `view-left-new` as `view-left-new-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0e4a70e499dbb6f1` |  |
| PanelLeftOpen | Mask, 24 | `view-left-new` as `view-left-new-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `0cfcfa1827f1d293` |  |
| PanelLeftOpen | Any, default | `view-left-new` as `view-left-new-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `0cfcfa1827f1d293` |  |
| PanelRight | Mask, 16 | `sidebar-expand-right` as `sidebar-expand-right-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `5d5598d032cbcbc3` |  |
| PanelRight | Mask, 24 | `sidebar-expand-right` as `sidebar-expand-right-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `2f47343ed35619c2` |  |
| PanelRight | Any, default | `sidebar-expand-right` as `sidebar-expand-right-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `2f47343ed35619c2` |  |
| PanelRightClose | Mask, 16 | none | breeze-dark | 16 | | | |
| PanelRightClose | Mask, 24 | none | breeze-dark | 24 | | | |
| PanelRightClose | Any, default | `view-right-close` as `view-right-close-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 35520 / 3812) | `77149974f586dbaf` |  |
| PanelRightOpen | Mask, 16 | `view-right-new` as `view-right-new-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `22d46d1c4720a09b` |  |
| PanelRightOpen | Mask, 24 | `view-right-new` as `view-right-new-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `9c963bf82f0ef4c9` |  |
| PanelRightOpen | Any, default | `view-right-new` as `view-right-new-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 208 / 3812) | `9c963bf82f0ef4c9` |  |
| Pause | Mask, 16 | `media-playback-pause` as `media-playback-pause-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8074e222aca5a0b4` |  |
| Pause | Mask, 24 | `media-playback-pause` as `media-playback-pause-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `9cfd9b8be640af51` |  |
| Pause | Any, default | `media-playback-pause` as `media-playback-pause-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `9cfd9b8be640af51` |  |
| Play | Mask, 16 | `media-playback-start` as `media-playback-start-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `363cd545fb27c682` |  |
| Play | Mask, 24 | `media-playback-start` as `media-playback-start-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `243c87b49d792616` |  |
| Play | Any, default | `media-playback-start` as `media-playback-start-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `243c87b49d792616` |  |
| Plus | Mask, 16 | `list-add` as `list-add-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `1056f394e1ea5211` |  |
| Plus | Mask, 24 | `list-add` as `list-add-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `5b0c1af1141cf5ff` |  |
| Plus | Any, default | `list-add` as `list-add-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `5b0c1af1141cf5ff` |  |
| Redo | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `35d1a62df009b25c` |  |
| Redo | Mask, 24 | `edit-redo` as `edit-redo-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `955e0ed367e2d0a4` |  |
| Redo | Any, default | `edit-redo` as `edit-redo-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `955e0ed367e2d0a4` |  |
| Redo2 | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `35d1a62df009b25c` |  |
| Redo2 | Mask, 24 | `edit-redo` as `edit-redo-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `955e0ed367e2d0a4` |  |
| Redo2 | Any, default | `edit-redo` as `edit-redo-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `955e0ed367e2d0a4` |  |
| RefreshCw | Mask, 16 | `view-refresh` as `view-refresh-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `24f82fc9713e3292` |  |
| RefreshCw | Mask, 24 | `view-refresh` as `view-refresh-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `83b2c993c96d45a7` |  |
| RefreshCw | Any, default | `view-refresh` as `view-refresh-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `83b2c993c96d45a7` |  |
| Replace | Mask, 16 | `edit-find-replace` as `edit-find-replace-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `dd777118da3bc308` |  |
| Replace | Mask, 24 | `edit-find-replace` as `edit-find-replace-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `2fbeab2a2905c245` |  |
| Replace | Any, default | `edit-find-replace` as `edit-find-replace-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `2fbeab2a2905c245` |  |
| ResizeCorner | Mask, 16 | `drag-handle` as `drag-handle-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `af4e5f77a029ca88` |  |
| ResizeCorner | Mask, 24 | `drag-handle` as `drag-handle-symbolic` | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `af4e5f77a029ca88` |  |
| ResizeCorner | Any, default | `drag-handle` as `drag-handle-symbolic` | breeze-dark | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `af4e5f77a029ca88` |  |
| RotateCw | Mask, 16 | `object-rotate-right` as `object-rotate-right-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `9656d0f9bca80529` |  |
| RotateCw | Mask, 24 | `object-rotate-right` as `object-rotate-right-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `52c4479b7a80d271` |  |
| RotateCw | Any, default | `object-rotate-right` as `object-rotate-right-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `52c4479b7a80d271` |  |
| Search | Mask, 16 | `edit-find` as `edit-find-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `5c91c0ff963d2781` |  |
| Search | Mask, 24 | `edit-find` as `edit-find-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `8744e655c2478b83` |  |
| Search | Any, default | `edit-find` as `edit-find-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `8744e655c2478b83` |  |
| Settings | Mask, 16 | `preferences-system` as `preferences-system-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `376b4d1b33c6e853` |  |
| Settings | Mask, 24 | `preferences-system` as `preferences-system-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `3d430d0a83862bbd` |  |
| Settings | Any, default | `preferences-system` as `preferences-system-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `3d430d0a83862bbd` |  |
| Settings2 | Mask, 16 | `configure` as `configure-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `376b4d1b33c6e853` |  |
| Settings2 | Mask, 24 | `configure` as `configure-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `3d430d0a83862bbd` |  |
| Settings2 | Any, default | `configure` as `configure-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `3d430d0a83862bbd` |  |
| SortAscending | Mask, 16 | `view-sort-ascending` as `view-sort-ascending-symbolic` | breeze-dark | 16; viewBox `0 0 4.2333332 4.2333335` | yes (deviation 239 / 3812) | `50b2e75540dad08e` |  |
| SortAscending | Mask, 24 | `view-sort-ascending` as `view-sort-ascending-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `be68dc79f9b9d547` |  |
| SortAscending | Any, default | `view-sort-ascending` as `view-sort-ascending-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `be68dc79f9b9d547` |  |
| SortDescending | Mask, 16 | `view-sort-descending` as `view-sort-descending-symbolic` | breeze-dark | 16; viewBox `0 0 4.2333332 4.2333335` | yes (deviation 240 / 3812) | `b5c18571723a627c` |  |
| SortDescending | Mask, 24 | `view-sort-descending` as `view-sort-descending-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `ab03fbb81865efbf` |  |
| SortDescending | Any, default | `view-sort-descending` as `view-sort-descending-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `ab03fbb81865efbf` |  |
| Square | Mask, 16 | `media-playback-stop` as `media-playback-stop-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `11f280085e7619e7` |  |
| Square | Mask, 24 | `media-playback-stop` as `media-playback-stop-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `e38e4674cabf420b` |  |
| Square | Any, default | `media-playback-stop` as `media-playback-stop-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 189 / 3812) | `e38e4674cabf420b` |  |
| SquareTerminal | Mask, 16 | `utilities-terminal` as `utilities-terminal-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `1be2dfd599493601` |  |
| SquareTerminal | Mask, 24 | `utilities-terminal` as `utilities-terminal-symbolic` | breeze-dark | 24; viewBox `0 0 22 22` | yes (deviation 96 / 3812) | `3c90a36635370c4d` |  |
| SquareTerminal | Any, default | `utilities-terminal` as `utilities-terminal-symbolic` | breeze-dark | default; viewBox `0 0 22 22` | yes (deviation 96 / 3812) | `3c90a36635370c4d` |  |
| Star | Mask, 16 | `non-starred` as `non-starred-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `9dbc44682de104bb` |  |
| Star | Mask, 24 | `non-starred` as `non-starred-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `68ce347ab0846ad0` |  |
| Star | Any, default | `non-starred` as `non-starred-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `68ce347ab0846ad0` |  |
| StarFill | Mask, 16 | `starred` as `starred-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `c0bdec7c4a50ef91` |  |
| StarFill | Mask, 24 | `starred` as `starred-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `d1afc9fb565a962d` |  |
| StarFill | Any, default | `starred` as `starred-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 239 / 3812) | `d1afc9fb565a962d` |  |
| StarOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `9dbc44682de104bb` |  |
| StarOff | Mask, 24 | `non-starred` as `non-starred-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `68ce347ab0846ad0` |  |
| StarOff | Any, default | `non-starred` as `non-starred-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `68ce347ab0846ad0` |  |
| Sun | Mask, 16 | `weather-clear` as `weather-clear-symbolic` | breeze-dark | 16; no viewBox | yes (deviation 125 / 3812) | `83f4080da35a24aa` |  |
| Sun | Mask, 24 | `weather-clear` as `weather-clear-symbolic` | breeze-dark | 24; no viewBox | yes (deviation 125 / 3812) | `83f4080da35a24aa` |  |
| Sun | Any, default | `weather-clear` as `weather-clear-symbolic` | breeze-dark | default; no viewBox | yes (deviation 125 / 3812) | `83f4080da35a24aa` |  |
| ThumbsDown | Mask, 16 | `rating-unrated` as `rating-unrated-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `cf6a0b6bd32b56e3` |  |
| ThumbsDown | Mask, 24 | `rating-unrated` as `rating-unrated-symbolic` | breeze-dark | 24; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `cf6a0b6bd32b56e3` |  |
| ThumbsDown | Any, default | `rating-unrated` as `rating-unrated-symbolic` | breeze-dark | default; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `cf6a0b6bd32b56e3` |  |
| ThumbsUp | Mask, 16 | `approved` as `approved-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `c6c8a33410d18004` |  |
| ThumbsUp | Mask, 24 | `approved` as `approved-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `3c012dccfd9b5cac` |  |
| ThumbsUp | Any, default | `approved` as `approved-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 224 / 3812) | `3c012dccfd9b5cac` |  |
| TriangleAlert | Mask, 16 | none | breeze-dark | 16 | | | |
| TriangleAlert | Mask, 24 | none | breeze-dark | 24 | | | |
| TriangleAlert | Any, default | `dialog-warning` (plain) | breeze-dark | default; viewBox `0 0 24 24` | no: several colours (deviation 48960 / 3812) | `b0e322a683478cc0` |  |
| Undo | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `85f9d13fc9de411c` |  |
| Undo | Mask, 24 | `edit-undo` as `edit-undo-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `ad03e5bec78233e2` |  |
| Undo | Any, default | `edit-undo` as `edit-undo-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `ad03e5bec78233e2` |  |
| Undo2 | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `85f9d13fc9de411c` |  |
| Undo2 | Mask, 24 | `edit-undo` as `edit-undo-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `ad03e5bec78233e2` |  |
| Undo2 | Any, default | `edit-undo` as `edit-undo-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 126 / 3812) | `ad03e5bec78233e2` |  |
| User | Mask, 16 | `system-users` as `system-users-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `57a0cc8df2d6eb9f` |  |
| User | Mask, 24 | `system-users` as `system-users-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `09c273a28b95b6e1` |  |
| User | Any, default | `system-users` as `system-users-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 192 / 3812) | `09c273a28b95b6e1` |  |
| WindowClose | Mask, 16 | `window-close` as `window-close-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `fec475d9f64dcf2c` |  |
| WindowClose | Mask, 24 | `window-close` as `window-close-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `5cc27d0f1f4f05bc` |  |
| WindowClose | Any, default | `window-close` as `window-close-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 96 / 3812) | `5cc27d0f1f4f05bc` |  |
| WindowMaximize | Mask, 16 | `window-maximize` as `window-maximize-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `5611e8e1a550b590` |  |
| WindowMaximize | Mask, 24 | `window-maximize` as `window-maximize-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `380953cb2089e9c1` |  |
| WindowMaximize | Any, default | `window-maximize` as `window-maximize-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `380953cb2089e9c1` |  |
| WindowMinimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `b0ac80e68c954385` |  |
| WindowMinimize | Mask, 24 | `window-minimize` as `window-minimize-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c6d8c47f59a6d1e7` |  |
| WindowMinimize | Any, default | `window-minimize` as `window-minimize-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 64 / 3812) | `c6d8c47f59a6d1e7` |  |
| WindowRestore | Mask, 16 | `window-restore` as `window-restore-symbolic` | breeze-dark | 16; viewBox `0 0 16 16` | yes (deviation 96 / 3812) | `7dfd1b73c66cb4dd` |  |
| WindowRestore | Mask, 24 | `window-restore` as `window-restore-symbolic` | breeze-dark | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d0fa8c2c8d3825ce` |  |
| WindowRestore | Any, default | `window-restore` as `window-restore-symbolic` | breeze-dark | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d0fa8c2c8d3825ce` |  |

### `Adwaita`

| IconName | Lookup | Name found | Theme searched | Size | One colour | Hash | Same hash in |
|---|---|---|---|---|---|---|---|
| ALargeSmall | Mask, 16 | `zoom-in` as `zoom-in-symbolic` | Adwaita | 16; viewBox `0 0 15.9844 16` | yes (deviation 0 / 3812) | `179c43170e5a108c` |  |
| ALargeSmall | Mask, 24 | `zoom-in` as `zoom-in-symbolic` | Adwaita | 24; viewBox `0 0 15.9844 16` | yes (deviation 0 / 3812) | `179c43170e5a108c` |  |
| ALargeSmall | Any, default | `zoom-in` as `zoom-in-symbolic` | Adwaita | default; viewBox `0 0 15.9844 16` | yes (deviation 239 / 3812) | `955aa8a6b33d1e3a` |  |
| ArrowDown | Mask, 16 | `go-bottom` as `go-bottom-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `cbce9487d10c4dea` |  |
| ArrowDown | Mask, 24 | `go-bottom` as `go-bottom-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `cbce9487d10c4dea` |  |
| ArrowDown | Any, default | `go-bottom` as `go-bottom-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `b81c569768455f30` |  |
| ArrowLeft | Mask, 16 | `go-first` as `go-first-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `68e9711c00a016af` |  |
| ArrowLeft | Mask, 24 | `go-first` as `go-first-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `68e9711c00a016af` |  |
| ArrowLeft | Any, default | `go-first` as `go-first-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `8a70f415f7e69afd` |  |
| ArrowRight | Mask, 16 | `go-last` as `go-last-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `bf8c3c88b2e9a11a` |  |
| ArrowRight | Mask, 24 | `go-last` as `go-last-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `bf8c3c88b2e9a11a` |  |
| ArrowRight | Any, default | `go-last` as `go-last-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `6f9035c13b322460` |  |
| ArrowUp | Mask, 16 | `go-top` as `go-top-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3789397b9b7eb9dc` |  |
| ArrowUp | Mask, 24 | `go-top` as `go-top-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3789397b9b7eb9dc` |  |
| ArrowUp | Any, default | `go-top` as `go-top-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `a752199c4024aada` |  |
| Asterisk | Mask, 16 | `starred` as `starred-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6052d683c5e94043` |  |
| Asterisk | Mask, 24 | `starred` as `starred-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6052d683c5e94043` |  |
| Asterisk | Any, default | `starred` as `starred-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `7b2227b686e04cc1` |  |
| Ban | Mask, 16 | `action-unavailable` as `action-unavailable-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ca33669f2852db32` |  |
| Ban | Mask, 24 | `action-unavailable` as `action-unavailable-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ca33669f2852db32` |  |
| Ban | Any, default | `action-unavailable` as `action-unavailable-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `fc8e08427f52f0d4` |  |
| Battery | Mask, 16 | `battery` as `battery-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ac4efcfb92198c9a` |  |
| Battery | Mask, 24 | `battery` as `battery-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ac4efcfb92198c9a` |  |
| Battery | Any, default | `battery` as `battery-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `e0d56af9db8e46c4` |  |
| BatteryCharging | Mask, 16 | `battery-full-charging` as `battery-full-charging-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c596244bb30006cf` |  |
| BatteryCharging | Mask, 24 | `battery-full-charging` as `battery-full-charging-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c596244bb30006cf` |  |
| BatteryCharging | Any, default | `battery-full-charging` as `battery-full-charging-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `9c64e3b1aee4e729` |  |
| BatteryFull | Mask, 16 | `battery-full` as `battery-full-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `1b2ceef4d77db5f1` |  |
| BatteryFull | Mask, 24 | `battery-full` as `battery-full-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `1b2ceef4d77db5f1` |  |
| BatteryFull | Any, default | `battery-full` as `battery-full-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `1199b362aabe5187` |  |
| BatteryLow | Mask, 16 | `battery-low` as `battery-low-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `434487d557a1c30d` |  |
| BatteryLow | Mask, 24 | `battery-low` as `battery-low-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `434487d557a1c30d` |  |
| BatteryLow | Any, default | `battery-low` as `battery-low-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `434487d557a1c30d` |  |
| BatteryMedium | Mask, 16 | `battery-good` as `battery-good-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `1ccc8e2a2b603f29` |  |
| BatteryMedium | Mask, 24 | `battery-good` as `battery-good-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `1ccc8e2a2b603f29` |  |
| BatteryMedium | Any, default | `battery-good` as `battery-good-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `d88a97ba6e81aa67` |  |
| BatteryWarning | Mask, 16 | `battery-caution` as `battery-caution-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 143 / 3812) | `13b2388f07ac754a` |  |
| BatteryWarning | Mask, 24 | `battery-caution` as `battery-caution-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 143 / 3812) | `13b2388f07ac754a` |  |
| BatteryWarning | Any, default | `battery-caution` as `battery-caution-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 143 / 3812) | `13b2388f07ac754a` |  |
| Bell | Mask, 16 | `alarm` as `alarm-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2f09986ccab74a91` |  |
| Bell | Mask, 24 | `alarm` as `alarm-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2f09986ccab74a91` |  |
| Bell | Any, default | `alarm` as `alarm-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `090f511aeca8a677` |  |
| BookOpen | Mask, 16 | `help-contents` as `help-contents-symbolic` | Adwaita | 16; no viewBox | yes (deviation 0 / 3812) | `d45ef39c76f4ad31` |  |
| BookOpen | Mask, 24 | `help-contents` as `help-contents-symbolic` | Adwaita | 24; no viewBox | yes (deviation 0 / 3812) | `d45ef39c76f4ad31` |  |
| BookOpen | Any, default | `help-contents` as `help-contents-symbolic` | Adwaita | default; no viewBox | yes (deviation 0 / 3812) | `d45ef39c76f4ad31` |  |
| Bot | Mask, 16 | `face-smile` as `face-smile-symbolic` | Adwaita | 16; no viewBox | yes (deviation 0 / 3812) | `1e8bf93ff365e18a` |  |
| Bot | Mask, 24 | `face-smile` as `face-smile-symbolic` | Adwaita | 24; no viewBox | yes (deviation 0 / 3812) | `1e8bf93ff365e18a` |  |
| Bot | Any, default | `face-smile` as `face-smile-symbolic` | Adwaita | default; no viewBox | yes (deviation 239 / 3812) | `2d6237548900b862` |  |
| Building2 | Mask, 16 | `network-workgroup` as `network-workgroup-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `badc447e7c43d04a` |  |
| Building2 | Mask, 24 | `network-workgroup` as `network-workgroup-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `badc447e7c43d04a` |  |
| Building2 | Any, default | `network-workgroup` as `network-workgroup-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `4e3d1477cf7d2adc` |  |
| Calendar | Mask, 16 | `x-office-calendar` as `x-office-calendar-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c17aec4823e2b700` |  |
| Calendar | Mask, 24 | `x-office-calendar` as `x-office-calendar-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c17aec4823e2b700` |  |
| Calendar | Any, default | `x-office-calendar` as `x-office-calendar-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `b9156b168b9cbdfa` |  |
| CaseSensitive | Mask, 16 | `format-text-rich` as `format-text-rich-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2f3d9ffabddcccbb` |  |
| CaseSensitive | Mask, 24 | `format-text-rich` as `format-text-rich-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2f3d9ffabddcccbb` |  |
| CaseSensitive | Any, default | `format-text-rich` as `format-text-rich-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `450f64abbc9643a9` |  |
| ChartPie | Mask, 16 | `x-office-spreadsheet` as `x-office-spreadsheet-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `828dec802cbd67fb` |  |
| ChartPie | Mask, 24 | `x-office-spreadsheet` as `x-office-spreadsheet-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `828dec802cbd67fb` |  |
| ChartPie | Any, default | `x-office-spreadsheet` as `x-office-spreadsheet-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `367c12e233629bb9` |  |
| Check | Mask, 16 | `object-select` as `object-select-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c644d75d68273e7` |  |
| Check | Mask, 24 | `object-select` as `object-select-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c644d75d68273e7` |  |
| Check | Any, default | `object-select` as `object-select-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `fb52cb18941178fd` |  |
| ChevronDown | Mask, 16 | `go-down` as `go-down-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `34ac62c114abb570` |  |
| ChevronDown | Mask, 24 | `go-down` as `go-down-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `34ac62c114abb570` |  |
| ChevronDown | Any, default | `go-down` as `go-down-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `bb9ab942bcd4f172` |  |
| ChevronLeft | Mask, 16 | `go-previous` as `go-previous-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `670472afbd491974` |  |
| ChevronLeft | Mask, 24 | `go-previous` as `go-previous-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `670472afbd491974` |  |
| ChevronLeft | Any, default | `go-previous` as `go-previous-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `d17caf51a19f625a` |  |
| ChevronRight | Mask, 16 | `go-next` as `go-next-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `17877726410215b6` |  |
| ChevronRight | Mask, 24 | `go-next` as `go-next-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `17877726410215b6` |  |
| ChevronRight | Any, default | `go-next` as `go-next-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `389c4eb99932b834` |  |
| ChevronsUpDown | Mask, 16 | `list-drag-handle` as `list-drag-handle-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `aafd8a8e11dee182` |  |
| ChevronsUpDown | Mask, 24 | `list-drag-handle` as `list-drag-handle-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `aafd8a8e11dee182` |  |
| ChevronsUpDown | Any, default | `list-drag-handle` as `list-drag-handle-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 128 / 3812) | `a4832084d003233c` |  |
| ChevronUp | Mask, 16 | `go-up` as `go-up-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `05884c17c1a7fa5f` |  |
| ChevronUp | Mask, 24 | `go-up` as `go-up-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `05884c17c1a7fa5f` |  |
| ChevronUp | Any, default | `go-up` as `go-up-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `d2ba1688312d86d9` |  |
| CircleAlert | Mask, 16 | `emblem-important` as `emblem-important-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `42a389f4861182ca` |  |
| CircleAlert | Mask, 24 | `emblem-important` as `emblem-important-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `42a389f4861182ca` |  |
| CircleAlert | Any, default | `emblem-important` as `emblem-important-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `a370416c16350014` |  |
| CircleCheck | Mask, 16 | `object-select` as `object-select-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c644d75d68273e7` |  |
| CircleCheck | Mask, 24 | `object-select` as `object-select-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c644d75d68273e7` |  |
| CircleCheck | Any, default | `object-select` as `object-select-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `fb52cb18941178fd` |  |
| CircleUser | Mask, 16 | `avatar-default` as `avatar-default-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `006726bd1a29826c` |  |
| CircleUser | Mask, 24 | `avatar-default` as `avatar-default-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `006726bd1a29826c` |  |
| CircleUser | Any, default | `avatar-default` as `avatar-default-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `eccc610bf28fa456` |  |
| CircleX | Mask, 16 | `dialog-error` as `dialog-error-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `b16efe53d49895e5` |  |
| CircleX | Mask, 24 | `dialog-error` as `dialog-error-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `b16efe53d49895e5` |  |
| CircleX | Any, default | `dialog-error` as `dialog-error-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `b3c8f743bdb3599d` |  |
| Close | Mask, 16 | `window-close` as `window-close-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0efc92212262d981` |  |
| Close | Mask, 24 | `window-close` as `window-close-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0efc92212262d981` |  |
| Close | Any, default | `window-close` as `window-close-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `a967f40fa5fb165f` |  |
| Copy | Mask, 16 | `edit-copy` as `edit-copy-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f314c37a34f2c86f` |  |
| Copy | Mask, 24 | `edit-copy` as `edit-copy-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f314c37a34f2c86f` |  |
| Copy | Any, default | `edit-copy` as `edit-copy-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `4247f45a507be13d` |  |
| Cpu | Mask, 16 | `computer` as `computer-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6f7be0707eb0e46b` |  |
| Cpu | Mask, 24 | `computer` as `computer-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6f7be0707eb0e46b` |  |
| Cpu | Any, default | `computer` as `computer-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `f21baa17ca0b2641` |  |
| Dash | Mask, 16 | `list-remove` as `list-remove-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f605218560cfd03` |  |
| Dash | Mask, 24 | `list-remove` as `list-remove-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f605218560cfd03` |  |
| Dash | Any, default | `list-remove` as `list-remove-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `39496ae2396959d3` |  |
| Delete | Mask, 16 | `edit-delete` as `edit-delete-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c479dca7fbf18b0` |  |
| Delete | Mask, 24 | `edit-delete` as `edit-delete-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c479dca7fbf18b0` |  |
| Delete | Any, default | `edit-delete` as `edit-delete-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `b06b93cc0394eff2` |  |
| Ellipsis | Mask, 16 | `view-more-horizontal` as `view-more-horizontal-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5d95b8a0f63a03d8` |  |
| Ellipsis | Mask, 24 | `view-more-horizontal` as `view-more-horizontal-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5d95b8a0f63a03d8` |  |
| Ellipsis | Any, default | `view-more-horizontal` as `view-more-horizontal-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 64 / 3812) | `4e3ca29372dec6d6` |  |
| EllipsisVertical | Mask, 16 | `view-more` as `view-more-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `9911559874f29e1e` |  |
| EllipsisVertical | Mask, 24 | `view-more` as `view-more-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `9911559874f29e1e` |  |
| EllipsisVertical | Any, default | `view-more` as `view-more-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `1e17e16815629eec` |  |
| ExternalLink | Mask, 16 | `insert-link` as `insert-link-symbolic` | Adwaita | 16; viewBox `0 0 16 15.9805` | yes (deviation 0 / 3812) | `4d000a861f1b5b39` |  |
| ExternalLink | Mask, 24 | `insert-link` as `insert-link-symbolic` | Adwaita | 24; viewBox `0 0 16 15.9805` | yes (deviation 0 / 3812) | `4d000a861f1b5b39` |  |
| ExternalLink | Any, default | `insert-link` as `insert-link-symbolic` | Adwaita | default; viewBox `0 0 16 15.9805` | yes (deviation 208 / 3812) | `01161bae9c590317` |  |
| Eye | Mask, 16 | `view-reveal` as `view-reveal-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5c6e6d69dc0c1b85` |  |
| Eye | Mask, 24 | `view-reveal` as `view-reveal-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5c6e6d69dc0c1b85` |  |
| Eye | Any, default | `view-reveal` as `view-reveal-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `c1bdb57aad7a6f23` |  |
| EyeOff | Mask, 16 | `view-conceal` as `view-conceal-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `aec6323ae3369280` |  |
| EyeOff | Mask, 24 | `view-conceal` as `view-conceal-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `aec6323ae3369280` |  |
| EyeOff | Any, default | `view-conceal` as `view-conceal-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `42ffb0dcb5a03eda` |  |
| File | Mask, 16 | `text-x-generic` as `text-x-generic-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `bbb0323cf36b9bca` |  |
| File | Mask, 24 | `text-x-generic` as `text-x-generic-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `bbb0323cf36b9bca` |  |
| File | Any, default | `text-x-generic` as `text-x-generic-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `98bc1ba0a3c16194` |  |
| FileText | Mask, 16 | `text-x-generic` as `text-x-generic-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `bbb0323cf36b9bca` |  |
| FileText | Mask, 24 | `text-x-generic` as `text-x-generic-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `bbb0323cf36b9bca` |  |
| FileText | Any, default | `text-x-generic` as `text-x-generic-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `98bc1ba0a3c16194` |  |
| Folder | Mask, 16 | `folder` as `folder-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2a0e275c95aa0b14` |  |
| Folder | Mask, 24 | `folder` as `folder-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2a0e275c95aa0b14` |  |
| Folder | Any, default | `folder` as `folder-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `d046a47d0d0d522e` |  |
| FolderClosed | Mask, 16 | `folder` as `folder-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2a0e275c95aa0b14` |  |
| FolderClosed | Mask, 24 | `folder` as `folder-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2a0e275c95aa0b14` |  |
| FolderClosed | Any, default | `folder` as `folder-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `d046a47d0d0d522e` |  |
| FolderOpen | Mask, 16 | `folder-open` as `folder-open-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c9e4d83a9b4d7779` |  |
| FolderOpen | Mask, 24 | `folder-open` as `folder-open-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c9e4d83a9b4d7779` |  |
| FolderOpen | Any, default | `folder-open` as `folder-open-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `35a8a2dd0d6e3c6f` |  |
| Frame | Mask, 16 | `selection-mode` as `selection-mode-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f70267e1e0665731` |  |
| Frame | Mask, 24 | `selection-mode` as `selection-mode-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `f70267e1e0665731` |  |
| Frame | Any, default | `selection-mode` as `selection-mode-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `f6ace20709da42a7` |  |
| GalleryVerticalEnd | Mask, 16 | `view-paged` as `view-paged-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `1cf5d6288915b068` |  |
| GalleryVerticalEnd | Mask, 24 | `view-paged` as `view-paged-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `1cf5d6288915b068` |  |
| GalleryVerticalEnd | Any, default | `view-paged` as `view-paged-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `1b78856ccbc36756` |  |
| Github | Mask, 16 | `applications-engineering` as `applications-engineering-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4fbf86d40cbd21fa` |  |
| Github | Mask, 24 | `applications-engineering` as `applications-engineering-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4fbf86d40cbd21fa` |  |
| Github | Any, default | `applications-engineering` as `applications-engineering-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `18186ae68ed38bac` |  |
| Globe | Mask, 16 | `web-browser` as `web-browser-symbolic` | Adwaita | 16; no viewBox | yes (deviation 0 / 3812) | `a4194c929c8329a8` |  |
| Globe | Mask, 24 | `web-browser` as `web-browser-symbolic` | Adwaita | 24; no viewBox | yes (deviation 0 / 3812) | `a4194c929c8329a8` |  |
| Globe | Any, default | `web-browser` as `web-browser-symbolic` | Adwaita | default; no viewBox | yes (deviation 298 / 3812) | `7fcb3c2a3fe12f18` |  |
| HardDrive | Mask, 16 | `drive-harddisk` as `drive-harddisk-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `7a7978ddb9b1f775` |  |
| HardDrive | Mask, 24 | `drive-harddisk` as `drive-harddisk-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `7a7978ddb9b1f775` |  |
| HardDrive | Any, default | `drive-harddisk` as `drive-harddisk-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `75570bf7a342bf13` |  |
| Heart | Mask, 16 | `starred` as `starred-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6052d683c5e94043` |  |
| Heart | Mask, 24 | `starred` as `starred-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6052d683c5e94043` |  |
| Heart | Any, default | `starred` as `starred-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `7b2227b686e04cc1` |  |
| HeartOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4102eabe2e71b4f7` |  |
| HeartOff | Mask, 24 | `non-starred` as `non-starred-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4102eabe2e71b4f7` |  |
| HeartOff | Any, default | `non-starred` as `non-starred-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 186 / 3812) | `8f3d23aabe67daf5` |  |
| Inbox | Mask, 16 | `mail-send-receive` as `mail-send-receive-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8ea596c88f1c2493` |  |
| Inbox | Mask, 24 | `mail-send-receive` as `mail-send-receive-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8ea596c88f1c2493` |  |
| Inbox | Any, default | `mail-send-receive` as `mail-send-receive-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `0aec59eb94d2c46d` |  |
| Info | Mask, 16 | `dialog-information` as `dialog-information-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `222b7ef886c8a9b4` |  |
| Info | Mask, 24 | `dialog-information` as `dialog-information-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `222b7ef886c8a9b4` |  |
| Info | Any, default | `dialog-information` as `dialog-information-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `8b9bcc23644950ba` |  |
| Inspector | Mask, 16 | `preferences-system-details` as `preferences-system-details-symbolic` | Adwaita | 16; no viewBox | yes (deviation 176 / 3812) | `c5713354683d78c8` |  |
| Inspector | Mask, 24 | `preferences-system-details` as `preferences-system-details-symbolic` | Adwaita | 24; no viewBox | yes (deviation 176 / 3812) | `c5713354683d78c8` |  |
| Inspector | Any, default | `preferences-system-details` as `preferences-system-details-symbolic` | Adwaita | default; no viewBox | yes (deviation 176 / 3812) | `54f5df90cd6b49c8` |  |
| LayoutDashboard | Mask, 16 | `view-grid` as `view-grid-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `617e3029d0a9feaa` |  |
| LayoutDashboard | Mask, 24 | `view-grid` as `view-grid-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `617e3029d0a9feaa` |  |
| LayoutDashboard | Any, default | `view-grid` as `view-grid-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `850dcb91289bc96c` |  |
| Loader | Mask, 16 | `content-loading` as `content-loading-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0580dd08401064f9` |  |
| Loader | Mask, 24 | `content-loading` as `content-loading-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0580dd08401064f9` |  |
| Loader | Any, default | `content-loading` as `content-loading-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 64 / 3812) | `55cb55da563899ab` |  |
| LoaderCircle | Mask, 16 | `content-loading` as `content-loading-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0580dd08401064f9` |  |
| LoaderCircle | Mask, 24 | `content-loading` as `content-loading-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0580dd08401064f9` |  |
| LoaderCircle | Any, default | `content-loading` as `content-loading-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 64 / 3812) | `55cb55da563899ab` |  |
| Map | Mask, 16 | `find-location` as `find-location-symbolic` | Adwaita | 16; viewBox `0 0 15.976562 16` | yes (deviation 0 / 3812) | `2487b1a320fd6b28` |  |
| Map | Mask, 24 | `find-location` as `find-location-symbolic` | Adwaita | 24; viewBox `0 0 15.976562 16` | yes (deviation 0 / 3812) | `2487b1a320fd6b28` |  |
| Map | Any, default | `find-location` as `find-location-symbolic` | Adwaita | default; viewBox `0 0 15.976562 16` | yes (deviation 208 / 3812) | `0a16c06869958562` |  |
| Maximize | Mask, 16 | `view-fullscreen` as `view-fullscreen-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ad500ec8e7361e4c` |  |
| Maximize | Mask, 24 | `view-fullscreen` as `view-fullscreen-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ad500ec8e7361e4c` |  |
| Maximize | Any, default | `view-fullscreen` as `view-fullscreen-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `15eaf548ad0ce582` |  |
| MemoryStick | Mask, 16 | `media-flash` as `media-flash-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `cab852e030d870ec` |  |
| MemoryStick | Mask, 24 | `media-flash` as `media-flash-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `cab852e030d870ec` |  |
| MemoryStick | Any, default | `media-flash` as `media-flash-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `111602dcc2a6c882` |  |
| Menu | Mask, 16 | `open-menu` as `open-menu-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8e0ad7ba45736b41` |  |
| Menu | Mask, 24 | `open-menu` as `open-menu-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8e0ad7ba45736b41` |  |
| Menu | Any, default | `open-menu` as `open-menu-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `87b88d44f09091ff` |  |
| Mic | Mask, 16 | `audio-input-microphone` as `audio-input-microphone-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `59ed0ae58b7898c5` |  |
| Mic | Mask, 24 | `audio-input-microphone` as `audio-input-microphone-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `59ed0ae58b7898c5` |  |
| Mic | Any, default | `audio-input-microphone` as `audio-input-microphone-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `ba2e4d72eac47317` |  |
| Minimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ad3b28af2fd32dcb` |  |
| Minimize | Mask, 24 | `window-minimize` as `window-minimize-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ad3b28af2fd32dcb` |  |
| Minimize | Any, default | `window-minimize` as `window-minimize-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `618df835281a120b` |  |
| Minus | Mask, 16 | `list-remove` as `list-remove-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f605218560cfd03` |  |
| Minus | Mask, 24 | `list-remove` as `list-remove-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3f605218560cfd03` |  |
| Minus | Any, default | `list-remove` as `list-remove-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `39496ae2396959d3` |  |
| Moon | Mask, 16 | `weather-clear-night` as `weather-clear-night-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `4e6384f8958c6cbb` |  |
| Moon | Mask, 24 | `weather-clear-night` as `weather-clear-night-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `4e6384f8958c6cbb` |  |
| Moon | Any, default | `weather-clear-night` as `weather-clear-night-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `4e6384f8958c6cbb` |  |
| Network | Mask, 16 | `network-workgroup` as `network-workgroup-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `badc447e7c43d04a` |  |
| Network | Mask, 24 | `network-workgroup` as `network-workgroup-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `badc447e7c43d04a` |  |
| Network | Any, default | `network-workgroup` as `network-workgroup-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `4e3d1477cf7d2adc` |  |
| Palette | Mask, 16 | `color-select` as `color-select-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2f7bc6880bf2a9df` |  |
| Palette | Mask, 24 | `color-select` as `color-select-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `2f7bc6880bf2a9df` |  |
| Palette | Any, default | `color-select` as `color-select-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `441ba69b431dc649` |  |
| PanelBottom | Mask, 16 | `view-dual` as `view-dual-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `359fbcfce2b4ab4d` |  |
| PanelBottom | Mask, 24 | `view-dual` as `view-dual-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `359fbcfce2b4ab4d` |  |
| PanelBottom | Any, default | `view-dual` as `view-dual-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `4330f84e4645feab` |  |
| PanelBottomOpen | Mask, 16 | `view-dual` as `view-dual-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `359fbcfce2b4ab4d` |  |
| PanelBottomOpen | Mask, 24 | `view-dual` as `view-dual-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `359fbcfce2b4ab4d` |  |
| PanelBottomOpen | Any, default | `view-dual` as `view-dual-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `4330f84e4645feab` |  |
| PanelLeft | Mask, 16 | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4c34f9ec02447eea` |  |
| PanelLeft | Mask, 24 | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4c34f9ec02447eea` |  |
| PanelLeft | Any, default | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 321 / 3812) | `c20b62a5c6f3fd48` |  |
| PanelLeftClose | Mask, 16 | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4c34f9ec02447eea` |  |
| PanelLeftClose | Mask, 24 | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4c34f9ec02447eea` |  |
| PanelLeftClose | Any, default | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 321 / 3812) | `c20b62a5c6f3fd48` |  |
| PanelLeftOpen | Mask, 16 | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4c34f9ec02447eea` |  |
| PanelLeftOpen | Mask, 24 | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4c34f9ec02447eea` |  |
| PanelLeftOpen | Any, default | `sidebar-show` as `sidebar-show-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 321 / 3812) | `c20b62a5c6f3fd48` |  |
| PanelRight | Mask, 16 | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4a4463fa61cb343f` |  |
| PanelRight | Mask, 24 | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4a4463fa61cb343f` |  |
| PanelRight | Any, default | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 321 / 3812) | `0413f4fb15c0ebad` |  |
| PanelRightClose | Mask, 16 | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4a4463fa61cb343f` |  |
| PanelRightClose | Mask, 24 | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4a4463fa61cb343f` |  |
| PanelRightClose | Any, default | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 321 / 3812) | `0413f4fb15c0ebad` |  |
| PanelRightOpen | Mask, 16 | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4a4463fa61cb343f` |  |
| PanelRightOpen | Mask, 24 | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4a4463fa61cb343f` |  |
| PanelRightOpen | Any, default | `sidebar-show-right` as `sidebar-show-right-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 321 / 3812) | `0413f4fb15c0ebad` |  |
| Pause | Mask, 16 | `media-playback-pause` as `media-playback-pause-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `38a3cb41866012b0` |  |
| Pause | Mask, 24 | `media-playback-pause` as `media-playback-pause-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `38a3cb41866012b0` |  |
| Pause | Any, default | `media-playback-pause` as `media-playback-pause-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `287721ad237c500e` |  |
| Play | Mask, 16 | `media-playback-start` as `media-playback-start-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `60535cce91fd3b04` |  |
| Play | Mask, 24 | `media-playback-start` as `media-playback-start-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `60535cce91fd3b04` |  |
| Play | Any, default | `media-playback-start` as `media-playback-start-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `b1911485b8b183ca` |  |
| Plus | Mask, 16 | `list-add` as `list-add-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4f16c5165bf42c7a` |  |
| Plus | Mask, 24 | `list-add` as `list-add-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4f16c5165bf42c7a` |  |
| Plus | Any, default | `list-add` as `list-add-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8b285d1be48bbb8c` |  |
| Redo | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `38f2bbd8e60f92e9` |  |
| Redo | Mask, 24 | `edit-redo` as `edit-redo-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `38f2bbd8e60f92e9` |  |
| Redo | Any, default | `edit-redo` as `edit-redo-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `be037db40f70ba63` |  |
| Redo2 | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `38f2bbd8e60f92e9` |  |
| Redo2 | Mask, 24 | `edit-redo` as `edit-redo-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `38f2bbd8e60f92e9` |  |
| Redo2 | Any, default | `edit-redo` as `edit-redo-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `be037db40f70ba63` |  |
| RefreshCw | Mask, 16 | `view-refresh` as `view-refresh-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `282cb8486eec37c3` |  |
| RefreshCw | Mask, 24 | `view-refresh` as `view-refresh-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `282cb8486eec37c3` |  |
| RefreshCw | Any, default | `view-refresh` as `view-refresh-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `a2b89c361e1ddcd1` |  |
| Replace | Mask, 16 | `edit-find-replace` as `edit-find-replace-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c412e3d6f6464b2d` |  |
| Replace | Mask, 24 | `edit-find-replace` as `edit-find-replace-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `c412e3d6f6464b2d` |  |
| Replace | Any, default | `edit-find-replace` as `edit-find-replace-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `6bb9568ecfcbb56b` |  |
| ResizeCorner | Mask, 16 | `list-drag-handle` as `list-drag-handle-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `aafd8a8e11dee182` |  |
| ResizeCorner | Mask, 24 | `list-drag-handle` as `list-drag-handle-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `aafd8a8e11dee182` |  |
| ResizeCorner | Any, default | `list-drag-handle` as `list-drag-handle-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 128 / 3812) | `a4832084d003233c` |  |
| RotateCw | Mask, 16 | `object-rotate-right` as `object-rotate-right-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `484ec7f7330a8731` |  |
| RotateCw | Mask, 24 | `object-rotate-right` as `object-rotate-right-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `484ec7f7330a8731` |  |
| RotateCw | Any, default | `object-rotate-right` as `object-rotate-right-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `2b15c03253c10333` |  |
| Search | Mask, 16 | `edit-find` as `edit-find-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5d2d0437e9d30b2d` |  |
| Search | Mask, 24 | `edit-find` as `edit-find-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `5d2d0437e9d30b2d` |  |
| Search | Any, default | `edit-find` as `edit-find-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `82dda30d7722d037` |  |
| Settings | Mask, 16 | `preferences-system` as `preferences-system-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `b33201d86820d61a` |  |
| Settings | Mask, 24 | `preferences-system` as `preferences-system-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `b33201d86820d61a` |  |
| Settings | Any, default | `preferences-system` as `preferences-system-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `61c3c39edb7a9394` |  |
| Settings2 | Mask, 16 | `preferences-other` as `preferences-other-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `e48917ee25f389ef` |  |
| Settings2 | Mask, 24 | `preferences-other` as `preferences-other-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `e48917ee25f389ef` |  |
| Settings2 | Any, default | `preferences-other` as `preferences-other-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `5c311dd3ad129df5` |  |
| SortAscending | Mask, 16 | `view-sort-ascending` as `view-sort-ascending-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `325ad1f8697f734e` |  |
| SortAscending | Mask, 24 | `view-sort-ascending` as `view-sort-ascending-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `325ad1f8697f734e` |  |
| SortAscending | Any, default | `view-sort-ascending` as `view-sort-ascending-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `6268785f5edb5570` |  |
| SortDescending | Mask, 16 | `view-sort-descending` as `view-sort-descending-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0fa0cb834e230c47` |  |
| SortDescending | Mask, 24 | `view-sort-descending` as `view-sort-descending-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0fa0cb834e230c47` |  |
| SortDescending | Any, default | `view-sort-descending` as `view-sort-descending-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `ed18b2c9cf23ebc1` |  |
| Square | Mask, 16 | `media-playback-stop` as `media-playback-stop-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4645c18b191f13c2` |  |
| Square | Mask, 24 | `media-playback-stop` as `media-playback-stop-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4645c18b191f13c2` |  |
| Square | Any, default | `media-playback-stop` as `media-playback-stop-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 128 / 3812) | `5b65d717b4cee114` |  |
| SquareTerminal | Mask, 16 | `utilities-terminal` as `utilities-terminal-symbolic` | Adwaita | 16; no viewBox | yes (deviation 223 / 3812) | `3941b80ab0d9d8ac` |  |
| SquareTerminal | Mask, 24 | `utilities-terminal` as `utilities-terminal-symbolic` | Adwaita | 24; no viewBox | yes (deviation 223 / 3812) | `3941b80ab0d9d8ac` |  |
| SquareTerminal | Any, default | `utilities-terminal` as `utilities-terminal-symbolic` | Adwaita | default; no viewBox | yes (deviation 192 / 3812) | `2562c95001daa3cc` |  |
| Star | Mask, 16 | `non-starred` as `non-starred-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4102eabe2e71b4f7` |  |
| Star | Mask, 24 | `non-starred` as `non-starred-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4102eabe2e71b4f7` |  |
| Star | Any, default | `non-starred` as `non-starred-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 186 / 3812) | `8f3d23aabe67daf5` |  |
| StarFill | Mask, 16 | `starred` as `starred-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6052d683c5e94043` |  |
| StarFill | Mask, 24 | `starred` as `starred-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6052d683c5e94043` |  |
| StarFill | Any, default | `starred` as `starred-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `7b2227b686e04cc1` |  |
| StarOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4102eabe2e71b4f7` |  |
| StarOff | Mask, 24 | `non-starred` as `non-starred-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `4102eabe2e71b4f7` |  |
| StarOff | Any, default | `non-starred` as `non-starred-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 186 / 3812) | `8f3d23aabe67daf5` |  |
| Sun | Mask, 16 | `weather-clear` as `weather-clear-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `709e99ff89923203` |  |
| Sun | Mask, 24 | `weather-clear` as `weather-clear-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `709e99ff89923203` |  |
| Sun | Any, default | `weather-clear` as `weather-clear-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `709e99ff89923203` |  |
| ThumbsDown | Mask, 16 | `process-stop` as `process-stop-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `693e7ee13649f5a0` |  |
| ThumbsDown | Mask, 24 | `process-stop` as `process-stop-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `693e7ee13649f5a0` |  |
| ThumbsDown | Any, default | `process-stop` as `process-stop-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `564a6db78347db0e` |  |
| ThumbsUp | Mask, 16 | `checkbox-checked` as `checkbox-checked-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8869b483ed9b5396` |  |
| ThumbsUp | Mask, 24 | `checkbox-checked` as `checkbox-checked-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `8869b483ed9b5396` |  |
| ThumbsUp | Any, default | `checkbox-checked` as `checkbox-checked-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `d59d5f8ef9b95cd8` |  |
| TriangleAlert | Mask, 16 | `dialog-warning` as `dialog-warning-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `9078d699872eed0a` |  |
| TriangleAlert | Mask, 24 | `dialog-warning` as `dialog-warning-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `9078d699872eed0a` |  |
| TriangleAlert | Any, default | `dialog-warning` as `dialog-warning-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `4d1e2957511b5c10` |  |
| Undo | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ae38e7741c933a30` |  |
| Undo | Mask, 24 | `edit-undo` as `edit-undo-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ae38e7741c933a30` |  |
| Undo | Any, default | `edit-undo` as `edit-undo-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `97a1457ffa359afa` |  |
| Undo2 | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ae38e7741c933a30` |  |
| Undo2 | Mask, 24 | `edit-undo` as `edit-undo-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ae38e7741c933a30` |  |
| Undo2 | Any, default | `edit-undo` as `edit-undo-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `97a1457ffa359afa` |  |
| User | Mask, 16 | `system-users` as `system-users-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `441f403e7120ae96` |  |
| User | Mask, 24 | `system-users` as `system-users-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `441f403e7120ae96` |  |
| User | Any, default | `system-users` as `system-users-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `b788714b0dc68898` |  |
| WindowClose | Mask, 16 | `window-close` as `window-close-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0efc92212262d981` |  |
| WindowClose | Mask, 24 | `window-close` as `window-close-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0efc92212262d981` |  |
| WindowClose | Any, default | `window-close` as `window-close-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 208 / 3812) | `a967f40fa5fb165f` |  |
| WindowMaximize | Mask, 16 | `window-maximize` as `window-maximize-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `a50bb530b77a3244` |  |
| WindowMaximize | Mask, 24 | `window-maximize` as `window-maximize-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `a50bb530b77a3244` |  |
| WindowMaximize | Any, default | `window-maximize` as `window-maximize-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `519c5e18e269c222` |  |
| WindowMinimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ad3b28af2fd32dcb` |  |
| WindowMinimize | Mask, 24 | `window-minimize` as `window-minimize-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ad3b28af2fd32dcb` |  |
| WindowMinimize | Any, default | `window-minimize` as `window-minimize-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `618df835281a120b` |  |
| WindowRestore | Mask, 16 | `window-restore` as `window-restore-symbolic` | Adwaita | 16; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `424369cb7988ab1d` |  |
| WindowRestore | Mask, 24 | `window-restore` as `window-restore-symbolic` | Adwaita | 24; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `424369cb7988ab1d` |  |
| WindowRestore | Any, default | `window-restore` as `window-restore-symbolic` | Adwaita | default; viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `ddd47091f98b81eb` |  |

### `lucide`

| IconName | Lookup | Name found | Theme searched | Size | One colour | Hash | Same hash in |
|---|---|---|---|---|---|---|---|
| ALargeSmall | Mask, 16 | a-large-small | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b8fee3994b702700` |  |
| ALargeSmall | Mask, 24 | a-large-small | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b8fee3994b702700` |  |
| ALargeSmall | Any, default | a-large-small | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b8fee3994b702700` |  |
| ArrowDown | Mask, 16 | arrow-down | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1f633e273dc7cd20` |  |
| ArrowDown | Mask, 24 | arrow-down | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1f633e273dc7cd20` |  |
| ArrowDown | Any, default | arrow-down | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1f633e273dc7cd20` |  |
| ArrowLeft | Mask, 16 | arrow-left | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2dc877d6d4621dff` |  |
| ArrowLeft | Mask, 24 | arrow-left | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2dc877d6d4621dff` |  |
| ArrowLeft | Any, default | arrow-left | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2dc877d6d4621dff` |  |
| ArrowRight | Mask, 16 | arrow-right | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `cc2d19335b1acca9` |  |
| ArrowRight | Mask, 24 | arrow-right | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `cc2d19335b1acca9` |  |
| ArrowRight | Any, default | arrow-right | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `cc2d19335b1acca9` |  |
| ArrowUp | Mask, 16 | arrow-up | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `905c661328a5e86c` |  |
| ArrowUp | Mask, 24 | arrow-up | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `905c661328a5e86c` |  |
| ArrowUp | Any, default | arrow-up | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `905c661328a5e86c` |  |
| Asterisk | Mask, 16 | asterisk | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e2831b476832ecab` |  |
| Asterisk | Mask, 24 | asterisk | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e2831b476832ecab` |  |
| Asterisk | Any, default | asterisk | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e2831b476832ecab` |  |
| Ban | Mask, 16 | ban | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `25c5bc41c874faca` |  |
| Ban | Mask, 24 | ban | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `25c5bc41c874faca` |  |
| Ban | Any, default | ban | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `25c5bc41c874faca` |  |
| Battery | Mask, 16 | battery | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `dd1214b42a4e679b` |  |
| Battery | Mask, 24 | battery | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `dd1214b42a4e679b` |  |
| Battery | Any, default | battery | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `dd1214b42a4e679b` |  |
| BatteryCharging | Mask, 16 | battery-charging | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6b9668fef57c307a` |  |
| BatteryCharging | Mask, 24 | battery-charging | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6b9668fef57c307a` |  |
| BatteryCharging | Any, default | battery-charging | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6b9668fef57c307a` |  |
| BatteryFull | Mask, 16 | battery-full | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6afc67b52a92bd3e` |  |
| BatteryFull | Mask, 24 | battery-full | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6afc67b52a92bd3e` |  |
| BatteryFull | Any, default | battery-full | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6afc67b52a92bd3e` |  |
| BatteryLow | Mask, 16 | battery-low | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `bea67590ea734535` |  |
| BatteryLow | Mask, 24 | battery-low | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `bea67590ea734535` |  |
| BatteryLow | Any, default | battery-low | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `bea67590ea734535` |  |
| BatteryMedium | Mask, 16 | battery-medium | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b2380295eecb2872` |  |
| BatteryMedium | Mask, 24 | battery-medium | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b2380295eecb2872` |  |
| BatteryMedium | Any, default | battery-medium | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b2380295eecb2872` |  |
| BatteryWarning | Mask, 16 | battery-warning | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7c5c239f881e87a0` |  |
| BatteryWarning | Mask, 24 | battery-warning | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7c5c239f881e87a0` |  |
| BatteryWarning | Any, default | battery-warning | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7c5c239f881e87a0` |  |
| Bell | Mask, 16 | bell | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `29c2f6fc90398fa4` |  |
| Bell | Mask, 24 | bell | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `29c2f6fc90398fa4` |  |
| Bell | Any, default | bell | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `29c2f6fc90398fa4` |  |
| BookOpen | Mask, 16 | book-open | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1db3559c20ae798e` |  |
| BookOpen | Mask, 24 | book-open | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1db3559c20ae798e` |  |
| BookOpen | Any, default | book-open | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1db3559c20ae798e` |  |
| Bot | Mask, 16 | bot | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `adc6b8dffe7699d9` |  |
| Bot | Mask, 24 | bot | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `adc6b8dffe7699d9` |  |
| Bot | Any, default | bot | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `adc6b8dffe7699d9` |  |
| Building2 | Mask, 16 | building-2 | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `20239c3bfa4823b2` |  |
| Building2 | Mask, 24 | building-2 | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `20239c3bfa4823b2` |  |
| Building2 | Any, default | building-2 | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `20239c3bfa4823b2` |  |
| Calendar | Mask, 16 | calendar | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6343634727750f63` |  |
| Calendar | Mask, 24 | calendar | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6343634727750f63` |  |
| Calendar | Any, default | calendar | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6343634727750f63` |  |
| CaseSensitive | Mask, 16 | case-sensitive | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f4037fb5de6f5d0d` |  |
| CaseSensitive | Mask, 24 | case-sensitive | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f4037fb5de6f5d0d` |  |
| CaseSensitive | Any, default | case-sensitive | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f4037fb5de6f5d0d` |  |
| ChartPie | Mask, 16 | chart-pie | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b1ddf72f33601cf5` |  |
| ChartPie | Mask, 24 | chart-pie | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b1ddf72f33601cf5` |  |
| ChartPie | Any, default | chart-pie | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b1ddf72f33601cf5` |  |
| Check | Mask, 16 | check | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `189a5e06b53b8e02` |  |
| Check | Mask, 24 | check | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `189a5e06b53b8e02` |  |
| Check | Any, default | check | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `189a5e06b53b8e02` |  |
| ChevronDown | Mask, 16 | chevron-down | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `39cf82981865e10b` |  |
| ChevronDown | Mask, 24 | chevron-down | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `39cf82981865e10b` |  |
| ChevronDown | Any, default | chevron-down | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `39cf82981865e10b` |  |
| ChevronLeft | Mask, 16 | chevron-left | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a2e0dc9448c0c309` |  |
| ChevronLeft | Mask, 24 | chevron-left | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a2e0dc9448c0c309` |  |
| ChevronLeft | Any, default | chevron-left | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a2e0dc9448c0c309` |  |
| ChevronRight | Mask, 16 | chevron-right | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c35b54572495dafc` |  |
| ChevronRight | Mask, 24 | chevron-right | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c35b54572495dafc` |  |
| ChevronRight | Any, default | chevron-right | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c35b54572495dafc` |  |
| ChevronsUpDown | Mask, 16 | chevrons-up-down | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `5647c2985ab17dba` |  |
| ChevronsUpDown | Mask, 24 | chevrons-up-down | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `5647c2985ab17dba` |  |
| ChevronsUpDown | Any, default | chevrons-up-down | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `5647c2985ab17dba` |  |
| ChevronUp | Mask, 16 | chevron-up | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7a21607dccdbc3eb` |  |
| ChevronUp | Mask, 24 | chevron-up | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7a21607dccdbc3eb` |  |
| ChevronUp | Any, default | chevron-up | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7a21607dccdbc3eb` |  |
| CircleAlert | Mask, 16 | circle-alert | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3991e64a2a9e56e3` |  |
| CircleAlert | Mask, 24 | circle-alert | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3991e64a2a9e56e3` |  |
| CircleAlert | Any, default | circle-alert | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3991e64a2a9e56e3` |  |
| CircleCheck | Mask, 16 | circle-check | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7034bdce8932a7db` |  |
| CircleCheck | Mask, 24 | circle-check | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7034bdce8932a7db` |  |
| CircleCheck | Any, default | circle-check | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7034bdce8932a7db` |  |
| CircleUser | Mask, 16 | circle-user | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `01b58c2b50baaa3c` |  |
| CircleUser | Mask, 24 | circle-user | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `01b58c2b50baaa3c` |  |
| CircleUser | Any, default | circle-user | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `01b58c2b50baaa3c` |  |
| CircleX | Mask, 16 | circle-x | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `510dad6703904b09` |  |
| CircleX | Mask, 24 | circle-x | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `510dad6703904b09` |  |
| CircleX | Any, default | circle-x | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `510dad6703904b09` |  |
| Close | Mask, 16 | x | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6d3da6417967f9bb` |  |
| Close | Mask, 24 | x | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6d3da6417967f9bb` |  |
| Close | Any, default | x | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6d3da6417967f9bb` |  |
| Copy | Mask, 16 | copy | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `abae062479c37985` |  |
| Copy | Mask, 24 | copy | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `abae062479c37985` |  |
| Copy | Any, default | copy | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `abae062479c37985` |  |
| Cpu | Mask, 16 | cpu | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1319ff14f05624fc` |  |
| Cpu | Mask, 24 | cpu | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1319ff14f05624fc` |  |
| Cpu | Any, default | cpu | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `1319ff14f05624fc` |  |
| Dash | Mask, 16 | minus | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| Dash | Mask, 24 | minus | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| Dash | Any, default | minus | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| Delete | Mask, 16 | delete | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f2d8989fb15968c9` |  |
| Delete | Mask, 24 | delete | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f2d8989fb15968c9` |  |
| Delete | Any, default | delete | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f2d8989fb15968c9` |  |
| Ellipsis | Mask, 16 | ellipsis | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `561923d19b3c59c4` |  |
| Ellipsis | Mask, 24 | ellipsis | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `561923d19b3c59c4` |  |
| Ellipsis | Any, default | ellipsis | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `561923d19b3c59c4` |  |
| EllipsisVertical | Mask, 16 | ellipsis-vertical | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ec8a19b106210e38` |  |
| EllipsisVertical | Mask, 24 | ellipsis-vertical | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ec8a19b106210e38` |  |
| EllipsisVertical | Any, default | ellipsis-vertical | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ec8a19b106210e38` |  |
| ExternalLink | Mask, 16 | external-link | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7916c04338ca007c` |  |
| ExternalLink | Mask, 24 | external-link | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7916c04338ca007c` |  |
| ExternalLink | Any, default | external-link | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7916c04338ca007c` |  |
| Eye | Mask, 16 | eye | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `63d326116668bb58` |  |
| Eye | Mask, 24 | eye | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `63d326116668bb58` |  |
| Eye | Any, default | eye | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `63d326116668bb58` |  |
| EyeOff | Mask, 16 | eye-off | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d17e527491eb7a25` |  |
| EyeOff | Mask, 24 | eye-off | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d17e527491eb7a25` |  |
| EyeOff | Any, default | eye-off | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d17e527491eb7a25` |  |
| File | Mask, 16 | file | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `94549399fa788307` |  |
| File | Mask, 24 | file | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `94549399fa788307` |  |
| File | Any, default | file | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `94549399fa788307` |  |
| FileText | Mask, 16 | file-text | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `0f7214fde52d62eb` |  |
| FileText | Mask, 24 | file-text | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `0f7214fde52d62eb` |  |
| FileText | Any, default | file-text | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `0f7214fde52d62eb` |  |
| Folder | Mask, 16 | folder | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d5648a3badb2b5eb` |  |
| Folder | Mask, 24 | folder | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d5648a3badb2b5eb` |  |
| Folder | Any, default | folder | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d5648a3badb2b5eb` |  |
| FolderClosed | Mask, 16 | folder-closed | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7bf71fad5e256724` |  |
| FolderClosed | Mask, 24 | folder-closed | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7bf71fad5e256724` |  |
| FolderClosed | Any, default | folder-closed | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7bf71fad5e256724` |  |
| FolderOpen | Mask, 16 | folder-open | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03e9625fa7097b6c` |  |
| FolderOpen | Mask, 24 | folder-open | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03e9625fa7097b6c` |  |
| FolderOpen | Any, default | folder-open | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03e9625fa7097b6c` |  |
| Frame | Mask, 16 | frame | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7290be2cfdd6b7c1` |  |
| Frame | Mask, 24 | frame | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7290be2cfdd6b7c1` |  |
| Frame | Any, default | frame | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7290be2cfdd6b7c1` |  |
| GalleryVerticalEnd | Mask, 16 | gallery-vertical-end | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `34c915b0516a5bc5` |  |
| GalleryVerticalEnd | Mask, 24 | gallery-vertical-end | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `34c915b0516a5bc5` |  |
| GalleryVerticalEnd | Any, default | gallery-vertical-end | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `34c915b0516a5bc5` |  |
| Github | Mask, 16 | github | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `37d78a48a14fca55` |  |
| Github | Mask, 24 | github | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `37d78a48a14fca55` |  |
| Github | Any, default | github | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `37d78a48a14fca55` |  |
| Globe | Mask, 16 | globe | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `fcd31032a541c243` |  |
| Globe | Mask, 24 | globe | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `fcd31032a541c243` |  |
| Globe | Any, default | globe | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `fcd31032a541c243` |  |
| HardDrive | Mask, 16 | hard-drive | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4c69509e41a65b5a` |  |
| HardDrive | Mask, 24 | hard-drive | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4c69509e41a65b5a` |  |
| HardDrive | Any, default | hard-drive | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4c69509e41a65b5a` |  |
| Heart | Mask, 16 | heart | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f4afd0a17d477808` |  |
| Heart | Mask, 24 | heart | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f4afd0a17d477808` |  |
| Heart | Any, default | heart | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f4afd0a17d477808` |  |
| HeartOff | Mask, 16 | heart-off | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03229642f8e29e6d` |  |
| HeartOff | Mask, 24 | heart-off | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03229642f8e29e6d` |  |
| HeartOff | Any, default | heart-off | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03229642f8e29e6d` |  |
| Inbox | Mask, 16 | inbox | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `93c10cf4909f7541` |  |
| Inbox | Mask, 24 | inbox | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `93c10cf4909f7541` |  |
| Inbox | Any, default | inbox | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `93c10cf4909f7541` |  |
| Info | Mask, 16 | info | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2ced3f3c5f9c8206` |  |
| Info | Mask, 24 | info | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2ced3f3c5f9c8206` |  |
| Info | Any, default | info | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2ced3f3c5f9c8206` |  |
| Inspector | Mask, 16 | scan | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `07a3dd6fc6b778a7` |  |
| Inspector | Mask, 24 | scan | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `07a3dd6fc6b778a7` |  |
| Inspector | Any, default | scan | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `07a3dd6fc6b778a7` |  |
| LayoutDashboard | Mask, 16 | layout-dashboard | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e4d6f45fbecca8cf` |  |
| LayoutDashboard | Mask, 24 | layout-dashboard | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e4d6f45fbecca8cf` |  |
| LayoutDashboard | Any, default | layout-dashboard | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e4d6f45fbecca8cf` |  |
| Loader | Mask, 16 | loader | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `158304850aee5105` |  |
| Loader | Mask, 24 | loader | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `158304850aee5105` |  |
| Loader | Any, default | loader | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `158304850aee5105` |  |
| LoaderCircle | Mask, 16 | loader-circle | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `849d15d65dcc9b27` |  |
| LoaderCircle | Mask, 24 | loader-circle | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `849d15d65dcc9b27` |  |
| LoaderCircle | Any, default | loader-circle | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `849d15d65dcc9b27` |  |
| Map | Mask, 16 | map | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `0828e8b2ab6e9985` |  |
| Map | Mask, 24 | map | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `0828e8b2ab6e9985` |  |
| Map | Any, default | map | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `0828e8b2ab6e9985` |  |
| Maximize | Mask, 16 | maximize | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3473c2280c2ccdcd` |  |
| Maximize | Mask, 24 | maximize | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3473c2280c2ccdcd` |  |
| Maximize | Any, default | maximize | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3473c2280c2ccdcd` |  |
| MemoryStick | Mask, 16 | memory-stick | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `41095ddd41b7ef5f` |  |
| MemoryStick | Mask, 24 | memory-stick | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `41095ddd41b7ef5f` |  |
| MemoryStick | Any, default | memory-stick | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `41095ddd41b7ef5f` |  |
| Menu | Mask, 16 | menu | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c597152d71e66658` |  |
| Menu | Mask, 24 | menu | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c597152d71e66658` |  |
| Menu | Any, default | menu | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c597152d71e66658` |  |
| Mic | Mask, 16 | mic | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `872289948c9e48c9` |  |
| Mic | Mask, 24 | mic | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `872289948c9e48c9` |  |
| Mic | Any, default | mic | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `872289948c9e48c9` |  |
| Minimize | Mask, 16 | minimize | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9baf0908ffb93669` |  |
| Minimize | Mask, 24 | minimize | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9baf0908ffb93669` |  |
| Minimize | Any, default | minimize | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9baf0908ffb93669` |  |
| Minus | Mask, 16 | minus | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| Minus | Mask, 24 | minus | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| Minus | Any, default | minus | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| Moon | Mask, 16 | moon | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c2487e7903f9895e` |  |
| Moon | Mask, 24 | moon | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c2487e7903f9895e` |  |
| Moon | Any, default | moon | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `c2487e7903f9895e` |  |
| Network | Mask, 16 | network | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9dcb18c10ef25175` |  |
| Network | Mask, 24 | network | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9dcb18c10ef25175` |  |
| Network | Any, default | network | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9dcb18c10ef25175` |  |
| Palette | Mask, 16 | palette | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `8a8bec3d4e679fb9` |  |
| Palette | Mask, 24 | palette | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `8a8bec3d4e679fb9` |  |
| Palette | Any, default | palette | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `8a8bec3d4e679fb9` |  |
| PanelBottom | Mask, 16 | panel-bottom | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `85af3f6d8720ae4d` |  |
| PanelBottom | Mask, 24 | panel-bottom | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `85af3f6d8720ae4d` |  |
| PanelBottom | Any, default | panel-bottom | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `85af3f6d8720ae4d` |  |
| PanelBottomOpen | Mask, 16 | panel-bottom-open | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9f10fea0ae82b2a6` |  |
| PanelBottomOpen | Mask, 24 | panel-bottom-open | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9f10fea0ae82b2a6` |  |
| PanelBottomOpen | Any, default | panel-bottom-open | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9f10fea0ae82b2a6` |  |
| PanelLeft | Mask, 16 | panel-left | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `43d93aaa0b7adb64` |  |
| PanelLeft | Mask, 24 | panel-left | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `43d93aaa0b7adb64` |  |
| PanelLeft | Any, default | panel-left | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `43d93aaa0b7adb64` |  |
| PanelLeftClose | Mask, 16 | panel-left-close | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d7888e1ca567930a` |  |
| PanelLeftClose | Mask, 24 | panel-left-close | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d7888e1ca567930a` |  |
| PanelLeftClose | Any, default | panel-left-close | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `d7888e1ca567930a` |  |
| PanelLeftOpen | Mask, 16 | panel-left-open | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `84053a974e5fc17b` |  |
| PanelLeftOpen | Mask, 24 | panel-left-open | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `84053a974e5fc17b` |  |
| PanelLeftOpen | Any, default | panel-left-open | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `84053a974e5fc17b` |  |
| PanelRight | Mask, 16 | panel-right | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7d5c9f7347e900c5` |  |
| PanelRight | Mask, 24 | panel-right | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7d5c9f7347e900c5` |  |
| PanelRight | Any, default | panel-right | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `7d5c9f7347e900c5` |  |
| PanelRightClose | Mask, 16 | panel-right-close | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `af9409fea82139eb` |  |
| PanelRightClose | Mask, 24 | panel-right-close | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `af9409fea82139eb` |  |
| PanelRightClose | Any, default | panel-right-close | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `af9409fea82139eb` |  |
| PanelRightOpen | Mask, 16 | panel-right-open | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9923edee84f6b9fd` |  |
| PanelRightOpen | Mask, 24 | panel-right-open | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9923edee84f6b9fd` |  |
| PanelRightOpen | Any, default | panel-right-open | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `9923edee84f6b9fd` |  |
| Pause | Mask, 16 | pause | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4dabefcb5d95296f` |  |
| Pause | Mask, 24 | pause | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4dabefcb5d95296f` |  |
| Pause | Any, default | pause | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `4dabefcb5d95296f` |  |
| Play | Mask, 16 | play | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ec2ec355b8f9ca25` |  |
| Play | Mask, 24 | play | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ec2ec355b8f9ca25` |  |
| Play | Any, default | play | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ec2ec355b8f9ca25` |  |
| Plus | Mask, 16 | plus | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03e9473a72237fdb` |  |
| Plus | Mask, 24 | plus | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03e9473a72237fdb` |  |
| Plus | Any, default | plus | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `03e9473a72237fdb` |  |
| Redo | Mask, 16 | redo | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `fcaf73aaf7eb52f0` |  |
| Redo | Mask, 24 | redo | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `fcaf73aaf7eb52f0` |  |
| Redo | Any, default | redo | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `fcaf73aaf7eb52f0` |  |
| Redo2 | Mask, 16 | redo-2 | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `784e7acc577aa056` |  |
| Redo2 | Mask, 24 | redo-2 | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `784e7acc577aa056` |  |
| Redo2 | Any, default | redo-2 | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `784e7acc577aa056` |  |
| RefreshCw | Mask, 16 | refresh-cw | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `295f60bde793c482` |  |
| RefreshCw | Mask, 24 | refresh-cw | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `295f60bde793c482` |  |
| RefreshCw | Any, default | refresh-cw | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `295f60bde793c482` |  |
| Replace | Mask, 16 | replace | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `286b17e2efbb7d84` |  |
| Replace | Mask, 24 | replace | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `286b17e2efbb7d84` |  |
| Replace | Any, default | replace | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `286b17e2efbb7d84` |  |
| ResizeCorner | Mask, 16 | grip | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `20bf11701fc08de7` |  |
| ResizeCorner | Mask, 24 | grip | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `20bf11701fc08de7` |  |
| ResizeCorner | Any, default | grip | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `20bf11701fc08de7` |  |
| RotateCw | Mask, 16 | rotate-cw | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b9cb7a3b2903bedd` |  |
| RotateCw | Mask, 24 | rotate-cw | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b9cb7a3b2903bedd` |  |
| RotateCw | Any, default | rotate-cw | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `b9cb7a3b2903bedd` |  |
| Search | Mask, 16 | search | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `af6743dd0fa70dcc` |  |
| Search | Mask, 24 | search | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `af6743dd0fa70dcc` |  |
| Search | Any, default | search | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `af6743dd0fa70dcc` |  |
| Settings | Mask, 16 | settings | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a7b286ab5e7c2b66` |  |
| Settings | Mask, 24 | settings | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a7b286ab5e7c2b66` |  |
| Settings | Any, default | settings | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a7b286ab5e7c2b66` |  |
| Settings2 | Mask, 16 | settings-2 | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6c93122f4b73eef4` |  |
| Settings2 | Mask, 24 | settings-2 | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6c93122f4b73eef4` |  |
| Settings2 | Any, default | settings-2 | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6c93122f4b73eef4` |  |
| SortAscending | Mask, 16 | arrow-up-narrow-wide | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `abbec29abe838bef` |  |
| SortAscending | Mask, 24 | arrow-up-narrow-wide | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `abbec29abe838bef` |  |
| SortAscending | Any, default | arrow-up-narrow-wide | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `abbec29abe838bef` |  |
| SortDescending | Mask, 16 | arrow-down-wide-narrow | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `952ccf91eb6ffbae` |  |
| SortDescending | Mask, 24 | arrow-down-wide-narrow | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `952ccf91eb6ffbae` |  |
| SortDescending | Any, default | arrow-down-wide-narrow | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `952ccf91eb6ffbae` |  |
| Square | Mask, 16 | square | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6bf893f97f7f2fd3` |  |
| Square | Mask, 24 | square | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6bf893f97f7f2fd3` |  |
| Square | Any, default | square | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6bf893f97f7f2fd3` |  |
| SquareTerminal | Mask, 16 | square-terminal | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a3ee8940abdadddf` |  |
| SquareTerminal | Mask, 24 | square-terminal | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a3ee8940abdadddf` |  |
| SquareTerminal | Any, default | square-terminal | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `a3ee8940abdadddf` |  |
| Star | Mask, 16 | star | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f821e12ea6198d02` |  |
| Star | Mask, 24 | star | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f821e12ea6198d02` |  |
| Star | Any, default | star | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `f821e12ea6198d02` |  |
| StarFill | Mask, 16 | none | lucide | 16 | | | |
| StarFill | Mask, 24 | none | lucide | 24 | | | |
| StarFill | Any, default | none | lucide | default | | | |
| StarOff | Mask, 16 | star-off | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `93c3f436ec604969` |  |
| StarOff | Mask, 24 | star-off | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `93c3f436ec604969` |  |
| StarOff | Any, default | star-off | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `93c3f436ec604969` |  |
| Sun | Mask, 16 | sun | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `63bfeaf421a61c58` |  |
| Sun | Mask, 24 | sun | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `63bfeaf421a61c58` |  |
| Sun | Any, default | sun | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `63bfeaf421a61c58` |  |
| ThumbsDown | Mask, 16 | thumbs-down | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e4a99a5f9c5d8e32` |  |
| ThumbsDown | Mask, 24 | thumbs-down | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e4a99a5f9c5d8e32` |  |
| ThumbsDown | Any, default | thumbs-down | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e4a99a5f9c5d8e32` |  |
| ThumbsUp | Mask, 16 | thumbs-up | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e9ced792fd279248` |  |
| ThumbsUp | Mask, 24 | thumbs-up | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e9ced792fd279248` |  |
| ThumbsUp | Any, default | thumbs-up | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `e9ced792fd279248` |  |
| TriangleAlert | Mask, 16 | triangle-alert | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `914f076fc5a3e2d8` |  |
| TriangleAlert | Mask, 24 | triangle-alert | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `914f076fc5a3e2d8` |  |
| TriangleAlert | Any, default | triangle-alert | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `914f076fc5a3e2d8` |  |
| Undo | Mask, 16 | undo | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ac919e6515080d18` |  |
| Undo | Mask, 24 | undo | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ac919e6515080d18` |  |
| Undo | Any, default | undo | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `ac919e6515080d18` |  |
| Undo2 | Mask, 16 | undo-2 | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `55139a7b08f4af81` |  |
| Undo2 | Mask, 24 | undo-2 | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `55139a7b08f4af81` |  |
| Undo2 | Any, default | undo-2 | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `55139a7b08f4af81` |  |
| User | Mask, 16 | user | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `34416b1d9eca5f48` |  |
| User | Mask, 24 | user | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `34416b1d9eca5f48` |  |
| User | Any, default | user | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `34416b1d9eca5f48` |  |
| WindowClose | Mask, 16 | x | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6d3da6417967f9bb` |  |
| WindowClose | Mask, 24 | x | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6d3da6417967f9bb` |  |
| WindowClose | Any, default | x | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `6d3da6417967f9bb` |  |
| WindowMaximize | Mask, 16 | maximize | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3473c2280c2ccdcd` |  |
| WindowMaximize | Mask, 24 | maximize | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3473c2280c2ccdcd` |  |
| WindowMaximize | Any, default | maximize | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `3473c2280c2ccdcd` |  |
| WindowMinimize | Mask, 16 | minus | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| WindowMinimize | Mask, 24 | minus | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| WindowMinimize | Any, default | minus | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `2c2773be630caae8` |  |
| WindowRestore | Mask, 16 | minimize-2 | lucide | 16; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `bae5cfb9e4154976` |  |
| WindowRestore | Mask, 24 | minimize-2 | lucide | 24; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `bae5cfb9e4154976` |  |
| WindowRestore | Any, default | minimize-2 | lucide | default; viewBox `0 0 24 24` | yes (deviation 0 / 3812) | `bae5cfb9e4154976` |  |

### `material`

| IconName | Lookup | Name found | Theme searched | Size | One colour | Hash | Same hash in |
|---|---|---|---|---|---|---|---|
| ALargeSmall | Mask, 16 | format_size | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a88174ee1b1257bf` |  |
| ALargeSmall | Mask, 24 | format_size | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a88174ee1b1257bf` |  |
| ALargeSmall | Any, default | format_size | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a88174ee1b1257bf` |  |
| ArrowDown | Mask, 16 | arrow_downward | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3e26cd6f096d9c81` |  |
| ArrowDown | Mask, 24 | arrow_downward | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3e26cd6f096d9c81` |  |
| ArrowDown | Any, default | arrow_downward | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3e26cd6f096d9c81` |  |
| ArrowLeft | Mask, 16 | arrow_back | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `88f8957553810159` |  |
| ArrowLeft | Mask, 24 | arrow_back | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `88f8957553810159` |  |
| ArrowLeft | Any, default | arrow_back | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `88f8957553810159` |  |
| ArrowRight | Mask, 16 | arrow_forward | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `936f83921ff2f627` |  |
| ArrowRight | Mask, 24 | arrow_forward | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `936f83921ff2f627` |  |
| ArrowRight | Any, default | arrow_forward | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `936f83921ff2f627` |  |
| ArrowUp | Mask, 16 | arrow_upward | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `408dec431c9dcaeb` |  |
| ArrowUp | Mask, 24 | arrow_upward | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `408dec431c9dcaeb` |  |
| ArrowUp | Any, default | arrow_upward | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `408dec431c9dcaeb` |  |
| Asterisk | Mask, 16 | emergency | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `469c7396dc49c644` |  |
| Asterisk | Mask, 24 | emergency | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `469c7396dc49c644` |  |
| Asterisk | Any, default | emergency | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `469c7396dc49c644` |  |
| Ban | Mask, 16 | block | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `ccde46a141acc8fd` |  |
| Ban | Mask, 24 | block | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `ccde46a141acc8fd` |  |
| Ban | Any, default | block | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `ccde46a141acc8fd` |  |
| Battery | Mask, 16 | battery_0_bar | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cfdfa433e593fb21` |  |
| Battery | Mask, 24 | battery_0_bar | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cfdfa433e593fb21` |  |
| Battery | Any, default | battery_0_bar | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cfdfa433e593fb21` |  |
| BatteryCharging | Mask, 16 | battery_charging_full | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fdb8634381734f9d` |  |
| BatteryCharging | Mask, 24 | battery_charging_full | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fdb8634381734f9d` |  |
| BatteryCharging | Any, default | battery_charging_full | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fdb8634381734f9d` |  |
| BatteryFull | Mask, 16 | battery_full | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `42a737acebf9e0d1` |  |
| BatteryFull | Mask, 24 | battery_full | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `42a737acebf9e0d1` |  |
| BatteryFull | Any, default | battery_full | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `42a737acebf9e0d1` |  |
| BatteryLow | Mask, 16 | battery_2_bar | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c5493292105763a5` |  |
| BatteryLow | Mask, 24 | battery_2_bar | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c5493292105763a5` |  |
| BatteryLow | Any, default | battery_2_bar | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c5493292105763a5` |  |
| BatteryMedium | Mask, 16 | battery_4_bar | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `5f0ef7b392fa4c4f` |  |
| BatteryMedium | Mask, 24 | battery_4_bar | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `5f0ef7b392fa4c4f` |  |
| BatteryMedium | Any, default | battery_4_bar | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `5f0ef7b392fa4c4f` |  |
| BatteryWarning | Mask, 16 | battery_alert | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `30adf28695235d6f` |  |
| BatteryWarning | Mask, 24 | battery_alert | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `30adf28695235d6f` |  |
| BatteryWarning | Any, default | battery_alert | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `30adf28695235d6f` |  |
| Bell | Mask, 16 | notifications | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6401581aaa606a2b` |  |
| Bell | Mask, 24 | notifications | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6401581aaa606a2b` |  |
| Bell | Any, default | notifications | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6401581aaa606a2b` |  |
| BookOpen | Mask, 16 | menu_book | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6c7b4d09dae78fab` |  |
| BookOpen | Mask, 24 | menu_book | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6c7b4d09dae78fab` |  |
| BookOpen | Any, default | menu_book | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6c7b4d09dae78fab` |  |
| Bot | Mask, 16 | smart_toy | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `d3db1d9a386c9230` |  |
| Bot | Mask, 24 | smart_toy | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `d3db1d9a386c9230` |  |
| Bot | Any, default | smart_toy | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `d3db1d9a386c9230` |  |
| Building2 | Mask, 16 | apartment | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2cb3a135c16fc0c1` |  |
| Building2 | Mask, 24 | apartment | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2cb3a135c16fc0c1` |  |
| Building2 | Any, default | apartment | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2cb3a135c16fc0c1` |  |
| Calendar | Mask, 16 | calendar_today | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8038d286df2967ef` |  |
| Calendar | Mask, 24 | calendar_today | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8038d286df2967ef` |  |
| Calendar | Any, default | calendar_today | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8038d286df2967ef` |  |
| CaseSensitive | Mask, 16 | match_case | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3abb57935b8ba31c` |  |
| CaseSensitive | Mask, 24 | match_case | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3abb57935b8ba31c` |  |
| CaseSensitive | Any, default | match_case | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3abb57935b8ba31c` |  |
| ChartPie | Mask, 16 | pie_chart | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `91b155986303b21a` |  |
| ChartPie | Mask, 24 | pie_chart | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `91b155986303b21a` |  |
| ChartPie | Any, default | pie_chart | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `91b155986303b21a` |  |
| Check | Mask, 16 | check | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2ae18cefec327c70` |  |
| Check | Mask, 24 | check | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2ae18cefec327c70` |  |
| Check | Any, default | check | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2ae18cefec327c70` |  |
| ChevronDown | Mask, 16 | expand_more | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `004e2471cc072695` |  |
| ChevronDown | Mask, 24 | expand_more | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `004e2471cc072695` |  |
| ChevronDown | Any, default | expand_more | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `004e2471cc072695` |  |
| ChevronLeft | Mask, 16 | chevron_left | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af6d603b256d2efb` |  |
| ChevronLeft | Mask, 24 | chevron_left | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af6d603b256d2efb` |  |
| ChevronLeft | Any, default | chevron_left | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af6d603b256d2efb` |  |
| ChevronRight | Mask, 16 | chevron_right | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4bf17df083399c13` |  |
| ChevronRight | Mask, 24 | chevron_right | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4bf17df083399c13` |  |
| ChevronRight | Any, default | chevron_right | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4bf17df083399c13` |  |
| ChevronsUpDown | Mask, 16 | unfold_more | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `9d54ea4e6feb3947` |  |
| ChevronsUpDown | Mask, 24 | unfold_more | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `9d54ea4e6feb3947` |  |
| ChevronsUpDown | Any, default | unfold_more | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `9d54ea4e6feb3947` |  |
| ChevronUp | Mask, 16 | expand_less | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `41eb4068d5b54313` |  |
| ChevronUp | Mask, 24 | expand_less | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `41eb4068d5b54313` |  |
| ChevronUp | Any, default | expand_less | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `41eb4068d5b54313` |  |
| CircleAlert | Mask, 16 | error | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4b21c19d04fa396c` |  |
| CircleAlert | Mask, 24 | error | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4b21c19d04fa396c` |  |
| CircleAlert | Any, default | error | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4b21c19d04fa396c` |  |
| CircleCheck | Mask, 16 | check_circle | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `9256b188ee564053` |  |
| CircleCheck | Mask, 24 | check_circle | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `9256b188ee564053` |  |
| CircleCheck | Any, default | check_circle | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `9256b188ee564053` |  |
| CircleUser | Mask, 16 | account_circle | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `590be00a07bb7530` |  |
| CircleUser | Mask, 24 | account_circle | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `590be00a07bb7530` |  |
| CircleUser | Any, default | account_circle | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `590be00a07bb7530` |  |
| CircleX | Mask, 16 | cancel | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cdc93dd3852ea67e` |  |
| CircleX | Mask, 24 | cancel | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cdc93dd3852ea67e` |  |
| CircleX | Any, default | cancel | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cdc93dd3852ea67e` |  |
| Close | Mask, 16 | close | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fc6127371af54034` |  |
| Close | Mask, 24 | close | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fc6127371af54034` |  |
| Close | Any, default | close | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fc6127371af54034` |  |
| Copy | Mask, 16 | content_copy | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8154654f7a78a865` |  |
| Copy | Mask, 24 | content_copy | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8154654f7a78a865` |  |
| Copy | Any, default | content_copy | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8154654f7a78a865` |  |
| Cpu | Mask, 16 | memory | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4933e4c408ca415b` |  |
| Cpu | Mask, 24 | memory | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4933e4c408ca415b` |  |
| Cpu | Any, default | memory | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4933e4c408ca415b` |  |
| Dash | Mask, 16 | remove | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3d0cb70e5048b387` |  |
| Dash | Mask, 24 | remove | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3d0cb70e5048b387` |  |
| Dash | Any, default | remove | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3d0cb70e5048b387` |  |
| Delete | Mask, 16 | delete | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `140a5c625f7044f4` |  |
| Delete | Mask, 24 | delete | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `140a5c625f7044f4` |  |
| Delete | Any, default | delete | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `140a5c625f7044f4` |  |
| Ellipsis | Mask, 16 | more_horiz | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a6a26c69d86e9de5` |  |
| Ellipsis | Mask, 24 | more_horiz | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a6a26c69d86e9de5` |  |
| Ellipsis | Any, default | more_horiz | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a6a26c69d86e9de5` |  |
| EllipsisVertical | Mask, 16 | more_vert | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2e2797211ff335c0` |  |
| EllipsisVertical | Mask, 24 | more_vert | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2e2797211ff335c0` |  |
| EllipsisVertical | Any, default | more_vert | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2e2797211ff335c0` |  |
| ExternalLink | Mask, 16 | open_in_new | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c7293f6f824bc3fc` |  |
| ExternalLink | Mask, 24 | open_in_new | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c7293f6f824bc3fc` |  |
| ExternalLink | Any, default | open_in_new | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c7293f6f824bc3fc` |  |
| Eye | Mask, 16 | visibility | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cf91a3806f7b503b` |  |
| Eye | Mask, 24 | visibility | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cf91a3806f7b503b` |  |
| Eye | Any, default | visibility | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `cf91a3806f7b503b` |  |
| EyeOff | Mask, 16 | visibility_off | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af9331109f8e3336` |  |
| EyeOff | Mask, 24 | visibility_off | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af9331109f8e3336` |  |
| EyeOff | Any, default | visibility_off | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af9331109f8e3336` |  |
| File | Mask, 16 | description | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af8a80fc7e5696a4` |  |
| File | Mask, 24 | description | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af8a80fc7e5696a4` |  |
| File | Any, default | description | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af8a80fc7e5696a4` |  |
| FileText | Mask, 16 | description | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af8a80fc7e5696a4` |  |
| FileText | Mask, 24 | description | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af8a80fc7e5696a4` |  |
| FileText | Any, default | description | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `af8a80fc7e5696a4` |  |
| Folder | Mask, 16 | folder | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `81e869b947ac3949` |  |
| Folder | Mask, 24 | folder | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `81e869b947ac3949` |  |
| Folder | Any, default | folder | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `81e869b947ac3949` |  |
| FolderClosed | Mask, 16 | folder | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `81e869b947ac3949` |  |
| FolderClosed | Mask, 24 | folder | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `81e869b947ac3949` |  |
| FolderClosed | Any, default | folder | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `81e869b947ac3949` |  |
| FolderOpen | Mask, 16 | folder_open | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `35e5511261786ae3` |  |
| FolderOpen | Mask, 24 | folder_open | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `35e5511261786ae3` |  |
| FolderOpen | Any, default | folder_open | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `35e5511261786ae3` |  |
| Frame | Mask, 16 | crop_free | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `7ceb9cc0ec1e81e3` |  |
| Frame | Mask, 24 | crop_free | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `7ceb9cc0ec1e81e3` |  |
| Frame | Any, default | crop_free | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `7ceb9cc0ec1e81e3` |  |
| GalleryVerticalEnd | Mask, 16 | view_carousel | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `01c47451487c51cb` |  |
| GalleryVerticalEnd | Mask, 24 | view_carousel | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `01c47451487c51cb` |  |
| GalleryVerticalEnd | Any, default | view_carousel | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `01c47451487c51cb` |  |
| Github | Mask, 16 | code | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `7ade63fea3fe5317` |  |
| Github | Mask, 24 | code | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `7ade63fea3fe5317` |  |
| Github | Any, default | code | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `7ade63fea3fe5317` |  |
| Globe | Mask, 16 | language | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a2ca72c3dd1ed447` |  |
| Globe | Mask, 24 | language | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a2ca72c3dd1ed447` |  |
| Globe | Any, default | language | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a2ca72c3dd1ed447` |  |
| HardDrive | Mask, 16 | hard_drive | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `509e260e458add09` |  |
| HardDrive | Mask, 24 | hard_drive | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `509e260e458add09` |  |
| HardDrive | Any, default | hard_drive | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `509e260e458add09` |  |
| Heart | Mask, 16 | favorite | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `91f53ef6991b4b92` |  |
| Heart | Mask, 24 | favorite | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `91f53ef6991b4b92` |  |
| Heart | Any, default | favorite | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `91f53ef6991b4b92` |  |
| HeartOff | Mask, 16 | heart_broken | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fffecea431fc6e07` |  |
| HeartOff | Mask, 24 | heart_broken | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fffecea431fc6e07` |  |
| HeartOff | Any, default | heart_broken | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fffecea431fc6e07` |  |
| Inbox | Mask, 16 | inbox | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `939cf8863c709795` |  |
| Inbox | Mask, 24 | inbox | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `939cf8863c709795` |  |
| Inbox | Any, default | inbox | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `939cf8863c709795` |  |
| Info | Mask, 16 | info | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `be61f462a0112248` |  |
| Info | Mask, 24 | info | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `be61f462a0112248` |  |
| Info | Any, default | info | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `be61f462a0112248` |  |
| Inspector | Mask, 16 | developer_mode | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4ca14d2211827d1f` |  |
| Inspector | Mask, 24 | developer_mode | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4ca14d2211827d1f` |  |
| Inspector | Any, default | developer_mode | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4ca14d2211827d1f` |  |
| LayoutDashboard | Mask, 16 | dashboard | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `676f49beb654099a` |  |
| LayoutDashboard | Mask, 24 | dashboard | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `676f49beb654099a` |  |
| LayoutDashboard | Any, default | dashboard | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `676f49beb654099a` |  |
| Loader | Mask, 16 | progress_activity | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2b2247b3ae7bb7e6` |  |
| Loader | Mask, 24 | progress_activity | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2b2247b3ae7bb7e6` |  |
| Loader | Any, default | progress_activity | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2b2247b3ae7bb7e6` |  |
| LoaderCircle | Mask, 16 | autorenew | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bacd2e25da57f060` |  |
| LoaderCircle | Mask, 24 | autorenew | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bacd2e25da57f060` |  |
| LoaderCircle | Any, default | autorenew | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bacd2e25da57f060` |  |
| Map | Mask, 16 | map | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `534e79ece7fd9d17` |  |
| Map | Mask, 24 | map | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `534e79ece7fd9d17` |  |
| Map | Any, default | map | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `534e79ece7fd9d17` |  |
| Maximize | Mask, 16 | open_in_full | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `43ed96c0095a9d02` |  |
| Maximize | Mask, 24 | open_in_full | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `43ed96c0095a9d02` |  |
| Maximize | Any, default | open_in_full | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `43ed96c0095a9d02` |  |
| MemoryStick | Mask, 16 | memory_alt | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c80e1d82929b5b63` |  |
| MemoryStick | Mask, 24 | memory_alt | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c80e1d82929b5b63` |  |
| MemoryStick | Any, default | memory_alt | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c80e1d82929b5b63` |  |
| Menu | Mask, 16 | menu | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `47ed956a79668929` |  |
| Menu | Mask, 24 | menu | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `47ed956a79668929` |  |
| Menu | Any, default | menu | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `47ed956a79668929` |  |
| Mic | Mask, 16 | mic | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fa50126bed1cf303` |  |
| Mic | Mask, 24 | mic | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fa50126bed1cf303` |  |
| Mic | Any, default | mic | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fa50126bed1cf303` |  |
| Minimize | Mask, 16 | minimize | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `76a0ab43a39996b3` |  |
| Minimize | Mask, 24 | minimize | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `76a0ab43a39996b3` |  |
| Minimize | Any, default | minimize | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `76a0ab43a39996b3` |  |
| Minus | Mask, 16 | remove | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3d0cb70e5048b387` |  |
| Minus | Mask, 24 | remove | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3d0cb70e5048b387` |  |
| Minus | Any, default | remove | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3d0cb70e5048b387` |  |
| Moon | Mask, 16 | dark_mode | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `000c666e15295ae7` |  |
| Moon | Mask, 24 | dark_mode | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `000c666e15295ae7` |  |
| Moon | Any, default | dark_mode | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `000c666e15295ae7` |  |
| Network | Mask, 16 | lan | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `e2fac9d8c49dceec` |  |
| Network | Mask, 24 | lan | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `e2fac9d8c49dceec` |  |
| Network | Any, default | lan | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `e2fac9d8c49dceec` |  |
| Palette | Mask, 16 | palette | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bc33243e31dc6afa` |  |
| Palette | Mask, 24 | palette | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bc33243e31dc6afa` |  |
| Palette | Any, default | palette | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bc33243e31dc6afa` |  |
| PanelBottom | Mask, 16 | dock_to_bottom | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `36da1faaa6af806e` |  |
| PanelBottom | Mask, 24 | dock_to_bottom | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `36da1faaa6af806e` |  |
| PanelBottom | Any, default | dock_to_bottom | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `36da1faaa6af806e` |  |
| PanelBottomOpen | Mask, 16 | web_asset | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3fe88e7b33da343c` |  |
| PanelBottomOpen | Mask, 24 | web_asset | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3fe88e7b33da343c` |  |
| PanelBottomOpen | Any, default | web_asset | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3fe88e7b33da343c` |  |
| PanelLeft | Mask, 16 | side_navigation | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8b7b59939f100b6f` |  |
| PanelLeft | Mask, 24 | side_navigation | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8b7b59939f100b6f` |  |
| PanelLeft | Any, default | side_navigation | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8b7b59939f100b6f` |  |
| PanelLeftClose | Mask, 16 | left_panel_close | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6326609c0b07448e` |  |
| PanelLeftClose | Mask, 24 | left_panel_close | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6326609c0b07448e` |  |
| PanelLeftClose | Any, default | left_panel_close | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `6326609c0b07448e` |  |
| PanelLeftOpen | Mask, 16 | left_panel_open | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `f81e9ae9ecbbcb4e` |  |
| PanelLeftOpen | Mask, 24 | left_panel_open | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `f81e9ae9ecbbcb4e` |  |
| PanelLeftOpen | Any, default | left_panel_open | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `f81e9ae9ecbbcb4e` |  |
| PanelRight | Mask, 16 | right_panel_close | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ba29e503ef18fd` |  |
| PanelRight | Mask, 24 | right_panel_close | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ba29e503ef18fd` |  |
| PanelRight | Any, default | right_panel_close | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ba29e503ef18fd` |  |
| PanelRightClose | Mask, 16 | right_panel_close | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ba29e503ef18fd` |  |
| PanelRightClose | Mask, 24 | right_panel_close | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ba29e503ef18fd` |  |
| PanelRightClose | Any, default | right_panel_close | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ba29e503ef18fd` |  |
| PanelRightOpen | Mask, 16 | right_panel_open | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a4bb5ef109d2688f` |  |
| PanelRightOpen | Mask, 24 | right_panel_open | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a4bb5ef109d2688f` |  |
| PanelRightOpen | Any, default | right_panel_open | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `a4bb5ef109d2688f` |  |
| Pause | Mask, 16 | pause | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `5bdf5262f54c6e2f` |  |
| Pause | Mask, 24 | pause | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `5bdf5262f54c6e2f` |  |
| Pause | Any, default | pause | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `5bdf5262f54c6e2f` |  |
| Play | Mask, 16 | play_arrow | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8ea75ee19d2bf68d` |  |
| Play | Mask, 24 | play_arrow | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8ea75ee19d2bf68d` |  |
| Play | Any, default | play_arrow | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8ea75ee19d2bf68d` |  |
| Plus | Mask, 16 | add | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3b9ece034dbe3262` |  |
| Plus | Mask, 24 | add | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3b9ece034dbe3262` |  |
| Plus | Any, default | add | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3b9ece034dbe3262` |  |
| Redo | Mask, 16 | redo | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3c1ec4357d6648c7` |  |
| Redo | Mask, 24 | redo | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3c1ec4357d6648c7` |  |
| Redo | Any, default | redo | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3c1ec4357d6648c7` |  |
| Redo2 | Mask, 16 | redo | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3c1ec4357d6648c7` |  |
| Redo2 | Mask, 24 | redo | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3c1ec4357d6648c7` |  |
| Redo2 | Any, default | redo | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3c1ec4357d6648c7` |  |
| RefreshCw | Mask, 16 | refresh | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `95565df612a01340` |  |
| RefreshCw | Mask, 24 | refresh | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `95565df612a01340` |  |
| RefreshCw | Any, default | refresh | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `95565df612a01340` |  |
| Replace | Mask, 16 | find_replace | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `58594725ede94d25` |  |
| Replace | Mask, 24 | find_replace | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `58594725ede94d25` |  |
| Replace | Any, default | find_replace | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `58594725ede94d25` |  |
| ResizeCorner | Mask, 16 | drag_indicator | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `dac5e6c26e67e711` |  |
| ResizeCorner | Mask, 24 | drag_indicator | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `dac5e6c26e67e711` |  |
| ResizeCorner | Any, default | drag_indicator | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `dac5e6c26e67e711` |  |
| RotateCw | Mask, 16 | rotate_right | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c1410f961603fabb` |  |
| RotateCw | Mask, 24 | rotate_right | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c1410f961603fabb` |  |
| RotateCw | Any, default | rotate_right | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c1410f961603fabb` |  |
| Search | Mask, 16 | search | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2895b952363343c0` |  |
| Search | Mask, 24 | search | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2895b952363343c0` |  |
| Search | Any, default | search | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2895b952363343c0` |  |
| Settings | Mask, 16 | settings | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `b34731f36a1dce2a` |  |
| Settings | Mask, 24 | settings | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `b34731f36a1dce2a` |  |
| Settings | Any, default | settings | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `b34731f36a1dce2a` |  |
| Settings2 | Mask, 16 | tune | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2b9fe417d266bc82` |  |
| Settings2 | Mask, 24 | tune | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2b9fe417d266bc82` |  |
| Settings2 | Any, default | tune | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `2b9fe417d266bc82` |  |
| SortAscending | Mask, 16 | arrow_upward | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `408dec431c9dcaeb` |  |
| SortAscending | Mask, 24 | arrow_upward | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `408dec431c9dcaeb` |  |
| SortAscending | Any, default | arrow_upward | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `408dec431c9dcaeb` |  |
| SortDescending | Mask, 16 | arrow_downward | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3e26cd6f096d9c81` |  |
| SortDescending | Mask, 24 | arrow_downward | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3e26cd6f096d9c81` |  |
| SortDescending | Any, default | arrow_downward | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `3e26cd6f096d9c81` |  |
| Square | Mask, 16 | square | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `84eb470268265d7c` |  |
| Square | Mask, 24 | square | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `84eb470268265d7c` |  |
| Square | Any, default | square | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `84eb470268265d7c` |  |
| SquareTerminal | Mask, 16 | terminal | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `e83a830a9be6900d` |  |
| SquareTerminal | Mask, 24 | terminal | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `e83a830a9be6900d` |  |
| SquareTerminal | Any, default | terminal | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `e83a830a9be6900d` |  |
| Star | Mask, 16 | star | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `89f7198ae1cad26c` |  |
| Star | Mask, 24 | star | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `89f7198ae1cad26c` |  |
| Star | Any, default | star | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `89f7198ae1cad26c` |  |
| StarFill | Mask, 16 | star_fill1 | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4e3f734a1d70e58d` |  |
| StarFill | Mask, 24 | star_fill1 | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4e3f734a1d70e58d` |  |
| StarFill | Any, default | star_fill1 | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `4e3f734a1d70e58d` |  |
| StarOff | Mask, 16 | none | material | 16 | | | |
| StarOff | Mask, 24 | none | material | 24 | | | |
| StarOff | Any, default | none | material | default | | | |
| Sun | Mask, 16 | light_mode | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `426997e4cd0a8834` |  |
| Sun | Mask, 24 | light_mode | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `426997e4cd0a8834` |  |
| Sun | Any, default | light_mode | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `426997e4cd0a8834` |  |
| ThumbsDown | Mask, 16 | thumb_down | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `1ee3f5b1623bf477` |  |
| ThumbsDown | Mask, 24 | thumb_down | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `1ee3f5b1623bf477` |  |
| ThumbsDown | Any, default | thumb_down | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `1ee3f5b1623bf477` |  |
| ThumbsUp | Mask, 16 | thumb_up | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c2caf62d47deb4b0` |  |
| ThumbsUp | Mask, 24 | thumb_up | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c2caf62d47deb4b0` |  |
| ThumbsUp | Any, default | thumb_up | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `c2caf62d47deb4b0` |  |
| TriangleAlert | Mask, 16 | warning | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ddc294000f091c` |  |
| TriangleAlert | Mask, 24 | warning | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ddc294000f091c` |  |
| TriangleAlert | Any, default | warning | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `71ddc294000f091c` |  |
| Undo | Mask, 16 | undo | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `853cb1c32f618efe` |  |
| Undo | Mask, 24 | undo | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `853cb1c32f618efe` |  |
| Undo | Any, default | undo | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `853cb1c32f618efe` |  |
| Undo2 | Mask, 16 | undo | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `853cb1c32f618efe` |  |
| Undo2 | Mask, 24 | undo | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `853cb1c32f618efe` |  |
| Undo2 | Any, default | undo | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `853cb1c32f618efe` |  |
| User | Mask, 16 | person | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8faa8431c8257e96` |  |
| User | Mask, 24 | person | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8faa8431c8257e96` |  |
| User | Any, default | person | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `8faa8431c8257e96` |  |
| WindowClose | Mask, 16 | close | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fc6127371af54034` |  |
| WindowClose | Mask, 24 | close | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fc6127371af54034` |  |
| WindowClose | Any, default | close | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `fc6127371af54034` |  |
| WindowMaximize | Mask, 16 | open_in_full | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `43ed96c0095a9d02` |  |
| WindowMaximize | Mask, 24 | open_in_full | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `43ed96c0095a9d02` |  |
| WindowMaximize | Any, default | open_in_full | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `43ed96c0095a9d02` |  |
| WindowMinimize | Mask, 16 | minimize | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `76a0ab43a39996b3` |  |
| WindowMinimize | Mask, 24 | minimize | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `76a0ab43a39996b3` |  |
| WindowMinimize | Any, default | minimize | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `76a0ab43a39996b3` |  |
| WindowRestore | Mask, 16 | close_fullscreen | material | 16; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bec8b69b67c54e91` |  |
| WindowRestore | Mask, 24 | close_fullscreen | material | 24; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bec8b69b67c54e91` |  |
| WindowRestore | Any, default | close_fullscreen | material | default; viewBox `0 -960 960 960` | yes (deviation 0 / 3812) | `bec8b69b67c54e91` |  |

### `AdwaitaLegacy` (listed, not judged)

| IconName | Lookup | Name found | Theme searched | Size | One colour | Hash | Same hash in |
|---|---|---|---|---|---|---|---|
| ALargeSmall | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ArrowDown | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ArrowLeft | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ArrowRight | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ArrowUp | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Asterisk | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Ban | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Battery | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| BatteryCharging | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| BatteryFull | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| BatteryLow | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| BatteryMedium | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| BatteryWarning | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Bell | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| BookOpen | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Bot | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Building2 | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Calendar | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| CaseSensitive | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ChartPie | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Check | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ChevronDown | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ChevronLeft | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ChevronRight | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ChevronsUpDown | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ChevronUp | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| CircleAlert | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| CircleCheck | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| CircleUser | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| CircleX | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Close | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Copy | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Cpu | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Dash | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Delete | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Ellipsis | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| EllipsisVertical | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ExternalLink | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Eye | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| EyeOff | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| File | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| FileText | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Folder | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| FolderClosed | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| FolderOpen | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Frame | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| GalleryVerticalEnd | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Github | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Globe | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| HardDrive | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Heart | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| HeartOff | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Inbox | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Info | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Inspector | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| LayoutDashboard | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Loader | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| LoaderCircle | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Map | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Maximize | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| MemoryStick | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Menu | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Mic | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Minimize | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Minus | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Moon | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Network | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Palette | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelBottom | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelBottomOpen | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelLeft | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelLeftClose | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelLeftOpen | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelRight | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelRightClose | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| PanelRightOpen | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Pause | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Play | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Plus | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Redo | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Redo2 | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| RefreshCw | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Replace | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ResizeCorner | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| RotateCw | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Search | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Settings | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Settings2 | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| SortAscending | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| SortDescending | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Square | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| SquareTerminal | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Star | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| StarFill | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| StarOff | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Sun | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ThumbsDown | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| ThumbsUp | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| TriangleAlert | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Undo | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| Undo2 | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| User | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| WindowClose | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| WindowMaximize | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| WindowMinimize | Mask, 16 | none | AdwaitaLegacy | 16 | | | |
| WindowRestore | Mask, 16 | none | AdwaitaLegacy | 16 | | | |

### `char-white` (listed, not judged)

| IconName | Lookup | Name found | Theme searched | Size | One colour | Hash | Same hash in |
|---|---|---|---|---|---|---|---|
| ALargeSmall | Mask, 16 | `format-font-size-more` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `987a4eaad3c705d1` |  |
| ArrowDown | Mask, 16 | `go-bottom` as `go-bottom-symbolic` | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `5c89b236d4b85bb2` |  |
| ArrowLeft | Mask, 16 | `go-first` as `go-first-symbolic` | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `0dcc49e26f972e52` |  |
| ArrowRight | Mask, 16 | `go-last` as `go-last-symbolic` | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `6530ef697b5c2499` |  |
| ArrowUp | Mask, 16 | `go-top` as `go-top-symbolic` | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `c77179d0cceaace9` |  |
| Asterisk | Mask, 16 | `rating` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `1cc471eb4cc4a545` |  |
| Ban | Mask, 16 | `action-unavailable` as `action-unavailable-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `05bfd7ea988dcac4` |  |
| Battery | Mask, 16 | `battery` as `battery-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `359062c61988d1fb` |  |
| BatteryCharging | Mask, 16 | `battery-100-charging` (plain) | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `8f8404908fd840c4` |  |
| BatteryFull | Mask, 16 | `battery-100` (plain) | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `dd0761b48a08da1e` |  |
| BatteryLow | Mask, 16 | `battery-020` (plain) | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `503547fe654e5d8d` |  |
| BatteryMedium | Mask, 16 | `battery-good` as `battery-good-symbolic` | char-white | 16; no viewBox | yes (deviation 219 / 3812) | `0feafb8fe71d55bf` |  |
| BatteryWarning | Mask, 16 | `battery-caution` as `battery-caution-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `503547fe654e5d8d` |  |
| Bell | Mask, 16 | `alarm` as `alarm-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `4ce99a6933b3e10b` |  |
| BookOpen | Mask, 16 | `help-contents` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `b28185dc07bd2cbe` |  |
| Bot | Mask, 16 | `face-smile` as `face-smile-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `f3e313370047ebb2` |  |
| Building2 | Mask, 16 | `applications-office` as `applications-office-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `f51ba1fb7ce59f6f` |  |
| Calendar | Mask, 16 | `view-calendar` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `38c91aade9047d09` |  |
| CaseSensitive | Mask, 16 | `format-text-uppercase` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `ab1e5b5510deccec` |  |
| ChartPie | Mask, 16 | `office-chart-pie` (plain) | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `06dfcc891263e313` |  |
| Check | Mask, 16 | `dialog-ok` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `4d3b9c8a97a2d7cc` |  |
| ChevronDown | Mask, 16 | `go-down` as `go-down-symbolic` | char-white | 16; no viewBox | yes (deviation 160 / 3812) | `5c2a921aca5611a2` |  |
| ChevronLeft | Mask, 16 | `go-previous` as `go-previous-symbolic` | char-white | 16; no viewBox | yes (deviation 192 / 3812) | `630d5c6ae0c896e8` |  |
| ChevronRight | Mask, 16 | `go-next` as `go-next-symbolic` | char-white | 16; no viewBox | yes (deviation 192 / 3812) | `4c0324a05a2d75c4` |  |
| ChevronsUpDown | Mask, 16 | none | char-white | 16 | | | |
| ChevronUp | Mask, 16 | `go-up` as `go-up-symbolic` | char-white | 16; no viewBox | yes (deviation 192 / 3812) | `4b9ea98d48452f8e` |  |
| CircleAlert | Mask, 16 | `emblem-important` as `emblem-important-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `963dd8fdedc5d160` |  |
| CircleCheck | Mask, 16 | `emblem-ok-symbolic` (symbolic, as named) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `3c9915d5f66b91b5` |  |
| CircleUser | Mask, 16 | `user-identity` (plain) | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `e0508186d8b938cb` |  |
| CircleX | Mask, 16 | `dialog-error` as `dialog-error-symbolic` | char-white | 16; no viewBox | yes (deviation 240 / 3812) | `c644148a6627a637` |  |
| Close | Mask, 16 | `tab-close` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `e9d33093bf9caa86` |  |
| Copy | Mask, 16 | `edit-copy` as `edit-copy-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `aa270a6b6bede14f` |  |
| Cpu | Mask, 16 | `cpu` as `cpu-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `1c4a5040e0225a14` |  |
| Dash | Mask, 16 | `list-remove` as `list-remove-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `b4e30537754ebc13` |  |
| Delete | Mask, 16 | `edit-delete` as `edit-delete-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `4c2ce90091db8449` |  |
| Ellipsis | Mask, 16 | `view-more-horizontal` as `view-more-horizontal-symbolic` | char-white | 16; no viewBox | yes (deviation 128 / 3812) | `e7acbcde31d02de4` |  |
| EllipsisVertical | Mask, 16 | `view-more` as `view-more-symbolic` | char-white | 16; no viewBox | yes (deviation 128 / 3812) | `bcb02679068f6e6a` |  |
| ExternalLink | Mask, 16 | `insert-link` as `insert-link-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `cba14d59438cdbbf` |  |
| Eye | Mask, 16 | none | char-white | 16 | | | |
| EyeOff | Mask, 16 | `view-hidden` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `dd942db199b75de6` |  |
| File | Mask, 16 | `text-x-generic` (plain) | char-white | 16; no viewBox | yes (deviation 96 / 3812) | `89077b8bc86b12a0` |  |
| FileText | Mask, 16 | `text-x-generic` (plain) | char-white | 16; no viewBox | yes (deviation 96 / 3812) | `89077b8bc86b12a0` |  |
| Folder | Mask, 16 | `folder` as `folder-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `a48d797a68ca665e` |  |
| FolderClosed | Mask, 16 | `folder` as `folder-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `a48d797a68ca665e` |  |
| FolderOpen | Mask, 16 | `folder-open` as `folder-open-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `941c5c2310db9848` |  |
| Frame | Mask, 16 | `select-rectangular` (plain) | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `25399b787a7b7d7c` |  |
| GalleryVerticalEnd | Mask, 16 | `view-list-icons` (plain) | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `7460c768517f0bcd` |  |
| Github | Mask, 16 | `applications-engineering` as `applications-engineering-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `7627cdba2ea81e73` |  |
| Globe | Mask, 16 | `globe` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `a747238fb608d79d` |  |
| HardDrive | Mask, 16 | `drive-harddisk` as `drive-harddisk-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `f0a843b3a48e307f` |  |
| Heart | Mask, 16 | `emblem-favorite` as `emblem-favorite-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `84b22d70a547cda3` |  |
| HeartOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `ae93590d9dda4709` |  |
| Inbox | Mask, 16 | `mail-folder-inbox` (plain) | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `f51f8a2cb3bc698d` |  |
| Info | Mask, 16 | `dialog-information` as `dialog-information-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `567c026e50d60cbf` |  |
| Inspector | Mask, 16 | `code-context` (plain) | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `554ee0f1dd6612ae` |  |
| LayoutDashboard | Mask, 16 | `view-grid` as `view-grid-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `538e1b4b1219dc7a` |  |
| Loader | Mask, 16 | `process-working-symbolic` (symbolic, as named) | char-white | 16; no viewBox | yes (deviation 240 / 3812) | `d79fcd498b651260` |  |
| LoaderCircle | Mask, 16 | `process-working-symbolic` (symbolic, as named) | char-white | 16; no viewBox | yes (deviation 240 / 3812) | `d79fcd498b651260` |  |
| Map | Mask, 16 | `find-location` as `find-location-symbolic` | char-white | 16; no viewBox | yes (deviation 192 / 3812) | `e7ba650a3e866956` |  |
| Maximize | Mask, 16 | `view-fullscreen` as `view-fullscreen-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `f00328b0b8265266` |  |
| MemoryStick | Mask, 16 | `media-flash` as `media-flash-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `b26bd43391589a56` |  |
| Menu | Mask, 16 | `open-menu` as `open-menu-symbolic` | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `4ee4143a9c925dbc` |  |
| Mic | Mask, 16 | `audio-input-microphone` as `audio-input-microphone-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `117c67a1dc399f82` |  |
| Minimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `e7d83fe74dee5c1b` |  |
| Minus | Mask, 16 | `list-remove` as `list-remove-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `b4e30537754ebc13` |  |
| Moon | Mask, 16 | `weather-clear-night` as `weather-clear-night-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `5e73f9f4299b253e` |  |
| Network | Mask, 16 | `network-workgroup` as `network-workgroup-symbolic` | char-white | 16; no viewBox | yes (deviation 384 / 3812) | `dfa22f8257b06565` |  |
| Palette | Mask, 16 | `palette` as `palette-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `5dccf6bf0b9121ac` |  |
| PanelBottom | Mask, 16 | `view-split-top-bottom` (plain) | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `ad7e657da5afb6c4` |  |
| PanelBottomOpen | Mask, 16 | `view-split-top-bottom` (plain) | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `ad7e657da5afb6c4` |  |
| PanelLeft | Mask, 16 | none | char-white | 16 | | | |
| PanelLeftClose | Mask, 16 | `view-left-close` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `a6092aa72cdb9cc5` |  |
| PanelLeftOpen | Mask, 16 | none | char-white | 16 | | | |
| PanelRight | Mask, 16 | none | char-white | 16 | | | |
| PanelRightClose | Mask, 16 | `view-right-close` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `8d82febc35209d91` |  |
| PanelRightOpen | Mask, 16 | `view-right-new` (plain) | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `fcbb9fca26224816` |  |
| Pause | Mask, 16 | `media-playback-pause` as `media-playback-pause-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `7a40c51f863b668e` |  |
| Play | Mask, 16 | `media-playback-start` as `media-playback-start-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `bc5cd3016abcbb42` |  |
| Plus | Mask, 16 | `list-add` as `list-add-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `24e2e6a74e0f174c` |  |
| Redo | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `411bf203e977fe34` |  |
| Redo2 | Mask, 16 | `edit-redo` as `edit-redo-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `411bf203e977fe34` |  |
| RefreshCw | Mask, 16 | `view-refresh` as `view-refresh-symbolic` | char-white | 16; no viewBox | yes (deviation 372 / 3812) | `68d095c550d5f7f6` |  |
| Replace | Mask, 16 | `edit-find-replace` as `edit-find-replace-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `a35d05db2db9a3d1` |  |
| ResizeCorner | Mask, 16 | none | char-white | 16 | | | |
| RotateCw | Mask, 16 | `object-rotate-right` as `object-rotate-right-symbolic` | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `d9e7173edee11adc` |  |
| Search | Mask, 16 | `edit-find` as `edit-find-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `8e9c40868346553c` |  |
| Settings | Mask, 16 | `preferences-system` as `preferences-system-symbolic` | char-white | 16; no viewBox | yes (deviation 332 / 3812) | `dd724e0a8e927dd2` |  |
| Settings2 | Mask, 16 | `configure` (plain) | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `df969c7864dba825` |  |
| SortAscending | Mask, 16 | `view-sort-ascending` as `view-sort-ascending-symbolic` | char-white | 16; no viewBox | yes (deviation 224 / 3812) | `24708d6071e1f91f` |  |
| SortDescending | Mask, 16 | `view-sort-descending` as `view-sort-descending-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `908f3ba4a09557a9` |  |
| Square | Mask, 16 | `media-playback-stop` as `media-playback-stop-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `2445da73fa13a655` |  |
| SquareTerminal | Mask, 16 | `utilities-terminal` as `utilities-terminal-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `cad31ef6ec22ce4c` |  |
| Star | Mask, 16 | `non-starred` as `non-starred-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `ae93590d9dda4709` |  |
| StarFill | Mask, 16 | `starred` as `starred-symbolic` | char-white | 16; no viewBox | yes (deviation 223 / 3812) | `734597c40987f463` |  |
| StarOff | Mask, 16 | `non-starred` as `non-starred-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `ae93590d9dda4709` |  |
| Sun | Mask, 16 | `weather-clear` as `weather-clear-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `aca437cf7889fbe2` |  |
| ThumbsDown | Mask, 16 | `rating-unrated` (plain) | char-white | 16; no viewBox | yes (deviation 154 / 3812) | `6673cb09f8fbfed0` |  |
| ThumbsUp | Mask, 16 | `checkbox-checked` as `checkbox-checked-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `4b338ebe943f0a0c` |  |
| TriangleAlert | Mask, 16 | `dialog-warning` as `dialog-warning-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `a2a240661e729427` |  |
| Undo | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `5877272ec5c71cf6` |  |
| Undo2 | Mask, 16 | `edit-undo` as `edit-undo-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `5877272ec5c71cf6` |  |
| User | Mask, 16 | `system-users` as `system-users-symbolic` | char-white | 16; no viewBox | yes (deviation 239 / 3812) | `53a0c700d3c1a955` |  |
| WindowClose | Mask, 16 | `window-close` as `window-close-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `12767ddc59081a2a` |  |
| WindowMaximize | Mask, 16 | `window-maximize` as `window-maximize-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `876e6a4b6ee3183f` |  |
| WindowMinimize | Mask, 16 | `window-minimize` as `window-minimize-symbolic` | char-white | 16; no viewBox | yes (deviation 0 / 3812) | `e7d83fe74dee5c1b` |  |
| WindowRestore | Mask, 16 | `window-restore` as `window-restore-symbolic` | char-white | 16; no viewBox | yes (deviation 208 / 3812) | `876e6a4b6ee3183f` |  |

## 2. The counts

| Theme or set | Served at 16 px | Served at 24 px | Empty at 16 px |
|---|---|---|---|
| `breeze` | 98 | 98 | 8 |
| `breeze-dark` | 98 | 98 | 8 |
| `Adwaita` | 106 | 106 | 0 |
| `lucide` | 105 | 105 | 1 |
| `material` | 105 | 105 | 1 |

## 3. The nine essentials at 16 px

| Theme or set | WindowClose | WindowMinimize | WindowMaximize | WindowRestore | Close | ChevronDown | Check | Minus | Plus |
|---|---|---|---|---|---|---|---|---|---|
| `breeze` | found | found | found | found | found | found | found | found | found |
| `breeze-dark` | found | found | found | found | found | found | found | found | found |
| `Adwaita` | found | found | found | found | found | found | found | found | found |
| `lucide` | found | found | found | found | found | found | found | found | found |
| `material` | found | found | found | found | found | found | found | found | found |

## 4. The four names' candidates (`FreedesktopLoader::new(name)` at 16 px)

### `breeze` (sheet `breeze-four.png`: per row, as loaded, then as a mask)

| Sheet row | IconName | Name | Of | Found | Size | One colour | Hash | Served |
|---|---|---|---|---|---|---|---|---|
| 1 | Check | `dialog-ok` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` | yes |
| 2 | Check | `object-select` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` | yes |
| 3 | Check | `emblem-default` | the role | yes | viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` | yes |
| 4 | CircleCheck | `emblem-ok-symbolic` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` | yes |
| 5 | CircleCheck | `object-select` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` | yes |
| 6 | CircleCheck | `object-select-symbolic` | the role | yes | viewBox `0 0 16 16` | yes (deviation 192 / 3812) | `e022555e435e1f81` | yes |
| 7 | Loader | `process-working-symbolic` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `182da088be4153db` | yes |
| 8 | Loader | `content-loading` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `d75f6815f779e541` | no |
| 9 | Loader | `content-loading-symbolic` | the role | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `d75f6815f779e541` | no |
| 10 | LoaderCircle | `process-working-symbolic` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 224 / 3812) | `182da088be4153db` | yes |
| 11 | LoaderCircle | `content-loading` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `d75f6815f779e541` | no |

### `breeze-dark` (sheet `breeze-dark-four.png`: per row, as loaded, then as a mask)

| Sheet row | IconName | Name | Of | Found | Size | One colour | Hash | Served |
|---|---|---|---|---|---|---|---|---|
| 1 | Check | `dialog-ok` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | yes |
| 2 | Check | `object-select` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | yes |
| 3 | Check | `emblem-default` | the role | yes | viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | yes |
| 4 | CircleCheck | `emblem-ok-symbolic` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | yes |
| 5 | CircleCheck | `object-select` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | yes |
| 6 | CircleCheck | `object-select-symbolic` | the role | yes | viewBox `0 0 16 16` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | yes |
| 7 | Loader | `process-working-symbolic` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `2f8f10c84a9bd7b5` | yes |
| 8 | Loader | `content-loading` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3acfd1544910e8da` | no |
| 9 | Loader | `content-loading-symbolic` | the role | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3acfd1544910e8da` | no |
| 10 | LoaderCircle | `process-working-symbolic` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 240 / 3812) | `2f8f10c84a9bd7b5` | yes |
| 11 | LoaderCircle | `content-loading` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `3acfd1544910e8da` | no |

### `Adwaita` (sheet `Adwaita-four.png`: per row, as loaded, then as a mask)

| Sheet row | IconName | Name | Of | Found | Size | One colour | Hash | Served |
|---|---|---|---|---|---|---|---|---|
| 1 | Check | `dialog-ok` | the entry | no | | | | |
| 2 | Check | `object-select` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c644d75d68273e7` | yes |
| 3 | Check | `emblem-default` | the role | yes | 16 × 16 px | no: pixels | `851229929d602652` | no |
| 4 | CircleCheck | `emblem-ok-symbolic` | the entry | no | | | | |
| 5 | CircleCheck | `object-select` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c644d75d68273e7` | yes |
| 6 | CircleCheck | `object-select-symbolic` | the role | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `6c644d75d68273e7` | yes |
| 7 | Loader | `process-working-symbolic` | the entry | no | | | | |
| 8 | Loader | `content-loading` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0580dd08401064f9` | yes |
| 9 | Loader | `content-loading-symbolic` | the role | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0580dd08401064f9` | yes |
| 10 | LoaderCircle | `process-working-symbolic` | the entry | no | | | | |
| 11 | LoaderCircle | `content-loading` | the entry | yes | viewBox `0 0 16 16` | yes (deviation 0 / 3812) | `0580dd08401064f9` | yes |

## 5. `Loader` and `LoaderCircle` as strips

| Theme | IconName | Lookup | viewBox (pixels: `0 0` and their size) | A strip (height ≥ 2 × width) |
|---|---|---|---|---|
| `breeze` | Loader | Mask, 16 | `0 0 16 16` | no |
| `breeze` | Loader | Mask, 24 | `0 0 24 24` | no |
| `breeze` | Loader | Any, default | `0 0 24 24` | no |
| `breeze` | LoaderCircle | Mask, 16 | `0 0 16 16` | no |
| `breeze` | LoaderCircle | Mask, 24 | `0 0 24 24` | no |
| `breeze` | LoaderCircle | Any, default | `0 0 24 24` | no |
| `breeze-dark` | Loader | Mask, 16 | `0 0 16 16` | no |
| `breeze-dark` | Loader | Mask, 24 | `0 0 24 24` | no |
| `breeze-dark` | Loader | Any, default | `0 0 24 24` | no |
| `breeze-dark` | LoaderCircle | Mask, 16 | `0 0 16 16` | no |
| `breeze-dark` | LoaderCircle | Mask, 24 | `0 0 24 24` | no |
| `breeze-dark` | LoaderCircle | Any, default | `0 0 24 24` | no |
| `Adwaita` | Loader | Mask, 16 | `0 0 16 16` | no |
| `Adwaita` | Loader | Mask, 24 | `0 0 16 16` | no |
| `Adwaita` | Loader | Any, default | `0 0 16 16` | no |
| `Adwaita` | LoaderCircle | Mask, 16 | `0 0 16 16` | no |
| `Adwaita` | LoaderCircle | Mask, 24 | `0 0 16 16` | no |
| `Adwaita` | LoaderCircle | Any, default | `0 0 16 16` | no |

- `FreedesktopLoader::new("process-working-symbolic")` in `breeze`: finds an icon.
- `FreedesktopLoader::new("process-working-symbolic")` in `breeze-dark`: finds an icon.

## 6. Light and dark (16 px `Mask` hashes)

### `breeze` and `breeze-dark`

| IconName | Light | Dark | Equal |
|---|---|---|---|
| ALargeSmall | `923128e4bc8ba8dd` | `55547edbd62de30b` | no |
| ArrowDown | `75c53f657763633a` | `1c695d87e607adb4` | no |
| ArrowLeft | `a1518e5a43dcca80` | `79b710d6b9593806` | no |
| ArrowRight | `ab62b1109accdc24` | `fee9919441086b56` | no |
| ArrowUp | `deb929073b7ee1fc` | `46d14502e49717aa` | no |
| Asterisk | `5a3b736f175ac491` | `9714fd08848b74bb` | no |
| Ban | `8cdc76f415046d88` | `9a7c0efd4465ea64` | no |
| Battery | none | none | yes |
| BatteryCharging | none | none | yes |
| BatteryFull | `3e34fce44b653cbc` | `8d8e9465a1534e6c` | no |
| BatteryLow | `0baae81a71835745` | `4909564e158b9b37` | no |
| BatteryMedium | `36db1e16af058774` | `cfa263f9b36fc324` | no |
| BatteryWarning | `52177a1e5e7f1629` | `910727770d86e262` | no |
| Bell | `ee5d7c84a6529a4f` | `cf4f27b8dad609f2` | no |
| BookOpen | `f6246c957b35ad80` | `193017aadc2040a6` | no |
| Bot | none | none | yes |
| Building2 | `d3f7482bd819ed38` | `c20bc20887411bfa` | no |
| Calendar | `28dc851305cf41c0` | `8ebf5fcf4fbf567c` | no |
| CaseSensitive | none | none | yes |
| ChartPie | `904b8d2fd8bb9fb2` | `1f4707f34bc6c66a` | no |
| Check | `e022555e435e1f81` | `553a50e52c12bb0b` | no |
| ChevronDown | `ed5ca0625947021f` | `b0ac80e68c954385` | no |
| ChevronLeft | `59fc678f1a03c743` | `632a9cca63757d29` | no |
| ChevronRight | `1eeb8df06e3c9fae` | `584f2b9786ce5f50` | no |
| ChevronsUpDown | `ef154ac18ca46115` | `67c2e36088422b3e` | no |
| ChevronUp | `dd9afec33cf1712e` | `5611e8e1a550b590` | no |
| CircleAlert | `61bd666bc22647c6` | `33e9c517d3920eb2` | no |
| CircleCheck | `e022555e435e1f81` | `553a50e52c12bb0b` | no |
| CircleUser | `914b9c2ca206b71b` | `ea61bff2647f04d3` | no |
| CircleX | none | none | yes |
| Close | `99cf5eaec3ba2564` | `fec475d9f64dcf2c` | no |
| Copy | `5aa7d4b71ec43245` | `a74bcc63c8e01533` | no |
| Cpu | `a9ede46ca4499e33` | `50de050d5526f8eb` | no |
| Dash | `0a9be8349d2c0b4f` | `e573a187602106c2` | no |
| Delete | `5119f2098b513357` | `bfc4d808ef85f23a` | no |
| Ellipsis | `f2113864328a80fa` | `a4da1d13fad43619` | no |
| EllipsisVertical | `f2113864328a80fa` | `a4da1d13fad43619` | no |
| ExternalLink | `c8f71f17ffb78bc0` | `3b338c5326c6923e` | no |
| Eye | `4accbe13da9b26d9` | `4a844410717b90a9` | no |
| EyeOff | `95d80f5dd7d324a8` | `591a476dc8467a00` | no |
| File | `3f6bf16d1d553b0a` | `3f6bf16d1d553b0a` | yes |
| FileText | `3f6bf16d1d553b0a` | `3f6bf16d1d553b0a` | yes |
| Folder | `f6b1ad730ce91a0c` | `5dec625f544f9b52` | no |
| FolderClosed | `f6b1ad730ce91a0c` | `5dec625f544f9b52` | no |
| FolderOpen | `2b4b8abd20b075d7` | `36b1c1895c2cc557` | no |
| Frame | `a00bcfc2c955810c` | `ab3a59c4bede8d8a` | no |
| GalleryVerticalEnd | `d662bdf56edc92fd` | `de75685057dc6877` | no |
| Github | `f4f31c07d19a5bcd` | `4e0db9289694aa8b` | no |
| Globe | `4806ea6517aed73c` | `f6124cda917cb08e` | no |
| HardDrive | `d37ebe49449c838d` | `efb8fd30e8ce8673` | no |
| Heart | `19cd0488306aaded` | `342a7d3826c31b57` | no |
| HeartOff | `ee5d5a02d49d98b3` | `9dbc44682de104bb` | no |
| Inbox | `60a509048b0ab957` | `4f23b52995e6ee43` | no |
| Info | none | none | yes |
| Inspector | `a4f0e2e5c32b8ed6` | `6f4f3ad3b32c4ba2` | no |
| LayoutDashboard | `61435d0ac1b031d0` | `f0fc293a7191f5e0` | no |
| Loader | `182da088be4153db` | `2f8f10c84a9bd7b5` | no |
| LoaderCircle | `182da088be4153db` | `2f8f10c84a9bd7b5` | no |
| Map | `7d4a6e33cfcf2afd` | `315f326e0a6f49f7` | no |
| Maximize | `a280239f514179ff` | `f7bd04a2764fa755` | no |
| MemoryStick | `77cf71460b0b4526` | `f5364ac5ff7bc27a` | no |
| Menu | `5bd0ec38997e0814` | `56980dc5605a12fc` | no |
| Mic | `1cfba4bce84ce52d` | `c337894c4f6896a9` | no |
| Minimize | `ed5ca0625947021f` | `b0ac80e68c954385` | no |
| Minus | `0a9be8349d2c0b4f` | `e573a187602106c2` | no |
| Moon | `341efe5e9f225ade` | `9c352462d523de4e` | no |
| Network | `c061e469bea64a65` | `cf43d5533545a057` | no |
| Palette | `78637aebe2c9156d` | `2d804cb91f80da4b` | no |
| PanelBottom | `cba7cf686842597e` | `b8cb87cb1cdeed7e` | no |
| PanelBottomOpen | `cba7cf686842597e` | `b8cb87cb1cdeed7e` | no |
| PanelLeft | `03b4d50e736b1032` | `e6229aca219088fc` | no |
| PanelLeftClose | `03b4d50e736b1032` | `e6229aca219088fc` | no |
| PanelLeftOpen | `e9bb5fc14f0fea07` | `0e4a70e499dbb6f1` | no |
| PanelRight | `fd1a4c017b6e7d41` | `5d5598d032cbcbc3` | no |
| PanelRightClose | none | none | yes |
| PanelRightOpen | `631efb9b927d80a5` | `22d46d1c4720a09b` | no |
| Pause | `b079ec02f5ad64be` | `8074e222aca5a0b4` | no |
| Play | `74f3d2e357272d64` | `363cd545fb27c682` | no |
| Plus | `6b2e1d04402d3f13` | `1056f394e1ea5211` | no |
| Redo | `6ad935df86eb9db0` | `35d1a62df009b25c` | no |
| Redo2 | `6ad935df86eb9db0` | `35d1a62df009b25c` | no |
| RefreshCw | `cbb75da0febc2e5a` | `24f82fc9713e3292` | no |
| Replace | `e00377dc24e99350` | `dd777118da3bc308` | no |
| ResizeCorner | `022c022365c08b4c` | `af4e5f77a029ca88` | no |
| RotateCw | `fc68f8c17b8db1bf` | `9656d0f9bca80529` | no |
| Search | `5a2c1613135c28bb` | `5c91c0ff963d2781` | no |
| Settings | `8f7bfe2900a0f78d` | `376b4d1b33c6e853` | no |
| Settings2 | `8f7bfe2900a0f78d` | `376b4d1b33c6e853` | no |
| SortAscending | `fefb013de39cd030` | `50b2e75540dad08e` | no |
| SortDescending | `3e0c1a7566b5af1c` | `b5c18571723a627c` | no |
| Square | `4ca62b989db0cc51` | `11f280085e7619e7` | no |
| SquareTerminal | `55279cac63820c07` | `1be2dfd599493601` | no |
| Star | `ee5d5a02d49d98b3` | `9dbc44682de104bb` | no |
| StarFill | `1a04acab760691f9` | `c0bdec7c4a50ef91` | no |
| StarOff | `ee5d5a02d49d98b3` | `9dbc44682de104bb` | no |
| Sun | `4f620d3f7bb4bb0e` | `83f4080da35a24aa` | no |
| ThumbsDown | `1f03abbaea43a7dd` | `cf6a0b6bd32b56e3` | no |
| ThumbsUp | `58ac5a540c8134c8` | `c6c8a33410d18004` | no |
| TriangleAlert | none | none | yes |
| Undo | `7b784837cb2428b4` | `85f9d13fc9de411c` | no |
| Undo2 | `7b784837cb2428b4` | `85f9d13fc9de411c` | no |
| User | `e354a0cac3a5a5e9` | `57a0cc8df2d6eb9f` | no |
| WindowClose | `99cf5eaec3ba2564` | `fec475d9f64dcf2c` | no |
| WindowMaximize | `dd9afec33cf1712e` | `5611e8e1a550b590` | no |
| WindowMinimize | `ed5ca0625947021f` | `b0ac80e68c954385` | no |
| WindowRestore | `f808ee3371596663` | `7dfd1b73c66cb4dd` | no |

Equal: 10 of 106 (two names with no icon on either side count as equal).

### `Adwaita` and `Adwaita`

| IconName | Light | Dark | Equal |
|---|---|---|---|
| ALargeSmall | `179c43170e5a108c` | `179c43170e5a108c` | yes |
| ArrowDown | `cbce9487d10c4dea` | `cbce9487d10c4dea` | yes |
| ArrowLeft | `68e9711c00a016af` | `68e9711c00a016af` | yes |
| ArrowRight | `bf8c3c88b2e9a11a` | `bf8c3c88b2e9a11a` | yes |
| ArrowUp | `3789397b9b7eb9dc` | `3789397b9b7eb9dc` | yes |
| Asterisk | `6052d683c5e94043` | `6052d683c5e94043` | yes |
| Ban | `ca33669f2852db32` | `ca33669f2852db32` | yes |
| Battery | `ac4efcfb92198c9a` | `ac4efcfb92198c9a` | yes |
| BatteryCharging | `c596244bb30006cf` | `c596244bb30006cf` | yes |
| BatteryFull | `1b2ceef4d77db5f1` | `1b2ceef4d77db5f1` | yes |
| BatteryLow | `434487d557a1c30d` | `434487d557a1c30d` | yes |
| BatteryMedium | `1ccc8e2a2b603f29` | `1ccc8e2a2b603f29` | yes |
| BatteryWarning | `13b2388f07ac754a` | `13b2388f07ac754a` | yes |
| Bell | `2f09986ccab74a91` | `2f09986ccab74a91` | yes |
| BookOpen | `d45ef39c76f4ad31` | `d45ef39c76f4ad31` | yes |
| Bot | `1e8bf93ff365e18a` | `1e8bf93ff365e18a` | yes |
| Building2 | `badc447e7c43d04a` | `badc447e7c43d04a` | yes |
| Calendar | `c17aec4823e2b700` | `c17aec4823e2b700` | yes |
| CaseSensitive | `2f3d9ffabddcccbb` | `2f3d9ffabddcccbb` | yes |
| ChartPie | `828dec802cbd67fb` | `828dec802cbd67fb` | yes |
| Check | `6c644d75d68273e7` | `6c644d75d68273e7` | yes |
| ChevronDown | `34ac62c114abb570` | `34ac62c114abb570` | yes |
| ChevronLeft | `670472afbd491974` | `670472afbd491974` | yes |
| ChevronRight | `17877726410215b6` | `17877726410215b6` | yes |
| ChevronsUpDown | `aafd8a8e11dee182` | `aafd8a8e11dee182` | yes |
| ChevronUp | `05884c17c1a7fa5f` | `05884c17c1a7fa5f` | yes |
| CircleAlert | `42a389f4861182ca` | `42a389f4861182ca` | yes |
| CircleCheck | `6c644d75d68273e7` | `6c644d75d68273e7` | yes |
| CircleUser | `006726bd1a29826c` | `006726bd1a29826c` | yes |
| CircleX | `b16efe53d49895e5` | `b16efe53d49895e5` | yes |
| Close | `0efc92212262d981` | `0efc92212262d981` | yes |
| Copy | `f314c37a34f2c86f` | `f314c37a34f2c86f` | yes |
| Cpu | `6f7be0707eb0e46b` | `6f7be0707eb0e46b` | yes |
| Dash | `3f605218560cfd03` | `3f605218560cfd03` | yes |
| Delete | `6c479dca7fbf18b0` | `6c479dca7fbf18b0` | yes |
| Ellipsis | `5d95b8a0f63a03d8` | `5d95b8a0f63a03d8` | yes |
| EllipsisVertical | `9911559874f29e1e` | `9911559874f29e1e` | yes |
| ExternalLink | `4d000a861f1b5b39` | `4d000a861f1b5b39` | yes |
| Eye | `5c6e6d69dc0c1b85` | `5c6e6d69dc0c1b85` | yes |
| EyeOff | `aec6323ae3369280` | `aec6323ae3369280` | yes |
| File | `bbb0323cf36b9bca` | `bbb0323cf36b9bca` | yes |
| FileText | `bbb0323cf36b9bca` | `bbb0323cf36b9bca` | yes |
| Folder | `2a0e275c95aa0b14` | `2a0e275c95aa0b14` | yes |
| FolderClosed | `2a0e275c95aa0b14` | `2a0e275c95aa0b14` | yes |
| FolderOpen | `c9e4d83a9b4d7779` | `c9e4d83a9b4d7779` | yes |
| Frame | `f70267e1e0665731` | `f70267e1e0665731` | yes |
| GalleryVerticalEnd | `1cf5d6288915b068` | `1cf5d6288915b068` | yes |
| Github | `4fbf86d40cbd21fa` | `4fbf86d40cbd21fa` | yes |
| Globe | `a4194c929c8329a8` | `a4194c929c8329a8` | yes |
| HardDrive | `7a7978ddb9b1f775` | `7a7978ddb9b1f775` | yes |
| Heart | `6052d683c5e94043` | `6052d683c5e94043` | yes |
| HeartOff | `4102eabe2e71b4f7` | `4102eabe2e71b4f7` | yes |
| Inbox | `8ea596c88f1c2493` | `8ea596c88f1c2493` | yes |
| Info | `222b7ef886c8a9b4` | `222b7ef886c8a9b4` | yes |
| Inspector | `c5713354683d78c8` | `c5713354683d78c8` | yes |
| LayoutDashboard | `617e3029d0a9feaa` | `617e3029d0a9feaa` | yes |
| Loader | `0580dd08401064f9` | `0580dd08401064f9` | yes |
| LoaderCircle | `0580dd08401064f9` | `0580dd08401064f9` | yes |
| Map | `2487b1a320fd6b28` | `2487b1a320fd6b28` | yes |
| Maximize | `ad500ec8e7361e4c` | `ad500ec8e7361e4c` | yes |
| MemoryStick | `cab852e030d870ec` | `cab852e030d870ec` | yes |
| Menu | `8e0ad7ba45736b41` | `8e0ad7ba45736b41` | yes |
| Mic | `59ed0ae58b7898c5` | `59ed0ae58b7898c5` | yes |
| Minimize | `ad3b28af2fd32dcb` | `ad3b28af2fd32dcb` | yes |
| Minus | `3f605218560cfd03` | `3f605218560cfd03` | yes |
| Moon | `4e6384f8958c6cbb` | `4e6384f8958c6cbb` | yes |
| Network | `badc447e7c43d04a` | `badc447e7c43d04a` | yes |
| Palette | `2f7bc6880bf2a9df` | `2f7bc6880bf2a9df` | yes |
| PanelBottom | `359fbcfce2b4ab4d` | `359fbcfce2b4ab4d` | yes |
| PanelBottomOpen | `359fbcfce2b4ab4d` | `359fbcfce2b4ab4d` | yes |
| PanelLeft | `4c34f9ec02447eea` | `4c34f9ec02447eea` | yes |
| PanelLeftClose | `4c34f9ec02447eea` | `4c34f9ec02447eea` | yes |
| PanelLeftOpen | `4c34f9ec02447eea` | `4c34f9ec02447eea` | yes |
| PanelRight | `4a4463fa61cb343f` | `4a4463fa61cb343f` | yes |
| PanelRightClose | `4a4463fa61cb343f` | `4a4463fa61cb343f` | yes |
| PanelRightOpen | `4a4463fa61cb343f` | `4a4463fa61cb343f` | yes |
| Pause | `38a3cb41866012b0` | `38a3cb41866012b0` | yes |
| Play | `60535cce91fd3b04` | `60535cce91fd3b04` | yes |
| Plus | `4f16c5165bf42c7a` | `4f16c5165bf42c7a` | yes |
| Redo | `38f2bbd8e60f92e9` | `38f2bbd8e60f92e9` | yes |
| Redo2 | `38f2bbd8e60f92e9` | `38f2bbd8e60f92e9` | yes |
| RefreshCw | `282cb8486eec37c3` | `282cb8486eec37c3` | yes |
| Replace | `c412e3d6f6464b2d` | `c412e3d6f6464b2d` | yes |
| ResizeCorner | `aafd8a8e11dee182` | `aafd8a8e11dee182` | yes |
| RotateCw | `484ec7f7330a8731` | `484ec7f7330a8731` | yes |
| Search | `5d2d0437e9d30b2d` | `5d2d0437e9d30b2d` | yes |
| Settings | `b33201d86820d61a` | `b33201d86820d61a` | yes |
| Settings2 | `e48917ee25f389ef` | `e48917ee25f389ef` | yes |
| SortAscending | `325ad1f8697f734e` | `325ad1f8697f734e` | yes |
| SortDescending | `0fa0cb834e230c47` | `0fa0cb834e230c47` | yes |
| Square | `4645c18b191f13c2` | `4645c18b191f13c2` | yes |
| SquareTerminal | `3941b80ab0d9d8ac` | `3941b80ab0d9d8ac` | yes |
| Star | `4102eabe2e71b4f7` | `4102eabe2e71b4f7` | yes |
| StarFill | `6052d683c5e94043` | `6052d683c5e94043` | yes |
| StarOff | `4102eabe2e71b4f7` | `4102eabe2e71b4f7` | yes |
| Sun | `709e99ff89923203` | `709e99ff89923203` | yes |
| ThumbsDown | `693e7ee13649f5a0` | `693e7ee13649f5a0` | yes |
| ThumbsUp | `8869b483ed9b5396` | `8869b483ed9b5396` | yes |
| TriangleAlert | `9078d699872eed0a` | `9078d699872eed0a` | yes |
| Undo | `ae38e7741c933a30` | `ae38e7741c933a30` | yes |
| Undo2 | `ae38e7741c933a30` | `ae38e7741c933a30` | yes |
| User | `441f403e7120ae96` | `441f403e7120ae96` | yes |
| WindowClose | `0efc92212262d981` | `0efc92212262d981` | yes |
| WindowMaximize | `a50bb530b77a3244` | `a50bb530b77a3244` | yes |
| WindowMinimize | `ad3b28af2fd32dcb` | `ad3b28af2fd32dcb` | yes |
| WindowRestore | `424369cb7988ab1d` | `424369cb7988ab1d` | yes |

Equal: 106 of 106 (two names with no icon on either side count as equal).

## 7. The timings, per switch (milliseconds)

From `catppuccin-latte Light`, the first preset whose icon set is Lucide; one warm-up run, not recorded, then 5 runs, each from the start. The handle's side: `load_gpui_icon(icon, Freedesktop, Some(theme), Some(16), None, IconUse::Mask)` for the 106 names, once per icon theme the switch loads (a table per icon theme of both modes not yet loaded since the set last changed), and `mask_drawable` of each icon the showcase's two lists load for that switch. The showcase's side: `load_all_icons` and `load_gpui_icons`, then `to_image_source` of each icon, where its set, theme or tint changed; not timed: the animation caches and the redraw.

| Step | Handle loads | Handle, each run | Median | Largest | Showcase reloads | Showcase, each run | Median | Largest | Handle's median above |
|---|---|---|---|---|---|---|---|---|---|
| kde-breeze Light | `breeze`, `breeze-dark` | 25.308, 25.096, 25.540, 25.281, 25.199 | 25.281 | 25.540 | yes | 15.611, 15.661, 15.845, 15.721, 15.599 | 15.661 | 15.845 | **yes** |
| kde-breeze Dark | none | 5.465, 5.508, 5.518, 5.495, 5.454 | 5.495 | 5.518 | yes | 15.762, 15.769, 15.890, 15.850, 15.723 | 15.769 | 15.890 | no |
| adwaita Light | `Adwaita` | 18.345, 18.303, 18.664, 18.422, 18.381 | 18.381 | 18.664 | yes | 17.595, 17.679, 17.705, 17.672, 17.655 | 17.672 | 17.705 | **yes** |
| adwaita Dark | none | 4.466, 4.484, 4.481, 4.493, 4.460 | 4.481 | 4.493 | yes | 17.601, 17.795, 17.731, 17.715, 17.579 | 17.715 | 17.795 | no |
| the system's theme (kde-breeze, Dark) | none | 5.498, 5.500, 5.503, 5.476, 5.488 | 5.498 | 5.503 | yes | 15.777, 15.929, 15.847, 15.794, 15.795 | 15.795 | 15.929 | no |

## 8. The slots that take an `Icon` alone

### `breeze`

| IconName | Source | One colour | Hash | Lost |
|---|---|---|---|---|
| Eye | the table | yes (deviation 192 / 3812) | `1dc8484e189a2ae1` | no |
| CircleCheck | role `DialogSuccess` | yes (deviation 192 / 3812) | `e022555e435e1f81` | no |
| ALargeSmall | the table | yes (deviation 224 / 3812) | `e19e6c42696770aa` | no |
| Inbox | the table | yes (deviation 192 / 3812) | `374afc69ce7b513b` | no |
| Bell | role `Notification` | yes (deviation 222 / 3812) | `884ff976d7ae02c0` | no |
| CaseSensitive | the table | no: several colours (deviation 43232 / 3812) | `c40671b6ef2fbda1` | **yes** |
| LayoutDashboard | the table | yes (deviation 64 / 3812) | `a7c1011c765d5399` | no |
| GalleryVerticalEnd | the table | yes (deviation 192 / 3812) | `af165021e10806db` | no |
| ChartPie | the table | yes (deviation 192 / 3812) | `dec0a360557f800d` | no |
| Star | the table | yes (deviation 160 / 3812) | `b89b9ad88781d92e` | no |
| Palette | the table | yes (deviation 192 / 3812) | `78637aebe2c9156d` | no |
| Settings | role `ActionSettings` | yes (deviation 192 / 3812) | `e1d5125229ac97a7` | no |
| Sun | the table | yes (deviation 424 / 3812) | `4f620d3f7bb4bb0e` | no |
| Moon | the table | yes (deviation 570 / 3812) | `341efe5e9f225ade` | no |
| CircleX | role `DialogError` | no: several colours (deviation 47685 / 3812) | `24927e71c687bd88` | **yes** |
| Check | role `StatusCheck` | yes (deviation 192 / 3812) | `e022555e435e1f81` | no |
| Search | role `ActionSearch` | yes (deviation 160 / 3812) | `ff9666c43d679605` | no |
| Delete | role `ActionDelete` | yes (deviation 160 / 3812) | `ed98fe8d2f0c27ad` | no |
| Loader | role `StatusBusy` | yes (deviation 0 / 3812) | `d75f6815f779e541` | no |
| Heart | the table | yes (deviation 160 / 3812) | `19cd0488306aaded` | no |
| Copy | role `ActionCopy` | yes (deviation 192 / 3812) | `104e9cc4f434dedf` | no |
| ChevronDown | role `NavDown` | yes (deviation 128 / 3812) | `3280a69a3d9440bd` | no |
| ChevronRight | role `NavForward` | yes (deviation 128 / 3812) | `7926862058791def` | no |

### `breeze-dark`

| IconName | Source | One colour | Hash | Lost |
|---|---|---|---|---|
| Eye | the table | yes (deviation 239 / 3812) | `5cdc5b6fc90a949f` | no |
| CircleCheck | role `DialogSuccess` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | no |
| ALargeSmall | the table | yes (deviation 240 / 3812) | `e6bd6c080071176c` | no |
| Inbox | the table | yes (deviation 192 / 3812) | `b74352ca4d75bfcd` | no |
| Bell | role `Notification` | yes (deviation 222 / 3812) | `4aabf14a414b7316` | no |
| CaseSensitive | the table | no: several colours (deviation 42560 / 3812) | `9c6ed2d9c3a22f75` | **yes** |
| LayoutDashboard | the table | yes (deviation 0 / 3812) | `ed912250c2305bc7` | no |
| GalleryVerticalEnd | the table | yes (deviation 189 / 3812) | `9c5f22f4c6a786e1` | no |
| ChartPie | the table | yes (deviation 239 / 3812) | `9bfac420ee835bb7` | no |
| Star | the table | yes (deviation 96 / 3812) | `68ce347ab0846ad0` | no |
| Palette | the table | yes (deviation 240 / 3812) | `2d804cb91f80da4b` | no |
| Settings | role `ActionSettings` | yes (deviation 239 / 3812) | `3d430d0a83862bbd` | no |
| Sun | the table | yes (deviation 125 / 3812) | `83f4080da35a24aa` | no |
| Moon | the table | yes (deviation 240 / 3812) | `9c352462d523de4e` | no |
| CircleX | role `DialogError` | no: several colours (deviation 47685 / 3812) | `1f9e83b1c2409605` | **yes** |
| Check | role `StatusCheck` | yes (deviation 239 / 3812) | `553a50e52c12bb0b` | no |
| Search | role `ActionSearch` | yes (deviation 126 / 3812) | `8744e655c2478b83` | no |
| Delete | role `ActionDelete` | yes (deviation 160 / 3812) | `b2405516db73293c` | no |
| Loader | role `StatusBusy` | yes (deviation 0 / 3812) | `3acfd1544910e8da` | no |
| Heart | the table | yes (deviation 64 / 3812) | `342a7d3826c31b57` | no |
| Copy | role `ActionCopy` | yes (deviation 239 / 3812) | `3c59a9b43a1f2973` | no |
| ChevronDown | role `NavDown` | yes (deviation 64 / 3812) | `c6d8c47f59a6d1e7` | no |
| ChevronRight | role `NavForward` | yes (deviation 64 / 3812) | `11622b3c81512089` | no |

### `Adwaita`

| IconName | Source | One colour | Hash | Lost |
|---|---|---|---|---|
| Eye | the table | yes (deviation 208 / 3812) | `c1bdb57aad7a6f23` | no |
| CircleCheck | role `DialogSuccess` | yes (deviation 239 / 3812) | `fb52cb18941178fd` | no |
| ALargeSmall | the table | yes (deviation 239 / 3812) | `955aa8a6b33d1e3a` | no |
| Inbox | the table | yes (deviation 208 / 3812) | `0aec59eb94d2c46d` | no |
| Bell | the table | yes (deviation 239 / 3812) | `090f511aeca8a677` | no |
| CaseSensitive | the table | yes (deviation 239 / 3812) | `450f64abbc9643a9` | no |
| LayoutDashboard | the table | yes (deviation 208 / 3812) | `850dcb91289bc96c` | no |
| GalleryVerticalEnd | the table | yes (deviation 239 / 3812) | `1b78856ccbc36756` | no |
| ChartPie | the table | yes (deviation 239 / 3812) | `367c12e233629bb9` | no |
| Star | the table | yes (deviation 186 / 3812) | `8f3d23aabe67daf5` | no |
| Palette | the table | yes (deviation 239 / 3812) | `441ba69b431dc649` | no |
| Settings | role `ActionSettings` | yes (deviation 239 / 3812) | `61c3c39edb7a9394` | no |
| Sun | the table | yes (deviation 208 / 3812) | `709e99ff89923203` | no |
| Moon | the table | yes (deviation 208 / 3812) | `4e6384f8958c6cbb` | no |
| CircleX | role `DialogError` | yes (deviation 239 / 3812) | `b3c8f743bdb3599d` | no |
| Check | role `StatusCheck` | no: pixels | `b88c60508fb682f0` | pixels: drawn as nothing today too, not counted |
| Search | role `ActionSearch` | yes (deviation 239 / 3812) | `82dda30d7722d037` | no |
| Delete | role `ActionDelete` | yes (deviation 239 / 3812) | `b06b93cc0394eff2` | no |
| Loader | role `StatusBusy` | yes (deviation 64 / 3812) | `55cb55da563899ab` | no |
| Heart | the table | yes (deviation 208 / 3812) | `7b2227b686e04cc1` | no |
| Copy | role `ActionCopy` | yes (deviation 239 / 3812) | `4247f45a507be13d` | no |
| ChevronDown | role `NavDown` | yes (deviation 208 / 3812) | `bb9ab942bcd4f172` | no |
| ChevronRight | role `NavForward` | yes (deviation 208 / 3812) | `389c4eb99932b834` | no |

## 9. The conditions

| # | Condition | Verdict |
|---|---|---|
| 1 | an essential missing at 16 px | not met |
| 2 | more than half of the 106 empty at 16 px (≥ 54) | not met |
| 3 | the four names' first name found does not draw the glyph | by eye, from the `<theme>-four.png` sheets (item 4); served at 16 px: `breeze` Check: `dialog-ok` as `dialog-ok-symbolic`; `breeze` CircleCheck: `emblem-ok-symbolic` (symbolic, as named); `breeze` Loader: `process-working-symbolic` (symbolic, as named); `breeze` LoaderCircle: `process-working-symbolic` (symbolic, as named); `breeze-dark` Check: `dialog-ok` as `dialog-ok-symbolic`; `breeze-dark` CircleCheck: `emblem-ok-symbolic` (symbolic, as named); `breeze-dark` Loader: `process-working-symbolic` (symbolic, as named); `breeze-dark` LoaderCircle: `process-working-symbolic` (symbolic, as named); `Adwaita` Check: `object-select` as `object-select-symbolic`; `Adwaita` CircleCheck: `object-select` as `object-select-symbolic`; `Adwaita` Loader: `content-loading` as `content-loading-symbolic`; `Adwaita` LoaderCircle: `content-loading` as `content-loading-symbolic` |
| 4 | `Loader` or `LoaderCircle` a strip, or no `process-working-symbolic` | not met |
| 5 | W9's size condition | not met |
| 6 | a slot that takes an `Icon` alone would lose its icon | **met**: `breeze` CaseSensitive (no: several colours); `breeze` CircleX (no: several colours); `breeze-dark` CaseSensitive (no: several colours); `breeze-dark` CircleX (no: several colours) |
| 7 | the handle's median above the showcase's in a step | **met**: kde-breeze Light; adwaita Light |
