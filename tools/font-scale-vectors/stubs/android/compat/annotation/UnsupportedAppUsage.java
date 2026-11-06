// Stands in for the annotation the pinned Java code imports; it has no effect on the code.
package android.compat.annotation;

public @interface UnsupportedAppUsage {
    int maxTargetSdk() default Integer.MAX_VALUE;

    long trackingBug() default 0;
}
