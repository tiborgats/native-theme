/* gnome-vectors: the vector table generator for libadwaita's public colour
 * functions (the 0.8.0 plan, "How values are checked", route 1; spike G).
 *
 * Calls adw_rgba_to_standalone, adw_accent_color_to_rgba and
 * adw_accent_color_to_standalone_rgba (libadwaita 1.6+, adw-accent-color.h)
 * and writes one TSV row per call. Needs no display: none of the three
 * reads a display or the style manager.
 *
 *     gnome-vectors OUT.tsv
 *
 * The inputs of adw_rgba_to_standalone, each with dark FALSE and TRUE:
 *   - the grid: red, green and blue each over 0, 51, 102, 153, 204 and 255
 *     (/255), alpha 1, 216 colours;
 *   - every AdwAccentColor's colour from adw_accent_color_to_rgba, alpha 1,
 *     and again with alpha 0.5.
 * adw_accent_color_to_rgba is called for every AdwAccentColor, and
 * adw_accent_color_to_standalone_rgba for every AdwAccentColor with dark
 * FALSE and TRUE.
 *
 * Columns: function, input (the colour as #rrggbbaa or the accent's nick),
 * the input's four channels as the exact floats passed, dark (0, 1, or - where
 * the function takes no flag), the output as #rrggbbaa and its four channels
 * as the exact floats returned. Floats are printed with %.9g, which reads back
 * to the same float; 8-bit values round as gdk_rgba_to_string does,
 * (int) (0.5 + CLAMP (x, 0, 1) * 255.).
 */

#include <adwaita.h>
#include <stdio.h>

static const guint grid[] = { 0, 51, 102, 153, 204, 255 };

static const char *packages[] = { "libadwaita", "gtk4", "glib2" };

static guint
to_8bit (float x)
{
  return (guint) (0.5 + CLAMP (x, 0., 1.) * 255.);
}

static void
put_hex (GString *s, const GdkRGBA *c)
{
  g_string_append_printf (s, "#%02x%02x%02x%02x",
                          to_8bit (c->red), to_8bit (c->green), to_8bit (c->blue), to_8bit (c->alpha));
}

static void
put_floats (GString *s, const GdkRGBA *c)
{
  g_string_append_printf (s, "\t%.9g\t%.9g\t%.9g\t%.9g", c->red, c->green, c->blue, c->alpha);
}

static void
put_row (GString       *s,
         const char    *function,
         const char    *input_name,
         const GdkRGBA *input,
         const char    *dark,
         const GdkRGBA *output)
{
  g_string_append (s, function);
  g_string_append_c (s, '\t');
  if (input_name)
    g_string_append (s, input_name);
  else
    put_hex (s, input);
  if (input)
    put_floats (s, input);
  else
    g_string_append (s, "\t-\t-\t-\t-");
  g_string_append_printf (s, "\t%s\t", dark);
  put_hex (s, output);
  put_floats (s, output);
  g_string_append_c (s, '\n');
}

static void
put_standalone (GString *s, const char *input_name, const GdkRGBA *input)
{
  for (int dark = 0; dark <= 1; dark++) {
    GdkRGBA output;

    adw_rgba_to_standalone (input, dark, &output);
    put_row (s, "adw_rgba_to_standalone", input_name, input, dark ? "1" : "0", &output);
  }
}

static void
put_header (GString *s)
{
  char *out = NULL;
  int wait_status = 0;
  const char *argv[G_N_ELEMENTS (packages) + 3] = { "pacman", "-Q" };

  for (gsize i = 0; i < G_N_ELEMENTS (packages); i++)
    argv[i + 2] = packages[i];

  g_string_append_printf (s, "# gnome-vectors: libadwaita's public colour functions, called by libadwaita %u.%u.%u\n",
                          adw_get_major_version (), adw_get_minor_version (), adw_get_micro_version ());
  if (g_spawn_sync (NULL, (char **) argv, NULL, G_SPAWN_SEARCH_PATH | G_SPAWN_STDERR_TO_DEV_NULL,
                    NULL, NULL, &out, NULL, &wait_status, NULL) && out) {
    char **lines = g_strsplit (g_strstrip (out), "\n", -1);

    for (char **line = lines; *line; line++)
      g_string_append_printf (s, "# package: %s\n", *line);
    g_strfreev (lines);
  } else {
    g_string_append (s, "# package: pacman -Q failed\n");
  }
  g_free (out);

  g_string_append (s, "# grid: red, green, blue each over 0, 51, 102, 153, 204, 255 (/255), alpha 1; "
                      "every AdwAccentColor's colour at alpha 1 and 0.5\n");
  g_string_append (s, "# 8-bit: (int) (0.5 + CLAMP (x, 0, 1) * 255.); floats: %.9g\n");
  g_string_append (s, "function\tinput\tinput_r\tinput_g\tinput_b\tinput_a\tdark\t"
                      "output\toutput_r\toutput_g\toutput_b\toutput_a\n");
}

int
main (int    argc,
      char **argv)
{
  GString *s = g_string_new (NULL);
  GEnumClass *accent_class = g_type_class_ref (ADW_TYPE_ACCENT_COLOR);
  GError *error = NULL;
  int status = 0;

  if (argc != 2) {
    g_printerr ("usage: gnome-vectors OUT.tsv\n");
    return 1;
  }

  put_header (s);

  for (gsize r = 0; r < G_N_ELEMENTS (grid); r++) {
    for (gsize g = 0; g < G_N_ELEMENTS (grid); g++) {
      for (gsize b = 0; b < G_N_ELEMENTS (grid); b++) {
        GdkRGBA input = { grid[r] / 255.f, grid[g] / 255.f, grid[b] / 255.f, 1.f };

        put_standalone (s, NULL, &input);
      }
    }
  }

  for (guint i = 0; i < accent_class->n_values; i++) {
    AdwAccentColor accent = accent_class->values[i].value;
    const char *nick = accent_class->values[i].value_nick;
    GdkRGBA color, half;

    adw_accent_color_to_rgba (accent, &color);
    put_row (s, "adw_accent_color_to_rgba", nick, NULL, "-", &color);

    for (int dark = 0; dark <= 1; dark++) {
      GdkRGBA output;

      adw_accent_color_to_standalone_rgba (accent, dark, &output);
      put_row (s, "adw_accent_color_to_standalone_rgba", nick, NULL, dark ? "1" : "0", &output);
    }

    put_standalone (s, NULL, &color);
    half = color;
    half.alpha = .5f;
    put_standalone (s, NULL, &half);
  }

  if (!g_file_set_contents (argv[1], s->str, (gssize) s->len, &error)) {
    g_printerr ("gnome-vectors: %s\n", error->message);
    g_clear_error (&error);
    status = 1;
  }

  g_type_class_unref (accent_class);
  g_string_free (s, TRUE);

  return status;
}
