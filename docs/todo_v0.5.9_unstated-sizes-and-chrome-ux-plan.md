# v0.5.9: unstated sizes, and the showcase's chrome UX — plan

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development. Every subagent runs on Opus, as the maintainer instructed. Implementers use `subagent_type: implement`.

**Goal:**
- A sizing value no platform states stays absent from the resolver to the toolkit.
- Padding is stated per side.
- Native presets and readers state every value their platform documents.
- The showcase's panel toggles and theme controls sit where users expect them.

**Spec:** `docs/todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`

## Global Constraints

- NEVER LIE, NEVER INVENT:
  - Every preset or reader value cites a platform-facts line.
  - Every showcase claim cites a line that was read.
  - A value without a source is absent, never approximated.
- No runtime panics, no `unsafe`, no hardcoded theme values.
- Never mix icon sets. A missing icon is `None`, never a substitute.
- Before 1.0, breaking type changes are allowed, and there are no migration docs.
- Every task ends green and committed:
  - A green `env CARGO_BUILD_JOBS=4 ./pre-release-check.sh`. The expected release-time warnings are fine.
  - One commit of named files. No `git add -A`, and no Co-Authored-By or AI-attribution lines.
  - Push nothing, tag nothing.
- Every new gate or test ships a seed-and-fail proof.
- `docs/todo.md` is append-only, except for closing or updating a named item.

---

### Task 1: Audit sizing values against platform-facts (read-only)

- **Output:** `.superpowers/sdd/<plan>/audit-sizing.md`, one row per finding. The columns are:
  - source: the preset and variant, or the reader's file:line;
  - widget.field (per side for padding);
  - stated value;
  - platform-facts value;
  - platform-facts line;
  - class.
- **Classes:**
  - (a) documented but missing
  - (b) stated where the facts say (none) or give no row
  - (c) different value
  - (d) asymmetric
  - (e) a range
  - (f) per-context
- **Scope:** every sizing field (`_px` and dimension fields) in:
  - the native presets `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, and their `-live` twins;
  - the OS readers `kde/metrics.rs`, `windows.rs`, `macos.rs` and the GNOME reader.

  Report `ios` separately, because platform-facts has no iOS column.
- **No gate.** This is research. The controller rules on every row, per spec §1.4. The rulings go in the ledger and feed Tasks 3 and 4.

### Task 2: Research status-bar padding (spec §1.6)

- Fetch the upstream sources for each platform.
- Append the per-side rows to platform-facts §2.14, with citations. A platform with no stated value gets **(none)** and a reason.
- Run the pre-release check.
- Commit `docs(facts): the status bar's padding, per platform`.

### Task 3: Unstated stays unstated, padding per side (spec §1.1–§1.3, §2)

This is one task because the model's type change and its consumers must compile together.

1. **Write the failing tests.**
   - A colour-scheme preset without toolbar, dialog or status-bar padding resolves those sides to `None`.
   - A fixture with `padding_vertical_px = 0.0` resolves top and bottom to `Some(0.0)`.
   - A fixture with `padding_horizontal_px = 10` and `padding_right_px = 6` resolves to left 10, right 6. The same holds whichever of preset and overlay supplies which key.
   - `toolbar.bar_height` is `None` where absent.
   - Each gpui padding builder, and `geometry::toolbar`'s `min_h`, leaves an unstated side or height unset.
   - `control_height` and `tooltip_content` use upstream's mirrored padding for an unstated side.
   - iced's `button_padding` and `input_padding` fill an unstated side from iced's `DEFAULT_PADDING`.
   - gpui's `dialog_content_padding` returns the per-side padding.
2. **Implement.**
   - Model and resolver: spec §1.1–§1.3, including the fields the Task 1 rulings add.
   - Registry: `property-registry.toml` and the platform-facts conventions paragraph.
   - Stale rules: the two stale rules entries.
   - gpui: spec §2.1.
   - iced: spec §2.2, plus the iced showcase.
   - README builder table.
   - The showcase's `GEOMETRY_NOTES` and infos.
3. Run `cargo test --workspace`, then the pre-release check.
4. Commit `feat(model): padding per side; a size the platform does not state stays unstated`.

### Task 4: Native presets and readers state what their platform documents (spec §1.4–§1.5)

1. **Write the gate** `native_presets_state_documented_sizes` (spec §1.5). Include the Task 2 status-bar rows. Watch it fail on today's presets.
2. **Edit the presets and readers** to match the table and the Task 1 rulings.
   - Replace the chosen numbers for asymmetric rows (e.g. Windows tooltip 7) with the documented sides.
   - Remove values stated where the facts say (none), e.g. KDE `bar_height`.
   - Give every value a comment citing platform-facts.
   - Correct platform-facts where a ruling found it wrong.
3. **Seed proof:** remove the KDE dialog padding, watch the gate fail naming the row, then restore it.
4. Run the pre-release check.
5. Commit `fix(presets): native presets and readers state their platform's documented sizes`.

### Task 5: Sidebar icons keep to their slot (spec §3.5)

1. **Failing test:** icon and label bounds never intersect, for every Sidebar item, expanded and collapsed, under two icon sets.
2. **Diagnose** the overlap against upstream `SidebarMenuItem` (sidebar/menu.rs) and fix its cause.
3. Run the pre-release check.
4. Commit `fix(showcase): Sidebar icons keep to their slot`.

### Task 6: Panel toggles in the status bar; the title names the version (spec §3.1, §3.4)

1. **Failing tests:**
   - The left toggle sits at the status bar's left end, and the inspector toggle at its right end.
   - Clicking the left toggle collapses the Sidebar to its rail.
   - Clicking the inspector toggle hides the inspector.
   - Each toggle is selected while its panel is open.
   - Each toggle's icon comes from the chosen set.
   - The title-bar label and the OS window title both read `native-theme-gpui <version> showcase`.
   - The status bar no longer carries the version.
2. **Implement.**
   - Add the `demo::` helpers and their infos.
   - Remove the SidebarToggleButton and the inspector button from the toolbar.
   - Add the `showcase-exceptions.toml` entry.
   - Re-point `the_toolbar_is_the_models_toolbar` to the toolbar's new first two children.
3. Run the pre-release check.
4. Commit `feat(showcase): panel toggles in the status bar; the title names the version`.

### Task 7: Theme settings in the Sidebar header (spec §3.2–§3.3)

1. **Failing tests:**
   - The three labelled rows are in the Sidebar header.
   - They fit at `NAV_WIDTH`.
   - They are absent in the rail.
   - The preset, mode and icon-set tests drive the controls in their new place.
   - The toolbar holds exactly Command Palette, Reload Theme and Preferences.
2. **Implement:** move the three controls, add their `demo::label` labels, and add the infos.
3. Run the pre-release check.
4. Commit `feat(showcase): theme settings live, labelled, in the Sidebar`.

### Task 8: Docs and archive (spec §4)

- Update the CHANGELOG, including the earlier `[Unreleased]` entries that describe the old toolbar, status bar and title.
- Update the connector README.
- Update the screenshot item and append the new items in `docs/todo.md`.
- Move the three documents to `docs/archive/` and fix the links.
- Commit `docs: unstated sizes and chrome UX, implemented and archived`.
