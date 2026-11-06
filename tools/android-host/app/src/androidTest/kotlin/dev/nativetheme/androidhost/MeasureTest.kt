package dev.nativetheme.androidhost

import android.content.Context
import android.content.res.Configuration
import android.graphics.Bitmap
import android.graphics.Typeface
import android.graphics.fonts.Font
import android.graphics.text.TextRunShaper
import android.os.LocaleList
import android.os.SystemClock
import android.text.TextPaint
import android.util.TypedValue
import android.view.InputDevice
import android.view.KeyEvent
import android.view.MotionEvent
import androidx.activity.ComponentActivity
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.graphics.asAndroidBitmap
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.semantics.getOrNull
import androidx.compose.ui.test.captureToImage
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import java.util.Collections
import org.junit.Rule
import org.junit.Test

/**
 * The Android host's reads for AH-pass of the 0.8.0 measurement plan, each
 * written into the app's files/measure/:
 * - fonts.toml (U557): each sample text shaped by [TextRunShaper] with
 *   Typeface.DEFAULT (Compose's FontFamily.Default) under its locale, and the
 *   font file of each run of glyphs, the system's font fallback at work;
 * - popup-menu.toml (U564): the attributes Compose's context menu reads
 *   (ContextMenuUi.android.kt: Widget_PopupMenu's colorBackground,
 *   TextAppearance_Widget_PopupMenu_Large's textColorPrimary), resolved the
 *   same way in the activity's context, in a night-mode configuration of it
 *   and in the application's context;
 * - focus.toml and focus-*.png (U554): a filled button through a real tap
 *   (an injected touch event), a Tab key press (a key event, which leaves
 *   touch mode) and a second tap: after each step the window's touch mode,
 *   the button's focused semantics, the interactions its source emitted and
 *   the specimen's pixels.
 */
class MeasureTest {
    @get:Rule
    val rule = createAndroidComposeRule<ComponentActivity>()

    private fun outDir(): File {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        return File(context.filesDir, "measure").also { it.mkdirs() }
    }

    @Test
    fun fallbackFonts() {
        val text = StringBuilder(
            "# MeasureTest.fallbackFonts: TextRunShaper.shapeTextRun with Typeface.DEFAULT, text size 40 px, the\n" +
                "# sample's locale as the paint's textLocales; each run of consecutive glyphs drawn from one font.\n"
        )
        for ((locale, sample) in FALLBACK_SAMPLES) {
            val paint = TextPaint().apply {
                typeface = Typeface.DEFAULT
                textSize = 40f
                textLocales = LocaleList.forLanguageTags(locale)
            }
            val rtl = RTL_LANGUAGES.any { locale.startsWith(it) }
            val glyphs = TextRunShaper.shapeTextRun(sample, 0, sample.length, 0, sample.length, 0f, 0f, rtl, paint)
            val runs = mutableListOf<String>()
            var last: String? = null
            for (i in 0 until glyphs.glyphCount()) {
                val font = describe(glyphs.getFont(i))
                if (font != last) {
                    runs.add(font)
                    last = font
                }
            }
            text.append("\n[[sample]]\n")
                .append("locale = ${q(locale)}\n")
                .append("text = ${q(sample)}\n")
                .append("rtl = $rtl\n")
                .append("glyphs = ${glyphs.glyphCount()}\n")
                .append("fonts = [${runs.joinToString(", ") { q(it) }}]\n")
        }
        File(outDir(), "fonts.toml").writeText(text.toString())
    }

    private fun describe(font: Font): String {
        val axes = font.axes?.joinToString(" ") { "${it.tag}=${it.styleValue}" } ?: ""
        return "${font.file?.path ?: "(no file)"} index ${font.ttcIndex} weight ${font.style.weight} " +
            "slant ${font.style.slant} locales [${font.localeList.toLanguageTags()}] axes [$axes]"
    }

