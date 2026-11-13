// Stands in for the class aconfig generates from core/java/android/content/res/flags.aconfig, which the source
// tree does not hold. `font_scale_converter_public` turns FontScaleConverterFactory's cache of the converters it
// builds on; the harness restores the factory's tables before each row, so a row is what a process asking for its
// scale first gets, whatever the flag.
package android.content.res;

public final class Flags {
    public static final String FLAG_FONT_SCALE_CONVERTER_PUBLIC =
            "android.content.res.font_scale_converter_public";

    private Flags() {}

    public static boolean fontScaleConverterPublic() {
        return true;
    }
}
