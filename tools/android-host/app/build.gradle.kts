plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.compose)
}

android {
    namespace = "dev.nativetheme.androidhost"
    compileSdk = 37

    defaultConfig {
        applicationId = "dev.nativetheme.androidhost"
        minSdk = 37
        targetSdk = 37
        versionCode = 1
        versionName = "0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildFeatures {
        compose = true
    }
}

dependencies {
    val composeBom = platform(libs.compose.bom.beta)
    implementation(composeBom)
    implementation(libs.material3)
    implementation(libs.activity.compose)
    debugImplementation(libs.compose.ui.test.manifest)
    androidTestImplementation(composeBom)
    androidTestImplementation(libs.compose.ui.test.junit4)
    androidTestImplementation(libs.androidx.test.runner)
    androidTestImplementation(libs.androidx.test.ext.junit)
}
