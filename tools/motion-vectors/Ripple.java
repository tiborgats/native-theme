// The harness for RippleShader's uniforms and its AGSL, and RippleAnimationSession's lifecycle (generate.py).
import java.util.HashMap;
import java.util.Map;

public final class Ripple {
    private final Map<String, float[]> uniforms = new HashMap<>();

    private void setFloatUniform(String name, float... values) {
        uniforms.put(name, values.clone());
    }

    // RippleShader.java, copied unchanged:
    /* RIPPLE_SHADER */

    private float u(String name, int i) {
        return uniforms.get(name)[i];
    }

    // The AGSL of RippleShader.java:42-122, statement by statement in float.
    static float fract(float x) { return x - (float) Math.floor(x); }
    static float step(float edge, float x) { return x < edge ? 0f : 1f; }
    static float clamp(float x, float lo, float hi) { return Math.min(Math.max(x, lo), hi); }
    static float saturate(float x) { return clamp(x, 0f, 1f); }
    static float smoothstep(float e0, float e1, float x) {
        float t = clamp((x - e0) / (e1 - e0), 0f, 1f);
        return t * t * (3f - 2f * t);
    }
    static float mod(float x, float y) { return x - y * (float) Math.floor(x / y); }
    static float mix(float x, float y, float a) { return x * (1f - a) + y * a; }
    static float sin(float x) { return (float) Math.sin(x); }
    static float distance(float ax, float ay, float bx, float by) {
        float dx = ax - bx, dy = ay - by;
        return (float) Math.sqrt(dx * dx + dy * dy);
    }
    static final float PI = (float) 3.1415926535897932384626;

    static float triangleNoise(float nx, float ny) {
        nx = fract(nx * 5.3987f);
        ny = fract(ny * 5.4421f);
        float d = ny * (nx + 21.5351f) + nx * (ny + 14.3137f);
        nx += d;
        ny += d;
        float xy = nx * ny;
        return fract(xy * 95.4307f) + fract(xy * 75.04961f) - 1.0f;
    }
    static float threshold(float v, float l, float h) { return step(l, v) * (1.0f - step(h, v)); }
    float sparkles(float ux, float uy, float t) {
        float n = triangleNoise(ux, uy);
        float s = 0.0f;
        for (float i = 0; i < 4; i += 1) {
            float l = i * 0.1f;
            float h = l + 0.05f;
            float o = sin(PI * (t + 0.35f * i));
            s += threshold(n + o, l, h);
        }
        return saturate(s) * sparkleA;
    }
    static float softCircle(float ux, float uy, float x, float y, float radius, float blur) {
        float blurHalf = blur * 0.5f;
        float d = distance(ux, uy, x, y);
        float ratio = d / radius;
        if (Float.isNaN(ratio) || Float.isInfinite(ratio)) ratio = Float.isInfinite(ratio) ? ratio : 0f;
        return 1f - smoothstep(1f - blurHalf, 1f + blurHalf, ratio);
    }
    static float softRing(float ux, float uy, float x, float y, float radius, float progress, float blur) {
        float thickness = 0.05f * radius;
        float currentRadius = radius * progress;
        float outer = softCircle(ux, uy, x, y, currentRadius + thickness, blur);
        float inner = softCircle(ux, uy, x, y, Math.max(currentRadius - thickness, 0f), blur);
        return saturate(outer - inner);
    }
    static float subProgress(float start, float end, float progress) {
        float sub = clamp(progress, start, end);
        return (sub - start) / (end - start);
    }
    static float circleGrid(float rx, float ry, float cx0, float cy0, float cxc, float cyc, float rotx, float roty,
                            float cell) {
        float dx = cxc - cx0, dy = cyc - cy0;
        float x = rotx * dx + roty * dy + cxc;
        float y = -roty * dx + rotx * dy + cyc;
        x = mod(x, cell) / rx;
        y = mod(y, cell) / ry;
        float normalRadius = cell / ry * 0.5f;
        float radius = 0.65f * normalRadius;
        return softCircle(x, y, normalRadius, normalRadius, radius, radius * 50.0f);
    }
    float turbulence(float ux, float uy) {
        float sx = 0.8f, sy = 0.8f;
        ux = ux * sx;
        uy = uy * sy;
        float g1 = circleGrid(sx, sy, ux, uy, u("in_tCircle1", 0), u("in_tCircle1", 1), u("in_tRotation1", 0),
                u("in_tRotation1", 1), 0.17f);
        float g2 = circleGrid(sx, sy, ux, uy, u("in_tCircle2", 0), u("in_tCircle2", 1), u("in_tRotation2", 0),
                u("in_tRotation2", 1), 0.2f);
        float g3 = circleGrid(sx, sy, ux, uy, u("in_tCircle3", 0), u("in_tCircle3", 1), u("in_tRotation3", 0),
                u("in_tRotation3", 1), 0.275f);
        float v = (g1 * g1 + g2 - g3) * 0.5f;
        return saturate(0.45f + 0.8f * v);
    }

