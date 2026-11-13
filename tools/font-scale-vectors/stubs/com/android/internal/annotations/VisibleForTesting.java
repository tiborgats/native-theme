// Stands in for the annotation the pinned Java code imports; it has no effect on the code.
package com.android.internal.annotations;

public @interface VisibleForTesting {
    Visibility visibility() default Visibility.PRIVATE;

    enum Visibility {
        PROTECTED,
        PACKAGE,
        PRIVATE
    }
}
