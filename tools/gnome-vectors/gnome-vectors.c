/* gnome-vectors: the vector table of adw_standalone (specification S2.2 and
 * the paragraph "adw_standalone, one form", ADD-A128), by the observable
 * route: libadwaita's stylesheet has GTK compute the standalone colour, and
 * gtk_widget_get_color reads it.
 *
 *     gnome-vectors OUT.tsv
 *
 * Needs a display (run.sh starts mutter's headless one). Initialises
 * libadwaita and forces each scheme through AdwStyleManager. Each input colour
 * is named with @define-color, written as color(srgb r g b / a) from its exact
 * singles (rgb() would clamp its channels), and a label is styled
 *
 *     color: oklab(from @c var(--standalone-color-oklab));
 *
 * the stylesheet's relative colour of a named origin with the stylesheet's
 * own limit (_colors.scss:146, :151). The row's result is
 * gtk_widget_get_color's four singles, unrounded. Two more labels check each
 * row: one styled with @c alone must read back c's singles (the table's
 * header records each channel the read changes; a change above
 * READ_LIMIT fails), and one styled with the limit written out (min(l, 0.5)
 * a b in the light scheme, max(l, 0.85) a b in the dark one) must read the
 * stylesheet's result exactly. A CSS parsing error, a scheme the style
 * manager does not report, or a failed check exits 1, after writing the
 * table.
 *
 * The inputs, each in the light and the dark scheme, an input already listed
 * not repeated:
 *   1. the grid: red, green and blue each over 0, 51, 102, 153, 204 and 255
 *      (/255), alpha 1;
 *   2. every AdwAccentColor's colour (adw_accent_color_to_rgba) at alpha 1,
 *      then 0.75, then 0.5;
 *   3. the status backgrounds of each scheme, read in that scheme through
 *      @destructive_bg_color, @success_bg_color, @warning_bg_color and
 *      @error_bg_color (_colors.scss:11-22, :83-86), at alpha 1, then 0.75,
 *      then 0.5;
 *   4. out of gamut: in each scheme, the grid's result with the largest
 *      channel above 1 and the one with the smallest channel below 0, as
 *      inputs; and that smallest channel alone, the other two channels 0,
 *      whose cone responses l, m and s are all negative (linear sRGB to LMS,
 *      CSS Color 4's OKLab, has no negative coefficient), so the cube root
 *      meets negative numbers;
 *   5. L near the limits: for T the single nearest 0.5 and 0.85, greys whose
 *      L, as GTK computes it, is T, the largest L below T and the smallest L
 *      above T that GTK gives any of the 2 * PROBE_RADIUS + 1 greys around
 *      srgb(T^3) (CSS Color 4's transfer; a grey's L is the cube root of its
 *      linear value), each within 2^-24 of T, each the grey nearest srgb(T^3)
 *      with that L. GTK reports a grey's L through a label styled
 *      oklab(from @g calc((l - T) * 33554432) 0 0): L and T being singles,
 *      (l - T) * 2^25 is a whole number m, and the label's red channel is m^3
 *      through the sRGB curve, from which m is read back;
 *   6. the curve's thresholds: for 0.04045 (in) and 12.92 * 0.0031308 (out:
 *      the input whose linear value is 0.0031308, which a colour the limit
 *      leaves unchanged keeps through the round trip), the nearest single and
 *      its two neighbours v, each as the grey (v, v, v) and in each channel
 *      with the other two both 0 or both 1.
 *
 * Columns: c (srgb(r, g, b, a), the input's singles), scheme (light or dark),
 * result (srgb(r, g, b, a), gtk_widget_get_color's singles). Singles are
 * printed with %.9g, which reads back to the same single.
 */

#include <adwaita.h>
#include <math.h>
#include <stdio.h>

#define PROBE_RADIUS 64
#define PROBE_SCALE "33554432"
#define PROBE_REACH 2
#define READ_LIMIT 1e-6

