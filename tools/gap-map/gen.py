"""Rebuild §4–§8 and Appendix A of the main gap map from the tags and the merge output.

Usage: python3 -I gen.py <scratchpad> <repo>; reads <scratchpad>/merged.md (merge.py's output),
taxonomy.md, seeds-matched.md and tags-*.tsv; keeps the map's text before `## 4. The gaps`.
"""
import re
import sys
from pathlib import Path

sp, repo = Path(sys.argv[1]), Path(sys.argv[2])
m = (sp / "merged.md").read_text()
cut = lambda a, b: m[m.index(a) + len(a): m.index(b) if b else None].strip()
tot = m[: m.index("CAPMATRIX")].strip()
mat = cut("CAPMATRIX", "CAPROWS")
caprows = cut("CAPROWS", "ELEMENTS")
elems = cut("ELEMENTS", "CONTROLS")
ctl = cut("CONTROLS", None)
tax = {}
for line in (sp / "taxonomy.md").read_text().splitlines():
    mm = re.match(r"^\| (C\d\d) \| (.*) \|$", line)
    if mm:
        tax[mm.group(1)] = mm.group(2)
rows = ["| ID | Capability | macOS | Windows | KDE | GNOME | Material | iOS |",
        "|---|---|---:|---:|---:|---:|---:|---:|"]
for r in mat.splitlines()[2:]:
    c = [x.strip() for x in r.strip("|").split("|")]
    rows.append(f"| {c[0]} | {tax[c[0]]} | " + " | ".join(c[1:]) + " |")
captable = "\n".join(rows)
caprows = re.sub(r"^#### (C\d\d)$", lambda mm: f"#### {mm.group(1)} — {tax[mm.group(1)].split(':')[0]}",
                 caprows, flags=re.M)
seeds = (sp / "seeds-matched.md").read_text()
seedtable = seeds[seeds.index("| Group |"):].strip()
slugs = ["macos", "windows", "kde", "gnome", "material", "ios"]
names = ["macOS", "Windows", "KDE", "GNOME", "Material", "iOS"]
ver = {}
for s in slugs:
    ver[s] = sum(1 for l in (sp / f"tags-{s}.tsv").read_text().splitlines() if "V" in l.split("\t")[5])
vtext = ", ".join(f"{n} {ver[s]}" for s, n in zip(slugs, names))
fu = {}
for s in slugs:
    ids = [l.split("\t")[1] for l in (sp / f"tags-{s}.tsv").read_text().splitlines()]
    fu[s] = tuple(sum(1 for i in ids if re.search(k + r"-?\d+†$", i)) for k in ("SF", "RL", "FM"))

p3 = {}
p4_counts = []
for s_, n_ in zip(slugs, names):
    d_ = (repo / "docs" / f"todo_v0.7.1_model-completeness-gap-map-{s_}.md").read_text()
    inp = sum(1 for l in (sp / f"p3-{s_}.tsv").read_text().splitlines() if l.strip())
    added = len(set(re.findall(r"^\| ((?:[A-Z]+-)?P3-\d+†) \|", d_, flags=re.M)))
    marked = sum(1 for l in d_.splitlines() if l.startswith("|") and "(review 2026-10-28" in l)
    wd = sorted(set(re.findall(r"^\| ([A-Z][A-Za-z0-9.\-]*†?) \|[^\n]*Withdrawn \(review", d_, flags=re.M)))
    rv = len(set(re.findall(r"^\| ([A-Z]+-RV-?\d+†) \|", d_, flags=re.M)))
    p4 = len(set(re.findall(r"^\| ((?:[A-Z]+-)?P4-\d+†) \|", d_, flags=re.M)))
    p4_counts.append(p4)
    p3[s_] = f"| {n_} | {inp} / {added} | {marked} / {rv} / {len(wd)}" + (f" ({', '.join(wd)})" if wd else "") + f" | {p4} |"
p3_rows = "\n".join(p3[s_] for s_ in slugs)
p4_total = sum(p4_counts)
p4 = (sp / "p4-result.md").read_text()
p4_table = p4[p4.index("| Kind |"):p4.index("\n## ")].strip()
p4_kinds = p4[p4.index("## Kinds") + len("## Kinds"):].strip()
p = repo / "docs/todo_v0.7.1_model-completeness-gap-map.md"
doc = p.read_text()
head = doc[: doc.index("## 4. The gaps")]
head = re.sub(r"Status: .*?(?=\n\nThe first of)",
              "Status: complete (2026-10-27/28, branch `v0.7.1-rc1`, model at caeddd6b): surveyed,\n"
              "cross-checked in four passes and reviewed (§4). §6 lists the open work: the scope questions the rationale decides, and what\n"
              "comes before any value is collected.", head, flags=re.S)
