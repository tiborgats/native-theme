// AGP 9 compiles Kotlin itself (built-in Kotlin); the Kotlin plugin is only
// put on the classpath, so that the built-in Kotlin and the Compose compiler
// plugin are the same pinned version.
plugins {
    alias(libs.plugins.android.application) apply false
    alias(libs.plugins.kotlin.android) apply false
    alias(libs.plugins.kotlin.compose) apply false
}
