package dev.nativetheme.androidhost

import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.PressInteraction
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonGroup
import androidx.compose.material3.ButtonGroupDefaults
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import org.junit.Rule
import org.junit.Test

/**
 * A ButtonGroup's press animation on the test's virtual clock (U186 of the
 * 0.8.0 measurement plan's AH-pass): three Buttons with animateWidth in a
 * ButtonGroup; the middle one pressed (the interaction a real press emits),
 * released, and pressed again three frames into its run back, then released;
 * and, for comparison, pressed and released with no re-press. Each frame's
 * time and the three buttons' widths in px go to the app's
 * files/measure/button-group-<case>.csv.
 */
@OptIn(ExperimentalMaterial3ExpressiveApi::class)
class ButtonGroupTest {
    @get:Rule
    val rule = createComposeRule()

    @Test
    fun repress() {
        record("repress", repress = true)
    }

    @Test
    fun release() {
        record("release", repress = false)
    }

    private fun record(name: String, repress: Boolean) {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val out = File(context.filesDir, "measure").also { it.mkdirs() }
        rule.mainClock.autoAdvance = false
        val sources = List(3) { MutableInteractionSource() }
        rule.setContent {
            MaterialTheme(colorScheme = lightColorScheme()) {
                ButtonGroup(overflowIndicator = { ButtonGroupDefaults.OverflowIndicator(menuState = it) }) {
                    for (index in 0 until 3) {
                        customItem(
                            buttonGroupContent = {
                                val source = remember { sources[index] }
                                Button(
                                    onClick = {},
                                    modifier = Modifier.testTag("segment$index").animateWidth(source),
                                    interactionSource = source,
                                ) { Text("Item $index") }
                            },
                            menuContent = {},
                        )
                    }
                }
            }
        }
        rule.mainClock.advanceTimeByFrame()
        rule.waitForIdle()
        val csv = StringBuilder("event,frame,clock_ms,width0_px,width1_px,width2_px\n")
        var frame = 0
        val start = rule.mainClock.currentTime
        fun row(event: String) {
            val widths = (0 until 3).map { rule.onNodeWithTag("segment$it").fetchSemanticsNode().size.width }
            csv.append("$event,$frame,${rule.mainClock.currentTime - start},${widths.joinToString(",")}\n")
        }
        fun frames(count: Int) {
            repeat(count) {
                rule.mainClock.advanceTimeByFrame()
                rule.waitForIdle()
                frame++
                row("")
            }
        }
        fun emit(event: String, interaction: PressInteraction) {
            rule.runOnUiThread { check(sources[1].tryEmit(interaction)) }
            rule.waitForIdle()
            row(event)
        }
        row("start")
        val first = PressInteraction.Press(Offset(10f, 10f))
        emit("press", first)
        frames(40)
        emit("release", PressInteraction.Release(first))
        if (repress) {
            frames(3)
            val second = PressInteraction.Press(Offset(10f, 10f))
            emit("press again", second)
            frames(40)
            emit("release", PressInteraction.Release(second))
        }
        frames(40)
        File(out, "button-group-$name.csv").writeText(csv.toString())
    }
}