static const guint grid[] = { 0, 51, 102, 153, 204, 255 };
static const float alphas[] = { 1.f, .75f, .5f };
static const char *status_names[] = {
  "destructive_bg_color", "success_bg_color", "warning_bg_color", "error_bg_color"
};
static const double lightness_targets[] = { .5, .85 };
static const double thresholds[] = { .04045, 12.92 * .0031308 };

static const char *packages[] = { "libadwaita", "gtk4", "glib2" };

enum { LIGHT, DARK, N_SCHEMES };

static const char *scheme_names[] = { "light", "dark" };
static const char *written_limits[] = { "min(l, 0.5) a b", "max(l, 0.85) a b" };

typedef enum {
  GROUP_GRID,
  GROUP_ACCENT,
  GROUP_STATUS,
  GROUP_GAMUT,
  GROUP_LIGHTNESS,
  GROUP_THRESHOLD,
  N_GROUPS
} Group;

typedef struct {
  GdkRGBA c;
  GdkRGBA result[N_SCHEMES];
} Row;

typedef struct {
  GString   *defines;
  GPtrArray *rules;
} Batch;

static GtkCssProvider *provider;
static GArray *groups[N_GROUPS];
static GString *notes;
static guint next_id;
static gboolean failed;
static guint read_changes;
static float read_change_max, read_changed_low = INFINITY, read_changed_high = -INFINITY;

static guint
to_8bit (float x)
{
  return (guint) (0.5 + CLAMP (x, 0., 1.) * 255.);
}

static void
put_hex (GString *s, const GdkRGBA *c)
{
  g_string_append_printf (s, "#%02x%02x%02x", to_8bit (c->red), to_8bit (c->green), to_8bit (c->blue));
}

static void
put_single (GString *s, float x)
{
  char buf[G_ASCII_DTOSTR_BUF_SIZE];

  if (!isfinite (x))
    failed = TRUE;
  g_string_append (s, g_ascii_formatd (buf, sizeof buf, "%.9g", x));
}

static void
put_srgb (GString *s, const GdkRGBA *c)
{
  g_string_append (s, "srgb(");
  put_single (s, c->red);
  g_string_append (s, ", ");
  put_single (s, c->green);
  g_string_append (s, ", ");
  put_single (s, c->blue);
  g_string_append (s, ", ");
  put_single (s, c->alpha);
  g_string_append_c (s, ')');
}

/* The single's exact value to 17 digits, so that CSS's number parser, reading
 * it in double, lands on the same single. */
static void
put_css_number (GString *s, float x)
{
  char buf[G_ASCII_DTOSTR_BUF_SIZE];

  g_string_append (s, g_ascii_formatd (buf, sizeof buf, "%.17g", (double) x));
}

static gboolean
rgba_identical (const GdkRGBA *a, const GdkRGBA *b)
{
  return a->red == b->red && a->green == b->green && a->blue == b->blue && a->alpha == b->alpha;
}

static void
on_parsing_error (GtkCssProvider *self,
                  GtkCssSection  *section,
                  GError         *error,
                  gpointer        user_data)
{
  char *where = gtk_css_section_to_string (section);

  g_printerr ("gnome-vectors: CSS parsing error at %s: %s\n", where, error->message);
  g_free (where);
  failed = TRUE;
}

static void
batch_init (Batch *b)
{
  b->defines = g_string_new (NULL);
  b->rules = g_ptr_array_new_with_free_func (g_free);
}

static void
batch_clear (Batch *b)
{
  g_string_free (b->defines, TRUE);
  g_ptr_array_unref (b->rules);
}

static guint
batch_define (Batch *b, const GdkRGBA *c)
{
  guint id = next_id++;

  g_string_append_printf (b->defines, "@define-color nt_%u color(srgb ", id);
  put_css_number (b->defines, c->red);
  g_string_append_c (b->defines, ' ');
  put_css_number (b->defines, c->green);
  g_string_append_c (b->defines, ' ');
  put_css_number (b->defines, c->blue);
  g_string_append (b->defines, " / ");
  put_css_number (b->defines, c->alpha);
  g_string_append (b->defines, ");\n");

  return id;
}

