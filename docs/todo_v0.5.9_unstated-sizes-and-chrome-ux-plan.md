# v0.5.9: unstated sizes, and the showcase's chrome UX — plan

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development. Every subagent runs on Opus (the maintainer's instruction). Implementers go through `subagent_type: implement`.

**Goal:** a sizing value no platform states stays absent from resolver to toolkit, native presets state every value their platform documents, and the showcase's panel toggles and theme controls sit where users expect them.

**Architecture:**
- The resolved model moves from `f32` to `Option<f32>` for padding and `toolbar.bar_height`.
- Builders refine a property only when its value is stated.
- A table-driven gate checks the native presets against platform-facts.
- The showcase's chrome is rearranged. Panel toggles go to the status bar (Zed-style), the theme settings go into the Sidebar header, and the title carries the version.

**Spec:** `docs/todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`

## Global Constraints

- NEVER LIE, NEVER INVENT.
  - Every preset value cites a platform-facts line.
  - Every showcase claim cites a line that was read.
  - A value without a source is absent, never approximated.
- No runtime panics, no `unsafe`, no hardcoded theme values.
- Never mix icon sets. A missing icon is `None`, never a substitute.
- Pre-1.0: breaking type changes are allowed, and there are no migration docs.
- Every task ends with a green `env CARGO_BUILD_JOBS=4 ./pre-release-check.sh` and one commit of named files.
  - The expected release-time warnings are fine.
  - No `git add -A`.
  - No Co-Authored-By or AI-attribution lines.
  - Push nothing and tag nothing.
- Every new gate or test ships a seed-and-fail proof.
- docs/todo.md is append-only, except when closing or updating a named item.

---

### Task 1: Audit native presets' sizing values against platform-facts (read-only)

**Output:** `.superpowers/sdd/<plan>/audit-sizing.md` with one table row per mismatch. The columns are: preset, variant, widget.field, preset value, platform-facts value, platform-facts line, and class. The classes are:
- (a) documented but missing;
- (b) stated where facts say (none);
- (c) different value;
- (d) asymmetric.

**Scope:** every sizing field (`_px` and dimension fields) of `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, including their `-live` twins. Report `ios` separately if platform-facts covers it.

**Gate:** none, because this is research. The controller rules on each row. The rulings are ledgered and feed Tasks 3 and 5.

### Task 2: Research status-bar padding (spec §1.5)

- Fetch the upstream sources: Qt `qstatusbar.cpp` plus Breeze's metrics, GTK3 Adwaita's `_common.scss` statusbar rules, Win32/WinUI, and AppKit.
- Append the `border.padding_*` rows to platform-facts §2.14 with citations. A platform with no stated value gets **(none)** and a reason.
- Commit `docs(facts): the status bar's padding, per platform`.

### Task 3: The model can say "not stated" (spec §1.1–§1.2)

1. **Failing tests first.**
   - `kde-breeze` resolves `dialog.border.padding_horizontal` to `None` today. It gets its value in Task 5; until then the test asserts `None`, not `0.0`.
   - A community preset without paddings resolves them to `None`.
   - `toolbar.bar_height` is `None` where absent.
2. **Implement.**
   - Change the resolved padding fields, `toolbar.bar_height`, and every field the Task 1 rulings add, to `Option<f32>`.
   - Remove the `unwrap_or_default` fallbacks (validate_helpers.rs:283, :337, :587; resolved.rs:297).
   - Fix every internal consumer and validation.
   - Update the docs: border.rs, property-registry.toml, and inheritance-rules.toml:58-59.
3. `cargo test -p native-theme`, then pre-release-check.
4. Commit `feat(model): a size the platform does not state stays unstated`.

### Task 4: Connectors apply only what is stated (spec §2)

1. **Failing tests.**
   - For each gpui padding builder, and for `toolbar` `min_h`: with `None`, the refinement leaves that property unset.
   - The iced `button_padding`/`input_padding` return `None` when unstated.
2. **Implement.**
   - gpui `geometry.rs`: `control_height`, `lib.rs:~472`, the seams test, the README builder table, and the showcase `GEOMETRY_NOTES` and infos that describe padding.
   - iced: the functions return `Option<Padding>`, and the iced showcase follows.
3. pre-release-check.
4. Commit `feat(connectors): builders refine only what the theme states`.

### Task 5: Native presets state what their platform documents (spec §1.3–§1.4)

1. **Write the gate.**
   - `native_presets_state_documented_sizes`: a table covering every §1.1 field for every native preset and variant. Each row holds `Some(v)`/`None` and a platform-facts citation. It includes the §1.5 status-bar rows from Task 2.
   - Watch it fail on today's presets.
2. **Edit the presets.**
   - Set the values the table and the Task 1 rulings require.
   - Remove the stated-but-(none) values, e.g. KDE `bar_height`.
   - Each value gets a comment citing platform-facts.
3. **Seed proof:** remove the KDE dialog padding and watch the gate fail naming the row. Then restore it.
4. pre-release-check. Also run the preset-validator's checklist by hand: `docs/property-registry.toml` plus platform-facts.
5. Commit `fix(presets): native presets state their platform's documented sizes`.

### Task 6: Status bar holds the panel toggles (spec §3.1, §3.4)

1. **Failing tests.**
   - The left toggle sits at the status bar's left end and the inspector toggle at its right end.
   - Clicking the left toggle collapses the Sidebar to its rail; clicking the inspector toggle hides the inspector.
   - Each toggle is selected while its panel is open.
   - Each toggle's icon comes from the chosen set.
   - The title reads `native-theme-gpui <version> showcase`.
   - The status bar no longer carries the version.
2. **Implement.**
   - Add `demo::` helpers with infos.
   - Remove the SidebarToggleButton and the inspector button from the toolbar.
   - Add the `showcase-exceptions.toml` entry.
   - Move the `the_toolbar_is_the_models_toolbar` gap measurement to the new first two toolbar children.
3. pre-release-check.
4. Commit `feat(showcase): panel toggles in the status bar; the title names the version`.

### Task 7: Theme settings in the Sidebar header (spec §3.2–§3.3)

1. **Failing tests.**
   - The three labelled rows are in the Sidebar header and are absent in the rail.
   - `the_toolbar_switches_the_preset`, the colour-mode test and the icon-set test drive the controls in their new place.
   - The toolbar holds exactly Command Palette, Reload Theme and Preferences.
2. **Implement.** Move the three controls, add the labels (`demo::label`) and write their infos.
3. pre-release-check.
4. Commit `feat(showcase): theme settings live, labelled, in the Sidebar`.

### Task 8: Sidebar icons no longer overlap (spec §3.5)

1. **Failing test.** The icon and label bounds of every Sidebar item never intersect, expanded and collapsed, under two icon sets.
2. Diagnose against upstream `SidebarMenuItem` (sidebar/menu.rs) and fix the cause.
3. pre-release-check.
4. Commit `fix(showcase): Sidebar icons keep to their slot`.

### Task 9: Docs and archive (spec §4)

- Update the CHANGELOG.
- Append to docs/todo.md:
  - per-side padding (D3);
  - open audit rows;
  - a screenshot-review item for the changed chrome.
- Move the three documents to `docs/archive/` and fix the links.
- Commit `docs: unstated sizes and chrome UX, implemented and archived`.
