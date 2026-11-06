/* Stands in for the config.h libadwaita's meson build generates: the files
 * compiled here (adw-accent-color.c, adw-color-utils.c) read nothing from it
 * on the path the harness calls (HAVE_SINCOSF selects only how
 * oklch_to_oklab computes a sine and a cosine, which the snapping never
 * calls). */
