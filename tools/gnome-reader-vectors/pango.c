/* Calls Pango's public pango_font_description_from_string on each input and
 * prints what the description holds, through Pango's public getters.
 *
 * Reads one input per line on stdin: the string's UTF-8 bytes in hex (so an
 * input may hold any byte but NUL, a newline or a tab among them). Writes one
 * line per input, tab-separated:
 *
 *   <mask> <family> <style> <variant> <weight> <width> <gravity> <color>
 *   <size> <absolute> <variations> <features>
 *
 * <mask> is pango_font_description_get_set_fields as a decimal; each string
 * field is its bytes in hex, or "-" where the getter answers NULL; the others
 * are the getters' integers as decimals.
 */

#include <pango/pango.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void
put_hex (const char *s)
{
  if (s == NULL)
    {
      fputs ("-", stdout);
      return;
    }
  if (*s == '\0')
    {
      /* an empty string: "" is no hex, so it gets a mark of its own */
      fputs ("=", stdout);
      return;
    }
  for (; *s; s++)
    printf ("%02x", (unsigned char) *s);
}

static int
unhex (const char *hex, char *out, size_t cap)
{
  size_t n = strlen (hex);
  size_t i;

  if (n % 2 != 0 || n / 2 + 1 > cap)
    return 0;
  for (i = 0; i < n / 2; i++)
    {
      unsigned int byte;
      if (sscanf (hex + 2 * i, "%2x", &byte) != 1 || byte == 0)
        return 0;
      out[i] = (char) byte;
    }
  out[n / 2] = '\0';
  return 1;
}

int
main (void)
{
  static char line[65536];
  static char input[32768];

  while (fgets (line, sizeof line, stdin) != NULL)
    {
      PangoFontDescription *desc;
      size_t n = strlen (line);

      while (n > 0 && (line[n - 1] == '\n' || line[n - 1] == '\r'))
        line[--n] = '\0';
      /* "=" is the empty input */
      if (strcmp (line, "=") == 0)
        input[0] = '\0';
      else if (!unhex (line, input, sizeof input))
        return 2;

      desc = pango_font_description_from_string (input);
      printf ("%u\t", (unsigned) pango_font_description_get_set_fields (desc));
      put_hex (pango_font_description_get_family (desc));
      printf ("\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t",
              (int) pango_font_description_get_style (desc),
              (int) pango_font_description_get_variant (desc),
              (int) pango_font_description_get_weight (desc),
              (int) pango_font_description_get_width (desc),
              (int) pango_font_description_get_gravity (desc),
              (int) pango_font_description_get_color (desc),
              pango_font_description_get_size (desc),
              (int) pango_font_description_get_size_is_absolute (desc));
      put_hex (pango_font_description_get_variations (desc));
      fputs ("\t", stdout);
      put_hex (pango_font_description_get_features (desc));
      fputs ("\n", stdout);
      fflush (stdout);
      pango_font_description_free (desc);
    }
  return 0;
}
