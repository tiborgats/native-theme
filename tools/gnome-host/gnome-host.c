/* gnome-host: the GNOME measurement host of the 0.8.0 plan (D4; spike G).
 *
 * Renders libadwaita's buttons per style class and state to PNG files and
 * writes an index.toml beside them that records what was rendered and with
 * which versions and settings. It pins what libadwaita's documentation
 * screenshot tool pins (doc/tools/screenshot.c at 1.10.0): the system colour
 * scheme, contrast and accent through ADW_DEBUG_COLOR_SCHEME=default,
 * ADW_DEBUG_HIGH_CONTRAST=0 and ADW_DEBUG_ACCENT_COLOR=blue, then the scheme
 * forced light or dark through AdwStyleManager; GtkSettings' animations off,
 * decoration layout ":close", font "Adwaita Sans 11", font rendering manual
 * with hinted font metrics, and an empty icon theme name. As libadwaita's own
 * tests do (tests/meson.build), GDK_DEBUG=no-portals keeps portal settings
 * out and GSETTINGS_BACKEND=memory gives gsettings-desktop-schemas' defaults;
 * ADW_DISABLE_PORTAL=1 keeps libadwaita's own settings from the portal too.
 * GSK_RENDERER is set from --renderer. Each widget is drawn as screenshot.c
 * draws one: a GtkWidgetPaintable snapshotted and rendered to a texture by the
 * window's GskRenderer. A widget's paintable clips to the widget's bounds,
 * which would cut what a button draws outside them (an outer focus ring), so
 * each button sits in a cell of its own, inside the cell's margin, and the
 * cell is drawn.
 *
 * index.toml also records the colour a label with libadwaita's `accent` style
 * class gets from the stylesheet (`--accent-color`, derived from
 * `--accent-bg-color` in CSS) beside what adw_rgba_to_standalone returns for
 * the style manager's accent, to compare the stylesheet's derivation with the
 * public function.
 *
 *     gnome-host --scheme light|dark --renderer NAME --out DIR
 *
 * Exits 0 when every image was written with the state flags it asked for,
 * 3 when a widget rendered with other flags (index.toml records both), 1 on
 * any other failure.
 */

#include <adwaita.h>
#include <math.h>
#include <string.h>

#define MARGIN 8
#define LABEL "Button"
#define FRAMES_BEFORE_RENDER 5
#define TIMEOUT_SECONDS 30

typedef struct {
  const char *name;
  const char *css_class;
} Kind;

typedef struct {
  const char *name;
  GtkStateFlags flags;
  gboolean toggle_active;
  gboolean insensitive;
} State;

static const Kind kinds[] = {
  { "plain", NULL },
  { "flat", "flat" },
  { "suggested-action", "suggested-action" },
  { "destructive-action", "destructive-action" },
};

static const State states[] = {
  { "normal", 0, FALSE, FALSE },
  { "hover", GTK_STATE_FLAG_PRELIGHT, FALSE, FALSE },
  { "active", GTK_STATE_FLAG_ACTIVE, FALSE, FALSE },
  { "checked", 0, TRUE, FALSE },
  { "disabled", 0, FALSE, TRUE },
  { "focus-visible", GTK_STATE_FLAG_FOCUSED | GTK_STATE_FLAG_FOCUS_VISIBLE, FALSE, FALSE },
};

#define N_KINDS G_N_ELEMENTS (kinds)
#define N_STATES G_N_ELEMENTS (states)

static const struct {
  GtkStateFlags flag;
  const char *name;
} flag_names[] = {
  { GTK_STATE_FLAG_ACTIVE, "active" },
  { GTK_STATE_FLAG_PRELIGHT, "prelight" },
  { GTK_STATE_FLAG_SELECTED, "selected" },
  { GTK_STATE_FLAG_INSENSITIVE, "insensitive" },
  { GTK_STATE_FLAG_INCONSISTENT, "inconsistent" },
  { GTK_STATE_FLAG_FOCUSED, "focused" },
  { GTK_STATE_FLAG_BACKDROP, "backdrop" },
  { GTK_STATE_FLAG_DIR_LTR, "dir-ltr" },
  { GTK_STATE_FLAG_DIR_RTL, "dir-rtl" },
  { GTK_STATE_FLAG_LINK, "link" },
  { GTK_STATE_FLAG_VISITED, "visited" },
  { GTK_STATE_FLAG_CHECKED, "checked" },
  { GTK_STATE_FLAG_DROP_ACTIVE, "drop-active" },
  { GTK_STATE_FLAG_FOCUS_VISIBLE, "focus-visible" },
  { GTK_STATE_FLAG_FOCUS_WITHIN, "focus-within" },
};

