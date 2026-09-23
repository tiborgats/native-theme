# v0.5.9: unstated sizes, and the showcase's chrome UX — plan

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development.
>
> - Every subagent runs on Opus. The maintainer asked for this explicitly, and it overrides the global tiered-dispatch table.
> - Implementers use `subagent_type: implement`.
> - Tasks 1 and 2 are research without a mechanical gate. The controller checks their findings and rules on each one before any later task relies on them.

**Goal:**

- A sizing value no platform states stays absent all the way from the resolver to the toolkit.
- Padding is stated per side.
- For padding and `toolbar.bar_height`, native themes, whether static or live, state every value their platform documents.
- The showcase's panel toggles and theme controls sit where users expect them.

**Spec:** `docs/todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`

## Global Constraints

- **NEVER LIE, NEVER INVENT.**
  - Every preset or reader value cites a platform-facts line.
  - Every showcase claim cites a line that was read.
  - A value without a source is absent, never approximated.
  - A showcase constant used where the theme states nothing is labelled as the showcase's own.
- No runtime panics, no `unsafe`, no hardcoded theme values.
- Never mix icon sets. A missing icon is `None`, never a substitute.
- Before 1.0, breaking changes are allowed, and there are no migration docs.
- Every task ends with:
  - a green `env CARGO_BUILD_JOBS=4 ./pre-release-check.sh` (the expected release-time warnings are fine);
  - one commit of named files. No `git add -A`, no Co-Authored-By or AI-attribution lines. Push nothing, tag nothing.
- Every new gate or test ships a seed-and-fail proof.
- `docs/todo.md` is append-only, except for closing or updating a named item.

---

### Task 1: Audit sizing values against platform-facts (read-only)

**Output:** `.superpowers/sdd/<plan>/audit-sizing.md`, with two tables.

**Table A, the fields in this plan's scope** (padding sides and `toolbar.bar_height`). One row per finding, with these columns:

- source: the preset and variant, or the reader's file:line;
- widget.field and side;
- stated value;
- platform-facts cell;
- platform-facts line;
- class.

The classes follow spec §1.4:

- (a) documented but missing;
- (b) stated but not documented;
- (c) different;
- (d) asymmetric;
- (e) range;
- (f) per-context;
- (g) derivation;
- (h) "(none)" that means zero;
- (i) "(none)" that means unstated.

**Table B, every other sizing field** that some native platform leaves unstated while a native preset or reader states it. Examples: KDE `min_height`, `row_height`, dialog bounds, tooltip `max_width`. This table feeds the follow-up item; nothing in this plan changes those fields.

**Scope:**

- the native presets `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, with their `-live` twins;
- the reader constant functions named in spec §1.4;
- the colour-scheme presets' `bar_height`.

Report `ios` separately, because platform-facts has no iOS column.

**Gate:** none. The controller rules on every Table A row. The rulings are ledgered and feed Tasks 3 and 4.

### Task 2: Research the status-bar padding (spec §1.7)

- Fetch the upstream sources for each platform.
- Append the per-side rows to platform-facts §2.14, with citations. A platform without a value gets **(none)** and its reason.
- The controller checks every citation.
- Run pre-release-check.
- Commit `docs(facts): the status bar's padding, per platform`.

### Task 3: Per-side padding; unstated stays unstated (spec §1.1–§1.3, §2)

This is one task: the model's type changes and their consumers must compile together.

1. **Failing tests.**
   - Parsing:
     - `padding_horizontal_px = 10` gives left and right `Some(10)`, and top and bottom `None`.
     - `padding_vertical_px = 0.0` gives top and bottom `Some(0.0)`.
     - A table stating `padding_horizontal_px` together with `padding_left_px` is rejected, naming both keys.
   - Merging: a reader's left side over a preset's shorthand wins for left only.
   - Resolution:
     - A colour-scheme preset resolves its unstated toolbar, dialog and status-bar sides to `None`.
     - A negative side is a validation error.
     - `toolbar.bar_height` is `None` where absent.
   - gpui:
     - Each padding builder leaves unstated sides unset.
     - `geometry::toolbar` leaves `min_h` unset without a `bar_height`.
     - `geometry::button` and `geometry::input` set `min_h` and `h_auto`.
     - A seams test measures a real Button and a real Input: at scale 1 their height is the minimum, and at a large scale the text does not clip.
     - `tooltip_content` uses upstream's rem padding for an unstated side.
   - iced: `button_padding` and `input_padding` fill unstated sides from `DEFAULT_PADDING`.
