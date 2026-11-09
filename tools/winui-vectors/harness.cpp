// Calls WinUI's TextFormatting::GetScaledFontSize, its lines
// (microsoft-ui-xaml src/dxaml/xcp/core/text/common/TextFormatting.cpp:181-204)
// compiled unchanged after the stub below: generate.py writes them, as the
// pinned commit holds them, to GetScaledFontSize.inc. The stub holds what the
// lines read: XFLOAT (src/dxaml/xcp/inc/minxcptypes.h:39) and TextFormatting's
// two fields (src/dxaml/xcp/core/inc/corep.h:466, :493, :500).
//
// The math header decides which `log` the lines' `log(inputFontSize)` calls.
// MSVC, WinUI's compiler, declares the float overloads of <cmath> in the
// global namespace, so a float argument takes `float log(float)`; libstdc++'s
// <math.h> brings std's overloads there too, which the reference build uses.
// Built with DOUBLE_LOG, <cmath> alone leaves the C library's `double
// log(double)` global, the build generate.py compares against.
//
// Reads one call per line on stdin, "<size> <factor>", floats; prints
// GetScaledFontSize(factor) of a TextFormatting holding that size with its
// text scale factor enabled, "%.9g".

#ifdef DOUBLE_LOG
#include <cmath>
#else
#include <math.h>
#endif
#include <algorithm>
#include <cstdio>
#include <type_traits>

typedef float XFLOAT;

class TextFormatting
{
public:
    XFLOAT GetScaledFontSize(XFLOAT fontScale) const;

    XFLOAT m_eFontSize;
    bool m_isTextScaleFactorEnabled;
};

#ifdef DOUBLE_LOG
static_assert(std::is_same<decltype(log(1.0f)), double>::value, "the double log");
#else
static_assert(std::is_same<decltype(log(1.0f)), float>::value, "the float log");
#endif

#include "GetScaledFontSize.inc"

int main()
{
    float size;
    float factor;

    while (std::scanf("%f %f", &size, &factor) == 2)
    {
        TextFormatting formatting;
        formatting.m_eFontSize = size;
        formatting.m_isTextScaleFactorEnabled = true;
        std::printf("%.9g\n", formatting.GetScaledFontSize(factor));
    }
    return 0;
}