static const char *packages[] = {
  "libadwaita", "gtk4", "gsettings-desktop-schemas", "adwaita-fonts",
  "glib2", "pango", "cairo", "harfbuzz", "freetype2", "fontconfig",
  "librsvg", "gdk-pixbuf2", "libpng", "mesa", "libglvnd", "mutter",
  "xorg-server-xvfb",
};

static char *opt_scheme = NULL;
static char *opt_renderer = NULL;
static char *opt_out = NULL;

static GOptionEntry entries[] = {
  { "scheme", 0, 0, G_OPTION_ARG_STRING, &opt_scheme, "light or dark", "SCHEME" },
  { "renderer", 0, 0, G_OPTION_ARG_STRING, &opt_renderer, "the GSK_RENDERER value", "NAME" },
  { "out", 0, 0, G_OPTION_ARG_FILENAME, &opt_out, "the output directory", "DIR" },
  { NULL }
};

typedef struct {
  gboolean dark;
  GtkWidget *window;
  GtkWidget *buttons[N_KINDS][N_STATES];
  GtkWidget *cells[N_KINDS][N_STATES];
  GtkWidget *accent_label;
  guint frames;
  guint timeout_id;
  int status;
} Host;

static void
toml_string (GString *s, const char *value)
{
  g_string_append_c (s, '"');
  for (const char *p = value ? value : ""; *p; p++) {
    switch (*p) {
    case '"': g_string_append (s, "\\\""); break;
    case '\\': g_string_append (s, "\\\\"); break;
    case '\n': g_string_append (s, "\\n"); break;
    case '\t': g_string_append (s, "\\t"); break;
    default:
      if ((guchar) *p < 0x20)
        g_string_append_printf (s, "\\u%04x", (guint) (guchar) *p);
      else
        g_string_append_c (s, *p);
    }
  }
  g_string_append_c (s, '"');
}

static void
toml_kv_string (GString *s, const char *key, const char *value)
{
  g_string_append_printf (s, "%s = ", key);
  toml_string (s, value);
  g_string_append_c (s, '\n');
}

static void
toml_flags (GString *s, const char *key, GtkStateFlags flags)
{
  gboolean first = TRUE;

  g_string_append_printf (s, "%s = [", key);
  for (gsize i = 0; i < G_N_ELEMENTS (flag_names); i++) {
    if (!(flags & flag_names[i].flag))
      continue;
    if (!first)
      g_string_append (s, ", ");
    toml_string (s, flag_names[i].name);
    first = FALSE;
  }
  g_string_append (s, "]\n");
}

static char *
rgba_hex (const GdkRGBA *c)
{
  return g_strdup_printf ("#%02x%02x%02x%02x",
                          (guint) (0.5 + CLAMP (c->red, 0., 1.) * 255.),
                          (guint) (0.5 + CLAMP (c->green, 0., 1.) * 255.),
                          (guint) (0.5 + CLAMP (c->blue, 0., 1.) * 255.),
                          (guint) (0.5 + CLAMP (c->alpha, 0., 1.) * 255.));
}

static void
append_packages (GString *s)
{
  GPtrArray *argv = g_ptr_array_new ();
  char *out = NULL;
  char *err = NULL;
  int wait_status = 0;
  GError *error = NULL;

  g_ptr_array_add (argv, (gpointer) "pacman");
  g_ptr_array_add (argv, (gpointer) "-Q");
  for (gsize i = 0; i < G_N_ELEMENTS (packages); i++)
    g_ptr_array_add (argv, (gpointer) packages[i]);
  g_ptr_array_add (argv, NULL);

  g_string_append (s, "\n[packages]\n");
  if (!g_spawn_sync (NULL, (char **) argv->pdata, NULL, G_SPAWN_SEARCH_PATH,
                     NULL, NULL, &out, &err, &wait_status, &error)) {
    toml_kv_string (s, "error", error->message);
    g_clear_error (&error);
  } else {
    char **lines = g_strsplit (out, "\n", -1);

    for (char **line = lines; *line; line++) {
      char **fields = g_strsplit (*line, " ", 2);

      if (fields[0] && fields[1]) {
        toml_string (s, fields[0]);
        g_string_append (s, " = ");
        toml_string (s, fields[1]);
        g_string_append_c (s, '\n');
      }
      g_strfreev (fields);
    }
    g_strfreev (lines);
    if (err && *err)
      toml_kv_string (s, "pacman_stderr", g_strstrip (err));
  }

  g_free (out);
  g_free (err);
  g_ptr_array_unref (argv);
}

