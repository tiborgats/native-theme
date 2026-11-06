package dev.nativetheme.androidhost

import android.os.SystemClock
import androidx.compose.animation.core.LinearEasing
import androidx.compose.animation.core.RepeatMode
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.runtime.withFrameNanos
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.unit.dp
import java.util.Collections
import java.util.concurrent.atomic.AtomicInteger

/** What MainActivity's all-hosts mode records, read by AllHostsTest. */
object AllHostsProbe {
    val frames = AtomicInteger()
    private val events = Collections.synchronizedList(mutableListOf<String>())

    fun log(event: String) {
        events.add("${SystemClock.uptimeMillis()} $event")
    }

    fun drain(): List<String> = synchronized(events) { events.toList().also { events.clear() } }
}

/** A filled button whose interactions are logged and a box animated forever, every frame counted. */
@Composable
fun AllHostsContent() {
    val source = remember { MutableInteractionSource() }
    LaunchedEffect(source) {
        source.interactions.collect { AllHostsProbe.log("interaction ${it.javaClass.simpleName}") }
    }
    LaunchedEffect(Unit) {
        while (true) {
            withFrameNanos { AllHostsProbe.frames.incrementAndGet() }
        }
    }
    val transition = rememberInfiniteTransition(label = "all-hosts")
    val alpha by transition.animateFloat(
        initialValue = 0f,
        targetValue = 1f,
        animationSpec = infiniteRepeatable(tween(1000, easing = LinearEasing), RepeatMode.Reverse),
        label = "alpha",
    )
    Column(Modifier.windowInsetsPadding(WindowInsets.safeDrawing)) {
        Specimen(Scheme.Light, Style.Filled, ButtonState.Normal, source)
        Box(Modifier.size(48.dp).graphicsLayer { this.alpha = alpha }.background(Color.Red))
    }
}
