package dev.nativetheme.androidhost

import android.graphics.Bitmap
import android.os.SystemClock
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.PressInteraction
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
 * The press ripple over time, repeated (U70, U71 of the 0.8.0 measurement
 * plan's AH-pass): the light filled button is pressed by the interaction a
 * real press emits (as [CaptureTest] presses it), in a fresh activity each
 * time, and captured once [delayMs] after the press; each (repetition, delay)
 * pair is a test of its own. Writes ripple-r<repetition>-d<delay>.png and a
 * TOML fragment with the milliseconds measured from the press to the start
 * and to the end of the capture into the app's files/measure/. The ripple
 * animates on the render thread in real time, so the capture's moment is
 * what the fragment records, not the delay asked.
 */
@RunWith(Parameterized::class)
class RippleTest(private val repetition: Int, private val delayMs: Long) {
    @get:Rule
    val rule = createComposeRule()

    @Test
    fun capture() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val out = File(context.filesDir, "measure").also { it.mkdirs() }
        val source = MutableInteractionSource()
        rule.setContent { Specimen(Scheme.Light, Style.Filled, ButtonState.Normal, source) }
        rule.waitForIdle()
        val button = rule.onNodeWithTag(BUTTON_TAG).fetchSemanticsNode().boundsInRoot
        val press = PressInteraction.Press(Offset(button.width / 2f, button.height / 2f))
        var pressed = 0L
        rule.runOnIdle {
            pressed = SystemClock.uptimeMillis()
            check(source.tryEmit(press))
        }
        val wait = delayMs - (SystemClock.uptimeMillis() - pressed)
        if (wait > 0) Thread.sleep(wait)
        val start = SystemClock.uptimeMillis() - pressed
        val bitmap = rule.onNodeWithTag(SPECIMEN_TAG).captureToImage().asAndroidBitmap()
        val end = SystemClock.uptimeMillis() - pressed
        val name = "ripple-r$repetition-d$delayMs"
        File(out, "$name.png").outputStream().use { bitmap.compress(Bitmap.CompressFormat.PNG, 100, it) }
        File(out, "$name.toml").writeText(
            "\n[[ripple]]\n" +
                "file = \"$name.png\"\n" +
                "repetition = $repetition\n" +
                "delay_asked_ms = $delayMs\n" +
                "capture_start_ms = $start\n" +
                "capture_end_ms = $end\n"
        )
    }

    companion object {
        @JvmStatic
        @Parameterized.Parameters(name = "r{0}-d{1}")
        fun cases(): List<Array<Any>> =
            (0 until 3).flatMap { repetition ->
                listOf(50L, 150L, 300L, 600L, 1200L).map { delay -> arrayOf<Any>(repetition, delay) }
            }
    }
}