static void
append_settings (GString *s)
{
  GtkSettings *settings = gtk_settings_get_default ();
  char *font_name = NULL, *hintstyle = NULL, *rgba = NULL, *theme = NULL;
  char *icon_theme = NULL, *layout = NULL;
  int antialias = 0, hinting = 0, dpi = 0;
  gboolean animations = FALSE, hint_metrics = FALSE;
  GtkFontRendering rendering = GTK_FONT_RENDERING_AUTOMATIC;
  GEnumClass *rendering_class = g_type_class_ref (GTK_TYPE_FONT_RENDERING);
  GEnumValue *rendering_value;

  g_object_get (settings,
                "gtk-font-name", &font_name,
                "gtk-font-rendering", &rendering,
                "gtk-hint-font-metrics", &hint_metrics,
                "gtk-xft-antialias", &antialias,
                "gtk-xft-hinting", &hinting,
                "gtk-xft-hintstyle", &hintstyle,
                "gtk-xft-rgba", &rgba,
                "gtk-xft-dpi", &dpi,
                "gtk-enable-animations", &animations,
                "gtk-theme-name", &theme,
                "gtk-icon-theme-name", &icon_theme,
                "gtk-decoration-layout", &layout,
                NULL);
  rendering_value = g_enum_get_value (rendering_class, rendering);

  g_string_append (s, "\n[settings]\n");
  toml_kv_string (s, "gtk-font-name", font_name);
  toml_kv_string (s, "gtk-font-rendering", rendering_value ? rendering_value->value_nick : "unknown");
  g_string_append_printf (s, "gtk-hint-font-metrics = %s\n", hint_metrics ? "true" : "false");
  g_string_append_printf (s, "gtk-xft-antialias = %d\n", antialias);
  g_string_append_printf (s, "gtk-xft-hinting = %d\n", hinting);
  toml_kv_string (s, "gtk-xft-hintstyle", hintstyle);
  toml_kv_string (s, "gtk-xft-rgba", rgba);
  g_string_append_printf (s, "gtk-xft-dpi = %d\n", dpi);
  g_string_append_printf (s, "gtk-enable-animations = %s\n", animations ? "true" : "false");
  toml_kv_string (s, "gtk-theme-name", theme);
  toml_kv_string (s, "gtk-icon-theme-name", icon_theme);
  toml_kv_string (s, "gtk-decoration-layout", layout);

  g_type_class_unref (rendering_class);
  g_free (font_name);
  g_free (hintstyle);
  g_free (rgba);
  g_free (theme);
  g_free (icon_theme);
  g_free (layout);
}

static void
append_style_manager (GString *s)
{
  AdwStyleManager *manager = adw_style_manager_get_default ();
  GEnumClass *accent_class = g_type_class_ref (ADW_TYPE_ACCENT_COLOR);
  GEnumClass *scheme_class = g_type_class_ref (ADW_TYPE_COLOR_SCHEME);
  GEnumValue *accent = g_enum_get_value (accent_class, adw_style_manager_get_accent_color (manager));
  GEnumValue *scheme = g_enum_get_value (scheme_class, adw_style_manager_get_color_scheme (manager));
  GdkRGBA *accent_rgba = adw_style_manager_get_accent_color_rgba (manager);
  char *hex = rgba_hex (accent_rgba);

  g_string_append (s, "\n[style_manager]\n");
  toml_kv_string (s, "color_scheme", scheme ? scheme->value_nick : "unknown");
  g_string_append_printf (s, "dark = %s\n", adw_style_manager_get_dark (manager) ? "true" : "false");
  g_string_append_printf (s, "high_contrast = %s\n", adw_style_manager_get_high_contrast (manager) ? "true" : "false");
  toml_kv_string (s, "accent_color", accent ? accent->value_nick : "unknown");
  toml_kv_string (s, "accent_color_rgba", hex);
  g_string_append_printf (s, "system_supports_color_schemes = %s\n",
                          adw_style_manager_get_system_supports_color_schemes (manager) ? "true" : "false");
  g_string_append_printf (s, "system_supports_accent_colors = %s\n",
                          adw_style_manager_get_system_supports_accent_colors (manager) ? "true" : "false");

  g_free (hex);
  gdk_rgba_free (accent_rgba);
  g_type_class_unref (accent_class);
  g_type_class_unref (scheme_class);
}

