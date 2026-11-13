# m3-vectors

Generates the vector table of `m3_role` (specification S2.2 *Checked against their references*) by the `public_api`
route, under `native-theme/tests/vectors/`: `m3_role.tsv`, the roles of material-color-utilities' dynamic scheme as
its Java code at the pinned commit gives them.

`generate.py` reads each Java file from the source cache (`scripts/update_sources.sh`) as the commit
`docs/facts/sources.toml` pins holds it, through git's objects (`tools/pinned.py`), and compiles the code unchanged
with `Harness.java`, which calls the public API as an application does: `new SchemeTonalSpot(Hct.fromInt(seed),
isDark, contrastLevel, SpecVersion.SPEC_2021, Platform.PHONE)` and `MaterialDynamicColors`' role of each name, its
colour by `DynamicColor.getArgb`. The scheme is the one the Material facts' `token.material` states
(`docs/facts/material/roles.toml`), the roles those the facts call `m3_role` with, the seeds and contrast levels
synthetic. `stubs/` stands in for the annotations the code imports (AndroidX's and Error Prone's). The compiler and
the harness run under bubblewrap with no network, writing only the build folder.

Requires a JDK (`javac` and `java`, 21 or later) and bubblewrap. Run from the repository root, the source cache's
folder optional:

    python3 -I tools/m3-vectors/generate.py [source cache]
