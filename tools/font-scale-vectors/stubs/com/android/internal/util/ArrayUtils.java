// Stands in for the allocation helpers of com.android.internal.util.ArrayUtils that android.util.SparseArray and
// GrowingArrayUtils call, as Ravenwood runs them on a host JVM: the source's `newUnpadded*Array` asks
// dalvik.system.VMRuntime for an array of at least `minLen` elements, which Ravenwood's VMRuntime
// (ravenwood/libcore-fake/dalvik/system/VMRuntime.java, `newUnpaddedArray`) allocates with exactly that length.
// The rest of the class is not used.
package com.android.internal.util;

import java.lang.reflect.Array;

public final class ArrayUtils {
    private ArrayUtils() {}

    public static Object[] newUnpaddedObjectArray(int minLen) {
        return new Object[minLen];
    }

    public static int[] newUnpaddedIntArray(int minLen) {
        return new int[minLen];
    }

    public static long[] newUnpaddedLongArray(int minLen) {
        return new long[minLen];
    }

    public static boolean[] newUnpaddedBooleanArray(int minLen) {
        return new boolean[minLen];
    }

    public static float[] newUnpaddedFloatArray(int minLen) {
        return new float[minLen];
    }

    @SuppressWarnings("unchecked")
    public static <T> T[] newUnpaddedArray(Class<T> clazz, int minLen) {
        return (T[]) Array.newInstance(clazz, minLen);
    }
}
