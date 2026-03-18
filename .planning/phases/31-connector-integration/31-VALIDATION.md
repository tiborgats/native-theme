# Phase 31: Connector Integration - Validation Architecture

**Source:** Extracted from 31-RESEARCH.md Validation Architecture section

## Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | workspace Cargo.toml |
| Quick run command | `cargo test -p native-theme-gpui --lib && cargo test -p native-theme-iced --lib` |
| Full suite command | `cargo test --workspace` |

## Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CONN-01 | gpui: AnimatedIcon::Frames -> Vec<ImageSource> | unit | `cargo test -p native-theme-gpui --lib -- animated_frames` | Wave 0 |
| CONN-02 | gpui: Transform::Spin -> with_animation wrap | unit | `cargo test -p native-theme-gpui --lib -- spin` | Wave 0 |
| CONN-03 | iced: AnimatedIcon::Frames -> Vec<svg::Handle> | unit | `cargo test -p native-theme-iced --lib -- animated_frames` | Wave 0 |
| CONN-04 | iced: Transform::Spin -> rotation radians | unit | `cargo test -p native-theme-iced --lib -- spin_rotation` | Wave 0 |

## Sampling Rate
- **Per task commit:** `cargo test -p native-theme-gpui --lib && cargo test -p native-theme-iced --lib`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before verification

## Wave 0 Gaps
- [ ] `connectors/native-theme-gpui/src/icons.rs` -- tests for animated_frames_to_image_sources() and spin helper
- [ ] `connectors/native-theme-iced/src/icons.rs` -- tests for animated_frames_to_svg_handles() and spin_rotation_radians()

None of these require new test files -- tests go in the existing `#[cfg(test)] mod tests` blocks in each connector's icons.rs.