/* Styles one label per rule (the rules' class names never repeat, so no
 * cached style is reused) and reads each one's colour into out. */
static void
batch_run (Batch *b, GdkRGBA *out)
{
  GString *css = g_string_new (b->defines->str);
  guint base = next_id;

  next_id += b->rules->len;
  for (guint i = 0; i < b->rules->len; i++)
    g_string_append_printf (css, ".nt-%u { color: %s; }\n", base + i, (const char *) b->rules->pdata[i]);
  gtk_css_provider_load_from_string (provider, css->str);
  g_string_free (css, TRUE);

  for (guint i = 0; i < b->rules->len; i++) {
    GtkWidget *label = g_object_ref_sink (gtk_label_new (NULL));
    char name[32];

    g_snprintf (name, sizeof name, "nt-%u", base + i);
    gtk_widget_add_css_class (label, name);
    gtk_widget_get_color (label, &out[i]);
    g_object_unref (label);
  }
}

static void
force_scheme (int scheme)
{
  AdwStyleManager *manager = adw_style_manager_get_default ();

  adw_style_manager_set_color_scheme (manager,
                                      scheme == DARK ? ADW_COLOR_SCHEME_FORCE_DARK : ADW_COLOR_SCHEME_FORCE_LIGHT);
  while (g_main_context_iteration (NULL, FALSE))
    ;
  if (adw_style_manager_get_dark (manager) != (scheme == DARK)) {
    g_printerr ("gnome-vectors: AdwStyleManager does not report the %s scheme\n", scheme_names[scheme]);
    failed = TRUE;
  }
}

static void
add_input (Group group, float r, float g, float b, float a)
{
  Row row = { .c = { r, g, b, a } };

  for (int i = 0; i < N_GROUPS; i++)
    for (guint j = 0; j < groups[i]->len; j++)
      if (rgba_identical (&g_array_index (groups[i], Row, j).c, &row.c))
        return;
  g_array_append_val (groups[group], row);
}

static void
compute_group (Group group)
{
  GArray *rows = groups[group];

  for (int scheme = 0; scheme < N_SCHEMES; scheme++) {
    Batch b;
    GdkRGBA *out = g_new (GdkRGBA, 3 * rows->len);

    force_scheme (scheme);
    batch_init (&b);
    for (guint i = 0; i < rows->len; i++) {
      guint id = batch_define (&b, &g_array_index (rows, Row, i).c);

      g_ptr_array_add (b.rules, g_strdup_printf ("@nt_%u", id));
      g_ptr_array_add (b.rules, g_strdup_printf ("oklab(from @nt_%u var(--standalone-color-oklab))", id));
      g_ptr_array_add (b.rules, g_strdup_printf ("oklab(from @nt_%u %s)", id, written_limits[scheme]));
    }
    batch_run (&b, out);

    for (guint i = 0; i < rows->len; i++) {
      Row *row = &g_array_index (rows, Row, i);
      GString *s = g_string_new (NULL);

      put_srgb (s, &row->c);
      if (!rgba_identical (&out[3 * i], &row->c)) {
        const float in[] = { row->c.red, row->c.green, row->c.blue, row->c.alpha };
        const float back[] = { out[3 * i].red, out[3 * i].green, out[3 * i].blue, out[3 * i].alpha };

        for (gsize k = 0; k < G_N_ELEMENTS (in); k++) {
          if (in[k] == back[k])
            continue;
          read_changes++;
          read_change_max = MAX (read_change_max, fabsf (in[k] - back[k]));
          read_changed_low = MIN (read_changed_low, in[k]);
          read_changed_high = MAX (read_changed_high, in[k]);
          if (fabsf (in[k] - back[k]) > READ_LIMIT) {
            g_string_append (s, " named reads back as ");
            put_srgb (s, &out[3 * i]);
            g_printerr ("gnome-vectors: %s %s\n", scheme_names[scheme], s->str);
            failed = TRUE;
            break;
          }
        }
      }
      row->result[scheme] = out[3 * i + 1];
      if (!rgba_identical (&out[3 * i + 1], &out[3 * i + 2])) {
        g_string_truncate (s, 0);
        put_srgb (s, &row->c);
        g_string_append (s, ": the stylesheet's limit gives ");
        put_srgb (s, &out[3 * i + 1]);
        g_string_append_printf (s, ", %s gives ", written_limits[scheme]);
        put_srgb (s, &out[3 * i + 2]);
        g_printerr ("gnome-vectors: %s %s\n", scheme_names[scheme], s->str);
        failed = TRUE;
      }
      g_string_free (s, TRUE);
    }

    batch_clear (&b);
    g_free (out);
  }
}

