# Material's spinner and progress-bar variant combinations (for the Material lane)

ADD-A125's checks 37 and 38 declared `exclusive_variants` groups on `material-3`'s spinner as an example for the keyframe rule; check 39 found them partial, and the twenty-ninth-round direction moved the subject out. The catalogue and the traces read as before ADD-A125. The removed text, then the checks' findings, verbatim.

## The removed text (catalogue `spinner`, Variants)

```
MAT-N24), `@contained` (its contained form, MAT-N24); `exclusive_variants`, as no source draws them together (ADD-A125): in
  `[declare.widget.spinner]` `[@four_color, @wavy]` (Compose's wavy indicator takes one colour,
  `androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/WavyProgressIndicator.kt:281-284`, `:353-355`, and material-web, the source of `four-color`, draws no wave,
  `material-web@2.5.0 progress/internal/progress.ts:40`), and in `[declare.flavor.expressive.widget.spinner]`, which
  replaces it under `flavor:expressive`, `[@four_color, @wavy, @loading_indicator]` and `[@thick, @loading_indicator]`
  (`LoadingIndicator` takes one colour and has no stroke or wave, `LoadingIndicator.kt:98-102`, `:136-139`); `@loading_indicator` and `@contained` are flavour `expressive`'s,
```

## The checks' findings, verbatim

**check-37.md, item 3**

