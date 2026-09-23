# v0.5.9: unstated sizes, and the showcase's chrome UX — plan

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development. Every subagent runs on Opus, per the maintainer's instruction; implementers use `subagent_type: implement`.

**Goal:** A sizing value that no platform states stays absent from the resolver all the way to the toolkit. Native presets and readers state every value their platform documents. The showcase's panel toggles and theme controls sit where users expect them.

**Spec:** `docs/todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`

## Global Constraints

- NEVER LIE, NEVER INVENT.
  - Every preset or reader value cites a platform-facts line.
  - Every showcase claim cites a line that was read.
  - A value without a source is absent, never approximated.
- No runtime panics, no `unsafe`, and no hardcoded theme values.
- Never mix icon sets. A missing icon is `None`, never a substitute.
- Pre-1.0: breaking type changes are allowed, and there are no migration docs.
- Every task that changes code or docs ends with a green `env CARGO_BUILD_JOBS=4 ./pre-release-check.sh`. The expected release-time warnings are fine.
- Every task ends with one commit of named files: no `git add -A`, no Co-Authored-By or AI-attribution lines. Push nothing, tag nothing.
- Every new gate or test ships a seed-and-fail proof.
- `docs/todo.md` is append-only, except for closing or updating a named item.

---

### Task 1: Audit sizing values against platform-facts (read-only)

- **Output:** `.superpowers/sdd/<plan>/audit-sizing.md`, one row per mismatch. Columns: source (preset and variant, or reader file:line), widget.field, stated value, platform-facts value, platform-facts line, class.
- **Classes:**
  - (a) documented but missing;
  - (b) stated where the facts say (none) or give no row;
  - (c) different value;
  - (d) asymmetric.
- **Scope:** every sizing field (`_px` and dimension fields) in:
  - the native presets `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, with their `-live` twins;
  - the OS readers `kde/metrics.rs`, `windows.rs`, `macos.rs` and the GNOME reader.

  Report `ios` separately: platform-facts has no iOS column.
- **Gate:** none, because this is research. The controller rules on each row. The rulings go in the ledger and feed Tasks 3 and 4.

### Task 2: Research status-bar padding (spec §1.5)

- Fetch the upstream sources for each platform.
- Append the `border.padding_*` rows to platform-facts §2.14, with citations. A platform with no stated value gets **(none)** and a reason.
- Run pre-release-check.
- Commit `docs(facts): the status bar's padding, per platform`.

### Task 3: A size the platform does not state stays unstated (spec §1.1–§1.2, §2)

This is one task because the model's type change and its consumers must compile together.

1. **Write the failing tests.**
   - A colour-scheme preset without toolbar, dialog or status-bar padding resolves them to `None`.
   - A TOML fixture that states `padding_vertical_px = 0.0` resolves to `Some(0.0)`.
   - `toolbar.bar_height` is `None` where it is absent.
   - For every gpui padding builder and for `geometry::toolbar`'s `min_h`: given `None`, the refinement leaves that property unset.
   - `control_height` and `tooltip_content` use upstream's mirrored padding when none is stated.
   - iced's `button_padding`/`input_padding` and gpui's `dialog_content_padding` return `None` when the value is unstated.
2. **Implement.**
   - Model and resolver: spec §1.1–§1.2, including any fields the Task 1 rulings add.
   - The two stale rules documents (`inheritance-rules.toml:58-59` and `inheritance.rs:350-351`).
   - gpui: spec §2.1.
   - iced: spec §2.2, plus the iced showcase.
   - The README builder table.
   - The showcase's `GEOMETRY_NOTES` and infos.
3. Run `cargo test --workspace`, then pre-release-check.
4. Commit `feat(model): a size the platform does not state stays unstated`.

### Task 4: Native presets and readers state what their platform documents (spec §1.3–§1.4)

1. **Write the gate** `native_presets_state_documented_sizes` (spec §1.4). It includes the status-bar rows from Task 2. Watch it fail on today's presets.
2. **Edit the presets and readers** to match the table and the Task 1 rulings.
   - Remove the values stated where the facts say (none). For example, remove KDE's `bar_height`.
   - Give each preset value a comment citing platform-facts.
   - Fix each reader constant together with its comment.
3. **Seed proof.** Remove the KDE dialog padding, watch the gate fail naming the row, then restore it.
4. Run pre-release-check. Also check by hand against `docs/property-registry.toml` and platform-facts.
5. Commit `fix(presets): native presets and readers state their platform's documented sizes`.

### Task 5: Sidebar icons keep to their slot (spec §3.5)

1. **Failing test:** icon and label bounds never intersect, for every Sidebar item, expanded and collapsed, under two icon sets.
2. **Diagnose** the overlap against upstream `SidebarMenuItem` (sidebar/menu.rs) and fix its cause.
3. Run pre-release-check.
4. Commit `fix(showcase): Sidebar icons keep to their slot`.

### Task 6: Panel toggles in the status bar; the title names the version (spec §3.1, §3.4)

1. **Failing tests:**
   - The left toggle sits at the left end of the status bar and the inspector toggle at its right end.
   - Clicking the left toggle collapses the Sidebar to its rail; clicking the inspector toggle hides the inspector.
   - Each toggle is selected while its panel is open.
   - Each toggle's icon comes from the chosen set.
   - The title-bar label and the OS window title both read `native-theme-gpui <version> showcase`.
   - The status bar no longer carries the version.
2. **Implement.**
   - Add the `demo::` helpers and their infos.
   - Remove the SidebarToggleButton and the inspector button from the toolbar.
   - Add the `showcase-exceptions.toml` entry.
   - Re-point `the_toolbar_is_the_models_toolbar` to the toolbar's new first two children.
3. Run pre-release-check.
4. Commit `feat(showcase): panel toggles in the status bar; the title names the version`.

### Task 7: Theme settings in the Sidebar header (spec §3.2–§3.3)

1. **Failing tests:**
   - The three labelled rows are in the Sidebar header, fit at `NAV_WIDTH`, and are absent in the rail.
   - The preset, colour-mode and icon-set tests drive the controls in their new place.
   - The toolbar holds exactly Command Palette, Reload Theme and Preferences.
2. **Implement.**
   - Move the three controls.
   - Add the labels with `demo::label`.
   - Add the infos.
3. Run pre-release-check.
4. Commit `feat(showcase): theme settings live, labelled, in the Sidebar`.

### Task 8: Docs and archive (spec §4)

- Update the CHANGELOG.
- Append the `docs/todo.md` items.
- Move the three documents to `docs/archive/` and fix their links.
- Commit `docs: unstated sizes and chrome UX, implemented and archived`.