static void
add_grid (void)
{
  for (gsize r = 0; r < G_N_ELEMENTS (grid); r++)
    for (gsize g = 0; g < G_N_ELEMENTS (grid); g++)
      for (gsize b = 0; b < G_N_ELEMENTS (grid); b++)
        add_input (GROUP_GRID, grid[r] / 255.f, grid[g] / 255.f, grid[b] / 255.f, 1.f);
}

static void
add_accents (void)
{
  GEnumClass *accent_class = g_type_class_ref (ADW_TYPE_ACCENT_COLOR);

  for (gsize a = 0; a < G_N_ELEMENTS (alphas); a++) {
    for (guint i = 0; i < accent_class->n_values; i++) {
      GdkRGBA color;

      adw_accent_color_to_rgba (accent_class->values[i].value, &color);
      add_input (GROUP_ACCENT, color.red, color.green, color.blue, alphas[a]);
    }
  }

  g_type_class_unref (accent_class);
}

static void
add_status (void)
{
  GdkRGBA read[N_SCHEMES][G_N_ELEMENTS (status_names)];

  g_string_append (notes, "# Group 3, the status backgrounds read:");
  for (int scheme = 0; scheme < N_SCHEMES; scheme++) {
    Batch b;

    force_scheme (scheme);
    batch_init (&b);
    for (gsize i = 0; i < G_N_ELEMENTS (status_names); i++)
      g_ptr_array_add (b.rules, g_strdup_printf ("@%s", status_names[i]));
    batch_run (&b, read[scheme]);
    batch_clear (&b);

    g_string_append_printf (notes, "%s %s", scheme ? ";" : "", scheme_names[scheme]);
    for (gsize i = 0; i < G_N_ELEMENTS (status_names); i++) {
      g_string_append_printf (notes, " @%s ", status_names[i]);
      put_hex (notes, &read[scheme][i]);
    }
  }
  g_string_append (notes, ".\n");

  for (gsize a = 0; a < G_N_ELEMENTS (alphas); a++)
    for (int scheme = 0; scheme < N_SCHEMES; scheme++)
      for (gsize i = 0; i < G_N_ELEMENTS (status_names); i++)
        add_input (GROUP_STATUS, read[scheme][i].red, read[scheme][i].green, read[scheme][i].blue, alphas[a]);
}

static void
add_gamut (void)
{
  GArray *grid_rows = groups[GROUP_GRID];

  for (int scheme = 0; scheme < N_SCHEMES; scheme++) {
    const Row *high = NULL, *low = NULL;
    float highest = 1.f, lowest = 0.f;
    gsize low_channel = 0;

    for (guint i = 0; i < grid_rows->len; i++) {
      const Row *row = &g_array_index (grid_rows, Row, i);
      const float channels[] = { row->result[scheme].red, row->result[scheme].green, row->result[scheme].blue };

      for (gsize k = 0; k < G_N_ELEMENTS (channels); k++) {
        if (channels[k] > highest) {
          highest = channels[k];
          high = row;
        }
        if (channels[k] < lowest) {
          lowest = channels[k];
          low = row;
          low_channel = k;
        }
      }
    }
    if (high)
      add_input (GROUP_GAMUT, high->result[scheme].red, high->result[scheme].green, high->result[scheme].blue,
                 high->result[scheme].alpha);
    if (low) {
      add_input (GROUP_GAMUT, low->result[scheme].red, low->result[scheme].green, low->result[scheme].blue,
                 low->result[scheme].alpha);
      add_input (GROUP_GAMUT, low_channel == 0 ? lowest : 0.f, low_channel == 1 ? lowest : 0.f,
                 low_channel == 2 ? lowest : 0.f, 1.f);
    }
    g_string_append_printf (notes, "# Group 4, the %s scheme: the grid's largest channel ", scheme_names[scheme]);
    put_single (notes, highest);
    g_string_append (notes, high ? "" : " (none above 1)");
    g_string_append (notes, ", its smallest ");
    put_single (notes, lowest);
    g_string_append (notes, low ? "" : " (none below 0)");
    if (low)
      g_string_append_printf (notes, ", in the %s channel", (const char *[]) { "red", "green", "blue" }[low_channel]);
    g_string_append (notes, ".\n");
  }
}

