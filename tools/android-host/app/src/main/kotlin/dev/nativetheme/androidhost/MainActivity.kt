package dev.nativetheme.androidhost

import android.content.Intent
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

/**
 * Every style in the normal and disabled states, in both schemes, for a look by hand; with the [ALL_HOSTS] extra,
 * [AllHostsContent] for AllHostsTest, its lifecycle and window focus logged to [AllHostsProbe].
 */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        if (intent.getBooleanExtra(ALL_HOSTS, false)) {
            AllHostsProbe.log("onCreate")
            setContent { AllHostsContent() }
            return
        }
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

    override fun onStart() {
        super.onStart()
        AllHostsProbe.log("onStart")
    }

    override fun onResume() {
        super.onResume()
        AllHostsProbe.log("onResume")
    }

    override fun onPause() {
        super.onPause()
        AllHostsProbe.log("onPause")
    }

    override fun onStop() {
        super.onStop()
        AllHostsProbe.log("onStop")
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        AllHostsProbe.log("onNewIntent")
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        AllHostsProbe.log("onWindowFocusChanged $hasFocus")
    }

    companion object {
        /** The intent extra that opens the all-hosts mode (AllHostsTest). */
        const val ALL_HOSTS = "all_hosts"
    }
}