2. **Implement** spec §1.1–§1.3 and §2:
   - the split border types;
   - the readers setting sides;
   - the registry and platform-facts conventions;
   - the stale rules;
   - every consumer and test listed in §1.3;
   - gpui and iced;
   - the iced showcase;
   - the README builder table;
   - the gpui showcase's `GEOMETRY_NOTES`, infos and Theme Map rows.
3. Run `cargo test --workspace`, then pre-release-check.
4. Commit `feat(model): padding per side; a size the platform does not state stays unstated`.

### Task 4: Native themes state what their platform documents (spec §1.4–§1.6)

1. **Move the reader constants** out of the feature and OS gates (spec §1.5), with no change in value.
2. **Write the gate** `native_themes_state_documented_sizes` (spec §1.6), including Task 2's status-bar rows. Watch it fail on today's presets and readers.
3. **Edit the presets and readers** to match the table and the Task 1 rulings:
   - Replace the chosen numbers for asymmetric rows (Windows tooltip 7, and others) with the documented sides.
   - Remove KDE's `bar_height` and the colour-scheme presets' `bar_height_px = 40.0`.
   - Correct `kde/metrics.rs:20` and the Windows reader's `toolbar.item_gap` as ruled.
   - Give every value a comment citing platform-facts.
   - Correct platform-facts wherever a ruling found it wrong.
4. **Seed proofs** (spec §1.6).
5. Run pre-release-check.
6. Commit `fix(presets): native themes state their platform's documented sizes`.

### Task 5: Sidebar icons fit their items (spec §3.5)

1. **Failing test** (spec §3.5), under every native preset and one colour-scheme preset, expanded and in the rail.
2. **Implement:** `icon_size_small` for the page icons, and their info.
3. Run pre-release-check.
4. Commit `fix(showcase): Sidebar icons use the platform's small icon size`.

### Task 6: The chrome, rearranged (spec §3.1–§3.4, §3.6)

1. **Failing tests:**
   - Status bar:
     - The left toggle sits at the status bar's left end, and the inspector toggle at its right end.
     - The left toggle collapses the Sidebar to its rail, and the right toggle hides the inspector.
     - Each toggle is selected while its panel is open, and takes its icon from the chosen set.
     - The status bar no longer carries the version.
   - Sidebar header:
     - The three labelled rows sit in the Sidebar header, fit at `NAV_WIDTH`, and are absent in the rail.
     - The preset, mode and icon-set tests drive the controls in their new place.
   - Toolbar:
     - The toolbar holds exactly Command Palette, Reload Theme and Preferences.
     - The toolbar test measures its first two children.
     - The toolbar is padded under a colour-scheme preset, and its info names the showcase's own constant.
   - Title:
     - The title-bar label, the OS window title and the screenshot lookup string all equal `native-theme-gpui <version> showcase`.
   - End to end, under `kde-breeze`:
     - The status bar's first and last children are inset by the drawn padding.
     - The About content is inset by 10px.
2. **Implement** spec §3.1–§3.4:
   - the `demo::` helpers and their infos;
   - the showcase's own named constants;
   - the popover and hover-card content padding;
   - `showcase-exceptions.toml`;
   - `chrome_icon_names()`.
3. Run pre-release-check.
4. Commit `feat(showcase): panel toggles in the status bar, theme settings in the Sidebar, the version in the title`.

### Task 7: Docs and archive (spec §4)

- Update the CHANGELOG. This includes rewriting the earlier `[Unreleased]` entries about the toolbar, status bar and title.
- Update the connector README.
- In `docs/todo.md`, close the toolbar item, update the screenshot item and append the new items.
- Append a note on the new model to the other plan documents.
- Move the three documents to `docs/archive/` and fix their links.
- Commit `docs: unstated sizes and chrome UX, implemented and archived`.