/* CSS Color 4's sRGB transfer, for the search's start and the probe's reading
 * only; no row is computed with it. */
static double
srgb_of_linear (double x)
{
  return x <= .0031308 ? 12.92 * x : 1.055 * pow (x, 1. / 2.4) - .055;
}

static double
linear_of_srgb (double x)
{
  return x <= .04045 ? x / 12.92 : pow ((x + .055) / 1.055, 2.4);
}

static void
add_lightness (void)
{
  for (gsize t = 0; t < G_N_ELEMENTS (lightness_targets); t++) {
    const float target = (float) lightness_targets[t];
    float candidates[2 * PROBE_RADIUS + 1];
    double offsets[2 * PROBE_RADIUS + 1];
    double wanted[] = { -INFINITY, 0., INFINITY };
    GdkRGBA out[2 * PROBE_RADIUS + 1];
    GString *t_css = g_string_new (NULL);
    Batch b;

    candidates[0] = (float) srgb_of_linear (pow (lightness_targets[t], 3.));
    for (int j = 1; j <= PROBE_RADIUS; j++) {
      candidates[2 * j - 1] = nextafterf (j == 1 ? candidates[0] : candidates[2 * j - 3], 2.f);
      candidates[2 * j] = nextafterf (j == 1 ? candidates[0] : candidates[2 * j - 2], -1.f);
    }

    put_css_number (t_css, target);
    batch_init (&b);
    for (gsize j = 0; j < G_N_ELEMENTS (candidates); j++) {
      GdkRGBA grey = { candidates[j], candidates[j], candidates[j], 1.f };
      guint id = batch_define (&b, &grey);

      g_ptr_array_add (b.rules, g_strdup_printf ("oklab(from @nt_%u calc((l - %s) * " PROBE_SCALE ") 0 0)",
                                                 id, t_css->str));
    }
    batch_run (&b, out);
    batch_clear (&b);

    for (gsize j = 0; j < G_N_ELEMENTS (candidates); j++) {
      double m = cbrt (linear_of_srgb (out[j].red));

      offsets[j] = round (m);
      if (fabs (m - offsets[j]) > .01) {
        g_printerr ("gnome-vectors: the L probe of grey %.9g (T %s) reads %.9g, no whole number\n",
                    candidates[j], t_css->str, out[j].red);
        failed = TRUE;
      }
      if (offsets[j] < 0.)
        wanted[0] = MAX (wanted[0], offsets[j]);
      if (offsets[j] > 0.)
        wanted[2] = MIN (wanted[2], offsets[j]);
    }

    g_string_append_printf (notes, "# Group 5, T %s: greys", t_css->str);
    for (gsize w = 0; w < G_N_ELEMENTS (wanted); w++) {
      const double m = wanted[w];
      gsize j;

      for (j = 0; j < G_N_ELEMENTS (candidates); j++)
        if (offsets[j] == m)
          break;
      if (j == G_N_ELEMENTS (candidates) || fabs (m) > PROBE_REACH) {
        g_printerr ("gnome-vectors: no grey within %d singles has L within 2^-24 of T %s on side %d (m %g)\n",
                    PROBE_RADIUS, t_css->str, (int) w - 1, m);
        failed = TRUE;
        continue;
      }
      add_input (GROUP_LIGHTNESS, candidates[j], candidates[j], candidates[j], 1.f);
      g_string_append (notes, w ? ", " : " ");
      put_single (notes, candidates[j]);
      g_string_append (notes, " (L ");
      put_single (notes, (float) (target + ldexp (m, -25)));
      g_string_append_printf (notes, ", (L - T) 2^25 = %+.0f)", m);
    }
    g_string_append (notes, "; the grey nearest srgb(T^3) is ");
    put_single (notes, candidates[0]);
    g_string_append (notes, ".\n");
    g_string_free (t_css, TRUE);
  }
}