static void
append_rgba (GString *s, const char *key, const GdkRGBA *c)
{
  char *hex = rgba_hex (c);

  g_string_append_printf (s, "%s = ", key);
  toml_string (s, hex);
  g_string_append_printf (s, "\n%s_floats = [%.9g, %.9g, %.9g, %.9g]\n", key, c->red, c->green, c->blue, c->alpha);
  g_free (hex);
}

static void
append_accent_check (GString *s, Host *host)
{
  AdwStyleManager *manager = adw_style_manager_get_default ();
  GdkRGBA *accent = adw_style_manager_get_accent_color_rgba (manager);
  GdkRGBA label_color, standalone;

  gtk_widget_get_color (host->accent_label, &label_color);
  adw_rgba_to_standalone (accent, adw_style_manager_get_dark (manager), &standalone);

  g_string_append (s, "\n[accent_check]\n");
  append_rgba (s, "accent_bg", accent);
  append_rgba (s, "accent_label_color", &label_color);
  append_rgba (s, "adw_rgba_to_standalone", &standalone);
  g_string_append_printf (s, "equal = %s\n", gdk_rgba_equal (&label_color, &standalone) ? "true" : "false");

  gdk_rgba_free (accent);
}

static gboolean
render_to_png (GtkWidget  *widget,
               GskRenderer *renderer,
               const char *path,
               int        *out_width,
               int        *out_height)
{
  graphene_rect_t bounds;
  GdkPaintable *paintable;
  GtkSnapshot *snapshot;
  GskRenderNode *node;
  GdkTexture *texture;
  int x, y, width, height;
  gboolean ok;

  *out_width = 0;
  *out_height = 0;

  if (!gtk_widget_compute_bounds (widget, widget, &bounds)) {
    g_printerr ("gnome-host: no bounds for %s\n", path);
    return FALSE;
  }

  if (GTK_IS_NATIVE (widget)) {
    GdkSurface *surface = gtk_native_get_surface (GTK_NATIVE (widget));
    double tx, ty;

    gtk_native_get_surface_transform (GTK_NATIVE (widget), &tx, &ty);
    x = (int) floor (tx);
    y = (int) floor (ty);
    width = gdk_surface_get_width (surface);
    height = gdk_surface_get_height (surface);
  } else {
    x = gtk_widget_get_margin_start (widget);
    y = gtk_widget_get_margin_top (widget);
    width = (int) ceil (bounds.size.width) + x + gtk_widget_get_margin_end (widget);
    height = (int) ceil (bounds.size.height) + y + gtk_widget_get_margin_bottom (widget);
  }

  paintable = gtk_widget_paintable_new (widget);
  snapshot = gtk_snapshot_new ();
  gtk_snapshot_translate (snapshot, &GRAPHENE_POINT_INIT (x, y));
  gdk_paintable_snapshot (paintable, snapshot, bounds.size.width, bounds.size.height);
  node = gtk_snapshot_free_to_node (snapshot);
  g_object_unref (paintable);

  if (node == NULL) {
    g_printerr ("gnome-host: %s drew nothing\n", path);
    return FALSE;
  }

  texture = gsk_renderer_render_texture (renderer, node, &GRAPHENE_RECT_INIT (0, 0, width, height));
  gsk_render_node_unref (node);

  ok = gdk_texture_save_to_png (texture, path);
  if (!ok)
    g_printerr ("gnome-host: saving %s failed\n", path);

  *out_width = gdk_texture_get_width (texture);
  *out_height = gdk_texture_get_height (texture);
  g_object_unref (texture);

  return ok;
}

