package dev.nativetheme.androidhost

import androidx.compose.foundation.background
import androidx.compose.foundation.interaction.FocusInteraction
import androidx.compose.foundation.interaction.HoverInteraction
import androidx.compose.foundation.interaction.Interaction
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.PressInteraction
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.ElevatedButton
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.unit.dp

/** The baseline colour schemes; dynamic colour is never used. */
enum class Scheme(val colors: () -> ColorScheme) {
    Light({ lightColorScheme() }),
    Dark({ darkColorScheme() }),
}

enum class Style(val label: String) {
    Filled("Filled"),
    Tonal("Tonal"),
    Outlined("Outlined"),
    Text("Text"),
    Elevated("Elevated"),
}

/**
 * A button state. The interactive states are the interaction a real input
 * would emit into the button's [MutableInteractionSource]; [press] is the
 * pointer's position, the button's centre.
 */
enum class ButtonState(val enabled: Boolean) {
    Normal(true),
    Pressed(true),
    Hovered(true),
    Focused(true),
    Disabled(false);

    fun interaction(press: Offset): Interaction? = when (this) {
        Pressed -> PressInteraction.Press(press)
        Hovered -> HoverInteraction.Enter()
        Focused -> FocusInteraction.Focus()
        Normal, Disabled -> null
    }
}

const val SPECIMEN_TAG = "specimen"
const val BUTTON_TAG = "button"

/** The margin around the button, wide enough for its shadow. */
val SpecimenMargin = 16.dp

/** One button on the scheme's surface colour, with [SpecimenMargin] around it. */
@Composable
fun Specimen(
    scheme: Scheme,
    style: Style,
    state: ButtonState,
    interactionSource: MutableInteractionSource,
) {
    MaterialTheme(colorScheme = scheme.colors()) {
        Box(
            Modifier.testTag(SPECIMEN_TAG)
                .background(MaterialTheme.colorScheme.surface)
                .padding(SpecimenMargin)
        ) {
            val modifier = Modifier.testTag(BUTTON_TAG)
            val onClick = {}
            val label: @Composable () -> Unit = { Text(style.label) }
            when (style) {
                Style.Filled -> Button(onClick, modifier, state.enabled, interactionSource = interactionSource) { label() }
                Style.Tonal -> FilledTonalButton(onClick, modifier, state.enabled, interactionSource = interactionSource) { label() }
                Style.Outlined -> OutlinedButton(onClick, modifier, state.enabled, interactionSource = interactionSource) { label() }
                Style.Text -> TextButton(onClick, modifier, state.enabled, interactionSource = interactionSource) { label() }
                Style.Elevated -> ElevatedButton(onClick, modifier, state.enabled, interactionSource = interactionSource) { label() }
            }
        }
    }
}