static void
add_thresholds (void)
{
  for (gsize t = 0; t < G_N_ELEMENTS (thresholds); t++) {
    const float nearest = (float) thresholds[t];
    const float values[] = { nextafterf (nearest, 0.f), nearest, nextafterf (nearest, 1.f) };

    for (gsize v = 0; v < G_N_ELEMENTS (values); v++) {
      const float x = values[v];

      add_input (GROUP_THRESHOLD, x, x, x, 1.f);
      for (int other = 0; other <= 1; other++) {
        add_input (GROUP_THRESHOLD, x, other, other, 1.f);
        add_input (GROUP_THRESHOLD, other, x, other, 1.f);
        add_input (GROUP_THRESHOLD, other, other, x, 1.f);
      }
    }
  }
}

static void
put_header (GString *s)
{
  static const char *descriptions[N_GROUPS] = {
    "the grid, red, green and blue each over 0, 51, 102, 153, 204 and 255 (/255), alpha 1",
    "every AdwAccentColor's colour (adw_accent_color_to_rgba) at alpha 1, then 0.75, then 0.5",
    "the status backgrounds of each scheme (_colors.scss:11-22, :83-86), read in that scheme through "
      "@destructive_bg_color, @success_bg_color, @warning_bg_color and @error_bg_color, at alpha 1, then 0.75, "
      "then 0.5",
    "out of gamut, in each scheme the grid's result with the largest channel above 1 and the one with the "
      "smallest channel below 0, as inputs, and that smallest channel alone with the other two 0, whose cone "
      "responses l, m and s are all negative (linear sRGB to LMS, CSS Color 4's OKLab, has no negative "
      "coefficient), so the cube root meets negative numbers",
    "L near the limits, for T the single nearest 0.5 and 0.85 greys whose L, as GTK computes it, is T, the "
      "largest L below T and the smallest L above T that GTK gives any of the 129 greys (singles) around "
      "srgb(T^3) (CSS Color 4's transfer), each within 2^-24 of T, each the grey nearest srgb(T^3) with that L; "
      "GTK reports a grey's L by styling oklab(from @g calc((l - T) * 33554432) 0 0), whose red channel is "
      "(L - T) 2^25, a whole number, cubed and through the sRGB curve",
    "the curve's thresholds, for 0.04045 (in) and 12.92 x 0.0031308 (out, the input whose linear value is "
      "0.0031308, which a colour the limit leaves unchanged keeps through the round trip) the nearest single and "
      "its two neighbours v, as the grey (v, v, v) and in each channel with the other two both 0 or both 1",
  };
  char *out = NULL;
  int wait_status = 0;
  const char *argv[G_N_ELEMENTS (packages) + 3] = { "pacman", "-Q" };

  for (gsize i = 0; i < G_N_ELEMENTS (packages); i++)
    argv[i + 2] = packages[i];

  g_string_append (s, "# function: adw_standalone\n"
                      "# route: observable_behaviour\n"
                      "# source: gtk@4.24.1\n"
                      "# source: libadwaita@1.10.0\n"
                      "# generations: adwaita\n"
                      "# generator: tools/gnome-vectors\n"
                      "# inputs: c:colour scheme:enum<Scheme>\n"
                      "# outputs: result:colour\n");
  g_string_append_printf (s, "# written by tools/gnome-vectors on libadwaita %u.%u.%u and GTK %u.%u.%u; do not edit.",
                          adw_get_major_version (), adw_get_minor_version (), adw_get_micro_version (),
                          gtk_get_major_version (), gtk_get_minor_version (), gtk_get_micro_version ());
  if (g_spawn_sync (NULL, (char **) argv, NULL, G_SPAWN_SEARCH_PATH | G_SPAWN_STDERR_TO_DEV_NULL,
                    NULL, NULL, &out, NULL, &wait_status, NULL) && out) {
    char **lines = g_strsplit (g_strstrip (out), "\n", -1);
    char *joined = g_strjoinv (", ", lines);

    g_string_append_printf (s, " The packages (pacman -Q) %s.\n", joined);
    g_free (joined);
    g_strfreev (lines);
  } else {
    g_string_append (s, " The packages (pacman -Q) not read.\n");
  }
  g_free (out);

  g_string_append (s, "# Each row: c named by @define-color as color(srgb r g b / a) from its singles, a label styled "
                      "color: oklab(from @c var(--standalone-color-oklab)) under AdwStyleManager's forced scheme, "
                      "and gtk_widget_get_color's four singles as the result, unrounded (%.9g, which reads back "
                      "to the same single). Each input in both schemes, an input already listed not repeated.\n");
  for (int i = 0; i < N_GROUPS; i++)
    g_string_append_printf (s, "# Group %d (%u rows): %s.\n", i + 1, groups[i]->len * N_SCHEMES, descriptions[i]);
  g_string_append (s, notes->str);
  if (read_changes) {
    g_string_append_printf (s, "# The check reading c alone through gtk_widget_get_color: %u channels of c read "
                               "back changed, the inputs' channels between ", read_changes);
    put_single (s, read_changed_low);
    g_string_append (s, " and ");
    put_single (s, read_changed_high);
    g_string_append (s, ", by at most ");
    put_single (s, read_change_max);
    g_string_append (s, ".\n");
  } else {
    g_string_append (s, "# The check reading c alone through gtk_widget_get_color read back every c exactly.\n");
  }
  g_string_append (s, "c\tscheme\tresult\n");
}