static GtkStateFlags
expected_flags (const State *state)
{
  GtkStateFlags flags = state->flags;

  if (state->toggle_active)
    flags |= GTK_STATE_FLAG_CHECKED;
  if (state->insensitive)
    flags |= GTK_STATE_FLAG_INSENSITIVE;

  return flags;
}

static void
render_all (Host *host)
{
  GskRenderer *renderer = gtk_native_get_renderer (GTK_NATIVE (host->window));
  GdkDisplay *display = gtk_widget_get_display (host->window);
  GdkSurface *surface = gtk_native_get_surface (GTK_NATIVE (host->window));
  GString *s = g_string_new (NULL);
  GError *error = NULL;
  char *index_path;
  char *gtk_version, *adw_version;
  int width, height;
  gboolean all_as_requested = TRUE;
  GString *images = g_string_new (NULL);

  for (gsize k = 0; k < N_KINDS; k++) {
    for (gsize st = 0; st < N_STATES; st++) {
      GtkWidget *button = host->buttons[k][st];
      GtkStateFlags rendered = gtk_widget_get_state_flags (button);
      GtkStateFlags relevant = rendered & ~(GTK_STATE_FLAG_DIR_LTR | GTK_STATE_FLAG_DIR_RTL);
      GtkStateFlags expected = expected_flags (&states[st]);
      gboolean as_requested = relevant == expected;
      char *file = g_strdup_printf ("%s-%s.png", kinds[k].name, states[st].name);
      char *path = g_build_filename (opt_out, file, NULL);

      if (!render_to_png (host->cells[k][st], renderer, path, &width, &height))
        host->status = 1;

      if (!as_requested) {
        all_as_requested = FALSE;
        g_printerr ("gnome-host: %s rendered with other state flags than requested\n", file);
      }

      g_string_append (images, "\n[[image]]\n");
      toml_kv_string (images, "file", file);
      toml_kv_string (images, "kind", kinds[k].name);
      toml_kv_string (images, "state", states[st].name);
      toml_kv_string (images, "widget", G_OBJECT_TYPE_NAME (button));
      toml_kv_string (images, "css_class", kinds[k].css_class ? kinds[k].css_class : "");
      toml_kv_string (images, "label", LABEL);
      g_string_append_printf (images, "margin = %d\n", MARGIN);
      toml_flags (images, "flags_expected", expected);
      toml_flags (images, "flags_rendered", rendered);
      g_string_append_printf (images, "flags_as_requested = %s\n", as_requested ? "true" : "false");
      g_string_append_printf (images, "allocated_width = %d\nallocated_height = %d\n",
                              gtk_widget_get_width (button), gtk_widget_get_height (button));
      {
        graphene_point_t origin;

        if (gtk_widget_compute_point (button, host->cells[k][st], &GRAPHENE_POINT_INIT (0, 0), &origin))
          g_string_append_printf (images, "x_in_png = %g\ny_in_png = %g\n", origin.x, origin.y);
      }
      g_string_append_printf (images, "png_width = %d\npng_height = %d\n", width, height);

      g_free (path);
      g_free (file);
    }
  }

  {
    char *path = g_build_filename (opt_out, "window.png", NULL);

    if (!render_to_png (host->window, renderer, path, &width, &height))
      host->status = 1;
    g_string_append (images, "\n[[image]]\n");
    toml_kv_string (images, "file", "window.png");
    toml_kv_string (images, "kind", "window");
    toml_kv_string (images, "widget", G_OBJECT_TYPE_NAME (host->window));
    toml_flags (images, "flags_rendered", gtk_widget_get_state_flags (host->window));
    g_string_append_printf (images, "png_width = %d\npng_height = %d\n", width, height);
    g_free (path);
  }

  gtk_version = g_strdup_printf ("%u.%u.%u", gtk_get_major_version (), gtk_get_minor_version (),
                                 gtk_get_micro_version ());
  adw_version = g_strdup_printf ("%u.%u.%u", adw_get_major_version (), adw_get_minor_version (),
                                 adw_get_micro_version ());

  g_string_append (s, "# gnome-host's index: the images in this folder and how they were rendered.\n\n[run]\n");
  toml_kv_string (s, "scheme", host->dark ? "dark" : "light");
  toml_kv_string (s, "renderer_requested", opt_renderer);
  toml_kv_string (s, "renderer", G_OBJECT_TYPE_NAME (renderer));
  toml_kv_string (s, "display", G_OBJECT_TYPE_NAME (display));
  toml_kv_string (s, "display_name", gdk_display_get_name (display));
  g_string_append_printf (s, "surface_scale = %g\n", gdk_surface_get_scale (surface));
  g_string_append_printf (s, "window_is_active = %s\n",
                          gtk_window_is_active (GTK_WINDOW (host->window)) ? "true" : "false");
  g_string_append_printf (s, "all_flags_as_requested = %s\n", all_as_requested ? "true" : "false");
  toml_kv_string (s, "gtk_runtime_version", gtk_version);
  toml_kv_string (s, "libadwaita_runtime_version", adw_version);

  g_string_append (s, "\n[environment]\n");
  {
    static const char *vars[] = {
      "GSK_RENDERER", "GDK_DEBUG", "ADW_DISABLE_PORTAL", "GDK_BACKEND", "GSETTINGS_BACKEND",
      "ADW_DEBUG_COLOR_SCHEME", "ADW_DEBUG_HIGH_CONTRAST", "ADW_DEBUG_ACCENT_COLOR",
      "WAYLAND_DISPLAY", "DISPLAY", "GTK_A11Y", "NO_AT_BRIDGE", "LANG",
    };

    for (gsize i = 0; i < G_N_ELEMENTS (vars); i++) {
      const char *value = g_getenv (vars[i]);

      if (value)
        toml_kv_string (s, vars[i], value);
    }
  }

  append_settings (s);
  append_style_manager (s);
  append_accent_check (s, host);
  append_packages (s);
  g_string_append (s, images->str);

  index_path = g_build_filename (opt_out, "index.toml", NULL);
  if (!g_file_set_contents (index_path, s->str, (gssize) s->len, &error)) {
    g_printerr ("gnome-host: %s\n", error->message);
    g_clear_error (&error);
    host->status = 1;
  }

  if (host->status == 0 && !all_as_requested)
    host->status = 3;

  g_free (index_path);
  g_free (gtk_version);
  g_free (adw_version);
  g_string_free (images, TRUE);
  g_string_free (s, TRUE);
}