    float colorR, colorG, colorB, colorA, sparkleR = 1f, sparkleG = 1f, sparkleB = 1f, sparkleA = 0x8d / 255f;
    float originX, originY, touchX, touchY, progress;
    boolean hasMask;
    float boxW, boxH;

    float[] main(float px, float py) {
        float fadeIn = subProgress(0f, 0.13f, progress);
        float scaleIn = subProgress(0f, 1.0f, progress);
        float fadeOutNoise = subProgress(0.4f, 0.5f, progress);
        float fadeOutRipple = subProgress(0.4f, 1f, progress);
        float c = saturate(progress * 2.0f);
        float centerX = mix(touchX, originX, c), centerY = mix(touchY, originY, c);
        float maxRadius = u("in_maxRadius", 0);
        float ring = softRing(px, py, centerX, centerY, maxRadius, scaleIn, 1f);
        float alpha = Math.min(fadeIn, 1f - fadeOutNoise);
        float ux = px * u("in_resolutionScale", 0), uy = py * u("in_resolutionScale", 1);
        float dux = ux - mod(ux, u("in_noiseScale", 0)), duy = uy - mod(uy, u("in_noiseScale", 1));
        float turb = turbulence(ux, uy);
        float sparkleAlpha = sparkles(dux, duy, u("in_noisePhase", 0)) * ring * alpha * turb;
        float fade = Math.min(fadeIn, 1f - fadeOutRipple);
        float waveAlpha = softCircle(px, py, centerX, centerY, maxRadius * scaleIn, 1f) * fade * colorA;
        float[] wave = { colorR * waveAlpha, colorG * waveAlpha, colorB * waveAlpha, waveAlpha };
        float[] sparkle = { sparkleR * sparkleA, sparkleG * sparkleA, sparkleB * sparkleA, sparkleA };
        float mask = hasMask ? (px >= 0 && py >= 0 && px < boxW && py < boxH ? 1f : 0f) : 1f;
        float[] out = new float[4];
        for (int i = 0; i < 4; i++) out[i] = mix(wave[i], sparkle[i], sparkleAlpha) * mask;
        return out;
    }