    @Test
    fun popupMenuColors() {
        var composeContext: Context? = null
        rule.setContent { composeContext = LocalContext.current }
        rule.waitForIdle()
        val activityContext = checkNotNull(composeContext) { "no LocalContext" }
        val night = Configuration(activityContext.resources.configuration).apply {
            uiMode = (uiMode and Configuration.UI_MODE_NIGHT_MASK.inv()) or Configuration.UI_MODE_NIGHT_YES
        }
        val text = StringBuilder(
            "# MeasureTest.popupMenuColors: obtainStyledAttributes(style, [attr]) as ContextMenuUi.android.kt\n" +
                "# calls it; a value not resolved is recorded as has_value = false (Compose then uses its own default).\n"
        )
        val activity = rule.activity
        val activityTheme = activity.packageManager.getActivityInfo(activity.componentName, 0).themeResource
        text.append("\n[activity]\n")
            .append("class = ${q(activity.javaClass.name)}\n")
            .append("theme_resource = ${q(resourceName(activity, activityTheme))}\n")
            .append("application_theme_resource = ${q(resourceName(activity, activity.applicationInfo.theme))}\n")
            .append("target_sdk = ${activity.applicationInfo.targetSdkVersion}\n")
        for ((name, context) in listOf(
            "activity" to activityContext,
            "activity_night" to activityContext.createConfigurationContext(night),
            "application" to activityContext.applicationContext,
        )) {
            text.append(resolve(name, context))
        }
        File(outDir(), "popup-menu.toml").writeText(text.toString())
    }

    private fun resourceName(context: Context, id: Int): String =
        if (id == 0) "0" else runCatching { context.resources.getResourceName(id) }.getOrElse { "0x" + Integer.toHexString(id) }

    private fun resolve(name: String, context: Context): String {
        val text = StringBuilder("\n[[context]]\nname = ${q(name)}\n")
        val nightMode = context.resources.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK
        text.append("ui_mode_night = ${nightMode == Configuration.UI_MODE_NIGHT_YES}\n")
        val themeValue = TypedValue()
        if (context.theme.resolveAttribute(android.R.attr.colorBackground, themeValue, true)) {
            text.append("theme_color_background = ${q(hex(themeValue.data))}\n")
        }
        val background = context.obtainStyledAttributes(android.R.style.Widget_PopupMenu, intArrayOf(android.R.attr.colorBackground))
        text.append("widget_popup_menu_color_background_has_value = ${background.hasValue(0)}\n")
        if (background.hasValue(0)) {
            text.append("widget_popup_menu_color_background = ${q(hex(background.getColor(0, 0)))}\n")
        }
        background.recycle()
        val foreground = context.obtainStyledAttributes(
            android.R.style.TextAppearance_Widget_PopupMenu_Large,
            intArrayOf(android.R.attr.textColorPrimary),
        )
        text.append("text_appearance_popup_menu_large_text_color_primary_has_value = ${foreground.hasValue(0)}\n")
        val states = foreground.getColorStateList(0)
        foreground.recycle()
        if (states != null) {
            val enabled = states.getColorForState(intArrayOf(android.R.attr.state_enabled), SENTINEL)
            val disabled = states.getColorForState(intArrayOf(-android.R.attr.state_enabled), SENTINEL)
            text.append("text_color_primary_enabled = ${q(if (enabled == SENTINEL) "unresolved" else hex(enabled))}\n")
            text.append("text_color_primary_disabled = ${q(if (disabled == SENTINEL) "unresolved" else hex(disabled))}\n")
            text.append("text_color_primary_default = ${q(hex(states.defaultColor))}\n")
        }
        return text.toString()
    }

