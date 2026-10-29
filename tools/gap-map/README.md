# tools/gap-map

Generators of the v0.7.1 gap map's cross-platform sections (§4–§8 and Appendix A of
`docs/todo_v0.7.1_model-completeness-gap-map.md`), and their inputs. The platform files
(`docs/todo_v0.7.1_model-completeness-gap-map-<platform>.md`) are the source; these files are the
rows' tags (`tags-<platform>.tsv`: platform, row ID, section, element, capabilities, source kind,
summary), the pass-3 and pass-4 inputs, the capability taxonomy and the matched seeds.

    python3 -I tools/gap-map/merge.py tools/gap-map > tools/gap-map/merged.md
    python3 -I tools/gap-map/gen.py tools/gap-map <dir-holding-docs/>   # rewrites <dir>/docs/…-gap-map.md

`gen.py` keeps the map's text before `## 4. The gaps`, except that it re-wraps lines 4–5 of the
header; restore those two lines by hand after a run. Re-run both after any change to a row's tags.
The tools go with the v0.7.1 documents when they are archived.