    // RippleAnimationSession.java:42-48: 450 ms by (0.4, 0, 0.2, 1) to 0.5, then 375 ms linear to 1 from the later
    // of the release and the enter's end; the noise phase from the start's uptime by 7000 / 214 over 7000 ms.
    static double bezier(double s, double c1, double c2) {
        double omt = 1 - s;
        return 3 * omt * omt * s * c1 + 3 * omt * s * s * c2 + s * s * s;
    }
    static double fastOutSlowIn(double x) {
        if (x <= 0) return 0;
        if (x >= 1) return 1;
        double lo = 0, hi = 1;
        for (int i = 0; i < 200; i++) {
            double mid = (lo + hi) / 2;
            if (bezier(mid, 0.4, 0.2) < x) lo = mid; else hi = mid;
        }
        return bezier((lo + hi) / 2, 0.0, 1.0);
    }
    static Float progress(double sinceStart, Double sinceRelease) {
        double enter = sinceStart >= 450 ? 0.5 : 0.5 * fastOutSlowIn(sinceStart / 450);
        if (sinceRelease == null) return (float) enter;
        double released = Math.max(sinceStart - sinceRelease, 0);
        double exitStart = Math.max(released, 450);
        if (sinceStart < exitStart) return (float) enter;
        double e = sinceStart - exitStart;
        if (e >= 375) return null;
        return (float) (0.5 + 0.5 * (e / 375));
    }
    static float phase(float startUptime, double sinceStart) {
        long maxNoisePhase = 7000 / 214;
        float start = startUptime, end = startUptime + maxNoisePhase;
        float fraction = (float) Math.min(Math.max(sinceStart / 7000, 0), 1);
        return start + fraction * (end - start);
    }

    public static void main(String[] args) {
        double[][] lifecycle = { { 0, -1 }, { 100, -1 }, { 225, -1 }, { 449, -1 }, { 450, -1 }, { 2000, -1 },
                { 300, 100 }, { 500, 100 }, { 600, 0 }, { 700, 50 }, { 1000, 300 }, { 824, 0 }, { 825, 0 },
                { 1300, 500 } };
        for (double[] row : lifecycle) {
            Float p = progress(row[0], row[1] < 0 ? null : row[1]);
            System.out.println("ripple_progress " + row[0] + " " + row[1] + ": " + (p == null ? "none" : p));
        }
        float[][] phases = { { 120000f, 0 }, { 120000f, 3500 }, { 120000f, 7000 }, { 120000f, 9000 }, { 5f, 700 } };
        for (float[] row : phases) {
            System.out.println("ripple_phase " + row[0] + " " + row[1] + ": " + phase(row[0], row[1]));
        }
        // (progress, px, py) over a 120 x 48 box, touched at (30, 20), bounded, colour (0.2, 0.4, 0.6) at
        // alpha 0.1, radius half the diagonal (no padding), the phase 1000.
        float[][] points = { { 0.05f, 30, 20 }, { 0.1f, 40, 24 }, { 0.25f, 60, 24 }, { 0.3f, 10, 5 },
                { 0.45f, 80, 30 }, { 0.6f, 100, 40 }, { 0.6f, 130, 40 }, { 0.9f, 60, 24 }, { 0.0f, 30, 20 } };
        for (float[] row : points) {
            Ripple r = new Ripple();
            r.boxW = 120;
            r.boxH = 48;
            r.hasMask = true;
            // Compose's sRGB Color(...) packing (Color.kt:413-419), read back by Color.valueOf(int).
            r.colorR = (int) (0.2f * 255.0f + 0.5f) / 255.0f;
            r.colorG = (int) (0.4f * 255.0f + 0.5f) / 255.0f;
            r.colorB = (int) (0.6f * 255.0f + 0.5f) / 255.0f;
            r.colorA = (int) (0.1f * 255.0f + 0.5f) / 255.0f;
            r.originX = 120 >> 1;
            r.originY = 48 >> 1;
            r.touchX = 30;
            r.touchY = 20;
            r.progress = row[0];
            // Compose's targetRadius.roundToInt() (Ripple.android.kt:105).
            r.setRadius(Math.round((float) Math.sqrt(60f * 60f + 24f * 24f)));
            r.setResolution(120, 48);
            r.setNoisePhase(1000f);
            float[] out = r.main(row[1], row[2]);
            System.out.println("ripple_shade " + row[0] + " " + row[1] + " " + row[2] + ": " + out[0] + " "
                    + out[1] + " " + out[2] + " " + out[3]);
        }
    }
}
