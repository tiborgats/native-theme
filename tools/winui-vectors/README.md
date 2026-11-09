# winui-vectors

Generates the vector table of `win_ui` (specification S9.4's `WinUi`, S2.2 *Checked against their references*;
ADD-A141) by the `compiled_source` route, under `native-theme/tests/vectors/`: `win_ui.tsv`, WinUI's text-size curve
as `TextFormatting::GetScaledFontSize` at the pinned tag gives it (`microsoft-ui-xaml@2.5.1
src/dxaml/xcp/core/text/common/TextFormatting.cpp:181-204`, MIT).

`generate.py` reads the file from the source cache (`scripts/update_sources.sh`) as the commit
`docs/facts/sources.toml` pins holds it, through git's objects (`tools/pinned.py`), takes the function's lines,
checked to run from its signature to its closing brace, and compiles them unchanged after `harness.cpp`'s stub of
what they read: `XFLOAT` (`src/dxaml/xcp/inc/minxcptypes.h:39`) and `TextFormatting`'s font size and text-scale flag
(`src/dxaml/xcp/core/inc/corep.h:466`, `:493`, `:500`). The harness calls the function on a `TextFormatting` holding
each row's size, its text scale factor enabled. The reference build takes `float log(float)` for the lines'
`log(inputFontSize)`, the overload MSVC's global declarations give a `float`, through libstdc++'s `<math.h>`; a second
build calls the C library's `double log(double)`, and the table's header counts the rows on which the two differ. The
sizes and factors are synthetic. The compiler and the harness run under bubblewrap with no network, writing only the
build folder.

Requires g++ (the table's header records the one that wrote it) and bubblewrap. Run from the repository root, the
source cache's folder optional:

    python3 -I tools/winui-vectors/generate.py [source cache]