int
main (int    argc,
      char **argv)
{
  GString *s = g_string_new (NULL);
  GError *error = NULL;

  if (argc != 2) {
    g_printerr ("usage: gnome-vectors OUT.tsv\n");
    return 1;
  }

  adw_init ();
  provider = gtk_css_provider_new ();
  g_signal_connect (provider, "parsing-error", G_CALLBACK (on_parsing_error), NULL);
  gtk_style_context_add_provider_for_display (gdk_display_get_default (), GTK_STYLE_PROVIDER (provider),
                                              GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
  notes = g_string_new (NULL);
  for (int i = 0; i < N_GROUPS; i++)
    groups[i] = g_array_new (FALSE, FALSE, sizeof (Row));

  add_grid ();
  add_accents ();
  add_status ();
  add_lightness ();
  add_thresholds ();
  for (int i = 0; i < N_GROUPS; i++)
    if (i != GROUP_GAMUT)
      compute_group (i);
  add_gamut ();
  compute_group (GROUP_GAMUT);

  put_header (s);
  for (int i = 0; i < N_GROUPS; i++) {
    for (guint j = 0; j < groups[i]->len; j++) {
      const Row *row = &g_array_index (groups[i], Row, j);

      for (int scheme = 0; scheme < N_SCHEMES; scheme++) {
        put_srgb (s, &row->c);
        g_string_append_printf (s, "\t%s\t", scheme_names[scheme]);
        put_srgb (s, &row->result[scheme]);
        g_string_append_c (s, '\n');
      }
    }
  }

  if (!g_file_set_contents (argv[1], s->str, (gssize) s->len, &error)) {
    g_printerr ("gnome-vectors: %s\n", error->message);
    g_clear_error (&error);
    failed = TRUE;
  }

  for (int i = 0; i < N_GROUPS; i++)
    g_array_unref (groups[i]);
  g_string_free (notes, TRUE);
  g_string_free (s, TRUE);
  g_object_unref (provider);

  return failed ? 1 : 0;
}
