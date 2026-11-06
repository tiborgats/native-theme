// Stands in for the class android.util.MathUtils imports for `fitRect`, which the harness never calls; the
// methods only let MathUtils compile.
package android.graphics;

public class Rect {
    public boolean isEmpty() {
        throw new UnsupportedOperationException();
    }

    public int width() {
        throw new UnsupportedOperationException();
    }

    public int height() {
        throw new UnsupportedOperationException();
    }

    public void scale(float scale) {
        throw new UnsupportedOperationException();
    }
}
