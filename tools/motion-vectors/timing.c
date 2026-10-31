/* The harness for libadwaita's cubic Bezier and spring (generate.py): the functions are copied in unchanged. */
#include <float.h>
#include <glib.h>
#include <math.h>
#include <stdio.h>

#define DELTA 0.001
#define MAX_ITERATIONS 20000
#define ADW_DURATION_INFINITE ((guint) 0xffffffff)

typedef struct {
  double damping, mass, stiffness;
} AdwSpringParams;

static double adw_spring_params_get_damping (AdwSpringParams *p) { return p->damping; }
static double adw_spring_params_get_mass (AdwSpringParams *p) { return p->mass; }
static double adw_spring_params_get_stiffness (AdwSpringParams *p) { return p->stiffness; }

/* adw-spring-params.c:85-96 */
static AdwSpringParams spring_params_new (double damping_ratio, double mass, double stiffness)
{
  double critical_damping = 2 * sqrt (mass * stiffness);
  AdwSpringParams p = { damping_ratio * critical_damping, mass, stiffness };
  return p;
}

typedef struct {
  double value_from;
  double value_to;
  AdwSpringParams *spring_params;
  double initial_velocity;
  double velocity;
  double epsilon;
  gboolean clamp;
  guint estimated_duration;
} AdwSpringAnimation;

/* ADW_EASING */
/* ADW_SPRING */

/* adw_spring_animation_real_calculate_value (:265-282) */
static double value_at (AdwSpringAnimation *self, guint t)
{
  if (t >= self->estimated_duration)
    return self->value_to;
  return oscillate (self, t, NULL);
}

int main (void)
{
  static const double ts[] = { 0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0 };
  /* The model states the points and the spring's parameters as f32 (S2.7): each passed as the float it is. */
  static const float curves[][4] = { { 0.25f, 0.1f, 0.25f, 1.0f }, { 0.4f, 0.0f, 0.2f, 1.0f },
                                     { 0.3f, 1.5f, 0.7f, -0.5f } };
  for (size_t c = 0; c < G_N_ELEMENTS (curves); c++) {
    printf ("cubic_bezier %g %g %g %g:", curves[c][0], curves[c][1], curves[c][2], curves[c][3]);
    for (size_t i = 0; i < G_N_ELEMENTS (ts); i++)
      printf (" %.17g", ease_cubic_bezier (ts[i], 1, curves[c][0], curves[c][1], curves[c][2], curves[c][3]));
    printf ("\n");
  }
  /* CSS Easing Functions Level 1, 2.3.2: the steps transcribed. */
  static const char *jumps[] = { "jump_start", "jump_end", "jump_none", "jump_both" };
  for (int j = 0; j < 4; j++) {
    int count = 3;
    printf ("steps %d %s:", count, jumps[j]);
    for (size_t i = 0; i < G_N_ELEMENTS (ts); i++) {
      double step = floor (ts[i] * count);
      if (j == 0 || j == 3)
        step += 1;
      double n = j == 3 ? count + 1 : j == 2 ? count - 1 : count;
      if (step > n)
        step = n;
      if (step < 0)
        step = 0;
      printf (" %.17g", step / n);
    }
    printf ("\n");
  }
  /* (damping ratio, mass, stiffness, initial velocity, epsilon, clamp) */
  static const float springs[][6] = {
    { 1.0f, 1.0f, 1000.0f, 0.0f, 0.001f, 0 },
    { 0.5f, 1.0f, 400.0f, 0.0f, 0.001f, 0 },
    { 2.0f, 1.0f, 300.0f, 0.0f, 0.001f, 0 },
    { 0.5f, 1.0f, 400.0f, 0.0f, 0.001f, 1 },
  };
  for (size_t s = 0; s < G_N_ELEMENTS (springs); s++) {
    AdwSpringParams params = spring_params_new (springs[s][0], springs[s][1], springs[s][2]);
    AdwSpringAnimation self = { 0.0, 1.0, &params, springs[s][3], 0.0, springs[s][4], (gboolean) springs[s][5], 0 };
    self.estimated_duration = calculate_duration (&self);
    printf ("spring %g %g %g %g %g %d (duration %u):", springs[s][0], springs[s][1], springs[s][2], springs[s][3],
            springs[s][4], (int) springs[s][5], self.estimated_duration);
    for (size_t i = 0; i < G_N_ELEMENTS (ts); i++)
      printf (" %.17g", value_at (&self, (guint) (ts[i] * self.estimated_duration)));
    printf ("\n");
  }
  return 0;
}
