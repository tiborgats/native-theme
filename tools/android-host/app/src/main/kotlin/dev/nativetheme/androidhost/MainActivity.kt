package dev.nativetheme.androidhost

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier

/** Every style in the normal and disabled states, in both schemes, for a look by hand. */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            Column(Modifier.windowInsetsPadding(WindowInsets.safeDrawing).verticalScroll(rememberScrollState())) {
                for (scheme in Scheme.entries) {
                    for (style in Style.entries) {
                        Row {
                            for (state in listOf(ButtonState.Normal, ButtonState.Disabled)) {
                                Specimen(scheme, style, state, remember { MutableInteractionSource() })
                            }
                        }
                    }
                }
            }
        }
    }
}
