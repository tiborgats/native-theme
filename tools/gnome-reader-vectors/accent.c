/* Calls libadwaita's adw_accent_color_nearest_from_rgba (adw-accent-color.c),
 * compiled unchanged from the pinned source with adw-color-utils.c
 * (generate.py fetches them and builds all three with ADWAITA_COMPILATION
 * defined, as libadwaita's own build does). Reads one call per
 * line on stdin:
 *
 *   nearest <red> <green> <blue>
 *       the GdkRGBA's channels, floats as the portal's doubles become in
 *       adw-settings-impl-portal.c's get_fdo_accent_color (alpha 1); prints
 *       the AdwAccentColor as its integer.
 *   oklch <red> <green> <blue>
 *       adw_rgb_to_oklch's L, c and h for the same floats (the values the
 *       snapping compares); prints them "%.9g".
 *   named <accent>
 *       adw_accent_color_to_rgba's channels for the AdwAccentColor integer;
 *       prints them "%.9g".
 */

#include "adw-accent-color-private.h"
#include "adw-color-utils-private.h"

#include <stdio.h>
#include <string.h>

static int
read_rgba (GdkRGBA *rgba)
{
  float c[3];

  if (scanf ("%f %f %f", &c[0], &c[1], &c[2]) != 3)
    return 0;
  rgba->red = c[0];
  rgba->green = c[1];
  rgba->blue = c[2];
  rgba->alpha = 1.0f;
  return 1;
}

int
main (void)
{
  char op[16];

  while (scanf ("%15s", op) == 1)
    {
      GdkRGBA rgba;

      if (strcmp (op, "nearest") == 0)
        {
          if (!read_rgba (&rgba))
            return 2;
          printf ("%d\n", (int) adw_accent_color_nearest_from_rgba (&rgba));
        }
      else if (strcmp (op, "oklch") == 0)
        {
          float L, c, h;

          if (!read_rgba (&rgba))
            return 2;
          adw_rgb_to_oklch (rgba.red, rgba.green, rgba.blue, &L, &c, &h);
          printf ("%.9g %.9g %.9g\n", L, c, h);
        }
      else if (strcmp (op, "named") == 0)
        {
          int accent;

          if (scanf ("%d", &accent) != 1 || accent < ADW_ACCENT_COLOR_BLUE || accent > ADW_ACCENT_COLOR_SLATE)
            return 2;
          adw_accent_color_to_rgba ((AdwAccentColor) accent, &rgba);
          printf ("%.9g %.9g %.9g\n", rgba.red, rgba.green, rgba.blue);
        }
      else
        return 2;
      fflush (stdout);
    }
  return 0;
}
