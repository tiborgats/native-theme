// The harness of tools/font-scale-vectors: frameworks/base's FontScaleConverter at its pinned tag, called through
// its public API as android.util.TypedValue calls it (`applyDimension` for sp, `deriveDimension` back,
// TypedValue.java:430-445 and :471-493): `FontScaleConverter.forScale(fontScale)`, then the converter's
// `convertSpToDp` or `convertDpToSp`, or, where the scale has no converter, the size times or over the scale (the
// scaled density over the density). Each line of standard input is a row's inputs, tab-separated: the function
// (`android_non_linear` or `android_non_linear_inverse`), the size and the font scale, each Java's float literal of
// the f32 the row states. Each line of standard output is that row's result, as `Float.toString` writes it.

import android.content.res.FontScaleConverter;
import android.content.res.FontScaleConverterFactory;
import android.util.SparseArray;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;

public final class Harness {
  private Harness() {}

  public static void main(String[] args) throws Exception {
    // The tables the factory's static block puts in, restored before each row: `forScale` caches the converters
    // it builds, and a row is what a process asking for its scale first gets.
    SparseArray<FontScaleConverter> initial = FontScaleConverterFactory.sLookupTables.clone();
    BufferedReader in =
        new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
    StringBuilder out = new StringBuilder();
    String line;
    while ((line = in.readLine()) != null) {
      String[] cells = line.split("\t", -1);
      if (cells.length != 3) {
        throw new IllegalArgumentException("a row of three cells: " + line);
      }
      float size = Float.parseFloat(cells[1]);
      float fontScale = Float.parseFloat(cells[2]);
      FontScaleConverterFactory.sLookupTables = initial.clone();
      FontScaleConverter converter = FontScaleConverter.forScale(fontScale);
      float result;
      switch (cells[0]) {
        case "android_non_linear" ->
            result = converter == null ? size * fontScale : converter.convertSpToDp(size);
        case "android_non_linear_inverse" ->
            result = converter == null ? size / fontScale : converter.convertDpToSp(size);
        default -> throw new IllegalArgumentException("no function `" + cells[0] + "`");
      }
      out.append(Float.toString(result)).append('\n');
    }
    System.out.print(out);
  }
}
