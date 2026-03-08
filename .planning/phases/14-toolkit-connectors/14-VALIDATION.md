# Phase 14: Toolkit Connectors - Validation

## Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` (built-in) |
| Config file | Workspace Cargo.toml |
| Quick run command | `cargo test -p native-theme-iced` / `cargo test -p native-theme-gpui` |
| Full suite command | `cargo test --workspace` |

## Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CONN-01 | ThemeColors -> ThemeColor 108 fields | unit | `cargo test -p native-theme-gpui colors` | Wave 0 |
| CONN-02 | Fonts/geometry/spacing -> ThemeConfig | unit | `cargo test -p native-theme-gpui config` | Wave 0 |
| CONN-03 | Upstream PR proposal docs | manual-only | N/A (documentation review) | Wave 0 |
| CONN-04 | showcase.rs compiles and runs | smoke | `cargo build -p native-theme-gpui --example showcase` | Wave 0 |
| CONN-05 | ThemeColors -> iced Palette | unit | `cargo test -p native-theme-iced palette` | Wave 0 |
| CONN-06 | Per-widget Catalog/Style via iced built-in Catalog over custom Palette/Extended | unit | `cargo test -p native-theme-iced` | Wave 0 |
| CONN-07 | Geometry/spacing/metrics -> widget metric helpers and Style fields | unit | `cargo test -p native-theme-iced` | Wave 0 |
| CONN-08 | demo.rs compiles and runs | smoke | `cargo build -p native-theme-iced --example demo` | Wave 0 |
| CONN-09 | Theme selector in both examples | manual-only | N/A (visual inspection) | Wave 0 |

## Sampling Rate
- **Per task commit:** `cargo test -p native-theme-{iced,gpui}`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before verification

## Wave 0 Gaps
- [ ] `connectors/native-theme-iced/src/palette.rs` tests -- covers CONN-05 (all 6 Palette fields mapped correctly)
- [ ] `connectors/native-theme-iced/src/extended.rs` tests -- covers CONN-05 (Extended overrides)
- [ ] `connectors/native-theme-iced/src/lib.rs` tests -- covers CONN-06 (built-in Catalog via custom_with_fn), CONN-07 (widget metric helpers)
- [ ] `connectors/native-theme-iced/examples/demo.rs` -- covers CONN-08, CONN-09
- [ ] `connectors/native-theme-gpui/src/colors.rs` tests -- covers CONN-01 (108 field mapping)
- [ ] `connectors/native-theme-gpui/src/config.rs` tests -- covers CONN-02 (ThemeConfig mapping)
- [ ] `connectors/native-theme-gpui/examples/showcase.rs` -- covers CONN-04, CONN-09
- [ ] `connectors/native-theme-gpui/proposals/README.md` -- covers CONN-03

## CONN-06 Implementation Note

CONN-06 ("per-widget Catalog/Style for 8 core widgets") is satisfied via iced's **built-in Catalog implementations** over the custom Palette/Extended, not through explicit custom Catalog trait impls. When `Theme::custom_with_fn()` creates a theme from a custom palette, iced's built-in `button::Catalog`, `container::Catalog`, etc. implementations automatically use that palette's colors for all 8 widget types. This is the idiomatic iced 0.14 pattern per research (Open Question #3). A separate `catalog.rs` with manual Catalog trait impls is NOT needed.

## CONN-07 Implementation Note

CONN-07 ("widget metric mapping to Style fields") is addressed via helper functions in `lib.rs` that expose geometry and widget metric values (e.g., `border_radius()`, `button_padding()`, `scrollbar_width()`). In iced's architecture, metrics like padding and min-height are set on widget instances (e.g., `button("label").padding(padding)`), not through the Catalog/Style system. The geometry `radius` value flows through the Extended palette to Style `border` fields automatically via iced's built-in Catalog impls. The helper functions are the iced-correct pattern per research (Open Question #5).