fu_rows = "\n".join(f"| {n} | {fu[s][0]} | {fu[s][1]} | {fu[s][2]} |" for s, n in zip(slugs, names))
body = f"""## 4. The gaps

Every step ran per platform, against `docs/property-registry.toml` at caeddd6b:

- **pass 1** surveyed each platform on its own;
- **pass 2** checked each platform against what the other five found: every control another
  survey listed, every capability, and every "(none)" cell of the platform's column in
  `platform-facts.md` (for iOS, which has no column there, every `ios-13` preset value with
  no iOS source);
- **the follow-up** wrote rows for the parts of the seed groups no row covered (§5), asked
  the other platforms about 18 controls that pass 2 had put under a wrong control name,
  and, from a check of this map's completeness, wrote rows for the emoji font and the
  fallback chain for scripts the UI font lacks (no platform had one) and for right-to-left
  layout where a platform had no general row;
- **pass 3** asked each platform about every row the other five had added after pass 1,
  which pass 2, running on all six at once, could not;
- **the review** read each platform file again, by someone who had not written it: every row
  added after pass 1 and a sample of pass-1 rows checked at the cited source, false gaps
  (fields the model has), contradictions and duplicates withdrawn or corrected in place;
- **pass 4** grouped the rows pass 3 and the review added into 78 kinds and put each kind to
  all six platforms (Appendix B), and its rows were reviewed like the others.

Where the passes stop: each pass asked only about what the one before added, and each added
less (pass 2 197 rows, pass 3 and the review 123, pass 4 {p4_total}). Pass 4 adds rows only to
kinds already put to every platform, so it raises no question another platform has not
answered, and the map is closed there.

Rows added after pass 1 carry a trailing "†". Each platform's rows, with their sources,
are in its own file (§8); this section is the cross-platform view built from them, each row
tagged with the capabilities (§4.3) it needs the model to have.

Why this shape: one file for all platforms would hold some 800 KB of rows, and a file per
capability would split one platform's evidence over 28 files, so the map is this overview
and a file per platform. The capabilities group rows by the mechanism the model needs (a
shadow layer, a state, a motion curve), not by widget, because the rationale designs one
mechanism per capability and then applies it to every widget that needs it; a row that
needs several mechanisms carries several. The tables below and Appendix A are built from the
platform files' rows as they stand at this document's commit.

{tot}

The columns are the platform files' own sections: §4.1 an element the model has, §4.2 a
control it lacks, §4.3 a property that crosses widgets. A row counts once. Not counted are
42 pass-1 rows that only point to another row or record "no gap" (macOS 2, Windows 1, KDE
13, GNOME 9, Material 7, iOS 10). A row ID is unique within its file.

Pass 2, per platform (detail in each file's *Cross-check* section):

| Platform | Rows added (§4.1 / §4.2 / §4.3) | Controls others list: own control / inside an element / not on the platform | Capabilities: gained rows / no further row (with reason) | "(none)" cells: right / drawn, now a row |
|---|---|---|---|---|
| macOS | 27 / 11 / 9 | 7 / 29 / 21 | 19 / 9 | 32 / 29, and 2 that differ by generation |
| Windows | 12 / 4 / 12 | 8 / 11 / 27 | 19 / 9 | 43 / 13 |
| KDE | 15 / 16 / 11 | 12 / 20 / 18 (as the review corrected it) | 23 / 5 | 38 / 20 |
| GNOME | 9 / 7 / 8 | 8 / 12 / 31 | 12 / 16 | 44 / 19 |
| Material | 9 / 2 / 7 | 2 / 17 / 39 | 19 / 9 | 95 / 30 (as the review corrected it) |
| iOS | 21 / 14 / 3 | 12 / 18 / 20 | 22 / 6 | `ios-13` values with no source: 18 for things iOS does not draw / 61 to measure (as the review corrected it) |

The follow-up, per platform (each file's *Seed follow-up*, *Relabelled controls* and
*Fonts and mirroring* sections; IDs `…SF…†`, `…RL…†` and `…FM…†`):

| Platform | Seed rows | Relabelled-control rows | Font and mirroring rows |
|---|---:|---:|---:|
{fu_rows}

Pass 3 and the review, per platform (each file's *Pass 3* section; the review's changes are
marked "(review 2026-10-28: …)" in the rows and decision lines they changed, and the rows it
added are in a *Review additions* section (IDs `…RV…†`); a withdrawn row keeps its text after
"Withdrawn (review 2026-10-28): <reason>" and is not counted):

| Platform | Pass 3: input rows / rows added | Review: lines marked / rows added / rows withdrawn | Pass 4: rows added |
|---|---|---|---:|
{p3_rows}

### 4.1 Elements the model has: gaps per section

The rows of §4.1 of each platform file (parts, states and properties of an element the
model has), by the model section they belong to. Their capabilities are in §4.3.

{elems}

### 4.2 Standard controls the model does not have

The rows of §4.2 of each platform file, sorted by how many platforms list the control. A
"–" is settled in that platform's *Cross-check*, table A, its *Relabelled controls* section
or its *Pass 3* decisions: the platform has no such control (with the source that shows it), or draws it
inside an element the model has, whose rows then cover it (a tree view inside `[list]`).

{ctl}

### 4.3 Capabilities the model lacks, across all rows

One row can need several capabilities (the checked-and-hovered switch's border colour is
C07 and C12), so a column's counts add up to more than the platform's total. A count is how
many of that platform's rows need the capability; it shows where a capability matters, not
how hard it is. Where a platform's count is low or zero, its *Cross-check*, table B, says
why (Windows states its hover colours per state, so it needs no state-layer model, C08).
Two IDs are not capabilities of the model's types: C24 marks a §4.2 row (a whole control
the model lacks), and C25 a value the platform exposes at run time that no reader reads.
The row IDs per capability are in Appendix A.

{captable}

C27 and C28 were added after pass 1's tagging and applied from each row's own text (the
words *gradient*, *bevel*; *P3*, *HDR*, *wide gamut*, *extended range*), then checked by
hand; the later rows were tagged with all 28.
"""
narr = doc[doc.index("What the counts show, in brief:"): doc.index("## 5. Seeds")].rstrip()
body += "\n" + narr + "\n\n"
body += f"""## 5. Seeds

The gaps the repository had already written down, gathered before the platform research,
are in `todo_v0.7.1_model-completeness-gap-map-seeds.md`: 37 groups, each with every
file:line that records it (at caeddd6b, as are the citations in the table below), and in
its second part the 222 lines of `platform-facts.md` with a ❓ cell, a value no source gave,
which under this milestone's rule is a value to measure.

Each group was matched to the platform rows after pass 2 (G37, the "(none)" cells, was
settled by pass 2 itself): 24 were covered, 5 partly, and 7 are not gaps of the model. The
follow-up then wrote the rows the partly covered groups lacked, on every platform: a code
editor's colours and parts (G12; KDE's KTextEditor and GNOME's GtkSourceView, the other
platforms ship none), the drop zone under a dragged pane or dock (G19), where a modal
dialog is placed (G24), and rich text and Markdown (G27); G35's keycap is GN-2.40, asked
of every platform in the relabelled-control check. Of the 7 that are not model gaps, G03
(iced's renderer `snap`), G29–G31 (fields the model has) and G32 (the toolkits' own
fallbacks) are connector matters; G34 and G36 are presets that state no value or one
without a source, which is value work (§6.4). The table is the matching as it was before
the follow-up.

{seedtable}

## 6. Open work

1. **Scope, decided in the rationale.** Surfaces the OS draws outside the app's window,
   which an application neither draws nor styles (GNOME's on-screen keyboard, iOS widgets,
   Live Activities, notifications, Control Center, the shell's tiling previews, the macOS
   menu-bar extras' system part). The platform files decide this differently today: the
   Windows file leaves shell-drawn UI out, the macOS, KDE, GNOME, Material and iOS files
   have rows for some of it, each saying what draws it (e.g. MAC-N25, K-RL03†, GN-P3-1†,
   MAT-P3-1†, C-23); the rationale settles it once for all six.
   Libraries a platform's own applications use beside its
   toolkit: KDE's kirigami-addons (FormCard, date and time pickers) and KDDockWidgets
   (Kdenlive), GNOME's GtkSourceView (GNOME Text Editor, Builder) and libpanel (Builder),
   the Windows Community Toolkit, whose rows pass 2 included, and its experimental Labs
   (`MarkdownTextBlock`), KDE's KQuickCharts. Applications a platform runs that draw with
   another toolkit or an older stylesheet (GTK 3 applications on GNOME, WPF and unthemed
   comctl32 controls on Windows). SwiftUI and Mac Catalyst where they draw differently from
   AppKit and UIKit.
2. **Citations, before any value is collected.** The KDE file cites KDE's GitHub mirrors at
   `master`: each citation is re-pinned to the release tags `platform-facts.md` uses
   (Breeze and KWin 6.7.x, KF 6.31.0); qtbase is cited at v6.10.1 in some rows and v6.11.2
   in others, and the GNOME Text Editor scheme at `main`. The rows marked "to verify"
   ({vtext}) are settled, and the Windows rows the review could not check without
   fetching their Microsoft Learn pages again (listed in its findings: W1-149†, W1-151†,
   W1-156†, W3-49†, W3-56†, W-SF12†, W-SF13†, W-RL01†–W-RL03†, W-FM01†, W-FM02†, W-P3-1†,
   W-P3-2†, W-P3-10†, W-P3-13†, W-P3-15†) are checked.
3. **Measuring, as the surveys found it:**
   - the GitHub macOS runner images have Reduce Motion and Reduce Transparency **on**, so
     motion and materials need a probe pass with both off;
   - the probe's offscreen renders draw the non-key look, so key-window states need the
     controls in the key-window capture;
   - the window captures use `screencapture -o`, which leaves shadows out;
   - the probe opens no menu and does not render the extraLarge control size;
   - the Windows probe only queries; it renders nothing (`tools/os-probe/src/windows.rs`);
   - no KDE (Qt Widgets, Kirigami), GNOME (GTK 4 / libadwaita) or iOS (UIKit, simulator)
     probe app exists yet, nor a Material one (material-web, Compose); whether the macOS
     CI image has the iOS 13–18 simulator runtimes is unchecked.
4. **Value work the map found** (not model gaps, done with the values): the 222 ❓ lines of
   `platform-facts.md`; the "(none)" cells pass 2 classified as drawn; the 61 `ios-13`
   values iOS draws with no source; the seed groups G34 and G36; and the values found
   wrong (§7).

## 7. Values found wrong on the way

The research read sources that disagree with values native-theme already ships: wrong
cells of `platform-facts.md`, readers that read the wrong key or none, and presets that
state values for things a platform does not draw. They are not gaps of the model; they are
recorded in `docs/todo.md`, *Values the v0.7.1 gap map found against their sources*. Rows
of the platform files that a later step found wrong are corrected in place, and each
correction names what corrected it (pass 2 or the follow-up, 2026-10-28).

## 8. The platform files

| File | Platform | Row IDs |
|---|---|---|
| `todo_v0.7.1_model-completeness-gap-map-macos.md` | macOS 12–15 and 26–27 | `MAC-…` |
| `todo_v0.7.1_model-completeness-gap-map-windows.md` | Windows 11 | `W1-…`, `W2-…`, `W3-…`, `W-SF…`, `W-RL…`, `W-FM…`, `W-P3-…`, `W-RV-…` |
| `todo_v0.7.1_model-completeness-gap-map-kde.md` | KDE Plasma 6, Breeze | `K…` (Qt Widgets, KDecoration, KWin), `Q…` (Kirigami, qqc2-desktop-style), `K-SF…`, `K-RL…`, `K-FM…`, `K-P3-…`, `K-RV…`, `K-KA-…` (kirigami-addons, added during the specification) |
| `todo_v0.7.1_model-completeness-gap-map-gnome.md` | GNOME, libadwaita 1.10.0, GTK 4.24.1 | `GN-…` |
| `todo_v0.7.1_model-completeness-gap-map-material.md` | Material 3 and Expressive | `MAT-…` |
| `todo_v0.7.1_model-completeness-gap-map-ios.md` | iOS 13–18 and 26 | per section (`B-…`, `CK-…`, `X-MT-…`; `C-…` for the controls of §4.2; `SF-…`, `RL-…`, `FM-…`, `P3-…`) |
| `todo_v0.7.1_model-completeness-gap-map-seeds.md` | the repository's own records, and the follow-up's sources | `G01`–`G37` |

Each file states its sources and their refs at its top (later sources in the section that
added them), and no file states a value it did not read in a cited source. A platform
file's own *Counts* section, where it has one, is pass 1's.

## Appendix A. Rows per capability

{caprows}

## Appendix B. Pass 4: kinds put to every platform

Each cell names the rows that cover the kind on that platform, "new" where pass 4 added the
row (in the platform file's *Pass 4* section), or "absent" with the source that shows the
platform does not draw it.

{p4_table}

### The kinds

{p4_kinds}
"""
p.write_text(head + body)
print("written", len(head + body))
