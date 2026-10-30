package dev.nativetheme.androidhost

import android.graphics.Bitmap
import android.os.Build
import android.system.Os
import android.system.OsConstants
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.asAndroidBitmap
import androidx.compose.ui.test.captureToImage
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.runners.Parameterized

/**
 * Renders one (scheme, style, state) [Specimen] and writes it as a PNG, with
 * a TOML fragment (its button's bounds in the PNG) and device.toml (the
 * device's build, the density), into the app's files/capture/.
 *
 * Each specimen gets its own activity: Material's press ripple is the
 * platform's RippleDrawable in a RippleHostView that the window's
 * RippleContainer pools, and a host reused after a pressed button left the
 * composition draws no ripple (RippleHostView.disposeRipple hides its
 * drawable). The state's interaction is emitted once the button's collectors
 * (the ripple, the elevation) run. Compose's animations run on the test's
 * virtual clock, which waitForIdle advances until they end; the ripple
 * animates on the render thread in real time, so the capture waits
 * [SETTLE_MS] first.
 */
@RunWith(Parameterized::class)
class CaptureTest(
    private val scheme: Scheme,
    private val style: Style,
    private val state: ButtonState,
) {
    @get:Rule
    val rule = createComposeRule()

    @Test
    fun capture() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val out = File(context.filesDir, "capture")
        out.mkdirs()

        val source = MutableInteractionSource()
        rule.setContent { Specimen(scheme, style, state, source) }
        rule.waitForIdle()
        val button = rule.onNodeWithTag(BUTTON_TAG).fetchSemanticsNode().boundsInRoot
        val specimen = rule.onNodeWithTag(SPECIMEN_TAG).fetchSemanticsNode().boundsInRoot
        state.interaction(Offset(button.width / 2f, button.height / 2f))?.let { interaction ->
            rule.runOnIdle { check(source.tryEmit(interaction)) }
        }
        rule.waitForIdle()
        Thread.sleep(SETTLE_MS)
        rule.waitForIdle()

        val name = "${scheme.name}-${style.name}-${state.name}".lowercase()
        val bitmap = rule.onNodeWithTag(SPECIMEN_TAG).captureToImage().asAndroidBitmap()
        File(out, "$name.png").outputStream().use { bitmap.compress(Bitmap.CompressFormat.PNG, 100, it) }
        File(out, "$name.toml").writeText(
            "\n[[capture]]\n" +
                "file = \"$name.png\"\n" +
                "scheme = \"${scheme.name.lowercase()}\"\n" +
                "style = \"${style.name.lowercase()}\"\n" +
                "state = \"${state.name.lowercase()}\"\n" +
                "size_px = [${bitmap.width}, ${bitmap.height}]\n" +
                "button_px = [${button.left - specimen.left}, ${button.top - specimen.top}, " +
                "${button.width}, ${button.height}]\n"
        )

        val metrics = context.resources.displayMetrics
        File(out, "device.toml").writeText(
            "[device]\n" +
                "fingerprint = \"${Build.FINGERPRINT}\"\n" +
                "sdk_int = ${Build.VERSION.SDK_INT}\n" +
                "sdk_int_full = ${Build.VERSION.SDK_INT_FULL}\n" +
                "release = \"${Build.VERSION.RELEASE}\"\n" +
                "page_size = ${Os.sysconf(OsConstants._SC_PAGESIZE)}\n" +
                "density = ${metrics.density}\n" +
                "density_dpi = ${metrics.densityDpi}\n" +
                "font_scale = ${context.resources.configuration.fontScale}\n" +
                "settle_ms = $SETTLE_MS\n"
        )
    }

    companion object {
        const val SETTLE_MS = 1500L

        @JvmStatic
        @Parameterized.Parameters(name = "{0}-{1}-{2}")
        fun specimens(): List<Array<Any>> =
            Scheme.entries.flatMap { scheme ->
                Style.entries.flatMap { style ->
                    ButtonState.entries.map { state -> arrayOf<Any>(scheme, style, state) }
                }
            }
    }
}
