package dev.nativetheme.androidhost

import android.accessibilityservice.AccessibilityService
import android.content.Intent
import android.os.SystemClock
import android.view.KeyEvent
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import org.junit.Test

/**
 * The all-hosts passes of the 0.8.0 measurement plan on Android: MainActivity
 * in its all-hosts mode (a filled button and an infinite animation, the
 * frames Compose's frame clock delivers counted, the activity's lifecycle and
 * window focus and the button's interactions logged, all on the real clock).
 * The button is focused by a Tab key; the notification shade is opened (the
 * window loses focus, U190) and closed; then Home is pressed (the activity is
 * stopped, U187) and the activity brought back. The frame count and the log
 * at each step go to the app's files/measure/all-hosts.toml.
 */
class AllHostsTest {
    @Test
    fun deactivateAndMinimise() {
        val instrumentation = InstrumentationRegistry.getInstrumentation()
        val context = instrumentation.targetContext
        val out = File(context.filesDir, "measure").also { it.mkdirs() }
        val text = StringBuilder(
            "# AllHostsTest: the frames Compose's frame clock delivered (withFrameNanos) and the events logged\n" +
                "# (lifecycle, window focus, the button's interactions) at each step, in uptime milliseconds.\n"
        )
        val intent = Intent(context, MainActivity::class.java).putExtra(MainActivity.ALL_HOSTS, true)
        ActivityScenario.launch<MainActivity>(intent).use {
            fun step(name: String, waitMs: Long) {
                val before = AllHostsProbe.frames.get()
                val from = SystemClock.uptimeMillis()
                Thread.sleep(waitMs)
                val frames = AllHostsProbe.frames.get() - before
                val elapsed = SystemClock.uptimeMillis() - from
                val events = AllHostsProbe.drain()
                text.append("\n[[step]]\n")
                    .append("name = ${MeasureTest.q(name)}\n")
                    .append("elapsed_ms = $elapsed\n")
                    .append("frames = $frames\n")
                    .append("events = [${events.joinToString(", ") { MeasureTest.q(it) }}]\n")
            }
            step("0-running", 1500)
            instrumentation.sendKeyDownUpSync(KeyEvent.KEYCODE_TAB)
            step("1-tab", 1500)
            instrumentation.uiAutomation.performGlobalAction(AccessibilityService.GLOBAL_ACTION_NOTIFICATIONS)
            step("2-notification-shade-open", 2000)
            instrumentation.uiAutomation.performGlobalAction(AccessibilityService.GLOBAL_ACTION_BACK)
            step("3-notification-shade-closed", 2000)
            instrumentation.uiAutomation.performGlobalAction(AccessibilityService.GLOBAL_ACTION_HOME)
            step("4-home", 2500)
            context.startActivity(
                Intent(context, MainActivity::class.java)
                    .putExtra(MainActivity.ALL_HOSTS, true)
                    .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_REORDER_TO_FRONT)
            )
            step("5-brought-back", 2500)
        }
        File(out, "all-hosts.toml").writeText(text.toString())
    }
}
