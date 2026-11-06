// The harness of tools/m3-vectors: material-color-utilities' public API at its pinned commit, called as an
// application calls it. Each line of standard input is a row's inputs, tab-separated: the role (a DynamicColor
// name), the seed as 0xRRGGBB, the contrast level (Java's double literal of the f32 the row states) and the scheme
// (`light` or `dark`). Each line of standard output is that row's result, the role's colour as
// `DynamicColor.getArgb` returns it, in 8 hexadecimal digits AARRGGBB.

import dynamiccolor.ColorSpec.SpecVersion;
import dynamiccolor.DynamicColor;
import dynamiccolor.DynamicScheme;
import dynamiccolor.MaterialDynamicColors;
import hct.Hct;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;
import java.util.function.Supplier;
import scheme.SchemeTonalSpot;

public final class Harness {
  private Harness() {}

  public static void main(String[] args) throws Exception {
    Map<String, DynamicColor> colors = new HashMap<>();
    for (Supplier<DynamicColor> supplier : new MaterialDynamicColors().allDynamicColors()) {
      DynamicColor color = supplier.get();
      if (color != null) {
        colors.put(color.name, color);
      }
    }
    BufferedReader in =
        new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
    StringBuilder out = new StringBuilder();
    String line;
    while ((line = in.readLine()) != null) {
      String[] cells = line.split("\t", -1);
      if (cells.length != 4) {
        throw new IllegalArgumentException("a row of four cells: " + line);
      }
      DynamicColor color = colors.get(cells[0]);
      if (color == null) {
        throw new IllegalArgumentException("no DynamicColor `" + cells[0] + "`");
      }
      int seed = 0xff000000 | Integer.parseInt(cells[1], 16);
      double contrastLevel = Double.parseDouble(cells[2]);
      boolean isDark;
      switch (cells[3]) {
        case "light" -> isDark = false;
        case "dark" -> isDark = true;
        default -> throw new IllegalArgumentException("no scheme `" + cells[3] + "`");
      }
      DynamicScheme scheme =
          new SchemeTonalSpot(
              Hct.fromInt(seed),
              isDark,
              contrastLevel,
              SpecVersion.SPEC_2021,
              DynamicScheme.Platform.PHONE);
      out.append(String.format("%08x%n", color.getArgb(scheme)));
    }
    System.out.print(out);
  }
}