static void
render_and_quit (gpointer data)
{
  Host *host = data;

  if (host->timeout_id) {
    g_source_remove (host->timeout_id);
    host->timeout_id = 0;
  }
  render_all (host);
  gtk_window_destroy (GTK_WINDOW (host->window));
}

static gboolean
tick_cb (GtkWidget     *widget,
         GdkFrameClock *clock,
         gpointer       data)
{
  Host *host = data;

  if (++host->frames < FRAMES_BEFORE_RENDER)
    return G_SOURCE_CONTINUE;

  g_idle_add_once (render_and_quit, host);
  return G_SOURCE_REMOVE;
}

static void
timeout_cb (gpointer data)
{
  Host *host = data;

  host->timeout_id = 0;
  g_printerr ("gnome-host: no frame within %d s\n", TIMEOUT_SECONDS);
  host->status = 1;
  gtk_window_destroy (GTK_WINDOW (host->window));
}

static GtkWidget *
make_button (const Kind *kind, const State *state)
{
  GtkWidget *button = state->toggle_active ? gtk_toggle_button_new_with_label (LABEL)
                                           : gtk_button_new_with_label (LABEL);

  if (kind->css_class)
    gtk_widget_add_css_class (button, kind->css_class);
  if (state->toggle_active)
    gtk_toggle_button_set_active (GTK_TOGGLE_BUTTON (button), TRUE);
  if (state->insensitive)
    gtk_widget_set_sensitive (button, FALSE);
  if (state->flags)
    gtk_widget_set_state_flags (button, state->flags, FALSE);

  /* No button takes the window's initial focus, which would add focus flags
   * to the first one; the focus-visible state sets its flags itself. */
  gtk_widget_set_focusable (button, FALSE);
  gtk_widget_set_margin_start (button, MARGIN);
  gtk_widget_set_margin_end (button, MARGIN);
  gtk_widget_set_margin_top (button, MARGIN);
  gtk_widget_set_margin_bottom (button, MARGIN);
  gtk_widget_set_halign (button, GTK_ALIGN_START);
  gtk_widget_set_valign (button, GTK_ALIGN_START);

  return button;
}