    @Test
    fun focusVisibility() {
        val instrumentation = InstrumentationRegistry.getInstrumentation()
        val interactions = Collections.synchronizedList(mutableListOf<String>())
        rule.setContent {
            val source = remember { MutableInteractionSource() }
            LaunchedEffect(source) {
                source.interactions.collect { interactions.add(it.javaClass.simpleName) }
            }
            Specimen(Scheme.Light, Style.Filled, ButtonState.Normal, source)
        }
        rule.waitForIdle()
        val out = outDir()
        val text = StringBuilder(
            "# MeasureTest.focusVisibility: a filled button (light scheme); each step's touch mode, focused\n" +
                "# semantics, the interactions emitted since the previous step, and the specimen's PNG.\n"
        )

        fun step(name: String) {
            rule.waitForIdle()
            Thread.sleep(SETTLE_MS)
            rule.waitForIdle()
            val node = rule.onNodeWithTag(BUTTON_TAG).fetchSemanticsNode()
            val bitmap = rule.onNodeWithTag(SPECIMEN_TAG).captureToImage().asAndroidBitmap()
            File(out, "focus-$name.png").outputStream().use { bitmap.compress(Bitmap.CompressFormat.PNG, 100, it) }
            val emitted = synchronized(interactions) { interactions.toList().also { interactions.clear() } }
            text.append("\n[[step]]\n")
                .append("name = ${q(name)}\n")
                .append("png = ${q("focus-$name.png")}\n")
                .append("in_touch_mode = ${rule.activity.window.decorView.isInTouchMode}\n")
                .append("focused = ${node.config.getOrNull(SemanticsProperties.Focused)}\n")
                .append("interactions = [${emitted.joinToString(", ") { q(it) }}]\n")
        }

        fun tap() {
            val node = rule.onNodeWithTag(BUTTON_TAG).fetchSemanticsNode()
            val location = IntArray(2)
            rule.activity.window.decorView.getLocationOnScreen(location)
            val x = location[0] + node.boundsInWindow.center.x
            val y = location[1] + node.boundsInWindow.center.y
            val down = SystemClock.uptimeMillis()
            for ((action, time) in listOf(MotionEvent.ACTION_DOWN to down, MotionEvent.ACTION_UP to down + 80)) {
                val event = MotionEvent.obtain(down, time, action, x, y, 0)
                event.source = InputDevice.SOURCE_TOUCHSCREEN
                check(instrumentation.uiAutomation.injectInputEvent(event, true)) { "touch event not injected" }
                event.recycle()
                if (action == MotionEvent.ACTION_DOWN) Thread.sleep(80)
            }
        }

        step("0-initial")
        tap()
        step("1-tap")
        instrumentation.sendKeyDownUpSync(KeyEvent.KEYCODE_TAB)
        step("2-tab")
        tap()
        step("3-tap-again")
        File(out, "focus.toml").writeText(text.toString())
    }

    companion object {
        const val SETTLE_MS = 1500L
        const val SENTINEL = 0x00abcdef
        val RTL_LANGUAGES = listOf("ar", "fa", "he")

        /** Each sample's locale and text: the scripts of the model's per-language list, Han under each CJK locale, emoji and symbols. */
        val FALLBACK_SAMPLES = listOf(
            "en-US" to "Aa",
            "el-GR" to "Ωω",
            "ru-RU" to "Жж",
            "ar-SA" to "عربي",
            "fa-IR" to "فارسی",
            "he-IL" to "עברית",
            "hi-IN" to "हिन्दी",
            "bn-IN" to "বাংলা",
            "ta-IN" to "தமிழ்",
            "th-TH" to "ไทย",
            "lo-LA" to "ລາວ",
            "km-KH" to "ខ្មែរ",
            "my-MM" to "မြန်မာ",
            "si-LK" to "සිංහල",
            "hy-AM" to "Հայ",
            "ka-GE" to "ქართ",
            "am-ET" to "አማርኛ",
            "ja-JP" to "直あア",
            "zh-CN" to "直汉",
            "zh-TW" to "直漢",
            "zh-HK" to "直漢",
            "ko-KR" to "直한",
            "en-US" to "😀",
            "en-US" to "☃→✓",
        )

        fun hex(color: Int): String = "#%08x".format(color)

        fun q(value: String): String =
            "\"" + value.replace("\\", "\\\\").replace("\"", "\\\"") + "\""
    }
}
