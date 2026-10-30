package dev.nativetheme.androidhost

import android.graphics.Bitmap
import android.os.Build
import android.system.Os
import android.system.OsConstants
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.asAndroidBitmap
import androidx.compose.ui.test.captureToImage
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Renders every (scheme, style, state) [Specimen] and writes it as a PNG, with
 * index.toml (the device's build, the density, each PNG's button bounds), into
 * the app's files/capture/. Each specimen is composed fresh; an interactive
 * state's interaction is emitted once the button's collectors (the ripple,
 * the elevation) run. Compose's animations run on the test's virtual clock,
 * which waitForIdle advances until they end; the press ripple is the
 * platform's RippleDrawable, animated on the render thread in real time, so
 * each capture waits [SETTLE_MS] first.
 */
@RunWith(AndroidJUnit4::class)
class CaptureTest {
    @get:Rule
    val rule = createComposeRule()

    private class Shown(
        val scheme: Scheme,
        val style: Style,
        val state: ButtonState,
        val source: MutableInteractionSource,
    )

    @Test
    fun capture() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val out = File(context.filesDir, "capture")
        out.deleteRecursively()
        out.mkdirs()

        var shown by mutableStateOf<Shown?>(null)
        rule.setContent {
            shown?.let { current ->
                key(current) { Specimen(current.scheme, current.style, current.state, current.source) }
            }
        }

        val metrics = context.resources.displayMetrics
        val index = StringBuilder()
        index.append("[device]\n")
        index.append("fingerprint = \"${Build.FINGERPRINT}\"\n")
        index.append("sdk_int = ${Build.VERSION.SDK_INT}\n")
        index.append("sdk_int_full = ${Build.VERSION.SDK_INT_FULL}\n")
        index.append("release = \"${Build.VERSION.RELEASE}\"\n")
        index.append("page_size = ${Os.sysconf(OsConstants._SC_PAGESIZE)}\n")
        index.append("density = ${metrics.density}\n")
        index.append("density_dpi = ${metrics.densityDpi}\n")
        index.append("font_scale = ${context.resources.configuration.fontScale}\n")
        index.append("settle_ms = $SETTLE_MS\n")

        for (scheme in Scheme.entries) {
            for (style in Style.entries) {
                for (state in ButtonState.entries) {
                    val current = Shown(scheme, style, state, MutableInteractionSource())
                    rule.runOnIdle { shown = current }
                    rule.waitForIdle()
                    val button = rule.onNodeWithTag(BUTTON_TAG).fetchSemanticsNode().boundsInRoot
                    val specimen = rule.onNodeWithTag(SPECIMEN_TAG).fetchSemanticsNode().boundsInRoot
                    val press = Offset(button.width / 2f, button.height / 2f)
                    state.interaction(press)?.let { interaction ->
                        rule.runOnIdle { check(current.source.tryEmit(interaction)) }
                    }
                    rule.waitForIdle()
                    Thread.sleep(SETTLE_MS)
                    rule.waitForIdle()

                    val name = "${scheme.name}-${style.name}-${state.name}".lowercase()
                    val bitmap = rule.onNodeWithTag(SPECIMEN_TAG).captureToImage().asAndroidBitmap()
                    File(out, "$name.png").outputStream().use { bitmap.compress(Bitmap.CompressFormat.PNG, 100, it) }
                    index.append("\n[[capture]]\n")
                    index.append("file = \"$name.png\"\n")
                    index.append("scheme = \"${scheme.name.lowercase()}\"\n")
                    index.append("style = \"${style.name.lowercase()}\"\n")
                    index.append("state = \"${state.name.lowercase()}\"\n")
                    index.append("size_px = [${bitmap.width}, ${bitmap.height}]\n")
                    index.append(
                        "button_px = [${button.left - specimen.left}, ${button.top - specimen.top}, " +
                            "${button.width}, ${button.height}]\n"
                    )
                }
            }
        }
        File(out, "index.toml").writeText(index.toString())
    }

    private companion object {
        const val SETTLE_MS = 1500L
    }
}