static void
activate_cb (GApplication *app,
             gpointer      data)
{
  Host *host = data;
  GtkWidget *grid;

  adw_style_manager_set_color_scheme (adw_style_manager_get_default (),
                                      host->dark ? ADW_COLOR_SCHEME_FORCE_DARK
                                                 : ADW_COLOR_SCHEME_FORCE_LIGHT);

  g_object_set (gtk_settings_get_default (),
                "gtk-enable-animations", FALSE,
                "gtk-decoration-layout", ":close",
                "gtk-font-name", "Adwaita Sans 11",
                "gtk-font-rendering", GTK_FONT_RENDERING_MANUAL,
                "gtk-hint-font-metrics", TRUE,
                "gtk-icon-theme-name", "",
                NULL);

  host->window = adw_application_window_new (GTK_APPLICATION (app));
  grid = gtk_grid_new ();

  for (gsize k = 0; k < N_KINDS; k++) {
    for (gsize st = 0; st < N_STATES; st++) {
      host->buttons[k][st] = make_button (&kinds[k], &states[st]);
      host->cells[k][st] = gtk_box_new (GTK_ORIENTATION_HORIZONTAL, 0);
      gtk_box_append (GTK_BOX (host->cells[k][st]), host->buttons[k][st]);
      gtk_grid_attach (GTK_GRID (grid), host->cells[k][st], (int) st, (int) k, 1, 1);
    }
  }

  host->accent_label = gtk_label_new (LABEL);
  gtk_widget_add_css_class (host->accent_label, "accent");
  gtk_grid_attach (GTK_GRID (grid), host->accent_label, 0, (int) N_KINDS, 1, 1);

  adw_application_window_set_content (ADW_APPLICATION_WINDOW (host->window), grid);
  gtk_widget_set_can_target (host->window, FALSE);

  gtk_widget_add_tick_callback (host->window, tick_cb, host, NULL);
  host->timeout_id = g_timeout_add_seconds_once (TIMEOUT_SECONDS, timeout_cb, host);

  gtk_window_present (GTK_WINDOW (host->window));
}

int
main (int    argc,
      char **argv)
{
  GOptionContext *context = g_option_context_new (NULL);
  GError *error = NULL;
  Host host = { 0 };
  AdwApplication *app;

  g_option_context_add_main_entries (context, entries, NULL);
  if (!g_option_context_parse (context, &argc, &argv, &error)) {
    g_printerr ("gnome-host: %s\n", error->message);
    return 1;
  }
  g_option_context_free (context);

  if (!opt_scheme || !opt_renderer || !opt_out ||
      (strcmp (opt_scheme, "light") != 0 && strcmp (opt_scheme, "dark") != 0)) {
    g_printerr ("usage: gnome-host --scheme light|dark --renderer NAME --out DIR\n");
    return 1;
  }
  host.dark = strcmp (opt_scheme, "dark") == 0;

  if (g_mkdir_with_parents (opt_out, 0755) != 0) {
    g_printerr ("gnome-host: cannot create %s\n", opt_out);
    return 1;
  }

  g_setenv ("GSK_RENDERER", opt_renderer, TRUE);
  g_setenv ("GDK_DEBUG", "no-portals", TRUE);
  g_setenv ("ADW_DISABLE_PORTAL", "1", TRUE);
  g_setenv ("GSETTINGS_BACKEND", "memory", TRUE);
  g_setenv ("ADW_DEBUG_COLOR_SCHEME", "default", TRUE);
  g_setenv ("ADW_DEBUG_HIGH_CONTRAST", "0", TRUE);
  g_setenv ("ADW_DEBUG_ACCENT_COLOR", "blue", TRUE);
  g_setenv ("GTK_A11Y", "none", TRUE);
  g_setenv ("NO_AT_BRIDGE", "1", TRUE);

  app = adw_application_new (NULL, G_APPLICATION_NON_UNIQUE);
  g_signal_connect (app, "activate", G_CALLBACK (activate_cb), &host);
  if (g_application_run (G_APPLICATION (app), 0, NULL) != 0 && host.status == 0)
    host.status = 1;
  g_object_unref (app);

  return host.status;
}