3. **The `@four_color` example is a combination Compose does not draw, and it rests on an unstated rule order.**
   - Where: `:818-819` ("`material-3`'s `@four_color` keyframes under `@loading_indicator`").
   - Evidence:
     - Both variants set the whole loop: MAT-SN5 `spinner.values.loop @ variant @four_color`
       (`trace-material.md:433`), and MAT-N24/MAT-X12 `spinner.values.loop @ variant @loading_indicator` (`:576`,
       `:627`).
     - `keyframes` is a list, replaced whole (S6.1 `:2013-2014`). Under both variants the later rule's keyframes
       win entirely:
       - if `@loading_indicator`'s rule is later, the four-colour keyframes never reach the absent glyph;
       - if it is earlier, the loading indicator loses its shape keyframes.
     - Compose's `LoadingIndicator` takes one `color` (`androidx@e4d933c9
       compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/LoadingIndicator.kt:101`,
       `:138`, `:188`, `:230`), so no four-colour loading indicator exists.
     - The spinner declares no `exclusive_variants` group among `@wavy`, `@thick`, `@four_color` and
       `@loading_indicator` (`spec-widgets.md:2972-2976`).
   - Fix:
     - Declare `[@four_color, @loading_indicator]`, and likewise `@wavy` and `@thick` against `@loading_indicator`,
       as `exclusive_variants` groups on `material-3`, grounded on Compose's separate composables.
     - Drop the example or replace it with one that occurs (see #2).

**check-38.md, item 1**

1. **The new exclusive group applies its own ground to one pair only. The same ground excludes `@four_color` with
   `@wavy`, and `@wavy` and `@thick` with `@loading_indicator`. Left combinable, these pairs get a look no platform
   draws, and one of them can make the preset fail validation, depending on rule order.**
   - Where: `spec-widgets.md:2975-2976`; S22 `:4020` ("`material-3`'s spinner `[@four_color, @loading_indicator]`
     exclusive").
   - Evidence:
     - The stated ground is that Compose's `LoadingIndicator` takes one colour (`androidx@e4d933c9 …/LoadingIndicator.kt:101`).
       Compose's `CircularWavyProgressIndicator` also takes one colour (`WavyProgressIndicator.kt:281-284`,
       `:353-355`). material-web, the source of `four-color` (`material-web@2.5.0 progress/internal/progress.ts:40`),
       has no wave (no `wav` in `progress/`). So no source draws `@four_color` with `@wavy`.
     - `LoadingIndicator`'s parameters are `progress`, `modifier`, `color` and `polygons` (`:98-102`, `:136-139`). It
       has no stroke and no wave, so no source draws `@wavy` or `@thick` with `@loading_indicator` either.
     - S12 validates every combination of conditions that can hold together (`:2293`), and non-exclusive variants do
       hold together.
     - Under `{@thick, @loading_indicator}`, MAT-SN3's rule patches `indicator.glyph` (`trace-material.md:431`), and
       `@loading_indicator`'s rule makes `indicator.glyph` `Absent` (`spec-widgets.md:2999-3000`). If the
       `@loading_indicator` rule comes first, S12 refuses the preset (`:2986-2988`, a `Fields` patch meeting an
       `Absent` block).
     - If the rules come in the other order, the query answers that pair, and `@wavy` with it, instead of
       `NotAvailable(Variant)`. The theme then claims Material draws looks it does not draw.
     - No text fixes the order.
   - Fix:
     - Declare `exclusive_variants = [["@four_color", "@wavy", "@loading_indicator"], ["@thick", "@loading_indicator"]]`
       in `[declare.flavor.expressive.widget.spinner]`. The citations are `LoadingIndicator.kt:98-102`, `:136-139`,
       `WavyProgressIndicator.kt:281-284`, `:353-355` and `progress.ts:40`.
     - `[declare.widget.spinner]` (baseline) also needs `[["@four_color", "@wavy"]]`.
     - Check `@four_color` with `@thick` against MAT-SN3's source the same way. If `@thick` is Compose's alone, it
       joins the first group.
     - Restate S22's item.

**check-38.md, item 7**

7. **The new group's declaration site and citation.**
   - Where: `spec-widgets.md:2975-2976`.
   - Problems:
     - `@loading_indicator` is declared only in `[declare.flavor.expressive.widget.spinner]`, and a flavour's key
       replaces the widget's (`:2774-2777`). So the group can live only in that section. The text does not say so,
       and a base-section group would name an undeclared variant under `flavor:baseline`.
     - The citation `:101` is the determinate overload. The loop the exclusion concerns is the indeterminate one's
       (`:136-139`).
     - The entry's **Exclusive groups:** bullet (`:2982`) says "none". That is about flag groups, but it sits beside a
       new variant group.
   - Fix:
     - Name the section.
     - Cite `:98-102`, `:136-139`.
     - Optionally add "variant groups: see Variants".

**check-39.md, item 1**

1. **The new groups' ground, "no source draws them together", is applied to some pairs and not to others that fail
   it the same way: `@four_color` with `@thick` on the spinner, and all of `progress_bar`'s identical variant set.**
   - Where:
     - `spec-widgets.md:2975-2980` (the spinner's groups);
     - `spec-widgets.md:3115` (`progress_bar`: `material-3` `@wavy`, `@thick`, `@four_color`, with no group);
     - S22 `:4019`.
   - Evidence, the spinner:
     - `@thick` is Compose's alone. The thick tokens say "No longer tokenized as a variant, but rather a sample
       configuration in code" (`material-web@2.5.0 tokens/versions/latest/sass/_md-comp-progress-indicator-circular.scss:26-41`).
     - The only code configuration is Compose's `CircularThickWavyProgressIndicatorSample`
       (`androidx@e4d933c9 compose/material3/material3/samples/src/main/java/androidx/compose/material3/samples/ProgressIndicatorSamples.kt:192-209`).
       It is a `CircularWavyProgressIndicator`, which takes one `color` (`WavyProgressIndicator.kt:281-284`).
     - material-web's circular progress, the only source of `four-color`, reads the `v0_192` circular tokens, which
       have no thick entry (`tokens/versions/v0_192/_md-comp-circular-progress-indicator.scss:27-39`).
     - So no source draws `{@four_color, @thick}`. Both groups (baseline and expressive) leave it combinable, and the
       query answers it with a look no platform draws.
     - Check 38 #1 asked for exactly this test ("If `@thick` is Compose's alone, it joins the first group"). It was
       not applied.
   - Evidence, `progress_bar`:
     - The same three variants rest on the same sources: MAT-P1 (wavy), MAT-P4 (thick), MAT-P6 (four-colour)
       (`trace-material.md:198-207`).
     - `LinearWavyProgressIndicator` takes one `color` (`WavyProgressIndicator.kt:98-102`, `:170-173`).
     - The linear thick tokens are likewise "a sample configuration in code" (`_md-comp-progress-indicator-linear.scss:32-59`),
       and that configuration is `LinearThickWavyProgressIndicatorSample` (`ProgressIndicatorSamples.kt:98-118`).
     - material-web's four-colour linear animation (`progress/internal/_linear-progress.scss:150-167`, `:290`) draws
       no wave and no thick bar.
     - So `{@four_color, @wavy}` and `{@four_color, @thick}` are as undrawn on `progress_bar` as on the spinner. The
       entry's **Exclusive groups:** (`spec-widgets.md:3120-3121`) names only WinUI's flag group.
   - Fix:
     - Spinner, baseline: `[["@four_color", "@wavy"], ["@four_color", "@thick"]]`.
     - Spinner, expressive: add `["@four_color", "@thick"]`, or widen the first group to
       `[@four_color, @wavy, @thick, @loading_indicator]`. The second form is wrong: `@wavy` and `@thick` combine (the
       sample is both), so use a separate pair.
     - `progress_bar`: `exclusive_variants = [["@four_color", "@wavy"], ["@four_color", "@thick"]]` in
       `[declare.widget.progress_bar]`, with the citations above.
     - Restate S22's item for both widgets.
     - Alternatively, state why a thick four-colour ring counts as drawn. material-web's `active-indicator-width`
       token is customisable (`_circular-progress.scss:97-99`, `:142-143`), but a customised width is not the
       variant.

**check-39.md, item 2**

2. **`@contained` without `@loading_indicator` is a variant combination no source draws, yet the query answers it.
   S10 says such a form is one variant of the catalogue.**
   - Where:
     - `spec-widgets.md:2975`, `:2980` (`@contained` "its contained form", declared beside `@loading_indicator`);
     - `trace-material.md:575` (MAT-N24's container rule only `@ variants {@loading_indicator,@contained}`);
     - `spec-widgets.md:3003-3004` (`body.corners` absent "by a rule outside `@contained`");
     - S10 `:2805`.
   - Evidence:
     - Compose's contained form is its own composable, `ContainedLoadingIndicator` (`LoadingIndicator.kt:184-190`, `:227-232`),
       which always draws the morphing polygons.
     - Under `{@contained}`, `{@contained, @wavy}`, `{@contained, @thick}` or `{@contained, @four_color}`, no rule
       draws the container or the shape. Yet `body.corners` is present, since only a rule outside `@contained`
       removes it. The answer is an arc spinner carrying whatever corners the base states, with no fill or padding
       from MAT-N24.
     - No exclusive group excludes these sets. A group cannot express "only together with", and S10 has no
       per-variant `requires`.
     - S10 `:2805-2806` answers the case directly: "a form it draws only as two variants together is one variant of
       the catalogue".
     - S10 `:2799-2800` checks only listed variant sets at the gate. So nothing catches the undrawn answer.
   - Fix (choose one and record the options):
     - (a) Per S10, make the contained form one variant, `@contained_loading_indicator`, exclusive with
       `@loading_indicator`, `@wavy`, `@thick` and `@four_color`. MAT-N24's and MAT-X12's indicator rules are then
       stated under each of the two variants (the loop rule restated once).
     - (b) Keep two variants, and add a `requires` key per variant in S10 (`requires = { "@contained" =
       ["@loading_indicator"] }`), answered `NotAvailable(Variant)` like `requires_variants`. This is a new
       mechanism, a cost to argue.
     - (a) needs no new mechanism and matches S10's own sentence.

**check-39.md, item 5**

5. **The material-web "draws no wave" citation cites a line for an absence.**
   - Where: `spec-widgets.md:2977-2978`.
   - Problem: `progress.ts:40` is the `fourColor` property. It supports "the source of `four-color`", not "draws no
     wave". The absence holds: there is no `wav` in `progress/`. Elsewhere the project words absences as searches,
     not as line citations.
   - Fix: "material-web, the source of `four-color` (`progress/internal/progress.ts:40`), draws no wave (no wave
     in `progress/`)".

**check-39.md, item 10**

10. **(Pre-existing, found walking the groups) Material's thick-wavy sizes need a rule order or a combined rule.**
    - Where: `trace-material.md:198-203` (`progress_bar`), `:430-431` (spinner).
    - Evidence:
      - `progress_bar` under `{@wavy, @thick}` is 14 px high (`_md-comp-progress-indicator-linear.scss:56-59`,
        `thick.with-wave.height`; `ProgressIndicatorSamples.kt:115`, `Modifier.height(14.dp)`). That is neither
        MAT-P1's wavy height (`:64-65`, 10 px) nor MAT-P4's thick height (8 px).
      - MAT-P4's citation range (`:32-52, :62`, gap map `:160`) misses `:56-59`. No row states a rule
        `@ variants {@wavy,@thick}`, so a visible value is uncovered.
      - On the spinner, `{@wavy, @thick}` is 52 dp (`ProgressIndicatorSamples.kt:207`). That is MAT-SN3's size only
        if MAT-SN3's rule comes after MAT-SN2's 48, and no text fixes that order.
    - Fix:
      - Add a MAT-P4 row: `progress_bar.{indicator,body}.min_height @ variants {@wavy,@thick}` (`:56-59`).
      - State "after MAT-SN2" on MAT-SN3's spinner row.
      - Or hand both to the Material lane as a ledger queue row.
